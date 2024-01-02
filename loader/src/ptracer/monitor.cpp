#include <sys/system_properties.h>
#include <unistd.h>
#include <sys/stat.h>
#include <map>
#include <set>
#include <syscall.h>
#include <dirent.h>
#include <sys/stat.h>
#include <sys/signalfd.h>
#include <err.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <sys/epoll.h>
#include <sys/wait.h>
#include <sys/mount.h>
#include <time.h>
#include <fcntl.h>

#include "main.hpp"
#include "utils.hpp"
#include "files.hpp"
#include "misc.hpp"

using namespace std::string_view_literals;


#define STOPPED_WITH(sig, event) WIFSTOPPED(status) && (status >> 8 == ((sig) | (event << 8)))

static void updateStatus();

enum TracingState {
    TRACING = 1,
    STOPPING,
    STOPPED,
    EXITING
};

std::string monitor_stop_reason;

constexpr char SOCKET_NAME[] = "init_monitor";

std::string GetControlSocketName() {
    auto env = getenv(MAGIC_ENV);
    if (env == nullptr) return SOCKET_NAME;
    return std::string(SOCKET_NAME) + env;
}

struct EventLoop;

struct EventHandler {
    virtual int GetFd() = 0;
    virtual void HandleEvent(EventLoop& loop, uint32_t event) = 0;
};

struct EventLoop {
private:
    int epoll_fd_;
    bool running = false;
public:
    bool Init() {
        epoll_fd_ = epoll_create(1);
        if (epoll_fd_ == -1) {
            PLOGE("failed to create");
            return false;
        }
        return true;
    }

    void Stop() {
        running = false;
    }

    void Loop() {
        running = true;
        constexpr auto MAX_EVENTS = 2;
        struct epoll_event events[MAX_EVENTS];
        while (running) {
            int nfds = epoll_wait(epoll_fd_, events, MAX_EVENTS, -1);
            if (nfds == -1) {
                if (errno != EINTR)
                    PLOGE("epoll_wait");
                continue;
            }
            for (int i = 0; i < nfds; i++) {
                reinterpret_cast<EventHandler *>(events[i].data.ptr)->HandleEvent(*this,
                                                                                  events[i].events);
                if (!running) break;
            }
        }
    }

    bool RegisterHandler(EventHandler &handler, uint32_t events) {
        struct epoll_event ev{};
        ev.events = events;
        ev.data.ptr = &handler;
        if (epoll_ctl(epoll_fd_, EPOLL_CTL_ADD, handler.GetFd(), &ev) == -1) {
            PLOGE("failed to add event handler");
            return false;
        }
        return true;
    }

    bool UnregisterHandler(EventHandler &handler) {
        if (epoll_ctl(epoll_fd_, EPOLL_CTL_DEL, handler.GetFd(), nullptr) == -1) {
            PLOGE("failed to del event handler");
            return false;
        }
        return true;
    }

    ~EventLoop() {
        if (epoll_fd_ >= 0) close(epoll_fd_);
    }
};

static TracingState tracing_state = TRACING;


struct Status {
    bool supported = false;
    bool zygote_injected = false;
    bool daemon_running = false;
    pid_t daemon_pid = -1;
    std::string daemon_info;
    std::string daemon_error_info;
};

static Status status64;
static Status status32;

struct SocketHandler : public EventHandler {
    int sock_fd_;

    bool Init() {
        sock_fd_ = socket(PF_UNIX, SOCK_DGRAM | SOCK_CLOEXEC | SOCK_NONBLOCK, 0);
        if (sock_fd_ == -1) {
            PLOGE("socket create");
            return false;
        }
        struct sockaddr_un addr{
                .sun_family = AF_UNIX,
                .sun_path={0},
        };
        auto socket_name = GetControlSocketName();
        strcpy(addr.sun_path + 1, socket_name.c_str());
        socklen_t socklen = sizeof(sa_family_t) + strlen(addr.sun_path + 1) + 1;
        if (bind(sock_fd_, (struct sockaddr *) &addr, socklen) == -1) {
            PLOGE("bind socket");
            return false;
        }
        return true;
    }

    int GetFd() override {
        return sock_fd_;
    }

