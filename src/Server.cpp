#include "../includes/Server.class.hpp"
#include "../includes/main.hpp"
#include <arpa/inet.h> //close
#include <array>
#include <cstdio>
#include <errno.h>
#include <fmt/format.h>
#include <iostream>
#include <memory>
#include <netinet/in.h>
#include <stdexcept>
#include <stdio.h>
#include <stdlib.h>
#include <string.h> //strlen
#include <string>
#include <sys/socket.h>
#include <sys/time.h> //FD_SET, FD_ISSET, FD_ZERO macros
#include <sys/types.h>
#include <unistd.h> //close
#define PORT 4242
#define MAX_CLIENT 3

extern TintinReporter	g_report;

Server::Server() : stream(0), addr(0), mode(ShellMode::None)
{
}
Server::Server(Server const &config)
{
	*this = config;
}
Server &Server::operator=(Server const &rhs)
{
	this->stream = rhs.stream;
	this->addr = rhs.addr;
	this->mode = rhs.mode;
	return (*this);
}
Server::~Server()
{
}

std::string exec(const char *cmd)
{
	std::array<char, 128> buffer;
	std::string result;
	std::unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
	if (!pipe)
	{
		throw std::runtime_error("popen() failed!");
	}
	while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr)
	{
		result += buffer.data();
	}
	return (result);
}

void Server::run()
{
	int					opt;
	int					max_fd;
	struct sockaddr_in	address;
	fd_set				readfds;
	char				buffer[1025];
	char				cpy[4];

	opt = true;
	int server_fd, addrlen, i;
	if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == 0)
		unlock_file_and_exit(EXIT_FAILURE);
	if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR, (char *)&opt,
			sizeof(opt)) < 0)
		unlock_file_and_exit(EXIT_FAILURE);
	address.sin_family = AF_INET;
	address.sin_addr.s_addr = INADDR_ANY;
	address.sin_port = htons(PORT);
	if (bind(server_fd, (struct sockaddr *)&address, sizeof(address)) < 0)
		unlock_file_and_exit(EXIT_FAILURE);
	if (listen(server_fd, MAX_CLIENT) < 0)
		unlock_file_and_exit(EXIT_FAILURE);
	// accept the incoming connection
	addrlen = sizeof(address);
	int msg_size, new_socket, active_client, connected_clients = 0,
		client_socket[MAX_CLIENT], client_fd;
	for (i = 0; i < MAX_CLIENT; i++)
	{
		client_socket[i] = 0;
	}
	while (true)
	{
		FD_ZERO(&readfds);
		FD_SET(server_fd, &readfds);
		max_fd = server_fd;
		for (i = 0; i < MAX_CLIENT; i++)
		{
			client_fd = client_socket[i];
			if (client_fd > 0)
				FD_SET(client_fd, &readfds);
			if (client_fd > max_fd)
				max_fd = client_fd;
		}
		active_client = select(max_fd + 1, &readfds, NULL, NULL, NULL);
		if ((active_client < 0) && (errno != EINTR))
			unlock_file_and_exit(EXIT_FAILURE);
		if (FD_ISSET(server_fd, &readfds))
		{
			if ((new_socket = accept(server_fd, (struct sockaddr *)&address,
						(socklen_t *)&addrlen)) < 0)
				unlock_file_and_exit(EXIT_FAILURE);
			if (connected_clients >= MAX_CLIENT)
			{
				g_report.log(LogInfo::Warn, "Already 3 clients connected");
				if (send(new_socket, "Already 3 clients connected\r\n",
						strlen("Already 3 clients connected\r\n"), 0) == -1)
					perror("send");
				close(new_socket);
				continue ;
			}
			else
			{
				g_report.log(LogInfo::Info,
					fmt::v8::format("Connecting to new address : {}:{}",
						inet_ntoa(address.sin_addr), ntohs(address.sin_port)));
				connected_clients += 1;
			}
			for (i = 0; i < MAX_CLIENT; i++)
			{
				if (client_socket[i] == 0)
				{
					client_socket[i] = new_socket;
					break ;
				}
			}
		}
		for (i = 0; i < MAX_CLIENT; i++)
		{
			client_fd = client_socket[i];
			if (FD_ISSET(client_fd, &readfds))
			{
				if ((msg_size = read(client_fd, buffer, 1024)) == 0)
				{
					// Somebody disconnected , get his details and print
					getpeername(client_fd, (struct sockaddr *)&address,
						(socklen_t *)&addrlen);
					g_report.log(LogInfo::Info,
						fmt::v8::format("Host disconnected : {}:{}",
							inet_ntoa(address.sin_addr),
							ntohs(address.sin_port)));
					connected_clients -= 1;
					// Close the socket and mark as 0 in list for reuse
					close(client_fd);
					client_socket[i] = 0;
				}
				else
				{
					buffer[msg_size] = '\0';
					if (!strcmp("quit\n", buffer))
					{
						send(client_fd, "Exiting the daemon\r\n",
							strlen("Exiting the daemon\r\n"), 0);
						g_report.log(LogInfo::Info,
							fmt::v8::format("Exiting the daemon"));
						close(client_fd);
						g_report.sendRecap();
						exit(EXIT_SUCCESS);
					}
					else
					{
						strncpy(cpy, buffer, 3);
						cpy[3] = 0;
						if (!strcmp("sh ", cpy))
						{
							std::string str_cpy = buffer + 3;
							if (!str_cpy.empty() && str_cpy[str_cpy.length()
								- 1] == '\n')
							{
								str_cpy.erase(str_cpy.length() - 1);
							}
							g_report.log(LogInfo::Info,
								fmt::v8::format("new command : `{}`", str_cpy));
							std::string res = exec(str_cpy.c_str());
							send(client_fd, res.c_str(), res.length(), 0);
							g_report.log(LogInfo::Info,
								fmt::v8::format("res : `{}`", res));
						}
						else
						{
							g_report.log(LogInfo::Info, buffer);
						}
					}
				}
			}
		}
	}
}

void Server::set_mode(ShellMode mode)
{
	this->mode = mode;
}
