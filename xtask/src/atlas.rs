//! `xtask atlas` — build-profiler atlas.
//!
//! Turns every point in the build corpus into a reproducible, versioned recipe: probe results,
//! feature flags, dependency-graph snapshot, the optimal (working) pass pipeline, target settings,
//! known quirks, and verified outputs. A future build reads the recipe — installs the recorded
//! deps, applies the known pipeline + quirks, reproduces the verified outputs — instead of
//! rediscovering everything. The whole thing is GNU-free: only autoreconf-rs / acrs-* are invoked.
//!
//! Usage: `cargo xtask atlas <corpus-list> [out-dir]`
//!   corpus-list : file with one `owner/name` GitHub repo per line.
//!   out-dir     : recipe output dir (default `atlas/recipes`).
//! Tools resolved from env: AUTOCONF_RS, AUTOHEADER_RS, ACLOCAL_RS, AUTOMAKE_RS, and
//! `autoreconf-rs` on PATH (or AUTORECONF_RS).

use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

#[derive(Serialize)]
struct Recipe {
    schema: &'static str,
    repo: String,
    source: Source,
    toolchain: Toolchain,
    target: Target,
    pass_pipeline: Vec<Step>,
    probe_results: BTreeMap<String, u8>,
    feature_flags: FeatureFlags,
    dependencies: Dependencies,
    quirks: Vec<String>,
    outputs: Vec<Output>,
    status: String,
    verified: bool,
    diagnostic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    deep_expansion: Option<DeepExpansion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oracle: Option<Oracle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    divergence: Option<Divergence>,
    receipt: Receipt,
    environment: Environment,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    suggested_deps: Vec<SuggestedDep>,
    #[serde(skip_serializing_if = "Option::is_none")]
    directory_context: Option<DirectoryContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    makefile_forensics: Option<MakefileForensics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_requirements: Option<ToolRequirements>,
    #[serde(skip_serializing_if = "Option::is_none")]
    macro_inventory: Option<MacroInventory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditional_context: Option<ConditionalContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_aux_inventory: Option<ConfigAuxInventory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_surface: Option<LanguageSurface>,
    #[serde(skip_serializing_if = "Option::is_none")]
    libtool_context: Option<LibtoolContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gettext_intl_context: Option<GettextIntlContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    make_graph: Option<MakeGraph>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toolchain_interaction: Option<ToolchainInteraction>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    quirk_history: Vec<QuirkHistoryEntry>,
    verification: Verification,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpath_analysis: Option<VpathAnalysis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feature_probe_gap: Option<FeatureProbeGap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_to_generated_map: Option<ProvenanceMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dialect_reconciliation: Option<DialectReconciliation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    m4_side_effect_isolation: Option<M4SideEffectIsolation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallel_build_safety: Option<ParallelBuildSafety>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_environment_veil: Option<HostEnvironmentVeil>,
    #[serde(skip_serializing_if = "Option::is_none")]
    semantic_context: Option<SemanticContext>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    risk_factors: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    repair_hints: Vec<RepairHint>,
}

/// v3: Makefile pathology — classify WHY a generated Makefile fails to parse (the make layer's #1 root
/// is `missing separator`, but the symptom hides the cause: lost recipe-tab, unexpanded @VAR@ / automake
/// token, shell fragment in make context, unterminated construct). This turns an opaque string into a
/// fixable bug class, tied back to the generated file + line + preceding context.
#[derive(Serialize, Default)]
struct MakefileForensics {
    generated_makefiles: Vec<GeneratedMakefile>,
}
#[derive(Serialize)]
struct GeneratedMakefile {
    path: String,
    has_makefile_in: bool,
    has_makefile_am: bool,
    lines: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_parse_error: Option<MakeParseError>,
    /// Leftover `@VAR@` substitution placeholders make can't resolve (e.g. `@LIBOBJS@`, `@YACC@`).
    unexpanded_vars: Vec<String>,
    /// Leftover automake-internal tokens that should've been expanded (`%reldir%`, `$(am__objects_1)`).
    unexpanded_automake_tokens: Vec<String>,
    /// Recipe lines that start with spaces where make requires a leading TAB (the lost-tab bug class).
    recipe_tab_anomalies: usize,
}
#[derive(Serialize)]
struct MakeParseError {
    line: usize,
    kind: String,                 // missing-separator | unterminated | other
    text: String,
    previous_lines: Vec<String>,
    /// lost-tab | unexpanded-var | unexpanded-automake-token | shell-fragment-in-make | bare-macro | unknown
    probable_cause: String,
}

/// v3: build-time executables a recipe needs (distinct from link deps) — the command-not-found cluster.
/// Scanned from configure + generated Makefiles; missing ones become actionable package hints.
#[derive(Serialize, Default)]
struct ToolRequirements {
    detected: Vec<String>,
    missing: Vec<ToolMissing>,
}
#[derive(Serialize)]
struct ToolMissing {
    name: String,
    phase: String,            // configure | make | check | dist
    suggested_package: String,
}

/// v3 killer field: ranked, evidence-backed repair candidates derived from the forensic context — turns
/// each recipe from an evidence record into a self-training repair corpus ("what class of fix flips me
/// green?"). Emitted even before a fix is applied.
#[derive(Serialize)]
struct RepairHint {
    id: String,
    phase: String,
    confidence: f32,
    evidence: Vec<String>,
    action: String,
    expected_effect: String,
}

/// v3: m4 macro inventory — defined (m4/, aclocal.m4, acinclude.m4) vs called (configure.ac) vs
/// unresolved. Decides the fix class: load vendored macro / ship native / neutralize / not-standalone.
#[derive(Serialize, Default)]
struct MacroInventory {
    macro_dirs: Vec<String>,
    aclocal_m4_present: bool,
    acinclude_m4_present: bool,
    defined_macros: Vec<DefinedMacro>,
    called_macros: Vec<String>,
    unresolved_macros: Vec<String>, // called AC_/AX_/AM_/LT_ macros with no local definition
}
#[derive(Serialize)]
struct DefinedMacro {
    name: String,
    source: String,
    kind: String, // AC | AM | AX | LT | m4 | project-local
}

/// v3: shell/automake conditional balance — the syntax-structure roots (unbalanced if/case/for, leaked
/// text after conditional) become "this construct is unterminated" instead of opaque shell garbage.
#[derive(Serialize, Default)]
struct ConditionalContext {
    configure_if: usize,
    configure_fi: usize,
    configure_case: usize,
    configure_esac: usize,
    balanced: bool,
    automake_conditionals: Vec<String>, // AM_CONDITIONAL names
}

/// v3: config aux file inventory — install-sh/missing/depcomp/compile/config.guess/sub/ltmain.sh/ylwrap.
/// Missing helpers are a leading make/dist failure; this makes replay prescriptive (synthesize them).
#[derive(Serialize, Default)]
struct ConfigAuxInventory {
    aux_dir: String,
    present: Vec<String>,
    missing: Vec<String>,
}

/// v3: language surface — source suffixes + which compilers the project actually needs vs probes.
/// Distinguishes a real automake-rs bug from "needs a C++/Fortran compiler installed".
#[derive(Serialize, Default)]
struct LanguageSurface {
    source_suffixes: BTreeMap<String, usize>,
    configure_macros: Vec<String>, // AC_PROG_CC / AC_PROG_CXX / AC_PROG_F77 / ...
    needs_cxx: bool,
    needs_fortran: bool,
    sets_c_std: bool, // AC_PROG_CC_C99/C11 or -std present (else GCC14+/Clang18+ default-strict risk)
}

/// v3: libtool context — failures often masquerade as make failures; isolate the libtool surface.
#[derive(Serialize, Default)]
struct LibtoolContext {
    uses_libtool: bool,
    macros: Vec<String>,
    ltmain_present: bool,
    libtool_m4_sources: Vec<String>,
    age: String, // old | modern | unknown
}

/// v3: gettext/intltool context — classify as native-supported / needs-support-files / not-standalone.
#[derive(Serialize, Default)]
struct GettextIntlContext {
    uses_gettext: bool,
    uses_intltool: bool,
    po_dir_present: bool,
    missing_files: Vec<String>, // config.rpath, po/Makefile.in.in, ...
}

/// v3: make graph snapshot — targets, the load-bearing variables, generated files, recursion depth.
#[derive(Serialize, Default)]
struct MakeGraph {
    targets: Vec<String>,
    key_variables: BTreeMap<String, String>, // CC/CFLAGS/CPPFLAGS/LDFLAGS/LIBS/AR as the Makefile sets them
    generated_files: Vec<String>,            // Makefile/config.status/config.h/libtool present after configure
    recursion_depth: usize,                  // SUBDIRS nesting
    top_targets: Vec<String>,                // all/install/check/clean/dist present
    make_diagnostics: Vec<MakeDiagnostic>,   // classified make-command failures from the run log
}

/// v3: compiler/toolchain interaction — what the native compiler sees (bridge to codegen) + dialect risk.
#[derive(Serialize, Default)]
struct ToolchainInteraction {
    compiler: String,
    compiler_version: String,
    /// missing explicit C/C++ std + uses pre-C99 idioms -> GCC14+/Clang18+ default-strict failure risk.
    c_std_default_risk: bool,
    defines_sampled: Vec<String>, // -D macros the build passes (from Makefile CPPFLAGS/DEFS)
}

/// v3: quirk application history + effectiveness — turns ad-hoc quirks into learnable rules.
#[derive(Serialize)]
struct QuirkHistoryEntry {
    quirk_id: String,
    applied_at: String,     // configure | make | autoreconf | detected
    effect: String,         // success | partial | neutral
    effectiveness: String,  // high | medium | low | unknown
}

/// v3: verification + differential data — closes the loop vs the GNU oracle (and a future native codegen).
#[derive(Serialize, Default)]
struct Verification {
    #[serde(skip_serializing_if = "Option::is_none")]
    replay_success: Option<bool>,
    vs_gnu: String, // identical-status | ours-better | ours-worse | both-fail | not-compared
    #[serde(skip_serializing_if = "Vec::is_empty")]
    drift_noise: Vec<String>, // acceptable-noise classes when ours≠oracle (timestamps/comment-order)
    output_match: String,     // bit-exact | semantic | status-only | diff | not-compared (from oracle+outputs)
    #[serde(skip_serializing_if = "Option::is_none")]
    test_suite_pass_rate: Option<f32>, // null until a `make check` pass runs
}

/// v3: VPATH / out-of-tree + artifact side-effect analysis — the portability/parallel-build hazards.
#[derive(Serialize, Default)]
struct VpathAnalysis {
    /// Hardcoded `./` or `$(srcdir)/` paths embedded in Makefile.am shell rules (break VPATH/distcheck).
    hardcoded_src_paths: usize,
    /// Absolute build-path leakage in generated Makefile.in (breaks portability).
    abs_path_leakage: usize,
    /// BUILT_SOURCES declared (generated sources that must exist before compile).
    built_sources: Vec<String>,
    /// yacc/lex/codegen targets that generate sources (parallel-build race risk if deps unaligned).
    generated_source_targets: Vec<String>,
}

/// v3: dynamic feature-probe interception — host assumptions that break on cleaner/musl distros.
#[derive(Serialize, Default)]
struct FeatureProbeGap {
    headers_checked: Vec<String>,            // AC_CHECK_HEADERS
    /// headers #included across sources but never AC_CHECK'd — undocumented host assumptions.
    headers_included_unchecked: Vec<String>,
    /// hardcoded `-lfoo` in Makefile.am/macros not routed via PKG_CHECK_MODULES/AC_SEARCH_LIBS.
    implicit_link_libs: Vec<String>,
}

/// v3: source→generated provenance — tie a generated-shell/Makefile failure line back to its origin.
#[derive(Serialize, Default)]
struct ProvenanceMap {
    /// leaked/undefined macro -> the configure.ac/m4 file+line it was called from.
    configure_origins: Vec<MacroOrigin>,
    m4_trace_depth: usize, // max nested AC_DEFUN depth (engine stack/divergence risk)
    shadowed_macros: Vec<String>, // macros defined locally that override a standard/system def
}
#[derive(Serialize)]
struct MacroOrigin {
    macro_name: String,
    file: String,
    line: usize,
}

/// v3.1: dialect reconciliation — the deterministic execution envelope that catches modern-compiler
/// drift before the first TU. Legacy code (no explicit std) dies on GCC14+/Clang18+ default-strict
/// (implicit-function-declaration is now an ERROR). This block records the containment policy.
#[derive(Serialize, Default)]
struct DialectReconciliation {
    /// inferred required tier: c89_gnu | c99 | c11 | gnu++ | unknown (from configure macros + suffixes)
    enforce_standards_tier: String,
    /// modern -Werror flags that break vintage code and should be stripped at exec time.
    strip_modern_poison_flags: Vec<String>,
    inject_legacy_shims: bool, // -std=gnu89 -fpermissive needed (no std set + pre-C99 idioms)
    compiler_aliasing: BTreeMap<String, String>, // gcc-14 -> "-std=gnu89 -fpermissive", ...
}

/// v3.1: m4 side-effect isolation — legacy macros abuse the global m4 namespace; a broken macro in an
/// unvisited conditional can corrupt the whole flat Makefile.in stream. Records the hazards detected.
#[derive(Serialize, Default)]
struct M4SideEffectIsolation {
    /// unquoted AC_SUBST inside a conditional branch (implicit var decl that can break parsing).
    unquoted_subst_in_conditional: usize,
    /// local macro defs that shadow a standard/builtin macro (override historical tool-detection).
    shadowed_builtins: Vec<String>,
    /// AC_ARG_ENABLE/AC_ARG_WITH mutations (permitted) vs other global mutations (suspect).
    permitted_mutations: usize,
    suspect_global_mutations: usize,
}

/// v3.1: parallel-build safety — VPATH isolation + generated-source ordering (the make -j hazards).
#[derive(Serialize, Default)]
struct ParallelBuildSafety {
    vpath_out_of_tree_safe: bool, // no abs-path leakage / hardcoded srcdir paths detected
    generators: Vec<String>,      // yacc | lex | protoc | gperf present
    /// generated sources NOT declared in BUILT_SOURCES -> parallel race risk (inject dep edge).
    unordered_generated_sources: Vec<String>,
    built_sources_declared: bool,
}

/// v3.1: host environment veil — virtualize ancient system-header/symbol introspection so 20-year-old
/// platform checks don't leak broken/missing host definitions into the compile.
#[derive(Serialize, Default)]
struct HostEnvironmentVeil {
    /// headers the project checks/includes that have drifted or commonly need a fallback/mock.
    header_injection_candidates: Vec<String>,
    /// obsolete symbols the code uses that need aliasing (sys_errlist->strerror, etc.).
    symbol_aliasing_candidates: Vec<String>,
}

/// v3.1: compiler/semantic context — symbol surface + (future) native-codegen preview.
#[derive(Serialize, Default)]
struct SemanticContext {
    included_headers_sample: Vec<String>,
    undefined_symbols: Vec<String>, // from link errors in the make log
    provided_symbols_sample: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    llvm_native_preview: Option<bool>, // null until a native-codegen pass runs
}

/// v3.1: a single make-command diagnostic (command + exit + classified error).
#[derive(Serialize)]
struct MakeDiagnostic {
    command: String,
    error_type: String, // command-not-found | compiler-error | linker-error | missing-header | make-syntax
    message: String,
}

/// Deep subdir/directory context — the multi-directory build structure that drives (and breaks) the make
/// layer. Most non-trivial autotools projects recurse through SUBDIRS, and the per-subdir relative paths
/// (top_builddir/top_srcdir), config.h reachability, and per-directory build targets are exactly what a
/// recipe needs to debug a make failure without re-cloning. Captured by walking the cloned tree +
/// parsing configure.ac (AC_CONFIG_FILES/HEADERS) and each Makefile.am.
#[derive(Serialize, Default)]
struct DirectoryContext {
    /// AC_CONFIG_FILES targets, each with its computed relative top path (the SUBDIRS root cause: a
    /// subdir Makefile needs top_builddir=`..`, not `.`, or `-I$(top_builddir)` misses a top config.h).
    config_files: Vec<ConfigFileCtx>,
    /// AC_CONFIG_HEADERS targets (config.h locations) — the headers every subdir compile must reach.
    config_headers: Vec<String>,
    /// SUBDIRS recursion declared across Makefile.am files (the build tree the top Makefile drives).
    subdirs: Vec<String>,
    /// Per-directory build context: targets + whether its sources need a (possibly top-level) config.h.
    build_dirs: Vec<BuildDirCtx>,
    /// Max subdir depth of any config file — how many `..` levels the relative-path logic must handle.
    max_depth: usize,
    /// Directories whose sources `#include` config.h but that sit below the dir config.h is generated in
    /// — the repos where the relative `-I$(top_builddir)` path MUST be correct (the SUBDIRS make root).
    config_h_consumers_below_root: Vec<String>,
}
#[derive(Serialize)]
struct ConfigFileCtx {
    path: String,
    depth: usize,
    top_builddir: String, // the relative `..`-path a correct config.status must substitute for this file
    has_template: bool,   // a matching .in exists
}
#[derive(Serialize)]
struct BuildDirCtx {
    dir: String,
    /// Build-target declarations parsed from this dir's Makefile.am (bin_PROGRAMS, lib_LTLIBRARIES, …).
    targets: Vec<String>,
    subdirs: Vec<String>,        // this dir's own SUBDIRS
    sources_include_config_h: bool, // any source here #includes config.h (so needs it on the path)
    am_cppflags: String,         // AM_CPPFLAGS / INCLUDES line (the include-path the dir sets)
}
#[derive(Serialize)]
struct Source {
    url: String,
    git_sha: String,
    snapshot_utc: String,
}
#[derive(Serialize)]
struct Toolchain {
    autoconf_rs: String,
    automake_rs: String,
    m4_rs_core: String,
    gnu_free: bool,
}
#[derive(Serialize)]
struct Target {
    cc: String,
    cflags: String,
    host: String,
}
#[derive(Serialize)]
struct Step {
    step: String,
    tool: String,
    status: String,
}
#[derive(Serialize)]
struct FeatureFlags {
    configure_args: Vec<String>,
}
#[derive(Serialize)]
struct Dependencies {
    pkg_config: Vec<String>,
    system_libs: Vec<String>,
    headers_needed: Vec<String>,
    missing: Vec<String>,
}
#[derive(Serialize)]
struct Output {
    path: String,
    sha256: String,
    kind: String,
}