    void HandleEvent(EventLoop &loop, uint32_t event) override {
        struct [[gnu::packed]] MsgHead {
            Command cmd;
            int length;
            char data[0];
        };
        for (;;) {
            std::vector<uint8_t> buf;
            buf.resize(sizeof(MsgHead), 0);
            MsgHead &msg = *reinterpret_cast<MsgHead*>(buf.data());
            ssize_t real_size;
            auto nread = recv(sock_fd_, &msg, sizeof(msg), MSG_PEEK);
            if (nread == -1) {
                if (errno == EAGAIN) {
                    break;
                }
                PLOGE("read socket");
            }
            if (static_cast<size_t>(nread) < sizeof(Command)) {
                LOGE("read %zu < %zu", nread, sizeof(Command));
                continue;
            }
            if (msg.cmd >= Command::DAEMON64_SET_INFO) {
                if (nread != sizeof(msg)) {
                    LOGE("cmd %d size %zu != %zu", msg.cmd, nread, sizeof(MsgHead));
                    continue;
                }
                real_size = sizeof(MsgHead) + msg.length;
            } else {
                if (nread != sizeof(Command)) {
                    LOGE("cmd %d size %zu != %zu", msg.cmd, nread, sizeof(Command));
                    continue;
                }
                real_size = sizeof(Command);
            }
            buf.resize(real_size);
            nread = recv(sock_fd_, &msg, real_size, 0);
            if (nread == -1) {
                if (errno == EAGAIN) {
                    break;
                }
                PLOGE("recv");
                continue;
            }
            if (nread != real_size) {
                LOGE("real size %zu != %zu", real_size, nread);
                continue;
            }
            switch (msg.cmd) {
                case START:
                    if (tracing_state == STOPPING) {
                        tracing_state = TRACING;
                    } else if (tracing_state == STOPPED) {
                        ptrace(PTRACE_SEIZE, 1, 0, PTRACE_O_TRACEFORK);
                        LOGI("start tracing init");
                        tracing_state = TRACING;
                    }
                    updateStatus();
                    break;
                case STOP:
                    if (tracing_state == TRACING) {
                        LOGI("stop tracing requested");
                        tracing_state = STOPPING;
                        monitor_stop_reason = "user requested";
                        ptrace(PTRACE_INTERRUPT, 1, 0, 0);
                        updateStatus();
                    }
                    break;
                case EXIT:
                    LOGI("prepare for exit ...");
                    tracing_state = EXITING;
                    monitor_stop_reason = "user requested";
                    updateStatus();
                    loop.Stop();
                    break;
                case ZYGOTE64_INJECTED:
                    status64.zygote_injected = true;
                    updateStatus();
                    break;
                case ZYGOTE32_INJECTED:
                    status32.zygote_injected = true;
                    updateStatus();
                    break;
                case DAEMON64_SET_INFO:
                    LOGD("received daemon64 info %s", msg.data);
                    status64.daemon_info = std::string(msg.data);
                    updateStatus();
                    break;
                case DAEMON32_SET_INFO:
                    LOGD("received daemon32 info %s", msg.data);
                    status32.daemon_info = std::string(msg.data);
                    updateStatus();
                    break;
                case DAEMON64_SET_ERROR_INFO:
                    LOGD("received daemon64 error info %s", msg.data);
                    status64.daemon_running = false;
                    status64.daemon_error_info = std::string(msg.data);
                    updateStatus();
                    break;
                case DAEMON32_SET_ERROR_INFO:
                    LOGD("received daemon32 error info %s", msg.data);
                    status32.daemon_running = false;
                    status32.daemon_error_info = std::string(msg.data);
                    updateStatus();
                    break;
            }
        }
    }

    ~SocketHandler() {
        if (sock_fd_ >= 0) close(sock_fd_);
    }
};

constexpr auto MAX_RETRY_COUNT = 5;

#define CREATE_ZYGOTE_START_COUNTER(abi) \
struct timespec last_zygote##abi{.tv_sec = 0, .tv_nsec = 0}; \
int count_zygote##abi = 0; \
bool should_stop_inject##abi() { \
    struct timespec now{}; \
    clock_gettime(CLOCK_MONOTONIC, &now); \
    if (now.tv_sec - last_zygote##abi.tv_sec < 30) { \
        count_zygote##abi++; \
    } else { \
        count_zygote##abi = 0; \
    } \
    last_zygote##abi = now; \
    return count_zygote##abi >= MAX_RETRY_COUNT; \
}

