#include <cstdio>
#include <cstdlib>
#include <string_view>
#include <unistd.h>
#include <fcntl.h>
#include <time.h>
#include <sys/system_properties.h>

#include "main.hpp"
#include "utils.hpp"
#include "daemon.h"

using namespace std::string_view_literals;

int main(int argc, char **argv) {
    if (access("/system/lib" LP_SELECT("", "64"), R_OK) != 0) return 1;
    auto lock_fd = open("/dev/zygisk/lock" LP_SELECT("32", "64"), O_CLOEXEC | O_CREAT, O_RDONLY);
    if (lock_fd == -1) {
        PLOGE("create lock file");
        return 1;
    }
    struct flock lock{
        .l_type = F_RDLCK,
        .l_whence = SEEK_SET,
        .l_start = 0,
        .l_len = 0
    };
    if (fcntl(lock_fd, F_SETLK, &lock) == -1) {
        PLOGE("set file lock");
        close(lock_fd);
        return 1;
    }
    LOGI("zygote monitor started");
    struct timespec last_launch_time { .tv_sec = 0, .tv_nsec = 0 }, ts;
    int launch_count = 0;
    bool first = true;
    while (true) {
        auto pid = wait_for_zygote();
        if (pid == -1) break;
        LOGI("inject zygote %d", pid);
        if (first) first = false;
        else {
            LOGI("notify zygisk companion restart");
            zygiskd::ZygoteRestart();
        }
        clock_gettime(CLOCK_MONOTONIC, &ts);
        auto delta = ts.tv_sec - last_launch_time.tv_sec;
        if (delta > 30) launch_count++;
        else launch_count = 0;
        if (launch_count >= 5) {
            LOGE("zygote crash too much times, stop");
            break;
        }
        memcpy(&last_launch_time, &ts, sizeof(struct timespec));
        if (!trace_zygote(pid)) {
            break;
        }
    }
    __system_property_set("ctl.sigstop_off", "zygote");
    __system_property_set("ctl.sigstop_off", "zygote_secondary");
    return 1;
}
