use std::cmp::min;
use anyhow::{bail, Result};
use std::ffi::OsStr;
use std::{fs, thread};
use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::{mpsc, Mutex};
use std::time::{Duration, SystemTime};
use fuser::{FileAttr, Filesystem, FileType, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, ReplyOpen, Request};
use libc::ENOENT;
use log::{error, info};
use rustix::mount::mount_bind;
use rustix::path::Arg;
use crate::constants;
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

fn ptrace_zygote64(pid: u32) -> Result<()> {
    static LAST: Mutex<u32> = Mutex::new(0);

    let mut last = LAST.lock().unwrap();
    if *last == pid {
        return Ok(());
    }
    *last = pid;
    let (sender, receiver) = mpsc::channel::<()>();

    let worker = move || -> Result<()> {
        let mut child = Command::new(constants::PATH_PTRACE_BIN64).stdout(Stdio::piped()).arg(format!("{}", pid)).spawn()?;
        child.stdout.as_mut().unwrap().read_exact(&mut [0u8; 1])?;
        info!("child attached");
        sender.send(())?;
        let result = child.wait()?;
        info!("ptrace64 process status {}", result);
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

fn ptrace_zygote32(pid: u32) -> Result<()> {
    static LAST: Mutex<u32> = Mutex::new(0);

    let mut last = LAST.lock().unwrap();
    if *last == pid {
        return Ok(());
    }
    *last = pid;
    let (sender, receiver) = mpsc::channel::<()>();

    let worker = move || -> Result<()> {
        let mut child = Command::new(constants::PATH_PTRACE_BIN32).stdout(Stdio::piped()).arg(format!("{}", pid)).spawn()?;
        child.stdout.as_mut().unwrap().read_exact(&mut [0u8; 1])?;
        info!("child attached");
        sender.send(())?;
        let result = child.wait()?;
        info!("ptrace32 process status {}", result);
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
            match process {
                "zygote64" => ptrace_zygote64(pid).unwrap(),
                "zygote" => ptrace_zygote32(pid).unwrap(),
                _ => (),
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
