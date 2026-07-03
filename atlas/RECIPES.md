# Atlas Recipes — working / non-working roster

Total **986** recipes. **Working (built end-to-end): 2** · non-working: 0 partial · 250 not-standalone · 368 failed.

"Working" means the full pipeline (autoreconf → configure → make) succeeded under the GNU-free toolchain. `quirk_dependent` needed an auto-applied quirk; `sealed` needed none.

## ✅ Working (2)

| repo | court |
| --- | --- |
| cosmos72/twin | quirk_dependent |
| flyinghead/ircd-hybrid | quirk_dependent |

## 🟡 Non-working — partial (configure cleared, make failed) (0)

_None._

## ❌ Non-working — failed (ours fails before make) (368)

| repo | stage | first error |
| --- | --- | --- |
| 6WIND/quagga | CONFIGURE_RUN_FAIL | checking whether we are using SunPro compiler... ./configure: line 1488: syntax error near |
| A-Kyle/GrADS-CJK | CONFIGURE_RUN_FAIL | ./configure: line 1612: syntax error near unexpected token `seems' |
| a-sassmannshausen/guile-monads | CONFIGURE_RUN_FAIL | ./configure: line 1352: syntax error near unexpected token `2.0.11' |
| abc100m/libzdbcpp | CONFIGURE_RUN_FAIL |  |
| accellera-official/systemc | CONFIGURE_RUN_FAIL | whether we are using a Clang/LLVM C++ compiler... ./configure: line 1378: syntax error nea |
| adapteva/epiphany-libs | CONFIGURE_RUN_FAIL |  |
| ademakov/Oroch | CONFIGURE_RUN_FAIL | ./configure: line 1412: syntax error near unexpected token `done' |
| adiknoth/netatalk-debian | CONFIGURE_RUN_FAIL | ./configure: line 1376: AC_PROG_PERL: command not found |
| AdolfVonKleist/Phonetisaurus | CONFIGURE_RUN_FAIL | configure: error: Can't find OpenFST or one or more of its extensions. Use --with-openfst- |
| ADVANTECH-Corp/WiseSnail | CONFIGURE_RUN_FAIL | ./configure: line 1631: ./library/WiseCore/WiseCore_MQTT/configure: No such file or direct |
| affix/MSNC | CONFIGURE_RUN_FAIL | configure: error: You don't seem to have the curses headers installed |
| afrab/WSim | CONFIGURE_RUN_FAIL | ./configure: line 2183: syntax error near unexpected token `)' |
| agn453/ZXCC | CONFIGURE_RUN_FAIL | ./configure: line 1422: syntax error near unexpected token `because' |
| ahjragaas/inetutils | CONFIGURE_RUN_FAIL | ./configure: line 1424: syntax error near unexpected token `ftpd' |
| ahlstromcj/midicvt | CONFIGURE_RUN_FAIL | ./configure: line 1765: syntax error near unexpected token `esac' |
| ahmedammar/platform_external_gst_gstreamer | CONFIGURE_RUN_FAIL | ./configure: line 1339: AG_GST_INIT: command not found |
| ahorn/cpp-channel | CONFIGURE_RUN_FAIL | ./configure: line 1355: ./gtest/configure: No such file or directory |
| ahupowerdns/setgrouper | CONFIGURE_RUN_FAIL | configure: error: no C++ compiler found |
| ajnelson/photorec-testdisk | CONFIGURE_RUN_FAIL | ./configure: line 1447: pthread-config: command not found |
| alanjaouen/compilateur-cpp | CONFIGURE_RUN_FAIL | awk: cmd. line:1:                               ^ syntax error |
| albertok/web-polygraph | CONFIGURE_RUN_FAIL | configure: error: the compiler (g++) failed to pass a simple C++ test; check config.log fo |
| Albinlk/OpenThread | CONFIGURE_RUN_FAIL |  |
| alexmarsev/soundtouch | CONFIGURE_RUN_FAIL | ./configure: line 1655: syntax error near unexpected token `done' |
| alk/malloc-trace-replay | CONFIGURE_RUN_FAIL |  |
| allinurl/goaccess | CONFIGURE_RUN_FAIL | ./configure: line 1361: syntax error near unexpected token `to' |
| AlmuHS/GNUMach_SMP | CONFIGURE_RUN_FAIL | ./configure: line 1349: $'\f': command not found |
| alsa-project/alsa-firmware | CONFIGURE_RUN_FAIL | ./configure: line 1380: _AC_COMPILER_EXEEXT: command not found |
| AmkG/hl | CONFIGURE_RUN_FAIL | checking how to get an intptr_t type with ranges... configure: error: We can't find out wh |
| andrewshadura/tnat64 | CONFIGURE_RUN_FAIL | configure: error: 'Could not find library containing connect()' |
| AndyA/htop-osx | CONFIGURE_RUN_FAIL | ./configure: line 1630: syntax error near unexpected token `plpa-1.1' |
| anewhuahua/bilitw | CONFIGURE_RUN_FAIL | ./configure: line 1886: ./contrib/yaml-0.1.4/configure: No such file or directory |
| antiprism/mpd_oled | CONFIGURE_RUN_FAIL |  |
| AOMediaCodec/oac | CONFIGURE_RUN_FAIL | ./configure: line 1422: because: command not found |
| apache/xerces-c | CONFIGURE_RUN_FAIL | ./configure: line 1691: pthread-config: command not found |
| apereo/mod_auth_cas | CONFIGURE_RUN_FAIL | ./configure: line 1439: ax_cv_check__AC_LANG_ABBREVflags__-Wall=yes: command not found |
| apertium/apertium | CONFIGURE_RUN_FAIL | configure: error: You don't have xmllint installed. |
| aportelli/LatAnalyze | CONFIGURE_RUN_FAIL | ./configure: line 1425: AX_COMPILER_VENDOR: command not found |
| arbor/gzsig | CONFIGURE_RUN_FAIL | configure: error: OpenSSL not found |
| arbruijn/d2x-xl | CONFIGURE_RUN_FAIL | ./configure: line 1454: AC_STDC_HEADERS: command not found |
| archiecobbs/libnbcompat | CONFIGURE_RUN_FAIL | ./configure: line 1458: syntax error near unexpected token `;' |
| aristanetworks/EosSdk | CONFIGURE_RUN_FAIL | for std::unordered_set::operator==... configure: error: Your version of the STL seems to b |
| armadito/armadito-av | CONFIGURE_RUN_FAIL | ./configure: line 1371: syntax error near unexpected token `(' |
| artclarke/xuggle-xuggler | CONFIGURE_RUN_FAIL | ./configure: line 1796: ON-FAIL: command not found |
| arthurdejong/nss-pam-ldapd | CONFIGURE_RUN_FAIL | ./configure: line 1870: ac_cv_have_decl_setusershell,: command not found |
| asnelt/rrep | CONFIGURE_RUN_FAIL | configure: error: Invalid value for --with-included-regex: |
| assaferan/omf5 | CONFIGURE_RUN_FAIL | ./configure: line 1377: is: command not found |
| astromatic/psfex | CONFIGURE_RUN_FAIL | checking if compilation flags are set automatically... checking whether the classic INTEL  |
| astromatic/sextractor | CONFIGURE_RUN_FAIL | checking if compilation flags are set automatically... checking whether the classic INTEL  |
| astromatic/skymaker | CONFIGURE_RUN_FAIL | checking if compilation flags are set automatically... checking whether the classic INTEL  |
| autotools-mirror/autoconf | CONFIGURE_RUN_FAIL | ./configure: line 2679: syntax error: unexpected end of file |
| avr-aics-riken/234Compositor | CONFIGURE_RUN_FAIL | ./configure: line 2244: syntax error near unexpected token `(' |
| avrdudes/avarice | CONFIGURE_RUN_FAIL | ./configure: line 1402: ACTION-IF-,: command not found |
| awaw/dnsproxy | CONFIGURE_RUN_FAIL | checking for libevent... configure: error: |
| awgn/brute | CONFIGURE_RUN_FAIL | configure: error: x86_64-unknown-linux-gnu not supported |
| badoo/libpssh | CONFIGURE_RUN_FAIL | checking libevent install prefix... configure: error: Can't find libevent headers under  d |
| balde/balde | CONFIGURE_RUN_FAIL | configure: error: no -fvisibility=hidden support found in , balde requires -fvisibility=hi |
| bambulab/gmp | CONFIGURE_RUN_FAIL | ./configure: line 1367: syntax error near unexpected token `config.m4' |
| barak/oaklisp | CONFIGURE_RUN_FAIL | _AC_LANG_PREFIXFLAGS for maximum warnings... ./configure: line 1544: syntax error near une |
| baszoetekouw/pinfo | CONFIGURE_RUN_FAIL | ./configure: line 1422: syntax error near unexpected token `else' |
| bbc/vc2hqdecode | CONFIGURE_RUN_FAIL |  |
| bcoin-org/libtorsion | CONFIGURE_RUN_FAIL | configure: error: language C required |
| bdwgc/bdwgc | CONFIGURE_RUN_FAIL | ./configure: line 1566: syntax error near unexpected token `(' |
| benmwebb/dopewars | CONFIGURE_RUN_FAIL |  |
| benvanik/gflags | CONFIGURE_RUN_FAIL | ./configure: line 1715: pthread-config: command not found |
| benwbooth/tvision | CONFIGURE_RUN_FAIL | ./configure: line 1412: AC_STDC_HEADERS: command not found |
| BGI-shenzhen/LDBlockShow | CONFIGURE_RUN_FAIL | configure: error: You need zlib >= 1.2.3 to build |
| BGI-shenzhen/PopLDdecay | CONFIGURE_RUN_FAIL | configure: error: You need zlib >= 1.2.3 to build |
| binhqnguyen/ovs-srv6 | CONFIGURE_RUN_FAIL | configure: error: Cannot find openssl (use --disable-ssl to configure without SSL support) |
| BirolLab/ChopStitch | CONFIGURE_RUN_FAIL | configure: error: CHOPSTITCH must be compiled with a C++ compiler that supports OpenMP thr |
| BirolLab/ntCard | CONFIGURE_RUN_FAIL | configure: error: NTCARD must be compiled with a C++ compiler that supports OpenMP threadi |
| bitcoin-core/minisketch | CONFIGURE_RUN_FAIL | checking which field sizes to build... ./configure: line 1398: syntax error near unexpecte |
| BitzenyCoreDevelopers/cpuminer | CONFIGURE_RUN_FAIL | ./configure: line 1404: ac_cv_have_decl_be32dec,: command not found |
| bkearney/augeas | CONFIGURE_RUN_FAIL | ./configure: line 1423: syntax error near unexpected token `maximum' |
| BlockstreamResearch/secp256k1-zkp | CONFIGURE_RUN_FAIL | configure: error: Set enable_dev_mode before calling SECP_SET_DEFAULT |
| bloomen/libunittest | CONFIGURE_RUN_FAIL | ./configure: line 1515: pthread-config: command not found |
| bloq/cpptrade | CONFIGURE_RUN_FAIL | ./configure: line 1384: syntax error near unexpected token `done' |
| blueness/sthttpd | CONFIGURE_RUN_FAIL | ./configure: line 1363: syntax error near unexpected token `AR,' |
| BoldingBruggeman/netcdf3 | CONFIGURE_RUN_FAIL | ./configure: line 1619: syntax error near unexpected token `fi' |
| bonzini/smalltalk | CONFIGURE_RUN_FAIL |  |
| borkmann/lksctp-tools | CONFIGURE_RUN_FAIL |  |
| boundary/libdnet | CONFIGURE_RUN_FAIL | ./configure: line 1434: syntax error near unexpected token `after' |
| boundary/wireshark | CONFIGURE_RUN_FAIL | ./configure: line 2687: syntax error near unexpected token `(' |
| boxbackup/boxi | CONFIGURE_RUN_FAIL | ./configure: line 1389: AM_PROG_CC_STDC: command not found |
| boysetsfrog/vimpc | CONFIGURE_RUN_FAIL | ./configure: line 3511: syntax error: unexpected end of file |
| BrianGladman/mpfr | CONFIGURE_RUN_FAIL | ./configure: line 1374: is: command not found |
| brianmcgillion/udev | CONFIGURE_RUN_FAIL | configure: error: POSIX RT library not found |
| broadinstitute/VariantBam | CONFIGURE_RUN_FAIL | ./configure: line 1407: action: command not found |
| brunonymous/Powermanga | CONFIGURE_RUN_FAIL | ./configure: line 1414: AM_PATH_SDL: command not found |
| brynet/file | CONFIGURE_RUN_FAIL | ./configure: line 1513: syntax error near unexpected token `;' |
| bubbapizza/GCAM | CONFIGURE_RUN_FAIL | ./configure: line 1412: syntax error near unexpected token `)' |
| BunsenLabs/plank | CONFIGURE_RUN_FAIL | ./configure: line 1462: GLIB_GSETTINGS: command not found |
| bvanheu/libsigrok-ad | CONFIGURE_RUN_FAIL | ./configure: line 1338: syntax error near unexpected token `)' |
| bytedance/ovs-dpdk | CONFIGURE_RUN_FAIL | configure: error: Cannot find openssl (use --disable-ssl to configure without SSL support) |
| c-rack/squid-ecap-gzip | CONFIGURE_RUN_FAIL | configure: error: libecap is not found or unusable |
| calaos/calaos_base | CONFIGURE_RUN_FAIL | ./configure: line 1579: syntax error near unexpected token `CALAOS_COMMON,' |
| cannabisday/ovs-tsn | CONFIGURE_RUN_FAIL | configure: error: Cannot find openssl (use --disable-ssl to configure without SSL support) |
| canonical/dqlite | CONFIGURE_RUN_FAIL | ./configure: line 1397: syntax error near unexpected token `else' |
| cciechad/brlcad | CONFIGURE_RUN_FAIL | ./configure: line 1435: syntax error near unexpected token `fi' |
| cculianu/secp256k1 | CONFIGURE_RUN_FAIL | configure: error: invalid assembly optimization selection |
| cea-hpc/robinhood | CONFIGURE_RUN_FAIL | ./configure: line 3004: syntax error near unexpected token `)' |
| cederom/LibSWD | CONFIGURE_RUN_FAIL | ./configure: line 1583: syntax error near unexpected token `ON' |
| Certseeds/graphicsmagick | CONFIGURE_RUN_FAIL |  |
| cforall/cforall | CONFIGURE_RUN_FAIL |  |
| chaoran/fibril | CONFIGURE_RUN_FAIL | ./configure: line 1506: pthread-config: command not found |
| chaos/cerebro | CONFIGURE_RUN_FAIL | ./configure: line 1400: AC_LIB_LTDL: command not found |
| chaos/pdsh | CONFIGURE_RUN_FAIL | ./configure: line 585: syntax error near unexpected token `(' |
| chaos/powerman | CONFIGURE_RUN_FAIL | ./configure: line 585: syntax error near unexpected token `(' |
| chaos/slurm | CONFIGURE_RUN_FAIL | ./configure: line 1348: X_AC_GPL_LICENSED: command not found |
| chenall/grub4dos | CONFIGURE_RUN_FAIL | configure: error: unsupported CPU type |
| chiphackers/covered | CONFIGURE_RUN_FAIL | ./configure: line 1485: COVERED_TCLTK: command not found |
| chuckleb/virt-what | CONFIGURE_RUN_FAIL |  |
| cisco/libamvp | CONFIGURE_RUN_FAIL | ./configure: line 1453: syntax error near unexpected token `fi' |
| cisco/opus | CONFIGURE_RUN_FAIL | ./configure: line 1397: AC_MINGW32: command not found |
| cjcole/libgolle | CONFIGURE_RUN_FAIL | ./configure: line 1504: DX_PDF_FEATURE: command not found |
| CkNoSFeRaTU/pidgin | CONFIGURE_RUN_FAIL | ./configure: line 1514: LT_LIB_M: command not found |
| clsync/clsync | CONFIGURE_RUN_FAIL | whether _AC_LANG compiler accepts -fstack-check... ./configure: line 1640: ax_cv_check__AC |
| cluslab/metastack | CONFIGURE_RUN_FAIL | ./configure: line 585: syntax error near unexpected token `(' |
| ClusterLabs/cluster-glue | CONFIGURE_RUN_FAIL | ./configure: line 1932: syntax error near unexpected token `"%d.%d"' |
| ClusterLabs/libqb | CONFIGURE_RUN_FAIL | ./configure: 4: Syntax error: "\|" unexpected |
| cmand/yarrp | CONFIGURE_RUN_FAIL | configure: error: either specify a valid zlib installation with --with-zlib=DIR or disable |
| cmauri/eviacam | CONFIGURE_RUN_FAIL | ./configure: line 1572: syntax error near unexpected token `func' |
| cminyard/gensio | CONFIGURE_RUN_FAIL | checking for Linux epoll(7) interface with signals extension... ./configure: line 1855: sy |
| cnDelbert/libtiff | CONFIGURE_RUN_FAIL | configure: error: Unsupported size_t size ; please add support |
| CNGLDLab/LORG-Release | CONFIGURE_RUN_FAIL | configure: error: cannot find the Intel TBB library consider to give --with-tbb to link it |
| coapp-packages/libunistring | CONFIGURE_RUN_FAIL | ./configure: line 1346: is: command not found |
| cockpit-project/cockpit | CONFIGURE_RUN_FAIL | configure: error: Couldn't find crypt library. Try installing glibc-devel |
| code-saturne/code_saturne | CONFIGURE_RUN_FAIL | configure: error: directory specified by --with-salome= does not exist! |
| codecryptanalysis/mccl | CONFIGURE_RUN_FAIL | ./configure: line 1345: This: command not found |
| coin-or-tools/ThirdParty-ASL | CONFIGURE_RUN_FAIL | ./configure: line 1341: Copyright: command not found |
| coin-or-tools/ThirdParty-HSL | CONFIGURE_RUN_FAIL | ./configure: line 1341: Copyright: command not found |
| coin-or/Cbc | CONFIGURE_RUN_FAIL | ./configure: line 1341: Copyright: command not found |
| coin-or/OS | CONFIGURE_RUN_FAIL | ./configure: line 1341: Copyright: command not found |
| coin-or/Rehearse | CONFIGURE_RUN_FAIL | ./configure: line 1341: Copyright: command not found |
| coin3d/quarter | CONFIGURE_RUN_FAIL | ./configure: line 1355: SIM_AC_SETUP_MSVCPP_IFELSE: command not found |
| ColumPaget/gngeo-cjp | CONFIGURE_RUN_FAIL | configure: error: The specified m68k core is invalid |
| commandus/proto-sql | CONFIGURE_RUN_FAIL | configure: error: Could not find libprotobuf3. Try $ ./configure LDFLAGS='-Lyour-protobuf3 |
| cooljeanius/docbook-utils-0.6.14 | CONFIGURE_RUN_FAIL | ./configure: line 2108: syntax error near unexpected token `(' |
| cooljeanius/gcml2-0.7.1 | CONFIGURE_RUN_FAIL | checking for IMLIB - version >= 1.8.2... ./configure: line 1641: --cflags: command not fou |
| cooljeanius/libUnixToOSX | CONFIGURE_RUN_FAIL | Try 0 --help for more information.: syntax error in expression (error token is "Try 0 --he |
| cooljeanius/mdnsd | CONFIGURE_RUN_FAIL | ./configure: line 1372: syntax error near unexpected token `AM_SET_LEADING_DOT' |
| cooljeanius/pkg-config | CONFIGURE_RUN_FAIL | ./configure: line 1338: and: command not found |
| coova/coova-chilli | CONFIGURE_RUN_FAIL | ./configure: line 1432: AC_LBL_TPACKET_STATS: command not found |
| corazawaf/libcoraza | CONFIGURE_RUN_FAIL | configure: error: Go |
| couchbaselabs/breakpad | CONFIGURE_RUN_FAIL | ./configure: line 1468: ACTION-IF-,: command not found |
| cowsql/cowsql | CONFIGURE_RUN_FAIL | ./configure: line 1384: syntax error near unexpected token `else' |
| cowsql/raft | CONFIGURE_RUN_FAIL |  |
| cpputest/cpputest | CONFIGURE_RUN_FAIL |  |
| cprados/towitoko-linux | CONFIGURE_RUN_FAIL | ./configure: line 1394: syntax error near unexpected token `fi' |
| cpu-pool/cpuminer-opt-cpupower | CONFIGURE_RUN_FAIL | ./configure: line 1414: ac_cv_have_decl_be32dec,: command not found |
| cr-marcstevens/m4gb | CONFIGURE_RUN_FAIL | ./configure: line 1345: This: command not found |
| crdroidandroid/android_hardware_qcom_display | CONFIGURE_RUN_FAIL | ./configure: line 2102: syntax error near unexpected token `(' |
| cryptozeny/cpuminer-opt-sugarchain | CONFIGURE_RUN_FAIL | ./configure: line 1414: ac_cv_have_decl_be32dec,: command not found |
| crystalsnetworkdev/pq-crystals | CONFIGURE_RUN_FAIL | ./configure: line 1554: syntax error near unexpected token `ON' |
| CS198NDSGChanBrianJoe/html5rdp | CONFIGURE_RUN_FAIL | ./configure: line 1814: syntax error near unexpected token `;' |
| cschwan/hep-ga | CONFIGURE_RUN_FAIL |  |
| cslarsen/mickey-scheme | CONFIGURE_RUN_FAIL | configure: error: readline test failed (--without-readline to disable) |
| cstrope/indel-seq-gen | CONFIGURE_RUN_FAIL | ./configure: line 1465: syntax error near unexpected token `unistd.h' |
| cwi-dis/ambulant | CONFIGURE_RUN_FAIL | configure: error: Your platform is not currently supported |
| cybergarage/usql | CONFIGURE_RUN_FAIL | configure: error: uSQL for C++ needs ANTLR3C >= 3.2 |
| d99kris/namp-lite | CONFIGURE_RUN_FAIL | configure: error: Required library ncursesw not found. |
| dajobe/librdf | CONFIGURE_RUN_FAIL | expr: syntax error: unexpected argument '10000' |
| Dale-M/mcron | CONFIGURE_RUN_FAIL | ./configure: line 1399: syntax error near unexpected token `3.0' |
| dankamongmen/babl | CONFIGURE_GEN_FAIL |  |
| Dansguardian/dansguardian | CONFIGURE_RUN_FAIL |  |
| darkbitsorg/guichan | CONFIGURE_RUN_FAIL | ./configure: line 1484: syntax error near unexpected token `}' |
| darrenjs/log2mem | CONFIGURE_RUN_FAIL |  |
| datacratic/gperftools | CONFIGURE_RUN_FAIL | configure: error: cannot find the nanosleep function |
| daveshields/jikes | CONFIGURE_RUN_FAIL | ./configure: line 1409: syntax error near unexpected token `macro' |
| davexunit/guile-2d | CONFIGURE_RUN_FAIL | ./configure: line 1348: GUILE_PROGS: command not found |
| dbadapt/mutrace | CONFIGURE_RUN_FAIL | ./configure: line 1468: syntax error near unexpected token `else' |
| delphix/nfs-utils | CONFIGURE_RUN_FAIL | ./configure: line 1506: syntax error near unexpected token `else' |
| demorest/mark5access | CONFIGURE_RUN_FAIL | ./configure: line 2162: syntax error near unexpected token `(' |
| deskull-m/bakabakaband | CONFIGURE_RUN_FAIL | ./configure: line 1356: syntax error near unexpected token `game_libpath,' |
| desrt/systemd-shim | CONFIGURE_RUN_FAIL | ./configure: line 1356: syntax error near unexpected token `does' |
| detomon/BlipKit | CONFIGURE_RUN_FAIL | checking whether C compiler supports ... configure: error: C compiler seem not to support |
| detomon/json5 | CONFIGURE_RUN_FAIL | ./configure: line 1538: syntax error near unexpected token `unicode-table' |
| devicescape/aws_dynamo | CONFIGURE_RUN_FAIL | configure: error: no openssl; please install openssl or equivalent |
| devzero2000/POPT | CONFIGURE_RUN_FAIL | ./configure: line 1457: syntax error near unexpected token `-Wall,' |
| dex4er/fakechroot | CONFIGURE_RUN_FAIL | configure: error: invalid libpath specified |
| dex4er/nss-docker | CONFIGURE_RUN_FAIL | checking Docker socket path... ./configure: line 1780: syntax error near unexpected token  |
| dfrc-korea/carpe-sleuthkit | CLONE_FAIL |  |
| digitalocean/ovs | CLONE_FAIL |  |
| diixo/dbus | CLONE_FAIL |  |
| Distrotech/celt | CONFIGURE_RUN_FAIL | ./configure: line 1453: syntax error near unexpected token `tools="tools",' |
| Distrotech/evolution | CONFIGURE_RUN_FAIL | ./configure: line 5753: intltool-update: command not found |
| Distrotech/gtkimageview | CONFIGURE_RUN_FAIL | ./configure: line 1370: GNOME_COMMON_INIT: command not found |
| Distrotech/libcaca | CONFIGURE_RUN_FAIL | ./configure: line 1378: AC_LIBTOOL_CXX: command not found |
| Distrotech/libcddb | CONFIGURE_RUN_FAIL | ./configure: line 1374: syntax error near unexpected token `fi' |
| Distrotech/libdvdcss | CONFIGURE_RUN_FAIL | ./configure: line 1430: syntax error near unexpected token `do' |
| Distrotech/libmad | CONFIGURE_RUN_FAIL | ./configure: line 2451: syntax error near unexpected token `(' |
| Distrotech/libspectre | CONFIGURE_RUN_FAIL | ./configure: line 1400: AC_STDC_HEADERS: command not found |
| Distrotech/libwnck | CONFIGURE_RUN_FAIL | ./configure: line 4081: intltool-update: command not found |
| Distrotech/pulseaudio | CONFIGURE_RUN_FAIL | configure: error: git-version-gen failed |
| Distrotech/squid | CONFIGURE_RUN_FAIL | ./configure: line 1409: again,: command not found |
| Distrotech/Thunar | CONFIGURE_RUN_FAIL | ./configure: line 6057: intltool-update: command not found |
| djn3m0/debit | CONFIGURE_RUN_FAIL | ./configure: line 1378: AX_CHECK_ALIGNED_ACCESS_REQUIRED: command not found |
| dleonard0/pktstat | CONFIGURE_RUN_FAIL | checking for library containing socket... ./configure: line 1394: syntax error near unexpe |
| dmalhotra/pvfmm | CONFIGURE_RUN_FAIL | ./configure: line 1506: syntax error near unexpected token `and' |
| dreal-deps/gsl | CONFIGURE_RUN_FAIL | ./configure: line 1464: LT_LIB_M: command not found |
| dreal-deps/libunwind | CONFIGURE_RUN_FAIL | ./configure: line 1426: CHECK_ATOMIC_OPS: command not found |
| dreamcat4/php-fpm | CONFIGURE_RUN_FAIL | ./configure: line 1417: AC_FPM_CC: command not found |
| dreamlayers/synaesthesia | CONFIGURE_RUN_FAIL | ./configure: line 1383: fails: command not found |
| DrMcCoy/NWNTools | CONFIGURE_RUN_FAIL |  |
| drothlis/gstreamer | CONFIGURE_RUN_FAIL | ./configure: line 1339: AG_GST_INIT: command not found |
| duality-solutions/Dynamic-GPU-Miner-Nvidia | CONFIGURE_RUN_FAIL | ./configure: line 1412: ac_cv_have_decl_be32dec,: command not found |
| duosecurity/duo_unix | CONFIGURE_RUN_FAIL |  |
| dvab-sarma/android_external_alsa-lib | CONFIGURE_RUN_FAIL | ./configure: line 2433: syntax error near unexpected token `[_$as_cr_alnum]*_cv_[_$as_cr_a |
| dylex/xtmux | CONFIGURE_RUN_FAIL | ./configure: line 1720: syntax error near unexpected token `$'doesn\'t give us any other w |
| e-desouza/gzip-1.11 | CONFIGURE_RUN_FAIL | configure: error: <wchar.h> cannot be used with this compiler (cc  ). |
| e2guardian/e2guardian | CONFIGURE_RUN_FAIL |  |
| ebosnjak/libpng-1.5.4-vuln | CONFIGURE_RUN_FAIL | ./configure: line 1370: AC_PROG_LD: command not found |
| ecairn/sphinx-official | CONFIGURE_RUN_FAIL | ./configure: line 1341: syntax error near unexpected token `checking' |
| ecerulm/autotools-template | CONFIGURE_RUN_FAIL | ./configure: line 1715: syntax error near unexpected token `)' |
| echiu64/gutenprint | CONFIGURE_RUN_FAIL | ./configure: line 1348: STP_INIT: command not found |
| eddelbuettel/dieharder | CONFIGURE_RUN_FAIL | configure: error: Couldn't find libgsl. Please install the gsl package. |
| ederc/gb | CONFIGURE_RUN_FAIL | for OpenMP flag of _AC_LANG compiler... ./configure: line 1707: AX_LANG_COMPILER_MS: comma |
| ederc/gbla | CONFIGURE_RUN_FAIL | ./configure: line 1581: AX_LANG_COMPILER_MS: command not found |
| edf-hpc/pkg-nsca-ng | CONFIGURE_RUN_FAIL | ./configure: line 1514: syntax error near unexpected token `fi' |
| edrosten/tag | CONFIGURE_RUN_FAIL | configure: error: TooN is not optional. Use --with-TooN=dir to specify where it can be fou |
| efficient/memc3 | CONFIGURE_RUN_FAIL | ./configure: line 1548: syntax error near unexpected token `GCC' |
| egallesio/STklos | CONFIGURE_RUN_FAIL | configure: error: Unknown thread system: |
| eiichiroi/autotools-unittest | CONFIGURE_RUN_FAIL | ./configure: line 1501: pthread-config: command not found |
| eklitzke/spv | CONFIGURE_RUN_FAIL | checking to see if compiler understands -std=c++17... ./configure: line 1631: syntax error |
| ekmett/jitplusplus | CONFIGURE_RUN_FAIL | ./configure: line 1878: pthread-config: command not found |
| elbandi/lighttpd | CONFIGURE_RUN_FAIL | ./configure: line 1437: AM_C_PROTOTYPES: command not found |
| electimon/bmp | CONFIGURE_RUN_FAIL |  |
| Elzair/nazghul | CONFIGURE_RUN_FAIL |  |
| emcrisostomo/fswatch | CONFIGURE_GEN_FAIL |  |
| emuse/qmidiarp | CONFIGURE_RUN_FAIL | ./configure: line 1350: syntax error near unexpected token `ON' |
| enba94yf/binutils-2.42 | CONFIGURE_RUN_FAIL | ./configure: line 1367: ACX_LARGEFILE: command not found |
| endlessm/eos-shard | CONFIGURE_RUN_FAIL | ./configure: line 1356: syntax error near unexpected token `(' |
| endlessm/flatpak-builder | CONFIGURE_RUN_FAIL | ./configure: line 1364: LIBGLNX_CONFIGURE: command not found |
| endocode/connman | CONFIGURE_RUN_FAIL | ./configure: line 1354: COMPILER_FLAGS: command not found |
| Enigma-Game/Enigma | CONFIGURE_RUN_FAIL |  |
| envytools/valgrind | CONFIGURE_RUN_FAIL | ./configure: line 1400: syntax error near unexpected token `(' |
| ep-infosec/33_apache_duo_unix | CONFIGURE_RUN_FAIL |  |
| epfl-dias/shore-mt | CONFIGURE_RUN_FAIL | ./configure: line 1374: AC_REQUIRE_CPP: command not found |
| erikarn/libevent-adrian | CONFIGURE_RUN_FAIL | checking for socklen_t... checking whether our compiler supports __func__... ./configure:  |
| erthink/ReOpenLDAP | CONFIGURE_RUN_FAIL | checking configure arguments... ./configure: line 1516: syntax error near unexpected token |
| esrf-bliss/CCfits | CONFIGURE_RUN_FAIL | ./configure: line 1367: PFK_CXX_LIB_PATH: command not found |
| essej/sooperlooper | CONFIGURE_RUN_FAIL | ./configure: line 1479: AM_OPT_FLAGS: command not found |
| ester-project/ester | CONFIGURE_RUN_FAIL | ./configure: line 1524: --ldflags: command not found |
| esy-packages/esy-automake | CONFIGURE_RUN_FAIL | ./configure: line 1338: and: command not found |
| evilnet/x3 | CONFIGURE_RUN_FAIL | for va_copy... for __va_copy... checking which malloc to use... configure: error: Unknown  |
| evilpan/TurnServer | CONFIGURE_RUN_FAIL | ./configure: line 1626: syntax error near unexpected token `ON' |
| EvolBioInf/andi | CONFIGURE_RUN_FAIL | ./configure: line 1627: syntax error near unexpected token `and' |
| ewxrjk/with-readline | CONFIGURE_RUN_FAIL | ./configure: line 1407: AC_SET_MAKE: command not found |
| FabricAttachedMemory/libfam-atomic | CONFIGURE_RUN_FAIL | ./configure: line 1457: syntax error near unexpected token `fi' |
| facchinm/avrdude | CONFIGURE_RUN_FAIL | ./configure: line 1439: syntax error near unexpected token `AR,' |
| fairflow/espeak-ng-pt-br | CONFIGURE_RUN_FAIL | ./configure: line 1549: syntax error near unexpected token `for' |
| fancybits/libdvbpsi | CONFIGURE_RUN_FAIL | ./configure: line 1480: syntax error near unexpected token `done' |
| farsightsec/mtbl | CONFIGURE_RUN_FAIL | configure: error: liblz4 >= r130 required |
| fasseg/crumbs | CONFIGURE_RUN_FAIL | configure: error: Your platform is not currently supported |
| FauxFaux/fastjar | CONFIGURE_RUN_FAIL | Try `./configure --help for more information.: syntax error: invalid arithmetic operator ( |
| felix-001/tstool | CONFIGURE_RUN_FAIL | ./configure: line 1399: AC_STDC_HEADERS: command not found |
| fengy-research/UCNTracker | CONFIGURE_RUN_FAIL | checking valac is at least version 0.7.6... ./configure: line 1545: : command not found |
| fengye/swfdec | CONFIGURE_RUN_FAIL | configure: error: liboil-0.3 >= 0.3.1 is required to build swfdec |
| ferrandi/PandA-bambu | CONFIGURE_RUN_FAIL | ./configure: line 1359: syntax error near unexpected token `and' |
| filebench/filebench | CONFIGURE_RUN_FAIL | checking for off64_t... checking for boolean_t... checking for u_longlong_t... checking fo |
| FinchBerryOS/fbyo-coreutils | CONFIGURE_RUN_FAIL | configure: error: libsmack library was not found or not usable |
| firecore/libudfread | CONFIGURE_RUN_FAIL |  |
| firoorg/cpuminer | CONFIGURE_RUN_FAIL | ./configure: line 1414: ac_cv_have_decl_be32dec,: command not found |
| fizx/parsley | CONFIGURE_RUN_FAIL | configure: error: could not find pcre |
| flame/tblis-strassen | CONFIGURE_RUN_FAIL | ./configure: line 1673: syntax error near unexpected token `done' |
| flboudet/flobz | CONFIGURE_RUN_FAIL | checking the location of hash_map... ./configure: line 1936: syntax error near unexpected  |
| flowgrind/flowgrind | CONFIGURE_RUN_FAIL | ./configure: line 1862: AC_TYPE_UNSIGNED_LONG_LONG_INT: command not found |
| flux-framework/flux-foundry | CONFIGURE_GEN_FAIL |  |
| fmrico/libpng16 | CONFIGURE_RUN_FAIL |  |
| FNCS/fncs | CONFIGURE_RUN_FAIL | ./configure: line 1432: syntax error near unexpected token `ZMQ_LIBS,' |
| fontforge/libspiro | CONFIGURE_RUN_FAIL | configure: error: ERROR: Please install Math libraries and math.h include files for libm |
| fontforge/libuninameslist | CONFIGURE_RUN_FAIL | ./configure: line 1523: syntax error near unexpected token `else' |
| fork4jl/mpfr | CONFIGURE_RUN_FAIL | ./configure: line 1374: is: command not found |
| formorer/pkg-keepalived | CONFIGURE_RUN_FAIL | ./configure: line 1619: syntax error near unexpected token `causes' |
| fossci/libgcrypt | CONFIGURE_RUN_FAIL | configure: error: |
| fourmond/dvdcopy | CONFIGURE_RUN_FAIL | configure: error: cannot link to dvdread |
| FredericJacobs/obfsproxy-c | CONFIGURE_RUN_FAIL | ./configure: line 1502: syntax error near unexpected token `--' |
| freebsd/atf | CONFIGURE_RUN_FAIL | ./configure: line 1389: syntax error near unexpected token `C++' |
| freedesktop-unofficial-mirror/dbus__dbus-qt3 | CONFIGURE_RUN_FAIL | ./configure: line 1790: syntax error near unexpected token `fi' |
| freedesktop-unofficial-mirror/gstreamer-sdk__dbus | CONFIGURE_RUN_FAIL | ./configure: line 1742: syntax error near unexpected token `fi' |
| FreeMCU/freemcu | CONFIGURE_RUN_FAIL | ./configure: line 1632: ./libs/ptlib/configure: No such file or directory |
| FreeRADIUS/freeradius-client | CONFIGURE_RUN_FAIL | checking gethostbyaddr_r() syntax... ./configure: line 1434: syntax error near unexpected  |
| frida/xz | CONFIGURE_RUN_FAIL | configure: error: --enable-assembler accepts only no', x86_64'. |
| fripon/freeture | CONFIGURE_RUN_FAIL | ./configure: line 1376: syntax error near unexpected token `)' |
| frugalware/pacman-g2 | CONFIGURE_RUN_FAIL | configure: error: Your architecture is not supported |
| fuxedo/fuxedo | CONFIGURE_RUN_FAIL |  |
| GabrielDosReis/open-axiom | CONFIGURE_RUN_FAIL | configure: error: OpenAxiom requires a Lisp system.  Either separately build one (GCL-2.6. |
| gajgeospatial/libpng-1.6.40 | CONFIGURE_RUN_FAIL |  |
| GalliumOS/xfce4-session | CONFIGURE_RUN_FAIL | ./configure: line 1474: AC_CHECK_LIBM: command not found |
| gat3way/hashkill | CONFIGURE_RUN_FAIL | checking whether _AC_LANG compiler accepts ... ./configure: line 1637: syntax error near u |
| gavioto/fastdb | CONFIGURE_RUN_FAIL | checking whether compiling with debug options enabled... configure: error: invalid argumen |
| gbonacini/trollfs | CONFIGURE_RUN_FAIL | configure: error: could not find lib FUSE |
| gcp/opusfile | CONFIGURE_RUN_FAIL |  |
| gderosa/dansguardian | CONFIGURE_RUN_FAIL |  |
| gdnsd/gdnsd | CONFIGURE_RUN_FAIL | ./configure: line 1542: syntax error near unexpected token `else' |
| GeGuNa/trafficserver | CONFIGURE_RUN_FAIL |  |
| GenABEL-Project/ProbABEL | CONFIGURE_RUN_FAIL | ./configure: line 1411: syntax error near unexpected token `will' |
| genesi/gnome-terminal | CONFIGURE_RUN_FAIL | ./configure: line 1392: GNOME_COMMON_INIT: command not found |
| genome-vendor/gmap-gsnap | CONFIGURE_RUN_FAIL | ./configure: line 1449: ACX_PATH_PERL: command not found |
| gentoo/portage-utils | CONFIGURE_RUN_FAIL | configure: error: |
| gentoo/sandbox | CONFIGURE_RUN_FAIL | ./configure: line 1601: syntax error near unexpected token `fi' |
| geoffjay/libgnt | CONFIGURE_GEN_FAIL |  |
| geofft/sbuild | CONFIGURE_RUN_FAIL | ./configure: line 1348: syntax error near unexpected token `newline' |
| geomview/geomview | CONFIGURE_RUN_FAIL | ./configure: line 1502: syntax error near unexpected token `else' |
| GerHobbelt/html2db | CONFIGURE_RUN_FAIL | ./configure: line 1394: syntax error near unexpected token `)' |
| gigaflow-vswitch/gvs | CONFIGURE_RUN_FAIL | ./configure: line 1666: syntax error near unexpected token `fi' |
| GiterMirror/mpg123 | CONFIGURE_RUN_FAIL | ./configure: line 1399: failed: command not found |
| gitGNU/gnu_ld | CONFIGURE_RUN_FAIL | ./configure: line 1397: ACX_LARGEFILE: command not found |
| gitpan/libnf | CONFIGURE_RUN_FAIL |  |
| giuseppe/containers-dedup | CONFIGURE_RUN_FAIL | ./configure: line 1749: syntax error near unexpected token `(' |
| glebius/minidlna | CONFIGURE_RUN_FAIL | ./configure: line 1495: AC_STRUCT_DIRENT_D_TYPE: command not found |
| GNOME/goffice | CONFIGURE_RUN_FAIL | ./configure: command substitution: line 1638: syntax error near unexpected token `)' |
| gnosis/libunistring | CONFIGURE_RUN_FAIL | ./configure: line 1346: is: command not found |
| gnu-mirror-unofficial/commoncpp | CONFIGURE_RUN_FAIL |  |
| gnu-smalltalk/smalltalk | CONFIGURE_RUN_FAIL |  |
| GNUAspell/aspell | CONFIGURE_RUN_FAIL | checking if file locking and truncating is supported... checking if mmap and friends is su |
| GodshadowLQH/usbview | CONFIGURE_RUN_FAIL | ./configure: line 1371: syntax error near unexpected token `done' |
| golems/ach | CONFIGURE_RUN_FAIL | ./configure: line 1391: ac_cv_have_decl_PRIuPTR,PRIu64,PRIx64=no: command not found |
| golems/amino | CONFIGURE_RUN_FAIL | configure: error: BLAS is required. |
| goodspeed34/ws63flash | CONFIGURE_RUN_FAIL | configure: error: |
| gpac-buildbot/libmad | CONFIGURE_RUN_FAIL | ./configure: line 2451: syntax error near unexpected token `(' |
| gpg/libassuan | CONFIGURE_RUN_FAIL | ./configure: line 1986: [printf: command not found |
| gpudirect/libibverbs | CONFIGURE_RUN_FAIL | ./configure: line 1436: syntax error near unexpected token `;' |
| graemes/poolparty-x16r | CONFIGURE_RUN_FAIL | ./configure: line 1413: ac_cv_have_decl_be32dec,: command not found |
| graydon/monotone | CONFIGURE_RUN_FAIL | ./configure: line 1361: syntax error near unexpected token `is' |
| GREO/gnuradio-git | CONFIGURE_RUN_FAIL | ./configure: line 1407: GR_VERSION: command not found |
| GridOPTICS/FNCS | CONFIGURE_RUN_FAIL | ./configure: line 1471: syntax error near unexpected token `else' |
| groonga/groonga | CONFIGURE_RUN_FAIL | ./configure: line 2665: ./version.sh: No such file or directory |
| gszura/wx-nfp | CONFIGURE_RUN_FAIL | configure: error: |
| guardianproject/libsqlfs | CONFIGURE_RUN_FAIL | configure: error: --with-sqlcipher was given but test failed |
| gvvaughan/slingshot | CONFIGURE_RUN_FAIL | for a Lua interpreter with version >= 5.1, < 5.4... configure: error: cannot find suitable |
| gvz/avrdude | CONFIGURE_RUN_FAIL | ./configure: line 1437: syntax error near unexpected token `AR,' |
| gypified/libmpg123 | CONFIGURE_RUN_FAIL | ./configure: line 1399: failed: command not found |
| hackerschoice/gsocket | CONFIGURE_RUN_FAIL | configure: error: libnet 1.1.x not found |
| hamonikr-root/fontconfig | CONFIGURE_RUN_FAIL | for build system executable suffix... ./configure: line 1493: syntax error near unexpected |
| hamonikr-root/mate-screensaver | CONFIGURE_RUN_FAIL | ./configure: line 1366: intltool-update: command not found |
| handshake-org/hnsd | CONFIGURE_RUN_FAIL | configure: error: invalid network |
| HansWessels/gup | CONFIGURE_RUN_FAIL | ./configure: line 1354: GUP_CYGWIN: command not found |
| haozhangphd/QuantLib-noBoost-SWIG | CONFIGURE_RUN_FAIL | ./configure: line 1345: This: command not found |
| haproxytech/spoa-mirror | CONFIGURE_RUN_FAIL | ./configure: line 1405: syntax error near unexpected token `;;' |
| Harvard-PRINCESS/sablevm | CONFIGURE_RUN_FAIL | configure: error: bad value "" for --enable-magic |
| haussli/rancid | CONFIGURE_RUN_FAIL | ./configure: line 1412: AC_INCLUDES_DEFAULT: command not found |
| hb/claws_mail_opensync_plugin | CONFIGURE_RUN_FAIL |  |
| hb9xar/siproxd | CONFIGURE_RUN_FAIL | ./configure: line 1411: syntax error near unexpected token `recursive' |
| hdoddikindi/fstrm | CONFIGURE_RUN_FAIL | ./configure: line 1526: ac_cv_have_decl_fread_unlocked,: command not found |
| herczy/tinu | CONFIGURE_RUN_FAIL | ./configure: line 1530: syntax error near unexpected token `fi' |
| hermansr/psid64 | CONFIGURE_RUN_FAIL | ./configure: line 1450: AX_FUNC_MKDIR: command not found |
| hexagonal-sun/bic | CONFIGURE_RUN_FAIL | ./configure: line 1414: AX_LIB_READLINE: command not found |
| hezi/dosbox-x-gdb | CONFIGURE_RUN_FAIL | checking whether cc -E accepts ... ./configure: line 1645: will: command not found |
| hgst/libnvme | CONFIGURE_RUN_FAIL | ./configure: line 1476: pthread-config: command not found |
| hharte/tn5250 | CONFIGURE_RUN_FAIL | whether to build curses terminal... whether to use old key handler... ./configure: line 15 |
| hholzgra/connector-c-examples | CONFIGURE_RUN_FAIL | checking for mysql_config executable... ./configure: line 2527: syntax error: unexpected e |
| hhool/nut | CONFIGURE_RUN_FAIL | ./configure: line 1346: syntax error near unexpected token `else' |
| hiha-lang/hiha | CONFIGURE_RUN_FAIL | Try 0 --help for more information.: syntax error in expression (error token is "Try 0 --he |
| hkerem/squid3-ssl | CONFIGURE_RUN_FAIL | ./configure: line 1387: again,: command not found |
| hnwfs/lighttpd-plus | CONFIGURE_RUN_FAIL | ./configure: line 1437: AM_C_PROTOTYPES: command not found |
| HomerReid/buff-em | CONFIGURE_RUN_FAIL | ./configure: line 1381: AC_F77_WRAPPERS: command not found |
| horms/ovs | CONFIGURE_RUN_FAIL | ./configure: line 1378: syntax error near unexpected token `$'doesn\'t try enabling C99 in |
| hpc/cce-mpi-openmpi-1.4.4 | CONFIGURE_RUN_FAIL |  |
| hroptatyr/clob | CONFIGURE_RUN_FAIL | ./configure: line 1384: syntax error near unexpected token `(' |
| hroptatyr/echse | CONFIGURE_RUN_FAIL | ./configure: line 1397: syntax error near unexpected token `(' |
| hroptatyr/truffle | CONFIGURE_RUN_FAIL | ./configure: line 1384: syntax error near unexpected token `(' |
| hroptatyr/yuck | CONFIGURE_RUN_FAIL | ./configure: line 1384: syntax error near unexpected token `(' |
| huleyv/iperf2 | CONFIGURE_RUN_FAIL | ./configure: line 1384: syntax error near unexpected token `stdint.h' |
| IBMSpectrumComputing/lsf-drmaa | CONFIGURE_RUN_FAIL | ./configure: line 1342: FedStage: command not found |
| iczelia/xpar | CONFIGURE_RUN_FAIL | configure: error: --with-windows-target must be vista or win95 (got: ) |
| idiap/juicer | CONFIGURE_RUN_FAIL | configure: error: Required library TRACTER not found |
| iem-projects/ncview | CONFIGURE_RUN_FAIL | ./configure: line 1341: AC_PATH_NETCDF: command not found |
| igmhub/likely | CONFIGURE_RUN_FAIL | ./configure: line 1351: AX_EXT: command not found |

## ⚪ Non-working — not standalone (GNU autotools also fails; not our bug) (250)

| repo | stage |
| --- | --- |
| 315234/lyx-retina | CONFIGURE_GEN_FAIL |
| acaudwell/Logstalgia | CONFIGURE_RUN_FAIL |
| acerion/cwdaemon | CONFIGURE_RUN_FAIL |
| acoin-project/acoin | CONFIGURE_RUN_FAIL |
| adjacentlink/emane-layer-dlep | CONFIGURE_RUN_FAIL |
| adobe-research/libkafka | CONFIGURE_RUN_FAIL |
| AegisEmu/AegisEmu | CONFIGURE_RUN_FAIL |
| agalakhov/captdriver | CONFIGURE_RUN_FAIL |
| agordon/fastx_toolkit | CONFIGURE_RUN_FAIL |
| aissammouche/bitextor | CONFIGURE_RUN_FAIL |
| ajhc/ajhc | CONFIGURE_RUN_FAIL |
| ajnelson/geoproc | CONFIGURE_RUN_FAIL |
| ajnelson/regxml_extractor | CONFIGURE_RUN_FAIL |
| alexandervdm/gummi | CONFIGURE_RUN_FAIL |
| alsaplayer/alsaplayer | CONFIGURE_RUN_FAIL |
| andestech/nds-openocd | CONFIGURE_RUN_FAIL |
| ansisatteka/ovs-ipsec | CONFIGURE_RUN_FAIL |
| ant9000/libsigrok | CONFIGURE_RUN_FAIL |
| AnthonyDeroche/mod_authnz_jwt | CONFIGURE_RUN_FAIL |
| anyone-protocol/ator-protocol | CONFIGURE_GEN_FAIL |
| Apress/Why-Learn-C | CONFIGURE_RUN_FAIL |
| araisrobo/libwosi | CONFIGURE_RUN_FAIL |
| ArchimedesCAD/libredwg | CONFIGURE_RUN_FAIL |
| ArcticaProject/libpam-x2go | CONFIGURE_RUN_FAIL |
| argonne-lcf/THAPI | CONFIGURE_RUN_FAIL |
| arkadijs/asterisk-g72x | CONFIGURE_RUN_FAIL |
| arki55/fuse-fuse | CONFIGURE_RUN_FAIL |
| arkq/bluez-alsa | CONFIGURE_RUN_FAIL |
| armenb/sharktools | CONFIGURE_RUN_FAIL |
| ARPA-SIMC/arkimet | CONFIGURE_RUN_FAIL |
| ARPA-SIMC/dballe | CONFIGURE_RUN_FAIL |
| artyom-poptsov/metabash | CONFIGURE_RUN_FAIL |
| Ashod/garli | CONFIGURE_RUN_FAIL |
| assen-totin/mate-applet-streamer | CONFIGURE_RUN_FAIL |
| avr-aics-riken/CIOlib | CONFIGURE_RUN_FAIL |
| awsteiner/o2scl | CONFIGURE_RUN_FAIL |
| ayyi/samplecat | CONFIGURE_RUN_FAIL |
| azatoth/minidlna | CONFIGURE_RUN_FAIL |
| backuppc/backuppc-xs | CONFIGURE_RUN_FAIL |
| baoxuezhao/GPU-SExtractor | CONFIGURE_RUN_FAIL |
| bat/bat | CONFIGURE_RUN_FAIL |
| BayshoreNetworks/yextend | CONFIGURE_RUN_FAIL |
| bbc/bbcat-control | CONFIGURE_RUN_FAIL |
| bbc/bbcat-dsp | CONFIGURE_RUN_FAIL |
| bbc/bbcat-fileio | CONFIGURE_RUN_FAIL |
| bbidulock/xde-sounds | CONFIGURE_RUN_FAIL |
| bdonlan/libofx | CONFIGURE_RUN_FAIL |
| benchmark-subsetting/cere | CONFIGURE_RUN_FAIL |
| bert/geda-gaf | CONFIGURE_RUN_FAIL |
| bgpsecurity/rpstir | CONFIGURE_RUN_FAIL |
| BIC-MNI/bicpl | CONFIGURE_RUN_FAIL |
| BIC-MNI/minc | CONFIGURE_RUN_FAIL |
| BIMSBbioinfo/swineherd | CONFIGURE_RUN_FAIL |
| BirolLab/abyss | CONFIGURE_RUN_FAIL |
| bisco/uftrace | CONFIGURE_RUN_FAIL |
| bitblaze-fuzzball/fuzzball | CONFIGURE_RUN_FAIL |
| bitcoinbabys/flexinodes | CONFIGURE_RUN_FAIL |
| bitzeppelin/linphone-sdk | CONFIGURE_RUN_FAIL |
| bjoernvoss/RNAHeliCes | CONFIGURE_RUN_FAIL |
| bk138/wxservdisc | CONFIGURE_RUN_FAIL |
| BOINC/boinc | CONFIGURE_RUN_FAIL |
| bondhugula/pluto | CONFIGURE_RUN_FAIL |
| bradenmcd/uri-grammar | CONFIGURE_RUN_FAIL |
| brimworks/zile | CONFIGURE_RUN_FAIL |
| bsc-pm/sonar | CONFIGURE_RUN_FAIL |
| bsc-pm/tasycl | CONFIGURE_RUN_FAIL |
| bspeice/libcvautomation | CONFIGURE_RUN_FAIL |
| bulislaw/obexd-eds | CONFIGURE_RUN_FAIL |
| carboncointrust/CarboncoinCore | CONFIGURE_RUN_FAIL |
| cbg-ethz/shorah | CONFIGURE_GEN_FAIL |
| cbuchner1/ccminer | CONFIGURE_RUN_FAIL |
| cebix/psximager | CONFIGURE_RUN_FAIL |
| CESARBR/knot-network-nrf24 | CONFIGURE_RUN_FAIL |
| CESARBR/knot-service-source | CONFIGURE_RUN_FAIL |
| CESNET/GPUJPEG | CONFIGURE_RUN_FAIL |
| CESNET/libfastbit | CONFIGURE_RUN_FAIL |
| cfra/quagga-testing | CONFIGURE_RUN_FAIL |
| cgdb/cgdb | CONFIGURE_RUN_FAIL |
| chabbimilind/cctlib | CONFIGURE_RUN_FAIL |
| choeger/MetaModelica-autotools | CONFIGURE_RUN_FAIL |
| chosen1/SniffDet | CONFIGURE_RUN_FAIL |
| christian-sahlmann/gwyddion | CONFIGURE_GEN_FAIL |
| ChrisVine/guile-a-sync | CONFIGURE_RUN_FAIL |
| Chronic-Dev/libgcrypt | CONFIGURE_RUN_FAIL |
| ClusterLabs/fence-agents | CONFIGURE_RUN_FAIL |
| cminyard/ser2net | CONFIGURE_RUN_FAIL |
| cmu-sei/BigGrep | CONFIGURE_RUN_FAIL |
| CoachRun/boinc | CONFIGURE_RUN_FAIL |
| colaghost/coroutine_event | CONFIGURE_RUN_FAIL |
| common-tools-interface/cti | CONFIGURE_RUN_FAIL |
| compiz-reloaded/emerald | CONFIGURE_RUN_FAIL |
| COMSYS/tor4iot-tor | CONFIGURE_RUN_FAIL |
| ConsoleKit2/ConsoleKit2 | CONFIGURE_GEN_FAIL |
| coolwanglu/scanmem_ | CONFIGURE_RUN_FAIL |
| CoryXie/GRUB2 | CONFIGURE_RUN_FAIL |
| cosmicrays/DRAGON | CONFIGURE_RUN_FAIL |
| cpaasch/wireshark | CONFIGURE_RUN_FAIL |
| CRG-Barcelona/bwtool | CONFIGURE_RUN_FAIL |
| criort/libPRISM | CONFIGURE_RUN_FAIL |
| cruppstahl/upscaledb | CONFIGURE_RUN_FAIL |
| CryptoBridge/bridgecoin | CONFIGURE_RUN_FAIL |
| cryptode/cryptode | CONFIGURE_RUN_FAIL |
| CryptVenture/BitMoneyV2 | CONFIGURE_RUN_FAIL |
| CyanogenMod/android_external_protobuf-c | CONFIGURE_RUN_FAIL |
| cybermaggedon/cyberprobe | CONFIGURE_RUN_FAIL |
| CZ-NIC/fred-db | CONFIGURE_RUN_FAIL |
| danos/frr | CONFIGURE_RUN_FAIL |
| davelambert/guile-pcap | CONFIGURE_RUN_FAIL |
| davidgiven/libfirm | CONFIGURE_RUN_FAIL |
| dbmail/dbmail | CONFIGURE_RUN_FAIL |
| dcos/dcos-mesos-modules | CONFIGURE_RUN_FAIL |
| ddarriba/pll-modules | CONFIGURE_RUN_FAIL |
| DeNA/HandlerSocket-Plugin-for-MySQL | CONFIGURE_RUN_FAIL |
| detomastah/adwc | CONFIGURE_RUN_FAIL |
| DGCDev/digitalcoin | CONFIGURE_RUN_FAIL |
| dillo-browser/dillo | CONFIGURE_RUN_FAIL |
| Distrotech/libjpeg-turbo | CONFIGURE_RUN_FAIL |
| dividendcash/DividendCash | CONFIGURE_RUN_FAIL |
| dkosmari/gtk-sfml | CONFIGURE_RUN_FAIL |
| dkosmari/libwupsxx | CONFIGURE_RUN_FAIL |
| dkosmari/Papaya-HUD | CONFIGURE_RUN_FAIL |
| dkrotx/htmarkup | CONFIGURE_RUN_FAIL |
| dmp0x7c5/gobexfuse | CONFIGURE_RUN_FAIL |
| dns-stats/hedgehog | CONFIGURE_RUN_FAIL |
| dnstap/dnstap-ldns | CONFIGURE_RUN_FAIL |
| DocQMiner/tesseract-4.0.0-beta.1 | CONFIGURE_RUN_FAIL |
| doug65536/dgos | CONFIGURE_RUN_FAIL |
| Dr-Shadow/netsoul-purple | CONFIGURE_RUN_FAIL |
| DreamSourceLab/DSLogic-fw | CONFIGURE_RUN_FAIL |
| drewc/guix | CONFIGURE_RUN_FAIL |
| droogie/bluez-fuzzer | CONFIGURE_RUN_FAIL |
| dtbartle/cgminer-gc3355 | CONFIGURE_RUN_FAIL |
| dudochkin-victor/sqlheavy | CONFIGURE_GEN_FAIL |
| dyne/Freecoin | CONFIGURE_RUN_FAIL |
| dyne/FreeJ | CONFIGURE_RUN_FAIL |
| ecliptchain/eclipt-source | CONFIGURE_RUN_FAIL |
| eeight/tdheap | CONFIGURE_RUN_FAIL |
| ekpyron/oclp | CONFIGURE_RUN_FAIL |
| elima/gjs-commonjs | CONFIGURE_GEN_FAIL |
| ElvishArtisan/lwcore | CONFIGURE_RUN_FAIL |
| EmeraldMiningCo/Ebits | CONFIGURE_RUN_FAIL |
| endlessm/basin | CONFIGURE_RUN_FAIL |
| endlessm/eos-knowledge-lib | CONFIGURE_RUN_FAIL |
| endlessm/xapian-bridge | CONFIGURE_RUN_FAIL |
| epeec/TAGASPI | CONFIGURE_RUN_FAIL |
| epruesse/SINA | CONFIGURE_RUN_FAIL |
| equalitie/gnunet | CONFIGURE_GEN_FAIL |
| ESiWACE/esdm-netcdf-4.6.2-old | CONFIGURE_RUN_FAIL |
| esrille/escudo | CONFIGURE_RUN_FAIL |
| essej/freqtweak | CONFIGURE_RUN_FAIL |
| etr/libhttpserver | CONFIGURE_RUN_FAIL |
| evjeesm/hashset | CONFIGURE_RUN_FAIL |
| excamera/alfalfa | CONFIGURE_RUN_FAIL |
| Expensify/mk_livestatus | CONFIGURE_RUN_FAIL |
| experiencecoin/experiencecoin_legacy | CONFIGURE_RUN_FAIL |
| fandangos/libbluray | CONFIGURE_RUN_FAIL |
| fangq/medit | CONFIGURE_RUN_FAIL |
| farsightsec/axa | CONFIGURE_RUN_FAIL |
| farsightsec/dnstable | CONFIGURE_RUN_FAIL |
| farsightsec/sie-nmsg | CONFIGURE_RUN_FAIL |
| fedoracoin-dev/fedoracoin | CONFIGURE_RUN_FAIL |
| filiphanes/fts-elastic | CONFIGURE_RUN_FAIL |
| finit-project/finit | CONFIGURE_RUN_FAIL |
| FinTP/fintp_payloadevaluators | CONFIGURE_RUN_FAIL |
| firecore/libbluray | CONFIGURE_RUN_FAIL |
| firehol/firehol | CONFIGURE_RUN_FAIL |
| Firstyear/ds_rust | CONFIGURE_RUN_FAIL |
| fix8/fix8 | CONFIGURE_RUN_FAIL |
| flatpak/ppa-xdg-desktop-portal | CONFIGURE_RUN_FAIL |
| flightaware/tclreadline | CONFIGURE_RUN_FAIL |
| flux-framework/flux-pmix | CONFIGURE_GEN_FAIL |
| flux-framework/flux-security | CONFIGURE_RUN_FAIL |
| fomy/destor | CONFIGURE_RUN_FAIL |
| FOSSEE/scilab_for_xcos_on_cloud | CONFIGURE_RUN_FAIL |
| fractalcoin/fractalcoin | CONFIGURE_RUN_FAIL |
| Freeaqingme/libvmod-oauth | CONFIGURE_RUN_FAIL |
| freeipa/bind-dyndb-ldap | CONFIGURE_RUN_FAIL |
| fries/android-external-openvpn | CONFIGURE_RUN_FAIL |
| FrodeSolheim/fs-uae | CONFIGURE_RUN_FAIL |
| ftynse/ppcg-fb | CONFIGURE_RUN_FAIL |
| FundacionPesetacoin/PesetacoinCore | CONFIGURE_RUN_FAIL |
| futurerestore/idevicerestore | CONFIGURE_RUN_FAIL |
| Gahznt/otserver_source | CONFIGURE_RUN_FAIL |
| GaloisInc/gghlite-flint | CONFIGURE_RUN_FAIL |
| ganehag/open-modbusgateway | CONFIGURE_RUN_FAIL |
| gapcoin/gapcoin | CONFIGURE_RUN_FAIL |
| gass/dbpc-test | CONFIGURE_RUN_FAIL |
| geodynamics/citcoms | CONFIGURE_RUN_FAIL |
| ghani-1977/enigma2-openpli-sh4-2 | CONFIGURE_RUN_FAIL |
| giltirn/HPCortex | CONFIGURE_RUN_FAIL |
| gingi/fastbit | CONFIGURE_RUN_FAIL |
| gitGNU/gnu_ccd2cue | CONFIGURE_RUN_FAIL |
| gitGNU/gnu_foliot | CONFIGURE_RUN_FAIL |
| gitGNU/gnu_lash | CONFIGURE_RUN_FAIL |
| gitGNU/gnu_sipwitch | CONFIGURE_RUN_FAIL |
| GNOME/dasher | CONFIGURE_RUN_FAIL |
| GNOME/easytag | CONFIGURE_RUN_FAIL |
| GNOME/gnumeric | CONFIGURE_RUN_FAIL |
| GNOME/metacity | CONFIGURE_RUN_FAIL |
| GNUFreetalk/freetalk | CONFIGURE_RUN_FAIL |
| GNUnet-Mirror/GNUnet | CONFIGURE_RUN_FAIL |
| goatattack/goatattack | CONFIGURE_RUN_FAIL |
| gobolinux/GoboHide | CONFIGURE_RUN_FAIL |
| golosio/NeuronGPU | CONFIGURE_RUN_FAIL |
| google/certificate-transparency | CONFIGURE_RUN_FAIL |
| google/hiba | CONFIGURE_RUN_FAIL |
| Gotos/CuteCapture | CONFIGURE_RUN_FAIL |
| gpg/gpgme | CONFIGURE_RUN_FAIL |
| gpg/libgcrypt | CONFIGURE_RUN_FAIL |
| gpg/scute | CONFIGURE_RUN_FAIL |
| GPGTools/pinentry | CONFIGURE_RUN_FAIL |
| gphoto/libgphoto2-python | CONFIGURE_RUN_FAIL |
| gpudirect/libgdsync | CONFIGURE_RUN_FAIL |
| gramseyer/hotstuff | CONFIGURE_RUN_FAIL |
| gregkh/bti | CONFIGURE_RUN_FAIL |
| grinsfem/grins | CONFIGURE_RUN_FAIL |
| grke/burp | CONFIGURE_RUN_FAIL |
| grrrr/flext | CONFIGURE_RUN_FAIL |
| gssapi/mod_auth_gssapi | CONFIGURE_RUN_FAIL |
| gyoto/Gyoto | CONFIGURE_RUN_FAIL |
| h0tw1r3/libuuid-mingw | CONFIGURE_RUN_FAIL |
| hackerschoice/gsocket-relay | CONFIGURE_RUN_FAIL |
| haegrr/reprepro | CONFIGURE_RUN_FAIL |
| hallyn/upstart | CONFIGURE_RUN_FAIL |
| HDFGroup/vol-log-based | CONFIGURE_RUN_FAIL |
| HeapStats/heapstats | CONFIGURE_RUN_FAIL |
| heliocastro/gpgme | CONFIGURE_RUN_FAIL |
| hello31337/BI-SGX | CONFIGURE_RUN_FAIL |
| hercules-390/hyperion | CONFIGURE_GEN_FAIL |
| hermet/enventor | CONFIGURE_RUN_FAIL |
| HewlettPackard/nagios-plugins-hpilo | CONFIGURE_RUN_FAIL |
| hexhex/mergingplugin | CONFIGURE_RUN_FAIL |
| hfiguiere/libopenraw | CONFIGURE_RUN_FAIL |
| hfst/hfst-ospell | CONFIGURE_RUN_FAIL |
| hgneng/ekho | CONFIGURE_RUN_FAIL |
| hightman/xunsearch | CONFIGURE_RUN_FAIL |
| hollowiette/xrdp | CONFIGURE_RUN_FAIL |
| holzschu/lib-tex | CONFIGURE_RUN_FAIL |
| hongyi-zhao/lyx | CONFIGURE_RUN_FAIL |
| hpc/libdftw | CONFIGURE_RUN_FAIL |
| hpc/supermagic | CONFIGURE_RUN_FAIL |
| htrb/ngraph-gtk | CONFIGURE_RUN_FAIL |
| huceke/xine-lib-vaapi | CONFIGURE_GEN_FAIL |
| hunter-packages/libmicrohttpd | CONFIGURE_GEN_FAIL |
| iagorubio/cssed-vte-plugin | CONFIGURE_RUN_FAIL |
| iainlane/mo | CONFIGURE_GEN_FAIL |
| iamashwin99/octopus-debian-package | CONFIGURE_RUN_FAIL |
| IanSav/enigma2-Beyonwiz | CONFIGURE_RUN_FAIL |
| ibm-power-utilities/powerpc-utils | CONFIGURE_RUN_FAIL |
| IBM/corosync-qdisk | CONFIGURE_RUN_FAIL |

