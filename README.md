# Filecrawler

The goal of this repository is to implement a simple program in different programming languages to learn more about them.

The program to implement is a file crawler that given a directory will iterate recursively in it and calculate the SH1-256 of the files it find.

## Implementations

- [x] python
- [x] golang
- [x] rust

## Run

First, create a `.env` file and add the var `DIR` with a path to the directory on your file system you would like to crawl.

Then, using **docker-compose**, you can run any implementations.

```bash
docker-compose run python
docker-compose run golang
docker-compose run rust
```
