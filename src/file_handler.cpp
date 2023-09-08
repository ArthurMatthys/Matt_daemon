#include "../includes/TintinReporter.class.hpp"
#include "main.hpp"
#include <cstdio>
#include <cstdlib>
#include <fcntl.h>
#include <fmt/format.h>
#include <iostream>
#include <ostream>
#include <sys/file.h>
#include <unistd.h>

long	get_max_fd(void)
{
	long	ret;

	ret = sysconf(_SC_OPEN_MAX);
	if (ret == -1)
	{
		unlock_file_and_exit(EXIT_FAILURE);
	}
	return (ret);
}

void	close_fds(void)
{
	long	fd_max;

	fd_max = get_max_fd();
	for (long i = 3; i <= fd_max; i++)
	{
		close(i);
	}
}

void	create_lock_file(const char *filename, TintinReporter report)
{
	int	fd;

	fd = open(filename, O_RDWR | O_CREAT | O_EXCL, 0600);
	if (fd < 0)
	{
		if (errno == 17)
			report.log(LogInfo::Error,
				fmt::v8::format("The lock file is locked by another process : {}",
					errno));
		std::cerr << "There is already a daemon running" << std::endl;
		exit(EXIT_FAILURE);
	}
	if (fd >= 0 && flock(fd, LOCK_EX | LOCK_NB) < 0)
	{
		close(fd);
		exit(EXIT_FAILURE);
	}
}

void	unlock_file(void)
{
	int	fd;

	fd = open(LOCK_FILE, O_RDONLY, 0600);
	if (fd < 0)
	{
		exit(EXIT_FAILURE);
	}
	if (fd >= 0 && flock(fd, LOCK_UN) < 0)
	{
		close(fd);
		exit(EXIT_FAILURE);
	}
	if (remove(LOCK_FILE) == -1)
	{
		exit(EXIT_FAILURE);
	}
}

void	unlock_file_and_exit(int status)
{
	unlock_file();
	exit(status);
}

void	redirect_stream(void)
{
	int	null_fd;

	if (close(STDIN_FILENO) == -1)
	{
		unlock_file_and_exit(EXIT_FAILURE);
	}
	null_fd = open("/dev/null", O_RDWR);
	if (null_fd != 0)
	{
		unlock_file_and_exit(EXIT_FAILURE);
	}
	if (dup2(STDIN_FILENO, STDOUT_FILENO) == -1)
	{
		unlock_file_and_exit(EXIT_FAILURE);
	}
	if (dup2(STDIN_FILENO, STDERR_FILENO) == -1)
	{
		unlock_file_and_exit(EXIT_FAILURE);
	}
}
