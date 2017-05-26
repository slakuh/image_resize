extern crate image;

use arguments::Arguments;
use image::{DynamicImage, GenericImage, ImageError, ImageFormat, ImageResult};
use job::{Format, Job, ResizeType};
use std::fs::{self, File};
use std::path::PathBuf;

const DEFAULT_SIZE: u32 = 1080;
const SUFFIX: &str = "-m";

//#[derive(Default)]
pub struct Resize<'a> {
    path: PathBuf,
    job: &'a Job,
    width_old: u32,
    width_new:u32,
    height_old: u32,
    height_new: u32,    
}

impl<'a> Resize<'a> {
    pub fn new(path: PathBuf, job: &'a Job) -> Self {
        Resize {
            path: path, 
            job: job,
            width_old: 0,
            width_new: 0,
            height_old: 0,
            height_new: 0,}  
    }

    fn load_image(&mut self) -> Result<DynamicImage, ImageError>{
        let mut img = image::open(&self.path)?;
        self.width_old = img.width();
        self.height_old = img.height();
        Ok(img)
    }

    pub fn resize(&mut self) -> Result<(), ImageError> {
        let img = self.load_image()?;
        //let is_size_changed = self.calc_sizes();
        if self.calc_sizes() {
            println!("{} x {}", self.width_new, self.height_new);
            let resized_image = img.resize_exact(self.width_new, self.height_new, self.job.filter);
            let path_new = self.rename_image();
            let ref mut fout = File::create(&path_new).unwrap();
            let _ = resized_image.save(fout, self.image_save_format());
        } else {
            println!("Didn't resize image.");
        }



       Ok(())
    }

    /// calculates sizes for resizing and returns true if image should be resized
    fn calc_sizes(&mut self) -> bool {
        let mut is_size_changed = false;

        //let mut self.job.width = self.job.width;
        //let mut self.job.height = self.job.height;
        // dodjeljuje osnovne vrijednosti ako visina i širina nisu zadane                
        if self.job.width == 0 && self.job.height == 0 {
            self.width_new = DEFAULT_SIZE;
            self.height_new  = DEFAULT_SIZE; 
            println!("1");
        }

        if self.width_new != 0 && self.height_new != 0 {
            println!("2");
            if self.width_old > self.height_old {
                is_size_changed = self.resize_to_width();
            } else {
                is_size_changed = self.resize_to_height();
            }
        } else if self.job.width != 0 {
            self.width_new = self.job.width;
            is_size_changed = self.resize_to_width();
        } else if self.job.height != 0 {
            self.height_new = self.job.height;
            is_size_changed = self.resize_to_height();
        } else {
            unreachable!("Resize::calc_sizes()");
        }

        is_size_changed
    }

    fn resize_to_width(&mut self) -> bool {
        let calc_new_size = match self.job.resize {
            ResizeType::Increase => self.width_old < self.width_new,
            ResizeType::Decrease => self.width_old > self.width_new,
            ResizeType::Eather => true,
            ResizeType::Neither => false,
        };

        if calc_new_size {
            self.height_new = self.width_new * self.height_old / self.width_old;
        }
        calc_new_size
    }

     fn resize_to_height(&mut self) -> bool {
        let calc_new_size = match self.job.resize {
            ResizeType::Increase => self.height_old < self.height_new,
            ResizeType::Decrease => self.height_old > self.height_new,
            ResizeType::Eather => true,    
            ResizeType::Neither => false,                   
        };

        if calc_new_size {
            self.width_new = self.height_new * self.width_old / self.height_old;            
        }
        calc_new_size
    }

    fn image_save_format(&self) -> ImageFormat {
        match self.job.format {
            Format::Jpeg => ImageFormat::JPEG,
            Format::Png => ImageFormat::PNG,
        }
    }

    // extenzija prema save formatu
    fn rename_image(&self) -> PathBuf {        
        let mut file_stem = self.path.file_stem()
            .expect("Resize::rename_image()").to_str()
            .expect("Resize::rename_image()").to_string();
            /*
        let file_extension = self. path.extension()
            .expect("Resize::rename_image()").to_str()
            .expect("Resize::rename_image()").to_string();
        */
        let name = file_stem + SUFFIX + "." + &self.extension();

        let mut path_new = self.path.clone();
        path_new.set_file_name(&name);

        path_new        
    }

    fn extension(&self) -> String {        
        match self.job.format {
            Format::Jpeg => String::from("jpg"),
            Format::Png => String::from("png"),
        }
    }
}



pub fn resize_images(args: &Arguments ) {
    unimplemented!()
}