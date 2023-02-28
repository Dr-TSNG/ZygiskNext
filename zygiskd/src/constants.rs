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

pub const PROP_NATIVE_BRIDGE: &str = "ro.dalvik.vm.native.bridge";
pub const PROP_CTL_RESTART: &str = "ctl.restart";
pub const ZYGISK_LOADER: &str = "libzygiskloader.so";

pub const SOCKET_PLACEHOLDER: &str = "socket_placeholder";

pub const PATH_MODULES_DIR: &str = "..";
pub const PATH_MODULE_PROP: &str = "module.prop";
pub const PATH_ZYGISKD32: &str = "bin/zygiskd32";
pub const PATH_ZYGISKD64: &str = "bin/zygiskd64";
pub const PATH_TMP_DIR: &str = concatcp!("/dev/", SOCKET_PLACEHOLDER);
pub const PATH_TMP_PROP: &str = concatcp!("/dev/", SOCKET_PLACEHOLDER, "/module.prop");

pub const STATUS_LOADED: &str = "😋 Zygisksu is loaded";
pub const STATUS_CRASHED: &str = "❌ Zygiskd has crashed";
pub const STATUS_ROOT_IMPL_NONE: &str = "❌ Unknown root implementation";
pub const STATUS_ROOT_IMPL_TOO_OLD: &str = "❌ Root implementation version too old";
pub const STATUS_ROOT_IMPL_ABNORMAL: &str = "❌ Abnormal root implementation version";
pub const STATUS_ROOT_IMPL_MULTIPLE: &str = "❌ Multiple root implementations installed";

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
pub const PROCESS_ROOT_IS_KSU: u32 = 1 << 29;
pub const PROCESS_ROOT_IS_MAGISK: u32 = 1 << 30;
pub const PROCESS_IS_SYSUI: u32 = 1 << 31;
