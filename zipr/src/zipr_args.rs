use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgAction, Command};
use zipr::zipr;


pub(crate) type CResult<T> = Result<T,Box<dyn Error>>; 
//const ZIP_FILE_SIGNATURE:[u8;4]= [0x50,0x4b,0x03,0x04];


#[derive(Debug)]
pub struct Config{
    files:Vec<String>,
    is_create:bool,
    is_unzip:bool,
    output_path:String,
}

pub fn run(config:Config)->CResult<()>{
    println!("{:?}",&config);
    let mut files = vec![];
    zipr::get_files(&mut files, &config.files)?;

    for f in &files{
        println!("{}",f.display());
    }

    let mut output_pathbuf = PathBuf::from(&config.output_path);

    if config.is_unzip{
        zipr::unzip(output_pathbuf, &files)?;
    }else if config.is_create {
        zipr::create_zip_file(output_pathbuf, &files)?;
    }

    Ok(())
}

pub fn args()->CResult<Config>{
    let args = Command::new("zipr")
        .about("zip")
        .author("Ipsen87k")
        .version("0.1.0")
        .args(
            [Arg::new("files")
                .value_name("FILES")
                .help("Input file(s)")
                .default_value(".")
                .num_args(0..)]
        )
        .args(
            [Arg::new("unzip")
                .short('u')
                .action(ArgAction::SetTrue)
                .help("Unzip zip files")
                .conflicts_with("create")]
        )
        .args(
            [Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .help("Input output path")
                .default_value(".")
                .action(ArgAction::Append)]
        )
        .args(
            [Arg::new("create")
                .short('c')
                .long("create")
                .action(ArgAction::SetTrue)
                .help("Create zip file")]
        )
        .get_matches();
    
    Ok(Config{
        files:args.get_many("files").unwrap().cloned().collect(),
        is_create:args.get_flag("create"),
        is_unzip:args.get_flag("unzip"),
        output_path:args.get_one("output").cloned().unwrap(),
    })
}
