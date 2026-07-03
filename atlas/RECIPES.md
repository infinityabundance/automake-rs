# Atlas Recipes — working / non-working roster

Total **986** recipes. **Working (built end-to-end): 61** · non-working: 380 partial · 10 not-standalone · 535 failed.

"Working" means the full pipeline (autoreconf → configure → make) succeeded under the GNU-free toolchain. `quirk_dependent` needed an auto-applied quirk; `sealed` needed none.

## ✅ Working (61)

| repo | court |
| --- | --- |
| aadel112/libBlondie | quirk_dependent |
| aalex/oscsend | quirk_dependent |
| abrt/faf | quirk_dependent |
| aconchillo/guile-json | quirk_dependent |
| ademakov/Oroch | quirk_dependent |
| arcalex/racktk | quirk_dependent |
| ArcticaProject/lightdm-remote-session-arctica | quirk_dependent |
| arjunchitturi/htmlstreamparser | quirk_dependent |
| aspiers/stow | quirk_dependent |
| barak/vobcopy | quirk_dependent |
| BGI-shenzhen/LDBlockShow | quirk_dependent |
| BGI-shenzhen/PopLDdecay | quirk_dependent |
| bingmann/flex-bison-cpp-example | quirk_dependent |
| boundarydevices/devregs | quirk_dependent |
| bromanbro/taggins | quirk_dependent |
| bryteise/ister | quirk_dependent |
| charlescui/CBenchmark | quirk_dependent |
| Chipmaster/kirk | quirk_dependent |
| circulosmeos/gztool | quirk_dependent |
| clone/xml2 | quirk_dependent |
| commiyou/iniparser | quirk_dependent |
| compiz-reloaded/compiz-bcop | quirk_dependent |
| containers/oci-umount | quirk_dependent |
| CookieAvenger/Tiny-Manga-Downloader | quirk_dependent |
| crackpkcs12/crackpkcs12 | quirk_dependent |
| cybergarage/uhttp-cc | quirk_dependent |
| cybergarage/usql | quirk_dependent |
| darkbitsorg/guichan | quirk_dependent |
| DE-IBH/imvirt | quirk_dependent |
| df7cb/sdate | quirk_dependent |
| dmtx/dmtx-wrappers | quirk_dependent |
| dterweij/ndjbdns | quirk_dependent |
| EasyRPG/Tools | quirk_dependent |
| elmar/ldap-git-backup | quirk_dependent |
| emk/eshell | quirk_dependent |
| endlessm/eos-browser-tools | quirk_dependent |
| endlessm/gnome-user-docs | quirk_dependent |
| enki/libev | quirk_dependent |
| fasseg/crumbs | quirk_dependent |
| FlexW/tiger-compiler | quirk_dependent |
| flyinghead/ircd-hybrid | quirk_dependent |
| fumiyas/wcwidth-cjk | quirk_dependent |
| ganesh503/Asus-Aura | quirk_dependent |
| GArik/bash-completion | quirk_dependent |
| Geballin/PgBrowse | quirk_dependent |
| GENI-NSF/geni-tools | quirk_dependent |
| geofft/sbuild | quirk_dependent |
| giellalt/keyboard-olo | quirk_dependent |
| giellalt/template-shared-und | quirk_dependent |
| gizero/autotools-skeleton | quirk_dependent |
| glv2/bruteforce-luks | quirk_dependent |
| glv2/bruteforce-salted-openssl | quirk_dependent |
| glv2/bruteforce-wallet | quirk_dependent |
| gnaservicesinc/Challenge4Access | quirk_dependent |
| godsflaw/xor_toolkit | quirk_dependent |
| GodshadowLQH/usbview | quirk_dependent |
| greyltc/android_external_sshfs | quirk_dependent |
| gvallee/c_hello_world | quirk_dependent |
| hafslund/cc2531-sniffer | quirk_dependent |
| hanya/aobook-haiku | quirk_dependent |
| haozhangphd/QuantLib-noBoost-SWIG | quirk_dependent |

## 🟡 Non-working — partial (configure cleared, make failed) (380)

