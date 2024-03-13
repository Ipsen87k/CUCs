use std::{ffi::OsStr, io::{self, Write}, path::{Path, PathBuf}, thread};

use clap::{Arg, Command};

use crate::{chrono_wrap, ir_core, spinner, util, CResult};

pub struct Config{
    file:String,
    output:String,
    width:String,
    height:String,
}

pub fn create_args()->CResult<Config>{
    let matches = Command::new("ir")
        .about("Image Resize")
        .author("Ipsen87k")
        .version("0.1.0")
        .disable_help_flag(true)
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input file")
                .required(true)
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .default_value(".")
                .value_name("OUTPUT")
                .help("Input Output path")
        )
        .arg(
            Arg::new("width")
                .short('w')
                .long("width")
                .value_name("WIDTH")
                .help("Input width value")
                .default_value("0")
        )
        .arg(
            Arg::new("height")
                .short('h')
                .long("height")
                .value_name("HEIGHT")
                .help("Input height value")
                .default_value("0")
        )
        .get_matches();
    

    Ok(
        Config{
            file:matches.get_one("file").cloned().unwrap(),
            output:matches.get_one("output").cloned().unwrap(),
            width:matches.get_one("width").cloned().unwrap(),
            height:matches.get_one("height").cloned().unwrap(),
        }
    )
}

fn get_save_filename<P>(save_path:PathBuf,save_filename:P)->PathBuf
where
    P:AsRef<Path>
{
    save_path.join(save_filename)
}


pub fn run(config:Config)->CResult<()>{
    let width = config.width.clone().parse::<u32>()?;
    let height = config.height.clone().parse::<u32>()?;

    let (img_edit,img_edit_clone)=util::create_arc_mutex(ir_core::ImgEdit::new(&config.file));
    let (img_execute_end,img_execute_end_clone )= util::create_arc_mutex(false);

    let handle = thread::spawn(move ||{
        let mut locked_img_edit = img_edit_clone.lock().unwrap();
        locked_img_edit.resize(width, height).unwrap();
        let mut locked_img_execute_end = img_execute_end_clone.lock().unwrap();
        *locked_img_execute_end = true;
    });

    spinner::spinner_show(img_execute_end);
    handle.join().unwrap();

    let binding = PathBuf::from(&*config.file);
    let filepath = binding.as_path();
    let filename = format!("{}_{}.{}",&filepath.file_stem().unwrap().to_string_lossy(),chrono_wrap::get_timestamp(),&filepath.extension().unwrap().to_string_lossy());


    let mut locked_img_edit = img_edit.lock().unwrap();
    locked_img_edit.save(get_save_filename(PathBuf::from(&config.output), filename))?;

    Ok(())
}