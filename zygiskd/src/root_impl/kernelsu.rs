use crate::constants::{MAX_KSU_VERSION, MIN_KSU_VERSION};

const KERNEL_SU_OPTION: u32 = 0xdeadbeefu32;

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
    unsafe {
        libc::prctl(
            KERNEL_SU_OPTION as i32,
            CMD_GET_VERSION,
            &mut version as *mut i32,
            0,
            0,
        )
    };
    const MAX_OLD_VERSION: i32 = MIN_KSU_VERSION - 1;
    match version {
        0 => None,
        MIN_KSU_VERSION..=MAX_KSU_VERSION => Some(Version::Supported),
        1..=MAX_OLD_VERSION => Some(Version::TooOld),
        _ => Some(Version::Abnormal),
    }
}

pub fn uid_granted_root(uid: i32) -> bool {
    let mut result: u32 = 0;
    let mut granted = false;
    unsafe {
        libc::prctl(
            KERNEL_SU_OPTION as i32,
            CMD_UID_GRANTED_ROOT,
            uid,
            &mut granted as *mut bool,
            &mut result as *mut u32,
        )
    };
    if result != KERNEL_SU_OPTION {
        log::warn!("uid_granted_root failed");
    }
    granted
}

pub fn uid_should_umount(uid: i32) -> bool {
    let mut result: u32 = 0;
    let mut umount = false;
    unsafe {
        libc::prctl(
            KERNEL_SU_OPTION as i32,
            CMD_UID_SHOULD_UMOUNT,
            uid,
            &mut umount as *mut bool,
            &mut result as *mut u32,
        )
    };
    if result != KERNEL_SU_OPTION {
        log::warn!("uid_granted_root failed");
    }
    umount
}
