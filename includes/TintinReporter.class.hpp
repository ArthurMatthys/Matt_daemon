#ifndef TINTINREPORTER_CLASS_H
#define TINTINREPORTER_CLASS_H

#include "MailConfig.class.hpp"
#include <string>

enum LogInfo { Debug, Info, Warn, Error };

class TintinReporter {
  public:
    TintinReporter(void);
    TintinReporter(TintinReporter const &reporter);
    TintinReporter &operator=(TintinReporter const &rhs);
    virtual ~TintinReporter(void);

    void sendRecap();
    void log(std::string msg, LogInfo info);

  private:
    std::string logfile;
    MailConfig mail;
    bool mail_active;
};

#endif
