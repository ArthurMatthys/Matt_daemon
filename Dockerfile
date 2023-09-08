FROM ubuntu:latest
RUN apt-get update && apt-get install -y g++ libcurl4-openssl-dev libfmt-dev make netcat net-tools vim
WORKDIR /app
COPY . .
RUN bash

