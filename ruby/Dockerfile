FROM ruby:3.1.2

RUN mkdir /app

COPY . /app

WORKDIR /app

COPY . .

CMD ruby filecrawler.rb /dir