#pragma once

#include <dirent.h>
#include <functional>
#include <memory>
#include <mntent.h>
#include <pthread.h>
#include <stdio.h>
#include <string_view>

#include "logging.h"

#define DISALLOW_COPY_AND_MOVE(clazz) \
clazz(const clazz &) = delete; \
clazz(clazz &&) = delete;

class mutex_guard {
    DISALLOW_COPY_AND_MOVE(mutex_guard)
public:
    explicit mutex_guard(pthread_mutex_t &m): mutex(&m) {
        pthread_mutex_lock(mutex);
    }
    void unlock() {
        pthread_mutex_unlock(mutex);
        mutex = nullptr;
    }
    ~mutex_guard() {
        if (mutex) pthread_mutex_unlock(mutex);
    }
private:
    pthread_mutex_t *mutex;
};

using thread_entry = void *(*)(void *);
int new_daemon_thread(thread_entry entry, void *arg);

template<typename T, typename Impl>
class stateless_allocator {
public:
    using value_type = T;
    T *allocate(size_t num) { return static_cast<T*>(Impl::allocate(sizeof(T) * num)); }
    void deallocate(T *ptr, size_t num) { Impl::deallocate(ptr, sizeof(T) * num); }
    stateless_allocator()                           = default;
    stateless_allocator(const stateless_allocator&) = default;
    stateless_allocator(stateless_allocator&&)      = default;
    template <typename U>
    stateless_allocator(const stateless_allocator<U, Impl>&) {}
    bool operator==(const stateless_allocator&) { return true; }
    bool operator!=(const stateless_allocator&) { return false; }
};

using sFILE = std::unique_ptr<FILE, decltype(&fclose)>;
using sDIR = std::unique_ptr<DIR, decltype(&closedir)>;
sDIR make_dir(DIR *dp);
sFILE make_file(FILE *fp);

static inline sDIR open_dir(const char *path) {
    return make_dir(opendir(path));
}

static inline sDIR xopen_dir(const char *path) {
    return make_dir(opendir(path));
}

static inline sDIR xopen_dir(int dirfd) {
    return make_dir(fdopendir(dirfd));
}

static inline sFILE open_file(const char *path, const char *mode) {
    return make_file(fopen(path, mode));
}

static inline sFILE xopen_file(const char *path, const char *mode) {
    return make_file(fopen(path, mode));
}

static inline sFILE xopen_file(int fd, const char *mode) {
    return make_file(fdopen(fd, mode));
}

template<class T>
static inline void default_new(T *&p) { p = new T(); }

template<class T>
static inline void default_new(std::unique_ptr<T> &p) { p.reset(new T()); }

struct StringCmp {
    using is_transparent = void;
    bool operator()(std::string_view a, std::string_view b) const { return a < b; }
};

/*
 * Bionic's atoi runs through strtol().
 * Use our own implementation for faster conversion.
 */
int parse_int(std::string_view s);

void parse_mnt(const char* file, const std::function<bool(mntent*)>& fn);

template <typename T>
static inline T align_to(T v, int a) {
    static_assert(std::is_integral<T>::value);
    return (v + a - 1) / a * a;
}
