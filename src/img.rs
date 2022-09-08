use crate::app::WkhtmlError;
use crate::app::WkhtmlInput;
use crate::core::Core;
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
    pub app: Core,
    pub options: HashMap<&'a str, &'a str>,
    pub format: ImgFormat,
}

impl<'a> ImgApp<'a> {
    pub fn new() -> Result<Self, WkhtmlError> {
        let wkhtmltoimg_cmd =
            env::var("WKHTMLTOIMG_CMD").unwrap_or_else(|_| "wkhtmltoimage".to_string());

        Ok(Self {
            app: Core::new(wkhtmltoimg_cmd)?,
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
        self.set_arg("format", format)?;
        Ok(self)
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
            self.set_arg(&key, value)?;
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

    fn default_options() -> HashMap<&'a str, &'a str> {
        HashMap::from([])
    }

    fn validate_option(key: &str) -> bool {
        let options: Vec<&'static str> = vec![
            "allow", // Allow the file or files from the specified folder to be loaded (repeatable)
            "bypass-proxy-for", // Bypass proxy for host (repeatable)
            "cache-dir", // Web cache directory
            "checkbox-checked-svg", // Use this SVG file when rendering checked checkboxes
            "checked-svg", // Use this SVG file when rendering unchecked checkboxes
            "cookie", // Set an additional cookie (repeatable)
            "cookie-jar", // Read and write cookies from and to the supplied cookie jar file
            "crop-h", // Set height for cropping
            "crop-w", // Set width for cropping
            "crop-x", // Set x coordinate for cropping (default 0)
            "crop-y", // Set y coordinate for cropping (default 0)
            "custom-header", // Set an additional HTTP header (repeatable)
            "custom-header-propagation", // Add HTTP headers specified by --custom-header for each resource request.
            "no-custom-header-propagation", // Do not add HTTP headers specified by --custom-header for each resource request.
            "debug-javascript",             // Show javascript debugging output
            "no-debug-javascript",          // Do not show javascript debugging output (default)
            "encoding",                     // Set the default text encoding, for input
            "format",                       // Output format
            "height", // Set screen height (default is calculated from page content) (default 0)
            "images", // Do load or print images (default)
            "no-images", // Do not load or print images
            "disable-javascript", // Do not allow web pages to run javascript
            "enable-javascript", // Do allow web pages to run javascript (default)
            "javascript-delay", // Wait some milliseconds for javascript finish (default 200)
            "load-error-handling", // Specify how to handle pages that fail to load: abort, ignore or skip (default abort)
            "load-media-error-handling", // Specify how to handle media files that fail to load: abort, ignore or skip (default ignore)
            "disable-local-file-access", // Do not allowed conversion of a local file to read in other local files, unless explicitly allowed with allow
            "enable-local-file-access", // Allowed conversion of a local file to read in other local files. (default)
            "minimum-font-size",        // Minimum font size
            "password",                 // HTTP Authentication password
            "disable-plugins",          // Disable installed plugins (default)
            "enable-plugins",           // Enable installed plugins (plugins will likely not work)
            "post",                     // Add an additional post field
            "post-file",                // Post an additional file
            "proxy",                    // Use a proxy
            "quality",                  // Output image quality (between 0 and 100) (default 94)
            "quiet",                    // Be less verbose
            "radiobutton-checked-svg",  // Use this SVG file when rendering checked radio-buttons
            "radiobutton-svg",          // Use this SVG file when rendering unchecked radio-buttons
            "run-script", // Run this additional javascript after the page is done loading (repeatable)
            "disable-smart-width", // Use the specified width even if it is not large enough for the content
            "enable-smart-width",  // Extend --width to fit unbreakable content (default)
            "stop-slow-scripts",   // Stop slow running javascript
            "no-stop-slow-scripts", // Do not stop slow running javascript (default)
            "transparent",         // Make the background transparent in pngs *
            "use-xserver", // Use the X server (some plugins and other stuff might not work without X11)
            "user-style-sheet", // Specify a user style sheet, to load with every page
            "username",    // HTTP Authentication username
            "width",       // Set screen width (default is 1024)
            "window-status", // Wait until window.status is equal to this string before rendering page
            "zoom",          // Use this zoom factor (default 1)
        ];
        options.contains(&key)
    }
}
