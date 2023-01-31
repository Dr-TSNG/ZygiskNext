#pragma once

#include <string_view>
#include <string>
#include <vector>

#if defined(__LP64__)
# define LP_SELECT(lp32, lp64) lp64
#else
# define LP_SELECT(lp32, lp64) lp32
#endif
constexpr std::string_view kZygiskSocket = LP_SELECT("zygiskd32", "zygiskd64") "socket_placeholder";

class UniqueFd {
    using Fd = int;
public:
    UniqueFd() = default;

    UniqueFd(Fd fd) : fd_(fd) {}

    ~UniqueFd() { close(fd_); }

    // Disallow copy
    UniqueFd(const UniqueFd&) = delete;

    UniqueFd& operator=(const UniqueFd&) = delete;

    // Allow move
    UniqueFd(UniqueFd&& other) { std::swap(fd_, other.fd_); }

    UniqueFd& operator=(UniqueFd&& other) {
        std::swap(fd_, other.fd_);
        return *this;
    }

    // Implict cast to Fd
    operator const Fd&() const { return fd_; }

private:
    Fd fd_ = -1;
};

namespace zygiskd {

    struct Module {
        std::string name;
        void* handle;
        int id;

        inline explicit Module(int id, std::string name, void* handle)
                : name(name), handle(handle), id(id) {}
    };

    enum class SocketAction {
        PingHeartBeat,
        ReadNativeBridge,
        ReadModules,
        RequestCompanionSocket,
        GetModuleDir,
    };

    bool PingHeartbeat();

    std::string ReadNativeBridge();

    std::vector<Module> ReadModules();

    UniqueFd ConnectCompanion(size_t index);

    UniqueFd GetModuleDir(size_t index);
}
