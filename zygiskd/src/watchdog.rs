use crate::{constants, utils};
use anyhow::{bail, Result};
use nix::fcntl::{flock, FlockArg};
use nix::unistd::{getgid, getuid};
use std::os::unix::prelude::AsRawFd;
use std::process::{Child, Command};
use std::sync::mpsc;
use std::{fs, thread};
use std::path::Path;
use std::time::Duration;

static mut LOCK_FILE: Option<fs::File> = None;

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
    //TODO: remove magisk context after debug finished
    if context != "u:r:su:s0" && context != "u:r:magisk:s0" {
        bail!("SELinux context incorrect: {context}");
    }

    Ok(())
}

fn ensure_single_instance() -> Result<()> {
    log::info!("Ensure single instance");
    let metadata = fs::metadata(constants::PATH_ZYGISKSU_DIR);
    if metadata.is_err() || !metadata.unwrap().is_dir() {
        bail!("Zygisksu is not installed");
    }
    unsafe {
        match fs::File::create(constants::PATH_DAEMON_LOCK) {
            Ok(file) => LOCK_FILE = Some(file),
            Err(e) => bail!("Failed to open lock file: {e}"),
        };
        let fd = LOCK_FILE.as_ref().unwrap().as_raw_fd();
        if let Err(e) = flock(fd, FlockArg::LockExclusiveNonblock) {
            bail!("Failed to acquire lock: {e}. Maybe another instance is running?");
        }
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
