# codegg-v8 — the Squeeze

eggv8 is still the Transmuter. Nothing here compresses; the form is
TRANSMUTED and RESTORED, the first law is conservation, and the FNV-64 of
the original bytes gates every restore. v8's charter, in Vladimir's words:
"check the site's geometry… keep squeezing… there's more juice left to
squeeze… the geometry is almost unbreakable and sound." So v8 did exactly
two kinds of work: it SQUEEZED the form (filters, context mixing,
right-sized ribs) and it CHECKED the geometry (an audit in the site's own
bulk discipline — the counts are below).

The chain is 100% the house's own. No borrowed transforms (BCJ was
considered and rejected); every borrowed IDEA is attributed inline where it
lives.

## The failures, first

- **wubbadub.html loses to gzip -9 (37.1% vs 30.8%).** The armor floor on a
  ~27 KB payload is ~21% and the mixer cannot replace what matches do for a
  92 KB HTML file. Predicted a loss at 33–36, landed 37.1 — worse than
  predicted. It stays in the table.
- **real-test.db never caught xz -9 (15.65% vs 15.4%).** 0.25 points short
  after every stage. The bonus never landed.
- **The APM stage costs two files:** real-test.bmp +1,536 B and
  wallpaper.jpg +1,536 B versus the pre-APM build. Kept anyway: net
  −20,992 B across the corpus, and no verdict flips. The losses are the
  price of the wins and both are printed.
- **The M2 filter prediction for the BMP was wrong** — in the good
  direction, which is still wrong. The plan predicted 23–29% from
  pixel-delta physics; the row-stride delta produced 2.35%. The sample
  probe, left to decide alone, would have VETOED that filter (each sample
  slice starts with a full verbatim stride); only the full trial saw it.
  Samples nominate and prune; whole files decide. Convicted by `probe`,
  fixed at M2.
- **alarm01.wav's sample preferred the wrong filter** (byte-delta-2 over
  W16 by price and by sample encode; the full file says W16 by 7 points).
  Same lesson, same fix: all pruned candidates get full trials.
- **The speed target of the series remains modest:** transmute 2.5–5.9
  MB/s, restore 3.3–109 MB/s. Weight first was the instruction; this is
  what weight-first costs.
- **v7's geometry had two real defects, found by design review, proven by
  the audit, fixed here** (and v7's README stays as it was — the series
  keeps its fossils):
  1. *The ragged-tail pigeonhole.* `stripe_order` leaves one short group
     when s % G ≠ 0, so in the tail rows a contiguous run of k slots can
     hit one group ⌈k/(ng−1)⌉ times. A naive continuous rib formula
     G=⌈s/5⌉ claims capacity it does not have; the audit keeps that
     formula in as a negative control and it fails at 250 sizes where the
     argmin policy holds.
  2. *Clustered replicas.* v7 packed hdr0, meta0, the whole check table,
     hdr1 and meta1 into the first ~2 KB of small artifacts; one 4 KB head
     scratch killed two of three copies and the byte-vote returned garbage.

## The three new site readings

1. **The overlay reading → filters** (`src/filter.rs`).
   wubbadub.html:1304: "Lay the two stalks over each other at matching
   place values… nothing has to line up with anything but its own weight."
   spectrometer.html:238: "the same value, respelled." The site's layout is
   order-preserving (spec.md:30), so near values have near spellings and a
   difference is small. A signal respelled as its differences at its own
   stride is the site's subtraction-on-the-grid. Honest note: the site's
   audio page is spectral (rate→pitch, no samples); it does not ground PCM
   transforms — the overlay geometry does. Prior art attributed: PNG
   filters, shorten/FLAC deltas.
2. **The Spectrometer reading, part two → context mixing** (`dyadic.rs`).
   spectrometer.html:7: "One integer in full — its stalk, its square, its
   three regions, its value as a light wave"; :67: "the stalk, with its
   four readings nested underneath." v7 chose ONE reading per file; v8
   reads all the depths at once — o0, o1, o2, a hash of the last four
   bytes, and a match bank — each weighted by how well it has been
   predicting. Integer logistic mixing; the SQUASH table is generated once
   and checked in (`src/squash_tab.rs`); STRETCH is derived from it by
   integer search; no float ever touches the coding path. Attribution:
   Matt Mahoney's PAQ/lpaq lineage, Shkarin's SSE for the APM stage.