/// Deep-expansion forensics on the GENERATED configure — the diagnostics that otherwise require
/// hand-spelunking each configure on the VM. Captured per recipe so a bug is readable from the atlas.
#[derive(Serialize, Default)]
struct DeepExpansion {
    configure_lines: usize,
    /// Autoconf/m4 macro calls that survived into the generated shell unexpanded (AC_/AX_/AM_/LT_/
    /// PKG_/AS_/m4_/_AC_…) — each is a missing or partial macro definition leaking its arg list.
    leaked_macros: Vec<LeakedMacro>,
    /// `cat … <<_ACEOF >conftest` openers vs lone `_ACEOF` terminators. A negative imbalance is the
    /// missing-heredoc-opener bug (compile probe emitted as raw shell -> `syntax error near '('`).
    heredoc_openers: usize,
    heredoc_terminators: usize,
    heredoc_imbalance: i64,
    /// Each `./configure: line N: syntax error …` cross-referenced to the offending source line(s).
    syntax_errors: Vec<SyntaxError>,
    /// Malformed `${…}` cache-var references (embedded newline, PID-garbage `$$`, empty `${}`).
    cache_var_anomalies: Vec<String>,
    /// Leftover `@VAR@` substitution placeholders config.status never filled.
    residual_placeholders: Vec<String>,
    /// The last `checking for …` message printed before configure died — names the actual probe ours
    /// broke on (the real divergence), not the cascade line where the shell finally reports a syntax
    /// error. This is the actionable root: "ours died during <this check>".
    failed_during_check: String,
    /// Lines of configure-run output AFTER the last checking message (the crash's immediate fallout).
    failure_tail: Vec<String>,
    /// Conftest archaeology: C preprocessor directives MANGLED by m4 expanding `include`/`ifdef`/
    /// `define` builtins inside conftest source — `#include <x>` -> `# <x>`, `#ifdef Y` -> `# Y`.
    /// Each entry is `line N: <mangled text> (<which directive eaten>)`. A non-empty list means
    /// compile/link probes are silently failing on corrupted conftests (deep autotools bug).
    conftest_corruption: Vec<String>,
    /// Count of intact vs mangled `#include`/directive lines in the generated configure — the headline
    /// corruption ratio for fast triage.
    conftest_directives_intact: usize,
    conftest_directives_mangled: usize,
    /// The actual C conftest programs the macros emitted (the body between `<<_ACEOF >conftest` and
    /// `_ACEOF`) — the deep macro-OUTPUT context. Debugging a compile/link probe needs to see the
    /// exact C source that was compiled (intact or mangled), not infer it.
    conftest_programs: Vec<String>,
}
/// The GNU-autotools oracle outcome for the SAME repo, run right after ours on a git-reset tree.
/// This is the compass: `classification` says whether a failure is OUR bug (real succeeds, we don't —
/// fixable, with the divergence below showing where) or not-standalone (real fails too).
#[derive(Serialize, Default)]
struct Oracle {
    real_autoreconf: String,     // ok | fail
    real_configure: String,      // ok | fail | skipped
    real_make: String,           // ok | fail | skipped
    real_configure_lines: usize,
    real_first_error: String,
    /// BOTH_OK | OURS_BUG_CONFIGURE | OURS_BUG_MAKE | OURS_GEN_FAIL | NOT_STANDALONE |
    /// BOTH_CONFIGURE_FAIL | BOTH_MAKE_FAIL | OURS_BETTER | UNKNOWN
    classification: String,
}
/// Where ours diverges from a real run that got further — only populated for OURS_BUG_* recipes.
#[derive(Serialize, Default)]
struct Divergence {
    stage: String,                         // autoreconf | configure | make
    ours_error: String,                    // ours' first hard error line
    ours_error_context: Vec<String>,       // generated-shell lines around the failure (the actual bug site)
    ours_configure_lines: usize,
    real_configure_lines: usize,
    macros_ours_left_undefined: Vec<String>, // leaked macros = the macros to define/fix
}
#[derive(Serialize)]
struct LeakedMacro {
    name: String,
    line: usize,
    context: String,
}
#[derive(Serialize)]
struct SyntaxError {
    line: usize,
    token: String,
    source: String,
    /// The surrounding generated-shell construct (a few lines either side) — the syntactic context a
    /// human needs to see WHY it broke (the enclosing if/case/heredoc), not just the one error line.
    block: Vec<String>,
}

/// Sealed build-court receipt — makes each recipe auditable: the probe-execution trace (every HAVE_*
/// decision + why), the quirk rules that fired, and a sha256 hash-chain over the recipe's load-bearing
/// fields (toolchain + probe trace + outputs + oracle verdict). court_status is the verdict.
#[derive(Serialize, Default)]
struct Receipt {
    /// sealed (FUNC_OK, matches oracle) | partial (configure cleared, make failed) |
    /// quirk_dependent (needed a quirk) | not_standalone (oracle also fails) | failed
    court_status: String,
    probe_trace: Vec<ProbeStep>,
    quirks_matched: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    quirks_applied: Vec<QuirkApplied>,
    receipt_hash: String,
    schema: &'static str,
}
#[derive(Serialize)]
struct ProbeStep {
    name: String,   // HAVE_FOO_H / func | -llib
    kind: String,   // header | func | lib
    result: String, // yes | no
    reason: String, // ok | header-not-found | symbol-not-found | link-failed | not-recorded
}
/// An auto-applied quirk fix and whether it actually helped (re-run cleared a stage it didn't before).
#[derive(Serialize)]
struct QuirkApplied {
    id: String,
    action: String,  // the GNU-free fix applied (configure flag / mkdir / env)
    verified: bool,  // true = the build got further AFTER applying it
}
/// Build-environment fingerprint for hermeticity: the exact toolchain + paths + env that shaped the
/// build, so a recipe is reproducible across machines and time.
#[derive(Serialize, Default)]
struct Environment {
    cc: String,
    cc_version: String,
    host_triplet: String,
    pkg_config_version: String,
    make_version: String,
    relevant_env: Vec<String>,
    // v3 enrichment: platform fingerprint + POSIX parity + oracle provenance + env hermeticity.
    kernel_version: String,
    libc_name: String,
    libc_version: String,
    pkg_config_path: Vec<String>,
    env_vars_influential: BTreeMap<String, String>, // CC/CFLAGS/CPPFLAGS/LDFLAGS/LIBS/CPATH/...
    posix_flavor: String,                            // gnu | bsd | unknown (sed/grep flavor)
    shell: String,                                   // /bin/sh -> dash|bash (bashism risk)
    oracle_tool_versions: BTreeMap<String, String>,  // autoconf/automake/m4/perl versions (oracle provenance)
    env_var_whitelist: Vec<String>,                  // locked influential vars (ACLOCAL_PATH/AUTOMAKE_JOBS/...)
}
/// Missing-dep inference: a package that would satisfy a failed header/lib probe.
#[derive(Serialize)]
struct SuggestedDep {
    missing: String, // foo.h | -lfoo
    kind: String,    // header | lib
    package: String, // distro package that provides it
}

fn tool(env_var: &str, default: &str) -> String {
    std::env::var(env_var).unwrap_or_else(|_| default.to_string())
}

/// Run `cmd` (wrapped in coreutils `timeout`) in `dir`, capturing combined stdout+stderr to a FILE.
/// Two layers stop a build from ever hanging the worker:
///   * `timeout -k 10 -s KILL` — SIGKILL at the deadline (unignorable) + a 10s grace.
///   * redirect to a file, NOT a pipe — `.output()` would block reading the stdout pipe until EOF,
///     which orphaned grandchildren (that survive the kill) hold open forever. `.status()` only
///     waits for the direct child (timeout), so leaked children can never wedge us.
/// Toolchain interceptor shim (ATLAS_SHIM=1). Creates a temp dir of compiler shims (cc/gcc/clang/c++/
/// g++/clang++) that strip modern poison `-Werror` flags and append legacy-leniency flags, then exec the
/// REAL compiler in /usr/bin. Prepended to PATH for the make step only — zero Makefile mutation, so
/// generated files stay byte-exact with the GNU oracle. Returns the shim dir to prepend to PATH.
fn setup_compiler_shim() -> Option<PathBuf> {
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    let dir = std::env::temp_dir().join(format!("atlas_shim_{}", std::process::id()));
    std::fs::create_dir_all(&dir).ok()?;
    // C shims: downgrade the modern-default errors that kill legacy C; -fcommon for tentative-def code.
    let c_lenient = "-Wno-error=implicit-function-declaration -Wno-error=int-conversion -Wno-error=incompatible-pointer-types -Wno-error=implicit-int -Wno-implicit-function-declaration -fcommon";
    let cxx_lenient = "-Wno-error -fpermissive";
    let mk = |name: &str, real: &str, extra: &str| -> Option<()> {
        let p = dir.join(name);
        let mut f = std::fs::File::create(&p).ok()?;
        // strip standalone -Werror and -Werror=<modern poison>; pass everything else; append leniency.
        let script = format!(
            "#!/bin/sh\nNEW=\"\"\nfor a in \"$@\"; do\n  case \"$a\" in\n    -Werror|-Werror=implicit-function-declaration|-Werror=int-conversion|-Werror=incompatible-pointer-types|-Werror=implicit-int) ;;\n    *) NEW=\"$NEW $a\" ;;\n  esac\ndone\nexec {real} $NEW {extra}\n",
            real = real, extra = extra
        );
        f.write_all(script.as_bytes()).ok()?;
        let mut perm = f.metadata().ok()?.permissions();
        perm.set_mode(0o755);
        f.set_permissions(perm).ok()?;
        Some(())
    };
    // resolve real compilers (skip our shim dir): prefer /usr/bin then /bin
    let real = |name: &str| -> String {
        for base in ["/usr/bin", "/bin", "/usr/local/bin"] {
            let c = std::path::Path::new(base).join(name);
            if c.is_file() { return c.to_string_lossy().to_string(); }
        }
        name.to_string()
    };
    for n in ["cc", "gcc", "clang"] { mk(n, &real(n), c_lenient); }
    for n in ["c++", "g++", "clang++"] { mk(n, &real(n), cxx_lenient); }
    Some(dir)
}

fn run_timed(dir: &Path, secs: u32, program: &str, args: &[&str]) -> (bool, String) {
    use std::fs::File;
    let log = dir.join(".atlas_runlog");
    let f = match File::create(&log) {
        Ok(f) => f,
        Err(e) => return (false, e.to_string()),
    };
    let f2 = match f.try_clone() {
        Ok(f) => f,
        Err(e) => return (false, e.to_string()),
    };
    let status = Command::new("timeout")
        .arg("-k")
        .arg("10")
        .arg("-s")
        .arg("KILL")
        .arg(secs.to_string())
        .arg(program)
        .args(args)
        .current_dir(dir)
        .stdin(std::process::Stdio::null())
        .stdout(f)
        .stderr(f2)
        .status();
    let text = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_file(&log);
    match status {
        Ok(s) => (s.success(), text),
        Err(e) => (false, format!("{}\n{}", text, e)),
    }
}

fn first_line(program: &str, arg: &str) -> String {
    Command::new(program)
        .arg(arg)
        .output()
        .ok()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .next()
                .unwrap_or("")
                .to_string()
        })
        .unwrap_or_default()
}

fn sha256_16(path: &Path) -> Option<String> {
    let data = std::fs::read(path).ok()?;
    let mut h = Sha256::new();
    h.update(&data);
    Some(format!("{:x}", h.finalize())[..16].to_string())
}

pub fn run() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let list = match args.get(2) {
        Some(p) => p.clone(),
        None => {
            eprintln!("usage: cargo xtask atlas <corpus-list> [out-dir]");
            return ExitCode::from(2);
        }
    };
    let out_dir = PathBuf::from(args.get(3).cloned().unwrap_or_else(|| "atlas/recipes".into()));
    std::fs::create_dir_all(&out_dir).ok();

    let acrs_ac = tool("AUTOCONF_RS", "acrs-autoconf");
    let am = tool("AUTOMAKE_RS", "automake");
    let ars = tool("AUTORECONF_RS", "autoreconf-rs");
    let ac_ver = first_line(&acrs_ac, "--version");
    let am_ver = first_line(&am, "--version");

    let body = match std::fs::read_to_string(&list) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("atlas: cannot read {}: {}", list, e);
            return ExitCode::from(2);
        }
    };

    let mut n = 0;
    let mut ok = 0;
    for repo in body.lines().map(str::trim).filter(|l| !l.is_empty()) {
        n += 1;
        let slug = repo.replace('/', "__");
        let base = std::env::temp_dir().join(format!("atlasx_{}", slug));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).ok();
        let d = base.join("s");

        let mut pipeline = Vec::new();
        let mut status = "CLONE_FAIL".to_string();
        let mut git_sha = String::new();
        let mut diagnostic = String::new();

        let (cloned, _) = run_timed(
            &base,
            120,
            "git",
            &["clone", "--depth", "1", "-q", &format!("https://github.com/{}", repo), d.to_str().unwrap()],
        );
        if cloned {
            git_sha = Command::new("git")
                .args(["rev-parse", "HEAD"])
                .current_dir(&d)
                .output()
                .ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_default();

            let has_ac = d.join("configure.ac").exists() || d.join("configure.in").exists();
            if !has_ac {
                status = "NO_AC".to_string();
            } else {
                let ac_text = std::fs::read_to_string(d.join("configure.ac"))
                    .or_else(|_| std::fs::read_to_string(d.join("configure.in")))
                    .unwrap_or_default();
                let mut quirks = quirks_from_ac(&ac_text, &d);

                status = "CONFIGURE_GEN_FAIL".to_string();
                let (_b, _bl) = run_timed(&d, 150, &ars, &["-fi", "."]);
                pipeline.push(Step {
                    step: "autoreconf".into(),
                    tool: "autoreconf-rs".into(),
                    status: if d.join("configure").exists() { "ok".into() } else { "fail".into() },
                });

                let mut cf_log = String::new();
                let mut mk_log = String::new();
                // ATLAS_SCAN_ONLY=1: stop after generating configure. The deep_expansion ranking
                // (leaked macros, heredoc balance, residual @VAR@) is static analysis of the GENERATED
                // configure — it needs neither a configure-run nor make. Scan-only sweeps the full 1000
                // in a fraction of the time (no per-repo 120s/300s timeouts), giving the ranked backlog
                // fast. The full run (with run+make, for FUNC_OK) stays the default.
                let scan_only = std::env::var("ATLAS_SCAN_ONLY").is_ok();
                if d.join("configure").exists() {
                    if scan_only {
                        status = "CONFIGURE_GENERATED".to_string();
                    } else {
                        status = "CONFIGURE_RUN_FAIL".to_string();
                        let (cfok, cfl) = run_timed(&d, 180, "./configure", &[]);
                        cf_log = cfl;
                        pipeline.push(Step {
                            step: "configure".into(),
                            tool: "./configure".into(),
                            status: if cfok { "ok".into() } else { "fail".into() },
                        });
                        if cfok {
                            status = "MAKE_FAIL".to_string();
                            // Toolchain interceptor shim (ATLAS_SHIM=1): prepend a PATH dir of compiler
                            // shims that strip modern poison -Werror flags + add legacy-leniency
                            // (-Wno-error=implicit-function-declaration/int-conversion + -fcommon, and
                            // -fpermissive for C++). Lets vintage code build under GCC14+/Clang18+ default
                            // strictness WITHOUT mutating any Makefile (forensic byte-parity preserved).
                            let _shim = if std::env::var("ATLAS_SHIM").is_ok() { setup_compiler_shim() } else { None };
                            let saved_path = std::env::var("PATH").unwrap_or_default();
                            if let Some(sd) = &_shim { std::env::set_var("PATH", format!("{}:{}", sd.display(), saved_path)); }
                            let (mkok, mkl) = run_timed(&d, 300, "make", &["-j2"]);
                            if _shim.is_some() { std::env::set_var("PATH", &saved_path); }
                            mk_log = mkl;
                            pipeline.push(Step {
                                step: "make".into(),
                                tool: "make".into(),
                                status: if mkok { "ok".into() } else { "fail".into() },
                            });
                            if mkok {
                                status = "FUNC_OK".to_string();
                                ok += 1;
                            }
                        }
                    }
                }
                // === auto-apply quirks: if configure failed, try GNU-free quirk fixes and re-run ===
                let quirks_matched = match_quirks(&ac_text, &d, &cf_log);
                let mut quirks_applied = Vec::new();
                if !scan_only && status == "CONFIGURE_RUN_FAIL" {
                    let (ap, ns, oki) = auto_apply_quirks(&d, &quirks_matched, &status);
                    if !ap.is_empty() {
                        pipeline.push(Step {
                            step: "auto-quirk".into(),
                            tool: "quirk-engine".into(),
                            status: if ns == "FUNC_OK" || ns == "MAKE_FAIL" { "ok".into() } else { "fail".into() },
                        });
                        quirks_applied = ap;
                        status = ns;
                        if oki { ok += 1; }
                    }
                }
                let (probes, libs, hdrs, pkgs) = collect(&d, &cf_log, &mk_log);
                if status != "FUNC_OK" {
                    diagnostic = diag_line(&cf_log, &mk_log);
                }
                let verified = status == "FUNC_OK";
                let outputs = if verified { collect_outputs(&d) } else { Vec::new() };
                let deep_expansion = analyze_expansion(&d, &cf_log);
                // The compass: run the GNU-autotools oracle on the same repo and classify ours vs it.
                let (oracle, divergence) = if std::env::var("ATLAS_ORACLE").is_ok() {
                    let (o, dv) = run_oracle(&d, &status, &deep_expansion);
                    (Some(o), dv)
                } else {
                    (None, None)
                };
                if !libs.is_empty() {
                    quirks.push(format!("LIBS={}", libs.join(" ")));
                }
                // === build-court receipt + hermeticity + missing-dep inference ===
                let environment = fingerprint_environment();
                let probe_trace = build_probe_trace(&probes, &cf_log);
                let suggested_deps = infer_missing_deps(&hdrs, &cf_log);
                let toolchain = Toolchain {
                    autoconf_rs: ac_ver.clone(),
                    automake_rs: am_ver.clone(),
                    m4_rs_core: "0.1.4".into(),
                    gnu_free: true,
                };
                let court = court_status(&status, &oracle, &quirks_matched);
                let receipt_hash = compute_receipt_hash(&toolchain, &probes, &outputs, &oracle, &court);
                let receipt = Receipt {
                    court_status: court,
                    probe_trace,
                    quirks_matched,
                    quirks_applied,
                    receipt_hash,
                    schema: "automake-rs.build-court/v1",
                };
                // === v3 deep context: directory / makefile / macro / tool / language / libtool / gettext ===
                let directory_context = analyze_directory_context(&d, &ac_text);
                let makefile_forensics = analyze_makefile_forensics(&d);
                let tool_requirements = analyze_tool_requirements(&d, &ac_text, &diagnostic);
                let macro_inventory = analyze_macro_inventory(&d, &ac_text);
                let conditional_context = analyze_conditional_context(&d, &ac_text);
                let config_aux_inventory = analyze_config_aux(&d, &ac_text);
                let language_surface = analyze_language_surface(&d, &ac_text);
                let libtool_context = analyze_libtool(&d, &ac_text);
                let gettext_intl_context = analyze_gettext(&d, &ac_text);
                let make_graph = analyze_make_graph(&d, &directory_context, &mk_log);
                let toolchain_interaction = analyze_toolchain_interaction(&d, &ac_text, &environment, &language_surface);
                let quirk_hist = quirk_history(&receipt.quirks_matched, &receipt.quirks_applied, &status);
                let verification = analyze_verification(&status, &oracle);
                let vpath_analysis = analyze_vpath(&d);
                let feature_probe_gap = analyze_feature_probe_gap(&d, &ac_text);
                let source_to_generated_map = analyze_provenance(&d, &ac_text, &deep_expansion, &macro_inventory);
                // v3.1 deterministic-envelope blocks
                let dialect_reconciliation = analyze_dialect_reconciliation(&ac_text, &language_surface);
                let m4_side_effect_isolation = analyze_m4_isolation(&ac_text, &macro_inventory);
                let parallel_build_safety = analyze_parallel_build_safety(&vpath_analysis);
                let semantic_context = analyze_semantic_context(&mk_log, &feature_probe_gap);
                let host_environment_veil = analyze_host_veil(&feature_probe_gap, semantic_context.as_ref().map(|s| s.undefined_symbols.as_slice()).unwrap_or(&[]));
                let risk_factors = compute_risk_factors(&directory_context, &macro_inventory, &libtool_context, &gettext_intl_context, &dialect_reconciliation, &source_to_generated_map, &parallel_build_safety);
                let repair_hints = compute_repair_hints(&makefile_forensics, &directory_context, &macro_inventory, &tool_requirements);
                write_recipe(
                    &out_dir,
                    &slug,
                    Recipe {
                        schema: "automake-rs.build-atlas/v3",
                        repo: repo.to_string(),
                        source: Source {
                            url: format!("https://github.com/{}", repo),
                            git_sha: git_sha.clone(),
                            snapshot_utc: "2026-06-27".into(),
                        },
                        toolchain,
                        target: Target {
                            cc: "cc".into(),
                            cflags: "-g -O2".into(),
                            host: "x86_64-pc-linux-gnu".into(),
                        },
                        pass_pipeline: pipeline,
                        probe_results: probes,
                        feature_flags: FeatureFlags { configure_args: Vec::new() },
                        dependencies: Dependencies {
                            pkg_config: pkgs,
                            system_libs: libs,
                            headers_needed: hdrs.clone(),
                            missing: hdrs,
                        },
                        quirks,
                        outputs,
                        status: status.clone(),
                        verified,
                        diagnostic,
                        deep_expansion,
                        oracle,
                        divergence,
                        receipt,
                        environment,
                        suggested_deps,
                        directory_context,
                        makefile_forensics,
                        tool_requirements,
                        macro_inventory,
                        conditional_context,
                        config_aux_inventory,
                        language_surface,
                        libtool_context,
                        gettext_intl_context,
                        make_graph,
                        toolchain_interaction,
                        quirk_history: quirk_hist,
                        verification,
                        vpath_analysis,
                        feature_probe_gap,
                        source_to_generated_map,
                        dialect_reconciliation,
                        m4_side_effect_isolation,
                        parallel_build_safety,
                        host_environment_veil,
                        semantic_context,
                        risk_factors,
                        repair_hints,
                    },
                );
                println!("[{:>3}] {:<40} {}", n, repo, status);
                let _ = std::fs::remove_dir_all(&base);
                continue;
            }
        }
        // clone/no-ac path
        write_recipe(
            &out_dir,
            &slug,
            Recipe {
                schema: "automake-rs.build-atlas/v3",
                repo: repo.to_string(),
                source: Source { url: format!("https://github.com/{}", repo), git_sha, snapshot_utc: "2026-06-27".into() },
                toolchain: Toolchain { autoconf_rs: ac_ver.clone(), automake_rs: am_ver.clone(), m4_rs_core: "0.1.4".into(), gnu_free: true },
                target: Target { cc: "cc".into(), cflags: "-g -O2".into(), host: "x86_64-pc-linux-gnu".into() },
                pass_pipeline: pipeline,
                probe_results: BTreeMap::new(),
                feature_flags: FeatureFlags { configure_args: Vec::new() },
                dependencies: Dependencies { pkg_config: vec![], system_libs: vec![], headers_needed: vec![], missing: vec![] },
                quirks: vec![],
                outputs: vec![],
                status: status.clone(),
                verified: false,
                diagnostic,
                deep_expansion: None,
                oracle: None,
                divergence: None,
                receipt: Receipt {
                    court_status: if status == "CLONE_FAIL" { "failed".into() } else { "failed".into() },
                    schema: "automake-rs.build-court/v1",
                    ..Default::default()
                },
                environment: fingerprint_environment(),
                suggested_deps: vec![],
                directory_context: None,
                makefile_forensics: None,
                tool_requirements: None,
                macro_inventory: None,
                conditional_context: None,
                config_aux_inventory: None,
                language_surface: None,
                libtool_context: None,
                gettext_intl_context: None,
                make_graph: None,
                toolchain_interaction: None,
                quirk_history: vec![],
                verification: Verification::default(),
                vpath_analysis: None,
                feature_probe_gap: None,
                source_to_generated_map: None,
                dialect_reconciliation: None,
                m4_side_effect_isolation: None,
                parallel_build_safety: None,
                host_environment_veil: None,
                semantic_context: None,
                risk_factors: vec![],
                repair_hints: vec![],
            },
        );
        println!("[{:>3}] {:<40} {}", n, repo, status);
        let _ = std::fs::remove_dir_all(&base);
    }

    write_index(&out_dir);
    println!("\natlas: {} recipes, {} FUNC_OK ({}%)", n, ok, if n > 0 { ok * 100 / n } else { 0 });
    ExitCode::SUCCESS
}

