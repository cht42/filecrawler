use data_encoding::HEXLOWER;
use dpc_pariter::IteratorExt;
use sha2::{Digest, Sha256};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::{fs, io};
use walkdir::{DirEntry, WalkDir};

pub struct Config {
    pub dir: PathBuf,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let dir = String::from(args[1].clone());
        let dir = PathBuf::from(&dir);

        Ok(Config { dir })
    }
}

pub fn run(config: &Config) -> Result<(), io::Error> {
    visit_dirs(&config.dir)
}

fn sha256_digest(path: &PathBuf) -> io::Result<String> {
    let input = fs::File::open(path)?;
    let mut reader = BufReader::new(input);

    let digest = {
        let mut hasher = Sha256::new();
        let mut buffer = [0; 30 * 1024];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        hasher.finalize()
    };
    Ok(HEXLOWER.encode(digest.as_ref()))
}

pub fn calculate_sha256(path: &DirEntry) -> io::Result<()> {
    let hash = sha256_digest(&path.path().to_path_buf())?;

    println!("{:?} {:?}", path.path(), hash);
    Ok(())
}

pub fn visit_dirs(dir: &Path) -> io::Result<()> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .parallel_map_custom(
            |o| o.threads(16),
            |entry| {
                if entry.metadata().unwrap().is_file() {
                    match calculate_sha256(&entry) {
                        Err(err) => println!("Problem with file {:?}: {}", entry, err),
                        _ => (),
                    }
                }
            },
        )
        .for_each(drop);

    Ok(())
}
