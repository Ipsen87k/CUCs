use std::{error::Error, ffi::OsString, fs::File, io::{Read, Seek, SeekFrom}, str::FromStr};

use assert_cmd::Command;
use clap::builder;


type MResult = Result<(),Box<dyn Error>>;
const UTF8_NAME_ZIP_FILE :&str= "tests/日本語てすと.zip";
const APP_NAME:&str = "zipr";
const ZIP_HEADER_BYTES:[u8;4] = [0x50, 0x4b, 0x03, 0x04];
#[test]
fn unzip_test()->MResult{
    Command::cargo_bin(APP_NAME)?
        .args(["-u",UTF8_NAME_ZIP_FILE])
        .assert()
        .failure();
    Ok(())
}

#[test]
fn unicode()->MResult{
    let mut file = File::open(UTF8_NAME_ZIP_FILE)?;
    loop {
        let mut signature = [0;4];
        file.read(&mut signature)?;

        if signature == ZIP_HEADER_BYTES{
            println!("sig = {:?}",signature);
            break;
        }
    }

    file.seek(SeekFrom::Current(4))?;

    let comp_method = {
        let mut buf=[0u8;2];
        file.read(&mut buf)?;
        let comp = u16::from_le_bytes(buf);
        println!("comp = {}",comp);
    };

    file.seek(SeekFrom::Current(8))?;

    let comp_size = {
        let mut buf = [0u8;4];
        file.read(&mut buf)?;
        let size = u32::from_le_bytes(buf);
        println!("size = {}",size);
    };

    let uncomp_size={
        let mut buf = [0u8;4];
        file.read(&mut buf)?;
        let uncomp_size=u32::from_le_bytes(buf);
        println!("uncomp_size = {}",uncomp_size);
    };

    let name_length = {
        let mut buf = [0u8;2];
        file.read(&mut buf)?;
        let name_len = u16::from_le_bytes(buf) as usize;
        println!("name length = {}",name_len);

        name_len
    };

    let extra_field_length = {
        let mut buf = [0u8; 2];
        file.read(&mut buf)?;
        u16::from_le_bytes(buf) as usize
    };

    let filename = {
        let mut buf = vec![0u8;name_length];
        file.read_exact(&mut buf)?;

        let name = String::from_utf8(buf)?;
        println!("name = {}",name);
    };
    Ok(())
}