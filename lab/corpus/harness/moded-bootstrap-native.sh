#!/usr/bin/env bash
export PATH="/usr/bin:/bin" LC_ALL=C TZ=UTC0
export AUTOCONF_RS=/home/dev/acrs-autoconf AUTOHEADER_RS=/home/dev/acrs-autoheader ACLOCAL_RS=/home/dev/acrs-aclocal
export AUTOMAKE_RS="$HOME/automake-rs/target/release/automake"
ARS="$HOME/automake-rs/target/release/autoreconf-rs"
R=/home/dev/moded.tsv; : > "$R"; V=/home/dev/modedv; rm -rf "$V"; mkdir -p "$V"
rec(){ flock /home/dev/.modedlock -c "echo -e \"$1\" >> $R"; }
clone(){ for t in 1 2 3; do timeout 150 git clone --depth 1 -q "https://github.com/$1" "$2" 2>/dev/null && return 0; sleep 3; done; return 1; }
w(){
  repo="$1"; s=$(echo "$repo"|tr / _); d="$V/$s"; rm -rf "$d"
  clone "$repo" "$d" || { rec "$repo\tCLONE_FAIL"; return; }
  cd "$d" || return; rm -rf autom4te.cache
  { [ -f configure.ac ] || [ -f configure.in ]; } || { rec "$repo\tNO_CONFIGURE_AC"; return; }
  # the REAL native driver, zero GNU tools:
  timeout 150 "$ARS" -fi . >/tmp/md_boot_$s 2>&1
  [ -s configure ] || { rec "$repo\tCONFIGURE_GEN_FAIL"; return; }
  if ! timeout 200 ./configure >/tmp/md_cf_$s 2>&1; then rec "$repo\tCONFIGURE_RUN_FAIL"; return; fi
  if timeout 400 make -j2 >/tmp/md_mk_$s 2>&1; then rec "$repo\tFUNC_OK"; else rec "$repo\tMAKE_FAIL"; fi
  rm -rf "$d"
}
export -f w rec clone; export V ARS R PATH
cat /home/dev/modeb_set.txt | xargs -P 3 -I {} bash -c 'w "$@"' _ {}
echo "MODED_DONE ok=$(grep -c FUNC_OK $R)/$(wc -l <$R)" >> /home/dev/moded_progress.log
