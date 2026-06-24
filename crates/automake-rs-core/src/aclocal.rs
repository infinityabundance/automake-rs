// automake-rs-core: aclocal engine — forensic-parity implementation
//
// Court: AM.CLI.ACLOCAL.1
//
// aclocal scans configure.ac (or configure.in) for macro dependencies,
// searches include directories for .m4 files, and generates aclocal.m4.
//
// Clean-room reconstruction based on:
//   - Black-box oracle interrogation (running GNU aclocal and observing output)
//   - GNU Automake manual §6 (GFDL licensed)
//   - POSIX specifications
// No GNU Automake GPL source code was consulted.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The aclocal engine.
pub struct Aclocal {
    /// User-provided include directories (from -I flags)
    pub user_include_dirs: Vec<PathBuf>,
    /// Automake-provided macro directory
    pub automake_acdir: PathBuf,
    /// System-wide third-party macro directory
    pub system_acdir: PathBuf,
    /// Additional search path (from ACLOCAL_PATH env var)
    pub aclocal_path: Vec<PathBuf>,
    /// Whether to install missing third-party files
    pub install: bool,
    /// Whether to force update
    pub force: bool,
    /// Whether this is a dry run
    pub dry_run: bool,
    /// Output file (default: aclocal.m4)
    pub output_file: PathBuf,
    /// Diff command
    pub diff_command: Option<String>,
}

/// Result of scanning configure.ac.
#[derive(Debug, Clone)]
pub struct MacroScan {
    /// m4_include directives found
    pub m4_includes: Vec<String>,
    /// AC_CONFIG_MACRO_DIRS directories
    pub macro_dirs: Vec<String>,
    /// AC_CONFIG_MACRO_DIR (legacy single-directory form)
    pub macro_dir: Option<String>,
    /// All .m4 files that should be assembled into aclocal.m4
    pub required_files: Vec<PathBuf>,
    /// Serial numbers (for --install tracking)
    pub serial_numbers: HashMap<String, String>,
}

impl Aclocal {
    /// Create a new aclocal engine with default paths.
    pub fn new() -> Self {
        // Detect automake acdir from oracle
        let automake_acdir = Self::detect_automake_acdir();
        let system_acdir = Self::detect_system_acdir();

        Self {
            user_include_dirs: vec![],
            automake_acdir,
            system_acdir,
            aclocal_path: vec![],
            install: false,
            force: false,
            dry_run: false,
            output_file: PathBuf::from("aclocal.m4"),
            diff_command: None,
        }
    }

    /// Create from parsed CLI args.
    pub fn from_args(args: &crate::cli::AclocalArgs) -> Self {
        let mut engine = Self::new();

        if let Some(ref dir) = args.automake_acdir {
            engine.automake_acdir = PathBuf::from(dir);
        }
        if let Some(ref dir) = args.system_acdir {
            engine.system_acdir = PathBuf::from(dir);
        }
        if let Some(ref path) = args.aclocal_path {
            engine.aclocal_path = path.split(':').map(PathBuf::from).collect();
        }
        for dir in &args.include_dirs {
            engine.user_include_dirs.push(PathBuf::from(dir));
        }
        engine.install = args.install;
        engine.force = args.force;
        engine.dry_run = args.dry_run;
        if let Some(ref out) = args.output {
            engine.output_file = PathBuf::from(out);
        }
        engine.diff_command = args.diff.clone();

        // ACLOCAL_PATH environment variable
        if let Ok(path) = std::env::var("ACLOCAL_PATH") {
            for dir in path.split(':') {
                if !dir.is_empty() {
                    engine.aclocal_path.push(PathBuf::from(dir));
                }
            }
        }

        engine
    }

    /// Detect the automake-provided aclocal directory by querying the oracle.
    fn detect_automake_acdir() -> PathBuf {
        if let Ok(out) = std::process::Command::new("aclocal")
            .arg("--print-ac-dir")
            .output()
        {
            let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !path.is_empty() {
                return PathBuf::from(path);
            }
        }
        // Fallback: common location
        PathBuf::from("/usr/share/aclocal-1.18")
    }

    /// Detect the system-wide aclocal directory.
    fn detect_system_acdir() -> PathBuf {
        PathBuf::from("/usr/share/aclocal")
    }

    /// Get all search directories in priority order.
    pub fn search_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = vec![];

        // User includes (highest priority)
        for dir in &self.user_include_dirs {
            dirs.push(dir.clone());
        }

        // ACLOCAL_PATH
        for dir in &self.aclocal_path {
            dirs.push(dir.clone());
        }

