#include <android/log.h>
#include <unistd.h>

#include "logging.h"
#include "socket_utils.h"

namespace logging {
    static int logfd = -1;

    void setfd(int fd) {
        close(logfd);
        logfd = fd;
    }

    int getfd() {
        return logfd;
    }

    void log(int prio, const char* tag, const char* fmt, ...) {
        if (logfd == -1) {
            va_list ap;
            va_start(ap, fmt);
            __android_log_vprint(prio, tag, fmt, ap);
            va_end(ap);
        } else {
            char buf[BUFSIZ];
            va_list ap;
            va_start(ap, fmt);
            vsnprintf(buf, sizeof(buf), fmt, ap);
            va_end(ap);
            socket_utils::write_u8(logfd, prio);
            socket_utils::write_string(logfd, tag);
            socket_utils::write_string(logfd, buf);
        }
    }
}
