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
    if (argc >= 2 && argv[1] == "monitor"sv) {
        init_monitor();
        return 0;
    } else if (argc >= 3 && argv[1] == "trace"sv) {
        if (argc >= 4 && argv[3] == "--restart"sv) {
            constexpr auto companion = "./bin/zygisk-cp" LP_SELECT("32", "64");
            zygiskd::Init(getenv(MAGIC_PATH_ENV));
            zygiskd::ZygoteRestart();
            if (fork_dont_care() == 0) {
                LOGI("creating new zygisk companion");
                execl(companion, basename(companion), nullptr);
                PLOGE("failed to exec zygisk companion");
                exit(1);
            }
        }
        auto pid = strtol(argv[2], 0, 0);
        if (!trace_zygote(pid)) {
            kill(pid, SIGKILL);
            return 1;
        }
        return 0;
    } else if (argc >= 3 && argv[1] == "ctl"sv) {
        if (argv[2] == "start"sv) {
            send_control_command(START);
        } else if (argv[2] == "stop"sv) {
            send_control_command(STOP);
        } else if (argv[2] == "exit"sv) {
            send_control_command(EXIT);
        } else {
            printf("Usage: %s ctl start|stop|exit\n", argv[0]);
            return 1;
        }
        return 0;
    } else {
        LOGE("usage: %s monitor | trace <pid> | ctl <command>", argv[0]);
        return 1;
    }
}
