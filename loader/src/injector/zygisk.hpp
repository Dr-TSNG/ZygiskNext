#pragma once

#include <stdint.h>
#include <jni.h>
#include <vector>

#include "daemon.h"

extern void *self_handle;
extern std::vector<zygiskd::Module> preloaded_modules;

void hook_functions();
