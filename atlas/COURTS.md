# Build Courts — automake-rs Atlas gap analysis

Total recipes: **970**

## Court status

| status | count | meaning |
|---|---|---|
| failed | 406 | ours fails before make |
| not_standalone | 250 | oracle (GNU) also fails — not our bug |
| partial | 312 | configure cleared, make failed |
| quirk_dependent | 2 | FUNC_OK but needed a quirk rule |

## Oracle headroom

ours configure-clear: **314** · GNU configure-clear: **493** · fixable our-bug headroom: **179**

## Top fixable roots (real succeeds, ours fails)

- syntax:unbalanced-conditional — 33 repos
- syntax:token:( — 23 repos
- syntax:other — 18 repos
- syntax:unbalanced-loop — 17 repos
- syntax:leaked-text-after-conditional — 16 repos
- syntax:token:) — 7 repos
- syntax:token:; — 5 repos
- macro:AM_CONDITIONAL — 3 repos
- macro:AM_INIT_AUTOMAKE — 3 repos
- macro:AN_MAKEVAR — 3 repos
- macro:DX_HTML_FEATURE — 3 repos
- macro:AC_HAVE_LIBRARY — 2 repos
- macro:AM_MISSING_PROG — 2 repos
- macro:AM_SILENT_RULES — 2 repos
- macro:GUILE_PROGS — 2 repos

## Most-needed packages (missing-dep inference)


## Make-layer roots (the next front: 312 partial repos clear configure but fail make)

- makefile-missing-separator — 221 repos
- (no diagnostic captured) — 36 repos
- other — 33 repos
- command-not-found — 21 repos
- compiler-error — 1 repos
