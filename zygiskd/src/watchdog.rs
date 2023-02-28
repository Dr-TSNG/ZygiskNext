use crate::{constants, root_impl, utils};
use anyhow::{bail, Result};
use nix::unistd::{getgid, getuid, Pid};
use std::process::{Child, Command};
use std::sync::mpsc;
use std::{fs, io, thread};
use std::ffi::CString;
use std::io::{BufRead, Write};
use std::os::unix::net::UnixListener;
use std::time::Duration;
use binder::IBinder;
use nix::errno::Errno;
use nix::libc;
use nix::sys::signal::{kill, Signal};
use once_cell::sync::OnceCell;

static LOCK: OnceCell<UnixListener> = OnceCell::new();
static PROP_SECTIONS: OnceCell<[String; 2]> = OnceCell::new();

pub fn entry() -> Result<()> {
    log::info!("Start zygisksu watchdog");
    check_permission()?;
    ensure_single_instance()?;
    mount_prop()?;
    if check_and_set_hint()? == false {
        log::warn!("Requirements not met, exiting");
        utils::set_property(constants::PROP_NATIVE_BRIDGE, &utils::get_native_bridge())?;
        return Ok(());
    }
    let end = spawn_daemon();
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
    let name = String::from("zygiskwd") + constants::SOCKET_PLACEHOLDER;
    match utils::abstract_namespace_socket(&name) {
        Ok(socket) => { let _ = LOCK.set(socket); }
        Err(e) => bail!("Failed to acquire lock: {e}. Maybe another instance is running?")
    }
    Ok(())
}

fn mount_prop() -> Result<()> {
    let module_prop = if let root_impl::RootImpl::Magisk = root_impl::get_impl() {
        let magisk_path = Command::new("magisk").arg("--path").output()?;
        let mut magisk_path = String::from_utf8(magisk_path.stdout)?;
        magisk_path.pop(); // Removing '\n'
        let cwd = std::env::current_dir()?;
        let dir = cwd.file_name().unwrap().to_string_lossy();
        format!("{magisk_path}/.magisk/modules/{dir}/{}", constants::PATH_MODULE_PROP)
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
    let _ = PROP_SECTIONS.set(sections);

    fs::create_dir(constants::PATH_TMP_DIR)?;
    fs::File::create(constants::PATH_TMP_PROP)?;

    // FIXME: sys_mount cannot be compiled on 32 bit
    unsafe {
        let r = libc::mount(
            CString::new(constants::PATH_TMP_PROP)?.as_ptr(),
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
    let mut file = fs::File::create(constants::PATH_TMP_PROP)?;
    let sections = PROP_SECTIONS.get().unwrap();
    file.write_all(sections[0].as_bytes())?;
    file.write_all(b"[")?;
    file.write_all(hint.as_bytes())?;
    file.write_all(b"] ")?;
    file.write_all(sections[1].as_bytes())?;
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
        utils::set_property(constants::PROP_CTL_RESTART, "zygote")?;
    }
}
