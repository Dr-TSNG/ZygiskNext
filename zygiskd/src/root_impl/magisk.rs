use std::process::{Command, Stdio};
use crate::constants::MIN_MAGISK_VERSION;

pub enum Version {
    Supported,
    TooOld,
}

pub fn get_magisk() -> Option<Version> {
    let version: Option<i32> = Command::new("magisk")
        .arg("-V")
        .stdout(Stdio::piped())
        .spawn().ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .and_then(|output| output.trim().parse().ok());
    version.map(|version| {
        if version >= MIN_MAGISK_VERSION {
            Version::Supported
        } else {
            Version::TooOld
        }
    })
}

pub fn uid_granted_root(uid: i32) -> bool {
    Command::new("magisk")
        .arg("--sqlite")
        .arg(format!("select 1 from policies where uid={uid} and policy=2 limit 1"))
        .stdout(Stdio::piped())
        .spawn().ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|output| output.is_empty()) == Some(false)
}

pub fn uid_should_umount(uid: i32) -> bool {
    let output = Command::new("pm")
        .args(["list", "packages", "--uid", &uid.to_string()])
        .stdout(Stdio::piped())
        .spawn().ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok());
    let line = match output {
        Some(line) => line,
        None => return false,
    };
    let pkg = line
        .strip_prefix("package:")
        .and_then(|line| line.split(' ').next());
    let pkg = match pkg {
        Some(pkg) => pkg,
        None => return false,
    };
    Command::new("magisk")
        .arg("--sqlite")
        .arg(format!("select 1 from denylist where package_name=\"{pkg}\" limit 1"))
        .stdout(Stdio::piped())
        .spawn().ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|output| output.is_empty()) == Some(false)
}

// TODO: signature
// TODO: magisk random package name
pub fn uid_is_manager(uid: i32) -> bool {
    let output = Command::new("magisk")
        .arg("--sqlite")
        .arg(format!("select value from strings where key=\"requester\" limit 1"))
        .stdout(Stdio::piped())
        .spawn().ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|output| output.trim().to_string());
    if let Some(output) = output {
        if let Some(manager) = output.strip_prefix("value=") {
            if let Ok(s) = rustix::fs::stat(format!("/data/user_de/0/{}", manager)) {
                return s.st_uid == uid as u32;
            } else {
                return false;
            }
        }
    }
    if let Ok(s) = rustix::fs::stat("/data/user_de/0/com.topjohnwu.magisk") {
        if s.st_uid == uid as u32 {
            return true;
        }
    }
    if let Ok(s) = rustix::fs::stat("/data/user_de/0/io.github.vvb2060.magisk") {
        if s.st_uid == uid as u32 {
            return true;
        }
    }
    false
}
