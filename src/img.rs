use crate::app::WkhtmlError;
use crate::app::{App, WkhtmlInput};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone)]
pub enum ImgFormat {
    Jpg,
    Png,
    Bmp,
    Svg,
}

//display imgformat
impl std::fmt::Display for ImgFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ImgFormat::Jpg => write!(f, "jpg"),
            ImgFormat::Png => write!(f, "png"),
            ImgFormat::Bmp => write!(f, "bmp"),
            ImgFormat::Svg => write!(f, "svg"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImgApp<'a> {
    pub app: App,
    pub options: HashMap<&'a str, Option<&'a str>>,
    pub format: ImgFormat,
}

impl<'a> ImgApp<'a> {
    pub fn new() -> Result<Self, WkhtmlError> {
        let wkhtmltoimg_cmd =
            env::var("WKHTMLTOIMG_CMD").unwrap_or_else(|_| "wkhtmltoimage".to_string());

        Ok(Self {
            app: App::new(wkhtmltoimg_cmd)?,
            options: Self::default_options(),
            format: ImgFormat::Jpg,
        })
    }
    pub fn set_format(&mut self, format: ImgFormat) -> Result<&mut Self, WkhtmlError> {
        let format = match format {
            ImgFormat::Jpg => "jpg",
            ImgFormat::Png => "png",
            ImgFormat::Bmp => "bmp",
            ImgFormat::Svg => "svg",
        };
        self.set_arg("format", Some(format))?;
        Ok(self)
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
            self.set_arg(&key, value)?;
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
        let name = &format!("{}.{}", name, self.format.clone());
        match input {
            WkhtmlInput::File(path) => self.app.run_with_file(&path, name, self.build_args()),
            WkhtmlInput::Url(url) => self.app.run_with_url(&url, name, self.build_args()),
            WkhtmlInput::Html(html) => self.app.run_with_html(&html, name, self.build_args()),
        }
    }

    pub fn default_extension() -> &'a str {
        "jpg"
    }

    fn default_options() -> HashMap<&'a str, Option<&'a str>> {
        HashMap::from([
            ("allow", None), // Allow the file or files from the specified folder to be loaded (repeatable)
            ("bypass-proxy-for", None), // Bypass proxy for host (repeatable)
            ("cache-dir", None), // Web cache directory
            ("checkbox-checked-svg", None), // Use this SVG file when rendering checked checkboxes
            ("checked-svg", None), // Use this SVG file when rendering unchecked checkboxes
            ("cookie", None), // Set an additional cookie (repeatable)
            ("cookie-jar", None), // Read and write cookies from and to the supplied cookie jar file
            ("crop-h", None), // Set height for cropping
            ("crop-w", None), // Set width for cropping
            ("crop-x", None), // Set x coordinate for cropping (default 0)
            ("crop-y", None), // Set y coordinate for cropping (default 0)
            ("custom-header", None), // Set an additional HTTP header (repeatable)
            ("custom-header-propagation", None), // Add HTTP headers specified by --custom-header for each resource request.
            ("no-custom-header-propagation", None), // Do not add HTTP headers specified by --custom-header for each resource request.
            ("debug-javascript", None),             // Show javascript debugging output
            ("no-debug-javascript", None), // Do not show javascript debugging output (default)
            ("encoding", None),            // Set the default text encoding, for input
            ("format", Some(ImgApp::default_extension())), // Output format
            ("height", None), // Set screen height (default is calculated from page content) (default 0)
            ("images", None), // Do load or print images (default)
            ("no-images", None), // Do not load or print images
            ("disable-javascript", None), // Do not allow web pages to run javascript
            ("enable-javascript", None), // Do allow web pages to run javascript (default)
            ("javascript-delay", None), // Wait some milliseconds for javascript finish (default 200)
            ("load-error-handling", None), // Specify how to handle pages that fail to load: abort, ignore or skip (default abort)
            ("load-media-error-handling", None), // Specify how to handle media files that fail to load: abort, ignore or skip (default ignore)
            ("disable-local-file-access", None), // Do not allowed conversion of a local file to read in other local files, unless explicitly allowed with allow
            ("enable-local-file-access", None), // Allowed conversion of a local file to read in other local files. (default)
            ("minimum-font-size", None),        // Minimum font size
            ("password", None),                 // HTTP Authentication password
            ("disable-plugins", None),          // Disable installed plugins (default)
            ("enable-plugins", None), // Enable installed plugins (plugins will likely not work)
            ("post", None),           // Add an additional post field
            ("post-file", None),      // Post an additional file
            ("proxy", None),          // Use a proxy
            ("quality", None),        // Output image quality (between 0 and 100) (default 94)
            ("quiet", None),          // Be less verbose
            ("radiobutton-checked-svg", None), // Use this SVG file when rendering checked radio-buttons
            ("radiobutton-svg", None), // Use this SVG file when rendering unchecked radio-buttons
            ("run-script", None), // Run this additional javascript after the page is done loading (repeatable)
            ("disable-smart-width", None), // Use the specified width even if it is not large enough for the content
            ("enable-smart-width", None),  // Extend --width to fit unbreakable content (default)
            ("stop-slow-scripts", None),   // Stop slow running javascript
            ("no-stop-slow-scripts", None), // Do not stop slow running javascript (default)
            ("transparent", None),         // Make the background transparent in pngs *
            ("use-xserver", None), // Use the X server (some plugins and other stuff might not work without X11)
            ("user-style-sheet", None), // Specify a user style sheet, to load with every page
            ("username", None),    // HTTP Authentication username
            ("width", None),       // Set screen width (default is 1024)
            ("window-status", None), // Wait until window.status is equal to this string before rendering page
            ("zoom", None),          // Use this zoom factor (default 1)
        ])
    }
}
