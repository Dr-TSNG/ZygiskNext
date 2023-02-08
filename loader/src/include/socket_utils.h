#pragma once

#include <string>
#include <string_view>

#include "logging.h"

namespace socket_utils {

    ssize_t xread(int fd, void *buf, size_t count);

    ssize_t xwrite(int fd, const void *buf, size_t count);

    uint8_t read_u8(int fd);

    size_t read_usize(int fd);

    std::string read_string(int fd);

    bool write_u8(int fd, uint8_t val);

    int recv_fd(int fd);

    bool write_usize(int fd, size_t val);

    bool write_string(int fd, std::string_view str);
}
