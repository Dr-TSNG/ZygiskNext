use crate::{constants, utils};
use anyhow::{bail, Result};
use nix::unistd::{getgid, getuid};
use std::process::{Child, Command};
use std::sync::mpsc;
use std::{fs, thread};
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::time::Duration;

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
    let daemon32 = Command::new(constants::PATH_ZYGISKD32).arg("daemon").spawn();
    let daemon64 = Command::new(constants::PATH_ZYGISKD64).arg("daemon").spawn();
    let (sender, receiver) = mpsc::channel();
    let mut waiting = vec![];
    let mut spawn = |mut daemon: Child, socket: &'static str| {
        waiting.push(socket);
        let sender = sender.clone();
        thread::spawn(move || {
            let result = daemon.wait().unwrap();
            log::error!("Daemon process {} died: {}", daemon.id(), result);
            drop(daemon);
            sender.send(()).unwrap();
        });
    };
    if let Ok(it) = daemon32 { spawn(it, "/dev/socket/zygote_secondary") }
    if let Ok(it) = daemon64 { spawn(it, "/dev/socket/zygote") }

    waiting.into_iter().for_each(|socket| wait_zygote(socket));
    log::info!("Zygote ready, restore native bridge");
    utils::restore_native_bridge()?;

    let _ = receiver.recv();
    bail!("Daemon process died");
}

fn wait_zygote(socket: &str) -> () {
    let path = Path::new(socket);
    loop {
        if path.exists() { return; }
        log::debug!("{socket} not exists, wait for 1s...");
        thread::sleep(Duration::from_secs(1));
    }
}
