use std::process::{Command, Stdio};
use crate::constants::KPATCH_VER_CODE;

pub enum Version {
    Supported,
    TooOld,
}

pub fn get_kpatch() -> Option<crate::root_impl::kpatch::Version> {
    let version: Option<i32> = Command::new("kpatch")
        .arg("-v")
        .stdout(Stdio::piped())
        .spawn().ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .and_then(|output| output.trim().parse().ok());
    version.map(|version| {
        if version >= KPATCH_VER_CODE {
            Version::Supported
        } else {
            Version::TooOld
        }
    })
}

pub fn uid_granted_root(uid: i32) -> bool {
    Command::new("kpatch")
        .arg("$SUPERKEY sumgr list")
        .stdout(Stdio::piped())
        .spawn().ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| {
            let output_string = String::from_utf8(output.stdout).ok();
            if let Some(output_string) = output_string {
                output_string.split(' ').any(|x| x.parse::<i32>().unwrap() == uid).into()
            } else {
                return None;
            }
        })
        .unwrap_or(false)
}