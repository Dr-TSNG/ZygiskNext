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
        .arg(std::env::var("SUPERKEY").unwrap())
        .arg("sumgr")
        .arg("list")
        .stdout(Stdio::piped())
        .spawn().ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| {
            let output_string = String::from_utf8(output.stdout).ok();
            if let Some(output_string) = output_string {
                if !output_string.is_empty() {
                    for line in output_string.split('\n') {
                        if let Ok(parsed_uid) = line.trim().parse::<i32>() {
                            if parsed_uid == uid {
                                return Option::from(true);
                            }
                        }
                    }
                }
            }
            Option::from(false)
        })
        .unwrap_or(false)
}

pub fn uid_should_umount(uid: i32) -> bool {
    let output = Command::new("kpatch")
        .arg("$SUPERKEY")
        .arg("sumgr")
        .arg("list")
        .stdout(Stdio::piped())
        .spawn().ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok());

    if let Some(lines) = output {
        for line in lines.split("\n") {
            if line.contains(&uid.to_string()) {
                return false; // UID found, return false
            }
        }
    }

    true // UID not found in the output, return true
}