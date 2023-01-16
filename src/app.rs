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
pub struct App {
    pub pdf_app: PdfApp,
    pub img_app: ImgApp,
}

impl App{
    pub fn new() -> Result<Self, WkhtmlError> {
        Ok(Self {
            pdf_app: PdfApp::new()?,
            img_app: ImgApp::new()?,
        })
    }
}
