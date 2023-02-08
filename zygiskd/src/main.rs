#![allow(dead_code)]

mod constants;
mod utils;
mod watchdog;
mod zygisk;

use anyhow::{bail, Result};
use log::LevelFilter;
use nix::libc;

#[cfg(debug_assertions)]
static MAX_LOG_LEVEL: LevelFilter = LevelFilter::Trace;
#[cfg(not(debug_assertions))]
static MAX_LOG_LEVEL: LevelFilter = LevelFilter::Info;

fn init_android_logger(tag: &str) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(MAX_LOG_LEVEL)
            .with_tag(tag),
    );
}

fn entry() -> Result<()> {
    let process = std::env::args().next().unwrap();
    let process = process.split('/').last().unwrap();
    init_android_logger(process);
    match process {
        "zygiskwd" => {
            log::info!("Start zygisksu watchdog");
            watchdog::check_permission()?;
            watchdog::ensure_single_instance()?;
            watchdog::spawn_daemon()?;
        }
        "zygiskd32" => {
            log::info!("Start zygiskd32");
            unsafe { libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL); }
            zygisk::start(false)?;
            loop {}
        }
        "zygiskd64" => {
            log::info!("Start zygiskd64");
            unsafe { libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL); }
            zygisk::start(true)?;
        }
        _ => bail!("Unexpected process name: {process}")
    }
    Ok(())
}

fn main() {
    if let Err(e) = entry() {
        log::error!("Crashed: {}\n{}", e, e.backtrace());
    }
}
