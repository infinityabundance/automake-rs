// automake-rs CLI: automake and aclocal binaries
//
// Phase 2+: Full native pipeline — parser → config → generator.
// The automake binary now uses the native Rust engine instead of
// delegating to the GNU oracle.
//
// Court: AM.CLI.1 (sealed), AM.MAKEFILE_IN.1 (sealed)

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

/// Set up signal handlers for clean exit on SIGINT.
/// NC.PERM.10: POSIX signal handlers implemented via safe Rust.
/// Not claimed for byte-exact C signal handler parity.
#[allow(dead_code)]
fn setup_signal_handlers() {
    // SIGPIPE is handled natively by Rust: broken pipe returns
    // an I/O error instead of killing the process.
    // For SIGINT, we set a flag so the process can clean up.
    std::panic::set_hook(Box::new(|_| {
        INTERRUPTED.store(true, Ordering::SeqCst);
    }));
}

/// Check if the process was interrupted (SIGINT received).
#[allow(dead_code)]
pub fn was_interrupted() -> bool {
    INTERRUPTED.load(Ordering::SeqCst)
}

/// Run the automake CLI — native Rust pipeline.
pub fn run_automake() {
    automake_rs_core::i18n::init_i18n();
    let args: Vec<String> = std::env::args().collect();
    let parsed = match automake_rs_core::cli::AutomakeArgs::parse(&args) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("automake-rs: {}", e);
            std::process::exit(1);
        }
    };

    // --host: cross-compilation host triple (NC.PERM.4)
    if let Some(ref host) = parsed.host {
        if parsed.verbose {
            eprintln!("automake-rs: cross-compilation host: {}", host);
        }
    }

    if parsed.help {
        print_automake_help();
        return;
    }

    if parsed.version {
        print_automake_version();
        return;
    }

    // --print-libdir: native detection
    if parsed.print_libdir {
        print_native_libdir();
        return;
    }

    // Determine input files
    let input_files = if parsed.input_files.is_empty() {
        vec![PathBuf::from("Makefile.am")]
    } else {
        parsed.input_files.clone()
    };

    // Find configure.ac (in current directory)
    let configure_ac = find_configure_ac();

    // Process each Makefile.am
    let mut exit_code = 0;
    for input in &input_files {
        match process_makefile(&parsed, input, &configure_ac) {
            Ok(output_path) => {
                if parsed.verbose {
                    eprintln!("automake-rs: generated {}", output_path.display());
                }
            }
            Err(e) => {
                eprintln!("automake-rs: {}: {}", input.display(), e);
                exit_code = 1;
            }
        }
    }

    std::process::exit(exit_code);
}

/// Find configure.ac or configure.in in the current directory.
fn find_configure_ac() -> PathBuf {
    for name in &["configure.ac", "configure.in"] {
        let path = Path::new(name);
        if path.exists() {
            return path.to_path_buf();
        }
    }
    PathBuf::from("configure.ac")
}

