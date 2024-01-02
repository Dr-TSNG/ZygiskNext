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
pub const ZKSU_VERSION: &'static str = env!("ZKSU_VERSION");

#[cfg(debug_assertions)]
pub const MAX_LOG_LEVEL: LevelFilter = LevelFilter::Trace;
#[cfg(not(debug_assertions))]
pub const MAX_LOG_LEVEL: LevelFilter = LevelFilter::Info;



pub const PATH_MODULES_DIR: &str = "..";
pub const PATH_MODULE_PROP: &str = "module.prop";
pub const ZYGOTE_INJECTED: i32 = lp_select!(5, 4);
pub const DAEMON_SET_INFO: i32 = lp_select!(7, 6);
pub const DAEMON_SET_ERROR_INFO: i32 = lp_select!(9, 8);
pub const TMP_DIR: &str = "/debug_ramdisk/zygisksu";
pub const CONTROLLER_SOCKET: &str = concatcp!(TMP_DIR, "/init_monitor");
pub const PATH_CP_NAME: &str = concatcp!(TMP_DIR, lp_select!("/cp32.sock", "/cp64.sock"));

pub const MAX_RESTART_COUNT: i32 = 5;

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
}

// Zygisk process flags
bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ProcessFlags: u32 {
        const PROCESS_GRANTED_ROOT = 1 << 0;
        const PROCESS_ON_DENYLIST = 1 << 1;
        const PROCESS_IS_MANAGER = 1 << 28;
        const PROCESS_ROOT_IS_KSU = 1 << 29;
        const PROCESS_ROOT_IS_MAGISK = 1 << 30;
        const PROCESS_IS_SYSUI = 1 << 31;
    }
}
