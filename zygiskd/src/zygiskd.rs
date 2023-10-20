use std::ffi::c_void;
use crate::constants::{DaemonSocketAction, ProcessFlags};
use crate::utils::UnixStreamExt;
use crate::{constants, dl, root_impl, utils};
use anyhow::Result;
use passfd::FdPassingExt;
use std::sync::Arc;
use std::thread;
use std::fs;
use std::os::fd::OwnedFd;
use std::os::unix::{
    net::{UnixListener, UnixStream},
    prelude::AsRawFd,
};
use std::path::PathBuf;
use rustix::fs::fstat;
use rustix::process::{set_parent_process_death_signal, Signal};

type ZygiskCompanionEntryFn = unsafe extern "C" fn(i32);

struct Module {
    name: String,
    lib_fd: OwnedFd,
    entry: Option<ZygiskCompanionEntryFn>,
}

struct Context {
    modules: Vec<Module>,
}

pub fn main() -> Result<()> {
    set_parent_process_death_signal(Some(Signal::Kill))?;

    let arch = utils::get_property("ro.product.cpu.abi")?;
    log::debug!("Daemon architecture: {arch}");

    log::info!("Load modules");
    let modules = load_modules(&arch)?;

    let context = Context {
        modules,
    };
    let context = Arc::new(context);

    log::info!("Create socket");
    let listener = create_daemon_socket()?;

    log::info!("Handle zygote connections");
    for stream in listener.incoming() {
        let stream = stream?;
        let context = Arc::clone(&context);
        thread::spawn(move || {
            if let Err(e) = handle_daemon_action(stream, &context) {
                log::warn!("Error handling daemon action: {}\n{}", e, e.backtrace());
            }
        });
    }

    Ok(())
}

fn load_modules(arch: &str) -> Result<Vec<Module>> {
    let mut modules = Vec::new();
    let dir = match fs::read_dir(constants::PATH_MODULES_DIR) {
        Ok(dir) => dir,
        Err(e) => {
            log::warn!("Failed reading modules directory: {}", e);
            return Ok(modules);
        }
    };
    for entry in dir.into_iter() {
        let entry = entry?;
        let name = entry.file_name().into_string().unwrap();
        let so_path = entry.path().join(format!("zygisk/{arch}.so"));
        let disabled = entry.path().join("disable");
        if !so_path.exists() || disabled.exists() {
            continue;
        }
        log::info!("  Loading module `{name}`...");
        let lib_fd = match create_library_fd(&so_path) {
            Ok(fd) => fd,
            Err(e) => {
                log::warn!("  Failed to create memfd for `{name}`: {e}");
                continue;
            }
        };
        let entry = resolve_module(&so_path.to_string_lossy())?;
        let module = Module { name, lib_fd, entry };
        modules.push(module);
    }

    Ok(modules)
}

#[cfg(debug_assertions)]
fn create_library_fd(so_path: &PathBuf) -> Result<OwnedFd> {
    Ok(OwnedFd::from(fs::File::open(so_path)?))
}

#[cfg(not(debug_assertions))]
fn create_library_fd(so_path: &PathBuf) -> Result<OwnedFd> {
    let opts = memfd::MemfdOptions::default().allow_sealing(true);
    let memfd = opts.create("jit-cache")?;
    let file = fs::File::open(so_path)?;
    let mut reader = std::io::BufReader::new(file);
    let mut writer = memfd.as_file();
    std::io::copy(&mut reader, &mut writer)?;

    let mut seals = memfd::SealsHashSet::new();
    seals.insert(memfd::FileSeal::SealShrink);
    seals.insert(memfd::FileSeal::SealGrow);
    seals.insert(memfd::FileSeal::SealWrite);
    seals.insert(memfd::FileSeal::SealSeal);
    memfd.add_seals(&seals)?;

    Ok(OwnedFd::from(memfd.into_file()))
}