| repo | stage | first error |
| --- | --- | --- |
| 01micko/pup-volume-monitor | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| 0xADE1A1DE/AssemblyLine | MAKE_FAIL | /usr/bin/ld: cannot find -lassemblyline: No such file or directory |
| 4ZM/mfterm | MAKE_FAIL | make: *** No rule to make target 'libsp_a-spec_parser.h', needed by 'all'.  Stop. |
| 5GenCrypto/mife | MAKE_FAIL | make[2]: *** No rule to make target 'all'.  Stop. |
| 5nord/bison-example | MAKE_FAIL | main.c:2:10: fatal error: example/parser.h: No such file or directory |
| abhaykadam/vm | MAKE_FAIL | <command-line>: error: expected ',' or ';' before numeric constant |
| abihf/kamus | MAKE_FAIL | make[1]: --generate-dependencies: No such file or directory |
| abrt/abrt | MAKE_FAIL | ./../include/libabrt.h:17:10: fatal error: libreport/internal_libreport.h: No such file or |
| abrt/libreport | MAKE_FAIL | make[1]: *** No rule to make target 'all'.  Stop. |
| achernya/hesiod | MAKE_FAIL | hespwnam.c:116:5: error: 'struct passwd' has no member named 'pw_quota' |
| acidburn0zzz/spice-xpi | MAKE_FAIL | make[3]: *** No rule to make target 'nsISpicec.h', needed by 'all'.  Stop. |
| adapteva/epiphany-libs | MAKE_FAIL | e-server/src/ServerInfo.h:29:10: fatal error: epiphany-hal-data.h: No such file or directo |
| adjacentlink/emane-jammer-simple | MAKE_FAIL | make: *** No rule to make target '.spec', needed by 'all'.  Stop. |
| adjacentlink/emane-layer-dlep | MAKE_FAIL | dlepclientimpl.h:37:10: fatal error: emane/platformserviceprovider.h: No such file or dire |
| adsr/flow-tools | MAKE_FAIL | ftio.c:1717:50: error: expected ')' before 'PRIu32' |
| adulau/dcfldd | MAKE_FAIL | sys2.h:408:23: error: 'CHAR_MIN' undeclared here (not in a function) |
| afedchin/xbmc-addon-iptvsimple | MAKE_FAIL | g++: error: MAJOR.MINOR.MICRO: linker input file not found: No such file or directory |
| afrab/WSim | MAKE_FAIL | ../../arch/common/wsim_stdint.h:19:19: error: two or more data types in declaration specif |
| agronick/Relay | MAKE_FAIL | make[2]: *** [Makefile:596: relay] Error 1 |
| ahkok/bootchart | MAKE_FAIL | svg.c:92:85: error: 'VERSION' undeclared (first use in this function) |
| aizvorski/h264bitstream | MAKE_FAIL | cc: error: 0.2.0: linker input file not found: No such file or directory |
| alaskacommunications/akcom-udpecho | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| Albinlk/OpenThread | MAKE_FAIL | /bin/sh: 1: /scripts/mkversion: not found |
| alekstorm/tala | MAKE_FAIL | /tmp/atlasx_alekstorm__tala/s/src/sys/melder/melder.h:1012:59: error: ISO C++17 does not a |
| alexanderchuranov/Metaresc | MAKE_FAIL | make[2]: *** No rule to make target 'all'.  Stop. |
| alexlarsson/Glick2 | MAKE_FAIL | config.h:2:24: error: expected expression before '/' token |
| alexmarsev/soundtouch | MAKE_FAIL | ../../include/STTypes.h:142:14: error: #error "conflicting sample types defined" |
| alito/smallpotato | MAKE_FAIL | src/unches.c:193:44: error: 'VERSION' undeclared (first use in this function) |
| alk/malloc-trace-replay | MAKE_FAIL | make: *** No targets.  Stop. |
| alliedtelesis/apteryx-rest | MAKE_FAIL | make: p: No such file or directory |
| allinurl/goaccess | MAKE_FAIL | make[2]: *** No rule to make target 'all'.  Stop. |
| allinurl/gwsocket | MAKE_FAIL | src/gwsocket.c:45:10: fatal error: config.h: No such file or directory |
| alobbs/macchanger | MAKE_FAIL | main.c:160:33: error: 'VERSION' undeclared (first use in this function) |
| Alpacius/a6 | MAKE_FAIL | /usr/bin/ld: cannot find -la6: No such file or directory |
| alsaplayer/alsaplayer | MAKE_FAIL | make[1]: *** No rule to make target 'all'.  Stop. |
| AltSysrq/libbsd-minimal | MAKE_FAIL | /usr/include/errno.h:37:37: error: unknown type name '__THROW' |
| amadvance/advancecomp | MAKE_FAIL | rezip.cc:460:17: error: 'PACKAGE' was not declared in this scope; did you mean 'PACKAGE_UR |
| ampledata/ldsped | MAKE_FAIL | ldsped.h:50:10: fatal error: netax25/kernel_ax25.h: No such file or directory |
| anatol/google-coredumper | MAKE_FAIL | src/elfcore.c:52:10: fatal error: sys/sysctl.h: No such file or directory |
| anchor/filtergen | MAKE_FAIL | filtergen.c:187:20: error: 'PACKAGE' undeclared (first use in this function); did you mean |
| andre-martins/TurboParser | MAKE_FAIL | TaggerFeatures.cpp:(.text+0x80): undefined reference to `google::LogMessageFatal::LogMessa |
| andriymoroz/IES | MAKE_FAIL | platforms/common/packet/generic-rawsocket/fm_generic_rawsocket.c:441:33: error: 'UIO_MAXIO |
| AndyA/psips | MAKE_FAIL | /usr/bin/ld: cannot find -ltest-support: No such file or directory |
| AndyA/rfile | MAKE_FAIL | make[2]: tools/serializator.pl: Permission denied |
| andygrundman/libmediascan | MAKE_FAIL | mediascan.c:100:18: error: unknown type name 'AVCodecParser' |
| AnthonyBradford/optionmatrix | MAKE_FAIL | Makefile:228: *** Recursive variable 'CXXFLAGS' references itself (eventually).  Stop. |
| antiprism/mpd_oled | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| antonblanchard/kexec-lite | MAKE_FAIL | kexec_trampoline.S:58:2: error: #error no endianness defined! |
| antonblanchard/qtrace-tools | MAKE_FAIL | /tmp/atlasx_antonblanchard__qtrace-tools/s/htm/tests/basictest.c:37:(.text.startup+0x2e):  |
| apereo/mod_auth_cas | MAKE_FAIL | make[2]: *** No rule to make target 'mod_auth_cas.la.lo', needed by 'mod_auth_cas.la'.  St |
| araisrobo/libwosi | MAKE_FAIL | board.c:72:10: fatal error: pigpio.h: No such file or directory |
| arbor/gzsig | MAKE_FAIL | ssh.c:206:29: error: invalid use of incomplete typedef 'RSA' {aka 'struct rsa_st'} |
| arbruijn/d2x-xl | MAKE_FAIL | ../include/descent.h:1898:17: fatal error: OVR.h: No such file or directory |
| archiecobbs/libnbcompat | MAKE_FAIL | make[1]: *** No rule to make target '.libs/libnbcompat.la', needed by 'rmd160-test'.  Stop |
| archiecobbs/logwarn | MAKE_FAIL | state.c:58:41: error: 'PACKAGE' undeclared (first use in this function); did you mean 'PAC |
| archiecobbs/mtree-port | MAKE_FAIL | compare.c:186:54: error: 'struct stat' has no member named 'st_mtimespec' |
| ArcticaProject/libpam-x2go | MAKE_FAIL | Makefile.am.coverage:10: *** missing separator.  Stop. |
| armadito/armadito-av | MAKE_FAIL | stdpaths.c:64:23: error: 'LIBARMADITO_MODULES_PATH' undeclared (first use in this function |
| ARVE-Research/LPJ-LMfire | MAKE_FAIL | (.text+0x1b): undefined reference to `main' |
| asciinema/libtsm | MAKE_FAIL | src/gtktsm/gtktsm-terminal.c:32:10: fatal error: libtsm.h: No such file or directory |
| ashwinraghav/Cqual | MAKE_FAIL | /bin/sh: 1: emacs: not found |
| assaferan/omf5 | MAKE_FAIL | omf5.c:(.text.startup+0x44a): undefined reference to `test_greedy_overflow' |
| atinm/poker-eval | MAKE_FAIL | /tmp/atlasx_atinm__poker-eval/s/lib/mktab_basic.c:228:(.text.startup+0x38): undefined refe |
| audionuma/libtruepeak | MAKE_FAIL | cc: error: 0.1: linker input file not found: No such file or directory |
| aurelihein/exosip | MAKE_FAIL | eXtl_dtls.c:228:43: error: invalid use of incomplete typedef 'SSL' {aka 'struct ssl_st'} |
| autch/demucc | MAKE_FAIL | (.text+0x1b): undefined reference to `main' |
| avahi/nss-mdns | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| avr-aics-riken/JHPCN-DF | MAKE_FAIL | make: *** No targets specified and no makefile found.  Stop. |
| avr-aics-riken/Polylib4 | MAKE_FAIL | make: *** No targets specified and no makefile found.  Stop. |
| avrdudes/avarice | MAKE_FAIL | /bin/sh: 1: -DHAVE_CONFIG_H: not found |
| b/ION | MAKE_FAIL | ./ici/include/platform.h:279:10: fatal error: synch.h: No such file or directory |
| b4n/ctpl | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| Bader-Research/snap-graph | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| badoo/libpssh | MAKE_FAIL | task.c:113:65: error: 'LIBSSH2_LAST_IO_SEND' undeclared (first use in this function); did  |
| barak/djvulibre | MAKE_FAIL | Arrays.cpp:57:11: fatal error: config.h: No such file or directory |
| bat/bat | MAKE_FAIL | make[1]: *** No rule to make target '.includes//BCHistogramBase.h', needed by 'libBAT_rdic |
| BatchDrake/lfsrintruder | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| baxter104/fatx | MAKE_FAIL | /usr/include/fuse/fuse_common.h:33:2: error: #error Please add -D_FILE_OFFSET_BITS=64 to y |
| bdonlan/libofx | MAKE_FAIL | ofx_utilities.cpp:22:10: fatal error: ParserEventGeneratorKit.h: No such file or directory |
| beniz/hmdp | MAKE_FAIL | make[37488]: *** [Makefile:229: all-am] Killed |
| benlemasurier/stormfs | MAKE_FAIL | proxy.c:20:10: fatal error: glib.h: No such file or directory |
| benmwebb/dopewars | MAKE_FAIL | ../../src/network.h:32:10: fatal error: winsock2.h: No such file or directory |
| bergundy/IOQueue | MAKE_FAIL | ioqueue.h:44:10: fatal error: arrayqueue.h: No such file or directory |
| besm6/m20 | MAKE_FAIL | /tmp/atlasx_besm6__m20/s/as/sim.c:493:(.text+0xd0a): undefined reference to `sqrt' |
| bestouff/genext2fs | MAKE_FAIL | genext2fs.c:4351:28: error: expected ')' before 'VERSION' |
| BIC-MNI/minc | MAKE_FAIL | ./libsrc/minc.h:170:10: fatal error: hdf5.h: No such file or directory |
| bigmc/bigmc | MAKE_FAIL | make[1]: *** No rule to make target 'bgparser.hpp', needed by 'all'.  Stop. |
| bindle/rackgnome | MAKE_FAIL | Makefile:757: build-aux/makefile-version.am: No such file or directory |
| BioND/myrng | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| BirolLab/biobloom | MAKE_FAIL | g++: error: oost_cppflags: linker input file not found: No such file or directory |
| bisco/uftrace | MAKE_FAIL | prototype.c:16:10: fatal error: libdwarf.h: No such file or directory |
| bitcoin-core/minisketch | MAKE_FAIL | /usr/bin/ld: cannot find -lminisketch: No such file or directory |
| blakecaldwell/fluidmem | MAKE_FAIL | Makefile:545: *** missing separator.  Stop. |
| BlockstreamResearch/secp256k1-zkp | MAKE_FAIL | cc: error: _PKG_VERSION_MAJOR,: linker input file not found: No such file or directory |
| bloq/cpptrade | MAKE_FAIL | srvapi.cc:13:10: fatal error: evhtp.h: No such file or directory |
| blueness/fts-standalone | MAKE_FAIL | cc: error: 0.2: linker input file not found: No such file or directory |
| bmanojlovic/log4c | MAKE_FAIL | rc.c:349:20: error: 'VERSION' undeclared (first use in this function) |
| boundary/libdnet | MAKE_FAIL | addr.c:298:25: error: 'struct sockaddr_in6' has no member named 'sin6_len' |
| bpeel/prevodb | MAKE_FAIL | make[1]: *** No rule to make target 'all'.  Stop. |
| braice/MuMuDVB | MAKE_FAIL | log.c:76:10: fatal error: libdvben50221/en50221_errno.h: No such file or directory |
| briansorahan/libchuck | MAKE_FAIL | util_network.c:42:10: fatal error: winsock.h: No such file or directory |
| BroadbandForum/obuspa | MAKE_FAIL | (.text+0x1b): undefined reference to `main' |
| broadinstitute/VariantBam | MAKE_FAIL | make: *** [Makefile:445: all-recursive] Error 1 |
| brunonymous/Powermanga | MAKE_FAIL | (.text+0x1b): undefined reference to `main' |
| brynet/file | MAKE_FAIL | file.c:35:10: fatal error: imsg.h: No such file or directory |
| bsc-performance-tools/paraver-kernel | MAKE_FAIL | filtermanagement.cpp:26:10: fatal error: kfilter.h: No such file or directory |
| bubbapizza/GCAM | MAKE_FAIL | ../libgcode/gcode_svg.h:19:10: fatal error: glib.h: No such file or directory |
| BUILDS-/Derpnet | MAKE_FAIL | src/sockserv/Connection.cc:72:24: error: 'read' was not declared in this scope; did you me |
| bvanheu/libsigrok-ad | MAKE_FAIL | sched.c:20:10: fatal error: glib.h: No such file or directory |
| bytedeco/helloworld | MAKE_FAIL | cc: error: 2.0: linker input file not found: No such file or directory |
| Cacti/spine | MAKE_FAIL | common.h:66:10: fatal error: config/config.h: No such file or directory |
| CAIDA/libparsebgp | MAKE_FAIL | ../lib/parsebgp.h:41:35: error: 'PKG_MAJOR_VERSION' undeclared (first use in this function |
| carlos-lopez-garces/mapnik | MAKE_FAIL | make: *** No targets specified and no makefile found.  Stop. |
| catharsis/spotifile | MAKE_FAIL | /bin/sh: 1: Syntax error: "(" unexpected |
| cdevelop/libquickmail | MAKE_FAIL | /tmp/atlasx_cdevelop__libquickmail/s/quickmailprog.c:131:(.text.startup+0x33): undefined r |
| cdobrich/btnx | MAKE_FAIL | Package libdaemon was not found in the pkg-config search path. |
| cea-hpc/bridge | MAKE_FAIL | xmessage.c:28:10: fatal error: rpc/types.h: No such file or directory |
| cea-hpc/robinhood | MAKE_FAIL | make: *** No targets specified and no makefile found.  Stop. |
| cea-hpc/selFIe | MAKE_FAIL | selfie_papi.c:33:10: fatal error: papi.h: No such file or directory |
| cebix/psximager | MAKE_FAIL | psxbuild.cpp:24:10: fatal error: cdio/iso9660.h: No such file or directory |
| cederom/LibSWD | MAKE_FAIL | aminclude.am:35: *** missing separator.  Stop. |
| ceph/gf-complete | MAKE_FAIL | gf_method.c:17:10: fatal error: gf_complete.h: No such file or directory |
| cernekee/ocproxy | MAKE_FAIL | /usr/bin/ld: cannot find FLAGS: No such file or directory |
| cforall/cforall | MAKE_FAIL | make: *** No targets.  Stop. |
| cfsghost/mdsfs | MAKE_FAIL | make[1]: *** No rule to make target 'all'.  Stop. |
| chad3814/libhid | MAKE_FAIL | hid_opening.c:127:47: error: '%s' directive output may be truncated writing up to 4096 byt |
| chaos/cerebro | MAKE_FAIL | make[1]: *** No rule to make target 'all'.  Stop. |
| chaoskagami/corbenik | MAKE_FAIL | make[2]: *** No targets specified and no makefile found.  Stop. |
| chenbd/libwfd | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| chenbd/miracle | MAKE_FAIL | src/wifi/wifid-dbus.c:754:13: error: too few arguments to function 'sd_bus_add_object_vtab |
| cheshire-mouse/hexchat-indicator | MAKE_FAIL | indicator.c:25:10: fatal error: messaging-menu.h: No such file or directory |
| chokkan/liblbfgs | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| chrisidefix/nurbs | MAKE_FAIL | error.cpp:45:1: error: redefinition of 'PLib::Error::Error(const char*)' |
| ChrisLidbury/CLSmith | MAKE_FAIL | OutputMgr.cpp:266:44: error: 'PACKAGE_STRING' was not declared in this scope |
| christoph-cullmann/xblast | MAKE_FAIL | (.text+0x1b): undefined reference to `main' |
| Chronic-Dev/libirecovery | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| CICM/HoaLibrary-PD | MAKE_FAIL | ../hoa.library.hpp:12:10: fatal error: ThirdParty/CicmWrapper/Sources/cicm_wrapper.h: No s |
| cisco/libamvp | MAKE_FAIL | cc: error: libamvp_major_version.libamvp_minor_version.libamvp_micro_version: linker input |
| cisco/open-nFAPI | MAKE_FAIL | src/debug.c:30:10: fatal error: debug.h: No such file or directory |
| claesenm/approxsvm | MAKE_FAIL | /usr/bin/ld: cannot open output file bin/approx-svm: No such file or directory |
| claesenm/EnsembleSVM | MAKE_FAIL | make: *** No rule to make target '..//svm.cpp', needed by 'src/libsvm/svm.cpp'.  Stop. |
| CLowcay/wayland-terminal | MAKE_FAIL | /bin/sh: 1: ../wayland/scanner: not found |
| clsync/clsync | MAKE_FAIL | glibex.h:20:10: fatal error: glib.h: No such file or directory |
| cmbi/dssp | MAKE_FAIL | /usr/include/boost/math/tools/config.hpp:23:6: error: #warning "The minimum language stand |
| cmbi/hssp | MAKE_FAIL | make: mtrx/mkmat_h.pl: Permission denied |
| cmcqueen/aes-min | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| cmusatyalab/vmnetx | MAKE_FAIL | /tmp/atlasx_cmusatyalab__vmnetx/s/vmnetfs/vmnetfs.c:418:(.text+0xd): undefined reference t |
| codebutler/firesheep | MAKE_FAIL | cp: cannot stat './backend/backend': No such file or directory |
| ColumPaget/gngeo-cjp | MAKE_FAIL | scanline.c:6:10: fatal error: SDL.h: No such file or directory |
| commandus/proto-sql | MAKE_FAIL | sql_code_generator.cpp:107:27: error: 'scoped_ptr' is not a member of 'google::protobuf' |
| coova/coova-chilli | MAKE_FAIL | mssl.h:46:3: error: unknown type name 'ssl_t' |
| cosimoc/gnome-example-search-provider | MAKE_FAIL | gnome-search-example.c:8:10: fatal error: search-example-provider-generated.h: No such fil |
| cowsql/cowsql | MAKE_FAIL | cc: error: 1.15.9: linker input file not found: No such file or directory |
| cpichard/fission | MAKE_FAIL | engine/ComputeEngine.cpp:11:10: fatal error: llvm/Analysis/Verifier.h: No such file or dir |
| cpptest/cpptest | MAKE_FAIL | collectoroutput.cpp:30:11: fatal error: config.h: No such file or directory |
| cpputest/cpputest_simulated_gmock | MAKE_FAIL | /usr/include/CppUTestExt/GMock.h:40:10: fatal error: gmock/gmock.h: No such file or direct |
| cprados/towitoko-linux | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| cpu-pool/cpuminer-opt-cpupower | MAKE_FAIL | algo/blake/sph-blake2s.c:326:9: error: size of array element is not a multiple of its alig |
| cr-marcstevens/m4gb | MAKE_FAIL | src/config.hpp:28:10: fatal error: ../lib/config.h: No such file or directory |
| cryptozeny/cpuminer-opt-sugarchain | MAKE_FAIL | algo/blake/sph-blake2s.c:326:9: error: size of array element is not a multiple of its alig |
| cschwan/hep-ga | MAKE_FAIL | /usr/bin/ld: cannot find libgtest.a: No such file or directory |
| csete/gpredict | MAKE_FAIL | make: *** No rule to make target '.version', needed by 'all'.  Stop. |
| CumulusNetworks/ptm | MAKE_FAIL | ptm_conf.h:45:19: error: duplicate 'const' declaration specifier [-Werror=duplicate-decl-s |
| cybergarage/mupnp-cc | MAKE_FAIL | ../../include/mupnp/util/Vector.h:25:41: error: 'mupnp_shared_ptr' was not declared in thi |
| cybergarage/uecho | MAKE_FAIL | /tmp/atlasx_cybergarage__uecho/s/examples/controller/uechosearch/unix/../uechosearch.c:30: |
| CyberNinjas/pam_aad | MAKE_FAIL | pam_aad.c:4:10: fatal error: sds/sds.h: No such file or directory |
| cybernoid/archivemount | MAKE_FAIL | archivemount.c:223:70: error: 'VERSION' undeclared (first use in this function); did you m |
| daghovland/clp | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| dahlem/lz-prediction | MAKE_FAIL | /tmp/atlasx_dahlem__lz-prediction/s/src/CL.cc:47:(.text+0x4c): undefined reference to `boo |
| damien-lemoal/zonefs-tools | MAKE_FAIL | (.text+0x1b): undefined reference to `main' |
| dancor/wmctrl | MAKE_FAIL | main.c:37:10: fatal error: glib.h: No such file or directory |
| dangerousben/jsonval | MAKE_FAIL | main.c:33:57: error: 'PACKAGE' undeclared (first use in this function); did you mean 'PACK |
| DanielO/codec2 | MAKE_FAIL | /usr/bin/ld: cannot find -lcodec2: No such file or directory |
| danielver02/NHDS | MAKE_FAIL | (.text+0x1b): undefined reference to `main' |
| danos/pam_tacplus | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| darrenjs/log2mem | MAKE_FAIL | /usr/bin/ld: cannot find -llog2mem: No such file or directory |
| darshan-hpc/darshan | MAKE_FAIL | make[1]: *** No rule to make target 'all'.  Stop. |
| davemichael/NaCl-Quake | MAKE_FAIL | make[1]: *** No rule to make target 'd_copy.o', needed by 'sdlquake'.  Stop. |
| DavidGriffith/hf | MAKE_FAIL | /tmp/atlasx_DavidGriffith__hf/s/hfkernel/main.c:311:(.text+0xa85): undefined reference to  |
| davidsiaw/luacppinterface | MAKE_FAIL | luacppinclude.h:6:10: fatal error: lua.h: No such file or directory |
| davidt/Fyre | MAKE_FAIL | remote-client.h:36:10: fatal error: gnet.h: No such file or directory |
| dbadapt/mutrace | MAKE_FAIL | config.h:23:17: error: two or more data types in declaration specifiers |
| dbcode/protobuf-nginx | MAKE_FAIL | ngx_generate.cc:21:3: error: 'scoped_ptr' was not declared in this scope |
| dbmail/dbmail | MAKE_FAIL | make: *** No targets.  Stop. |
| dcjones/hat-trie | MAKE_FAIL | cc: error: 0.1.2: linker input file not found: No such file or directory |
| DE-IBH/apt-dater | MAKE_FAIL | make[1]: *** No rule to make target 'apt-dater.xml.inc', needed by 'all'.  Stop. |
| deepin-community/libtextwrap | MAKE_FAIL | /usr/bin/ld: cannot find -ltextwrap: No such file or directory |
| deepin-community/motif | MAKE_FAIL | makestrs.c:31:10: fatal error: config.h: No such file or directory |
| dell/libsmbios | MAKE_FAIL | make[1]: *** No rule to make target 'all'.  Stop. |
| demorest/psrfits_utils | MAKE_FAIL | cc: error: 1.0: linker input file not found: No such file or directory |
| derino/schencon | MAKE_FAIL | ILPScheduler.h:38:10: fatal error: ilcplex/ilocplex.h: No such file or directory |
| descent/d2x | MAKE_FAIL | ../include/gr.h:22:2: error: #error foo |
| desrt/systemd-shim | MAKE_FAIL | cgmanager.h:26:10: fatal error: glib.h: No such file or directory |
| devernay/glm | MAKE_FAIL | glmimg_sim.c:9:10: fatal error: simage.h: No such file or directory |
| dhvani-tts/dhvani-tts | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| diixo/dbus | MAKE_FAIL | Makefile:531: ../../aminclude_static.am: No such file or directory |
| Distrotech/ethtool | MAKE_FAIL | ethtool.c:181:25: error: 'PACKAGE' undeclared (first use in this function); did you mean ' |
| Distrotech/flux | MAKE_FAIL | ../config.h:1:26: error: too many decimal points in number |
| Distrotech/gnome-mines | MAKE_FAIL | Makefile:16: *** missing separator.  Stop. |
| Distrotech/libaccounts-glib | MAKE_FAIL | /tmp/atlasx_Distrotech__libaccounts-glib/s/build-aux/missing: line 81: automake-1.14: comm |
| Distrotech/libart | MAKE_FAIL | /bin/sh: 1: ./gen_art_config.sh: Permission denied |
| Distrotech/libjpeg-turbo | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| Distrotech/onig | MAKE_FAIL |  |
| Distrotech/popt | MAKE_FAIL | /tmp/atlasx_Distrotech__popt/s/missing: line 52: automake-1.9: command not found |
| Distrotech/psmisc | MAKE_FAIL | /tmp/atlasx_Distrotech__psmisc/s/config/missing: line 81: automake-1.13: command not found |
| Distrotech/radius | MAKE_FAIL | rscm_hash.c:131:15: error: too few arguments to function 'scm_i_make_string' |
| divVerent/s2tc | MAKE_FAIL | g++: error: 0.1: linker input file not found: No such file or directory |
| djandruczyk/eXtace | MAKE_FAIL | ../include/input.h:22:10: fatal error: esd.h: No such file or directory |
| dkosmari/gtk-sfml | MAKE_FAIL | examples/application-window.cpp:1:10: fatal error: SFML/Graphics.hpp: No such file or dire |
| dmatveev/libinotify-kqueue | MAKE_FAIL | ./compat.h:37:10: fatal error: sys/tree.h: No such file or directory |
| dmtx/dmtx-utils | MAKE_FAIL | /tmp/atlasx_dmtx__dmtx-utils/s/dmtxquery/dmtxquery.c:104:(.text.startup+0x100): undefined  |
| dora38/sshpass | MAKE_FAIL | config.h:19:25: error: 'assword' undeclared (first use in this function) |
| dorchard/flrc-lib | MAKE_FAIL |  |
| dpc/xmppconsole | MAKE_FAIL | src/li.c:4:10: fatal error: lua.h: No such file or directory |
| Dr-Shadow/netsoul-purple | MAKE_FAIL | Makefile:45: *** missing separator.  Stop. |
| drakkar-lig/ipv6-care | MAKE_FAIL |  |
| dreamlayers/synaesthesia | MAKE_FAIL | main.cc:277:1: error: 'DWORD' does not name a type |
| dreamlegacy/libusbtuner | MAKE_FAIL |  |
| Drive-Trust-Alliance/sedutil | MAKE_FAIL | /tmp/atlasx_Drive-Trust-Alliance__sedutil/s/Common/sedutil.cpp:38:(.text+0x25): undefined  |
| DrMcCoy/NWNTools | MAKE_FAIL | NwnDefines.h:127:17: error: 'strcasecmp' was not declared in this scope; did you mean 'wcs |
| drmingdrmer/lrc-erasure-code | MAKE_FAIL | cc: error: 1.0: linker input file not found: No such file or directory |
| drycpp/libposix | MAKE_FAIL | /usr/include/c++/13/cstdio:99:11: error: 'fpos_t' has not been declared in '::' |
| dsigma/dfu-util | MAKE_FAIL | suffix.c:59:42: error: 'PACKAGE' undeclared (first use in this function); did you mean 'PA |
| dun/conman | MAKE_FAIL | src/common.h:132:13: error: conflicting types for 'socklen_t'; have 'int' |
| duosecurity/duo_unix | MAKE_FAIL | ../config.h:2:24: error: 'x86_64' undeclared (first use in this function) |
| dupgit/fcl | MAKE_FAIL | fcl.c:42:10: fatal error: fcl.h: No such file or directory |
| dwest/grip | MAKE_FAIL | make[1]: *** No rule to make target 'all'.  Stop. |
| dylex/xtmux | MAKE_FAIL | <command-line>: error: invalid suffix "ax" on floating constant |
| eamonnmoloney/thrift-0.6.1 | MAKE_FAIL |  |
| eantcal/miptknzr | MAKE_FAIL | main.cc:30:10: fatal error: mip_unicode.h: No such file or directory |
| EarthScope/evalresp | MAKE_FAIL | /usr/bin/ld: cannot find -levalresp: No such file or directory |
| ecerulm/autotools-template | MAKE_FAIL | make: *** No targets.  Stop. |
| eddelbuettel/dieharder | MAKE_FAIL | make: *** No rule to make target 'libwulf.time', needed by 'all'.  Stop. |
| ederc/gb | MAKE_FAIL | cc: error: unrecognized command-line option '-no-install' |
| ederc/gbla | MAKE_FAIL | /usr/bin/ld: ./.libs/libgbla.so: undefined reference to `omp_unset_lock' |
| ederlf/horse | MAKE_FAIL | make[1]: *** No rule to make target '.libs/libhorse.a', needed by 'horse'.  Stop. |
| egallesio/STklos | MAKE_FAIL | misc.c:208:29: error: 'VERSION' undeclared (first use in this function); did you mean 'C_V |
| eklitzke/spv | MAKE_FAIL | ./addr.h:48:21: error: 'string' in namespace 'std' does not name a type |
| elima/FileTea | MAKE_FAIL | filetead-main.c:23:10: fatal error: evd.h: No such file or directory |
| elima/gjs-commonjs | MAKE_FAIL | /usr/include/mozjs-115/mozilla/AlreadyAddRefed.h:12:10: fatal error: utility: No such file |
| Elive/engage | MAKE_FAIL | e_mod_main.h:4:10: fatal error: e.h: No such file or directory |
| elmo2k3/had | MAKE_FAIL | mpd.h:33:10: fatal error: libmpd/libmpd.h: No such file or directory |
| elmo2k3/libhagraph | MAKE_FAIL | libhagraph_data.c:263:25: error: 'MYSQL' {aka 'struct st_mysql'} has no member named 'reco |
| emanueleaina/desktop-notifications-browser-extension | MAKE_FAIL | make[1]: *** No rule to make target 'all'.  Stop. |
| embtom/kmscube | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| EmilGedda/Leonardo | MAKE_FAIL | /tmp/atlasx_EmilGedda__Leonardo/s/src/main.cpp:16:(.text+0x159): undefined reference to `l |
| emscripten-ports/libpng | MAKE_FAIL | make: *** No rule to make target 'pnglibconf.out', needed by 'pnglibconf.h'.  Stop. |
| Emulators-Salvacam/openjazz | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| emuse/qmidiarp | MAKE_FAIL | aminclude.am:35: *** missing separator.  Stop. |
| endaaman/tym | MAKE_FAIL | /tmp/atlasx_endaaman__tym/s/src/builtin.c:578:(.text+0x1d0b): undefined reference to `lua_ |
| endlessm/xapian-bridge | MAKE_FAIL | make: *** No targets.  Stop. |
| endocode/connman | MAKE_FAIL | /bin/sh: 1: /tmp/atlasx_endocode__connman/s/include/log.h: Permission denied |
| energicryptocurrency/gen2-energihash | MAKE_FAIL | cc: error: 1.23.0: linker input file not found: No such file or directory |
| Enough-Software/pcap-http-analyzer | MAKE_FAIL | print.h:26:10: fatal error: json-glib/json-glib.h: No such file or directory |
| ericherman/libfastset | MAKE_FAIL | /usr/bin/ld: cannot find -lfastset: No such file or directory |
| eriknyquist/librxvm | MAKE_FAIL | rxvm.c:(.text+0x2b8): undefined reference to `lfix_to_str' |
| esden/sigrok | MAKE_FAIL | sigrok-cli.c:93:55: error: 'VERSION' undeclared (first use in this function) |
| esrille/esidl | MAKE_FAIL | make: *** No rule to make target 'lexer.cc', needed by 'all'.  Stop. |
| essej/freqtweak | MAKE_FAIL | FTmainwin.cpp:26:10: fatal error: wx/wxprec.h: No such file or directory |
| essej/sooperlooper | MAKE_FAIL | make[1]: *** No rule to make target 'all'.  Stop. |
| eurovibes/mixmaster | MAKE_FAIL | menu.c:246:64: error: 'VERSION' undeclared (first use in this function) |
| EvaisaDev/libwebsock | MAKE_FAIL | Makefile:16: *** missing separator.  Stop. |
| evilpan/TurnServer | MAKE_FAIL | /usr/include/features.h:196:3: error: #warning "_BSD_SOURCE and _SVID_SOURCE are deprecate |
| EvolBioInf/andi | MAKE_FAIL | esa.h:9:10: fatal error: config.h: No such file or directory |
| Evolix/uvrrpd | MAKE_FAIL | vrrp_ip4.c:183:17: error: converting a packed 'struct vrrphdr' pointer (alignment 1) to a  |
| ewxrjk/sftpserver | MAKE_FAIL | utils.h:180:39: error: expected declaration specifiers before 'attribute' |
| exo/tvm-rcx | MAKE_FAIL | /tmp/ccvAGmbC.s:10: Error: no such instruction: `r6 saved by ROM' |
| FabricAttachedMemory/libfam-atomic | MAKE_FAIL | Makefile:605: git.mk: No such file or directory |
| fan31415/bbq_new | MAKE_FAIL | Makefile:16: *** missing separator.  Stop. |
| fancybits/libdvbpsi | MAKE_FAIL | dvbinfo.c:704:28: error: 'LOG_PID' undeclared (first use in this function) |
| farsightsec/dnstable | MAKE_FAIL | dnstable/dnstable-private.h:48:10: fatal error: wdns.h: No such file or directory |
| farsightsec/dnstable-convert | MAKE_FAIL | dnstable_convert.c:34:10: fatal error: dnstable.h: No such file or directory |
| fasrc/slurm_showq | MAKE_FAIL | slurm_showq.h:64:10: fatal error: slurm/slurm.h: No such file or directory |
| fblomqvi/librs | MAKE_FAIL | cc: error: 0.2: linker input file not found: No such file or directory |
| fbx/foils_hid | MAKE_FAIL | ../include/foils/rudp_hid_client.h:25:10: fatal error: rudp/client.h: No such file or dire |
| feeblefakie/luxio | MAKE_FAIL | data.h:311:26: error: call of overloaded 'div(uint32_t&, uint32_t&)' is ambiguous |
| fengy-research/UCNTracker | MAKE_FAIL | make[1]: *** No rule to make target 'vala-doc', needed by 'all'.  Stop. |
| ffromani/vmon | MAKE_FAIL | <command-line>: error: 'vmon' undeclared (first use in this function) |
| filebench/filebench | MAKE_FAIL | vars.h:60:17: error: unknown type name 'boolean_t' |
| filiphanes/fts-elastic | MAKE_FAIL | /bin/sh: 1: --silent: not found |
| filosganga/libwurfl | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| FinchBerryOS/fbyo-utils | MAKE_FAIL | make: *** No targets specified and no makefile found.  Stop. |
| fireae/nhocr | MAKE_FAIL | <command-line>: error: expected primary-expression before '/' token |
| firecore/libudfread | MAKE_FAIL | cc: error: unrecognized command-line option '-Wpointer-arith-Wredundant-decls' |
| firnsy/yubipam | MAKE_FAIL | ../../config.h:1:23: error: expected expression before '/' token |
| flatcar/sysroot-wrappers | MAKE_FAIL | /tmp/atlasx_flatcar__sysroot-wrappers/s/src/cc_wrap.c:37:(.text.startup+0x16): undefined r |
| Floobits/diffshipper | MAKE_FAIL | src/dmp_lua.c:12:10: fatal error: dmp_lua_str.h: No such file or directory |
| flyfeifan/myHtop | MAKE_FAIL | Makefile:16: *** missing separator.  Stop. |
| fontforge/libuninameslist | MAKE_FAIL | /bin/sh: 1: --trace: not found |
| fordmason/cronolog | MAKE_FAIL | cronoutils.c:502:17: error: storage size of 'tm_initial' isn't known |
| ForgotFun/wifidog | MAKE_FAIL | commandline.c:129:66: error: expected ')' before 'VERSION' |
| frank-zago/xgalaga-sdl | MAKE_FAIL | frate.c:25:10: fatal error: SDL_framerate.h: No such file or directory |
| fredowski/ssw | MAKE_FAIL | Makefile:16: *** missing separator.  Stop. |
| fredrikwidlund/cfarmhash | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| fredrikwidlund/libclo | MAKE_FAIL | <artificial>:(.text.startup+0xa7): undefined reference to `clo_encode' |
| fredrikwidlund/libdynamic | MAKE_FAIL | cc: error: unrecognized command-line option '-std=gnu23'; did you mean '-std=gnu2x'? |
| fredrikwidlund/libdynamic_benchmark | MAKE_FAIL | src/map_google_densehash.cpp:7:10: fatal error: sparsehash/dense_hash_map: No such file or |
| fredrikwidlund/libreactorng | MAKE_FAIL | /usr/bin/ld: cannot find libreactor_test.a: No such file or directory |
| freedesktop-unofficial-mirror/gstreamer-sdk__dbus | MAKE_FAIL | dbus-test-main.c:(.text+0x6e): undefined reference to `dbus_internal_do_not_use_run_tests' |
| fremantle-gtk2/osso-applet-screencalibration | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| frobnitzem/libdag | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| FrodeSolheim/fs-uae | MAKE_FAIL | ./od-fs/FS/FS.h:4:10: fatal error: SDL3/SDL.h: No such file or directory |
| ftynse/ppcg-fb | MAKE_FAIL | ppcg.h:9:10: fatal error: pet.h: No such file or directory |
| FuangCao/cavan | MAKE_FAIL | (.text+0x1b): undefined reference to `main' |
| fuxedo/fuxedo | MAKE_FAIL | g++: error: 0.0.1: linker input file not found: No such file or directory |
| gabrielfalcao/go-horse | MAKE_FAIL | cc: error: unrecognized command-line option '-fnested-functions'; did you mean '-Wunused-f |
| GalliumOS/lxdm | MAKE_FAIL | ../config.h:15:26: error: expected expression before '/' token |
| GaloisInc/gghlite-flint | MAKE_FAIL | make: *** No targets.  Stop. |
| gass/dbpc-test | MAKE_FAIL | /tmp/atlasx_gass__dbpc-test/s/dbpc-server/dbpc_dbus.c:76:(.text+0x34): undefined reference |
| gat3way/hashkill | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| gavioto/fastdb | MAKE_FAIL | examples/testtimeseries.cpp:50:35: error: 'float fmax(float, float)' conflicts with a prev |
| gcp/opusfile | MAKE_FAIL | src/http.c:1521:5: error: invalid use of incomplete typedef 'BIO' {aka 'struct bio_st'} |
| gcp/sjeng | MAKE_FAIL | sjeng.c:523:42: error: expected ')' before 'VERSION' |
| GenABEL-Project/ProbABEL | MAKE_FAIL | make[2]: *** [Makefile:655: palinear] Error 1 |
| genebean/uptimed | MAKE_FAIL | urec.h:32:10: fatal error: sys/sysctl.h: No such file or directory |
| gentoo/cpuid2cpuflags | MAKE_FAIL | src/x86.h:10:15: error: unknown type name 'uint32_t' |
| GerHobbelt/html2db | MAKE_FAIL | make[2]: *** No rule to make target 'hello.o', needed by 'hello'.  Stop. |
| gerstner-hub/xwmfs | MAKE_FAIL | fuse/xwmfs_fuse_ops_impl.cxx:16:10: fatal error: cosmos/proc/process.hxx: No such file or  |
| ggreer/fsevents-tools | MAKE_FAIL | cc: error: unrecognized command-line option '-framework' |
| giannitedesco/ccid-utils | MAKE_FAIL | ../include/config.h:2:28: error: expected expression before '/' token |
| gitGNU/gnu_lash | MAKE_FAIL | make: *** No rule to make target 'svnversion.h', needed by 'all'.  Stop. |
| gitGNU/gnu_rpge | MAKE_FAIL | make: *** No targets specified and no makefile found.  Stop. |
| gitGNU/gnu_scambio | MAKE_FAIL | varbuf.c:25:10: fatal error: pth.h: No such file or directory |
| gitGNU/gnu_sipwitch | MAKE_FAIL | service.cpp:18:10: fatal error: ucommon/ucommon.h: No such file or directory |
| gitpan/libnf | MAKE_FAIL | /usr/bin/ld: cannot find -lnf: No such file or directory |
| gkobeaga/op-solver | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| glipari/rtscan | MAKE_FAIL | ar: ./.libs/property.o: No such file or directory |
| gmarcais/NoShell | MAKE_FAIL | make[2]: *** No rule to make target 'kill_self.o', needed by 'kill_self'.  Stop. |
| gmarcais/Quorum | MAKE_FAIL | make: *** No rule to make target 'src/error_correct_reads_cmdline.hpp', needed by 'all'.   |
| GNOME/alacarte | MAKE_FAIL | /bin/sh: 1: --trace: not found |
| GNOME/metacity | MAKE_FAIL | /bin/sh: 1: --trace: not found |
| gnu-mirror-unofficial/commoncpp | MAKE_FAIL | string.cpp:500:22: error: 'stristr' was not declared in this scope; did you mean 'strstr'? |
| gobolinux/GoboNet | MAKE_FAIL | ./config.h:1:26: error: expected expression before '/' token |
| google/ios-webkit-debug-proxy | MAKE_FAIL | ios_webkit_debug_proxy_main.c:12: error: "_GNU_SOURCE" redefined [-Werror] |
| gooroom/gtk3 | MAKE_FAIL | configure.ac:85: error: possibly undefined macro: AM_INIT_AUTOMAKE |
| gordonjcp/nekostring | MAKE_FAIL | neko_voice.h:17:10: fatal error: ladspa.h: No such file or directory |
| gpac-buildbot/avcap | MAKE_FAIL | DeviceCollector.cpp:38:11: fatal error: V4L1_Device.h: No such file or directory |
| grant-h/usbutils-portable | MAKE_FAIL | make[1]: *** No rule to make target 'all'.  Stop. |
| graygnuorg/pound | MAKE_FAIL | make[1]: *** No rule to make target 'cfg-lex.c', needed by 'all'.  Stop. |
| greatergoodguy/Game_Tech_Assignment1 | MAKE_FAIL | OgreFramework.hpp:8:10: fatal error: OgreCamera.h: No such file or directory |
| greg-kennedy/BridgeMail | MAKE_FAIL | cc: error: unrecognized argument to '-fsanitize=' option: 'integer' |
| gregkh/bti | MAKE_FAIL | config.c:38:10: fatal error: oauth.h: No such file or directory |
| grgbr/haveged | MAKE_FAIL | ../config.h:7:29: error: expected ')' before ':' token |
| grisbi/grisbi | MAKE_FAIL | make[1]: --generate-dependencies: No such file or directory |
| grobian/carbon-c-relay | MAKE_FAIL | relay.c:357:34: error: expected ')' before 'VERSION' |
| grondo/edac-utils | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| GroovIM/transport | MAKE_FAIL | ./build.sh: line 14: .././pth-2.0.7/configure: No such file or directory |
| GrumpyOldTroll/libmcrx | MAKE_FAIL | /tmp/atlasx_GrumpyOldTroll__libmcrx/s/test/mcrx-check.c:35:(.text.receive_cb+0x31): undefi |
| guardianproject/libsqlfs | MAKE_FAIL | sqlfs.c:2302:17: error: 'strncat' specified bound 1 equals source length [-Werror=stringop |
| gucong/robotxq | MAKE_FAIL | cchess.cpp:694:15: error: ISO C++ forbids comparison between pointer and integer [-fpermis |
| guillemj/xfstt | MAKE_FAIL | make[1]: *** No rule to make target 'all'.  Stop. |
| guillermocalvo/exceptions4c | MAKE_FAIL | /usr/bin/ld: cannot open output file bin/check/catch-all: No such file or directory |
| h4mu/rott94 | MAKE_FAIL | modexlib.c:422:1: error: unknown type name 'SDL_Window'; did you mean 'SDL_cond'? |
| habanero-rice/habanero-upc | MAKE_FAIL | Makefile:568: /include/upcxx.mak: No such file or directory |
| hackerschoice/gsocket | MAKE_FAIL | Makefile:231: *** missing separator.  Stop. |
| hadfl/lxadm | MAKE_FAIL | Makefile:45: *** missing separator.  Stop. |
| haegrr/testtool | MAKE_FAIL | main.c:759:59: error: 'PACKAGE' undeclared (first use in this function); did you mean 'PAC |
| hamidreza-s/NanoChat | MAKE_FAIL | nc.h:22:10: fatal error: nanomsg/nn.h: No such file or directory |
| heltilda/cicada | MAKE_FAIL | cmpile.c:1039:6: error: variably modified 'numBuffer' at file scope |
| herczy/tinu | MAKE_FAIL | elfcore.c:51:10: fatal error: sys/sysctl.h: No such file or directory |
| hezi/dosbox-x-gdb | MAKE_FAIL | callback.cpp:29:11: fatal error: emscripten.h: No such file or directory |
| hholzgra/connector-c-examples | MAKE_FAIL | mysql_create_db.c:41:10: error: implicit declaration of function 'mysql_create_db'; did yo |
| Hi-Angel/faux | MAKE_FAIL | faux/Makefile.am:7: *** missing separator.  Stop. |
| hillstoneUnited/hillstoneUnited | MAKE_FAIL | /usr/bin/ld: cannot find -lrcssnet3D: No such file or directory |
| holylobster/nuntius-linux | MAKE_FAIL | make: --generate-dependencies: No such file or directory |
| hpc/libdftw | MAKE_FAIL | libdftw/dftw.c:1:10: fatal error: libcircle.h: No such file or directory |
| hpc/xpmem | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| HSU-ANT/gstpeaq | MAKE_FAIL | peaq.c:23:10: fatal error: gst/gst.h: No such file or directory |
| hunspell/mythes | MAKE_FAIL | example.cxx:7:10: fatal error: hunspell.hxx: No such file or directory |
| huttmf/complexlib | MAKE_FAIL | /usr/bin/ld: cannot find -lcomplex: No such file or directory |
| hyperair/fptree | MAKE_FAIL | /usr/bin/ld: cannot find -lpsapi: No such file or directory |
| hyperic/sigar | MAKE_FAIL | sigar_util.c:742:10: fatal error: rpc/rpc.h: No such file or directory |
| hyrathb/mentohust | MAKE_FAIL | myconfig.c:219:50: error: 'VERSION' undeclared (first use in this function) |
| i4tv/gstreamill | MAKE_FAIL | utils.c:16:10: fatal error: glib/gstdio.h: No such file or directory |
| iagorubio/gnome-search-tool | MAKE_FAIL | gsearchtool-callbacks.c:105:56: error: 'VERSION' undeclared (first use in this function) |
| iainlane/mo | MAKE_FAIL | collect2: error: ld returned 1 exit status |
| ib/xarchiver | MAKE_FAIL | make[1]: *** No rule to make target 'all'.  Stop. |
| IBM/corosync-qdisk | MAKE_FAIL | persistent_reserve/device.c:81:24: error: 'PR_ERR_DEVICE_CANT_OPEN_KEYFILE' undeclared (fi |
| iczelia/xpar | MAKE_FAIL | (.text+0x1b): undefined reference to `main' |
| idosch/ethtool | MAKE_FAIL | ethtool.c:372:17: error: 'PACKAGE' undeclared (first use in this function); did you mean ' |

## ❌ Non-working — failed (ours fails before make) (535)

| repo | stage | first error |
| --- | --- | --- |
| 315234/lyx-retina | CONFIGURE_RUN_FAIL | ./configure: line 1367: LYX_CHECK_VERSION: command not found |
| 6WIND/quagga | CONFIGURE_RUN_FAIL | ./configure: line 3247: AC_WORDS_BIGENDIAN: command not found |
| A-Kyle/GrADS-CJK | CONFIGURE_RUN_FAIL | checking for supplibs directory... ./configure: line 2328: syntax error near unexpected to |
| a-sassmannshausen/guile-monads | CONFIGURE_RUN_FAIL | ./configure: line 1366: syntax error near unexpected token `2.0.11' |
| abc100m/libzdbcpp | CONFIGURE_RUN_FAIL |  |
| acaudwell/Logstalgia | CONFIGURE_RUN_FAIL | configure: error: Could not find a valid OpenGL implementation |
| accellera-official/systemc | CONFIGURE_RUN_FAIL | whether we are using a Clang/LLVM C compiler... configure: error: "sorry...architecture no |
| acerion/cwdaemon | CONFIGURE_RUN_FAIL | ./configure: line 1643: 0: command not found |
| acoin-project/acoin | CONFIGURE_RUN_FAIL | configure: error: libdb_cxx headers missing |
| adiknoth/netatalk-debian | CONFIGURE_RUN_FAIL | ./configure: line 1390: AC_PROG_PERL: command not found |
| adobe-research/libkafka | CONFIGURE_RUN_FAIL | ./configure: line 1397: syntax error near unexpected token `[_m4_provided_GTEST_LIB_CHECK] |
| AdolfVonKleist/Phonetisaurus | CONFIGURE_RUN_FAIL | configure: error: Can't find OpenFST or one or more of its extensions. Use --with-openfst- |
| ADVANTECH-Corp/WiseSnail | CONFIGURE_RUN_FAIL | ./configure: line 1673: ./library/WiseCore/WiseCore_MQTT/configure: No such file or direct |
| AegisEmu/AegisEmu | CONFIGURE_RUN_FAIL | configure: error: Subversion not found |
| affix/MSNC | CONFIGURE_RUN_FAIL | configure: error: You don't seem to have the curses headers installed |
| agalakhov/captdriver | CONFIGURE_RUN_FAIL | configure: error: CUPS library not found. |
| agn453/ZXCC | CONFIGURE_RUN_FAIL | ./configure: line 1561: ./cpmio/configure: No such file or directory |
| agordon/fastx_toolkit | CONFIGURE_RUN_FAIL | for long long int... ./configure: line 1389: syntax error near unexpected token `fi' |
| ahjragaas/inetutils | CONFIGURE_RUN_FAIL | ./configure: line 1534: syntax error near unexpected token `ftpd' |
| ahlstromcj/midicvt | CONFIGURE_RUN_FAIL |  |
| ahmedammar/platform_external_gst_gstreamer | CONFIGURE_RUN_FAIL | ./configure: line 1353: AG_GST_INIT: command not found |
| ahorn/cpp-channel | CONFIGURE_RUN_FAIL | ./configure: line 1369: ./gtest/configure: No such file or directory |
| ahupowerdns/setgrouper | CONFIGURE_RUN_FAIL | configure: error: no C++ compiler found |
| aissammouche/bitextor | CONFIGURE_RUN_FAIL | configure: error: You don't have bash installed. |
| ajhc/ajhc | CONFIGURE_RUN_FAIL | The ghc compiler was not found, please specify a location for it with the --with-hc flag |
| ajnelson/geoproc | CONFIGURE_RUN_FAIL | configure: error: GNU getopt not found. |
| ajnelson/photorec-testdisk | CONFIGURE_RUN_FAIL | configure: error: At least one of ncursesw/ncurses/pdcurses/curses library must be present |
| ajnelson/regxml_extractor | CONFIGURE_RUN_FAIL | configure: error: GNU getopt not found. |
| alanjaouen/compilateur-cpp | CONFIGURE_RUN_FAIL | awk: cmd. line:1:                               ^ syntax error |
| albertok/web-polygraph | CONFIGURE_RUN_FAIL | configure: error: the compiler (g++) failed to pass a simple C++ test; check config.log fo |
| alexandervdm/gummi | CONFIGURE_RUN_FAIL | configure: error: You need gtksourceview3 >= 3.4.0 to build |
| AlmuHS/GNUMach_SMP | CONFIGURE_RUN_FAIL | ./configure: line 1364: $'\f': command not found |
| alsa-project/alsa-firmware | CONFIGURE_RUN_FAIL |  |
| AmkG/hl | CONFIGURE_RUN_FAIL | checking how to get an intptr_t type with ranges... configure: error: We can't find out wh |
| andestech/nds-openocd | CONFIGURE_RUN_FAIL | ./configure: line 2108: syntax error near unexpected token `1,' |
| andrewshadura/tnat64 | CONFIGURE_RUN_FAIL | configure: error: 'Could not find library containing connect()' |
| AndyA/htop-osx | CONFIGURE_RUN_FAIL | ./configure: line 1810: syntax error near unexpected token `fi' |
| anewhuahua/bilitw | CONFIGURE_RUN_FAIL | ./configure: line 2015: ./contrib/yaml-0.1.4/configure: No such file or directory |
| ansisatteka/ovs-ipsec | CONFIGURE_RUN_FAIL | for Python 2.x for x >= 7... configure: error: cannot find python 2.7 or higher. |
| ant9000/libsigrok | CONFIGURE_RUN_FAIL | checking for pygobject-3.0 >= 3.0.0... ./configure: line 2769: syntax error near unexpecte |
| AnthonyDeroche/mod_authnz_jwt | CONFIGURE_RUN_FAIL |  |
| anyone-protocol/ator-protocol | CONFIGURE_RUN_FAIL | ./configure: line 2285: dirauth: command not found |
| AOMediaCodec/oac | CONFIGURE_RUN_FAIL | ./configure: line 2382: -mfma: command not found |
| apache/xerces-c | CONFIGURE_RUN_FAIL | ./configure: line 1796: pthread-config: command not found |
| apertium/apertium | CONFIGURE_RUN_FAIL | configure: error: You don't have xmllint installed. |
| aportelli/LatAnalyze | CONFIGURE_RUN_FAIL | configure: error: HDF5 library not found |
| Apress/Why-Learn-C | CONFIGURE_RUN_FAIL | configure: error:  not supported by C compiler |
| ArchimedesCAD/libredwg | CONFIGURE_RUN_FAIL | configure: error: SWIG is required to build.. |
| argonne-lcf/THAPI | CONFIGURE_RUN_FAIL | checking whether ln works... ./configure: line 3334: syntax error: unexpected end of file |
| aristanetworks/EosSdk | CONFIGURE_RUN_FAIL | for std::unordered_set::operator==... configure: error: Your version of the STL seems to b |
| arkadijs/asterisk-g72x | CONFIGURE_RUN_FAIL | checking whether C compiler accepts -march=native... ./configure: line 1621: 0: command no |
| arki55/fuse-fuse | CONFIGURE_RUN_FAIL | ./configure: line 1429: syntax error near unexpected token `(' |
| arkq/bluez-alsa | CONFIGURE_RUN_FAIL | configure: error: unable to find clock_gettime() function |
| armenb/sharktools | CONFIGURE_RUN_FAIL | configure: error: Required parameter: you need to set --with-wireshark-src=/path/to/wiresh |
| ARPA-SIMC/arkimet | CONFIGURE_RUN_FAIL | ./configure: line 2660: geos-config: command not found |
| ARPA-SIMC/dballe | CONFIGURE_RUN_FAIL | ./configure: line 1412: syntax error near unexpected token `;' |
| artclarke/xuggle-xuggler | CONFIGURE_RUN_FAIL | ./configure: line 1629: ./captive/configure: No such file or directory |
| arthurdejong/nss-pam-ldapd | CONFIGURE_RUN_FAIL | configure: error: PAM header files are missing |
| artyom-poptsov/metabash | CONFIGURE_RUN_FAIL | configure: error: required guile module not found: (ssh session) |
| Ashod/garli | CONFIGURE_RUN_FAIL | ./configure: line 1744: ACX_MPI: command not found |
| asnelt/rrep | CONFIGURE_RUN_FAIL | configure: error: Invalid value for --with-included-regex: |
| ASPLes/libaxl | CONFIGURE_RUN_FAIL | ./configure: line 1521: python: command not found |
| assen-totin/mate-applet-streamer | CONFIGURE_RUN_FAIL | configure: error: "*** GTK not found." |
| astromatic/psfex | CONFIGURE_RUN_FAIL | ******** Configuring:  PSFEx 3.24.2 -  (2026-07-03) ******** |
| astromatic/sextractor | CONFIGURE_RUN_FAIL | ******** Configuring:  SExtractor 2.29.0 -  (2026-07-03) ******** |
| astromatic/skymaker | CONFIGURE_RUN_FAIL | ******** Configuring:  SkyMaker 4.3.0 -  (2026-07-03) ******** |
| autotools-mirror/autoconf | CONFIGURE_RUN_FAIL | ./configure: line 2812: syntax error: unexpected end of file |
| avr-aics-riken/234Compositor | CONFIGURE_RUN_FAIL |  |
| avr-aics-riken/CIOlib | CONFIGURE_RUN_FAIL |  |
| awaw/dnsproxy | CONFIGURE_RUN_FAIL | checking for libevent... configure: error: |
| awgn/brute | CONFIGURE_RUN_FAIL | ./configure: line 1573: syntax error near unexpected token `string.h' |
| awsteiner/o2scl | CONFIGURE_RUN_FAIL | Boost locale not found. |
| ayumin/open-cobol | CONFIGURE_RUN_FAIL | ./configure: line 2865: syntax error near unexpected token `newline' |
| ayyi/samplecat | CONFIGURE_RUN_FAIL | configure: error: libyaml not found |
| azatoth/minidlna | CONFIGURE_RUN_FAIL | configure: error: libavutil headers not found or not usable |
| backuppc/backuppc-xs | CONFIGURE_RUN_FAIL | checking if md2man can create manpages... no - python3 not found |
| balde/balde | CONFIGURE_RUN_FAIL | configure: error: no -fvisibility=hidden support found in , balde requires -fvisibility=hi |
| bambulab/gmp | CONFIGURE_RUN_FAIL | ./configure: line 1465: syntax error near unexpected token `newline' |
| baoxuezhao/GPU-SExtractor | CONFIGURE_RUN_FAIL | *********** Configuring:    (2026-07-03) ********** |
| barak/djview4 | CONFIGURE_RUN_FAIL | conftest.d/conftest.sh: line 1: creating: command not found |
| barak/oaklisp | CONFIGURE_RUN_FAIL | for long long int... ./configure: line 1793: syntax error near unexpected token `fi' |
| baszoetekouw/pinfo | CONFIGURE_RUN_FAIL |  |
| BayshoreNetworks/yextend | CONFIGURE_RUN_FAIL | checking checking for Python libraries used in unit tests... configure: error: "Cannot run |
| bbc/bbcat-control | CONFIGURE_RUN_FAIL | configure: error: bbcat-base-0.1 >= 0.1.2.1 is required |
| bbc/bbcat-dsp | CONFIGURE_RUN_FAIL | configure: error: bbcat-base-0.1 >= 0.1.2.0 is required |
| bbc/bbcat-fileio | CONFIGURE_RUN_FAIL | configure: error: bbcat-base-0.1 >= 0.1.2.0 is required |
| bbc/vc2hqdecode | CONFIGURE_RUN_FAIL | configure: error: librt is required |
| bbidulock/xde-sounds | CONFIGURE_RUN_FAIL | configure: error: |
| bcoin-org/libtorsion | CONFIGURE_RUN_FAIL | ./configure: line 3495: syntax error near unexpected token `;' |
| bdwgc/bdwgc | CONFIGURE_RUN_FAIL | ./configure: line 1645: syntax error near unexpected token `(' |
| benchmark-subsetting/cere | CONFIGURE_RUN_FAIL | configure: error: At least LLVM version 7.0 is required |
| benegon/ntp | CONFIGURE_RUN_FAIL | ./configure: line 1823: syntax error near unexpected token `sntp/libopts' |
| benvanik/gflags | CONFIGURE_RUN_FAIL | ./configure: line 1854: pthread-config: command not found |
| benwbooth/tvision | CONFIGURE_RUN_FAIL | ./configure: line 1443: AC_STDC_HEADERS: command not found |
| bert/geda-gaf | CONFIGURE_RUN_FAIL | ./configure: line 1465: []: command not found |
| bgpsecurity/rpstir | CONFIGURE_RUN_FAIL | ./configure: line 1582: CFLAGS: command not found |
| BIC-MNI/bicpl | CONFIGURE_RUN_FAIL | ./configure: line 1384: smr_WITH_BUILD_PATH: command not found |
| BIMSBbioinfo/swineherd | CONFIGURE_RUN_FAIL | ./configure: line 1358: syntax error near unexpected token `3.0' |
| binhqnguyen/ovs-srv6 | CONFIGURE_RUN_FAIL | whether cc accepts -Werror... ./configure: line 2760: syntax error near unexpected token ` |
| BirolLab/abyss | CONFIGURE_RUN_FAIL |  |
| BirolLab/ChopStitch | CONFIGURE_RUN_FAIL | configure: error: CHOPSTITCH must be compiled with a C++ compiler that supports OpenMP thr |
| BirolLab/ntCard | CONFIGURE_RUN_FAIL | configure: error: NTCARD must be compiled with a C++ compiler that supports OpenMP threadi |
| bitblaze-fuzzball/fuzzball | CONFIGURE_RUN_FAIL | configure: error: m4_default([], [Cannot find [ocamlc]]) |
| bitcoinbabys/flexinodes | CONFIGURE_RUN_FAIL | configure: error: libdb_cxx headers missing |
| BitzenyCoreDevelopers/cpuminer | CONFIGURE_RUN_FAIL | ./configure: line 1801: syntax error near unexpected token `,' |
| bitzeppelin/linphone-sdk | CONFIGURE_GEN_FAIL |  |
| bjango/istatserverlinux | CONFIGURE_RUN_FAIL | configure: error: sqlite not found. please install libsqlite3-dev/sqlite-devel or a simila |
| bjoernvoss/RNAHeliCes | CONFIGURE_RUN_FAIL | ./configure: line 3055: syntax error: unexpected end of file |
| bk138/wxservdisc | CONFIGURE_RUN_FAIL | checking wxWidgets version... ./configure: line 1506: wx-config: command not found |
| bkearney/augeas | CONFIGURE_RUN_FAIL |  |
| bloomen/libunittest | CONFIGURE_RUN_FAIL | ./configure: line 1593: pthread-config: command not found |
| blueness/sthttpd | CONFIGURE_RUN_FAIL | ./configure: line 1377: syntax error near unexpected token `AR,' |
| BOINC/boinc | CONFIGURE_RUN_FAIL | ./configure: line 1352: BOINC,: command not found |
| BoldingBruggeman/netcdf3 | CONFIGURE_RUN_FAIL | ./configure: line 5104: syntax error: unexpected end of file |
| bondhugula/pluto | CONFIGURE_RUN_FAIL | configure: error: Please install LLVM FileCheck before configuring. |
| bonzini/smalltalk | CONFIGURE_RUN_FAIL | whether  produces any warnings... ./configure: line 1615: syntax error near unexpected tok |
| borkmann/lksctp-tools | CONFIGURE_RUN_FAIL | timeout: failed to run command './configure': Permission denied |
| boundary/wireshark | CONFIGURE_RUN_FAIL | ./configure: line 1415: syntax error near unexpected token `looks' |
| boxbackup/boxi | CONFIGURE_RUN_FAIL | checking wxWidgets version... ./configure: line 1420: wx-config: command not found |
| boysetsfrog/vimpc | CONFIGURE_RUN_FAIL | configure: error: vimpc requires boost library |
| bradenmcd/uri-grammar | CONFIGURE_RUN_FAIL | for boost_thread-mt library... configure: error: libboost_thread-mt not found |
| BrandRegard/gnash | CONFIGURE_RUN_FAIL | ./configure: line 1352: gnash,: command not found |
| BrianGladman/mpfr | CONFIGURE_RUN_FAIL | ./configure: line 2243: syntax error near unexpected token `line' |
| brianmcgillion/udev | CONFIGURE_RUN_FAIL | configure: error: POSIX RT library not found |
| brimworks/zile | CONFIGURE_RUN_FAIL | configure: error: Lua not found |
| bsc-pm/sonar | CONFIGURE_RUN_FAIL | ./configure: line 1478: This: command not found |
| bsc-pm/tasycl | CONFIGURE_RUN_FAIL | whether C compiler accepts -fsycl... ./configure: line 1551: -I/include/sycl: No such file |
| bspeice/libcvautomation | CONFIGURE_RUN_FAIL | configure: error: |
| bulislaw/obexd-eds | CONFIGURE_RUN_FAIL | configure: error: libopenobex is required |
| BunsenLabs/plank | CONFIGURE_RUN_FAIL | ./configure: line 1431: GLIB_GSETTINGS: command not found |
| bytedance/ovs-dpdk | CONFIGURE_RUN_FAIL | ./configure: line 3529: string.h: command not found |
| c-rack/squid-ecap-gzip | CONFIGURE_RUN_FAIL |  |
| calaos/calaos_base | CONFIGURE_RUN_FAIL |  |
| cannabisday/ovs-tsn | CONFIGURE_RUN_FAIL | ./configure: line 3542: string.h: command not found |
| canonical/dqlite | CONFIGURE_RUN_FAIL | ./configure: line 3235: syntax error: unexpected end of file |
| carboncointrust/CarboncoinCore | CONFIGURE_RUN_FAIL | configure: error: libdb_cxx headers missing |
| cbg-ethz/shorah | CONFIGURE_RUN_FAIL | ./configure: line 1480: syntax error near unexpected token `],[,,[Define' |
| cbuchner1/ccminer | CONFIGURE_RUN_FAIL | configure: error: OpenSSL library required |
| cciechad/brlcad | CONFIGURE_RUN_FAIL | *** Configuring BRL-CAD Release MAJOR_VERSION.MINOR_VERSION.PATCH_VERSION, Build 20260703  |
| cculianu/secp256k1 | CONFIGURE_RUN_FAIL | ./configure: line 2251: syntax error near unexpected token `_JTOPDIR=$_JTOPDIR' |
| Certseeds/graphicsmagick | CONFIGURE_RUN_FAIL | ./configure: line 2149: syntax error near unexpected token `done' |
| CESARBR/knot-network-nrf24 | CONFIGURE_RUN_FAIL | configure: error: "ell missing" |
| CESARBR/knot-service-source | CONFIGURE_RUN_FAIL | configure: error: "ell missing" |
| CESNET/GPUJPEG | CONFIGURE_RUN_FAIL | configure: error: Unsupported cuda compiler |
| CESNET/libfastbit | CONFIGURE_RUN_FAIL | checking if compiler supports __sync_add_and_fetch for 32-bit integers... checking if comp |
| cfra/quagga-testing | CONFIGURE_RUN_FAIL | ./configure: line 2756: AC_WORDS_BIGENDIAN: command not found |
| cgdb/cgdb | CONFIGURE_RUN_FAIL | configure: error: Please install flex before installing |
| cgwalters/git-evtag | CONFIGURE_RUN_FAIL |  |
| chabbimilind/cctlib | CONFIGURE_RUN_FAIL | configure: error: --with-libelf requires absolute path as argument; given '' |
| chaoran/fibril | CONFIGURE_RUN_FAIL | ./configure: line 1586: pthread-config: command not found |
| chaos/pdsh | CONFIGURE_RUN_FAIL | ./configure: line 597: syntax error near unexpected token `(' |
| chaos/powerman | CONFIGURE_RUN_FAIL | ./configure: line 597: syntax error near unexpected token `(' |
| chaos/slurm | CONFIGURE_RUN_FAIL | ./configure: line 1362: X_AC_GPL_LICENSED: command not found |
| chaupal/jxta-c | CONFIGURE_RUN_FAIL | configure: error: apr-util does not support SQLite3, as required by JXTA |
| chenall/grub4dos | CONFIGURE_RUN_FAIL |  |
| cherojeong/extundelete | CONFIGURE_RUN_FAIL | checking whether C compiler accepts -std=c99... ./configure: line 1480: 0: command not fou |
| chimari/MaCoPiX | CONFIGURE_RUN_FAIL | ./configure: line 1584: AC_FUNC_LSTAT: command not found |
| chiphackers/covered | CONFIGURE_RUN_FAIL | ./configure: line 1733: syntax error near unexpected token `test' |
| choeger/MetaModelica-autotools | CONFIGURE_RUN_FAIL | configure: error: Cannot find mlton |
| chosen1/SniffDet | CONFIGURE_RUN_FAIL | configure: error: "*** libnet10-config not found! You need libnet 1.0 to build sniffdet! * |
| christian-sahlmann/gwyddion | CONFIGURE_RUN_FAIL | ./configure: line 1740: gconftool-2: command not found |
| ChrisVine/guile-a-sync | CONFIGURE_RUN_FAIL |  |
| Chronic-Dev/libgcrypt | CONFIGURE_RUN_FAIL | ./configure: line 1499: AC_DIVERT_POP: command not found |
| chuckleb/virt-what | CONFIGURE_RUN_FAIL |  |
| cisco/opus | CONFIGURE_RUN_FAIL | ./configure: line 1430: AC_MINGW32: command not found |
| cjcole/libgolle | CONFIGURE_RUN_FAIL | ./configure: line 1541: DX_PDF_FEATURE: command not found |
| CkNoSFeRaTU/pidgin | CONFIGURE_RUN_FAIL | ./configure: line 2024: AC_PROG_INTLTOOL: command not found |
| cluslab/metastack | CONFIGURE_RUN_FAIL | ./configure: line 597: syntax error near unexpected token `(' |
| ClusterLabs/cluster-glue | CONFIGURE_RUN_FAIL | ./configure: line 1510: AC_LIBLTDL_CONVENIENCE: command not found |
| ClusterLabs/fence-agents | CONFIGURE_RUN_FAIL | whether the linker accepts -Wl,--enable-new-dtags... configure: error: "Linker support for |
| ClusterLabs/libqb | CONFIGURE_RUN_FAIL | ./configure: 4: Syntax error: "\|" unexpected |
| cmand/yarrp | CONFIGURE_RUN_FAIL | configure: error: either specify a valid zlib installation with --with-zlib=DIR or disable |
| cmauri/eviacam | CONFIGURE_RUN_FAIL | configure: error: libXext is required. |
| cminyard/gensio | CONFIGURE_RUN_FAIL | ./configure: line 5824: syntax error near unexpected token `<<<' |
| cminyard/ser2net | CONFIGURE_RUN_FAIL | configure: error: libgensio won't link, please install gensio dev package |
| cmu-sei/BigGrep | CONFIGURE_RUN_FAIL | ./configure: line 2957: syntax error: unexpected end of file |
| cnDelbert/libtiff | CONFIGURE_RUN_FAIL | configure: error: Unsupported size_t size 0; please add support |
| CNGLDLab/LORG-Release | CONFIGURE_RUN_FAIL | ./configure: line 1486: syntax error near unexpected token `;' |
| CoachRun/boinc | CONFIGURE_RUN_FAIL | ./configure: line 1352: BOINC,: command not found |
| coapp-packages/libunistring | CONFIGURE_RUN_FAIL | ./configure: line 2852: syntax error: unexpected end of file |
| cockpit-project/cockpit | CONFIGURE_RUN_FAIL | configure: error: Couldn't find crypt library. Try installing glibc-devel |
| code-saturne/code_saturne | CONFIGURE_RUN_FAIL | ./configure: line 3362: syntax error near unexpected token `fi' |
| codecryptanalysis/mccl | CONFIGURE_RUN_FAIL | ./configure: line 1359: This: command not found |
| coin-or-tools/ThirdParty-ASL | CONFIGURE_RUN_FAIL | ./configure: line 1364: AC_COIN_INITIALIZE: command not found |
| coin-or-tools/ThirdParty-HSL | CONFIGURE_RUN_FAIL | ./configure: line 1364: AC_COIN_INITIALIZE: command not found |
| coin-or/Cbc | CONFIGURE_RUN_FAIL | ./configure: line 1365: AC_COIN_INITIALIZE: command not found |
| coin-or/OS | CONFIGURE_RUN_FAIL | ./configure: line 1376: AC_COIN_CREATE_LIBTOOL: command not found |
| coin-or/Rehearse | CONFIGURE_RUN_FAIL | ./configure: line 1830: syntax error near unexpected token `sets' |
| coin3d/quarter | CONFIGURE_RUN_FAIL | ./configure: line 1369: SIM_AC_SETUP_MSVCPP_IFELSE: command not found |
| colaghost/coroutine_event | CONFIGURE_RUN_FAIL | for libevent directory... configure: error: libevent is required.If it's already installed |
| common-tools-interface/cti | CONFIGURE_RUN_FAIL | ./configure: line 1432: syntax error near unexpected token `(' |
| compiz-reloaded/emerald | CONFIGURE_RUN_FAIL | configure: error: Failed to check the decorator interface version |
| COMSYS/tor4iot-tor | CONFIGURE_RUN_FAIL | configure: error: Missing libraries; unable to proceed. |
| ConsoleKit2/ConsoleKit2 | CONFIGURE_RUN_FAIL | checking for optional package polkit-gobject-1 >= 0.92... not found |
| cooljeanius/docbook-utils-0.6.14 | CONFIGURE_RUN_FAIL | checking for a decent directory to use for jade_bindir... not found |
| cooljeanius/gawk | CONFIGURE_RUN_FAIL | ./configure: line 1378: syntax error near unexpected token `then' |
| cooljeanius/gcab | CONFIGURE_RUN_FAIL | ./configure: line 1415: AC_LIB_PROG_LD_GNU: command not found |
| cooljeanius/gcml2-0.7.1 | CONFIGURE_RUN_FAIL | checking for IMLIB - version >= 1.8.2... ./configure: line 1672: --cflags: command not fou |
| cooljeanius/libUnixToOSX | CONFIGURE_RUN_FAIL | Try 0 --help for more information.: syntax error in expression (error token is "Try 0 --he |
| cooljeanius/magicseteditor | CONFIGURE_RUN_FAIL | configure: error: Could not link against boost_regex ! |
| cooljeanius/mdnsd | CONFIGURE_RUN_FAIL | ./configure: line 1392: syntax error near unexpected token `AM_SET_LEADING_DOT' |
| cooljeanius/pkg-config | CONFIGURE_RUN_FAIL | ./configure: line 1364: syntax error near unexpected token `then' |
| coolwanglu/scanmem_ | CONFIGURE_RUN_FAIL |  |
| corazawaf/libcoraza | CONFIGURE_RUN_FAIL | configure: error: "Go does not found" |
| CoryXie/GRUB2 | CONFIGURE_RUN_FAIL | configure: error: unsupported CPU: "" |
| cosmicrays/DRAGON | CONFIGURE_RUN_FAIL | ./configure: line 1467: AC_F77_LIBRARY_LDFLAGS: command not found |
| cosmos72/twin | CONFIGURE_RUN_FAIL | ./configure: line 1692: syntax error near unexpected token `_LT_SYS_DYNAMIC_LINKER' |
| couchbaselabs/breakpad | CONFIGURE_RUN_FAIL | ./configure: line 1630: pthread-config: command not found |
| cowsql/raft | CONFIGURE_RUN_FAIL | ./configure: line 3364: syntax error: unexpected end of file |
| cpaasch/wireshark | CONFIGURE_RUN_FAIL | ./configure: line 1419: syntax error near unexpected token `looks' |
| cpputest/cpputest | CONFIGURE_RUN_FAIL | ./configure: line 1627: pthread-config: command not found |
| crdroidandroid/android_hardware_qcom_display | CONFIGURE_RUN_FAIL |  |
| CRG-Barcelona/bwtool | CONFIGURE_RUN_FAIL | configure: error: |
| criort/libPRISM | CONFIGURE_RUN_FAIL | configure:  *********************************************************** |
| cruppstahl/upscaledb | CONFIGURE_RUN_FAIL | whether the compiler supports GCC C++ ABI name demangling... ./configure: line 2009: synta |
| CryptoBridge/bridgecoin | CONFIGURE_RUN_FAIL | configure: error: cannot figure out how to use std::atomic |
| cryptode/cryptode | CONFIGURE_RUN_FAIL | ./configure: line 1494: syntax error near unexpected token `string.h' |
| CryptVenture/BitMoneyV2 | CONFIGURE_RUN_FAIL | configure: error: libdb_cxx headers missing |
| crystalsnetworkdev/pq-crystals | CONFIGURE_RUN_FAIL | ./configure: line 2469: syntax error near unexpected token `aes-ni,' |
| crystax/android-vendor-gnu-tar | CONFIGURE_RUN_FAIL | configure: error: The --with-packager-{bug-reports,version} options require --with-package |
| CS198NDSGChanBrianJoe/html5rdp | CONFIGURE_RUN_FAIL | ./configure: line 1881: syntax error near unexpected token `;' |
| cslarsen/mickey-scheme | CONFIGURE_RUN_FAIL | configure: error: readline test failed (--without-readline to disable) |
| csmith-project/creduce | CONFIGURE_RUN_FAIL | ./configure: line 1527: syntax error near unexpected token `(' |
| cstrope/indel-seq-gen | CONFIGURE_RUN_FAIL | ./configure: line 1695: syntax error near unexpected token `then' |
| cwi-dis/ambulant | CONFIGURE_RUN_FAIL | ./configure: line 1513: LT_AC_PROG_SED: command not found |
| CyanogenMod/android_external_protobuf-c | CONFIGURE_RUN_FAIL |  |
| cybermaggedon/cyberprobe | CONFIGURE_RUN_FAIL | ./configure: line 3809: syntax error: unexpected end of file |
| cydhaselton/mono-android | CONFIGURE_RUN_FAIL | configure: error: You need to install g++ |
| CZ-NIC/fred-db | CONFIGURE_RUN_FAIL | ls: cannot access '/etc/postgresql/': No such file or directory |
| d99kris/namp-lite | CONFIGURE_RUN_FAIL | configure: error: Required library ncursesw not found. |
| dajobe/librdf | CONFIGURE_RUN_FAIL | expr: syntax error: unexpected argument '10000' |
| Dale-M/mcron | CONFIGURE_RUN_FAIL | ./configure: line 1433: syntax error near unexpected token `3.0' |
| dankamongmen/babl | CONFIGURE_RUN_FAIL | ./configure: line 1366: syntax error near unexpected token `(' |
| danos/frr | CONFIGURE_RUN_FAIL | configure: error: cross-compilation is only possible with builddir separate from srcdir.   |
| Dansguardian/dansguardian | CONFIGURE_RUN_FAIL |  |
| datacratic/gperftools | CONFIGURE_RUN_FAIL | configure: error: cannot find the nanosleep function |
| davelambert/guile-pcap | CONFIGURE_RUN_FAIL | configure: error: Cannot find guile-srfi-srfi-4-v-1, required for compilation |
| daveshields/jikes | CONFIGURE_RUN_FAIL | for __S_IFDIR symbol... for _S_IFDIR symbol... for S_IFDIR symbol... configure: error: Cou |
| davexunit/guile-2d | CONFIGURE_RUN_FAIL | ./configure: line 1362: GUILE_PROGS: command not found |
| daveyc/gawk_zos | CONFIGURE_RUN_FAIL | checking for msgfmt... ./configure: line 1868: syntax error near unexpected token `newline |
| davidgiven/libfirm | CONFIGURE_RUN_FAIL | CFLAGS for gcc -fvisibility=hidden... ./configure: line 1446: unknown: command not found |
| dcos/dcos-mesos-modules | CONFIGURE_RUN_FAIL | configure: error: Invalid mesos path; use --with-mesos-root=<DIR> |
| ddarriba/pll-modules | CONFIGURE_RUN_FAIL | configure: error: could not find required installation of BISON |
| delphix/nfs-utils | CONFIGURE_RUN_FAIL | ./configure: line 1914: AC_LIBTIRPC: command not found |
| demorest/mark5access | CONFIGURE_RUN_FAIL |  |
| DeNA/HandlerSocket-Plugin-for-MySQL | CONFIGURE_RUN_FAIL | checking mysql source... configure: error: --with-mysql-source=PATH is required for standa |
| deskull-m/bakabakaband | CONFIGURE_RUN_FAIL | configure: error: nkf is not found. Please install nkf. |
| detomastah/adwc | CONFIGURE_RUN_FAIL | ./configure: line 2091: syntax error near unexpected token `'$(top_srcdir)/protocol'' |
| detomon/BlipKit | CONFIGURE_RUN_FAIL | configure: error: SDL was requested with --with-sdl, but SDL was not found. Use --without- |
| detomon/json5 | CONFIGURE_RUN_FAIL | ./configure: line 1546: syntax error near unexpected token `unicode-table' |
| devicescape/aws_dynamo | CONFIGURE_RUN_FAIL | configure: error: no openssl; please install openssl or equivalent |
| devzero2000/POPT | CONFIGURE_RUN_FAIL | ./configure: line 1531: syntax error near unexpected token `popt_cflags,' |
| dex4er/fakechroot | CONFIGURE_RUN_FAIL | whether opendir function calls __open function internally... whether opendir function call |
| dex4er/nss-docker | CONFIGURE_RUN_FAIL | whether prove accepts --failures... ./configure: line 1639: syntax error near unexpected t |
| dfrc-korea/carpe-sleuthkit | CONFIGURE_RUN_FAIL | checking for Cppunit - version >= 1.12.1... ./configure: line 1433: --cflags: command not  |
| DGCDev/digitalcoin | CONFIGURE_RUN_FAIL | configure: error: libdb_cxx headers missing |
| digitalocean/hivex | CONFIGURE_RUN_FAIL | not found |
| digitalocean/libguestfs | CONFIGURE_RUN_FAIL | not found |
| digitalocean/ovs | CONFIGURE_RUN_FAIL | ./configure: line 3065: string.h: command not found |
| dillo-browser/dillo | CONFIGURE_RUN_FAIL | configure: error: C++ compiler doesn't work |
| Distrotech/celt | CONFIGURE_RUN_FAIL | ./configure: line 1496: syntax error near unexpected token `tools="tools",' |
| Distrotech/diffutils | CONFIGURE_RUN_FAIL | configure: error: The --with-packager-{bug-reports,version} options require --with-package |
| Distrotech/esound | CONFIGURE_RUN_FAIL | checking for ARTS artsc - version >= 0.9.5... ./configure: line 1954: --cflags: command no |
| Distrotech/evolution | CONFIGURE_RUN_FAIL | ./configure: line 5753: intltool-update: command not found |
| Distrotech/findutils | CONFIGURE_RUN_FAIL | ./configure: line 1539: AM_C_PROTOTYPES: command not found |
| Distrotech/flac | CONFIGURE_RUN_FAIL | ./configure: line 2041: --cflags: command not found |
| Distrotech/gdbm | CONFIGURE_RUN_FAIL | checking for msgfmt... ./configure: line 1552: syntax error near unexpected token `newline |
| Distrotech/gtkimageview | CONFIGURE_RUN_FAIL | ./configure: line 1384: GNOME_COMMON_INIT: command not found |
| Distrotech/gzip | CONFIGURE_RUN_FAIL | configure: error: <wchar.h> cannot be used with this compiler (cc  ). |
| Distrotech/libcaca | CONFIGURE_RUN_FAIL | ./configure: line 1392: AC_LIBTOOL_CXX: command not found |
| Distrotech/libcddb | CONFIGURE_RUN_FAIL | ./configure: line 1374: syntax error near unexpected token `fi' |
| Distrotech/libdvdcss | CONFIGURE_RUN_FAIL | ./configure: line 1430: syntax error near unexpected token `do' |
| Distrotech/libgweather | CONFIGURE_RUN_FAIL | ./configure: line 15917: intltool-update: command not found |
| Distrotech/libmad | CONFIGURE_RUN_FAIL |  |
| Distrotech/libsecret | CONFIGURE_RUN_FAIL | ./configure: line 1416: intltool-update: command not found |
| Distrotech/libspectre | CONFIGURE_RUN_FAIL | ./configure: line 1400: AC_STDC_HEADERS: command not found |
| Distrotech/libtool | CONFIGURE_RUN_FAIL | ./configure: line 1644: syntax error near unexpected token `_LT_SYS_DYNAMIC_LINKER' |
| Distrotech/libwnck | CONFIGURE_RUN_FAIL | ./configure: line 4081: intltool-update: command not found |
| Distrotech/madplay | CONFIGURE_RUN_FAIL | checking for msgfmt... ./configure: line 1627: syntax error near unexpected token `newline |
| Distrotech/minicom | CONFIGURE_RUN_FAIL | ./configure: line 1902: syntax error near unexpected token `iconv,abcdefghijklmnopqrstuvwx |
| Distrotech/pulseaudio | CONFIGURE_RUN_FAIL | configure: error: git-version-gen failed |
| Distrotech/sharutils | CONFIGURE_RUN_FAIL | configure: error: <wchar.h> cannot be used with this compiler (cc  ). |
| Distrotech/squid | CONFIGURE_RUN_FAIL | ./configure: line 1409: again,: command not found |
| Distrotech/tar | CONFIGURE_RUN_FAIL | configure: error: The --with-packager-{bug-reports,version} options require --with-package |
| Distrotech/Thunar | CONFIGURE_RUN_FAIL | ./configure: line 6057: intltool-update: command not found |
| dividendcash/DividendCash | CONFIGURE_RUN_FAIL | configure: error: libdb_cxx headers missing |
| djn3m0/debit | CONFIGURE_RUN_FAIL | ./configure: line 1392: AX_CHECK_ALIGNED_ACCESS_REQUIRED: command not found |
| dkosmari/libwupsxx | CONFIGURE_RUN_FAIL | checking devkitPro path... not found |
| dkosmari/Papaya-HUD | CONFIGURE_RUN_FAIL | checking devkitPro path... not found |
| dleonard0/pktstat | CONFIGURE_RUN_FAIL | checking for library containing socket... ./configure: line 1408: syntax error near unexpe |
| dmalhotra/pvfmm | CONFIGURE_RUN_FAIL | ./configure: line 1431: syntax error near unexpected token `here,' |
| dmp0x7c5/gobexfuse | CONFIGURE_RUN_FAIL | configure: error: libical is required |
| dns-stats/hedgehog | CONFIGURE_RUN_FAIL | ./configure: line 1995: syntax error near unexpected token `else' |
| dnstap/dnstap-ldns | CONFIGURE_RUN_FAIL | configure: error: The protoc-c program was not found. Please install the protobuf-c compil |
| doug65536/dgos | CONFIGURE_RUN_FAIL | ./configure: line 5238: syntax error: unexpected end of file |
| dreal-deps/gsl | CONFIGURE_RUN_FAIL | ./configure: line 1464: LT_LIB_M: command not found |
| dreal-deps/libunwind | CONFIGURE_RUN_FAIL | ./configure: line 1426: CHECK_ATOMIC_OPS: command not found |
| dreamcat4/php-fpm | CONFIGURE_RUN_FAIL | checking for php configuration... configure: error: Please specify full path to php source |
| DreamSourceLab/DSLogic-fw | CONFIGURE_RUN_FAIL | configure: error: cannot find sdcc. |
| dreibh/sctplib | CONFIGURE_RUN_FAIL | checking whether this OS does have IPv6 stack... ./configure: line 1731: syntax error near |
| drewc/guix | CONFIGURE_RUN_FAIL |  |
| droogie/bluez-fuzzer | CONFIGURE_RUN_FAIL | configure: error: libical is required |
| drothlis/gstreamer | CONFIGURE_RUN_FAIL | ./configure: line 1353: AG_GST_INIT: command not found |
| dtbartle/cgminer-gc3355 | CONFIGURE_RUN_FAIL | ./configure: line 2436: syntax error near unexpected token `)' |
| duality-solutions/Dynamic-GPU-Miner-Nvidia | CONFIGURE_RUN_FAIL | configure: error: OpenSSL library required |
| dvab-sarma/android_external_alsa-lib | CONFIGURE_RUN_FAIL | ./configure: line 2433: syntax error near unexpected token `[_$as_cr_alnum]*_cv_[_$as_cr_a |
| dyne/Freecoin | CONFIGURE_RUN_FAIL | ./configure: line 2520: 0: command not found |
| dyne/FreeJ | CONFIGURE_RUN_FAIL | configure: error: *** Theora development files not found! |
| dyninc/OpenBFDD | CONFIGURE_RUN_FAIL |  |
| e-desouza/gzip-1.11 | CONFIGURE_RUN_FAIL | configure: error: <wchar.h> cannot be used with this compiler (cc  ). |
| e2guardian/e2guardian | CONFIGURE_RUN_FAIL |  |
| eastzone/snmp | CONFIGURE_RUN_FAIL | if libssl is wanted... if libssl wants a prefix... if libtomcrypt is wanted... if libtomcr |
| ebosnjak/libpng-1.5.4-vuln | CONFIGURE_RUN_FAIL | ./configure: line 1370: AC_PROG_LD: command not found |
| ecairn/sphinx-official | CONFIGURE_RUN_FAIL | ./configure: line 1341: syntax error near unexpected token `checking' |
| echiu64/gutenprint | CONFIGURE_RUN_FAIL | ./configure: line 1362: STP_INIT: command not found |
| edf-hpc/pkg-nsca-ng | CONFIGURE_RUN_FAIL | ./configure: line 1514: syntax error near unexpected token `fi' |
| edrosten/tag | CONFIGURE_RUN_FAIL | configure: error: TooN is not optional. Use --with-TooN=dir to specify where it can be fou |
| eeight/tdheap | CONFIGURE_RUN_FAIL | configure: error: Valgrind relies on GCC to be compiled |
| efficient/memc3 | CONFIGURE_RUN_FAIL | configure: error: libevent is required.  You can get it from http://www.monkey.org/~provos |
| eiichiroi/autotools-unittest | CONFIGURE_RUN_FAIL | ./configure: line 1569: pthread-config: command not found |
| ekmett/jitplusplus | CONFIGURE_RUN_FAIL | ./configure: line 1661: pthread-config: command not found |
| ekpyron/oclp | CONFIGURE_RUN_FAIL | ./configure: line 1405: syntax error near unexpected token `;' |
| elbandi/lighttpd | CONFIGURE_RUN_FAIL | ./configure: line 3032: syntax error near unexpected token `else' |
| electimon/bmp | CONFIGURE_RUN_FAIL |  |
| electronoora/komposter | CONFIGURE_RUN_FAIL | checking if Freetype2 is ok... configure: error: no |
| ElvishArtisan/lwcore | CONFIGURE_RUN_FAIL | configure: error: *** Qt4 not found *** |
| Elzair/nazghul | CONFIGURE_RUN_FAIL |  |
| emcrisostomo/fswatch | CONFIGURE_RUN_FAIL | for C compiler vendor... for C compiler version... ./configure: line 1736: syntax error ne |
| EmeraldMiningCo/Ebits | CONFIGURE_RUN_FAIL | configure: error: cannot figure out how to use std::atomic |
| enba94yf/binutils-2.42 | CONFIGURE_RUN_FAIL | ./configure: line 1367: ACX_LARGEFILE: command not found |
| endlessm/eos-knowledge-lib | CONFIGURE_RUN_FAIL | ./configure: line 1429: syntax error near unexpected token `1.30' |
| endlessm/eos-shard | CONFIGURE_RUN_FAIL | ./configure: line 1370: syntax error near unexpected token `(' |
| endlessm/flatpak-builder | CONFIGURE_RUN_FAIL | ./configure: line 1364: LIBGLNX_CONFIGURE: command not found |
| Enigma-Game/Enigma | CONFIGURE_RUN_FAIL | configure: error: SDL2_mixer is required to compile Enigma |
| enki/gvpe | CONFIGURE_RUN_FAIL | checking for msgfmt... ./configure: line 1414: syntax error near unexpected token `newline |
| envytools/valgrind | CONFIGURE_RUN_FAIL | configure: error: Valgrind relies on GCC to be compiled |
| ep-infosec/33_apache_duo_unix | CONFIGURE_RUN_FAIL |  |
| epfl-dias/shore-mt | CONFIGURE_RUN_FAIL | ./configure: line 1388: AC_REQUIRE_CPP: command not found |
| epruesse/SINA | CONFIGURE_RUN_FAIL | configure: error: libz.so not found! |
| equalitie/gnunet | CONFIGURE_RUN_FAIL |  |
| erikarn/libevent-adrian | CONFIGURE_RUN_FAIL | checking for socklen_t... checking whether our compiler supports __func__... ./configure:  |
| erthink/ReOpenLDAP | CONFIGURE_RUN_FAIL | checking configure arguments... ./configure: line 1570: syntax error near unexpected token |
| ESiWACE/esdm-netcdf-4.6.2-old | CONFIGURE_RUN_FAIL | configure: error: curl required for remote access. Install curl or build with --disable-da |
| esrf-bliss/CCfits | CONFIGURE_RUN_FAIL | ./configure: line 1381: PFK_CXX_LIB_PATH: command not found |
| esrille/escudo | CONFIGURE_RUN_FAIL | configure: error: Cannot find bison; bison parser generator is needed. |
| ester-project/ester | CONFIGURE_RUN_FAIL | ./configure: line 1557: --ldflags: command not found |
| esy-packages/esy-automake | CONFIGURE_RUN_FAIL | ./configure: line 1338: and: command not found |
| etr/libhttpserver | CONFIGURE_RUN_FAIL | configure: error: "you must configure in a separate build directory" |
| evergreen-library-system/Evergreen | CONFIGURE_RUN_FAIL | configure: error: Could not find osrf_config. |
| evilnet/x3 | CONFIGURE_RUN_FAIL | checking for time_t ... configure: error: Cannot detect format string for time_t |
| ewxrjk/with-readline | CONFIGURE_RUN_FAIL | ./configure: line 1440: AC_SET_MAKE: command not found |
| excamera/alfalfa | CONFIGURE_RUN_FAIL | configure: error: Unable to find libjpeg. |
| Expensify/mk_livestatus | CONFIGURE_RUN_FAIL | configure: error: unable to find the rrd_xport function |
| experiencecoin/experiencecoin_legacy | CONFIGURE_RUN_FAIL | configure: error: libdb_cxx headers missing |
| facchinm/avrdude | CONFIGURE_RUN_FAIL | ./configure: line 1472: syntax error near unexpected token `AR,' |
| fairflow/espeak-ng-pt-br | CONFIGURE_RUN_FAIL | ./configure: line 1654: syntax error near unexpected token `for' |
| fandangos/libbluray | CONFIGURE_RUN_FAIL | ./configure: line 1645: AC_STRUCT_DIRENT_D_TYPE: command not found |
| farsightsec/axa | CONFIGURE_RUN_FAIL | configure: error: required library not found |
| farsightsec/mtbl | CONFIGURE_RUN_FAIL | configure: error: liblz4 >= r130 required |
| farsightsec/sie-nmsg | CONFIGURE_RUN_FAIL | configure: error: The protoc-c program was not found. Please install the protobuf-c compil |
| FauxFaux/fastjar | CONFIGURE_RUN_FAIL | ./configure: line 1811: gl_00GNULIB: command not found |
| fedoracoin-dev/fedoracoin | CONFIGURE_RUN_FAIL | configure: error: libdb_cxx headers missing |
| felix-001/tstool | CONFIGURE_RUN_FAIL | ./configure: line 1399: AC_STDC_HEADERS: command not found |
| fengye/swfdec | CONFIGURE_RUN_FAIL | configure: error: liboil-0.3 >= 0.3.1 is required to build swfdec |
| ferrandi/PandA-bambu | CONFIGURE_RUN_FAIL | ./configure: line 1469: AC_COMPILE_STDCXX_17: command not found |
| fhcrc/mcl | CONFIGURE_RUN_FAIL | _____ weak test for C void* <=> unsigned int conversion... ./configure: line 1394: [ac_cv_ |
| FinchBerryOS/fbyo-coreutils | CONFIGURE_RUN_FAIL | configure: error: libsmack library was not found or not usable |
| finit-project/finit | CONFIGURE_RUN_FAIL | ./configure: line 1665: syntax error near unexpected token `alsa-utils,-a-z,_A-Z' |
| firecore/libbluray | CONFIGURE_RUN_FAIL | ./configure: line 1630: AC_STRUCT_DIRENT_D_TYPE: command not found |
| firehol/firehol | CONFIGURE_RUN_FAIL | ./configure: line 2159: syntax error near unexpected token `else' |
| firoorg/cpuminer | CONFIGURE_RUN_FAIL | configure: error: OpenSSL crypto library required |
| Firstyear/ds_rust | CONFIGURE_RUN_FAIL | configure: error: Rust |
| fix8/fix8 | CONFIGURE_RUN_FAIL | ./configure: line 1453: HAVE_SYS_TIME_H: command not found |
| fizx/parsley | CONFIGURE_RUN_FAIL | configure: error: could not find pcre |
| flame/tblis-strassen | CONFIGURE_RUN_FAIL | ./configure: line 1389: AC_F77_LIBRARY_LDFLAGS: command not found |
| flatpak/ppa-xdg-desktop-portal | CONFIGURE_RUN_FAIL | checking for msgfmt... ./configure: line 1543: syntax error near unexpected token `newline |
| flboudet/flobz | CONFIGURE_RUN_FAIL | checking the location of hash_map... ./configure: line 2005: syntax error near unexpected  |
| flightaware/tclreadline | CONFIGURE_RUN_FAIL | checking which tclConfig.sh to use... configure: error: Can't find Tcl libraries.  Use --w |
| flowgrind/flowgrind | CONFIGURE_RUN_FAIL | ./configure: line 2027: AC_TYPE_UNSIGNED_LONG_LONG_INT: command not found |
| flux-framework/flux-foundry | CONFIGURE_GEN_FAIL |  |
| flux-framework/flux-pmix | CONFIGURE_RUN_FAIL | ./configure: line 597: syntax error near unexpected token `(' |
| flux-framework/flux-security | CONFIGURE_RUN_FAIL | ./configure: line 597: syntax error near unexpected token `(' |
| fmrico/libpng16 | CONFIGURE_RUN_FAIL | ./configure: line 3379: syntax error: unexpected end of file |
| FNCS/fncs | CONFIGURE_RUN_FAIL | ./configure: line 1986: syntax error near unexpected token `[_AH_CHECK_HEADER' |
| fomy/destor | CONFIGURE_RUN_FAIL | configure: error: *** Working glib library not found *** |
| fontforge/libspiro | CONFIGURE_RUN_FAIL | configure: error: ERROR: Please install Math libraries and math.h include files for libm |
| fork4jl/mpfr | CONFIGURE_RUN_FAIL | ./configure: line 1374: is: command not found |
| formorer/pkg-keepalived | CONFIGURE_RUN_FAIL | Package libiptc was not found in the pkg-config search path. |
| fossci/libgcrypt | CONFIGURE_RUN_FAIL | ./configure: line 1352: mym4_package,mym4_version,https://bugs.gnupg.org: No such file or  |
| fourmond/dvdcopy | CONFIGURE_RUN_FAIL | configure: error: cannot link to dvdread |
| fractalcoin/fractalcoin | CONFIGURE_RUN_FAIL | configure: error: libdb_cxx headers missing |
| FredericJacobs/obfsproxy-c | CONFIGURE_RUN_FAIL | for library containing ntohl... rm: cannot remove 'conftest.c': No such file or directory |
| Freeaqingme/libvmod-oauth | CONFIGURE_RUN_FAIL | configure: error: No Varnish source tree specified |
| freebsd/atf | CONFIGURE_RUN_FAIL | ./configure: line 1355: syntax error near unexpected token `(' |
| freedesktop-unofficial-mirror/dbus__dbus-qt3 | CONFIGURE_RUN_FAIL | ./configure: line 1790: syntax error near unexpected token `fi' |
| freeipa/bind-dyndb-ldap | CONFIGURE_RUN_FAIL | checking libdns version... configure: error: Can't obtain libdns version. |
| FreeMCU/freemcu | CONFIGURE_RUN_FAIL | ./configure: line 1675: ./libs/ptlib/configure: No such file or directory |
| FreeRADIUS/freeradius-client | CONFIGURE_RUN_FAIL | checking gethostbyaddr_r() syntax... ./configure: line 1462: syntax error near unexpected  |
| frida/xz | CONFIGURE_RUN_FAIL | configure: error:  support not found |
| fries/android-external-openvpn | CONFIGURE_RUN_FAIL | ./configure: line 2804: syntax error near unexpected token `fi' |
| fripon/freeture | CONFIGURE_RUN_FAIL | ******** Configuring:  freeture  (2026-07-03) ******** |
| frugalware/pacman-g2 | CONFIGURE_RUN_FAIL | configure: error: Your architecture is not supported |
| FundacionPesetacoin/PesetacoinCore | CONFIGURE_RUN_FAIL | configure: error: libdb_cxx headers missing |
| futurerestore/idevicerestore | CONFIGURE_RUN_FAIL | configure: error: You need either strsep or strcspn to build |
| GabrielDosReis/open-axiom | CONFIGURE_RUN_FAIL | configure: error: OpenAxiom requires a Lisp system.  Either separately build one (GCL-2.6. |
| Gahznt/otserver_source | CONFIGURE_RUN_FAIL | configure: error: "Linking against GMP failed." |
| gajgeospatial/libpng-1.6.40 | CONFIGURE_RUN_FAIL | ./configure: line 3329: syntax error: unexpected end of file |
| GalliumOS/xfce4-session | CONFIGURE_RUN_FAIL | ./configure: line 1518: AC_CHECK_LIBM: command not found |
| ganehag/open-modbusgateway | CONFIGURE_RUN_FAIL | ./configure: line 1355: COMPILER_FLAGS: command not found |
| gapcoin/gapcoin | CONFIGURE_RUN_FAIL | configure: error: libdb_cxx headers missing |
| gbonacini/trollfs | CONFIGURE_RUN_FAIL | configure: error: could not find lib FUSE |
| gderosa/dansguardian | CONFIGURE_RUN_FAIL |  |
| gdnsd/gdnsd | CONFIGURE_RUN_FAIL | ./configure: line 1589: -fstack-protector-strong: command not found |
| GeGuNa/trafficserver | CONFIGURE_RUN_FAIL |  |
| genesi/gnome-terminal | CONFIGURE_RUN_FAIL | ./configure: line 1406: GNOME_COMMON_INIT: command not found |
| genome-vendor/gmap-gsnap | CONFIGURE_RUN_FAIL | checking for pthreads feature... ./configure: line 1806: syntax error near unexpected toke |
| gentoo/portage-utils | CONFIGURE_RUN_FAIL | configure: error: |
| gentoo/sandbox | CONFIGURE_RUN_FAIL | ./configure: line 1685: syntax error near unexpected token `dlfcn.h' |
| geodynamics/citcoms | CONFIGURE_RUN_FAIL | ./configure: line 1364: syntax error near unexpected token `auto' |
| geoffjay/libgnt | CONFIGURE_RUN_FAIL | ./configure: line 1454: --variable=g_ir_scanner: command not found |
| geomview/geomview | CONFIGURE_RUN_FAIL | ./configure: command substitution: line 1552: syntax error: unexpected end of file |
| ghani-1977/enigma2-openpli-sh4-2 | CONFIGURE_RUN_FAIL | configure: error: Could not find crypt |
| gigaflow-vswitch/gvs | CONFIGURE_RUN_FAIL | ./configure: line 3576: string.h: command not found |
| giltirn/HPCortex | CONFIGURE_RUN_FAIL | checking for c++17 compatibility... configure: error: Could |
| gingi/fastbit | CONFIGURE_RUN_FAIL | checking if compiler supports __sync_add_and_fetch for 32-bit integers... checking if comp |
| GiterMirror/mpg123 | CONFIGURE_RUN_FAIL | ./configure: line 1498: AC_INLINE: command not found |
| gitGNU/gnu_ccd2cue | CONFIGURE_RUN_FAIL | awk: cmd. line:1:                                   ^ syntax error |
| gitGNU/gnu_foliot | CONFIGURE_RUN_FAIL | configure: error: found development files for Guile 2.2,, but /usr/bin/guile has effective |
| gitGNU/gnu_ld | CONFIGURE_RUN_FAIL | ./configure: line 1416: ACX_LARGEFILE: command not found |
| giuseppe/containers-dedup | CONFIGURE_RUN_FAIL | ./configure: line 2187: syntax error near unexpected token `)' |
| glebius/minidlna | CONFIGURE_RUN_FAIL | ./configure: line 1406: syntax error near unexpected token `(' |
| GNOME/dasher | CONFIGURE_RUN_FAIL | ./configure: line 1387: AC_PROG_LD_GNU: command not found |
| GNOME/easytag | CONFIGURE_RUN_FAIL | configure: error: appdata-tools is required for appdata-xml.m4 |
| GNOME/gnumeric | CONFIGURE_RUN_FAIL | ./configure: line 1388: GTK_DOC_CHECK: command not found |
| GNOME/goffice | CONFIGURE_RUN_FAIL | ./configure: line 1617: syntax error near unexpected token `;' |
| gnosis/libunistring | CONFIGURE_RUN_FAIL | ./configure: line 3420: syntax error: unexpected end of file |
| gnu-smalltalk/smalltalk | CONFIGURE_RUN_FAIL | whether  produces any warnings... ./configure: line 1615: syntax error near unexpected tok |
| GNUAspell/aspell | CONFIGURE_RUN_FAIL | checking if file locking and truncating is supported... checking if mmap and friends is su |
| GNUFreetalk/freetalk | CONFIGURE_RUN_FAIL | configure: error: ERROR! readline not found.. |
| GNUnet-Mirror/GNUnet | CONFIGURE_RUN_FAIL |  |
| goatattack/goatattack | CONFIGURE_RUN_FAIL | configure: error: missing libraries:  libz (you can use --enable-internal-zlib) |
| gobolinux/GoboHide | CONFIGURE_RUN_FAIL | configure: error: generic netlink command line interface library (libnl-cli) not found |
| golems/ach | CONFIGURE_RUN_FAIL | configure: error: process-shared condition variables are required |
| golems/amino | CONFIGURE_RUN_FAIL | configure: error: BLAS is required. |
| golosio/NeuronGPU | CONFIGURE_RUN_FAIL | ./configure: line 2820: /patch.sh: No such file or directory |
| goodspeed34/ws63flash | CONFIGURE_RUN_FAIL | configure: error: |
| google/certificate-transparency | CONFIGURE_RUN_FAIL | ./configure: line 3325: syntax error: unexpected end of file |
| google/hiba | CONFIGURE_RUN_FAIL | configure: error: |
| Gotos/CuteCapture | CONFIGURE_RUN_FAIL | ./configure: line 1476: syntax error near unexpected token `)' |
| gpac-buildbot/libmad | CONFIGURE_RUN_FAIL |  |
| gpg/gpgme | CONFIGURE_RUN_FAIL | ./configure: line 1352: mym4_package,mym4_version,https://bugs.gnupg.org: No such file or  |
| gpg/libassuan | CONFIGURE_RUN_FAIL | ./configure: line 1352: mym4_package,mym4_version,https://bugs.gnupg.org: No such file or  |
| gpg/libgcrypt | CONFIGURE_RUN_FAIL | ./configure: line 1352: mym4_package,mym4_version,https://bugs.gnupg.org: No such file or  |
| gpg/scute | CONFIGURE_RUN_FAIL | ./configure: line 1352: mym4_package,mym4_version,https://bugs.gnupg.org: No such file or  |
| GPGTools/pinentry | CONFIGURE_RUN_FAIL | ./configure: line 2022: syntax error near unexpected token `else' |
| gphoto/libgphoto2-python | CONFIGURE_RUN_FAIL | ./configure: line 1393: GP_CHECK_SHELL_ENVIRONMENT: command not found |
| gpudirect/libgdsync | CONFIGURE_RUN_FAIL | configure: error: cuStreamBatchMemOp() not found.  libgdsync requires CUDA 8.0 or later. |
| gpudirect/libibverbs | CONFIGURE_RUN_FAIL | ./configure: line 1466: syntax error near unexpected token `;' |
| graemes/poolparty-x16r | CONFIGURE_RUN_FAIL | configure: error: OpenSSL library required |
| gramseyer/hotstuff | CONFIGURE_RUN_FAIL | configure: error: failed to find XDRC |
| graydon/monotone | CONFIGURE_RUN_FAIL | whether xgettext supports --flag... ./configure: line 1681: syntax error near unexpected t |
| GREO/gnuradio-git | CONFIGURE_RUN_FAIL | ./configure: line 1440: GR_VERSION: command not found |
| GridOPTICS/FNCS | CONFIGURE_RUN_FAIL | ./configure: line 2303: syntax error near unexpected token `[_AH_CHECK_HEADER' |
| grinsfem/grins | CONFIGURE_RUN_FAIL | ./configure: line 1365: AX_SPLIT_VERSION: command not found |
| grke/burp | CONFIGURE_RUN_FAIL | configure: error: Unable to find OpenSSL library |
| groonga/groonga | CONFIGURE_RUN_FAIL | ./configure: line 2338: ./version.sh: No such file or directory |
| grrrr/flext | CONFIGURE_RUN_FAIL | configure: error: path to system SDK headers required |
| gssapi/mod_auth_gssapi | CONFIGURE_RUN_FAIL | configure: error: Cannot find pkg-config. Please install pkg-config. |
| gszura/wx-nfp | CONFIGURE_RUN_FAIL | ./configure: line 1387: AM_OPTIONS_WXCONFIG: command not found |
| gvvaughan/slingshot | CONFIGURE_RUN_FAIL | for a Lua interpreter with version >= 5.1, < 5.4... configure: error: cannot find suitable |
| gvz/avrdude | CONFIGURE_RUN_FAIL | ./configure: line 1470: syntax error near unexpected token `AR,' |
| gyoto/Gyoto | CONFIGURE_RUN_FAIL | ./configure: line 2078: syntax error near unexpected token `else' |
| gypified/libmpg123 | CONFIGURE_RUN_FAIL | ./configure: line 1498: AC_INLINE: command not found |
| h0tw1r3/libuuid-mingw | CONFIGURE_RUN_FAIL | checking for UUID Library... configure: error: There isn't Microsoft UUID Library. |
| hackerschoice/gsocket-relay | CONFIGURE_RUN_FAIL | configure: error: libgsocket not found. Compile gsocket in ./gsocket or use --with-gsocket |
| haegrr/reprepro | CONFIGURE_RUN_FAIL | configure: error: Missing mkstemp or mkostemp |
| hallyn/upstart | CONFIGURE_RUN_FAIL | ./configure: line 1353: syntax error near unexpected token `[Copyright' |
| hamonikr-root/fontconfig | CONFIGURE_RUN_FAIL | ./configure: line 1945: AC_C_FLEXIBLE_ARRAY_MEMBER: command not found |
| hamonikr-root/mate-screensaver | CONFIGURE_RUN_FAIL | ./configure: line 1380: intltool-update: command not found |
| handshake-org/hnsd | CONFIGURE_RUN_FAIL | ./configure: line 3268: syntax error: unexpected end of file |
| HansWessels/gup | CONFIGURE_RUN_FAIL | ./configure: line 1381: AC_CYGWIN: command not found |
| haproxytech/spoa-mirror | CONFIGURE_RUN_FAIL | *** configuring for m4_esyscmd_s(basename ${PWD}) vm4_esyscmd_s(cat VERSION) *** |
| Harvard-PRINCESS/sablevm | CONFIGURE_RUN_FAIL | ./configure: line 1979: ./src/libffi/configure: No such file or directory |
| haussli/rancid | CONFIGURE_RUN_FAIL | ./configure: line 1416: AC_INCLUDES_DEFAULT: command not found |
| hb/claws_mail_opensync_plugin | CONFIGURE_RUN_FAIL | checking for msgfmt... ./configure: line 1544: syntax error near unexpected token `newline |
| hb9xar/siproxd | CONFIGURE_RUN_FAIL | ./configure: line 1453: syntax error near unexpected token `recursive' |
| HDFGroup/vol-log-based | CONFIGURE_RUN_FAIL | configure: error: |
| hdoddikindi/fstrm | CONFIGURE_RUN_FAIL | ./configure: line 1859: []: command not found |
| HeapStats/heapstats | CONFIGURE_RUN_FAIL | configure: error: ant not found. |
| heliocastro/gpgme | CONFIGURE_RUN_FAIL | ./configure: line 1352: mym4_package,mym4_version,: command not found |
| hello31337/BI-SGX | CONFIGURE_RUN_FAIL | configure: error: Need OpenSSL v1.1.0 or later |
| hercules-390/hyperion | CONFIGURE_RUN_FAIL | configure: error: in source build detected, aborting configure... |
| hermansr/psid64 | CONFIGURE_RUN_FAIL | ./configure: line 1478: AX_FUNC_MKDIR: command not found |
| hermet/enventor | CONFIGURE_RUN_FAIL | ./configure: line 1575: m4_translit([eet-eet], -A-Z, _a-z): syntax error in expression (er |
| HewlettPackard/nagios-plugins-hpilo | CONFIGURE_RUN_FAIL | ./configure: line 2606: syntax error: unexpected end of file |
| HewlettPackard/netperf | CONFIGURE_RUN_FAIL | checking for socklen_t equivalent... ./configure: line 1491: syntax error near unexpected  |
| hexagonal-sun/bic | CONFIGURE_RUN_FAIL | ./configure: line 1436: AX_LIB_READLINE: command not found |
| hexhex/mergingplugin | CONFIGURE_RUN_FAIL | ./configure: line 2444: syntax error near unexpected token `DLVHEX_USERPLUGINDIR,variable= |
| hfiguiere/libopenraw | CONFIGURE_RUN_FAIL | ./configure: line 2865: syntax error: unexpected end of file |
| hfst/hfst-ospell | CONFIGURE_RUN_FAIL | configure: error: libxml++ failed |
| hgneng/ekho | CONFIGURE_RUN_FAIL | configure: error: espeak-ng test failed |
| hgst/libnvme | CONFIGURE_RUN_FAIL | ./configure: line 1544: pthread-config: command not found |
| hharte/tn5250 | CONFIGURE_RUN_FAIL | configure: error: ** You need a curses-compatible library installed. |
| hhirsch/abook | CONFIGURE_RUN_FAIL | checking for msgfmt... ./configure: line 1455: syntax error near unexpected token `newline |
| hhool/nut | CONFIGURE_RUN_FAIL | ./configure: line 1899: syntax error near unexpected token `inplace-runtime,' |
| hightman/xunsearch | CONFIGURE_RUN_FAIL | configure: error: scws.h NOT FOUND. please check your scws install directory. >= 1.1.6 |
| hiha-lang/hiha | CONFIGURE_RUN_FAIL | Try 0 --help for more information.: syntax error in expression (error token is "Try 0 --he |
| hkerem/squid3-ssl | CONFIGURE_RUN_FAIL | ./configure: line 1854: LTDL_INIT: command not found |
| hnwfs/lighttpd-plus | CONFIGURE_RUN_FAIL | ./configure: line 3046: syntax error near unexpected token `else' |
| hollowiette/xrdp | CONFIGURE_RUN_FAIL | ./configure: line 1386: ./libpainter/configure: No such file or directory |
| holzschu/lib-tex | CONFIGURE_RUN_FAIL | ./configure: line 1504: syntax error near unexpected token `;;' |
| HomerReid/buff-em | CONFIGURE_RUN_FAIL | ./configure: line 1425: AC_F77_WRAPPERS: command not found |
| hongyi-zhao/lyx | CONFIGURE_RUN_FAIL | ./configure: line 1367: LYX_CHECK_VERSION: command not found |
| horms/ovs | CONFIGURE_RUN_FAIL | configure: error: bad value  for --enable-coverage |
| hpc/cce-mpi-openmpi-1.4.4 | CONFIGURE_RUN_FAIL | ./configure: line 1359: OMPI_LOAD_PLATFORM: command not found |
| hpc/Parallel-coreutils | CONFIGURE_RUN_FAIL | configure: error: The --with-packager-{bug-reports,version} options require --with-package |
| hpc/supermagic | CONFIGURE_RUN_FAIL | configure: error: cc cannot compile MPI applications. cannot continue. |
| hroptatyr/clob | CONFIGURE_RUN_FAIL | checking whether C compiler accepts -std=gnu11... ./configure: line 1478: AC_LANG_WERROR:  |
| hroptatyr/echse | CONFIGURE_RUN_FAIL | checking whether C compiler accepts -std=c11... ./configure: line 1491: AC_LANG_WERROR: co |
| hroptatyr/truffle | CONFIGURE_RUN_FAIL | checking whether C compiler accepts -std=gnu11... ./configure: line 1478: AC_LANG_WERROR:  |
| hroptatyr/yuck | CONFIGURE_RUN_FAIL | checking whether C compiler accepts -std=c11... ./configure: line 1478: AC_LANG_WERROR: co |
| htrb/ngraph-gtk | CONFIGURE_RUN_FAIL | configure: error: (Test for GtkSourceview failed.) |
| huceke/xine-lib-vaapi | CONFIGURE_RUN_FAIL | ./configure: line 1663: LT_AC_PROG_SED: command not found |
| huleyv/iperf2 | CONFIGURE_RUN_FAIL | ./configure: line 3212: syntax error near unexpected token `struct' |
| hunter-packages/libmicrohttpd | CONFIGURE_RUN_FAIL | configure: error: Package requirements () were not met: |
| hyperrealm/cbase | CONFIGURE_RUN_FAIL | checking for socklen_t in sys/socket.h or unistd.h... ./configure: line 1649: syntax error |
| iagorubio/cssed-vte-plugin | CONFIGURE_RUN_FAIL | configure: error: headers are not in /usr/include/cssed/ please use the same prefix you us |
| iamashwin99/octopus-debian-package | CONFIGURE_RUN_FAIL | how to get verbose linking output from g++ -std=c++14... ./configure: line 1569: syntax er |
| IanSav/enigma2-Beyonwiz | CONFIGURE_RUN_FAIL | configure: error: |
| ibm-power-utilities/powerpc-utils | CONFIGURE_RUN_FAIL | configure: error: librtas library is missing (--without-librtas to disable) |
| IBMSpectrumComputing/lsf-drmaa | CONFIGURE_RUN_FAIL | ./configure: line 1769: pthread-config: command not found |
| idiap/juicer | CONFIGURE_RUN_FAIL | configure: error: Required library TRACTER not found |
| iem-projects/ncview | CONFIGURE_RUN_FAIL | ./configure: line 1355: AC_PATH_NETCDF: command not found |
| ifwe/ucarp | CONFIGURE_RUN_FAIL | checking for msgfmt... ./configure: line 1486: syntax error near unexpected token `newline |
| igmhub/likely | CONFIGURE_RUN_FAIL | ./configure: line 1365: AX_EXT: command not found |

## ⚪ Non-working — not standalone (GNU autotools also fails; not our bug) (10)

| repo | stage |
| --- | --- |
| dkrotx/htmarkup | CONFIGURE_RUN_FAIL |
| DocQMiner/tesseract-4.0.0-beta.1 | CONFIGURE_RUN_FAIL |
| dudochkin-victor/sqlheavy | CONFIGURE_GEN_FAIL |
| ecliptchain/eclipt-source | CONFIGURE_RUN_FAIL |
| endlessm/basin | CONFIGURE_RUN_FAIL |
| epeec/TAGASPI | CONFIGURE_RUN_FAIL |
| evjeesm/hashset | CONFIGURE_RUN_FAIL |
| fangq/medit | CONFIGURE_RUN_FAIL |
| FinTP/fintp_payloadevaluators | CONFIGURE_RUN_FAIL |
| FOSSEE/scilab_for_xcos_on_cloud | CONFIGURE_RUN_FAIL |

