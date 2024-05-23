
use std::{path::{Path, PathBuf}, process::Command};

use crate::CResult;

pub fn convert_mp4_to_mp3(filepath:&str,output_path:&str)->CResult<()>{
    let output=Command::new("./resource/ffmpeg.exe")
        .args(&[
            "-i",filepath,
            "-vn",
            "-b:a","256k",
            "-f","mp3",
            output_path,
        ])
        .output()?;
    if output.status.success(){
        println!("sucess!!");
    }else{
        println!("not success!!");
    }
    Ok(())
}

pub fn chnage_extension(filename:&str,output:&str,ext:&str)->String{
    let path = Path::new(&output);
    if output.ends_with("/"){
        let filepathbuf = path.join(filename);
        format!("{}.{}",filepathbuf.to_str().unwrap(),ext)
    }else{
        format!("{}.{}",output,ext)
    }
}