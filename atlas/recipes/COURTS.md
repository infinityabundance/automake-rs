# Build Courts — automake-rs Atlas gap analysis

Total recipes: **434**

## Court status

| status | count | meaning |
|---|---|---|
| failed | 190 | ours fails before make |
| not_standalone | 102 | oracle (GNU) also fails — not our bug |
| partial | 139 | configure cleared, make failed |
| quirk_dependent | 3 | FUNC_OK but needed a quirk rule |

## Oracle headroom

ours configure-clear: **142** · GNU configure-clear: **242** · fixable our-bug headroom: **100**

## Top fixable roots (real succeeds, ours fails)

- syntax:other — 13 repos
- syntax:unbalanced-conditional — 10 repos
- syntax:token:( — 7 repos
- syntax:unbalanced-loop — 7 repos
- macro:AC_MSG_ERROR — 4 repos
- macro:AC_MSG_NOTICE — 4 repos
- macro:AC_SUBST — 4 repos
- macro:AS_ECHO — 4 repos
- macro:AC_BEFORE — 3 repos
- macro:AC_CONFIG_COMMANDS — 3 repos
- macro:AC_DEFINE — 3 repos
- macro:AC_PREREQ — 3 repos
- macro:AC_REQUIRE — 3 repos
- macro:AM_CONDITIONAL — 3 repos
- macro:AS_MKDIR_P — 3 repos

## Most-needed packages (missing-dep inference)

