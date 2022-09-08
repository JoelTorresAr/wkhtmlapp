use crate::{PdfApp, ImgApp};

#[derive(Debug, Clone)]
pub enum WkhtmlInput<'a> {
    File(&'a str),
    Url(&'a str),
    Html(&'a str),
}

#[derive(Debug, Clone)]
pub enum WkhtmlError {
    ServiceErr(String),
    RenderingErr(String),
}

// wkhtmlerror display
impl std::fmt::Display for WkhtmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WkhtmlError::ServiceErr(msg) => write!(f, "Service error: {}", msg),
            WkhtmlError::RenderingErr(msg) => write!(f, "Rendering error: {}", msg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct App<'a> {
    pub wkhtmltopdf_cmd: PdfApp<'a>,
    pub wkhtmltoimg_cmd: ImgApp<'a>,
}

impl <'a>App<'a> {
    pub fn new() -> Result<Self, WkhtmlError> {
        Ok(Self {
            wkhtmltopdf_cmd: PdfApp::new()?,
            wkhtmltoimg_cmd: ImgApp::new()?,
        })
    }
}
