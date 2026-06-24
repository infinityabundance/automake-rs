// automake-rs-core: CLI argument parsing
//
// Parses command-line arguments for the automake and aclocal binaries
// with 1:1 correspondence to GNU Automake's CLI surface.
//
// Court: AM.CLI.1 — CLI harness

use std::path::PathBuf;

/// Parsed automake command-line arguments.
#[derive(Debug, Clone, Default)]
pub struct AutomakeArgs {
    /// Input Makefile.am files (or directories)
    pub input_files: Vec<PathBuf>,
    /// Operation modes
    pub help: bool,
    pub version: bool,
    pub verbose: bool,
    /// --no-force: only update out-of-date Makefile.in's
    pub no_force: bool,
    /// -W CATEGORY / --warnings=CATEGORY
    pub warnings: Vec<String>,
    /// Dependency tracking
    pub ignore_deps: bool,
    pub include_deps: bool,
    /// Strictness flavors
    pub foreign: bool,
    pub gnu: bool,
    pub gnits: bool,
    /// Library files
    pub add_missing: bool,
    pub libdir: Option<String>,
    pub print_libdir: bool,
    pub copy: bool,
    pub force_missing: bool,
    /// Cross-compilation host triple (NC.PERM.4)
    pub host: Option<String>,
    /// Cross-compilation build triple (NC.PERM.4)
    pub build: Option<String>,
    /// Environment variables
    pub env_automake: Option<String>,
    pub env_aclocal: Option<String>,
    pub env_autoconf: Option<String>,
    pub env_autom4te: Option<String>,
    pub env_m4: Option<String>,
    pub env_make: Option<String>,
}

impl AutomakeArgs {
    pub fn parse(args: &[String]) -> Result<Self, String> {
        let mut parsed = AutomakeArgs::default();
        let mut i = 0;
        // Skip program name
        if !args.is_empty() {
            i = 1;
        }

        while i < args.len() {
            let arg = &args[i];
            match arg.as_str() {
                "--help" | "-h" => parsed.help = true,
                "--version" | "-V" => parsed.version = true,
                "-v" | "--verbose" => parsed.verbose = true,
                "--no-force" => parsed.no_force = true,
                "-i" | "--ignore-deps" => parsed.ignore_deps = true,
                "--include-deps" => parsed.include_deps = true,
                "--foreign" => parsed.foreign = true,
                "--gnu" => parsed.gnu = true,
                "--gnits" => parsed.gnits = true,
                "-a" | "--add-missing" => parsed.add_missing = true,
                "-c" | "--copy" => parsed.copy = true,
                "-f" | "--force-missing" => parsed.force_missing = true,
                "--print-libdir" => parsed.print_libdir = true,
                "-W" => {
                    i += 1;
                    if i < args.len() {
                        parsed.warnings.push(args[i].clone());
                    }
                }
                s if s.starts_with("--warnings=") => {
                    let cat = s.trim_start_matches("--warnings=");
                    parsed.warnings.push(cat.to_string());
                }
                s if s.starts_with("--libdir=") => {
                    parsed.libdir = Some(s.trim_start_matches("--libdir=").to_string());
                }
                s if s.starts_with("--host=") => {
                    parsed.host = Some(s.trim_start_matches("--host=").to_string());
                }
                s if s.starts_with("--build=") => {
                    parsed.build = Some(s.trim_start_matches("--build=").to_string());
                }
                s if s.starts_with("-") => {
                    // Unknown flag — passthrough for compatibility
                    eprintln!("automake-rs: warning: unknown flag: {}", s);
                }
                _ => {
                    // Treat as input file
                    parsed.input_files.push(PathBuf::from(arg));
                }
            }
            i += 1;
        }

        // Apply environment variables
        if let Ok(val) = std::env::var("AUTOMAKE") {
            parsed.env_automake = Some(val);
        }
        if let Ok(val) = std::env::var("ACLOCAL") {
            parsed.env_aclocal = Some(val);
        }
        if let Ok(val) = std::env::var("AUTOCONF") {
            parsed.env_autoconf = Some(val);
        }
        if let Ok(val) = std::env::var("AUTOM4TE") {
            parsed.env_autom4te = Some(val);
        }
        if let Ok(val) = std::env::var("M4") {
            parsed.env_m4 = Some(val);
        }
        if let Ok(val) = std::env::var("MAKE") {
            parsed.env_make = Some(val);
        }

        Ok(parsed)
    }

    /// Get the effective strictness (foreign, gnu, or gnits).
    pub fn strictness(&self) -> &str {
        if self.gnits {
            "gnits"
        } else if self.gnu {
            "gnu"
        } else if self.foreign {
            "foreign"
        } else {
            "gnu"
        } // default
    }

    /// Whether dependency tracking should be enabled.
    pub fn dependency_tracking_enabled(&self) -> Option<bool> {
        if self.ignore_deps {
            Some(false)
        } else if self.include_deps {
            Some(true)
        } else {
            None
        } // use default
    }
}

/// Parsed aclocal command-line arguments.
#[derive(Debug, Clone, Default)]
pub struct AclocalArgs {
    /// Operation modes
    pub help: bool,
    pub version: bool,
    pub verbose: bool,
    pub dry_run: bool,
    pub force: bool,
    pub install: bool,
    pub print_ac_dir: bool,
    /// -I DIR include paths
    pub include_dirs: Vec<String>,
    /// --automake-acdir=DIR
    pub automake_acdir: Option<String>,
    /// --system-acdir=DIR
    pub system_acdir: Option<String>,
    /// --aclocal-path=PATH
    pub aclocal_path: Option<String>,
    /// --diff[=COMMAND]
    pub diff: Option<String>,
    /// --output=FILE
    pub output: Option<String>,
    /// -W CATEGORY / --warnings=CATEGORY
    pub warnings: Vec<String>,
    /// Input files (configure.ac etc.)
    pub input_files: Vec<PathBuf>,
}

