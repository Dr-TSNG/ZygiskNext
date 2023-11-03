use crate::{constants, root_impl, utils};
use anyhow::{bail, Result};
use std::fs;
use std::future::Future;
use std::io::{BufRead, BufReader, Write};
use std::os::fd::AsRawFd;
use std::path::Path;
use std::pin::Pin;
use futures::stream::FuturesUnordered;
use futures::{FutureExt, pin_mut, select, StreamExt};
use futures::future::Fuse;
use log::{debug, error, info};
use rustix::mount::mount_bind;
use rustix::process::{getgid, getuid, kill_process, Pid, Signal};
use tokio::process::{Child, Command};
use tokio::task;
use tokio::task::JoinHandle;
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

fn wait_for_ptrace(is_32bit: bool) -> Option<JoinHandle<Result<()>>> {
    let lock_path = if is_32bit {
        if !Path::new(constants::PATH_PT_BIN32).is_file() {
            return None
        }
        constants::PATH_PT_LOCK32
    } else {
        if !Path::new(constants::PATH_PT_BIN64).is_file() {
            return None
        }
        constants::PATH_PT_LOCK64
    };
    info!("wait for ptrace 32={}", is_32bit);
    Some(task::spawn_blocking(move || -> Result<()> {
        let file = match fs::OpenOptions::new().write(true).open(lock_path) {
            Ok(f) => f,
            Err(e) => {
                bail!("failed to open lock: {}", e)
            }
        };
        unsafe {
            let lock = libc::flock {
                l_type: libc::F_WRLCK as libc::c_short,
                l_whence: libc::SEEK_SET as libc::c_short,
                l_start: 0,
                l_len: 0,
                l_pid: 0,
            };
            loop {
                if libc::fcntl(file.as_raw_fd(), libc::F_SETLKW, &lock) == 0 {
                   bail!("file lock obtained")
                } else {
                    let errno = *libc::__errno();
                    match errno {
                        libc::EINTR => continue,
                        _ => {
                            bail!("failed to wait on lock: {}", errno)
                        }
                    }
                }
            }
        }
    }))
}

async fn spawn_daemon() -> Result<()> {
    let mut lives = 5;
    let lock32 = match wait_for_ptrace(true) {
        Some(f) => f.fuse(),
        None => Fuse::terminated()
    };
    let lock64 = match wait_for_ptrace(false) {
        Some(f) => f.fuse(),
        None => Fuse::terminated()
    };
    pin_mut!(lock32, lock64);
    loop {
        let mut futures = FuturesUnordered::<Pin<Box<dyn Future<Output=Result<()>>>>>::new();
        let mut child_ids = vec![];
        let daemon32 = Command::new(constants::PATH_CP_BIN32).arg("daemon").spawn();
        let daemon64 = Command::new(constants::PATH_CP_BIN64).arg("daemon").spawn();
        async fn spawn_daemon(mut daemon: Child) -> Result<()> {
            let id = daemon.id().unwrap();
            let result = daemon.wait().await?;
            // FIXME: we must not get id here
            log::error!("Daemon process {} died: {}", id, result);
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

        let mut stop = false;

        select! {
            l32 = lock32 => {
                if let Ok(Err(it)) = l32 {
                    error!("wait on lock 32: {}", it);
                }
                error!("wait on lock 32");
                stop = true;
            },
            l64 = lock64 => {
                if let Ok(Err(it)) = l64 {
                    error!("wait on lock 64: {}", it);
                }
                error!("wait on lock 64");
                stop = true;
            },
            res = futures.select_next_some() => {
                if let Err(it) = res {
                    error!("wait on daemon: {}", it);
                }
                lives -= 1;
                if lives == 0 {
                    error!("Too many crashes, abort");
                    stop = true;
                }
                error!("wait on daemon");
            },
            complete => panic!("completed unexpectedly")
        }

        for child in child_ids {
            debug!("Killing child process {}", child);
            let _ = kill_process(Pid::from_raw(child as i32).unwrap(), Signal::Kill);
        }

        if stop {
            utils::set_property(constants::PROP_CTL_SIGSTOP_OFF, "zygote")?;
            utils::set_property(constants::PROP_CTL_SIGSTOP_OFF, "zygote_secondary")?;
        }
        error!("Restarting zygote...");
        utils::set_property(constants::PROP_CTL_RESTART, "zygote")?;
        if stop {
            bail!("Injecting failed or crashed too much times, Resetting ...");
        }
    }
}
