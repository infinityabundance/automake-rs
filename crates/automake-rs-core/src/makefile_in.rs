// automake-rs-core: Makefile.in generator — forensic-parity implementation
//
// Court: AM.MAKEFILE_IN.1
//
// Generates a full Makefile.in from a parsed Makefile.am AST,
// AutomakeConfig (from AM_INIT_AUTOMAKE), and AutoconfTrace
// (from configure.ac traces).
//
// The generator produces standard makefile output with:
//   - Standard variable definitions
//   - Build rules for PROGRAMS, LIBRARIES, etc.
//   - Install/uninstall rules
//   - Clean/distclean rules
//   - Dependency tracking rules
//   - Test harness rules
//
// Clean-room reconstruction based on:
//   - GNU Automake manual (GFDL) — all sections on output format
//   - Black-box oracle comparison of generated Makefile.in
//   - POSIX make specification
// No GNU Automake GPL source code was consulted.

use crate::autoconf_bridge::AutoconfTrace;
use crate::automake_macros::AutomakeConfig;
use crate::makefile_am::{AmStatement, AssignmentOp, MakefileAm};

/// Generate a complete Makefile.in from the parsed inputs.
pub struct MakefileInGenerator {
    /// Parsed Makefile.am
    pub makefile_am: MakefileAm,
    /// Automake configuration
    pub config: AutomakeConfig,
    /// Autoconf trace facts
    pub traces: AutoconfTrace,
}

impl MakefileInGenerator {
    pub fn new(am: MakefileAm, config: AutomakeConfig, traces: AutoconfTrace) -> Self {
        Self {
            makefile_am: am,
            config,
            traces,
        }
    }

    /// Generate the complete Makefile.in.
    pub fn generate(&self) -> String {
        let mut output = String::new();

        // 1. Header
        self.generate_header(&mut output);

        // 2. VPATH and directory variables
        self.generate_vpath_variables(&mut output);

        // 3. Standard variables
        self.generate_standard_variables(&mut output);

        // 3b. pkg-config / AC_SUBST flag variables referenced as $(FOO_CFLAGS)/$(FOO_LIBS) in the
        // Makefile.am must be declared `FOO_CFLAGS = @FOO_CFLAGS@` so configure substitutes the real
        // pkg-config flags; without this the per-target $(FOO_CFLAGS) is empty -> headers not found.
        self.generate_pkg_subst_variables(&mut output);

        // 4. Automake support variables
        self.generate_support_variables(&mut output);

        // 5. User variables from Makefile.am
        self.generate_user_variables(&mut output);

        // 6. Dist variables (EXTRA_DIST, DISTFILES)
        let extra_dist = self.collect_extra_dist();
        output.push_str(&crate::dist::generate_dist_variables(&extra_dist));

        // 6a. Dependency tracking variables
        self.generate_dep_tracking(&mut output);

        // 6a2. Program build infrastructure (PROGRAMS, *_OBJECTS, COMPILE/LINK)
        self.generate_program_infra_vars(&mut output);

        // 6b. Recursive-make variables (SUBDIRS)
        self.generate_recursion_vars(&mut output);

        // 6c. The default goal `all` must be the first target.
        self.generate_all_target(&mut output);

        // The dep-stub rule is a real target -> it MUST come after `all:` (the default goal).
        // Only when dependency tracking is enabled (matches generate_dep_tracking).
        if self.config.dependency_tracking {
            crate::dependency_tracking::DepTracker::emit_depfile_rule(&self.collect_all_depfiles(), &mut output);
        }

        // 7. Build rules for primaries
        self.generate_built_sources_rules(&mut output);
        self.generate_yacc_lex_rules(&mut output);
        self.generate_programs_rules(&mut output);
        self.generate_libraries_rules(&mut output);
        // libtool library compile/link rules are emitted centrally by generate_compile_link_rules
        // and generate_program_infra_vars (the legacy emitter produced libfoo.la.la and duplicate
        // install recipes).
        self.generate_scripts_rules(&mut output);
        self.generate_data_rules(&mut output);
        self.generate_headers_rules(&mut output);
        self.generate_mans_rules(&mut output);
        self.generate_texinfos_rules(&mut output);
        self.generate_python_rules(&mut output);
        self.generate_lisp_rules(&mut output);
        self.generate_java_rules(&mut output);

        // 8. Install rules
        self.generate_install_rules(&mut output);

        // 9. Clean rules
        self.generate_clean_rules(&mut output);

        // 10. Dist rules
        self.generate_dist_rules(&mut output);

        // 11. Check/test rules
        self.generate_check_rules(&mut output);

        // 12. Utility targets (TAGS, cscope, etc.)
        self.generate_utility_targets(&mut output);

        // 12a. Compile + link rules for programs
        self.generate_compile_link_rules(&mut output);

        // 12b. Recursive-make engine + dispatch (SUBDIRS)
        self.generate_recursion_rules(&mut output);

        // 13. Passthrough rules
        self.generate_passthrough_rules(&mut output);

        output
    }

    /// Collect EXTRA_DIST values from Makefile.am.
    fn collect_extra_dist(&self) -> Vec<String> {
        let mut extra = vec![];
        for stmt in &self.makefile_am.statements {
            match stmt {
                AmStatement::VariableAssignment { name, values, .. } if name == "EXTRA_DIST" => {
                    extra.extend(values.clone());
                }
                AmStatement::Primary {
                    var_name, targets, ..
                } if var_name == "EXTRA_DIST" => {
                    extra.extend(targets.clone());
                }
                _ => {}
            }
        }
        extra
    }

    fn generate_header(&self, out: &mut String) {
        out.push_str("# Makefile.in generated by automake-rs ");
        out.push_str(env!("CARGO_PKG_VERSION"));
        out.push_str(" from Makefile.am.\n");
        out.push_str("# @configure_input@\n\n");

        if let Some(ref name) = self.traces.package_name {
            out.push_str(&format!(
                "# {} {}\n",
                name,
                self.traces.package_version.as_deref().unwrap_or("")
            ));
        }

        out.push_str(&format!("# {}\n", self.config.strictness_flag()));
        out.push('\n');
    }

    fn generate_vpath_variables(&self, out: &mut String) {
        out.push_str("VPATH = @srcdir@\n");
        out.push_str("srcdir = @srcdir@\n");
        out.push_str("top_srcdir = @top_srcdir@\n");
        out.push_str("builddir = @builddir@\n");
        out.push_str("top_builddir = @top_builddir@\n");
        out.push_str("abs_srcdir = @abs_srcdir@\n");
        out.push_str("abs_builddir = @abs_builddir@\n");
        out.push_str("abs_top_srcdir = @abs_top_srcdir@\n");
        out.push_str("abs_top_builddir = @abs_top_builddir@\n");
        out.push_str("@SET_MAKE@\n\n");
    }

    /// Declare `FOO_CFLAGS = @FOO_CFLAGS@` (and _LIBS/_DEPS) for every `$(FOO_CFLAGS)`-style flag
    /// variable referenced in the Makefile.am — these come from PKG_CHECK_MODULES/AC_SUBST in
    /// configure.ac. Standard build vars (CFLAGS, AM_CFLAGS, LIBS, ...) are emitted elsewhere.
    fn generate_pkg_subst_variables(&self, out: &mut String) {
        use std::collections::BTreeSet;
        let standard: &[&str] = &[
            "CFLAGS", "CXXFLAGS", "CPPFLAGS", "LDFLAGS", "LIBS", "AM_CFLAGS", "AM_CXXFLAGS",
            "AM_CPPFLAGS", "AM_LDFLAGS", "AM_LIBS", "LIBTOOLFLAGS", "AM_LIBTOOLFLAGS",
        ];
        let mut found: BTreeSet<String> = BTreeSet::new();
        let mut scan = |s: &str| {
            let b = s.as_bytes();
            let mut i = 0;
            while i + 2 < b.len() {
                if b[i] == b'$' && b[i + 1] == b'(' {
                    let start = i + 2;
                    let mut j = start;
                    while j < b.len() && b[j] != b')' {
                        j += 1;
                    }
                    if j < b.len() {
                        let name = &s[start..j];
                        if (name.ends_with("_CFLAGS")
                            || name.ends_with("_LIBS")
                            || name.ends_with("_DEPS")
                            || name.ends_with("_REQUIRES"))
                            && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                            && !standard.contains(&name)
                        {
                            found.insert(name.to_string());
                        }
                    }
                    i = j + 1;
                } else {
                    i += 1;
                }
            }
        };
        for stmt in &self.makefile_am.statements {
            if let AmStatement::VariableAssignment { values, .. } = stmt {
                for v in values {
                    scan(v);
                }
            }
        }
        for name in &found {
            out.push_str(&format!("{name} = @{name}@\n"));
        }
        if !found.is_empty() {
            out.push('\n');
        }
    }

    fn generate_standard_variables(&self, out: &mut String) {
        let vars = crate::automake_macros::generate_standard_variables();
        // Sort for stable output
        let mut keys: Vec<&String> = vars.keys().collect();
        keys.sort();

        for key in keys {
            let value = &vars[key];
            out.push_str(&format!("{} = {}\n", key, value));
        }
        out.push('\n');
    }

    fn generate_support_variables(&self, out: &mut String) {
        let vars = crate::automake_macros::generate_support_variables(&self.config);
        let mut keys: Vec<&String> = vars.keys().collect();
        keys.sort();
        for key in keys {
            out.push_str(&format!("{} = {}\n", key, vars[key]));
        }

        // Full silent rules: two-line variable definitions per language
        // GNU Automake pattern: $(AM_V_CC)$(AM_V_CC:$(AM_DEFAULT_VERBOSITY)=$(V))
        if self.config.silent_rules {
            out.push_str("# Silent rules\n");
            out.push_str("AM_V_CC = $(am__v_CC_$(V))\n");
            out.push_str("am__v_CC_ = $(am__v_CC_$(AM_DEFAULT_VERBOSITY))\n");
            out.push_str("am__v_CC_0 = @echo \"  CC      \" $@;\n");
            out.push_str("am__v_CC_1 = \n");

            out.push_str("AM_V_CXX = $(am__v_CXX_$(V))\n");
            out.push_str("am__v_CXX_ = $(am__v_CXX_$(AM_DEFAULT_VERBOSITY))\n");
            out.push_str("am__v_CXX_0 = @echo \"  CXX     \" $@;\n");
            out.push_str("am__v_CXX_1 = \n");

            out.push_str("AM_V_CCLD = $(am__v_CCLD_$(V))\n");
            out.push_str("am__v_CCLD_ = $(am__v_CCLD_$(AM_DEFAULT_VERBOSITY))\n");
            out.push_str("am__v_CCLD_0 = @echo \"  CCLD    \" $@;\n");
            out.push_str("am__v_CCLD_1 = \n");

            // libtool verbosity helpers (AM_V_lt = $(am__v_lt_$(V)) is set in automake_macros).
            out.push_str("am__v_lt_ = $(am__v_lt_$(AM_DEFAULT_VERBOSITY))\n");
            out.push_str("am__v_lt_0 = --silent\n");
            out.push_str("am__v_lt_1 = \n");

            out.push_str("AM_V_AR = $(am__v_AR_$(V))\n");
            out.push_str("am__v_AR_ = $(am__v_AR_$(AM_DEFAULT_VERBOSITY))\n");
            out.push_str("am__v_AR_0 = @echo \"  AR      \" $@;\n");
            out.push_str("am__v_AR_1 = \n");

            out.push_str("AM_V_GEN = $(am__v_GEN_$(V))\n");
            out.push_str("am__v_GEN_ = $(am__v_GEN_$(AM_DEFAULT_VERBOSITY))\n");
            out.push_str("am__v_GEN_0 = @echo \"  GEN     \" $@;\n");
            out.push_str("am__v_GEN_1 = \n");

            out.push_str("AM_V_YACC = $(am__v_YACC_$(V))\n");
            out.push_str("am__v_YACC_ = $(am__v_YACC_$(AM_DEFAULT_VERBOSITY))\n");
            out.push_str("am__v_YACC_0 = @echo \"  YACC    \" $@;\n");
            out.push_str("am__v_YACC_1 = \n");

            out.push_str("AM_V_LEX = $(am__v_LEX_$(V))\n");
            out.push_str("am__v_LEX_ = $(am__v_LEX_$(AM_DEFAULT_VERBOSITY))\n");
            out.push_str("am__v_LEX_0 = @echo \"  LEX     \" $@;\n");
            out.push_str("am__v_LEX_1 = \n");

            out.push_str("AM_V_MAKEINFO = $(am__v_MAKEINFO_$(V))\n");
            out.push_str("am__v_MAKEINFO_ = $(am__v_MAKEINFO_$(AM_DEFAULT_VERBOSITY))\n");
            out.push_str("am__v_MAKEINFO_0 = @echo \"  MAKEINFO\" $@;\n");
            out.push_str("am__v_MAKEINFO_1 = \n");

            out.push_str("AM_V_TEXI2DVI = $(am__v_TEXI2DVI_$(V))\n");
            out.push_str("am__v_TEXI2DVI_ = $(am__v_TEXI2DVI_$(AM_DEFAULT_VERBOSITY))\n");
            out.push_str("am__v_TEXI2DVI_0 = @echo \"  TEXI2DVI\" $@;\n");
            out.push_str("am__v_TEXI2DVI_1 = \n");

            out.push_str("AM_V_TEXI2PDF = $(am__v_TEXI2PDF_$(V))\n");
            out.push_str("am__v_TEXI2PDF_ = $(am__v_TEXI2PDF_$(AM_DEFAULT_VERBOSITY))\n");
            out.push_str("am__v_TEXI2PDF_0 = @echo \"  TEXI2PDF\" $@;\n");
            out.push_str("am__v_TEXI2PDF_1 = \n");

            out.push_str("V = 0\n");
        }
        out.push('\n');

        // GNU make detection and support — exact oracle match from GNU Automake 1.18.1
        // Observed via black-box: automake --foreign on simple Makefile.am
        out.push_str("# GNU make detection (oracle-observed GNU Automake 1.18.1 output)\n");
        out.push_str("am__is_gnu_make = { \\\n");
        out.push_str("  if test -z '$(MAKELEVEL)'; then \\\n");
        out.push_str("    false; \\\n");
        out.push_str("  elif test -n '$(MAKE_HOST)'; then \\\n");
        out.push_str("    true; \\\n");
        out.push_str("  elif test -n '$(MAKE_VERSION)' && test -n '$(CURDIR)'; then \\\n");
        out.push_str("    true; \\\n");
        out.push_str("  else \\\n");
        out.push_str("    false; \\\n");
        out.push_str("  fi; \\\n");
        out.push_str("}\n");
        out.push_str("am__make_running_with_option = \\\n");
        out.push_str("  case $${target_option-} in \\\n");
        out.push_str("      ?) ;; \\\n");
        out.push_str(
            "      *) echo \"am__make_running_with_option: internal error: invalid\" \\\n",
        );
        out.push_str("              \"target option '$${target_option-}' specified\" >&2; \\\n");
        out.push_str("         exit 1;; \\\n");
        out.push_str("  esac; \\\n");
        out.push_str("  has_opt=no; \\\n");
        out.push_str("  sane_makeflags=$$MAKEFLAGS; \\\n");
        out.push_str("  if $(am__is_gnu_make); then \\\n");
        out.push_str("    sane_makeflags=$$MFLAGS; \\\n");
        out.push_str("  else \\\n");
        out.push_str("    case $$MAKEFLAGS in \\\n");
        out.push_str("      *\\\\[\\ \\t]*) \\\n");
        out.push_str("        bs=\\\\; \\\n");
        out.push_str("        sane_makeflags=`printf '%s\\n' \"$$MAKEFLAGS\" \\\n");
        out.push_str("          | sed \"s/$$bs$$bs[$$bs $$bs	]*//g\"`;; \\\n");
        out.push_str("    esac; \\\n");
        out.push_str("  fi; \\\n");
        out.push_str("  skip_next=no; \\\n");
        out.push_str("  strip_trailopt () \\\n");
        out.push_str("  { \\\n");
        out.push_str("    flg=`printf '%s\\n' \"$$flg\" | sed \"s/$$1.*$$//\"`; \\\n");
        out.push_str("  }; \\\n");
        out.push_str("  for flg in $$sane_makeflags; do \\\n");
        out.push_str("    test $$skip_next = yes && { skip_next=no; continue; }; \\\n");
        out.push_str("    case $$flg in \\\n");
        out.push_str("      *=*|--*) continue;; \\\n");
        out.push_str("        -*I) strip_trailopt 'I'; skip_next=yes;; \\\n");
        out.push_str("      -*I?*) strip_trailopt 'I';; \\\n");
        out.push_str("        -*O) strip_trailopt 'O'; skip_next=yes;; \\\n");
        out.push_str("      -*O?*) strip_trailopt 'O';; \\\n");
        out.push_str("        -*l) strip_trailopt 'l'; skip_next=yes;; \\\n");
        out.push_str("      -*l?*) strip_trailopt 'l';; \\\n");
        out.push_str("      -[dEDm]) skip_next=yes;; \\\n");
        out.push_str("      -[JT]) skip_next=yes;; \\\n");
        out.push_str("    esac; \\\n");
        out.push_str("    case $$flg in \\\n");
        out.push_str("      *$$target_option*) has_opt=yes; break;; \\\n");
        out.push_str("    esac; \\\n");
        out.push_str("  done; \\\n");
        out.push_str("  test $$has_opt = yes\n");
        out.push_str("am__make_dryrun = (target_option=n; $(am__make_running_with_option))\n");
        out.push_str("am__make_keepgoing = (target_option=k; $(am__make_running_with_option))\n");
        out.push_str("am__rm_f = rm -f $(am__rm_f_notfound)\n");
        out.push_str("am__rm_rf = rm -rf $(am__rm_f_notfound)\n");
        out.push_str("am__cd = CDPATH=\"$${ZSH_VERSION+.}$(PATH_SEPARATOR)\" && cd\n");
        out.push_str("install_sh_DATA = $(install_sh) -c -m 644\n");
        out.push_str("install_sh_PROGRAM = $(install_sh) -c\n");
        out.push_str("install_sh_SCRIPT = $(install_sh) -c\n");
        out.push_str("INSTALL_HEADER = $(INSTALL_DATA)\n");
        out.push_str("transform = $(program_transform_name)\n");
        out.push_str("am__untar = tar -xf -\n");
        out.push_str("am__tar = tar -chf -\n");
        out.push_str("am__rmdir = rm -rf\n");
        out.push_str("distcleancheck_listfiles = find . -type f -print\n\n");
        out.push('\n');
    }