fn quirks_from_ac(ac: &str, d: &Path) -> Vec<String> {
    let mut q = Vec::new();
    if ac.contains("AX_") {
        q.push("autoconf-archive-macros".into());
    }
    if ac.contains("LT_INIT") || ac.contains("AC_PROG_LIBTOOL") {
        q.push("libtool".into());
    }
    if ac.contains("PKG_CHECK_MODULES") {
        q.push("pkg-config".into());
    }
    if ac.contains("AM_GNU_GETTEXT") {
        q.push("gettext".into());
    }
    if ac.contains("AC_CONFIG_SUBDIRS") || d.join("Makefile.am").exists() && std::fs::read_to_string(d.join("Makefile.am")).unwrap_or_default().contains("SUBDIRS") {
        q.push("subdirs".into());
    }
    q
}

/// Run the REAL GNU autotools (system autoreconf/aclocal/autoconf + configure[+make]) on a git-reset
/// copy of the repo, then classify ours vs the oracle. `ours_status` is our pipeline's final status.
/// Returns the Oracle outcome and — when ours fails but the oracle got further — the Divergence that
/// shows exactly what to fix. Gated by ATLAS_ORACLE=1 (it ~doubles per-repo time).
fn run_oracle(
    d: &Path,
    ours_status: &str,
    de: &Option<DeepExpansion>,
) -> (Oracle, Option<Divergence>) {
    let mut o = Oracle::default();
    // snapshot ours' configure size + error before we reset the tree
    let ours_cfg_lines = std::fs::read_to_string(d.join("configure"))
        .map(|s| s.lines().count())
        .unwrap_or(0);
    // reset the working tree so the real run starts clean (remove our generated files)
    let _ = Command::new("git").arg("-C").arg(d).arg("clean").arg("-fdxq").status();
    let _ = Command::new("git").arg("-C").arg(d).arg("checkout").arg("--").arg(".").status();

    let (ar_ok, ar_log) = run_timed(d, 150, "autoreconf", &["-fi", "."]);
    o.real_autoreconf = if ar_ok && d.join("configure").exists() { "ok".into() } else { "fail".into() };
    if o.real_autoreconf != "ok" {
        o.real_first_error = first_err(&ar_log);
        o.real_configure = "skipped".into();
        o.real_make = "skipped".into();
    } else {
        let (cf_ok, cf_log) = run_timed(d, 180, "./configure", &[]);
        o.real_configure_lines = std::fs::read_to_string(d.join("configure")).map(|s| s.lines().count()).unwrap_or(0);
        o.real_configure = if cf_ok { "ok".into() } else { o.real_first_error = first_err(&cf_log); "fail".into() };
        if cf_ok {
            let (mk_ok, _mk) = run_timed(d, 300, "make", &["-j2"]);
            o.real_make = if mk_ok { "ok".into() } else { "fail".into() };
        } else {
            o.real_make = "skipped".into();
        }
    }

    let ours_cleared = matches!(ours_status, "MAKE_FAIL" | "FUNC_OK");
    let ours_made = ours_status == "FUNC_OK";
    let real_cleared = o.real_configure == "ok";
    let real_made = o.real_make == "ok";
    o.classification = if !real_cleared && o.real_autoreconf != "ok" {
        if ours_status == "CONFIGURE_GEN_FAIL" { "NOT_STANDALONE".into() } else { "OURS_BETTER".into() }
    } else if !real_cleared {
        if ours_cleared { "OURS_BETTER".into() } else { "BOTH_CONFIGURE_FAIL".into() }
    } else if !ours_cleared {
        if ours_status == "CONFIGURE_GEN_FAIL" { "OURS_GEN_FAIL".into() } else { "OURS_BUG_CONFIGURE".into() }
    } else if real_made && !ours_made {
        "OURS_BUG_MAKE".into()
    } else if ours_made && real_made {
        "BOTH_OK".into()
    } else {
        "BOTH_MAKE_FAIL".into()
    };

    // Divergence detail only for the fixable buckets (real got further than ours).
    let div = if o.classification.starts_with("OURS_BUG") || o.classification == "OURS_GEN_FAIL" {
        let dd = de.as_ref();
        Some(Divergence {
            stage: if o.classification == "OURS_BUG_MAKE" { "make".into() }
                   else if o.classification == "OURS_GEN_FAIL" { "autoreconf".into() }
                   else { "configure".into() },
            ours_error: dd.and_then(|x| x.syntax_errors.first().map(|s| format!("line {}: {} (near `{}`)", s.line, s.source, s.token)))
                .unwrap_or_default(),
            ours_error_context: dd.map(|x| x.syntax_errors.iter().take(3).map(|s| s.source.clone()).collect()).unwrap_or_default(),
            ours_configure_lines: ours_cfg_lines,
            real_configure_lines: o.real_configure_lines,
            macros_ours_left_undefined: dd.map(|x| {
                let mut v: Vec<String> = x.leaked_macros.iter().map(|m| m.name.clone()).collect();
                v.sort(); v.dedup(); v.truncate(15); v
            }).unwrap_or_default(),
        })
    } else {
        None
    };
    (o, div)
}

/// First hard error line from a log (configure/autoreconf), skipping known noise.
fn first_err(log: &str) -> String {
    for l in log.lines() {
        let ll = l.to_lowercase();
        if ll.contains("confdefs.h: no such file") { continue; }
        if ll.contains("error:") || ll.contains("syntax error") || ll.contains("command not found")
            || ll.contains("no such file") || ll.contains("undefined macro") || ll.contains("possibly undefined")
        {
            return l.trim().chars().take(120).collect();
        }
    }
    String::new()
}

/// Deep subdir/directory context: walks the cloned tree + parses configure.ac and each Makefile.am to
/// capture the multi-directory build structure — the AC_CONFIG_FILES targets with their relative
/// top_builddir paths, config.h locations, the SUBDIRS tree, per-dir build targets, and which dirs
/// consume a config.h that lives above them. This is the context that drives make-layer success.
fn analyze_directory_context(d: &Path, ac_text: &str) -> Option<DirectoryContext> {
    // collapse line-continuations + strip dnl comments so multi-line AC_CONFIG_FILES parse cleanly
    let flat = ac_text.replace("\\\n", " ");
    let extract_args = |mac_name: &str| -> Vec<String> {
        let mut out = Vec::new();
        let mut hay = flat.as_str();
        while let Some(p) = hay.find(mac_name) {
            let after = &hay[p + mac_name.len()..];
            let after = after.trim_start();
            if let Some(rest) = after.strip_prefix('(') {
                // take up to the matching close paren (shallow), strip [] quotes
                if let Some(end) = rest.find(')') {
                    let inner = rest[..end].replace(['[', ']'], " ");
                    // first arg only (before a comma) for FILES/HEADERS lists of files
                    let first = inner.split(',').next().unwrap_or("");
                    for tok in first.split_whitespace() {
                        out.push(tok.to_string());
                    }
                }
            }
            hay = &hay[p + mac_name.len()..];
        }
        out
    };
    let rel_top = |path: &str| -> (usize, String) {
        let dir = path.rsplit_once('/').map(|(a, _)| a).unwrap_or("");
        if dir.is_empty() || dir == "." {
            (0, ".".to_string())
        } else {
            let depth = dir.split('/').filter(|s| !s.is_empty() && *s != ".").count();
            (depth, vec![".."; depth].join("/"))
        }
    };

    let cfiles_raw = {
        let mut v = extract_args("AC_CONFIG_FILES");
        v.extend(extract_args("AC_OUTPUT")); // legacy AC_OUTPUT(files...) form
        v.retain(|f| !f.is_empty() && !f.contains('$') && (f.ends_with("Makefile") || f.ends_with(".in") || f.contains('.')));
        v.sort();
        v.dedup();
        v
    };
    let config_headers: Vec<String> = {
        let mut v = extract_args("AC_CONFIG_HEADERS");
        v.extend(extract_args("AM_CONFIG_HEADER"));
        v.retain(|f| !f.is_empty() && !f.contains('$'));
        v.sort();
        v.dedup();
        v
    };
    let header_dirs: Vec<String> = config_headers.iter()
        .map(|h| h.rsplit_once('/').map(|(a, _)| a.to_string()).unwrap_or_default())
        .collect();

    let mut config_files = Vec::new();
    let mut max_depth = 0usize;
    for f in &cfiles_raw {
        let (depth, top) = rel_top(f);
        max_depth = max_depth.max(depth);
        config_files.push(ConfigFileCtx {
            path: f.clone(),
            depth,
            top_builddir: top,
            has_template: d.join(format!("{}.in", f)).exists() || d.join(f).with_extension("in").exists(),
        });
    }

    // walk for Makefile.am (build dirs), depth <= 4
    let mut build_dirs = Vec::new();
    let mut consumers_below = Vec::new();
    collect_makefile_ams(d, d, 0, &mut build_dirs, &header_dirs, &mut consumers_below);
    build_dirs.sort_by(|a, b| a.dir.cmp(&b.dir));
    build_dirs.truncate(40);

    let subdirs: Vec<String> = build_dirs.iter().flat_map(|b| b.subdirs.clone()).collect();

    if config_files.is_empty() && config_headers.is_empty() && build_dirs.is_empty() {
        return None;
    }
    Some(DirectoryContext {
        config_files,
        config_headers,
        subdirs,
        build_dirs,
        max_depth,
        config_h_consumers_below_root: consumers_below,
    })
}

/// Walk for Makefile.am files; parse SUBDIRS, build targets, AM_CPPFLAGS, and whether sources here
/// #include config.h (and sit below the dir config.h is generated in — the SUBDIRS -I root cause).
fn collect_makefile_ams(root: &Path, dir: &Path, depth: usize, out: &mut Vec<BuildDirCtx>, header_dirs: &[String], consumers: &mut Vec<String>) {
    if depth > 4 || out.len() >= 40 {
        return;
    }
    let rel = dir.strip_prefix(root).ok().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
    let am = dir.join("Makefile.am");
    if let Ok(txt) = std::fs::read_to_string(&am) {
        let flat = txt.replace("\\\n", " ");
        let mut targets = Vec::new();
        let mut subdirs = Vec::new();
        let mut am_cppflags = String::new();
        for line in flat.lines() {
            let l = line.trim();
            if let Some(v) = l.strip_prefix("SUBDIRS") {
                subdirs.extend(v.trim_start_matches([' ', '=', '+']).split_whitespace().map(|s| s.to_string()));
            } else if l.starts_with("AM_CPPFLAGS") || l.starts_with("INCLUDES") {
                am_cppflags = l.splitn(2, '=').nth(1).unwrap_or("").trim().chars().take(120).collect();
            } else if let Some(eq) = l.find('=') {
                let lhs = l[..eq].trim();
                if lhs.ends_with("_PROGRAMS") || lhs.ends_with("_LTLIBRARIES") || lhs.ends_with("_LIBRARIES") {
                    for t in l[eq + 1..].split_whitespace() { if !t.is_empty() { targets.push(t.to_string()); } }
                }
            }
        }
        // does any source in this dir #include config.h?
        let mut includes_config_h = false;
        for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            let p = e.path();
            let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            if name.ends_with(".c") || name.ends_with(".h") || name.ends_with(".cc") || name.ends_with(".cpp") || name.ends_with(".cxx") {
                if let Ok(s) = std::fs::read_to_string(&p) {
                    if s.contains("config.h") { includes_config_h = true; break; }
                }
            }
        }
        if includes_config_h && !header_dirs.iter().any(|hd| hd == &rel || (hd.is_empty() && rel.is_empty())) && !rel.is_empty() {
            consumers.push(rel.clone());
        }
        if !targets.is_empty() || !subdirs.is_empty() || includes_config_h {
            out.push(BuildDirCtx {
                dir: if rel.is_empty() { ".".to_string() } else { rel.clone() },
                targets: { let mut t = targets; t.truncate(12); t },
                subdirs: subdirs.clone(),
                sources_include_config_h: includes_config_h,
                am_cppflags,
            });
        }
    }
    // recurse into subdirs
    for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = e.path();
        if p.is_dir() {
            let n = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            if n == ".git" || n == "autom4te.cache" || n.starts_with('.') { continue; }
            collect_makefile_ams(root, &p, depth + 1, out, header_dirs, consumers);
        }
    }
}

/// v3: walk generated Makefiles, classify the first parse error (the make-layer #1 root) + collect
/// unexpanded vars/tokens + lost-tab anomalies.
fn analyze_makefile_forensics(root: &Path) -> Option<MakefileForensics> {
    let mut gens = Vec::new();
    walk_makefiles(root, root, 0, &mut gens);
    if gens.is_empty() { return None; }
    gens.truncate(30);
    Some(MakefileForensics { generated_makefiles: gens })
}
fn walk_makefiles(root: &Path, dir: &Path, depth: usize, out: &mut Vec<GeneratedMakefile>) {
    if depth > 4 || out.len() >= 30 { return; }
    let mk = dir.join("Makefile");
    if mk.is_file() {
        if let Ok(txt) = std::fs::read_to_string(&mk) {
            let lines: Vec<&str> = txt.lines().collect();
            let rel = dir.strip_prefix(root).ok().map(|p| p.join("Makefile").to_string_lossy().to_string()).unwrap_or_else(|| "Makefile".into());
            let mut uvars: Vec<String> = Vec::new();
            let mut utok: Vec<String> = Vec::new();
            let mut tab_anom = 0usize;
            let mut first_err: Option<MakeParseError> = None;
            let mut in_rule = false;
            let mut prev_continues = false; // previous line ended with `\` (make line-continuation)
            let mut in_define = false;      // inside a make `define ... endef` block (verbatim shell)
            for (i, l) in lines.iter().enumerate() {
                // unexpanded @VAR@
                let mut rest = *l;
                while let Some(a) = rest.find('@') {
                    let r2 = &rest[a + 1..];
                    if let Some(b) = r2.find('@') {
                        let name = &r2[..b];
                        if !name.is_empty() && name.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_') {
                            let v = format!("@{}@", name);
                            if !uvars.contains(&v) && uvars.len() < 15 { uvars.push(v); }
                        }
                        rest = &r2[b + 1..];
                    } else { break; }
                }
                for t in ["%reldir%", "%canon_reldir%", "$(am__", "@am__"] {
                    if l.contains(t) && !utok.iter().any(|x| x == t) { utok.push(t.to_string()); }
                }
                // classify the first line make can't parse (missing separator territory)
                let starts_tab = l.starts_with('\t');
                let trimmed_now = l.trim_start();
                if trimmed_now.starts_with("define ") || trimmed_now == "define" { in_define = true; }
                // Only a STATEMENT-START line (not a `\`-continuation, not inside define/endef) can be a
                // parse error. This is the fix for the am__is_gnu_make shell-block false positive: its
                // `if test -z '$(MAKELEVEL)'` lines are continuations of `am__is_gnu_make = { \`.
                if first_err.is_none() && !prev_continues && !in_define {
                    let trimmed = trimmed_now;
                    let is_blank = trimmed.is_empty();
                    let is_comment = trimmed.starts_with('#');
                    let is_assign = l.find('=').map(|e| l[..e].chars().all(|c| c.is_ascii_alphanumeric() || "_+:. ".contains(c)) && !l[..e].contains('\t')).unwrap_or(false);
                    let is_rule = trimmed.contains(':') && !starts_tab && !trimmed.starts_with(':');
                    let is_directive = ["ifeq", "ifneq", "ifdef", "ifndef", "else", "endif", "include", "-include", "define", "endef", "export", "unexport", "vpath", "override", "if ", "fi", "case", "esac", "for ", "while ", "do", "done", "then", "elif", ":"].iter().any(|kw| trimmed.starts_with(kw));
                    // a non-tab, non-blank, non-comment, non-assignment, non-rule, non-directive line that
                    // looks like a recipe/shell or a leaked token -> the missing-separator site
                    if !starts_tab && !is_blank && !is_comment && !is_assign && !is_rule && !is_directive && !trimmed.starts_with('\\') {
                        let cause = if trimmed.starts_with('@') { "unexpanded-var" }
                            else if trimmed.contains("%reldir%") || trimmed.contains("$(am__") { "unexpanded-automake-token" }
                            else if in_rule && (l.starts_with(' ') || l.starts_with("  ")) { "lost-tab" }
                            else if trimmed.starts_with(|c: char| c.is_ascii_uppercase()) && trimmed.contains('(') { "bare-macro" }
                            else { "shell-fragment-in-make" };
                        first_err = Some(MakeParseError {
                            line: i + 1,
                            kind: "missing-separator".into(),
                            text: l.chars().take(80).collect(),
                            previous_lines: lines[i.saturating_sub(2)..i].iter().map(|s| s.chars().take(80).collect()).collect(),
                            probable_cause: cause.into(),
                        });
                    }
                }
                if starts_tab && l.trim().is_empty() { tab_anom += 1; }
                if trimmed_now == "endef" { in_define = false; }
                // a line "continues" if it ends with an unescaped backslash
                prev_continues = l.trim_end().ends_with('\\');
                in_rule = l.contains(':') && !l.starts_with('\t');
            }
            out.push(GeneratedMakefile {
                path: rel,
                has_makefile_in: dir.join("Makefile.in").exists(),
                has_makefile_am: dir.join("Makefile.am").exists(),
                lines: lines.len(),
                first_parse_error: first_err,
                unexpanded_vars: uvars,
                unexpanded_automake_tokens: utok,
                recipe_tab_anomalies: tab_anom,
            });
        }
    }
    for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = e.path();
        if p.is_dir() {
            let n = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            if n == ".git" || n.starts_with('.') { continue; }
            walk_makefiles(root, &p, depth + 1, out);
        }
    }
}

