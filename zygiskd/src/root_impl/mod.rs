mod kernelsu;
mod magisk;

use once_cell::sync::OnceCell;
use anyhow::{bail, Result};

enum RootImpl {
    KernelSU,
    Magisk,
}

static ROOT_IMPL: OnceCell<RootImpl> = OnceCell::new();

pub fn setup() -> Result<()> {
    if kernelsu::is_kernel_su()? {
        if let Ok(true) = magisk::is_magisk() {
            bail!("Multiple root implementation");
        }
        let _ = ROOT_IMPL.set(RootImpl::KernelSU);
    } else if magisk::is_magisk()? {
        let _ = ROOT_IMPL.set(RootImpl::Magisk);
    } else {
        bail!("Unknown root implementation");
    }
    Ok(())
}

pub fn uid_on_allowlist(uid: i32) -> bool {
    match ROOT_IMPL.get().unwrap() {
        RootImpl::KernelSU => kernelsu::uid_on_allowlist(uid),
        RootImpl::Magisk => magisk::uid_on_allowlist(uid),
    }
}

pub fn uid_on_denylist(uid: i32) -> bool {
    match ROOT_IMPL.get().unwrap() {
        RootImpl::KernelSU => kernelsu::uid_on_denylist(uid),
        RootImpl::Magisk => magisk::uid_on_denylist(uid),
    }
}
