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
