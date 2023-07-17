#include "../includes/TintinReporter.class.hpp"
#include "main.hpp"
#include <algorithm>
#include <chrono>
#include <curl/curl.h>
#include <fcntl.h>
#include <filesystem>
#include <format>
#include <fstream>
#include <iostream>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#define FROM_MAIL "<matthys.arthur16@gmail.com>"
#define TO_MAIL "<matthys.arthur16@gmail.com>"
#define SUBJECT "Recap Matt Daemon"
#define SIZE_BUFF 16384

TintinReporter::TintinReporter(bool send_mail, char *mail_to)
    : send_mail(send_mail), mail_to(mail_to) {
    this->username = std::getenv("SMTPUSERNAME");
    this->username = std::getenv("SMTPPASSWORD");
    if (send_mail && !this->username) {
        std::cerr << "No username for smtp, please export a `SMTPUSERNAME`"
                  << std::endl;
        exit(EXIT_FAILURE);
    }
    if (send_mail && !this->password) {
        std::cerr << "No username for smtp, please export a `SMTPPASSWORD`"
                  << std::endl;
        exit(EXIT_FAILURE);
    }
}

TintinReporter::TintinReporter(TintinReporter const &reporter) {
    *this = reporter;
}

TintinReporter &TintinReporter::operator=(TintinReporter const &rhs) {
    this->send_mail = rhs.send_mail;
    this->username = rhs.username;
    this->password = rhs.password;
    this->mail_to = rhs.mail_to;
    return (*this);
}

TintinReporter::~TintinReporter() {}

struct upload_status {
    size_t bytes_read;
};

static size_t payload_source(char *ptr, size_t size, size_t nmemb,
                             void *userp) {
    struct upload_status *upload_ctx = (struct upload_status *)userp;
    size_t room = size * nmemb;
    std::string msg;
    char content[SIZE_BUFF];
    int fd;
    fd = open(LOG_FILE, O_RDONLY);
    if (fd == -1)
        unlock_file_and_exit(EXIT_FAILURE);

    int ret = read(fd, content, SIZE_BUFF);
    if (ret == -1)
        unlock_file_and_exit(EXIT_FAILURE);

    msg = std::format("To: {}\r\n"
                      "From: {}\r\n"
                      "Subject: {}\r\n"
                      "\r\n"
                      "{}"
                      "\r\n",
                      TO_MAIL, FROM_MAIL, SUBJECT, content);

    if ((size == 0) || (nmemb == 0) || ((size * nmemb) < 1)) {
        return 0;
    }
    const char *data;
    data = &msg.c_str()[upload_ctx->bytes_read];

    if (data) {
        size_t len = strlen(data);
        if (room < len)
            len = room;
        memcpy(ptr, data, len);
        upload_ctx->bytes_read += len;

        return len;
    }
    return 0;
}

void TintinReporter::sendRecap() {
    CURL *curl;
    CURLcode res = CURLE_OK;
    struct curl_slist *recipients = NULL;
    struct upload_status upload_ctx = {0};

    curl = curl_easy_init();
    if (curl) {
        curl_easy_setopt(curl, CURLOPT_USERNAME, "USERNAME");
        curl_easy_setopt(curl, CURLOPT_PASSWORD, "PASSWORD");

        curl_easy_setopt(curl, CURLOPT_URL, "smtps://smtp.gmail.com");

        recipients = curl_slist_append(recipients, TO_MAIL);
        curl_easy_setopt(curl, CURLOPT_MAIL_RCPT, recipients);

        curl_easy_setopt(curl, CURLOPT_READFUNCTION, payload_source);
        curl_easy_setopt(curl, CURLOPT_READDATA, &upload_ctx);
        curl_easy_setopt(curl, CURLOPT_UPLOAD, 1L);

        curl_easy_setopt(curl, CURLOPT_VERBOSE, 1L);

        res = curl_easy_perform(curl);

        if (res != CURLE_OK)
            unlock_file_and_exit(EXIT_FAILURE);

        curl_slist_free_all(recipients);

        curl_easy_cleanup(curl);
    }
}

void TintinReporter::log(LogInfo info, std::string msg) {
    report_log(info, msg);
}

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
        ret = "\x1B[35mWARN\x1B[0m";
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
    if (!msg.empty() && msg[msg.length() - 1] == '\n') {
        msg.erase(msg.length() - 1);
    }
    std::strftime(buffer, 32, "%d / %m / %Y - %H : %M : %S", ptm);

    std::ofstream log("/var/log/matt_daemon/matt_daemon.log",
                      std::ios::app | std::ios::out);

    log << std::format("{} - {} : {}", buffer, get_loginfo(info), msg)
        << std::endl;
}
