FROM golang:1.19

RUN mkdir /app

COPY . /app

WORKDIR /app

RUN go build -o filecrawler .

CMD ./filecrawler /dir