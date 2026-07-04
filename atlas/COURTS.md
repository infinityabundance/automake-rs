# Build Courts — automake-rs Atlas gap analysis

Total recipes: **1000**

## Court status

| status | count | meaning |
|---|---|---|
| failed | 330 | ours fails before make |
| not_standalone | 225 | oracle (GNU) also fails — not our bug |
| partial | 383 | configure cleared, make failed |
| quirk_dependent | 62 | FUNC_OK but needed a quirk rule |

## Head-to-head vs GNU autotools

**1000** repos built by both toolchains. Rust vs GNU: **234 win** (23%) · **49 tie** (4%) · **384 loss** (38%) · **333 both-fail** (33%).

| outcome | repos | meaning |
|---|---|---|
| ours-better | 234 | Rust toolchain got further than GNU |
| identical-status | 49 | both reached `make` — dead heat |
| ours-worse | 384 | GNU got further than the Rust toolchain (our bug) |
| both-fail | 333 | neither toolchain finished (upstream/env, not our bug) |

### Ladder — how far each toolchain reached (same repos)

| stage reached | Rust | GNU |
|---|---|---|
| make ok | 62 | 333 |
| configure ok | 383 | 208 |
| configure generated | 554 | 300 |
| failed to generate | 1 | 159 |

### Contingency (Rust rows × GNU cols)

| Rust ↓ / GNU → | make | config | gen | none |
|---|---|---|---|---|
| **make** | 49 | 3 | 4 | 6 |
| **config** | 158 | 105 | 71 | 49 |
| **gen** | 126 | 100 | 224 | 104 |
| **none** | 0 | 0 | 1 | 0 |

### Rust wins (234) — we build, GNU does not

| repo | Rust reached |
|---|---|
| 4ZM/mfterm | configure ok |
| 5GenCrypto/mife | configure ok |
| ASPLes/libaxl | configure generated |
| AdolfVonKleist/Phonetisaurus | configure generated |
| AmkG/hl | configure generated |
| ArcticaProject/libpam-x2go | configure ok |
| BIC-MNI/minc | configure ok |
| BioND/myrng | configure ok |
| BirolLab/biobloom | configure ok |
| BroadbandForum/obuspa | configure ok |
| BunsenLabs/plank | configure generated |
| CICM/HoaLibrary-PD | configure ok |
| CNGLDLab/LORG-Release | configure generated |
| CS198NDSGChanBrianJoe/html5rdp | configure generated |
| CkNoSFeRaTU/pidgin | configure generated |
| ConsoleKit2/ConsoleKit2 | configure generated |
| Dansguardian/dansguardian | configure generated |
| DavidGriffith/hf | configure ok |
| Distrotech/Thunar | configure generated |
| Distrotech/diffutils | configure generated |
| Distrotech/evolution | configure generated |
| Distrotech/findutils | configure generated |
| Distrotech/gnome-mines | configure ok |
| Distrotech/gtkimageview | configure generated |
| Distrotech/libaccounts-glib | configure ok |
| Distrotech/libgweather | configure generated |
| Distrotech/libjpeg-turbo | configure ok |
| Distrotech/libsecret | configure generated |
| Distrotech/libtool | configure generated |
| Distrotech/libwnck | configure generated |
| Distrotech/onig | configure ok |
| Distrotech/popt | configure generated |
| Distrotech/pulseaudio | configure generated |
| Dr-Shadow/netsoul-purple | configure ok |
| EasyRPG/Tools | make ok |
| Elive/engage | configure ok |
| Enigma-Game/Enigma | configure generated |
| FauxFaux/fastjar | configure generated |
| FinTP/fintp_payloadevaluators | configure ok |
| FinchBerryOS/fbyo-coreutils | configure ok |

### Our bugs (384) — GNU builds further than us (the actionable backlog)

