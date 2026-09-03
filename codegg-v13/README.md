# codegg-v13 — the Value Underneath

**IN FLIGHT.** This campaign is at **M3b**. M0 (the fork), M1 (the peel frame
plus the JPEG coefficient model), M2 (the deflate peel), M3a (the spelling
exception) and M3b (the three site readings, all three tried and all three
deleted) are done and measured; M3c (the model side), M3d (peeling deeper) and
M4 (the seal) are not started. Everything below is measured, and the misses are
printed beside the wins in `PREDICTIONS.md` -- including three filed predictions
that missed at M3b.

*(The failures-first rewrite of the body below belongs to M4; only this status
line was corrected at M3b, because it said M2 had not started.)*

eggv13 is still the Transmuter. Transmutation, not compression. The first law is
conservation and the FNV-64 of the original bytes gates every restore. The
charter verse is the site's own tooltip for `form` (spec.html:359-360):

> Plain is the digits as written; pushed respells the same value using −1s.
> **The value underneath does not change — compare the two and only the colours
> differ.**

## The thesis

A JPEG's Huffman bits and its quantised DCT coefficients are the same value in
two spellings. On several rows v12 modelled somebody else's spelling — the
colours — and called the result our form. v13 reads the value underneath: peel
the foreign code, model what it actually says, re-spell it byte-exactly on the
way out.

## THE LAW OF THE PEEL

1. **A peel is a bijection or it is not used.** At transmute time the peel
   re-encodes its own output and compares against the original bytes. One byte
   off and the peel is discarded for that file and the raw bytes go through the
   ordinary pipeline. The decoder only ever sees peels the encoder proved
   invertible on this exact input, before anything was written.
2. **The recipe rides inside** and is judged with the values, by argmin on the
   ARMORED total — never on inner bytes.
3. **Refuse, do not guess.** Progressive, arithmetic-coded, 12-bit, truncated,
   corrupt: keep the bytes and print the reason.
4. Every peel rides v12's write-time round-trip law, which v12-M3 paid for in
   blood (a stereo artifact no restore could read, eleven minutes live).

## What M1 built

- `src/peel.rs` — the frame. `MODEL_PEEL` (24) in the header's model byte; the
  payload opens with a 15-byte preamble (peel id, the recipe's model and
  lengths, the values' model and length), then the recipe stream, then the
  values stream. **ONE constant**, `PEEL_MAX`, bounds the id space and both
  sides read it.
- `src/jpeg.rs` — the JPEG peel: the v12 python probe ported to Rust. Every MCU
  Huffman-decoded, DC prediction reset at each restart, the file's own DQT/DHT
  and the whole marker skeleton kept **verbatim** in the recipe (what the stream
  said, never what a library would have said), re-encoded with the spec's
  padding. Baseline SOF0, 8-bit, 1..4 components, any sampling, restarts or
  none, interleaved or single-component.
- `src/jcoef.rs` — **the milestone**: the coefficient model. Contexts are the
  band index, the same position in the block above and to the left, the count of
  nonzeros already placed in the block, the quantisation value at that position,
  and the component; the DC is coded against a two-dimensional MED predictor on
  the DC plane rather than the JPEG's own one-dimensional difference. One
  adaptive 16-bit probability per context, no mixer (packJPG's shape, chosen for
  the speed floor). Encoder and decoder are **one routine** driven through a
  `Coder` trait, so the two sides cannot drift.

## The reading that made it necessary

v12 proved the door opens and then measured what was behind it. The RAW
coefficient dump of `wallpaper.jpg` — 64 × i16 per block, 55,296,000 B — is
**2,192,684 B** under `xz -9` and **2,208,961 B** under our own MIX12 arm, both
*heavier* than the JPEG's own 1,602,311 B of Huffman coding. **A dump is not a
model.** The same coefficients, modelled, are **1,233,099 B**.

## M1, measured

| | bytes |
|---|---|
| wallpaper.jpg | 1,602,752 (entropy-coded: 1,602,311) |
| recipe | 451 raw → **272** (the CM12 arm beat storing it) + 15 B of preamble |
| values | 55,296,000 raw → **1,233,099** = **76.96%** of the entropy coding |
| inner | **1,233,386** |
| price (armor v4, unmoved) | **4,812** — 256-B squares, 18 parity, 0 CT, 204 sites; 1.17× the 4,096 pigeonhole floor |
| **armored total** | **1,238,198** |
| v12's total on the same row | 1,513,903 → **−275,705 = 17.21% of the entropy-coded bytes** |
| naked `xz -9` (an exhibit, never a bar) | 1,571,824 → lighter by 333,626 |

## Run it

```bash
cd codegg-v13 && cargo build --release          # std only, offline
cargo test --release                            # incl. the peel's own pipeline law
cargo clippy --all-targets -- -D warnings       # clean under rustc 1.98
target/release/eggv13 transmute <file> [--survive BYTES] [--tier BLK] [--parity T] [--ct triple|incw|none] [--judge]
target/release/eggv13 restore <f>.egg13|.egg12|.egg11|.egg10|.egg9|.egg8 [--wound start:len]
target/release/eggv13 info <f>.egg13            # the promise with its number; a peeled form names its recipe
EGG_PEEL=1 target/release/eggv13 transmute x.jpg   # print the peel's reading, or its refusal and the reason
EGG_NO_PEEL=1 target/release/eggv13 transmute x.jpg   # the ordinary pipeline, for a controlled comparison
node tools/mkjpegsuite.js                       # build corpus-jpeg (provenance in the file's header)
node tools/jpegsuite.js                         # the peel's conservation suite
node tools/drills.js                            # the drill battery, peel battery included
node tools/ledger13.js                          # the 20-row ledger; EGG_PRED=<json> judges every row to the byte
```

Format **EG13 v7** (`.egg13`); eggv13 restores `.egg12` through the same armor v4
(the format did not move at v13 — only the name did) and `.egg11` / `.egg10` /
`.egg9` / `.egg8` through `src/armor11.rs`, v11's armor v3, verbatim.

## Attribution

ITU T.81 (the JPEG codec, the canonical Huffman decode, the 1-bit padding rule);
packJPG (Matthias Stirner) and Lepton (Dropbox) — the peel idea and the
coefficient-model context shape; LOCO-I / JPEG-LS (the MED predictor on the DC
plane); Reed & Solomon 1960, Berlekamp–Massey, Chien, Forney (the armor,
unmoved from v12); Krachkovsky & Lee 1997 and Bleichenbacher, Kiayias & Yung
2003 (collaborative decoding of interleaved RS); Matt Mahoney's zpaq / lpaq /
paq8 and Byron Knoll's cmix (the 16-bit coder, StateMap and APM); Fowler–Noll–Vo
(FNV-64, the conservation hash); Igor Pavlov's LZMA (the token shapes); XZ Utils
5.8.3 (the rival's stream, exact bytes). The site supplied every reading.
