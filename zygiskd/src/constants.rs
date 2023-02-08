use const_format::concatcp;
use log::LevelFilter;
use num_enum::TryFromPrimitive;

pub const VERSION_NAME: &str = env!("VERSION_NAME");
pub const VERSION_CODE: &str = env!("VERSION_CODE");
pub const VERSION_FULL: &str = concatcp!(VERSION_NAME, " (", VERSION_CODE, ")");

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

pub const SOCKET_PLACEHOLDER: &str = "socket_placeholder";

pub const PATH_KSU_MODULE_DIR: &str = "/data/adb/ksu/modules";
pub const PATH_ZYGISKSU_DIR: &str = concatcp!(PATH_KSU_MODULE_DIR, "/zygisksu");
pub const PATH_ZYGISKD32: &str = concatcp!(PATH_ZYGISKSU_DIR, "/bin/zygiskd32");
pub const PATH_ZYGISKD64: &str = concatcp!(PATH_ZYGISKSU_DIR, "/bin/zygiskd64");
pub const PATH_DAEMON_LOCK: &str = concatcp!(PATH_ZYGISKSU_DIR, "/zygiskd.lock");

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum DaemonSocketAction {
    PingHeartbeat,
    RequestLogcatFd,
    ReadNativeBridge,
    ReadModules,
    RequestCompanionSocket,
    GetModuleDir,
}
