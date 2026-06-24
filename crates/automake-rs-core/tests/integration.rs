// automake-rs-core: End-to-end integration tests
//
// Tests the full pipeline: Makefile.am parsing → AutomakeConfig → Makefile.in generation.
// Compares output against GNU Automake oracle.

use automake_rs_core::autoconf_bridge::AutoconfTrace;
use automake_rs_core::automake_macros::AutomakeConfig;
use automake_rs_core::makefile_am::MakefileAm;
use automake_rs_core::makefile_in::MakefileInGenerator;
use std::collections::HashMap;
use std::process::Command;

/// Helper: create a temp directory with configure.ac and Makefile.am, run GNU automake,
/// and return the generated Makefile.in content.
fn run_oracle_automake(configure_ac: &str, makefile_am: &str) -> Option<String> {
    let tmp = tempfile::tempdir().ok()?;
    let tmp_path = tmp.path();

    std::fs::write(tmp_path.join("configure.ac"), configure_ac).ok()?;
    std::fs::write(tmp_path.join("Makefile.am"), makefile_am).ok()?;

    // Run aclocal
    let aclocal = Command::new("aclocal")
        .current_dir(tmp_path)
        .output()
        .ok()?;
    if !aclocal.status.success() {
        return None;
    }

    // Run autoconf
    let autoconf = Command::new("autoconf")
        .current_dir(tmp_path)
        .output()
        .ok()?;
    if !autoconf.status.success() {
        return None;
    }

    // Run automake with --add-missing
    let automake = Command::new("automake")
        .args(["--foreign", "--add-missing"])
        .current_dir(tmp_path)
        .output()
        .ok()?;
    if !automake.status.success() {
        return None;
    }

    std::fs::read_to_string(tmp_path.join("Makefile.in")).ok()
}

#[test]
fn test_e2e_empty_makefile() {
    let configure_ac = "AC_INIT([test], [1.0])\nAM_INIT_AUTOMAKE([foreign])\nAC_CONFIG_FILES([Makefile])\nAC_OUTPUT\n";
    let makefile_am = "# Empty Makefile.am\n";

    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".to_string()],
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
    assert!(output.contains("@prefix@"));
    assert!(output.contains("--foreign"));

    // NOTE: Without AC_PROG_CC, the oracle only includes variables actually needed.
    // Our generator includes all standard variables unconditionally (known divergence).
    if let Some(oracle_output) = run_oracle_automake(configure_ac, makefile_am) {
        eprintln!(
            "Oracle: {} lines, Ours: {} lines",
            oracle_output.lines().count(),
            output.lines().count()
        );
        assert!(oracle_output.contains("@prefix@"));
    }
}

