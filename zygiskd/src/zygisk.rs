use crate::constants::DaemonSocketAction;
use crate::utils::{restore_native_bridge, UnixStreamExt};
use crate::{constants, utils};
use anyhow::{bail, Result};
use memfd::Memfd;
use nix::{
    libc::{self, dlsym},
    unistd::getppid,
};
use passfd::FdPassingExt;
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::ffi::c_void;
use std::fs;
use std::os::unix::{
    net::{UnixListener, UnixStream},
    prelude::AsRawFd,
};
use std::path::PathBuf;
use std::process::Command;

type ZygiskCompanionEntryFn = unsafe extern "C" fn(i32);

struct Module {
    name: String,
    memfd: Memfd,
    companion_entry: Option<ZygiskCompanionEntryFn>,
}

struct Context {
    native_bridge: String,
    modules: Vec<Module>,
}

pub fn start(is64: bool) -> Result<()> {
    check_parent()?;
    let arch = get_arch(is64)?;
    log::debug!("Daemon architecture: {arch}");

    log::info!("Load modules");
    let modules = load_modules(arch)?;

    let context = Context {
        native_bridge: utils::get_native_bridge(),
        modules,
    };
    let context = Arc::new(context);

    log::info!("Create socket");
    let listener = create_daemon_socket(is64)?;

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

fn check_parent() -> Result<()> {
    let parent = fs::read_to_string(format!("/proc/{}/cmdline", getppid().as_raw()))?;
    let parent = parent.split('/').last().unwrap().trim_end_matches('\0');
    if parent != "zygiskwd" {
        bail!("Daemon is not started by watchdog: {parent}");
    }
    Ok(())
}

fn get_arch(is64: bool) -> Result<&'static str> {
    let output = Command::new("getprop").arg("ro.product.cpu.abi").output()?;
    let system_arch = String::from_utf8(output.stdout)?;
    let is_arm = system_arch.contains("arm");
    let is_x86 = system_arch.contains("x86");
    match (is_arm, is_x86, is64) {
        (true, _, false) => Ok("armeabi-v7a"),
        (true, _, true) => Ok("arm64-v8a"),
        (_, true, false) => Ok("x86"),
        (_, true, true) => Ok("x86_64"),
        _ => bail!("Unsupported system architecture: {}", system_arch),
    }
}

fn load_modules(arch: &str) -> Result<Vec<Module>> {
    let mut modules = Vec::new();
    let dir = match fs::read_dir(constants::PATH_KSU_MODULE_DIR) {
        Ok(dir) => dir,
        Err(e) => {
            log::warn!("Failed reading modules directory: {}", e);
            return Ok(modules);
        }
    };
    for entry_result in dir.into_iter() {
        let entry = entry_result?;
        let name = entry.file_name().into_string().unwrap();
        let so_path = entry.path().join(format!("zygisk/{arch}.so"));
        if !so_path.exists() {
            continue;
        }
        log::info!("  Loading module `{name}`...");
        let memfd = match create_memfd(&name, &so_path) {
            Ok(memfd) => memfd,
            Err(e) => {
                log::warn!("  Failed to create memfd for `{name}`: {e}");
                continue;
            }
        };
        let companion_entry = match preload_module(&memfd) {
            Ok(entry) => entry,
            Err(e) => {
                log::warn!("  Failed to preload `{name}`: {e}");
                continue;
            }
        };
        let module = Module {
            name,
            memfd,
            companion_entry,
        };
        modules.push(module);
    }

    Ok(modules)
}

fn create_memfd(name: &str, so_path: &PathBuf) -> Result<Memfd> {
    let opts = memfd::MemfdOptions::default().allow_sealing(true);
    let memfd = opts.create(name)?;

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

    Ok(memfd)
}

fn preload_module(memfd: &Memfd) -> Result<Option<ZygiskCompanionEntryFn>> {
    unsafe {
        let path = format!("/proc/self/fd/{}", memfd.as_raw_fd());
        let filename = std::ffi::CString::new(path)?;
        let handle = libc::dlopen(filename.as_ptr(), libc::RTLD_LAZY);
        if handle.is_null() {
            let e = std::ffi::CStr::from_ptr(libc::dlerror())
                .to_string_lossy()
                .into_owned();
            bail!("dlopen failed: {}", e);
        }
        let symbol = std::ffi::CString::new("zygisk_companion_entry")?;
        let entry = dlsym(handle, symbol.as_ptr());
        if entry.is_null() {
            return Ok(None);
        }
        let fnptr = std::mem::transmute::<*mut c_void, ZygiskCompanionEntryFn>(entry);
        Ok(Some(fnptr))
    }
}

fn create_daemon_socket(is64: bool) -> Result<UnixListener> {
    utils::set_socket_create_context("u:r:zygote:s0")?;
    let suffix = if is64 { "zygiskd64" } else { "zygiskd32" };
    let name = String::from(suffix) + constants::SOCKET_PLACEHOLDER;
    let listener = utils::abstract_namespace_socket(&name)?;
    log::debug!("Daemon socket: {name}");
    Ok(listener)
}

fn handle_daemon_action(mut stream: UnixStream, context: &Context) -> Result<()> {
    let action = stream.read_u8()?;
    let action = DaemonSocketAction::try_from(action)?;
    log::debug!("New daemon action {:?}", action);
    match action {
        DaemonSocketAction::PingHeartbeat => {
            restore_native_bridge()?;
        }
        DaemonSocketAction::ReadNativeBridge => {
            stream.write_usize(context.native_bridge.len())?;
            stream.write_all(context.native_bridge.as_bytes())?;
        }
        DaemonSocketAction::ReadModules => {
            stream.write_usize(context.modules.len())?;
            for module in context.modules.iter() {
                stream.write_usize(module.name.len())?;
                stream.write_all(module.name.as_bytes())?;
                stream.send_fd(module.memfd.as_raw_fd())?;
            }
        }
        DaemonSocketAction::RequestCompanionSocket => {
            let index = stream.read_usize()?;
            let module = &context.modules[index];
            log::debug!("New companion request from module {}", module.name);

            match module.companion_entry {
                Some(entry) => {
                    stream.write_u8(1)?;
                    unsafe { entry(stream.as_raw_fd()); }
                }
                None => {
                    stream.write_u8(0)?;
                }
            }
        }
    }
    Ok(())
}
