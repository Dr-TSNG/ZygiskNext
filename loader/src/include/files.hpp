#include <dirent.h>
#include <functional>
#include <string>
#include <vector>

struct mount_info {
    unsigned int id;
    unsigned int parent;
    dev_t device;
    std::string root;
    std::string target;
    std::string vfs_option;
    struct {
        unsigned int shared;
        unsigned int master;
        unsigned int propagate_from;
    } optional;
    std::string type;
    std::string source;
    std::string fs_option;
};

void file_readline(bool trim, FILE *fp, const std::function<bool(std::string_view)> &fn);
void file_readline(bool trim, const char *file, const std::function<bool(std::string_view)> &fn);
void file_readline(const char *file, const std::function<bool(std::string_view)> &fn);

std::vector<mount_info> parse_mount_info(const char *pid);

using sFILE = std::unique_ptr<FILE, decltype(&fclose)>;
using sDIR = std::unique_ptr<DIR, decltype(&closedir)>;
sDIR make_dir(DIR *dp);
sFILE make_file(FILE *fp);

static inline sDIR open_dir(const char *path) {
    return make_dir(opendir(path));
}

static inline sDIR xopen_dir(const char *path) {
    return make_dir(opendir(path));
}

static inline sDIR xopen_dir(int dirfd) {
    return make_dir(fdopendir(dirfd));
}

static inline sFILE open_file(const char *path, const char *mode) {
    return make_file(fopen(path, mode));
}

static inline sFILE xopen_file(const char *path, const char *mode) {
    return make_file(fopen(path, mode));
}

static inline sFILE xopen_file(int fd, const char *mode) {
    return make_file(fdopen(fd, mode));
}
