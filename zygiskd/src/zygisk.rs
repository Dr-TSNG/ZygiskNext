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
use std::os::fd::FromRawFd;
use std::os::unix::{
    net::{UnixListener, UnixStream},
    prelude::AsRawFd,
};
use std::path::PathBuf;
use std::process::Command;
use nix::sys::socket::{AddressFamily, SockFlag, SockType, UnixAddr};

type ZygiskCompanionEntryFn = unsafe extern "C" fn(i32);

struct Module {
    name: String,
    memfd: Memfd,
    companion_entry: Option<ZygiskCompanionEntryFn>,
}

struct Context {
    native_bridge: String,
    modules: Arc<Vec<Module>>,
    listener: UnixListener,
}

pub fn start(is64: bool) -> Result<()> {
    check_parent()?;
    let arch = get_arch(is64)?;
    log::debug!("Daemon architecture: {arch}");

    log::info!("Load modules");
    let modules = load_modules(arch)?;

    log::info!("Create socket");
    let listener = create_daemon_socket(is64)?;

    let context = Context {
        native_bridge: utils::get_native_bridge(),
        modules: Arc::new(modules),
        listener,
    };

    log::info!("Start to listen zygote connections");
    handle_daemon_actions(context)?;

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
    let name = if is64 { "zygiskd64" } else { "zygiskd32" };
    // TODO: Replace with SockAddrExt::from_abstract_name when it's stable
    let addr = UnixAddr::new_abstract(name.as_bytes())?;
    let socket = nix::sys::socket::socket(AddressFamily::Unix, SockType::Stream, SockFlag::empty(), None)?;
    nix::sys::socket::bind(socket, &addr)?;
    log::debug!("Listening on {}", addr);
    log::debug!("Socket fd: {}", socket);
    let listener = unsafe { UnixListener::from_raw_fd(socket) };
    Ok(listener)
}

fn handle_daemon_actions(context: Context) -> Result<()> {
    loop {
        let (mut stream, _) = context.listener.accept()?;
        let action = stream.read_u8()?;
        match DaemonSocketAction::try_from(action) {
            // First connection from zygote
            Ok(DaemonSocketAction::ReadNativeBridge) => {
                restore_native_bridge()?;
                stream.write_usize(context.native_bridge.len())?;
                stream.write_all(context.native_bridge.as_bytes())?;
            }
            Ok(DaemonSocketAction::ReadModules) => {
                stream.write_usize(context.modules.len())?;
                for module in context.modules.iter() {
                    stream.write_usize(module.name.len())?;
                    stream.write_all(module.name.as_bytes())?;
                    stream.send_fd(module.memfd.as_raw_fd())?;
                }
            }
            Ok(DaemonSocketAction::RequestCompanionSocket) => {
                let (server, client) = UnixStream::pair()?;
                stream.send_fd(client.as_raw_fd())?;
                let modules_ref = Arc::clone(&context.modules);
                thread::spawn(move || {
                    if let Err(e) = create_companion(server, modules_ref.as_ref()) {
                        log::warn!("Companion thread exited: {e}");
                    }
                });
            }
            Err(_) => bail!("Invalid action code: {action}")
        }
    }
}

fn create_companion(mut server: UnixStream, modules: &Vec<Module>) -> Result<()> {
    loop {
        let index = match server.read_usize() {
            Ok(index) => index,
            Err(_) => return Ok(()), // EOF
        };
        let module = &modules[index];
        log::debug!("New companion request from module {}", module.name);

        match module.companion_entry {
            Some(entry) => {
                let (sock_app, sock_companion) = UnixStream::pair()?;
                server.send_fd(sock_app.as_raw_fd())?;
                unsafe { entry(sock_companion.as_raw_fd()); }
            }
            None => (),
        }
    }
}
