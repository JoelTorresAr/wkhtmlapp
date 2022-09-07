pub mod app;
pub mod img;
pub mod pdf;
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
        let args = HashMap::from([("enable-smart-shrinking", Some("true"))]);
        pdf_app.set_args(args).unwrap();

        // Test building PDF from HTML
        let html_code = r#"<html><body><div>DEMO</div></body></html>"#;
        let res = pdf_app.run(WkhtmlInput::Html(html_code), "demo");
        assert!(res.is_ok(), "{}", res.unwrap_err());

        // Test building PDF from file
        let res = pdf_app.run(WkhtmlInput::File("examples/index.html"), "demo");
        assert!(res.is_ok(), "{}", res.unwrap_err());

        // Test building PDF from URL
        let res = pdf_app.run(WkhtmlInput::Url("https://www.rust-lang.org/en-US/"), "demo");
        assert!(res.is_ok(), "{}", res.unwrap_err());
    }

    #[test]
    fn test_img() {
        // Test building image from FILE
        let mut image_app = ImgApp::new().expect("Failed to init image Application");
        let args = HashMap::from([("height", Some("100")), ("width", Some("100"))]);

        // Test building image from file
        let res = image_app
            .set_format(ImgFormat::Png)
            .unwrap()
            .set_args(args)
            .unwrap()
            .run(WkhtmlInput::File("examples/index.html"), "demo");
        assert!(res.is_ok(), "{}", res.unwrap_err());

        // Test building image from URL
        let res = image_app.run(WkhtmlInput::Url("https://www.rust-lang.org/en-US/"), "demo");
        assert!(res.is_ok(), "{}", res.unwrap_err());
    }
}
