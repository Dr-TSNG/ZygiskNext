#include <linux/un.h>
#include <sys/socket.h>
#include <unistd.h>

#include "daemon.h"
#include "dl.h"
#include "socket_utils.h"

namespace zygiskd {

    bool sMagicRead = false;
    static std::string sSocketName;

    void ReadMagic() {
        sMagicRead = true;
        char magic[PATH_MAX]{0};
        auto fp = fopen(kZygiskMagic, "r");
        if (fp == nullptr) {
            PLOGE("Open magic file");
            return;
        }
        fgets(magic, PATH_MAX, fp);
        fclose(fp);
        sSocketName.append(LP_SELECT("zygiskd32", "zygiskd64")).append(magic);
        LOGD("Socket name: %s", sSocketName.data());
    }

    int Connect(uint8_t retry) {
        if (!sMagicRead) ReadMagic();
        int fd = socket(PF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0);
        struct sockaddr_un addr{
                .sun_family = AF_UNIX,
                .sun_path={0},
        };
        strcpy(addr.sun_path + 1, sSocketName.data());
        socklen_t socklen = sizeof(sa_family_t) + strlen(addr.sun_path + 1) + 1;

        while (retry--) {
            int r = connect(fd, reinterpret_cast<struct sockaddr*>(&addr), socklen);
            if (r == 0) return fd;
            LOGW("Retrying to connect to zygiskd, sleep 1s");
            sleep(1);
        }

        close(fd);
        return -1;
    }

    bool PingHeartbeat() {
        UniqueFd fd = Connect(5);
        if (fd == -1) {
            PLOGE("Connect to zygiskd");
            return false;
        }
        socket_utils::write_u8(fd, (uint8_t) SocketAction::PingHeartBeat);
        return true;
    }

    int RequestLogcatFd() {
        int fd = Connect(1);
        if (fd == -1) {
            PLOGE("RequestLogcatFd");
            return -1;
        }
        socket_utils::write_u8(fd, (uint8_t) SocketAction::RequestLogcatFd);
        return fd;
    }

    std::string ReadNativeBridge() {
        UniqueFd fd = Connect(1);
        if (fd == -1) {
            PLOGE("ReadNativeBridge");
            return "";
        }
        socket_utils::write_u8(fd, (uint8_t) SocketAction::ReadNativeBridge);
        return socket_utils::read_string(fd);
    }

    uint32_t GetProcessFlags(uid_t uid) {
        UniqueFd fd = Connect(1);
        if (fd == -1) {
            PLOGE("GetProcessFlags");
            return 0;
        }
        socket_utils::write_u8(fd, (uint8_t) SocketAction::GetProcessFlags);
        socket_utils::write_u32(fd, uid);
        return socket_utils::read_u32(fd);
    }

    std::vector<Module> ReadModules() {
        std::vector<Module> modules;
        UniqueFd fd = Connect(1);
        if (fd == -1) {
            PLOGE("ReadModules");
            return modules;
        }
        socket_utils::write_u8(fd, (uint8_t) SocketAction::ReadModules);
        size_t len = socket_utils::read_usize(fd);
        for (size_t i = 0; i < len; i++) {
            std::string name = socket_utils::read_string(fd);
            int module_fd = socket_utils::recv_fd(fd);
            modules.emplace_back(name, module_fd);
        }
        return modules;
    }

    int ConnectCompanion(size_t index) {
        int fd = Connect(1);
        if (fd == -1) {
            PLOGE("ConnectCompanion");
            return -1;
        }
        socket_utils::write_u8(fd, (uint8_t) SocketAction::RequestCompanionSocket);
        socket_utils::write_usize(fd, index);
        if (socket_utils::read_u8(fd) == 1) {
            return fd;
        } else {
            return -1;
        }
    }

    int GetModuleDir(size_t index) {
        int fd = Connect(1);
        if (fd == -1) {
            PLOGE("GetModuleDir");
            return -1;
        }
        socket_utils::write_u8(fd, (uint8_t) SocketAction::GetModuleDir);
        socket_utils::write_usize(fd, index);
        return socket_utils::recv_fd(fd);
    }
}
