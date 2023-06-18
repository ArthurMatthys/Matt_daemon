#include "main.hpp"
#include <cstdio>
#include <cstdlib>
#include <fcntl.h>
#include <iostream>
#include <ostream>
#include <sys/file.h>
#include <unistd.h>

long get_max_fd() {
    long ret = sysconf(_SC_OPEN_MAX);
    if (ret == -1) {
        exit(EXIT_FAILURE);
    }
    return ret;
}

void close_fds() {
    long fd_max = get_max_fd();
    for (long i = 3; i <= fd_max; i++) {
        close(i);
    }
}

void create_lock_file(const char *filename) {
    int fd = open(filename, O_RDWR | O_CREAT | O_EXCL, 0666);
    if (fd < 0) {
        std::cout << "yo8" << std::endl;
        exit(EXIT_FAILURE);
    }
    if (fd >= 0 && flock(fd, LOCK_EX | LOCK_NB) < 0) {
        close(fd);
        exit(EXIT_FAILURE);
    }
}

void unlock_file(const char *filename) {
    int fd = open(filename, O_RDONLY, 0666);
    if (fd < 0) {
        exit(EXIT_FAILURE);
    }
    if (fd >= 0 && flock(fd, LOCK_UN) < 0) {
        close(fd);
        exit(EXIT_FAILURE);
    }
    if (remove(filename) == -1) {
        exit(EXIT_FAILURE);
    }
}

void redirect_stream() {
    if (close(STDIN_FILENO) == -1) {
        exit(EXIT_FAILURE);
    }
    int null_fd = open("/dev/null", O_RDWR);
    if (null_fd != 0) {
        exit(EXIT_FAILURE);
    }
    if (dup2(STDIN_FILENO, STDOUT_FILENO) == -1) {
        exit(EXIT_FAILURE);
    }
    if (dup2(STDIN_FILENO, STDERR_FILENO) == -1) {
        exit(EXIT_FAILURE);
    }
}
