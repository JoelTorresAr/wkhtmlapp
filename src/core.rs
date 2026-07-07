use crate::{WkhtmlError, WkhtmlInput};

use self::uuid::Uuid;
use log::{debug, error, info};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::{env, fs, io::Write};
use uuid;

const USE_STDIN_MARKER: &str = "-";
const NO_WKHTMLTOPDF_ERR: &str = "wkhtmltopdf tool is not found. Please install it.";

#[derive(Debug, Clone)]
pub struct Core {
    pub wkhtmltox_cmd: String,
    pub work_dir: PathBuf,
}

impl Core {
    pub fn new(wkhtmltox_cmd: String) -> Result<Self, WkhtmlError> {
        Self::bin_checks(&wkhtmltox_cmd).map_err(WkhtmlError::ServiceErr)?;
        let work_dir = env::var("WKHTMLAPP_WORK_DIR");
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
        if let Ok(value) = env::var("APP_DEBUG") {
            value == "true"
        } else {
            cfg!(debug_assertions)
        }
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
        self.work_dir = Self::parse_dir(work_dir)?;
        Ok(self)
    }

    /// Re-creates the work dir if missing. Temp cleaners (Windows Disk
    /// Cleanup/Storage Sense, systemd-tmpfiles, macOS periodic cleanup) can
    /// delete it while the app runs; without this, every render fails with
    /// "QPainter::begin(): Returned false / Unable to write to destination"
    /// until the process restarts.
    fn ensure_work_dir(&self) -> Result<(), WkhtmlError> {
        fs::create_dir_all(&self.work_dir).map_err(|e| {
            WkhtmlError::ServiceErr(format!("Failed to create working directory, due to: {}", e))
        })
    }

    pub fn get_out_path(&self, name: &str) -> PathBuf {
        let temp_name = format!("{}-{}", Uuid::new_v4(), name);
        self.work_dir.join(temp_name)
    }

    pub fn build_args(options: &HashMap<String, String>) -> Vec<String> {
        let mut args = Vec::new();
        for (key, v) in options {
            if *v == "false" {
                continue;
            }
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
                    args.push(v.to_string());
                }
            }
        }
        args
    }

    pub fn run(
        &self,
        input: WkhtmlInput,
        name: &str,
        args: Vec<String>,
    ) -> Result<PathBuf, WkhtmlError> {
        match input {
            WkhtmlInput::File(path) => self.run_with_file(path, name, args),
            WkhtmlInput::Url(url) => self.run_with_url(url, name, args),
            WkhtmlInput::Html(html) => self.run_with_html(html, name, args),
        }
    }

    pub fn run_with_url(
        &self,
        url: &str,
        name: &str,
        args: Vec<String>,
    ) -> Result<PathBuf, WkhtmlError> {
        self.ensure_work_dir()?;
        let out_path = self.get_out_path(name);
        let mut cmd = Command::new(&self.wkhtmltox_cmd);
        cmd.args(args)
            .arg(url)
            .arg(&out_path)
            .stdout(Stdio::piped());

        if !Self::get_debug() {
            cmd.stderr(Stdio::piped());
        }

        let child = cmd.spawn().map_err(|e| {
            WkhtmlError::RenderingErr(format!("Failed to spawn child process: {}", e))
        })?;

        let output = child.wait_with_output().map_err(|e| {
            WkhtmlError::RenderingErr(format!("Failed to spawn child process: {}", e))
        })?;

        #[cfg(debug_assertions)]
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
    ) -> Result<PathBuf, WkhtmlError> {
        self.ensure_work_dir()?;
        let out_path = self.get_out_path(name);
        let mut cmd = Command::new(&self.wkhtmltox_cmd);
        cmd.args(args)
            .arg(file_path)
            .arg(&out_path)
            .stdout(Stdio::piped());

        if !Self::get_debug() {
            cmd.stderr(Stdio::piped());
        }

        let child = cmd.spawn().map_err(|e| {
            WkhtmlError::RenderingErr(format!("Failed to spawn child process: {}", e))
        })?;

        let output = child.wait_with_output().map_err(|e| {
            WkhtmlError::RenderingErr(format!("Failed to spawn child process: {}", e))
        })?;

        #[cfg(debug_assertions)]
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
    ) -> Result<PathBuf, WkhtmlError> {
        self.ensure_work_dir()?;
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

        #[cfg(debug_assertions)]
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

#[cfg(test)]
mod tests {
    use super::*;

    // Regression: temp cleaners can delete the work dir while the app runs;
    // renders must re-create it instead of failing until process restart.
    #[test]
    fn ensure_work_dir_recreates_deleted_dir() {
        let work_dir = env::temp_dir().join(format!("wkhtmlapp-test-{}", Uuid::new_v4()));
        let core = Core {
            wkhtmltox_cmd: "wkhtmltopdf".to_string(),
            work_dir: work_dir.clone(),
        };

        // Dir does not exist yet (simulates cleaner having removed it).
        assert!(!work_dir.exists());
        core.ensure_work_dir().expect("should create missing work dir");
        assert!(work_dir.is_dir());

        // Idempotent when it already exists.
        core.ensure_work_dir().expect("should be a no-op when dir exists");

        // Deleted again mid-life → recreated again.
        fs::remove_dir_all(&work_dir).unwrap();
        assert!(!work_dir.exists());
        core.ensure_work_dir().expect("should re-create deleted work dir");
        assert!(work_dir.is_dir());

        let _ = fs::remove_dir_all(&work_dir);
    }
}
