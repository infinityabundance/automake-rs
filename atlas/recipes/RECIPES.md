# Atlas Recipes — working / non-working roster

Total **434** recipes. **Working (built end-to-end): 3** · non-working: 133 partial · 102 not-standalone · 196 failed.

"Working" means the full pipeline (autoreconf → configure → make) succeeded under the GNU-free toolchain. `quirk_dependent` needed an auto-applied quirk; `sealed` needed none.

## ✅ Working (3)

| repo | court |
| --- | --- |
| ayumin/open-cobol | quirk_dependent |
| chimari/MaCoPiX | quirk_dependent |
| cosmos72/twin | quirk_dependent |

## 🟡 Non-working — partial (configure cleared, make failed) (133)

| repo | stage | first error |
| --- | --- | --- |
| 5nord/bison-example | MAKE_FAIL |  |
| abhaykadam/vm | MAKE_FAIL |  |
| abihf/kamus | MAKE_FAIL |  |
| acidburn0zzz/spice-xpi | MAKE_FAIL |  |
| adulau/dcfldd | MAKE_FAIL |  |
| agronick/Relay | MAKE_FAIL | ./configure: line 1362: GLIB_GSETTINGS: command not found |
| aizvorski/h264bitstream | MAKE_FAIL | ./configure: line 1383: LT_PATH_LD: command not found |
| alliedtelesis/apteryx-rest | MAKE_FAIL |  |
| allinurl/gwsocket | MAKE_FAIL |  |
| alobbs/macchanger | MAKE_FAIL |  |
| Alpacius/a6 | MAKE_FAIL |  |
| ampledata/ldsped | MAKE_FAIL |  |
| anatol/google-coredumper | MAKE_FAIL |  |
| anchor/filtergen | MAKE_FAIL |  |
| AndyA/rfile | MAKE_FAIL |  |
| andygrundman/libmediascan | MAKE_FAIL |  |
| antonblanchard/kexec-lite | MAKE_FAIL |  |
| antonblanchard/qtrace-tools | MAKE_FAIL |  |
| arcalex/racktk | MAKE_FAIL |  |
| arjunchitturi/htmlstreamparser | MAKE_FAIL |  |
| audionuma/libtruepeak | MAKE_FAIL |  |
| b/ION | MAKE_FAIL |  |
| Bader-Research/snap-graph | MAKE_FAIL | for _AC_LANG compiler vendor... ./configure: line 1429: break_AC_LANG_ABBREV: command not  |
| barak/vobcopy | MAKE_FAIL | ./configure: line 1364: AX_CFLAGS_WARN_ALL: command not found |
| BatchDrake/lfsrintruder | MAKE_FAIL |  |
| benegon/ntp | MAKE_FAIL |  |
| beniz/hmdp | MAKE_FAIL |  |
| benlemasurier/stormfs | MAKE_FAIL |  |
| bergundy/IOQueue | MAKE_FAIL |  |
| BioND/myrng | MAKE_FAIL |  |
| BirolLab/biobloom | MAKE_FAIL |  |
| bjango/istatserverlinux | MAKE_FAIL |  |
| blakecaldwell/fluidmem | MAKE_FAIL |  |
| blueness/fts-standalone | MAKE_FAIL |  |
| bpeel/prevodb | MAKE_FAIL |  |
| BrandRegard/gnash | MAKE_FAIL |  |
| bryteise/ister | MAKE_FAIL |  |
| bytedeco/helloworld | MAKE_FAIL |  |
| Cacti/spine | MAKE_FAIL |  |
| CAIDA/libparsebgp | MAKE_FAIL |  |
| carlos-lopez-garces/mapnik | MAKE_FAIL | ./configure: line 1354: AM_PROG_CC_STDC: command not found |
| cdobrich/btnx | MAKE_FAIL |  |
| ceph/gf-complete | MAKE_FAIL | ./configure: line 1892: ax_cv_check__AC_LANG_ABBREVflags__-mavx=yes: command not found |
| cfsghost/mdsfs | MAKE_FAIL | ./configure: line 1341: AC_PROG_INTLTOOL: command not found |
| chaoskagami/corbenik | MAKE_FAIL |  |
| chaupal/jxta-c | MAKE_FAIL | ./configure: line 1379: AM_PROG_CC_STDC: command not found |
| chenbd/libwfd | MAKE_FAIL |  |
| chenbd/miracle | MAKE_FAIL |  |
| cheshire-mouse/hexchat-indicator | MAKE_FAIL |  |
| chokkan/liblbfgs | MAKE_FAIL |  |
| chrisidefix/nurbs | MAKE_FAIL | ./configure: line 1353: PLIB_INSIDE_MINDSEYE: command not found |
| ChrisLidbury/CLSmith | MAKE_FAIL |  |
| christoph-cullmann/xblast | MAKE_FAIL |  |
| cisco/open-nFAPI | MAKE_FAIL |  |
| claesenm/approxsvm | MAKE_FAIL |  |
| clone/xml2 | MAKE_FAIL |  |
| cmbi/hssp | MAKE_FAIL |  |
| cmcqueen/aes-min | MAKE_FAIL |  |
| codebutler/firesheep | MAKE_FAIL | ./configure: line 1387: has: command not found |
| commiyou/iniparser | MAKE_FAIL |  |
| cooljeanius/gcab | MAKE_FAIL |  |
| cosimoc/gnome-example-search-provider | MAKE_FAIL | ./configure: line 1392: GLIB_GSETTINGS: command not found |
| cpptest/cpptest | MAKE_FAIL |  |
| crystax/android-vendor-gnu-tar | MAKE_FAIL |  |
| csete/gpredict | MAKE_FAIL |  |
| csmith-project/creduce | MAKE_FAIL |  |
| CumulusNetworks/ptm | MAKE_FAIL |  |
| cybergarage/mupnp-cc | MAKE_FAIL |  |
| cybergarage/uhttp-cc | MAKE_FAIL |  |
| dahlem/lz-prediction | MAKE_FAIL | ./configure: line 1366: [ext],[mandatory]printf: command not found |
| damien-lemoal/zonefs-tools | MAKE_FAIL |  |
| dangerousben/jsonval | MAKE_FAIL |  |
| DanielO/codec2 | MAKE_FAIL |  |
| darshan-hpc/darshan | MAKE_FAIL |  |
| daveyc/gawk_zos | MAKE_FAIL | configure.ac:47: error: possibly undefined macro: AM_INIT_AUTOMAKE |
| davidt/Fyre | MAKE_FAIL | ./configure: line 1415: AM_BINRELOC: command not found |
| dbcode/protobuf-nginx | MAKE_FAIL |  |
| deepin-community/libtextwrap | MAKE_FAIL |  |
| deepin-community/motif | MAKE_FAIL | ./configure: line 1407: LT_LIB_XTHREADS: command not found |
| df7cb/sdate | MAKE_FAIL |  |
| dhvani-tts/dhvani-tts | MAKE_FAIL |  |
| digitalocean/hivex | MAKE_FAIL | /tmp/atlasx_digitalocean__hivex/s/build-aux/missing: line 81: automake-1.15: command not f |
| digitalocean/libguestfs | MAKE_FAIL |  |
| Distrotech/diffutils | MAKE_FAIL | /tmp/atlasx_Distrotech__diffutils/s/build-aux/missing: line 81: automake-1.14: command not |
| Distrotech/esound | MAKE_FAIL |  |
| Distrotech/findutils | MAKE_FAIL |  |
| Distrotech/flac | MAKE_FAIL |  |
| Distrotech/gdbm | MAKE_FAIL |  |
| Distrotech/libart | MAKE_FAIL |  |
| Distrotech/libsecret | MAKE_FAIL |  |
| Distrotech/minicom | MAKE_FAIL | /tmp/atlasx_Distrotech__minicom/s/missing: line 52: automake-1.11: command not found |
| djandruczyk/eXtace | MAKE_FAIL | checking for esd_monitor_stream in -lesd... ./configure: line 1456: esd-config: command no |
| Drive-Trust-Alliance/sedutil | MAKE_FAIL |  |
| drmingdrmer/lrc-erasure-code | MAKE_FAIL |  |
| dsigma/dfu-util | MAKE_FAIL |  |
| dterweij/ndjbdns | MAKE_FAIL |  |
| EasyRPG/Tools | MAKE_FAIL |  |
| elmo2k3/libhagraph | MAKE_FAIL |  |
| embtom/kmscube | MAKE_FAIL |  |
| endlessm/eos-browser-tools | MAKE_FAIL | ./configure: line 1363: GLIB_GSETTINGS: command not found |
| enki/libev | MAKE_FAIL |  |
| Enough-Software/pcap-http-analyzer | MAKE_FAIL |  |
| ericherman/libfastset | MAKE_FAIL |  |
| eriknyquist/librxvm | MAKE_FAIL |  |
| esrille/esidl | MAKE_FAIL |  |
| eurovibes/mixmaster | MAKE_FAIL |  |
| ewxrjk/sftpserver | MAKE_FAIL | ./configure: line 1361: AC_SET_MAKE: command not found |
| firnsy/yubipam | MAKE_FAIL |  |
| fordmason/cronolog | MAKE_FAIL |  |
| frank-zago/xgalaga-sdl | MAKE_FAIL |  |
| fredrikwidlund/libclo | MAKE_FAIL |  |
| fredrikwidlund/libdynamic | MAKE_FAIL |  |
| fredrikwidlund/libdynamic_benchmark | MAKE_FAIL |  |
| fumiyas/wcwidth-cjk | MAKE_FAIL |  |
| ganesh503/Asus-Aura | MAKE_FAIL |  |
| GArik/bash-completion | MAKE_FAIL |  |
| genebean/uptimed | MAKE_FAIL |  |
| giannitedesco/ccid-utils | MAKE_FAIL | ./configure: line 1361: AM_PROG_CC_STDC: command not found |
| gizero/autotools-skeleton | MAKE_FAIL |  |
| glv2/bruteforce-wallet | MAKE_FAIL |  |
| gmarcais/Quorum | MAKE_FAIL |  |
| godsflaw/xor_toolkit | MAKE_FAIL |  |
| google/ios-webkit-debug-proxy | MAKE_FAIL |  |
| grant-h/usbutils-portable | MAKE_FAIL |  |
| greyltc/android_external_sshfs | MAKE_FAIL |  |
| GroovIM/transport | MAKE_FAIL |  |
| gucong/robotxq | MAKE_FAIL |  |
| habanero-rice/habanero-upc | MAKE_FAIL |  |
| hafslund/cc2531-sniffer | MAKE_FAIL |  |
| hpc/xpmem | MAKE_FAIL |  |
| hyperic/sigar | MAKE_FAIL |  |
| hyrathb/mentohust | MAKE_FAIL |  |
| idosch/ethtool | MAKE_FAIL |  |

