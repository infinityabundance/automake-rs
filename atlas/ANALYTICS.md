# Atlas Analytics — corpus intelligence

Total recipes: **986** · court mix: 368 failed, 250 not_standalone, 2 quirk_dependent

## Quirk hotspots (automation candidates)

Quirks matched across recipes — the most frequent are the highest-leverage to auto-apply.

| quirk | repos |
| --- | --- |
| vendored-aclocal | 616 |
| has-m4-macro-dir | 342 |
| uses-libtool | 279 |
| uses-ax-archive | 274 |
| uses-pkg-config | 238 |
| uses-subdir-objects | 232 |
| uses-maintainer-mode | 200 |
| uses-libtool-old | 155 |
| has-acinclude | 85 |
| uses-pthread-check | 62 |
| perl-in-configure | 54 |
| uses-gettext | 53 |
| uses-python | 49 |
| uses-intltool | 16 |
| emits-config-commands-post | 3 |

## Top failure roots (the check ours died on)

| check | repos |
| --- | --- |
| checking for C++ compiler | 10 |
| checking pkg-config is at least version 0.9.0 | 10 |
| checking for unistd.h | 9 |
| checking whether pthreads work with -mt | 9 |
| checking for python | 7 |
| checking for the pthreads flag | 6 |
| checking endianness | 5 |
| checking for $ac_func | 4 |
| checking for $ac_hdr | 4 |
| checking for 64-bit off_t | 3 |

## Dependency patterns

**Most-needed headers**

| header | repos |
| --- | --- |
| /home/dev/atb_01nc9k/t/mk:glib.h | 1 |
| /home/dev/atb_2H0O9n/t/mk:config.h | 1 |
| /home/dev/atb_31qL2c/t/mk:gtk/gtk.h | 1 |
| /home/dev/atb_4Cefdn/t/mk:OgreCamera.h | 1 |
| /home/dev/atb_5BC83e/t/mk:ilcplex/ilocplex.h | 1 |
| /home/dev/atb_5UqC8c/t/mk:glib/gstdio.h | 1 |
| /home/dev/atb_5gOhho/t/mk:dmp_lua_str.h | 1 |
| /home/dev/atb_5xUS32/t/mk:evd.h | 1 |
| /home/dev/atb_6cI3be/t/mk:llvm/Analysis/Verifier.h | 1 |
| /home/dev/atb_7Uhod6/t/mk:apteryx.h | 1 |
| /home/dev/atb_8SHFLY/t/mk:fcl.h | 1 |
| /home/dev/atb_AIwVAK/t/mk:arrayqueue.h | 1 |

**Most-missing deps**

| dep | repos |
| --- | --- |
| /home/dev/atb_01nc9k/t/mk:glib.h | 1 |
| /home/dev/atb_2H0O9n/t/mk:config.h | 1 |
| /home/dev/atb_31qL2c/t/mk:gtk/gtk.h | 1 |
| /home/dev/atb_4Cefdn/t/mk:OgreCamera.h | 1 |
| /home/dev/atb_5BC83e/t/mk:ilcplex/ilocplex.h | 1 |
| /home/dev/atb_5UqC8c/t/mk:glib/gstdio.h | 1 |
| /home/dev/atb_5gOhho/t/mk:dmp_lua_str.h | 1 |
| /home/dev/atb_5xUS32/t/mk:evd.h | 1 |
| /home/dev/atb_6cI3be/t/mk:llvm/Analysis/Verifier.h | 1 |
| /home/dev/atb_7Uhod6/t/mk:apteryx.h | 1 |
| /home/dev/atb_8SHFLY/t/mk:fcl.h | 1 |
| /home/dev/atb_AIwVAK/t/mk:arrayqueue.h | 1 |

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
| 27709 | cosmos72/twin | quirk_dependent |
| 25735 | hhool/nut | failed |
| 25282 | hkerem/squid3-ssl | failed |
| 24516 | CoryXie/GRUB2 | not_standalone |
| 22513 | Distrotech/Thunar | failed |

## Partial -> full shortlist

**0** recipes cleared configure but failed make; **0** of those are `OURS_BUG_MAKE` (GNU makes it, we don't) — the closest wins. Top blockers:

| blocker | repos |
| --- | --- |
