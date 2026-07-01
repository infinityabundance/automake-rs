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
        // We want automake's OWN macro dir (defines AM_INIT_AUTOMAKE etc. in init.m4). On some systems
        // `aclocal --print-ac-dir` reports /usr/share/aclocal (the third-party dir, no automake macros),
        // so only trust it when it actually contains init.m4.
        if let Ok(out) = std::process::Command::new("aclocal")
            .arg("--print-ac-dir")
            .output()
        {
            let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !path.is_empty() && Path::new(&path).join("init.m4").exists() {
                return PathBuf::from(path);
            }
        }
        // Otherwise pick the newest /usr/share/aclocal-1.* that carries automake's macros.
        if let Ok(entries) = fs::read_dir("/usr/share") {
            let mut cands: Vec<PathBuf> = entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| {
                    p.file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| n.starts_with("aclocal-1."))
                        .unwrap_or(false)
                        && p.join("init.m4").exists()
                })
                .collect();
            cands.sort();
            if let Some(p) = cands.pop() {
                return p;
            }
        }
        // Last-resort fallback.
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

        // Build a macro-name -> defining-.m4-file index over all candidate dirs, then include ONLY the
        // files whose macros are (transitively) required by configure.ac. GNU aclocal is trace-driven:
        // it never dumps every .m4 in the acdir. The old "slurp every .m4" behavior pulled libtool.m4 /
        // ltdl.m4 into non-libtool projects, and m4-rs then ran away expanding LTDL_INIT/_LTDL_CONVENIENCE
        // (a 6-line configure.ac -> a 113 MB configure). Required-only inclusion is both faithful and the
        // fix for that explosion.
        // Index each macro DEFINITION to its file AND its body text. The closure below follows uses per
        // MACRO BODY, not per whole file: grab-bag files like lt~obsolete.m4 define a commonly-needed
        // macro (AC_PROG_EGREP) alongside dozens of obsolete libtool aliases. Pulling the file for
        // AC_PROG_EGREP must NOT then drag in libtool.m4 just because other (unused) definitions in the
        // same file mention LT_INIT. Following only AC_PROG_EGREP's own body keeps the closure tight.
        let mut macro_to_file: HashMap<String, PathBuf> = HashMap::new();
        let mut macro_body: HashMap<String, String> = HashMap::new();
        // Earlier dirs win (user -I, then macro_dirs, then automake acdir, then system acdir).
        for dir in &all_include_dirs {
            if let Ok(entries) = fs::read_dir(dir) {
                let mut files: Vec<PathBuf> = entries
                    .flatten()
                    .map(|e| e.path())
                    .filter(|p| p.extension().map(|e| e == "m4").unwrap_or(false))
                    .collect();
                files.sort(); // deterministic order within a dir
                for path in files {
                    let content = match fs::read(&path) {
                        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
                        Err(_) => continue,
                    };
                    for (name, body) in Self::extract_macro_defs(&content) {
                        if !macro_to_file.contains_key(&name) {
                            macro_to_file.insert(name.clone(), path.clone());
                            macro_body.insert(name, body);
                        }
                    }
                }
            }
        }

        // Transitive closure over macro NAMES: seed from macros invoked in configure.ac, follow only the
        // macros each required macro's own body invokes. Include the file of every required macro.
        let mut required_paths: Vec<PathBuf> = Vec::new();
        let mut included: HashSet<PathBuf> = HashSet::new();
        let mut worklist: Vec<String> = Self::extract_used_macros(&content, &macro_to_file);
        let mut resolved: HashSet<String> = HashSet::new();
        while let Some(name) = worklist.pop() {
            if !resolved.insert(name.clone()) {
                continue;
            }
            if let Some(path) = macro_to_file.get(&name) {
                if included.insert(path.clone()) {
                    required_paths.push(path.clone());
                }
                if let Some(body) = macro_body.get(&name) {
                    for dep in Self::extract_used_macros(body, &macro_to_file) {
                        if !resolved.contains(&dep) {
                            worklist.push(dep);
                        }
                    }
                }
            }
        }
        scan.required_files = required_paths;

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

    /// (macro_name, body_text) for each definition in an .m4 file: AC_DEFUN / AC_DEFUN_ONCE / AU_DEFUN /
    /// AU_ALIAS / m4_define[_default] / m4_defun[_once] / define. The body is the definition's argument(s)
    /// after the name, used to follow per-macro dependencies (so the closure stays tight — see scan()).
    fn extract_macro_defs(content: &str) -> Vec<(String, String)> {
        // Longer names first so AC_DEFUN_ONCE is tried before AC_DEFUN (boundary check also disambiguates).
        const DEFINERS: &[&str] = &[
            "AC_DEFUN_ONCE",
            "AC_DEFUN",
            "AU_DEFUN",
            "AU_ALIAS",
            "m4_define_default",
            "m4_defun_once",
            "m4_defun",
            "m4_define",
            "define",
        ];
        let bytes = content.as_bytes();
        let mut defs = Vec::new();
        for def in DEFINERS {
            let mut from = 0;
            while let Some(rel) = content[from..].find(def) {
                let start = from + rel;
                let after = start + def.len();
                from = after;
                // token boundary before the definer (not the tail of a longer identifier)
                if start > 0 {
                    let prev = bytes[start - 1];
                    if prev.is_ascii_alphanumeric() || prev == b'_' {
                        continue;
                    }
                }
                // `(` after optional whitespace
                let mut j = after;
                while j < bytes.len() && (bytes[j] as char).is_whitespace() {
                    j += 1;
                }
                if j >= bytes.len() || bytes[j] != b'(' {
                    continue;
                }
                let Some(args) = Self::parse_call_args(content, j) else {
                    continue;
                };
                if args.is_empty() {
                    continue;
                }
                let name: String = args[0]
                    .trim()
                    .trim_start_matches('[')
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .collect();
                // Only register macro-SHAPED names. libtool.m4 & friends use bare single letters ("A")
                // and short lowercase words as internal define()s; if those became index keys, an
                // unrelated body that merely contains the word "A" would drag the whole defining file
                // (and its real macro subtree) into the closure. Real macro names are >=2 chars and carry
                // an underscore or an uppercase letter (AC_*, AM_*, _LT_*, lt_join, m4_foreach, PKG_*...).
                if !Self::is_macro_shaped(&name) {
                    continue;
                }
                let body = if args.len() > 1 { args[1..].join(",") } else { String::new() };
                defs.push((name, body));
            }
        }
        defs
    }

    /// Parse a balanced `( ... )` call starting at byte index `open` (which must be `(`), splitting into
    /// top-level comma-separated arguments. m4 `[ ]` quotes and nested `( )` are respected. Returns None
    /// if the parens are unbalanced (truncated input).
    fn parse_call_args(content: &str, open: usize) -> Option<Vec<String>> {
        let bytes = content.as_bytes();
        let mut i = open + 1;
        let mut paren: i32 = 1;
        let mut brack: i32 = 0;
        let mut args: Vec<String> = Vec::new();
        let mut cur = String::new();
        while i < bytes.len() {
            let c = bytes[i] as char;
            match c {
                '[' => {
                    brack += 1;
                    cur.push(c);
                }
                ']' => {
                    brack -= 1;
                    cur.push(c);
                }
                '(' if brack == 0 => {
                    paren += 1;
                    cur.push(c);
                }
                ')' if brack == 0 => {
                    paren -= 1;
                    if paren == 0 {
                        args.push(cur);
                        return Some(args);
                    }
                    cur.push(c);
                }
                ',' if brack == 0 && paren == 1 => {
                    args.push(std::mem::take(&mut cur));
                }
                _ => cur.push(c),
            }
            i += 1;
        }
        None
    }

    /// Macro names USED in a text that are present in the known macro->file index. Identifiers are
    /// `[A-Za-z_][A-Za-z0-9_]*`; only those defined by some candidate .m4 file are returned (plain shell
    /// words and autoconf built-ins are ignored). `dnl` comments are stripped to end-of-line first, so a
    /// macro merely mentioned in a comment does not pull its defining file into the closure.
    fn extract_used_macros(content: &str, known: &HashMap<String, PathBuf>) -> Vec<String> {
        let mut out = Vec::new();
        for raw_line in content.lines() {
            // strip from a `dnl` token (m4's to-end-of-line comment) onward
            let line = Self::strip_dnl(raw_line);
            let bytes = line.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                let c = bytes[i];
                if c.is_ascii_alphabetic() || c == b'_' {
                    let start = i;
                    while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                        i += 1;
                    }
                    let ident = &line[start..i];
                    if known.contains_key(ident) {
                        out.push(ident.to_string());
                    }
                } else {
                    i += 1;
                }
            }
        }
        out
    }

    /// A macro-shaped identifier: >= 2 chars and carries an underscore or an uppercase letter. Excludes
    /// bare single letters and short all-lowercase shell words that .m4 files use as throwaway define()s.
    fn is_macro_shaped(id: &str) -> bool {
        id.len() >= 2 && (id.contains('_') || id.bytes().any(|b| b.is_ascii_uppercase()))
    }

    /// Truncate a line at the first `dnl` token (m4 discard-to-newline comment).
    fn strip_dnl(line: &str) -> &str {
        let bytes = line.as_bytes();
        let mut from = 0;
        while let Some(rel) = line[from..].find("dnl") {
            let pos = from + rel;
            let before_ok = pos == 0 || !(bytes[pos - 1].is_ascii_alphanumeric() || bytes[pos - 1] == b'_');
            let after = pos + 3;
            let after_ok = after >= bytes.len() || !(bytes[after].is_ascii_alphanumeric() || bytes[after] == b'_');
            if before_ok && after_ok {
                return &line[..pos];
            }
            from = pos + 3;
        }
        line
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
            // Must actually INVOKE TEST_MACRO: aclocal (correctly) only pulls/installs .m4 files whose
            // macros are used by configure.ac, not every file in the search path.
            "AC_INIT([test], [1.0])\nAC_CONFIG_MACRO_DIR([m4])\nTEST_MACRO\nAC_OUTPUT\n",
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
            "AC_INIT([test], [1.0])\nAC_CONFIG_MACRO_DIR([m4])\nOTHER_MACRO\nAC_OUTPUT\n",
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

    #[test]
    fn test_extract_macro_defs_name_and_body() {
        let m4 = "AC_DEFUN([FOO_BAR], [call BAZ_QUX and stuff])\n\
                  m4_define([lt_join], [something])\n";
        let defs = Aclocal::extract_macro_defs(m4);
        assert!(defs.iter().any(|(n, b)| n == "FOO_BAR" && b.contains("BAZ_QUX")));
        assert!(defs.iter().any(|(n, _)| n == "lt_join"));
    }

    #[test]
    fn test_is_macro_shaped_excludes_bare_letters() {
        // The libtool "A" bug: a bare single letter must NOT count as a macro name.
        assert!(!Aclocal::is_macro_shaped("A"));
        assert!(!Aclocal::is_macro_shaped("a"));
        assert!(!Aclocal::is_macro_shaped("cc")); // short all-lowercase, no underscore
        assert!(Aclocal::is_macro_shaped("AC_PROG_CC"));
        assert!(Aclocal::is_macro_shaped("_LT_LANG"));
        assert!(Aclocal::is_macro_shaped("lt_join"));
        assert!(Aclocal::is_macro_shaped("PKG_CHECK_MODULES"));
    }

    #[test]
    fn test_extract_used_macros_strips_dnl_and_filters() {
        let mut known: HashMap<String, PathBuf> = HashMap::new();
        known.insert("AC_PROG_CC".to_string(), PathBuf::from("x.m4"));
        known.insert("LT_INIT".to_string(), PathBuf::from("libtool.m4"));
        // LT_INIT appears only after a `dnl` comment -> must be ignored (not pulled).
        let body = "AC_PROG_CC dnl see LT_INIT for details\n";
        let used = Aclocal::extract_used_macros(body, &known);
        assert!(used.contains(&"AC_PROG_CC".to_string()));
        assert!(
            !used.contains(&"LT_INIT".to_string()),
            "macro mentioned only in a dnl comment must not be treated as used"
        );
    }
}
