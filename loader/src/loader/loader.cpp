#include <string_view>
#include <sys/system_properties.h>
#include <unistd.h>
#include <array>

#include "daemon.h"
#include "dl.h"
#include "logging.h"
#include "native_bridge_callbacks.h"

extern "C" [[gnu::visibility("default")]]
uint8_t NativeBridgeItf[sizeof(NativeBridgeCallbacks<__ANDROID_API_R__>) * 2]{0};

namespace {
    constexpr auto kZygoteProcesses = {"zygote", "zygote32", "zygote64", "usap32", "usap64"};
    constexpr auto kInjector = "/system/" LP_SELECT("lib", "lib64") "/libinjector.so";

    void* sOriginalBridge = nullptr;
}

__used __attribute__((destructor))
void Destructor() {
    if (sOriginalBridge) {
        dlclose(sOriginalBridge);
    }
}

__used __attribute__((constructor))
void Constructor() {
    if (getuid() != 0) {
        return;
    }

    std::string_view cmdline = getprogname();
    if (std::none_of(
            kZygoteProcesses.begin(), kZygoteProcesses.end(),
            [&](const char* p) { return cmdline == p; }
    )) {
        LOGW("Not started as zygote (cmdline=%s)", cmdline.data());
        return;
    }

    std::string native_bridge;
    do {
        LOGD("Ping heartbeat");
        if (!zygiskd::PingHeartbeat()) break;

        LOGI("Read native bridge");
        native_bridge = zygiskd::ReadNativeBridge();

        LOGI("Load injector");
        auto handle = DlopenExt(kInjector, RTLD_NOW);
        if (handle == nullptr) {
            LOGE("Failed to dlopen injector: %s", dlerror());
            break;
        }
        auto entry = dlsym(handle, "entry");
        if (entry == nullptr) {
            LOGE("Failed to dlsym injector entry: %s", dlerror());
            dlclose(handle);
            break;
        }
        reinterpret_cast<void (*)(void*)>(entry)(handle);
    } while (false);

    if (native_bridge.empty() || native_bridge == "0") return;
    LOGI("Load original native bridge: %s", native_bridge.data());
    sOriginalBridge = dlopen(native_bridge.data(), RTLD_NOW);
    if (sOriginalBridge == nullptr) {
        LOGE("dlopen failed: %s", dlerror());
        return;
    }

    auto* original_native_bridge_itf = dlsym(sOriginalBridge, "NativeBridgeItf");
    if (original_native_bridge_itf == nullptr) {
        LOGE("dlsym failed: %s", dlerror());
        return;
    }

    long sdk = 0;
    char value[PROP_VALUE_MAX + 1];
    if (__system_property_get("ro.build.version.sdk", value) > 0) {
        sdk = strtol(value, nullptr, 10);
    }

    auto callbacks_size = 0;
    if (sdk >= __ANDROID_API_R__) {
        callbacks_size = sizeof(NativeBridgeCallbacks<__ANDROID_API_R__>);
    } else if (sdk == __ANDROID_API_Q__) {
        callbacks_size = sizeof(NativeBridgeCallbacks<__ANDROID_API_Q__>);
    }

    memcpy(NativeBridgeItf, original_native_bridge_itf, callbacks_size);
}
