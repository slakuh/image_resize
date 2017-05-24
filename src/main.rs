extern crate clap;
extern crate image;
extern crate pause;
extern crate user;

mod arguments;
mod job;
mod resize;

use resize::Resize;
use arguments::{Arguments};

fn main() {
    // user::user();
    let arguments = Arguments::new();
    let resize = Resize::new(arguments.images[0].clone(), &arguments.job);

    pause::pause();
}
