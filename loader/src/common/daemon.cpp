#include <linux/un.h>
#include <sys/socket.h>
#include <unistd.h>

#include "daemon.h"
#include "dl.h"
#include "socket_utils.h"

namespace zygiskd {

    int Connect(uint8_t retry) {
        int fd = socket(PF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0);
        struct sockaddr_un addr{
                .sun_family = AF_UNIX,
                .sun_path={0},
        };
        strncpy(addr.sun_path + 1, kZygiskSocket.data(), kZygiskSocket.size());
        socklen_t socklen = sizeof(sa_family_t) + strlen(addr.sun_path + 1) + 1;

        while (retry--) {
            int r = connect(fd, reinterpret_cast<struct sockaddr*>(&addr), socklen);
            if (r == 0) return fd;
            LOGW("retrying to connect to zygiskd, sleep 1s");
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
        if (socket_utils::read_u8(fd) == 1) {
            return fd;
        } else {
            return -1;
        }
    }
}
