// automake-oracle-rs: GNU Automake oracle admission
//
// Locates the system GNU Automake binaries (automake, aclocal), captures
// their identity fingerprints, runs smoke tests, and emits an oracle profile
// that all subsequent parity courts reference.
//
// Clean-room design: we interrogate the binaries as black-box oracles.
// No implementation code is consulted — only binary output is captured.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};

/// An admitted oracle — a specific set of Automake-related binaries with known identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleProfile {
    /// Human label, e.g. "gnu_automake_1_17_default"
    pub kind: String,
    /// Timestamp of admission
    pub admitted_at: String,
    /// Platform triple (e.g. "x86_64-unknown-linux-gnu")
    pub platform: String,
    /// Locale used for admission (e.g. "C")
    pub locale: String,
    /// Shell used by the oracle infrastructure
    pub shell: String,
    /// OS release info
    pub os_release: String,
    /// Profiles for each oracle binary
    pub binaries: HashMap<String, BinaryProfile>,
    /// Subordinate oracles (autoconf, m4, make, etc.)
    pub subordinate_oracles: HashMap<String, SubordinateOracle>,
    /// Feature flags detected
    pub features: OracleFeatures,
    /// Registry of receipts admitted against this oracle
    pub receipt_registry: Vec<String>,
}

/// Profile for a single oracle binary (e.g., automake, aclocal).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryProfile {
    /// Path to the executable
    pub path: String,
    /// Raw output of `--version`
    pub version_output: String,
    /// SHA-256 of the executable binary
    pub sha256: String,
    /// CLI flags detected via --help
    pub supported_flags: Vec<String>,
    /// Environment variables recognized
    pub env_vars: Vec<String>,
}

/// Profile for a subordinate oracle (binary Automake depends on but doesn't ship).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubordinateOracle {
    /// Binary name (e.g., "autoconf", "m4", "make")
    pub name: String,
    /// Path to the executable
    pub path: String,
    /// Version output
    pub version_output: String,
    /// SHA-256 of the binary
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OracleFeatures {
    pub multiple_output: bool,
    pub dependency_tracking: bool,
    pub silent_rules: bool,
    pub parallel_tests: bool,
    pub tap_tests: bool,
    pub libtool_support: bool,
    pub warnings_categories: Vec<String>,
    pub strictness_modes: Vec<String>,
}

