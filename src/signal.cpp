#include "TintinReporter.class.hpp"
#include "main.hpp"
#include <csignal>
#include <cstdlib>
#include <fmt/format.h>

extern TintinReporter	g_report;

void	handle_sig(int i)
{
	sigset_t mask, oldmask;
	sigfillset(&mask);
	sigprocmask(SIG_BLOCK, &mask, &oldmask);
	report_log(LogInfo::Info,
		fmt::v8::format("Received signal {}. Exiting the daemon", i));
	g_report.sendRecap();
	unlock_file_and_exit(EXIT_SUCCESS);
	sigprocmask(SIG_SETMASK, &oldmask, NULL);
}

void	set_sig_handlers(void)
{
	for (int i = 1; i < NSIG; i++)
	{
		// NSIG on LIBC of rust, we have NSIG = 32. On csignal, it's 65 --'
		if (i == SIGKILL || i == SIGSTOP || i == SIGCHLD || i >= 32)
		{
			continue ;
		}
		if (signal(i, handle_sig) == SIG_ERR)
		{
			unlock_file_and_exit(EXIT_FAILURE);
		}
	}
}