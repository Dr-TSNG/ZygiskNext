use crate::{constants, root_impl, utils};
use anyhow::{bail, Result};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::time::Duration;
use futures::{FutureExt, pin_mut};
use futures::future::{Fuse};
use log::{debug, error, info};
use rustix::mount::mount_bind;
use rustix::process::{getgid, getuid};
use tokio::process::{Child, Command};
use tokio::{select, task};
use tokio::time::Instant;
use crate::utils::{get_property, get_property_serial, LateInit, wait_property};

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
    let mut lives = constants::MAX_RESTART_COUNT;
    let mut last_restart_time = Instant::now();
    let (sender, mut receiver) = tokio::sync::mpsc::channel(1);
    task::spawn_blocking(move || {
        let mut serial = 0u32;
        let mut last_state = "running".to_string();
        info!("zygote property monitor started");
        loop {
            let old_serial = serial;
            serial = wait_property(constants::ZYGOTE_SERVICE_PROP, serial).expect("failed to wait on property");
            let new_state = get_property(constants::ZYGOTE_SERVICE_PROP).expect("failed to get property");
            if last_state == "running" && new_state != "running" {
                info!("new zygote state: {} serial {} -> {}", new_state, old_serial, serial);
                sender.blocking_send(old_serial).expect("failed to notify");
            }
            last_state = new_state
        }
    });
    let mut restart_serial = 0u32;
    loop {
        let daemon32 = Command::new(constants::PATH_CP_BIN32).arg("daemon").spawn();
        let daemon64 = Command::new(constants::PATH_CP_BIN64).arg("daemon").spawn();
        async fn spawn_daemon(mut daemon: Child, mut killer: tokio::sync::watch::Receiver<()>) {
            let id = daemon.id().unwrap();
            select! {
                result = daemon.wait() => {
                    log::error!("Daemon process {} died: {}", id, result
                        .expect("failed to get daemon process exit reason")
                    );
                },
                _ = killer.changed() => {
                    log::warn!("Kill daemon process {}", id);
                    daemon.kill().await.expect("failed to kill");
                }
            }
        }
        let (tx, rx) = tokio::sync::watch::channel(());
        let wait32 = match daemon32 {
            Ok(child) => {
                spawn_daemon(child, rx.clone()).fuse()
            }
            Err(..) => {
                Fuse::terminated()
            }
        };
        let wait64 = match daemon64 {
            Ok(child) => {
                spawn_daemon(child, rx.clone()).fuse()
            }
            Err(..) => {
                Fuse::terminated()
            }
        };

        pin_mut!(wait32, wait64);

        let mut restart_zygote = true;

        select! {
            _ = &mut wait32 => {},
            _ = &mut wait64 => {},
            _ = async {
                // we expect a serial different from last restart
                loop {
                    if restart_serial != receiver.recv().await.expect("no serial received") {
                        break;
                    }
                }
            } => {
                restart_zygote = false;
            }
        }

        // kill all remain daemons
        tx.send(())?;

        // wait for all daemons
        loop {
            futures::select! {
                _ = wait32 => {},
                _ = wait64 => {},
                complete => { break; }
            }
        }

        let current = Instant::now();
        if current - last_restart_time >= Duration::new(30, 0) {
            lives = constants::MAX_RESTART_COUNT;
            log::warn!("reset live count to {}", lives);
        } else {
            lives -= 1;
            log::warn!("remain live count {}", lives);
        }
        if lives == 0 {
            bail!("Too many crashes, abort");
        }
        last_restart_time = current;

        error!("Daemons are going to restart ...");

        if restart_zygote {
            error!("Restarting zygote...");
            restart_serial = get_property_serial(constants::ZYGOTE_SERVICE_PROP)?;
            debug!("serial before restart {}", restart_serial);
            utils::set_property(constants::PROP_CTL_RESTART, "zygote")?;
        }
    }
}
