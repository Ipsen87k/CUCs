use std::{fs::File, io::{self, BufRead, BufReader}};

use clap::{Arg, Command};

use crate::{cplay_core::Sound, error::Error, CResult};


#[derive(Debug)]
pub struct Config{
    files:Vec<String>,
}

pub fn create_args()->Config{
    let matches = Command::new("cplay")
        .version("0.1.0")
        .author("Ipsen87k")
        .about("play music")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .default_value("")
                .num_args(1..)
        )
        .get_matches();

    Config{
        files:matches.get_many("files").unwrap().cloned().collect(),
    }
}

pub fn run(config:Config) -> CResult<()>{
    let mut sound = Sound::new()?;
    for filename in config.files{
        match open(&filename) {
            Err(e) => eprintln!("{filename}:{e:?}"),
            Ok(f) => {
                sound.append(f)?;
            },
        }
    }
    sound.play();
    Ok(())
}

fn open(filename:&str)->CResult<Box<BufReader<File>>>{
    match filename {
        //""=>Ok(Box::new(BufReader::new(io::stdin()))),
        _=>Ok(Box::new(BufReader::new(File::open(filename).map_err(|e| Error::IoError(e))?)))
    }
}