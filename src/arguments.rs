use clap::{Arg, App, AppSettings};
use image::FilterType;
use job::{Format, Job, JobBuilder, ResizeType};
use std::env;
use std::path::PathBuf;
use pause;

const SUPPORTED_IMAGES: [&str; 7] = ["jpg", "png", "jpeg", "gif", "bmp", "tif", "tiff"];
const SUPPORTED_SAVES: [&str; 2] = ["jpg", "png"];
const FILTERS: [&str; 5] = ["nn", "lf", "cf", "gf", "l"];
const RESIZE_TYPE: [&str; 4] = ["inc", "dec", "non", "iod"];

pub struct Arguments {
    pub job: Job,
    pub images: Vec<PathBuf>,
}

impl Arguments {

    pub fn new() -> Self {
        Arguments {
            job: Arguments::job_from_clap(),
            images: Arguments::image_paths(),
        }
    }

    fn job_from_clap() -> Job {
        let matches = App::new("\nImage Resize")
                                .version("0.1.0")
                                .author("Slaven Kuhinek © 2017")
                                .about("\nImage Resize je program za smanjivanje fotografija \
                                koji korisiti naredbeni redak za unos parametara za promjenu veličine fotografija.")
                                .settings(&[
                                    AppSettings::WaitOnError, 
                                    AppSettings::ColorAlways,
                                    AppSettings::AllowExternalSubcommands,
                                    
                                    ])
                                .arg(Arg::with_name("width")
                                    .short("W")
                                    .long("width")
                                    .value_name("NUMBER")
                                    .help("Sets a width of resized image. \
                                    Image will be resized proportionaly to this value.")
                                    .takes_value(true))
                                .arg(Arg::with_name("height")
                                    .short("H")
                                    .long("height")
                                    .value_name("NUMBER")
                                    .help("Sets a height of resized image. \
                                    Image will be resized proportionaly to this value.")
                                    .takes_value(true))
                                .arg(Arg::with_name("format")
                                    .short("f")
                                    .long("format")
                                    .value_name("STRING")
                                    .help("Sets a format in witch resized image will be saved to a disk. \
                                    Accepted format are jpg or png.")
                                    .takes_value(true)
                                    .possible_values(&SUPPORTED_SAVES))
                                .arg(Arg::with_name("filter")
                                    .short("F")
                                    .long("filter")
                                    .value_name("STRING")
                                    .help("Sets an image resize filter. Filters are: \n\
                                    nn - Nearest Neighbor,\n\
                                    lf - Linear Filter,\n\
                                    cf - Cubic Filter,\n\
                                    gf - Gaussian Filter,\n\
                                    l  - Lanczos3\n")
                                    .takes_value(true)
                                    .possible_values(&FILTERS))
                                .arg(Arg::with_name("resize")
                                    .short("r")
                                    .long("resize")
                                    .value_name("STRING")
                                    .help("Sets an option for resize. Options are:\n\
                                    inc - Image size will be increased but not decreased,\n\
                                    dec - Image size will be decreased but not increased,\n\
                                    non - Image size will stay unchanged, \
                                    can be used for converting to another image format\n\
                                    iod - Image size will be increased or decreased to specified size,\n")
                                    .takes_value(true)
                                    .possible_values(&RESIZE_TYPE))
                                .get_matches();

        
        let mut job = Job::new();
           

        if let Some(width) = matches.value_of("width") {
            match width.parse::<u32>() {
                Ok(w) => job.change_width(w),
                Err(_) => println!("Value \"{}\" for image WIDTH isn't number. \
                Using a default value \"{}\".", width, job.width),
            }
        } 

        if let Some(height) = matches.value_of("height") {
            match height.parse::<u32>() {
                Ok(h) => job.change_height(h),
                Err(_) => println!("Value \"{}\" for image HEIGHT isn't number. \
                Using a default value \"{}\".", height, job.height),
            }
        } 

        if let Some(format) = matches.value_of("format") {
            //pause::print(format!("{}",format));
            //println!("{}", format);
            match format {
                "jpg" => job.change_format(Format::Jpeg),
                "png" => job.change_format(Format::Png),
                _ => unreachable!("while matching format in Arguments::job_from_clap()"),
            }
        } 

        if let Some(filter) = matches.value_of("filter") {
            //println!("{}", filter);
            match filter {
                "nn" => job.change_filter(FilterType::Nearest),
                "lf" => job.change_filter(FilterType::Triangle),
                "cf" => job.change_filter(FilterType::CatmullRom),
                "gf" => job.change_filter(FilterType::Gaussian),
                "l" => job.change_filter(FilterType::Lanczos3),                                
                _ => unreachable!("while matching filters in Arguments::job_from_clap()"),
            }
        } 

        if let Some(resize) = matches.value_of("resize") {
            //println!("{}", filter);
            match resize {
                "inc" => job.change_resize(ResizeType::Increase),
                "dec" => job.change_resize(ResizeType::Decrease),
                "non" => job.change_resize(ResizeType::Neither),
                "iod" => job.change_resize(ResizeType::Eather),                               
                _ => unreachable!("while resize type in Arguments::job_from_clap()"),
            }
        }
        //pause::print(format!("{}, {}", job.width, job.height));
        job
    }

    pub fn from(builder: JobBuilder) -> Self {
        Arguments {
            job: builder.execute(),
            images: Arguments::image_paths(),
        }
        
    }

    fn image_paths() -> Vec<PathBuf> {
        //pause::pause();
        let mut images: Vec<PathBuf> = Vec::new();
        for argument in env::args().skip(1) {
            
            let path = PathBuf::from(argument);
            
            if path.is_file() {
                let file_extension = path.extension()
                    .expect("arguments::image_paths()").to_str()
                    .expect("arguments::image_paths()").to_lowercase();
          
                for item in &SUPPORTED_IMAGES {
                    if item == &file_extension {
                        images.push(path.clone());
                        
                    }
                }
                            
            }          
        }
        images
    }
}