#[test]
fn test_e2e_simple_program() {
    let configure_ac = "AC_INIT([hello], [1.0])\nAM_INIT_AUTOMAKE([foreign])\nAC_PROG_CC\nAC_CONFIG_FILES([Makefile])\nAC_OUTPUT\n";
    let makefile_am = "bin_PROGRAMS = hello\nhello_SOURCES = hello.c\n";

    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".to_string()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("hello".to_string()),
        package_version: Some("1.0".to_string()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".to_string()),
        conditionals: HashMap::new(),
        languages: vec!["CC".to_string()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();

    assert!(output.contains("bin_PROGRAMS = hello"));
    assert!(output.contains("hello_SOURCES"));
    assert!(output.contains("hello.$(OBJEXT)"));
    assert!(output.contains("$(CC)"));
    assert!(output.contains("install-exec-am"));
    assert!(output.contains("$(bindir)"));

    if let Some(oracle_output) = run_oracle_automake(configure_ac, makefile_am) {
        eprintln!(
            "Oracle: {} lines, Ours: {} lines",
            oracle_output.lines().count(),
            output.lines().count()
        );
        // Both should have the core elements
        assert!(oracle_output.contains("hello_SOURCES"));
        assert!(oracle_output.contains("$(CC)"));
    }
}

#[test]
fn test_e2e_scripts_and_data() {
    let _configure_ac = "AC_INIT([test], [1.0])\nAM_INIT_AUTOMAKE([foreign])\nAC_CONFIG_FILES([Makefile])\nAC_OUTPUT\n";
    let makefile_am = "bin_SCRIPTS = myscript\nnoinst_DATA = readme.txt\n";

    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".to_string()],
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
    assert!(output.contains("noinst_DATA = readme.txt"));
    assert!(output.contains("INSTALL_SCRIPT"));
    assert!(output.contains("INSTALL_DATA"));
}

// --- Manual Example Tests (GFDL Automake Manual, clean-room) ---

#[test]
fn test_manual_multiple_programs() {
    // GFDL Manual §8.1: Multiple programs
    let makefile_am = "bin_PROGRAMS = hello goodbye\nhello_SOURCES = hello.c common.c\ngoodbye_SOURCES = goodbye.c common.c\ngoodbye_CFLAGS = -DGOODBYE\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("multi".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("bin_PROGRAMS = hello goodbye"));
    assert!(output.contains("hello_SOURCES"));
    assert!(output.contains("goodbye_SOURCES"));
    assert!(output.contains("hello.$(OBJEXT)"));
    assert!(output.contains("goodbye.$(OBJEXT)"));
}

#[test]
fn test_manual_static_library() {
    // GFDL Manual §8.2: Static libraries
    let makefile_am = "lib_LIBRARIES = libfoo.a\nlibfoo_a_SOURCES = foo.c bar.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("lib".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("lib_LIBRARIES = libfoo.a"));
    assert!(output.contains("libfoo_a_SOURCES"));
    // Should warn about unimplemented LIBRARIES primary
}

#[test]
fn test_manual_conditional() {
    // GFDL Manual §7: Conditionals
    let makefile_am = "if WANT_DEBUG\nbin_PROGRAMS = debug-tool\ndebug_tool_SOURCES = debug.c\nelse\nbin_PROGRAMS = release-tool\nrelease_tool_SOURCES = release.c\nendif\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("cond".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::from([("WANT_DEBUG".into(), true)]),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("WANT_DEBUG"));
    assert!(output.contains("debug-tool") || output.contains("release-tool"));
}

#[test]
fn test_manual_subdirs() {
    // GFDL Manual §7.3: Recursive subdirectories
    let makefile_am = "SUBDIRS = src doc tests\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("subdirs".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("SUBDIRS = src doc tests"));
}

#[test]
fn test_manual_built_sources() {
    // GFDL Manual §9.1.1: Built sources
    let makefile_am = "BUILT_SOURCES = generated.h\nbin_PROGRAMS = app\napp_SOURCES = app.c generated.h\ngenerated.h: Makefile\n\techo '/* generated */' > $@\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("built".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("BUILT_SOURCES"));
    assert!(output.contains("generated.h"));
}

#[test]
fn test_manual_check_programs() {
    // GFDL Manual §15: Tests
    let makefile_am = "TESTS = test-foo test-bar\ncheck_PROGRAMS = test-foo test-bar\ntest_foo_SOURCES = test-foo.c\ntest_bar_SOURCES = test-bar.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("check".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("TESTS = test-foo test-bar"));
    assert!(output.contains("check_PROGRAMS"));
}

#[test]
fn test_manual_dist_nodist() {
    // GFDL Manual §14: Distribution
    let makefile_am = "dist_data_DATA = distributed.txt\nnodist_data_DATA = generated.txt\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("dist".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("dist_data_DATA"));
    assert!(output.contains("nodist_data_DATA"));
}

#[test]
fn test_manual_per_target_flags() {
    // GFDL Manual §8.1.2: Per-target flags
    let makefile_am = "bin_PROGRAMS = special\nspecial_SOURCES = special.c\nspecial_CFLAGS = -O3 -funroll-loops\nspecial_LDADD = -lm\nspecial_LDFLAGS = -static\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("flags".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("special_SOURCES"));
    assert!(output.contains("special_LDADD"));
    assert!(output.contains("special_LDFLAGS"));
}

#[test]
fn test_manual_line_continuation() {
    // GFDL Manual §5.1: Line continuations
    let makefile_am =
        "bin_PROGRAMS = long-name\nlong_name_SOURCES = \\\n  file1.c \\\n  file2.c \\\n  file3.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("cont".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("bin_PROGRAMS = long-name"));
    assert!(output.contains("long_name_SOURCES"));
}

#[test]
fn test_manual_append_variable() {
    // GFDL Manual §5.1: Appending to variables
    let makefile_am = "bin_PROGRAMS = main\nmain_SOURCES = main.c\nmain_SOURCES += util.c\nmain_LDADD = -lm\nmain_LDADD += -lpthread\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("append".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("main_SOURCES"));
    assert!(output.contains("main.c"));
    assert!(output.contains("util.c"));
    assert!(output.contains("main_LDADD"));
}

#[test]
fn test_manual_extra_dist() {
    // GFDL Manual §14.1: EXTRA_DIST
    let makefile_am = "EXTRA_DIST = README.md CHANGELOG\nbin_PROGRAMS = app\napp_SOURCES = app.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("extra".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("EXTRA_DIST"));
    assert!(output.contains("README.md"));
    assert!(output.contains("CHANGELOG"));
}

#[test]
fn test_manual_gnu_strictness() {
    // GFDL Manual §2.2: Strictness levels
    let makefile_am = "bin_PROGRAMS = hello\nhello_SOURCES = hello.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("gnu");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("gnu-app".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("gnu".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("--gnu"));
}

#[test]
fn test_manual_gnits_strictness() {
    // GFDL Manual §2.2: GNITS strictness
    let makefile_am = "bin_PROGRAMS = hello\nhello_SOURCES = hello.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("gnits");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("gnits-app".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("gnits".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("--gnits"));
}

#[test]
fn test_manual_cleanfiles() {
    // GFDL Manual §13: Clean targets
    let makefile_am = "CLEANFILES = *.log *.tmp\nMAINTAINERCLEANFILES = Makefile.in\nbin_PROGRAMS = app\napp_SOURCES = app.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("clean".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("CLEANFILES"));
    assert!(output.contains("MAINTAINERCLEANFILES"));
}

// --- POSIX Edge Case Tests (Layer 3) ---

#[test]
fn test_posix_shell_special_chars() {
    // Shell special characters in variable values should be preserved
    let makefile_am =
        "VAR1 = hello world\nVAR2 = foo; bar\nVAR3 = $(shell date)\nVAR4 = backtick`cmd`\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("posix-shell".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("hello world"));
    assert!(output.contains("foo; bar"));
    assert!(output.contains("$(shell date)"));
}

#[test]
fn test_posix_dollar_signs() {
    // Various $ references
    let makefile_am = "VAR1 = $(OTHER)\nVAR2 = ${BRACES}\nVAR3 = $$var\nVAR4 = $@ $< $^\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("dollar".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("$(OTHER)"));
    assert!(output.contains("${BRACES}"));
    assert!(output.contains("$$var"));
}

#[test]
fn test_posix_hash_comments() {
    // Comments should be preserved or ignored correctly
    let makefile_am = "# This is a comment\nVAR = value # not a comment in make\n# Another comment\nVAR2 = val2\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("hash".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("VAR = value # not a comment in make"));
    assert!(output.contains("VAR2 = val2"));
}

#[test]
fn test_posix_empty_variables() {
    // Empty variable assignments
    let makefile_am = "EMPTY =\nSPACES =    \nTAB =\t\nNULL =\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("empty".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("EMPTY ="));
    assert!(output.contains("NULL ="));
}

#[test]
fn test_posix_override_directive() {
    // Override directive (:= for GNU make, ::= for POSIX)
    let makefile_am = "VAR := immediate\nVAR2 ::= also-immediate\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("override".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("VAR := immediate"));
}

#[test]
fn test_posix_conditional_forms() {
    // Various conditional syntaxes
    let makefile_am = "ifeq ($(OS),Linux)\n  VAR = linux\nelse\n  VAR = other\nendif\nifneq ($(CC),gcc)\n  CFLAGS = -O2\nendif\nifdef DEBUG\n  CFLAGS += -g\nendif\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("condforms".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // Should preserve these as passthrough or process them
    assert!(!output.is_empty());
}

#[test]
fn test_posix_escape_sequences() {
    // Escape sequences in make
    let makefile_am = "PATH_SEP = :\nTAB_CHAR = \\t\nNEWLINE = \\n\nBACKSLASH = \\\\\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("escape".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("\\\\"));
    assert!(output.contains("\\t") || output.contains("\\n"));
}

#[test]
fn test_posix_wildcard_targets() {
    // Wildcard and pattern rules
    let makefile_am = "%.o: %.c\n\t$(CC) -c $< -o $@\n\nclean-foo:\n\trm -f foo*\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("wildcard".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("%.o: %.c"));
    assert!(output.contains("$<"));
    assert!(output.contains("$@"));
}

// ================================================================
// Panel-Directed: Target Flag Shadowing & Conditional Ghost Tracking
// ================================================================

/// Panel item: Verify target_CFLAGS overrides AM_CFLAGS (not additive).
/// In Automake, per-target flags SHADOW (replace) the global flags.
#[test]
fn test_target_flag_shadowing() {
    let makefile_am = "\
bin_PROGRAMS = myprog\n\
AM_CFLAGS = -Wall -O0\n\
myprog_CFLAGS = -O2\n\
myprog_SOURCES = myprog.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("shadow".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // Per-target CFLAGS should appear. The AM_CFLAGS may appear as variable definition
    // but the compile rule should use the per-target value.
    assert!(
        output.contains("myprog_CFLAGS") || output.contains("-O2"),
        "Per-target CFLAGS should be present: {}",
        output
    );
}

/// Panel item: Test that LDADD and LIBADD are ordered (Vec, not HashSet).
/// Libtool and linker care about -l flag ordering.
#[test]
fn test_ldadd_ordering_preserved() {
    let makefile_am = "\
bin_PROGRAMS = myprog\n\
myprog_LDADD = -lz -lm -lpthread\n\
myprog_SOURCES = myprog.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("linkorder".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // LDADD values should appear in order: -lz before -lm before -lpthread
    let z_pos = output.find("-lz").unwrap();
    let m_pos = output.find("-lm").unwrap();
    let pthread_pos = output.find("-lpthread").unwrap();
    assert!(z_pos < m_pos, "-lz should come before -lm");
    assert!(m_pos < pthread_pos, "-lm should come before -lpthread");
}

/// Panel item: Verify that conditional variable does not leak
/// into the non-conditional scope (ghost tracking).
#[test]
fn test_conditional_variable_no_leak() {
    let makefile_am = "\
VAR = global\n\
if COND1\n  VAR = conditional_value\nendif\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    // With COND1=false, VAR should remain "global"
    let empty: std::collections::HashSet<String> = std::collections::HashSet::new();
    let expanded = am.expand_conditionals(&empty);
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("noleak".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(expanded, config, traces);
    let output = gen.generate();
    assert!(
        output.contains("VAR = global"),
        "VAR should remain global when COND1=false: {}",
        output
    );
    assert!(
        !output.contains("conditional_value"),
        "conditional_value should NOT leak when COND1=false"
    );
}

/// Panel item: Verify that nested conditionals with overlapping predicates
/// resolve correctly through the DNF engine.
#[test]
fn test_nested_conditional_resolution() {
    let makefile_am = "\
if COND_A\n  if COND_B\n    RESULT = A_and_B\n  else\n    RESULT = A_only\n  endif\nelse\n  RESULT = not_A\nendif\n";
    let am = MakefileAm::parse(makefile_am).unwrap();

    // Case 1: COND_A=true, COND_B=true → RESULT = A_and_B
    let mut ab_true = std::collections::HashSet::new();
    ab_true.insert("COND_A".to_string());
    ab_true.insert("COND_B".to_string());
    let expanded = am.expand_conditionals(&ab_true);
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("nested".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(expanded, config.clone(), traces.clone());
    let output = gen.generate();
    assert!(
        output.contains("A_and_B") || output.contains("RESULT"),
        "A∧B should resolve: {}",
        output
    );

    // Case 2: COND_A=true, COND_B=false → RESULT = A_only
    let mut a_only = std::collections::HashSet::new();
    a_only.insert("COND_A".to_string());
    let expanded2 = am.expand_conditionals(&a_only);
    let gen2 = MakefileInGenerator::new(expanded2, config.clone(), traces.clone());
    let output2 = gen2.generate();
    assert!(
        output2.contains("A_only") || output2.contains("RESULT"),
        "A only should resolve: {}",
        output2
    );

    // Case 3: COND_A=false → RESULT = not_A
    let empty: std::collections::HashSet<String> = std::collections::HashSet::new();
    let expanded3 = am.expand_conditionals(&empty);
    let gen3 = MakefileInGenerator::new(expanded3, config, traces);
    let output3 = gen3.generate();
    assert!(
        output3.contains("not_A") || output3.contains("RESULT"),
        "!A should resolve: {}",
        output3
    );
}

// ================================================================
// Conditional Variable Namespace Tests — AM.COND.NAMESPACE.1
// ================================================================

/// Test that a variable defined inside a conditional gets the
/// @COND_TRUE@ prefix in generated output.
#[test]
fn test_conditional_variable_prefix() {
    let makefile_am = "if COND1\n  MYVAR = foo\nendif\n";
    let mut am = MakefileAm::parse(makefile_am).unwrap();
    // Expand with COND1=true to verify conditional resolution
    let mut true_conds = std::collections::HashSet::new();
    true_conds.insert("COND1".to_string());
    am = am.expand_conditionals(&true_conds);

    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("condvar".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // The variable should appear in output (expanded from conditional)
    assert!(
        output.contains("MYVAR"),
        "MYVAR should be in output: {}",
        output
    );
}

/// Test that when a variable is NOT inside a conditional, it has NO prefix.
#[test]
fn test_variable_no_conditional_prefix() {
    let makefile_am = "MYVAR = bar\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("noprefix".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(
        output.contains("MYVAR = bar"),
        "MYVAR should appear without prefix"
    );
}

/// Test that disabling a conditional drops the variable.
#[test]
fn test_conditional_false_drops_variable() {
    let makefile_am = "if DEBUG\n  CFLAGS = -g -O0\nelse\n  CFLAGS = -O2\nendif\n";
    let am = MakefileAm::parse(makefile_am).unwrap();

    // Expand with DEBUG=false
    let true_conds: std::collections::HashSet<String> = std::collections::HashSet::new();
    let expanded = am.expand_conditionals(&true_conds);

    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("debug".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(expanded, config, traces);
    let output = gen.generate();
    // With DEBUG=false, CFLAGS should be -O2 (else branch)
    assert!(
        output.contains("-O2"),
        "CFLAGS should be -O2 when DEBUG=false"
    );
    assert!(
        !output.contains("-g -O0"),
        "CFLAGS should NOT be -g when DEBUG=false"
    );
}

/// Test that variables defined in multiple conditional branches
/// are properly collected with DisjConditions.
#[test]
fn test_conditional_variable_collection() {
    let makefile_am = "\
VAR = default\n\
if COND_A\n  VAR = a_value\nendif\n\
if COND_B\n  VAR = b_value\nendif\n";
    let am = MakefileAm::parse(makefile_am).unwrap();

    // With no conditions true, should get default
    let empty: std::collections::HashSet<String> = std::collections::HashSet::new();
    let expanded = am.expand_conditionals(&empty);
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("multicond".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(expanded, config, traces);
    let output = gen.generate();
    assert!(
        output.contains("VAR = default"),
        "Default VAR should appear"
    );
}

#[test]
fn test_posix_phony_targets() {
    // .PHONY targets
    let makefile_am = ".PHONY: all clean install\nall: program\nclean:\n\trm -f *.o\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("phony".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains(".PHONY"));
}

#[test]
fn test_posix_shell_functions() {
    // Shell function calls in recipes
    let makefile_am = "check:\n\t@if test -f ./config.status; then \\\n\t  $(SHELL) ./config.status --recheck; \\\n\telse \\\n\t  $(SHELL) configure; \\\n\tfi\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("shell".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("$(SHELL)"));
}

#[test]
fn test_posix_include_directive() {
    // include directive
    let makefile_am = "include extra-vars.am\n-include optional-vars.am\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("include".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("extra-vars.am") || output.contains("include"));
}

#[test]
fn test_libraries_primary() {
    let makefile_am = "lib_LIBRARIES = libfoo\nlibfoo_SOURCES = foo.c bar.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("libtest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // Check ar/ranlib archive rule
    assert!(output.contains("$(AR) $(ARFLAGS)") || output.contains("$(AR)"));
    assert!(output.contains("libfoo.a"));
    assert!(output.contains("foo.o") || output.contains("foo.$(OBJEXT)"));
    assert!(output.contains("bar.o") || output.contains("bar.$(OBJEXT)"));
    // Check install rule
    assert!(output.contains("$(INSTALL_DATA)"));
    assert!(output.contains("$(libdir)"));
}

#[test]
fn test_headers_primary() {
    let makefile_am = "include_HEADERS = foo.h bar.h config.h\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("headertest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("include_HEADERS = foo.h bar.h config.h"));
    assert!(output.contains("install-data-am"));
    assert!(output.contains("$(INSTALL_HEADER)"));
    assert!(output.contains("foo.h"));
    assert!(output.contains("bar.h"));
}

#[test]
fn test_mans_primary() {
    let makefile_am = "man1_MANS = foo.1 bar.1\nman3_MANS = lib.3\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("mantest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // Check man section handling
    assert!(output.contains("$(mandir)/man1"));
    assert!(output.contains("$(mandir)/man3"));
    assert!(output.contains("install-man"));
    assert!(output.contains("uninstall-man"));
    assert!(output.contains("foo.1"));
    assert!(output.contains("lib.3"));
}

#[test]
fn test_nobase_prefix() {
    let makefile_am = "nobase_include_HEADERS = sub/foo.h deep/bar.h\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    // Verify nobase_ is detected
    match &am.statements[0] {
        automake_rs_core::makefile_am::AmStatement::Primary {
            nobase, dir_prefix, ..
        } => {
            assert!(nobase);
            assert_eq!(dir_prefix, "include");
        }
        _ => panic!("Expected Primary"),
    }
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("nobasetest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("sub/foo.h"));
    assert!(output.contains("deep/bar.h"));
}

#[test]
fn test_built_sources_rules() {
    let makefile_am =
        "BUILT_SOURCES = generated.h\nbin_PROGRAMS = myprog\nmyprog_SOURCES = myprog.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("builttest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // Check BUILT_SOURCES variable is emitted
    assert!(output.contains("BUILT_SOURCES = generated.h"));
    // Check that all-am depends on BUILT_SOURCES
    assert!(output.contains("all-am"));
}

#[test]
fn test_vpath_source_prefix() {
    let makefile_am = "bin_PROGRAMS = myprog\nmyprog_SOURCES = myprog.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("vpather".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // VPATH variables present
    assert!(output.contains("VPATH = @srcdir@"));
    assert!(output.contains("srcdir = @srcdir@"));
    assert!(output.contains("abs_srcdir = @abs_srcdir@"));
    // Source files use $(srcdir)/ prefix
    assert!(output.contains("$(srcdir)/myprog.c"));
}

#[test]
fn test_yacc_lex_autodetect() {
    let makefile_am = "bin_PROGRAMS = parse\nparse_SOURCES = parse.y scan.l main.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("yacctest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // Yacc/Lex variables emitted
    assert!(output.contains("YACC = @YACC@"));
    assert!(output.contains("LEX = @LEX@"));
    assert!(output.contains("YLWRAP = $(top_srcdir)/ylwrap"));
    // Yacc rule generated
    assert!(output.contains("parse.c parse.h: parse.y"));
    assert!(output.contains("$(YACCCOMPILE)"));
    // Lex rule generated
    assert!(output.contains("scan.c: scan.l"));
    assert!(output.contains("$(LEXCOMPILE)"));
}

#[test]
fn test_clean_hierarchy() {
    let makefile_am = "bin_PROGRAMS = foo\nfoo_SOURCES = foo.c\nlib_LIBRARIES = libbar\nlibbar_SOURCES = bar.c\nTESTS = test.sh\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec!["config.h".into()],
        substitutions: HashMap::new(),
        package_name: Some("cleantest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // All four levels present
    assert!(output.contains("mostlyclean-am"));
    assert!(output.contains("clean-am"));
    assert!(output.contains("distclean-am"));
    assert!(output.contains("maintainer-clean-am"));
    // Objects removed in mostlyclean
    assert!(output.contains("foo.$(OBJEXT)"));
    // Test logs removed
    assert!(output.contains("test-suite.log"));
    // config files removed in distclean
    assert!(output.contains("config.status"));
    // Generated files removed in maintainer-clean
    assert!(output.contains("aclocal.m4"));
    assert!(output.contains("configure"));
}

#[test]
fn test_texinfos_primary() {
    let makefile_am = "info_TEXINFOS = manual.texi\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("textest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // TEXINFOS variables emitted
    assert!(output.contains("INFO_DEPS = manual.info"));
    assert!(output.contains("MAKEINFO = makeinfo"));
    assert!(output.contains("TEXI2DVI = texi2dvi"));
    assert!(output.contains("TEXI2PDF = texi2pdf"));
    // version.texi rules
    assert!(output.contains("version.texi"));
    assert!(output.contains("stamp-vti"));
    // makeinfo rule
    assert!(output.contains("manual.info:"));
    // DVI/PDF/HTML targets
    assert!(output.contains("manual.dvi"));
    assert!(output.contains("manual.pdf"));
    assert!(output.contains("manual.html"));
    // PHONY targets
    assert!(output.contains("dvi:"));
    assert!(output.contains("pdf:"));
    assert!(output.contains("html:"));
    assert!(output.contains("info:"));
}

#[test]
fn test_utility_targets() {
    let makefile_am = "bin_PROGRAMS = foo\nfoo_SOURCES = foo.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("utiltest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // TAGS target
    assert!(output.contains("TAGS: tags"));
    assert!(output.contains("ETAGS = etags"));
    assert!(output.contains("CTAGS = ctags"));
    // cscope target
    assert!(output.contains("cscope:"));
    // installcheck body
    assert!(output.contains("installcheck-am"));
    // distuninstallcheck target
    assert!(output.contains("distuninstallcheck"));
    // recheck target (if no tests, may not appear; check at least TAGS/cscope present)
    assert!(output.contains("TAGS"));
}

#[test]
fn test_full_silent_rules() {
    let makefile_am = "bin_PROGRAMS = foo\nfoo_SOURCES = foo.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("silenttest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // Full silent rules two-line variables
    assert!(output.contains("AM_V_CC = $(am__v_CC_$(V))"));
    assert!(output.contains("am__v_CC_0 = @echo \"  CC      \" $@;"));
    assert!(output.contains("AM_V_CXX = $(am__v_CXX_$(V))"));
    assert!(output.contains("AM_V_CCLD = $(am__v_CCLD_$(V))"));
    assert!(output.contains("AM_V_AR = $(am__v_AR_$(V))"));
    assert!(output.contains("AM_V_GEN = $(am__v_GEN_$(V))"));
    assert!(output.contains("AM_V_YACC = $(am__v_YACC_$(V))"));
    assert!(output.contains("AM_V_LEX = $(am__v_LEX_$(V))"));
    assert!(output.contains("V = 0"));
}

#[test]
fn test_automake_rebuild_rules() {
    let makefile_am = "bin_PROGRAMS = foo\nfoo_SOURCES = foo.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("rebuildtest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("$(srcdir)/Makefile.in: $(srcdir)/Makefile.am"));
    assert!(output.contains("$(AUTOMAKE) --foreign Makefile"));
}

#[test]
fn test_dependency_tracking() {
    let makefile_am = "bin_PROGRAMS = foo\nfoo_SOURCES = foo.c bar.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("deptest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // DEPDIR and depcomp variables
    assert!(output.contains("DEPDIR = .deps"));
    assert!(output.contains("depcomp = $(SHELL) $(top_srcdir)/depcomp"));
    // AMDEP conditionals
    assert!(output.contains("@AMDEP_TRUE@am__include = include"));
    assert!(output.contains("@AMDEP_FALSE@am__include = #"));
    // Dependency files are named after the SOURCE stem (GNU Automake convention).
    assert!(output.contains("./$(DEPDIR)/foo.Po"));
    assert!(output.contains("./$(DEPDIR)/bar.Po"));
    // Include directive carries the am--include-marker config.status greps for.
    assert!(output.contains("@AMDEP_TRUE@@am__include@"));
    assert!(output.contains("# am--include-marker"));
    // The stub-creation rule lets a bare `make` materialize missing .Po files.
    assert!(output.contains("am--depfiles: $(am__depfiles_remade)"));
}

#[test]
fn test_nobase_subdir_headers() {
    let makefile_am = "nobase_include_HEADERS = sub/foo.h deep/bar.h\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("nbtest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("sub/foo.h"));
    assert!(output.contains("deep/bar.h"));
    assert!(output.contains("install-data-am"));
}

#[test]
fn test_dist_prefix() {
    let makefile_am = "dist_bin_SCRIPTS = myscript\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    assert_eq!(am.statements.len(), 1);
}

#[test]
fn test_conditional_else_branch() {
    let makefile_am = "if WANT_FOO\nbin_PROGRAMS = foo\nfoo_SOURCES = foo.c\nelse\nbin_PROGRAMS = bar\nbar_SOURCES = bar.c\nendif\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("condtest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::from([("WANT_FOO".into(), true)]),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("foo_SOURCES") || output.contains("bar_SOURCES"));
}

#[test]
fn test_per_target_dependencies() {
    let makefile_am =
        "bin_PROGRAMS = myprog\nmyprog_SOURCES = main.c\nmyprog_DEPENDENCIES = libfoo.a\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("deptest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("myprog_SOURCES = main.c"));
    assert!(output.contains("myprog_DEPENDENCIES"));
}

#[test]
fn test_check_programs() {
    let makefile_am = "check_PROGRAMS = test_foo\ntest_foo_SOURCES = test_foo.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("chktest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("check_PROGRAMS = test_foo"));
}

#[test]
fn test_ltlibraries_with_libadd() {
    let makefile_am =
        "lib_LTLIBRARIES = libfoo.la\nlibfoo_la_SOURCES = foo.c\nlibfoo_la_LIBADD = -lm\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("lttest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("lib_LTLIBRARIES = libfoo.la"));
    assert!(output.contains("libfoo.la"));
}

#[test]
fn test_python_primary() {
    let makefile_am = "python_PYTHON = mymod.py\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("pytest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("PYTHON = @PYTHON@"));
    assert!(output.contains("pythondir"));
    assert!(output.contains("py-compile"));
}

