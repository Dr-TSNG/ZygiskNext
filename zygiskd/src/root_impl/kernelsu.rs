use nix::libc;
use nix::libc::prctl;
use crate::constants::{MIN_KSU_VERSION, MAX_KSU_VERSION};

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
    unsafe { prctl(KERNEL_SU_OPTION as i32, CMD_GET_VERSION, &mut version as *mut i32, 0, 0) };
    match version {
        0 => None,
        MIN_KSU_VERSION..=MAX_KSU_VERSION => Some(Version::Supported),
        1..=MIN_KSU_VERSION => Some(Version::TooOld),
        _ => Some(Version::Abnormal)
    }
}

pub fn uid_granted_root(uid: i32) -> bool {
    let mut result: u32 = 0;
    let mut granted = false;
    unsafe { prctl(KERNEL_SU_OPTION as i32, CMD_UID_GRANTED_ROOT, uid, &mut granted as *mut bool, std::ptr::addr_of_mut!(result).cast::<libc::c_void>()) };
    if result != KERNEL_SU_OPTION {
        log::warn!("uid_granted_root failed");
    }
    granted
}

pub fn uid_should_umount(uid: i32) -> bool {
    let mut result: u32 = 0;
    let mut umount = false;
    unsafe { prctl(KERNEL_SU_OPTION as i32, CMD_UID_SHOULD_UMOUNT, uid, &mut umount as *mut bool, std::ptr::addr_of_mut!(result).cast::<libc::c_void>()) };
    if result != KERNEL_SU_OPTION {
        log::warn!("uid_granted_root failed");
    }
    umount
}
