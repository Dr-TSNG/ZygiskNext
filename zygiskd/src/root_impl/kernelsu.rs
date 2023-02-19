use nix::libc::prctl;

const KERNEL_SU_OPTION: i32 = 0xdeadbeefu32 as i32;

const CMD_GET_VERSION: u64 = 2;
const CMD_GET_ALLOW_LIST: u64 = 5;
const CMD_GET_DENY_LIST: u64 = 6;

pub fn is_kernel_su() -> bool {
    let mut version = 0;
    unsafe { prctl(KERNEL_SU_OPTION, CMD_GET_VERSION, &mut version as *mut i32) };
    version > 0
}

pub fn uid_on_allowlist(uid: i32) -> bool {
    let mut size = 1024u32;
    let mut uids = vec![0; size as usize];
    unsafe { prctl(KERNEL_SU_OPTION, CMD_GET_ALLOW_LIST, uids.as_mut_ptr(), &mut size as *mut u32) };
    uids.resize(size as usize, 0);
    uids.contains(&uid)
}

pub fn uid_on_denylist(uid: i32) -> bool {
    let mut size = 1024u32;
    let mut uids = vec![0; size as usize];
    unsafe { prctl(KERNEL_SU_OPTION, CMD_GET_DENY_LIST, uids.as_mut_ptr(), &mut size as *mut u32) };
    uids.resize(size as usize, 0);
    uids.contains(&uid)
}
