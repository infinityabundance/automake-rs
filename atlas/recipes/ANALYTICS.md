# Atlas Analytics — corpus intelligence

Total recipes: **434** · court mix: 190 failed, 102 not_standalone, 139 partial, 3 quirk_dependent

## Quirk hotspots (automation candidates)

Quirks matched across recipes — the most frequent are the highest-leverage to auto-apply.

| quirk | repos |
| --- | --- |
| vendored-aclocal | 434 |
| has-m4-macro-dir | 188 |
| uses-libtool | 173 |
| uses-pkg-config | 147 |
| uses-subdir-objects | 144 |
| uses-ax-archive | 143 |
| uses-maintainer-mode | 113 |
| uses-libtool-old | 102 |
| has-acinclude | 44 |
| perl-in-configure | 39 |
| uses-gettext | 38 |
| uses-pthread-check | 31 |
| uses-python | 27 |
| uses-intltool | 11 |
| emits-config-commands-post | 2 |

## Top failure roots (the check ours died on)

| check | repos |
| --- | --- |
| checking whether pthreads work with -mt | 9 |
| checking for malloc | 6 |
| checking for strstr | 6 |
| checking for unistd.h | 6 |
| checking pkg-config is at least version 0.9.0 | 6 |
| checking for C++ compiler | 5 |
| checking endianness | 4 |
| checking for memset | 4 |
| checking for python | 4 |
| checking that generated files are newer than configure | 4 |

## Dependency patterns

**Most-needed headers**

| header | repos |
| --- | --- |

## Heavy hitters (configure size = complexity proxy)

| configure lines | repo | court |
| --- | --- | --- |
| 3888900 | hroptatyr/truffle | failed |
| 3888897 | hroptatyr/yuck | failed |
| 39480 | Distrotech/diffutils | partial |
| 37561 | BOINC/boinc | not_standalone |
| 33403 | digitalocean/hivex | partial |
| 31220 | cydhaselton/mono-android | failed |
| 28582 | ayumin/open-cobol | quirk_dependent |
| 27709 | cosmos72/twin | quirk_dependent |
| 25289 | hkerem/squid3-ssl | failed |
| 24557 | chimari/MaCoPiX | quirk_dependent |
| 19212 | csmith-project/creduce | partial |
| 14102 | daveyc/gawk_zos | partial |

## Partial -> full shortlist

**139** recipes cleared configure but failed make; **69** of those are `OURS_BUG_MAKE` (GNU makes it, we don't) — the closest wins. Top blockers:

| blocker | repos |
| --- | --- |
| leaked-macro:AC_SET_MAKE | 1 |
| leaked-macro:AM_PROG_CC_STDC | 1 |
| leaked-macro:AX_CFLAGS_WARN_ALL | 1 |
| leaked-macro:GLIB_GSETTINGS | 1 |
| leaked-macro:LT_PATH_LD | 1 |
| leaked-macro:automake-1.15 | 1 |
| leaked-macro:ax_cv_check__AC_LANG_ABBREVflags__-mavx=yes | 1 |
| leaked-macro:break_AC_LANG_ABBREV | 1 |
| leaked-macro:esd-config | 1 |
| undefined-macro | 1 |
