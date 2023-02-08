#![allow(dead_code)]

mod companion;
mod constants;
mod dl;
mod utils;
mod watchdog;
mod zygiskd;

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

fn main() {
    let process = std::env::args().next().unwrap();
    let nice_name = process.split('/').last().unwrap();
    init_android_logger(nice_name);

    let cli = Args::parse();
    let result = match cli.command {
        Commands::Watchdog => watchdog::entry(),
        Commands::Daemon => zygiskd::entry(lp_select!(false, true)),
        Commands::Companion { fd } => companion::entry(fd),
    };

    if let Err(e) = &result {
        log::error!("Crashed: {}\n{}", e, e.backtrace());
    }
}
