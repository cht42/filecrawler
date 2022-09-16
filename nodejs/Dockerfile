FROM node:16.13

RUN mkdir /app

COPY . /app

WORKDIR /app

RUN npm install

CMD node filecrawler.js /dir