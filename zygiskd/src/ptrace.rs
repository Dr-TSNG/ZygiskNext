use log::{debug, error, info};
use std::ffi::CString;
use std::env;
use std::io::Write;
use rustix::path::Arg;
use proc_maps::{get_process_maps, MapRange, Pid};
use ptrace_do::{RawProcess, TracedProcess};
use rustix::process::getpid;
use crate::{constants, dl, lp_select};
use anyhow::{bail, Result};

const ANDROID_LIBC: &str = "bionic/libc.so";
const ANDROID_LIBDL: &str = "bionic/libdl.so";

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
        libc::dlopen as usize,
    )?;
    let dlsym_remote = find_remote_procedure(
        pid as i32,
        ANDROID_LIBDL,
        libc::dlsym as usize,
    )?;
    let errno_remote = find_remote_procedure(
        pid as i32,
        ANDROID_LIBC,
        libc::__errno as usize,
    )?;
    let dlerror_remote = find_remote_procedure(
        pid as i32,
        ANDROID_LIBDL,
        libc::dlerror as usize,
    )?;
    let strlen_remote = find_remote_procedure(
        pid as i32,
        ANDROID_LIBC,
        libc::strlen as usize,
    )?;

    let tracer = TracedProcess::attach(RawProcess::new(pid as i32))?;
    std::io::stdout().write(b"1")?;
    info!("attached process {}", pid);
    std::io::stdout().flush()?;
    let frame = tracer.next_frame()?;
    debug!("Waited for a frame");

    // Map a buffer in the remote process
    debug!("remote mmap addr {:x}", mmap_remote);
    let mmap_params: [usize; 6] = [
        0,
        0x1000,
        (libc::PROT_READ | libc::PROT_WRITE) as usize,
        (libc::MAP_ANONYMOUS | libc::MAP_PRIVATE) as usize,
        0,
        0,
    ];
    let mut arr: Vec<u8> = Vec::new();
    for p in mmap_params {
        arr.extend_from_slice(&p.to_le_bytes());
    }
    arr.as_slice();
    let (regs, mut frame) = frame.invoke_remote(
        mmap_remote,
        libc_base,
        &mmap_params,
    )?;
    let buf_addr = regs.return_value();
    debug!("remote stopped at addr {:x}", regs.program_counter());
    if regs.program_counter() != libc_base {
        let mut data = std::mem::MaybeUninit::<libc::siginfo_t>::uninit();
        let siginfo = unsafe {
            libc::ptrace(libc::PTRACE_GETSIGINFO, pid, 0, &data);
            data.assume_init()
        };
        bail!("stopped at unexpected addr {:x} signo {} si_code {} si_addr {:?}", regs.program_counter(), siginfo.si_signo, siginfo.si_code, unsafe { siginfo.si_addr() });
    }
    if buf_addr == usize::MAX {
        debug!("errno remote {:x}", errno_remote);
        let (regs, mut frame) = frame.invoke_remote(
            errno_remote,
            libc_base,
            & [],
        )?;
        debug!("errno called");
        if regs.program_counter() != libc_base {
            bail!("stopped at unexpected addr {:x} when getting errno", regs.program_counter());
        }
        let err_addr = regs.return_value();
        let mut buf = [0u8; 4];
        frame.read_memory_mut(err_addr, &mut buf)?;
        let err = i32::from_le_bytes(buf);
        bail!("remote failed with {}", err);
    }
    debug!("Buffer addr: {:x}", buf_addr);

    // Load zygisk into remote process=
    frame.write_memory(buf_addr, zygisk_lib.as_bytes_with_nul())?;
    let (regs, mut frame) = frame.invoke_remote(
        dlopen_remote,
        libc_base,
        &[buf_addr, libc::RTLD_NOW as usize],
    )?;
    let handle = regs.return_value();
    debug!("Load zygisk into remote process: {:x}", handle);
    if regs.program_counter() != libc_base {
        let mut data = std::mem::MaybeUninit::<libc::siginfo_t>::uninit();
        let siginfo = unsafe {
            libc::ptrace(libc::PTRACE_GETSIGINFO, pid, 0, &data);
            data.assume_init()
        };
        bail!("stopped at unexpected addr {:x} signo {} si_code {} si_addr {:?}", regs.program_counter(), siginfo.si_signo, siginfo.si_code, unsafe { siginfo.si_addr() });
    }
    if handle == 0 {
        debug!("got handle 0");
        let (regs, mut frame) = frame.invoke_remote(
            dlerror_remote,
            libc_base,
            & [],
        )?;
        let err_addr = regs.return_value();
        if err_addr == 0 {
            bail!("dlerror err addr 0");
        }
        debug!("err addr {:x}", err_addr);
        let (regs, mut frame) = frame.invoke_remote(
            strlen_remote,
            libc_base,
            & [err_addr],
        )?;
        let len = regs.return_value();
        if len == 0 {
            bail!("dlerror len 0");
        }
        debug!("err len {}", len);
        let mut buf = vec![0u8; len];
        frame.read_memory_mut(err_addr, buf.as_mut_slice())?;
        bail!("err {:?}", buf);
    }

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
}

pub fn main() -> Result<()> {
    info!("Start zygisk ptrace");
    let args: Vec<String> = env::args().collect();
    let pid = args[1].parse::<u32>().unwrap();
    info!("ptracing {} pid {}", lp_select!("zygote32", "zygote64"), pid);
    ptrace_zygote(pid)?;
    Ok(())
}
