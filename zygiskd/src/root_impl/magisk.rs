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

pub fn uid_on_allowlist(uid: i32) -> bool {
    // TODO: uid_on_allowlist
    return false;
}

pub fn uid_on_denylist(uid: i32) -> bool {
    // TODO: uid_on_denylist
    return false;
}
