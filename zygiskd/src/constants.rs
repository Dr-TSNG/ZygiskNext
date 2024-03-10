use crate::lp_select;
use bitflags::bitflags;
use konst::primitive::parse_i32;
use konst::unwrap_ctx;
use log::LevelFilter;
use num_enum::TryFromPrimitive;

pub const MIN_KSU_VERSION: i32 = unwrap_ctx!(parse_i32(env!("MIN_KSU_VERSION")));
pub const MAX_KSU_VERSION: i32 = unwrap_ctx!(parse_i32(env!("MAX_KSU_VERSION")));
pub const MIN_MAGISK_VERSION: i32 = unwrap_ctx!(parse_i32(env!("MIN_MAGISK_VERSION")));
pub const MIN_APATCH_VERSION: i32 = unwrap_ctx!(parse_i32(env!("MIN_APATCH_VERSION")));
pub const ZKSU_VERSION: &str = env!("ZKSU_VERSION");

#[cfg(debug_assertions)]
pub const MAX_LOG_LEVEL: LevelFilter = LevelFilter::Trace;
#[cfg(not(debug_assertions))]
pub const MAX_LOG_LEVEL: LevelFilter = LevelFilter::Info;

pub const PATH_MODULES_DIR: &str = "..";
pub const ZYGOTE_INJECTED: i32 = lp_select!(5, 4);
pub const DAEMON_SET_INFO: i32 = lp_select!(7, 6);
pub const DAEMON_SET_ERROR_INFO: i32 = lp_select!(9, 8);
pub const SYSTEM_SERVER_STARTED: i32 = 10;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum DaemonSocketAction {
    PingHeartbeat,
    RequestLogcatFd,
    GetProcessFlags,
    ReadModules,
    RequestCompanionSocket,
    GetModuleDir,
    ZygoteRestart,
    SystemServerStarted,
}

// Zygisk process flags
bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ProcessFlags: u32 {
        const PROCESS_GRANTED_ROOT = 1 << 0;
        const PROCESS_ON_DENYLIST = 1 << 1;
        const PROCESS_ROOT_IS_APATCH = 1 << 27;
        const PROCESS_IS_MANAGER = 1 << 28;
        const PROCESS_ROOT_IS_KSU = 1 << 29;
        const PROCESS_ROOT_IS_MAGISK = 1 << 30;
        const PROCESS_IS_SYSUI = 1 << 31;
    }
}