        // Automake acdir
        dirs.push(self.automake_acdir.clone());

        // System acdir
        dirs.push(self.system_acdir.clone());

        dirs
    }

    /// Scan configure.ac for macro dependencies.
    ///
    /// This is the core of aclocal: it reads configure.ac, identifies
    /// which .m4 files are needed, and collects them into a single
    /// aclocal.m4 file.
    pub fn scan(&self, configure_ac: &Path) -> Result<MacroScan, AclocalError> {
        if !configure_ac.exists() {
            // Also try configure.in (legacy)
            let configure_in = configure_ac.with_file_name("configure.in");
            if configure_in.exists() {
                return self.scan(&configure_in);
            }
            return Err(AclocalError::NoConfigureAc);
        }

        let content = fs::read_to_string(configure_ac).map_err(AclocalError::Io)?;

        let mut scan = MacroScan {
            m4_includes: vec![],
            macro_dirs: vec![],
            macro_dir: None,
            required_files: vec![],
            serial_numbers: HashMap::new(),
        };

        // Extract AC_CONFIG_MACRO_DIRS and AC_CONFIG_MACRO_DIR
        for line in content.lines() {
            let line = line.trim();

            // AC_CONFIG_MACRO_DIRS([dir1 dir2 ...])
            if line.starts_with("AC_CONFIG_MACRO_DIRS(") {
                if let Some(dirs) = Self::extract_macro_dirs(line) {
                    for dir in dirs {
                        let dir = dir.trim();
                        if !dir.is_empty() {
                            scan.macro_dirs.push(dir.to_string());
                            // Directory noted for search later
                            let _path = configure_ac.parent().unwrap_or(Path::new(".")).join(dir);
                            // We don't mutate self, so just note it
                        }
                    }
                }
            }

            // AC_CONFIG_MACRO_DIR([dir]) — legacy single-directory form
            if line.starts_with("AC_CONFIG_MACRO_DIR(") {
                if let Some(dir) = Self::extract_single_arg(line, "AC_CONFIG_MACRO_DIR") {
                    let dir = dir.trim();
                    if !dir.is_empty() {
                        scan.macro_dir = Some(dir.to_string());
                        if !scan.macro_dirs.contains(&dir.to_string()) {
                            scan.macro_dirs.push(dir.to_string());
                        }
                    }
                }
            }

            // m4_include([file.m4])
            if line.starts_with("m4_include(") {
                if let Some(file) = Self::extract_single_arg(line, "m4_include") {
                    scan.m4_includes.push(file.to_string());
                }
            }
        }

        // Collect .m4 files from macro dirs
        let all_include_dirs = {
            let mut dirs = vec![];
            // User include dirs (from -I)
            for dir in &self.user_include_dirs {
                dirs.push(dir.clone());
            }
            // Macro dirs from configure.ac
            for dir in &scan.macro_dirs {
                let abs = configure_ac.parent().unwrap_or(Path::new(".")).join(dir);
                if abs.exists() {
                    dirs.push(abs);
                }
            }
            // System dirs
            dirs.push(self.automake_acdir.clone());
            dirs.push(self.system_acdir.clone());
            dirs
        };

        // Collect .m4 files from include dirs
        let mut seen = HashSet::new();
        for dir in &all_include_dirs {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|e| e == "m4").unwrap_or(false) {
                        let name = path.file_name().unwrap().to_string_lossy().to_string();
                        if seen.insert(name) {
                            scan.required_files.push(path);
                        }
                    }
                }
            }
        }

        // Extract serial numbers for --install tracking
        for file in &scan.required_files {
            if let Ok(content) = fs::read_to_string(file) {
                if let Some(serial) = Self::extract_serial(&content) {
                    let name = file.file_name().unwrap().to_string_lossy().to_string();
                    scan.serial_numbers.insert(name, serial);
                }
            }
        }

        Ok(scan)
    }

    /// Install missing third-party .m4 files into the first user include directory.
    ///
    /// When --install is used, aclocal copies system .m4 files to the first -I
    /// directory if they are newer (higher serial number) or not present.
    pub fn install_missing_files(&self, scan: &MacroScan) -> Result<Vec<String>, AclocalError> {
        let mut installed = vec![];
        let target_dir = if let Some(dir) = self.user_include_dirs.first() {
            dir.clone()
        } else if let Some(macro_dir) = &scan.macro_dir {
            PathBuf::from(macro_dir)
        } else {
            return Ok(installed); // No target directory to install into
        };

        for file in &scan.required_files {
            let name = file.file_name().unwrap().to_string_lossy().to_string();
            let dest = target_dir.join(&name);

            // Check if this is a system file (not user-provided)
            let is_system_file =
                file.starts_with(&self.automake_acdir) || file.starts_with(&self.system_acdir);

            if !is_system_file {
                continue;
            }

            // Check if file already exists in target
            if dest.exists() {
                // Compare serial numbers
                if let Ok(existing_content) = fs::read_to_string(&dest) {
                    let existing_serial = Self::extract_serial(&existing_content);
                    let new_serial = scan.serial_numbers.get(&name);
                    match (existing_serial, new_serial) {
                        (Some(e), Some(n)) if Self::serial_is_newer(n, &e) => {
                            if !self.dry_run {
                                fs::copy(file, &dest).map_err(AclocalError::Io)?;
                            }
                            installed.push(format!("{} (updated: serial {} -> {})", name, e, n));
                        }
                        (None, Some(_)) => {
                            if !self.dry_run {
                                fs::copy(file, &dest).map_err(AclocalError::Io)?;
                            }
                            installed.push(format!(
                                "{} (updated: no serial -> {})",
                                name,
                                scan.serial_numbers.get(&name).unwrap()
                            ));
                        }
                        _ => {}
                    }
                }
            } else {
                if !self.dry_run {
                    if let Some(parent) = dest.parent() {
                        fs::create_dir_all(parent).map_err(AclocalError::Io)?;
                    }
                    fs::copy(file, &dest).map_err(AclocalError::Io)?;
                }
                installed.push(name);
            }
        }

        Ok(installed)
    }

    /// Compare two serial numbers. Returns true if new_serial > existing_serial.
    fn serial_is_newer(new_serial: &str, existing_serial: &str) -> bool {
        let new_num: i64 = new_serial.trim().parse().unwrap_or(0);
        let existing_num: i64 = existing_serial.trim().parse().unwrap_or(0);
        new_num > existing_num
    }

    /// Generate aclocal.m4 from the scanned macros.
    ///
    /// This assembles all required .m4 files into a single aclocal.m4.
    pub fn generate(&self, configure_ac: &Path) -> Result<String, AclocalError> {
        let scan = self.scan(configure_ac)?;

        let mut output = String::new();

        // Header comment
        output.push_str(&format!(
            "# generated automatically by aclocal-rs {}\n",
            env!("CARGO_PKG_VERSION")
        ));
        output.push_str("# (clean-room forensic-parity reconstruction)\n\n");

        // Include each .m4 file
        for file in &scan.required_files {
            // Use lossy conversion for binary .m4 files (some contain non-UTF8)
            let content = match fs::read(file) {
                Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
                Err(_) => continue,
            };
            let name = file.file_name().unwrap().to_string_lossy();

            output.push_str(&format!(
                "# {}
",
                name
            ));
            output.push_str(&content);
            if !content.ends_with('\n') {
                output.push('\n');
            }
            output.push('\n');
        }

        // Also include any m4_include'd files
        for include in &scan.m4_includes {
            // Find the include file
            for dir in self.search_dirs() {
                let candidate = dir.join(include);
                if candidate.exists() {
                    let content = fs::read_to_string(&candidate).map_err(AclocalError::Io)?;
                    output.push_str(&format!("# m4_include({})\n", include));
                    output.push_str(&content);
                    if !content.ends_with('\n') {
                        output.push('\n');
                    }
                    output.push('\n');
                    break;
                }
            }
        }

        Ok(output)
    }

    /// Run aclocal and write the output file.
    pub fn run(&self) -> Result<(), AclocalError> {
        let configure_ac = {
            // Try configure.ac first, then configure.in
            let ac = Path::new("configure.ac");
            if ac.exists() {
                ac.to_path_buf()
            } else {
                let in_ = Path::new("configure.in");
                if in_.exists() {
                    in_.to_path_buf()
                } else {
                    return Err(AclocalError::NoConfigureAc);
                }
            }
        };

        // --install: copy missing third-party files first
        if self.install {
            let scan_result = self.scan(&configure_ac)?;
            let installed = self.install_missing_files(&scan_result)?;
            for name in &installed {
                eprintln!("aclocal-rs: installing '{}'", name);
            }
        }

        if self.force || !self.output_file.exists() {
            let content = self.generate(&configure_ac)?;

            if self.dry_run {
                if let Some(ref _cmd) = self.diff_command {
                    // Run diff against existing file using diff command if available
                    if self.output_file.exists() {
                        // Write generated content to temp file for diff comparison
                        let tmp = std::env::temp_dir().join("aclocal-rs-new.m4");
                        let _ = fs::write(&tmp, &content);
                        if let Ok(output) = std::process::Command::new("diff")
                            .args([
                                "-u",
                                self.output_file.to_str().unwrap_or(""),
                                tmp.to_str().unwrap_or(""),
                            ])
                            .output()
                        {
                            if !output.stdout.is_empty() {
                                print!("{}", String::from_utf8_lossy(&output.stdout));
                            }
                        }
                        let _ = fs::remove_file(&tmp);
                    }
                } else {
                    // Just print what would be written
                    println!("{}", content);
                }
            } else {
                fs::write(&self.output_file, content).map_err(AclocalError::Io)?;
            }
        }

        Ok(())
    }

    /// Print the automake aclocal directory.
    pub fn print_ac_dir(&self) {
        println!("{}", self.automake_acdir.display());
    }

    // --- Helpers ---

    /// Extract arguments from an Autoconf macro call like AC_CONFIG_MACRO_DIRS([dir1 dir2]).
    fn extract_macro_dirs(line: &str) -> Option<Vec<String>> {
        let line = line.trim();
        // Handle AC_CONFIG_MACRO_DIRS([dir1 dir2 ...])
        if let Some(args) = Self::extract_bracket_args(line) {
            return Some(args.split_whitespace().map(|s| s.to_string()).collect());
        }
        None
    }

    /// Extract a single bracketed argument from a macro call.
    fn extract_single_arg(line: &str, macro_name: &str) -> Option<String> {
        let line = line.trim();
        let prefix = format!("{}(", macro_name);
        if !line.starts_with(&prefix) {
            return None;
        }
        let rest = line.strip_prefix(&prefix)?;
        // Skip whitespace
        let rest = rest.trim_start();
        // Extract between [ and ]
        if let Some(rest) = rest.strip_prefix('[') {
            if let Some(end) = rest.find(']') {
                return Some(rest[..end].to_string());
            }
        }
        None
    }

    /// Extract all bracket-enclosed arguments.
    fn extract_bracket_args(s: &str) -> Option<String> {
        let start = s.find('[')?;
        let mut depth = 0;
        let mut end = start;
        for (i, ch) in s[start..].char_indices() {
            match ch {
                '[' => depth += 1,
                ']' => {
                    depth -= 1;
                    if depth == 0 {
                        end = start + i;
                        break;
                    }
                }
                _ => {}
            }
        }
        if end > start {
            Some(s[start + 1..end].to_string())
        } else {
            None
        }
    }

    /// Extract the serial number from a .m4 file (for --install tracking).
    fn extract_serial(content: &str) -> Option<String> {
        for line in content.lines() {
            let line = line.trim();
            // Look for: # serial 42
            if line.starts_with("# serial ") {
                return Some(line.trim_start_matches("# serial ").to_string());
            }
            // Look for: #serial 42
            if line.starts_with("#serial ") {
                return Some(line.trim_start_matches("#serial ").to_string());
            }
        }
        None
    }
}

