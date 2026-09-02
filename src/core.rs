use crate::{WkhtmlError, WkhtmlInput};

use self::uuid::Uuid;
use log::{debug, error, info};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::{env, fs, io::Write};
use uuid;

const USE_STDIN_MARKER: &str = "-";

/// Options that take two space-separated values, e.g. `--cookie name value`.
const TWO_VALUE_OPTIONS: &[&str] = &["cookie", "custom-header", "post", "post-file", "replace"];

/// TOC-specific options: wkhtmltopdf requires them after the `toc` object.
const TOC_OPTIONS: &[&str] = &[
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
];

/// Shared engine that spawns the wkhtmltox binary and manages the work dir.
#[derive(Debug, Clone)]
pub struct Core {
    pub wkhtmltox_cmd: String,
    pub work_dir: PathBuf,
}

/// Prevents a console window from flashing on Windows when the parent is a GUI
/// application (`windows_subsystem = "windows"`): without `CREATE_NO_WINDOW`,
/// every wkhtmltox spawn opens a visible console for the child process.
/// No-op on other platforms.
#[cfg(windows)]
fn hide_console_window(cmd: &mut Command) {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    cmd.creation_flags(CREATE_NO_WINDOW);
}

#[cfg(not(windows))]
fn hide_console_window(_cmd: &mut Command) {}

