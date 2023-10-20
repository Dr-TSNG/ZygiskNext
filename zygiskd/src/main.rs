#![feature(exclusive_range_pattern)]
#![allow(dead_code)]

mod constants;
mod dl;
mod fuse;
mod root_impl;
mod utils;
mod watchdog;
mod zygiskd;

use anyhow::Result;


fn init_android_logger(tag: &str) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(constants::MAX_LOG_LEVEL)
            .with_tag(tag),
    );
}

async fn start(name: &str) -> Result<()> {
    root_impl::setup();
    match name.trim_start_matches("zygisk-") {
        "wd" => watchdog::main().await?,
        "fuse" => fuse::main()?,
        "cp" => zygiskd::main()?,
        _ => println!("Available commands: wd, fuse, cp"),
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let process = std::env::args().next().unwrap();
    let nice_name = process.split('/').last().unwrap();
    init_android_logger(nice_name);

    if let Err(e) = start(nice_name).await {
        log::error!("Crashed: {}\n{}", e, e.backtrace());
    }
}
