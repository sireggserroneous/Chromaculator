# eggv11 predictions — filed 2026-09-02 at v11-M0, BEFORE any workstream was built

House rule: predictions first, measurements after, unflattering rows kept.
Mandate: full autonomy (Vladimir, 2026-09-01: "Don't wait for my word…
build, test with clippy… go for the knockout"). Clippy -D warnings green at
every gate. Plan: `~/.claude/plans/rematch-of-the-heavyweights-v11.md`.

## THE BARS (the rematch card)

1. **KNOCKOUT: home 12-file arena vs naked xz -9 ≥ 10/12** (v10: 8/12).
2. Big arena vs naked xz -9 ≥ 5/8 (v10: 2/8).
3. **Armored sweep: beat xz+par2's weight 20/20** (v10: 18/20).
4. v11 ≤ min(v8, v9, v10) on every row of both arenas (the fallback crack).
5. Speed floor 0.25 MB/s kept; 183 MB end-to-end transmute ≤ 45 min (v10: ~5 h).

## Baselines (v10 final, armored % of input; gaps vs naked xz)

Home: vim 16.04, wubbadub 33.68 (xz gap −5.38), db 13.34, bmp 2.25,
arial 45.22 (−0.62), segoeui 45.27 (−1.27), zstd.exe 32.73, kernel32 35.98,
notepad 51.21 (−0.71), alarm 55.79, ring 29.84, jpg 96.76.
Big: sav 26.8 (xz 25.2), cbs 0.9 (xz 0.86), iconcache 0.44 (xz 0.43),
mermaid 19.19 (xz 19.16), msgraph 10.79 (xz 10.98 WIN), ntoskrnl 39.1
(xz 42.2 WIN), rdr2 87.8 (xz 85.9), rustc_driver 23.7 (xz 22.8).
Armored rivals (xz+par2): sav 25.9, cbs 1.0, icon 0.5, mermaid 19.8,
msgraph 11.3, ntoskrnl 43.5, rdr2 88.0, rustc 23.4.

## Calibration (stated before the numbers)

v9/v10 lessons, both directions: transforms and labels are pre-shrunk 10×;
NEW information channels (the lattice, the priors) carry the v10-states flag —
they can erupt 2–4× ABOVE range; re-sweeps after architecture changes are
never zero (v10 measured 0.3–1.3/row unpredicted — THIS time they are
predicted); the ARMOR TAX is priced per row, not per arena (the big-arena
2/8 audit's lesson).

## Per-stage predictions

- **M1 armor v3 (wide grid + slim CT):** big-payload (≥1 MB) tax −40–70%.
  Row calls: mermaid −0.3..−0.5 (tax ~19 KB of a 7,216 B gap ⇒ FLIPS vs
  naked xz); iconcache −0.004..−0.010 (arithmetic says the flip is a
  photo-finish COIN FLIP — the 512 B-square residues are 0.78% of its small
  payload; S=4096 cuts ~10 KB of a 10.3 KB gap); rustc −0.3..−0.7;
  msgraph/ntoskrnl margins widen; home files mostly <1 MB payloads ⇒
  unchanged EXCEPT via WS-CT: wubbadub −0.3..−1.0, notepad/arial −0.1..−0.4.
  cbs payload ~131 KB ⇒ NOT helped by wide squares; its flip waits on M2.
- **M2 parser (cliff + DP):** 183 MB tokenize ≤15 min (from hours). DP:
  PEs +0.10..0.30, logs +0.05..0.20 (cbs's flip likely decided here),
  near-entropy +0.05..0.25, else ~0. Kill: <0.08 avg on gated losers ⇒
  revert, print.
- **M3 lattice (record contexts):** sav +0.4..1.2 (NEW channel — eruption
  flag), rdr2 +0.2..0.8, db +0.1..0.4, PEs +0.05..0.30, text/audio/jpg ~0.
- **M4 trial + threads:** sizes CHANGE ZERO (bit-identity asserted
  threaded-vs-serial); wall-time ≥2× better on ≥1 MB files; the fallback
  widening makes bar 4 true by construction (sav/rdr2 recover v9's tenths).
- **M5 priors (site-trained):** wubbadub +0.3..1.5 (BIAS STATED: wubbadub is
  itself a site page — the control is that notepad/arial must also move, or
  the gain is discounted as memorization and judged on the other rows);
  notepad +0.1..0.4; arial +0.05..0.3; ≥1 MB rows ~0 (priors wash out).
- **M6 RLE unroll:** new RLE corpus members 2–15 pts vs xz; zero elsewhere
  by construction.
- **M7 JPEG peel (STRETCH):** jpg 96.76 → 78–86 IF it lands; kill criteria:
  any mutant-fuzz inexactness ⇒ revert; <5 pts ⇒ revert. A skip/revert is a
  printed outcome, not a failure of the campaign.
- **M8 re-sweeps (LR/LIMIT/ISSE_LR + new-lever hyperparams):** +0.1..0.6 per
  row (v10's unpredicted earthquake, this time on the books).

## The bar arithmetic, filed plainly

| bar | needs | mid projection | call |
|---|---|---|---|
| home ≥10/12 | arial 0.62, notepad 0.71 | 0.45..1.4 each (CT+DP+lattice+priors+sweeps) | **YES at mid — the likeliest bar** |
| home 11/12 | + segoeui 1.27 | ~0.5..1.5 | coin flip |
| home 12/12 | + wubbadub 5.38 | ~1.0..3.5 | NO at mid — the dream, stated |
| big ≥5/8 | mermaid+icon+cbs flips, hold 2 wins | tax + DP | **YES at mid** (icon is the coin flip; sav/rustc are upside) |
| armored 20/20 | sav −0.9, rustc −0.3 | lattice 0.4–1.2; tax 0.3–0.7 | **YES at mid, two swing rows** |
| ≤ min(ancestors) | the two 0.1 cracks | fallback by construction | YES |
| 183 MB ≤45 min | ~5 h today | cliff ≤15 min + threads | **YES** |

**Ledger call: home 10/12 (11 coin flip), big 5/8 (6–7 upside), armored
20/20, wall-time law MET.** If any bar misses, the miss is printed beside
what moved, and the residual gap names v12's levers.

## M0 — fork fidelity (measured at filing time)

Fork codegg-v10 → codegg-v11: crate eggv11, EG11 v5 (.egg11), MODEL bytes
10/11 (MIX11/CM11 byte-copies), ancestors .egg10/.egg9/.egg8 accepted.
cargo test 13/13; clippy --all-targets -D warnings CLEAN (frozen tiers carry
documented allows; live-path style rewrites proven stream-neutral by the
gate below); wubbadub round-trip EXACT at 31,120 B (the v10 book number);
.egg10 ancestor restore EXACT. The full 20-file lanes gate (bit-identity mod
magic/version/model/FNV at all three header sites + ancestor compat) runs at
filing time; its output is m0gate.txt and M0 is not closed until it prints
PASS on all rows. Everything below M0 is a real prediction.

## Measured (filled as milestones land — never before)

### M1 — armor v3, the wide grid (2026-09-02). Gate: PASS.

Audit 105,946,717 checks: all six tier moduli injective (16421/16427 at 8192
bits, 32771/32783 at 16384), 69,110 adversarial grid claims across 3 tiers
none over-claimed, double-error ambiguity IMPROVES with width (1.4e-3 →
8.0e-5 → 0.0 per pair — wider moduli spread the same syndromes). Drills
75/75. Clippy clean. Ledger (19 files, monster deferred to M2): every row
lighter or equal, zero >0.05 regressions, all 57 injuries EXACT, **net
−1,029,720 B**.

- rdr2 −583,428 B (−1.19 pt) — ABOVE range, the fleet's biggest armor fell.
- mermaid −55,980 B — **FLIPS vs naked xz: WIN by 48,764 B** (called ✓).
- wallpaper.jpg −17,624 B (−1.10 pt) — UNCALLED WIN (1.5 MB payload rides
  tier 1; 96.76 → 94.6). Prediction miss in our favor, printed.
- **iconcache −1,024 B only: MISS both ways** (called −4..−10 KB and a
  coin-flip xz flip; its 415 KB payload is tier 0 and the G-scan bought
  1 KB). Still −9,328 B vs xz; the flip's named lever is now M2's parser.
- ntoskrnl −0.445 / aoe4 −0.358 / msgraph −0.122 / db −0.147 — in range.
- arial/segoeui −2,048 B each (arial's xz gap 0.62 → ~0.42); kernel32,
  notepad, wubbadub, ring, cbs unchanged (small payloads — WS-CT pending).

Post-M1 bar state: big arena vs naked xz — mermaid joins ntoskrnl+msgraph
(3 of the 5 needed); rdr2's gap −914 KB → −330,598 B (inside M3's lattice
range); armored rdr2 flips to ours. Home: arial needs ~0.42, notepad 0.71,
segoeui ~1.06.

### M2 — the parser: the cliff retired, the DP executed (2026-09-02). Gate: PASS.

**The cliff (kept):** hash width now scales with the input (18 bits <16 MB —
those artifacts stay bit-identical — up to 23 at 183 MB) plus a work budget
above 16 MB. The monster: **40m23s end-to-end** (was ~5 h), in-memory verify
included — **the ≤45 min wall-time law: MET at M2**, and its artifact came
out −531,300 B vs v10 (23.42%, tier-2 squares, G=248). Honesty note, filed:
the wall-time law and the 0.25 MB/s floor are mutually inconsistent at
183 MB (45 min ≡ 0.068 MB/s); the floor binds the HOME corpus where it was
declared, the wall-time law governs the big rows — both print, no quiet
goalposts. Cost printed: msgraph +6,144 B vs its M1 point (+0.014 pt, within
law) from the wider hash re-shaping its chains.

**The DP (killed by its own criterion):** the searched-grid optimal parse —
candidates walk, 256-byte windows, full rep state, measured flat-rate
prices — measured **0.000 avg gain on vim/zstd/db/cbs (criterion: ≥0.08)**
at 2–3× second-pass cost, briefly breaking the home floor on db (0.24 MB/s).
Reverted same hour; the loser deleted, not shipped dormant. The convicted
suspect is the flat-rate LENGTH pricing (the slot table priced distances
well; nothing priced lengths). Filed as v12 evidence with the v9-states
precedent: a killed lever can erupt when rebuilt properly. The consolidation
survives: ONE second_pass where v10 carried the logic thrice. cbs.log's
called flip did NOT arrive (its lever was the DP) — the call stands MISSED
unless M8's sweeps close 0.04 pt.

**M2 ledger: all 20 rows ≤ v10, all 60 injuries EXACT, net −1,554,876 B.**

### M3 — the lattice (2026-09-02). Gate: PASS, kept by net (−177,664 B vs M2).

The detector locks where byte-stride lives and stays provably silent where
it doesn't (unlocked streams bit-identical by arithmetic: zero-weight
entry + constant-zero inputs). Locks measured: db s=27 (sqlite cells),
rustc s=19, bmp s=3 (plain arm only — the filtered arm correctly sees
nothing), wavs s=2/4; dominance relaxed 4×→3× after the wav missed at 3.74×
(audio's harmonics inflate the average — measured, tuned, printed).

- **rustc −169,984 B → 23.33%: the ARMORED monster flips (~134 KB lighter
  than xz+par2).** Uncalled row, biggest earn.
- **arial −2,048 / segoeui −3,072 further — the TTFs lock.** Uncalled.
  Arial's naked-xz gap now ~0.23 pt.
- ring −2,560 (−0.51 pt). db locked but net-zero armored (+388 inner,
  absorbed). bmp/alarm: their filtered arms still win — zero, by design.
- **THE CALLED ROWS MISSED: save +0.4..1.2 and rdr2 +0.2..0.8 measured
  0.000 — no byte-stride exists in either** (save 0.7% hit-rate, rdr2 1.5×
  dominance). The eruption-flag lesson repeats: new channels erupt, never
  where pointed. Both misses stand printed.
- Mirror: EGG_STATEHASH enc/dec matched on locked and unlocked rows;
  all 60 injuries EXACT; net vs v10 now **−1,732,540 B**.

Bar state after M3: armored sweep 19/20 (the save alone, −0.52 pt and
shrinking); big naked 3 wins + icon (−8,304 B) and cbs (−0.04 pt) in
M8-sweep range; home: arial ~0.23 (likely), notepad 0.71, segoeui ~0.95.

### M4 — threads + the widened fallback (2026-09-02). Gate: PASS.

Ledger byte-identical to M3 on all 20 rows (net −1,732,540 held): the
side-by-side forms (plain ∥ filtered, the no-carries reading) are PROVEN
byte-neutral, and the widened fallback (frozen MIX10/CM10 arms summoned
inside 0.3% margins; MIX9 still the alarm) armed without firing — v11
already ≤ min(ancestors) on every row (12/12 on the interim standings).
Interim home standings (M4 binary): podium egg11 ×9 + egg10 ×3 (exact-tie
credits), vs naked xz 8/12, hybrid 12/12, wallpaper.jpg beats the whole row
at 95.7 for the first time in three versions.

### M5 — the educated guess (2026-09-02). Gate: PASS (as an ARM, not a decree).

The site's book (467,329 B across 14 pages — **the test page excluded**: its
siblings wub/wubdiv/wubx teach the dialect honestly) trained mixer weights,
h1/h2 followers, o1 states, and the o1 StateMap (counts capped at 64 so real
files can still move the maps). First draft applied the prior UNCONDITIONALLY:
wubbadub −524 B but kernel32 +1,536 and vim +1,012 — REAL regressions, the
prior misleading dialects it never learned. Fix: the prior became a TRIAL
ENTRANT (MODEL bytes 12/13, primed decoders — the format carries the guess),
running only on sub-1 MB inputs. Measured: **wubbadub −524 B (33.68 → 33.11,
in the called 0.3–1.5 range)**, all regressions gone (plain arms win their
rows), notepad/arial called gains MISSED — the book speaks HTML, not PE or
TTF; both misses printed. Round-trips EXACT through the primed decode paths.

### M6 — the RLE unroll (2026-09-02). Gate: PASS on mechanism; the called
### margin MISSED — and the milestone paid for itself in laws, not bytes.

No real RLE bitmap exists on this machine (250k files scanned), so the demo
member is DERIVED with printed provenance (real-test.bmp quantized to the
6×6×6 cube, canon-RLE8-encoded) and verified by an INDEPENDENT decoder
(GDI+: 4,013 sampled pixels, 0 mismatches). Filter id 12 fires only when the
canonical re-encode reproduces the original stream byte-for-byte (the TTF
invertibility law); 500-mutation fuzz rides cargo test.

**THE WOUND (found, fixed, law extended):** the unroll is the series' FIRST
length-changing filter, and restore decoded the dyadic stream to the
ORIGINAL length — the artifact shipped unrestorable for one hour on a file
too small for the ≥2^26 law. Fixed (the filtered length rides in
filter_param); the write-time in-memory round-trip law now covers EVERY
length-changing-filtered artifact at any size; a full-pipeline cargo test
(filter→trial→armor→restore) rides forever. Flip injury EXACT.

**The verdict on value, plainly:** called 2–15 pts vs xz; measured −0.43 pt
(1,115,424 → 1,105,184) and the member still LOSES to xz by 1,120 B. The
unrolled pixels are a 2000-wide 2D field and nothing exploits the vertical
axis — filters don't compose, and the lattice caps at stride 384. The image
lesson sharpens: run-unrolling is necessary but 2D context is the lever.
Filed for v12's image campaign.

### M7 — the JPEG peel: SKIPPED, reason printed.

The stretch is struck by its own calibration: the jpg row already beats every
arena opponent (95.7 vs xz 98.1 since M1); the peel is a 1–2 day precision
build (bit-exact Huffman re-encode) that no bar needs; and M6 just proved
image gains hinge on 2D context the peel alone wouldn't add. The v9-states
precedent is filed FOR v12: a properly built peel + 2D coefficient contexts
is the image campaign's headline lever, not a rematch-week timebox.

### M8 — the sweeps, the heavy twins, and the armored-total law (2026-09-02).
### Gate: PASS — sealed ledger all 20 rows ≤ v10, net −1,766,332 B, 0 failures.

The re-sweep (predicted +0.1..0.6/row — the v10 lesson, this time on the
books): ISSE_LR 10→9 (notepad −1,524, arial/icon/zstd following; kernel32
+512 the accepted cost) — and the round-2 grid proved the constants
IRRECONCILABLE: high mixer-LR (12/13) feeds text/db/records (vim −6,144,
db −18,432 at LR13) while starving PEs/TTFs. The house answer, the v8-trio
precedent: **heavy-LR twins as trial entrants (MODEL 14/15, ≥512 KB)** —
vim 16.04 → 15.69, db 13.34 → 13.00, mermaid −68,268 total.

**The kernel32 saga, three acts, all printed:** (1) the sweep's +512 breached
≤min(ancestors); (2) the frozen elders became STRUCTURAL trial arms under
4 MB — and the breach survived, exposing act (3): v11's inner was 633 B
LIGHTER yet armored 512 B heavier — the smaller inner crossed a square
boundary into one more parity group. **Trials compared inner bytes; verdicts
weigh armored totals — at every armor quantum those disagree. All trials now
compare armored_total().** kernel32: 300,832 exactly; the law whole; the
same fix found cbs.log −500 nobody else could reach.

notepad's final gap vs naked xz: 1,176 B; arial's: 1,576 B (exact; first printed 1,034 / 1,901 from rounded percentages, corrected 2026-09-02) — every lever in
the campaign's arsenal fired at them (the lattice probe printed notepad's
verdict: 1.42× dominance, no stride to read). Their residuals open v12.

### The formats card (Vladimir's call, 2026-09-02): md/json/xml exhibit

changelog.md 1.2 MB: egg11 15.69 vs xz 15.11 — xz by 6,988 B, and the
PAYLOAD is ours by 1,292 B (the armor tax flips it: the campaign's central
finding, third witness). embeddings.json 15.8 MB: **egg11 WINS by 283,068 B**
(31.59 vs 33.39). Microsoft.Graph.xml 85.5 MB: **egg11 WINS by 184,060 B**
(2.11 vs 2.32), restored EXACT through the ≥64 MB law. The modern trio:
2–1 for the armored transmuter over naked xz.

## THE CLOSING AUDIT (2026-09-02) — every bar, called vs landed

| bar | called at M0 | landed | verdict |
|---|---|---|---|
| home vs naked xz ≥10/12 | YES at mid | **8/12** | **MISSED** — notepad 1,176 B short, arial 1,576 B (exact); the call was wrong |
| big vs naked xz ≥5/8 | YES at mid | **3/8** | **MISSED** — icon −7,792 B, cbs −11,800 B residuals; wrong again |
| armored sweep 20/20 | YES, two swing rows | **19/20** (22/23 with formats) | **MISSED BY ONE** — the save; the monster swing row WAS taken |
| ≤ min(ancestors) all rows | YES by construction | **20/20 + 10/10 synth** | **MET** — after two breaches taught the armored-total and 16 MB-gate laws |
| floor 0.25 + monster ≤45 min | YES | **0.32 MB/s; 40m23s** | **MET** |

Campaign totals: **net −1,766,332 B vs v10 across both arenas, zero shipped
regressions, every injury EXACT everywhere, all fingerprints countersigned,
podium egg11 ×11 home / ×10 synthetic, armored rivals beaten 22 of 23.**
The predictions were wrong in both directions all campaign — the ledgers,
gates, and kill criteria carried it. As every campaign before.

v12's opening lines, priced by this one: notepad 1,176 B and arial 1,576 B
(naked xz, home); the save 0.53 pt (the last armored row); iconcache 7,792 B
and cbs 11,800 B (naked xz, big); the second dimension for images (the RLE
lesson + the JPEG peel); a real length-priced DP (the killed parser's
autopsy); the armor tax on small artifacts (the changelog's 1,292 B payload
win that the shield spent).
