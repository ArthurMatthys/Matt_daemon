#include "TintinReporter.class.hpp"
#include "main.hpp"
#include <csignal>
#include <cstdlib>
#include <format>

extern TintinReporter g_report;

void handle_sig(int i) {

    report_log(LogInfo::Info,
               std::format("Received signal {}. Exiting the daemon", i));

    g_report.sendRecap();

    unlock_file_and_exit(EXIT_SUCCESS);
}

void set_sig_handlers() {
    for (int i = 1; i < NSIG; i++) {
        // NSIG on LIBC of rust, we have NSIG = 32. On csignal, it's 65 --'
        if (i == SIGKILL || i == SIGSTOP || i == SIGCHLD || i >= 32) {
            continue;
        }
        if (signal(i, handle_sig) == SIG_ERR) {
            unlock_file_and_exit(EXIT_FAILURE);
        }
    }
}
