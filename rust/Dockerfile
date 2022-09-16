FROM rust:1.63

RUN mkdir /app

COPY filecrawler /app

WORKDIR /app

RUN cargo build --release

CMD /app/target/release/filecrawler /dir