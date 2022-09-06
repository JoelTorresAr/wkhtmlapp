use self::uuid::Uuid;
use log::{debug, error, info};
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::{env, fs, io::Write};
use uuid;

const USE_STDIN_MARKER: &str = "-";
const NO_WKHTMLTOPDF_ERR: &str = "wkhtmltopdf tool is not found. Please install it.";

#[derive(Debug, Clone)]
pub enum WkhtmlInput {
    File(String),
    Url(String),
    Html(String),
}

#[derive(Debug, Clone)]
pub enum WkhtmlError {
    ServiceErr(String),
    RenderingErr(String),
}

// wkhtmlerror display
impl std::fmt::Display for WkhtmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WkhtmlError::ServiceErr(msg) => write!(f, "Service error: {}", msg),
            WkhtmlError::RenderingErr(msg) => write!(f, "Rendering error: {}", msg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct App {
    pub wkhtmltox_cmd: String,
    pub work_dir: PathBuf,
}

impl App {
    pub fn new(wkhtmltox_cmd: String) -> Result<Self, WkhtmlError> {
        dotenv::dotenv().ok();
        Self::bin_checks(&wkhtmltox_cmd).map_err(WkhtmlError::ServiceErr)?;
        let work_dir = env::var("WKHTMLTOPDF_WORK_DIR");
        let work_dir = work_dir.unwrap_or_else(|_| Self::default_work_dir());
        fs::create_dir_all(&work_dir).map_err(|e| {
            WkhtmlError::ServiceErr(format!("Failed to create working directory, due to: {}", e))
        })?;
        let work_dir = Self::parse_dir(&work_dir)?;

        Ok(Self {
            wkhtmltox_cmd,
            work_dir,
        })
    }

    pub fn default_work_dir() -> String {
        let root = env::temp_dir();
        root.join("wkhtmlapp").to_str().unwrap().to_string()
    }

    pub fn parse_dir(dir: &str) -> Result<PathBuf, WkhtmlError> {
        let path = PathBuf::from(dir);
        if path.is_dir() {
            Ok(path)
        } else {
            Err(WkhtmlError::ServiceErr(format!(
                "Directory {} is not found",
                dir
            )))
        }
    }

    pub fn bin_checks(wkhtmltox_cmd: &str) -> Result<(), String> {
        info!("Bootstrap check for {} tool", wkhtmltox_cmd);
        let status = Command::new(wkhtmltox_cmd)
            .arg("-V")
            .spawn()
            .map_err(|e| format!("Failed to spawn child process: {}", e))
            .and_then(|mut p| {
                p.wait().map_err(|e| {
                    format!("Failed to wait for {} tool , error: {}", wkhtmltox_cmd, e)
                })
            });

        status
            .and_then(|s| {
                if s.success() {
                    Ok(())
                } else {
                    Err(NO_WKHTMLTOPDF_ERR.to_string())
                }
            })
            .map_err(|e| {
                error!("{:?}", e);
                NO_WKHTMLTOPDF_ERR.to_string()
            })
    }

    pub fn get_debug() -> bool {
        env::var("APP_DEBUG").unwrap_or_else(|_| "true".to_string()) == "true"
    }

    pub fn depure(output: &Output) {
        debug!("status: {}", output.status);
        debug!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        debug!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    pub fn set_work_dir(&mut self, work_dir: &str) -> Result<&mut Self, WkhtmlError> {
        fs::create_dir_all(work_dir).map_err(|e| {
            WkhtmlError::ServiceErr(format!("Failed to create working directory, due to: {}", e))
        })?;
        self.work_dir = Self::parse_dir(&work_dir)?;
        Ok(self)
    }

    pub fn get_out_path(&self, name: &str) -> String {
        let temp_name = format!("{}-{}", Uuid::new_v4(), name);
        self.work_dir.join(temp_name).to_str().unwrap().to_string()
    }

    pub fn run(
        &self,
        input: WkhtmlInput,
        name: &str,
        args: Vec<String>,
    ) -> Result<String, WkhtmlError> {
        match input {
            WkhtmlInput::File(path) => self.run_with_file(&path, name, args),
            WkhtmlInput::Url(url) => self.run_with_url(&url, name, args),
            WkhtmlInput::Html(html) => self.run_with_html(&html, name, args),
        }
    }

    pub fn run_with_url(
        &self,
        url: &str,
        name: &str,
        args: Vec<String>,
    ) -> Result<String, WkhtmlError> {
        let out_path = self.get_out_path(name);
        let mut cmd = Command::new(&self.wkhtmltox_cmd);
        cmd.args(args)
            .arg(url)
            .arg(&out_path)
            .stdout(Stdio::piped());
        //.stderr(Stdio::piped());

        if !Self::get_debug() {
            cmd.stderr(Stdio::piped());
        }

        let child = cmd.spawn().map_err(|e| {
            WkhtmlError::RenderingErr(format!("Failed to spawn child process: {}", e))
        })?;

        debug!("Running command: {:?}", cmd);

        let output = child.wait_with_output().map_err(|e| {
            WkhtmlError::RenderingErr(format!("Failed to spawn child process: {}", e))
        })?;

        Self::depure(&output);

        if output.status.success() {
            Ok(out_path)
        } else {
            Err(WkhtmlError::RenderingErr(format!(
                "Failed to render, error: {}",
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    }

    pub fn run_with_file(
        &self,
        file_path: &str,
        name: &str,
        args: Vec<String>,
    ) -> Result<String, WkhtmlError> {
        let out_path = self.get_out_path(name);
        let mut cmd = Command::new(&self.wkhtmltox_cmd);
        cmd.args(args)
            .arg(file_path)
            .arg(&out_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if !Self::get_debug() {
            cmd.stderr(Stdio::piped());
        }

        let child = cmd.spawn().map_err(|e| {
            WkhtmlError::RenderingErr(format!("Failed to spawn child process: {}", e))
        })?;

        debug!("Running command: {:?}", cmd);

        let output = child.wait_with_output().map_err(|e| {
            WkhtmlError::RenderingErr(format!("Failed to spawn child process: {}", e))
        })?;

        Self::depure(&output);

        if output.status.success() {
            Ok(out_path)
        } else {
            Err(WkhtmlError::RenderingErr(format!(
                "Failed to render, error: {}",
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    }

    pub fn run_with_html(
        &self,
        html: &str,
        name: &str,
        args: Vec<String>,
    ) -> Result<String, WkhtmlError> {
        let out_path = self.get_out_path(name);
        let mut cmd = Command::new(&self.wkhtmltox_cmd);
        cmd.args(args)
            .arg(USE_STDIN_MARKER)
            .arg(&out_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped());

        if !Self::get_debug() {
            cmd.stderr(Stdio::piped());
        }

        debug!("Running command: {:?}", cmd);

        let mut child = cmd.spawn().map_err(|e| {
            WkhtmlError::RenderingErr(format!("Failed to spawn child process: {}", e))
        })?;

        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| WkhtmlError::RenderingErr("Failed to open stdin".to_string()))?;

        stdin
            .write_all(html.as_bytes())
            .map_err(|e| WkhtmlError::RenderingErr(format!("Failed to write to stdin: {}", e)))?;

        let output = child.wait_with_output().map_err(|e| {
            WkhtmlError::RenderingErr(format!("Failed to wait for child process: {}", e))
        })?;

        Self::depure(&output);

        if output.status.success() {
            Ok(out_path)
        } else {
            Err(WkhtmlError::RenderingErr(format!(
                "Failed to render, error: {}",
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    }
}
