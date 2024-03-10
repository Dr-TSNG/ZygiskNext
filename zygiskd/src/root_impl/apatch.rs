use std::process::{Command, Stdio};
use std::fs::File;
use std::fs;
use std::io::{BufRead, BufReader};
use csv::Reader;
use serde::Deserialize;
use crate::constants::MIN_APATCH_VERSION;

pub enum Version {
    Supported,
    TooOld,
    Abnormal,
}

pub fn get_apatch() -> Option<Version> {
    let file_path = "/data/adb/ap/version";
    let contents = fs::read_to_string(file_path).ok()?;
    let version: Option<i32> = contents.trim().parse().ok();
    const MAX_OLD_VERSION: i32 = MIN_APATCH_VERSION - 1;
    match version {
        Some(0) => None,
        Some(v) if v >= MIN_APATCH_VERSION && v <= 999999 => Some(Version::Supported),
        Some(v) if v >= 1 && v <= MAX_OLD_VERSION => Some(Version::TooOld),
        _ => None,
    }
}

#[derive(Deserialize)]
struct PackageConfig {
    pkg: String,
    exclude: i32,
    allow: i32,
    uid: i32,
    to_uid: i32,
    sctx: String,
}

fn read_package_config() -> Result<Vec<PackageConfig>, std::io::Error> {
    let file = File::open("/data/adb/ap/package_config")?;
    let mut reader = csv::Reader::from_reader(file);

    let mut package_configs = Vec::new();
    for record in reader.deserialize() {
        match record {
            Ok(config) => package_configs.push(config),
            Err(error) => {
                log::warn!("Error deserializing record");
            }
        }
    }

    Ok(package_configs)
}

pub fn uid_granted_root(uid: i32) -> bool {
    match read_package_config() {
        Ok(package_configs) => {
            package_configs
                .iter()
                .find(|config| config.uid == uid)
                .map(|config| config.allow == 1)
                .unwrap_or(false)
        }
        Err(err) => {
            log::warn!("Error reading package config");
            return false;
        }
    }
}

pub fn uid_should_umount(uid: i32) -> bool {
    match read_package_config() {
        Ok(package_configs) => {
            package_configs
                .iter()
                .find(|config| config.uid == uid)
                .map(|config| {
                    match config.exclude {
                        0 => false,
                        1 => true,
                        _ => true,
                    }
                })
                .unwrap_or(true)
        }
        Err(err) => {
            log::warn!("Error reading package configs");
            false
        }
    }
}

// TODO: signature
pub fn uid_is_manager(uid: i32) -> bool {
    if let Ok(s) = rustix::fs::stat("/data/user_de/0/me.bmax.apatch") {
        return s.st_uid == uid as u32;
    }
    false
}