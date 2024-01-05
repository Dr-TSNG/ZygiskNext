use std::fs;
use std::os::android::fs::MetadataExt;
use crate::constants::MIN_MAGISK_VERSION;
use std::process::{Command, Stdio};
use log::info;
use crate::utils::LateInit;

const MAGISK_OFFICIAL: &str = "com.topjohnwu.magisk";
const MAGISK_THIRD_PARTIES: &[(&str, &str)] = &[
    ("alpha", "io.github.vvb2060.magisk"),
    ("kitsune", "io.github.huskydg.magisk"),
];

pub enum Version {
    Supported,
    TooOld,
}

static VARIANT: LateInit<&str> = LateInit::new();

pub fn get_magisk() -> Option<Version> {
    if !VARIANT.initiated() {
        Command::new("magisk")
            .arg("-v")
            .stdout(Stdio::piped())
            .spawn()
            .ok()
            .and_then(|child| child.wait_with_output().ok())
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|version| {
                let third_party = MAGISK_THIRD_PARTIES.iter().find_map(|v| {
                    version.contains(v.0).then_some(v.1)
                });
                VARIANT.init(third_party.unwrap_or(MAGISK_OFFICIAL));
                info!("Magisk variant: {}", *VARIANT);
            });
    }
    Command::new("magisk")
        .arg("-V")
        .stdout(Stdio::piped())
        .spawn()
        .ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .and_then(|output| output.trim().parse::<i32>().ok())
        .map(|version| {
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
        .arg(format!(
            "select 1 from policies where uid={uid} and policy=2 limit 1"
        ))
        .stdout(Stdio::piped())
        .spawn()
        .ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|output| output.is_empty())
        == Some(false)
}

pub fn uid_should_umount(uid: i32) -> bool {
    let output = Command::new("pm")
        .args(["list", "packages", "--uid", &uid.to_string()])
        .stdout(Stdio::piped())
        .spawn()
        .ok()
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
        .arg(format!(
            "select 1 from denylist where package_name=\"{pkg}\" limit 1"
        ))
        .stdout(Stdio::piped())
        .spawn()
        .ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|output| output.is_empty())
        == Some(false)
}

// TODO: signature
pub fn uid_is_manager(uid: i32) -> bool {
    let output = Command::new("magisk")
        .arg("--sqlite")
        .arg(format!("select value from strings where key=\"requester\" limit 1"))
        .stdout(Stdio::piped())
        .spawn()
        .ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|output| output.trim().to_string());
    if let Some(output) = output {
        if let Some(manager) = output.strip_prefix("value=") {
            return fs::metadata(format!("/data/user_de/0/{}", manager))
                .map(|s| s.st_uid() == uid as u32)
                .unwrap_or(false);
        }
    }
    fs::metadata(format!("/data/user_de/0/{}", *VARIANT))
        .map(|s| s.st_uid() == uid as u32)
        .unwrap_or(false)
}
