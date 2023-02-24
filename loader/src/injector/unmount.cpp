#include <sys/mount.h>

#include "logging.h"
#include "misc.hpp"
#include "zygisk.hpp"

using namespace std::string_view_literals;

static void lazy_unmount(const char* mountpoint) {
    if (umount2(mountpoint, MNT_DETACH) != -1) {
        LOGD("Unmounted (%s)", mountpoint);
    } else {
        LOGW("Failed to unmount: %s (%s)", strerror(errno), mountpoint);
    }
}

#define PARSE_OPT(name, flag)   \
    if (opt == name) {          \
        flags |= (flag);        \
        return true;            \
    }

void revert_unmount() {
    std::vector<std::string> targets;
    std::list<std::pair<std::string, std::string>> backups;

    targets.emplace_back("/data/adb/ksu/modules");
    parse_mnt("/proc/self/mounts", [&](mntent* mentry) {
        if (str_starts(mentry->mnt_fsname, "/data/adb/")) {
            targets.emplace_back(mentry->mnt_dir);
        }
        if (mentry->mnt_type == "overlay"sv) {
            if (str_contains(mentry->mnt_opts, "/data/adb/ksu/modules")) {
                targets.emplace_back(mentry->mnt_dir);
            } else {
                backups.emplace_back(mentry->mnt_dir, mentry->mnt_opts);
            }
        }
        return true;
    });

    for (auto& s: reversed(targets)) {
        lazy_unmount(s.data());
    }

    parse_mnt("/proc/self/mounts", [&](mntent* mentry) {
        if (mentry->mnt_type == "overlay"sv) {
            backups.remove_if([&](auto& mnt) {
                return mnt.first == mentry->mnt_dir && mnt.second == mentry->mnt_opts;
            });
        }
        return true;
    });

    for (auto& mnt: backups) {
        auto opts = split_str(mnt.second, ",");
        unsigned long flags = 0;
        opts.remove_if([&](auto& opt) {
            PARSE_OPT(MNTOPT_RO, MS_RDONLY)
            PARSE_OPT(MNTOPT_NOSUID, MS_NOSUID)
            PARSE_OPT("relatime", MS_RELATIME)
            return false;
        });
        auto mnt_data = join_str(opts, ",");
        if (mount("overlay", mnt.first.data(), "overlay", flags, mnt_data.data()) != -1) {
            LOGD("Remounted (%s)", mnt.first.data());
        } else {
            LOGW("Failed to remount: %s (%s, %s)", strerror(errno), mnt.first.data(), mnt_data.data());
        }
    }
}
