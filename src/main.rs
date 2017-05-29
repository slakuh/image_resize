extern crate clap;
extern crate image;
extern crate pause;
extern crate rayon;
extern crate term;
extern crate user;

mod arguments;
mod job;
mod resize;

use resize::{Resize};
use arguments::{Arguments};
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    // user::user();

    let arguments = Arguments::new();
    //println!("{:?}", arguments.images.clone());
    resize_images(&arguments);
}

pub fn resize_images(args: &Arguments ) {
    println!("\nResizing...\n");

    let is_error = AtomicBool::new(false);

    args.images.par_iter().for_each(|image| {//.map(|image| {      
        //let data = data.clone();  
        let name = image.file_name()        
            .expect("arguments::image_paths()").to_str()
            .expect("arguments::image_paths()").to_string();
        
        let mut resize = Resize::new(image.clone(), &args.job);
        if let Err(e) = resize.resize() {
            //is_error = true;
            let mut t = term::stdout().unwrap();
            t.fg(term::color::BRIGHT_RED).unwrap();
            write!(t, " ERROR: ").unwrap(); // {}\n   |--> {}\n", &name, e).unwrap();
            t.reset().unwrap();
            write!(t, "{}\n", &name).unwrap();
            t.fg(term::color::BRIGHT_RED).unwrap();
            write!(t, "   |--> ").unwrap(); // {}\n   |--> {}\n", &name, e).unwrap();
            t.reset().unwrap();
            t.fg(term::color::BRIGHT_WHITE).unwrap();
            write!(t, "{}\n", e).unwrap();
            t.reset().unwrap();
            is_error.store(true, Ordering::Relaxed);
        } else {
            println!(" resized: {}", &name);
        }
    });
        
    if is_error.load(Ordering::Relaxed) {
        pause::pause();
    }
}

