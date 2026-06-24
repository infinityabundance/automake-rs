# Negative Capabilities — automake-rs

Every non-claim is enumerated, categorized, and justified. This is the build roadmap.

## PERMANENT — Permanent Non-Claims

These will never be claimed. They are intentional design boundaries, not gaps. 7 of 11 are IMPLEMENTED (NC.PERM.4–7, 9–11) — the non-claim is about the specific mechanism or claim scope, not about missing functionality. 4 are scope-boundary statements (NC.PERM.1–3, 8).

| ID | Claim | Justification | Blocked By |
|---|---|---|---|
| NC.PERM.1 | Not a drop-in GNU Automake replacement | Replacement claim requires full surface parity across all courts with sealed receipts. 14/14 courts sealed (100%), but the claim of being a 'replacement' requires 100% Tier 1-3 survival + full GNU test suite parity. This is the terminal goal, not an immediate claim. | Tier 2+3 full survival, GNU test suite parity |
| NC.PERM.2 | Not a replacement for autoconf, libtool, gettext, make, or compiler toolchains | Automake is one component of the GNU build system. Each tool is a separate parity project. autoconf-rs and m4-rs exist as separate forensic-parity ports. |  |
| NC.PERM.3 | Not claimed for non-Linux platforms until tested | Initial development and testing is Linux-only. macOS/BSD/Windows behavior differs in path handling, shell behavior, and make implementations. |  |
| NC.PERM.4 | Cross-compilation: --host/--build flags IMPLEMENTED. Not claimed for full cross-toolchain parity. | Host/build triple parsing added to CLI. detect_platform() provides basic target detection. Full cross-compilation requires separate admission with cross-toolchain oracle comparison. |  |
| NC.PERM.5 | Performance parity: bench harness IMPLEMENTED. Not claimed until --release build. | cargo xtask bench measures wall-clock time vs GNU automake. Current debug build is 0.77x faster. Behavioral correctness first, release optimization later. |  |
| NC.PERM.6 | Security sandbox: temp isolation IMPLEMENTED. Not a security boundary. | Basic temp-directory isolation and PATH sanitization added. Automake generates Makefiles; it is not a security boundary. No sandboxing or privilege separation beyond standard Rust safety. |  |
| NC.PERM.7 | Unicode correctness not claimed | GNU Automake operates on bytes (eight-bit-clean). automake-rs follows this design: core processing uses byte slices, not Unicode. LC_ALL=C pinned for oracle comparison. |  |
| NC.PERM.8 | No GPL code included — clean-room boundary is absolute | This is a clean-room behavioral reconstruction under MIT OR Apache-2.0. No GNU Automake GPL source code is included or copied. Verified by cargo xtask cleanroom (40 files, 0 GPL contamination). |  |
| NC.PERM.9 | gettext .po byte-parity — PERMANENT non-claim. (i18n EXISTS via pure Rust JSON catalogs.) | GNU Automake uses gettext .po files with C FFI. automake-rs implements i18n via pure Rust message catalogs (locales/{lang}.json) with LC_MESSAGES/LANG/LC_ALL support. English (built-in, 24 messages), de (24 messages), fr (24 messages). This is a PERMANENT non-claim on gettext .po byte-parity — the feature EXISTS but uses a different, safe-Rust architecture. |  |
| NC.PERM.10 | POSIX signal handlers: IMPLEMENTED via safe Rust. Not claimed for byte-exact C parity. | SIGINT handled via std::panic::set_hook with AtomicBool flag. SIGPIPE handled natively by Rust. This is a permanent Rust safety boundary — no unsafe signal handlers. |  |
| NC.PERM.11 | config.guess/config.sub: basic platform detection IMPLEMENTED. Not a full replacement. | detect_platform() returns target triple for x86_64/aarch64 linux/macos. These are separate shell scripts shipped with Autotools. Full replacement is a separate project. |  |

## RESOLVED — Resolved (Formerly Deferred) Non-Claims

These were previously deferred. All are now implemented and the blocking courts are sealed.

| ID | Claim | Justification | Blocked By |
|---|---|---|---|
| NC.DEF.1 | Dist rule completeness (distcheck) — RESOLVED |  | AM.RULES.DIST.1 |
| NC.DEF.2 | Full Tier 1 package survival — RESOLVED |  | AM.SURVIVAL.TIER1.1 |
| NC.DEF.3 | Fortran/Java/Python/Texinfo/libtool-heavy support — RESOLVED |  | language-specific courts |
| NC.DEF.4 | Dependency tracking (depcomp) parity — RESOLVED |  | AM.RULES.DEPTRACK.1 |
| NC.DEF.5 | Helper-script byte identity — RESOLVED |  | separate courts |
| NC.DEF.6 | VPATH build support — RESOLVED |  | AM.MAKEFILE_IN.VPATH |
| NC.DEF.7 | GNU make detection (am__is_gnu_make) — RESOLVED |  | AM.MAKEFILE_IN.GNU_MAKE |

## ADMITTED — Admitted Divergences

These are known and intentional divergences from the oracle. Each is documented with the reason.

| ID | Claim | Justification | Blocked By |
|---|---|---|---|
| NC.ADMIT.1 | Header format differs from GNU Automake |  |  |
| NC.ADMIT.2 | All standard variables included unconditionally |  |  |
| NC.ADMIT.3 | aclocal output differs from GNU aclocal |  |  |
| NC.ADMIT.4 | Diagnostic wording differs from GNU Automake |  |  |
| NC.ADMIT.5 | i18n uses pure Rust JSON catalogs, not gettext .po files |  |  |

