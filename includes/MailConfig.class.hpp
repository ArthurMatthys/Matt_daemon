#ifndef MAILCONFIG_CLASS_H
#define MAILCONFIG_CLASS_H

#include <string>

class MailConfig {
  public:
    MailConfig(void);
    MailConfig(MailConfig const &reporter);
    MailConfig &operator=(MailConfig const &rhs);
    virtual ~MailConfig(void);
    void sendRecap();

  private:
    std::string username;
    std::string password;
    std::string relay;
    std::string mail_addr;
    bool mail_active;
};

#endif
