FROM ubuntu:20.04

RUN apt update && apt install -y build-essential clang libssl-dev

RUN mkdir /app

COPY . /app

WORKDIR /app

RUN make

CMD ./filecrawler /dir


