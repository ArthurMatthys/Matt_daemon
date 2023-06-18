#include "../includes/Client.class.hpp"

Client::Client() : stream(0), addr(0), mode(ShellMode::None) {}
Client::Client(Client const &config) { *this = config; }
Client &Client::operator=(Client const &rhs) {
    this->stream = rhs.stream;
    this->addr = rhs.addr;
    this->mode = rhs.mode;
    return (*this);
}
Client::~Client() {}

void Client::set_mode(ShellMode mode) { this->mode = mode; }
