use data_encoding::HEXLOWER;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs::DirEntry;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::{fs, io};

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

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    visit_dirs(&config.dir, &calculate_sha256);

    Ok(())
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
    let hash = sha256_digest(&path.path())?;

    println!("{:?} {:?}", path.path(), hash);
    Ok(())
}

pub fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry) -> io::Result<()>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)?.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                match visit_dirs(&path, cb) {
                    Err(_) => println!("Error for dir {:?}", path),
                    _ => continue,
                };
            } else {
                cb(&entry).unwrap_or_else(|err| {
                    println!("Problem with file {:?}: {}", path, err);
                });
            }
        }
    }
    Ok(())
}
