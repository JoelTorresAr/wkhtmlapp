mod core;
mod app;
mod pdf;
mod img;
pub use app::*;
pub use img::*;
pub use pdf::*;
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{ImgApp, ImgFormat, PdfApp, WkhtmlInput};

    #[test]
    fn test_pdf() {
        let _ = env_logger::init();
        let mut pdf_app = PdfApp::new().expect("Failed to init PDF Application");
        let args = HashMap::from([("enable-smart-shrinking", "true")]);
        pdf_app.set_args(args).unwrap();

        // Test building PDF from HTML
        let html_code = r#"<html><body><div>DEMO</div></body></html>"#;
        let res = pdf_app.run(WkhtmlInput::Html(html_code), "demo");
        assert!(res.is_ok(), "{}", res.unwrap_err());

        // Test building PDF from file
        let res = pdf_app.run(WkhtmlInput::File("examples/index.html"), "demo");
        assert!(res.is_ok(), "{}", res.unwrap_err());

        // Test building PDF from URL
        let res = pdf_app.run(WkhtmlInput::Url("https://wkhtmltopdf.org/"), "demo");
        assert!(res.is_ok(), "{}", res.unwrap_err());
    }

    #[test]
    fn test_img() {
        // Test building image from FILE
        let mut image_app = ImgApp::new().expect("Failed to init image Application");
        let args = HashMap::from([("height", "100"), ("width", "100")]);

        // Test building image from file
        let res = image_app
            .set_format(ImgFormat::Png)
            .unwrap()
            .set_args(args)
            .unwrap()
            .run(WkhtmlInput::File("examples/index.html"), "demo");
        assert!(res.is_ok(), "{}", res.unwrap_err());

        // Test building image from URL
        let res = image_app.run(WkhtmlInput::Url("https://wkhtmltopdf.org/"), "demo");
        assert!(res.is_ok(), "{}", res.unwrap_err());
    }
}
