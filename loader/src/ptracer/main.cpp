#include <cstdio>
#include <cstdlib>
#include <string_view>
#include <unistd.h>

#include "main.hpp"
#include "utils.hpp"

using namespace std::string_view_literals;

int main(int argc, char **argv) {
    if (argc >= 2 && argv[1] == "prop_monitor"sv) {
        if (access("/system/lib" LP_SELECT("", "64"), R_OK) == 0) prop_monitor_main();
    } else if (argc >= 3 && argv[1] == "trace-zygote"sv) {
        auto pid = strtol(argv[2], nullptr, 0);
        trace_zygote_main(pid);
    } else {
        if (argc >= 2) LOGE("unknown command %s", argv[1]);
        else LOGE("no command specified");
    }
    return 0;
}
