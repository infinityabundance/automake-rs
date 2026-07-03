# Atlas Build Atlas — corpus compile report

Generated: 2026-07-03T11:32:12.719363Z  ·  toolchain: autoconf-rs 0.1.60 / automake (GNU automake) UNKNOWN

**Total recipes: 366**

## Compile ladder (how far each repo got)

| stage | repos | pct |
| --- | --- | --- |
| FUNC_OK | 53 | 14% |
| MAKE_FAIL | 265 | 72% |
| CONFIGURE_RUN_FAIL | 48 | 13% |
| CONFIGURE_GEN_FAIL | 0 | 0% |
| NO_AC | 0 | 0% |
| CLONE_FAIL | 0 | 0% |

**CONFIGURE-CLEAN (reached ./configure success): 318 / 366 (86%)**
**FULLY COMPILES (make exit 0 = FUNC_OK): 53 / 366 (14%)**

## Quirk hotspots

| quirk | repos |
| --- | --- |
| subdirs | 254 |
| libtool | 178 |
| pkg-config | 118 |
| autoconf-archive-macros | 56 |
| gettext | 41 |

## Top failure diagnostics (non-FUNC_OK)

| count | first-line diagnostic |
| --- | --- |
| 59 | `collect2: error: ld returned 1 exit status` |
| 8 | `cc: error: VER: linker input file not found: No such file or directory` |
| 7 | `FILE:L: error: 'PACKAGE' undeclared (first use in this function); did you mean` |
| 7 | `FILE:L: error: 'VERSION' undeclared (first use in this function)` |
| 6 | `./configure: line N: AM_PROG_CC_STDC: command not found` |
| 6 | `checking for msgfmt... ./configure: line N: syntax error near unexpected token` |
| 5 | `./configure: line N: GLIB_GSETTINGS: command not found` |
| 4 | `configure: error: The --with-packager-{bug-reports,version} options require --` |
| 4 | `FILE:L: error: expected ')' before 'VERSION'` |
| 4 | `FILE:L: error: expected expression before '/' token` |
| 3 | `./configure: line N: syntax error near unexpected token `('` |
| 3 | `./configure: line N: AM_GLIB_GNU_GETTEXT: command not found` |
| 2 | `configure: error: <wchar.h> cannot be used with this compiler (cc  ).` |
| 2 | `./configure: line N: intltool-update: command not found` |
| 2 | `./configure: line N: LT_PATH_LD: command not found` |
| 1 | `./configure: line N: -show: command not found` |
| 1 | `./configure: line N: python: command not found` |
| 1 | `FILE:L: error: unknown type name '__THROW'` |
| 1 | `src/sockserv/Connection.cc:72:24: error: 'read' was not declared in this scope` |
| 1 | `g++: error: oost_cppflags: linker input file not found: No such file or direct` |
| 1 | `./configure: line N: gnash,: command not found` |
| 1 | `FILE:L: error: 'PKG_MAJOR_VERSION' undeclared (first use in this function)` |
| 1 | `FILE:L: error: 'PACKAGE_STRING' was not declared in this scope` |
| 1 | `FILE:L: error: duplicate 'const' declaration specifier [-Werror=duplicate-decl` |
| 1 | `checking for ARTS artsc - version >= VER... ./configure: line N: --cflags: com` |
| 1 | `./configure: line N: AM_C_PROTOTYPES: command not found` |
| 1 | `./configure: line N: --cflags: command not found` |
| 1 | `./configure: line N: GNOME_MAINTAINER_MODE_DEFINES: command not found` |
| 1 | `./configure: line N: --variable=g_ir_scanner: command not found` |
| 1 | `./configure: line N: ac_cv_sizeof_[][_][_]char[_][_]=1: command not found` |

## FUNC_OK repos (fully compiled)

ArcticaProject/lightdm-remote-session-arctica, Chipmaster/kirk, CookieAvenger/Tiny-Manga-Downloader, DE-IBH/imvirt, EasyRPG/Tools, FlexW/tiger-compiler, GArik/bash-completion, GENI-NSF/geni-tools, Geballin/PgBrowse, aadel112/libBlondie, aalex/oscsend, abrt/faf, aconchillo/guile-json, arcalex/racktk, arjunchitturi/htmlstreamparser, aspiers/stow, barak/vobcopy, bingmann/flex-bison-cpp-example, boundarydevices/devregs, bromanbro/taggins, bryteise/ister, charlescui/CBenchmark, circulosmeos/gztool, clone/xml2, commiyou/iniparser, compiz-reloaded/compiz-bcop, containers/oci-umount, crackpkcs12/crackpkcs12, cybergarage/uhttp-cc, df7cb/sdate, dmtx/dmtx-wrappers, dreamlegacy/libusbtuner, dterweij/ndjbdns, eamonnmoloney/thrift-0.6.1, elmar/ldap-git-backup, emk/eshell, endlessm/eos-browser-tools, endlessm/gnome-user-docs, enki/libev, fumiyas/wcwidth-cjk, ganesh503/Asus-Aura, giellalt/keyboard-olo, giellalt/template-shared-und, gizero/autotools-skeleton, glv2/bruteforce-luks, glv2/bruteforce-salted-openssl, glv2/bruteforce-wallet, gnaservicesinc/Challenge4Access, godsflaw/xor_toolkit, greyltc/android_external_sshfs, gvallee/c_hello_world, hafslund/cc2531-sniffer, hanya/aobook-haiku
