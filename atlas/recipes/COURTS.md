# Build Courts — automake-rs Atlas gap analysis

Total recipes: **982**

## Court status

| status | count | meaning |
|---|---|---|
| failed | 364 | ours fails before make |
| not_standalone | 250 | oracle (GNU) also fails — not our bug |
| partial | 366 | configure cleared, make failed |
| quirk_dependent | 2 | FUNC_OK but needed a quirk rule |

## Oracle headroom

ours configure-clear: **368** · GNU configure-clear: **520** · fixable our-bug headroom: **152**

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
- macro:GUILE_PROGS — 2 repos
- syntax:token:-- — 2 repos
- syntax:token:} — 2 repos
- macro:AC_LANG_COMPILER — 1 repos

## Most-needed packages (missing-dep inference)

