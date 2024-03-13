use std::{error::Error, fs::{self, File}, io::{self, BufReader, Read, Write}, path::{PathBuf}};

use chrono::Local;
use glob::glob;
use zip::{write::FileOptions, ZipArchive, ZipWriter};

use crate::CResult;

pub fn get_files(file_stores:&mut Vec<PathBuf>,files:&Vec<String>)->CResult<()>{
    for file in files{
        for entry in glob(file.as_str())?{
            file_stores.push(entry?);
        }
    }
    Ok(())
}

pub fn create_zip_file(mut output_path:PathBuf,files:& Vec<PathBuf>)->CResult<()>{

    if (&*output_path).to_str().unwrap() == "." || !((&*output_path).to_str().unwrap().contains(".zip")){
        output_path = PathBuf::from(format!("{}.zip",Local::now().to_string()));
    }
    let file = File::create(output_path)?;
    let mut zip_writer = ZipWriter::new(file);
    let opt = FileOptions::default();
    for zip_filepath in files{
        let mut orginal_file = file_open(zip_filepath)?;
        if zip_filepath.is_dir(){
            zip_writer.add_directory(zip_filepath.to_string_lossy().into_owned(), opt)?;
        }else{
            let filename = zip_filepath.file_name().unwrap().to_string_lossy().into_owned();
            zip_writer.start_file(filename,opt)?;
            let mut buffer = vec![];
            orginal_file.read_to_end(&mut buffer)?;
            zip_writer.write_all(&buffer)?;
            zip_writer.flush()?;
        }
    }
    zip_writer.finish()?;

    Ok(())
}

pub fn unzip(mut output_path:PathBuf,zip_filepaths:&Vec<PathBuf>)->CResult<()>{
    for zip_filepath in zip_filepaths{
        let zip_file = file_open(&zip_filepath)?;
        let mut archives = ZipArchive::new(zip_file)?;
        for i in 0..archives.len(){
            let mut file = archives.by_index(i)?;
            let mut utf8_filename = "".to_string();
            let output_pathbuf = output_path.join(&file.name());
            println!("{:?}",file.header_start());
            if (&*file.name()).ends_with('/'){
                fs::create_dir_all(output_pathbuf)?;
            }else{
                let mut outfile = File::create(output_pathbuf)?;
                io::copy(&mut file, &mut outfile)?;
            }
        }
    
    }

    Ok(())
}


fn file_open(path:&PathBuf)->CResult<Box<BufReader<File>>>{
    match path {
        //"-"=>Ok((Box::new(BufReader::new(io::stdin())))),
        _=>Ok(Box::new(BufReader::new(File::open(path)?))),
    }
}
