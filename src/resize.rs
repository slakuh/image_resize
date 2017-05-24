extern crate image;

use arguments::Arguments;
use image::{DynamicImage, GenericImage, ImageResult};
use job::{Job, ResizeType};
use std::path::PathBuf;

const DEFAULT_SIZE: u32 = 1080;

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
    pub fn new(path: PathBuf, job: &'a Job) -> ImageResult<Self> {
        Ok(Resize {
            path: path, 
            job: job,
            width_old: 0,
            width_new: 0,
            height_old: 0,
            height_new: 0,})        
    }

    fn load_image(&mut self) -> ImageResult<DynamicImage>{
        let mut img = image::open(&self.path)?;
        self.width_old = img.width();
        self.height_old = img.height();
        Ok(img)
    }

    pub fn resize(&mut self) -> ImageResult<()> {
        let img = self.load_image()?;
        let is_size_changed = self.calc_size();



       Ok(())
    }

    /// returns true if image should be resized
    fn calc_size(&mut self) -> bool {
        let mut is_size_changed = false;

        // dodjeljuje osnovne vrijednosti za visinu i širinu
        // ako nisu zadane
        
        if self.job.width == 0 && self.job.height == 0 {
            self.width_new = DEFAULT_SIZE;
            self.height_new = DEFAULT_SIZE; 
        }

        if self.job.width != 0 && self.job.height != 0 {
            if self.width_old > self.height_old {
                is_size_changed = self.resize_to_width();
            } else {
                is_size_changed = self.resize_to_height();
            }

        } else if self.job.width != 0 {
            is_size_changed = self.resize_to_width();
        } else if self.job.height != 0 {
            is_size_changed = self.resize_to_height();
        } else {
            unreachable!("resize::calc_size()");
        }

        is_size_changed

    }

    fn resize_to_width(&mut self) -> bool {
        let calc_new_size;
        match self.job.resize {
            ResizeType::Increase => calc_new_size = self.width_old < self.width_new,
            ResizeType::Decrease => calc_new_size = self.width_old > self.width_new,
            ResizeType::Eather => calc_new_size = true,
            ResizeType::Neither => calc_new_size = false,
        }

        if calc_new_size {
            self.height_new = self.width_new * self.height_old / self.width_old;
        }
        calc_new_size
    }

     fn resize_to_height(&mut self) -> bool {
        let calc_new_size;// = false;
        match self.job.resize {
            ResizeType::Increase => calc_new_size = self.height_old < self.height_new,
            ResizeType::Decrease => calc_new_size = self.height_old > self.height_new,
            ResizeType::Eather => calc_new_size = true,    
            ResizeType::Neither => calc_new_size = false,                   
        }

        if calc_new_size {
            self.width_new = self.height_new * self.width_old / self.height_old;            
        }
        calc_new_size
    }

    fn rename_image(&self) {
        unimplemented!()
    }
}

pub fn resize_images(args: &Arguments ) {
    unimplemented!()
}