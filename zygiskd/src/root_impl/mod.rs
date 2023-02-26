mod kernelsu;
mod magisk;

use once_cell::sync::OnceCell;

pub enum RootImpl {
    None,
    TooOld,
    Abnormal,
    Multiple,
    KernelSU,
    Magisk,
}

static ROOT_IMPL: OnceCell<RootImpl> = OnceCell::new();

pub fn setup() {
    let ksu_version = kernelsu::get_kernel_su();
    let magisk_version = magisk::get_magisk();

    let _ = match (ksu_version, magisk_version) {
        (None, None) => ROOT_IMPL.set(RootImpl::None),
        (Some(_), Some(_)) => ROOT_IMPL.set(RootImpl::Multiple),
        (Some(ksu_version), None) => {
            let val = match ksu_version {
                kernelsu::Version::Supported => RootImpl::KernelSU,
                kernelsu::Version::TooOld => RootImpl::TooOld,
                kernelsu::Version::Abnormal => RootImpl::Abnormal,
            };
            ROOT_IMPL.set(val)
        }
        (None, Some(magisk_version)) => {
            let val = match magisk_version {
                magisk::Version::Supported => RootImpl::Magisk,
                magisk::Version::TooOld => RootImpl::TooOld,
            };
            ROOT_IMPL.set(val)
        }
    };
}

pub fn get_impl() -> &'static RootImpl {
    ROOT_IMPL.get().unwrap()
}

pub fn uid_on_allowlist(uid: i32) -> bool {
    match ROOT_IMPL.get().unwrap() {
        RootImpl::KernelSU => kernelsu::uid_on_allowlist(uid),
        RootImpl::Magisk => magisk::uid_on_allowlist(uid),
        _ => unreachable!(),
    }
}

pub fn uid_on_denylist(uid: i32) -> bool {
    match ROOT_IMPL.get().unwrap() {
        RootImpl::KernelSU => kernelsu::uid_on_denylist(uid),
        RootImpl::Magisk => magisk::uid_on_denylist(uid),
        _ => unreachable!(),
    }
}
