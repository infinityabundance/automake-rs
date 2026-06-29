# Atlas Analytics — corpus intelligence

Total recipes: **982** · court mix: 364 failed, 250 not_standalone, 366 partial, 2 quirk_dependent

## Quirk hotspots (automation candidates)

Quirks matched across recipes — the most frequent are the highest-leverage to auto-apply.

| quirk | repos |
| --- | --- |
| vendored-aclocal | 978 |
| has-m4-macro-dir | 450 |
| uses-libtool | 387 |
| uses-pkg-config | 354 |
| uses-ax-archive | 328 |
| uses-subdir-objects | 319 |
| uses-maintainer-mode | 271 |
| uses-libtool-old | 240 |
| has-acinclude | 106 |
| uses-gettext | 93 |
| uses-pthread-check | 75 |
| uses-python | 71 |
| perl-in-configure | 69 |
| uses-intltool | 29 |
| emits-config-commands-post | 4 |

## Top failure roots (the check ours died on)

| check | repos |
| --- | --- |
| checking for strstr | 15 |
| checking for python | 14 |
| checking for strtol | 13 |
| checking for unistd.h | 12 |
| checking for C++ compiler | 10 |
| checking pkg-config is at least version 0.9.0 | 10 |
| checking for the pthreads flag | 9 |
| checking whether pthreads work with -mt | 9 |
| checking for malloc | 8 |
| checking that generated files are newer than configure | 8 |

## Dependency patterns

**Most-needed headers**

| header | repos |
| --- | --- |

## Heavy hitters (configure size = complexity proxy)

| configure lines | repo | court |
| --- | --- | --- |
| 3888144 | hroptatyr/truffle | failed |
| 3888141 | hroptatyr/yuck | failed |
| 3888110 | hroptatyr/clob | failed |
| 3849820 | hroptatyr/echse | failed |
| 38589 | crystax/android-vendor-gnu-tar | partial |
| 37422 | BOINC/boinc | not_standalone |
| 35283 | Distrotech/squid | failed |
| 34958 | CoachRun/boinc | not_standalone |
| 34816 | Distrotech/tar | partial |
| 34089 | Distrotech/radius | partial |
| 33403 | digitalocean/hivex | partial |
| 31230 | Distrotech/libtool | partial |

## Partial -> full shortlist

**366** recipes cleared configure but failed make; **163** of those are `OURS_BUG_MAKE` (GNU makes it, we don't) — the closest wins. Top blockers:

| blocker | repos |
| --- | --- |
| leaked-macro:AM_PROG_CC_STDC | 2 |
| leaked-macro:LT_PATH_LD | 2 |
| leaked-macro:AC_CHECK_VA_COPY | 1 |
| leaked-macro:AC_FUNC_LSTAT | 1 |
| leaked-macro:AC_OPTIMIZE | 1 |
| leaked-macro:AC_SET_MAKE | 1 |
| leaked-macro:AC_TYPE_UNSIGNED_LONG_LONG_INT | 1 |
| leaked-macro:AM_PATH_CCACHE | 1 |
| leaked-macro:AM_PATH_GLIB_2_0 | 1 |
| leaked-macro:AX_CFLAGS_WARN_ALL | 1 |
| leaked-macro:GLIB_GSETTINGS | 1 |
| leaked-macro:LT_LIB_M | 1 |
