use image::FilterType;
// osnovne vrijednosti za Job struct
const FORMAT: Format = Format::Jpeg;
const FILTER: FilterType = FilterType::Nearest;
const RESIZE: ResizeType = ResizeType::Decrease;
const SUFFIX: &str = "-m";
const RENAME_ALL: bool = false;

/// Izlazni format u kojem će biti spremljena slika
pub enum Format {
    // svi podržani formati biti će konvertirani u jpg
    Jpeg,
    // svi podržani formati biti će konvertirani u png
    Png,
    // pokušat će sačuvati originalni format, znači png
    // će biti spremljen kao png,
    // svi ostali izmjenjeni formati bit će spremljni kao jpg
    // JpegButKeepPng,
    // Png je ciljan format, ali će jpeg biti spremljen kao jpeg,
    // ostali izmijenjeni formati kao png
    // PngButKeepJpeg,
}

pub enum ResizeType {
    /// samo će povećati sliku
    Increase,
    /// samo će smanjiti sliku
    Decrease,
    /// neće mjenjati velićinu slike
    Neither,
    /// poovećat će ili smanjiti sliku na zadanu velićinu
    Eather,
}

// Setings that are applayed for all images
pub struct Job {
    pub format: Format,
    pub width: u32,
    pub height: u32,
    pub filter: FilterType,
    pub resize: ResizeType,
    pub suffix: String,
    pub rename_all: bool,
}

pub struct JobBuilder {
    format: Format,
    width: u32,
    height: u32,
    filter: FilterType,
    resize: ResizeType,
    suffix: String,
    rename_all: bool,
}

impl JobBuilder {
    /// kreira JobBuilder sa defaultnim vrijdnostima
    pub fn new() -> Self {
        JobBuilder {
            format: FORMAT,
            width: 0,
            height: 0,
            filter: FILTER,
            resize: RESIZE,
            suffix: SUFFIX.to_string(),
            rename_all: RENAME_ALL,
        }
    }

    pub fn format(&mut self, format: Format) -> &mut Self {
        self.format = format;   
        self
    }
    
    pub fn width(&mut self, width: u32) -> &mut Self {
        self.width = width;   
        self
    }

    pub fn height(&mut self, height: u32) -> &mut Self {
        self.height = height;   
        self
    }

    pub fn filter(&mut self, filter: FilterType) -> &mut Self {
        self.filter = filter;   
        self
    }

    pub fn resize(&mut self, resize: ResizeType) -> &mut Self {
        self.resize = resize;   
        self
    }

    pub fn suffix(&mut self, suffix: String) -> &mut Self {
        self.suffix = suffix;
        self
    }

    pub fn execute(self) -> Job {
        Job {
            format: self.format,
            width: self.width,
            height: self.height,
            filter: self.filter,
            resize: self.resize,
            suffix: self.suffix,   
            rename_all: self.rename_all,         
        }
    }      
}

impl Job {
    /// Kreira new job sa defaultnim vrijednostima
    pub fn new() -> Self {
        JobBuilder::new().execute()
    }
}