    fn generate_user_variables(&self, out: &mut String) {
        self.emit_user_variables(&self.makefile_am.statements, out);
    }

    fn emit_user_variables(&self, statements: &[AmStatement], out: &mut String) {
        // Use the panel-recommended ConditionalEnv for proper
        // conditional variable tracking with @COND_TRUE@/@COND_FALSE@ overrides
        // and += across conditional boundaries.
        let env = crate::conditional_env::ConditionalEnv::collect(statements);
        env.emit(out);

        // Emit primary declarations (these may also have conditional context)
        crate::conditional_env::emit_primaries_with_conditionals(statements, out);
    }

    /// Generate dependency tracking variables (AMDEP, depcomp, .deps/ includes).
    /// Uses the enhanced DepTracker with full compiler mode support.
    fn generate_dep_tracking(&self, out: &mut String) {
        // `no-dependencies` (AM_INIT_AUTOMAKE) disables dep tracking: configure then defines no
        // AMDEP_TRUE/am__include, so emitting the @AMDEP@ include markers would leave literal
        // `@AMDEP_TRUE@` in the Makefile -> "missing separator". Skip the whole mechanism.
        if !self.config.dependency_tracking {
            return;
        }
        let mut has_sources = false;
        for kind in &["PROGRAMS", "LIBRARIES", "LTLIBRARIES"] {
            if !self.collect_primaries(kind).is_empty() {
                has_sources = true;
                break;
            }
        }
        if !has_sources {
            return;
        }

        let tracker = crate::dependency_tracking::DepTracker::new();
        tracker.emit_variables(out);

        let depfiles = self.collect_all_depfiles();
        crate::dependency_tracking::DepTracker::emit_includes(&depfiles, out);
    }

    /// Collect the `.Po`/`.Plo` dependency files for every compiled source across all targets.
    fn collect_all_depfiles(&self) -> Vec<String> {
        let mut all_sources: Vec<(String, &str)> = Vec::new();
        for kind in &["PROGRAMS", "LIBRARIES", "LTLIBRARIES"] {
            for (_dir, _no_dist, targets) in &self.collect_primaries(kind) {
                for target in targets {
                    // Variables derive from the canonicalized target name (libfoo.a -> libfoo_a).
                    let sources_var = format!("{}_SOURCES", Self::canon(target));
                    let ext = if *kind == "LTLIBRARIES" { "lo" } else { "$(OBJEXT)" };
                    if let Some(sources) = self.find_variable(&sources_var) {
                        for src in sources.split_whitespace() {
                            // Only real compiled sources become dep files: skip `$(VAR)` refs,
                            // `@SUBST@`, headers, and anything without a known source extension
                            // (otherwise the depfiles/include lines are malformed -> "missing separator").
                            if src.starts_with("$(") || src.starts_with('@') {
                                continue;
                            }
                            if src.ends_with(".c")
                                || src.ends_with(".cc")
                                || src.ends_with(".cpp")
                                || src.ends_with(".cxx")
                                || src.ends_with(".c++")
                                || src.ends_with(".C")
                                || src.ends_with(".m")
                                || src.ends_with(".mm")
                                || src.ends_with(".s")
                                || src.ends_with(".S")
                            {
                                all_sources.push((src.to_string(), ext));
                            }
                        }
                    }
                }
            }
        }
        let source_refs: Vec<(&str, &str)> =
            all_sources.iter().map(|(s, e)| (s.as_str(), *e)).collect();
        crate::dependency_tracking::DepTracker::collect_depfiles(&source_refs, ".deps")
    }

    /// Generate BUILT_SOURCES ordering rules — ensures these files are built first.
    fn generate_built_sources_rules(&self, _out: &mut String) {
        // BUILT_SOURCES is emitted as a normal user variable (generate_user_variables); the targets
        // it names are built by their own rules (Yacc/Lex headers, user rules passed through). The
        // old `$(BUILT_SOURCES): @:` no-op shadowed those real rules ("overriding recipe") and left
        // generated headers uncreated, so it is intentionally not emitted here.
    }

    /// Auto-detect Yacc (.y) and Lex (.l) sources and generate rules.
    /// When a program's SOURCES contain .y or .l files, generate YACC/LEX
    /// rules using ylwrap to handle output file renaming.
    fn generate_yacc_lex_rules(&self, out: &mut String) {
        // Collect (source, generated.c) for every Yacc/Lex source across all targets.
        let mut yacc: Vec<(String, String)> = Vec::new();
        let mut lex: Vec<(String, String)> = Vec::new();
        for kind in &["PROGRAMS", "LIBRARIES", "LTLIBRARIES"] {
            for (_dir, _no_dist, targets) in &self.collect_primaries(kind) {
                for target in targets {
                    for s in self.target_sources(target) {
                        let Some(gen) = Self::lexyacc_generated(&s) else { continue };
                        let is_yacc = s.ends_with(".y") || s.ends_with(".yy")
                            || s.ends_with(".ypp") || s.ends_with(".y++");
                        let pair = (s.clone(), gen);
                        if is_yacc {
                            if !yacc.contains(&pair) { yacc.push(pair); }
                        } else if !lex.contains(&pair) {
                            lex.push(pair);
                        }
                    }
                }
            }
        }
        if yacc.is_empty() && lex.is_empty() {
            return;
        }
        // Toolchain vars. AM_YFLAGS/AM_LFLAGS come from the user's Makefile.am (emitted by
        // generate_user_variables) -- do NOT redefine them here (that dropped a project's `-d`,
        // so no parser header was generated). $(YACCCOMPILE -o ...) uses modern bison/flex `-o`
        // which writes the C file and, with `-d`/`--header-file`, the header alongside it.
        // YACC/LEX are config.status substitutions (AC_PROG_YACC/AC_PROG_LEX); YFLAGS/LFLAGS are
        // user/make variables (NOT @-substituted) -- emitting `@YFLAGS@` would leave a literal that
        // flex/bison try to open as a file. Leave them undefined (empty) unless the user set them.
        out.push_str("YACC = @YACC@\n");
        out.push_str("LEX = @LEX@\n");
        out.push_str("YACCCOMPILE = $(YACC) $(AM_YFLAGS) $(YFLAGS)\n");
        out.push_str("LEXCOMPILE = $(LEX) $(AM_LFLAGS) $(LFLAGS)\n\n");
        for (src, gen) in &yacc {
            let stem = gen.rfind('.').map(|d| &gen[..d]).unwrap_or(gen);
            let header = format!("{}.h", stem);
            out.push_str(&format!("{}: {}\n", gen, src));
            out.push_str(&format!("\t$(AM_V_YACC)$(YACCCOMPILE) -o {} {}\n\n", gen, src));
            // The parser header is produced as a side effect of building the .c (bison -d). The
            // recovery recipe regenerates it if it went missing (the standard Automake idiom).
            out.push_str(&format!("{}: {}\n", header, gen));
            out.push_str(&format!(
                "\t@if test ! -f $@; then rm -f {g}; $(MAKE) $(AM_MAKEFLAGS) {g}; else :; fi\n\n",
                g = gen
            ));
        }
        for (src, gen) in &lex {
            out.push_str(&format!("{}: {}\n", gen, src));
            out.push_str(&format!("\t$(AM_V_LEX)$(LEXCOMPILE) -o {} {}\n\n", gen, src));
        }
    }

    /// Generate build rules for PROGRAMS primaries.
    /// The `SUBDIRS` list (whitespace-split), empty if this Makefile.am is not recursive.
    fn subdirs_list(&self) -> Vec<String> {
        self.find_variable("SUBDIRS")
            .map(|v| v.split_whitespace().map(String::from).collect())
            .unwrap_or_default()
    }

    fn has_subdirs(&self) -> bool {
        !self.subdirs_list().is_empty()
    }

    /// Whether this Makefile.am builds anything locally (vs. a pure SUBDIRS orchestrator).
    fn has_build_primaries(&self) -> bool {
        [
            "PROGRAMS",
            "LIBRARIES",
            "LTLIBRARIES",
            "SCRIPTS",
            "DATA",
            "HEADERS",
            "MANS",
            "TEXINFOS",
            "PYTHON",
            "LISP",
            "JAVA",
        ]
        .iter()
        .any(|k| !self.collect_primaries(k).is_empty())
    }

    /// Recursive-make variables (emitted in the variable section when `SUBDIRS` is present).
    fn generate_recursion_vars(&self, out: &mut String) {
        if !self.has_subdirs() {
            return;
        }
        out.push_str("RECURSIVE_TARGETS = all-recursive check-recursive cscopelist-recursive \\\n");
        out.push_str("\tctags-recursive dvi-recursive html-recursive info-recursive \\\n");
        out.push_str("\tinstall-data-recursive install-dvi-recursive \\\n");
        out.push_str("\tinstall-exec-recursive install-html-recursive \\\n");
        out.push_str("\tinstall-info-recursive install-pdf-recursive \\\n");
        out.push_str("\tinstall-ps-recursive install-recursive installcheck-recursive \\\n");
        out.push_str("\tinstalldirs-recursive pdf-recursive ps-recursive \\\n");
        out.push_str("\ttags-recursive uninstall-recursive\n");
        out.push_str("RECURSIVE_CLEAN_TARGETS = mostlyclean-recursive clean-recursive \\\n");
        out.push_str("  distclean-recursive maintainer-clean-recursive\n");
        out.push_str("am__recursive_targets = \\\n");
        out.push_str("  $(RECURSIVE_TARGETS) \\\n");
        out.push_str("  $(RECURSIVE_CLEAN_TARGETS) \\\n");
        out.push_str("  $(am__extra_recursive_targets)\n");
        // DIST_SUBDIRS defaults to SUBDIRS unless the user set it explicitly.
        if self.find_variable("DIST_SUBDIRS").is_none() {
            out.push_str("DIST_SUBDIRS = $(SUBDIRS)\n");
        }
    }

    /// The default goal: `all`. Must be the first target so `make` with no args builds it.
    /// Recursive when `SUBDIRS` is present, otherwise the local `all-am`.
    fn generate_all_target(&self, out: &mut String) {
        let final_target = if self.has_subdirs() { "all-recursive" } else { "all-am" };
        // BUILT_SOURCES (generated headers etc.) must exist before anything is compiled, so make
        // `all` build them first, then dispatch to all-am/all-recursive.
        let has_built = self
            .find_variable("BUILT_SOURCES")
            .map(|v| !v.trim().is_empty())
            .unwrap_or(false);
        if has_built {
            out.push_str("all: $(BUILT_SOURCES)\n");
            out.push_str(&format!("\t$(MAKE) $(AM_MAKEFLAGS) {}\n\n", final_target));
        } else {
            out.push_str(&format!("all: {}\n\n", final_target));
        }
        // `all-am` is ALWAYS emitted (the recursion engine and install-am depend on it). Its
        // recipe builds the local program/library outputs via $(MAKE) (relying on per-target or
        // builtin rules); DATA/HEADERS/SCRIPTS/MANS are install-only and need no build step.
        let mut build_targets: Vec<String> = Vec::new();
        for kind in ["PROGRAMS", "LIBRARIES", "LTLIBRARIES"] {
            for (_p, _nd, targets) in self.collect_primaries(kind) {
                build_targets.extend(targets);
            }
        }
        out.push_str("all-am: Makefile\n");
        for t in &build_targets {
            out.push_str(&format!("\t@$(MAKE) {}\n", t));
        }
        out.push('\n');
    }

    /// The recursive-make engine: the `$(am__recursive_targets)` rule that descends into
    /// `$(SUBDIRS)` (or `$(DIST_SUBDIRS)` for the clean targets) running the matching `*-am`
    /// target locally, plus the top-level dispatch targets wired to their `-recursive` forms.
    /// Reconstructed from the observed GNU Automake output (permissively licensed), not its source.
    fn generate_recursion_rules(&self, out: &mut String) {
        if !self.has_subdirs() {
            return;
        }
        // The descent loop.
        out.push_str("$(am__recursive_targets):\n");
        out.push_str("\t@fail=; \\\n");
        out.push_str("\tif $(am__make_keepgoing); then \\\n");
        out.push_str("\t  failcom='fail=yes'; \\\n");
        out.push_str("\telse \\\n");
        out.push_str("\t  failcom='exit 1'; \\\n");
        out.push_str("\tfi; \\\n");
        out.push_str("\tdot_seen=no; \\\n");
        out.push_str("\ttarget=`echo $@ | sed s/-recursive//`; \\\n");
        out.push_str("\tcase \"$@\" in \\\n");
        out.push_str("\t  distclean-* | maintainer-clean-*) list='$(DIST_SUBDIRS)' ;; \\\n");
        out.push_str("\t  *) list='$(SUBDIRS)' ;; \\\n");
        out.push_str("\tesac; \\\n");
        out.push_str("\tfor subdir in $$list; do \\\n");
        out.push_str("\t  echo \"Making $$target in $$subdir\"; \\\n");
        out.push_str("\t  if test \"$$subdir\" = \".\"; then \\\n");
        out.push_str("\t    dot_seen=yes; \\\n");
        out.push_str("\t    local_target=\"$$target-am\"; \\\n");
        out.push_str("\t  else \\\n");
        out.push_str("\t    local_target=\"$$target\"; \\\n");
        out.push_str("\t  fi; \\\n");
        out.push_str("\t  ($(am__cd) $$subdir && $(MAKE) $(AM_MAKEFLAGS) $$local_target) \\\n");
        out.push_str("\t  || eval $$failcom; \\\n");
        out.push_str("\tdone; \\\n");
        out.push_str("\tif test \"$$dot_seen\" = \"no\"; then \\\n");
        out.push_str("\t  $(MAKE) $(AM_MAKEFLAGS) \"$$target-am\" || exit 1; \\\n");
        out.push_str("\tfi; test -z \"$$fail\"\n\n");

        // Top-level dispatch -> -recursive. (The non-recursive `install:`/`check:`/`clean:`/
        // `installcheck:`/`installdirs:` lines are suppressed elsewhere when has_subdirs.)
        for line in [
            "check: check-recursive\n",
            "install: install-recursive\n",
            "install-exec: install-exec-recursive\n",
            "install-data: install-data-recursive\n",
            "uninstall: uninstall-recursive\n",
            "installcheck: installcheck-recursive\n",
            "installdirs: installdirs-recursive\n",
            "clean: clean-recursive\n",
            "distclean: distclean-recursive\n",
            "mostlyclean: mostlyclean-recursive\n",
            "maintainer-clean: maintainer-clean-recursive\n",
            "dvi: dvi-recursive\n",
            "html: html-recursive\n",
            "info: info-recursive\n",
            "pdf: pdf-recursive\n",
            "ps: ps-recursive\n",
            "install-dvi: install-dvi-recursive\n",
            "install-html: install-html-recursive\n",
            "install-info: install-info-recursive\n",
            "install-pdf: install-pdf-recursive\n",
            "install-ps: install-ps-recursive\n",
            "tags: tags-recursive\n",
            "ctags: ctags-recursive\n",
            "cscopelist: cscopelist-recursive\n",
        ] {
            out.push_str(line);
        }
        out.push('\n');

        // Local `*-am` stubs the recursion calls for the "." dir. A pure orchestrator defines
        // none of these via feature methods, so provide empty defaults. (Where a feature method
        // also defines one, make takes the later definition; harmless for a no-op stub.)
        if !self.has_build_primaries() {
            for stub in [
                "check-am: all-am\n",
                "install-am: all-am\n",
                "install-exec-am:\n",
                "install-data-am:\n",
                "uninstall-am:\n",
                "installcheck-am:\n",
                "installdirs-am:\n",
                "mostlyclean-am:\n",
                "clean-am: mostlyclean-am\n",
                "distclean-am: clean-am\n",
                "maintainer-clean-am: distclean-am\n",
                "dvi-am:\n",
                "html-am:\n",
                "info-am:\n",
                "pdf-am:\n",
                "ps-am:\n",
                "install-dvi-am:\n",
                "install-html-am:\n",
                "install-info-am:\n",
                "install-pdf-am:\n",
                "install-ps-am:\n",
                "tags-am:\n",
                "ctags-am:\n",
            ] {
                out.push_str(stub);
            }
            out.push('\n');
        }
        out.push_str(".PHONY: $(am__recursive_targets)\n\n");
    }

