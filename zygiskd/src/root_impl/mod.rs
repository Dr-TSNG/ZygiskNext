mod kernelsu;
mod magisk;

pub fn uid_on_allowlist(uid: i32) -> bool {
    if kernelsu::is_kernel_su() {
        kernelsu::uid_on_allowlist(uid)
    } else if magisk::is_magisk() {
        magisk::uid_on_allowlist(uid)
    } else {
        log::warn!("Unknown root implementation");
        false
    }
}

pub fn uid_on_denylist(uid: i32) -> bool {
    if kernelsu::is_kernel_su() {
        kernelsu::uid_on_denylist(uid)
    } else if magisk::is_magisk() {
        magisk::uid_on_denylist(uid)
    } else {
        log::warn!("Unknown root implementation");
        false
    }
}
