# Atlas Analytics — corpus intelligence

Total recipes: **986** · court mix: 409 failed, 250 not_standalone, 324 partial, 3 quirk_dependent

## Quirk hotspots (automation candidates)

Quirks matched across recipes — the most frequent are the highest-leverage to auto-apply.

| quirk | repos |
| --- | --- |
| vendored-aclocal | 985 |
| has-m4-macro-dir | 455 |
| uses-libtool | 390 |
| uses-pkg-config | 357 |
| uses-ax-archive | 333 |
| uses-subdir-objects | 320 |
| uses-maintainer-mode | 275 |
| uses-libtool-old | 241 |
| has-acinclude | 108 |
| uses-gettext | 94 |
| uses-pthread-check | 76 |
| perl-in-configure | 72 |
| uses-python | 72 |
| uses-intltool | 29 |
| emits-config-commands-post | 4 |

## Top failure roots (the check ours died on)

| check | repos |
| --- | --- |
| checking whether pthreads work with -mt | 23 |
| checking for python | 14 |
| checking for strstr | 13 |
| checking for strtol | 13 |
| checking for C++ compiler | 10 |
| checking pkg-config is at least version 0.9.0 | 10 |
| checking for unistd.h | 9 |
| checking endianness | 8 |
| checking for malloc | 8 |
| checking for size_t | 8 |

## Dependency patterns

**Most-needed headers**

| header | repos |
| --- | --- |

## Heavy hitters (configure size = complexity proxy)

| configure lines | repo | court |
| --- | --- | --- |
| 3888900 | hroptatyr/truffle | failed |
| 3888897 | hroptatyr/yuck | failed |
| 3888866 | hroptatyr/clob | failed |
| 3850569 | hroptatyr/echse | failed |
| 47807 | hpc/Parallel-coreutils | partial |
| 39480 | Distrotech/diffutils | partial |
| 37674 | BOINC/boinc | not_standalone |
| 36368 | cooljeanius/magicseteditor | not_standalone |
| 35290 | Distrotech/squid | failed |
| 35210 | CoachRun/boinc | not_standalone |
| 34089 | Distrotech/radius | partial |
| 33975 | Distrotech/sharutils | partial |

## Partial -> full shortlist

**324** recipes cleared configure but failed make; **149** of those are `OURS_BUG_MAKE` (GNU makes it, we don't) — the closest wins. Top blockers:

| blocker | repos |
| --- | --- |
| leaked-macro:AM_PROG_CC_STDC | 2 |
| leaked-macro:LT_PATH_LD | 2 |
| leaked-macro:AC_FUNC_LSTAT | 1 |
| leaked-macro:AC_SET_MAKE | 1 |
| leaked-macro:AM_PATH_CCACHE | 1 |
| leaked-macro:AM_PATH_GLIB_2_0 | 1 |
| leaked-macro:AX_CFLAGS_WARN_ALL | 1 |
| leaked-macro:GLIB_GSETTINGS | 1 |
| leaked-macro:LT_LIB_M | 1 |
| leaked-macro:ac_cv_have_decl_strtol, | 1 |
| leaked-macro:automake-1.15 | 1 |
| leaked-macro:ax_cv_check__AC_LANG_ABBREVflags__-mavx=yes | 1 |
