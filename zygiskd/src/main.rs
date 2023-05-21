#![allow(dead_code)]

mod companion;
mod constants;
mod dl;
mod magic;
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
}


fn init_android_logger(tag: &str) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(constants::MAX_LOG_LEVEL)
            .with_tag(tag),
    );
}

async fn start() -> Result<()> {
    root_impl::setup();
    magic::setup()?;
    let cli = Args::parse();
    match cli.command {
        Commands::Watchdog => watchdog::entry().await?,
        Commands::Daemon => zygiskd::entry()?,
    };
    Ok(())
}

#[tokio::main]
async fn main() {
    let process = std::env::args().next().unwrap();
    let nice_name = process.split('/').last().unwrap();
    init_android_logger(nice_name);

    if let Err(e) = start().await {
        log::error!("Crashed: {}\n{}", e, e.backtrace());
    }
}
