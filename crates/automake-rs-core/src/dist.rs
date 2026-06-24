// automake-rs-core: dist/distcheck rule generation
//
// Court: AM.RULES.DIST.1
//
// Handles EXTRA_DIST accumulation, DISTFILES enumeration, distdir
// recipe generation, and dist/dist-all targets.
//
// Clean-room references:
//   - GNU Automake manual §14 (GFDL)
//   - Black-box oracle interrogation

/// Dist file lists and behavior.
#[derive(Debug, Clone)]
pub struct DistConfig {
    /// Extra files to distribute (from EXTRA_DIST variable)
    pub extra_dist: Vec<String>,
    /// Subdirectories for distribution (from DIST_SUBDIRS)
    pub dist_subdirs: Vec<String>,
    /// Optional dist hook
    pub dist_hook: Option<String>,
    /// Optional distcheck hook
    pub distcheck_hook: Option<String>,
}

impl DistConfig {
    pub fn new() -> Self {
        Self {
            extra_dist: vec![],
            dist_subdirs: vec![],
            dist_hook: None,
            distcheck_hook: None,
        }
    }
}

impl Default for DistConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Standard files that GNU Automake automatically distributes if found.
pub const STANDARD_DIST_FILES: &[&str] = &[
    // Always distributed if found
    "ABOUT-GNU",
    "ABOUT-NLS",
    "BACKLOG",
    "COPYING",
    "COPYING.DOC",
    "COPYING.LESSER",
    "COPYING.LIB",
    "TODO",
    "ar-lib",
    "compile",
    "config.guess",
    "config.rpath",
    "config.sub",
    "depcomp",
    "install-sh",
    "libversion.in",
    "ltcf-c.sh",
    "ltcf-cxx.sh",
    "ltcf-gcj.sh",
    "ltconfig",
    "ltmain.sh",
    "mdate-sh",
    "missing",
    "mkinstalldirs",
    "py-compile",
    "texinfo.tex",
    "ylwrap",
    // .md variants
    "AUTHORS",
    "ChangeLog",
    "INSTALL",
    "NEWS",
    "README",
    "THANKS",
    // Conditional
    "README-alpha",
    "config.h.bot",
    "config.h.top",
    "configure",
    "configure.ac",
    "configure.in",
    "acconfig.h",
    "aclocal.m4",
    "stamp-vti",
];

/// Files that are always distributed regardless of existence.
pub const ALWAYS_DIST_FILES: &[&str] = &["Makefile.am", "Makefile.in"];

/// Build the DISTFILES list from EXTRA_DIST and standard files.
pub fn build_distfiles(extra_dist: &[String]) -> Vec<String> {
    let mut files: Vec<String> = vec![];

    // Always-distributed files
    for f in ALWAYS_DIST_FILES {
        files.push(f.to_string());
    }

    // Standard files that exist
    for f in STANDARD_DIST_FILES {
        let path = std::path::Path::new(f);
        let md_name = format!("{}.md", f);
        let md_path = std::path::Path::new(&md_name);
        if (path.exists() || md_path.exists()) && !files.contains(&f.to_string()) {
            files.push(f.to_string());
        }
    }

    // EXTRA_DIST
    for f in extra_dist {
        if !files.contains(f) {
            files.push(f.clone());
        }
    }

    files.sort();
    files
}

/// Generate the DISTFILES and EXTRA_DIST variable assignments.
pub fn generate_dist_variables(extra_dist: &[String]) -> String {
    let distfiles = build_distfiles(extra_dist);
    let mut out = String::new();

    if !extra_dist.is_empty() {
        out.push_str(&format!("EXTRA_DIST = {}\n", extra_dist.join(" ")));
    }
    out.push_str(&format!("DISTFILES = {}\n\n", distfiles.join(" ")));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_distfiles_includes_always() {
        let files = build_distfiles(&[]);
        assert!(files.contains(&"Makefile.am".to_string()));
        assert!(files.contains(&"Makefile.in".to_string()));
    }

    #[test]
    fn test_build_distfiles_includes_extra() {
        let files = build_distfiles(&["custom.txt".to_string()]);
        assert!(files.contains(&"custom.txt".to_string()));
    }

    #[test]
    fn test_generate_dist_variables() {
        let out = generate_dist_variables(&["extra.txt".to_string()]);
        assert!(out.contains("EXTRA_DIST = extra.txt"));
        assert!(out.contains("DISTFILES ="));
        assert!(out.contains("Makefile.am"));
    }
}
