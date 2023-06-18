#include "../includes/TintinReporter.class.hpp"

TintinReporter::TintinReporter()
    : logfile(""), mail(MailConfig()), mail_active(false) {}

TintinReporter::TintinReporter(TintinReporter const &reporter) {
    *this = reporter;
}

TintinReporter &TintinReporter::operator=(TintinReporter const &rhs) {
    this->logfile = rhs.logfile;
    this->mail = rhs.mail;
    this->mail_active = rhs.mail_active;
    return (*this);
}

TintinReporter::~TintinReporter() {}
