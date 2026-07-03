# Atlas Analytics — corpus intelligence

Total recipes: **986** · court mix: 535 failed, 10 not_standalone, 380 partial, 61 quirk_dependent

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
| uses-libtool-old | 240 |
| has-acinclude | 108 |
| uses-gettext | 94 |
| uses-pthread-check | 76 |
| uses-python | 72 |
| perl-in-configure | 71 |
| uses-intltool | 29 |
| emits-config-commands-post | 4 |

## Top failure roots (the check ours died on)

| check | repos |
| --- | --- |
| checking for C compiler | 14 |
| checking for C++ compiler | 12 |
| checking for strstr | 12 |
| checking for strtol | 11 |
| checking whether pthreads work with -mt | 11 |
| checking for msgfmt | 9 |
| checking for strdup | 9 |
| checking for unistd.h | 9 |
| checking for MSG_NOSIGNAL | 8 |
| checking for python | 8 |

## Dependency patterns

**Most-needed headers**

| header | repos |
| --- | --- |
| glib.h | 6 |
| config.h | 5 |
| sys/sysctl.h | 3 |
| conf/config.h | 2 |
| lua.h | 2 |
| ../lib/config.h | 1 |
| OVR.h | 1 |
| OgreCamera.h | 1 |
| ParserEventGeneratorKit.h | 1 |
| SDL.h | 1 |
| SDL3/SDL.h | 1 |
| SDL_framerate.h | 1 |

**Most-missing deps**

| dep | repos |
| --- | --- |
| glib.h | 6 |
| config.h | 5 |
| sys/sysctl.h | 3 |
| conf/config.h | 2 |
| lua.h | 2 |
| ../lib/config.h | 1 |
| OVR.h | 1 |
| OgreCamera.h | 1 |
| ParserEventGeneratorKit.h | 1 |
| SDL.h | 1 |
| SDL3/SDL.h | 1 |
| SDL_framerate.h | 1 |

## Heavy hitters (configure size = complexity proxy)

| configure lines | repo | court |
| --- | --- | --- |
| 4206770 | cpputest/cpputest | failed |
| 4206250 | emcrisostomo/fswatch | failed |
| 4206203 | cbg-ethz/shorah | failed |
| 4206109 | flux-framework/flux-pmix | failed |
| 4205925 | anyone-protocol/ator-protocol | failed |
| 36368 | cooljeanius/magicseteditor | failed |
| 35283 | Distrotech/squid | failed |
| 34089 | Distrotech/radius | partial |
| 29567 | gooroom/gtk3 | partial |
| 25218 | hhool/nut | failed |
| 22513 | Distrotech/Thunar | failed |
| 22502 | Distrotech/evolution | failed |

## Partial -> full shortlist

**380** recipes cleared configure but failed make; **1** of those are `OURS_BUG_MAKE` (GNU makes it, we don't) — the closest wins. Top blockers:

| blocker | repos |
| --- | --- |
| Makefile:16: *** missing separator.  Sto | 1 |
