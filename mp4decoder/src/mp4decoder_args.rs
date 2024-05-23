use std::{ffi::OsStr, path::Path};

use clap::{Arg, Command};

use crate::{mp4decoder_core::{chnage_extension, convert_mp4_to_mp3}, CResult};


pub struct Config{
    file:String,
    output:String,

}

pub fn create_args()->CResult<Config>{
    let matches = Command::new("mp4dec")
        .about("mp4decoder mp3")
        .author("Ipsen87K")
        .version("0.1.0")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .required(true)
                .help("Input File")
        )
        .arg(
            Arg::new("output")
                .value_name("OUTPUT")
                .help("Input OutputPath")
                .short('o')
                .long("output")
                .default_value(".")
        )
        .get_matches();


    Ok(
        Config{
            file:matches.get_one("file").cloned().unwrap(),
            output:matches.get_one("output").cloned().unwrap(),
        }
    )
}

pub fn run(config:Config)->CResult<()>{
    let output_path = output_path_check(&config.file,&config.output);
    println!("outputpath - {}",output_path);
    convert_mp4_to_mp3(&config.file, &output_path)?;
    Ok(())
}

fn output_path_check(filepath:&str,output_path:&str)->String{
    let path = Path::new(filepath);
    let output_filepath=Path::new(output_path);
    

    if output_path == "."{
        format!("{}.{}",path.file_stem().unwrap().to_str().unwrap(),"mp3")
    }
    else if !output_path.contains("mp3"){
        if let Some(_name) = output_filepath.file_name(){
            println!("{}",_name.to_str().unwrap());
            chnage_extension(conv_osstr_to_str(path.file_stem()),output_path, "mp3")
        }else{
            format!("{}.{}",output_filepath.join(path.file_name().unwrap()).to_str().unwrap(),"mp3")
        }
    }else{
        format!("{}",output_path)
    }
}

fn conv_osstr_to_str(target_osstr:Option<&OsStr>)->&str{
    target_osstr.unwrap().to_str().unwrap()
}