/// Result of running an oracle command.
#[derive(Debug, Clone)]
pub struct OracleRun {
    pub exit_status: ExitStatus,
    pub exit_code: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

/// Configuration for oracle admission.
#[derive(Debug, Clone)]
pub struct OracleConfig {
    /// Explicit paths to oracle binaries
    pub binary_paths: HashMap<String, Option<PathBuf>>,
    /// Locale to use (default "C")
    pub locale: String,
    /// Shell to use
    pub shell: String,
    /// Additional environment variables
    pub env: HashMap<String, String>,
}

impl Default for OracleConfig {
    fn default() -> Self {
        let mut binary_paths = HashMap::new();
        binary_paths.insert("automake".to_string(), None);
        binary_paths.insert("aclocal".to_string(), None);
        // Subordinate oracles: Autoconf binaries, m4, make, shell
        binary_paths.insert("autoconf".to_string(), None);
        binary_paths.insert("autoheader".to_string(), None);
        binary_paths.insert("autom4te".to_string(), None);
        binary_paths.insert("autoreconf".to_string(), None);
        binary_paths.insert("m4".to_string(), None);
        binary_paths.insert("make".to_string(), None);

        Self {
            binary_paths,
            locale: "C".to_string(),
            shell: "/bin/sh".to_string(),
            env: HashMap::new(),
        }
    }
}

/// Locate a binary on the system.
pub fn locate_binary(name: &str, explicit_path: Option<&PathBuf>) -> Result<PathBuf, OracleError> {
    if let Some(path) = explicit_path {
        if path.exists() {
            return Ok(path.clone());
        }
        return Err(OracleError::NotFound(format!(
            "explicit path not found for {}: {}",
            name,
            path.display()
        )));
    }

    // Try common locations
    let candidates = &[
        name,
        &format!("/usr/bin/{}", name),
        &format!("/usr/local/bin/{}", name),
    ];

    for candidate in candidates {
        let path = Path::new(candidate);
        if path.exists() {
            return Ok(path.to_path_buf());
        }
    }

    Err(OracleError::NotFound(format!(
        "{} not found on PATH. Install GNU Automake or set binary_paths.",
        name
    )))
}

/// Run the oracle with given stdin and arguments.
pub fn run_oracle(
    oracle_path: &Path,
    args: &[&str],
    stdin: &[u8],
    working_dir: Option<&Path>,
    env: &HashMap<String, String>,
) -> io::Result<OracleRun> {
    let mut cmd = Command::new(oracle_path);
    cmd.args(args);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.env_clear();

    // Set controlled environment
    cmd.env("PATH", "/usr/bin:/bin:/usr/local/bin");
    cmd.env("LC_ALL", "C");
    cmd.env("LANG", "C");
    cmd.env("TZ", "UTC");
    for (k, v) in env {
        cmd.env(k, v);
    }

    if let Some(dir) = working_dir {
        cmd.current_dir(dir);
    }

    let mut child = cmd.spawn()?;

    // Write stdin
    if let Some(mut sin) = child.stdin.take() {
        sin.write_all(stdin)?;
    }

    let output = child.wait_with_output()?;

    Ok(OracleRun {
        exit_status: output.status,
        exit_code: output.status.code(),
        stdout: output.stdout,
        stderr: output.stderr,
    })
}

/// Run the oracle with stdin from a string and capture output as strings.
pub fn run_oracle_text(
    oracle_path: &Path,
    args: &[&str],
    stdin: &str,
    working_dir: Option<&Path>,
    env: &HashMap<String, String>,
) -> io::Result<OracleRun> {
    run_oracle(oracle_path, args, stdin.as_bytes(), working_dir, env)
}

/// Compute SHA-256 of a binary file.
pub fn compute_sha256(path: &Path) -> Result<String, OracleError> {
    let bytes = std::fs::read(path).map_err(|e| OracleError::Io(e.to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Ok(format!("{:x}", hasher.finalize()))
}

/// Detect supported flags from --help output.
fn detect_flags(help_output: &str) -> Vec<String> {
    let mut flags = Vec::new();
    // Parse common Automake flags from --help
    let known_flags = &[
        "--add-missing",
        "-a",
        "--copy",
        "-c",
        "--force-missing",
        "-f",
        "--foreign",
        "--gnu",
        "--gnits",
        "--ignore-deps",
        "-i",
        "--include-deps",
        "--no-force",
        "--verbose",
        "-v",
        "--warnings",
        "-W",
        "--help",
        "-h",
        "--version",
        "-V",
    ];

    for flag in known_flags {
        if help_output.contains(flag) {
            flags.push(flag.to_string());
        }
    }
    flags
}

/// Detect environment variables from --help output.
fn detect_env_vars(help_output: &str) -> Vec<String> {
    let mut vars = Vec::new();
    let known_vars = &[
        "AUTOMAKE", "ACLOCAL", "AUTOCONF", "AUTOM4TE", "M4", "MAKE", "WARNINGS",
    ];

    for var in known_vars {
        if help_output.contains(var) {
            vars.push(var.to_string());
        }
    }
    vars
}

/// Detect warnings categories.
fn detect_warnings_categories(help_output: &str) -> Vec<String> {
    let mut cats = Vec::new();
    let known = &[
        "gnu",
        "gnits",
        "foreign",
        "portability",
        "syntax",
        "unsupported",
        "all",
        "none",
        "no-",
        "error",
    ];

    for cat in known {
        if help_output.to_lowercase().contains(cat) || help_output.contains(cat) {
            cats.push(cat.to_string());
        }
    }
    cats
}

/// Extract the profile kind from version output.
fn extract_profile_kind(version_output: &str) -> String {
    if let Some(line) = version_output.lines().next() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        for (i, part) in parts.iter().enumerate() {
            if part
                .chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
            {
                let version = part.trim_end_matches(',');
                return format!("gnu_automake_{}_default", version.replace('.', "_"));
            }
            if *part == "Automake)" && i + 1 < parts.len() {
                let version = parts[i + 1].trim_end_matches(',');
                return format!("gnu_automake_{}_default", version.replace('.', "_"));
            }
        }
    }
    "gnu_automake_unknown_default".to_string()
}

fn read_os_release() -> String {
    if let Ok(contents) = std::fs::read_to_string("/etc/os-release") {
        for line in contents.lines() {
            if line.starts_with("PRETTY_NAME=") {
                return line
                    .trim_start_matches("PRETTY_NAME=")
                    .trim_matches('"')
                    .to_string();
            }
        }
    }
    format!("{} {}", std::env::consts::OS, std::env::consts::ARCH)
}

fn chrono_now() -> String {
    use std::time::SystemTime;
    let dur = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", dur.as_secs())
}

/// Admit all oracle binaries and build a complete OracleProfile.
pub fn admit_oracle(config: &OracleConfig) -> Result<OracleProfile, OracleError> {
    let platform = format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH);
    let os_release = read_os_release();

    // 1. Admit automake
    let am_path = locate_binary(
        "automake",
        config.binary_paths.get("automake").and_then(|p| p.as_ref()),
    )?;
    let abs_am = std::fs::canonicalize(&am_path).map_err(|e| OracleError::Io(e.to_string()))?;

    let version_run = run_oracle_text(&abs_am, &["--version"], "", None, &config.env)
        .map_err(|e| OracleError::Execution(format!("automake --version: {}", e)))?;
    let version_output = String::from_utf8_lossy(&version_run.stdout).to_string();

    if !version_output.contains("GNU Automake") && !version_output.contains("GNU automake") {
        return Err(OracleError::NotGnu(format!(
            "binary at {} does not identify as GNU Automake:\n{}",
            abs_am.display(),
            version_output
        )));
    }

    let sha256 = compute_sha256(&abs_am)?;
    let kind = extract_profile_kind(&version_output);

    // Detect flags
    let help_run = run_oracle_text(&abs_am, &["--help"], "", None, &config.env)
        .map_err(|e| OracleError::Execution(format!("automake --help: {}", e)))?;
    let help_output = String::from_utf8_lossy(&help_run.stdout);
    let supported_flags = detect_flags(&help_output);
    let env_vars = detect_env_vars(&help_output);
    let warnings_categories = detect_warnings_categories(&help_output);

    let mut binaries = HashMap::new();
    binaries.insert(
        "automake".to_string(),
        BinaryProfile {
            path: abs_am.to_string_lossy().to_string(),
            version_output: version_output.clone(),
            sha256: sha256.clone(),
            supported_flags,
            env_vars,
        },
    );

    // 2. Admit aclocal
    let ac_path = locate_binary(
        "aclocal",
        config.binary_paths.get("aclocal").and_then(|p| p.as_ref()),
    )?;
    let abs_ac = std::fs::canonicalize(&ac_path).map_err(|e| OracleError::Io(e.to_string()))?;
    let ac_version_run = run_oracle_text(&abs_ac, &["--version"], "", None, &config.env)
        .map_err(|e| OracleError::Execution(format!("aclocal --version: {}", e)))?;
    let ac_version = String::from_utf8_lossy(&ac_version_run.stdout).to_string();
    let ac_sha256 = compute_sha256(&abs_ac)?;

    let ac_help_run = run_oracle_text(&abs_ac, &["--help"], "", None, &config.env)
        .map_err(|e| OracleError::Execution(format!("aclocal --help: {}", e)))?;
    let ac_help = String::from_utf8_lossy(&ac_help_run.stdout);
    let ac_flags = detect_flags(&ac_help);
    let ac_env_vars = detect_env_vars(&ac_help);

    binaries.insert(
        "aclocal".to_string(),
        BinaryProfile {
            path: abs_ac.to_string_lossy().to_string(),
            version_output: ac_version,
            sha256: ac_sha256,
            supported_flags: ac_flags,
            env_vars: ac_env_vars,
        },
    );

    // 3. Admit subordinate oracles
    let mut subordinates = HashMap::new();
    for sub_name in &["autoconf", "autoheader", "autom4te", "m4", "make"] {
        let explicit = config.binary_paths.get(*sub_name).and_then(|p| p.as_ref());
        if let Ok(sub_path) = locate_binary(sub_name, explicit) {
            let abs_sub = match std::fs::canonicalize(&sub_path) {
                Ok(p) => p,
                Err(_) => continue,
            };
            let sub_ver = run_oracle_text(&abs_sub, &["--version"], "", None, &config.env);
            if let Ok(run) = sub_ver {
                let sub_version = String::from_utf8_lossy(&run.stdout).to_string();
                let sub_hash = compute_sha256(&abs_sub).unwrap_or_default();
                subordinates.insert(
                    sub_name.to_string(),
                    SubordinateOracle {
                        name: sub_name.to_string(),
                        path: abs_sub.to_string_lossy().to_string(),
                        version_output: sub_version,
                        sha256: sub_hash,
                    },
                );
            }
        }
    }

    // 4. Detect features
    let features = OracleFeatures {
        multiple_output: help_output.contains("--add-missing"),
        warnings_categories,
        strictness_modes: vec![
            "foreign".to_string(),
            "gnu".to_string(),
            "gnits".to_string(),
        ],
        ..Default::default()
    };

    // 5. Run smoke test
    smoke_test_automake(&abs_am, config)?;

    // 6. Build profile
    let profile = OracleProfile {
        kind,
        platform,
        locale: config.locale.clone(),
        shell: config.shell.clone(),
        os_release,
        binaries,
        subordinate_oracles: subordinates,
        features,
        admitted_at: chrono_now(),
        receipt_registry: vec!["AM.ORACLE.1".to_string()],
    };

    Ok(profile)
}

/// Run a smoke test: create a minimal configure.ac + Makefile.am, run aclocal + automake,
/// and verify it produces a Makefile.in.
fn smoke_test_automake(am_path: &Path, config: &OracleConfig) -> Result<(), OracleError> {
    use std::fs;

    let tmp = tempdir().map_err(|e| OracleError::Io(e.to_string()))?;
    let tmp_path = PathBuf::from(&tmp);

    // Minimal configure.ac — must be valid for automake 1.16+
    let configure_ac = concat!(
        "AC_INIT([smoke-test],[1.0])\n",
        "AM_INIT_AUTOMAKE([foreign subdir-objects])\n",
        "AC_CONFIG_FILES([Makefile])\n",
        "AC_OUTPUT\n"
    );
    fs::write(tmp_path.join("configure.ac"), configure_ac)
        .map_err(|e| OracleError::Io(e.to_string()))?;

    // Minimal Makefile.am (no programs — just an empty file test)
    let makefile_am = "# Smoke test Makefile.am\n";
    fs::write(tmp_path.join("Makefile.am"), makefile_am)
        .map_err(|e| OracleError::Io(e.to_string()))?;

    // Step 1: Run aclocal to generate aclocal.m4
    let aclocal_path = locate_binary("aclocal", None)
        .map_err(|e| OracleError::Execution(format!("aclocal not found: {}", e)))?;
    let aclocal_run = run_oracle_text(
        &aclocal_path,
        &["--verbose"],
        "",
        Some(&tmp_path),
        &config.env,
    )
    .map_err(|e| OracleError::Execution(format!("aclocal: {}", e)))?;
    if !aclocal_run.exit_status.success() {
        return Err(OracleError::SmokeFailure(format!(
            "aclocal smoke test failed (exit {:?}):\nstderr: {}",
            aclocal_run.exit_code,
            String::from_utf8_lossy(&aclocal_run.stderr)
        )));
    }

    // Step 2: Run autoconf to generate configure
    let autoconf_path = locate_binary("autoconf", None)
        .map_err(|e| OracleError::Execution(format!("autoconf not found: {}", e)))?;
    let autoconf_run = run_oracle_text(&autoconf_path, &[], "", Some(&tmp_path), &config.env)
        .map_err(|e| OracleError::Execution(format!("autoconf: {}", e)))?;
    if !autoconf_run.exit_status.success() {
        return Err(OracleError::SmokeFailure(format!(
            "autoconf smoke test failed (exit {:?}):\nstderr: {}",
            autoconf_run.exit_code,
            String::from_utf8_lossy(&autoconf_run.stderr)
        )));
    }

    // Step 3: Run automake
    let run = run_oracle_text(
        am_path,
        &["--foreign", "--add-missing"],
        "",
        Some(&tmp_path),
        &config.env,
    )
    .map_err(|e| OracleError::Execution(format!("automake smoke test: {}", e)))?;

    if !run.exit_status.success() {
        return Err(OracleError::SmokeFailure(format!(
            "automake smoke test failed (exit {:?}):\nstderr: {}",
            run.exit_code,
            String::from_utf8_lossy(&run.stderr)
        )));
    }

    // Verify Makefile.in was produced
    if !tmp_path.join("Makefile.in").exists() {
        return Err(OracleError::SmokeFailure(
            "automake ran but no Makefile.in was generated".to_string(),
        ));
    }

    Ok(())
}

fn tempdir() -> Result<String, io::Error> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let dir = format!("/tmp/am-oracle-smoke-{}", ts);
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Save the oracle profile to a JSON file.
pub fn save_profile(profile: &OracleProfile, path: &Path) -> io::Result<()> {
    let json = serde_json::to_string_pretty(profile)?;
    std::fs::write(path, json)
}

/// Load an oracle profile from a JSON file.
pub fn load_profile(path: &Path) -> io::Result<OracleProfile> {
    let json = std::fs::read_to_string(path)?;
    let profile: OracleProfile = serde_json::from_str(&json)?;
    Ok(profile)
}

/// Errors that can occur during oracle operations.
#[derive(Debug)]
pub enum OracleError {
    NotFound(String),
    NotGnu(String),
    Execution(String),
    SmokeFailure(String),
    Io(String),
}

impl std::fmt::Display for OracleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OracleError::NotFound(s) => write!(f, "oracle not found: {}", s),
            OracleError::NotGnu(s) => write!(f, "not GNU Automake: {}", s),
            OracleError::Execution(s) => write!(f, "execution error: {}", s),
            OracleError::SmokeFailure(s) => write!(f, "smoke test failed: {}", s),
            OracleError::Io(s) => write!(f, "I/O error: {}", s),
        }
    }
}

