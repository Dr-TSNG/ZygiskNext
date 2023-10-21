use std::cmp::min;
use anyhow::{bail, Result};
use std::ffi::{CString, OsStr};
use std::{fs, thread};
use std::sync::{mpsc, Mutex};
use std::time::{Duration, SystemTime};
use fuser::{FileAttr, Filesystem, FileType, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, ReplyOpen, Request};
use libc::ENOENT;
use log::{debug, error, info};
use proc_maps::{get_process_maps, MapRange, Pid};
use ptrace_do::{RawProcess, TracedProcess};
use rustix::mount::mount_bind;
use rustix::path::Arg;
use rustix::process::getpid;
use crate::{constants, dl};
use crate::utils::LateInit;

pub struct DelegateFilesystem;

const fn attr(inode: u64, size: u64, kind: FileType) -> FileAttr {
    FileAttr {
        ino: inode,
        size,
        blocks: 0,
        atime: SystemTime::UNIX_EPOCH,
        mtime: SystemTime::UNIX_EPOCH,
        ctime: SystemTime::UNIX_EPOCH,
        crtime: SystemTime::UNIX_EPOCH,
        kind,
        perm: 0o644,
        nlink: 0,
        uid: 0,
        gid: 0,
        rdev: 0,
        blksize: 0,
        flags: 0,
    }
}

const ANDROID_LIBC: &str = "bionic/libc.so";
const ANDROID_LIBDL: &str = "bionic/libdl.so";

const INO_DIR: u64 = 1;
const INO_PCL: u64 = 2;

static ATTR_DIR: FileAttr = attr(INO_DIR, 0, FileType::Directory);
static ATTR_PCL: LateInit<FileAttr> = LateInit::new();

static PCL_CONTENT: LateInit<Vec<u8>> = LateInit::new();

const ENTRIES: &[(u64, FileType, &str)] = &[
    (INO_DIR, FileType::Directory, "."),
    (INO_DIR, FileType::Directory, ".."),
    (INO_PCL, FileType::RegularFile, "preloaded-classes"),
];

const TTL: Duration = Duration::from_secs(1);

impl Filesystem for DelegateFilesystem {
    fn lookup(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent != INO_DIR {
            reply.error(ENOENT);
            return;
        }
        match name.as_str().unwrap() {
            "preloaded-classes" => reply.entry(&TTL, &ATTR_PCL, 0),
            _ => reply.error(ENOENT),
        }
    }

    fn getattr(&mut self, _req: &Request<'_>, ino: u64, reply: ReplyAttr) {
        match ino {
            INO_DIR => reply.attr(&TTL, &ATTR_DIR),
            INO_PCL => reply.attr(&TTL, &ATTR_PCL),
            _ => reply.error(ENOENT),
        }
    }

    fn open(&mut self, req: &Request<'_>, ino: u64, _flags: i32, reply: ReplyOpen) {
        if ino == INO_PCL {
            let pid = req.pid();
            let process = format!("/proc/{}/cmdline", pid);
            let process = fs::read_to_string(process).unwrap();
            let process = &process[..process.find('\0').unwrap()];
            info!("Process {} is reading preloaded-classes", process);
            if process == "zygote64" {
                ptrace_zygote(pid).unwrap();
            }
        }
        reply.opened(0, 0);
    }