/// v3 package map for a build-time tool name.
fn tool_pkg(name: &str) -> &'static str {
    match name {
        "bison" | "yacc" => "bison", "flex" | "lex" => "flex", "gperf" => "gperf",
        "help2man" => "help2man", "makeinfo" => "texinfo", "pod2man" | "perl" => "perl",
        "python" | "python3" => "python3", "ruby" => "ruby", "gtkdocize" => "gtk-doc-tools",
        "glib-genmarshal" | "glib-mkenums" => "libglib2.0-dev-bin", "intltoolize" => "intltool",
        "xgettext" | "msgfmt" => "gettext", "pkg-config" => "pkg-config", "libtoolize" => "libtool",
        "xsltproc" => "xsltproc", "asciidoc" => "asciidoc", "rst2man" => "python3-docutils",
        "doxygen" => "doxygen", "swig" => "swig", "nasm" => "nasm", "yasm" => "yasm",
        _ => "",
    }
}
/// v3: build-time tool requirements from configure.ac macros + generated Makefiles + the make diagnostic.
fn analyze_tool_requirements(d: &Path, ac_text: &str, diagnostic: &str) -> Option<ToolRequirements> {
    let mut detected: Vec<String> = Vec::new();
    let mut add = |s: &str, v: &mut Vec<String>| { if !s.is_empty() && !v.iter().any(|x| x == s) { v.push(s.to_string()); } };
    for (macro_, tool) in [("AC_PROG_YACC", "yacc"), ("AC_PROG_LEX", "lex"), ("AM_PATH_PYTHON", "python"),
        ("AC_PROG_CXX", "c++"), ("AM_PROG_AR", "ar"), ("GTK_DOC_CHECK", "gtkdocize"),
        ("IT_PROG_INTLTOOL", "intltoolize"), ("AM_GNU_GETTEXT", "xgettext"), ("AC_PROG_F77", "f77")] {
        if ac_text.contains(macro_) { add(tool, &mut detected); }
    }
    for tool in ["perl", "python", "bison", "flex", "help2man", "makeinfo", "pkg-config", "swig", "doxygen", "gperf"] {
        if ac_text.contains(&format!("AC_PATH_PROG")) && ac_text.contains(tool) { add(tool, &mut detected); }
    }
    // missing tools: from the make "command not found" diagnostic
    let mut missing = Vec::new();
    if let Some(p) = diagnostic.find(": command not found") {
        let pre = &diagnostic[..p];
        let name = pre.rsplit([' ', '/', ':', '`', '\'']).find(|s| !s.is_empty()).unwrap_or("");
        let name = name.trim_start_matches('@').trim_end_matches('@');
        if !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.') {
            missing.push(ToolMissing { name: name.to_string(), phase: "make".into(), suggested_package: tool_pkg(name).to_string() });
        }
    }
    let _ = d;
    if detected.is_empty() && missing.is_empty() { return None; }
    Some(ToolRequirements { detected, missing })
}

/// v3: m4 macro inventory — defined (m4/, acinclude.m4, aclocal.m4) vs called vs unresolved.
fn analyze_macro_inventory(d: &Path, ac_text: &str) -> Option<MacroInventory> {
    let mut defined = Vec::new();
    let mut defined_names = std::collections::BTreeSet::new();
    let mut macro_dirs = Vec::new();
    // macro dirs from AC_CONFIG_MACRO_DIR + common locations
    for cand in ["m4", "build-aux/m4", "macros", "config"] {
        if d.join(cand).is_dir() { macro_dirs.push(cand.to_string()); }
    }
    let scan_defs = |path: &Path, src: &str, defined: &mut Vec<DefinedMacro>, names: &mut std::collections::BTreeSet<String>| {
        if let Ok(txt) = std::fs::read_to_string(path) {
            let mut hay = txt.as_str();
            while let Some(p) = hay.find("AC_DEFUN") {
                let after = hay[p..].find('(').map(|i| &hay[p + i + 1..]).unwrap_or("");
                let inner = after.trim_start().trim_start_matches('[');
                let name: String = inner.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
                if !name.is_empty() && names.insert(name.clone()) && defined.len() < 60 {
                    let kind = if name.starts_with("AX_") { "AX" } else if name.starts_with("AM_") { "AM" }
                        else if name.starts_with("LT_") || name.contains("LIBTOOL") { "LT" }
                        else if name.starts_with("AC_") { "AC" } else { "project-local" };
                    defined.push(DefinedMacro { name, source: src.to_string(), kind: kind.into() });
                }
                hay = &hay[p + 8..];
            }
        }
    };
    for dir in &macro_dirs {
        for e in std::fs::read_dir(d.join(dir)).into_iter().flatten().flatten() {
            let p = e.path();
            if p.extension().and_then(|s| s.to_str()) == Some("m4") {
                let src = p.strip_prefix(d).ok().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
                scan_defs(&p, &src, &mut defined, &mut defined_names);
            }
        }
    }
    let acinclude = d.join("acinclude.m4");
    if acinclude.is_file() { scan_defs(&acinclude, "acinclude.m4", &mut defined, &mut defined_names); }
    // called macros: AC_/AX_/AM_/LT_/IT_/GTK_ name immediately followed by ( or whitespace in configure.ac
    let mut called = std::collections::BTreeSet::new();
    let bytes = ac_text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_ascii_uppercase() {
            let start = i;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') { i += 1; }
            let name = &ac_text[start..i];
            if (name.starts_with("AC_") || name.starts_with("AX_") || name.starts_with("AM_") || name.starts_with("LT_") || name.starts_with("IT_") || name.starts_with("GTK_") || name.starts_with("PKG_")) && name.len() > 3 {
                called.insert(name.to_string());
            }
        } else { i += 1; }
    }
    // unresolved: AX_/custom called macros with no local def (the ones needing vendored/native impl)
    let unresolved: Vec<String> = called.iter()
        .filter(|m| (m.starts_with("AX_") || m.starts_with("IT_") || m.starts_with("GTK_")) && !defined_names.contains(*m))
        .take(30).cloned().collect();
    let called_v: Vec<String> = called.into_iter().take(60).collect();
    if defined.is_empty() && called_v.is_empty() { return None; }
    Some(MacroInventory {
        macro_dirs,
        aclocal_m4_present: d.join("aclocal.m4").is_file(),
        acinclude_m4_present: acinclude.is_file(),
        defined_macros: defined,
        called_macros: called_v,
        unresolved_macros: unresolved,
    })
}

/// v3: shell/automake conditional balance from the generated configure + AM_CONDITIONAL names.
fn analyze_conditional_context(d: &Path, ac_text: &str) -> Option<ConditionalContext> {
    let cfg = std::fs::read_to_string(d.join("configure")).ok()?;
    let (mut ifc, mut fic, mut casec, mut esacc) = (0usize, 0usize, 0usize, 0usize);
    for l in cfg.lines() {
        let t = l.trim();
        if t == "fi" || t.ends_with("; fi") || t.starts_with("fi ") || t.starts_with("fi;") { fic += 1; }
        if t.starts_with("if ") || t == "if" || t.ends_with("; then") || t.contains("; then ") { ifc += 1; }
        if t == "esac" { esacc += 1; }
        if t.starts_with("case ") && t.contains(" in") { casec += 1; }
    }
    let mut am_cond = Vec::new();
    let mut hay = ac_text;
    while let Some(p) = hay.find("AM_CONDITIONAL") {
        let after = hay[p..].find('(').map(|i| &hay[p + i + 1..]).unwrap_or("");
        let inner = after.trim_start().trim_start_matches('[');
        let name: String = inner.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
        if !name.is_empty() && !am_cond.contains(&name) && am_cond.len() < 30 { am_cond.push(name); }
        hay = &hay[p + 14..];
    }
    Some(ConditionalContext {
        configure_if: ifc, configure_fi: fic, configure_case: casec, configure_esac: esacc,
        balanced: ifc == fic && casec == esacc,
        automake_conditionals: am_cond,
    })
}

/// v3: config aux file inventory (install-sh, missing, depcomp, compile, config.guess/sub, ltmain, ylwrap).
fn analyze_config_aux(d: &Path, ac_text: &str) -> Option<ConfigAuxInventory> {
    // aux dir from AC_CONFIG_AUX_DIR
    let aux_dir = ac_text.find("AC_CONFIG_AUX_DIR")
        .and_then(|p| ac_text[p..].find('(').map(|i| &ac_text[p + i + 1..]))
        .map(|s| s.trim_start().trim_start_matches('[').chars().take_while(|c| *c != ']' && *c != ')').collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty()).unwrap_or_else(|| ".".into());
    let need_libtool = ac_text.contains("LT_INIT") || ac_text.contains("AC_PROG_LIBTOOL");
    let mut required = vec!["install-sh", "missing", "depcomp", "compile"];
    if need_libtool { required.push("ltmain.sh"); }
    if ac_text.contains("AC_CANONICAL") { required.push("config.guess"); required.push("config.sub"); }
    let base = if aux_dir == "." { d.to_path_buf() } else { d.join(&aux_dir) };
    let mut present = Vec::new();
    let mut missing = Vec::new();
    for f in &required {
        if base.join(f).exists() || d.join(f).exists() { present.push(f.to_string()); } else { missing.push(f.to_string()); }
    }
    Some(ConfigAuxInventory { aux_dir, present, missing })
}

/// v3: language surface — source suffixes + needed compilers + C-std-set risk.
fn analyze_language_surface(d: &Path, ac_text: &str) -> Option<LanguageSurface> {
    let mut suffixes: BTreeMap<String, usize> = BTreeMap::new();
    count_suffixes(d, d, 0, &mut suffixes);
    let mut macros = Vec::new();
    for m in ["AC_PROG_CC", "AC_PROG_CXX", "AC_PROG_F77", "AC_PROG_FC", "AC_PROG_OBJC", "AC_PROG_CC_C99", "AC_PROG_CC_C11"] {
        if ac_text.contains(m) { macros.push(m.to_string()); }
    }
    let needs_cxx = ac_text.contains("AC_PROG_CXX") || ["cc", "cpp", "cxx", "C"].iter().any(|s| suffixes.get(*s).copied().unwrap_or(0) > 0);
    let needs_fortran = ac_text.contains("AC_PROG_F77") || ac_text.contains("AC_PROG_FC") || ["f", "f90", "f77", "for"].iter().any(|s| suffixes.get(*s).copied().unwrap_or(0) > 0);
    let sets_c_std = ac_text.contains("AC_PROG_CC_C99") || ac_text.contains("AC_PROG_CC_C11") || ac_text.contains("-std=") || ac_text.contains("AX_CXX_COMPILE_STDCXX");
    Some(LanguageSurface { source_suffixes: suffixes, configure_macros: macros, needs_cxx, needs_fortran, sets_c_std })
}
fn count_suffixes(root: &Path, dir: &Path, depth: usize, out: &mut BTreeMap<String, usize>) {
    if depth > 4 { return; }
    let _ = root;
    for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = e.path();
        if p.is_dir() {
            let n = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            if n == ".git" || n.starts_with('.') { continue; }
            count_suffixes(root, &p, depth + 1, out);
        } else if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
            if ["c", "cc", "cpp", "cxx", "C", "h", "hpp", "f", "f90", "f77", "m", "mm", "s", "S", "go", "rs"].contains(&ext) {
                *out.entry(ext.to_string()).or_default() += 1;
            }
        }
    }
}

/// v3: libtool context.
fn analyze_libtool(d: &Path, ac_text: &str) -> Option<LibtoolContext> {
    let uses = ac_text.contains("LT_INIT") || ac_text.contains("AC_PROG_LIBTOOL") || ac_text.contains("AM_PROG_LIBTOOL");
    if !uses && !d.join("ltmain.sh").exists() { return None; }
    let mut macros = Vec::new();
    for m in ["LT_INIT", "AC_PROG_LIBTOOL", "AM_PROG_LIBTOOL", "LT_LANG"] { if ac_text.contains(m) { macros.push(m.to_string()); } }
    let mut srcs = Vec::new();
    for cand in ["m4/libtool.m4", "libtool.m4", "aclocal.m4", "acinclude.m4"] {
        if d.join(cand).is_file() && std::fs::read_to_string(d.join(cand)).map(|s| s.contains("LT_INIT") || s.contains("libtool")).unwrap_or(false) { srcs.push(cand.to_string()); }
    }
    let age = if ac_text.contains("AC_PROG_LIBTOOL") && !ac_text.contains("LT_INIT") { "old" } else if ac_text.contains("LT_INIT") { "modern" } else { "unknown" };
    Some(LibtoolContext { uses_libtool: uses, macros, ltmain_present: d.join("ltmain.sh").exists() || d.join("build-aux/ltmain.sh").exists(), libtool_m4_sources: srcs, age: age.into() })
}

/// v3: gettext/intltool context.
fn analyze_gettext(d: &Path, ac_text: &str) -> Option<GettextIntlContext> {
    let uses_gettext = ac_text.contains("AM_GNU_GETTEXT") || ac_text.contains("AM_ICONV");
    let uses_intltool = ac_text.contains("IT_PROG_INTLTOOL") || ac_text.contains("INTLTOOL");
    if !uses_gettext && !uses_intltool { return None; }
    let po = d.join("po").is_dir();
    let mut missing = Vec::new();
    if uses_gettext {
        if !d.join("config.rpath").exists() { missing.push("config.rpath".to_string()); }
        if po && !d.join("po/Makefile.in.in").exists() { missing.push("po/Makefile.in.in".to_string()); }
    }
    Some(GettextIntlContext { uses_gettext, uses_intltool, po_dir_present: po, missing_files: missing })
}

/// v3: derive ranked repair candidates from the forensic context — the self-training repair corpus.
fn compute_repair_hints(mf: &Option<MakefileForensics>, dc: &Option<DirectoryContext>, mi: &Option<MacroInventory>, tr: &Option<ToolRequirements>) -> Vec<RepairHint> {
    let mut hints = Vec::new();
    if let Some(m) = mf {
        for g in &m.generated_makefiles {
            if let Some(e) = &g.first_parse_error {
                let (action, conf) = match e.probable_cause.as_str() {
                    "lost-tab" => ("preserve-recipe-tab-in-rule-emission", 0.9),
                    "unexpanded-var" => ("substitute-standard-var (extend STD_VAR_SED)", 0.85),
                    "unexpanded-automake-token" => ("expand-automake-token (%reldir%/$(am__))", 0.8),
                    "bare-macro" => ("define-or-neutralize-leaked-macro", 0.7),
                    _ => ("classify-shell-fragment-in-make", 0.5),
                };
                hints.push(RepairHint {
                    id: format!("makefile-{}", e.probable_cause), phase: "make".into(), confidence: conf,
                    evidence: vec![format!("{}:{} {}", g.path, e.line, e.kind), e.text.clone()],
                    action: action.into(), expected_effect: "partial -> FUNC_OK candidate".into(),
                });
            }
            if !g.unexpanded_vars.is_empty() {
                hints.push(RepairHint {
                    id: "unexpanded-makefile-vars".into(), phase: "make".into(), confidence: 0.8,
                    evidence: g.unexpanded_vars.iter().take(5).cloned().collect(),
                    action: "add these to STD_VAR_SED / AC_SUBST handling".into(), expected_effect: "removes raw @VAR@ from Makefile".into(),
                });
            }
        }
    }
    if let Some(d) = dc {
        if !d.config_h_consumers_below_root.is_empty() {
            hints.push(RepairHint {
                id: "subdir-top-builddir".into(), phase: "make".into(), confidence: 0.88,
                evidence: d.config_h_consumers_below_root.iter().take(5).map(|s| format!("{} consumes config.h below root", s)).collect(),
                action: "ensure ac_subst_file sets relative top_builddir per subdir (0.1.19)".into(),
                expected_effect: "config.h found in subdir compiles".into(),
            });
        }
    }
    if let Some(m) = mi {
        if !m.unresolved_macros.is_empty() {
            let has_dir = !m.macro_dirs.is_empty();
            hints.push(RepairHint {
                id: if has_dir { "load-vendored-aclocal-dir" } else { "ship-or-neutralize-macro" }.into(),
                phase: "autoreconf".into(), confidence: if has_dir { 0.85 } else { 0.6 },
                evidence: m.unresolved_macros.iter().take(5).cloned().collect(),
                action: if has_dir { format!("aclocal -I {}", m.macro_dirs.join(" -I ")) } else { "ship native impl or neutralize".into() },
                expected_effect: "resolves unexpanded AX_/custom macros".into(),
            });
        }
    }
    if let Some(t) = tr {
        for mtool in &t.missing {
            hints.push(RepairHint {
                id: "missing-build-tool".into(), phase: mtool.phase.clone(), confidence: 0.75,
                evidence: vec![format!("{}: command not found", mtool.name)],
                action: if mtool.suggested_package.is_empty() { format!("provide {}", mtool.name) } else { format!("install {}", mtool.suggested_package) },
                expected_effect: "unblocks the build step needing this tool".into(),
            });
        }
    }
    hints.truncate(12);
    hints
}

/// v3: make graph snapshot from the top generated Makefile + tree.
fn analyze_make_graph(d: &Path, dc: &Option<DirectoryContext>, mk_log: &str) -> Option<MakeGraph> {
    let mk = std::fs::read_to_string(d.join("Makefile")).ok()?;
    let mut targets = Vec::new();
    let mut key = BTreeMap::new();
    for l in mk.lines() {
        let t = l.trim_end();
        // target rules: `name:` at col 0 (not a recipe, not an assignment)
        if !l.starts_with('\t') && !l.starts_with(' ') {
            if let Some(c) = t.find(':') {
                let lhs = &t[..c];
                if !lhs.is_empty() && !lhs.contains('=') && lhs.chars().all(|ch| ch.is_ascii_alphanumeric() || "_-./ $()".contains(ch)) && t.as_bytes().get(c + 1) != Some(&b'=') {
                    for tg in lhs.split_whitespace() { if targets.len() < 40 && !targets.iter().any(|x| x == tg) { targets.push(tg.to_string()); } }
                }
            }
            for kv in ["CC", "CFLAGS", "CPPFLAGS", "CXX", "CXXFLAGS", "LDFLAGS", "LIBS", "AR", "RANLIB", "DEFS", "DEFAULT_INCLUDES"] {
                if let Some(rest) = t.strip_prefix(kv) {
                    let rest = rest.trim_start();
                    if let Some(v) = rest.strip_prefix('=') { key.entry(kv.to_string()).or_insert_with(|| v.trim().chars().take(160).collect()); }
                }
            }
        }
    }
    let mut generated = Vec::new();
    for f in ["Makefile", "config.status", "config.h", "libtool", "stamp-h1", "config.log"] {
        if d.join(f).exists() { generated.push(f.to_string()); }
    }
    let depth = dc.as_ref().map(|x| x.max_depth).unwrap_or(0);
    let top_targets: Vec<String> = ["all", "install", "check", "clean", "dist", "all-am", "all-recursive"]
        .iter().filter(|t| targets.iter().any(|x| x == *t)).map(|s| s.to_string()).collect();
    // classify make-command failures from the run log
    let mut make_diagnostics = Vec::new();
    for l in mk_log.lines() {
        let ll = l.to_lowercase();
        let etype = if ll.contains("command not found") || (ll.contains("not found") && ll.contains("make")) { "command-not-found" }
            else if ll.contains("undefined reference") || ll.contains("cannot find -l") || ll.contains("ld:") { "linker-error" }
            else if ll.contains("no such file") && ll.contains(".h") { "missing-header" }
            else if ll.contains("missing separator") || ll.contains("*** ") { "make-syntax" }
            else if ll.contains(": error:") || ll.contains("fatal error:") { "compiler-error" }
            else { continue };
        if make_diagnostics.len() < 10 {
            make_diagnostics.push(MakeDiagnostic { command: String::new(), error_type: etype.into(), message: l.trim().chars().take(120).collect() });
        }
    }
    Some(MakeGraph { targets, key_variables: key, generated_files: generated, recursion_depth: depth, top_targets, make_diagnostics })
}

