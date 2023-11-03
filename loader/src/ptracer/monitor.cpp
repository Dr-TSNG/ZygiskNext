#include <sys/system_properties.h>
#include <unistd.h>
#include <sys/stat.h>

#include "main.hpp"
#include "utils.hpp"
#include "files.hpp"
#include "misc.hpp"

using namespace std::string_view_literals;

int find_zygote() {
    LOGD("find zygote");
    auto sockets = ScanUnixSockets();
    auto dir = xopen_dir("/proc");
    for (dirent *entry; (entry = readdir(dir.get()));) {
        auto pid = parse_int(entry->d_name);
        char comm[18];
        char state;
        if (pid == -1 || pid == 1) continue;
        auto stat_file = xopen_file((std::string("/proc/") + std::to_string(pid) + "/stat").c_str(), "r");
        if (stat_file == nullptr) continue;
        if (fscanf(stat_file.get(), "%*d %17s %c", comm, &state) != 2
            || comm != "(init)"sv
            || state != 'T') {
            continue;
        }
        LOGD("%d is stopped init", pid);
        auto fd_dir = xopen_dir((std::string("/proc/") + std::to_string(pid) + "/fd").c_str());
        if (fd_dir == nullptr) continue;
        for (dirent *fd_entry; (fd_entry = readdir(fd_dir.get()));) {
            if (fd_entry->d_name == "."sv || fd_entry->d_name == ".."sv) continue;
            struct stat st{};
            if (stat((std::string("/proc/") + std::to_string(pid) + "/fd/" + fd_entry->d_name).c_str(), &st) == -1) {
                PLOGE("stat /proc/%d/fd/%s", pid, fd_entry->d_name);
                continue;
            }
            if ((st.st_mode & S_IFSOCK) == 0) continue;
            auto it = sockets.find(st.st_ino);
            if (it != sockets.end() && it->second == LP_SELECT("/dev/socket/zygote_secondary", "/dev/socket/zygote")) {
                LOGD("%d is zygote", pid);
                return pid;
            }
        }
    }
    return -1;
}

int wait_for_zygote() {
    auto name = "init.svc." LP_SELECT("zygote_secondary", "zygote");
    auto prop = __system_property_find(name);
    if (prop == nullptr) {
        __system_property_set(name, "stopped");
        prop = __system_property_find(name);
        if (prop == nullptr) {
            LOGE("failed to create prop");
            exit(1);
        }
    }
    std::string last_state = "running";
    char val[PROP_VALUE_MAX];
    uint32_t new_serial = 0;
    while (true) {
        __system_property_wait(prop, new_serial, &new_serial, nullptr);
        __system_property_get(name, val);
        LOGD("%s(%u): %s\n", name, new_serial, val);
        if (val != last_state && val == "running"sv) {
            LOGI("zygote is running, find zygote");
            int pid = -1;
            for (int i = 0; i < 5; i++) {
                pid = find_zygote();
                if (pid != -1) break;
                else {
                    LOGW("could not find zygote, wait 1s");
                    sleep(1);
                }
            }
            if (pid == -1) {
                LOGE("failed to find zygote");
                exit(1);
            }
            return pid;
        }
        last_state = val;
    }
}
