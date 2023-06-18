#include "../includes/TintinReporter.class.hpp"
#include <chrono>
#include <filesystem>
#include <format>
#include <fstream>
#include <iostream>

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

TintinReporter::~TintinReporter() { std::cout << "Riperino" << std::endl; }

std::string get_loginfo(LogInfo info) {
    std::string ret;
    switch (info) {
    case LogInfo::Debug:
        ret = "\x1B[34mDEBUG\x1B[0m";
        break;
    case LogInfo::Info:
        ret = "\x1B[33mINFO\x1B[0m";
        break;
    case LogInfo::Warn:
        ret = "\x1B[35mWarn\x1B[0m";
        break;
    case LogInfo::Error:
        ret = "\x1B[31mERROR\x1B[0m";
        break;
    }
    return ret;
}

void report_log(LogInfo info, std::string msg) {
    std::filesystem::create_directories("/var/log/matt_daemon");

    auto now = std::chrono::system_clock::now();
    std::time_t t = std::chrono::system_clock::to_time_t(now);
    std::tm *ptm = std::localtime(&t);
    char buffer[32];
    std::strftime(buffer, 32, "%d / %m / %Y - %H : %M : %S", ptm);

    std::ofstream log("/var/log/matt_daemon/matt_daemon.log",
                      std::ios::app | std::ios::out);

    log << std::format("{} - {} : {}", buffer, get_loginfo(info), msg)
        << std::endl;
}