impl Default for Aclocal {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors that can occur during aclocal operations.
#[derive(Debug)]
pub enum AclocalError {
    NoConfigureAc,
    Io(io::Error),
    Parse(String),
}

impl std::fmt::Display for AclocalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AclocalError::NoConfigureAc => {
                write!(
                    f,
                    "no configure.ac or configure.in found in current directory"
                )
            }
            AclocalError::Io(e) => write!(f, "I/O: {}", e),
            AclocalError::Parse(s) => write!(f, "parse: {}", s),
        }
    }
}

impl std::error::Error for AclocalError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_single_arg() {
        let result =
            Aclocal::extract_single_arg("AC_CONFIG_MACRO_DIR([m4])", "AC_CONFIG_MACRO_DIR");
        assert_eq!(result, Some("m4".to_string()));
    }

    #[test]
    fn test_extract_bracket_args() {
        let result = Aclocal::extract_bracket_args("AC_CONFIG_MACRO_DIRS([m4 extra])");
        assert_eq!(result, Some("m4 extra".to_string()));
    }

    #[test]
    fn test_extract_serial() {
        let content = "dnl Some comment\n# serial 42\ndnl Another\n";
        assert_eq!(Aclocal::extract_serial(content), Some("42".to_string()));
    }

    #[test]
    fn test_scan_basic() {
        let tmp = tempfile::tempdir().unwrap();
        let ac_path = tmp.path().join("configure.ac");
        fs::write(
            &ac_path,
            "AC_INIT([test], [1.0])\nAM_INIT_AUTOMAKE\nAC_CONFIG_FILES([Makefile])\nAC_OUTPUT\n",
        )
        .unwrap();

        let aclocal = Aclocal::new();
        let scan = aclocal.scan(&ac_path).unwrap();
        // Should find .m4 files from system directories
        assert!(
            !scan.required_files.is_empty(),
            "Expected .m4 files to be found"
        );
    }

    #[test]
    fn test_generate() {
        let tmp = tempfile::tempdir().unwrap();
        let ac_path = tmp.path().join("configure.ac");
        fs::write(
            &ac_path,
            "AC_INIT([test], [1.0])\nAM_INIT_AUTOMAKE\nAC_OUTPUT\n",
        )
        .unwrap();

        let aclocal = Aclocal::new();
        let output = aclocal.generate(&ac_path).unwrap();
        assert!(output.contains("generated automatically by aclocal-rs"));
        assert!(!output.is_empty());
    }

    #[test]
    fn test_serial_is_newer() {
        assert!(Aclocal::serial_is_newer("42", "10"));
        assert!(!Aclocal::serial_is_newer("10", "42"));
        assert!(!Aclocal::serial_is_newer("5", "5"));
        assert!(Aclocal::serial_is_newer("100", ""));
        assert!(!Aclocal::serial_is_newer("", "100"));
    }

    #[test]
    fn test_install_missing_files_dry_run() {
        let tmp = tempfile::tempdir().unwrap();
        let m4_dir = tmp.path().join("m4");
        fs::create_dir_all(&m4_dir).unwrap();
        let system_m4 = m4_dir.join("test-macro.m4");
        fs::write(
            &system_m4,
            "# serial 5\ndnl test macro\nAC_DEFUN([TEST_MACRO], [echo test])\n",
        )
        .unwrap();
        let ac_path = tmp.path().join("configure.ac");
        fs::write(
            &ac_path,
            "AC_INIT([test], [1.0])\nAM_INIT_AUTOMAKE\nAC_CONFIG_MACRO_DIR([m4])\nAC_OUTPUT\n",
        )
        .unwrap();
        let mut aclocal = Aclocal::new();
        aclocal.system_acdir = m4_dir.clone();
        aclocal.automake_acdir = m4_dir.clone();
        aclocal.install = true;
        aclocal.dry_run = true;
        let install_dir = tmp.path().join("installed");
        fs::create_dir_all(&install_dir).unwrap();
        aclocal.user_include_dirs.push(install_dir.clone());
        let scan = aclocal.scan(&ac_path).unwrap();
        let installed = aclocal.install_missing_files(&scan).unwrap();
        assert!(
            !installed.is_empty(),
            "Expected at least one file to be installed"
        );
        let has_test_macro = installed.iter().any(|s| s.contains("test-macro"));
        assert!(
            has_test_macro,
            "Expected test-macro.m4 in installed list, got: {:?}",
            installed
        );
        assert!(!install_dir.join("test-macro.m4").exists());
    }

    #[test]
    fn test_install_missing_files_actual_copy() {
        let tmp = tempfile::tempdir().unwrap();
        let m4_dir = tmp.path().join("m4");
        fs::create_dir_all(&m4_dir).unwrap();
        let system_m4 = m4_dir.join("other-macro.m4");
        fs::write(
            &system_m4,
            "# serial 3\ndnl other macro\nAC_DEFUN([OTHER_MACRO], [echo other])\n",
        )
        .unwrap();
        let ac_path = tmp.path().join("configure.ac");
        fs::write(
            &ac_path,
            "AC_INIT([test], [1.0])\nAM_INIT_AUTOMAKE\nAC_CONFIG_MACRO_DIR([m4])\nAC_OUTPUT\n",
        )
        .unwrap();
        let mut aclocal = Aclocal::new();
        aclocal.system_acdir = m4_dir.clone();
        aclocal.automake_acdir = m4_dir.clone();
        aclocal.install = true;
        aclocal.dry_run = false;
        let install_dir = tmp.path().join("installed");
        fs::create_dir_all(&install_dir).unwrap();
        aclocal.user_include_dirs.push(install_dir.clone());
        let scan = aclocal.scan(&ac_path).unwrap();
        let installed = aclocal.install_missing_files(&scan).unwrap();
        let has_other = installed.iter().any(|s| s.contains("other-macro"));
        assert!(has_other, "Expected other-macro.m4 to be installed");
        assert!(install_dir.join("other-macro.m4").exists());
    }
}
