use anyhow::{bail, Result};
use std::process::{Command, Stdio};
use crate::constants::MIN_MAGISK_VERSION;

pub fn is_magisk() -> Result<bool> {
    let version: Option<i32> = Command::new("magisk")
        .arg("--version")
        .stdout(Stdio::piped())
        .spawn().ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .and_then(|output| output.parse().ok());
    if let Some(version) = version {
        if version < MIN_MAGISK_VERSION {
            bail!("Magisk version too old: {}", version);
        }
        return Ok(true);
    }
    Ok(false)
}

pub fn uid_on_allowlist(uid: i32) -> bool {
    // TODO: uid_on_allowlist
    return false;
}

pub fn uid_on_denylist(uid: i32) -> bool {
    // TODO: uid_on_denylist
    return false;
}