## ❌ Non-working — failed (ours fails before make) (196)

| repo | stage | first error |
| --- | --- | --- |
| 6WIND/quagga | CONFIGURE_RUN_FAIL | checking whether we are using SunPro compiler... ./configure: line 1488: syntax error near |
| A-Kyle/GrADS-CJK | CONFIGURE_RUN_FAIL | ./configure: line 1619: syntax error near unexpected token `seems' |
| a-sassmannshausen/guile-monads | CONFIGURE_RUN_FAIL | ./configure: line 1352: syntax error near unexpected token `2.0.11' |
| accellera-official/systemc | CONFIGURE_RUN_FAIL | whether we are using a Clang/LLVM C++ compiler... ./configure: line 1378: syntax error nea |
| adapteva/epiphany-libs | CONFIGURE_RUN_FAIL | ./configure: line 1917: syntax error near unexpected token `"e-lib"' |
| ademakov/Oroch | CONFIGURE_RUN_FAIL | ./configure: line 1412: syntax error near unexpected token `done' |
| adiknoth/netatalk-debian | CONFIGURE_RUN_FAIL | ./configure: line 1376: AC_PROG_PERL: command not found |
| AdolfVonKleist/Phonetisaurus | CONFIGURE_RUN_FAIL | ./configure: line 1390: syntax error near unexpected token `>' |
| affix/MSNC | CONFIGURE_RUN_FAIL | ./configure: line 1473: syntax error near unexpected token `location' |
| agn453/ZXCC | CONFIGURE_RUN_FAIL | ./configure: line 1422: syntax error near unexpected token `because' |
| ahmedammar/platform_external_gst_gstreamer | CONFIGURE_RUN_FAIL | ./configure: line 1339: AG_GST_INIT: command not found |
| ahupowerdns/setgrouper | CONFIGURE_RUN_FAIL | configure: error: no C++ compiler found |
| ajnelson/photorec-testdisk | CONFIGURE_RUN_FAIL | ./configure: line 1447: pthread-config: command not found |
| alanjaouen/compilateur-cpp | CONFIGURE_RUN_FAIL | awk: cmd. line:1:                               ^ syntax error |
| alaskacommunications/akcom-udpecho | CONFIGURE_RUN_FAIL | ./configure: line 1495: syntax error near unexpected token `0,' |
| albertok/web-polygraph | CONFIGURE_RUN_FAIL | configure: error: the compiler (g++) failed to pass a simple C++ test; check config.log fo |
| Albinlk/OpenThread | CONFIGURE_RUN_FAIL |  |
| alexmarsev/soundtouch | CONFIGURE_RUN_FAIL | ./configure: line 1670: syntax error near unexpected token `done' |
| alk/malloc-trace-replay | CONFIGURE_RUN_FAIL |  |
| allinurl/goaccess | CONFIGURE_RUN_FAIL | ./configure: line 1361: syntax error near unexpected token `to' |
| AlmuHS/GNUMach_SMP | CONFIGURE_RUN_FAIL | ./configure: line 1349: $'\f': command not found |
| AltSysrq/libbsd-minimal | CONFIGURE_RUN_FAIL | ./configure: line 1520: syntax error near unexpected token `F_CLOSEM,' |
| AmkG/hl | CONFIGURE_RUN_FAIL | checking how to get an intptr_t type with ranges... configure: error: We can't find out wh |
| andrewshadura/tnat64 | CONFIGURE_RUN_FAIL | configure: error: 'Could not find library containing connect()' |
| anewhuahua/bilitw | CONFIGURE_RUN_FAIL | ./configure: line 1885: syntax error near unexpected token `tar' |
| arbor/gzsig | CONFIGURE_RUN_FAIL | configure: error: OpenSSL not found |
| arthurdejong/nss-pam-ldapd | CONFIGURE_RUN_FAIL | ./configure: line 1351: syntax error near unexpected token `compat' |
| ARVE-Research/LPJ-LMfire | CONFIGURE_RUN_FAIL | ./configure: line 1377: syntax error near unexpected token `NETCDF_CC,' |
| asciinema/libtsm | CONFIGURE_RUN_FAIL | ./configure: line 1541: syntax error near unexpected token `GtkTsm' |
| asnelt/rrep | CONFIGURE_RUN_FAIL | configure: error: Invalid value for --with-included-regex: |
| aspiers/stow | CONFIGURE_RUN_FAIL | ./configure: line 1389: syntax error near unexpected token `[' |
| assaferan/omf5 | CONFIGURE_RUN_FAIL | ./configure: line 1377: is: command not found |
| aurelihein/exosip | CONFIGURE_RUN_FAIL | ./configure: line 1374: syntax error near unexpected token `scripts' |
| autotools-mirror/autoconf | CONFIGURE_RUN_FAIL | whether directories can have trailing spaces... for GNU M4 that supports accurate traces.. |
| avr-aics-riken/234Compositor | CONFIGURE_RUN_FAIL | ./configure: line 2252: syntax error near unexpected token `(' |
| avrdudes/avarice | CONFIGURE_RUN_FAIL | ./configure: line 1652: syntax error near unexpected token `fi' |
| b4n/ctpl | CONFIGURE_RUN_FAIL | ./configure: line 1402: syntax error near unexpected token `1.9' |
| badoo/libpssh | CONFIGURE_RUN_FAIL | checking libevent install prefix... configure: error: Can't find libevent headers under  d |
| balde/balde | CONFIGURE_RUN_FAIL | configure: error: no -fvisibility=hidden support found in , balde requires -fvisibility=hi |
| barak/djview4 | CONFIGURE_RUN_FAIL | conftest.d/conftest.sh: line 1: creating: command not found |
| barak/djvulibre | CONFIGURE_RUN_FAIL | ./configure: line 1413: syntax error near unexpected token `_WIN32,have_os_win32=yes,have_ |
| barak/oaklisp | CONFIGURE_RUN_FAIL | _AC_LANG_PREFIXFLAGS for maximum warnings... ./configure: line 1537: syntax error near une |
| baszoetekouw/pinfo | CONFIGURE_RUN_FAIL | ./configure: line 1429: syntax error near unexpected token `else' |
| bcoin-org/libtorsion | CONFIGURE_RUN_FAIL | configure: error: language C required |
| bdwgc/bdwgc | CONFIGURE_RUN_FAIL | ./configure: line 1574: syntax error near unexpected token `(' |
| benvanik/gflags | CONFIGURE_RUN_FAIL | ./configure: line 1715: pthread-config: command not found |
| benwbooth/tvision | CONFIGURE_RUN_FAIL | ./configure: line 1412: AC_STDC_HEADERS: command not found |
| bigmc/bigmc | CONFIGURE_RUN_FAIL | ./configure: line 1650: syntax error near unexpected token `fi' |
| bindle/rackgnome | CONFIGURE_RUN_FAIL | ./configure: line 1478: syntax error near unexpected token `0,' |
| binhqnguyen/ovs-srv6 | CONFIGURE_RUN_FAIL | configure: error: Cannot find openssl (use --disable-ssl to configure without SSL support) |
| BirolLab/ChopStitch | CONFIGURE_RUN_FAIL | configure: error: CHOPSTITCH must be compiled with a C++ compiler that supports OpenMP thr |
| BirolLab/ntCard | CONFIGURE_RUN_FAIL | configure: error: NTCARD must be compiled with a C++ compiler that supports OpenMP threadi |
| BlockstreamResearch/secp256k1-zkp | CONFIGURE_RUN_FAIL | configure: error: Set enable_dev_mode before calling SECP_SET_DEFAULT |
| bloomen/libunittest | CONFIGURE_RUN_FAIL | ./configure: line 1515: pthread-config: command not found |
| blueness/sthttpd | CONFIGURE_RUN_FAIL | ./configure: line 1363: syntax error near unexpected token `AR,' |
| bmanojlovic/log4c | CONFIGURE_RUN_FAIL | ./configure: line 1725: syntax error near unexpected token `1.95.1' |
| BoldingBruggeman/netcdf3 | CONFIGURE_RUN_FAIL | ./configure: line 1619: syntax error near unexpected token `fi' |
| BrianGladman/mpfr | CONFIGURE_RUN_FAIL | ./configure: line 1374: is: command not found |
| brianmcgillion/udev | CONFIGURE_RUN_FAIL | ./configure: line 1378: syntax error near unexpected token `1.10' |
| broadinstitute/VariantBam | CONFIGURE_RUN_FAIL | ./configure: line 1407: action: command not found |
| brunonymous/Powermanga | CONFIGURE_RUN_FAIL | ./configure: line 1414: AM_PATH_SDL: command not found |
| brynet/file | CONFIGURE_RUN_FAIL | ./configure: line 1513: syntax error near unexpected token `;' |
| BunsenLabs/plank | CONFIGURE_RUN_FAIL | ./configure: line 1462: GLIB_GSETTINGS: command not found |
| bytedance/ovs-dpdk | CONFIGURE_RUN_FAIL | configure: error: Cannot find openssl (use --disable-ssl to configure without SSL support) |
| cannabisday/ovs-tsn | CONFIGURE_RUN_FAIL | configure: error: Cannot find openssl (use --disable-ssl to configure without SSL support) |
| canonical/dqlite | CONFIGURE_RUN_FAIL | for joinable pthread attribute... whether more special flags are required for pthreads...  |
| cernekee/ocproxy | CONFIGURE_RUN_FAIL | ./configure: line 1437: syntax error near unexpected token `WFLAGS,' |
| cforall/cforall | CONFIGURE_RUN_FAIL | ./configure: line 1407: syntax error near unexpected token `DOifskipcompile' |
| chad3814/libhid | CONFIGURE_RUN_FAIL | ./configure: line 1702: syntax error near unexpected token `[$1' |
| chaos/pdsh | CONFIGURE_RUN_FAIL | ./configure: line 585: syntax error near unexpected token `(' |
| chaos/slurm | CONFIGURE_RUN_FAIL | ./configure: line 1348: X_AC_GPL_LICENSED: command not found |
| chenall/grub4dos | CONFIGURE_RUN_FAIL | configure: error: unsupported CPU type |
| chiphackers/covered | CONFIGURE_RUN_FAIL | ./configure: line 1485: COVERED_TCLTK: command not found |
| chuckleb/virt-what | CONFIGURE_RUN_FAIL |  |
| cisco/libamvp | CONFIGURE_RUN_FAIL | ./configure: line 1453: syntax error near unexpected token `fi' |
| cisco/opus | CONFIGURE_RUN_FAIL | ./configure: line 1397: AC_MINGW32: command not found |
| cjcole/libgolle | CONFIGURE_RUN_FAIL | ./configure: line 1519: DX_PDF_FEATURE: command not found |
| CkNoSFeRaTU/pidgin | CONFIGURE_RUN_FAIL | ./configure: line 1441: syntax error near unexpected token `__SUNPRO_C,' |
| claesenm/EnsembleSVM | CONFIGURE_RUN_FAIL | ./configure: line 1749: syntax error near unexpected token `fi' |
| cmand/yarrp | CONFIGURE_RUN_FAIL | ./configure: line 1646: pthread-config: command not found |
| coapp-packages/libunistring | CONFIGURE_RUN_FAIL | ./configure: line 1346: is: command not found |
| cockpit-project/cockpit | CONFIGURE_RUN_FAIL | configure: error: Couldn't find crypt library. Try installing glibc-devel |
| codecryptanalysis/mccl | CONFIGURE_RUN_FAIL | ./configure: line 1345: This: command not found |
| coin-or-tools/ThirdParty-ASL | CONFIGURE_RUN_FAIL | ./configure: line 1341: Copyright: command not found |
| coin-or/Cbc | CONFIGURE_RUN_FAIL | ./configure: line 1341: Copyright: command not found |
| ColumPaget/gngeo-cjp | CONFIGURE_RUN_FAIL | ./configure: line 1425: syntax error near unexpected token `$SDL_VERSION,,printf' |
| commandus/proto-sql | CONFIGURE_RUN_FAIL | configure: error: Could not find libprotobuf3. Try $ ./configure LDFLAGS='-Lyour-protobuf3 |
| cooljeanius/docbook-utils-0.6.14 | CONFIGURE_RUN_FAIL | ./configure: line 2108: syntax error near unexpected token `(' |
| cooljeanius/gcml2-0.7.1 | CONFIGURE_RUN_FAIL | checking for IMLIB - version >= 1.8.2... ./configure: line 1641: --cflags: command not fou |
| cooljeanius/libUnixToOSX | CONFIGURE_RUN_FAIL | Try 0 --help for more information.: syntax error in expression (error token is "Try 0 --he |
| cooljeanius/pkg-config | CONFIGURE_RUN_FAIL | ./configure: line 1338: and: command not found |
| coova/coova-chilli | CONFIGURE_RUN_FAIL | ./configure: line 1432: AC_LBL_TPACKET_STATS: command not found |
| corazawaf/libcoraza | CONFIGURE_RUN_FAIL | configure: error: Go |
| couchbaselabs/breakpad | CONFIGURE_RUN_FAIL | ./configure: line 1557: pthread-config: command not found |
| cowsql/raft | CONFIGURE_RUN_FAIL |  |
| cpputest/cpputest | CONFIGURE_GEN_FAIL |  |
| cpu-pool/cpuminer-opt-cpupower | CONFIGURE_RUN_FAIL | configure: error: OpenSSL crypto library required |
| cr-marcstevens/m4gb | CONFIGURE_RUN_FAIL | ./configure: line 1345: This: command not found |
| cryptozeny/cpuminer-opt-sugarchain | CONFIGURE_RUN_FAIL | configure: error: OpenSSL crypto library required |
| CS198NDSGChanBrianJoe/html5rdp | CONFIGURE_RUN_FAIL | ./configure: line 1538: syntax error near unexpected token `png_get_io_ptr,' |
| cschwan/hep-ga | CONFIGURE_RUN_FAIL |  |
| cslarsen/mickey-scheme | CONFIGURE_RUN_FAIL | configure: error: readline test failed (--without-readline to disable) |
| cwi-dis/ambulant | CONFIGURE_RUN_FAIL | configure: error: Your platform is not currently supported |
| cydhaselton/mono-android | CONFIGURE_RUN_FAIL | timeout: failed to run command ‘./configure’: No such file or directory |
| d99kris/namp-lite | CONFIGURE_RUN_FAIL | ./configure: line 1342: syntax error near unexpected token `2.0.0,,' |
| dajobe/librdf | CONFIGURE_RUN_FAIL | expr: syntax error: unexpected argument '10000' |
| Dale-M/mcron | CONFIGURE_RUN_FAIL | ./configure: line 1399: syntax error near unexpected token `3.0' |
| dankamongmen/babl | CONFIGURE_GEN_FAIL |  |
| danos/pam_tacplus | CONFIGURE_RUN_FAIL | ./configure: line 1646: syntax error near unexpected token `rt_debug_defines' |
| darrenjs/log2mem | CONFIGURE_RUN_FAIL |  |
| datacratic/gperftools | CONFIGURE_RUN_FAIL | configure: error: cannot find the nanosleep function |
| daveshields/jikes | CONFIGURE_RUN_FAIL | ./configure: line 1409: syntax error near unexpected token `macro' |
| davexunit/guile-2d | CONFIGURE_RUN_FAIL | ./configure: line 1348: GUILE_PROGS: command not found |
| DavidGriffith/hf | CONFIGURE_RUN_FAIL | ./configure: line 1735: syntax error near unexpected token `1.2.0,,' |
| delphix/nfs-utils | CONFIGURE_RUN_FAIL | ./configure: line 1506: syntax error near unexpected token `else' |
| descent/d2x | CONFIGURE_RUN_FAIL | ./configure: line 1446: AC_STDC_HEADERS: command not found |
| deskull-m/bakabakaband | CONFIGURE_RUN_FAIL | ./configure: line 1356: syntax error near unexpected token `game_libpath,' |
| detomon/json5 | CONFIGURE_RUN_FAIL | ./configure: line 1546: syntax error near unexpected token `unicode-table' |
| devicescape/aws_dynamo | CONFIGURE_RUN_FAIL | configure: error: no openssl; please install openssl or equivalent |
| devzero2000/POPT | CONFIGURE_RUN_FAIL | ./configure: line 1457: syntax error near unexpected token `-Wall,' |
| digitalocean/ovs | CONFIGURE_RUN_FAIL | ./configure: line 1593: syntax error near unexpected token `fi' |
| Distrotech/gtkimageview | CONFIGURE_RUN_FAIL | ./configure: line 1370: GNOME_COMMON_INIT: command not found |
| Distrotech/libcaca | CONFIGURE_RUN_FAIL | ./configure: line 1370: syntax error near unexpected token `OBJC' |
| Distrotech/pulseaudio | CONFIGURE_RUN_FAIL | configure: error: git-version-gen failed |
| djn3m0/debit | CONFIGURE_RUN_FAIL | ./configure: line 1378: AX_CHECK_ALIGNED_ACCESS_REQUIRED: command not found |
| dleonard0/pktstat | CONFIGURE_RUN_FAIL | checking for library containing socket... ./configure: line 1394: syntax error near unexpe |
| dmalhotra/pvfmm | CONFIGURE_RUN_FAIL | ./configure: line 1506: syntax error near unexpected token `and' |
| dmtx/dmtx-utils | CONFIGURE_RUN_FAIL | ./configure: line 1342: syntax error near unexpected token `common' |
| dmtx/dmtx-wrappers | CONFIGURE_RUN_FAIL | ./configure: line 1419: syntax error near unexpected token `phpize' |
| dreibh/sctplib | CONFIGURE_RUN_FAIL | ./configure: line 1447: syntax error near unexpected token `sys/time.h' |
| DrMcCoy/NWNTools | CONFIGURE_RUN_FAIL |  |
| drycpp/libposix | CONFIGURE_RUN_FAIL | ./configure: line 1348: syntax error near unexpected token `lib' |
| dyninc/OpenBFDD | CONFIGURE_RUN_FAIL | ./configure: line 1378: syntax error near unexpected token `Wall,' |
| e2guardian/e2guardian | CONFIGURE_RUN_FAIL |  |
| ecerulm/autotools-template | CONFIGURE_RUN_FAIL | ./configure: line 1715: syntax error near unexpected token `)' |
| echiu64/gutenprint | CONFIGURE_RUN_FAIL | ./configure: line 1348: STP_INIT: command not found |
| ederc/gb | CONFIGURE_RUN_FAIL | for OpenMP flag of _AC_LANG compiler... ./configure: line 1715: AX_LANG_COMPILER_MS: comma |
| edrosten/tag | CONFIGURE_RUN_FAIL | configure: error: TooN is not optional. Use --with-TooN=dir to specify where it can be fou |
| egallesio/STklos | CONFIGURE_RUN_FAIL | configure: error: Unknown thread system: |
| elbandi/lighttpd | CONFIGURE_RUN_FAIL | ./configure: line 1437: AM_C_PROTOTYPES: command not found |
| emcrisostomo/fswatch | CONFIGURE_GEN_FAIL |  |
| emuse/qmidiarp | CONFIGURE_RUN_FAIL | ./configure: line 1350: syntax error near unexpected token `ON' |
| endlessm/eos-shard | CONFIGURE_RUN_FAIL | ./configure: line 1356: syntax error near unexpected token `(' |
| enki/gvpe | CONFIGURE_RUN_FAIL | configure: error: cannot find install-sh, install.sh, or shtool in "." "./.." "./../.." |
| envytools/valgrind | CONFIGURE_RUN_FAIL | ./configure: line 1400: syntax error near unexpected token `(' |
| ester-project/ester | CONFIGURE_RUN_FAIL | ./configure: line 1524: --ldflags: command not found |
| evilpan/TurnServer | CONFIGURE_RUN_FAIL | ./configure: line 1649: syntax error near unexpected token `ON' |
| farsightsec/mtbl | CONFIGURE_RUN_FAIL | configure: error: liblz4 >= r130 required |
| fasseg/crumbs | CONFIGURE_RUN_FAIL | configure: error: Your platform is not currently supported |
| FauxFaux/fastjar | CONFIGURE_RUN_FAIL | Try `./configure --help for more information.: syntax error: invalid arithmetic operator ( |
| fengy-research/UCNTracker | CONFIGURE_RUN_FAIL | checking valac is at least version 0.7.6... ./configure: line 1545: : command not found |
| filebench/filebench | CONFIGURE_RUN_FAIL | checking for off64_t... checking for boolean_t... checking for u_longlong_t... checking fo |
| filosganga/libwurfl | CONFIGURE_RUN_FAIL | ./configure: line 1438: syntax error near unexpected token `,have_check="yes",' |
| fizx/parsley | CONFIGURE_RUN_FAIL | configure: error: could not find pcre |
| FNCS/fncs | CONFIGURE_RUN_FAIL | ./configure: line 1432: syntax error near unexpected token `ZMQ_LIBS,' |
| ForgotFun/wifidog | CONFIGURE_RUN_FAIL | ./configure: line 1488: syntax error near unexpected token `socket' |
| fourmond/dvdcopy | CONFIGURE_RUN_FAIL | configure: error: cannot link to dvdread |
| fripon/freeture | CONFIGURE_RUN_FAIL | ./configure: line 1376: syntax error near unexpected token `)' |
| fuxedo/fuxedo | CONFIGURE_RUN_FAIL |  |
| GabrielDosReis/open-axiom | CONFIGURE_RUN_FAIL | configure: error: OpenAxiom requires a Lisp system.  Either separately build one (GCL-2.6. |
| GalliumOS/xfce4-session | CONFIGURE_RUN_FAIL | ./configure: line 1482: AC_CHECK_LIBM: command not found |
| gavioto/fastdb | CONFIGURE_RUN_FAIL | checking whether compiling with debug options enabled... configure: error: invalid argumen |
| gbonacini/trollfs | CONFIGURE_RUN_FAIL | configure: error: could not find lib FUSE |
| gderosa/dansguardian | CONFIGURE_RUN_FAIL |  |
| gdnsd/gdnsd | CONFIGURE_RUN_FAIL | ./configure: line 1611: pthread-config: command not found |
| GenABEL-Project/ProbABEL | CONFIGURE_RUN_FAIL | ./configure: line 1411: syntax error near unexpected token `will' |
| geofft/sbuild | CONFIGURE_RUN_FAIL | ./configure: line 1348: syntax error near unexpected token `newline' |
| GerHobbelt/html2db | CONFIGURE_RUN_FAIL | ./configure: line 1394: syntax error near unexpected token `)' |
| giuseppe/containers-dedup | CONFIGURE_RUN_FAIL | ./configure: line 1405: syntax error near unexpected token `limits.h' |
| GNOME/goffice | CONFIGURE_RUN_FAIL | ./configure: command substitution: line 1638: syntax error near unexpected token `)' |
| golems/ach | CONFIGURE_RUN_FAIL | ./configure: line 1395: syntax error near unexpected token `PRIuPTR,' |
| goodspeed34/ws63flash | CONFIGURE_RUN_FAIL | configure: error: |
| gordonjcp/nekostring | CONFIGURE_RUN_FAIL | ./configure: line 1414: syntax error near unexpected token `2.0.0,' |
| gpudirect/libibverbs | CONFIGURE_RUN_FAIL | ./configure: line 1436: syntax error near unexpected token `;' |
| graydon/monotone | CONFIGURE_RUN_FAIL | ./configure: line 1361: syntax error near unexpected token `is' |
| graygnuorg/pound | CONFIGURE_RUN_FAIL | ./configure: line 1475: AC_TYPE_UNSIGNED_LONG_LONG_INT: command not found |
| grobian/carbon-c-relay | CONFIGURE_RUN_FAIL | ./configure: line 1505: syntax error near unexpected token `dispatch/dispatch.h' |
| groonga/groonga | CONFIGURE_RUN_FAIL | ./configure: line 2665: ./version.sh: No such file or directory |
| guardianproject/libsqlfs | CONFIGURE_RUN_FAIL | configure: error: --with-sqlcipher was given but test failed |
| gvvaughan/slingshot | CONFIGURE_RUN_FAIL | for a Lua interpreter with version >= 5.1, < 5.4... configure: error: cannot find suitable |
| hackerschoice/gsocket | CONFIGURE_RUN_FAIL | configure: error: libnet 1.1.x not found |
| HansWessels/gup | CONFIGURE_RUN_FAIL | ./configure: line 1354: GUP_CYGWIN: command not found |
| hermansr/psid64 | CONFIGURE_RUN_FAIL | ./configure: line 1465: AX_FUNC_MKDIR: command not found |
| HewlettPackard/netperf | CONFIGURE_RUN_FAIL | ./configure: line 1355: syntax error near unexpected token `src/missing' |
| hexagonal-sun/bic | CONFIGURE_RUN_FAIL | ./configure: line 1414: AX_LIB_READLINE: command not found |
| hgst/libnvme | CONFIGURE_RUN_FAIL | ./configure: line 1476: pthread-config: command not found |
| hholzgra/connector-c-examples | CONFIGURE_RUN_FAIL | checking for mysql_config executable... ./configure: line 2527: syntax error: unexpected e |
| hkerem/squid3-ssl | CONFIGURE_RUN_FAIL | ./configure: line 1387: again,: command not found |
| hnwfs/lighttpd-plus | CONFIGURE_RUN_FAIL | ./configure: line 1437: AM_C_PROTOTYPES: command not found |
| HomerReid/buff-em | CONFIGURE_RUN_FAIL | ./configure: line 1356: syntax error near unexpected token `LT_INIT__DISABLE-SHARED,' |
| hroptatyr/truffle | CONFIGURE_RUN_FAIL | ./configure: line 1384: syntax error near unexpected token `(' |
| hroptatyr/yuck | CONFIGURE_RUN_FAIL | ./configure: line 1384: syntax error near unexpected token `(' |
| HSU-ANT/gstpeaq | CONFIGURE_RUN_FAIL | ./configure: line 1474: syntax error near unexpected token `gst-launch' |
| idiap/juicer | CONFIGURE_RUN_FAIL | ./configure: line 1408: syntax error near unexpected token `Required' |
| iem-projects/ncview | CONFIGURE_RUN_FAIL | ./configure: line 1341: AC_PATH_NETCDF: command not found |
| igmhub/likely | CONFIGURE_RUN_FAIL | ./configure: line 1351: AX_EXT: command not found |

## ⚪ Non-working — not standalone (GNU autotools also fails; not our bug) (102)

| repo | stage |
| --- | --- |
| 315234/lyx-retina | CONFIGURE_GEN_FAIL |
| acaudwell/Logstalgia | CONFIGURE_RUN_FAIL |
| adjacentlink/emane-layer-dlep | CONFIGURE_RUN_FAIL |
| adobe-research/libkafka | CONFIGURE_RUN_FAIL |
| AegisEmu/AegisEmu | CONFIGURE_RUN_FAIL |
| aissammouche/bitextor | CONFIGURE_RUN_FAIL |
| ajnelson/regxml_extractor | CONFIGURE_RUN_FAIL |
| alexandervdm/gummi | CONFIGURE_RUN_FAIL |
| ant9000/libsigrok | CONFIGURE_RUN_FAIL |
| AnthonyDeroche/mod_authnz_jwt | CONFIGURE_RUN_FAIL |
| araisrobo/libwosi | CONFIGURE_RUN_FAIL |
| ArcticaProject/libpam-x2go | CONFIGURE_RUN_FAIL |
| arkadijs/asterisk-g72x | CONFIGURE_RUN_FAIL |
| arkq/bluez-alsa | CONFIGURE_RUN_FAIL |
| armenb/sharktools | CONFIGURE_RUN_FAIL |
| ayyi/samplecat | CONFIGURE_RUN_FAIL |
| bat/bat | CONFIGURE_RUN_FAIL |
| bbidulock/xde-sounds | CONFIGURE_RUN_FAIL |
| bert/geda-gaf | CONFIGURE_RUN_FAIL |
| bgpsecurity/rpstir | CONFIGURE_RUN_FAIL |
| BIC-MNI/bicpl | CONFIGURE_RUN_FAIL |
| BIMSBbioinfo/swineherd | CONFIGURE_RUN_FAIL |
| bitcoinbabys/flexinodes | CONFIGURE_RUN_FAIL |
| bjoernvoss/RNAHeliCes | CONFIGURE_RUN_FAIL |
| BOINC/boinc | CONFIGURE_RUN_FAIL |
| brimworks/zile | CONFIGURE_RUN_FAIL |
| bsc-pm/sonar | CONFIGURE_RUN_FAIL |
| bspeice/libcvautomation | CONFIGURE_RUN_FAIL |
| carboncointrust/CarboncoinCore | CONFIGURE_RUN_FAIL |
| CESARBR/knot-network-nrf24 | CONFIGURE_RUN_FAIL |
| cfra/quagga-testing | CONFIGURE_RUN_FAIL |
| cgdb/cgdb | CONFIGURE_RUN_FAIL |
| chabbimilind/cctlib | CONFIGURE_RUN_FAIL |
| choeger/MetaModelica-autotools | CONFIGURE_RUN_FAIL |
| christian-sahlmann/gwyddion | CONFIGURE_GEN_FAIL |
| ChrisVine/guile-a-sync | CONFIGURE_RUN_FAIL |
| cminyard/ser2net | CONFIGURE_RUN_FAIL |
| common-tools-interface/cti | CONFIGURE_RUN_FAIL |
| COMSYS/tor4iot-tor | CONFIGURE_RUN_FAIL |
| cpaasch/wireshark | CONFIGURE_RUN_FAIL |
| criort/libPRISM | CONFIGURE_RUN_FAIL |
| cruppstahl/upscaledb | CONFIGURE_RUN_FAIL |
| CryptVenture/BitMoneyV2 | CONFIGURE_RUN_FAIL |
| CyanogenMod/android_external_protobuf-c | CONFIGURE_RUN_FAIL |
| danos/frr | CONFIGURE_RUN_FAIL |
| davidgiven/libfirm | CONFIGURE_RUN_FAIL |
| DeNA/HandlerSocket-Plugin-for-MySQL | CONFIGURE_RUN_FAIL |
| detomastah/adwc | CONFIGURE_RUN_FAIL |
| DGCDev/digitalcoin | CONFIGURE_RUN_FAIL |
| dillo-browser/dillo | CONFIGURE_RUN_FAIL |
| dividendcash/DividendCash | CONFIGURE_RUN_FAIL |
| dkosmari/gtk-sfml | CONFIGURE_RUN_FAIL |
| dkosmari/Papaya-HUD | CONFIGURE_RUN_FAIL |
| dns-stats/hedgehog | CONFIGURE_RUN_FAIL |
| DreamSourceLab/DSLogic-fw | CONFIGURE_RUN_FAIL |
| dtbartle/cgminer-gc3355 | CONFIGURE_RUN_FAIL |
| dyne/Freecoin | CONFIGURE_RUN_FAIL |
| eeight/tdheap | CONFIGURE_RUN_FAIL |
| endlessm/eos-knowledge-lib | CONFIGURE_RUN_FAIL |
| endlessm/xapian-bridge | CONFIGURE_RUN_FAIL |
| equalitie/gnunet | CONFIGURE_GEN_FAIL |
| esrille/escudo | CONFIGURE_RUN_FAIL |
| etr/libhttpserver | CONFIGURE_RUN_FAIL |
| excamera/alfalfa | CONFIGURE_RUN_FAIL |
| farsightsec/dnstable | CONFIGURE_RUN_FAIL |
| finit-project/finit | CONFIGURE_RUN_FAIL |
| firehol/firehol | CONFIGURE_RUN_FAIL |
| fix8/fix8 | CONFIGURE_RUN_FAIL |
| flightaware/tclreadline | CONFIGURE_RUN_FAIL |
| flux-framework/flux-pmix | CONFIGURE_GEN_FAIL |
| flux-framework/flux-security | CONFIGURE_RUN_FAIL |
| fractalcoin/fractalcoin | CONFIGURE_RUN_FAIL |
| freeipa/bind-dyndb-ldap | CONFIGURE_RUN_FAIL |
| fries/android-external-openvpn | CONFIGURE_RUN_FAIL |
| FrodeSolheim/fs-uae | CONFIGURE_RUN_FAIL |
| gapcoin/gapcoin | CONFIGURE_RUN_FAIL |
| gingi/fastbit | CONFIGURE_RUN_FAIL |
| GNOME/dasher | CONFIGURE_RUN_FAIL |
| GNOME/easytag | CONFIGURE_RUN_FAIL |
| GNOME/gnumeric | CONFIGURE_RUN_FAIL |
| gobolinux/GoboHide | CONFIGURE_RUN_FAIL |
| golosio/NeuronGPU | CONFIGURE_RUN_FAIL |
| google/certificate-transparency | CONFIGURE_RUN_FAIL |
| google/hiba | CONFIGURE_RUN_FAIL |
| gpg/scute | CONFIGURE_RUN_FAIL |
| gramseyer/hotstuff | CONFIGURE_RUN_FAIL |
| grke/burp | CONFIGURE_RUN_FAIL |
| gssapi/mod_auth_gssapi | CONFIGURE_RUN_FAIL |
| gyoto/Gyoto | CONFIGURE_RUN_FAIL |
| HDFGroup/vol-log-based | CONFIGURE_RUN_FAIL |
| hello31337/BI-SGX | CONFIGURE_RUN_FAIL |
| hermet/enventor | CONFIGURE_RUN_FAIL |
| HewlettPackard/nagios-plugins-hpilo | CONFIGURE_RUN_FAIL |
| hfiguiere/libopenraw | CONFIGURE_RUN_FAIL |
| hgneng/ekho | CONFIGURE_RUN_FAIL |
| hightman/xunsearch | CONFIGURE_RUN_FAIL |
| hillstoneUnited/hillstoneUnited | CONFIGURE_RUN_FAIL |
| holzschu/lib-tex | CONFIGURE_RUN_FAIL |
| hongyi-zhao/lyx | CONFIGURE_RUN_FAIL |
| htrb/ngraph-gtk | CONFIGURE_RUN_FAIL |
| huceke/xine-lib-vaapi | CONFIGURE_GEN_FAIL |
| IBM/corosync-qdisk | CONFIGURE_RUN_FAIL |

