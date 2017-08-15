extern crate image;

use image::{DynamicImage, GenericImage, ImageError, ImageFormat};
use job::{Format, Job, ResizeType};

use std::fs::{File};
use std::fs;
use std::path::PathBuf;

const DEFAULT_SIZE: u32 = 1080;


//#[derive(Default)]
pub struct Resize<'a> {
    path: &'a PathBuf,
    job: &'a Job,
    width_original: u32,
    height_original: u32,
    width_old: u32,
    width_new:u32,
    height_old: u32,
    height_new: u32,
}

impl<'a> Resize<'a> {
    pub fn new(path: &'a PathBuf, job: &'a Job) -> Self {
        Resize {
            path: path,
            job: job,
            width_original: 0,
            height_original: 0,
            width_old: 0,
            width_new: 0,
            height_old: 0,
            height_new: 0,}
    }

    fn load_image(&mut self) -> Result<DynamicImage, ImageError>{
        let img = image::open(&self.path)?;
        self.width_original= img.width();
        self.height_original = img.height();
        self.width_old = img.width();
        self.height_old = img.height();
        Ok(img)
    }

    pub fn resize(&mut self) -> Result<(), ImageError> {
        let img = self.load_image()?;
        //let is_size_changed = self.calc_sizes();
        if self.resize_or_convert() {
            let resized_image = img.resize_exact(self.width_new, self.height_new, self.job.filter);
            let path_new = self.rename_image();
            let fout = &mut File::create(&path_new).unwrap();
            let _ = resized_image.save(fout, self.image_save_format());
//..... ovdje mogu dodati sa else if da promijeni naziv filea ako je slika premala ili prevelika za
        } else if self.job.rename_all {
            let path_new = self.rename_image();
            //let fout = &mut File::create(&path_new).unwrap();
            let _ = fs::copy(self.path, path_new);
        } else {
            return Err(ImageError::DimensionError);
        }
       Ok(())
    }

    /// Images with difrent format that doesn't needs to be resized
    /// will be converted to seted format (jpg or png).
    /// Returns true if image size should be resized or
    /// converted to another format.
    fn resize_or_convert(&mut self) -> bool {
        let mut to_convert = self.calc_sizes();
        let extension = self.path.extension()
            .expect("Resize::rename_image()").to_str()
            .expect("Resize::rename_image()").to_lowercase();
        // decides to convert another format to a set format (jpg or png)
        if !to_convert && extension != self.extension() {
            self.width_new = self.width_original;
            self.height_new = self.height_original;
            to_convert = true;
        }

        to_convert
    }

    /// calculates sizes for resizing and returns true if image should be resized
    fn calc_sizes(&mut self) -> bool {
        let is_size_changed;

        // dodjeljuje osnovne vrijednosti ako visina i širina nisu zadane
        if self.job.width == 0 && self.job.height == 0 {
            self.width_new = DEFAULT_SIZE;
            self.height_new  = DEFAULT_SIZE;
        }

        if self.width_new != 0 && self.height_new != 0 {
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


    fn rename_image(&self) -> PathBuf {
        let file_stem = self.path.file_stem()
            .expect("Resize::rename_image()").to_str()
            .expect("Resize::rename_image()").to_string();
        let name = file_stem + &self.job.suffix + "." + &self.extension();

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

