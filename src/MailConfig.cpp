#include "main.hpp"
#include <algorithm>
#include <curl/curl.h>
#include <fcntl.h>
#include <format>
#include <iostream>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#define FROM_MAIL "<matthys.arthur16@gmail.com>"
#define TO_MAIL "<matthys.arthur16@gmail.com>"
#define SUBJECT "Recap Matt Daemon"
#define SIZE_BUFF 16384

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

void sendMailRecap() {
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
