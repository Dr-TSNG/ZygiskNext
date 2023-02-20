#![allow(dead_code)]

mod companion;
mod constants;
mod dl;
mod root_impl;
mod utils;
mod watchdog;
mod zygiskd;

use anyhow::Result;
use clap::{Subcommand, Parser};

#[derive(Parser, Debug)]
#[command(author, version = constants::VERSION_FULL, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start zygisk watchdog
    Watchdog,
    /// Start zygisk daemon
    Daemon,
    /// Start zygisk companion
    Companion { fd: i32 },
}


fn init_android_logger(tag: &str) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(constants::MAX_LOG_LEVEL)
            .with_tag(tag),
    );
}

fn start() -> Result<()> {
    root_impl::setup()?;
    let cli = Args::parse();
    match cli.command {
        Commands::Watchdog => watchdog::entry()?,
        Commands::Daemon => zygiskd::entry()?,
        Commands::Companion { fd } => companion::entry(fd)?,
    };
    Ok(())
}

fn main() {
    let process = std::env::args().next().unwrap();
    let nice_name = process.split('/').last().unwrap();
    init_android_logger(nice_name);

    if let Err(e) = start() {
        log::error!("Crashed: {}\n{}", e, e.backtrace());
    }
}
