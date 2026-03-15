use crate::app::WkhtmlError;
use crate::app::WkhtmlInput;
use crate::core::Core;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::env;

#[derive(Debug, Clone)]
pub struct PdfApp {
    pub app: Core,
    pub options: HashMap<String, String>,
}

impl PdfApp {
    pub fn new() -> Result<Self, WkhtmlError> {
        let wkhtmltopdf_cmd =
            env::var("WKHTMLTOPDF_CMD").unwrap_or_else(|_| "wkhtmltopdf".to_string());

        Ok(Self {
            app: Core::new(wkhtmltopdf_cmd)?,
            options: HashMap::new(),
        })
    }

    pub fn set_work_dir(&mut self, work_dir: &str) -> Result<&mut Self, WkhtmlError> {
        self.app.set_work_dir(work_dir)?;
        Ok(self)
    }

    pub fn set_args(&mut self, args: HashMap<&str, &str>) -> Result<&mut Self, WkhtmlError> {
        for (key, value) in args {
            self.set_arg(key, value)?;
        }
        Ok(self)
    }

    pub fn set_arg(&mut self, key: &str, arg: &str) -> Result<&mut Self, WkhtmlError> {
        if Self::validate_option(key) {
            self.options.insert(key.into(), arg.into());
            Ok(self)
        } else {
            Err(WkhtmlError::ServiceErr(format!("Invalid option: {}", key)))
        }
    }

    pub fn run(&self, input: WkhtmlInput, name: &str) -> Result<PathBuf, WkhtmlError> {
        let name = format!("{}.pdf", name);
        let args = Core::build_args(&self.options);
        self.app.run(input, &name, args)
    }

    fn validate_option(key: &str) -> bool {
        static OPTIONS: std::sync::LazyLock<HashSet<&'static str>> = std::sync::LazyLock::new(|| {
            HashSet::from([
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
                "ignore-load-errors",
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
                "redirect-delay",
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
                "toc-depth",
                "toc-font-name",
                "toc-l1-font-size",
                "toc-header-text",
                "toc-header-font-name",
                "toc-header-font-size",
                "toc-level-indentation",
                "disable-toc-links",
                "toc-text-size-shrink",
                "xsl-style-sheet",
            ])
        });
        OPTIONS.contains(key)
    }
}
