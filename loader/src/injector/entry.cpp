#include "daemon.h"
#include "logging.h"
#include "zygisk.hpp"
#include "module.hpp"

using namespace std;

void *self_handle = nullptr;

extern "C" [[gnu::visibility("default")]]
void entry(void* handle) {
    LOGI("Zygisk library injected, version %s", ZKSU_VERSION);
    self_handle = handle;

    zygiskd::Init();
    if (!zygiskd::PingHeartbeat()) {
        LOGE("Zygisk daemon is not running");
        return;
    }

#ifdef NDEBUG
    logging::setfd(zygiskd::RequestLogcatFd());
#endif

    LOGI("Start hooking");
    hook_functions();
}
