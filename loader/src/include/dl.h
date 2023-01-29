#pragma once

#include <dlfcn.h>

void *DlopenExt(const char *path, int flags);

void *DlopenMem(int memfd, int flags);
