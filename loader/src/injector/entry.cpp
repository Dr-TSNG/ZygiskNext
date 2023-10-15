#include "daemon.h"
#include "logging.h"
#include "zygisk.hpp"
#include "module.hpp"

using namespace std;

void *self_handle = nullptr;

extern "C" [[gnu::visibility("default")]]
void entry(void *handle) {
#ifdef NDEBUG
    logging::setfd(zygiskd::RequestLogcatFd());
#endif
    self_handle = handle;

    LOGD("Load injector successfully");
    hook_functions();
}
