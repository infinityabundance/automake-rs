# Build Courts — automake-rs Atlas gap analysis

Total recipes: **434**

## Court status

| status | count | meaning |
|---|---|---|
| failed | 196 | ours fails before make |
| not_standalone | 102 | oracle (GNU) also fails — not our bug |
| partial | 133 | configure cleared, make failed |
| quirk_dependent | 3 | FUNC_OK but needed a quirk rule |

## Oracle headroom

ours configure-clear: **136** · GNU configure-clear: **242** · fixable our-bug headroom: **106**

## Top fixable roots (real succeeds, ours fails)

- syntax:backtick-in-source — 34 repos
- macro:AC_CHECK_DECL — 10 repos
- macro:AC_CHECK_HEADERS_ONCE — 8 repos
- syntax:unbalanced-conditional — 7 repos
- macro:AC_DEFINE — 6 repos
- macro:AC_MSG_ERROR — 6 repos
- macro:AC_LANG_CONFTEST — 5 repos
- macro:AC_SUBST — 5 repos
- macro:AC_CONFIG_COMMANDS_PRE — 4 repos
- macro:AC_ERROR — 4 repos
- macro:AC_MSG_NOTICE — 4 repos
- macro:AC_RUN_LOG — 4 repos
- macro:AS_ECHO — 4 repos
- macro:AC_BEFORE — 3 repos
- macro:AC_CONFIG_COMMANDS — 3 repos

## Most-needed packages (missing-dep inference)