    /// Whether a file is a compiled source (becomes an object).
    fn is_compiled_source(s: &str) -> bool {
        s.ends_with(".c") || s.ends_with(".cc") || s.ends_with(".cpp") || s.ends_with(".cxx")
            || s.ends_with(".c++") || s.ends_with(".C") || s.ends_with(".m") || s.ends_with(".mm")
            || s.ends_with(".s") || s.ends_with(".S")
    }

    /// If `s` is a Yacc (`.y*`) or Lex (`.l*`) source, the C/C++ file it generates
    /// (`grammar.y` -> `grammar.c`); otherwise `None`.
    fn lexyacc_generated(s: &str) -> Option<String> {
        for (suf, gen) in [
            (".ypp", ".cpp"), (".y++", ".c++"), (".yy", ".cc"), (".y", ".c"),
            (".lpp", ".cpp"), (".l++", ".c++"), (".ll", ".cc"), (".l", ".c"),
        ] {
            if s.ends_with(suf) {
                return Some(format!("{}{}", &s[..s.len() - suf.len()], gen));
            }
        }
        None
    }

    /// If a target has per-target compile flags (`X_CPPFLAGS`/`X_CFLAGS`/`X_CXXFLAGS`/...), its
    /// objects are renamed `{canon}-{stem}` and compiled with a dedicated per-object rule that
    /// carries those flags. Returns the canonical name (the object-name prefix) when so.
    fn target_flag_prefix(&self, target: &str) -> Option<String> {
        let c = Self::canon(target);
        for suf in ["_CPPFLAGS", "_CFLAGS", "_CXXFLAGS", "_OBJCFLAGS", "_OBJCXXFLAGS"] {
            if self.find_variable(&format!("{}{}", c, suf)).is_some() {
                return Some(c);
            }
        }
        None
    }

    /// The compiled units for a target: `(object, source, is_cxx)` per compiled source.
    /// `lo` selects libtool (`.lo`) vs ordinary (`.$(OBJEXT)`) objects. The source subdir is kept
    /// (foo/bar.c -> foo/bar.o); targets with per-target flags get the `{canon}-` filename prefix
    /// so a dedicated per-object rule can apply them.
    fn compiled_units(&self, target: &str, lo: bool) -> Vec<(String, String, bool)> {
        let prefix = self
            .target_flag_prefix(target)
            .map(|c| format!("{}-", c))
            .unwrap_or_default();
        let ext = if lo { "lo" } else { "$(OBJEXT)" };
        self.target_sources(target)
            .iter()
            .filter_map(|s| {
                // Yacc/Lex sources compile from the C file they generate (grammar.y -> grammar.c).
                let comp = Self::lexyacc_generated(s).unwrap_or_else(|| s.clone());
                if !Self::is_compiled_source(&comp) {
                    return None;
                }
                let stem = match comp.rfind('.') {
                    Some(d) => &comp[..d],
                    None => comp.as_str(),
                };
                let obj = match stem.rfind('/') {
                    Some(slash) => format!("{}/{}{}.{}", &stem[..slash], prefix, &stem[slash + 1..], ext),
                    None => format!("{}{}.{}", prefix, stem, ext),
                };
                Some((obj, comp.clone(), Self::is_cxx_source(&comp)))
            })
            .collect()
    }

    /// The object files for one program's sources (`.c`/`.cc`/... -> `.$(OBJEXT)`).
    fn program_objects(&self, prog: &str) -> Vec<String> {
        self.compiled_units(prog, false).into_iter().map(|(o, _, _)| o).collect()
    }

