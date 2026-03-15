use crate::app::WkhtmlError;
use crate::app::WkhtmlInput;
use crate::core::Core;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::env;

#[derive(Debug, Clone)]
pub enum ImgFormat {
    Jpg,
    Png,
    Bmp,
    Svg,
}

impl Default for ImgFormat {
    fn default() -> Self {
        ImgFormat::Jpg
    }
}

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
pub struct ImgApp {
    pub app: Core,
    pub options: HashMap<String, String>,
    pub format: ImgFormat,
}

impl ImgApp {
    pub fn new() -> Result<Self, WkhtmlError> {
        let wkhtmltoimg_cmd =
            env::var("WKHTMLTOIMG_CMD").unwrap_or_else(|_| "wkhtmltoimage".to_string());

        Ok(Self {
            app: Core::new(wkhtmltoimg_cmd)?,
            options: HashMap::new(),
            format: ImgFormat::default(),
        })
    }

    pub fn set_format(&mut self, format: ImgFormat) -> Result<&mut Self, WkhtmlError> {
        let fmt_str = format.to_string();
        self.format = format;
        self.set_arg("format", &fmt_str)?;
        Ok(self)
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
        let name = format!("{}.{}", name, self.format);
        let args = Core::build_args(&self.options);
        self.app.run(input, &name, args)
    }

    fn validate_option(key: &str) -> bool {
        static OPTIONS: std::sync::LazyLock<HashSet<&'static str>> = std::sync::LazyLock::new(|| {
            HashSet::from([
                "allow",
                "bypass-proxy-for",
                "cache-dir",
                "checkbox-checked-svg",
                "checked-svg",
                "cookie",
                "cookie-jar",
                "crop-h",
                "crop-w",
                "crop-x",
                "crop-y",
                "custom-header",
                "custom-header-propagation",
                "no-custom-header-propagation",
                "debug-javascript",
                "no-debug-javascript",
                "encoding",
                "format",
                "height",
                "images",
                "no-images",
                "disable-javascript",
                "enable-javascript",
                "javascript-delay",
                "load-error-handling",
                "load-media-error-handling",
                "disable-local-file-access",
                "enable-local-file-access",
                "minimum-font-size",
                "password",
                "disable-plugins",
                "enable-plugins",
                "post",
                "post-file",
                "proxy",
                "quality",
                "quiet",
                "radiobutton-checked-svg",
                "radiobutton-svg",
                "run-script",
                "disable-smart-width",
                "enable-smart-width",
                "stop-slow-scripts",
                "no-stop-slow-scripts",
                "transparent",
                "use-xserver",
                "user-style-sheet",
                "username",
                "width",
                "window-status",
                "zoom",
            ])
        });
        OPTIONS.contains(key)
    }
}
