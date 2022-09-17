FROM openjdk:17

RUN mkdir /app

COPY . /app

WORKDIR /app

RUN javac FileCrawler.java

CMD java FileCrawler /dir