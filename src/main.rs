#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate clap;
extern crate image;
extern crate pause;
extern crate rayon;
extern crate term;
extern crate user;

mod arguments;
mod job;
mod resize;

use arguments::Arguments;
use image::ImageError;
use resize::Resize;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    user::user();
    let arguments = Arguments::new();
    resize_images(&arguments);
}

pub fn resize_images(args: &Arguments ) {

    let is_error = AtomicBool::new(false);

    args.images.par_iter().for_each(|path| {        
        let name = path.file_name()        
            .expect("main::resize_images()").to_str()
            .expect("main::resize_images()").to_string();
        
        let mut resize = Resize::new(path, &args.job);
        if let Err(e) = resize.resize() {
            is_error.store(true, Ordering::Relaxed);
            match e {
                ImageError::DimensionError => error("WARNING", &name, &e, term::color::BRIGHT_YELLOW),
                _ => error("ERROR", &name, &e, term::color::BRIGHT_RED),
            }
        } else {
            println!(" resized: {}", &name);
        }
    });

    // pausing execution of program on error or warning.
    if is_error.load(Ordering::Relaxed) {
        pause::pause();
    }
}

fn error(error: &str, file_name: &str, e: &ImageError, color: u16){
    let mut t = term::stdout().unwrap();
    t.fg(color).unwrap();
    write!(t, "\n {}: ", error).unwrap();
    t.reset().unwrap();
    write!(t, "{}\n", file_name).unwrap();
    t.fg(color).unwrap();
    write!(t, " |---> ").unwrap();
    t.reset().unwrap();
    t.fg(term::color::BRIGHT_WHITE).unwrap();
    write!(t, "{}\n\n", e).unwrap();
    t.reset().unwrap();
}
