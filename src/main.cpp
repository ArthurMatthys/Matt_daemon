#include "../includes/TintinReporter.class.hpp"
#include "Server.class.hpp"
#include "main.hpp"
#include <cstring>
#include <fcntl.h>
#include <format>
#include <iostream>
#include <sys/file.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

using namespace std;
const char *LOCK_FILE = "/var/lock/matt_daemon.lock";
const char *LOG_FILE = "/var/log/matt_daemon/matt_daemon.log";

TintinReporter g_report = TintinReporter(false, NULL);

void fork_exit_parent() {
    pid_t c_pid = fork();
    if (c_pid == -1) {
        exit(EXIT_FAILURE);
    } else if (c_pid > 0) {
        exit(EXIT_SUCCESS);
    } else {
    }
}

void set_sid() {
    if (setsid() == -1) {
        exit(EXIT_FAILURE);
    };
}

void change_working_dir() {
    if (chdir("/") == -1) {
        unlock_file_and_exit(EXIT_FAILURE);
    };
}

int main(int argc, char **argv) {

    bool send_mail;
    char *mail_to = NULL;
    if (argc == 1) {
        send_mail = false;
    } else if (argc == 3 && strcmp(argv[1], "-m") == 0) {
        mail_to = argv[2];
        send_mail = true;
    } else {
        exit(EXIT_FAILURE);
    }

    std::cout << "hello ";
    std::cout << std::getenv("SMTPUSERNAME") << std::endl;
    std::cout << std::getenv("SMTPPASSWORD") << std::endl;

    g_report = TintinReporter(send_mail, mail_to);

    g_report.log(LogInfo::Info, "Entering Daemon mode");

    fork_exit_parent();
    set_sid();
    fork_exit_parent();
    g_report.log(LogInfo::Info, std::format("Starting with pid {}", getpid()));

    create_lock_file(LOCK_FILE, g_report);
    umask(0);
    change_working_dir();
    close_fds();
    set_sig_handlers();
    redirect_stream();
    g_report.log(LogInfo::Info, "Daemon started properly");
    Server server = Server();

    server.run();

    // launch server
    g_report.sendRecap();
    /* unlock_file(); */

    return 0;
}
