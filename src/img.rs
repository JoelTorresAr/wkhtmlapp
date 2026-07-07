use crate::app::WkhtmlError;
use crate::app::WkhtmlInput;
use crate::core::Core;
use std::collections::{HashMap, HashSet};
use std::env;
use std::path::PathBuf;

/// Output image format (defaults to `Jpg`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImgFormat {
    #[default]
    Jpg,
    Png,
    Bmp,
    Svg,
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

/// HTML-to-image converter backed by the `wkhtmltoimage` binary
/// (override the binary path with the `WKHTMLTOIMG_CMD` env var).
#[derive(Debug, Clone)]
pub struct ImgApp {
    pub app: Core,
    /// Single-value options; setting the same key again replaces it.
    pub options: HashMap<String, String>,
    /// Repeatable options added via [`ImgApp::add_arg`], kept in insertion order.
    pub extra_args: Vec<(String, String)>,
    pub format: ImgFormat,
}

impl ImgApp {
    pub fn new() -> Result<Self, WkhtmlError> {
        let wkhtmltoimg_cmd =
            env::var("WKHTMLTOIMG_CMD").unwrap_or_else(|_| "wkhtmltoimage".to_string());

        Ok(Self {
            app: Core::new(wkhtmltoimg_cmd)?,
            options: HashMap::new(),
            extra_args: Vec::new(),
            format: ImgFormat::default(),
        })
    }

    pub fn set_format(&mut self, format: ImgFormat) -> Result<&mut Self, WkhtmlError> {
        self.format = format;
        self.set_arg("format", &format.to_string())
    }

    pub fn set_work_dir(&mut self, work_dir: &str) -> Result<&mut Self, WkhtmlError> {
        self.app.set_work_dir(work_dir)?;
        Ok(self)
    }

    /// Removes leftover output files from the work dir (outputs are never
    /// deleted automatically).
    pub fn clear_work_dir(&self) -> Result<(), WkhtmlError> {
        self.app.clear_work_dir()
    }

    pub fn set_args(&mut self, args: HashMap<&str, &str>) -> Result<&mut Self, WkhtmlError> {
        for (key, value) in args {
            self.set_arg(key, value)?;
        }
        Ok(self)
    }

    /// Sets an option, replacing any previous value for the same key.
    /// Use `"true"`/`"false"` to enable/disable flags. Two-value options
    /// (`cookie`, `custom-header`, `post`, `post-file`) take both values
    /// separated by a space: `set_arg("cookie", "name value")`.
    pub fn set_arg(&mut self, key: &str, arg: &str) -> Result<&mut Self, WkhtmlError> {
        if Self::validate_option(key) {
            self.options.insert(key.into(), arg.into());
            Ok(self)
        } else {
            Err(WkhtmlError::ServiceErr(format!("Invalid option: {}", key)))
        }
    }

    /// Appends a repeatable option (`cookie`, `custom-header`, `allow`,
    /// `run-script`, ...) without replacing previously added values.
    pub fn add_arg(&mut self, key: &str, arg: &str) -> Result<&mut Self, WkhtmlError> {
        if Self::validate_option(key) {
            self.extra_args.push((key.into(), arg.into()));
            Ok(self)
        } else {
            Err(WkhtmlError::ServiceErr(format!("Invalid option: {}", key)))
        }
    }

    /// Renders the input to an image inside the work dir and returns its path.
    /// `name` must be a plain file name (no path separators or `..`).
    pub fn run(&self, input: WkhtmlInput, name: &str) -> Result<PathBuf, WkhtmlError> {
        let name = format!("{}.{}", name, self.format);
        let args = Core::build_args(&self.options, &self.extra_args);
        self.app.run(input, &name, args)
    }

    fn validate_option(key: &str) -> bool {
        static OPTIONS: std::sync::LazyLock<HashSet<&'static str>> =
            std::sync::LazyLock::new(|| {
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
