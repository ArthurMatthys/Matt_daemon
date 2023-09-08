#include "TintinReporter.class.hpp"
#include "main.hpp"
#include <csignal>
#include <cstdlib>
#include <fmt/format.h>

extern TintinReporter	g_report;

void	handle_sig(int i)
{
	report_log(LogInfo::Info,
		fmt::v8::format("Received signal {}. Exiting the daemon", i));
	g_report.sendRecap();
	exit(EXIT_SUCCESS);
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

// void	handle_sig_null(int i)
// {
// 	report_log(LogInfo::Info,
// 		fmt::v8::format("Received signal {}. Exiting the daemon", i));
// 	(void)i;
// }

// void	set_sig_null(void)
// {
// 	for (int i = 1; i < NSIG; i++)
// 	{
// 		// NSIG on LIBC of rust, we have NSIG = 32. On csignal, it's 65 --'
// 		if (i == SIGKILL || i == SIGSTOP || i == SIGCHLD || i >= 32)
// 		{
// 			continue ;
// 		}
// 		if (signal(i, handle_sig_null) == SIG_ERR)
// 		{
// 			unlock_file_and_exit(EXIT_FAILURE);
// 		}
// 	}
// }
