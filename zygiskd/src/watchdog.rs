use crate::utils::LateInit;
use crate::{constants, magic, root_impl, utils};
use anyhow::{bail, Result};
use binder::IBinder;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use nix::errno::Errno;
use nix::libc;
use nix::sys::signal::{kill, Signal};
use nix::unistd::{getgid, getuid, Pid};
use std::ffi::CString;
use std::future::Future;
use std::io::{BufRead, Write};
use std::os::unix::net::UnixListener;
use std::pin::Pin;
use std::time::Duration;
use std::{fs, io};
use tokio::process::{Child, Command};

static LOCK: LateInit<UnixListener> = LateInit::new();
static PROP_SECTIONS: LateInit<[String; 2]> = LateInit::new();

pub async fn entry() -> Result<()> {
    log::info!("Start zygisksu watchdog");
    check_permission()?;
    ensure_single_instance()?;
    mount_prop().await?;
    if check_and_set_hint()? == false {
        log::warn!("Requirements not met, exiting");
        utils::set_property(constants::PROP_NATIVE_BRIDGE, &utils::get_native_bridge())?;
        return Ok(());
    }
    let end = spawn_daemon().await;
    set_prop_hint(constants::STATUS_CRASHED)?;
    end
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
    let name = format!("zygiskwd{}", magic::MAGIC.as_str());
    match utils::abstract_namespace_socket(&name) {
        Ok(socket) => LOCK.init(socket),
        Err(e) => bail!("Failed to acquire lock: {e}. Maybe another instance is running?"),
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
        format!(
            "{magisk_path}/.magisk/modules/{dir}/{}",
            constants::PATH_MODULE_PROP
        )
    } else {
        constants::PATH_MODULE_PROP.to_string()
    };
    log::info!("Mount {module_prop}");
    let module_prop_file = fs::File::open(&module_prop)?;
    let mut section = 0;
    let mut sections: [String; 2] = [String::new(), String::new()];
    let lines = io::BufReader::new(module_prop_file).lines();
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

    fs::create_dir(magic::PATH_TMP_DIR.as_str())?;
    fs::File::create(magic::PATH_TMP_PROP.as_str())?;

    // FIXME: sys_mount cannot be compiled on 32 bit
    unsafe {
        let r = libc::mount(
            CString::new(magic::PATH_TMP_PROP.as_str())?.as_ptr(),
            CString::new(module_prop)?.as_ptr(),
            std::ptr::null(),
            libc::MS_BIND,
            std::ptr::null(),
        );
        Errno::result(r)?;
    }

    Ok(())
}

fn set_prop_hint(hint: &str) -> Result<()> {
    let mut file = fs::File::create(magic::PATH_TMP_PROP.as_str())?;
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
        let mut futures = FuturesUnordered::<Pin<Box<dyn Future<Output = Result<()>>>>>::new();
        let mut child_ids = vec![];

        let daemon32 = Command::new(constants::PATH_ZYGISKD32)
            .arg("daemon")
            .spawn();
        let daemon64 = Command::new(constants::PATH_ZYGISKD64)
            .arg("daemon")
            .spawn();
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

            log::info!("System server ready, restore native bridge");
            utils::set_property(constants::PROP_NATIVE_BRIDGE, &utils::get_native_bridge())?;

            loop {
                if binder.ping_binder().is_err() {
                    break;
                }
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
            let _ = kill(Pid::from_raw(child as i32), Signal::SIGKILL);
        }

        lives -= 1;
        if lives == 0 {
            bail!("Too many crashes, abort");
        }

        log::error!("Restarting zygote...");
        utils::set_property(constants::PROP_NATIVE_BRIDGE, constants::ZYGISK_LOADER)?;
        utils::set_property(constants::PROP_CTL_RESTART, "zygote")?;
    }
}
