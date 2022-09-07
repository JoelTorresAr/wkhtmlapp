use crate::app::WkhtmlError;
use crate::app::{App, WkhtmlInput};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone)]
pub struct PdfApp<'a> {
    pub app: App,
    pub options: HashMap<&'a str, Option<&'a str>>,
}

impl<'a> PdfApp<'a> {
    pub fn new() -> Result<Self, WkhtmlError> {
        let wkhtmltopdf_cmd =
            env::var("WKHTMLTOPDF_CMD").unwrap_or_else(|_| "wkhtmltopdf".to_string());

        Ok(Self {
            app: App::new(wkhtmltopdf_cmd)?,
            options: Self::default_options(),
        })
    }

    fn build_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        for (key, value) in &self.options {
            match value {
                Some(v) => {
                    if *v != "false" {
                        if *v == "true" {
                            if *key == "toc" || *key == "cover" {
                                args.push(key.to_string());
                            } else {
                                args.push(format!("--{}", key));
                            }
                        } else {
                            if *key == "toc" || *key == "cover" {
                                args.push(key.to_string());
                            } else {
                                args.push(format!("--{}", key));
                                args.push(format!("{}", v));
                                //args.push(format!("--{} \"{}\"", key, v));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        args
    }

    pub fn set_work_dir(&mut self, work_dir: &str) -> Result<&mut Self, WkhtmlError> {
        self.app.set_work_dir(work_dir)?;
        Ok(self)
    }

    pub fn set_args(
        &mut self,
        args: HashMap<&'a str, Option<&'a str>>,
    ) -> Result<&mut Self, WkhtmlError> {
        for (key, value) in args {
            self.set_arg(key, value)?;
        }
        Ok(self)
    }

    pub fn set_arg(&mut self, key: &'a str, arg: Option<&'a str>) -> Result<&mut Self, WkhtmlError> {
        if self.options.contains_key(key) {
            self.options.insert(key, arg);
            Ok(self)
        } else {
            Err(WkhtmlError::ServiceErr(format!("Invalid option: {}", key)))
        }
    }

    pub fn run(&self, input: WkhtmlInput, name: &str) -> Result<String, WkhtmlError> {
        let name = &format!("{}.pdf", name);
        match input {
            WkhtmlInput::File(path) => self.app.run_with_file(&path, name, self.build_args()),
            WkhtmlInput::Url(url) => self.app.run_with_url(&url, name, self.build_args()),
            WkhtmlInput::Html(html) => self.app.run_with_html(&html, name, self.build_args()),
        }
    }

    fn default_options() -> HashMap<&'static str, Option<&'a str>> {
        HashMap::from([
            // Global options
            ("collate", None),
            ("no-collate", None),
            ("cookie-jar", None),
            ("copies", None),
            ("dpi", None),
            ("extended-help", None),
            ("grayscale", None),
            ("help", None),
            ("htmldoc", None),
            ("ignore-load-errors", None), // old v0.9
            ("image-dpi", None),
            ("image-quality", None),
            ("license", None),
            ("log-level", None),
            ("lowquality", None),
            ("manpage", None),
            ("margin-bottom", None),
            ("margin-left", None),
            ("margin-right", None),
            ("margin-top", None),
            ("orientation", None),
            ("page-height", None),
            ("page-size", None),
            ("page-width", None),
            ("no-pdf-compression", None),
            ("quiet", None),
            ("read-args-from-stdin", None),
            ("readme", None),
            ("title", None),
            ("use-xserver", None),
            ("version", None),
            // Outline options
            ("dump-default-toc-xsl", None),
            ("dump-outline", None),
            ("outline", None),
            ("no-outline", None),
            ("outline-depth", None),
            ("output-format", None),
            // Page options
            ("allow", None),
            ("background", None),
            ("no-background", None),
            ("bypass-proxy-for", None),
            ("cache-dir", None),
            ("checkbox-checked-svg", None),
            ("checkbox-svg", None),
            ("cookie", None),
            ("custom-header", None),
            ("custom-header-propagation", None),
            ("no-custom-header-propagation", None),
            ("debug-javascript", None),
            ("no-debug-javascript", None),
            ("default-header", None),
            ("encoding", None),
            ("disable-external-links", None),
            ("enable-external-links", None),
            ("disable-forms", None),
            ("enable-forms", None),
            ("images", None),
            ("no-images", None),
            ("disable-internal-links", None),
            ("enable-internal-links", None),
            ("disable-javascript", None),
            ("enable-javascript", None),
            ("javascript-delay", None),
            ("keep-relative-links", None),
            ("load-error-handling", None),
            ("load-media-error-handling", None),
            ("disable-local-file-access", None),
            ("enable-local-file-access", None),
            ("minimum-font-size", None),
            ("exclude-from-outline", None),
            ("include-in-outline", None),
            ("page-offset", None),
            ("password", None),
            ("disable-plugins", None),
            ("enable-plugins", None),
            ("post", None),
            ("post-file", None),
            ("print-media-type", None),
            ("no-print-media-type", None),
            ("proxy", None),
            ("proxy-hostname-lookup", None),
            ("radiobutton-checked-svg", None),
            ("radiobutton-svg", None),
            ("redirect-delay", None), // old v0.9
            ("resolve-relative-links", None),
            ("run-script", None),
            ("disable-smart-shrinking", None),
            ("enable-smart-shrinking", None),
            ("ssl-crt-path", None),
            ("ssl-key-password", None),
            ("ssl-key-path", None),
            ("stop-slow-scripts", None),
            ("no-stop-slow-scripts", None),
            ("disable-toc-back-links", None),
            ("enable-toc-back-links", None),
            ("user-style-sheet", None),
            ("username", None),
            ("viewport-size", None),
            ("window-status", None),
            ("zoom", None),
            // Headers and footer options
            ("footer-center", None),
            ("footer-font-name", None),
            ("footer-font-size", None),
            ("footer-html", None),
            ("footer-left", None),
            ("footer-line", None),
            ("no-footer-line", None),
            ("footer-right", None),
            ("footer-spacing", None),
            ("header-center", None),
            ("header-font-name", None),
            ("header-font-size", None),
            ("header-html", None),
            ("header-left", None),
            ("header-line", None),
            ("no-header-line", None),
            ("header-right", None),
            ("header-spacing", None),
            ("replace", None),
            // Cover object
            ("cover", None),
            // TOC object
            ("toc", None),
            // TOC options
            ("disable-dotted-lines", None),
            ("toc-depth", None),        // old v0.9
            ("toc-font-name", None),    // old v0.9
            ("toc-l1-font-size", None), // old v0.9
            ("toc-header-text", None),
            ("toc-header-font-name", None), // old v0.9
            ("toc-header-font-size", None), // old v0.9
            ("toc-level-indentation", None),
            ("disable-toc-links", None),
            ("toc-text-size-shrink", None),
            ("xsl-style-sheet", None),
        ])
    }
}