impl AclocalArgs {
    pub fn parse(args: &[String]) -> Result<Self, String> {
        let mut parsed = AclocalArgs::default();
        let mut i = 0;
        if !args.is_empty() {
            i = 1;
        }

        while i < args.len() {
            let arg = &args[i];
            match arg.as_str() {
                "--help" | "-h" => parsed.help = true,
                "--version" | "-V" => parsed.version = true,
                "--verbose" => parsed.verbose = true,
                "--dry-run" => parsed.dry_run = true,
                "--force" => parsed.force = true,
                "--install" => parsed.install = true,
                "--print-ac-dir" => parsed.print_ac_dir = true,
                "-I" => {
                    i += 1;
                    if i < args.len() {
                        parsed.include_dirs.push(args[i].clone());
                    }
                }
                "-W" => {
                    i += 1;
                    if i < args.len() {
                        parsed.warnings.push(args[i].clone());
                    }
                }
                s if s.starts_with("--warnings=") => {
                    let cat = s.trim_start_matches("--warnings=");
                    parsed.warnings.push(cat.to_string());
                }
                s if s.starts_with("--automake-acdir=") => {
                    parsed.automake_acdir =
                        Some(s.trim_start_matches("--automake-acdir=").to_string());
                }
                s if s.starts_with("--system-acdir=") => {
                    parsed.system_acdir = Some(s.trim_start_matches("--system-acdir=").to_string());
                }
                s if s.starts_with("--aclocal-path=") => {
                    parsed.aclocal_path = Some(s.trim_start_matches("--aclocal-path=").to_string());
                }
                s if s.starts_with("--diff=") => {
                    parsed.diff = Some(s.trim_start_matches("--diff=").to_string());
                }
                "--diff" => {
                    parsed.diff = Some("diff -u".to_string());
                    parsed.install = true;
                    parsed.dry_run = true;
                }
                s if s.starts_with("--output=") => {
                    parsed.output = Some(s.trim_start_matches("--output=").to_string());
                }
                s if s.starts_with("-") => {
                    eprintln!("aclocal-rs: warning: unknown flag: {}", s);
                }
                _ => {
                    parsed.input_files.push(PathBuf::from(arg));
                }
            }
            i += 1;
        }

        Ok(parsed)
    }
}

/// Oracle version string — loaded once from the oracle profile.
pub fn oracle_version() -> String {
    if let Ok(data) = std::fs::read_to_string("reports/oracle-profile.json") {
        if let Ok(profile) = serde_json::from_str::<serde_json::Value>(&data) {
            if let Some(binaries) = profile.get("binaries") {
                if let Some(am) = binaries.get("automake") {
                    if let Some(ver) = am.get("version_output") {
                        return ver.as_str().unwrap_or("unknown").to_string();
                    }
                }
            }
        }
    }
    "automake (GNU automake) UNKNOWN".to_string()
}

/// Oracle version string for aclocal.
pub fn oracle_version_aclocal() -> String {
    if let Ok(data) = std::fs::read_to_string("reports/oracle-profile.json") {
        if let Ok(profile) = serde_json::from_str::<serde_json::Value>(&data) {
            if let Some(binaries) = profile.get("binaries") {
                if let Some(ac) = binaries.get("aclocal") {
                    if let Some(ver) = ac.get("version_output") {
                        return ver.as_str().unwrap_or("unknown").to_string();
                    }
                }
            }
        }
    }
    "aclocal (GNU automake) UNKNOWN".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automake_args_parse_help() {
        let args = vec!["automake".to_string(), "--help".to_string()];
        let parsed = AutomakeArgs::parse(&args).unwrap();
        assert!(parsed.help);
    }

    #[test]
    fn test_automake_args_parse_version() {
        let args = vec!["automake".to_string(), "-V".to_string()];
        let parsed = AutomakeArgs::parse(&args).unwrap();
        assert!(parsed.version);
    }

    #[test]
    fn test_automake_args_parse_complex() {
        let args = vec![
            "automake".to_string(),
            "--foreign".to_string(),
            "-a".to_string(),
            "-c".to_string(),
            "-v".to_string(),
            "-W".to_string(),
            "all".to_string(),
            "Makefile.am".to_string(),
        ];
        let parsed = AutomakeArgs::parse(&args).unwrap();
        assert!(parsed.foreign);
        assert!(parsed.add_missing);
        assert!(parsed.copy);
        assert!(parsed.verbose);
        assert_eq!(parsed.warnings, vec!["all"]);
        assert_eq!(parsed.input_files.len(), 1);
    }

    #[test]
    fn test_automake_args_strictness_default() {
        let parsed = AutomakeArgs::default();
        assert_eq!(parsed.strictness(), "gnu");
    }

    #[test]
    fn test_aclocal_args_parse() {
        let args = vec![
            "aclocal".to_string(),
            "-I".to_string(),
            "m4".to_string(),
            "--install".to_string(),
            "--verbose".to_string(),
        ];
        let parsed = AclocalArgs::parse(&args).unwrap();
        assert_eq!(parsed.include_dirs, vec!["m4"]);
        assert!(parsed.install);
        assert!(parsed.verbose);
    }

    #[test]
    fn test_oracle_version() {
        let ver = oracle_version();
        assert!(!ver.is_empty());
        assert!(ver.contains("automake"));
    }

    #[test]
    fn test_oracle_version_aclocal() {
        let ver = oracle_version_aclocal();
        assert!(!ver.is_empty());
        assert!(ver.contains("aclocal"));
    }
}
