#include <lsplt.hpp>

#include "logging.h"

extern "C"
void entry(void* handle) {
    LOGD("Injector handle: %p", handle);

}
