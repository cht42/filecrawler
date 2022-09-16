use filecrawler::Config;
use std::time::Instant;
use std::{env, process};

fn main() {
    let now = Instant::now();

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    println!("Crawling {:?}", config.dir);

    if let Err(e) = filecrawler::run(&config) {
        eprintln!("Application error: {e}");

        process::exit(1);
    }

    let elapsed = now.elapsed();
    println!("Took {:?} to crawl {:?}", elapsed, &config.dir)
}
