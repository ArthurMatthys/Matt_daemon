#include "main.hpp"
#include <chrono>
#include <csignal>
#include <cstdlib>
#include <ctime>
#include <fcntl.h>
#include <filesystem>
#include <format>
#include <fstream>
#include <iostream>
#include <unistd.h>

namespace fs = std::filesystem;

void handle_sig(int i) {
    fs::create_directories("sandbox/1/2/a");

    auto now = std::chrono::system_clock::now();
    std::time_t t = std::chrono::system_clock::to_time_t(now);
    std::tm *ptm = std::localtime(&t);
    char buffer[32];
    std::strftime(buffer, 32, "%d / %m / %Y - %H : %M : %S", ptm);

    std::ofstream log("/var/log/matt_daemon/matt_daemon.log",
                      std::ios::app | std::ios::out);

    char msg[128];
    sprintf(msg, "%s : Received signal %02i. Exiting the daemon", buffer, i);

    std::cout << msg;
    log << msg << std::endl;

    unlock_file(LOCK_FILE);

    exit(EXIT_SUCCESS);
}

void set_sig_handlers() {
    for (int i = 1; i < NSIG; i++) {
        std::cout << i << std::endl;
        // NSIG on LIBC of rust, we have NSIG = 32. On csignal, it's 65 --'
        if (i == SIGKILL || i == SIGSTOP || i == SIGCHLD || i >= 32) {
            continue;
        }
        if (signal(i, handle_sig) == SIG_ERR) {
            std::cout << "Oupsi" << std::endl;
            exit(EXIT_FAILURE);
        }
    }
}
