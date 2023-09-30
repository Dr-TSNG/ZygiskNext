use bitflags::bitflags;
use const_format::concatcp;
use konst::primitive::parse_i32;
use konst::unwrap_ctx;
use log::LevelFilter;
use num_enum::TryFromPrimitive;

pub const VERSION_NAME: &str = env!("VERSION_NAME");
pub const VERSION_CODE: &str = env!("VERSION_CODE");
pub const VERSION_FULL: &str = concatcp!(VERSION_NAME, " (", VERSION_CODE, ")");
pub const MIN_KSU_VERSION: i32 = unwrap_ctx!(parse_i32(env!("MIN_KSU_VERSION")));
pub const MIN_KSU_VERSION_1: i32 = MIN_KSU_VERSION - 1;
pub const MAX_KSU_VERSION: i32 = unwrap_ctx!(parse_i32(env!("MAX_KSU_VERSION")));
pub const MIN_MAGISK_VERSION: i32 = unwrap_ctx!(parse_i32(env!("MIN_MAGISK_VERSION")));

#[cfg(debug_assertions)]
pub const MAX_LOG_LEVEL: LevelFilter = LevelFilter::Trace;
#[cfg(not(debug_assertions))]
pub const MAX_LOG_LEVEL: LevelFilter = LevelFilter::Info;

pub const PROP_NATIVE_BRIDGE: &str = "ro.dalvik.vm.native.bridge";
pub const PROP_CTL_RESTART: &str = "ctl.restart";
pub const ZYGISK_LOADER: &str = "libzygisk_loader.so";
pub const ZYGISK_MAGIC: &str = "/system/zygisk_magic";

pub const PATH_MODULES_DIR: &str = "..";
pub const PATH_MODULE_PROP: &str = "module.prop";
pub const PATH_ZYGISKD32: &str = "bin/zygiskd32";
pub const PATH_ZYGISKD64: &str = "bin/zygiskd64";

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
bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ProcessFlags: u32 {
        const PROCESS_GRANTED_ROOT = 1 << 0;
        const PROCESS_ON_DENYLIST = 1 << 1;
        const PROCESS_ROOT_IS_KSU = 1 << 29;
        const PROCESS_ROOT_IS_MAGISK = 1 << 30;
        const PROCESS_IS_SYSUI = 1 << 31;
    }
}
