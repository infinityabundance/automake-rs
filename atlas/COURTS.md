# Build Courts — automake-rs Atlas gap analysis

Total recipes: **986**

## Court status

| status | count | meaning |
|---|---|---|
| failed | 368 | ours fails before make |
| not_standalone | 250 | oracle (GNU) also fails — not our bug |
| quirk_dependent | 2 | FUNC_OK but needed a quirk rule |

## Oracle headroom

ours configure-clear: **320** · GNU configure-clear: **269** · fixable our-bug headroom: **0**

## Top fixable roots (real succeeds, ours fails)

- syntax:unbalanced-conditional — 33 repos
- syntax:token:( — 23 repos
- syntax:other — 17 repos
- syntax:unbalanced-loop — 17 repos
- syntax:leaked-text-after-conditional — 16 repos
- syntax:token:) — 7 repos
- syntax:token:; — 5 repos
- macro:AN_MAKEVAR — 3 repos
- macro:DX_HTML_FEATURE — 3 repos
- macro:GUILE_PROGS — 2 repos
- syntax:token:-- — 2 repos
- syntax:token:} — 2 repos
- macro:AC_LANG_COMPILER — 1 repos
- macro:AM_CHECK_PYTHON_HEADERS — 1 repos
- macro:AM_PYTHON_CHECK_VERSION — 1 repos

## Most-needed packages (missing-dep inference)


## Make-layer roots (the next front: 0 partial repos clear configure but fail make)

