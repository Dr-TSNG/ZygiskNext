use anyhow::Result;
use nix::unistd::gettid;
use std::{fs, io::{Read, Write}, os::unix::net::UnixStream, process::Command};
use std::os::fd::FromRawFd;
use std::os::unix::net::UnixListener;
use nix::sys::socket::{AddressFamily, SockFlag, SockType, UnixAddr};
use rand::distributions::{Alphanumeric, DistString};

pub fn random_string() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 8)
}

pub fn set_socket_create_context(context: &str) -> Result<()> {
    let path = "/proc/thread-self/attr/sockcreate";
    match fs::write(path, context) {
        Ok(_) => Ok(()),
        Err(_) => {
            let path = format!("/proc/self/task/{}/attr/sockcreate", gettid().as_raw());
            fs::write(path, context)?;
            Ok(())
        }
    }
}

pub fn get_native_bridge() -> String {
    std::env::var("NATIVE_BRIDGE").unwrap_or_default()
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

// TODO: Replace with SockAddrExt::from_abstract_name when it's stable
pub fn abstract_namespace_socket(name: &str) -> Result<UnixListener> {
    let addr = UnixAddr::new_abstract(name.as_bytes())?;
    let socket = nix::sys::socket::socket(AddressFamily::Unix, SockType::Stream, SockFlag::empty(), None)?;
    nix::sys::socket::bind(socket, &addr)?;
    nix::sys::socket::listen(socket, 2)?;
    let listener = unsafe { UnixListener::from_raw_fd(socket) };
    Ok(listener)
}
