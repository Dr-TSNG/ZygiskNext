use crate::constants::DaemonSocketAction;
use crate::utils::UnixStreamExt;
use crate::{constants, lp_select, root_impl, utils};
use anyhow::{bail, Result};
use memfd::Memfd;
use nix::{
    fcntl::{fcntl, FcntlArg, FdFlag},
    libc::self,
};
use passfd::FdPassingExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::c_char;
use std::fs;
use std::os::unix::{
    net::{UnixListener, UnixStream},
    prelude::AsRawFd,
};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

struct Module {
    name: String,
    memfd: Memfd,
    companion: Mutex<Option<UnixStream>>,
}

struct Context {
    native_bridge: String,
    modules: Vec<Module>,
}

pub fn entry() -> Result<()> {
    unsafe { libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) };

    let arch = get_arch()?;
    log::debug!("Daemon architecture: {arch}");

    log::info!("Load modules");
    let modules = load_modules(arch)?;

    let context = Context {
        native_bridge: utils::get_native_bridge(),
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

fn get_arch() -> Result<&'static str> {
    let output = Command::new("getprop").arg("ro.product.cpu.abi").output()?;
    let system_arch = String::from_utf8(output.stdout)?;
    if system_arch.contains("arm") {
        return Ok(lp_select!("armeabi-v7a", "arm64-v8a"));
    }
    if system_arch.contains("x86") {
        return Ok(lp_select!("x86", "x86_64"));
    }
    bail!("Unsupported system architecture: {}", system_arch);
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
    for entry_result in dir.into_iter() {
        let entry = entry_result?;
        let name = entry.file_name().into_string().unwrap();
        let so_path = entry.path().join(format!("zygisk/{arch}.so"));
        let disabled = entry.path().join("disable");
        if !so_path.exists() || disabled.exists() {
            continue;
        }
        log::info!("  Loading module `{name}`...");
        let memfd = match create_memfd(&so_path) {
            Ok(memfd) => memfd,
            Err(e) => {
                log::warn!("  Failed to create memfd for `{name}`: {e}");
                continue;
            }
        };
        let companion = match spawn_companion(&name, &memfd) {
            Ok(companion) => companion,
            Err(e) => {
                log::warn!("  Failed to spawn companion for `{name}`: {e}");
                continue;
            }
        };

        let companion = Mutex::new(companion);
        let module = Module { name, memfd, companion };
        modules.push(module);
    }

    Ok(modules)
}

fn create_memfd(so_path: &PathBuf) -> Result<Memfd> {
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

    Ok(memfd)
}

fn create_daemon_socket() -> Result<UnixListener> {
    utils::set_socket_create_context("u:r:zygote:s0")?;
    let prefix = lp_select!("zygiskd32", "zygiskd64");
    let name = String::from(prefix) + constants::SOCKET_PLACEHOLDER;
    let listener = utils::abstract_namespace_socket(&name)?;
    log::debug!("Daemon socket: {name}");
    Ok(listener)
}

fn spawn_companion(name: &str, memfd: &Memfd) -> Result<Option<UnixStream>> {
    let (mut daemon, companion) = UnixStream::pair()?;
    // Remove FD_CLOEXEC flag
    fcntl(companion.as_raw_fd(), FcntlArg::F_SETFD(FdFlag::empty()))?;

    let process = std::env::args().next().unwrap();
    let nice_name = process.split('/').last().unwrap();
    Command::new(&process)
        .arg0(format!("{}-{}", nice_name, name))
        .arg("companion")
        .arg(format!("{}", companion.as_raw_fd()))
        .spawn()?;
    drop(companion);

    daemon.write_string(name)?;
    daemon.send_fd(memfd.as_raw_fd())?;
    match daemon.read_u8()? {
        0 => Ok(None),
        1 => Ok(Some(daemon)),
        _ => bail!("Invalid companion response"),
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
                let tag = std::ffi::CString::new(tag)?;
                let message = stream.read_string()?;
                let message = std::ffi::CString::new(message)?;
                unsafe {
                    __android_log_print(level as i32, tag.as_ptr(), message.as_ptr());
                }
            }
        }
        DaemonSocketAction::ReadNativeBridge => {
            stream.write_string(&context.native_bridge)?;
        }
        DaemonSocketAction::GetProcessFlags => {
            let uid = stream.read_u32()? as i32;
            let mut flags = 0u32;
            if root_impl::uid_on_allowlist(uid) {
                flags |= constants::PROCESS_GRANTED_ROOT;
            }
            if root_impl::uid_on_denylist(uid) {
                flags |= constants::PROCESS_ON_DENYLIST;
            }
            // TODO: PROCESS_IS_SYSUI?
            stream.write_u32(flags)?;
        }
        DaemonSocketAction::ReadModules => {
            stream.write_usize(context.modules.len())?;
            for module in context.modules.iter() {
                stream.write_string(&module.name)?;
                stream.send_fd(module.memfd.as_raw_fd())?;
            }
        }
        DaemonSocketAction::RequestCompanionSocket => {
            let index = stream.read_usize()?;
            let module = &context.modules[index];
            let mut companion = module.companion.lock().unwrap();
            match companion.as_ref() {
                Some(sock) => {
                    if let Err(_) = sock.send_fd(stream.as_raw_fd()) {
                        log::error!("Companion of module `{}` crashed", module.name);
                        companion.take();
                        stream.write_u8(0)?;
                    }
                    // Ok: Send by companion
                }
                None => {
                    stream.write_u8(0)?;
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

extern "C" {
    fn __android_log_print(prio: i32, tag: *const c_char, fmt: *const c_char, ...) -> i32;
}
