use nix::libc::prctl;
use crate::constants::{MIN_KSU_VERSION, MAX_KSU_VERSION};

const KERNEL_SU_OPTION: i32 = 0xdeadbeefu32 as i32;

const CMD_GET_VERSION: usize = 2;
const CMD_UID_GRANTED_ROOT: usize = 12;
const CMD_UID_SHOULD_UMOUNT: usize = 13;

pub enum Version {
    Supported,
    TooOld,
    Abnormal,
}

pub fn get_kernel_su() -> Option<Version> {
    let mut version = 0;
    unsafe { prctl(KERNEL_SU_OPTION, CMD_GET_VERSION, &mut version as *mut i32) };
    match version {
        0 => None,
        MIN_KSU_VERSION..=MAX_KSU_VERSION => Some(Version::Supported),
        1..=MIN_KSU_VERSION => Some(Version::TooOld),
        _ => Some(Version::Abnormal)
    }
}

pub fn uid_granted_root(uid: i32) -> i32 {
    let mut granted = 0;
    unsafe { prctl(KERNEL_SU_OPTION, CMD_UID_GRANTED_ROOT, uid, &mut granted as *mut bool) };
    granted
}

pub fn uid_should_umount(uid: i32) -> i32 {
    let mut umount = 0;
    unsafe { prctl(KERNEL_SU_OPTION, CMD_UID_SHOULD_UMOUNT, uid, &mut umount as *mut bool) };
    umount
}
