use std::ffi::c_void;
use std::os::fd::{FromRawFd, RawFd};
use std::os::unix::net::UnixStream;
use std::thread;
use anyhow::Result;
use nix::libc;
use passfd::FdPassingExt;
use crate::utils::UnixStreamExt;
use crate::dl;

type ZygiskCompanionEntryFn = unsafe extern "C" fn(i32);

pub fn entry(fd: i32) -> Result<()> {
    unsafe { libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) };
    let mut stream = unsafe { UnixStream::from_raw_fd(fd) };
    let name = stream.read_string()?;
    let library = stream.recv_fd()?;
    let entry = load_module(library)?;
    unsafe { libc::close(library) };

    let entry = match entry {
        Some(entry) => {
            log::debug!("Companion process created for `{name}`");
            stream.write_u8(1)?;
            entry
        }
        None => {
            log::debug!("No companion entry for `{name}`");
            stream.write_u8(0)?;
            return Ok(());
        }
    };

    loop {
        let fd = stream.recv_fd()?;
        log::trace!("New companion request from module `{name}`");
        thread::spawn(move || {
            unsafe {
                let mut s = UnixStream::from_raw_fd(fd);
                match s.write_u8(1) { // Ack
                    Ok(_) => entry(fd),
                    Err(_) => log::warn!("Ack failed?"),
                }
            };
        });
    }
}

fn load_module(fd: RawFd) -> Result<Option<ZygiskCompanionEntryFn>> {
    unsafe {
        let path = format!("/proc/self/fd/{fd}");
        let handle = dl::dlopen(&path, libc::RTLD_NOW)?;
        let symbol = std::ffi::CString::new("zygisk_companion_entry")?;
        let entry = libc::dlsym(handle, symbol.as_ptr());
        if entry.is_null() {
            return Ok(None);
        }
        let fnptr = std::mem::transmute::<*mut c_void, ZygiskCompanionEntryFn>(entry);
        Ok(Some(fnptr))
    }
}
