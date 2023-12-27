#include <mntent.h>
#include <sys/mount.h>

#include "files.hpp"
#include "logging.h"
#include "misc.hpp"
#include "zygisk.hpp"

using namespace std::string_view_literals;

namespace {
    constexpr auto MODULE_DIR = "/data/adb/modules";
    constexpr auto KSU_OVERLAY_SOURCE = "KSU";
    constexpr auto ZYGISK_FUSE_SOURCE = "zygisk";
    const std::vector<std::string> KSU_PARTITIONS{"/system", "/vendor", "/product", "/system_ext", "/odm", "/oem"};

    void lazy_unmount(const char* mountpoint) {
        if (umount2(mountpoint, MNT_DETACH) != -1) {
            LOGD("Unmounted (%s)", mountpoint);
        } else {
#ifndef NDEBUG
            PLOGE("Unmount (%s)", mountpoint);
#endif
        }
    }
}

void revert_unmount_ksu() {
    std::string ksu_loop;
    std::vector<std::string> targets;

    // Unmount ksu module dir last
    targets.emplace_back(MODULE_DIR);

    for (auto& info: parse_mount_info("self")) {
        if (info.target == MODULE_DIR) {
            ksu_loop = info.source;
            continue;
        }
        // Unmount everything mounted to /data/adb
        if (info.target.starts_with("/data/adb")) {
            targets.emplace_back(info.target);
        }
        // Unmount ksu overlays
        if (info.type == "overlay"
            && info.source == KSU_OVERLAY_SOURCE
            && std::find(KSU_PARTITIONS.begin(), KSU_PARTITIONS.end(), info.target) != KSU_PARTITIONS.end()) {
            targets.emplace_back(info.target);
        }
        // Unmount fuse
        if (info.type == "fuse" && info.source == ZYGISK_FUSE_SOURCE) {
            targets.emplace_back(info.target);
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
}


void revert_unmount_kpatch() {
    std::vector<std::string> targets;

    targets.emplace_back(MODULE_DIR);

    for (auto& info: parse_mount_info("self")) {
        // Unmount everything mounted to /data/adb
        if (info.target.starts_with("/data/adb")) {
            targets.emplace_back(info.target);
        }
        // Unmount fuse
        if (info.type == "fuse" && info.source == ZYGISK_FUSE_SOURCE) {
            targets.emplace_back(info.target);
        }
    }
    // Do unmount
    for (auto& s: reversed(targets)) {
        lazy_unmount(s.data());
    }
}