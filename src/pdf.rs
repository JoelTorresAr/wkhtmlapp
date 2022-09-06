use crate::app::WkhtmlError;
use crate::app::{App, WkhtmlInput};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone)]
pub struct PdfApp {
    pub app: App,
    pub options: HashMap<String, Option<String>>,
}

impl PdfApp {
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
                    if v != "false" {
                        if v == "true" {
                            if key == "toc" || key == "cover" {
                                args.push(key.to_string());
                            } else {
                                args.push(format!("--{}", key));
                            }
                        } else {
                            if key == "toc" || key == "cover" {
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
        args: HashMap<String, Option<String>>,
    ) -> Result<&mut Self, WkhtmlError> {
        for (key, value) in args {
            self.set_arg(&key, value)?;
        }
        Ok(self)
    }

    pub fn set_arg(&mut self, key: &str, arg: Option<String>) -> Result<&mut Self, WkhtmlError> {
        if self.options.contains_key(key) {
            self.options.insert(key.to_string(), arg);
            Ok(self)
        } else {
            Err(WkhtmlError::Service(format!("Invalid option: {}", key)))
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

    fn default_options() -> HashMap<String, Option<String>> {
        HashMap::from([
            // Global options
            ("collate".to_string(), None),
            ("no-collate".to_string(), None),
            ("cookie-jar".to_string(), None),
            ("copies".to_string(), None),
            ("dpi".to_string(), None),
            ("extended-help".to_string(), None),
            ("grayscale".to_string(), None),
            ("help".to_string(), None),
            ("htmldoc".to_string(), None),
            ("ignore-load-errors".to_string(), None), // old v0.9
            ("image-dpi".to_string(), None),
            ("image-quality".to_string(), None),
            ("license".to_string(), None),
            ("log-level".to_string(), None),
            ("lowquality".to_string(), None),
            ("manpage".to_string(), None),
            ("margin-bottom".to_string(), None),
            ("margin-left".to_string(), None),
            ("margin-right".to_string(), None),
            ("margin-top".to_string(), None),
            ("orientation".to_string(), None),
            ("page-height".to_string(), None),
            ("page-size".to_string(), None),
            ("page-width".to_string(), None),
            ("no-pdf-compression".to_string(), None),
            ("quiet".to_string(), None),
            ("read-args-from-stdin".to_string(), None),
            ("readme".to_string(), None),
            ("title".to_string(), None),
            ("use-xserver".to_string(), None),
            ("version".to_string(), None),
            // Outline options
            ("dump-default-toc-xsl".to_string(), None),
            ("dump-outline".to_string(), None),
            ("outline".to_string(), None),
            ("no-outline".to_string(), None),
            ("outline-depth".to_string(), None),
            ("output-format".to_string(), None),
            // Page options
            ("allow".to_string(), None),
            ("background".to_string(), None),
            ("no-background".to_string(), None),
            ("bypass-proxy-for".to_string(), None),
            ("cache-dir".to_string(), None),
            ("checkbox-checked-svg".to_string(), None),
            ("checkbox-svg".to_string(), None),
            ("cookie".to_string(), None),
            ("custom-header".to_string(), None),
            ("custom-header-propagation".to_string(), None),
            ("no-custom-header-propagation".to_string(), None),
            ("debug-javascript".to_string(), None),
            ("no-debug-javascript".to_string(), None),
            ("default-header".to_string(), None),
            ("encoding".to_string(), None),
            ("disable-external-links".to_string(), None),
            ("enable-external-links".to_string(), None),
            ("disable-forms".to_string(), None),
            ("enable-forms".to_string(), None),
            ("images".to_string(), None),
            ("no-images".to_string(), None),
            ("disable-internal-links".to_string(), None),
            ("enable-internal-links".to_string(), None),
            ("disable-javascript".to_string(), None),
            ("enable-javascript".to_string(), None),
            ("javascript-delay".to_string(), None),
            ("keep-relative-links".to_string(), None),
            ("load-error-handling".to_string(), None),
            ("load-media-error-handling".to_string(), None),
            ("disable-local-file-access".to_string(), None),
            ("enable-local-file-access".to_string(), None),
            ("minimum-font-size".to_string(), None),
            ("exclude-from-outline".to_string(), None),
            ("include-in-outline".to_string(), None),
            ("page-offset".to_string(), None),
            ("password".to_string(), None),
            ("disable-plugins".to_string(), None),
            ("enable-plugins".to_string(), None),
            ("post".to_string(), None),
            ("post-file".to_string(), None),
            ("print-media-type".to_string(), None),
            ("no-print-media-type".to_string(), None),
            ("proxy".to_string(), None),
            ("proxy-hostname-lookup".to_string(), None),
            ("radiobutton-checked-svg".to_string(), None),
            ("radiobutton-svg".to_string(), None),
            ("redirect-delay".to_string(), None), // old v0.9
            ("resolve-relative-links".to_string(), None),
            ("run-script".to_string(), None),
            ("disable-smart-shrinking".to_string(), None),
            ("enable-smart-shrinking".to_string(), None),
            ("ssl-crt-path".to_string(), None),
            ("ssl-key-password".to_string(), None),
            ("ssl-key-path".to_string(), None),
            ("stop-slow-scripts".to_string(), None),
            ("no-stop-slow-scripts".to_string(), None),
            ("disable-toc-back-links".to_string(), None),
            ("enable-toc-back-links".to_string(), None),
            ("user-style-sheet".to_string(), None),
            ("username".to_string(), None),
            ("viewport-size".to_string(), None),
            ("window-status".to_string(), None),
            ("zoom".to_string(), None),
            // Headers and footer options
            ("footer-center".to_string(), None),
            ("footer-font-name".to_string(), None),
            ("footer-font-size".to_string(), None),
            ("footer-html".to_string(), None),
            ("footer-left".to_string(), None),
            ("footer-line".to_string(), None),
            ("no-footer-line".to_string(), None),
            ("footer-right".to_string(), None),
            ("footer-spacing".to_string(), None),
            ("header-center".to_string(), None),
            ("header-font-name".to_string(), None),
            ("header-font-size".to_string(), None),
            ("header-html".to_string(), None),
            ("header-left".to_string(), None),
            ("header-line".to_string(), None),
            ("no-header-line".to_string(), None),
            ("header-right".to_string(), None),
            ("header-spacing".to_string(), None),
            ("replace".to_string(), None),
            // Cover object
            ("cover".to_string(), None),
            // TOC object
            ("toc".to_string(), None),
            // TOC options
            ("disable-dotted-lines".to_string(), None),
            ("toc-depth".to_string(), None),        // old v0.9
            ("toc-font-name".to_string(), None),    // old v0.9
            ("toc-l1-font-size".to_string(), None), // old v0.9
            ("toc-header-text".to_string(), None),
            ("toc-header-font-name".to_string(), None), // old v0.9
            ("toc-header-font-size".to_string(), None), // old v0.9
            ("toc-level-indentation".to_string(), None),
            ("disable-toc-links".to_string(), None),
            ("toc-text-size-shrink".to_string(), None),
            ("xsl-style-sheet".to_string(), None),
        ])
    }
}
