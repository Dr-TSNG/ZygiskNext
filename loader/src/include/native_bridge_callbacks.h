#pragma once

#include <android/api-level.h>
#include <cstdint>

template<unsigned>
struct NativeBridgeCallbacks;

template<>
struct NativeBridgeCallbacks<__ANDROID_API_Q__> {
    [[maybe_unused]] uint32_t version;
    [[maybe_unused]] void *initialize;
    [[maybe_unused]] void *loadLibrary;
    [[maybe_unused]] void *getTrampoline;
    [[maybe_unused]] void *isSupported;
    [[maybe_unused]] void *getAppEnv;
    [[maybe_unused]] void *isCompatibleWith;
    [[maybe_unused]] void *getSignalHandler;
    [[maybe_unused]] void *unloadLibrary;
    [[maybe_unused]] void *getError;
    [[maybe_unused]] void *isPathSupported;
    [[maybe_unused]] void *initAnonymousNamespace;
    [[maybe_unused]] void *createNamespace;
    [[maybe_unused]] void *linkNamespaces;
    [[maybe_unused]] void *loadLibraryExt;
    [[maybe_unused]] void *getVendorNamespace;
    [[maybe_unused]] void *getExportedNamespace;
};

template<>
struct NativeBridgeCallbacks<__ANDROID_API_R__> : NativeBridgeCallbacks<__ANDROID_API_Q__> {
    [[maybe_unused]] void *preZygoteFork;
};