3. **The bulk reading → the audit** (`src/audit.rs`, `eggv8 audit`).
   The site ships a checked table ("298,545 checks, all passing"). This is
   that discipline pointed at the armor.

## The chain

    bytes → FILTER (id stamped in the header, chosen by full trial)
          → NIBS → TOKENS (LZ77 family, whole-file window, price-aware,
            lazy probe cached, two-step lazy)
          → ONE DYADIC POINT (range coder; literal bits mixed from five
            models through 60 weight vectors and an APM; the lighter of
            {8-bit ctx, 16-bit ctx, mixed} kept per file — a regression
            against v7 is impossible by construction)
          → ARMOR v2 (below)

## Armor v2 (format EGG8, version 2)

Same three nested scales as v6/v7 — residues mod 8219/8221 per 512 B
square, T Reed–Solomon parities per group of G, voted file scale — with the
two defects fixed:

- **Rib policy is an argmin, not a formula:** minimize container total over
  G ∈ 4..126, T ∈ {2,3}, subject to ⌈9/ng_eff⌉ ≤ T (a 4 KB scratch can
  straddle at most 9 slots; ng_eff = the number of maximum-length groups in
  the merged stripe), under a 35% overhead cap, cap relaxed before the
  guarantee ever is. Below s=12 (~6 KB inner) the guarantee cannot be given
  at all and the label says so. Overhead measured: 30 KB → 18.4% (v7:
  ~33%), 200 KB → 4.1% (v7: ~7.5%), 1 MB → 2.630% (v7: 2.628% — unchanged,
  +12 B for three FNV-32s), 4 MB → 2.47%.
- **Replicas spread to head / middle / end,** each [header, meta] copy
  carrying an FNV-32. Selection: any checksum-verified copy; two verified
  copies that disagree REFUSE (never choose); byte-vote is the last resort;
  with both end sites dead the container is scanned for the magic and a hit
  counts only if its own geometry puts a site exactly where it was found.
- **The check table is right-sized:** ct ≤ 1024 B rides as the meta itself
  (triplicate, checksummed, no level-2 RS); bigger check tables keep
  level-2 RS with their slots interleaved among the payload rows — ONE
  merged stripe, both levels, one shared `slot_off` map used by armor,
  dearmor, info, audit and the drill harness alike.
- **The physics floor, said out loud:** the 4 KB-scratch guarantee needs
  ≥9 parity slots (≈4.6–6.1 KB) no matter how small the file. Below
  ~16–24 KB of artifact that guarantee cannot be cheap. wubbadub pays it
  and loses honestly.

A drill taught us the armor was righter than the test: killing T+1 squares
of a CHECK-TABLE group loses only checks — the payload is intact and the
FNV-64 arbitrates EXACT. The battery now derives that verdict from the
geometry instead of hardcoding suspicion.

## The audit (`eggv8 audit`, `--full` for the whole battery)

    62,556,548 checks, all passing [45 s]

- (a) stripe pigeonhole: policy exhaustive s=1..2000 (1989 guaranteed
  geometries hold under every 9-slot window; 11 tiny sizes honestly
  unguaranteed), 10 adversarial (G,T) shapes × 2000 sizes (18,336 claims,
  none over-claimed), 200 log-sampled sizes to 10^6 slots. Negative
  control: the naive G=⌈s/5⌉ formula breaks at 250 sizes where the argmin
  holds (first at s=12) — the audit sees.
- (b) ±2^k injectivity: 8192/8192 signed syndromes distinct for BOTH 8219
  and 8221; sampled double-error ambiguity 280/200,000 pairs (1.4e-3) —
  why the retry ladder's last rung trusts parity only.
