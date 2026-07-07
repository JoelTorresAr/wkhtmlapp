//! Bindings to the `wkhtmltopdf` and `wkhtmltoimage` CLI tools.
//!
//! Convert HTML (raw code, local file, or URL) to PDF or image. Each render
//! spawns the external binary, writes the output to a work dir with a unique
//! (UUID-prefixed) name, and returns the resulting [`std::path::PathBuf`].
//!
//! ```no_run
//! use wkhtmlapp::{PdfApp, WkhtmlInput};
//!
//! # fn main() -> Result<(), wkhtmlapp::WkhtmlError> {
//! let mut pdf_app = PdfApp::new()?;
//! pdf_app.set_arg("page-size", "A4")?;
//! let report = pdf_app.run(WkhtmlInput::Html("<h1>Demo</h1>"), "demo")?;
//! println!("PDF at {:?}", report);
//! # Ok(())
//! # }
//! ```
//!
//! Environment variables: `WKHTMLTOPDF_CMD` / `WKHTMLTOIMG_CMD` (binary
//! paths) and `WKHTMLAPP_WORK_DIR` (output directory, defaults to the OS
//! temp dir). Output files are never deleted automatically — see
//! `clear_work_dir()`.
mod app;
mod core;
mod img;
mod pdf;
pub use crate::core::Core;
pub use app::*;
pub use img::*;
pub use pdf::*;

// Integration tests: require the wkhtmltox binaries installed (and network
// access for the URL cases), so they are #[ignore]d by default.
// Run them locally with: cargo test -- --include-ignored
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{ImgApp, ImgFormat, PdfApp, WkhtmlInput};

    #[test]
    #[ignore = "requires wkhtmltopdf installed and network access"]
    fn test_pdf() {
        let _ = env_logger::try_init();
        let mut pdf_app = PdfApp::new().expect("Failed to init PDF Application");
        let args = HashMap::from([("enable-smart-shrinking", "true")]);
        pdf_app.set_args(args).unwrap();

        // Test building PDF from HTML
        let html_code = r#"<html><body><div>DEMO</div></body></html>"#;
        let res = pdf_app.run(WkhtmlInput::Html(html_code), "demo");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        assert!(res.unwrap().extension().unwrap() == "pdf");

        // Test building PDF from file
        let res = pdf_app.run(WkhtmlInput::File("examples/index.html"), "demo");
        assert!(res.is_ok(), "{}", res.unwrap_err());

        // Test building PDF from URL
        let res = pdf_app.run(WkhtmlInput::Url("https://wkhtmltopdf.org/"), "demo");
        assert!(res.is_ok(), "{}", res.unwrap_err());
    }

    #[test]
    #[ignore = "requires wkhtmltoimage installed and network access"]
    fn test_img() {
        let _ = env_logger::try_init();
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

    #[test]
    #[ignore = "requires wkhtmltopdf installed"]
    fn test_rejects_path_traversal_names() {
        let pdf_app = PdfApp::new().expect("Failed to init PDF Application");
        let res = pdf_app.run(WkhtmlInput::Html("<p>x</p>"), "../escape");
        assert!(res.is_err());
    }
}
