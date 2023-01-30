#include <linux/un.h>
#include <sys/socket.h>
#include <unistd.h>

#include "daemon.h"
#include "dl.h"
#include "socket_utils.h"

namespace zygiskd {

    UniqueFd Connect(uint8_t retry) {
        UniqueFd fd = socket(PF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0);
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
        return -1;
    }

    bool PingHeartbeat() {
        LOGD("Daemon socket: %s", kZygiskSocket);
        auto fd = Connect(5);
        if (fd == -1) {
            PLOGE("Connect to zygiskd");
            return false;
        }
        socket_utils::write_u8(fd, (uint8_t) SocketAction::PingHeartBeat);
        return true;
    }

    std::string ReadNativeBridge() {
        auto fd = Connect(1);
        if (fd == -1) {
            PLOGE("ReadNativeBridge");
            return "";
        }
        socket_utils::write_u8(fd, (uint8_t) SocketAction::ReadNativeBridge);
        return socket_utils::read_string(fd);
    }

    std::vector<Module> ReadModules() {
        std::vector<Module> modules;
        auto fd = Connect(1);
        if (fd == -1) {
            PLOGE("ReadModules");
            return modules;
        }
        socket_utils::write_u8(fd, (uint8_t) SocketAction::ReadModules);
        size_t len = socket_utils::read_usize(fd);
        for (size_t i = 0; i < len; i++) {
            std::string name = socket_utils::read_string(fd);
            UniqueFd module_fd = socket_utils::recv_fd(fd);
            auto handle = DlopenMem(module_fd, RTLD_NOW);
            if (handle == nullptr) {
                LOGW("Failed to dlopen module %s: %s", name.data(), dlerror());
                continue;
            }
            modules.emplace_back(name, handle);
        }
        return modules;
    }
}
