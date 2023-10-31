#include <sys/ptrace.h>
#include <unistd.h>
#include <sys/uio.h>
#include <sys/auxv.h>
#include <elf.h>
#include <link.h>
#include <vector>
#include <string>
#include <sys/mman.h>
#include <sys/wait.h>
#include <cstdlib>
#include <cstdio>
#include <dlfcn.h>
#include <signal.h>
#include <sys/system_properties.h>
#include <string>
#include "utils.hpp"

bool inject_on_main(int pid, const char *lib_path) {
    // parsing KernelArgumentBlock
    // https://cs.android.com/android/platform/superproject/main/+/main:bionic/libc/private/KernelArgumentBlock.h;l=30;drc=6d1ee77ee32220e4202c3066f7e1f69572967ad8
    struct user_regs_struct regs{}, backup{};
    auto map = MapInfo::Scan(std::to_string(pid));
    if (!get_regs(pid, regs)) return false;
    auto arg = reinterpret_cast<uintptr_t *>(regs.REG_SP);
    LOGD("kernel argument %p %s", arg, get_addr_mem_region(map, arg).c_str());
    int argc;
    auto argv = reinterpret_cast<char **>(reinterpret_cast<uintptr_t *>(arg) + 1);
    LOGD("argv %p", argv);
    read_proc(pid, arg, &argc, sizeof(argc));
    LOGD("argc %d", argc);
    auto envp = argv + argc + 1;
    LOGD("envp %p", envp);
    auto p = envp;
    while (true) {
        uintptr_t *buf;
        read_proc(pid, (uintptr_t *) p, &buf, sizeof(buf));
        if (buf != nullptr) ++p;
        else break;
    }
    ++p;
    auto auxv = reinterpret_cast<ElfW(auxv_t) *>(p);
    LOGD("auxv %p %s", auxv, get_addr_mem_region(map, auxv).c_str());
    auto v = auxv;
    void *entry_addr = nullptr;
    void *addr_of_entry_addr = nullptr;
    while (true) {
        ElfW(auxv_t) buf;
        read_proc(pid, (uintptr_t *) v, &buf, sizeof(buf));
        if (buf.a_type == AT_ENTRY) {
            entry_addr = reinterpret_cast<void *>(buf.a_un.a_val);
            addr_of_entry_addr = reinterpret_cast<char *>(v) + offsetof(ElfW(auxv_t), a_un);
            LOGD("entry address %p %s (v=%p, entry_addr=%p)", entry_addr,
                 get_addr_mem_region(map, entry_addr).c_str(), v, addr_of_entry_addr);
            break;
        }
        if (buf.a_type == AT_NULL) break;
        v++;
    }
    if (entry_addr == nullptr) {
        LOGE("failed to get entry");
        return false;
    }

    // Replace the program entry with an invalid address
    // For arm32 compatibility, we set the last bit to the same as the entry address
    uintptr_t break_addr = (-0x05ec1cff & ~1) | ((uintptr_t) entry_addr & 1);
    if (!write_proc(pid, (uintptr_t *) addr_of_entry_addr, &break_addr, sizeof(break_addr))) return false;
    ptrace(PTRACE_CONT, pid, 0, 0);
    int status;
    if (waitpid(pid, &status, __WALL) == -1) {
        PLOGE("wait");
        return false;
    }
    if (WIFSTOPPED(status) && WSTOPSIG(status) == SIGSEGV) {
        if (!get_regs(pid, regs)) return false;
        if (regs.REG_IP != break_addr) {
            LOGE("stopped at unknown addr %p", (void *) regs.REG_IP);
            return false;
        }
        // The linker has been initialized now, we can do dlopen
        LOGD("stopped at entry");

        // restore entry address
        if (!write_proc(pid, (uintptr_t *) addr_of_entry_addr, &entry_addr, sizeof(entry_addr))) return false;

        // backup registers
        memcpy(&backup, &regs, sizeof(regs));
        map = MapInfo::Scan(std::to_string(pid));
        auto local_map = MapInfo::Scan();
        auto libc_base = find_module_base(map, "libc.so");

        // call dlopen
        auto dlopen_addr = find_func_addr(local_map, map, "libdl.so", "dlopen");
        if (dlopen_addr == nullptr) return false;
        std::vector<long> args;
        auto str = push_string(pid, regs, lib_path);
        args.clear();
        args.push_back((long) str);
        args.push_back((long) RTLD_NOW);
        auto remote_handle = remote_call(pid, regs, (uintptr_t) dlopen_addr, (uintptr_t) libc_base, args);
        LOGD("remote handle %p", (void *) remote_handle);
        if (remote_handle == 0) {
            LOGE("handle is null");
            return false;
        }

        // call dlsym(handle, "entry")
        auto dlsym_addr = find_func_addr(local_map, map, "libdl.so", "dlsym");
        if (dlsym_addr == nullptr) return false;
        args.clear();
        str = push_string(pid, regs, "entry");
        args.push_back(remote_handle);
        args.push_back((long) str);
        auto injector_entry = remote_call(pid, regs, (uintptr_t) dlsym_addr, (uintptr_t) libc_base, args);
        LOGD("injector entry %p", (void*) injector_entry);
        if (injector_entry == 0) {
            LOGE("injector entry is null");
            return false;
        }

        // call injector entry(handle)
        args.clear();
        args.push_back(remote_handle);
        remote_call(pid, regs, injector_entry, (uintptr_t) libc_base, args);

        // reset pc to entry
        backup.REG_IP = (long) entry_addr;
        LOGD("invoke entry");
        // restore registers
        if (!set_regs(pid, backup)) return false;

        return true;

        /*
        ptrace(PTRACE_CONT, pid, 0, 0);
        waitpid(pid, &status, __WALL);
        if (WIFSTOPPED(status)) {
            siginfo_t siginfo;
            ptrace(PTRACE_GETSIGINFO, pid, 0, &siginfo);
            LOGD("process stopped by signal %d %s si_code=%d si_addr=%p", WSTOPSIG(status),
                 strsignal(WSTOPSIG(status)), siginfo.si_code, siginfo.si_addr);
            pause();
        } else {
            LOGD("other reason %d", status);
        }*/
    } else {
        LOGE("stopped by other reason: %d", status);
    }
    return false;
}