    fn read(&mut self, _req: &Request<'_>, ino: u64, _fh: u64, offset: i64, size: u32, _flags: i32, _lock_owner: Option<u64>, reply: ReplyData) {
        let offset = offset as usize;
        let size = size as usize;
        if ino == INO_PCL {
            let len = PCL_CONTENT.len();
            if offset >= len {
                reply.data(&[]);
            } else {
                let end = min(offset + size, len);
                reply.data(&PCL_CONTENT[offset..end]);
            }
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir(&mut self, _req: &Request<'_>, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
        if ino != INO_DIR {
            reply.error(ENOENT);
            return;
        }
        for (i, entry) in ENTRIES.iter().enumerate().skip(offset as usize) {
            if reply.add(entry.0, (i + 1) as i64, entry.1, entry.2) {
                break;
            }
        }
        reply.ok();
    }
}

fn find_module_for_pid(pid: Pid, library: &str) -> Result<MapRange> {
    let maps = get_process_maps(pid)?;
    for map in maps.into_iter() {
        if let Some(p) = map.filename() {
            if p.as_str()?.contains(library) {
                return Ok(map);
            }
        }
    }
    bail!("Cannot find module {library} for pid {pid}");
}

fn find_remote_procedure(
    pid: Pid,
    library: &str,
    local_addr: usize,
) -> Result<usize> {
    let local_module = find_module_for_pid(getpid().as_raw_nonzero().get(), library)?;
    debug!(
        "Identifed local range {library} ({:?}) at {:x}",
        local_module.filename(),
        local_module.start()
    );

    let remote_module = find_module_for_pid(pid, library)?;
    debug!(
        "Identifed remote range {library} ({:?}) at {:x}",
        remote_module.filename(),
        remote_module.start()
    );

    Ok(local_addr - local_module.start() + remote_module.start())
}

fn ptrace_zygote(pid: u32) -> Result<()> {
    static LAST: Mutex<u32> = Mutex::new(0);

    let mut last = LAST.lock().unwrap();
    if *last == pid {
        return Ok(());
    }
    *last = pid;
    let (sender, receiver) = mpsc::channel::<()>();

    let worker = move || -> Result<()> {
        info!("Injecting into pid {}", pid);
        let zygisk_lib = format!("{}/{}", constants::PATH_SYSTEM_LIB, constants::ZYGISK_LIBRARY);
        let lib_dir = CString::new(constants::PATH_SYSTEM_LIB)?;
        let zygisk_lib = CString::new(zygisk_lib)?;
        let libc_base = find_module_for_pid(pid as i32, ANDROID_LIBC)?.start();
        let mmap_remote = find_remote_procedure(
            pid as i32,
            ANDROID_LIBC,
            libc::mmap as usize,
        )?;
        let munmap_remote = find_remote_procedure(
            pid as i32,
            ANDROID_LIBC,
            libc::munmap as usize,
        )?;
        let dlopen_remote = find_remote_procedure(
            pid as i32,
            ANDROID_LIBDL,
            dl::android_dlopen_ext as usize,
        )?;
        let dlsym_remote = find_remote_procedure(
            pid as i32,
            ANDROID_LIBDL,
            libc::dlsym as usize,
        )?;

        let tracer = TracedProcess::attach(RawProcess::new(pid as i32))?;
        sender.send(())?;
        let frame = tracer.next_frame()?;
        debug!("Waited for a frame");

        // Map a buffer in the remote process
        let mmap_params: [usize; 6] = [
            0,
            0x1000,
            (libc::PROT_READ | libc::PROT_WRITE) as usize,
            (libc::MAP_ANONYMOUS | libc::MAP_PRIVATE) as usize,
            0,
            0,
        ];
        let (regs, mut frame) = frame.invoke_remote(
            mmap_remote,
            libc_base,
            &mmap_params,
        )?;
        let buf_addr = regs.return_value();
        debug!("Buffer addr: {:x}", buf_addr);

        // Find the address of __loader_android_create_namespace
        let sym = CString::new("__loader_android_create_namespace")?;
        frame.write_memory(buf_addr, sym.as_bytes_with_nul())?;
        let (regs, mut frame) = frame.invoke_remote(
            dlsym_remote,
            libc_base,
            &[libc::RTLD_DEFAULT as usize, buf_addr],
        )?;
        let android_create_namespace_remote = regs.return_value();
        debug!("__loader_android_create_namespace addr: {:x}", android_create_namespace_remote);

        // Create a linker namespace for remote process
        frame.write_memory(buf_addr, zygisk_lib.as_bytes_with_nul())?;
        frame.write_memory(buf_addr + 0x100, lib_dir.as_bytes_with_nul())?;
        let ns_params: [usize; 7] = [
            buf_addr,                                   // name
            buf_addr + 0x100,                           // ld_library_path
            0,                                          // default_library_path
            dl::ANDROID_NAMESPACE_TYPE_SHARED as usize, // type
            0,                                          // permitted_when_isolated_path
            0,                                          // parent
            dlopen_remote,                              // caller_addr
        ];
        let (regs, mut frame) = frame.invoke_remote(
            android_create_namespace_remote,
            libc_base,
            &ns_params,
        )?;
        let ns_addr = regs.return_value();
        debug!("Linker namespace addr: {:x}", ns_addr);

        // Load zygisk into remote process
        let info = dl::AndroidDlextinfo {
            flags: dl::ANDROID_DLEXT_USE_NAMESPACE,
            reserved_addr: std::ptr::null_mut(),
            reserved_size: 0,
            relro_fd: 0,
            library_fd: 0,
            library_fd_offset: 0,
            library_namespace: ns_addr as *mut _,
        };
        let info = unsafe {
            std::slice::from_raw_parts(
                &info as *const _ as *const u8,
                std::mem::size_of::<dl::AndroidDlextinfo>(),
            )
        };
        frame.write_memory(buf_addr + 0x200, info)?;
        let (regs, mut frame) = frame.invoke_remote(
            dlopen_remote,
            libc_base,
            &[buf_addr, libc::RTLD_NOW as usize, buf_addr + 0x200],
        )?;
        let handle = regs.return_value();
        debug!("Load zygisk into remote process: {:x}", handle);

        let entry = CString::new("entry")?;
        frame.write_memory(buf_addr, entry.as_bytes_with_nul())?;
        let (regs, frame) = frame.invoke_remote(
            dlsym_remote,
            libc_base,
            &[handle, buf_addr],
        )?;
        let entry = regs.return_value();
        debug!("Call zygisk entry: {:x}", entry);
        let (_, frame) = frame.invoke_remote(
            entry,
            libc_base,
            &[handle],
        )?;

        // Cleanup
        let _ = frame.invoke_remote(
            munmap_remote,
            libc_base,
            &[buf_addr],
        )?;
        debug!("Cleaned up");
        Ok(())
    };

    thread::spawn(move || {
        if let Err(e) = worker() {
            error!("Crashed: {:?}", e);
        }
    });

    receiver.recv()?;
    Ok(())
}

pub fn main() -> Result<()> {
    info!("Start zygisk fuse");
    fs::create_dir(constants::PATH_WORK_DIR)?;
    fs::create_dir(constants::PATH_FUSE_DIR)?;
    PCL_CONTENT.init(fs::read(constants::PATH_PCL)?);
    ATTR_PCL.init(attr(INO_PCL, PCL_CONTENT.len() as u64, FileType::RegularFile));
    let options = [
        fuser::MountOption::FSName(String::from("zygisk")),
        fuser::MountOption::AllowOther,
        fuser::MountOption::RO,
    ];
    let session = fuser::spawn_mount2(
        DelegateFilesystem,
        constants::PATH_FUSE_DIR,
        &options,
    )?;
    mount_bind(constants::PATH_FUSE_PCL, constants::PATH_PCL)?;
    match session.guard.join() {
        Err(e) => bail!("Fuse mount crashed: {:?}", e),
        _ => bail!("Fuse mount exited unexpectedly"),
    }
}
