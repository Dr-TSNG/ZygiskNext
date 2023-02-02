#include <sys/mount.h>

#include "logging.h"
#include "misc.hpp"
#include "zygisk.hpp"

using namespace std::string_view_literals;

static void lazy_unmount(const char* mountpoint) {
    if (umount2(mountpoint, MNT_DETACH) != -1)
        LOGD("Unmounted (%s)", mountpoint);
}

#define OVERLAY_MNT(dir) (mentry->mnt_type == "overlay"sv && std::string_view(mentry->mnt_dir).starts_with("/" #dir))

void revert_unmount() {
    parse_mnt("/proc/self/mounts", [](mntent* mentry) {
        if (OVERLAY_MNT("system") || OVERLAY_MNT("vendor") || OVERLAY_MNT("product") || OVERLAY_MNT("system_ext")) {
            lazy_unmount(mentry->mnt_fsname);
        }
        return true;
    });
}