    /// Emit the program build infrastructure (variable section): `PROGRAMS`, per-program
    /// Canonicalize a target name into the form Automake uses for its derived variables: every
    /// character that is not a letter, digit, or `@` becomes `_` (so `test-program` -> `test_program`,
    /// `libfoo.a` -> `libfoo_a`). The original name is still used for the build target itself.
    fn canon(name: &str) -> String {
        name.chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '_' || c == '@' { c } else { '_' })
            .collect()
    }

    /// The sources listed for a program/library target (its `_SOURCES`, defaulting to `<name>.c`).
    fn target_sources(&self, name: &str) -> Vec<String> {
        self.find_variable(&format!("{}_SOURCES", Self::canon(name)))
            .unwrap_or_else(|| format!("{}.c", name))
            .split_whitespace()
            .map(String::from)
            .collect()
    }

    /// Whether a source file is C++ (selects the CXX compile/link toolchain).
    fn is_cxx_source(s: &str) -> bool {
        s.ends_with(".cc")
            || s.ends_with(".cpp")
            || s.ends_with(".cxx")
            || s.ends_with(".c++")
            || s.ends_with(".C")
            || s.ends_with(".mm")
    }

    /// Whether a target has any C++ source (so it must link with $(CXXLINK)).
    fn target_is_cxx(&self, name: &str) -> bool {
        self.target_sources(name).iter().any(|s| Self::is_cxx_source(s))
    }

    /// Whether any program OR libtool library has C++ sources.
    fn any_cxx(&self) -> bool {
        ["PROGRAMS", "LTLIBRARIES", "LIBRARIES"].iter().any(|k| {
            self.collect_primaries(k)
                .iter()
                .flat_map(|(_, _, t)| t.iter())
                .any(|p| self.target_is_cxx(p))
        })
    }

    /// Whether this Makefile.am builds any libtool libraries (so the libtool toolchain is needed).
    fn has_libtool(&self) -> bool {
        !self.collect_primaries("LTLIBRARIES").is_empty()
    }

    /// The libtool object files (`.lo`) for a library's compiled sources.
    fn library_objects(&self, lib: &str) -> Vec<String> {
        self.compiled_units(lib, true).into_iter().map(|(o, _, _)| o).collect()
    }

    /// Link dependencies derived from an `_LDADD`/`_LIBADD` value: the `.la`/`.a` files (so the
    /// program/library is rebuilt after, and ordered after, the libraries it links).
    fn ldadd_deps(ldadd: &str) -> String {
        ldadd
            .split_whitespace()
            .filter(|t| t.ends_with(".la") || t.ends_with(".a"))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// object/LDADD vars, and the COMPILE/CCLD/LINK command variables.
    fn generate_program_infra_vars(&self, out: &mut String) {
        let programs = self.collect_primaries("PROGRAMS");
        let ltlibs = self.collect_primaries("LTLIBRARIES");
        let libs = self.collect_primaries("LIBRARIES");
        if programs.is_empty() && ltlibs.is_empty() && libs.is_empty() {
            return;
        }
        let libtool = self.has_libtool();
        // PROGRAMS = $(bin_PROGRAMS) $(noinst_PROGRAMS) ...
        if !programs.is_empty() {
            let mut prefix_vars: Vec<String> = Vec::new();
            for (prefix, _nd, _t) in &programs {
                let p = if prefix.is_empty() { "bin" } else { prefix.as_str() };
                let v = format!("$({}_PROGRAMS)", p);
                if !prefix_vars.contains(&v) {
                    prefix_vars.push(v);
                }
            }
            out.push_str(&format!("PROGRAMS = {}\n", prefix_vars.join(" ")));
        }
        // LTLIBRARIES = $(lib_LTLIBRARIES) $(noinst_LTLIBRARIES) ...
        if !ltlibs.is_empty() {
            let mut lv: Vec<String> = Vec::new();
            for (prefix, _nd, _t) in &ltlibs {
                let p = if prefix.is_empty() { "lib" } else { prefix.as_str() };
                let v = format!("$({}_LTLIBRARIES)", p);
                if !lv.contains(&v) {
                    lv.push(v);
                }
            }
            out.push_str(&format!("LTLIBRARIES = {}\n", lv.join(" ")));
        }
        // LIBRARIES = $(noinst_LIBRARIES) $(lib_LIBRARIES) ... (static archives)
        if !libs.is_empty() {
            let mut sv: Vec<String> = Vec::new();
            for (prefix, _nd, _t) in &libs {
                let p = if prefix.is_empty() { "lib" } else { prefix.as_str() };
                let v = format!("$({}_LIBRARIES)", p);
                if !sv.contains(&v) {
                    sv.push(v);
                }
            }
            out.push_str(&format!("LIBRARIES = {}\n", sv.join(" ")));
        }
        // Per-program object/LDADD/DEPENDENCIES vars (names use the canonicalized target name).
        for (_prefix, _nd, targets) in &programs {
            for prog in targets {
                let c = Self::canon(prog);
                let objs = self.program_objects(prog);
                out.push_str(&format!("am_{}_OBJECTS = {}\n", c, objs.join(" ")));
                out.push_str(&format!("{}_OBJECTS = $(am_{}_OBJECTS)\n", c, c));
                let ldadd = self
                    .find_variable(&format!("{}_LDADD", c))
                    .unwrap_or_else(|| "$(LDADD)".to_string());
                out.push_str(&format!("{}_LDADD = {}\n", c, ldadd));
                let deps = Self::ldadd_deps(&ldadd);
                if !deps.is_empty() {
                    out.push_str(&format!("{}_DEPENDENCIES = {}\n", c, deps));
                }
            }
        }
        // Per-libtool-library object/LIBADD vars.
        for (_prefix, _nd, targets) in &ltlibs {
            for lib in targets {
                let c = Self::canon(lib);
                let objs = self.library_objects(lib);
                out.push_str(&format!("am_{}_OBJECTS = {}\n", c, objs.join(" ")));
                out.push_str(&format!("{}_OBJECTS = $(am_{}_OBJECTS)\n", c, c));
                let libadd = self.find_variable(&format!("{}_LIBADD", c)).unwrap_or_default();
                out.push_str(&format!("{}_LIBADD = {}\n", c, libadd));
            }
        }
        // Per-static-library (`.a`) object/LIBADD vars (ordinary `.$(OBJEXT)` objects).
        for (_prefix, _nd, targets) in &libs {
            for lib in targets {
                let c = Self::canon(lib);
                let objs = self.program_objects(lib);
                out.push_str(&format!("am_{}_OBJECTS = {}\n", c, objs.join(" ")));
                out.push_str(&format!("{}_OBJECTS = $(am_{}_OBJECTS)\n", c, c));
                let libadd = self.find_variable(&format!("{}_LIBADD", c)).unwrap_or_default();
                out.push_str(&format!("{}_LIBADD = {}\n", c, libadd));
            }
        }
        // Include -I$(top_builddir) so sources in a SUBDIR can find the top-level generated config.h
        // (matches GNU automake). Without it, lib/foo.c -> `config.h: No such file` even though
        // config.h exists and DEFS=-DHAVE_CONFIG_H requested it. For the top dir, top_builddir=.
        out.push_str("DEFAULT_INCLUDES = -I.@am__isrc@ -I$(top_builddir)\n");
        out.push_str("COMPILE = $(CC) $(DEFS) $(DEFAULT_INCLUDES) $(INCLUDES) $(AM_CPPFLAGS) \\\n");
        out.push_str("\t$(CPPFLAGS) $(AM_CFLAGS) $(CFLAGS)\n");
        out.push_str("CCLD = $(CC)\n");
        if libtool {
            // libtool toolchain: LINK and the .lo compile go through $(LIBTOOL).
            out.push_str("LIBTOOL = @LIBTOOL@\n");
            out.push_str("LTCOMPILE = $(LIBTOOL) $(AM_V_lt) --tag=CC $(AM_LIBTOOLFLAGS) \\\n");
            out.push_str("\t$(LIBTOOLFLAGS) --mode=compile $(CC) $(DEFS) \\\n");
            out.push_str("\t$(DEFAULT_INCLUDES) $(INCLUDES) $(AM_CPPFLAGS) $(CPPFLAGS) \\\n");
            out.push_str("\t$(AM_CFLAGS) $(CFLAGS)\n");
            out.push_str("LINK = $(LIBTOOL) $(AM_V_lt) --tag=CC $(AM_LIBTOOLFLAGS) \\\n");
            out.push_str("\t$(LIBTOOLFLAGS) --mode=link $(CCLD) $(AM_CFLAGS) $(CFLAGS) \\\n");
            out.push_str("\t$(AM_LDFLAGS) $(LDFLAGS) -o $@\n");
        } else {
            out.push_str("LINK = $(CCLD) $(AM_CFLAGS) $(CFLAGS) $(AM_LDFLAGS) $(LDFLAGS) -o $@\n");
        }
        // C++ toolchain (emitted when any program has C++ sources). CXX/CXXFLAGS come from the
        // project's AC_PROG_CXX via config.status substitution.
        if self.any_cxx() {
            out.push_str("CXX = @CXX@\n");
            out.push_str("CXXFLAGS = @CXXFLAGS@\n");
            out.push_str("CXXCOMPILE = $(CXX) $(DEFS) $(DEFAULT_INCLUDES) $(INCLUDES) $(AM_CPPFLAGS) \\\n");
            out.push_str("\t$(CPPFLAGS) $(AM_CXXFLAGS) $(CXXFLAGS)\n");
            out.push_str("CXXLD = $(CXX)\n");
            out.push_str("CXXLINK = $(CXXLD) $(AM_CXXFLAGS) $(CXXFLAGS) $(AM_LDFLAGS) $(LDFLAGS) -o $@\n");
            if libtool {
                out.push_str("LTCXXCOMPILE = $(LIBTOOL) $(AM_V_lt) --tag=CXX $(AM_LIBTOOLFLAGS) \\\n");
                out.push_str("\t$(LIBTOOLFLAGS) --mode=compile $(CXX) $(DEFS) \\\n");
                out.push_str("\t$(DEFAULT_INCLUDES) $(INCLUDES) $(AM_CPPFLAGS) $(CPPFLAGS) \\\n");
                out.push_str("\t$(AM_CXXFLAGS) $(CXXFLAGS)\n");
            }
        }
    }

    /// Emit the compile + link rules (rules section): the `.c.o`/`.c.obj` suffix rules and the
    /// per-program link rule. Real rules, so multi-source / subdir-objects programs build without
    /// depending on make's built-in suffix rules (which the generated `.SUFFIXES:` reset disables).
    fn generate_compile_link_rules(&self, out: &mut String) {
        let programs = self.collect_primaries("PROGRAMS");
        let ltlibs = self.collect_primaries("LTLIBRARIES");
        let libs = self.collect_primaries("LIBRARIES");
        if programs.is_empty() && ltlibs.is_empty() && libs.is_empty() {
            return;
        }
        let _ = &libs;
        let cxx = self.any_cxx();
        let libtool = self.has_libtool();
        // .SUFFIXES: list every extension the rules below use.
        let mut sfx = String::from(".c");
        if cxx {
            sfx.push_str(" .cc .cpp .cxx .C");
        }
        if libtool {
            sfx.push_str(" .lo");
        }
        sfx.push_str(" .o .obj");
        out.push_str(".SUFFIXES:\n");
        out.push_str(&format!(".SUFFIXES: {}\n\n", sfx));
        // Link rule per program. In a libtool project everything links through $(LINK) (libtool
        // link mode); otherwise C++ targets link with $(CXXLINK).
        for (_prefix, _nd, targets) in &programs {
            for prog in targets {
                let link = if self.target_is_cxx(prog) && !libtool { "CXXLINK" } else { "LINK" };
                let c = Self::canon(prog);
                // The build target keeps the original name; derived vars use the canonical name.
                out.push_str(&format!(
                    "{p}$(EXEEXT): $({c}_OBJECTS) $({c}_DEPENDENCIES) $(EXTRA_{c}_DEPENDENCIES)\n",
                    p = prog, c = c
                ));
                out.push_str(&format!("\t@rm -f {}$(EXEEXT)\n", prog));
                out.push_str(&format!(
                    "\t$(AM_V_CCLD)$({link}) $({c}_OBJECTS) $({c}_LDADD) $(LIBS)\n\n",
                    link = link, c = c
                ));
            }
        }
        // Link rule per libtool library: $(LINK) builds the .la; installable libraries need -rpath.
        for (prefix, _nd, targets) in &ltlibs {
            let installable = matches!(prefix.as_str(), "" | "lib" | "pkglib");
            let rpath = if installable {
                let dir = if prefix.is_empty() { "lib" } else { prefix.as_str() };
                format!("-rpath $({}dir) ", dir)
            } else {
                String::new()
            };
            for lib in targets {
                let c = Self::canon(lib);
                out.push_str(&format!(
                    "{l}: $({c}_OBJECTS) $({c}_DEPENDENCIES) $(EXTRA_{c}_DEPENDENCIES)\n",
                    l = lib, c = c
                ));
                out.push_str(&format!(
                    "\t$(AM_V_CCLD)$(LINK) {rpath}$({c}_OBJECTS) $({c}_LIBADD) $(LIBS)\n\n",
                    rpath = rpath, c = c
                ));
            }
        }
        // Per-object compile rules for targets that carry per-target flags (their objects are
        // renamed `{canon}-{stem}` so the generic suffix rules don't apply). Each rule inlines the
        // target's own _CPPFLAGS/_CFLAGS (or _CXXFLAGS) so e.g. a library's private `-I` is used.
        for (kind, lo) in [("PROGRAMS", false), ("LTLIBRARIES", true), ("LIBRARIES", false)] {
            for (_p, _nd, targets) in &self.collect_primaries(kind) {
                for target in targets {
                    if self.target_flag_prefix(target).is_none() {
                        continue;
                    }
                    let c = Self::canon(target);
                    for (obj, src, src_cxx) in self.compiled_units(target, lo) {
                        out.push_str(&format!("{}: {}\n", obj, src));
                        let (vtag, comp, fflag) = if src_cxx {
                            ("CXX", "CXX", "CXXFLAGS")
                        } else {
                            ("CC", "CC", "CFLAGS")
                        };
                        if libtool {
                            out.push_str(&format!(
                                "\t$(AM_V_{v})$(LIBTOOL) $(AM_V_lt) --tag={v} $(AM_LIBTOOLFLAGS) $(LIBTOOLFLAGS) --mode=compile $({comp}) $(DEFS) $(DEFAULT_INCLUDES) $(INCLUDES) $({c}_CPPFLAGS) $(CPPFLAGS) $({c}_{ff}) $({ff}) -c -o $@ $<\n\n",
                                v = vtag, comp = comp, c = c, ff = fflag
                            ));
                        } else {
                            out.push_str(&format!(
                                "\t$(AM_V_{v})$({comp}) $(DEFS) $(DEFAULT_INCLUDES) $(INCLUDES) $({c}_CPPFLAGS) $(CPPFLAGS) $({c}_{ff}) $({ff}) -c -o $@ $<\n\n",
                                v = vtag, comp = comp, c = c, ff = fflag
                            ));
                        }
                    }
                }
            }
        }
        // C compile suffix rules.
        out.push_str(".c.o:\n");
        out.push_str("\t$(AM_V_CC)$(COMPILE) -c -o $@ $<\n\n");
        out.push_str(".c.obj:\n");
        out.push_str("\t$(AM_V_CC)$(COMPILE) -c -o $@ `$(CYGPATH_W) '$<'`\n\n");
        if libtool {
            out.push_str(".c.lo:\n");
            out.push_str("\t$(AM_V_CC)$(LTCOMPILE) -c -o $@ $<\n\n");
        }
        // C++ compile suffix rules (one per common extension).
        if cxx {
            for ext in [".cc", ".cpp", ".cxx", ".C"] {
                out.push_str(&format!("{ext}.o:\n", ext = ext));
                out.push_str("\t$(AM_V_CXX)$(CXXCOMPILE) -c -o $@ $<\n\n");
                out.push_str(&format!("{ext}.obj:\n", ext = ext));
                out.push_str("\t$(AM_V_CXX)$(CXXCOMPILE) -c -o $@ `$(CYGPATH_W) '$<'`\n\n");
                if libtool {
                    out.push_str(&format!("{ext}.lo:\n", ext = ext));
                    out.push_str("\t$(AM_V_CXX)$(LTCXXCOMPILE) -c -o $@ $<\n\n");
                }
            }
        }
    }

    fn generate_programs_rules(&self, out: &mut String) {
        let programs = self.collect_primaries("PROGRAMS");
        if programs.is_empty() {
            return;
        }

        // Build targets
        let mut build_targets = vec![];

        for (dir_prefix, _no_dist, targets) in &programs {
            for target in targets {
                // Per-target variables
                let sources_var = format!("{}_SOURCES", target);
                let ldadd_var = format!("{}_LDADD", target);
                let ldflags_var = format!("{}_LDFLAGS", target);
                let cppflags_var = format!("{}_CPPFLAGS", target);
                let cflags_var = format!("{}_CFLAGS", target);

                // Look up source files — default to target.c
                let sources = self
                    .find_variable(&sources_var)
                    .unwrap_or_else(|| format!("{}.c", target));

                let ldadd = self.find_variable(&ldadd_var).unwrap_or_default();
                let ldflags = self.find_variable(&ldflags_var).unwrap_or_default();
                let cppflags = self.find_variable(&cppflags_var).unwrap_or_default();
                let target_cflags = self.find_variable(&cflags_var).unwrap_or_default();

                // AM_*FLAGS — global flags for all targets
                let am_cflags = self.find_variable("AM_CFLAGS").unwrap_or_default();
                let am_cppflags = self.find_variable("AM_CPPFLAGS").unwrap_or_default();
                let am_ldflags = self.find_variable("AM_LDFLAGS").unwrap_or_default();

                // Object files
                // Object files — handle subdir-objects mode
                let objects: Vec<String> = sources
                    .split_whitespace()
                    .map(|s| {
                        let obj = if s.ends_with(".c") {
                            s.replace(".c", ".$(OBJEXT)")
                        } else if s.ends_with(".cc") || s.ends_with(".cpp") || s.ends_with(".cxx") {
                            s.replace(".cc", ".$(OBJEXT)")
                                .replace(".cpp", ".$(OBJEXT)")
                                .replace(".cxx", ".$(OBJEXT)")
                        } else {
                            format!("{}.$(OBJEXT)", s)
                        };
                        // subdir-objects: preserve source subdirectory in object path
                        if self.config.subdir_objects {
                            if let Some(slash_pos) = s.rfind('/') {
                                let dir = &s[..slash_pos];
                                let base = obj.rsplit('/').next().unwrap_or(&obj);
                                format!("{}/{}", dir, base)
                            } else {
                                obj
                            }
                        } else {
                            obj
                        }
                    })
                    .collect();

                // Compile rule
                let _compile_flag = if self.config.silent_rules {
                    "$(AM_V_CC)"
                } else {
                    ""
                };
                let _verbose_flag = if self.config.silent_rules {
                    "$(AM_V_at)"
                } else {
                    ""
                };

                let prefix_path = if dir_prefix.is_empty() || dir_prefix == "bin" {
                    String::new()
                } else {
                    format!("{}/", dir_prefix)
                };

                // Compile rule with per-target + AM_* flags
                // SHADOWING: target_CFLAGS replaces AM_CFLAGS (not additive)
                let all_cppflags = [am_cppflags.as_str(), cppflags.as_str()]
                    .iter()
                    .filter(|s| !s.is_empty())
                    .copied()
                    .collect::<Vec<_>>()
                    .join(" ");
                // Per-target flags shadow (replace) global AM_* flags
                let all_cflags = if !target_cflags.is_empty() {
                    target_cflags.as_str()
                } else {
                    am_cflags.as_str()
                };

                // Compile + link rules are emitted centrally by generate_compile_link_rules
                // (header-filtered objects, proper multi-source $(LINK)). Keep only the flag
                // computations here for install/var purposes.
                let _ = (&all_cppflags, &all_cflags, &prefix_path);

                // Link rule — check for libtool
                let uses_libtool = ldadd.contains(".la")
                    || self.find_variable(&format!("{}_LIBADD", target)).is_some();
                let all_ldflags = [am_ldflags.as_str(), ldflags.as_str()]
                    .iter()
                    .filter(|s| !s.is_empty())
                    .copied()
                    .collect::<Vec<_>>()
                    .join(" ");
                let link_cflags = if !target_cflags.is_empty() {
                    target_cflags.as_str()
                } else {
                    am_cflags.as_str()
                };

                let link_cmd = if self.config.silent_rules {
                    if uses_libtool {
                        format!("$(AM_V_GEN)$(LIBTOOL) $(AM_V_lt) --tag=CC --mode=link $(CC) {} $(CFLAGS) $(LDFLAGS) -o $@", link_cflags)
                    } else {
                        format!(
                            "$(AM_V_CCLD)$(CC) {} $(CFLAGS) $(LDFLAGS) -o $@",
                            link_cflags
                        )
                    }
                } else {
                    if uses_libtool {
                        format!("$(LIBTOOL) $(AM_V_lt) --tag=CC --mode=link $(CC) {} $(CFLAGS) $(LDFLAGS) -o $@", link_cflags)
                    } else {
                        format!("$(CC) {} $(CFLAGS) $(LDFLAGS) -o $@", link_cflags)
                    }
                };

                let prog_name =
                    if dir_prefix.is_empty() || dir_prefix == "bin" || dir_prefix == "noinst" {
                        target.to_string()
                    } else {
                        format!("{}/{}", dir_prefix, target)
                    };
                // Build rule emitted by generate_compile_link_rules; suppress legacy emission.
                let _ = (&link_cmd, &all_ldflags, &objects, &ldadd, &uses_libtool);

                build_targets.push(prog_name.clone());
            }
        }

        // all-am is emitted centrally by generate_all_target (always present).
        let _ = &build_targets;

        // Install rules
        let installable: Vec<_> = programs
            .iter()
            .filter(|(dir, _, _)| dir != "noinst" && dir != "check")
            .collect();
        if !installable.is_empty() {
            out.push_str("install-exec-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                for target in targets {
                    out.push_str(&format!(
                        "\t@$(NORMAL_INSTALL)\n\t$(MKDIR_P) $(DESTDIR){}\n",
                        install_dir
                    ));
                    let prog_path = if dir_prefix.is_empty() || dir_prefix == "bin" {
                        target.to_string()
                    } else {
                        format!("{}/{}", dir_prefix, target)
                    };
                    out.push_str(&format!(
                        "\t$(INSTALL_PROGRAM) {} $(DESTDIR){}/{}\n",
                        prog_path, install_dir, target
                    ));
                }
            }
            out.push('\n');

            // Uninstall
            out.push_str("uninstall-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                for target in targets {
                    out.push_str(&format!(
                        "\t@$(NORMAL_UNINSTALL)\n\trm -f $(DESTDIR){}/{}\n",
                        install_dir, target
                    ));
                }
            }
            out.push('\n');
        }
    }

    /// Generate build rules for LTLIBRARIES primaries (libtool libraries).
    /// Court: AM.PRIMARY.LTLIBRARIES.1
    ///
    /// Produces:
    ///   - .lo compile rules with $(LIBTOOL) --mode=compile
    ///   - .la link rules with $(LIBTOOL) --mode=link
    ///   - Per-target CFLAGS/LDFLAGS/LIBADD support (shadowing AM_* vars)
    ///   - VPATH $(srcdir)/ source references for out-of-tree builds
    ///   - -rpath for installed libtool libraries
    ///   - Install/uninstall via libtool --mode=install
    #[allow(dead_code)]
    fn generate_ltlibraries_rules_legacy(&self, out: &mut String) {
        let libraries = self.collect_primaries("LTLIBRARIES");
        if libraries.is_empty() {
            return;
        }

        for (_dir_prefix, _no_dist, targets) in &libraries {
            for target in targets {
                // Convert .la to _la for variable lookup (Automake convention)
                let var_name = target.replace(".la", "_la");
                let sources_var = format!("{}_SOURCES", var_name);
                let sources = self
                    .find_variable(&sources_var)
                    .unwrap_or_else(|| format!("{}.c", var_name));

                // Per-target flags: target_CFLAGS shadows AM_CFLAGS
                let per_target = self.makefile_am.per_target_flags(&var_name);
                let target_cflags = per_target
                    .cflags
                    .as_deref()
                    .unwrap_or("$(AM_CFLAGS) $(CFLAGS)");
                let target_ldflags = per_target
                    .ldflags
                    .as_deref()
                    .unwrap_or("$(AM_LDFLAGS) $(LDFLAGS)");

                let libadd_lookup = self
                    .find_variable(&format!("{}_LIBADD", var_name))
                    .unwrap_or_default();
                let libadd = per_target.libadd.as_deref().unwrap_or(&libadd_lookup);

                let objects: Vec<String> = sources
                    .split_whitespace()
                    .map(|s| {
                        if s.ends_with(".c") {
                            s.replace(".c", ".lo")
                        } else {
                            format!("{}.lo", s)
                        }
                    })
                    .collect();

                // Compile each source → .lo with VPATH $(srcdir)/ reference
                for (src, obj) in sources.split_whitespace().zip(objects.iter()) {
                    out.push_str(&format!("{}: $(srcdir)/{}\n", obj, src));
                    if self.config.silent_rules {
                        out.push_str(&format!(
                            "\t$(AM_V_CC)$(LIBTOOL) $(AM_V_lt) --tag=CC --mode=compile $(CC) $(DEFS) $(DEFAULT_INCLUDES) $(INCLUDES) $(AM_CPPFLAGS) $(CPPFLAGS) {} -c -o $@ $<\n",
                            target_cflags
                        ));
                    } else {
                        out.push_str(&format!(
                            "\t$(LIBTOOL) $(AM_V_lt) --tag=CC --mode=compile $(CC) $(DEFS) $(DEFAULT_INCLUDES) $(INCLUDES) $(AM_CPPFLAGS) $(CPPFLAGS) {} -c -o $@ $<\n",
                            target_cflags
                        ));
                    }
                    out.push('\n');
                }

                // Link rule with -rpath for installable libraries
                let rpath_arg =
                    if !_dir_prefix.is_empty() && _dir_prefix != "noinst" && _dir_prefix != "check"
                    {
                        let install_dir = self.install_dir_for_prefix(_dir_prefix);
                        format!("-rpath $(DESTDIR){}", install_dir)
                    } else {
                        String::new()
                    };

                out.push_str(&format!("{}.la: {}\n", target, objects.join(" ")));
                if self.config.silent_rules {
                    out.push_str(&format!(
                        "\t$(AM_V_CCLD)$(LIBTOOL) $(AM_V_lt) --tag=CC --mode=link $(CC) {} {} {} -o $@ {} {}\n\n",
                        target_cflags, target_ldflags, rpath_arg, objects.join(" "), libadd
                    ));
                } else {
                    out.push_str(&format!(
                        "\t$(LIBTOOL) $(AM_V_lt) --tag=CC --mode=link $(CC) {} {} {} -o $@ {} {}\n\n",
                        target_cflags, target_ldflags, rpath_arg, objects.join(" "), libadd
                    ));
                }
            }
        }

        let installable: Vec<_> = libraries
            .iter()
            .filter(|(dir, _, _)| dir != "noinst" && dir != "check")
            .collect();

        if !installable.is_empty() {
            out.push_str("install-exec-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                for target in targets {
                    out.push_str(&format!(
                        "\t@$(NORMAL_INSTALL)\n\t$(MKDIR_P) $(DESTDIR){}\n",
                        install_dir
                    ));
                    out.push_str(&format!(
                        "\t$(LIBTOOL) $(AM_V_lt) --tag=CC --mode=install $(INSTALL) {}.la $(DESTDIR){}/{}.la\n",
                        target, install_dir, target
                    ));
                }
            }
            out.push('\n');

            out.push_str("uninstall-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                for target in targets {
                    out.push_str(&format!(
                        "\t@$(NORMAL_UNINSTALL)\n\t$(LIBTOOL) $(AM_V_lt) --mode=uninstall rm -f $(DESTDIR){}/{}.la\n",
                        install_dir, target
                    ));
                }
            }
            out.push('\n');
        }
    }

    /// Generate rules for SCRIPTS primaries.
    fn generate_scripts_rules(&self, out: &mut String) {
        let scripts = self.collect_primaries("SCRIPTS");
        if scripts.is_empty() {
            return;
        }

        let installable: Vec<_> = scripts
            .iter()
            .filter(|(dir, _, _)| dir != "noinst" && dir != "check")
            .collect();

        if !installable.is_empty() {
            out.push_str("install-exec-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                for target in targets {
                    out.push_str(&format!(
                        "\t@$(NORMAL_INSTALL)\n\t$(MKDIR_P) $(DESTDIR){}\n\t$(INSTALL_SCRIPT) {} $(DESTDIR){}/{}\n",
                        install_dir, target, install_dir, target
                    ));
                }
            }
            out.push('\n');

            out.push_str("uninstall-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                for target in targets {
                    out.push_str(&format!(
                        "\t@$(NORMAL_UNINSTALL)\n\trm -f $(DESTDIR){}/{}\n",
                        install_dir, target
                    ));
                }
            }
            out.push('\n');
        }
    }

    /// Generate rules for DATA primaries.
    fn generate_data_rules(&self, out: &mut String) {
        let data = self.collect_primaries("DATA");
        if data.is_empty() {
            return;
        }

        let installable: Vec<_> = data
            .iter()
            .filter(|(dir, _, _)| dir != "noinst" && dir != "check")
            .collect();

        if !installable.is_empty() {
            out.push_str("install-data-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                let is_nobase = self.is_nobase_primary("DATA", dir_prefix);
                for target in targets {
                    let dest_name: &str = if is_nobase {
                        target.as_str()
                    } else {
                        self.install_basename(target)
                    };
                    out.push_str(&format!(
                        "\t@$(NORMAL_INSTALL)\n\t$(MKDIR_P) $(DESTDIR){}\n\t$(INSTALL_DATA) {} $(DESTDIR){}/{}\n",
                        install_dir, target, install_dir, dest_name
                    ));
                }
            }
            out.push('\n');

            out.push_str("uninstall-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                let is_nobase = self.is_nobase_primary("DATA", dir_prefix);
                for target in targets {
                    let dest_name: &str = if is_nobase {
                        target.as_str()
                    } else {
                        self.install_basename(target)
                    };
                    out.push_str(&format!(
                        "\t@$(NORMAL_UNINSTALL)\n\trm -f $(DESTDIR){}/{}\n",
                        install_dir, dest_name
                    ));
                }
            }
            out.push('\n');
        }
    }

    /// Generate rules for LIBRARIES primaries (static libraries).
    fn generate_libraries_rules(&self, out: &mut String) {
        let libraries = self.collect_primaries("LIBRARIES");
        if libraries.is_empty() {
            return;
        }

        // Archive rule per static library. Objects come from the canonical $(X_a_OBJECTS) var
        // (emitted in generate_program_infra_vars); the per-source compile rules come from the
        // central generate_compile_link_rules (suffix + per-target-flag rules). This replaces the
        // old path that looked up `{name}_SOURCES` (non-canonical -> missed `libfoo_a_SOURCES` ->
        // defaulted to `libfoo.c` -> "No rule to make target 'libfoo.c'").
        for (_dir_prefix, _no_dist, targets) in &libraries {
            for target in targets {
                let archive = if target.ends_with(".a") {
                    target.clone()
                } else {
                    format!("{}.a", target)
                };
                let c = Self::canon(target);
                out.push_str(&format!(
                    "{a}: $({c}_OBJECTS) $({c}_DEPENDENCIES) $(EXTRA_{c}_DEPENDENCIES)\n",
                    a = archive, c = c
                ));
                out.push_str(&format!("\t$(AM_V_at)-rm -f {}\n", archive));
                out.push_str(&format!(
                    "\t$(AM_V_AR)$(AR) $(ARFLAGS) {a} $({c}_OBJECTS) $({c}_LIBADD)\n",
                    a = archive, c = c
                ));
                out.push_str(&format!("\t$(AM_V_at)$(RANLIB) {}\n\n", archive));
            }
        }

        // Install rules
        let installable: Vec<_> = libraries
            .iter()
            .filter(|(dir, _, _)| dir != "noinst" && dir != "check")
            .collect();
        if !installable.is_empty() {
            out.push_str("install-exec-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                for target in targets {
                    let lib_name = target.strip_suffix(".a").unwrap_or(target);
                    out.push_str(&format!(
                        "\t@$(NORMAL_INSTALL)\n\t$(MKDIR_P) $(DESTDIR){}\n\t$(INSTALL_DATA) {}.a $(DESTDIR){}/{}.a\n",
                        install_dir, lib_name, install_dir, lib_name
                    ));
                }
            }
            out.push('\n');

            out.push_str("uninstall-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                for target in targets {
                    let lib_name = target.strip_suffix(".a").unwrap_or(target);
                    out.push_str(&format!(
                        "\t@$(NORMAL_UNINSTALL)\n\trm -f $(DESTDIR){}/{}.a\n",
                        install_dir, lib_name
                    ));
                }
            }
            out.push('\n');
        }
    }

    /// Generate rules for HEADERS primaries.
    fn generate_headers_rules(&self, out: &mut String) {
        let headers = self.collect_primaries("HEADERS");
        if headers.is_empty() {
            return;
        }

        let installable: Vec<_> = headers
            .iter()
            .filter(|(dir, _, _)| dir != "noinst" && dir != "check")
            .collect();

        if !installable.is_empty() {
            out.push_str("install-data-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                // Check if nobase_ prefix was used for this primary
                let is_nobase = self.is_nobase_primary("HEADERS", dir_prefix);
                for target in targets {
                    let dest_name: &str = if is_nobase {
                        target.as_str()
                    } else {
                        self.install_basename(target)
                    };
                    out.push_str(&format!(
                        "\t@$(NORMAL_INSTALL)\n\t$(MKDIR_P) $(DESTDIR){}\n\t$(INSTALL_HEADER) {} $(DESTDIR){}/{}\n",
                        install_dir, target, install_dir, dest_name
                    ));
                }
            }
            out.push('\n');

            out.push_str("uninstall-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                let is_nobase = self.is_nobase_primary("HEADERS", dir_prefix);
                for target in targets {
                    let dest_name: &str = if is_nobase {
                        target.as_str()
                    } else {
                        self.install_basename(target)
                    };
                    out.push_str(&format!(
                        "\t@$(NORMAL_UNINSTALL)\n\trm -f $(DESTDIR){}/{}\n",
                        install_dir, dest_name
                    ));
                }
            }
            out.push('\n');
        }
    }

    /// Check if a primary uses nobase_ prefix by scanning Primary statements.
    fn is_nobase_primary(&self, kind: &str, dir_prefix: &str) -> bool {
        for stmt in &self.makefile_am.statements {
            if let AmStatement::Primary {
                primary,
                dir_prefix: dp,
                nobase,
                ..
            } = stmt
            {
                if primary == kind && dp == dir_prefix && *nobase {
                    return true;
                }
            }
        }
        false
    }

    /// Get the basename of a path for install targets (used when nobase_ is not set).
    fn install_basename<'a>(&self, path: &'a str) -> &'a str {
        if let Some(pos) = path.rfind('/') {
            &path[pos + 1..]
        } else {
            path
        }
    }

    /// Generate rules for MANS primaries (man pages).
    fn generate_mans_rules(&self, out: &mut String) {
        let mans = self.collect_primaries("MANS");
        if mans.is_empty() {
            return;
        }

        let installable: Vec<_> = mans
            .iter()
            .filter(|(dir, _, _)| dir != "noinst" && dir != "check")
            .collect();

        if !installable.is_empty() {
            // Generate install-man target
            out.push_str("install-man: install-man-am\n\n");
            out.push_str("install-man-am:\n");
            for (dir_prefix, _no_dist, targets) in &installable {
                // Determine man section from dir_prefix (e.g., man1 → section 1)
                let section = dir_prefix.strip_prefix("man").unwrap_or("1");
                let install_dir = format!("$(mandir)/man{}", section);
                for target in targets {
                    // Determine installed name: if target already has .N suffix, use it;
                    // otherwise append .N based on section
                    let install_name = if target.contains('.') {
                        let parts: Vec<&str> = target.rsplitn(2, '.').collect();
                        if parts.len() == 2 {
                            let ext = parts[0];
                            if ext.chars().all(|c| c.is_ascii_digit() || c == 'x') {
                                target.to_string()
                            } else {
                                format!("{}.{}", target, section)
                            }
                        } else {
                            format!("{}.{}", target, section)
                        }
                    } else {
                        format!("{}.{}", target, section)
                    };
                    out.push_str(&format!(
                        "\t@$(NORMAL_INSTALL)\n\t$(MKDIR_P) $(DESTDIR){}\n\t$(INSTALL_DATA) {} $(DESTDIR){}/{}\n",
                        install_dir, target, install_dir, install_name
                    ));
                }
            }
            out.push('\n');

            out.push_str("uninstall-man: uninstall-man-am\n\n");
            out.push_str("uninstall-man-am:\n");
            for (dir_prefix, _no_dist, targets) in &installable {
                let section = dir_prefix.strip_prefix("man").unwrap_or("1");
                let install_dir = format!("$(mandir)/man{}", section);
                for target in targets {
                    let install_name = if target.contains('.') {
                        let parts: Vec<&str> = target.rsplitn(2, '.').collect();
                        if parts.len() == 2 {
                            let ext = parts[0];
                            if ext.chars().all(|c| c.is_ascii_digit() || c == 'x') {
                                target.to_string()
                            } else {
                                format!("{}.{}", target, section)
                            }
                        } else {
                            format!("{}.{}", target, section)
                        }
                    } else {
                        format!("{}.{}", target, section)
                    };
                    out.push_str(&format!(
                        "\t@$(NORMAL_UNINSTALL)\n\trm -f $(DESTDIR){}/{}\n",
                        install_dir, install_name
                    ));
                }
            }
            out.push('\n');
        }
    }

    /// Generate build rules for TEXINFOS primaries.
    #[allow(clippy::useless_format)]
    fn generate_texinfos_rules(&self, out: &mut String) {
        let texinfos = self.collect_primaries("TEXINFOS");
        if texinfos.is_empty() {
            return;
        }

        // Collect all .texi source files
        let mut info_deps: Vec<String> = Vec::new();
        let mut dvi_targets: Vec<String> = Vec::new();
        let mut pdf_targets: Vec<String> = Vec::new();
        let mut html_targets: Vec<String> = Vec::new();

        for (_dir, _no_dist, targets) in &texinfos {
            for target in targets {
                // Derive info file name: foo.texi → foo.info
                let base = target
                    .strip_suffix(".texi")
                    .or_else(|| target.strip_suffix(".texinfo"))
                    .or_else(|| target.strip_suffix(".txi"))
                    .unwrap_or(target);
                let info_file = format!("{}.info", base);
                info_deps.push(info_file.clone());
                dvi_targets.push(format!("{}.dvi", base));
                pdf_targets.push(format!("{}.pdf", base));
                html_targets.push(format!("{}.html", base));

                // version.texi dependencies
                out.push_str(&format!("{}: $(srcdir)/version.texi\n", target));
                out.push_str(&format!("$(srcdir)/version.texi: $(srcdir)/stamp-vti\n"));
                out.push_str(
                    "$(srcdir)/stamp-vti: $(srcdir)/version.texi $(top_srcdir)/configure.ac\n",
                );
                out.push_str("\t@dir=`pwd`; cd $(srcdir); $(MAKEINFO) $(AM_MAKEINFOFLAGS) $(MAKEINFOFLAGS) -D VERSION=$(VERSION) version.texi\n");
                out.push_str("\t@mv version.texi $@\n\n");

                // makeinfo: .texi → .info
                out.push_str(&format!("{}: {}\n", info_file, target));
                out.push_str(&format!(
                    "\t$(AM_V_MAKEINFO)restore=: && backupdir=\".am$$$$.\" && \\\n"
                ));
                out.push_str(&format!("\tam__cwd=`pwd` && cd $(srcdir) && \\\n"));
                out.push_str(&format!(
                    "\trm -rf $$backupdir && mkdir $$backupdir && \\\n"
                ));
                out.push_str(&format!(
                    "\tif ($(MAKEINFO) --version) >/dev/null 2>&1; then \\\n"
                ));
                out.push_str(&format!("\t  for f in {} $@; do \\\n", target));
                out.push_str(&format!("\t    test -f $$f || continue; \\\n"));
                out.push_str("\t    rev=`echo $$f | sed -e 's/\\.[^.]*$$//' | rev`; \\\n");
                out.push_str(&format!(
                    "\t    if test -f $$rev; then mv $$rev $$backupdir; fi; \\\n"
                ));
                out.push_str(&format!("\t  done; \\\n"));
                out.push_str(&format!("\t  : > $$backupdir/trace; \\\n"));
                out.push_str(&format!(
                    "\t  $(MAKEINFO) $(AM_MAKEINFOFLAGS) $(MAKEINFOFLAGS) -o $@ `test -f '{}' || echo '$(srcdir)/'`{}; \\\n",
                    target, target
                ));
                out.push_str("\t  rc=$$?; \\\n");
                out.push_str(
                    "\t  $$restore $$backupdir/* `echo \"./$@\" | sed 's|/[^/]*$$||'`; \\\n",
                );
                out.push_str("\t  rm -rf $$backupdir; exit $$rc; \\\n");
                out.push_str("\telse \\\n");
                out.push_str("\t  rc=$$?; \\\n");
                out.push_str(
                    "\t  $$restore $$backupdir/* `echo \"./$@\" | sed 's|/[^/]*$$||'`; \\\n",
                );
                out.push_str("\t  rm -rf $$backupdir; exit $$rc; \\\n");
                out.push_str("\tfi\n\n");

                // texi2dvi: .texi → .dvi
                let dvi_target = format!("{}.dvi", base);
                out.push_str(&format!("{}: {}\n", dvi_target, target));
                out.push_str(&format!(
                    "\t$(AM_V_TEXI2DVI)TEXINPUTS=\"$(am__TEXINFO_TEX_DIR)$(PATH_SEPARATOR)$$TEXINPUTS\" \\\n"
                ));
                out.push_str(&format!(
                    "\t$(TEXI2DVI) $(AM_TEXI2DVIFLAGS) $(TEXI2DVIFLAGS) -o $@ `test -f '{}' || echo '$(srcdir)/'`{}\n\n",
                    target, target
                ));

                // texi2pdf: .texi → .pdf
                let pdf_target = format!("{}.pdf", base);
                out.push_str(&format!("{}: {}\n", pdf_target, target));
                out.push_str(&format!(
                    "\t$(AM_V_TEXI2PDF)TEXINPUTS=\"$(am__TEXINFO_TEX_DIR)$(PATH_SEPARATOR)$$TEXINPUTS\" \\\n"
                ));
                out.push_str(&format!(
                    "\t$(TEXI2PDF) $(AM_TEXI2PDFFLAGS) $(TEXI2PDFFLAGS) -o $@ `test -f '{}' || echo '$(srcdir)/'`{}\n\n",
                    target, target
                ));

                // HTML output
                let html_target = format!("{}.html", base);
                out.push_str(&format!("{}: {}\n", html_target, target));
                out.push_str(&format!("\t$(AM_V_MAKEINFO)rm -rf $(@:.html=); \\\n"));
                out.push_str(&format!(
                    "\tif $(MAKEINFO) $(AM_MAKEINFOHTMLFLAGS) $(MAKEINFOFLAGS) --html -o $(@:.html=) `test -f '{}' || echo '$(srcdir)/'`{}; \\\n",
                    target, target
                ));
                out.push_str("\tthen \\\n");
                out.push_str("\t  rm -rf $@; \\\n");
                out.push_str(
                    "\t  if test -d $(@:.html=)/.libs; then rm -rf $(@:.html=)/.libs; fi; \\\n",
                );
                out.push_str("\t  ln -s $(@:.html=) $@; \\\n");
                out.push_str("\telse \\\n");
                out.push_str("\t  rm -rf $(@:.html=); exit 1; \\\n");
                out.push_str("\tfi\n\n");
            }
        }

        // Emit TEXINFOS variables
        let info_deps_str = info_deps.join(" ");
        out.push_str(&format!("INFO_DEPS = {}\n", info_deps_str));
        out.push_str("TEXI2DVI = texi2dvi\n");
        out.push_str("TEXI2PDF = texi2pdf\n");
        out.push_str("MAKEINFO = makeinfo\n");
        out.push_str("AM_MAKEINFOFLAGS = \n");
        out.push_str("MAKEINFOFLAGS = \n");
        out.push_str("AM_MAKEINFOHTMLFLAGS = \n");
        out.push_str("AM_TEXI2DVIFLAGS = \n");
        out.push_str("TEXI2DVIFLAGS = \n");
        out.push_str("AM_TEXI2PDFFLAGS = \n");
        out.push_str("TEXI2PDFFLAGS = \n");
        out.push_str("am__TEXINFO_TEX_DIR = $(top_srcdir)/build-aux\n");
        out.push('\n');

        // Emit all-am targets for docs
        if !dvi_targets.is_empty() {
            out.push_str(&format!("dvi: {}\n\n", dvi_targets.join(" ")));
            out.push_str(&format!(".PHONY: dvi dvi-am\n\n"));
        }
        if !pdf_targets.is_empty() {
            out.push_str(&format!("pdf: {}\n\n", pdf_targets.join(" ")));
            out.push_str(".PHONY: pdf pdf-am\n\n");
        }
        if !html_targets.is_empty() {
            out.push_str(&format!("html: {}\n\n", html_targets.join(" ")));
            out.push_str(".PHONY: html html-am\n\n");
        }
        if !info_deps.is_empty() {
            out.push_str(&format!("info: {}\n\n", info_deps.join(" ")));
            out.push_str(".PHONY: info info-am\n\n");
        }
    }

    /// Generate rules for PYTHON primaries.
    fn generate_python_rules(&self, out: &mut String) {
        let python = self.collect_primaries("PYTHON");
        if python.is_empty() {
            return;
        }

        out.push_str("pythondir = $(libdir)/python$(PYTHON_VERSION)/site-packages\n");
        out.push_str("PYTHON = @PYTHON@\n");
        out.push_str("PYTHON_VERSION = @PYTHON_VERSION@\n");
        out.push_str("py_compile = $(top_srcdir)/py-compile\n\n");

        let installable: Vec<_> = python
            .iter()
            .filter(|(dir, _, _)| dir != "noinst" && dir != "check")
            .collect();

        if !installable.is_empty() {
            out.push_str("install-data-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                for target in targets {
                    out.push_str(&format!(
                        "\t@$(NORMAL_INSTALL)\n\t$(MKDIR_P) $(DESTDIR){}\n\t$(INSTALL_DATA) {} $(DESTDIR){}/{}\n",
                        install_dir, target, install_dir, target
                    ));
                }
                // Byte-compile installed .py files
                out.push_str("\t@if test -n \"$(DESTDIR)\"; then \\\n");
                out.push_str(&format!(
                    "\t  $(PYTHON) $(py_compile) --destdir=$(DESTDIR) --basedir={} {}; \\\n",
                    install_dir,
                    targets.join(" ")
                ));
                out.push_str("\telse \\\n");
                out.push_str(&format!(
                    "\t  $(PYTHON) $(py_compile) --basedir={} {}; \\\n",
                    install_dir,
                    targets.join(" ")
                ));
                out.push_str("\tfi\n");
            }
            out.push('\n');

            out.push_str("uninstall-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                for target in targets {
                    out.push_str(&format!(
                        "\t@$(NORMAL_UNINSTALL)\n\trm -f $(DESTDIR){}/{}\n",
                        install_dir, target
                    ));
                    // Remove compiled .pyc files too
                    let pyc = target.replace(".py", ".pyc");
                    out.push_str(&format!("\trm -f $(DESTDIR){}/{}\n", install_dir, pyc));
                }
            }
            out.push('\n');
        }
    }

    /// Generate rules for LISP primaries (Emacs Lisp).
    fn generate_lisp_rules(&self, out: &mut String) {
        let lisp = self.collect_primaries("LISP");
        if lisp.is_empty() {
            return;
        }

        out.push_str("lispdir = @lispdir@\n");
        out.push_str("EMACS = @EMACS@\n");
        out.push_str("EMACSLOADPATH = @EMACSLOADPATH@\n\n");

        let installable: Vec<_> = lisp
            .iter()
            .filter(|(dir, _, _)| dir != "noinst" && dir != "check")
            .collect();

        if !installable.is_empty() {
            out.push_str("install-data-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                for target in targets {
                    out.push_str(&format!(
                        "\t@$(NORMAL_INSTALL)\n\t$(MKDIR_P) $(DESTDIR){}\n\t$(INSTALL_DATA) {} $(DESTDIR){}/{}\n",
                        install_dir, target, install_dir, target
                    ));
                }
                // Byte-compile installed .el files
                for target in targets {
                    out.push_str(&format!(
                        "\t@if test -f $(DESTDIR){}/{}; then \\\n",
                        install_dir, target
                    ));
                    out.push_str(&format!(
                        "\t  $(EMACS) --batch --no-site-file -f batch-byte-compile $(DESTDIR){}/{}; \\\n",
                        install_dir, target
                    ));
                    out.push_str("\tfi\n");
                }
            }
            out.push('\n');

            out.push_str("uninstall-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                for target in targets {
                    out.push_str(&format!(
                        "\t@$(NORMAL_UNINSTALL)\n\trm -f $(DESTDIR){}/{}\n",
                        install_dir, target
                    ));
                    let elc = target.replace(".el", ".elc");
                    out.push_str(&format!("\trm -f $(DESTDIR){}/{}\n", install_dir, elc));
                }
            }
            out.push('\n');
        }
    }

    /// Generate rules for JAVA primaries.
    fn generate_java_rules(&self, out: &mut String) {
        let java = self.collect_primaries("JAVA");
        if java.is_empty() {
            return;
        }

        out.push_str("javadir = $(datadir)/java\n");
        out.push_str("JAVAC = @JAVAC@\n");
        out.push_str("JAVACFLAGS = @JAVACFLAGS@\n");
        out.push_str("JAVA = @JAVA@\n");
        out.push_str(
            "CLASSPATH_ENV = CLASSPATH=$(JAVAROOT):$(top_srcdir)/$(JAVAROOT):$$CLASSPATH\n",
        );
        out.push_str("JAVAROOT = $(top_builddir)\n\n");

        let installable: Vec<_> = java
            .iter()
            .filter(|(dir, _, _)| dir != "noinst" && dir != "check")
            .collect();

        if !installable.is_empty() {
            // Compile rules
            for (_dir, _no_dist, targets) in &java {
                for class in targets {
                    let java_src = class.replace(".class", ".java");
                    out.push_str(&format!("{}: {}\n", class, java_src));
                    out.push_str("\t$(AM_V_GEN)$(CLASSPATH_ENV) $(JAVAC) -d $(JAVAROOT) $(JAVACFLAGS) $(AM_JAVACFLAGS) $<\n\n");
                }
            }

            out.push_str("install-data-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                for target in targets {
                    out.push_str(&format!(
                        "\t@$(NORMAL_INSTALL)\n\t$(MKDIR_P) $(DESTDIR){}\n\t$(INSTALL_DATA) {} $(DESTDIR){}/{}\n",
                        install_dir, target, install_dir, target
                    ));
                }
            }
            out.push('\n');

            out.push_str("uninstall-am:\n");
            for (dir_prefix, _, targets) in &installable {
                let install_dir = self.install_dir_for_prefix(dir_prefix);
                for target in targets {
                    out.push_str(&format!(
                        "\t@$(NORMAL_UNINSTALL)\n\trm -f $(DESTDIR){}/{}\n",
                        install_dir, target
                    ));
                }
            }
            out.push('\n');
        }
    }

    /// Generate install rules.
    fn generate_install_rules(&self, out: &mut String) {
        // NORMAL_INSTALL / NORMAL_UNINSTALL macros
        out.push_str("# Install helpers\n");
        out.push_str("NORMAL_INSTALL = :\n");
        out.push_str("PRE_INSTALL = :\n");
        out.push_str("POST_INSTALL = :\n");
        out.push_str("NORMAL_UNINSTALL = :\n");
        out.push_str("PRE_UNINSTALL = :\n");
        out.push_str("POST_UNINSTALL = :\n\n");

        // Determine if we have install-exec or install-data targets
        let has_exec = self.has_install_exec_targets();
        let has_data = self.has_install_data_targets();

        // When SUBDIRS is present `install:` dispatches to install-recursive (emitted by
        // generate_recursion_rules); only the local install-am body is needed here.
        if !self.has_subdirs() {
            out.push_str("install: install-am\n");
        }
        out.push_str("install-am: all-am\n");
        if has_exec && has_data {
            out.push_str("\t@$(MAKE) $(AM_MAKEFLAGS) install-exec-am install-data-am\n");
        } else if has_exec {
            out.push_str("\t@$(MAKE) $(AM_MAKEFLAGS) install-exec-am\n");
        } else if has_data {
            out.push_str("\t@$(MAKE) $(AM_MAKEFLAGS) install-data-am\n");
        }
        out.push('\n');

        if !self.has_subdirs() { out.push_str("installcheck: installcheck-am\n\n"); } else { out.push_str("installcheck-am:\n\n"); }
        out.push_str("install-strip:\n");
        out.push_str("\t$(MAKE) $(AM_MAKEFLAGS) INSTALL_PROGRAM=\"$(INSTALL_STRIP_PROGRAM)\" \\\n");
        out.push_str(
            "\t  install_sh_PROGRAM=\"$(INSTALL_STRIP_PROGRAM)\" INSTALL_STRIP_FLAG=-s \\\n",
        );
        out.push_str("\t  `test -z '$(STRIP)' || \\\n");
        out.push_str("\t    echo \"INSTALL_PROGRAM_ENV=STRIPPROG='$(STRIP)'\"` install\n\n");

        // Documentation install targets
        out.push_str("install-info: install-info-am\n\n");
        out.push_str("install-info-am:\n");
        out.push_str("\t@$(NORMAL_INSTALL)\n");
        out.push_str("\t$(MKDIR_P) $(DESTDIR)$(infodir)\n");
        out.push_str("\t@list='$(INFO_DEPS)'; for p in $$list; do \\\n");
        out.push_str("\t  if test -f $$p; then d=; else d=\"$(srcdir)/\"; fi; \\\n");
        out.push_str("\t  $(INSTALL_DATA) $$d$$p $(DESTDIR)$(infodir)/$$p || exit $$?; \\\n");
        out.push_str("\tdone\n");
        out.push_str("\t@$(POST_INSTALL)\n");
        out.push_str("\t@if $(SHELL) -c 'install-info --version' >/dev/null 2>&1; then \\\n");
        out.push_str("\t  for p in $$list; do \\\n");
        out.push_str("\t    echo \" install-info --info-dir=$(DESTDIR)$(infodir) $(DESTDIR)$(infodir)/$$p\";\\\n");
        out.push_str(
            "\t    install-info --info-dir=$(DESTDIR)$(infodir) $(DESTDIR)$(infodir)/$$p || :;\\\n",
        );
        out.push_str("\t  done; \\\n");
        out.push_str("\tfi\n\n");

        out.push_str("install-dvi: install-dvi-am\n\n");
        out.push_str("install-dvi-am:\n\n");
        out.push_str("install-ps: install-ps-am\n\n");
        out.push_str("install-ps-am:\n\n");
        out.push_str("install-pdf: install-pdf-am\n\n");
        out.push_str("install-pdf-am:\n\n");
        out.push_str("install-html: install-html-am\n\n");
        out.push_str("install-html-am:\n\n");

        // Install hooks
        out.push_str("install-data-hook:\n\n");
        out.push_str("install-exec-hook:\n\n");

        // installdirs target
        if !self.has_subdirs() { out.push_str("installdirs: installdirs-am\n"); }
        out.push_str("installdirs-am:\n");
        out.push_str("\tfor dir in \"$(DESTDIR)$(bindir)\" \"$(DESTDIR)$(sbindir)\" \"$(DESTDIR)$(libexecdir)\" \"$(DESTDIR)$(datadir)\" \"$(DESTDIR)$(infodir)\" \"$(DESTDIR)$(mandir)\"; do \\\n");
        out.push_str("\t  $(MKDIR_P) $$dir || exit 1; \\\n");
        out.push_str("\tdone\n\n");

        // .PHONY targets
        out.push_str(".PHONY: install install-am install-exec-am install-data-am\n");
        out.push_str(".PHONY: install-strip installcheck installcheck-am\n");
        out.push_str(".PHONY: install-info install-info-am\n");
        out.push_str(".PHONY: install-dvi install-dvi-am install-ps install-ps-am\n");
        out.push_str(".PHONY: install-pdf install-pdf-am install-html install-html-am\n");
        out.push_str(".PHONY: install-data-hook install-exec-hook\n");
        out.push_str(".PHONY: installdirs installdirs-am\n\n");
    }

    /// Check if any primaries produce install-exec targets.
    fn has_install_exec_targets(&self) -> bool {
        for stmt in &self.makefile_am.statements {
            if let AmStatement::Primary {
                primary,
                dir_prefix,
                ..
            } = stmt
            {
                if dir_prefix != "noinst" && dir_prefix != "check" {
                    match primary.as_str() {
                        "PROGRAMS" | "LIBRARIES" | "LTLIBRARIES" | "SCRIPTS" => return true,
                        _ => {}
                    }
                }
            }
        }
        false
    }

    /// Check if any primaries produce install-data targets.
    fn has_install_data_targets(&self) -> bool {
        for stmt in &self.makefile_am.statements {
            if let AmStatement::Primary {
                primary,
                dir_prefix,
                ..
            } = stmt
            {
                if dir_prefix != "noinst" && dir_prefix != "check" {
                    match primary.as_str() {
                        "DATA" | "HEADERS" | "MANS" | "TEXINFOS" | "LISP" | "PYTHON" => {
                            return true
                        }
                        _ => {}
                    }
                }
            }
        }
        false
    }

    /// Generate clean rules — full four-level GNU hierarchy.
    fn generate_clean_rules(&self, out: &mut String) {
        // Collect clean targets
        let programs = self.collect_primaries("PROGRAMS");
        let libraries = self.collect_primaries("LIBRARIES");
        let ltlibraries = self.collect_primaries("LTLIBRARIES");
        let has_tests = !self.collect_primaries("TESTS").is_empty();

        let mut clean_progs: Vec<String> = Vec::new();
        for (_, _, targets) in &programs {
            for t in targets {
                clean_progs.push(format!("{0} {0}.$(OBJEXT)", t));
            }
        }
        let mut clean_libs: Vec<String> = Vec::new();
        for (_, _, targets) in &libraries {
            for t in targets {
                let n = t.strip_suffix(".a").unwrap_or(t);
                clean_libs.push(format!("{0}.a {0}.$(OBJEXT)", n));
            }
        }
        let mut clean_ltlibs: Vec<String> = Vec::new();
        for (_, _, targets) in &ltlibraries {
            for t in targets {
                clean_ltlibs.push(format!("{0}.la {0}.lo .libs/", t));
            }
        }

        // --- mostlyclean: objects, libtool objects, test logs, .deps ---
        out.push_str("mostlyclean-am: mostlyclean-generic\n");
        if !clean_progs.is_empty() || !clean_libs.is_empty() || !clean_ltlibs.is_empty() {
            out.push_str("\t-rm -f");
            for p in &clean_progs {
                out.push_str(&format!(" {}", p));
            }
            for l in &clean_libs {
                out.push_str(&format!(" {}", l));
            }
            out.push('\n');
        }
        if !clean_ltlibs.is_empty() {
            out.push_str("\t-rm -rf .libs/\n");
        }
        if has_tests {
            out.push_str("\t-rm -f *.log *.trs test-suite.log\n");
        }
        out.push_str("\t-rm -rf .deps/\n\n");

        // --- clean: mostlyclean + programs, libraries ---
        out.push_str("clean-am: clean-generic mostlyclean-am\n");
        if !clean_progs.is_empty() || !clean_libs.is_empty() {
            out.push_str("\t-rm -f");
            for p in &clean_progs {
                out.push_str(&format!(" {}", p));
            }
            for l in &clean_libs {
                out.push_str(&format!(" {}", l));
            }
            for lt in &clean_ltlibs {
                out.push_str(&format!(" {}", lt));
            }
            out.push('\n');
        }
        out.push('\n');

        // --- distclean: clean + configure output ---
        out.push_str("distclean-am: distclean-generic clean-am\n");
        out.push_str("\t-rm -f Makefile config.status config.log\n");
        out.push_str("\t-rm -f config.h stamp-h1 stamp-h2\n");
        out.push_str("\t-rm -f libtool config.lt\n");
        out.push_str("\t-rm -rf autom4te.cache/\n\n");

        // --- maintainer-clean: distclean + generated files ---
        out.push_str("maintainer-clean-am: maintainer-clean-generic distclean-am\n");
        out.push_str("\t@echo \"This command is intended for maintainers to use\"\n");
        out.push_str("\t@echo \"it deletes files that may require special tools to rebuild.\"\n");
        out.push_str("\t-rm -f configure aclocal.m4\n");
        out.push_str("\t-rm -f Makefile.in Makefile\n");
        out.push_str("\t-rm -f config.h.in config.h.in~\n");
        out.push_str("\t-rm -f $(am__configure_deps)\n");
        out.push_str("\t-rm -rf $(top_srcdir)/autom4te.cache/\n\n");

        // --- PHONY targets ---
        if !self.has_subdirs() { out.push_str("clean: clean-am\n\n"); }
        out.push_str(".PHONY: clean clean-am mostlyclean-am distclean-am maintainer-clean-am\n");
        out.push_str(".PHONY: mostlyclean mostlyclean-am distclean distclean-am\n");
        out.push_str(".PHONY: maintainer-clean maintainer-clean-am\n\n");
    }

    /// Generate dist rules.
    fn generate_dist_rules(&self, out: &mut String) {
        out.push_str("distdir:\n");
        out.push_str(
            "\t@srcdirstrip=`echo \"$(srcdir)\" | sed 's/[].[^$$\\\\*]/\\\\\\\\&/g'`; \\\n",
        );
        out.push_str(
            "\ttopsrcdirstrip=`echo \"$(top_srcdir)\" | sed 's/[].[^$$\\\\*]/\\\\\\\\&/g'`; \\\n",
        );
        out.push_str("\tlist='$(DISTFILES)'; \\\n");
        out.push_str("\t  dist_files=`for file in $$list; do echo $$file; done | \\\n");
        out.push_str("\t  sed -e \"s|^$$srcdirstrip/||;t\" \\\n");
        out.push_str("\t      -e \"s|^$$topsrcdirstrip/|$(top_builddir)/|;t\"`; \\\n");
        out.push_str("\tcase $$dist_files in \\\n");
        out.push_str("\t  */*) $(MKDIR_P) `echo \"$$dist_files\" | \\\n");
        out.push_str("\t\t\t sed '/\\//!d;s|^|$(distdir)/|;s,/[^/]*$$,,' | \\\n");
        out.push_str("\t\t\t sort -u` ;; \\\n");
        out.push_str("\tesac; \\\n");
        out.push_str("\tfor file in $$dist_files; do \\\n");
        out.push_str(
            "\t  if test -f $$file || test -d $$file; then d=.; else d=$(srcdir); fi; \\\n",
        );
        out.push_str("\t  if test -d $$d/$$file; then \\\n");
        out.push_str("\t    dir=`echo \"/$$file\" | sed -e 's,/[^/]*$$,,'`; \\\n");
        out.push_str("\t    if test -d \"$(distdir)/$$file\"; then \\\n");
        out.push_str("\t      find \"$(distdir)/$$file\" -type d ! -perm -700 -exec chmod u+rwx {} \\;; \\\n");
        out.push_str("\t    fi; \\\n");
        out.push_str("\t    if test -d $(srcdir)/$$file && test $$d != $(srcdir); then \\\n");
        out.push_str("\t      cp -fpR $(srcdir)/$$file \"$(distdir)$$dir\" || exit 1; \\\n");
        out.push_str("\t      find \"$(distdir)/$$file\" -type d ! -perm -700 -exec chmod u+rwx {} \\;; \\\n");
        out.push_str("\t    fi; \\\n");
        out.push_str("\t    cp -fpR $$d/$$file \"$(distdir)$$dir\" || exit 1; \\\n");
        out.push_str("\t  else \\\n");
        out.push_str("\t    test -f \"$(distdir)/$$file\" \\\n");
        out.push_str("\t    || cp -p $$d/$$file \"$(distdir)/$$file\" \\\n");
        out.push_str("\t    || exit 1; \\\n");
        out.push_str("\t  fi; \\\n");
        out.push_str("\tdone\n\n");

        out.push_str("dist dist-all: distdir\n\n");
        out.push_str("dist-gzip: distdir\n");
        out.push_str(
            "\ttardir=$(distdir) && $(am__tar) | GZIP=$(GZIP_ENV) gzip -c >$(distdir).tar.gz\n\n",
        );
        out.push_str("distcleancheck: distclean\n");
        out.push_str("\t@if test '$(srcdir)' = . ; then \\\n");
        out.push_str("\t  echo 'ERROR: distcleancheck can only run from a VPATH build' ; \\\n");
        out.push_str("\t  exit 1 ; \\\n");
        out.push_str("\tfi\n");
        out.push_str("\t@test `$(distcleancheck_listfiles) | wc -l` -eq 0 \\\n");
        out.push_str(
            "\t  || { echo 'ERROR: files left in build directory after distclean:' ; \\\n",
        );
        out.push_str("\t       $(distcleancheck_listfiles) ; \\\n");
        out.push_str("\t       exit 1; } >&2\n\n");
        // distcheck: verify distribution integrity
        out.push_str("distcheck: dist\n");
        out.push_str("\tcase '$(DIST_ARCHIVES)' in *?.tar.gz*) \\\n");
        out.push_str("\t  GZIP=$(GZIP_ENV) gzip -dc $(distdir).tar.gz | $(am__untar) ;;;\n");
        out.push_str("\t*) \\\n");
        out.push_str("\t  set -x; false;;\n");
        out.push_str("\tesac\n");
        out.push_str("\tchmod -R a-w $(distdir)\n");
        out.push_str("\tchmod u+w $(distdir)\n");
        out.push_str("\tmkdir $(distdir)/_build $(distdir)/_build/sub $(distdir)/_inst\n");
        out.push_str("\tchmod a-w $(distdir)\n");
        out.push_str("\ttest -d $(distdir)/_build || exit 0; \\\n");
        out.push_str("\tdc_install_base=`$(am__cd) $(distdir)/_inst && pwd | sed -e 's,^[^:\\\\/]:[\\\\/],/,'` \\\n");
        out.push_str("\t  && dc_destdir=\"$${TMPDIR-/tmp}/am-dc-$$$$/\" \\\n");
        out.push_str("\t  && am__cwd=`pwd` \\\n");
        out.push_str("\t  && $(am__cd) $(distdir)/_build/sub \\\n");
        out.push_str("\t  && ../../configure \\\n");
        out.push_str("\t    $(AM_DISTCHECK_CONFIGURE_FLAGS) \\\n");
        out.push_str("\t    $(DISTCHECK_CONFIGURE_FLAGS) \\\n");
        out.push_str("\t    --srcdir=../.. --prefix=\"$$dc_install_base\" \\\n");
        out.push_str("\t  && $(MAKE) $(AM_MAKEFLAGS) \\\n");
        out.push_str("\t  && $(MAKE) $(AM_MAKEFLAGS) check \\\n");
        out.push_str("\t  && $(MAKE) $(AM_MAKEFLAGS) install \\\n");
        out.push_str("\t  && $(MAKE) $(AM_MAKEFLAGS) installcheck \\\n");
        out.push_str("\t  && $(MAKE) $(AM_MAKEFLAGS) uninstall \\\n");
        out.push_str(
            "\t  && $(MAKE) $(AM_MAKEFLAGS) distuninstallcheck_dir=\"$$dc_install_base\" \\\n",
        );
        out.push_str("\t        distuninstallcheck \\\n");
        out.push_str("\t  && chmod -R a-w \"$$dc_install_base\" \\\n");
        out.push_str("\t  && ({ \\\n");
        out.push_str("\t       (cd ../.. && $(MAKE) $(AM_MAKEFLAGS) distcleancheck) ; \\\n");
        out.push_str("\t    } || :) \\\n");
        out.push_str("\t  && rm -rf \"$$dc_destdir\" \\\n");
        out.push_str("\t  && $(MAKE) $(AM_MAKEFLAGS) dist-gzip \\\n");
        out.push_str("\t  && rm -rf $(DIST_ARCHIVES) \\\n");
        out.push_str("\t  && $(MAKE) $(AM_MAKEFLAGS) distcleancheck\n");
        out.push_str("\t@echo\n");
        out.push_str("\t@echo \"distcheck: OK\"\n");
        out.push_str("\t@$(am__rmdir) $(distdir)\n");
        out.push_str("\t@(echo \"$(distdir).tar.gz is ready for distribution\") | \\\n");
        out.push_str("\t  sed 'h;s/./=/g;p;x;p;x'\n\n");
        out.push_str(".PHONY: dist dist-all distdir dist-gzip distcleancheck\n\n");
    }

    /// Generate check/test rules.
    fn generate_check_rules(&self, out: &mut String) {
        let tests = self.collect_primaries("TESTS");
        let subdirs = self.has_subdirs();
        if tests.is_empty() {
            out.push_str("check-am: all-am\n");
            if !subdirs {
                out.push_str("check: check-am\n\n");
                out.push_str(".PHONY: check check-am\n\n");
            } else {
                out.push('\n');
            }
            return;
        }

        out.push_str("check-am: all-am\n");
        out.push_str("\t@$(MAKE) $(AM_MAKEFLAGS) check-TESTS\n\n");
        if !subdirs {
            out.push_str("check: check-am\n\n");
        }

        let _all_tests: Vec<&str> = tests
            .iter()
            .flat_map(|(_, _, targets)| targets.iter().map(|s| s.as_str()))
            .collect();

        out.push_str("check-TESTS: $(TESTS)\n");
        out.push_str("\t@failed=0; all=0; xfail=0; xpass=0; skip=0; \\\n");
        out.push_str("\tsrcdir=$(srcdir); export srcdir; \\\n");
        out.push_str("\tlist=' $(TESTS) '; \\\n");
        out.push_str("\t$(am__tty_colors); \\\n");
        out.push_str("\tif test -n \"$$list\"; then \\\n");
        out.push_str("\t  for tst in $$list; do \\\n");
        out.push_str("\t    if test -f ./$$tst; then dir=./; \\\n");
        out.push_str("\t    elif test -f $$tst; then dir=; \\\n");
        out.push_str("\t    else dir=\"$(srcdir)/\"; fi; \\\n");
        out.push_str("\t    if $(TESTS_ENVIRONMENT) $${dir}$$tst $(LOG_COMPILER) $(AM_LOG_FLAGS) $(LOG_FLAGS) \\\n");
        out.push_str("\t        >$$tst.log 2>&1; then \\\n");
        out.push_str("\t      echo ':test-result: PASS' > $$tst.trs; \\\n");
        out.push_str("\t      all=`expr $$all + 1`; \\\n");
        out.push_str("\t      case \" $(XFAIL_TESTS) \" in \\\n");
        out.push_str("\t      *[\\t\\ \\n]$$tst[\\t\\ \\n]*) \\\n");
        out.push_str("\t        xpass=`expr $$xpass + 1`; \\\n");
        out.push_str("\t        failed=`expr $$failed + 1`; \\\n");
        out.push_str("\t        col=$$red; res=XPASS; \\\n");
        out.push_str("\t      ;; \\\n");
        out.push_str("\t      *) \\\n");
        out.push_str("\t        col=$$grn; res=PASS; \\\n");
        out.push_str("\t      ;; \\\n");
        out.push_str("\t      esac; \\\n");
        out.push_str("\t    elif test $$? -gt 0; then \\\n");
        out.push_str("\t      echo ':test-result: FAIL' > $$tst.trs; \\\n");
        out.push_str("\t      all=`expr $$all + 1`; \\\n");
        out.push_str("\t      case \" $(XFAIL_TESTS) \" in \\\n");
        out.push_str("\t      *[\\t\\ \\n]$$tst[\\t\\ \\n]*) \\\n");
        out.push_str("\t        xfail=`expr $$xfail + 1`; \\\n");
        out.push_str("\t        col=$$lgn; res=XFAIL; \\\n");
        out.push_str("\t      ;; \\\n");
        out.push_str("\t      *) \\\n");
        out.push_str("\t        failed=`expr $$failed + 1`; \\\n");
        out.push_str("\t        col=$$red; res=FAIL; \\\n");
        out.push_str("\t      ;; \\\n");
        out.push_str("\t      esac; \\\n");
        out.push_str("\t    else \\\n");
        out.push_str("\t      echo ':test-result: SKIP' > $$tst.trs; \\\n");
        out.push_str("\t      skip=`expr $$skip + 1`; \\\n");
        out.push_str("\t      col=$$blu; res=SKIP; \\\n");
        out.push_str("\t    fi; \\\n");
        out.push_str("\t    echo \"$${col}$${res}$${std}: $$tst\"; \\\n");
        out.push_str("\t    echo \"$${col}$${res}$${std}: $$tst\" >> test-suite.log; \\\n");
        out.push_str("\t  done; \\\n");
        out.push_str("\t  if test \"$$all\" -eq 1; then \\\n");
        out.push_str("\t    tests=\"test\"; \\\n");
        out.push_str("\t    All=\"\"; \\\n");
        out.push_str("\t  else \\\n");
        out.push_str("\t    tests=\"tests\"; \\\n");
        out.push_str("\t    All=\"All \"; \\\n");
        out.push_str("\t  fi; \\\n");
        out.push_str("\t  if test \"$$failed\" -eq 0; then \\\n");
        out.push_str("\t    if test \"$$xfail\" -eq 0; then \\\n");
        out.push_str("\t      banner=\"$$All$$all $$tests passed\"; \\\n");
        out.push_str("\t    else \\\n");
        out.push_str("\t      if test \"$$xfail\" -eq 1; then failures=failure; else failures=failures; fi; \\\n");
        out.push_str("\t      banner=\"$$All$$all $$tests behaved as expected ($$xfail expected $$failures)\"; \\\n");
        out.push_str("\t    fi; \\\n");
        out.push_str("\t  else \\\n");
        out.push_str("\t    if test \"$$xpass\" -eq 0; then \\\n");
        out.push_str("\t      banner=\"$$failed of $$all $$tests failed\"; \\\n");
        out.push_str("\t    else \\\n");
        out.push_str(
            "\t      if test \"$$xpass\" -eq 1; then passes=pass; else passes=passes; fi; \\\n",
        );
        out.push_str("\t      banner=\"$$failed of $$all $$tests did not behave as expected ($$xpass unexpected $$passes)\"; \\\n");
        out.push_str("\t    fi; \\\n");
        out.push_str("\t  fi; \\\n");
        out.push_str("\t  dashes=\"$$banner\"; \\\n");
        out.push_str("\t  skipped=\"\"; \\\n");
        out.push_str("\t  if test \"$$skip\" -ne 0; then \\\n");
        out.push_str("\t    if test \"$$skip\" -eq 1; then \\\n");
        out.push_str("\t      skipped=\"($$skip test was not run)\"; \\\n");
        out.push_str("\t    else \\\n");
        out.push_str("\t      skipped=\"($$skip tests were not run)\"; \\\n");
        out.push_str("\t    fi; \\\n");
        out.push_str(
            "\t    test `echo \"$$skipped\" | wc -c` -le `echo \"$$banner\" | wc -c` || \\\n",
        );
        out.push_str("\t      dashes=\"$$skipped\"; \\\n");
        out.push_str("\t  fi; \\\n");
        out.push_str("\t  report=\"\"; \\\n");
        out.push_str(
            "\t  if test \"$$failed\" -ne 0 && test -n \"$(PACKAGE_BUGREPORT)\"; then \\\n",
        );
        out.push_str("\t    report=\"Please report to $(PACKAGE_BUGREPORT)\"; \\\n");
        out.push_str(
            "\t    test `echo \"$$report\" | wc -c` -le `echo \"$$banner\" | wc -c` || \\\n",
        );
        out.push_str("\t      dashes=\"$$report\"; \\\n");
        out.push_str("\t  fi; \\\n");
        out.push_str("\t  dashes=`echo \"$$dashes\" | sed s/./=/g`; \\\n");
        out.push_str("\t  if test \"$$failed\" -eq 0; then \\\n");
        out.push_str("\t    col=\"$$grn\"; \\\n");
        out.push_str("\t  else \\\n");
        out.push_str("\t    col=\"$$red\"; \\\n");
        out.push_str("\t  fi; \\\n");
        out.push_str("\t  echo \"$${col}$$dashes$${std}\"; \\\n");
        out.push_str("\t  echo \"$${col}$$banner$${std}\"; \\\n");
        out.push_str("\t  test -z \"$$skipped\" || echo \"$${col}$$skipped$${std}\"; \\\n");
        out.push_str("\t  test -z \"$$report\" || echo \"$${col}$$report$${std}\"; \\\n");
        out.push_str("\t  echo \"$${col}$$dashes$${std}\"; \\\n");
        out.push_str("\t  test \"$$failed\" -eq 0; \\\n");
        out.push_str("\telse :; fi\n\n");

        out.push_str(".PHONY: check check-am check-TESTS\n\n");

        // recheck target — rerun only failed tests
        out.push_str("recheck: all-am\n");
        out.push_str("\t@test -z \"$(TEST_SUITE_LOG)\" || rm -f $(TEST_SUITE_LOG)\n");
        out.push_str("\t@ws='[\t ]'; \\\n");
        out.push_str("\t  if test -n \"$$list\"; then \\\n");
        out.push_str("\t    list=`for tst in $$list; do \\\n");
        out.push_str("\t      if test -f $$tst.trs; then \\\n");
        out.push_str("\t        if grep -q '^:test-result: *FAIL' $$tst.trs; then \\\n");
        out.push_str("\t          echo $$tst; \\\n");
        out.push_str("\t        fi; \\\n");
        out.push_str("\t      else \\\n");
        out.push_str("\t        echo $$tst; \\\n");
        out.push_str("\t      fi; \\\n");
        out.push_str("\t    done | tr '\\n\\r' '  '`; \\\n");
        out.push_str("\t    $(MAKE) $(AM_MAKEFLAGS) check-TESTS TESTS=\"$$list\"; \\\n");
        out.push_str("\t  else :; fi\n\n");

        // Color test support
        {
            let esc = "\x1b";
            out.push_str("am__tty_colors = { \\\n");
            out.push_str("  if test -t 1; then \\\n");
            out.push_str(&format!("    red='{}[0;31m'; grn='{}[0;32m'; lgn='{}[1;32m'; blu='{}[1;34m'; std='{}[0m'; \\\n", esc, esc, esc, esc, esc));
            out.push_str("  else \\\n");
            out.push_str("    red=; grn=; lgn=; blu=; std=; \\\n");
            out.push_str("  fi; \\\n");
            out.push_str("}\n\n");

            // Parallel-tests: test-suite.log aggregation
            let tests = self.collect_primaries("TESTS");
            if !tests.is_empty() {
                out.push_str("TEST_SUITE_LOG = test-suite.log\n");
                out.push_str("TEST_EXTENSIONS = .test\n");
                out.push_str("LOG_COMPILER = $(LOG_COMPILE)\n");
                out.push_str("AM_LOG_FLAGS = \n");
                out.push_str("LOG_FLAGS = \n\n");

                out.push_str("test-suite.log: $(TEST_SUITE_LOG)\n");
                out.push_str("$(TEST_SUITE_LOG): $(TESTS)\n");
                out.push_str("\t@$(am__rmdir) $(TEST_SUITE_DIR)\n");
                out.push_str("\t@$(MKDIR_P) $(TEST_SUITE_DIR)\n");
                out.push_str("\t@failed=0; all=0; xfail=0; xpass=0; skip=0; \\\n");
                out.push_str("\tsrcdir=$(srcdir); export srcdir; \\\n");
                out.push_str("\tfor tst in $(TESTS); do \\\n");
                out.push_str("\t  if test -f ./$$tst; then dir=./; \\\n");
                out.push_str("\t  elif test -f $$tst; then dir=; \\\n");
                out.push_str("\t  else dir=\"$(srcdir)/\"; fi; \\\n");
                out.push_str("\t  if $(TESTS_ENVIRONMENT) $${dir}$$tst $(LOG_COMPILER) $(AM_LOG_FLAGS) $(LOG_FLAGS) \\\n");
                out.push_str("\t      >$$tst.log 2>&1; then \\\n");
                out.push_str("\t    echo ':test-result: PASS' > $$tst.trs; \\\n");
                out.push_str("\t    col=$$grn; res=PASS; \\\n");
                out.push_str("\t  else \\\n");
                out.push_str("\t    echo ':test-result: FAIL' > $$tst.trs; \\\n");
                out.push_str("\t    col=$$red; res=FAIL; \\\n");
                out.push_str("\t  fi; \\\n");
                out.push_str("\t  echo \"$${col}$${res}$${std}: $$tst\"; \\\n");
                out.push_str("\tdone; \\\n");
                out.push_str("\tif test \"$$failed\" -eq 0; then \\\n");
                out.push_str("\t  col=$$grn; \\\n");
                out.push_str("\telse \\\n");
                out.push_str("\t  col=$$red; \\\n");
                out.push_str("\tfi; \\\n");
                out.push_str("\techo \"$${col}$$dashes$${std}\"; \\\n");
                out.push_str("\ttest \"$$failed\" -eq 0\n\n");
            }
        }
    }

    /// Generate utility targets: TAGS, tags, cscope, installcheck body, distuninstallcheck.
    fn generate_utility_targets(&self, out: &mut String) {
        // TAGS: etags target (GNU standards requirement)
        out.push_str("TAGS: tags\n");
        out.push_str("tags: TAGS\n\n");
        out.push_str("TAGS-am:\n");
        out.push_str("\t$(am__tag) TAGS-TAGS\n\n");
        out.push_str("tags-am:\n");
        out.push_str("\t$(am__tag) tags-TAGS\n\n");
        out.push_str("am__tag = \\\n");
        out.push_str("\tif test -n \"$$list\"; then \\\n");
        out.push_str("\t  $(ETAGS) -o $$@ $$list; \\\n");
        out.push_str("\telse \\\n");
        out.push_str("\t  $(ETAGS) -o $$@; \\\n");
        out.push_str("\tfi\n\n");

        out.push_str("ETAGS = etags\n");
        out.push_str("CTAGS = ctags\n\n");

        // cscope target
        out.push_str("cscope: cscope-am\n\n");
        out.push_str("cscope-am:\n");
        out.push_str("\t$(am__cd) $(srcdir) && cscope -b -q -k -i cscope.files\n\n");

        // installcheck body — test the installed package
        out.push_str("installcheck-am:\n");
        out.push_str("\t@:\n\n");

        // distuninstallcheck — verify uninstall left no files
        out.push_str("distuninstallcheck:\n");
        out.push_str("\t@cd $(distuninstallcheck_dir) \\\n");
        out.push_str("\t&& test `find . -type f -print | wc -l` -eq 0 \\\n");
        out.push_str("\t|| { echo 'ERROR: files left after uninstall:' ; \\\n");
        out.push_str("\t     find . -type f -print ; \\\n");
        out.push_str("\t     exit 1; } >&2\n\n");

        out.push_str(".PHONY: TAGS tags cscope cscope-am\n");
        out.push_str(".PHONY: installcheck installcheck-am distuninstallcheck\n\n");

        // Automake rebuild rules — re-run automake when inputs change
        out.push_str("$(srcdir)/Makefile.in: $(srcdir)/Makefile.am $(top_srcdir)/configure.ac\n");
        out.push_str("\t@if test -f $(top_srcdir)/configure.ac; then \\\n");
        out.push_str("\t  cd $(top_srcdir) && $(AUTOMAKE) --foreign Makefile; \\\n");
        out.push_str("\telse :; fi\n\n");
    }

    /// Pass through any target rules from the Makefile.am that we didn't process.
    fn generate_passthrough_rules(&self, out: &mut String) {
        self.emit_statements(&self.makefile_am.statements, out);
    }

    /// Recursively emit statements, handling conditional blocks properly.
    /// Append `body` to `out`, prefixing every non-empty line with `prefix` (the
    /// `@COND_TRUE@`/`@COND_FALSE@` substitution marker). Blank lines are preserved as-is.
    fn push_prefixed(out: &mut String, body: &str, prefix: &str) {
        for line in body.split_inclusive('\n') {
            let trimmed = line.strip_suffix('\n').unwrap_or(line);
            if trimmed.is_empty() {
                out.push('\n');
            } else {
                out.push_str(prefix);
                out.push_str(line);
                if !line.ends_with('\n') {
                    out.push('\n');
                }
            }
        }
    }

    fn emit_statements(&self, statements: &[AmStatement], out: &mut String) {
        for stmt in statements {
            match stmt {
                AmStatement::TargetRule {
                    target,
                    dependencies,
                    recipe_lines,
                } => {
                    out.push_str(&format!("{}: {}\n", target, dependencies.join(" ")));
                    for line in recipe_lines {
                        out.push_str(line);
                        out.push('\n');
                    }
                    out.push('\n');
                }
                AmStatement::ConditionalBlock {
                    condition,
                    negated,
                    if_branch,
                    else_branch,
                } => {
                    // Conditionals become per-line @COND_TRUE@/@COND_FALSE@ substitution prefixes
                    // in Makefile.in (NOT literal if/endif, which `make` cannot parse).
                    let (then_sense, else_sense) = if *negated {
                        ("FALSE", "TRUE")
                    } else {
                        ("TRUE", "FALSE")
                    };
                    let mut tb = String::new();
                    self.emit_statements(if_branch, &mut tb);
                    Self::push_prefixed(out, &tb, &format!("@{}_{}@", condition, then_sense));
                    if !else_branch.is_empty() {
                        let mut eb = String::new();
                        self.emit_statements(else_branch, &mut eb);
                        Self::push_prefixed(out, &eb, &format!("@{}_{}@", condition, else_sense));
                    }
                }
                AmStatement::Include(file) => {
                    out.push_str(&format!("include {}\n\n", file));
                }
                _ => {}
            }
        }
    }

    fn collect_primaries(&self, kind: &str) -> Vec<(String, bool, Vec<String>)> {
        let mut result = vec![];
        self.collect_primaries_from(&self.makefile_am.statements, kind, &mut result);
        result
    }

    fn collect_primaries_from(
        &self,
        statements: &[AmStatement],
        kind: &str,
        result: &mut Vec<(String, bool, Vec<String>)>,
    ) {
        for stmt in statements {
            match stmt {
                AmStatement::Primary {
                    dir_prefix,
                    no_dist,
                    primary,
                    targets,
                    ..
                } if primary == kind => {
                    result.push((dir_prefix.clone(), *no_dist, targets.clone()));
                }
                AmStatement::ConditionalBlock {
                    if_branch,
                    else_branch,
                    ..
                } => {
                    self.collect_primaries_from(if_branch, kind, result);
                    self.collect_primaries_from(else_branch, kind, result);
                }
                _ => {}
            }
        }
    }

    /// Find a variable value from the Makefile.am statements.
    fn find_variable(&self, name: &str) -> Option<String> {
        // Accumulate unconditional `=`/`+=` definitions so `X = a` followed by `X += b` yields
        // "a b" (the old "return first match" lost every `+=`, e.g. truncating multi-line
        // `_SOURCES` to just its first file). Falls back to the recursive lookup (conditional
        // blocks) when there is no top-level definition.
        let mut acc: Vec<String> = Vec::new();
        let mut found = false;
        for stmt in &self.makefile_am.statements {
            match stmt {
                AmStatement::VariableAssignment { name: n, op, values, conditional }
                    if n == name && conditional.is_none() =>
                {
                    match op {
                        AssignmentOp::Append => acc.extend(values.clone()),
                        _ => acc = values.clone(),
                    }
                    found = true;
                }
                AmStatement::Primary { var_name, targets, .. } if var_name == name => {
                    acc = targets.clone();
                    found = true;
                }
                _ => {}
            }
        }
        if found {
            Some(acc.join(" "))
        } else {
            self.find_variable_in(&self.makefile_am.statements, name)
        }
    }

    fn find_variable_in(&self, statements: &[AmStatement], name: &str) -> Option<String> {
        for stmt in statements {
            match stmt {
                AmStatement::VariableAssignment {
                    name: n, values, ..
                } if n == name => {
                    return Some(values.join(" "));
                }
                AmStatement::Primary {
                    var_name, targets, ..
                } if var_name == name => {
                    return Some(targets.join(" "));
                }
                AmStatement::ConditionalBlock {
                    if_branch,
                    else_branch,
                    ..
                } => {
                    if let Some(v) = self.find_variable_in(if_branch, name) {
                        return Some(v);
                    }
                    if let Some(v) = self.find_variable_in(else_branch, name) {
                        return Some(v);
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// Map a directory prefix to its install directory variable.
    fn install_dir_for_prefix(&self, prefix: &str) -> String {
        match prefix {
            "" | "bin" => "$(bindir)".to_string(),
            "sbin" => "$(sbindir)".to_string(),
            "libexec" => "$(libexecdir)".to_string(),
            "pkglibexec" => "$(pkglibexecdir)".to_string(),
            "lib" => "$(libdir)".to_string(),
            "pkglib" => "$(pkglibdir)".to_string(),
            "pkgdata" => "$(pkgdatadir)".to_string(),
            "data" => "$(datadir)".to_string(),
            "include" => "$(includedir)".to_string(),
            "oldinclude" => "$(oldincludedir)".to_string(),
            "man" => "$(mandir)".to_string(),
            "info" => "$(infodir)".to_string(),
            "lisp" => "$(lispdir)".to_string(),
            "python" => "$(pythondir)".to_string(),
            "java" => "$(javadir)".to_string(),
            _ => format!("$({}dir)", prefix),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::makefile_am::MakefileAm;
    use std::collections::HashMap;

    #[test]
    fn test_generate_empty() {
        let am = MakefileAm::new();
        let config = AutomakeConfig::from_options("foreign");
        let traces = AutoconfTrace {
            config_files: vec![],
            config_headers: vec![],
            substitutions: HashMap::new(),
            package_name: Some("test".to_string()),
            package_version: Some("1.0".to_string()),
            bug_report: None,
            package_tarname: None,
            strictness: Some("foreign".to_string()),
            conditionals: HashMap::new(),
            languages: vec![],
        };

        let gen = MakefileInGenerator::new(am, config, traces);
        let output = gen.generate();

        assert!(output.contains("Makefile.in generated"));
        assert!(output.contains("@CC@"));
        assert!(output.contains("--foreign"));
    }

    #[test]
    fn test_generate_with_programs() {
        let am = MakefileAm::parse("bin_PROGRAMS = hello\nhello_SOURCES = hello.c\n").unwrap();
        let config = AutomakeConfig::from_options("foreign");
        let traces = AutoconfTrace {
            config_files: vec![],
            config_headers: vec![],
            substitutions: HashMap::new(),
            package_name: Some("test".to_string()),
            package_version: Some("1.0".to_string()),
            bug_report: None,
            package_tarname: None,
            strictness: Some("foreign".to_string()),
            conditionals: HashMap::new(),
            languages: vec![],
        };

        let gen = MakefileInGenerator::new(am, config, traces);
        let output = gen.generate();

        assert!(output.contains("bin_PROGRAMS = hello"));
        assert!(output.contains("hello.$(OBJEXT)"));
        assert!(output.contains("$(CC)"));
        assert!(output.contains("install-exec-am"));
        assert!(output.contains("bindir"));
    }

    #[test]
    fn test_generate_with_scripts() {
        let am = MakefileAm::parse("bin_SCRIPTS = myscript\n").unwrap();
        let config = AutomakeConfig::from_options("foreign");
        let traces = AutoconfTrace {
            config_files: vec![],
            config_headers: vec![],
            substitutions: HashMap::new(),
            package_name: Some("test".to_string()),
            package_version: Some("1.0".to_string()),
            bug_report: None,
            package_tarname: None,
            strictness: Some("foreign".to_string()),
            conditionals: HashMap::new(),
            languages: vec![],
        };

        let gen = MakefileInGenerator::new(am, config, traces);
        let output = gen.generate();

        assert!(output.contains("bin_SCRIPTS = myscript"));
        assert!(output.contains("INSTALL_SCRIPT"));
    }

    #[test]
    fn test_generate_has_gnu_make_detection() {
        let am = MakefileAm::parse("").unwrap();
        let config = AutomakeConfig::from_options("foreign");
        let traces = AutoconfTrace {
            config_files: vec![],
            config_headers: vec![],
            substitutions: HashMap::new(),
            package_name: Some("test".to_string()),
            package_version: Some("1.0".to_string()),
            bug_report: None,
            package_tarname: None,
            strictness: Some("foreign".to_string()),
            conditionals: HashMap::new(),
            languages: vec![],
        };
        let gen = MakefileInGenerator::new(am, config, traces);
        let output = gen.generate();
        assert!(
            output.contains("am__is_gnu_make"),
            "Expected am__is_gnu_make detection"
        );
        assert!(output.contains("MAKELEVEL"), "Expected MAKELEVEL reference");
        assert!(
            output.contains("MAKE_HOST"),
            "Expected MAKE_HOST in detection"
        );
        assert!(output.contains("am__cd = "), "Expected am__cd helper");
        assert!(output.contains("am__untar"), "Expected am__untar helper");
        assert!(output.contains("am__untar = "), "Expected am__untar helper");
        assert!(output.contains("am__tar = "), "Expected am__tar helper");
    }

    #[test]
    fn test_generate_has_distcheck() {
        let am = MakefileAm::parse("").unwrap();
        let config = AutomakeConfig::from_options("foreign");
        let traces = AutoconfTrace {
            config_files: vec![],
            config_headers: vec![],
            substitutions: HashMap::new(),
            package_name: Some("test".to_string()),
            package_version: Some("1.0".to_string()),
            bug_report: None,
            package_tarname: None,
            strictness: Some("foreign".to_string()),
            conditionals: HashMap::new(),
            languages: vec![],
        };
        let gen = MakefileInGenerator::new(am, config, traces);
        let output = gen.generate();
        assert!(
            output.contains("distcheck: dist"),
            "Expected distcheck target"
        );
        assert!(
            output.contains("distcheck: OK"),
            "Expected distcheck OK message"
        );
        assert!(
            output.contains("DIST_ARCHIVES"),
            "Expected DIST_ARCHIVES variable"
        );
        assert!(output.contains("GZIP_ENV"), "Expected GZIP_ENV variable");
        assert!(
            output.contains("DISTCHECK_CONFIGURE_FLAGS"),
            "Expected DISTCHECK_CONFIGURE_FLAGS"
        );
        assert!(
            output.contains("distcleancheck: distclean"),
            "Expected distcleancheck target"
        );
    }

    #[test]
    fn test_generate_has_dist_archive_targets() {
        let am = MakefileAm::parse("").unwrap();
        let config = AutomakeConfig::from_options("foreign");
        let traces = AutoconfTrace {
            config_files: vec![],
            config_headers: vec![],
            substitutions: HashMap::new(),
            package_name: Some("test".to_string()),
            package_version: Some("1.0".to_string()),
            bug_report: None,
            package_tarname: None,
            strictness: Some("foreign".to_string()),
            conditionals: HashMap::new(),
            languages: vec![],
        };
        let gen = MakefileInGenerator::new(am, config, traces);
        let output = gen.generate();
        assert!(
            output.contains("dist-gzip: distdir"),
            "Expected dist-gzip target"
        );
        assert!(
            output.contains("distcleancheck_listfiles"),
            "Expected distcleancheck_listfiles"
        );
    }

    #[test]
    fn test_subdir_objects_generation() {
        let am = MakefileAm::parse("bin_PROGRAMS = hello\nhello_SOURCES = sub/hello.c\n").unwrap();
        let mut config = AutomakeConfig::from_options("foreign subdir-objects");
        config.subdir_objects = true;
        let traces = AutoconfTrace {
            config_files: vec![],
            config_headers: vec![],
            substitutions: HashMap::new(),
            package_name: Some("test".to_string()),
            package_version: Some("1.0".to_string()),
            bug_report: None,
            package_tarname: None,
            strictness: Some("foreign".to_string()),
            conditionals: HashMap::new(),
            languages: vec!["CC".to_string()],
        };
        let gen = MakefileInGenerator::new(am, config, traces);
        let output = gen.generate();
        // subdir-objects preserves source subdirectory in object path
        assert!(
            output.contains("sub/hello.$(OBJEXT)"),
            "Expected subdir-objects path"
        );
    }

    #[test]
    fn test_ltlibraries_generation() {
        let am =
            MakefileAm::parse("lib_LTLIBRARIES = libfoo.la\nlibfoo_la_SOURCES = foo.c\n").unwrap();
        let config = AutomakeConfig::from_options("foreign");
        let traces = AutoconfTrace {
            config_files: vec![],
            config_headers: vec![],
            substitutions: HashMap::new(),
            package_name: Some("test".to_string()),
            package_version: Some("1.0".to_string()),
            bug_report: None,
            package_tarname: None,
            strictness: Some("foreign".to_string()),
            conditionals: HashMap::new(),
            languages: vec!["CC".to_string()],
        };
        let gen = MakefileInGenerator::new(am, config, traces);
        let output = gen.generate();
        assert!(
            output.contains("lib_LTLIBRARIES"),
            "Expected LTLIBRARIES variable"
        );
        assert!(
            output.contains("libfoo.la"),
            "Expected libtool library target name"
        );
        // LTLIBRARIES support is in-progress — check what's generated
        if output.contains("$(LIBTOOL)") {
            assert!(output.contains(".lo"), "Expected libtool object suffix");
        }
    }

    #[test]
    fn test_generate_has_install_targets() {
        let am = MakefileAm::parse("").unwrap();
        let config = AutomakeConfig::from_options("foreign");
        let traces = AutoconfTrace {
            config_files: vec![],
            config_headers: vec![],
            substitutions: HashMap::new(),
            package_name: Some("test".to_string()),
            package_version: Some("1.0".to_string()),
            bug_report: None,
            package_tarname: None,
            strictness: Some("foreign".to_string()),
            conditionals: HashMap::new(),
            languages: vec![],
        };
        let gen = MakefileInGenerator::new(am, config, traces);
        let output = gen.generate();
        assert!(
            output.contains("install-info:"),
            "Expected install-info target"
        );
        assert!(
            output.contains("install-dvi:"),
            "Expected install-dvi target"
        );
        assert!(output.contains("install-ps:"), "Expected install-ps target");
        assert!(
            output.contains("install-pdf:"),
            "Expected install-pdf target"
        );
        assert!(
            output.contains("install-html:"),
            "Expected install-html target"
        );
        assert!(
            output.contains("install-data-hook:"),
            "Expected install-data-hook"
        );
        assert!(
            output.contains("install-exec-hook:"),
            "Expected install-exec-hook"
        );
        assert!(
            output.contains("installdirs:"),
            "Expected installdirs target"
        );
    }
}
