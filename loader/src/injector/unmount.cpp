#include <mntent.h>
#include <sys/mount.h>

#include "files.hpp"
#include "logging.h"
#include "misc.hpp"
#include "zygisk.hpp"

using namespace std::string_view_literals;

namespace {
    constexpr auto MODULE_DIR = "/data/adb/modules";

    struct overlay_backup {
        std::string target;
        std::string vfs_option;
        std::string fs_option;
    };

    void lazy_unmount(const char* mountpoint) {
        if (umount2(mountpoint, MNT_DETACH) != -1) {
            LOGD("Unmounted (%s)", mountpoint);
        } else {
            PLOGE("Unmount (%s)", mountpoint);
        }
    }
}

#define PARSE_OPT(name, flag)   \
    if (opt == (name)) {        \
        flags |= (flag);        \
        return true;            \
    }

void revert_unmount_ksu() {
    std::string ksu_loop;
    std::vector<std::string> targets;
    std::list<overlay_backup> backups;

    // Unmount ksu module dir last
    targets.emplace_back(MODULE_DIR);

    for (auto& info: parse_mount_info("self")) {
        if (info.target == MODULE_DIR) {
            ksu_loop = info.source;
            continue;
        }
        // Unmount everything on /data/adb except ksu module dir
        if (info.target.starts_with("/data/adb")) {
            targets.emplace_back(info.target);
        }
        // Unmount ksu overlays
        if (info.type == "overlay") {
            if (str_contains(info.fs_option, MODULE_DIR)) {
                targets.emplace_back(info.target);
            } else {
                auto backup = overlay_backup{
                        .target = info.target,
                        .vfs_option = info.vfs_option,
                        .fs_option = info.fs_option,
                };
                backups.emplace_back(backup);
            }
        }
    }
    for (auto& info: parse_mount_info("self")) {
        // Unmount everything from ksu loop except ksu module dir
        if (info.source == ksu_loop && info.target != MODULE_DIR) {
            targets.emplace_back(info.target);
        }
    }

    // Do unmount
    for (auto& s: reversed(targets)) {
        lazy_unmount(s.data());
    }

    // Affirm unmounted system overlays
    for (auto& info: parse_mount_info("self")) {
        if (info.type == "overlay") {
            backups.remove_if([&](overlay_backup& mnt) {
                return mnt.target == info.target && mnt.fs_option == info.fs_option;
            });
        }
    }

    // Restore system overlays
    for (auto& mnt: backups) {
        auto opts = split_str(mnt.vfs_option, ",");
        opts.splice(opts.end(), split_str(mnt.fs_option, ","));
        unsigned long flags = 0;
        opts.remove_if([&](auto& opt) {
            PARSE_OPT(MNTOPT_RO, MS_RDONLY)
            PARSE_OPT(MNTOPT_NOSUID, MS_NOSUID)
            PARSE_OPT("relatime", MS_RELATIME)
            return false;
        });
        auto mnt_data = join_str(opts, ",");
        if (mount("overlay", mnt.target.data(), "overlay", flags, mnt_data.data()) != -1) {
            LOGD("Remounted (%s)", mnt.target.data());
        } else {
            PLOGE("Remount (%s, %s)", mnt.target.data(), mnt.fs_option.data());
        }
    }
}

void revert_unmount_magisk() {
    std::vector<std::string> targets;

    // Unmount dummy skeletons and MAGISKTMP
    // since mirror nodes are always mounted under skeleton, we don't have to specifically unmount
    for (auto& info: parse_mount_info("self")) {
        if (info.source == "magisk" || info.source == "worker" || // magisktmp tmpfs
            info.root.starts_with("/adb/modules")) { // bind mount from data partition
            targets.push_back(info.target);
        }
    }

    for (auto& s: reversed(targets)) {
        lazy_unmount(s.data());
    }
}
