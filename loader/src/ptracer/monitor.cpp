#include <sys/system_properties.h>
#include <unistd.h>

#include "main.hpp"
#include "utils.hpp"

void prop_monitor_main() {
    LOGI("prop monitor started");
    // if service is not running, pid = ""
    auto name = "init.svc_debug_pid." LP_SELECT("zygote_secondary", "zygote"); // argv[1];
    LOGI("start monitoring %s", name);
    auto prop = __system_property_find(name);
    if (prop == nullptr) {
        __system_property_set(name, "");
        prop = __system_property_find(name);
        if (prop == nullptr) {
            LOGE("failed to create prop");
            exit(1);
        }
    }
    char val[PROP_VALUE_MAX];
    uint32_t new_serial = 0;
    while (true) {
        __system_property_wait(prop, new_serial, &new_serial, nullptr);
        __system_property_get(name, val);
        LOGD("%s(%u): %s\n", name, new_serial, val);
        auto pid = strtol(val, nullptr, 0);
        if (pid != 0) {
            LOGD("start ptrace %ld", pid);
            if (fork_dont_care() == 0) {
                execl("/proc/self/exe", "zygisk-ptracer", "trace-zygote", val, nullptr);
                PLOGE("failed to exec");
                exit(1);
            }
        }
    }
}
