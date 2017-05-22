extern crate clap;
extern crate image;
extern crate pause;
extern crate user;

mod arguments;
mod job;

use arguments::{Arguments};

fn main() {
    // user::user();
    let arguments = Arguments::new();

    pause::pause();
}
