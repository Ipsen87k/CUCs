use std::{error::Error, path::{Path, PathBuf}};

use ir::spinner;




type CResult = Result<(),Box<dyn Error>>;
const IMGAE_PATH_1:&str="tests/test_4919_2763.jpeg";
const RESULT_PATH:&str = "tests/result";

fn get_result_path<P> (filename:P)->PathBuf
where
    P:AsRef<Path>
{
    let path = PathBuf::from(RESULT_PATH);
    path.join(filename)
}

#[test]
fn image_resize_test()->CResult{
    let img = image::open(IMGAE_PATH_1)?;
    let resize_img = img.resize(1920, 1080, image::imageops::FilterType::Lanczos3);
    
    resize_img.save(get_result_path("test1.jpeg"))?;
    Ok(())
}

#[test]
fn spinner_test()->CResult{
    //spinner::spinner_show(0);

    Ok(())
}
