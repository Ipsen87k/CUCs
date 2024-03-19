use std::{error::Error, ffi::OsString, fs::File, io::{BufReader, Read, Seek, SeekFrom}, str::FromStr};

use assert_cmd::Command;
use clap::builder;
use zip::{unstable::stream::ZipStreamFileMetadata, ZipArchive};


type MResult = Result<(),Box<dyn Error>>;
const UTF8_NAME_ZIP_FILE :&str= "tests/ごはん.zip";
const APP_NAME:&str = "zipr";
const ZIP_HEADER_BYTES:[u8;4] = [0x50, 0x4b, 0x03, 0x04];
const ZIP_CENTRAL_DIR_EOF:[u8;4] = [0x50,0x4b,0x05,0x06];
#[test]
fn unzip_test()->MResult{
    Command::cargo_bin(APP_NAME)?
        .args(["-u",UTF8_NAME_ZIP_FILE])
        .assert()
        .failure();
    Ok(())
}

#[test]
fn zipreader_test()->MResult{
    let mut file = File::open(UTF8_NAME_ZIP_FILE)?;
    let mut zwriter = ZipArchive::new(file)?;
    for i in 0..zwriter.len(){
        let zip_archive_file = zwriter.by_index(i)?;
        println!("name = {}",zip_archive_file.enclosed_name().unwrap().display());
        println!("utf8 bytes = {}",zip_archive_file.name().len());
    }
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

#[test]
fn get_filename()->MResult{
    let file = File::open(UTF8_NAME_ZIP_FILE)?;
    let mut bufreader = BufReader::new(file);
    let mut central_dir_eof_sig = [0u8;4];
    loop {
        bufreader.read(&mut central_dir_eof_sig)?;

        if central_dir_eof_sig == ZIP_CENTRAL_DIR_EOF{
            break;
        }
    }
    //bufreader.seek(SeekFrom::Current(26))?;
    let mut buf = [0u8;2];
    bufreader.read(&mut buf)?;

    let len = u16::from_le_bytes(buf) as usize;
    println!("len = {}",len);

    //bufreader.seek(SeekFrom::Current(2))?;
    // let mut ext_buf = [0u8;2];
    // bufreader.read(&mut ext_buf)?;
    // let ext_len = u16::from_le_bytes(ext_buf) as usize;
    // println!("ext_len = {}",ext_len);

    // let mut name_buf = vec![0u8;len];
    // bufreader.read(&mut name_buf)?;

    // let name = String::from_utf8(name_buf)?;
    // println!("utf 8 name = {}",name);
    Ok(())
}