use anyhow::{bail, Result};
use std::ffi::{c_char, c_void};

pub const ANDROID_NAMESPACE_TYPE_SHARED: u64 = 0x2;
pub const ANDROID_DLEXT_USE_NAMESPACE: u64 = 0x200;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AndroidNamespace {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct AndroidDlextinfo {
    pub flags: u64,
    pub reserved_addr: *mut c_void,
    pub reserved_size: libc::size_t,
    pub relro_fd: libc::c_int,
    pub library_fd: libc::c_int,
    pub library_fd_offset: libc::off64_t,
    pub library_namespace: *mut AndroidNamespace,
}

extern "C" {
    pub fn android_dlopen_ext(
        filename: *const c_char,
        flags: libc::c_int,
        extinfo: *const AndroidDlextinfo,
    ) -> *mut c_void;
}

type AndroidCreateNamespaceFn = unsafe extern "C" fn(
    *const c_char,          // name
    *const c_char,          // ld_library_path
    *const c_char,          // default_library_path
    u64,                    // type
    *const c_char,          // permitted_when_isolated_path
    *mut AndroidNamespace,  // parent
    *const c_void,          // caller_addr
) -> *mut AndroidNamespace;

pub unsafe fn dlopen(path: &str, flags: i32) -> Result<*mut c_void> {
    let filename = std::ffi::CString::new(path)?;
    let filename = filename.as_ptr() as *mut _;
    let dir = libc::dirname(filename);
    let mut info = AndroidDlextinfo {
        flags: 0,
        reserved_addr: std::ptr::null_mut(),
        reserved_size: 0,
        relro_fd: 0,
        library_fd: 0,
        library_fd_offset: 0,
        library_namespace: std::ptr::null_mut(),
    };

    let android_create_namespace_fn = libc::dlsym(
        libc::RTLD_DEFAULT,
        std::ffi::CString::new("__loader_android_create_namespace")?.as_ptr(),
    );
    let android_create_namespace_fn: AndroidCreateNamespaceFn = std::mem::transmute(android_create_namespace_fn);
    let ns = android_create_namespace_fn(
        filename, dir, std::ptr::null(),
        ANDROID_NAMESPACE_TYPE_SHARED,
        std::ptr::null(), std::ptr::null_mut(),
        &dlopen as *const _ as *const c_void,
    );
    if ns != std::ptr::null_mut() {
        info.flags = ANDROID_DLEXT_USE_NAMESPACE;
        info.library_namespace = ns;
        log::debug!("Open {} with namespace {:p}", path, ns);
    } else {
        log::debug!("Cannot create namespace for {}", path);
    };

    let result = android_dlopen_ext(filename, flags, &info);
    if result.is_null() {
        let e = std::ffi::CStr::from_ptr(libc::dlerror()).to_string_lossy();
        bail!(e);
    }
    Ok(result)
}
