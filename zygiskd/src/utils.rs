use anyhow::Result;
use std::{fs, io::{Read, Write}, os::unix::net::UnixStream};
use std::ffi::{c_char, c_void, CStr, CString};
use std::os::fd::{AsFd, AsRawFd};
use std::os::unix::net::{UnixDatagram, UnixListener};
use std::process::Command;
use std::sync::OnceLock;
use bitflags::Flags;
use rustix::net::{AddressFamily, bind_unix, connect_unix, listen, SendFlags, sendto_unix, socket, SocketAddrUnix, SocketType};
use rustix::path::Arg;
use rustix::thread::gettid;

#[cfg(target_pointer_width = "64")]
#[macro_export]
macro_rules! lp_select {
    ($lp32:expr, $lp64:expr) => { $lp64 };
}
#[cfg(target_pointer_width = "32")]
#[macro_export]
macro_rules! lp_select {
    ($lp32:expr, $lp64:expr) => { $lp32 };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_select {
    ($debug:expr, $release:expr) => { $debug };
}
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_select {
    ($debug:expr, $release:expr) => { $release };
}

pub struct LateInit<T> {
    cell: OnceLock<T>,
}

impl<T> LateInit<T> {
    pub const fn new() -> Self {
        LateInit { cell: OnceLock::new() }
    }

    pub fn init(&self, value: T) {
        assert!(self.cell.set(value).is_ok())
    }
}

impl<T> std::ops::Deref for LateInit<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.cell.get().unwrap()
    }
}

pub fn set_socket_create_context(context: &str) -> Result<()> {
    let path = "/proc/thread-self/attr/sockcreate";
    match fs::write(path, context) {
        Ok(_) => Ok(()),
        Err(_) => {
            let path = format!("/proc/self/task/{}/attr/sockcreate", gettid().as_raw_nonzero());
            fs::write(path, context)?;
            Ok(())
        }
    }
}

pub fn get_current_attr() -> Result<String> {
    let s = fs::read("/proc/self/attr/current")?;
    Ok(s.to_string_lossy().to_string())
}

pub fn chcon(path: &str, context: &str) -> Result<()> {
    Command::new("chcon").arg(context).arg(path).status()?;
    Ok(())
}

pub fn log_raw(level: i32, tag: &str, message: &str) -> Result<()> {
    let tag = CString::new(tag)?;
    let message = CString::new(message)?;
    unsafe {
        __android_log_print(level, tag.as_ptr(), message.as_ptr());
    }
    Ok(())
}

pub fn get_property(name: &str) -> Result<String> {
    let name = CString::new(name)?;
    let mut buf = vec![0u8; 92];
    let prop = unsafe {
        __system_property_get(name.as_ptr(), buf.as_mut_ptr() as *mut c_char);
        CStr::from_bytes_until_nul(&buf)?
    };
    Ok(prop.to_string_lossy().to_string())
}

pub fn set_property(name: &str, value: &str) -> Result<()> {
    let name = CString::new(name)?;
    let value = CString::new(value)?;
    unsafe {
        __system_property_set(name.as_ptr(), value.as_ptr());
    }
    Ok(())
}

pub fn wait_property(name: &str, serial: u32) -> Result<u32> {
    let name = CString::new(name)?;
    let info = unsafe {
        __system_property_find(name.as_ptr())
    };
    let mut serial = serial;
    unsafe {
        __system_property_wait(info, serial, &mut serial, std::ptr::null());
    }
    Ok(serial)
}

pub fn get_property_serial(name: &str) -> Result<u32> {
    let name = CString::new(name)?;
    let info = unsafe {
        __system_property_find(name.as_ptr())
    };
    Ok(unsafe {
        __system_property_serial(info)
    })
}

pub fn switch_mount_namespace(pid: i32) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let mnt = fs::File::open(format!("/proc/{}/ns/mnt", pid))?;
    rustix::thread::move_into_link_name_space(mnt.as_fd(), None)?;
    std::env::set_current_dir(cwd)?;
    Ok(())
}

pub trait UnixStreamExt {
    fn read_u8(&mut self) -> Result<u8>;
    fn read_u32(&mut self) -> Result<u32>;
    fn read_usize(&mut self) -> Result<usize>;
    fn read_string(&mut self) -> Result<String>;
    fn write_u8(&mut self, value: u8) -> Result<()>;
    fn write_u32(&mut self, value: u32) -> Result<()>;
    fn write_usize(&mut self, value: usize) -> Result<()>;
    fn write_string(&mut self, value: &str) -> Result<()>;
}

impl UnixStreamExt for UnixStream {
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_ne_bytes(buf))
    }

    fn read_usize(&mut self) -> Result<usize> {
        let mut buf = [0u8; std::mem::size_of::<usize>()];
        self.read_exact(&mut buf)?;
        Ok(usize::from_ne_bytes(buf))
    }

    fn read_string(&mut self) -> Result<String> {
        let len = self.read_usize()?;
        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }

    fn write_u8(&mut self, value: u8) -> Result<()> {
        self.write_all(&value.to_ne_bytes())?;
        Ok(())
    }

    fn write_u32(&mut self, value: u32) -> Result<()> {
        self.write_all(&value.to_ne_bytes())?;
        Ok(())
    }

    fn write_usize(&mut self, value: usize) -> Result<()> {
        self.write_all(&value.to_ne_bytes())?;
        Ok(())
    }

    fn write_string(&mut self, value: &str) -> Result<()> {
        self.write_usize(value.len())?;
        self.write_all(value.as_bytes())?;
        Ok(())
    }
}

pub fn unix_listener_from_path(path: &str) -> Result<UnixListener> {
    let _ = fs::remove_file(path);
    let addr = SocketAddrUnix::new(path)?;
    let socket = socket(AddressFamily::UNIX, SocketType::STREAM, None)?;
    bind_unix(&socket, &addr)?;
    listen(&socket, 2)?;
    chcon(path, "u:object_r:magisk_file:s0")?;
    Ok(UnixListener::from(socket))
}

pub fn unix_datagram_sendto_abstract(path: &str, buf: &[u8]) -> Result<()> {
    // FIXME: shall we set create context every time?
    set_socket_create_context(get_current_attr()?.as_str())?;
    let addr = SocketAddrUnix::new_abstract_name(path.as_bytes())?;
    let socket = socket(AddressFamily::UNIX, SocketType::DGRAM, None)?;
    connect_unix(&socket, &addr)?;
    sendto_unix(socket, buf, SendFlags::empty(), &addr)?;
    set_socket_create_context("u:r:zygote:s0")?;
    Ok(())
}

pub fn check_unix_socket(stream: &UnixStream, block: bool) -> bool {
    unsafe {
        let mut pfd = libc::pollfd {
            fd: stream.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };
        let timeout = if block { -1 } else { 0 };
        libc::poll(&mut pfd, 1, timeout);
        if pfd.revents & !libc::POLLIN != 0 {
            return false;
        }
    }
    return true;
}

extern "C" {
    fn __android_log_print(prio: i32, tag: *const c_char, fmt: *const c_char, ...) -> i32;
    fn __system_property_get(name: *const c_char, value: *mut c_char) -> u32;
    fn __system_property_set(name: *const c_char, value: *const c_char) -> u32;
    fn __system_property_find(name: *const c_char) -> *const c_void;
    fn __system_property_wait(info: *const c_void, old_serial: u32, new_serial: *mut u32, timeout: *const libc::timespec) -> bool;
    fn __system_property_serial(info: *const c_void) -> u32;
}
