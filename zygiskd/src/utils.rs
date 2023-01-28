use crate::constants;
use anyhow::Result;
use nix::unistd::gettid;
use std::{fs, io::{Read, Write}, os::unix::net::UnixStream, process::Command};

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
    std::env::var("NATIVE_BRIDGE").unwrap_or("0".to_string())
}

pub fn restore_native_bridge() -> Result<()> {
    let exec = format!("resetprop {} {}", constants::PROP_NATIVE_BRIDGE, get_native_bridge());
    Command::new(exec).spawn()?.wait()?;
    Ok(())
}

pub trait UnixStreamExt {
    fn read_u8(&mut self) -> Result<u8>;
    fn read_u32(&mut self) -> Result<u32>;
    fn read_usize(&mut self) -> Result<usize>;
    fn write_usize(&mut self, value: usize) -> Result<()>;
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
        Ok(u32::from_le_bytes(buf))
    }

    fn read_usize(&mut self) -> Result<usize> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(usize::from_le_bytes(buf))
    }

    fn write_usize(&mut self, value: usize) -> Result<()> {
        self.write_all(&value.to_ne_bytes())?;
        Ok(())
    }
}
