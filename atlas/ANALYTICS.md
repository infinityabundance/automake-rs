# Atlas Analytics — corpus intelligence

Total recipes: **970** · court mix: 406 failed, 250 not_standalone, 312 partial, 2 quirk_dependent

## Quirk hotspots (automation candidates)

Quirks matched across recipes — the most frequent are the highest-leverage to auto-apply.

| quirk | repos |
| --- | --- |
| vendored-aclocal | 954 |
| has-m4-macro-dir | 444 |
| uses-libtool | 379 |
| uses-pkg-config | 350 |
| uses-ax-archive | 325 |
| uses-subdir-objects | 311 |
| uses-maintainer-mode | 264 |
| uses-libtool-old | 234 |
| has-acinclude | 103 |
| uses-gettext | 88 |
| uses-pthread-check | 73 |
| uses-python | 69 |
| perl-in-configure | 65 |
| uses-intltool | 28 |
| emits-config-commands-post | 4 |

## Top failure roots (the check ours died on)

| check | repos |
| --- | --- |
| checking for python | 13 |
| checking for strtol | 13 |
| checking for strstr | 11 |
| checking for C++ compiler | 10 |
| checking for unistd.h | 10 |
| checking pkg-config is at least version 0.9.0 | 10 |
| checking for the pthreads flag | 9 |
| checking whether pthreads work with -mt | 9 |
| checking that generated files are newer than configure | 8 |
| checking endianness | 7 |

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
| 37422 | BOINC/boinc | not_standalone |
| 35283 | Distrotech/squid | failed |
| 34958 | CoachRun/boinc | not_standalone |
| 34816 | Distrotech/tar | partial |
| 34089 | Distrotech/radius | partial |
| 33403 | digitalocean/hivex | partial |
| 31230 | Distrotech/libtool | partial |
| 29530 | Distrotech/gzip | partial |

## Partial -> full shortlist

**312** recipes cleared configure but failed make; **133** of those are `OURS_BUG_MAKE` (GNU makes it, we don't) — the closest wins. Top blockers:

| blocker | repos |
| --- | --- |
| Makefile:16: *** missing separator.  Sto | 99 |
| make: *** No targets specified and no ma | 7 |
| Makefile:15: *** missing separator.  Sto | 2 |
| leaked-macro:automake-1.13 | 2 |
| leaked-macro:GLIB_GSETTINGS | 1 |
| leaked-macro:LT_PATH_LD | 1 |
| leaked-macro:automake-1.15 | 1 |
| leaked-macro:esd-config | 1 |
| leaked-macro:strlcat | 1 |
| make: *** No targets.  Stop. | 1 |
