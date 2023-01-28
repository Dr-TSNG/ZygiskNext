use const_format::concatcp;
use num_enum::TryFromPrimitive;

pub const PROP_NATIVE_BRIDGE: &str = "ro.dalvik.vm.native.bridge";

pub const KSU_MODULE_DIR: &str = "/data/adb/ksu/modules";
pub const ZYGISKSU_DIR: &str = concatcp!(KSU_MODULE_DIR, "/zygisksu");
pub const ZYGISKWD: &str = concatcp!(ZYGISKSU_DIR, "/zygiskwd");
pub const ZYGISKD32: &str = concatcp!(ZYGISKSU_DIR, "/zygiskd32");
pub const ZYGISKD64: &str = concatcp!(ZYGISKSU_DIR, "/zygiskd64");
pub const DAEMON_LOCK: &str = concatcp!(ZYGISKSU_DIR, "/zygiskd.lock");

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum DaemonSocketAction {
    ReadNativeBridge,
    ReadModules,
    RequestCompanionSocket,
}
