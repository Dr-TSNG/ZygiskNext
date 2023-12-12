#pragma once

#include <string_view>
#include <string>
#include <unistd.h>
#include <vector>

#if defined(__LP64__)
# define LP_SELECT(lp32, lp64) lp64
#else
# define LP_SELECT(lp32, lp64) lp32
#endif

constexpr auto kCPSocketName = "/" LP_SELECT("cp32", "cp64") ".sock";
constexpr const auto MAGIC_PATH_ENV = "MAGIC_PATH";
constexpr const auto MAGIC_ENV = "MAGIC";

class UniqueFd {
    using Fd = int;
public:
    UniqueFd() = default;

    UniqueFd(Fd fd) : fd_(fd) {}

    ~UniqueFd() { if (fd_ >= 0) close(fd_); }

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
        UniqueFd memfd;

        inline explicit Module(std::string name, int memfd) : name(name), memfd(memfd) {}
    };

    enum class SocketAction {
        PingHeartBeat,
        RequestLogcatFd,
        GetProcessFlags,
        ReadModules,
        RequestCompanionSocket,
        GetModuleDir,
        ZygoteRestart,
    };

    void Init(const char *path);

    bool PingHeartbeat();

    int RequestLogcatFd();

    std::vector<Module> ReadModules();

    uint32_t GetProcessFlags(uid_t uid);

    int ConnectCompanion(size_t index);

    int GetModuleDir(size_t index);

    void ZygoteRestart();
}
