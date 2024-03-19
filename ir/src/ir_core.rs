use std::path::Path;

use image::{imageops, DynamicImage, GenericImageView};

use crate::CResult;

#[derive(Clone)]
pub struct ImgEdit{
    image:DynamicImage,
    result_image:Option<DynamicImage>,
}

impl ImgEdit {
    pub fn new<P>(filepath:P)->Self
    where
        P:AsRef<Path>
    {
        Self { 
            image: image::open(filepath).expect("Cannot file open"),
            result_image:None
        }
    }

    pub fn resize(&mut self,width:u32,height:u32)->CResult<()>{
        let resize_img = self.image.resize(width, height, imageops::FilterType::Lanczos3);
        self.result_image = Some(resize_img);
        Ok(())
    }

    pub fn get_dimension(&self)->(u32,u32){
        self.image.dimensions()
    }

    pub fn save<P>(&mut self,save_path:P)->CResult<()>
    where 
        P:AsRef<Path>
    {
        if let Some(result_img) = &self.result_image{
            result_img.save(save_path)?;
        }

        Ok(())
    }
}

