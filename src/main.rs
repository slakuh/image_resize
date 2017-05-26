extern crate clap;
extern crate image;
extern crate pause;
extern crate user;

mod arguments;
mod job;
mod resize;

use resize::{Resize};
use arguments::{Arguments};
use std::path::PathBuf;

fn main() {
    // user::user();
    let path = PathBuf::from(r"C:\proj\image_resize\target\debug\test.jpg");
    let arguments = Arguments::new();
    let mut resize = Resize::new(path, &arguments.job);
    if let Err(e) = resize.resize() {
        println!("{}", e)
    };

    //resize.resize();

    //pause::pause();
}