| repo | GNU reached | our first error |
|---|---|---|
| 01micko/pup-volume-monitor | make ok | collect2: error: ld returned 1 exit status |
| 0xADE1A1DE/AssemblyLine | make ok | /usr/bin/ld: cannot find -lassemblyline: No such file or directory |
| 6WIND/quagga | make ok | ./configure: line 3247: AC_WORDS_BIGENDIAN: command not found |
| A-Kyle/GrADS-CJK | configure ok | checking for supplibs directory... ./configure: line 2328: syntax error near une |
| a-sassmannshausen/guile-monads | make ok | ./configure: line 1366: syntax error near unexpected token `2.0.11' |
| abc100m/libzdbcpp | make ok |  |
| abhaykadam/vm | make ok | <command-line>: error: expected ',' or ';' before numeric constant |
| accellera-official/systemc | make ok | whether we are using a Clang/LLVM C compiler... configure: error: "sorry...archi |
| achernya/hesiod | make ok | hespwnam.c:116:5: error: 'struct passwd' has no member named 'pw_quota' |
| adiknoth/netatalk-debian | configure ok | ./configure: line 1390: AC_PROG_PERL: command not found |
| adulau/dcfldd | make ok | sys2.h:408:23: error: 'CHAR_MIN' undeclared here (not in a function) |
| ADVANTECH-Corp/WiseSnail | configure ok | ./configure: line 1673: ./library/WiseCore/WiseCore_MQTT/configure: No such file |
| affix/MSNC | make ok | configure: error: You don't seem to have the curses headers installed |
| agn453/ZXCC | make ok | ./configure: line 1561: ./cpmio/configure: No such file or directory |
| ahkok/bootchart | make ok | svg.c:92:85: error: 'VERSION' undeclared (first use in this function) |
| ahorn/cpp-channel | make ok | ./configure: line 1369: ./gtest/configure: No such file or directory |
| ahupowerdns/setgrouper | make ok | configure: error: no C++ compiler found |
| aizvorski/h264bitstream | make ok | cc: error: 0.2.0: linker input file not found: No such file or directory |
| ajnelson/photorec-testdisk | configure ok | configure: error: At least one of ncursesw/ncurses/pdcurses/curses library must  |
| alexmarsev/soundtouch | make ok | ../../include/STTypes.h:142:14: error: #error "conflicting sample types defined" |
| alito/smallpotato | make ok | src/unches.c:193:44: error: 'VERSION' undeclared (first use in this function) |
| allinurl/goaccess | make ok | make[2]: *** No rule to make target 'all'.  Stop. |
| allinurl/gwsocket | make ok | src/gwsocket.c:45:10: fatal error: config.h: No such file or directory |
| alobbs/macchanger | make ok | main.c:160:33: error: 'VERSION' undeclared (first use in this function) |
| Alpacius/a6 | make ok | /usr/bin/ld: cannot find -la6: No such file or directory |
| alsa-project/alsa-firmware | make ok |  |
| amadvance/advancecomp | make ok | rezip.cc:460:17: error: 'PACKAGE' was not declared in this scope; did you mean ' |
| andrewshadura/tnat64 | make ok | configure: error: 'Could not find library containing connect()' |
| AndyA/htop-osx | configure ok | ./configure: line 1810: syntax error near unexpected token `fi' |
| AndyA/psips | make ok | /usr/bin/ld: cannot find -ltest-support: No such file or directory |
| AndyA/rfile | make ok | make[2]: tools/serializator.pl: Permission denied |
| anewhuahua/bilitw | configure ok | ./configure: line 2015: ./contrib/yaml-0.1.4/configure: No such file or director |
| AnthonyBradford/optionmatrix | make ok | Makefile:228: *** Recursive variable 'CXXFLAGS' references itself (eventually).  |
| antiprism/mpd_oled | make ok | collect2: error: ld returned 1 exit status |
| AOMediaCodec/oac | make ok | ./configure: line 2382: -mfma: command not found |
| apache/xerces-c | make ok | ./configure: line 1796: pthread-config: command not found |
| apereo/mod_auth_cas | make ok | make[2]: *** No rule to make target 'mod_auth_cas.la.lo', needed by 'mod_auth_ca |
| apertium/apertium | make ok | configure: error: You don't have xmllint installed. |
| archiecobbs/libnbcompat | make ok | make[1]: *** No rule to make target '.libs/libnbcompat.la', needed by 'rmd160-te |
| aristanetworks/EosSdk | configure ok | for std::unordered_set::operator==... configure: error: Your version of the STL  |
| armadito/armadito-av | make ok | stdpaths.c:64:23: error: 'LIBARMADITO_MODULES_PATH' undeclared (first use in thi |
| arthurdejong/nss-pam-ldapd | make ok | configure: error: PAM header files are missing |
| asciinema/libtsm | make ok | src/gtktsm/gtktsm-terminal.c:32:10: fatal error: libtsm.h: No such file or direc |
| asnelt/rrep | make ok | configure: error: Invalid value for --with-included-regex: |
| astromatic/psfex | make ok | ******** Configuring:  PSFEx 3.24.2 -  (2026-07-04) ******** |
| astromatic/sextractor | make ok | ******** Configuring:  SExtractor 2.29.0 -  (2026-07-04) ******** |
| astromatic/skymaker | make ok | ******** Configuring:  SkyMaker 4.3.0 -  (2026-07-04) ******** |
| atinm/poker-eval | make ok | /tmp/atlasx_atinm__poker-eval/s/lib/mktab_basic.c:228:(.text.startup+0x38): unde |
| autch/demucc | make ok | (.text+0x1b): undefined reference to `main' |
| autotools-mirror/autoconf | make ok | ./configure: line 2812: syntax error: unexpected end of file |
| avahi/nss-mdns | make ok | collect2: error: ld returned 1 exit status |
| avr-aics-riken/234Compositor | configure ok |  |
| avrdudes/avarice | make ok | /bin/sh: 1: -DHAVE_CONFIG_H: not found |
| awaw/dnsproxy | make ok | checking for libevent... configure: error: |
| awgn/brute | configure ok | ./configure: line 1573: syntax error near unexpected token `string.h' |
| Bader-Research/snap-graph | make ok | collect2: error: ld returned 1 exit status |
| balde/balde | configure ok | configure: error: no -fvisibility=hidden support found in , balde requires -fvis |
| bambulab/gmp | make ok | ./configure: line 1465: syntax error near unexpected token `newline' |
| barak/djview4 | configure ok | conftest.d/conftest.sh: line 1: creating: command not found |
| barak/djvulibre | make ok | Arrays.cpp:57:11: fatal error: config.h: No such file or directory |
| barak/oaklisp | configure ok | for long long int... ./configure: line 1793: syntax error near unexpected token  |
| baszoetekouw/pinfo | make ok |  |
| BatchDrake/lfsrintruder | make ok | collect2: error: ld returned 1 exit status |
| bbc/vc2hqdecode | make ok | configure: error: librt is required |
| bcoin-org/libtorsion | make ok | ./configure: line 3495: syntax error near unexpected token `;' |
| bdwgc/bdwgc | make ok | ./configure: line 1645: syntax error near unexpected token `(' |
| benegon/ntp | configure ok | ./configure: line 1823: syntax error near unexpected token `sntp/libopts' |
| benlemasurier/stormfs | make ok | proxy.c:20:10: fatal error: glib.h: No such file or directory |
| benmwebb/dopewars | make ok | ../../src/network.h:32:10: fatal error: winsock2.h: No such file or directory |
| benvanik/gflags | make ok | ./configure: line 1854: pthread-config: command not found |
| benwbooth/tvision | configure ok | ./configure: line 1443: AC_STDC_HEADERS: command not found |
| bestouff/genext2fs | make ok | genext2fs.c:4351:28: error: expected ')' before 'VERSION' |
| bigmc/bigmc | make ok | make[1]: *** No rule to make target 'bgparser.hpp', needed by 'all'.  Stop. |
| binhqnguyen/ovs-srv6 | configure ok | whether cc accepts -Werror... ./configure: line 2760: syntax error near unexpect |
| BirolLab/ChopStitch | configure ok | configure: error: CHOPSTITCH must be compiled with a C++ compiler that supports  |
| BirolLab/ntCard | configure ok | configure: error: NTCARD must be compiled with a C++ compiler that supports Open |
| bitcoin-core/minisketch | make ok | /usr/bin/ld: cannot find -lminisketch: No such file or directory |
| BitzenyCoreDevelopers/cpuminer | make ok | ./configure: line 1801: syntax error near unexpected token `,' |
| bjango/istatserverlinux | make ok | configure: error: sqlite not found. please install libsqlite3-dev/sqlite-devel o |
| BlockstreamResearch/secp256k1-zkp | make ok | cc: error: _PKG_VERSION_MAJOR,: linker input file not found: No such file or dir |

## Oracle headroom

ours configure-clear: **445** · GNU configure-clear: **541** · fixable our-bug headroom: **96**

## Top fixable roots (real succeeds, ours fails)

- syntax:other — 33 repos
- syntax:token:( — 21 repos
- syntax:unbalanced-loop — 14 repos
- syntax:leaked-text-after-conditional — 11 repos
- syntax:unbalanced-conditional — 11 repos
- macro:ovs_cv_m4_translit — 4 repos
- macro:AN_MAKEVAR — 3 repos
- macro:GUILE_PROGS — 2 repos
- macro:LTDL_INIT — 2 repos
- macro:fim4_require — 2 repos
- syntax:token:; — 2 repos
- syntax:token:<<< — 2 repos
- macro:ARG_DISBL_SET — 1 repos
- macro:ENABLE_m4_translit — 1 repos
- macro:LIBCURL_CHECK_CONFIG — 1 repos

## Most-needed packages (missing-dep inference)


## Make-layer roots (the next front: 383 partial repos clear configure but fail make)

- compiler-error — 153 repos
- missing-header-at-compile — 88 repos
- other — 60 repos
- no-rule-to-make-target — 38 repos
- undefined-reference (link) — 30 repos
- makefile-missing-separator — 8 repos
- permission-denied — 4 repos
- command-not-found — 1 repos
- makefile/shell-syntax-error — 1 repos
