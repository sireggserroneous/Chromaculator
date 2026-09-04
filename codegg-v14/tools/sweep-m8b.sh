#!/usr/bin/env bash
set -u
cd /c/Users/vcepe/eggsprojects/Chromaculator/codegg-v11
FILES="corpus-real/notepad.exe corpus-real/segoeui.ttf corpus-real/arial.ttf corpus-real/kernel32.dll corpus-real/zstd.exe corpus-real/vim-version9.txt corpus-real/real-test.db corpus-big/iconcache48.db"
SP=$(mktemp -d)
run_config() {
  local label="$1"
  cargo build --release 2>/dev/null | true
  local total=0
  local line="$label:"
  for f in $FILES; do
    ./target/release/eggv11 transmute "$f" -o "$SP/x.egg11" >/dev/null 2>&1
    local sz=$(stat -c %s "$SP/x.egg11")
    total=$((total + sz))
    line="$line $(basename $f)=$sz"
  done
  echo "$line TOTAL=$total"
}
set_const() { sed -i "s/const $1: u32 = [0-9]*;/const $1: u32 = $2;/" src/mix11.rs; }
set_const ISSE_LR 8;  run_config "ISSE8";  set_const ISSE_LR 10
set_const ISSE_LR 7;  run_config "ISSE7";  set_const ISSE_LR 10
set_const ISSE_LR 9;  set_const MIX10_LR 12; run_config "ISSE9+LR12"; set_const ISSE_LR 10; set_const MIX10_LR 11
set_const MIX10_LR 13; run_config "LR13"; set_const MIX10_LR 11
set_const ISSE_LR 9; set_const SM_LIMIT 2047; run_config "ISSE9+SM2047"; set_const ISSE_LR 10; set_const SM_LIMIT 1023
set_const ISSE_LR 8; set_const MIX10_LR 12; run_config "ISSE8+LR12"; set_const ISSE_LR 10; set_const MIX10_LR 11
cargo build --release 2>/dev/null | true
rm -rf "$SP"
echo "round 2 done (base restored)"
