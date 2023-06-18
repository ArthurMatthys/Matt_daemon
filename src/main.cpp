#include "TintinReporter.class.hpp"
#include "main.hpp"
#include <fcntl.h>
#include <iostream>
#include <sys/file.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

using namespace std;
const char *LOCK_FILE = "/var/lock/matt_daemon.lock";
const char *LOG_FILE = "/var/log/matt_daemon/matt_daemon.log";

void fork_exit_parent() {
    pid_t c_pid = fork();
    if (c_pid == -1) {
        exit(EXIT_FAILURE);
    } else if (c_pid > 0) {
        //  wait(nullptr);
        cout << "printed from parent process " << getpid() << endl;
        exit(EXIT_SUCCESS);
    } else {
        cout << "printed from child process " << getpid() << endl;
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

int main() {

    TintinReporter report = TintinReporter();

    fork_exit_parent();
    set_sid();
    fork_exit_parent();
    create_lock_file(LOCK_FILE);
    umask(0);
    change_working_dir();
    close_fds();
    set_sig_handlers();
    /* redirect_stream(); */

    while (true) {
        sleep(1);
        cout << "yolo" << endl;
    }

    // launch server
    // send mail (?)
    unlock_file();

    return 0;
}