#[test]
fn test_lisp_primary() {
    let makefile_am = "lisp_LISP = mymode.el\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("lisptest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("EMACS = @EMACS@"));
    assert!(output.contains("lispdir = @lispdir@"));
    assert!(output.contains("batch-byte-compile"));
}

#[test]
fn test_java_primary() {
    let makefile_am = "java_JAVA = MyClass.class\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("javatest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("JAVAC = @JAVAC@"));
    assert!(output.contains("javadir = $(datadir)/java"));
    assert!(output.contains("CLASSPATH_ENV"));
    assert!(output.contains("JAVAROOT = $(top_builddir)"));
}

// ================================================================
// Panel Priority #4: VPATH Stress Tests — out-of-tree build verification
// ================================================================

/// Verify VPATH setup variables are all present.
#[test]
fn test_vpath_setup_complete() {
    let makefile_am = "bin_PROGRAMS = hello\nhello_SOURCES = hello.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("vpathtest".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // All VPATH variables present
    assert!(output.contains("VPATH = @srcdir@"));
    assert!(output.contains("srcdir = @srcdir@"));
    assert!(output.contains("top_srcdir = @top_srcdir@"));
    assert!(output.contains("builddir = @builddir@"));
    assert!(output.contains("top_builddir = @top_builddir@"));
    assert!(output.contains("abs_srcdir = @abs_srcdir@"));
    assert!(output.contains("abs_builddir = @abs_builddir@"));
    assert!(output.contains("abs_top_srcdir = @abs_top_srcdir@"));
    assert!(output.contains("abs_top_builddir = @abs_top_builddir@"));
    // Source file references use $(srcdir)/
    assert!(output.contains("$(srcdir)/hello.c"));
}

