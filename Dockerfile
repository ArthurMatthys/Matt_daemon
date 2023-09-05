FROM ubuntu:latest
RUN apt-get update && apt-get install -y g++ libcurl4-openssl-dev libfmt-dev make netcat
WORKDIR /app
COPY . .
RUN bash

