FROM node:16.13

RUN mkdir /app

COPY . /app

WORKDIR /app

RUN npm install

CMD ./node_modules/ts-node/dist/bin.js filecrawler.ts /dir