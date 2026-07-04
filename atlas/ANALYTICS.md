# Atlas Analytics — corpus intelligence

Total recipes: **1000** · court mix: 330 failed, 225 not_standalone, 383 partial, 62 quirk_dependent

## Quirk hotspots (automation candidates)

Quirks matched across recipes — the most frequent are the highest-leverage to auto-apply.

| quirk | repos |
| --- | --- |
| vendored-aclocal | 999 |
| has-m4-macro-dir | 466 |
| uses-libtool | 396 |
| uses-pkg-config | 362 |
| uses-ax-archive | 338 |
| uses-subdir-objects | 325 |
| uses-maintainer-mode | 282 |
| uses-libtool-old | 244 |
| has-acinclude | 111 |
| uses-gettext | 98 |
| uses-pthread-check | 77 |
| uses-python | 74 |
| perl-in-configure | 73 |
| uses-intltool | 30 |
| emits-config-commands-post | 4 |

## Top failure roots (the check ours died on)

| check | repos |
| --- | --- |
| checking for C compiler | 15 |
| checking for C++ compiler | 13 |
| checking for strstr | 12 |
| checking for strtol | 11 |
| checking whether pthreads work with -mt | 11 |
| checking for msgfmt | 10 |
| checking for strdup | 9 |
| checking for unistd.h | 9 |
| checking for MSG_NOSIGNAL | 8 |
| checking for malloc | 8 |

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
| 4825556 | Distrotech/Thunar | failed |
| 4206770 | cpputest/cpputest | failed |
| 4206250 | emcrisostomo/fswatch | failed |
| 4206203 | cbg-ethz/shorah | not_standalone |
| 4206112 | flux-framework/flux-foundry | failed |
| 4206109 | flux-framework/flux-pmix | not_standalone |
| 4205925 | anyone-protocol/ator-protocol | not_standalone |
| 93300 | FinchBerryOS/fbyo-coreutils | partial |
| 56714 | FOSSEE/scilab_for_xcos_on_cloud | not_standalone |
| 45995 | DistributedSpectrum/libcurl | partial |
| 36368 | cooljeanius/magicseteditor | not_standalone |
| 29567 | gooroom/gtk3 | partial |

## Partial -> full shortlist

**383** recipes cleared configure but failed make; **158** of those are `OURS_BUG_MAKE` (GNU makes it, we don't) — the closest wins. Top blockers:

| blocker | repos |
| --- | --- |
| collect2: error: ld returned 1 exit stat | 15 |
| (.text+0x1b): undefined reference to `ma | 4 |
| make[1]: *** No rule to make target 'all | 4 |
| cc: error: unrecognized command-line opt | 3 |
| ../config.h:2:24: error: 'x86_64' undecl | 2 |
| /usr/bin/ld: cannot open output file bin | 2 |
| aminclude.am:35: *** missing separator.  | 2 |
| cc: error: 0.2: linker input file not fo | 2 |
| cc: error: 1.0: linker input file not fo | 2 |
| ../../include/STTypes.h:142:14: error: # | 1 |
| ../../include/mupnp/util/Vector.h:25:41: | 1 |
| ../../src/network.h:32:10: fatal error:  | 1 |