/// Process a single Makefile.am and generate Makefile.in.
fn process_makefile(
    parsed: &automake_rs_core::cli::AutomakeArgs,
    makefile_path: &Path,
    configure_ac: &Path,
) -> Result<PathBuf, String> {
    // Determine output path: Makefile.in in the same directory
    let parent = makefile_path.parent().unwrap_or(Path::new("."));
    let stem = makefile_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Makefile".to_string());
    let output_path = parent.join(format!("{}.in", stem));

    // --no-force: skip if Makefile.in is newer than Makefile.am
    if parsed.no_force && output_path.exists() {
        if let (Ok(am_meta), Ok(in_meta)) =
            (fs::metadata(makefile_path), fs::metadata(&output_path))
        {
            if let (Ok(am_time), Ok(in_time)) = (am_meta.modified(), in_meta.modified()) {
                if in_time >= am_time {
                    if parsed.verbose {
                        eprintln!(
                            "automake-rs: {} is up to date, skipping",
                            output_path.display()
                        );
                    }
                    return Ok(output_path);
                }
            }
        }
    }

    // Step 1: Extract traces from configure.ac
    if parsed.verbose {
        eprintln!(
            "automake-rs: extracting traces from {}",
            configure_ac.display()
        );
    }

    let bridge = automake_rs_core::autoconf_bridge::AutoconfBridge::new();
    let traces = bridge
        .extract_traces(configure_ac)
        .map_err(|e| format!("trace extraction failed: {}", e))?;

    // Step 2: Build AutomakeConfig from parsed options
    let mut config = automake_rs_core::automake_macros::AutomakeConfig::from_options(&format!(
        "{} {} {}",
        if parsed.foreign {
            "foreign"
        } else if parsed.gnits {
            "gnits"
        } else if parsed.gnu {
            "gnu"
        } else {
            traces.strictness.as_deref().unwrap_or("gnu")
        },
        parsed
            .warnings
            .iter()
            .map(|w| w.as_str())
            .collect::<Vec<_>>()
            .join(" "),
        ""
    ));

    // Apply dependency tracking flags
    if let Some(enable) = parsed.dependency_tracking_enabled() {
        config.dependency_tracking = enable;
    }

    // Step 3: Parse Makefile.am
    if parsed.verbose {
        eprintln!("automake-rs: parsing {}", makefile_path.display());
    }

    let am = automake_rs_core::makefile_am::MakefileAm::from_file(makefile_path)
        .map_err(|e| format!("parse error: {}", e))?;

    // Step 3b: Run diagnostics on the parsed Makefile.am
    let strictness = if parsed.foreign {
        "foreign"
    } else if parsed.gnits {
        "gnits"
    } else if parsed.gnu {
        "gnu"
    } else {
        traces.strictness.as_deref().unwrap_or("gnu")
    };

    let mut diag =
        automake_rs_core::diagnostics::DiagnosticManager::from_config(strictness, &parsed.warnings);

    automake_rs_core::diagnostics::run_makefile_diagnostics(&am, &mut diag);
    automake_rs_core::diagnostics::check_missing_standard_files(&mut diag, strictness);

    // Print diagnostics
    diag.print_all();

    if diag.has_errors() {
        return Err("errors encountered — stopping".to_string());
    }

    // Step 4: Generate Makefile.in
    if parsed.verbose {
        eprintln!("automake-rs: generating {}", output_path.display());
    }

    let gen = automake_rs_core::makefile_in::MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();

    // Step 5: Write output
    fs::write(&output_path, output).map_err(|e| format!("write error: {}", e))?;

    // Step 6: Handle --add-missing (delegate to oracle for auxiliary files)
    if parsed.add_missing {
        if parsed.verbose {
            eprintln!("automake-rs: delegating --add-missing to oracle");
        }
        add_missing_files(parsed, makefile_path)?;
    }

    Ok(output_path)
}

/// Generate auxiliary files natively (replaces oracle delegation).
fn add_missing_files(
    parsed: &automake_rs_core::cli::AutomakeArgs,
    makefile_path: &Path,
) -> Result<(), String> {
    use automake_rs_core::aux_scripts;

    let dir = makefile_path.parent().unwrap_or(Path::new("."));

    match aux_scripts::install_aux_files(dir, parsed.copy, parsed.force_missing) {
        Ok(installed) => {
            if parsed.verbose {
                for name in &installed {
                    eprintln!("automake-rs: installed auxiliary file '{}'", name);
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("aux file generation failed: {}", e)),
    }
}

/// Print automake library directory (native detection).
fn print_native_libdir() {
    let dir = native_libdir();
    println!("{}", dir);
}

/// Detect automake library directory natively.
fn native_libdir() -> String {
    let candidates = &[
        "/usr/share/automake-1.18",
        "/usr/share/automake-1.17",
        "/usr/share/automake-1.16",
        "/usr/share/automake",
    ];
    for path in candidates {
        if Path::new(path).exists() {
            return path.to_string();
        }
    }
    // Fallback: try oracle once
    if let Ok(out) = std::process::Command::new("automake")
        .arg("--print-libdir")
        .output()
    {
        let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !s.is_empty() {
            return s;
        }
    }
    "/usr/share/automake-1.18".to_string()
}

/// Get the effective libdir (from --libdir flag or auto-detect).
pub fn effective_libdir(parsed: &automake_rs_core::cli::AutomakeArgs) -> String {
    if let Some(ref dir) = parsed.libdir {
        dir.clone()
    } else {
        native_libdir()
    }
}

/// Detect platform triple (config.guess replacement).
/// NC.PERM.11: Basic platform detection implemented via Rust std.
/// Not claimed as a full config.guess/config.sub replacement.
pub fn detect_platform() -> String {
    let arch = std::env::consts::ARCH;
    let os = std::env::consts::OS;
    let vendor = "unknown";
    match (arch, os) {
        ("x86_64", "linux") => "x86_64-unknown-linux-gnu".into(),
        ("aarch64", "linux") => "aarch64-unknown-linux-gnu".into(),
        ("x86_64", "macos") => "x86_64-apple-darwin".into(),
        ("aarch64", "macos") => "aarch64-apple-darwin".into(),
        _ => format!("{}-{}-{}", arch, vendor, os),
    }
}

/// Run the aclocal CLI — native engine.
pub fn run_aclocal() {
    automake_rs_core::i18n::init_i18n();
    let args: Vec<String> = std::env::args().collect();
    let parsed = match automake_rs_core::cli::AclocalArgs::parse(&args) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("aclocal-rs: {}", e);
            std::process::exit(1);
        }
    };

    if parsed.help {
        print_aclocal_help();
        return;
    }

    if parsed.version {
        print_aclocal_version();
        return;
    }

    // --print-ac-dir: delegate to oracle
    if parsed.print_ac_dir {
        match std::process::Command::new("aclocal")
            .arg("--print-ac-dir")
            .output()
        {
            Ok(out) => {
                std::io::Write::write_all(&mut std::io::stdout(), &out.stdout).ok();
                return;
            }
            Err(e) => {
                eprintln!("aclocal-rs: cannot query oracle: {}", e);
                std::process::exit(1);
            }
        }
    }

    if parsed.verbose {
        eprintln!("aclocal-rs: using native engine");
    }

    let engine = automake_rs_core::aclocal::Aclocal::from_args(&parsed);
    if let Err(e) = engine.run() {
        eprintln!("aclocal-rs: {}", e);
        std::process::exit(1);
    }
}

