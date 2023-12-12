mod constants;
mod dl;
mod root_impl;
mod utils;
mod zygiskd;
mod companion;

use std::future::Future;
use crate::constants::ZKSU_VERSION;

fn init_android_logger(tag: &str) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(constants::MAX_LOG_LEVEL)
            .with_tag(tag),
    );
}

fn start() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 && args[1] == "companion" {
        let fd: i32 = args[2].parse().unwrap();
        companion::entry(fd);
        return;
    } else if args.len() == 2 && args[1] == "version" {
        println!("Zygisk Next daemon {}", ZKSU_VERSION);
        return;
    } else if args.len() == 2 && args[1] == "root" {
        root_impl::setup();
        println!("root impl: {:?}", root_impl::get_impl());
        return;
    }

    utils::switch_mount_namespace(1).expect("switch mnt ns");
    root_impl::setup();
    log::info!("current root impl: {:?}", root_impl::get_impl());
    zygiskd::main().expect("zygiskd main");
}

fn main() {
    let process = std::env::args().next().unwrap();
    let nice_name = process.split('/').last().unwrap();
    init_android_logger(nice_name);

    start();
}
