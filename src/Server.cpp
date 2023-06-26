#include "../includes/Server.class.hpp"
#include "../includes/main.hpp"
#include <arpa/inet.h> //close
#include <errno.h>
#include <format>
#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h> //strlen
#include <sys/socket.h>
#include <sys/time.h> //FD_SET, FD_ISSET, FD_ZERO macros
#include <sys/types.h>
#include <unistd.h> //close

#define PORT 4242
#define MAX_CLIENT 3

Server::Server() : stream(0), addr(0), mode(ShellMode::None) {}
Server::Server(Server const &config) { *this = config; }
Server &Server::operator=(Server const &rhs) {
    this->stream = rhs.stream;
    this->addr = rhs.addr;
    this->mode = rhs.mode;
    return (*this);
}
Server::~Server() {}

void Server::run(TintinReporter report) {

    int opt = true;
    int server_fd, addrlen, i;
    int max_fd;
    struct sockaddr_in address;

    fd_set readfds;

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

    int msg_size, new_socket, active_client,
        connected_clients = 0, client_socket[MAX_CLIENT], client_fd;
    char buffer[1025];
    for (i = 0; i < MAX_CLIENT; i++) {
        client_socket[i] = 0;
    }

    while (true) {
        FD_ZERO(&readfds);

        FD_SET(server_fd, &readfds);
        max_fd = server_fd;

        for (i = 0; i < MAX_CLIENT; i++) {
            client_fd = client_socket[i];

            if (client_fd > 0)
                FD_SET(client_fd, &readfds);

            if (client_fd > max_fd)
                max_fd = client_fd;
        }

        active_client = select(max_fd + 1, &readfds, NULL, NULL, NULL);

        if ((active_client < 0) && (errno != EINTR))
            unlock_file_and_exit(EXIT_FAILURE);

        if (FD_ISSET(server_fd, &readfds)) {
            if ((new_socket = accept(server_fd, (struct sockaddr *)&address,
                                     (socklen_t *)&addrlen)) < 0)
                unlock_file_and_exit(EXIT_FAILURE);
            if (connected_clients >= MAX_CLIENT) {
                report.log(LogInfo::Warn, "Already 3 clients connected");
                if (send(new_socket, "Already 3 clients connected\r\n",
                         strlen("Already 3 clients connected\r\n"), 0) == -1)
                    perror("send");
                close(new_socket);
                continue;
            } else {
                report.log(LogInfo::Info,
                           std::format("Connecting to new address : {}:{}",
                                       inet_ntoa(address.sin_addr),
                                       ntohs(address.sin_port)));
                connected_clients += 1;
            }

            for (i = 0; i < MAX_CLIENT; i++) {
                if (client_socket[i] == 0) {
                    client_socket[i] = new_socket;

                    break;
                }
            }
        }

        for (i = 0; i < MAX_CLIENT; i++) {
            client_fd = client_socket[i];

            if (FD_ISSET(client_fd, &readfds)) {
                if ((msg_size = read(client_fd, buffer, 1024)) == 0) {
                    // Somebody disconnected , get his details and print
                    getpeername(client_fd, (struct sockaddr *)&address,
                                (socklen_t *)&addrlen);
                    report.log(LogInfo::Info,
                               std::format("Host disconnected : {}:{}",
                                           inet_ntoa(address.sin_addr),
                                           ntohs(address.sin_port)));

                    connected_clients -= 1;
                    // Close the socket and mark as 0 in list for reuse
                    close(client_fd);
                    client_socket[i] = 0;
                }

                // Echo back the message that came in
                else {
                    // set the string terminating NULL byte on the end
                    // of the data read
                    buffer[msg_size] = '\0';
                    if (strcmp("quit\n", buffer) == 0) {
                        send(client_fd, "exiting client\r\n",
                             strlen("exiting client\r\n"), 0);
                        report.log(LogInfo::Info,
                                   std::format("Host disconnected : {}:{}",
                                               inet_ntoa(address.sin_addr),
                                               ntohs(address.sin_port)));
                        connected_clients -= 1;
                        if (close(client_fd) == -1)
                            unlock_file_and_exit(EXIT_FAILURE);
                        client_socket[i] = 0;
                        continue;
                    } else {
                        report.log(LogInfo::Info, buffer);
                    }
                }
            }
        }
    }
}

void Server::set_mode(ShellMode mode) { this->mode = mode; }