- (c) repair-boundary maps: five artifact sizes × five regions × blind and
  addressed; every measured EXACT boundary ≥ the geometry's own floor
  (payload 4 MB: 67,584 B measured vs 66,049 theory; head boundaries larger
  by exactly the absorbed replica site).
- (d) fuzz × 10,000 (deterministic xorshift): scratches to 50%, truncations
  to 90%, random containers, header/meta-targeted, 1–64-bit storms —
  4,675 EXACT, 5,325 honest, **0 silent**. Accept-wrong physics ≈ 2^-64
  per attempt (FNV-64; not cryptographic, adversaries out of scope).
- (e) adversarial multi-wounds with verdicts DERIVED from the geometry
  functions: disjoint scratch pairs, T+1 in one payload group (refuses),
  T payload + T check-table (repairs), two replica sites + a slot wound
  (repairs off the surviving verified copy), mid-site straddles,
  scratch+storm — all as derived, never silent.

Black-box battery: `node tools/drills.js` — 75/75, including 4 KB head
scratch EXACT, two-sites-killed EXACT, and geometry-derived stripe wounds.
Filter property test: `cargo test --release` — apply∘undo identity over
every length 0..1024 × every filter id (the tails are the fail class), plus
the borrow-bit assertion that W16 beats byte-delta-4 on 16-bit ramps.

## The tournament (real corpus, 12 files off this machine)

Rule unchanged: wrong-or-no data after any injury forfeits; smallest
lossless survivor wins. Injuries: 1-byte flip (blind), 4 KB scratch
(addressed), 4 KB truncation.

| file | orig B | gz* | zstd -19 | xz -9 | e6+zstd | egg7 | egg8 | filter |
|---|---|---|---|---|---|---|---|---|
| vim-version9.txt | 2,035,039 | 24.2 | 18.7 | 18.2 | 20.1 | 21.4 | **19.6** | none |
| wubbadub.html | 92,408 | 30.8 | 29.0 | 28.3 | 39.0 | 40.1 | **37.1** | none |
| real-test.db | 9,551,872 | 19.5 | 17.3 | 15.4 | 17.7 | 16.4 | **15.7** | none |
| real-test.bmp | 12,000,054 | 30.5 | 30.2 | 30.1 | 31.0 | 31.2 | **2.3** | stride 6000 |
| arial.ttf | 1,045,720 | 53.6 | 48.2 | 44.6 | 51.8 | 53.3 | **48.6** | none |
| segoeui.ttf | 959,752 | 56.5 | 48.1 | 44.0 | 51.8 | 53.0 | **48.3** | none |
| zstd.exe | 1,601,409 | 41.7 | 36.2 | 33.4 | 38.9 | 40.1 | **36.5** | none |
| kernel32.dll | 836,208 | 45.9 | 40.3 | 37.8 | 43.4 | 45.2 | **41.1** | none |
| notepad.exe | 360,448 | 55.5 | 52.6 | 50.5 | 57.0 | 58.0 | **54.2** | none |
| alarm01.wav | 491,516 | 78.2 | 77.3 | 70.1 | 83.3 | 84.5 | **61.7** | W16 ch=2 |
| ring01.wav | 498,420 | 58.9 | 56.5 | 49.8 | 61.1 | 62.0 | **41.9** | W16 ch=2 |
| wallpaper.jpg | 1,602,752 | 98.9 | 98.0 | 98.1 | 100.5 | 97.7 | **96.8** | none |

The compressor columns are their UNDAMAGED weights; under the tournament's
injuries every one of them forfeited every file (gzip/zstd/xz dq, brotli
returned WRONG DATA with a success code on 6 of 12 — RFC 7932 has no
mandatory checksum; that lesson is three versions old now).

**Podium: egg8 × 12.** All 36 injuries restored EXACT, countersigned by
certutil SHA-256 (`node tools/verify.js corpus-real/*` — the OS's own hash,
no code of ours in the verdict path).

**The victory ledger — the stretch bar, all three at once (armor ON, every
injury EXACT):**

