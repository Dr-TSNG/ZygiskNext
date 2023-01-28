use crate::constants::DaemonSocketAction;
use crate::utils::{restore_native_bridge, UnixStreamExt};
use crate::{constants, utils};
use anyhow::{anyhow, Result};
use memfd::Memfd;
use nix::{
    libc::{self, dlsym},
    unistd::getppid,
};
use passfd::FdPassingExt;
use std::cell::Cell;
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::{
    ffi::c_void,
    fs,
    os::unix::{
        net::{UnixListener, UnixStream},
        prelude::AsRawFd,
    },
    path::PathBuf,
};

type ZygiskCompanionEntryFn = unsafe extern "C" fn(i32);

struct Module {
    name: String,
    memfd: Memfd,
    companion_entry: Option<ZygiskCompanionEntryFn>,
}

struct Context {
    native_bridge: String,
    modules: Arc<Vec<Module>>,
    stream: Cell<UnixStream>,
}

pub fn start(is64: bool) -> Result<()> {
    // check_parent()?;
    let arch = get_arch(is64)?;
    log::debug!("Daemon architecture: {arch}");

    log::info!("Load modules");
    let modules = load_modules(arch)?;

    log::info!("Create socket");
    let listener = create_daemon_socket(is64)?;

    log::info!("Waiting for connection");
    let (stream, _) = listener.accept()?;
    drop(listener);

    let context = Context {
        native_bridge: utils::get_native_bridge(),
        modules: Arc::new(modules),
        stream: Cell::new(stream),
    };

    log::info!("Connection established");
    restore_native_bridge()?;
    handle_daemon_actions(context)?;

    Ok(())
}

fn check_parent() -> Result<()> {
    let parent = fs::read_link(format!("/proc/{}/exe", getppid().as_raw()))?;
    let parent = parent.file_name().unwrap().to_str().unwrap();
    if parent != constants::ZYGISKWD {
        return Err(anyhow!("zygiskd is not started by watchdog"));
    }
    Ok(())
}

fn get_arch(is64: bool) -> Result<&'static str> {
    // let output = Command::new("getprop ro.product.cpu.abi").output()?;
    // let system_arch = String::from_utf8(output.stdout)?;
    let system_arch = "x86_64"; // DEBUGGING
    let is_arm = system_arch.contains("arm");
    let is_x86 = system_arch.contains("x86");
    if is64 {
        match (is_arm, is_x86) {
            (true, _) => Ok("arm64-v8a"),
            (_, true) => Ok("x86_64"),
            _ => Err(anyhow!("Unsupported system architecture: {}", system_arch)),
        }
    } else {
        match (is_arm, is_x86) {
            (true, _) => Ok("armeabi-v7a"),
            (_, true) => Ok("x86"),
            _ => Err(anyhow!("Unsupported system architecture: {}", system_arch)),
        }
    }
}

fn load_modules(arch: &str) -> Result<Vec<Module>> {
    let mut modules = Vec::new();
    let dir = match fs::read_dir(constants::KSU_MODULE_DIR) {
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
            Ok(companion_entry) => companion_entry,
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

fn create_memfd(name: &str, so_path: &PathBuf) -> Result<memfd::Memfd> {
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
            return Err(anyhow!("dlopen failed: {}", e));
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
    let socket_name = if is64 { "zygiskd64" } else { "zygiskd32" };
    let listener = UnixListener::bind(socket_name)?;
    Ok(listener)
}

fn handle_daemon_actions(mut context: Context) -> Result<()> {
    let stream = context.stream.get_mut();
    loop {
        let action = stream.read_u8()?;
        match DaemonSocketAction::try_from(action) {
            Ok(DaemonSocketAction::ReadNativeBridge) => {
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
            Err(_) => {
                return Err(anyhow!("Invalid action code: {action}"));
            }
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

        unsafe {
            match module.companion_entry {
                Some(entry) => {
                    let (sock_app, sock_companion) = UnixStream::pair()?;
                    server.send_fd(sock_app.as_raw_fd())?;
                    entry(sock_companion.as_raw_fd());
                }
                None => (),
            }
        }
    }
}
