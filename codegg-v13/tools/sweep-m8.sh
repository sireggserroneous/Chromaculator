#!/usr/bin/env bash
# v11-M8 sweep: one constant at a time over the CONTESTED rows; the winner
# config faces the full 20-file ledger. Architecture changed since v10's
# sweep (lattice inputs, priors, NINPUT 12) -- the re-sweep is a first-class
# lever (the v10 lesson, worth 0.3-1.3/row there).
set -u
cd "$(dirname "$0")/.."
FILES="corpus-real/notepad.exe corpus-real/segoeui.ttf corpus-real/arial.ttf corpus-real/kernel32.dll corpus-real/zstd.exe corpus-real/vim-version9.txt corpus-real/wubbadub.html corpus-real/real-test.db corpus-real/alarm01.wav corpus-real/ring01.wav corpus-big/cbs.log corpus-big/iconcache48.db"
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
set_const() { # name value
  sed -i "s/const $1: u32 = [0-9]*;/const $1: u32 = $2;/" src/mix11.rs
}
echo "== baseline (LR=11 ISSE=10 LIMIT=1023)"
run_config "base"
for v in 10 12; do
  set_const MIX10_LR $v; run_config "MIX10_LR=$v"; set_const MIX10_LR 11
done
for v in 9 11; do
  set_const ISSE_LR $v; run_config "ISSE_LR=$v"; set_const ISSE_LR 10
done
for v in 511 2047; do
  set_const SM_LIMIT $v; run_config "SM_LIMIT=$v"; set_const SM_LIMIT 1023
done
cargo build --release 2>/dev/null | true
rm -rf "$SP"
echo "sweep done (consts restored to base)"
