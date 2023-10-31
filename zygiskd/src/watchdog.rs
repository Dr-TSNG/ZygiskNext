use crate::{constants, root_impl, utils};
use anyhow::{bail, Result};
use std::fs;
use std::future::Future;
use std::io::{BufRead, BufReader, Write};
use std::pin::Pin;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use log::{debug, error, info};
use rustix::mount::mount_bind;
use rustix::process::{getgid, getuid, kill_process, Pid, Signal};
use tokio::process::{Child, Command};
use crate::utils::LateInit;

static PROP_SECTIONS: LateInit<[String; 2]> = LateInit::new();

pub async fn main() -> Result<()> {
    let result = run().await;
    if result.is_err() {
        set_prop_hint(constants::STATUS_CRASHED)?;
    }
    result
}

async fn run() -> Result<()> {
    info!("Start zygisk watchdog");
    check_permission()?;
    mount_prop().await?;
    if check_and_set_hint()? == false {
        log::warn!("Requirements not met, exiting");
        return Ok(());
    }
    spawn_daemon().await?;
    Ok(())
}

fn check_permission() -> Result<()> {
    info!("Check permission");
    let uid = getuid();
    if !uid.is_root() {
        bail!("UID is not 0");
    }

    let gid = getgid();
    if !gid.is_root() {
        bail!("GID is not 0");
    }

    let context = fs::read_to_string("/proc/self/attr/current")?;
    let context = context.trim_end_matches('\0');
    if context != "u:r:su:s0" && context != "u:r:magisk:s0" {
        bail!("SELinux context incorrect: {context}");
    }

    Ok(())
}

async fn mount_prop() -> Result<()> {
    let module_prop_file = fs::File::open(constants::PATH_MODULE_PROP)?;
    let mut section = 0;
    let mut sections: [String; 2] = [String::new(), String::new()];
    let lines = BufReader::new(module_prop_file).lines();
    for line in lines {
        let line = line?;
        if line.starts_with("description=") {
            sections[0].push_str("description=");
            sections[1].push_str(line.trim_start_matches("description="));
            sections[1].push('\n');
            section = 1;
        } else {
            sections[section].push_str(&line);
            sections[section].push('\n');
        }
    }
    PROP_SECTIONS.init(sections);

    info!("Mount {} -> {}", constants::PATH_PROP_OVERLAY, constants::PATH_MODULE_PROP);
    fs::File::create(constants::PATH_PROP_OVERLAY)?;
    mount_bind(constants::PATH_PROP_OVERLAY, constants::PATH_MODULE_PROP)?;
    Ok(())
}

fn set_prop_hint(hint: &str) -> Result<()> {
    let mut file = fs::File::create(constants::PATH_PROP_OVERLAY)?;
    file.write_all(PROP_SECTIONS[0].as_bytes())?;
    file.write_all(b"[")?;
    file.write_all(hint.as_bytes())?;
    file.write_all(b"] ")?;
    file.write_all(PROP_SECTIONS[1].as_bytes())?;
    Ok(())
}

fn check_and_set_hint() -> Result<bool> {
    let root_impl = root_impl::get_impl();
    match root_impl {
        root_impl::RootImpl::None => set_prop_hint(constants::STATUS_ROOT_IMPL_NONE)?,
        root_impl::RootImpl::TooOld => set_prop_hint(constants::STATUS_ROOT_IMPL_TOO_OLD)?,
        root_impl::RootImpl::Abnormal => set_prop_hint(constants::STATUS_ROOT_IMPL_ABNORMAL)?,
        root_impl::RootImpl::Multiple => set_prop_hint(constants::STATUS_ROOT_IMPL_MULTIPLE)?,
        _ => {
            set_prop_hint(constants::STATUS_LOADED)?;
            return Ok(true);
        }
    }
    Ok(false)
}

async fn spawn_daemon() -> Result<()> {
    let mut lives = 5;
    loop {
        let mut futures = FuturesUnordered::<Pin<Box<dyn Future<Output=Result<()>>>>>::new();
        let mut child_ids = vec![];
        let daemon32 = Command::new(constants::PATH_CP_BIN32).arg("daemon").spawn();
        let daemon64 = Command::new(constants::PATH_CP_BIN64).arg("daemon").spawn();
        async fn spawn_daemon(mut daemon: Child) -> Result<()> {
            let result = daemon.wait().await?;
            log::error!("Daemon process {} died: {}", daemon.id().unwrap(), result);
            Ok(())
        }
        if let Ok(it) = daemon32 {
            child_ids.push(it.id().unwrap());
            futures.push(Box::pin(spawn_daemon(it)));
        }
        if let Ok(it) = daemon64 {
            child_ids.push(it.id().unwrap());
            futures.push(Box::pin(spawn_daemon(it)));
        }

        if let Err(e) = futures.next().await.unwrap() {
            error!("{}", e);
        }

        for child in child_ids {
            debug!("Killing child process {}", child);
            let _ = kill_process(Pid::from_raw(child as i32).unwrap(), Signal::Kill);
        }

        lives -= 1;
        if lives == 0 {
            bail!("Too many crashes, abort");
        }

        error!("Restarting zygote...");
        utils::set_property(constants::PROP_CTL_RESTART, "zygote")?;
    }
}
