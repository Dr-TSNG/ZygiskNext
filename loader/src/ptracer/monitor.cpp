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
#include <time.h>

#include "main.hpp"
#include "utils.hpp"
#include "files.hpp"
#include "misc.hpp"

using namespace std::string_view_literals;


#define STOPPED_WITH(sig, event) WIFSTOPPED(status) && (status >> 8 == ((sig) | (event << 8)))


enum TracingState {
    TRACING = 1,
    STOPPING,
    STOPPED
};

constexpr char SOCKET_NAME[] = "init_monitor";

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
        strcpy(addr.sun_path + 1, SOCKET_NAME);
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
        Command cmd;
        for (;;) {
            auto nread = read(sock_fd_, &cmd, sizeof(cmd));
            if (nread == -1) {
                if (errno == EAGAIN) {
                    break;
                }
                PLOGE("read socket");
            }
            if (nread != sizeof(cmd)) {
                LOGE("read %zu != %zu", nread, sizeof(cmd));
                continue;
            }
            switch (cmd) {
                case START:
                    if (tracing_state == STOPPING) {
                        tracing_state = TRACING;
                    } else if (tracing_state == STOPPED) {
                        ptrace(PTRACE_SEIZE, 1, 0, PTRACE_O_TRACEFORK);
                        LOGI("start tracing init");
                        tracing_state = TRACING;
                    }
                    break;
                case STOP:
                    if (tracing_state == TRACING) {
                        LOGI("stop tracing requested");
                        tracing_state = STOPPING;
                        ptrace(PTRACE_INTERRUPT, 1, 0, 0);
                    }
                    break;
                case EXIT:
                    LOGI("prepare for exit ...");
                    loop.Stop();
                    break;
            }
        }
    }

    ~SocketHandler() {
        if (sock_fd_ >= 0) close(sock_fd_);
    }
};

constexpr auto MAX_RETRY_COUNT = 2;

struct timespec last_zygote64{.tv_sec = 0, .tv_nsec = 0};
int count_zygote64 = 0;
bool should_stop_inject64() {
    struct timespec now{};
    clock_gettime(CLOCK_MONOTONIC, &now);
    if (now.tv_sec - last_zygote64.tv_sec < 30) {
        count_zygote64++;
    } else {
        count_zygote64 = 0;
    }
    last_zygote64 = now;
    return count_zygote64 >= MAX_RETRY_COUNT;
}

struct timespec last_zygote32{.tv_sec = 0, .tv_nsec = 0};
int count_zygote32 = 0;
bool should_stop_inject32() {
    struct timespec now{};
    clock_gettime(CLOCK_MONOTONIC, &now);
    if (now.tv_sec - last_zygote32.tv_sec < 30) {
        count_zygote32++;
    } else {
        count_zygote32 = 0;
    }
    last_zygote32 = now;
    return count_zygote32 >= MAX_RETRY_COUNT;
}

struct PtraceHandler : public EventHandler {
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
                        LOGI("forked %ld", child_pid);
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
                auto state = process.find(pid);
                if (state == process.end()) {
                    LOGI("new process %d attached", pid);
                    process.emplace(pid);
                    ptrace(PTRACE_SETOPTIONS, pid, 0, PTRACE_O_TRACEEXEC);
                    ptrace(PTRACE_CONT, pid, 0, 0);
                    continue;
                } else {
                    if (STOPPED_WITH(SIGTRAP, PTRACE_EVENT_EXEC)) {
                        auto program = get_program(pid);
                        LOGD("%d program %s", pid, program.c_str());
                        const char* tracer = nullptr;
                        do {
                            if (tracing_state != TRACING) {
                                LOGW("stop injecting %d because not tracing", pid);
                                break;
                            }
                            if (program == "/system/bin/app_process64") {
                                tracer = "./bin/zygisk-ptrace64";
                                if (should_stop_inject64()) {
                                    LOGW("zygote64 restart too much times, stop injecting");
                                    tracing_state = STOPPING;
                                    ptrace(PTRACE_INTERRUPT, 1, 0, 0);
                                    break;
                                }
                            } else if (program == "/system/bin/app_process32") {
                                tracer = "./bin/zygisk-ptrace32";
                                if (should_stop_inject32()) {
                                    LOGW("zygote32 restart too much times, stop injecting");
                                    tracing_state = STOPPING;
                                    ptrace(PTRACE_INTERRUPT, 1, 0, 0);
                                    break;
                                }
                            }
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
                                              std::to_string(pid).c_str(), nullptr);
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

                    } else {
                        LOGD("process %d received unknown status %s", pid,
                             parse_status(status).c_str());
                    }
                    process.erase(state);
                    if (WIFSTOPPED(status)) {
                        LOGI("detach process %d", pid);
                        ptrace(PTRACE_DETACH, pid, 0, 0);
                    }
                }
            }
        }
        LOGD("sigchld handle done");
    }

    ~PtraceHandler() {
        if (signal_fd_ >= 0) close(signal_fd_);
    }
};

void init_monitor() {
    LOGI("init monitor started");
    SocketHandler socketHandler{};
    socketHandler.Init();
    PtraceHandler ptraceHandler{};
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
    strcpy(addr.sun_path + 1, SOCKET_NAME);
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
