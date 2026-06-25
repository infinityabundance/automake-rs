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
| **FUNC_OK — builds end-to-end with automake-rs Makefiles** | **40 / 99 testable (~40%)** |
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