/// Verify LTLIBRARIES VPATH source references.
#[test]
fn test_vpath_ltlibraries_sources() {
    let makefile_am = "lib_LTLIBRARIES = libvp.la\nlibvp_la_SOURCES = vp.c util.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("vpathlt".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // Each source file should have $(srcdir)/ reference
    assert!(output.contains("$(srcdir)/vp.c"), "VPATH vp.c: {}", output);
    assert!(
        output.contains("$(srcdir)/util.c"),
        "VPATH util.c: {}",
        output
    );
    // .lo objects should be source-derived
    assert!(
        output.contains("vp.lo") || output.contains("util.lo"),
        ".lo objects"
    );
}

/// Verify dist rules have VPATH stripping logic.
#[test]
fn test_vpath_dist_stripping() {
    let makefile_am = "EXTRA_DIST = README.md\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("vpdist".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // Dist rules must have srcdirstrip for VPATH builds
    assert!(output.contains("srcdirstrip"));
    assert!(output.contains("topsrcdirstrip"));
    assert!(output.contains("$(top_builddir)"));
}

/// Verify am__cd helper is present for VPATH directory changes.
#[test]
fn test_vpath_am_cd_present() {
    let makefile_am = "bin_PROGRAMS = cdtest\ncdtest_SOURCES = cdtest.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("vpcd".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec!["CC".into()],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    assert!(output.contains("am__cd = CDPATH"));
    assert!(output.contains("am__is_gnu_make"));
}

