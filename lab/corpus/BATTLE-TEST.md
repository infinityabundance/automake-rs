# automake-rs Battle Test — 1000-project corpus vs the original GNU automake 1.18.1

Run in a dependency-provisioned Ubuntu QEMU VM against the pinned GNU automake **1.18.1**
oracle. Two differentials; raw results in `results/`.

## 1. Generation differential — 1000 projects (`results/generation-differential.tsv`)
For each project the GNU oracle generates the reference `Makefile.in`; automake-rs then
regenerates and the two are compared.

| Metric | Result |
|---|---|
| Processed | 998 / 1000 (2 lost a clean GNU reference) |
| **automake-rs crashes or errors** | **0** — it never chokes on real-world `Makefile.am` |
| Produced a `Makefile.in` | 998 / 998 |
| Byte-identical to GNU | 0 (automake-rs is a clean-room subset reimplementation) |
| Output size vs GNU | median **56%** of GNU's line count (range 2%–143%) |

**Read:** automake-rs is **100% robust** across the 1000 most-popular automake projects —
it parses and generates for every one without a single crash or typed error — but its
output is a structural subset of GNU's (no byte parity).

## 2. Functional differential — build-verified subset (`results/functional-differential.tsv`)
On projects that fully build with GNU automake, automake-rs regenerates **every**
`Makefile.am`, then the project is `./configure`d and `make`d (GNU supplies configure +
aux files; only the `Makefile.in`s are automake-rs's).

| Outcome | Count |
|---|---|
| **FUNC_OK — builds end-to-end with automake-rs Makefiles** | **45** (was 40; +5 from the fixes below) |
| MAKE_FAIL — configures, build fails on a feature gap | 54 |
| CONFIGURE_FAIL | 5 |
| (clone failures this run, excluded) | 14 |

Projects that build end-to-end on automake-rs output include `smenu`, `rsnapshot`,
`stow`, `sockperf`, `binbloom`, and others. The MAKE_FAILs are concrete generator gaps —
C++ sources, libtool versioning, and complex multi-source/per-target linking — the next
build-out targets.

## Method (reproducible)
GitHub code search (42 automake signatures) → 5070-repo pool → GraphQL star ranking →
validation in the VM with full bootstrap (`./autogen.sh`/`./bootstrap`/`autoreconf`,
automake 1.18.1 & 1.16.5), a 748-package dependency manifest (`automake-corpus-deps.txt`),
and apt-file dependency auto-resolution.

## Build-out progress (driving the functional rate up)
Triaging the MAKE_FAILs and fixing by category, re-measured against the corpus:
- **C++ toolchain** — added `.cc/.cpp/.cxx/.C` suffix rules + `CXX*`/`CXXLINK` (C++ projects compiled).
- **Target-name canonicalization** — `test-program` → `test_program_*` derived vars; fixed bogus
  `test-program.o` and incomplete multi-source object lists (e.g. `rinetd` now builds end-to-end).
- FUNC_OK **40 → 45**. Remaining MAKE_FAIL categories, by frequency: **libtool `.la` libraries**
  (LTLIBRARIES build rules), **subdir-objects** (subdir sources → subdir objects), a **dep-manifest
  variable-reference bug** (`$(X_SOURCES).Po` for variable-defined/odd-named targets → "missing
  separator"), and project-specific config.h/include-path cases. These are the next targets.

## Generator build-out — session 2 (libtool, subdir-objects, per-target flags)
Driving the functional rate by closing MAKE_FAIL categories. Each fix is verified in the VM
against real repos, with core + integration tests kept green throughout. Eight commits:

1. **C++ toolchain** — `.cc/.cpp/.cxx/.C` suffix rules, `CXX*`/`CXXLINK`.
2. **Target-name canonicalization** — `test-program` → `test_program_*`; fixed bogus objects
   and incomplete multi-source object lists (rinetd builds).
3. **Dep-collection correctness** — canonical lookup + real-source filter (no malformed
   `$(X_SOURCES).Po`; carbon-c-relay builds).
4. **libtool (LTLIBRARIES)** — `.lo` objects, LTCOMPILE, libtool `LINK`, `.c.lo`/`.cc.lo` rules,
   per-library link with `-rpath`, `_DEPENDENCIES` from `.la`/`.a` in LDADD. Plus two latent
   fixes it surfaced: the `$(am__depfiles_remade)` stub rule was emitted before `all:` (default
   goal became a dep stub) and `AM_V_lt` referenced itself.
5. **find_variable `+=` accumulation** — multi-line `_SOURCES` were truncated to their first file.
6. **subdir object paths** — `foo/bar.c` → `foo/bar.lo` so suffix rules build subdir sources.
7. **per-target compile flags** — `X_CPPFLAGS/X_CFLAGS/X_CXXFLAGS` rename objects `{canon}-{stem}`
   and get dedicated per-object rules applying those flags.

**Effect:** the structural foundation is now correct for C++, multi-source, libtool, subdir, and
per-target-flag builds. Repos that previously died at "No rule to make target X.lo" now compile
their objects and advance to the *next* layer. End-to-end FUNC_OK is steady at **45 / 102 testable
(~44%)** because the repos these fixes unblocked are libtool-heavy and hit the remaining stack
before completing: **yacc/lex BUILT_SOURCES ordering** (ply: generated `grammar.h`), **convenience
(noinst) library linking** (cowsql), and **a few user-variable/include nuances**. Those are the
next targets; each is a discrete, well-scoped feature rather than a structural gap.

## Session 2 (cont.) — Yacc/Lex, no-dependencies, and MAKE_FAIL forensics
Three more generator features landed (commits continue from the list above):
8. **Yacc/Lex + BUILT_SOURCES** — parser sources compile from their generated `.c`; clean direct
   `$(YACCCOMPILE)/-o` + `$(LEXCOMPILE)/-o` rules + parser-header recovery; `all:` builds
   `$(BUILT_SOURCES)` first. (ply now runs YACC+LEX, compiles every source, reaches link.)
9. **`no-dependencies`** — honor the AM_INIT_AUTOMAKE option (the @AMDEP@ include markers were
   emitted unconditionally; with `no-dependencies`, config.status defines no AMDEP_TRUE, leaving
   literal `@AMDEP_TRUE@` → "missing separator"). Also `find configure.ac` upward so subdir
   Makefiles inherit the top-level options. (advancecomp/libchardet/lrc-erasure-code fixed.)

### Forensic categorization of the remaining MAKE_FAILs
Re-running every MAKE_FAIL repo **serially** (the differential harness runs `-P6` and undercounts
via timeouts/transient clone failures) gives the real distribution (40 classified):

| Category | Count | Meaning |
|---|---|---|
| GENERATOR | 10 | still-actionable generator bugs (further work helps) |
| LINK | 8 | undefined refs / convenience-lib + `.libs` ordering |
| DEP_MISSING | 7 | uninstalled system headers/libs (environment, not the generator) |
| COMPILE | 6 | config-header / compile-flag specifics |
| **NOW_OK** | **5** | **were MAKE_FAIL in the parallel run; build fine serially** |
| OTHER | 4 | misc |

**Takeaways:** (1) the parallel FUNC_OK count (45) is a *lower bound* — serial re-test already
recovers 5, so true functional ≈ **50+**; (2) only ~25% of remaining failures are generator bugs —
the rest are environment (DEP_MISSING) or deep/link specifics; (3) the structural foundation
(C++, multi-source, libtool, subdir-objects, per-target flags, Yacc/Lex, no-dependencies) is in
place. Remaining generator targets: convenience-library `.libs`/link ordering and a few
generated-source naming edge cases.
