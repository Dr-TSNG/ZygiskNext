use bitflags::bitflags;
use const_format::concatcp;
use konst::primitive::parse_i32;
use konst::unwrap_ctx;
use log::LevelFilter;
use num_enum::TryFromPrimitive;
use crate::lp_select;

pub const MIN_KSU_VERSION: i32 = unwrap_ctx!(parse_i32(env!("MIN_KSU_VERSION")));
pub const MAX_KSU_VERSION: i32 = unwrap_ctx!(parse_i32(env!("MAX_KSU_VERSION")));
pub const MIN_MAGISK_VERSION: i32 = unwrap_ctx!(parse_i32(env!("MIN_MAGISK_VERSION")));

#[cfg(debug_assertions)]
pub const MAX_LOG_LEVEL: LevelFilter = LevelFilter::Trace;
#[cfg(not(debug_assertions))]
pub const MAX_LOG_LEVEL: LevelFilter = LevelFilter::Info;

pub const PROP_CTL_RESTART: &str = "ctl.restart";

pub const PATH_PCL: &str = "/system/etc/preloaded-classes";
pub const PATH_ZYGISK_LIB: &str = concatcp!(lp_select!("/system/lib", "/system/lib64"), "/libzygisk.so");
pub const PATH_WORK_DIR: &str = "/dev/zygisk"; // TODO: Replace with /debug_ramdisk/zygisk
pub const PATH_PROP_OVERLAY: &str = concatcp!(PATH_WORK_DIR, "/module.prop");
pub const PATH_CP_SOCKET: &str = concatcp!(PATH_WORK_DIR, lp_select!("/cp32.sock", "/cp64.sock"));
pub const PATH_FUSE_DIR: &str = concatcp!(PATH_WORK_DIR, "/fuse");
pub const PATH_FUSE_PCL: &str = concatcp!(PATH_FUSE_DIR, "/preloaded-classes");
pub const PATH_FUSE_ABORT_OVERLAY: &str = concatcp!(PATH_WORK_DIR, "/abort");

pub const PATH_MODULES_DIR: &str = "..";
pub const PATH_MODULE_PROP: &str = "module.prop";
pub const PATH_CP_BIN32: &str = "bin/zygisk-cp32";
pub const PATH_CP_BIN64: &str = "bin/zygisk-cp64";
pub const PATH_PTRACE_BIN32: &str = "bin/zygisk-ptrace32";
pub const PATH_PTRACE_BIN64: &str = "bin/zygisk-ptrace64";


pub const STATUS_LOADED: &str = "😋 Zygisk Next is loaded";
pub const STATUS_CRASHED: &str = "❌ Zygisk Next has crashed";
pub const STATUS_ROOT_IMPL_NONE: &str = "❌ Unknown root implementation";
pub const STATUS_ROOT_IMPL_TOO_OLD: &str = "❌ Root implementation version too old";
pub const STATUS_ROOT_IMPL_ABNORMAL: &str = "❌ Abnormal root implementation version";
pub const STATUS_ROOT_IMPL_MULTIPLE: &str = "❌ Multiple root implementations installed";

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum DaemonSocketAction {
    PingHeartbeat,
    RequestLogcatFd,
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