/// v3: compiler/toolchain interaction + C-dialect risk.
fn analyze_toolchain_interaction(d: &Path, ac_text: &str, env: &Environment, ls: &Option<LanguageSurface>) -> Option<ToolchainInteraction> {
    let sets_std = ls.as_ref().map(|l| l.sets_c_std).unwrap_or(false);
    // pre-C99 idiom risk: uses implicit-int / old func decls without an explicit std + modern compiler
    let mut risk = !sets_std;
    // sample -D defines from the top Makefile
    let mut defines = Vec::new();
    if let Ok(mk) = std::fs::read_to_string(d.join("Makefile")) {
        for l in mk.lines() {
            for tok in l.split_whitespace() {
                if let Some(dval) = tok.strip_prefix("-D") {
                    if !dval.is_empty() && defines.len() < 20 && !defines.iter().any(|x| x == dval) { defines.push(dval.to_string()); }
                }
            }
        }
    }
    // bare compiler builtins unguarded (a dialect hazard)
    let _ = ac_text;
    if sets_std { risk = false; }
    Some(ToolchainInteraction {
        compiler: env.cc.clone(),
        compiler_version: env.cc_version.clone(),
        c_std_default_risk: risk,
        defines_sampled: defines,
    })
}

/// v3: quirk history + effectiveness — from the receipt's matched/applied quirks vs the outcome.
fn quirk_history(quirks_matched: &[String], quirks_applied: &[QuirkApplied], status: &str) -> Vec<QuirkHistoryEntry> {
    let mut out = Vec::new();
    for q in quirks_applied {
        let (effect, eff) = if q.verified && (status == "FUNC_OK" || status == "MAKE_FAIL") { ("success", "high") }
            else if q.verified { ("partial", "medium") } else { ("neutral", "low") };
        out.push(QuirkHistoryEntry { quirk_id: q.id.clone(), applied_at: "configure".into(), effect: effect.into(), effectiveness: eff.into() });
    }
    for m in quirks_matched {
        if !out.iter().any(|e| &e.quirk_id == m) {
            out.push(QuirkHistoryEntry { quirk_id: m.clone(), applied_at: "detected".into(), effect: "neutral".into(), effectiveness: "unknown".into() });
        }
    }
    out.truncate(20);
    out
}

/// v3: verification + differential vs the GNU oracle (+ drift noise classes).
fn analyze_verification(status: &str, oracle: &Option<Oracle>) -> Verification {
    let vs = match oracle {
        Some(o) => {
            let ours_ok = status == "FUNC_OK";
            match o.classification.as_str() {
                "BOTH_OK" => "identical-status",
                "OURS_BETTER" => "ours-better",
                c if c.starts_with("OURS_BUG") => "ours-worse",
                "NOT_STANDALONE" | "BOTH_CONFIGURE_FAIL" | "BOTH_MAKE_FAIL" => "both-fail",
                _ => if ours_ok { "ours-ok" } else { "not-compared" },
            }
        }
        None => "not-compared",
    };
    let output_match = match (status, vs) {
        ("FUNC_OK", "identical-status") => "status-match",
        ("FUNC_OK", _) => "ours-built",
        (_, "both-fail") => "both-fail",
        _ => "not-compared",
    };
    Verification { replay_success: None, vs_gnu: vs.to_string(), drift_noise: Vec::new(), output_match: output_match.to_string(), test_suite_pass_rate: None }
}

/// v3.1: dialect reconciliation policy — what std tier + poison-flag strip the project needs.
fn analyze_dialect_reconciliation(ac_text: &str, ls: &Option<LanguageSurface>) -> Option<DialectReconciliation> {
    let sets_std = ls.as_ref().map(|l| l.sets_c_std).unwrap_or(false);
    let needs_cxx = ls.as_ref().map(|l| l.needs_cxx).unwrap_or(false);
    let tier = if ac_text.contains("AC_PROG_CC_C99") { "c99" }
        else if ac_text.contains("AC_PROG_CC_C11") { "c11" }
        else if ac_text.contains("AX_CXX_COMPILE_STDCXX") || needs_cxx { "gnu++" }
        else if sets_std { "explicit" } else { "c89_gnu" };
    // legacy projects with no explicit std die on modern default-strict -> strip + shim
    let inject = !sets_std && tier == "c89_gnu";
    let strip = if inject { vec![
        "-Werror=implicit-function-declaration".to_string(),
        "-Werror=int-conversion".to_string(),
        "-Werror=incompatible-pointer-types".to_string(),
    ] } else { Vec::new() };
    let mut aliasing = BTreeMap::new();
    if inject {
        aliasing.insert("gcc-14+".to_string(), "-std=gnu89 -fpermissive".to_string());
        aliasing.insert("clang-18+".to_string(), "-std=gnu89 -Wno-error".to_string());
    }
    Some(DialectReconciliation { enforce_standards_tier: tier.into(), strip_modern_poison_flags: strip, inject_legacy_shims: inject, compiler_aliasing: aliasing })
}

/// v3.1: m4 side-effect isolation hazards.
fn analyze_m4_isolation(ac_text: &str, mi: &Option<MacroInventory>) -> Option<M4SideEffectIsolation> {
    // unquoted AC_SUBST inside a conditional (if/case branch) — crude: AC_SUBST not wrapped in [] within a conditional line region
    let mut unquoted_in_cond = 0usize;
    let mut depth = 0i32;
    for l in ac_text.lines() {
        let t = l.trim();
        if t.starts_with("if ") || t.starts_with("case ") || t.starts_with("AS_IF") || t.starts_with("AM_COND") { depth += 1; }
        if t == "fi" || t == "esac" || t.starts_with("])") { depth = (depth - 1).max(0); }
        if depth > 0 && t.contains("AC_SUBST(") && !t.contains("AC_SUBST([") { unquoted_in_cond += 1; }
    }
    let shadowed: Vec<String> = mi.as_ref().map(|m| m.defined_macros.iter()
        .filter(|dm| (dm.name.starts_with("AC_") || dm.name.starts_with("AM_")) && dm.kind != "project-local")
        .map(|dm| dm.name.clone()).take(15).collect()).unwrap_or_default();
    let permitted = ac_text.matches("AC_ARG_ENABLE").count() + ac_text.matches("AC_ARG_WITH").count();
    Some(M4SideEffectIsolation {
        unquoted_subst_in_conditional: unquoted_in_cond,
        shadowed_builtins: shadowed,
        permitted_mutations: permitted,
        suspect_global_mutations: 0,
    })
}

/// v3.1: parallel-build safety from vpath_analysis + directory build dirs.
fn analyze_parallel_build_safety(vp: &Option<VpathAnalysis>) -> Option<ParallelBuildSafety> {
    let vp = vp.as_ref()?;
    let gens: Vec<String> = vp.generated_source_targets.iter().take(15).cloned().collect();
    // generated sources not in BUILT_SOURCES -> race risk
    let unordered: Vec<String> = vp.generated_source_targets.iter()
        .filter(|g| !vp.built_sources.iter().any(|b| g.contains(b.trim_end_matches(['.', 'c', 'h']))))
        .take(15).cloned().collect();
    Some(ParallelBuildSafety {
        vpath_out_of_tree_safe: vp.abs_path_leakage == 0 && vp.hardcoded_src_paths < 3,
        generators: gens,
        unordered_generated_sources: if vp.built_sources.is_empty() { vp.generated_source_targets.iter().take(15).cloned().collect() } else { unordered },
        built_sources_declared: !vp.built_sources.is_empty(),
    })
}

/// v3.1: host environment veil — drifted/mockable headers + obsolete symbols the code uses.
fn analyze_host_veil(fpg: &Option<FeatureProbeGap>, sc_undef: &[String]) -> Option<HostEnvironmentVeil> {
    // headers commonly drifted / needing fallback
    const DRIFT: &[&str] = &["sys/sysctl.h", "sys/cdefs.h", "malloc.h", "values.h", "varargs.h", "sys/errno.h", "linux/sysctl.h", "sys/timeb.h"];
    let mut hdr = Vec::new();
    if let Some(f) = fpg {
        for h in f.headers_included_unchecked.iter().chain(f.headers_checked.iter()) {
            if DRIFT.contains(&h.as_str()) && !hdr.contains(h) { hdr.push(h.clone()); }
        }
    }
    // obsolete symbols commonly needing aliasing
    const OBSOLETE: &[&str] = &["sys_errlist", "sys_nerr", "gets", "bzero", "bcopy", "index", "rindex"];
    let mut sym: Vec<String> = sc_undef.iter().filter(|s| OBSOLETE.iter().any(|o| s.contains(o))).cloned().take(10).collect();
    for o in OBSOLETE { if sc_undef.iter().any(|s| s.contains(o)) && !sym.iter().any(|x| x.contains(o)) { sym.push(o.to_string()); } }
    if hdr.is_empty() && sym.is_empty() { return None; }
    Some(HostEnvironmentVeil { header_injection_candidates: hdr, symbol_aliasing_candidates: sym })
}

/// v3.1: semantic context — included headers + undefined/provided symbols from the make log + sources.
fn analyze_semantic_context(mk_log: &str, fpg: &Option<FeatureProbeGap>) -> Option<SemanticContext> {
    let mut undef = Vec::new();
    for l in mk_log.lines() {
        if let Some(p) = l.find("undefined reference to ") {
            let s = l[p + 23..].trim().trim_matches(['`', '\'', '"']);
            let s: String = s.chars().take(60).collect();
            if !s.is_empty() && undef.len() < 15 && !undef.contains(&s) { undef.push(s); }
        }
    }
    let included: Vec<String> = fpg.as_ref().map(|f| f.headers_included_unchecked.iter().chain(f.headers_checked.iter()).take(20).cloned().collect()).unwrap_or_default();
    if undef.is_empty() && included.is_empty() { return None; }
    Some(SemanticContext { included_headers_sample: included, undefined_symbols: undef, provided_symbols_sample: Vec::new(), llvm_native_preview: None })
}

/// v3.1: top-level risk factors — the brittle aspects, aggregated for fast triage.
fn compute_risk_factors(dc: &Option<DirectoryContext>, mi: &Option<MacroInventory>, lc: &Option<LibtoolContext>, gx: &Option<GettextIntlContext>, dr: &Option<DialectReconciliation>, prov: &Option<ProvenanceMap>, pbs: &Option<ParallelBuildSafety>) -> Vec<String> {
    let mut r = Vec::new();
    if dc.as_ref().map(|x| !x.config_h_consumers_below_root.is_empty()).unwrap_or(false) { r.push("subdir-config-h-include-path".into()); }
    if mi.as_ref().map(|x| !x.unresolved_macros.is_empty()).unwrap_or(false) { r.push("unresolved-macros".into()); }
    if lc.as_ref().map(|x| x.age == "old").unwrap_or(false) { r.push("ancient-libtool".into()); }
    if gx.as_ref().map(|x| !x.missing_files.is_empty()).unwrap_or(false) { r.push("gettext-support-files-missing".into()); }
    if dr.as_ref().map(|x| x.inject_legacy_shims).unwrap_or(false) { r.push("modern-compiler-strictness (no explicit C std)".into()); }
    if prov.as_ref().map(|x| x.m4_trace_depth > 30).unwrap_or(false) { r.push("deep-macro-expansion (stack/divergence risk)".into()); }
    if pbs.as_ref().map(|x| !x.unordered_generated_sources.is_empty()).unwrap_or(false) { r.push("parallel-build-race (gen-source not in BUILT_SOURCES)".into()); }
    if pbs.as_ref().map(|x| !x.vpath_out_of_tree_safe).unwrap_or(false) { r.push("vpath-unsafe (abs-path/hardcoded-srcdir leakage)".into()); }
    r
}

/// v3: VPATH / out-of-tree + artifact side-effect analysis from Makefile.am files + generated Makefile.
fn analyze_vpath(d: &Path) -> Option<VpathAnalysis> {
    let mut hardcoded = 0usize;
    let mut built_sources = Vec::new();
    let mut gen_targets = Vec::new();
    let mut abs_leak = 0usize;
    let mut scan = |path: &std::path::Path| {
        if let Ok(txt) = std::fs::read_to_string(path) {
            let flat = txt.replace("\\\n", " ");
            for l in flat.lines() {
                let t = l.trim();
                if t.contains("./") && (t.contains("$(") || t.starts_with("\t")) { hardcoded += 1; }
                if let Some(v) = t.strip_prefix("BUILT_SOURCES") {
                    for s in v.trim_start_matches([' ', '=', '+']).split_whitespace() { if built_sources.len() < 20 { built_sources.push(s.to_string()); } }
                }
                if (t.contains(".y") || t.contains(".l") || t.contains("yacc") || t.contains("lex") || t.contains("gperf")) && t.contains(':') {
                    let tgt = t.split(':').next().unwrap_or("").trim();
                    if !tgt.is_empty() && gen_targets.len() < 20 && !gen_targets.iter().any(|x| x == tgt) { gen_targets.push(tgt.to_string()); }
                }
            }
        }
    };
    // scan Makefile.am tree (depth-limited)
    fn walk_am(root: &Path, dir: &Path, depth: usize, f: &mut dyn FnMut(&std::path::Path)) {
        if depth > 4 { return; }
        let am = dir.join("Makefile.am");
        if am.is_file() { f(&am); }
        for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            let p = e.path();
            if p.is_dir() { let n = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(); if n == ".git" || n.starts_with('.') { continue; } walk_am(root, &p, depth + 1, f); }
        }
    }
    walk_am(d, d, 0, &mut scan);
    // abs-path leakage in generated top Makefile.in
    if let Ok(mki) = std::fs::read_to_string(d.join("Makefile.in")) {
        for l in mki.lines() { if l.contains("/home/") || l.contains("/tmp/") || l.contains("/root/") { abs_leak += 1; } }
    }
    if hardcoded == 0 && built_sources.is_empty() && gen_targets.is_empty() && abs_leak == 0 { return None; }
    Some(VpathAnalysis { hardcoded_src_paths: hardcoded, abs_path_leakage: abs_leak, built_sources, generated_source_targets: gen_targets })
}

/// v3: dynamic feature-probe gap — headers checked vs included; implicit -l libs.
fn analyze_feature_probe_gap(d: &Path, ac_text: &str) -> Option<FeatureProbeGap> {
    // headers checked via AC_CHECK_HEADERS / AC_CHECK_HEADER
    let mut checked = std::collections::BTreeSet::new();
    for mac in ["AC_CHECK_HEADERS", "AC_CHECK_HEADER"] {
        let mut hay = ac_text;
        while let Some(p) = hay.find(mac) {
            if let Some(op) = hay[p..].find('(') {
                let inner = &hay[p + op + 1..];
                if let Some(cl) = inner.find(')') {
                    for h in inner[..cl].replace(['[', ']', ','], " ").split_whitespace() {
                        if h.ends_with(".h") { checked.insert(h.to_string()); }
                    }
                }
            }
            hay = &hay[p + mac.len()..];
        }
    }
    // headers #included across sources (sampled), system-ish (contain / or angle includes)
    let mut included = std::collections::BTreeSet::new();
    fn walk_src(root: &Path, dir: &Path, depth: usize, inc: &mut std::collections::BTreeSet<String>) {
        if depth > 3 || inc.len() > 200 { return; }
        for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            let p = e.path();
            if p.is_dir() { let n = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(); if n == ".git" || n.starts_with('.') { continue; } walk_src(root, &p, depth + 1, inc); }
            else if matches!(p.extension().and_then(|s| s.to_str()), Some("c") | Some("h") | Some("cc") | Some("cpp")) {
                if let Ok(s) = std::fs::read_to_string(&p) {
                    for l in s.lines().take(120) {
                        let t = l.trim();
                        if let Some(r) = t.strip_prefix("#include <") {
                            if let Some(h) = r.split('>').next() { if h.ends_with(".h") { inc.insert(h.to_string()); } }
                        }
                    }
                }
            }
        }
    }
    walk_src(d, d, 0, &mut included);
    // included-but-unchecked SYSTEM headers (those with a / path or known system ones)
    let unchecked: Vec<String> = included.iter()
        .filter(|h| !checked.contains(*h) && (h.contains('/') || ["unistd.h","fcntl.h","sys/types.h","stdint.h","inttypes.h","dlfcn.h","pthread.h"].contains(&h.as_str())))
        .take(25).cloned().collect();
    // implicit -l libs in Makefile.am
    let mut implicit = std::collections::BTreeSet::new();
    if let Ok(am) = std::fs::read_to_string(d.join("Makefile.am")) {
        for tok in am.replace("\\\n", " ").split_whitespace() {
            if let Some(lib) = tok.strip_prefix("-l") { if !lib.is_empty() && lib.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') { implicit.insert(format!("-l{}", lib)); } }
        }
    }
    let checked_v: Vec<String> = checked.into_iter().take(40).collect();
    if checked_v.is_empty() && unchecked.is_empty() && implicit.is_empty() { return None; }
    Some(FeatureProbeGap { headers_checked: checked_v, headers_included_unchecked: unchecked, implicit_link_libs: implicit.into_iter().take(20).collect() })
}

/// v3: source→generated provenance — leaked-macro origins in configure.ac/m4, m4 trace depth, shadowed defs.
fn analyze_provenance(d: &Path, ac_text: &str, de: &Option<DeepExpansion>, mi: &Option<MacroInventory>) -> Option<ProvenanceMap> {
    // map each leaked macro to its configure.ac call site
    let mut origins = Vec::new();
    if let Some(dx) = de {
        for lm in &dx.leaked_macros {
            // find the macro name in configure.ac -> line
            if let Some(line) = ac_text.lines().position(|l| l.contains(&lm.name)) {
                origins.push(MacroOrigin { macro_name: lm.name.clone(), file: "configure.ac".into(), line: line + 1 });
            }
            if origins.len() >= 20 { break; }
        }
    }
    // m4 trace depth: max nested AC_DEFUN/define depth across m4 files (approx by brace/paren nesting of AC_DEFUN bodies)
    let mut max_depth = 0usize;
    let est_depth = |txt: &str| -> usize {
        let mut d = 0i32; let mut mx = 0i32;
        for ch in txt.chars() { if ch == '(' { d += 1; mx = mx.max(d); } else if ch == ')' { d -= 1; } }
        mx.max(0) as usize
    };
    max_depth = max_depth.max(est_depth(ac_text));
    for dir in ["m4", "build-aux/m4"] {
        for e in std::fs::read_dir(d.join(dir)).into_iter().flatten().flatten() {
            if e.path().extension().and_then(|s| s.to_str()) == Some("m4") {
                if let Ok(t) = std::fs::read_to_string(e.path()) { max_depth = max_depth.max(est_depth(&t)); }
            }
        }
    }
    // shadowed: locally-defined macros whose name is a STANDARD AC_/AM_ macro (override of systemic def)
    let shadowed: Vec<String> = mi.as_ref().map(|m| m.defined_macros.iter()
        .filter(|dm| (dm.name.starts_with("AC_") || dm.name.starts_with("AM_")) && dm.kind != "project-local")
        .map(|dm| dm.name.clone()).take(20).collect()).unwrap_or_default();
    if origins.is_empty() && max_depth == 0 && shadowed.is_empty() { return None; }
    Some(ProvenanceMap { configure_origins: origins, m4_trace_depth: max_depth, shadowed_macros: shadowed })
}