impl std::error::Error for OracleError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_profile_kind() {
        assert_eq!(
            extract_profile_kind("automake (GNU automake) 1.17\n"),
            "gnu_automake_1_17_default"
        );
        assert_eq!(
            extract_profile_kind("GNU Automake 1.16.5\n"),
            "gnu_automake_1_16_5_default"
        );
    }

    #[test]
    fn test_locate_automake() {
        let _config = OracleConfig::default();
        match locate_binary("automake", None) {
            Ok(path) => {
                eprintln!("Found automake at: {}", path.display());
                assert!(path.exists());
            }
            Err(OracleError::NotFound(_)) => {
                eprintln!(
                    "automake not found — skipping locate test (CI/container may not have it)"
                );
            }
            Err(e) => panic!("unexpected error: {}", e),
        }
    }

    #[test]
    fn test_admit_oracle() {
        let config = OracleConfig::default();
        match admit_oracle(&config) {
            Ok(profile) => {
                eprintln!("Admitted oracle: {:?}", profile.kind);
                eprintln!("  platform: {}", profile.platform);
                for (name, bp) in &profile.binaries {
                    eprintln!("  {}: {} (sha256: {})", name, bp.path, &bp.sha256[..16]);
                }
                for (name, sub) in &profile.subordinate_oracles {
                    eprintln!("  subordinate {}: {}", name, sub.path);
                }
            }
            Err(OracleError::NotFound(_)) => {
                eprintln!("automake not found — skipping admission test");
            }
            Err(e) => panic!("admission failed: {}", e),
        }
    }
}
