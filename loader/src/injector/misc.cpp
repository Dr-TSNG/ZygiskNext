#include "misc.hpp"

int new_daemon_thread(thread_entry entry, void *arg) {
    pthread_t thread;
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);
    errno = pthread_create(&thread, &attr, entry, arg);
    if (errno) {
        PLOGE("pthread_create");
    }
    return errno;
}

int parse_int(std::string_view s) {
    int val = 0;
    for (char c : s) {
        if (!c) break;
        if (c > '9' || c < '0')
            return -1;
        val = val * 10 + c - '0';
    }
    return val;
}

void parse_mnt(const char* file, const std::function<bool(mntent*)>& fn) {
    auto fp = sFILE(setmntent(file, "re"), endmntent);
    if (fp) {
        mntent mentry{};
        char buf[PATH_MAX];
        while (getmntent_r(fp.get(), &mentry, buf, sizeof(buf))) {
            if (!fn(&mentry))
                break;
        }
    }
}

sDIR make_dir(DIR *dp) {
    return sDIR(dp, [](DIR *dp){ return dp ? closedir(dp) : 1; });
}

sFILE make_file(FILE *fp) {
    return sFILE(fp, [](FILE *fp){ return fp ? fclose(fp) : 1; });
}
