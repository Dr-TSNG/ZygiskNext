use crate::{constants, utils};
use anyhow::{bail, Result};
use nix::unistd::{getgid, getuid, Pid};
use std::process::{Child, Command};
use std::sync::mpsc;
use std::{fs, thread};
use std::os::unix::net::UnixListener;
use std::time::Duration;
use binder::IBinder;
use nix::sys::signal::{kill, Signal};

static mut LOCK: Option<UnixListener> = None;

pub fn entry() -> Result<()> {
    log::info!("Start zygisksu watchdog");
    check_permission()?;
    ensure_single_instance()?;
    spawn_daemon()
}

fn check_permission() -> Result<()> {
    log::info!("Check permission");
    let uid = getuid();
    if uid.as_raw() != 0 {
        bail!("UID is not 0");
    }

    let gid = getgid();
    if gid.as_raw() != 0 {
        bail!("GID is not 0");
    }

    let context = fs::read_to_string("/proc/self/attr/current")?;
    let context = context.trim_end_matches('\0');
    if context != "u:r:su:s0" && context != "u:r:magisk:s0" {
        bail!("SELinux context incorrect: {context}");
    }

    Ok(())
}

fn ensure_single_instance() -> Result<()> {
    log::info!("Ensure single instance");
    let name = String::from("zygiskwd") + constants::SOCKET_PLACEHOLDER;
    match utils::abstract_namespace_socket(&name) {
        Ok(socket) => unsafe { LOCK = Some(socket) },
        Err(e) => bail!("Failed to acquire lock: {e}. Maybe another instance is running?")
    }
    Ok(())
}

fn spawn_daemon() -> Result<()> {
    let mut lives = 5;
    loop {
        let daemon32 = Command::new(constants::PATH_ZYGISKD32).arg("daemon").spawn();
        let daemon64 = Command::new(constants::PATH_ZYGISKD64).arg("daemon").spawn();
        let mut child_ids = vec![];
        let (sender, receiver) = mpsc::channel();
        let mut spawn = |mut daemon: Child| {
            child_ids.push(daemon.id());
            let sender = sender.clone();
            thread::spawn(move || {
                let result = daemon.wait().unwrap();
                log::error!("Daemon process {} died: {}", daemon.id(), result);
                drop(daemon);
                let _ = sender.send(());
            });
        };
        if let Ok(it) = daemon32 { spawn(it) }
        if let Ok(it) = daemon64 { spawn(it) }

        let mut binder = loop {
            if receiver.try_recv().is_ok() {
                bail!("Daemon died before system server ready");
            }
            match binder::get_service("activity") {
                Some(binder) => break binder,
                None => {
                    log::trace!("System server not ready, wait for 1s...");
                    thread::sleep(Duration::from_secs(1));
                }
            };
        };
        log::info!("System server ready, restore native bridge");
        utils::set_property(constants::PROP_NATIVE_BRIDGE, &utils::get_native_bridge())?;

        loop {
            if receiver.try_recv().is_ok() || binder.ping_binder().is_err() { break; }
            thread::sleep(Duration::from_secs(1))
        }
        for child in child_ids {
            let _ = kill(Pid::from_raw(child as i32), Signal::SIGKILL);
        }

        lives -= 1;
        if lives == 0 {
            bail!("Too many crashes, abort");
        }

        log::error!("Restarting zygote...");
        utils::set_property(constants::PROP_NATIVE_BRIDGE, constants::ZYGISK_LOADER)?;
        utils::set_property(constants::PROP_SVC_ZYGOTE, "restart")?;
        thread::sleep(Duration::from_secs(2));
    }
}
