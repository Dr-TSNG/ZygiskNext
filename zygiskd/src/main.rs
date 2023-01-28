#![allow(dead_code)]

mod watchdog;
mod constants;
mod zygisk;
mod utils;

use anyhow::{anyhow, Result};
use log::LevelFilter;
use nix::libc;

fn init_android_logger(tag: &str) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag(tag),
    );
}

fn main() -> Result<()> {
    let process = std::env::args().next().unwrap();
    let process = process.split('/').last().unwrap();
    env_logger::init();
    // init_android_logger(process);
    match process {
        "zygiskwd" => {
            log::info!("Start zygisksu watchdog");
            // watchdog::check_permission()?;
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
        _ => return { 
            Err(anyhow!("Unexpected process name: {process}"))
        },
    }
    Ok(())
}
