# Build Courts — automake-rs Atlas gap analysis

Total recipes: **1000**

## Court status

| status | count | meaning |
|---|---|---|
| failed | 555 | ours fails before make |
| partial | 383 | configure cleared, make failed |
| quirk_dependent | 62 | FUNC_OK but needed a quirk rule |

## Oracle headroom

ours configure-clear: **445** · GNU configure-clear: **0** · fixable our-bug headroom: **0**

## Top fixable roots (real succeeds, ours fails)


## Most-needed packages (missing-dep inference)


## Make-layer roots (the next front: 383 partial repos clear configure but fail make)

- compiler-error — 152 repos
- missing-header-at-compile — 88 repos
- other — 60 repos
- no-rule-to-make-target — 38 repos
- undefined-reference (link) — 30 repos
- makefile-missing-separator — 8 repos
- permission-denied — 4 repos
- command-not-found — 2 repos
- makefile/shell-syntax-error — 1 repos