impl Core {
    /// Verifies the binary exists and prepares the work dir
    /// (`WKHTMLAPP_WORK_DIR` env var, or the OS temp dir by default).
    pub fn new(wkhtmltox_cmd: String) -> Result<Self, WkhtmlError> {
        Self::bin_checks(&wkhtmltox_cmd).map_err(WkhtmlError::ServiceErr)?;
        let work_dir = env::var_os("WKHTMLAPP_WORK_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(Self::default_work_dir);
        fs::create_dir_all(&work_dir).map_err(|e| {
            WkhtmlError::ServiceErr(format!("Failed to create working directory, due to: {}", e))
        })?;

        Ok(Self {
            wkhtmltox_cmd,
            work_dir,
        })
    }

    pub fn default_work_dir() -> PathBuf {
        env::temp_dir().join("wkhtmlapp")
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

    /// Checks the binary responds to `-V` without leaking its output
    /// to the parent process' stdio.
    pub fn bin_checks(wkhtmltox_cmd: &str) -> Result<(), String> {
        info!("Bootstrap check for {} tool", wkhtmltox_cmd);
        let mut check = Command::new(wkhtmltox_cmd);
        check
            .arg("-V")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        hide_console_window(&mut check);
        let status = check.status();

        match status {
            Ok(s) if s.success() => Ok(()),
            Ok(s) => {
                error!("{} -V exited with {}", wkhtmltox_cmd, s);
                Err(format!(
                    "{} tool is not found. Please install it.",
                    wkhtmltox_cmd
                ))
            }
            Err(e) => {
                error!("Failed to spawn {}: {}", wkhtmltox_cmd, e);
                Err(format!(
                    "{} tool is not found. Please install it.",
                    wkhtmltox_cmd
                ))
            }
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

    /// Removes all files (not subdirectories) from the work dir. Rendered
    /// outputs are never deleted automatically; call this periodically to
    /// avoid filling the disk.
    pub fn clear_work_dir(&self) -> Result<(), WkhtmlError> {
        if !self.work_dir.exists() {
            return Ok(());
        }
        let entries = fs::read_dir(&self.work_dir).map_err(|e| {
            WkhtmlError::ServiceErr(format!("Failed to read working directory, due to: {}", e))
        })?;
        for entry in entries {
            let path = entry
                .map_err(|e| {
                    WkhtmlError::ServiceErr(format!(
                        "Failed to read working directory, due to: {}",
                        e
                    ))
                })?
                .path();
            if path.is_file() {
                fs::remove_file(&path).map_err(|e| {
                    WkhtmlError::ServiceErr(format!("Failed to remove {:?}, due to: {}", path, e))
                })?;
            }
        }
        Ok(())
    }

    /// Rejects names that would escape the work dir (path traversal).
    fn validate_name(name: &str) -> Result<(), WkhtmlError> {
        if name.is_empty() || name.contains(['/', '\\']) || name.contains("..") {
            Err(WkhtmlError::ServiceErr(format!(
                "Invalid output name: {:?} (must not contain path separators or '..')",
                name
            )))
        } else {
            Ok(())
        }
    }

    pub fn get_out_path(&self, name: &str) -> PathBuf {
        let temp_name = format!("{}-{}", Uuid::new_v4(), name);
        self.work_dir.join(temp_name)
    }

    /// Builds the CLI argument list. Options are emitted in sorted order
    /// (deterministic runs), `extra_args` keep insertion order and may repeat
    /// keys (e.g. several `cookie`s). `cover`/`toc` are emitted last as
    /// positional objects — `cover` keeps its input value and TOC options are
    /// placed after the `toc` object, as wkhtmltopdf requires.
    pub fn build_args(
        options: &HashMap<String, String>,
        extra_args: &[(String, String)],
    ) -> Vec<String> {
        let mut sorted: Vec<(&str, &str)> = options
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        sorted.sort_unstable();
        let all = sorted
            .into_iter()
            .chain(extra_args.iter().map(|(k, v)| (k.as_str(), v.as_str())));

        let mut args = Vec::new();
        let mut toc_args = Vec::new();
        let mut cover: Option<&str> = None;
        let mut toc = false;

        for (key, value) in all {
            match key {
                _ if value == "false" => {}
                "cover" => cover = Some(value),
                "toc" => toc = true,
                _ if TOC_OPTIONS.contains(&key) => Self::push_option(&mut toc_args, key, value),
                _ => Self::push_option(&mut args, key, value),
            }
        }

        if let Some(input) = cover {
            args.push("cover".to_string());
            if input != "true" {
                args.push(input.to_string());
            }
        }
        if toc {
            args.push("toc".to_string());
        }
        args.append(&mut toc_args);
        args
    }

    fn push_option(args: &mut Vec<String>, key: &str, value: &str) {
        args.push(format!("--{}", key));
        if value == "true" {
            return;
        }
        if TWO_VALUE_OPTIONS.contains(&key) {
            if let Some((first, second)) = value.split_once(char::is_whitespace) {
                args.push(first.to_string());
                args.push(second.trim_start().to_string());
                return;
            }
        }
        args.push(value.to_string());
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
        self.execute(url, None, name, args)
    }

    pub fn run_with_file(
        &self,
        file_path: &str,
        name: &str,
        args: Vec<String>,
    ) -> Result<PathBuf, WkhtmlError> {
        self.execute(file_path, None, name, args)
    }

    pub fn run_with_html(
        &self,
        html: &str,
        name: &str,
        args: Vec<String>,
    ) -> Result<PathBuf, WkhtmlError> {
        self.execute(USE_STDIN_MARKER, Some(html), name, args)
    }

    /// Single render path: spawns the binary with the input as last argument
    /// (or via stdin when `html` is given), always capturing stderr so
    /// failures carry the real wkhtmltox error message.
    fn execute(
        &self,
        input_arg: &str,
        html: Option<&str>,
        name: &str,
        args: Vec<String>,
    ) -> Result<PathBuf, WkhtmlError> {
        Self::validate_name(name)?;
        self.ensure_work_dir()?;
        let out_path = self.get_out_path(name);
        let mut cmd = Command::new(&self.wkhtmltox_cmd);
        cmd.args(args)
            .arg(input_arg)
            .arg(&out_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        hide_console_window(&mut cmd);
        if html.is_some() {
            cmd.stdin(Stdio::piped());
        }

        let mut child = cmd.spawn().map_err(|e| {
            WkhtmlError::RenderingErr(format!("Failed to spawn child process: {}", e))
        })?;

        if let Some(html) = html {
            child
                .stdin
                .as_mut()
                .ok_or_else(|| WkhtmlError::RenderingErr("Failed to open stdin".to_string()))?
                .write_all(html.as_bytes())
                .map_err(|e| {
                    WkhtmlError::RenderingErr(format!("Failed to write to stdin: {}", e))
                })?;
        }

        let output = child.wait_with_output().map_err(|e| {
            WkhtmlError::RenderingErr(format!("Failed to wait for child process: {}", e))
        })?;

        Self::depure(&output);

        if output.status.success() {
            Ok(out_path)
        } else {
            // Drop partial output so failed renders don't leave garbage behind.
            let _ = fs::remove_file(&out_path);
            Err(WkhtmlError::RenderingErr(format!(
                "Failed to render ({}): {}",
                output.status,
                String::from_utf8_lossy(&output.stderr).trim()
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn build_args_is_deterministic_and_sorted() {
        let options = opts(&[("zoom", "2"), ("dpi", "300"), ("grayscale", "true")]);
        let args = Core::build_args(&options, &[]);
        assert_eq!(args, vec!["--dpi", "300", "--grayscale", "--zoom", "2"]);
    }

    #[test]
    fn build_args_skips_false_values() {
        let options = opts(&[("grayscale", "false"), ("toc", "false"), ("cover", "false")]);
        assert!(Core::build_args(&options, &[]).is_empty());
    }

    // Regression: cover is a positional object whose input value was dropped.
    #[test]
    fn build_args_emits_cover_with_input() {
        let options = opts(&[("cover", "cover.html"), ("dpi", "300")]);
        let args = Core::build_args(&options, &[]);
        assert_eq!(args, vec!["--dpi", "300", "cover", "cover.html"]);
    }

    #[test]
    fn build_args_places_toc_options_after_toc_object() {
        let options = opts(&[("toc", "true"), ("toc-depth", "2"), ("dpi", "300")]);
        let args = Core::build_args(&options, &[]);
        assert_eq!(args, vec!["--dpi", "300", "toc", "--toc-depth", "2"]);
    }

    #[test]
    fn build_args_splits_two_value_options() {
        let options = opts(&[("cookie", "session abc123")]);
        let args = Core::build_args(&options, &[]);
        assert_eq!(args, vec!["--cookie", "session", "abc123"]);
    }

    #[test]
    fn build_args_keeps_repeatable_extra_args_in_order() {
        let extra = vec![
            ("cookie".to_string(), "a 1".to_string()),
            ("cookie".to_string(), "b 2".to_string()),
            ("allow".to_string(), "/tmp/assets".to_string()),
        ];
        let args = Core::build_args(&HashMap::new(), &extra);
        assert_eq!(
            args,
            vec![
                "--cookie",
                "a",
                "1",
                "--cookie",
                "b",
                "2",
                "--allow",
                "/tmp/assets"
            ]
        );
    }

    #[test]
    fn validate_name_rejects_path_traversal() {
        assert!(Core::validate_name("report.pdf").is_ok());
        assert!(Core::validate_name("").is_err());
        assert!(Core::validate_name("../escape.pdf").is_err());
        assert!(Core::validate_name("a/b.pdf").is_err());
        assert!(Core::validate_name("a\\b.pdf").is_err());
    }

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
        core.ensure_work_dir()
            .expect("should create missing work dir");
        assert!(work_dir.is_dir());

        // Idempotent when it already exists.
        core.ensure_work_dir()
            .expect("should be a no-op when dir exists");

        // Deleted again mid-life → recreated again.
        fs::remove_dir_all(&work_dir).unwrap();
        assert!(!work_dir.exists());
        core.ensure_work_dir()
            .expect("should re-create deleted work dir");
        assert!(work_dir.is_dir());

        let _ = fs::remove_dir_all(&work_dir);
    }

    #[test]
    fn clear_work_dir_removes_only_files() {
        let work_dir = env::temp_dir().join(format!("wkhtmlapp-clear-{}", Uuid::new_v4()));
        fs::create_dir_all(&work_dir).unwrap();
        fs::write(work_dir.join("a.pdf"), b"x").unwrap();
        fs::create_dir_all(work_dir.join("keep")).unwrap();

        let core = Core {
            wkhtmltox_cmd: "wkhtmltopdf".to_string(),
            work_dir: work_dir.clone(),
        };
        core.clear_work_dir().expect("should clear files");

        assert!(!work_dir.join("a.pdf").exists());
        assert!(work_dir.join("keep").is_dir());

        let _ = fs::remove_dir_all(&work_dir);
    }
}
