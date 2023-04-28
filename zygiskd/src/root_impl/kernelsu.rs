use nix::libc::prctl;
use crate::constants::{MIN_KSU_VERSION, MAX_KSU_VERSION};

const KERNEL_SU_OPTION: i32 = 0xdeadbeefu32 as i32;

const CMD_GET_VERSION: usize = 2;
const CMD_GET_ALLOW_LIST: usize = 5;

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

#[inline(never)]
pub fn uid_on_allowlist(uid: i32) -> bool {
    let mut size = 1024u32;
    let mut uids = vec![0; size as usize];
    unsafe { prctl(KERNEL_SU_OPTION, CMD_GET_ALLOW_LIST, uids.as_mut_ptr(), &mut size as *mut u32) };
    uids.resize(size as usize, 0);
    uids.contains(&uid)
}

#[inline(never)]
pub fn uid_on_denylist(uid: i32) -> bool {
    false
}
