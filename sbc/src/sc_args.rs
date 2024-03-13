use clap::{Arg, ArgAction, Command};

use crate::CResult;



pub struct Config{
    strings:Vec<String>,
    is_count:bool,
    is_byte:bool,
}

pub fn create_args()->CResult<Config>{
    let matches= Command::new("sc")
        .about("String Count")
        .author("Ipsen87k")
        .version("0.1.0")
        .arg(
            Arg::new("strings")
                .value_name("STRINGS")
                .default_value("")
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("Show strings count")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("byte")
                .short('b')
                .long("byte")
                .help("Show strings bytes")
                .action(ArgAction::SetTrue)
        )
        .get_matches();
    Ok(
        Config{
            strings:matches.get_many("strings").unwrap().cloned().collect(),
            is_count:matches.get_flag("count"),
            is_byte:matches.get_flag("byte"),
        }
    )
}

pub fn run(config:Config)->CResult<()>{
    for str in &config.strings{
        println!("{}",str);
        if config.is_count{
            println!("count = {}",str.chars().count());
        }
        if config.is_byte{
            println!("utf8 bytes = {:?}",str.len());
            println!("{:?}",str.as_bytes());
        }
        println!("---------------");
    }
    Ok(())
}