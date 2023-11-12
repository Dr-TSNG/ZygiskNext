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
        auto pid = strtol(argv[2], 0, 0);
        return !trace_zygote(pid);
    } else if (argc >= 3 && argv[1] == "ctl"sv) {
        // TODO
        return 1;
    } else {
        LOGE("usage: %s monitor | trace <pid> | ctl <command>", argv[0]);
        return 1;
    }
}