/// Forensic scan of the generated `configure` plus the configure run-log. Surfaces the exact
/// expansion failures (leaked macros, heredoc imbalance, syntax-error source context, malformed
/// cache vars, residual @VAR@) so each can be fixed from the recipe without re-running on the VM.
fn analyze_expansion(d: &Path, cf_log: &str) -> Option<DeepExpansion> {
    let cfg = std::fs::read_to_string(d.join("configure")).ok()?;
    let lines: Vec<&str> = cfg.lines().collect();
    let mut de = DeepExpansion {
        configure_lines: lines.len(),
        ..Default::default()
    };

    const MACRO_PREFIXES: &[&str] =
        &["AC_", "AX_", "AM_", "LT_", "PKG_", "AS_", "AH_", "_AC_", "_AX_", "_LT_", "m4_", "AT_"];
    for (i, l) in lines.iter().enumerate() {
        let t = l.trim_start();
        // A leaked macro call: <PREFIX><UPPER/under name>( at the start of a shell statement.
        if let Some(pfx) = MACRO_PREFIXES.iter().find(|p| t.starts_with(**p)) {
            // name = leading run of [A-Za-z0-9_], must be followed by '(' to be a macro call.
            let name: String = t.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
            let after = &t[name.len()..];
            if after.starts_with('(') && name.len() > pfx.len() && de.leaked_macros.len() < 40 {
                de.leaked_macros.push(LeakedMacro {
                    name,
                    line: i + 1,
                    context: t.chars().take(72).collect(),
                });
            }
        }
        // Heredoc accounting: count ALL `<<_ACEOF` openers (conftest probes, config.status, help)
        // vs lone `_ACEOF` terminators. A genuine imbalance == a missing opener (the compile-probe
        // emitted as raw shell -> `syntax error near '('`), not just legit multi-heredoc usage.
        if l.contains("<<_ACEOF") || l.contains("<<\\_ACEOF") {
            de.heredoc_openers += 1;
        }
        if l.trim() == "_ACEOF" {
            de.heredoc_terminators += 1;
        }
        // Malformed cache-var refs.
        if t.contains("${") {
            if let Some(rest) = t.split("${").nth(1) {
                let head = rest.chars().take(2).collect::<String>();
                if head.starts_with('$') || head.starts_with(' ') || head.starts_with('}') {
                    if de.cache_var_anomalies.len() < 20 {
                        de.cache_var_anomalies.push(format!("line {}: {}", i + 1, t.chars().take(60).collect::<String>()));
                    }
                }
            }
        }
        // Residual substitution placeholders (@VAR@ that config.status never filled).
        let mut rest = *l;
        while let Some(a) = rest.find('@') {
            let tail = &rest[a + 1..];
            if let Some(b) = tail.find('@') {
                let var = &tail[..b];
                if !var.is_empty()
                    && var.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    && var.chars().next().map(|c| c.is_ascii_alphabetic() || c == '_').unwrap_or(false)
                    && de.residual_placeholders.len() < 30
                    && !de.residual_placeholders.iter().any(|s| s == var)
                {
                    de.residual_placeholders.push(var.to_string());
                }
                rest = &tail[b + 1..];
            } else {
                break;
            }
        }
    }
    de.heredoc_imbalance = de.heredoc_terminators as i64 - de.heredoc_openers as i64;

    // Conftest archaeology: detect C preprocessor directives mangled by m4 (the deep autotools bug
    // where `include`/`ifdef`/`define` builtins expand inside conftest C source). Track only inside
    // conftest heredoc regions so real `# comment` shell lines aren't misread.
    {
        // in_conftest spans ONLY a real conftest heredoc: `cat … <<_ACEOF >conftest.$ac_ext` … `_ACEOF`
        // (not the prologue's `/* confdefs.h */` init, which would flag real shell comments).
        let mut in_conftest = false;
        let mut cur_program: Vec<String> = Vec::new();
        for (i, l) in lines.iter().enumerate() {
            let t = l.trim_start();
            if l.contains("<<_ACEOF") && (l.contains(">conftest") || l.contains("confdefs.h -")) {
                in_conftest = true;
                cur_program.clear();
            } else if l.trim() == "_ACEOF" {
                if in_conftest && !cur_program.is_empty() && de.conftest_programs.len() < 6 {
                    de.conftest_programs.push(cur_program.join("\\n"));
                }
                in_conftest = false;
            } else if in_conftest && cur_program.len() < 12 {
                cur_program.push(l.trim().chars().take(76).collect());
            }
            if t.starts_with("#include") || t.starts_with("#ifdef") || t.starts_with("#ifndef")
                || t.starts_with("#define") || t.starts_with("#if ") || t.starts_with("#endif") {
                de.conftest_directives_intact += 1;
            }
            // Mangled directive inside a conftest. Only the UNAMBIGUOUS forms are flagged: `# <hdr>`
            // (an eaten `#include <...>`) and `# "hdr"` (eaten `#include "..."`). The earlier `# WORD`
            // heuristic (eaten `#ifdef`/`#define`) is indistinguishable from a legitimate shell comment
            // (e.g. AC_DEFINE's `# Define unquoted: USE_STRUCT_MNTTAB`) and produced 55 false positives,
            // so it's dropped — `#ifdef`/`#define` corruption is still implied when `# <` co-occurs.
            if in_conftest && (t.starts_with("# <") || t.starts_with("# \"")) {
                de.conftest_directives_mangled += 1;
                if de.conftest_corruption.len() < 20 {
                    de.conftest_corruption.push(format!("line {}: {} (include eaten)", i + 1, t.chars().take(40).collect::<String>()));
                }
            }
        }
    }

    // Cross-reference each "./configure: line N: syntax error" with its source line.
    for ll in cf_log.lines() {
        if ll.contains("syntax error") {
            if let Some(npos) = ll.find("line ") {
                let num: String = ll[npos + 5..].chars().take_while(|c| c.is_ascii_digit()).collect();
                if let Ok(n) = num.parse::<usize>() {
                    let token = ll
                        .split("unexpected token")
                        .nth(1)
                        .map(|s| s.trim().trim_matches('`').trim_matches('\'').chars().take(24).collect::<String>())
                        .unwrap_or_default();
                    let source = lines.get(n.saturating_sub(1)).map(|s| s.trim().chars().take(72).collect()).unwrap_or_default();
                    // capture the enclosing construct: 4 lines before .. 3 after the error line
                    let lo = n.saturating_sub(5);
                    let hi = (n + 3).min(lines.len());
                    let block: Vec<String> = (lo..hi)
                        .map(|j| format!("{}: {}", j + 1, lines[j].chars().take(76).collect::<String>()))
                        .collect();
                    if de.syntax_errors.len() < 20 {
                        de.syntax_errors.push(SyntaxError { line: n, token, source, block });
                    }
                }
            }
        }
    }
    // Name the probe ours died on: the last `checking …` message printed before the run ended, plus
    // the few lines after it (the crash fallout). configure prints "checking X... " with no newline,
    // so the message + the failure often share a line; we split on "checking" to recover it.
    {
        let log_lines: Vec<&str> = cf_log.lines().collect();
        let mut last_check_idx = None;
        for (i, l) in log_lines.iter().enumerate() {
            if l.contains("checking ") {
                last_check_idx = Some(i);
            }
        }
        if let Some(i) = last_check_idx {
            if let Some(pos) = log_lines[i].find("checking ") {
                de.failed_during_check = log_lines[i][pos..].chars().take(80).collect();
            }
            de.failure_tail = log_lines[i..]
                .iter()
                .take(5)
                .map(|s| s.trim().chars().take(80).collect::<String>())
                .filter(|s| !s.is_empty())
                .collect();
        } else if !log_lines.is_empty() {
            de.failure_tail = log_lines.iter().rev().take(4).rev()
                .map(|s| s.trim().chars().take(80).collect::<String>())
                .filter(|s| !s.is_empty()).collect();
        }
    }
    Some(de)
}

/// Fingerprint the build environment (hermeticity layer): the exact toolchain + the env vars that
/// shape a build, so a recipe is reproducible across machines.
fn fingerprint_environment() -> Environment {
    let run = |prog: &str, args: &[&str]| -> String {
        Command::new(prog).args(args).output().ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).lines().next().unwrap_or("").trim().to_string())
            .unwrap_or_default()
    };
    let mut relevant_env = Vec::new();
    let mut env_vars_influential = BTreeMap::new();
    for k in ["CC", "CXX", "CFLAGS", "CXXFLAGS", "CPPFLAGS", "LDFLAGS", "LIBS", "PKG_CONFIG_PATH", "CPATH", "LIBRARY_PATH", "ACLOCAL_PATH", "AUTOMAKE_JOBS"] {
        if let Ok(v) = std::env::var(k) {
            if !v.is_empty() { relevant_env.push(format!("{}={}", k, v)); env_vars_influential.insert(k.to_string(), v); }
        }
    }
    // libc detection (glibc via ldd --version; else musl)
    let ldd = run("ldd", &["--version"]);
    let (libc_name, libc_version) = if ldd.to_lowercase().contains("musl") { ("musl".to_string(), String::new()) }
        else if ldd.to_lowercase().contains("glibc") || ldd.to_lowercase().contains("gnu libc") {
            ("glibc".to_string(), ldd.rsplit(' ').next().unwrap_or("").to_string())
        } else { ("unknown".to_string(), String::new()) };
    // POSIX flavor: GNU coreutils vs BSD (sed --version succeeds on GNU)
    let sed_v = run("sed", &["--version"]);
    let posix_flavor = if sed_v.to_lowercase().contains("gnu") { "gnu" } else if sed_v.is_empty() { "bsd-or-busybox" } else { "unknown" }.to_string();
    let shell = std::fs::read_link("/bin/sh").ok().map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|| "/bin/sh".into());
    let pkg_config_path: Vec<String> = std::env::var("PKG_CONFIG_PATH").unwrap_or_default().split(':').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
    let mut oracle_tool_versions = BTreeMap::new();
    for (k, prog) in [("autoconf", "autoconf"), ("automake", "automake"), ("m4", "m4"), ("perl", "perl"), ("libtool", "libtool")] {
        let v = run(prog, &["--version"]);
        if !v.is_empty() { oracle_tool_versions.insert(k.to_string(), v.rsplit(' ').next().unwrap_or("").to_string()); }
    }
    let env_var_whitelist = vec!["ACLOCAL_PATH".to_string(), "AUTOMAKE_JOBS".to_string(), "PKG_CONFIG_PATH".to_string(), "CC".to_string(), "CFLAGS".to_string()];
    Environment {
        cc: std::env::var("CC").unwrap_or_else(|_| "cc".into()),
        cc_version: run("cc", &["--version"]),
        host_triplet: run("cc", &["-dumpmachine"]),
        pkg_config_version: run("pkg-config", &["--version"]),
        make_version: run("make", &["--version"]),
        relevant_env,
        kernel_version: run("uname", &["-r"]),
        libc_name,
        libc_version,
        pkg_config_path,
        env_vars_influential,
        posix_flavor,
        shell,
        oracle_tool_versions,
        env_var_whitelist,
    }
}

/// Probe-failure root-cause: turn the generated config.h HAVE_* results + the configure run-log into a
/// per-probe trace that says WHY each probe passed or failed (header not found, symbol not found, link
/// failed) rather than just yes/no.
fn build_probe_trace(probes: &BTreeMap<String, u8>, cf_log: &str) -> Vec<ProbeStep> {
    let log_l = cf_log.to_lowercase();
    let mut trace = Vec::new();
    for (name, &val) in probes.iter() {
        let yes = val == 1;
        // derive kind from the HAVE_ name
        let kind = if name.ends_with("_H") || name.contains("HEADER") { "header" }
            else if name.starts_with("HAVE_LIB") { "lib" }
            else { "func" };
        let reason = if yes { "ok".to_string() } else {
            // why did it fail? look for a clue in the run log
            let stem = name.trim_start_matches("HAVE_").to_lowercase();
            if log_l.contains(&format!("{}: no such file", stem.replace('_', "."))) { "header-not-found".into() }
            else if kind == "lib" && log_l.contains("cannot find -l") { "link-failed".into() }
            else if log_l.contains("undefined reference") { "symbol-not-found".into() }
            else { "probe-returned-no".into() }
        };
        if trace.len() < 200 {
            trace.push(ProbeStep { name: name.clone(), kind: kind.into(), result: if yes {"yes".into()} else {"no".into()}, reason });
        }
    }
    trace
}

/// Missing-dep inference: for failed header/lib probes (and any "X.h: No such file" in the log),
/// suggest the distro package that would satisfy it.
fn infer_missing_deps(hdrs_needed: &[String], cf_log: &str) -> Vec<SuggestedDep> {
    // header/lib stem -> providing package (Debian/Ubuntu names; the common autotools deps)
    const PKGS: &[(&str, &str, &str)] = &[
        ("zlib.h", "header", "zlib1g-dev"), ("zconf.h", "header", "zlib1g-dev"),
        ("openssl/", "header", "libssl-dev"), ("curl/", "header", "libcurl4-openssl-dev"),
        ("pcre.h", "header", "libpcre3-dev"), ("pcre2.h", "header", "libpcre2-dev"),
        ("fuse.h", "header", "libfuse-dev"), ("fuse3/", "header", "libfuse3-dev"),
        ("ncurses.h", "header", "libncurses-dev"), ("curses.h", "header", "libncurses-dev"),
        ("sqlite3.h", "header", "libsqlite3-dev"), ("expat.h", "header", "libexpat1-dev"),
        ("libxml/", "header", "libxml2-dev"), ("png.h", "header", "libpng-dev"),
        ("jpeglib.h", "header", "libjpeg-dev"), ("ffi.h", "header", "libffi-dev"),
        ("gmp.h", "header", "libgmp-dev"), ("readline/", "header", "libreadline-dev"),
        ("libusb", "header", "libusb-1.0-0-dev"), ("alsa/", "header", "libasound2-dev"),
        ("X11/", "header", "libx11-dev"), ("GL/", "header", "libgl-dev"),
        ("dbus/", "header", "libdbus-1-dev"), ("systemd/", "header", "libsystemd-dev"),
        ("pcap.h", "header", "libpcap-dev"), ("ldns/", "header", "libldns-dev"),
        ("cryptsetup", "header", "libcryptsetup-dev"), ("gcrypt.h", "header", "libgcrypt20-dev"),
        ("event.h", "header", "libevent-dev"), ("json-c/", "header", "libjson-c-dev"),
    ];
    let log_l = cf_log.to_lowercase();
    let mut out: Vec<SuggestedDep> = Vec::new();
    let mut consider = |needle: &str| {
        for (stem, kind, pkg) in PKGS {
            if needle.contains(stem) && !out.iter().any(|s| s.package == *pkg) {
                out.push(SuggestedDep { missing: needle.to_string(), kind: (*kind).to_string(), package: (*pkg).to_string() });
            }
        }
    };
    for h in hdrs_needed { consider(&h.to_lowercase()); }
    // also scan the run log for "X.h: No such file"
    for l in log_l.lines() {
        if l.contains("no such file") && l.contains(".h") {
            if let Some(h) = l.split(':').next() { consider(h.trim()); }
        }
    }
    out.truncate(20);
    out
}

/// Quirk rule engine: versioned heuristics matched on configure.ac text, file presence, or run-log.
/// Each fired rule is recorded so we know which quirks a build depended on.
fn match_quirks(ac_text: &str, d: &Path, cf_log: &str) -> Vec<String> {
    // (id, predicate-kind, needle): kind a=ac_text, f=file-exists, l=run-log
    const RULES: &[(&str, char, &str)] = &[
        ("uses-libtool", 'a', "LT_INIT"),
        ("uses-libtool-old", 'a', "AC_PROG_LIBTOOL"),
        ("uses-pkg-config", 'a', "PKG_CHECK_MODULES"),
        ("uses-intltool", 'a', "IT_PROG_INTLTOOL"),
        ("uses-gettext", 'a', "AM_GNU_GETTEXT"),
        ("uses-python", 'a', "AM_PATH_PYTHON"),
        ("uses-subdir-objects", 'a', "subdir-objects"),
        ("uses-maintainer-mode", 'a', "AM_MAINTAINER_MODE"),
        ("has-m4-macro-dir", 'f', "m4"),
        ("has-acinclude", 'f', "acinclude.m4"),
        ("vendored-aclocal", 'f', "aclocal.m4"),
        ("uses-ax-archive", 'a', "AX_"),
        ("uses-pthread-check", 'a', "AX_PTHREAD"),
        ("emits-config-commands-post", 'a', "AC_CONFIG_COMMANDS_POST"),
        ("perl-in-configure", 'a', "PERL"),
    ];
    let mut out = Vec::new();
    for (id, kind, needle) in RULES {
        let hit = match kind {
            'a' => ac_text.contains(needle),
            'f' => d.join(needle).exists(),
            'l' => cf_log.contains(needle),
            _ => false,
        };
        if hit { out.push(id.to_string()); }
    }
    out
}

/// GNU-free fix for a matched quirk: a `./configure` flag (or empty if the quirk has no auto-fix).
/// Never copies GNU aux or invokes GNU tools — only flags/env/mkdir are permitted.
fn quirk_fix(id: &str) -> Option<String> {
    match id {
        "uses-maintainer-mode" => Some("--disable-maintainer-mode".into()),
        "uses-subdir-objects" => Some("--disable-dependency-tracking".into()),
        _ => None,
    }
}

/// Auto-apply quirks: when configure failed, collect GNU-free configure flags from the matched fixable
/// quirks, re-run configure (+make), and record which quirks actually got the build further. Returns
/// (applied-with-verdict, possibly-improved-status). `ok_inc` is set true if it newly reached FUNC_OK.
fn auto_apply_quirks(d: &Path, matched: &[String], cur_status: &str) -> (Vec<QuirkApplied>, String, bool) {
    let mut applied = Vec::new();
    let fixes: Vec<(String, String)> = matched
        .iter()
        .filter_map(|id| quirk_fix(id).map(|f| (id.clone(), f)))
        .collect();
    if fixes.is_empty() || matches!(cur_status, "MAKE_FAIL" | "FUNC_OK") {
        return (applied, cur_status.to_string(), false);
    }
    let flag_args: Vec<&str> = fixes.iter().map(|(_, f)| f.as_str()).collect();
    let (cfok, _) = run_timed(d, 180, "./configure", &flag_args);
    let mut new_status = cur_status.to_string();
    let mut ok_inc = false;
    if cfok {
        new_status = "MAKE_FAIL".to_string();
        let (mkok, _) = run_timed(d, 300, "make", &["-j2"]);
        if mkok { new_status = "FUNC_OK".to_string(); ok_inc = true; }
    }
    // verified = applying the quirks actually cleared configure (which had failed before)
    for (id, action) in fixes {
        applied.push(QuirkApplied { id, action, verified: cfok });
    }
    (applied, new_status, ok_inc)
}

/// Build-court verdict for a recipe.
fn court_status(status: &str, oracle: &Option<Oracle>, quirks: &[String]) -> String {
    let cls = oracle.as_ref().map(|o| o.classification.as_str()).unwrap_or("");
    if status == "FUNC_OK" {
        if quirks.is_empty() { "sealed".into() } else { "quirk_dependent".into() }
    } else if status == "MAKE_FAIL" {
        "partial".into()
    } else if cls == "NOT_STANDALONE" || cls.starts_with("BOTH_") {
        "not_standalone".into()
    } else {
        "failed".into()
    }
}

/// sha256 hash-chain over the recipe's load-bearing fields — the sealed-receipt anchor.
fn compute_receipt_hash(
    toolchain: &Toolchain, probes: &BTreeMap<String, u8>, outputs: &[Output],
    oracle: &Option<Oracle>, court: &str,
) -> String {
    let mut h = Sha256::new();
    h.update(format!("ac={} am={} m4={}\n", toolchain.autoconf_rs, toolchain.automake_rs, toolchain.m4_rs_core).as_bytes());
    for (k, v) in probes.iter() { h.update(format!("{}={}\n", k, v).as_bytes()); }
    for o in outputs { h.update(format!("{}@{}\n", o.path, o.sha256).as_bytes()); }
    if let Some(o) = oracle { h.update(format!("oracle={}\n", o.classification).as_bytes()); }
    h.update(format!("court={}\n", court).as_bytes());
    format!("{:x}", h.finalize())
}

fn collect(d: &Path, cf: &str, mk: &str) -> (BTreeMap<String, u8>, Vec<String>, Vec<String>, Vec<String>) {
    // probe results from the generated config header
    let mut probes = BTreeMap::new();
    for entry in std::fs::read_dir(d).into_iter().flatten().flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if (name == "config.h" || name.ends_with("config.h")) && !name.ends_with(".in") {
            if let Ok(txt) = std::fs::read_to_string(entry.path()) {
                for l in txt.lines() {
                    if let Some(rest) = l.strip_prefix("#define HAVE_") {
                        if let Some(var) = rest.split_whitespace().next() {
                            probes.insert(format!("HAVE_{}", var), 1);
                        }
                    }
                }
            }
        }
    }
    let libs: Vec<String> = std::fs::read_to_string(d.join("Makefile"))
        .unwrap_or_default()
        .lines()
        .find(|l| l.starts_with("LIBS ="))
        .map(|l| l.trim_start_matches("LIBS =").split_whitespace().map(|s| s.to_string()).collect())
        .unwrap_or_default();
    let mut hdrs: Vec<String> = Vec::new();
    for log in [cf, mk] {
        for l in log.lines() {
            if let Some(idx) = l.find(".h: No such file") {
                let pre = &l[..idx + 2];
                if let Some(h) = pre.rsplit([' ', '"', '<', ':']).next() {
                    if h.ends_with(".h") && !hdrs.contains(&h.to_string()) {
                        hdrs.push(h.to_string());
                    }
                }
            }
        }
    }
    let mut pkgs: Vec<String> = Vec::new();
    for l in cf.lines() {
        if l.contains("pkg-config") || l.contains(">= ") {
            if pkgs.len() < 8 {
                pkgs.push(l.trim().chars().take(80).collect());
            }
        }
    }
    (probes, libs, hdrs, pkgs)
}

