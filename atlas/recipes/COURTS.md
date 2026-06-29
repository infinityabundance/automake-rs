# Build Courts — automake-rs Atlas gap analysis

Total recipes: **986**

## Court status

| status | count | meaning |
|---|---|---|
| failed | 409 | ours fails before make |
| not_standalone | 250 | oracle (GNU) also fails — not our bug |
| partial | 324 | configure cleared, make failed |
| quirk_dependent | 3 | FUNC_OK but needed a quirk rule |

## Oracle headroom

ours configure-clear: **327** · GNU configure-clear: **529** · fixable our-bug headroom: **202**

## Top fixable roots (real succeeds, ours fails)

- syntax:other — 29 repos
- syntax:unbalanced-conditional — 28 repos
- syntax:token:( — 15 repos
- syntax:unbalanced-loop — 15 repos
- macro:AC_SUBST — 10 repos
- macro:AC_DEFINE — 9 repos
- macro:AC_MSG_ERROR — 9 repos
- macro:AC_REQUIRE — 8 repos
- macro:AC_MSG_NOTICE — 6 repos
- macro:AC_COMPILE_IFELSE — 5 repos
- macro:AC_LANG_PUSH — 5 repos
- macro:AS_ECHO — 5 repos
- macro:m4_defn — 5 repos
- syntax:token:) — 5 repos
- macro:AC_BEFORE — 4 repos

## Most-needed packages (missing-dep inference)