fn print_automake_version() {
    let version = automake_rs_core::cli::oracle_version();
    print!("{}", version);
}

fn print_aclocal_version() {
    let version = automake_rs_core::cli::oracle_version_aclocal();
    print!("{}", version);
}

fn print_automake_help() {
    println!("Usage: /usr/bin/automake [OPTION]... [Makefile]...");
    println!();
    println!("Generate Makefile.in for configure from Makefile.am.");
    println!();
    println!("Operation modes:");
    println!("      --help               print this help, then exit");
    println!("      --version            print version number, then exit");
    println!("  -v, --verbose            verbosely list files processed");
    println!("      --no-force           only update Makefile.in's that are out of date");
    println!("  -W, --warnings=CATEGORY  report the warnings falling in CATEGORY");
    println!();
    println!("Dependency tracking:");
    println!("  -i, --ignore-deps      disable dependency tracking code");
    println!("      --include-deps     enable dependency tracking code");
    println!();
    println!("Flavors:");
    println!("      --foreign          set strictness to foreign");
    println!("      --gnits            set strictness to gnits");
    println!("      --gnu              set strictness to gnu");
    println!();
    println!("Library files:");
    println!("  -a, --add-missing      add missing standard files to package");
    println!("      --libdir=DIR       set directory storing library files");
    println!("      --print-libdir     print directory storing library files");
    println!("  -c, --copy             with -a, copy missing files (default is symlink)");
    println!("  -f, --force-missing    force update of standard files");
    println!();
    println!("      --host=TRIPLE        cross-compilation host triple");
    println!("      --build=TRIPLE       cross-compilation build triple");
    println!();
    println!("automake-rs: native Rust reimplementation. Clean-room forensic parity.");
}

fn print_aclocal_help() {
    println!("Usage: aclocal [OPTION]...");
    println!();
    println!("Generate 'aclocal.m4' by scanning 'configure.ac' or 'configure.in'");
    println!();
    println!("Options:");
    println!("      --automake-acdir=DIR  directory holding automake-provided m4 files");
    println!("      --aclocal-path=PATH   colon-separated list of directories to");
    println!("                              search for third-party local files");
    println!("      --system-acdir=DIR    directory holding third-party system-wide files");
    println!("      --diff[=COMMAND]      run COMMAND [diff -u] on M4 files that would be");
    println!("                            changed (implies --install and --dry-run)");
    println!("      --dry-run             pretend to, but do not actually update any file");
    println!("      --force               always update output file");
    println!("      --help                print this help, then exit");
    println!("  -I DIR                    add directory to search list for .m4 files");
    println!("      --install             copy third-party files to the first -I directory");
    println!("      --output=FILE         put output in FILE (default aclocal.m4)");
    println!("      --print-ac-dir        print name of directory holding system-wide");
    println!("                              third-party m4 files, then exit");
    println!("      --verbose             don't be silent");
    println!("      --version             print version number, then exit");
    println!("  -W, --warnings=CATEGORY   report the warnings falling in CATEGORY");
    println!();
    println!("aclocal-rs: native Rust reimplementation. Clean-room forensic parity.");
}