fn collect_outputs(d: &Path) -> Vec<Output> {
    let mut outs = Vec::new();
    walk(d, d, &mut outs, 0);
    outs.truncate(12);
    outs
}
fn walk(root: &Path, dir: &Path, outs: &mut Vec<Output>, depth: usize) {
    if depth > 3 || outs.len() >= 12 {
        return;
    }
    for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = e.path();
        let name = e.file_name().to_string_lossy().into_owned();
        if name == ".git" {
            continue;
        }
        if p.is_dir() {
            walk(root, &p, outs, depth + 1);
        } else {
            let is_lib = name.ends_with(".a") || name.contains(".so");
            let is_exe = !name.contains('.')
                && std::fs::metadata(&p)
                    .map(|m| {
                        use std::os::unix::fs::PermissionsExt;
                        m.permissions().mode() & 0o111 != 0
                    })
                    .unwrap_or(false);
            if (is_lib || is_exe) && outs.len() < 12 {
                if let Some(h) = sha256_16(&p) {
                    outs.push(Output {
                        path: p.strip_prefix(root).unwrap_or(&p).display().to_string(),
                        sha256: h,
                        kind: if is_lib { "library".into() } else { "executable".into() },
                    });
                }
            }
        }
    }
}

fn diag_line(cf: &str, mk: &str) -> String {
    for log in [mk, cf] {
        for l in log.lines() {
            let ll = l.to_lowercase();
            if ll.contains("confdefs.h: no such file") { continue; }
            if ll.contains("configure: error")
                || ll.contains("syntax error")
                || ll.contains("command not found")
                || ll.contains(": error:")
                || ll.contains("no such file")
                // make-layer error formats (were missed -> 302/366 partial repos had no diagnostic):
                || ll.contains("undefined reference")
                || ll.contains("no rule to make target")
                || ll.contains("missing separator")
                || ll.contains("recipe for target")
                || ll.contains("] error ")            // `make[1]: *** [foo] Error 1`
                || ll.contains("*** ")                // make fatal
                || ll.contains("not found")           // `<tool>: not found`
                || ll.contains("permission denied")
            {
                return l.trim().chars().take(140).collect();
            }
        }
    }
    // Fallback: if make produced output but matched no known pattern, return its last non-empty line —
    // better than an empty diagnostic (which dumped 302 partial repos into "no diagnostic captured").
    for l in mk.lines().rev() {
        let t = l.trim();
        if !t.is_empty() { return t.chars().take(140).collect(); }
    }
    String::new()
}

/// `xtask atlas-replay <recipe.json> [--keep]` — REPRODUCER + regression gate. Reads a recipe, clones
/// the repo at the pinned git_sha into a clean dir, re-applies the recorded pass_pipeline + the
/// configure flags from feature_flags/quirks_applied, rebuilds, and verifies the rebuilt artifacts'
/// sha256s against the recipe's `outputs`. Emits a replay receipt (reproduced | diverged | build_failed
/// | clone_failed) and exits non-zero on anything but a clean reproduction — so it gates regressions.
pub fn replay() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let arg = match args.get(2) {
        Some(p) => p.clone(),
        None => {
            eprintln!("usage: cargo xtask atlas-replay <recipe.json | owner/name | slug> [--keep]");
            eprintln!("  resolves a recipe by path, by `owner/name`, or by `owner__name` slug under atlas/recipes/");
            return ExitCode::from(2);
        }
    };
    let keep = args.iter().any(|a| a == "--keep");
    // Resolve the recipe: an existing path, else a repo slug under atlas/recipes/ (owner/name or owner__name).
    let recipe_path = {
        let direct = PathBuf::from(&arg);
        if direct.is_file() {
            direct
        } else {
            let slug = arg.replace('/', "__");
            let cand = PathBuf::from("atlas/recipes").join(format!("{}.json", slug));
            if cand.is_file() { cand } else {
                eprintln!("atlas-replay: no recipe found for '{}' (tried {} and atlas/recipes/{}.json)", arg, direct.display(), slug);
                return ExitCode::from(2);
            }
        }
    };
    eprintln!("atlas-replay: recipe {}", recipe_path.display());
    let v: serde_json::Value = match std::fs::read_to_string(&recipe_path).ok().and_then(|s| serde_json::from_str(&s).ok()) {
        Some(v) => v,
        None => { eprintln!("atlas-replay: cannot read/parse {}", recipe_path.display()); return ExitCode::from(2); }
    };

    let repo = v["repo"].as_str().unwrap_or("").to_string();
    let url = v["source"]["url"].as_str().filter(|s| !s.is_empty()).map(|s| s.to_string())
        .unwrap_or_else(|| format!("https://github.com/{}", repo));
    let pinned_sha = v["source"]["git_sha"].as_str().unwrap_or("").to_string();
    // configure flags: feature_flags.configure_args + any --flag actions the recipe's quirks applied
    let mut cfg_args: Vec<String> = v["feature_flags"]["configure_args"].as_array().into_iter().flatten()
        .filter_map(|x| x.as_str().map(|s| s.to_string())).collect();
    if let Some(qa) = v["receipt"]["quirks_applied"].as_array() {
        for q in qa { if let Some(a) = q["action"].as_str() { if a.starts_with("--") && !cfg_args.iter().any(|c| c == a) { cfg_args.push(a.to_string()); } } }
    }
    let recipe_outputs: BTreeMap<String, String> = v["outputs"].as_array().into_iter().flatten()
        .filter_map(|o| Some((o["path"].as_str()?.to_string(), o["sha256"].as_str()?.to_string()))).collect();

    // autoreconf-rs resolves AUTOCONF_RS / AUTOMAKE_RS from the env itself (same as the atlas run).
    let ars = tool("AUTORECONF_RS", "autoreconf-rs");

    let base = std::env::temp_dir().join(format!("atlasreplay_{}", repo.replace('/', "__")));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).ok();
    let d = base.join("s");

    let mut steps: Vec<serde_json::Value> = Vec::new();
    let mut replay_status = "clone_failed";
    let mut sha_used = String::new();

    eprintln!("  [1/5] clone {} ...", url);
    let (cloned, _) = run_timed(&base, 120, "git", &["clone", "-q", &url, d.to_str().unwrap()]);
    eprintln!("        clone: {}", if cloned { "ok" } else { "FAIL" });
    steps.push(serde_json::json!({"step": "clone", "status": if cloned {"ok"} else {"fail"}}));
    let mut out_compare = serde_json::json!({});
    let mut matched = 0usize; let mut mismatched = 0usize; let mut missing = 0usize;
    if cloned {
        // pin to the recorded sha for a faithful replay; fall back to HEAD if the sha is gone.
        let pin_ok = !pinned_sha.is_empty() && Command::new("git").args(["checkout", "-q", &pinned_sha]).current_dir(&d).status().map(|s| s.success()).unwrap_or(false);
        sha_used = if pin_ok { pinned_sha.clone() } else {
            Command::new("git").args(["rev-parse", "HEAD"]).current_dir(&d).output().ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default()
        };
        eprintln!("  [2/5] checkout {} ({})", &sha_used[..sha_used.len().min(12)], if pin_ok { "pinned" } else { "HEAD fallback — pinned sha gone" });
        steps.push(serde_json::json!({"step": "checkout", "sha": sha_used, "pinned": pin_ok, "status": if pin_ok {"ok"} else {"head-fallback"}}));

        replay_status = "build_failed";
        eprintln!("  [3/5] autoreconf-rs -fi ...");
        let (_a, _) = run_timed(&d, 150, &ars, &["-fi", "."]);
        let gen_ok = d.join("configure").exists();
        eprintln!("        autoreconf: {}", if gen_ok { "ok (configure generated)" } else { "FAIL (no configure)" });
        steps.push(serde_json::json!({"step": "autoreconf", "tool": ars, "status": if gen_ok {"ok"} else {"fail"}}));
        if gen_ok {
            eprintln!("  [4/5] ./configure {} ...", cfg_args.join(" "));
            let argref: Vec<&str> = cfg_args.iter().map(|s| s.as_str()).collect();
            let (cfok, cfl) = run_timed(&d, 180, "./configure", &argref);
            eprintln!("        configure: {}", if cfok { "ok" } else { "FAIL" });
            steps.push(serde_json::json!({"step": "configure", "args": cfg_args, "status": if cfok {"ok"} else {"fail"}}));
            if cfok {
                eprintln!("  [5/5] make -j2 ...");
                let (mkok, mkl) = run_timed(&d, 300, "make", &["-j2"]);
                eprintln!("        make: {}", if mkok { "ok" } else { "FAIL" });
                steps.push(serde_json::json!({"step": "make", "status": if mkok {"ok"} else {"fail"}}));
                let _ = (cfl, mkl);
                // verify: rebuilt artifacts vs recipe outputs (by path -> sha256)
                let rebuilt = collect_outputs(&d);
                let rebuilt_map: BTreeMap<String, String> = rebuilt.iter().map(|o| (o.path.clone(), o.sha256.clone())).collect();
                let mut details = Vec::new();
                for (path, want) in &recipe_outputs {
                    match rebuilt_map.get(path) {
                        Some(got) if got == want => { matched += 1; details.push(serde_json::json!({"path": path, "verdict": "match"})); }
                        Some(got) => { mismatched += 1; details.push(serde_json::json!({"path": path, "verdict": "hash-mismatch", "recipe": want, "replay": got})); }
                        None => { missing += 1; details.push(serde_json::json!({"path": path, "verdict": "missing-in-replay"})); }
                    }
                }
                out_compare = serde_json::json!({"matched": matched, "mismatched": mismatched, "missing": missing, "details": details});
                if !recipe_outputs.is_empty() {
                    eprintln!("        verify: {} matched, {} hash-mismatch, {} missing (of {} recorded outputs)", matched, mismatched, missing, recipe_outputs.len());
                }
                replay_status = if !mkok { "build_failed" }
                    else if recipe_outputs.is_empty() { "reproduced_no_outputs" }
                    else if mismatched == 0 && missing == 0 { "reproduced" }
                    else { "diverged" };
            }
        }
    }

    let receipt = serde_json::json!({
        "schema": "automake-rs.replay-receipt/v1",
        "repo": repo, "recipe": recipe_path.display().to_string(),
        "replay_status": replay_status,
        "pinned_sha": pinned_sha, "sha_replayed": sha_used,
        "configure_args": cfg_args,
        "steps": steps,
        "output_verification": out_compare,
    });
    let receipt_path = base.join("replay-receipt.json");
    let _ = std::fs::write(&receipt_path, serde_json::to_string_pretty(&receipt).unwrap_or_default() + "\n");
    println!("{}", serde_json::to_string_pretty(&receipt).unwrap_or_default());
    println!("\natlas-replay: {} — {} (receipt: {})", repo, replay_status, receipt_path.display());
    if !keep { let _ = std::fs::remove_dir_all(&base); }

    match replay_status {
        "reproduced" | "reproduced_no_outputs" => ExitCode::SUCCESS,
        _ => ExitCode::from(1), // gate: any divergence / failure is non-zero
    }
}

/// `xtask atlas-diff <baseline-dir> <experiment-dir>` — A/B compare two recipe sets by court verdict.
/// Reports flips (baseline-not-clear -> experiment-clear), regressions (clear -> not-clear), and the net,
/// over the repos present in BOTH sets. "Clear" = configure cleared (sealed | quirk_dependent | partial).
/// This is how a corpus-wide change (e.g. the leaked-macro neutralizer) is gated: ship only if net > 0.
pub fn diff() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let (a_dir, b_dir) = match (args.get(2), args.get(3)) {
        (Some(a), Some(b)) => (PathBuf::from(a), PathBuf::from(b)),
        _ => {
            eprintln!("usage: cargo xtask atlas-diff <baseline-dir> <experiment-dir>");
            return ExitCode::from(2);
        }
    };
    let load = |d: &Path| -> BTreeMap<String, String> {
        let mut m = BTreeMap::new();
        for e in std::fs::read_dir(d).into_iter().flatten().flatten() {
            let p = e.path();
            if p.extension().and_then(|s| s.to_str()) != Some("json")
                || p.file_name().and_then(|s| s.to_str()) == Some("INDEX.json")
            {
                continue;
            }
            if let Ok(txt) = std::fs::read_to_string(&p) {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&txt) {
                    if let (Some(repo), Some(cs)) =
                        (v["repo"].as_str(), v.get("receipt").and_then(|r| r["court_status"].as_str()))
                    {
                        m.insert(repo.to_string(), cs.to_string());
                    }
                }
            }
        }
        m
    };
    let base = load(&a_dir);
    let exp = load(&b_dir);
    let is_clear = |c: &str| c == "sealed" || c == "quirk_dependent" || c == "partial";

    let mut flips: Vec<(String, String, String)> = Vec::new();
    let mut regress: Vec<(String, String, String)> = Vec::new();
    let mut compared = 0usize;
    for (repo, nc) in &exp {
        if let Some(bc) = base.get(repo) {
            compared += 1;
            match (is_clear(bc), is_clear(nc)) {
                (false, true) => flips.push((repo.clone(), bc.clone(), nc.clone())),
                (true, false) => regress.push((repo.clone(), bc.clone(), nc.clone())),
                _ => {}
            }
        }
    }
    println!("atlas-diff: {} -> {}", a_dir.display(), b_dir.display());
    println!("compared {} repos present in both", compared);
    println!("  FLIPS  (failed -> clear):     {}", flips.len());
    println!("  REGRESS (clear -> failed):    {}", regress.len());
    println!("  NET: {:+}", flips.len() as i64 - regress.len() as i64);
    if !flips.is_empty() {
        println!("\nflips:");
        for (r, b, n) in flips.iter().take(40) {
            println!("  + {}: {} -> {}", r, b, n);
        }
    }
    if !regress.is_empty() {
        println!("\nREGRESSIONS:");
        for (r, b, n) in &regress {
            println!("  - {}: {} -> {}", r, b, n);
        }
    }
    ExitCode::SUCCESS
}

fn write_recipe(out_dir: &Path, slug: &str, rec: Recipe) {
    let path = out_dir.join(format!("{}.json", slug));
    if let Ok(s) = serde_json::to_string_pretty(&rec) {
        let _ = std::fs::write(path, s + "\n");
    }
}

/// Classify a `divergence.ours_error` into an ACTIONABLE root. The shell error format is
/// `line N: <offending> (near `<token>`)` — that backtick is the SHELL's own quoting, not a backtick in
/// the source (the old "backtick-in-source" bucket was an artifact of matching it). So: if <offending>
/// is a macro call (UPPER/_ identifier immediately followed by `(`), the root is that leaked macro
/// (`macro:NAME`, aggregating with macros_ours_left_undefined); otherwise bucket by the near-token
/// (fi/else -> unbalanced conditional; a punctuation token -> that syntax token).
/// Classify a `make` failure diagnostic into an actionable error class — the make-layer "next front"
/// after configure-clear. Groups the diverse per-repo messages into the handful of underlying causes.
fn classify_make_error(d: &str) -> String {
    let dl = d.to_lowercase();
    if d.is_empty() {
        "(no diagnostic captured)".to_string()
    } else if dl.contains("no rule to make target") {
        "no-rule-to-make-target".to_string()
    } else if dl.contains("undefined reference") {
        "undefined-reference (link)".to_string()
    } else if dl.contains("missing separator") {
        "makefile-missing-separator".to_string()
    } else if (dl.contains("no such file") || dl.contains("not found")) && dl.contains(".h") {
        "missing-header-at-compile".to_string()
    } else if dl.contains("command not found") {
        "command-not-found".to_string()
    } else if dl.contains("syntax error") {
        "makefile/shell-syntax-error".to_string()
    } else if dl.contains(": error:") || dl.contains("error:") {
        "compiler-error".to_string()
    } else if dl.contains("permission denied") {
        "permission-denied".to_string()
    } else {
        "other".to_string()
    }
}

fn bucket_error(e: &str) -> String {
    let txt = e.find(": ").map(|p| &e[p + 2..]).unwrap_or(e);
    let offending = txt.split(" (near").next().unwrap_or(txt).trim();
    let mac: String = offending.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
    // A leaked macro appears as `IDENT(args` (no space before the paren — shell functions are called
    // without parens, so `ident(` at an error site is an unexpanded m4 macro). Accept any identifier,
    // not just UPPER-case ones, so m4_require/_LT_*/ac_* internals are classified instead of dumped into
    // syntax:other. Require a macro-ish shape (has an underscore or an uppercase letter) to avoid
    // catching a stray C token.
    if !mac.is_empty()
        && mac.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
        && (mac.contains('_') || mac.chars().any(|c| c.is_ascii_uppercase()))
        && offending[mac.len()..].starts_with('(')
    {
        return format!("macro:{}", mac);
    }
    // A conditional keyword followed by prose (`fi because it's not...`) is leaked AC_MSG/comment text
    // that escaped a macro body — distinct from a truly unbalanced `fi` standing alone.
    let after_kw = |kw: &str| offending.starts_with(kw)
        && offending[kw.len()..].starts_with(|c: char| c == ' ' || c == '\t')
        && offending[kw.len()..].trim().chars().next().map(|c| c.is_ascii_alphabetic()).unwrap_or(false);
    if after_kw("fi") || after_kw("else") || after_kw("then") {
        return "syntax:leaked-text-after-conditional".to_string();
    }
    let near = txt.find("near `").map(|i| &txt[i + 6..]).and_then(|s| s.split('`').next()).unwrap_or("").trim();
    match near {
        "fi" | "else" | "then" | "elif" => "syntax:unbalanced-conditional".to_string(),
        "done" | "do" => "syntax:unbalanced-loop".to_string(),
        "esac" | "in" => "syntax:unbalanced-case".to_string(),
        "" => "syntax:other".to_string(),
        t if t.chars().all(|c| !c.is_ascii_alphanumeric()) => format!("syntax:token:{}", t),
        _ => "syntax:other".to_string(),
    }
}

