#ifndef MAILCONFIG_CLASS_H
#define MAILCONFIG_CLASS_H

#include <string>

class MailConfig {
  public:
    std::string username;
    std::string password;
    std::string relay;
    std::string mail_addr;

    MailConfig(void);
    MailConfig(MailConfig const &reporter);
    MailConfig &operator=(MailConfig const &rhs);
    virtual ~MailConfig(void);
};

#endif
