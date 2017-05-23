use arguments::Arguments;
use image::FilterType;
use job::{Format, Job, /*JobBuilder,*/ ResizeType};
use std::path::PathBuf;

#[derive(Default)]
struct Resize {
    path: PathBuf,
    width_old: u32,
    widith_new:u32,
    height_old: u32,
    height_new: u32,    
}

impl Resize {
    fn new(path: PathBuf, job: &Job) -> Self {
        let mut resize = Resize {path: path, ..Default::default()};
        resize
    }


    fn resize(&self, job: &Job) {
        unimplemented!()
    }

    fn new_size(sizes: Resize) -> Resize {
        unimplemented!()
    }
}

pub fn resize_images(args: &Arguments ) {
    unimplemented!()
}