CREATE_ZYGOTE_START_COUNTER(64)
CREATE_ZYGOTE_START_COUNTER(32)


static bool ensure_daemon_created(bool is_64bit) {
    auto &status = is_64bit ? status64 : status32;
    status.zygote_injected = false;
    if (status.daemon_pid == -1) {
        auto pid = fork();
        if (pid < 0) {
            PLOGE("create daemon (64=%s)", is_64bit ? "true" : "false");
            return false;
        } else if (pid == 0) {
            std::string daemon_name = "./bin/zygiskd";
            daemon_name += is_64bit ? "64" : "32";
            execl(daemon_name.c_str(), daemon_name.c_str(), nullptr);
            PLOGE("exec daemon %s failed", daemon_name.c_str());
            exit(1);
        } else {
            status.supported = true;
            status.daemon_pid = pid;
            status.daemon_running = true;
            return true;
        }
    } else {
        return status.daemon_running;
    }
}

struct SigChldHandler : public EventHandler {
private:
    int signal_fd_;
    struct signalfd_siginfo fdsi;
    int status;
    std::set<pid_t> process;
public:
    bool Init() {
        sigset_t mask;
        sigemptyset(&mask);
        sigaddset(&mask, SIGCHLD);
        if (sigprocmask(SIG_BLOCK, &mask, nullptr) == -1) {
            PLOGE("set sigprocmask");
            return false;
        }
        signal_fd_ = signalfd(-1, &mask, SFD_NONBLOCK | SFD_CLOEXEC);
        if (signal_fd_ == -1) {
            PLOGE("create signalfd");
            return false;
        }
        ptrace(PTRACE_SEIZE, 1, 0, PTRACE_O_TRACEFORK);
        return true;
    }

    int GetFd() override {
        return signal_fd_;
    }

