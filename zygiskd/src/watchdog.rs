use crate::constants;
use anyhow::{anyhow, Result};
use nix::fcntl::{flock, FlockArg};
use nix::unistd::{getgid, getuid};
use std::os::unix::prelude::AsRawFd;
use std::process::{Child, Command};
use std::sync::mpsc;
use std::{fs, thread};

static mut LOCK_FILE: Option<fs::File> = None;

pub fn check_permission() -> Result<()> {
    log::info!("Check permission");
    let uid = getuid();
    if uid.as_raw() != 0 {
        return Err(anyhow!("UID is not 0"));
    }

    let gid = getgid();
    if gid.as_raw() != 0 {
        return Err(anyhow!("GID is not 0"));
    }

    let context = fs::read_to_string("/proc/self/attr/current")?;
    if context != "u:r:su:s0" {
        return Err(anyhow!("SELinux context is not u:r:su:s0"));
    }

    Ok(())
}

pub fn ensure_single_instance() -> Result<()> {
    log::info!("Ensure single instance");
    let metadata = fs::metadata(constants::ZYGISKSU_DIR);
    if metadata.is_err() || !metadata.unwrap().is_dir() {
        return Err(anyhow!("Zygisksu is not installed"));
    }
    unsafe {
        match fs::File::create(constants::DAEMON_LOCK) {
            Ok(file) => LOCK_FILE = Some(file),
            Err(e) => return Err(anyhow!("Failed to open lock file: {e}")),
        };
        let fd = LOCK_FILE.as_ref().unwrap().as_raw_fd();
        if let Err(e) = flock(fd, FlockArg::LockExclusiveNonblock) {
            return Err(anyhow!(
                "Failed to acquire lock: {e}. Maybe another instance is running?"
            ));
        }
    }
    Ok(())
}

pub fn spawn_daemon() -> Result<()> {
    let daemon32 = Command::new(constants::ZYGISKD32).spawn()?;
    let daemon64 = Command::new(constants::ZYGISKD64).spawn()?;
    let (sender, receiver) = mpsc::channel();
    let spawn = |mut daemon: Child| {
        let sender = sender.clone();
        thread::spawn(move || {
            let result = daemon.wait().unwrap();
            log::error!("Daemon process {} died: {}", daemon.id(), result);
            drop(daemon);
            sender.send(()).unwrap();
        });
    };
    spawn(daemon32);
    spawn(daemon64);
    let _ = receiver.recv();
    Err(anyhow!("Daemon process died"))
}