fn create_daemon_socket() -> Result<UnixListener> {
    utils::set_socket_create_context("u:r:zygote:s0")?;
    log::debug!("Daemon socket: {}", constants::PATH_CP_SOCKET);
    let listener = utils::unix_listener_from_path(constants::PATH_CP_SOCKET)?;
    Ok(listener)
}

fn resolve_module(path: &str) -> Result<Option<ZygiskCompanionEntryFn>> {
    unsafe {
        let handle = dl::dlopen(path, libc::RTLD_NOW)?;
        let symbol = std::ffi::CString::new("zygisk_companion_entry")?;
        let entry = libc::dlsym(handle, symbol.as_ptr());
        if entry.is_null() {
            return Ok(None);
        }
        let fnptr = std::mem::transmute::<*mut c_void, ZygiskCompanionEntryFn>(entry);
        Ok(Some(fnptr))
    }
}

fn handle_daemon_action(mut stream: UnixStream, context: &Context) -> Result<()> {
    let action = stream.read_u8()?;
    let action = DaemonSocketAction::try_from(action)?;
    log::trace!("New daemon action {:?}", action);
    match action {
        DaemonSocketAction::PingHeartbeat => {
            // Do nothing
        }
        DaemonSocketAction::RequestLogcatFd => {
            loop {
                let level = match stream.read_u8() {
                    Ok(level) => level,
                    Err(_) => break,
                };
                let tag = stream.read_string()?;
                let message = stream.read_string()?;
                utils::log_raw(level as i32, &tag, &message)?;
            }
        }
        DaemonSocketAction::GetProcessFlags => {
            let uid = stream.read_u32()? as i32;
            let mut flags = ProcessFlags::empty();
            if root_impl::uid_granted_root(uid) {
                flags |= ProcessFlags::PROCESS_GRANTED_ROOT;
            }
            if root_impl::uid_should_umount(uid) {
                flags |= ProcessFlags::PROCESS_ON_DENYLIST;
            }
            match root_impl::get_impl() {
                root_impl::RootImpl::KernelSU => flags |= ProcessFlags::PROCESS_ROOT_IS_KSU,
                root_impl::RootImpl::Magisk => flags |= ProcessFlags::PROCESS_ROOT_IS_MAGISK,
                _ => unreachable!(),
            }
            log::trace!("Uid {} granted root: {}", uid, flags.contains(ProcessFlags::PROCESS_GRANTED_ROOT));
            log::trace!("Uid {} on denylist: {}", uid, flags.contains(ProcessFlags::PROCESS_ON_DENYLIST));
            stream.write_u32(flags.bits())?;
        }
        DaemonSocketAction::ReadModules => {
            stream.write_usize(context.modules.len())?;
            for module in context.modules.iter() {
                stream.write_string(&module.name)?;
                stream.send_fd(module.lib_fd.as_raw_fd())?;
            }
        }
        DaemonSocketAction::RequestCompanionSocket => {
            let index = stream.read_usize()?;
            let module = &context.modules[index];
            match module.entry {
                None => {
                    stream.write_u8(0)?;
                    return Ok(());
                }
                Some(companion) => {
                    stream.write_u8(1)?;
                    let st0 = fstat(&stream)?;
                    unsafe { companion(stream.as_raw_fd()); }
                    // Only close client if it is the same file so we don't
                    // accidentally close a re-used file descriptor.
                    // This check is required because the module companion
                    // handler could've closed the file descriptor already.
                    if let Ok(st1) = fstat(&stream) {
                        if st0.st_dev != st1.st_dev || st0.st_ino != st1.st_ino {
                            std::mem::forget(stream);
                        }
                    }
                }
            }
        }
        DaemonSocketAction::GetModuleDir => {
            let index = stream.read_usize()?;
            let module = &context.modules[index];
            let dir = format!("{}/{}", constants::PATH_MODULES_DIR, module.name);
            let dir = fs::File::open(dir)?;
            stream.send_fd(dir.as_raw_fd())?;
        }
    }
    Ok(())
}
