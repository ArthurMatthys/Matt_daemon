#include "../includes/MailConfig.class.hpp"

MailConfig::MailConfig()
    : username(""), password(""), relay(""), mail_addr("") {}
MailConfig::MailConfig(MailConfig const &config) { *this = config; }
MailConfig &MailConfig::operator=(MailConfig const &rhs) {
    this->username = rhs.username;
    this->password = rhs.password;
    this->relay = rhs.relay;
    this->mail_addr = rhs.mail_addr;
    return (*this);
}
MailConfig::~MailConfig() {}