| bar | needed | got |
|---|---|---|
| (i) lighter than the STRONGEST gzip -9 (min of CLI and zlib per file) | ≥10 of 12 | **11 of 12** (wubbadub the honest loss) |
| (ii) lighter than the egg6+zstd hybrid — dethrone the borrowed compressor | ≥6 of 12 | **12 of 12** |
| (iii) lighter than xz -9 | ≥3 of 12 | **4 of 12** (bmp, alarm01, ring01, jpg) |

The pigeonhole held: 4 MiB of crypto-random transmutes to 100.9% (LARGER,
asserted as a PASS in the drills — a form that cannot say no to random data
would be lying about everything else).

## Synthetic corpus (the v7 regression bar: ≥5/8 structured vs gzip -9)

| file | orig B | egg6 | e6+zstd | egg7 | egg8 | winner |
|---|---|---|---|---|---|---|
| archive.zst | 946,623 | 107.3 | 107.3 | 107.7 | **102.8** | egg8 |
| big.xml | 4,156,948 | 102.5 | 10.9 | 11.9 | **10.9** | egg8 |
| corpus-1489k.bin | 1,489,000 | 102.6 | 30.7 | 31.1 | **29.5** | egg8 |
| data.csv | 4,960,761 | 102.4 | 24.5 | 24.3 | **23.9** | egg8 |
| photo.bin | 4,194,304 | **102.5** | 102.5 | 103.3 | 102.8 | egg6 |
| program.exe | 265,216 | 108.0 | 48.1 | 49.7 | **46.3** | egg8 |
| real-test.bmp | 12,000,054 | 102.4 | 31.0 | 31.2 | **2.3** | egg8 |
| real-test.db | 9,551,872 | 102.4 | 17.7 | 16.4 | **15.7** | egg8 |
| repo-bundle.bin | 215,055 | 108.4 | 36.7 | 38.2 | **32.0** | egg8 |
| server-log.json | 7,549,839 | 102.4 | **13.5** | 14.3 | 13.6 | egg6+zstd |

Podium: egg8 × 8, egg6 × 1, egg6+zstd × 1. The regression bar HELD at 7 of
8 structured files lighter than the strongest gzip -9 (v7 scored 5): the
loss is repo-bundle.bin at 32.0% vs gzip's 31.98% — five hundredths of a
point, printed, not rounded away. The two losses on the podium are honest
ones too: photo.bin is crypto-random (egg6's plain armor is simply thinner
than a transmuter that must also carry a model byte), and server-log.json
still likes zstd's window discipline by 0.1 points. Both pre-compressed
members transmute LARGER than 100% — the pigeonhole, kept.

## Speed (honest, no gate)

transmute 2.5–5.9 MB/s (three model trials per candidate arm ride on
std::threads — deterministic tie-breaks, the coder itself is never
threaded); restore 3.3–109 MB/s. Worst file stays above the 1 MB/s sanity
floor. v7 averaged 2.4 MB/s doing a third of the modeling.

## Attribution

Elias, Rissanen, Witten–Neal–Cleary (arithmetic coding); Ziv–Lempel 1977
(the match layer); Igor Pavlov's LZMA (rep offsets, slot/align shapes,
matched-byte steering); Matt Mahoney's PAQ/lpaq (logistic mixing, APM),
Dmitry Shkarin (SSE); PNG filters and shorten/FLAC (delta prior art);
Avizienis 1971 / Mandelbaum 1976 (residue codes); Reed–Solomon 1960 via
Lagrange evaluation; Fowler–Noll–Vo (the hashes). The site supplied the
geometry and the vocabulary: the dyadic disc, the overlay, the nested
readings, the bulk discipline.

## Run it

```bash
cd codegg-v8 && cargo build --release        # std only, offline
target/release/eggv8 transmute <file>        # .egg8; armor ON, filters auto
target/release/eggv8 restore <file>.egg8     # conservation-gated
target/release/eggv8 audit [--full]          # the geometry audit, counts printed
target/release/eggv8 probe <file>            # the filter decision, traced
cargo test --release                         # filter property tests
node tools/drills.js                         # black-box battery (75 drills)
node tools/standings.js corpus-real/*        # the tournament + victory ledger
node tools/verify.js corpus-real/*           # certutil countersign
```
