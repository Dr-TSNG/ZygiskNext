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
    let output: Option<String> = Command::new("magisk")
        .arg("--sqlite")
        .arg("select uid from policies where policy=2")
        .stdout(Stdio::piped())
        .spawn().ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok());
    let lines = match &output {
        Some(output) => output.lines(),
        None => return false,
    };
    lines.into_iter().any(|line| {
        line.trim().strip_prefix("uid=").and_then(|uid| uid.parse().ok()) == Some(uid)
    })
}

pub fn uid_on_denylist(uid: i32) -> bool {
    // TODO: uid_on_denylist
    return false;
}
