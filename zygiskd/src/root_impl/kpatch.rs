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

pub fn uid_should_umount(uid: i32) -> bool {
    let output = Command::new("kpatch")
        .arg("sumgr")
        .arg("list")
        .stdout(Stdio::piped())
        .spawn().ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok());

    let lines = match output {
        Some(lines) => lines.split("\n").collect(),
        None => return false,
    };

    for line in lines {
        let parts = line.to_string().split(':').collect::<Vec<&str>>();
        if parts.len() == 3 && parts[0] == &uid.to_string() {
            return false;
        }
    }
    return true;
}