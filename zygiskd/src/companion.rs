use std::ffi::c_void;
use std::os::fd::{AsRawFd, FromRawFd, RawFd};
use std::os::unix::net::UnixStream;
use std::thread;
use anyhow::Result;
use passfd::FdPassingExt;
use rustix::fs::fstat;
use tokio::io::AsyncWriteExt;
use crate::utils::{check_unix_socket, UnixStreamExt};
use crate::dl;

type ZygiskCompanionEntryFn = unsafe extern "C" fn(i32);

pub fn entry(fd: i32) {
    log::info!("companion entry fd={}", fd);
    let mut stream = unsafe { UnixStream::from_raw_fd(fd) };
    let name = stream.read_string().expect("read name");
    let library = stream.recv_fd().expect("receive library fd");
    let entry = load_module(library).expect("load module");
    unsafe { libc::close(library) };

    let entry = match entry {
        Some(entry) => {
            log::debug!("Companion process created for `{name}`");
            stream.write_u8(1).expect("reply 1");
            entry
        }
        None => {
            log::debug!("No companion entry for `{name}`");
            stream.write_u8(0).expect("reply 0");
            return ();
        }
    };

    loop {
        if !check_unix_socket(&stream, true) {
            log::info!("Something bad happened in zygiskd, terminate companion");
            std::process::exit(0);
        }
        let fd = stream.recv_fd().expect("recv fd");
        log::trace!("New companion request from module `{name}` fd=`{fd}`");
        let mut stream = unsafe { UnixStream::from_raw_fd(fd) };
        stream.write_u8(1).expect("reply success");
        thread::spawn(move || {
            let st0 = fstat(&stream).expect("failed to stat stream");
            unsafe { entry(stream.as_raw_fd()); }
            // Only close client if it is the same file so we don't
            // accidentally close a re-used file descriptor.
            // This check is required because the module companion
            // handler could've closed the file descriptor already.
            if let Ok(st1) = fstat(&stream) {
                if st0.st_dev != st1.st_dev || st0.st_ino != st1.st_ino {
                    std::mem::forget(stream);
                }
            }
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