fn write_index(out_dir: &Path) {
    let mut by_status: BTreeMap<String, usize> = BTreeMap::new();
    let mut total = 0;
    let mut lines = Vec::new();
    // Corpus-wide expansion-bug tallies — rank the fixes by how many repos each unblocks.
    let mut leaked: BTreeMap<String, usize> = BTreeMap::new();
    let mut syntax_tokens: BTreeMap<String, usize> = BTreeMap::new();
    let mut residual: BTreeMap<String, usize> = BTreeMap::new();
    let mut heredoc_broken = 0usize;
    let mut with_leaks = 0usize;
    let mut conftest_corrupt = 0usize;
    // Oracle compass: classification counts + the fixable backlog (roots of OURS_BUG repos, ranked).
    let mut classif: BTreeMap<String, usize> = BTreeMap::new();
    let mut fixable_roots: BTreeMap<String, usize> = BTreeMap::new();
    let mut failed_checks: BTreeMap<String, usize> = BTreeMap::new();
    let mut courts: BTreeMap<String, usize> = BTreeMap::new();
    let mut sugg_pkgs: BTreeMap<String, usize> = BTreeMap::new();
    let mut ours_clear = 0usize;
    let mut real_clear = 0usize;
    // Analytics: quirk hotspots (automation candidates), dependency patterns, heavy hitters,
    // partial->full candidates (configure cleared but make fails where GNU makes — the closest wins).
    let mut quirk_hot: BTreeMap<String, usize> = BTreeMap::new();
    let mut hdr_needed: BTreeMap<String, usize> = BTreeMap::new();
    let mut dep_missing: BTreeMap<String, usize> = BTreeMap::new();
    let mut heavy: Vec<(usize, String, String)> = Vec::new();
    let mut p2f_diag: BTreeMap<String, usize> = BTreeMap::new();
    let mut make_fail_class: BTreeMap<String, usize> = BTreeMap::new();
    let mut partial_total = 0usize;
    let mut p2f_make = 0usize;
    // working/non-working roster for RECIPES.md: (court_status, status, repo, diagnostic)
    let mut roster: Vec<(String, String, String, String)> = Vec::new();
    let mut entries: Vec<_> = std::fs::read_dir(out_dir).into_iter().flatten().flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for e in entries {
        let p = e.path();
        if p.extension().and_then(|s| s.to_str()) != Some("json") || p.file_name().and_then(|s| s.to_str()) == Some("INDEX.json") {
            continue;
        }
        if let Ok(txt) = std::fs::read_to_string(&p) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&txt) {
                let st = v["status"].as_str().unwrap_or("?").to_string();
                let repo = v["repo"].as_str().unwrap_or("?").to_string();
                *by_status.entry(st.clone()).or_default() += 1;
                total += 1;
                lines.push(serde_json::json!({"repo": repo, "status": st, "verified": v["verified"]}));
                if let Some(de) = v.get("deep_expansion") {
                    if let Some(arr) = de["leaked_macros"].as_array() {
                        if !arr.is_empty() { with_leaks += 1; }
                        // count each macro NAME once per repo (repos-unblocked, not raw occurrences)
                        let mut seen = std::collections::BTreeSet::new();
                        for m in arr {
                            if let Some(name) = m["name"].as_str() { seen.insert(name.to_string()); }
                        }
                        for name in seen { *leaked.entry(name).or_default() += 1; }
                    }
                    if de["heredoc_imbalance"].as_i64().unwrap_or(0) != 0 { heredoc_broken += 1; }
                    if de["conftest_directives_mangled"].as_u64().unwrap_or(0) > 0 { conftest_corrupt += 1; }
                    if let Some(arr) = de["syntax_errors"].as_array() {
                        for s in arr {
                            if let Some(tok) = s["token"].as_str() { if !tok.is_empty() { *syntax_tokens.entry(tok.to_string()).or_default() += 1; } }
                        }
                    }
                    if let Some(arr) = de["residual_placeholders"].as_array() {
                        let mut seen = std::collections::BTreeSet::new();
                        for r in arr { if let Some(s) = r.as_str() { seen.insert(s.to_string()); } }
                        for s in seen { *residual.entry(s).or_default() += 1; }
                    }
                    // the actual probe ours died on (the divergence root, not the cascade line)
                    if st != "FUNC_OK" {
                        if let Some(c) = de["failed_during_check"].as_str() {
                            if !c.is_empty() {
                                // normalize: drop trailing "... <value>" so similar checks bucket together
                                let norm = c.split("...").next().unwrap_or(c).trim().to_string();
                                *failed_checks.entry(norm).or_default() += 1;
                            }
                        }
                    }
                }
                if st == "MAKE_FAIL" || st == "FUNC_OK" { ours_clear += 1; }
                if let Some(c) = v.get("receipt").and_then(|r| r["court_status"].as_str()) {
                    *courts.entry(c.to_string()).or_default() += 1;
                }
                roster.push((
                    v.get("receipt").and_then(|r| r["court_status"].as_str()).unwrap_or("?").to_string(),
                    st.clone(),
                    repo.clone(),
                    v["diagnostic"].as_str().unwrap_or("").chars().take(90).collect::<String>(),
                ));
                if let Some(arr) = v.get("suggested_deps").and_then(|s| s.as_array()) {
                    let mut seen = std::collections::BTreeSet::new();
                    for s in arr { if let Some(p) = s["package"].as_str() { seen.insert(p.to_string()); } }
                    for p in seen { *sugg_pkgs.entry(p).or_default() += 1; }
                }
                // quirk hotspots — the automation candidates (count each quirk once per repo)
                if let Some(arr) = v.get("receipt").and_then(|r| r["quirks_matched"].as_array()) {
                    for q in arr { if let Some(s) = q.as_str() { *quirk_hot.entry(s.to_string()).or_default() += 1; } }
                }
                // dependency patterns — headers needed / deps missing
                if let Some(dep) = v.get("dependencies") {
                    if let Some(arr) = dep["headers_needed"].as_array() {
                        for h in arr { if let Some(s) = h.as_str() { *hdr_needed.entry(s.to_string()).or_default() += 1; } }
                    }
                    if let Some(arr) = dep["missing"].as_array() {
                        for m in arr { if let Some(s) = m.as_str() { *dep_missing.entry(s.to_string()).or_default() += 1; } }
                    }
                }
                // heavy hitters — configure size as a complexity proxy
                if let Some(de) = v.get("deep_expansion") {
                    let cl = de["configure_lines"].as_u64().unwrap_or(0) as usize;
                    if cl > 0 {
                        let court = v.get("receipt").and_then(|r| r["court_status"].as_str()).unwrap_or("?").to_string();
                        heavy.push((cl, repo.clone(), court));
                    }
                }
                // partial -> full candidates: configure cleared, make failed, and GNU makes it (OURS_BUG_MAKE)
                let court = v.get("receipt").and_then(|r| r["court_status"].as_str()).unwrap_or("");
                if court == "partial" {
                    partial_total += 1;
                    // classify the make failure (the next-front root): all partial repos, by error class.
                    *make_fail_class.entry(classify_make_error(v["diagnostic"].as_str().unwrap_or(""))).or_default() += 1;
                    if v.get("oracle").and_then(|o| o["classification"].as_str()) == Some("OURS_BUG_MAKE") {
                        p2f_make += 1;
                        if let Some(d) = v["diagnostic"].as_str() { if !d.is_empty() {
                            // bucket leaked-macro / undefined-macro diagnostics together by the macro name
                            let bucket = if let Some(pos) = d.find(": command not found") {
                                let pre = &d[..pos];
                                format!("leaked-macro:{}", pre.rsplit([' ', ':']).next().unwrap_or(pre))
                            } else if d.contains("undefined macro") {
                                "undefined-macro".to_string()
                            } else { d.chars().take(40).collect::<String>() };
                            *p2f_diag.entry(bucket).or_default() += 1;
                        }}
                    }
                }
                if let Some(orc) = v.get("oracle") {
                    if let Some(c) = orc["classification"].as_str() { *classif.entry(c.to_string()).or_default() += 1; }
                    if orc["real_configure"].as_str() == Some("ok") { real_clear += 1; }
                }
                // fixable backlog: roots of repos where real got further than ours
                if let Some(dv) = v.get("divergence") {
                    let mut seen = std::collections::BTreeSet::new();
                    if let Some(arr) = dv["macros_ours_left_undefined"].as_array() {
                        for m in arr { if let Some(s) = m.as_str() { seen.insert(format!("macro:{}", s)); } }
                    }
                    if seen.is_empty() {
                        if let Some(e) = dv["ours_error"].as_str() { if !e.is_empty() {
                            seen.insert(bucket_error(e));
                        }}
                    }
                    for s in seen { *fixable_roots.entry(s).or_default() += 1; }
                }
            }
        }
    }
    // rank helper: map -> Vec of {name,repos} sorted desc, top 30
    let rank = |m: &BTreeMap<String, usize>| -> Vec<serde_json::Value> {
        let mut v: Vec<_> = m.iter().map(|(k, c)| (k.clone(), *c)).collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        v.into_iter().take(30).map(|(k, c)| serde_json::json!({"name": k, "repos": c})).collect()
    };
    let heavy_hitters_json: Vec<serde_json::Value> = {
        let mut h = heavy.clone();
        h.sort_by(|a, b| b.0.cmp(&a.0));
        h.into_iter().take(15).map(|(l, r, c)| serde_json::json!({"repo": r, "configure_lines": l, "court": c})).collect()
    };
    let index = serde_json::json!({
        "schema": "automake-rs.build-atlas/index/v2",
        "total": total,
        "by_status": by_status,
        "expansion_bugs": {
            "repos_with_leaked_macros": with_leaks,
            "repos_with_heredoc_imbalance": heredoc_broken,
            "repos_with_conftest_corruption": conftest_corrupt,
            "top_leaked_macros": rank(&leaked),
            "top_syntax_tokens": rank(&syntax_tokens),
            "top_residual_placeholders": rank(&residual),
        },
        "oracle_compass": {
            "ours_configure_clear": ours_clear,
            "real_configure_clear": real_clear,
            "headroom_our_bugs": real_clear.saturating_sub(ours_clear),
            "classification": classif,
            "fixable_backlog_roots": rank(&fixable_roots),
            "died_during_check": rank(&failed_checks),
        },
        "courts": courts.clone(),
        "suggested_packages": rank(&sugg_pkgs),
        "analytics": {
            "quirk_hotspots": rank(&quirk_hot),
            "most_needed_headers": rank(&hdr_needed),
            "most_missing_deps": rank(&dep_missing),
            "heavy_hitters": heavy_hitters_json,
            "partial_to_full": {
                "partial_total": partial_total,
                "ours_bug_make": p2f_make,
                "top_blockers": rank(&p2f_diag),
            },
            "make_failure_roots": rank(&make_fail_class),
        },
        "recipes": lines,
    });
    let _ = std::fs::write(out_dir.join("INDEX.json"), serde_json::to_string_pretty(&index).unwrap_or_default() + "\n");

    // Human-readable docs live at the atlas DOC dir (alongside README.md/SCHEMA.md), not inside the
    // recipe-data dir — so when out_dir is `<atlas>/recipes`, write the .md to `<atlas>/`. (Avoids the
    // stale-duplicate trap where atlas/COURTS.md and atlas/recipes/COURTS.md drift apart.)
    let doc_dir = if out_dir.file_name().and_then(|s| s.to_str()) == Some("recipes") {
        out_dir.parent().unwrap_or(out_dir)
    } else {
        out_dir
    };

    // COURTS.md — human-readable gap-analysis summary of the build-court verdicts.
    let mut md = String::new();
    md.push_str("# Build Courts — automake-rs Atlas gap analysis\n\n");
    md.push_str(&format!("Total recipes: **{}**\n\n## Court status\n\n", total));
    md.push_str("| status | count | meaning |\n|---|---|---|\n");
    let meaning = |s: &str| match s {
        "sealed" => "FUNC_OK, no quirks — fully reproduced",
        "quirk_dependent" => "FUNC_OK but needed a quirk rule",
        "partial" => "configure cleared, make failed",
        "not_standalone" => "oracle (GNU) also fails — not our bug",
        "failed" => "ours fails before make",
        _ => "",
    };
    for (k, c) in courts.iter() {
        md.push_str(&format!("| {} | {} | {} |\n", k, c, meaning(k)));
    }
    md.push_str(&format!("\n## Oracle headroom\n\nours configure-clear: **{}** · GNU configure-clear: **{}** · fixable our-bug headroom: **{}**\n\n",
        ours_clear, real_clear, real_clear.saturating_sub(ours_clear)));
    md.push_str("## Top fixable roots (real succeeds, ours fails)\n\n");
    for r in rank(&fixable_roots).iter().take(15) {
        md.push_str(&format!("- {} — {} repos\n", r["name"].as_str().unwrap_or(""), r["repos"]));
    }
    md.push_str("\n## Most-needed packages (missing-dep inference)\n\n");
    for r in rank(&sugg_pkgs).iter().take(15) {
        md.push_str(&format!("- {} — {} repos\n", r["name"].as_str().unwrap_or(""), r["repos"]));
    }
    md.push_str(&format!("\n## Make-layer roots (the next front: {} partial repos clear configure but fail make)\n\n", partial_total));
    for r in rank(&make_fail_class).iter().take(12) {
        md.push_str(&format!("- {} — {} repos\n", r["name"].as_str().unwrap_or(""), r["repos"]));
    }
    let _ = std::fs::write(doc_dir.join("COURTS.md"), md);

    // ANALYTICS.md — self-documenting corpus intelligence: quirk hotspots (automation candidates),
    // failure modes, dependency patterns, heavy hitters, and the partial->full shortlist.
    let mut a = String::new();
    a.push_str("# Atlas Analytics — corpus intelligence\n\n");
    a.push_str(&format!("Total recipes: **{}** · court mix: {}\n\n",
        total, courts.iter().map(|(k, c)| format!("{} {}", c, k)).collect::<Vec<_>>().join(", ")));

    a.push_str("## Quirk hotspots (automation candidates)\n\nQuirks matched across recipes — the most frequent are the highest-leverage to auto-apply.\n\n| quirk | repos |\n| --- | --- |\n");
    for r in rank(&quirk_hot).iter().take(15) {
        a.push_str(&format!("| {} | {} |\n", r["name"].as_str().unwrap_or(""), r["repos"]));
    }

    a.push_str("\n## Top failure roots (the check ours died on)\n\n| check | repos |\n| --- | --- |\n");
    for r in rank(&failed_checks).iter().take(10) {
        a.push_str(&format!("| {} | {} |\n", r["name"].as_str().unwrap_or(""), r["repos"]));
    }

    a.push_str("\n## Dependency patterns\n\n**Most-needed headers**\n\n| header | repos |\n| --- | --- |\n");
    for r in rank(&hdr_needed).iter().take(12) {
        a.push_str(&format!("| {} | {} |\n", r["name"].as_str().unwrap_or(""), r["repos"]));
    }
    if !dep_missing.is_empty() {
        a.push_str("\n**Most-missing deps**\n\n| dep | repos |\n| --- | --- |\n");
        for r in rank(&dep_missing).iter().take(12) {
            a.push_str(&format!("| {} | {} |\n", r["name"].as_str().unwrap_or(""), r["repos"]));
        }
    }

    a.push_str("\n## Heavy hitters (configure size = complexity proxy)\n\n| configure lines | repo | court |\n| --- | --- | --- |\n");
    let mut hh = heavy.clone();
    hh.sort_by(|x, y| y.0.cmp(&x.0));
    for (l, r, c) in hh.into_iter().take(12) {
        a.push_str(&format!("| {} | {} | {} |\n", l, r, c));
    }

    a.push_str(&format!("\n## Partial -> full shortlist\n\n**{}** recipes cleared configure but failed make; **{}** of those are `OURS_BUG_MAKE` (GNU makes it, we don't) — the closest wins. Top blockers:\n\n| blocker | repos |\n| --- | --- |\n", partial_total, p2f_make));
    for r in rank(&p2f_diag).iter().take(12) {
        a.push_str(&format!("| {} | {} |\n", r["name"].as_str().unwrap_or(""), r["repos"]));
    }
    let _ = std::fs::write(doc_dir.join("ANALYTICS.md"), a);

    // RECIPES.md — the working / non-working roster. "Working" = built end-to-end (FUNC_OK, i.e. court
    // sealed or quirk_dependent). Non-working is split: partial (configure cleared, make failed),
    // not_standalone (GNU also fails — not our bug), failed (ours fails before make).
    roster.sort_by(|x, y| x.2.to_lowercase().cmp(&y.2.to_lowercase()));
    let count = |court: &str| roster.iter().filter(|(c, _, _, _)| c == court).count();
    let working = count("sealed") + count("quirk_dependent");
    let mut r = String::new();
    r.push_str("# Atlas Recipes — working / non-working roster\n\n");
    r.push_str(&format!("Total **{}** recipes. **Working (built end-to-end): {}** · non-working: {} partial · {} not-standalone · {} failed.\n\n",
        total, working, count("partial"), count("not_standalone"), count("failed")));
    r.push_str("\"Working\" means the full pipeline (autoreconf → configure → make) succeeded under the GNU-free toolchain. `quirk_dependent` needed an auto-applied quirk; `sealed` needed none.\n\n");

    r.push_str(&format!("## ✅ Working ({})\n\n", working));
    let work_rows: Vec<_> = roster.iter().filter(|(c, _, _, _)| c == "sealed" || c == "quirk_dependent").collect();
    if work_rows.is_empty() {
        r.push_str("_None in this scan slice._\n\n");
    } else {
        r.push_str("| repo | court |\n| --- | --- |\n");
        for (c, _s, repo, _d) in &work_rows { r.push_str(&format!("| {} | {} |\n", repo, c)); }
        r.push('\n');
    }

    for (title, court, with_diag) in [
        ("🟡 Non-working — partial (configure cleared, make failed)", "partial", true),
        ("❌ Non-working — failed (ours fails before make)", "failed", true),
        ("⚪ Non-working — not standalone (GNU autotools also fails; not our bug)", "not_standalone", false),
    ] {
        let rows: Vec<_> = roster.iter().filter(|(c, _, _, _)| c == court).collect();
        r.push_str(&format!("## {} ({})\n\n", title, rows.len()));
        if rows.is_empty() { r.push_str("_None._\n\n"); continue; }
        if with_diag {
            r.push_str("| repo | stage | first error |\n| --- | --- | --- |\n");
            for (_c, s, repo, d) in rows { r.push_str(&format!("| {} | {} | {} |\n", repo, s, d.replace('|', "\\|"))); }
        } else {
            r.push_str("| repo | stage |\n| --- | --- |\n");
            for (_c, s, repo, _d) in rows { r.push_str(&format!("| {} | {} |\n", repo, s)); }
        }
        r.push('\n');
    }
    let _ = std::fs::write(doc_dir.join("RECIPES.md"), r);
}

/// `xtask atlas-index <out-dir>` — rebuild INDEX.json from existing recipes (no builds). Used after
/// parallel atlas workers populate the recipe dir.
pub fn index_only() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let out_dir = PathBuf::from(args.get(2).cloned().unwrap_or_else(|| "atlas/recipes".into()));
    write_index(&out_dir);
    println!("atlas-index: rebuilt INDEX.json in {}", out_dir.display());
    ExitCode::SUCCESS
}

/// `xtask atlas-query <term> [out-dir]` — find every recipe that touches a dependency, header,
/// function, macro, package, or quirk. The ecosystem search interface over the corpus.
pub fn query() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let term = match args.get(2) {
        Some(t) => t.to_lowercase(),
        None => {
            eprintln!("usage: xtask atlas-query <term> [out-dir]");
            return ExitCode::FAILURE;
        }
    };
    let out_dir = PathBuf::from(args.get(3).cloned().unwrap_or_else(|| "atlas/recipes".into()));
    let mut hits: Vec<(String, String, String)> = Vec::new(); // repo, where, detail
    for e in std::fs::read_dir(&out_dir).into_iter().flatten().flatten() {
        let p = e.path();
        if p.extension().and_then(|s| s.to_str()) != Some("json")
            || p.file_name().and_then(|s| s.to_str()) == Some("INDEX.json") { continue; }
        let txt = match std::fs::read_to_string(&p) { Ok(t) => t, Err(_) => continue };
        let v: serde_json::Value = match serde_json::from_str(&txt) { Ok(v) => v, Err(_) => continue };
        let repo = v["repo"].as_str().unwrap_or("?").to_string();
        let st = v["status"].as_str().unwrap_or("?").to_string();
        let mut found = |where_: &str, detail: String| hits.push((repo.clone(), where_.to_string(), detail));
        let dep = &v["dependencies"];
        for (key, label) in [("pkg_config","pkg-config"),("system_libs","lib"),("headers_needed","header")] {
            if let Some(arr) = dep[key].as_array() {
                for x in arr { if let Some(s)=x.as_str() { if s.to_lowercase().contains(&term) { found(label, format!("{} ({})", s, st)); } } }
            }
        }
        if let Some(obj) = v["probe_results"].as_object() {
            for k in obj.keys() { if k.to_lowercase().contains(&term) { found("probe", format!("{} ({})", k, st)); break; } }
        }
        if let Some(arr) = v["suggested_deps"].as_array() {
            for x in arr { if let Some(s)=x["package"].as_str() { if s.to_lowercase().contains(&term) { found("suggested-pkg", format!("{} ({})", s, st)); } } }
        }
        if let Some(arr) = v["receipt"]["quirks_matched"].as_array() {
            for x in arr { if let Some(s)=x.as_str() { if s.to_lowercase().contains(&term) { found("quirk", format!("{} ({})", s, st)); } } }
        }
        if let Some(de) = v.get("deep_expansion") {
            if let Some(arr) = de["leaked_macros"].as_array() {
                for m in arr { if let Some(s)=m["name"].as_str() { if s.to_lowercase().contains(&term) { found("leaked-macro", format!("{} ({})", s, st)); break; } } }
            }
        }
    }
    println!("atlas-query \"{}\": {} hit(s)", term, hits.len());
    for (repo, where_, detail) in hits.iter().take(100) {
        println!("  {:<40} [{}] {}", repo, where_, detail);
    }
    ExitCode::SUCCESS
}
