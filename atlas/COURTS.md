# Build Courts — automake-rs Atlas gap analysis

Total recipes: **986**

## Court status

| status | count | meaning |
|---|---|---|
| failed | 535 | ours fails before make |
| not_standalone | 10 | oracle (GNU) also fails — not our bug |
| partial | 380 | configure cleared, make failed |
| quirk_dependent | 61 | FUNC_OK but needed a quirk rule |

## Oracle headroom

ours configure-clear: **441** · GNU configure-clear: **21** · fixable our-bug headroom: **0**

## Top fixable roots (real succeeds, ours fails)

- syntax:other — 2 repos
- syntax:token:( — 2 repos
- syntax:unbalanced-conditional — 2 repos
- macro:AM_CHECK_PYTHON_HEADERS — 1 repos
- macro:AM_PYTHON_CHECK_VERSION — 1 repos
- syntax:leaked-text-after-conditional — 1 repos
- syntax:token:) — 1 repos
- syntax:token:} — 1 repos
- syntax:unbalanced-loop — 1 repos

## Most-needed packages (missing-dep inference)


## Make-layer roots (the next front: 380 partial repos clear configure but fail make)

- compiler-error — 146 repos
- missing-header-at-compile — 85 repos
- other — 56 repos
- no-rule-to-make-target — 37 repos
- undefined-reference (link) — 30 repos
- makefile-missing-separator — 13 repos
- (no diagnostic captured) — 5 repos
- permission-denied — 4 repos
- command-not-found — 3 repos
- makefile/shell-syntax-error — 1 repos
