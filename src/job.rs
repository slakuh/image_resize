use image::FilterType;
// osnovne vrijednosti za Job struct
const FORMAT: Format = Format::Jpeg;
const FILTER: FilterType = FilterType::Nearest;
const RESIZE: ResizeType = ResizeType::Decrease;

/// Izlazni format u kojem će biti spremljena slika
pub enum Format {
    Jpeg,
    Png,
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


pub struct Job {
    pub format: Format,
    pub width: u32,
    pub height: u32,
    pub filter: FilterType,
    pub resize: ResizeType    
}

pub struct JobBuilder {
    format: Format,
    width: u32,
    height: u32,
    filter: FilterType,
    resize: ResizeType
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

    pub fn execute(self) -> Job {
        Job {
            format: self.format,
            width: self.width,
            height: self.height,
            filter: self.filter,
            resize: self.resize,
        }
    }      
}

impl Job {
    /// Kreira new job sa defaultnim vrijednostima
    pub fn new() -> Self {
        JobBuilder::new().execute()            
    }

    pub fn change_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn change_height(&mut self, height: u32) {
        self.height = height;
    }    

    pub fn change_format(&mut self, format: Format) {
        self.format = format;
    }    


    pub fn change_filter(&mut self, filter: FilterType) {
        self.filter = filter;
    }    

    pub fn change_resize(&mut self, resize: ResizeType) {
        self.resize = resize;
    }    


    
}

