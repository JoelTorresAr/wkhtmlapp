use crate::app::WkhtmlError;
use crate::app::{App, WkhtmlInput};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone)]
pub struct PdfApp<'a> {
    pub app: App,
    pub options: HashMap<&'a str, &'a str>,
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
        for (key, v) in &self.options {
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
        args
    }

    pub fn set_work_dir(&mut self, work_dir: &str) -> Result<&mut Self, WkhtmlError> {
        self.app.set_work_dir(work_dir)?;
        Ok(self)
    }

    pub fn set_args(&mut self, args: HashMap<&'a str, &'a str>) -> Result<&mut Self, WkhtmlError> {
        for (key, value) in args {
            self.set_arg(key, value)?;
        }
        Ok(self)
    }

    pub fn set_arg(&mut self, key: &'a str, arg: &'a str) -> Result<&mut Self, WkhtmlError> {
        if Self::validate_option(key) {
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

    fn default_options() -> HashMap<&'static str, &'a str> {
        HashMap::from([])
    }

    fn validate_option(key: &str) -> bool {
        let options: Vec<&'static str> = vec![
            // Global options
            "collate",
            "no-collate",
            "cookie-jar",
            "copies",
            "dpi",
            "extended-help",
            "grayscale",
            "help",
            "htmldoc",
            "ignore-load-errors", // old v0.9
            "image-dpi",
            "image-quality",
            "license",
            "log-level",
            "lowquality",
            "manpage",
            "margin-bottom",
            "margin-left",
            "margin-right",
            "margin-top",
            "orientation",
            "page-height",
            "page-size",
            "page-width",
            "no-pdf-compression",
            "quiet",
            "read-args-from-stdin",
            "readme",
            "title",
            "use-xserver",
            "version",
            // Outline options
            "dump-default-toc-xsl",
            "dump-outline",
            "outline",
            "no-outline",
            "outline-depth",
            "output-format",
            // Page options
            "allow",
            "background",
            "no-background",
            "bypass-proxy-for",
            "cache-dir",
            "checkbox-checked-svg",
            "checkbox-svg",
            "cookie",
            "custom-header",
            "custom-header-propagation",
            "no-custom-header-propagation",
            "debug-javascript",
            "no-debug-javascript",
            "default-header",
            "encoding",
            "disable-external-links",
            "enable-external-links",
            "disable-forms",
            "enable-forms",
            "images",
            "no-images",
            "disable-internal-links",
            "enable-internal-links",
            "disable-javascript",
            "enable-javascript",
            "javascript-delay",
            "keep-relative-links",
            "load-error-handling",
            "load-media-error-handling",
            "disable-local-file-access",
            "enable-local-file-access",
            "minimum-font-size",
            "exclude-from-outline",
            "include-in-outline",
            "page-offset",
            "password",
            "disable-plugins",
            "enable-plugins",
            "post",
            "post-file",
            "print-media-type",
            "no-print-media-type",
            "proxy",
            "proxy-hostname-lookup",
            "radiobutton-checked-svg",
            "radiobutton-svg",
            "redirect-delay", // old v0.9
            "resolve-relative-links",
            "run-script",
            "disable-smart-shrinking",
            "enable-smart-shrinking",
            "ssl-crt-path",
            "ssl-key-password",
            "ssl-key-path",
            "stop-slow-scripts",
            "no-stop-slow-scripts",
            "disable-toc-back-links",
            "enable-toc-back-links",
            "user-style-sheet",
            "username",
            "viewport-size",
            "window-status",
            "zoom",
            // Headers and footer options
            "footer-center",
            "footer-font-name",
            "footer-font-size",
            "footer-html",
            "footer-left",
            "footer-line",
            "no-footer-line",
            "footer-right",
            "footer-spacing",
            "header-center",
            "header-font-name",
            "header-font-size",
            "header-html",
            "header-left",
            "header-line",
            "no-header-line",
            "header-right",
            "header-spacing",
            "replace",
            // Cover object
            "cover",
            // TOC object
            "toc",
            // TOC options
            "disable-dotted-lines",
            "toc-depth",        // old v0.9
            "toc-font-name",    // old v0.9
            "toc-l1-font-size", // old v0.9
            "toc-header-text",
            "toc-header-font-name", // old v0.9
            "toc-header-font-size", // old v0.9
            "toc-level-indentation",
            "disable-toc-links",
            "toc-text-size-shrink",
            "xsl-style-sheet",
        ];
        options.contains(&key)
    }
}