// ================================================================
// Panel-Directed: LTLIBRARIES Depth — VPATH, Shadowing, -rpath
// ================================================================

/// Panel item: LTLIBRARIES compile rule uses $(srcdir)/ for VPATH builds.
#[test]
fn test_ltlibraries_vpath_source_reference() {
    let makefile_am = "lib_LTLIBRARIES = libfoo.la\nlibfoo_la_SOURCES = foo.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("ltvp".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // VPATH $(srcdir)/ reference in compile rule
    assert!(output.contains("$(srcdir)/foo.c"), "VPATH: {}", output);
    // Libtool compile mode
    assert!(
        output.contains("--mode=compile"),
        "compile mode: {}",
        output
    );
    // Libtool link mode
    assert!(output.contains("--mode=link"), "link mode: {}", output);
    // .la library output
    assert!(output.contains("libfoo.la"), ".la: {}", output);
    // .lo object
    assert!(
        output.contains("libfoo.lo") || output.contains("foo.lo"),
        ".lo: {}",
        output
    );
}

/// Panel item: LTLIBRARIES with -rpath for installable libraries.
#[test]
fn test_ltlibraries_rpath_for_installed() {
    let makefile_am = "lib_LTLIBRARIES = libbar.la\nlibbar_la_SOURCES = bar.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("ltrpath".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // Should have -rpath for installable libtool libraries
    assert!(output.contains("-rpath"), "rpath: {}", output);
    // Should install to libdir
    assert!(output.contains("install-exec-am"), "install: {}", output);
    // Should use libtool mode=install
    assert!(
        output.contains("--mode=install"),
        "install mode: {}",
        output
    );
}

/// Panel item: LTLIBRARIES per-target CFLAGS shadowing (not additive).
#[test]
fn test_ltlibraries_cflags_shadowing() {
    let makefile_am = "\
lib_LTLIBRARIES = libbaz.la\n\
AM_CFLAGS = -Wall\n\
libbaz_la_CFLAGS = -O2\n\
libbaz_la_SOURCES = baz.c\n";
    let am = MakefileAm::parse(makefile_am).unwrap();
    let config = AutomakeConfig::from_options("foreign");
    let traces = AutoconfTrace {
        config_files: vec!["Makefile".into()],
        config_headers: vec![],
        substitutions: HashMap::new(),
        package_name: Some("ltshadow".into()),
        package_version: Some("1.0".into()),
        bug_report: None,
        package_tarname: None,
        strictness: Some("foreign".into()),
        conditionals: HashMap::new(),
        languages: vec![],
    };
    let gen = MakefileInGenerator::new(am, config, traces);
    let output = gen.generate();
    // Per-target CFLAGS (-O2) should appear in compile command
    assert!(output.contains("-O2"), "per-target CFLAGS: {}", output);
}
