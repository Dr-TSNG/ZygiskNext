#include <sys/mount.h>

#include "logging.h"
#include "misc.hpp"
#include "zygisk.hpp"

using namespace std::string_view_literals;

static void lazy_unmount(const char* mountpoint) {
    if (umount2(mountpoint, MNT_DETACH) != -1)
        LOGD("Unmounted (%s)", mountpoint);
}

void revert_unmount() {
    parse_mnt("/proc/self/mounts", [](mntent* mentry) {
        if (mentry->mnt_fsname == "/data/adb/ksu/modules"sv ||
            std::string_view(mentry->mnt_opts).find("/data/adb/ksu/modules") != std::string_view::npos) {
            lazy_unmount(mentry->mnt_fsname);
        }
        return true;
    });
}
