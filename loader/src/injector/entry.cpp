#include "logging.h"
#include "zygisk.hpp"
#include "module.hpp"

using namespace std;

void *self_handle = nullptr;
vector<zygiskd::Module> preloaded_modules;

[[gnu::destructor]] [[maybe_unused]]
static void zygisk_cleanup_wait() {
    if (self_handle) {
        // Wait 10us to make sure none of our code is executing
        timespec ts = { .tv_sec = 0, .tv_nsec = 10000L };
        nanosleep(&ts, nullptr);
    }
}

void preload_modules() {
    LOGI("Preload modules");
    preloaded_modules = zygiskd::ReadModules();
    for (auto& module : preloaded_modules) {
        LOGD("  Preloaded `%s`", module.name.data());
    }
}

extern "C" __used void entry(void *handle) {
    LOGD("Load injector successful");
    self_handle = handle;
    preload_modules();
    hook_functions();
}

// The following code runs in zygote/app process

static inline bool should_load_modules(uint32_t flags) {
    return (flags & UNMOUNT_MASK) != UNMOUNT_MASK &&
           (flags & PROCESS_IS_MAGISK_APP) != PROCESS_IS_MAGISK_APP;
}

