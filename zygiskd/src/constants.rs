use const_format::concatcp;
use num_enum::TryFromPrimitive;

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
    ReadNativeBridge,
    ReadModules,
    RequestCompanionSocket,
}
