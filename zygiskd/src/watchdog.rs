use crate::{constants, root_impl, utils};
use anyhow::{bail, Result};
use std::fs;
use std::future::Future;
use std::io::{BufRead, BufReader, Write};
use std::pin::Pin;
use std::time::Duration;
use binder::IBinder;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use log::info;
use rustix::mount::mount_bind;
use rustix::process::{getgid, getuid, kill_process, Pid, Signal};
use tokio::process::{Child, Command};
use crate::utils::LateInit;

static PROP_SECTIONS: LateInit<[String; 2]> = LateInit::new();

pub async fn main() -> Result<()> {
    let result = run().await;
    set_prop_hint(constants::STATUS_CRASHED)?;
    result
}

async fn run() -> Result<()> {
    info!("Start zygisksu watchdog");
    check_permission()?;
    mount_prop().await?;
    if check_and_set_hint()? == false {
        log::warn!("Requirements not met, exiting");
        return Ok(());
    }
    spawn_daemon().await?;
    Ok(())
}

fn spawn_fuse() -> Result<()> {
    Command::new("bin/zygisk-fuse").spawn()?;
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
    let module_prop = if let root_impl::RootImpl::Magisk = root_impl::get_impl() {
        let magisk_path = Command::new("magisk").arg("--path").output().await?;
        let mut magisk_path = String::from_utf8(magisk_path.stdout)?;
        magisk_path.pop(); // Removing '\n'
        let cwd = std::env::current_dir()?;
        let dir = cwd.file_name().unwrap().to_string_lossy();
        format!("{magisk_path}/.magisk/modules/{dir}/{}", constants::PATH_MODULE_PROP)
    } else {
        constants::PATH_MODULE_PROP.to_string()
    };
    info!("Mount {module_prop}");
    let module_prop_file = fs::File::open(&module_prop)?;
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

    fs::File::create(constants::PATH_PROP_OVERLAY)?;
    mount_bind(constants::PATH_PROP_OVERLAY, &module_prop)?;
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

        let daemon32 = Command::new(constants::PATH_CP32_BIN).spawn();
        let daemon64 = Command::new(constants::PATH_CP64_BIN).spawn();
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

        async fn binder_listener() -> Result<()> {
            let mut binder = loop {
                match binder::get_service("activity") {
                    Some(binder) => break binder,
                    None => {
                        log::trace!("System server not ready, wait for 1s...");
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                };
            };

            info!("System server ready");

            loop {
                if binder.ping_binder().is_err() { break; }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            log::error!("System server died");
            Ok(())
        }
        futures.push(Box::pin(binder_listener()));

        if let Err(e) = futures.next().await.unwrap() {
            log::error!("{}", e);
        }

        for child in child_ids {
            log::debug!("Killing child process {}", child);
            let _ = kill_process(Pid::from_raw(child as i32).unwrap(), Signal::Kill);
        }

        lives -= 1;
        if lives == 0 {
            bail!("Too many crashes, abort");
        }

        log::error!("Restarting zygote...");
        utils::set_property(constants::PROP_CTL_RESTART, "zygote")?;
    }
}
