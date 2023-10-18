use anyhow::Result;
use std::{fs, io::{Read, Write}, os::unix::net::UnixStream, process::Command};
use std::ffi::c_char;
use std::os::unix::net::UnixListener;
use std::sync::OnceLock;
use rand::distributions::{Alphanumeric, DistString};
use rustix::net::{AddressFamily, bind_unix, listen, socket, SocketAddrUnix, SocketType};
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

pub fn random_string() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 8)
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

pub fn get_native_bridge() -> String {
    std::env::var("NATIVE_BRIDGE").unwrap_or_default()
}

pub fn log_raw(level: i32, tag: &str, message: &str) -> Result<()> {
    let tag = std::ffi::CString::new(tag)?;
    let message = std::ffi::CString::new(message)?;
    unsafe {
        __android_log_print(level as i32, tag.as_ptr(), message.as_ptr());
    }
    Ok(())
}

pub fn get_property(name: &str) -> Result<String> {
    let name = std::ffi::CString::new(name)?;
    let mut buf = vec![0u8; 92];
    unsafe {
        __system_property_get(name.as_ptr(), buf.as_mut_ptr() as *mut c_char);
    }
    Ok(String::from_utf8(buf)?)
}

pub fn set_property(name: &str, value: &str) -> Result<()> {
    Command::new("resetprop")
        .arg(name)
        .arg(value)
        .spawn()?.wait()?;
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

pub fn abstract_namespace_socket(name: &str) -> Result<UnixListener> {
    let addr = SocketAddrUnix::new_abstract_name(name.as_bytes())?;
    let socket = socket(AddressFamily::UNIX, SocketType::STREAM, None)?;
    bind_unix(&socket, &addr)?;
    listen(&socket, 2)?;
    Ok(UnixListener::from(socket))
}

extern "C" {
    fn __android_log_print(prio: i32, tag: *const c_char, fmt: *const c_char, ...) -> i32;
    fn __system_property_get(name: *const c_char, value: *mut c_char) -> u32;
}
