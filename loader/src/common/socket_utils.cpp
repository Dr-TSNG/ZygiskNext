#include <cstddef>
#include <sys/socket.h>
#include <unistd.h>

#include "socket_utils.h"

namespace socket_utils {

    ssize_t xread(int fd, void* buf, size_t count) {
        size_t read_sz = 0;
        ssize_t ret;
        do {
            ret = read(fd, (std::byte*) buf + read_sz, count - read_sz);
            if (ret < 0) {
                if (errno == EINTR) continue;
                PLOGE("read");
                return ret;
            }
            read_sz += ret;
        } while (read_sz != count && ret != 0);
        if (read_sz != count) {
            PLOGE("read (%d != %d)", count, read_sz);
        }
        return read_sz;
    }

    ssize_t xwrite(int fd, const void* buf, size_t count) {
        size_t write_sz = 0;
        ssize_t ret;
        do {
            ret = write(fd, (std::byte*) buf + write_sz, count - write_sz);
            if (ret < 0) {
                if (errno == EINTR) continue;
                PLOGE("write");
                return ret;
            }
            write_sz += ret;
        } while (write_sz != count && ret != 0);
        if (write_sz != count) {
            PLOGE("write (%d != %d)", count, write_sz);
        }
        return write_sz;
    }

    ssize_t xrecvmsg(int sockfd, struct msghdr* msg, int flags) {
        int rec = recvmsg(sockfd, msg, flags);
        if (rec < 0) PLOGE("recvmsg");
        return rec;
    }

    void* recv_fds(int sockfd, char* cmsgbuf, size_t bufsz, int cnt) {
        iovec iov = {
                .iov_base = &cnt,
                .iov_len  = sizeof(cnt),
        };
        msghdr msg = {
                .msg_iov        = &iov,
                .msg_iovlen     = 1,
                .msg_control    = cmsgbuf,
                .msg_controllen = bufsz
        };

        xrecvmsg(sockfd, &msg, MSG_WAITALL);
        cmsghdr* cmsg = CMSG_FIRSTHDR(&msg);

        if (msg.msg_controllen != bufsz ||
            cmsg == nullptr ||
            // TODO: pass from rust: 20, expected: 16
            // cmsg->cmsg_len != CMSG_LEN(sizeof(int) * cnt) ||
            cmsg->cmsg_level != SOL_SOCKET ||
            cmsg->cmsg_type != SCM_RIGHTS) {
            return nullptr;
        }

        return CMSG_DATA(cmsg);
    }

    template<typename T>
    inline T read_exact_or(int fd, T fail) {
        T res;
        return sizeof(T) == xread(fd, &res, sizeof(T)) ? res : fail;
    }

    template<typename T>
    inline bool write_exact(int fd, T val) {
        return sizeof(T) == xwrite(fd, &val, sizeof(T));
    }

    uint8_t read_u8(int fd) {
        return read_exact_or<uint8_t>(fd, 0);
    }

    size_t read_usize(int fd) {
        return read_exact_or<size_t>(fd, 0);
    }

    bool write_usize(int fd, size_t val) {
        return write_exact<size_t>(fd, val);
    }

    std::string read_string(int fd) {
        auto len = read_usize(fd);
        char buf[len + 1];
        buf[len] = '\0';
        xread(fd, buf, len);
        return buf;
    }

    bool write_u8(int fd, uint8_t val) {
        return write_exact<uint8_t>(fd, val);
    }

    int recv_fd(int sockfd) {
        char cmsgbuf[CMSG_SPACE(sizeof(int))];

        void* data = recv_fds(sockfd, cmsgbuf, sizeof(cmsgbuf), 1);
        if (data == nullptr) return -1;

        int result;
        memcpy(&result, data, sizeof(int));
        return result;
    }
}
