use const_format::concatcp;
use konst::primitive::parse_i32;
use konst::unwrap_ctx;
use log::LevelFilter;
use num_enum::TryFromPrimitive;

pub const VERSION_NAME: &str = env!("VERSION_NAME");
pub const VERSION_CODE: &str = env!("VERSION_CODE");
pub const VERSION_FULL: &str = concatcp!(VERSION_NAME, " (", VERSION_CODE, ")");
pub const MIN_KSU_VERSION: i32 = unwrap_ctx!(parse_i32(env!("MIN_KSU_VERSION")));
pub const MAX_KSU_VERSION: i32 = unwrap_ctx!(parse_i32(env!("MAX_KSU_VERSION")));
pub const MIN_MAGISK_VERSION: i32 = unwrap_ctx!(parse_i32(env!("MIN_MAGISK_VERSION")));

#[cfg(debug_assertions)]
pub const MAX_LOG_LEVEL: LevelFilter = LevelFilter::Trace;
#[cfg(not(debug_assertions))]
pub const MAX_LOG_LEVEL: LevelFilter = LevelFilter::Info;

#[cfg(target_pointer_width = "64")]
#[macro_export]
macro_rules! lp_select {
    ($lp32:expr, $lp64:expr) => { $lp64 };
}
#[cfg(target_pointer_width = "32")]
#[macro_export]
macro_rules! lp_select {
    ($lp32:expr, $lp64:expr) => { $lp32 };
}

pub const PROP_NATIVE_BRIDGE: &str = "ro.dalvik.vm.native.bridge";
pub const PROP_SVC_ZYGOTE: &str = "init.svc.zygote";
pub const ZYGISK_LOADER: &str = "libzygiskloader.so";

pub const SOCKET_PLACEHOLDER: &str = "socket_placeholder";

pub const PATH_MODULE_DIR: &str = "..";
pub const PATH_ZYGISKD32: &str = "bin/zygiskd32";
pub const PATH_ZYGISKD64: &str = "bin/zygiskd64";

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum DaemonSocketAction {
    PingHeartbeat,
    RequestLogcatFd,
    ReadNativeBridge,
    GetProcessFlags,
    ReadModules,
    RequestCompanionSocket,
    GetModuleDir,
}

// Zygisk process flags
pub const PROCESS_GRANTED_ROOT: u32 = 1 << 0;
pub const PROCESS_ON_DENYLIST: u32 = 1 << 1;
pub const PROCESS_IS_SYSUI: u32 = 1 << 31;
