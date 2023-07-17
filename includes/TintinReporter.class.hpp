#ifndef TINTINREPORTER_CLASS_H
#define TINTINREPORTER_CLASS_H

#include <string>

enum LogInfo { Debug, Info, Warn, Error };
std::string get_loginfo(LogInfo info);

class TintinReporter {
  public:
    TintinReporter(bool send_mail, char *mail_to);
    TintinReporter(TintinReporter const &reporter);
    TintinReporter &operator=(TintinReporter const &rhs);
    virtual ~TintinReporter(void);

    void sendRecap();
    void log(LogInfo info, std::string msg);

  private:
    bool send_mail;
    char *username;
    char *password;
    char *mail_to;
};

void report_log(LogInfo info, std::string msg);

#endif
