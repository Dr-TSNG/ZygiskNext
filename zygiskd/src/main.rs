mod constants;
mod dl;
mod root_impl;
mod utils;
mod watchdog;
mod zygiskd;

use std::future::Future;
use anyhow::Result;

fn init_android_logger(tag: &str) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(constants::MAX_LOG_LEVEL)
            .with_tag(tag),
    );
}

fn async_start<F: Future>(future: F) -> F::Output {
    let async_runtime = tokio::runtime::Runtime::new().unwrap();
    async_runtime.block_on(future)
}

fn start(name: &str) -> Result<()> {
    utils::switch_mount_namespace(1)?;
    root_impl::setup();
    match name.trim_start_matches("zygisk-") {
        "wd" => async_start(watchdog::main())?,
        lp_select!("cp32", "cp64") => zygiskd::main()?,
        _ => println!("Available commands: wd, fuse, cp, ptrace"),
    }
    Ok(())
}

fn main() {
    let process = std::env::args().next().unwrap();
    let nice_name = process.split('/').last().unwrap();
    init_android_logger(nice_name);

    if let Err(e) = start(nice_name) {
        log::error!("Crashed: {}\n{}", e, e.backtrace());
    }
}