    void HandleEvent(EventLoop &loop, uint32_t event) override {
        for (;;) {
            ssize_t s = read(signal_fd_, &fdsi, sizeof(fdsi));
            if (s == -1) {
                if (errno == EAGAIN) break;
                PLOGE("read signalfd");
                continue;
            }
            if (s != sizeof(fdsi)) {
                LOGW("read %zu != %zu", s, sizeof(fdsi));
                continue;
            }
            if (fdsi.ssi_signo != SIGCHLD) {
                LOGW("no sigchld received");
                continue;
            }
            int pid;
            while ((pid = waitpid(-1, &status, __WALL | WNOHANG)) != 0) {
                if (pid == -1) {
                    if (tracing_state == STOPPED && errno == ECHILD) break;
                    PLOGE("waitpid");
                }
                if (pid == 1) {
                    if (STOPPED_WITH(SIGTRAP, PTRACE_EVENT_FORK)) {
                        long child_pid;
                        ptrace(PTRACE_GETEVENTMSG, pid, 0, &child_pid);
                        LOGV("forked %ld", child_pid);
                    } else if (STOPPED_WITH(SIGTRAP, PTRACE_EVENT_STOP) &&
                               tracing_state == STOPPING) {
                        if (ptrace(PTRACE_DETACH, 1, 0, 0) == -1)
                            PLOGE("failed to detach init");
                        tracing_state = STOPPED;
                        LOGI("stop tracing init");
                        continue;
                    }
                    if (WIFSTOPPED(status)) {
                        if (WPTEVENT(status) == 0) {
                            if (WSTOPSIG(status) != SIGSTOP && WSTOPSIG(status) != SIGTSTP && WSTOPSIG(status) != SIGTTIN && WSTOPSIG(status) != SIGTTOU) {
                                LOGW("inject signal sent to init: %s %d",
                                     sigabbrev_np(WSTOPSIG(status)), WSTOPSIG(status));
                                ptrace(PTRACE_CONT, pid, 0, WSTOPSIG(status));
                            } else {
                                LOGW("suppress stopping signal sent to init: %s %d",
                                     sigabbrev_np(WSTOPSIG(status)), WSTOPSIG(status));
                            }
                            continue;
                        }
                        ptrace(PTRACE_CONT, pid, 0, 0);
                    }
                    continue;
                }
#define CHECK_DAEMON_EXIT(abi) \
                if (status##abi.supported && pid == status64.daemon_pid) { \
                    auto status_str = parse_status(status); \
                    LOGW("daemon" #abi "pid %d exited: %s", pid, status_str.c_str()); \
                    status##abi.daemon_running = false; \
                    if (status##abi.daemon_error_info.empty()) { \
                        status##abi.daemon_error_info = status_str; \
                    } \
                    updateStatus(); \
                    continue; \
                }
                CHECK_DAEMON_EXIT(64)
                CHECK_DAEMON_EXIT(32)
                auto state = process.find(pid);
                if (state == process.end()) {
                    LOGV("new process %d attached", pid);
                    process.emplace(pid);
                    ptrace(PTRACE_SETOPTIONS, pid, 0, PTRACE_O_TRACEEXEC);
                    ptrace(PTRACE_CONT, pid, 0, 0);
                    continue;
                } else {
                    if (STOPPED_WITH(SIGTRAP, PTRACE_EVENT_EXEC)) {
                        auto program = get_program(pid);
                        LOGV("%d program %s", pid, program.c_str());
                        const char* tracer = nullptr;
                        do {
                            if (tracing_state != TRACING) {
                                LOGW("stop injecting %d because not tracing", pid);
                                break;
                            }
#define PRE_INJECT(abi, is_64) \
                            if (program == "/system/bin/app_process"#abi) { \
                                tracer = "./bin/zygisk-ptrace"#abi; \
                                if (should_stop_inject##abi()) { \
                                    LOGW("zygote" #abi " restart too much times, stop injecting"); \
                                    tracing_state = STOPPING; \
                                    monitor_stop_reason = "zygote crashed"; \
                                    ptrace(PTRACE_INTERRUPT, 1, 0, 0); \
                                    break; \
                                } \
                                if (!ensure_daemon_created(is_64)) { \
                                    LOGW("daemon" #abi " not running, stop injecting"); \
                                    tracing_state = STOPPING; \
                                    monitor_stop_reason = "daemon not running"; \
                                    ptrace(PTRACE_INTERRUPT, 1, 0, 0); \
                                    break; \
                                } \
                            }
                            PRE_INJECT(64, true)
                            PRE_INJECT(32, false)
                            if (tracer != nullptr) {
                                LOGD("stopping %d", pid);
                                kill(pid, SIGSTOP);
                                ptrace(PTRACE_CONT, pid, 0, 0);
                                waitpid(pid, &status, __WALL);
                                if (STOPPED_WITH(SIGSTOP, 0)) {
                                    LOGD("detaching %d", pid);
                                    ptrace(PTRACE_DETACH, pid, 0, SIGSTOP);
                                    status = 0;
                                    auto p = fork_dont_care();
                                    if (p == 0) {
                                        execl(tracer, basename(tracer), "trace",
                                              std::to_string(pid).c_str(), "--restart", nullptr);
                                        PLOGE("failed to exec, kill");
                                        kill(pid, SIGKILL);
                                        exit(1);
                                    } else if (p == -1) {
                                        PLOGE("failed to fork, kill");
                                        kill(pid, SIGKILL);
                                    }
                                }
                            }
                        } while (false);
                        updateStatus();
                    } else {
                        LOGE("process %d received unknown status %s", pid,
                             parse_status(status).c_str());
                    }
                    process.erase(state);
                    if (WIFSTOPPED(status)) {
                        LOGV("detach process %d", pid);
                        ptrace(PTRACE_DETACH, pid, 0, 0);
                    }
                }
            }
        }
    }

    ~SigChldHandler() {
        if (signal_fd_ >= 0) close(signal_fd_);
    }
};

static std::string prop_path;
static std::string pre_section;
static std::string post_section;

static void updateStatus() {
    auto prop = xopen_file(prop_path.c_str(), "w");
    std::string status_text = "monitor:";
    switch (tracing_state) {
        case TRACING:
            status_text += "😋tracing";
            break;
        case STOPPING:
            [[fallthrough]];
        case STOPPED:
            status_text += "❌stopped";
            break;
        case EXITING:
            status_text += "❌exited";
            break;
    }
    if (tracing_state != TRACING && !monitor_stop_reason.empty()) {
        status_text += "(";
        status_text += monitor_stop_reason;
        status_text += ")";
    }
    status_text += ",";
#define WRITE_STATUS_ABI(suffix) \
    if (status##suffix.supported) { \
        status_text += " zygote" #suffix ":"; \
        if (tracing_state != TRACING) status_text += "❓unknown,"; \
        else if (status##suffix.zygote_injected) status_text += "😋injected,"; \
        else status_text += "❌not injected,"; \
        status_text += " daemon" #suffix ":"; \
        if (status##suffix.daemon_running) {  \
            status_text += "😋running";       \
            if (!status##suffix.daemon_info.empty()) { \
                status_text += "("; \
                status_text += status##suffix.daemon_info; \
                status_text += ")"; \
            } \
        } else { \
            status_text += "❌crashed"; \
            if (!status##suffix.daemon_error_info.empty()) { \
                status_text += "("; \
                status_text += status##suffix.daemon_error_info; \
                status_text += ")"; \
            } \
        } \
    }
    WRITE_STATUS_ABI(64)
    WRITE_STATUS_ABI(32)
    fprintf(prop.get(), "%s[%s] %s", pre_section.c_str(), status_text.c_str(), post_section.c_str());
}

static bool prepare_environment() {
    auto path = getenv(MAGIC_PATH_ENV);
    if (path == nullptr) {
        LOGE("path is null, is MAGIC_PATH_ENV specified?");
        return false;
    }
    prop_path = std::string(path) + "/module.prop";
    close(open(prop_path.c_str(), O_WRONLY | O_CREAT | O_TRUNC, 0644));
    auto orig_prop = xopen_file("./module.prop", "r");
    if (orig_prop == nullptr) {
        PLOGE("failed to open orig prop");
        return false;
    }
    bool post = false;
    file_readline(false, orig_prop.get(), [&](std::string_view line) -> bool {
        if (line.starts_with("description=")) {
            post = true;
            pre_section += "description=";
            post_section += line.substr(sizeof("description"));
        } else {
            if (post) {
                post_section += line;
            } else {
                pre_section += line;
            }
        }
        return true;
    });
    int old_ns;
    char wd[128];
    if (getcwd(wd, sizeof(wd)) == nullptr) {
        PLOGE("get cwd");
        return false;
    }
    if (!switch_mnt_ns(1, &old_ns)) return false;
    if (chdir(wd) == -1) {
        PLOGE("chdir %s", wd);
        return false;
    }
    if (mount(prop_path.c_str(), "/data/adb/modules/zygisksu/module.prop", nullptr, MS_BIND, nullptr) == -1) {
        PLOGE("failed to mount prop");
        return false;
    }
    if (!switch_mnt_ns(0, &old_ns)) return false;
    if (chdir(wd) == -1) {
        PLOGE("chdir %s", wd);
        return false;
    }
    updateStatus();
    return true;
}

void init_monitor() {
    LOGI("Zygisk Next %s", ZKSU_VERSION);
    LOGI("init monitor started");
    if (!prepare_environment()) {
        exit(1);
    }
    SocketHandler socketHandler{};
    socketHandler.Init();
    SigChldHandler ptraceHandler{};
    ptraceHandler.Init();
    EventLoop looper;
    looper.Init();
    looper.RegisterHandler(socketHandler, EPOLLIN | EPOLLET);
    looper.RegisterHandler(ptraceHandler, EPOLLIN | EPOLLET);
    looper.Loop();
    LOGI("exit");
}

void send_control_command(Command cmd) {
    int sockfd = socket(PF_UNIX, SOCK_DGRAM | SOCK_CLOEXEC, 0);
    if (sockfd == -1) err(EXIT_FAILURE, "socket");
    struct sockaddr_un addr{
            .sun_family = AF_UNIX,
            .sun_path={0},
    };
    auto socket_name = GetControlSocketName();
    strcpy(addr.sun_path + 1, socket_name.c_str());
    socklen_t socklen = sizeof(sa_family_t) + strlen(addr.sun_path + 1) + 1;
    auto nsend = sendto(sockfd, (void *) &cmd, sizeof(cmd), 0, (sockaddr *) &addr, socklen);
    if (nsend == -1) {
        err(EXIT_FAILURE, "send");
    } else if (nsend != sizeof(cmd)) {
        printf("send %ld != %ld\n", nsend, sizeof(cmd));
        exit(1);
    }
    printf("command sent\n");
}