#define STOPPED_WITH(sig, event) (WIFSTOPPED(status) && WSTOPSIG(status) == (sig) && (status >> 16) == (event))

void trace_zygote_main(int pid) {
    int status;
    LOGI("tracing %d (tracer %d)", pid, getpid());
    if (ptrace(PTRACE_SEIZE, pid, 0, PTRACE_O_TRACEEXEC) == -1) {
        PLOGE("seize");
        return;
    }
    wait_pid(pid, &status, __WALL);
    if (STOPPED_WITH(SIGSTOP, PTRACE_EVENT_STOP)) {
        // if SIGSTOP is delivered before we seized it
        LOGD("process is already stopped");
        kill(pid, SIGCONT);
        ptrace(PTRACE_CONT, pid, 0, 0);
        wait_pid(pid, &status, __WALL);
        if (STOPPED_WITH(SIGTRAP, PTRACE_EVENT_STOP)) {
            ptrace(PTRACE_CONT, pid, 0, 0);
            wait_pid(pid, &status, __WALL);
            if (STOPPED_WITH(SIGCONT, 0)) {
                LOGD("received SIGCONT");
                ptrace(PTRACE_CONT, pid, 0, 0);
            }
        } else {
            LOGE("unknown state %s, not SIGTRAP + EVENT_STOP", parse_status(status).c_str());
        }
    } else if (STOPPED_WITH(SIGSTOP, 0)) {
        // if SIGSTOP is delivered after we seized it
        LOGD("process received SIGSTOP, suppress");
        ptrace(PTRACE_CONT, pid, 0, 0);
    } else {
        LOGE("unknown state %s, neither EVENT_STOP nor SIGSTOP", parse_status(status).c_str());
        exit(1);
    }
    wait_pid(pid, &status, __WALL);
    // enter the app_process
    if (STOPPED_WITH(SIGTRAP, PTRACE_EVENT_EXEC)) {
        LOGD("app_process exec-ed");
        if (!inject_on_main(pid, "/dev/zygisk/lib" LP_SELECT("", "64") "/libzygisk.so")) {
            LOGE("failed to inject");
            exit(1);
        }
    } else {
        LOGE("unknown status %d", status);
        exit(1);
    }
    ptrace(PTRACE_DETACH, pid, 0, 0);
}
