# codegg-v14 PREDICTIONS — LET THE NUMBERS TALK

Plan: `C:\Users\vcepe\.claude\plans\v14-let-the-numbers-talk.md`, which
supersedes `so-give-me-a-iterative-quill.md`.

v13's own record — every measurement this campaign is judged against — is
`codegg-v13/PREDICTIONS.md`, kept whole and not copied here.

## Why this campaign exists, in one table

v13 sealed with **23/23 on the armored bar** and **net −17,963,568 vs sealed
v11**, then met its first two outside specialists on 2026-09-04:

| | v13 | rival | verdict |
|---|---|---|---|
| `corpus-jpeg`, the 40 rows both return byte-exactly | inner 24,432,195 | **packJPG 24,385,599** | **we lose by 0.19%, and 18 rows to 22** |
| `alarm01.wav` | 235,196 | **FLAC 210,087** | **we lose by 10.68%** |
| `ring01.wav` | 130,753 | **FLAC 101,165** | **we lose by 22.63%** |
| progressive (SOF2) JPEGs | **refused** | **packJPG reads them** | 51 of 60 to our 43 |
| the three injuries, every row | **3/3 EXACT** | **both forfeit 0/3** | ours alone |

**Ratio is not our claim any more. Armor is the only currency we hold outright,
and it has a live defect.** That is why N1 is the campaign's most important
milestone rather than its warm-up.

**We do not dismiss the losses. We build, we test, we put up a fight, and the
numbers do the talking** (Vladimir, 2026-09-04).

## House rules that earned themselves at v13

- Predictions FILED **before** the code that could move them. Misses printed
  FIRST. Losers **deleted**, never shelved.
- **A measurement that disagrees with the mechanism is not a finding until a
  control says the instrument works.** A no-op setting that must reproduce the
  baseline byte-for-byte goes in the sweep BEFORE the interesting settings. v13
  burned two numbers this way: an `i32` overflow whose wrapped result I
  published a mechanism for, and a rival run at defaults whose limitation I
  wrote up as a forfeit when the tool had a flag for it.
- **Run every rival at its STRONGEST documented setting, and read its `--help`
  before tabling a forfeit.** A rival we beat by mis-invoking it is worth less
  than no measurement at all.
- **Peels are cheap, arms are expensive.** A peel is magic-gated and costs
  nothing on files that do not match; an arm runs on everything and pays into
  the speed floor, which is now at **0.268 MB/s against 0.25**.
- **Report the RECIPE first** for any peel: raw→coded %, share of the shipped
  inner, a bar, and the RETURN measured against the `EGG_NO_PEEL=1` control.
- **Test the claims that KILLED something.** v13's second session found that the
  whole S1c verdict rested on `alpha` vs `a_alpha` three characters apart in one
  expression — slip it and a fresh 16-bit symbol costs 0 bits, every wide width
  manufactures a win, and the table still looks well-formed. Mutation-check the
  load-bearing line; do not assert it.

---

## N0 FILED (2026-09-04, at the fork, BEFORE the fork ledger ran)

`codegg-v13` → `codegg-v14`, crate `eggv14`, format **EG14 v8**. The armor did
not move, so `.egg13` and `.egg12` are read by the same armor v4 path (one
constant per accepted magic, both sides reading it) and `armor11.rs` still owns
`.egg11` and older.

**FILED: the fork reproduces all 20 sealed rows TO THE BYTE and restores every
ancestor artifact. Any row that moves is a fork bug, not a finding.**

---

## N1 FILED (2026-09-04, BEFORE ONE LINE of drill or armor code)

### The defect, as inherited

`armor.rs`'s `consistent()` (line 1241) reads **`bad.iter().take(4)`** — four of
up to 2,048 codewords — and is **provably vacuous at its caller** (line 1194):
the codewords it checks were just fixed by per-codeword Berlekamp–Massey, so
they pass by construction. The brute-force accept path then rests on **one
16-bit check, ~32 expected false accepts**. FNV-64 still saves conservation — we
never hand back wrong bytes — but **repairable wounds become refusals**.

### The gap that lets it hide, now read from the code rather than remembered

`audit.rs` generates two kinds of wound and never their combination:

- **`kill_squares`** (line 94) noises **whole squares** — fully dead, positions
  known, so they enter the erasure set `E`;
- **section (f)** (line 575) flips **single bits** in `e = ⌊t/2⌋ ` squares —
  partly hit, positions unknown, so they must be LOCATED by the syndromes.

**Nothing in the tree wounds both at once.** Mixed damage is exactly the case
`consistent()` exists to judge: some squares erased, others needing location,
and a partial location taken only if it makes the bad codewords consistent as a
set. The tournament's 4 KB scratch is ADDRESSED (pure erasure) and the drills'
blind 4 KB lands inside 3–5 squares of a synthetic container.

### THE DRILL COMES FIRST. A fix without a failing drill is a guess.

Audit section **(i)**: for every tier and placement, wounds combining **k fully
dead squares** with **m partly hit squares** (a few bytes each, so several
codewords see an error), blind and named, sweeping `k` and `m` across and past
the classical bound `2m + k ≤ t`.

**FILED:**

1. **The drill finds at least one REPAIRABLE wound that this build REFUSES**, at
   some tier, and it is a mixed case — `k ≥ 1` and `m ≥ 1`.
2. **Zero wounds return wrong bytes.** Conservation is not in question here,
   only reach. If any wound returns wrong bytes with a success code, that is a
   far larger finding than the one being chased and it goes first.
3. Widening `consistent()` from `take(4)` to every bad codeword costs **under 2%
   of restore wall clock** and **zero bytes** in any container.
4. **No sealed row's SIZE moves by a single byte** — this is a restore-path
   change and it cannot touch what the encoder writes.

**If the drill finds nothing, the defect is DOWNGRADED and printed as
downgraded, and `consistent()` is left alone.** That is a real possible outcome
and it is not a failure of the milestone — it is the milestone working. The
`take(4)` would then be documented as sufficient-in-practice with the drill as
its evidence, instead of carrying a memory that calls it vacuous.

---

## N0 MEASURED (2026-09-04) -- the baseline is sound, and the gate is taken

Before the fork's own ledger could mean anything, the sealed v13 numbers had to
be re-established against the source that actually exists: a **second session**
was working in `codegg-v13` concurrently and landed a `deflate.rs` guard at
06:54 (`fde5652`) plus `sites.rs` tests at 07:03 (`19419b0`), both AFTER v13's
ledger ran at 21:39-23:1x. It also explains the thirteen ancestor `.gitignore`
files stamped 20:18:48 that the M4 gate had to flag as unaccounted for.

**FILED: zero rows move. MEASURED: zero rows moved.** Six rows chosen so a miss
would be diagnosable -- `aoe4-autosave.sav` mandatory (the only sealed row whose
winning form IS the deflate peel), plus non-peel controls because
`looks_like_deflate` is entered on 9 of the 20 rows -- all six reproduce their
sealed size and restore EXACT, and the 29-file deflate suite reproduces its
verdict exactly.

`codegg-v14` forks from that verified state. Crate `eggv14`, format **EG14 v8**;
`.egg13` and `.egg12` read by the same armor v4 path, `armor11.rs` still owning
`.egg11` and older. `cargo clippy` clean, `cargo test --release` **45 passed**,
`EG14` magic confirmed on the wire, round trip EXACT.

---

## N1 MEASURED (2026-09-04) — the defect is REAL, its consequence was BACKWARDS, and the feared path is UNREACHABLE

Two drills, one control, and the conclusion is the opposite of the one inherited.

### The filed predictions, judged first

| filed | measured | verdict |
|---|---|---|
| the drill finds ≥1 **repairable wound this build refuses** | **0**, in 3,174 mixed wounds | **MISS** |
| **zero wounds return wrong bytes** | **0** wrong data, everywhere | **HIT** |
| widening `consistent()` costs <2% and 0 bytes | it changes **not one outcome** | see below |
| no sealed row's size moves | it is a restore-path change; nothing moved | **HIT** |

### (i) The mixed-damage drill — the case nothing in the tree generated

`kill_squares` noises **whole** squares (erasures, positions known); section (f)
flips **single bits** in ⌊t/2⌋ squares (errors, positions unknown). Nothing
wounded both ways at once, which is the only situation `consistent()` judges.

**3,174 mixed wounds: 2,481 EXACT within `2m+k ≤ t`, 0 REFUSED, 0 wrong data.**
The reach is intact.

### The control that made the result mean something

"No refusals" is not reassurance unless the drill reached the code. It did:

| | |
|---|---|
| `consistent()` reached | **2,311 times** |
| bad codewords handed to it | 263,052 |
| bad codewords it **read** (`take(4)`) | **8,327 — 3.2%** |
| rejections | **2** of 2,311 |
| **widened to all 263,052** | **identical** — same 2,311 calls, same 2 rejections, same tallies, same 3,091,770 checks |

**The `take(4)` reads 3.2% of the evidence and reaches the same verdict every
single time.** So the description in the inherited note is accurate and its
consequence — *"repairable wounds become refusals"* — is **backwards**. A check
that almost always says yes cannot cause refusals; it causes acceptances, and
the exposure would be miscorrection. Measured: none.

### The split by caller, which relocated the whole problem

| caller | reached |
|---|---|
| the partial-location path — the one the note calls "provably vacuous at caller 1188" | **2 times** in 3,091,770 checks |
| `confirm_subset` — the one priced at "~32 expected false accepts" | **2,333 times, accepted 2,333** |

The caller singled out is barely exercised. The real traffic accepts 100% of
what it is offered — and every one of those took the `found.len() == k` early
return, so **the subset-enumeration loop was never entered.**

### THE FINDING: that loop cannot be entered, and the reason is a determinant

The locator tests each position's vector `(1, X, X², …, X^(m−1))` for membership
in a `k`-dimensional span. **Those are Vandermonde rows, and any `m` of them
with distinct nodes are linearly independent.** So `k+1` distinct positions
cannot all lie in a `k`-dimensional space while `k < m` — and `k < m` is exactly
the condition under which the locator runs. **`found.len() > k` is impossible.**

The caller's own comment estimates the false-positive rate at `~n/65536^(m−k)`,
which treats the candidate as a **random** vector. It is a structured one. The
true rate is **zero**.

Measured beside the proof: **2,355 locator runs, 0 over-reports, max excess 0**,
mean `m` 16.2, mean rank 7.4, smallest `m−k` seen **1** — including 24 trials
built for the best case the armor allows, **n = 62,518 squares** (NMAX 65,535)
with `m−k = 1`, where the random-vector estimate predicts 0.95 false positives
per run and would have produced about 23. **It produced none.**

### What shipped

**The subset search is DELETED** (house rule: nothing ships dormant, and dead
code is worse than dormant). Kept: the residual check on `found.len() == k`,
which is real. The impossible case now **refuses** rather than enumerating,
because a locator that over-reports has broken an invariant and guessing a
subset would be the worst available answer. `consistent()` keeps its `take(4)`,
now documented with the drill as its evidence rather than a note calling it
vacuous.

After the deletion: `cargo clippy` clean, `cargo test --release` **45 passed**,
`audit --full` **3,091,771 checks, 0 failing**, and **every tally identical** to
before it — which is what "dead" means.

### Two of my own aims were wrong before the arithmetic was right, and they are the lesson

1. **First construction: 2 wounded squares.** The locator ran **zero** times —
   with `t = 18`, per-codeword Berlekamp–Massey fixes 2 errors and returns long
   before the collaborative path. The locator only runs when per-codeword
   decoding fails.
2. **Second construction: a rank-1 twin pair** (the trick section (h) uses).
   **Counterproductive.** `m = t − |E| = 18`, `k = rank = ` the number of
   distinct damaged positions, and lowering the rank *raises* `m−k`, which
   divides the false-positive rate by another 65,536. Ten wounds give `m−k = 8`
   and `n/65536⁸`.

Only the third aim — `t−1 = 17` **independent** wounds, giving `m−k = 1` — even
reached the interesting regime. **The arithmetic in the caller's comment was the
map the whole time, and I ignored it for two rounds.** Filed as a miss against
myself: the drill's aim is part of the drill.

### The defect, restated for the record

**DOWNGRADED, and more than downgraded.** `take(4)` is a 3.2% sample that has
never once disagreed with reading everything. The "~32 expected false accepts"
priced a code path that a Vandermonde determinant forbids. What was real is that
**the tree had no mixed-damage drill at all** — and now it has one, at 3,174
wounds a run.

---

## N2 MEASURED (2026-09-04) — the gate is correct and buys NO SPEED. The mechanism I assumed was wrong.

### The filed predictions, judged

| filed | measured | verdict |
|---|---|---|
| worst home row returns to **0.283–0.290 MB/s** | `kernel32.dll` 0.278–0.280 → **0.281–0.287** | **touched, not met** |
| the five 2D wins are kept **to the byte** | all five kept, all still model 28 | **HIT** |
| no row's size moves | **all twelve home rows byte-identical** | **HIT** |

### The measurement that kills the premise

The gate skips the 2D arm where a dialect book already matched. Five home rows
stop paying for a CM pass they always lose (`kernel32.dll` 2D=297,069 against
CM12-PE's 284,319; `segoeui.ttf` 2D=408,943 against CM12-TTF's 404,871). On a
**bigger** PE row, where that wasted pass is real work rather than a rounding
error, it should have shown clearly:

```
ntoskrnl.exe (13 MB, PE)   ungated  168,176 ms / 162,096 ms   (0.078 / 0.080 MB/s)
                             gated   162,208 ms / 162,413 ms   (0.080 / 0.080 MB/s)
```

**No gain.** And that makes the `kernel32.dll` movement (0.278 → 0.287, ~3%)
indistinguishable from run-to-run noise rather than a result.

### Why, and it redirects the milestone

**The roster's wall clock is set by its SLOWEST arm, not by the number of arms.**
Fifteen-odd arms run in parallel `std::thread` workers on a machine with cores to
spare, so the row finishes when the longest one finishes. Removing a *loser*
shortens nothing unless the loser was also the *slowest*.

I assumed "one fewer full CM pass" meant "less wall clock". It means less CPU
work, which is not the same thing and is not what the 0.25 MB/s floor measures.
**Filed as a miss against the mechanism, not just the number.**

### What is kept, and on what grounds

The gate **stays** — but the justification is no longer speed. It is that a form
whose magic names a dialect has a specialist arm primed for it, so the rectangle
is the wrong reading by construction, and the house does not do work that cannot
win. It also frees real CPU on a loaded machine, which the SOLO number cannot
see. **No speed claim attaches to it.**

### What N2 actually owes now

**Find the slowest arm and shorten IT.** That is the only thing that can move
the floor, and it has never been measured: `EGG_ARMS=1` prints every arm's
*size* and nothing prints every arm's *time*. The next step is an instrument —
per-arm wall clock on the roster — and then a decision about the arm at the top
of that list.

Until that exists, **the floor stands at 0.278–0.280 MB/s against 0.25 and any
new always-on arm is still blocked**, which was N2's whole reason for existing.

---

## N2b FILED (2026-09-04, BEFORE the instrument) — which arm sets the clock?

N2 measured that removing a losing arm does not move the wall clock, and named
the reason: **the roster finishes when its SLOWEST arm finishes, not when the sum
of its arms finishes.** That makes exactly one question worth asking, and this
project has never asked it — `EGG_ARMS=1` prints every arm's SIZE and nothing
anywhere prints an arm's TIME.

### The instrument

A per-arm elapsed-millis counter, printed under `EGG_ARMS=1` beside the sizes,
sorted slowest first, with the row's own wall clock beside it. An INSTRUMENT, in
the house sense: it prints what the roster already does and decides nothing.

### FILED, and the reasoning is from the code rather than a hunch

The roster is 15–17 arms. The LZ arms (`MIX*`) code TOKENS — fewer decisions
than one per byte — while the literal-only arms (`CM*`) push every byte through
the twelve-input mixer twice, once per nibble. That would make the CM arms
slower per byte. But `big_arms` then runs a **price replay**: `second_pass` on
the v11 LZ arm and `second_pass12` on the v12 one, which encodes those arms a
SECOND time.

1. **The slowest arm is an LZ arm carrying its price replay — `MIX12`, or
   `MIX12H` on rows ≥ 512 KB.** Called at **≥ 1.6× the median arm**.
2. **The row's wall clock lands within 1.0–1.3× of the slowest arm**, which is
   what "max, not sum" predicts and what would make N2's negative result
   inevitable rather than surprising.
3. **The spread between slowest and fastest arm is > 2×.**
4. `NUM` and `2D` cost about the same as `CM12`, since they ARE `CM12` with two
   sparse inputs re-pointed — **within ±15% of it**, and therefore not the arms
   that set the clock. If they are, N2's whole diagnosis is wrong and the gate
   should have worked.

**What would make this wrong:** if the times come back roughly flat, then no
single arm sets the clock, the wall clock is a genuine sum, and the parallelism
is not doing what the code assumes. That would be a bigger finding than the one
being chased, and it goes first.

## N2b MEASURED (2026-09-04) — the clock is set by a PLATEAU, not by an arm

The first per-arm timing ever taken in this project. It says my prediction was
wrong on the arm, wrong on the row, and it explains N2's negative result
completely.

### The filed predictions, judged

| filed | measured | verdict |
|---|---|---|
| the slowest arm is an **LZ arm** (`MIX12`/`MIX12H`) carrying its price replay | the **CM arms** are slowest by ~2×: `CM12P` 1,433 ms against `MIX12` 718 ms | **MISS** |
| the row's clock lands within **1.0–1.3×** of the slowest arm | the ROW is **2.07×** it (2,973 ms vs 1,433 ms); the big-roster STAGE alone is 1.34× | **MISS on the row, near on the stage** |
| the spread slowest-to-fastest is **> 2×** | 1,433 ms against `MIX10`'s ~450 ms — **> 3×** | **HIT** |
| `NUM`/`2D` cost about the same as `CM12`, **within ±15%** | on `vim-version9.txt`, **2D 1,699 ms against CM12 1,654 ms = +2.7%** | **HIT** |

**Where my reasoning went wrong, and it is instructive:** I wrote that the CM
arms push every byte through the twelve-input mixer twice while the LZ arms code
tokens, then talked myself out of my own first half by over-weighting the price
replay. The replay is real but it is **its own slot** (`MIX11-replay` 899 ms)
and the LZ arms are still half the cost of the CM arms. The first sentence of my
own reasoning was the answer.

### `kernel32.dll` (836,208 B), row 2,973 ms

```
armtime[plain] CM12P=1433 CM12=1411 CM12H=1357 CM12-book=1337 CM11H=1278
               CM11P=1228 CM11=1195 CM10=1099 | MIX11P=782 MIX12P=719
               MIX12H=711 MIX11H=705 MIX12=670 ... MIX10=453
stages         cheap-v8 trial 137 ms + tokenize 78 ms + BIG ROSTER 1917 ms
               (the remaining 841 ms is the peel arm, armor and the
                write-time round-trip law)
```

### `vim-version9.txt` (2,035,039 B), row 4,187 ms — the row the 2D arm WINS

```
armtime[plain] 2D=1699 CM12H=1670 CM12=1654 CM11=1526 CM11H=1516 CM10=1413
               | MIX11-replay=899 MIX12H=356 MIX12=350 MIX11H=323 MIX11=306
               MIX10=249
stages         cheap-v8 trial 783 ms + tokenize 763 ms + BIG ROSTER 2623 ms
```

### THE FINDING, and it closes N2

**There is no slowest arm. There is a PLATEAU.** Eight CM-class arms sit within
1,099–1,433 ms of each other on `kernel32.dll` and six within 1,413–1,699 ms on
`vim-version9.txt`. They run in parallel, so the stage ends when the last of
them ends — and **removing any ONE of them moves nothing, because the next one
is 2% behind it.**

That is exactly why N2's dialect gate bought nothing, and it is a stronger
statement than "the max, not the sum": *no single-arm change can ever move this
clock.* The gate was never going to work, and neither would gating any other
single arm.

**And the arm that sets the clock on `vim-version9.txt` is `2D` itself, at 1,699
ms — the arm that WINS that row.** So the one arm at the top of the list is the
one that cannot be dropped.

### What can actually move the floor, in order of measured size

1. **The shared CM inner loop.** Eight to nine arms are all `CM`-class and all
   within a few percent, so a change to the common path moves every one of them
   at once. It is the only lever that touches the plateau rather than a member
   of it.
2. **A whole CLASS, not a member.** `CM11`, `CM11H`, `CM10` cost 1,099–1,526 ms
   each and are FROZEN ancestor entrants, kept so "on every small file the
   elders always get their say" after `kernel32.dll` once breached the
   `<= min(ancestors)` law by one armor quantum. Dropping the class would move
   the plateau — and would break that law. **That is a trade to price, not a
   tuning, and it needs its own filed prediction.**
3. **`tokenize` and the cheap-v8 trial: 37% of the row on a text file**
   (763 + 783 of 4,187 ms), against 7% on `kernel32.dll`. Nobody has ever looked
   at either for speed, and on text they are the second-largest cost after the
   plateau.

### The instrument, and one defect it had first

`EGG_ARMS=1` now prints `armtime[...]` slowest-first and a `stages[...]` line.
Sizes are unchanged on every row checked — `kernel32.dll` 283,604,
`vim-version9.txt` 273,743, `alarm01.wav` 240,008, `wallpaper.jpg` 1,126,497 —
`cargo test` 45, `audit` 0 failing.

**The first cut used ONE bank of slots and raced.** `plain_transmute` runs
`big_arms` on the plain form and on the chosen filtered form **side by side in a
single thread scope**, so both rosters wrote the same 18 slots and both printed
whichever finished last — two identical lines, which is how it was caught. There
are three banks now, chosen by the caller's label. **The same lesson a third
time: the instrument gets a control before its output becomes a finding.**

---

## N2c FILED (2026-09-04, BEFORE the controls ran) — three questions N2b never asked

N2b concluded from one run that the clock is set by a PLATEAU of eight CM arms
and that **the shared CM inner loop is the only lever**. Before writing a line
of that, three things were checkable and unchecked.

1. **Is the plateau arithmetic, or is it the scheduler?** The machine is an
   Intel Core Ultra 9 285K — **24 cores, 8 P + 16 E, no SMT, 36 MB L3** — and no
   measurement in this project has ever accounted for the P/E split. FILED: the
   plateau's *membership and order* is stable run to run. If the membership
   shuffles, the plateau is the scheduler and N2c stops.
2. **How many arm threads are actually live?** `main.rs:775-788` runs `big_arms`
   on the plain form and on the chosen filtered form in the SAME thread scope.
   FILED: if a filter survives on `kernel32.dll`, the live count is ~30 on 24
   cores and N2's "removing an arm buys nothing" is true only at 18.
3. **What is the 841 ms nobody has named?** 2,973 − (137 + 78 + 1,917). FILED:
   the largest single component is **`Vec` teardown at process exit, >= 200 ms**
   (up to 30 models x ~71 MiB of tables). If under 50 ms, say so and move on.

## N2c MEASURED (2026-09-04) — two misses, and the tail was something else entirely

### The filed predictions, judged

| filed | measured | verdict |
|---|---|---|
| the plateau's membership **and order** is stable run to run | membership **100% stable** over 5 runs and 4 affinity masks — the 8 CM arms are ALWAYS above the 6 MIX arms, no crossing. **Order inside the class is noise: +-20%**, and the top arm changes every run (CM12H, CM12P, CM12H, CM12P, CM12H) | **HIT on membership, MISS on order** |
| the tail's largest component is **`Vec` teardown, >= 200 ms** | internal row 2,884 ms vs external wall 2,894 ms. **Process start + teardown is 10 ms.** The hypothesis is dead | **MISS** |
| a filter survives on `kernel32.dll`, so ~30 arm threads run | `armtime[filtered]` prints; **32 slots across two banks**, and the filtered form **WINS the row** (`filter 10:0`, 278,792 -> armored 283,604) | **HIT** |

### N2b's headline needs correcting, and so does one of my own numbers

**"The roster's wall clock is set by its SLOWEST arm, not by the number of
arms" is not what the data says.** Pinning the process to 8, 16 and 24 cores:

```
cores    row      BIG ROSTER   slowest arm   stage/max
   8    4,707 ms    3,601 ms     3,084 ms      1.17
  16    3,172 ms    2,133 ms     1,725 ms      1.24
  24    2,894 ms    1,838 ms     1,411 ms      1.30
```

The stage **scales with core count** — 8 -> 16 buys 1.69x, 16 -> 24 buys a
further 1.16x. So the roster is still CPU-limited at 24 cores with ~30 arms, and
arm CPU is not free. It is not "max" and it is not "sum": it is a queue with a
tail, and `stage/max` grows as cores are added because the queue drains.

**And a number I computed and had to throw away.** Summing the 32 `armtime`
values against the stage gave a beautifully stable **15.5x, five runs, +-0.1**,
which looked like a throughput law. It is an artifact: `timed()` measures
ELAPSED time, not CPU time, so an arm's number inflates when its thread is
descheduled. The 8-core run proves it — the same arithmetic gives **17.5x on 8
cores**, which is impossible. *Control before mechanism, a fourth time: the
check that killed it was running the same formula at a different core count.*

### THE TAIL, NAMED — and it is 29% of the floor row

```
tail[836208 B]: armor 2 ms + round-trip 822 ms (RAN) + write 0 ms
```

The 841 ms is **the in-memory round-trip law** (`main.rs:1011`), and it fires on
`kernel32.dll` **not because of size but because `fid != 0`** — the filtered
form wins the row, and v12-M3 extended the law to every filtered form. It is a
**serial, single-threaded full decode**: one thread, no roster, no parallelism
to hide behind. `armor()` is 2 ms and the write is 0.

`vim-version9.txt` is the control: the plain form wins there, `fid == 0`, the
law is **skipped**, and its tail is ~18 ms of 4,116.

### What this does to the lever

The round-trip decode runs the SAME `predict`/`learn`/`byte_update` as the arms.
So on the floor row the CM inner loop owns:

- the big roster, **1,823 ms** (throughput-limited across ~30 threads), plus
- the round-trip decode, **822 ms** (one thread, at full effect),

= **2,645 of a 2,875 ms row — 92%.** N2c's plan put the ceiling at "roster is
64% of the row"; with the tail named it is 92%, and a cut in the shared loop is
paid **twice**, once amortised and once whole. The lever is bigger than the
plan claimed, and it is the same lever.

**Not chased here, recorded for N3:** at ~30 threads on 24 cores the roster is
CPU-limited, so dropping arm CPU *does* move the clock — N2's null result holds
only where the roster is underloaded (`ntoskrnl.exe`, 11 slots). The arm-count
question is a scheduling question, not a byte question, and it is not this
milestone's.

## N2c PHASE 1 — the two ablations, judged

Throwaway builds, never shipped, each reverted after its reading.

| filed | measured | verdict |
|---|---|---|
| shrinking `o3s`/`o6s`/`ind2s` from `1<<20` to `1<<17` buckets is worth **>= 15%**; **under 5% and the memory hypothesis is dead** | **-5.9%** round-trip, -5.2% row | **MISS** — 48 MiB of tables per arm cut to 6 MiB, across ~30 concurrent arms, buys 5.9%. Just past its own kill line |
| the lattice scan is worth **4-10%**, not the top item | **-8.4%** round-trip with `lat_state = 2` (the detector removed outright) | **HIT** |

**Consequence, taken as filed:** the loop is **not** memory-bound in any way worth
chasing. Phase 2's item 2.7 (warming the seven bucket lines before `claim`)
was **dropped unbuilt** — its ceiling is a fraction of a 5.9% that costs 48 MiB
of context resolution to buy.

## N2c — A DEFECT IN THE MEASUREMENT PROTOCOL, found the only way it could be

`codegen-units = 1` measured **+3.3%** (a loss) and `target-cpu=x86-64-v3`
measured **+4.7%** (a worse loss). Two consecutive losses of the same size from
two unrelated flags is not a mechanism, so the baseline was re-read — and **the
baseline itself had moved from 2,821 ms to 2,933 ms.** Interleaved A/B in one
shell, base and candidate alternating, `target-cpu=v3` is **worth zero**.

**The machine drifts ~4% between measurement sessions, which is the same size as
every effect this milestone is chasing.** Every reading below is therefore
INTERLEAVED A/B, medians of 4-5 alternating runs. An accidental A/A run (a patch
that silently failed to apply) puts the noise floor at **+-0.2% on the
round-trip, +-1.2% on the roster stage** — so the round-trip, being one
single-threaded CM12 decode, is the instrument, and the per-arm `armtime`
numbers (+-20% run to run) are not.

## N2c PHASE 2 — the bit-exact wins, each measured alone

| # | change | round-trip | kept? |
|---|---|---|---|
| 2.1a | `codegen-units = 1` | **-1.2%** | no — inside the drift band, and it costs 60% more build time |
| 2.1b | `-C target-cpu=x86-64-v3` | **0.0%** | no — zero, and it would specialise the binary to this machine |
| **2.2** | the lattice scan: branchless, unit-stride, with a **const-length** steady-state path so the trip count is known at compile time | **-4.1%** | **YES** |
| **2.3** | `nf.update(b)` skipped where the lens is not `Num` — `NumField`'s state is observable only through `key0`/`key1`, which 8 of the 9 CM12 arms discard | **-2.3%** | **YES** |
| **2.4** | `pos % pixel` (a hardware `div` per byte on the 2D arm) carried as a rolling counter; the three per-byte `Lens` dispatches hoisted | *(with 2.3)* | **YES** |
| 2.5 | the 12-wide mixer accumulator split into 4 partial sums | **+0.1%** | no — LLVM was already breaking that chain. **Reverted** |
| 2.6 | `len_bucket(mlen)` computed once per bit instead of twice | **0.0%** | no — LLVM had already CSE'd it. **Reverted** |

**Two of the three items I expected to pay, paid nothing**, and both for the same
reason: the compiler had already done them. What paid was the work the compiler
could not remove — a branch it could not predict (2.2), a call whose result it
could not prove unused across a `&mut self` boundary (2.3), and a division it
could not strength-reduce (2.4).

### The result

```
kernel32.dll (the floor row)   2,818 -> 2,687 ms   0.297 -> 0.311 MB/s   -4.6%
  of which: BIG ROSTER         1,783 -> 1,696 ms                         -4.9%
            round-trip           814 ->   762 ms                         -6.4%
vim-version9.txt               4,090 -> 3,922 ms   0.498 -> 0.519 MB/s   -4.1%
  of which: BIG ROSTER         2,624 -> 2,450 ms                         -6.6%
```

The floor row clears 0.25 by **24%** where it cleared by 11%.

### Byte identity — the gate, and one gate that turned out not to be one

**14 of the 20 sealed rows re-measured, 0 moved, 0 failures, 42/42 injuries
EXACT**: the 12 `corpus-real` rows plus `cbs.log` and `ntoskrnl.exe`. Every lens
is covered — Plain (models 16/17/19/21/22/23), **Grid** (28: `alarm01.wav`,
`vim-version9.txt`, `cbs.log`), **Num** (27: `corpus/data.csv`, byte-identical
against the pre-change binary and restores EXACT), and the **peel** path (24:
`wallpaper.jpg`). **NOT re-run: `mermaid-bundle.js`, `msgraph.dll`,
`rdr2-shaders.vkcache`, `aoe4-autosave.sav`, `iconcache48.db`,
`rustc_driver.dll`** — six rows, hours each, and no lens they exercise is
untested above. That is a gap, and it is named rather than papered over.

**`tools/m0gate.js` is not the bit-exactness gate the N2c plan took it for.** It
asserts that eggv14's containers are IDENTICAL to eggv11's and match v11's
sealed sizes — the **v12-M0 fork** condition, which v12-M1 broke on purpose and
v13 broke by 17.9 MB. It reports FAIL on all 14 rows and has done since v12-M1;
running it on an unmodified v14 would fail exactly the same way. **Its live half
did pass, and it is the half a `mix11.rs` edit needs:** `.egg11`, `.egg10`,
`.egg9` and `.egg8` containers all restore **EXACT** through the modified binary,
14/14 — and the `.egg11`/`.egg10` restore paths run `mix11`/`mix10` decode, so
that is a real gate on the change, not a formality.

`cargo clippy` clean - `cargo test --release` **45** - `audit --full`
**3,091,771 checks, 0 failing**.

### What is left on the table, measured rather than guessed

- The **round-trip law is 28% of the floor row** and is a single serial decode.
  It cannot be removed (it is a safety law) and it cannot start earlier (it needs
  the winner), but every future cut to the shared CM loop is paid **twice** on
  any filtered or peeled row.
- The lattice's remaining **4.3%** (8.4% ablated - 4.1% taken) is its two mixer
  inputs and its lock work, which are **not** free: removing them moves bytes.
- On `vim-version9.txt`, `tokenize` + the cheap-v8 trial are still **36%** of the
  row (1,480 of 4,090 ms) and nobody has ever looked at either for speed.

---

## N3 — THE CARD, and the save row is a LOSS on our own headline

The card was installed for real this time: **precomp v0.4.7** (with preflate
0.3.5), **paq8px v216**, **zpaqfranz v64.8** — GitHub release binaries, run from
a scratch directory, never put on PATH. FLAC 1.5.0 and 7-Zip were already here.
Every rival below **restores the original byte for byte** and was checked doing
it; a number from a tool that cannot restore does not count.

### The row: `aoe4-autosave.sav`, 66,417,543 B — our largest win

Re-measured on this build first, so the comparison is against a number this
binary actually produces, not against the v13 record: **8,759,079 armored,
identical to sealed v13**, round-trip verified in memory, in 9m22s (v13 recorded
55–110 minutes for this row; N2c and the machine account for the rest).

Our own instrument breaks it down:

```
peel id 2 -- recipe 82,088,840 B raw -> 3,505,440 B (model 26)
             values 296,540,843 B raw -> 5,248,812 B (model 17)
             + 15 B preamble          = 8,754,267 B inner
```

The file is ONE gzip member at offset 0, zero trailing bytes, 4.465x; the inner
is Relic Chunky with no nested gzip.

### The card, on that row

| form | bytes | vs our inner | time |
|---|---|---|---|
| **precomp -cn + zpaq -m5** | **5,197,337** | **-40.6%** | 1m14s |
| precomp -cn + 7-Zip LZMA2 -mx9 | 5,443,648 | -37.8% | 20 s |
| precomp, its own default lzma2 | 6,028,745 | -31.1% | 5 s |
| **ours (peel + CM)** | **8,754,267** | — | 9m22s |
| zpaq -m5, no peeling | 18,262,092 | +108.6% | 2m24s |

**This is the largest loss the card has ever produced, and it is on the row this
project treats as its headline.** Precomp carries no shield — our 4,812 B of
armor is not in its number — but 3.3 MB is not an armor argument.

### WHERE the loss is: 97.9% of it is the recipe, 2.1% is the model

Both tools split the same file the same way, so the payload is directly
comparable. On the identical 296,540,843-byte values stream:

| coder on the values stream | bytes | vs ours |
|---|---|---|
| **zpaq -m5** | **5,172,122** | **-1.46%** |
| ours (model 17) | 5,248,812 | — |
| 7-Zip LZMA2 -mx9 | 5,420,208 | +3.26% |
| xz -9e | 5,453,332 | +3.90% |
| xz -9 | 5,525,752 | +5.28% |

**CORRECTION, filed against my own first reading of this row.** The first pass
measured only the LZMA rivals, saw us 3.2% ahead of the best of them, and
published "the loss is not the model". Then `precomp -cn + zpaq -m5` came back
at **5,197,337** — smaller than our values coding ALONE while also carrying the
recipe — and zpaq on the bare values stream is **5,172,122**. We are second of
five on the payload, ahead of every LZMA and **1.46% behind zpaq**. The rival
that mattered was the one I had not run yet, and the claim went out before it
did.

The gap decomposes cleanly:

```
our inner                                    8,754,267
precomp -cn + zpaq -m5                       5,197,337
                                             ---------
gap                                          3,556,930
  of which recipe  (3,505,440 - 24,160)      3,481,280   97.9%
  of which payload (5,248,812 - 5,172,122)      76,690    2.1%
```

**Our recipe is 40.0% of our shipped inner. Precomp does the same job in 24 KB.**
Give us precomp's recipe efficiency and the row lands near 5,273,000 — still
1.5% behind precomp+zpaq, but a 39.8% improvement on where we are, and the
argument moves from a rout to a model contest.

### The mechanism, named

We **store** a recipe: 82,088,840 raw bytes of it, describing how to re-emit the
deflate stream, then code that down to 3.5 MB. Preflate **predicts** the recipe —
it reconstructs zlib's parameters (block splits, tree choices, the match
decisions a given level would have made) and stores only the DIFFERENCE where
the prediction fails. On a stream produced by a stock zlib at a stock level,
that difference is nearly nothing, which is why 24 KB covers 66 MB.

That is a difference in kind, not in tuning, and it is the single largest
byte lever this project has ever had a measurement for: **3.48 MB on one row,
against the -17.96 MB v13 banked across all twenty.**

### Not yet run

zpaq and paq8px on the other rows; precomp on the remaining deflate-bearing
rows; PAR3 and Lepton still named-and-deferred (Lepton needs a build and there
is no gcc/cmake here). (`precomp -cn + zpaq -m5` landed: 5,197,337.)

### The card on the twelve home rows — zpaq -m5, and the shape of what we own

Naked against naked (our INNER, before the 4,812 B shield; zpaq carries none):

| row | input | ours (inner) | zpaq -m5 | delta |
|---|---|---|---|---|
| real-test.bmp | 12,000,054 | **256,462** | 3,568,770 | **+1291.54%** |
| ring01.wav | 498,420 | **130,753** | 201,932 | **+54.44%** |
| wallpaper.jpg | 1,602,752 | **1,121,685** | 1,516,991 | **+35.24%** |
| alarm01.wav | 491,516 | **235,196** | 288,598 | **+22.71%** |
| segoeui.ttf | 959,752 | **404,871** | 404,987 | +0.03% |
| notepad.exe | 360,448 | 171,352 | **170,835** | -0.30% |
| arial.ttf | 1,045,720 | 441,542 | **437,385** | -0.94% |
| zstd.exe | 1,601,409 | 484,103 | **474,147** | -2.06% |
| wubbadub.html | 92,408 | 22,809 | **22,271** | -2.36% |
| kernel32.dll | 836,208 | 278,792 | **269,792** | -3.23% |
| real-test.db | 9,551,872 | 1,063,337 | **1,006,283** | -5.37% |
| vim-version9.txt | 2,035,039 | 268,931 | **251,331** | -6.54% |

**zpaq -m5 — free, off the shelf, ~4 MB/s — beats us on 7 of 12 home rows**,
including the floor row (-3.23%) and `vim-version9.txt` (-6.54%). We beat it on
5, and the split is not random: **every row we win is a row where we have a
STRUCTURAL reading** — the RLE/2D bitmap (12.9x), the audio residue arms, the
JPEG peel — and every row we lose is one where a general context-mixing model
meets general bytes.

### paq8px v216 -9LAET, its strongest documented posture

| row | ours (inner) | paq8px | delta | paq8px time |
|---|---|---|---|---|
| wubbadub.html | 22,809 | **16,579** | **-27.31%** | 39 s |
| kernel32.dll | 278,792 | **195,733** | **-29.79%** | 233 s |

**~30% behind on text and on PE**, at **87x our wall clock** (paq8px does
`kernel32.dll` at 0.0036 MB/s against our 0.311). The structural rows are still
running.

### THE CARD'S VERDICT, and it is a strategy finding rather than a bug

Three rivals, three different answers, and together they say one thing:

1. **Our general CM model is mid-pack.** Ahead of every LZMA (3.3-5.3% on the
   save's payload), 1.5% behind zpaq, ~30% behind paq8px. It is not our edge and
   no amount of tuning it will make it one.
2. **Our structural readings ARE our edge, and they are decisive** — +1292%,
   +54%, +35%, +23% where a reading exists, against -0.3% to -6.5% where none
   does.
3. **And the one place a structural reading is BROKEN, we lose worst of all.**
   The deflate recipe is 40.0% of the save's inner and 97.9% of a 40.6% loss to
   a tool that solved it in 24 KB.

The lesson points the same way three times: **spend on structure, not on the
model.** Which makes the deflate recipe the first thing to fix, because it is
the one structural reading we have that a free tool beats by 145x — and N4's
ZIP peel would ship that same recipe across an entire new file class.

### paq8px, all six rows — a harsher verdict than the zpaq column

| row | ours (inner) | paq8px -9LAET | delta |
|---|---|---|---|
| real-test.bmp | **256,462** | 2,304,390 | **+798.53%** |
| wallpaper.jpg | 1,121,685 | **855,907** | -23.69% |
| vim-version9.txt | 268,931 | **199,844** | -25.69% |
| wubbadub.html | 22,809 | **16,579** | -27.31% |
| kernel32.dll | 278,792 | **195,733** | -29.79% |
| alarm01.wav | 235,196 | **143,511** | -38.98% |

**paq8px beats us on five of six by 24-39%, and our JPEG peel is one of them.**
It loses only on `real-test.bmp`, where we are still 8x ahead of it. At 87x our
wall clock (0.0036 MB/s on kernel32.dll against our 0.311), so it is a ratio
ceiling rather than a competitor — but the earlier claim that "structure is our
edge" holds against **zpaq's speed class**, not against the ratio state of the
art. Against paq8px the only structural reading that survives is the RLE/2D one.

### WHERE THE RECIPE GAP GENERALISES — the 29-file deflate suite under precomp

precomp `-cn`, recipe overhead measured against each file's true INFLATED size
(not against its compressed input), all 29 restored EXACT:

| what it recompressed | recipe overhead |
|---|---|
| `gz-l1` .. `gz-l9`, 92,408 B inflated, every zlib level | **115 B each** |
| gz-default 120 - huffonly 127 - stored-only 124 - binary 115 - smallwindow 173 | 115-173 B |
| gz-filtered 6,147 - gz-rle 5,398 - memlevel1 10,039 | 5-10 KB |
| **ours, aoe4-autosave.sav** | **3,505,440 B** |

As a share of the stream coded: precomp **0.0081%** on the save and ~0.12% on
the stock-zlib family; **ours 1.1821%**. The parse is not merely predictable on
one file — **115 bytes covers 92,408 inflated bytes at EVERY zlib level from 1
to 9**, and only an unusual strategy (Z_FILTERED, Z_RLE, memLevel=1) pushes the
correction stream to 1-2% of the stream. The save is stock-zlib-like, which is
exactly why precomp reaches 0.0081% on it.

**And one column where WE are ahead:** precomp's default **declined**
`bare-l6.deflate` and `zlib-l6.zz` outright (0/0 streams -- raw deflate and raw
zlib headers need `-intense`), passing them through at +95 B. Our peel reads
both (WRAP_RAW). The suite's hostiles are a wash: both sides restore all 29.

### N3b FILED — predict the parse, do not store it

`deflate.rs`'s own header says its approach "is preflate's approach (Dirk
Steinke) and precomp's (Christian Schneider)". **It is not.** Ours stores "every
match's length and distance" -- 38,340,574 tokens on the save, which is
substantially all of the 82,088,840 raw recipe. Preflate re-runs a
zlib-compatible matcher over the inflated bytes, PREDICTS the token zlib would
have emitted at each position, and stores only the corrections.

FILED, before any code:

1. On `aoe4-autosave.sav`, a stock zlib-style lazy matcher agrees with the
   stored parse on **>= 99.5% of the 38,340,574 tokens**. Under 95% and the
   whole approach is wrong for this file and N3b stops.
2. The row's inner falls from 8,754,267 to **under 5,400,000** -- i.e. the
   recipe falls by at least 3.35 MB -- putting us within 4% of precomp+zpaq's
   5,197,337 instead of 68% behind.
3. The correction stream costs **under 2% of the stream** on gz-filtered, gz-rle
   and memlevel1, the three suite files where preflate itself needs 5-10 KB.
4. **No row moves except the peeled ones**, and all 29 deflate-suite files still
   re-spell bit for bit. The peel's law is unchanged: one byte of difference and
   the peel is discarded for that file.

## N3b MEASURED (step 1) — the parse IS predictable, and v12's door was never the right door

`deflate.rs` records why v13 chose to store the parse: *"v12 shut this door by
searching for the encoder: 1,260 zlib configurations, best agreement 3 bytes of
4,096, first difference at byte 0."*

**v12 compared COMPRESSED BYTES.** Huffman tree construction and block splitting
diverge on the first block for any re-compression, so that search could only ever
return zero — it was measuring the wrong layer. Preflate never re-compresses: it
keeps the stream's own declared trees and block boundaries (which our recipe
already stores, and cheaply) and predicts only the **match/literal parse**.

Measured at the parse layer, on the first 4,000,000 output bytes of
`aoe4-autosave.sav` (1,420,054 original tokens):

| predictor | tokens | positions shared | identical where shared | **original tokens predicted exactly** |
|---|---|---|---|---|
| zlib -6 | 1,411,188 | 99.08% | 99.44% | **98.52%** |
| zlib -9 | 1,407,545 | 98.53% | 99.36% | 97.90% |
| zlib -1 | 1,175,077 | 79.94% | 85.89% | 68.66% |

**A first cut of this measurement compared tokens ELEMENTWISE by index and
reported 2.2-2.8% agreement.** That is an artifact: one differing token shifts
every index after it, so the number measures desynchronisation, not disagreement.
Aligning by OUTPUT POSITION is the only comparison that means anything, and the
token counts differing by just 0.6% was the clue that the elementwise reading
was wrong.

**Filed prediction 1 said >= 99.5%, with < 95% stopping the milestone. Measured
98.52% — above the kill line by a wide margin, below the filed figure.** It is
judged a MISS on the number and a PASS on the gate, and the gap is explained:
this predictor runs zlib INDEPENDENTLY and desynchronises, which is what the
0.92% of unshared positions are. Preflate advances by the ACTUAL token, so it
never desyncs, and it also INFERS the matcher parameters per stream rather than
assuming stock level 6. **98.52% is therefore a floor, not the estimate.** The
lockstep measurement is step 2 and it is what prediction 1 should have named.

Crude byte arithmetic at the floor: 1.48% of 38,340,574 tokens is ~567,000
corrections, plus a flag stream at that density (~38.3M x H(0.0148) = ~531 KB
coded) — call it ~1.1 MB against today's 3,505,440, so **~2.4 MB even if lockstep
buys nothing.** precomp's 24,160 B says lockstep plus parameter inference buys a
great deal more.

## N3b step 1b — HOW FAR THE DEFLATE PEEL REACHES, and the answer is one row

Before spending a milestone on the recipe, the milestone had to be sized: how
many corpus rows does the deflate peel touch at all? `precomp -cn -intense`
across all twenty rows finds streams in **twelve** of them —

```
kernel32.dll 10/10   zstd.exe 46/46   ntoskrnl.exe 77/77   msgraph.dll 4/4
notepad.exe 1/1      real-test.bmp 2/2  real-test.db 1/1   segoeui.ttf 1/2
wallpaper.jpg 1/1 (+JPG 1/1)           rdr2-shaders.vkcache 5/6
aoe4-autosave.sav 10/10 (+GZip 1/1)    rustc_driver.dll 1594/1597
```

— which looks like a corpus-wide opportunity and **is not one.** `-intense`
scans for raw zlib headers, and inside a PE image most hits are short byte runs
that happen to inflate. The count decides nothing; whether peeling BUYS bytes
decides it:

| row | streams | ours (inner) | zpaq -m5 alone | precomp -intense + zpaq | verdict |
|---|---|---|---|---|---|
| kernel32.dll | 10 | 278,792 | 269,792 | 271,365 | peel **LOSES** +0.58% |
| zstd.exe | 46 | 484,103 | 474,147 | 476,059 | peel **LOSES** +0.40% |
| ntoskrnl.exe | 77 | 4,670,575 | 4,437,863 | 4,512,893 | peel **LOSES** +1.69% |
| msgraph.dll | 4 | 3,207,068 | 3,145,272 | 3,146,364 | peel **LOSES** +0.03% |

**Peeling those streams loses on every binary row.** The expansion costs more
than the peel saves, on the strongest coder available. So the deflate peel
reaches **one row** — `aoe4-autosave.sav`, a real single 66 MB gzip member.

**N3b is therefore re-sized before it is built:** ~3.45 MB on one row (3.3% of
the ~105.5 MB sealed corpus) plus unblocking N4's ZIP peel, where deflate
members are the whole file rather than an artifact of a header scan. It is not a
corpus-wide win, and the earlier framing implied it might be.

**Two more rows for the zpaq column, taken from the same runs:** `ntoskrnl.exe`
**-5.0%** (4,437,863 against our 4,670,575) and `msgraph.dll` **-1.9%**
(3,145,272 against 3,207,068). zpaq -m5 now beats us on **9 of the 14 rows
measured**.

### The weight of the corpus is somewhere nobody has looked

Summing the sealed twenty: **`rdr2-shaders.vkcache` (41,860,990) and
`rustc_driver.dll` (37,436,978) are 79.3 MB of ~105.5 MB — 75% of the corpus by
weight — and no rival has ever been run on either.** Every card measurement in
this project's history has been taken on rows that together are a quarter of
what we ship. A 5% gap on those two is 4 MB, the size of all of N3b, and it
costs an hour to find out.

## N3b step 2 MEASURED — the matcher is built, and 38,340,574 tokens reduce to 22

`src/zmatch.rs`, a zlib-compatible `deflate_slow`/`deflate_fast` matcher with
zlib's configuration table, hash chains and `longest_match`, plus `infer()`
which reads the parameters out of the stream instead of assuming them. Wired to
a `zprobe <file>` subcommand so nothing ships dormant.

**On `aoe4-autosave.sav`, in our own code, over the WHOLE 296,540,843-byte
stream:**

```
1,171 blocks, 38,340,574 tokens (25,661,244 matches), recipe today =
  meta 312,466 + flags 4,792,572 + lens 25,661,244 + dists 51,322,488 B

  inferred zlib level 4 memLevel 9 from a 4 MB sample      1,501 ms
  WHOLE stream: 22 tokens need correction = 99.99994%      2,365 ms
```

**Twenty-two tokens out of 38.3 million.** The three parse streams -- 81,776,304
raw bytes, 99.6% of the recipe -- become two parameter bytes and 22 corrections.
This independently reproduces the Python reading (99.99994% over 12 MB) at full
scale, in the code that will ship it.

Filed prediction 1 (>= 99.5% agreement) is a **HIT** once the parameters are
inferred. The earlier 98.52% reading stands as what a FIXED level-6 predictor
gets; the same file at level 6 agrees on 82.7% of the whole stream. **The
parameter search is the finding, not the matcher.**

### What the matcher does NOT yet do, measured rather than assumed

`zprobe` over the deflate suite says the prediction is not universal:

| file | inferred | predicted exactly |
|---|---|---|
| gz-l9.gz / gz-default.gz / memlevel1.gz | level 9 | **100.00%** |
| gz-l6.gz | level 6 memLevel 8 | 99.44% |
| **gz-l1.gz** | level 1 | **54.96%** |
| **smallwindow.gz** | level 1 | **40.18%** |

Two defects, both real and both named before they are excused. `deflate_fast`
(levels 1..=3, the greedy path) is wrong -- 54.96% where the file IS level 1.
And `windowBits` is not modelled at all: the matcher assumes a 32 KB window, so
`smallwindow.gz` cannot be predicted by construction.

**Neither blocks the milestone, because the recipe will choose per file.** The
format keeps the stored parse as its fallback and takes the predicted one only
where prediction measurably wins; a file the matcher cannot predict simply ships
as it does today. The law of the peel is unchanged and still decides: re-spell,
compare against the original bytes, and one byte of difference discards the peel
for that file.

### The arithmetic this buys, stated before the format is built

Recipe raw 82,088,840 -> ~312,600 (meta, unchanged, plus 2 parameter bytes and
22 corrections) = **262x smaller raw**. Coded, today's 82,088,840 -> 3,505,440
is 4.27%; meta is block structure and Huffman tables rather than a token stream,
so it will not code that well. **Filed: the row's inner lands between 5,300,000
and 5,450,000** (today 8,754,267), i.e. the recipe falls by at least 3.30 MB. If
it lands above 5,600,000 the meta stream is the next thing to look at, and that
gets written down before anything else is tried.

## N3b step 3 SHIPPED — the recipe is predicted, and the row falls 39.6%

```
aoe4-autosave.sav: 66,417,543 B -> 5,289,632 transmuted -> 5,294,444 armored (7.97%)
  peel id 2 -- recipe 312,738 B raw ->    40,805 B (model 26)
               values 296,540,843 B raw -> 5,248,812 B (model 17)
  round-trip verified in memory before write
```

**8,759,079 -> 5,294,444. A 3,464,635 B win, -39.6% on the row**, and the row
runs 90 seconds FASTER (7m51s against 9m22s) because building 82 MB of parse
streams cost more than inferring a matcher and walking it.

### The filed prediction, judged

| filed | measured | verdict |
|---|---|---|
| the row's inner lands between **5,300,000 and 5,450,000** | **5,289,632** | **MISS, on the low side** -- better than the range by 10,368 B |
| if it lands above 5,600,000, `meta` is the next thing to look at | 40,805 B coded from 312,738 raw | not triggered |

The recipe went 3,505,440 -> 40,805, which is **98.8% of it gone**, and `meta`
coded far better than I allowed for (13.0% of raw, against the 4.27% the old
token-heavy recipe managed -- I had assumed block structure would code WORSE
than a token stream and sized the range around that).

### What the format does

Blob **version 2**: `nmatch` is 0, which empties the length and distance
sections by the arithmetic version 1 already used, and the flags section is
spent on two matcher parameters plus fixed 10-byte corrections. Version 1 is
written byte for byte as before, so **every container already in the world still
reads**. `expand()` rebuilds the parse on the decode side, once the inflated
values are in hand; the encode side keeps its streams in memory so the peel's
own law re-spells exactly as it always did.

**The decision is per file and it is a measurement, not a policy:** `try_predict`
runs only after the stored parse exists, takes the prediction only if it
reproduces that parse EXACTLY *and* costs fewer bytes, and otherwise changes
nothing. A greedy level we model badly, a window we do not model, a stored
block -- each keeps the stored parse and the row is what it was.

### The gates

- **Restore EXACT** on the shipped 66 MB container, read back from disk.
- **The 29-file deflate suite: 29 EXACT, 0 WRONG, 0 LOST** -- 14 took the peeled
  form, 2 refused with a reason, 13 were passed over by the argmin.
- 8 rows byte-identical; clippy clean; **51 tests**, including a new pipeline
  test that takes the save's own recipe through blob -> from_blob -> expand ->
  re-spell and compares the rebuilt flags, lengths and distances to the
  originals before checking the bytes.

### Two defects this milestone found in its own instruments

1. **`infer` reported its DEFAULT config and 8.1M corrections on a file that
   needs 20.** The 4 MB sample was cut mid-token, so the parse `infer` drove
   covered more bytes than the token slice it was scored against, and *every*
   config failed the comparison. Now the sample ends on a token boundary and a
   `debug_assert` in `infer` states the invariant it depends on.
2. **Corrections cannot be (index, token) patches.** A free-running matcher that
   disagrees 20 times emits **38,340,647 tokens against 38,340,574** -- a
   disagreement splits a match into literals and shifts every index after it.
   That was measured before the format was designed, which is the only reason
   the format is a lockstep rather than a patch list.

### Where the row now stands

| form | bytes | vs ours |
|---|---|---|
| precomp -cn + zpaq -m5 | 5,197,337 | -1.8% |
| **ours** | **5,294,444** | — |
| precomp -cn + 7-Zip LZMA2 | 5,443,648 | **+2.8% (we win)** |
| precomp, own lzma2 default | 6,028,745 | +13.9% |

**We still do not take that row outright** -- precomp+zpaq keeps it by 1.8%,
down from 40.6%. But across the whole corpus:

```
ours                            101,901,149
zpaq -m5 alone                  117,029,003   +14.85%  we WIN
precomp on the save + zpaq      103,964,248   +2.02%   we WIN
```

**The corpus-wide comparison against the best pipeline anyone can actually run
flips from a 1.33% LOSS to a 2.02% WIN**, and the projection filed before the
format was built (101,911,517) lands within 0.01% of the measurement.

## N3c — the PE gap is the MODEL, and the 0.5% margin has a price nobody had costed

The card's second-largest deficit is **1,962,323 B across six PE rows**, all
losing to `zpaq -m5` by 0.3-5.0%. Six rows failing the same way is one mechanism,
so the cheap question came first: is `FILTER_BCJ` (x86 rel32 -> absolute, LZMA's
Bra86) even firing on the rows that matter?

| row | filtered | plain | outcome |
|---|---|---|---|
| notepad.exe | 171,352 | 174,329 | **TAKEN** |
| ntoskrnl.exe | 4,670,575 | 4,964,397 | **TAKEN**, and it is worth -5.9% |
| zstd.exe | 481,857 | 484,103 | nominated, **discarded by the 0.5% margin** |

**It is firing.** `ntoskrnl.exe` carries 232,712 B of the deficit and already has
the transform applied, and still loses 4.98% to zpaq. **So the PE gap is a model
gap, not a filter-selection bug** -- which is a negative result worth having
before anyone spends a milestone on filter tuning.

### The margin, now with its price attached

`zstd.exe`'s filtered form is **2,246 B smaller** and is thrown away for missing
the margin by 0.036 of a percentage point (0.464% against the 0.5% required).
That looks like a defect and is probably not one: a filtered form sets
`fid != 0`, which fires the in-memory round-trip law (`main.rs`), and N2c
measured that law at **822 ms of a 2,875 ms row -- 28%**. The rule is therefore
trading up to 0.5% of bytes against a full serial decode of the winning arm.

**Filed, unbuilt:** the margin has never been priced against that decode. On
`zstd.exe` it costs 2,246 B; the question is what it costs across the corpus and
whether the decode is worth it at every size. That is a trade to measure, not a
constant to change, and it needs its own prediction before anyone touches it.

## N4 PRICED before it is built — a ZIP row costs us HALF

N4 was queued as "the largest new file class" with an unpriced payoff, because
no ZIP row exists in the corpus. Two real ZIPs off this machine, measured from
scratch rather than added to the corpus (that is a decision to take, not to
assume):

| row | members | ours (model 16, no ZIP peel) | zpaq -m5 | precomp -cn + zpaq -m5 |
|---|---|---|---|---|
| `python312.zip` (FL Studio's bundled Python 3.12 stdlib) | **599 deflate**, 0 stored | 3,753,980 | 3,742,241 (-0.31%) | **1,847,299 (-50.79%)** |
| `IPF-2.2.10205.3620.zip` (Alienware IPFCore) | 31 deflate, 3 stored | 5,653,821 | 5,642,170 (-0.21%) | **2,534,113 (-55.18%)** |

**We lose 50-55% on ZIP, and the whole of it is the peel.** zpaq beats us by only
0.2-0.3% there, because an unpeeled ZIP is incompressible to everyone -- the
rival's entire advantage is that it opens the members and we do not.

Against the board that makes N4 the largest lever by a wide margin: **half of any
ZIP row**, against 1,962,323 B (3-5%) for the PE model gap and 265,778 B for the
JPEG peel. And it now inherits the PREDICTED recipe rather than the stored one,
which is exactly what N3b was sequenced first to buy: `python312.zip`'s 599
members would each have carried a stored parse under v13's format.

A third shape found and worth recording: `intellij.libraries.icu4j.jar` is 35.6
MB of **5,826 STORED members and zero deflate** -- a ZIP that needs no peel at
all, only member-boundary awareness. N4 should not assume every ZIP is deflate.

## N6 PRICED — FLAC reproduces, and it is not the ceiling

At its strongest documented setting (`-8 -e -p --keep-foreign-metadata`), both
rows **restore EXACT**:

| row | ours | FLAC | gap |
|---|---|---|---|
| alarm01.wav | 235,196 | 209,449 | **-10.95%** |
| ring01.wav | 130,753 | 98,450 | **-24.71%** |

v13's readings (-10.68% / -22.63%) reproduce and were slightly conservative.
**N6 against FLAC is worth 58,050 B over two rows.** But paq8px gets
`alarm01.wav` to 143,511 -- 38.98% under us and 31.7% under FLAC -- so LPC+Rice
is not the ceiling on that row, it is a step toward it. The audio deficit
against the state of the art is ~124,000 B, and a FLAC-shaped peel collects
under half of it.

## The 0.5% filter margin, PRICED and CLOSED

Filed at N3c as a trade nobody had costed. Across `corpus-real`:

| row | verdict | bytes |
|---|---|---|
| zstd.exe | discarded by the margin | **2,246 left on the table** |
| segoeui.ttf | discarded by the margin | **1,198 left on the table** |
| arial.ttf | discarded | the filtered form was **954 B BIGGER** -- correctly refused |

**3,444 B across the whole home corpus**, and in exchange every one of those rows
avoids setting `fid != 0` and firing the in-memory round-trip law -- a full
serial decode of the winning arm, which N2c measured at 28% of a row. **The
margin is a good trade and does not want changing.** Measured and closed rather
than left as a suspicion.

### N4 FORECAST, filed before a line of it is written

What matters is not what precomp gets on a ZIP but what WE would get. Inflating
every member and handing the concatenated payload to our own coder:

| row | our values coded | precomp -cn + zpaq (whole file) | ours today |
|---|---|---|---|
| python312.zip | **1,865,974** (model 21) | 1,847,299 | 3,753,980 |
| ipf.zip | **2,587,675** (model 22) | 2,534,113 | 5,653,821 |

Our model is within **1.0%** and **2.1%** of the rival's COMPLETE number while
still owing its own recipe and the ZIP wrappers. With N3b's predicted recipe --
599 members of block structure and Huffman tables, call it 40-60 KB coded on
`python312.zip`, plus ~60 KB of local headers and central directory that ship
verbatim -- the row should land near **1,920,000-1,960,000**.

**FILED: `python312.zip` goes from 3,753,980 to under 2,000,000 -- a fall of at
least 45% -- and lands within 6% of precomp+zpaq.** Above 2,100,000 and the
per-member meta is the thing to look at, exactly as `meta` was the open question
on the save.

**The cost is now concrete, not a guess.** `peel::members` (`peel.rs:83`)
already reads the central directory and is tested against a `tiny_zip` fixture
and a hostile bad-offset case; `deflate::peel` already handles one member; THE
CHAIN already reaches depth 2. The gap is exactly what the handoff said: `Peeled`
holds `deflate: Option<Deflate>` -- **one** member -- where a ZIP needs many, and
the blob needs to carry N recipes plus the inter-member bytes verbatim.

**And the shape that would break a careless N4:**
`intellij.libraries.icu4j.jar`, 35.6 MB of **5,826 STORED members, zero
deflate**. Nothing to peel; only the member boundaries matter. A ZIP peel that
assumes deflate would refuse it, or worse, expand it.

## N5 — CANNOT be priced from this machine, and the reason is itself a finding

The corpus holds exactly **8 progressive (SOF2) files of 60**, which matches the
handoff's account of packJPG's 51/60 against our 43 to the file. Our JPEG peel
refuses them, so they ride the ordinary ladder (models 5 and 17 -- no JPEG model
at all).

Measured against the only JPEG-capable rival installed, on 5 of the 8:

| file | ours | paq8px | gap |
|---|---|---|---|
| Dark000 | 409,508 | 398,628 | -2.7% |
| Dark001 | 95,354 | 85,651 | -10.2% |
| Dark002 | 182,821 | 173,093 | -5.3% |
| Dark003 | 80,798 | 71,288 | -11.8% |
| Light000 | 559,554 | 547,577 | -2.1% |

**paq8px is 2-12% ahead on progressive, against 23.69% on baseline
`wallpaper.jpg`.** The obvious reading is that paq8px does not model progressive
JPEG either, and on these files both of us are running a general model. So the
gap above is NOT the size of N5's prize -- it is the size of our general-model
deficit on JPEG-shaped bytes, and it says nothing about what a progressive peel
would buy.

**N5 stays named-and-deferred, and now for a stated reason rather than by
inheritance: its only real rival is packJPG, which is not installed, and no
tool on this machine can price it.** Anyone picking N5 up should install packJPG
first and measure, exactly as N4 was priced before it was built.

## The audio ceiling is far above FLAC

paq8px on `ring01.wav`: **53,567 against our 130,753 -- we are 59.03% over**, and
paq8px is **45.6% under FLAC's 98,450**. With `alarm01.wav` (143,511 against our
235,196) the audio deficit against the ceiling is **168,871 B over two rows**,
where a FLAC-shaped LPC+Rice peel collects 58,050 of it -- about a third.

That reframes N6: it was scoped as "build FLAC's mechanism as a WAV-gated peel",
and that remains a real 58 KB, but it is not the row's ceiling and should not be
sold as one.

### The N4 forecast, re-grounded on MEASURED per-member cost

The first forecast estimated the per-member recipe. It has now been measured:
599 raw deflate members were extracted from `python312.zip` (reading each LOCAL
header's own name and extra lengths, not the central directory's -- the two
disagree, which `peel::members` already knows) and 60 of them probed.

| | 60 members measured | scaled to 599 |
|---|---|---|
| tokens / corrections | 484,548 / 944 = **99.805% predicted** | ~9,424 corrections |
| predicted parse | 9,560 B raw | **~95,441 B** |
| meta (per member 337 B) | 20,195 B raw | **~201,613 B** |
| the STORED parse this replaces | 1,034,509 B raw | **~10.3 MB** |
| lockstep failures | **0 of 60** | |

Recipe **~297 KB raw against ~10.3 MB stored**, so the row lands near
**1,925,000** -- values 1,865,974, recipe ~39,000 coded at the save's observed
13% of raw, ZIP wrappers ~20,000. That is inside the filed "under 2,000,000" and
**48.7% under today's 3,753,980**.

**Prediction quality is 99.805% here against 99.99995% on the save.** Small
members are harder -- less signal for the parameter search, and 7 of 60 predict
perfectly where the save's single 296 MB stream did. It is still 100x better
than storing the parse, and 0 of 60 failed lockstep, so the fallback never fires
on this file. Worth knowing before N4 is built: **the ZIP case is a
many-small-streams case, and its recipe is dominated by META (201 KB) rather
than by corrections (95 KB)** -- the reverse of the save, where meta was 312 KB
and the corrections were 142 BYTES. If N4 lands above 2,100,000, per-member meta
is the thing to attack, and this is the number to attack it against.

## N4 step 0 — the two silent failures inside the loop, fixed before the loop exists

N4 runs the predictor once per ZIP member -- 599 times on `python312.zip`. Two
things in that loop failed **silently**, which is the shape that has cost this
project a day already, so they were fixed before a line of N4.

### 1. `infer` invented a configuration when none fit

It returned `(Cfg::new(6, 8), usize::MAX)` on total failure, and **both callers
discarded the count** (`let (cfg, _) = infer(...)`). So "no zlib configuration
explains this stream" was indistinguishable from "level 6 explains it
perfectly". That is exactly how a mid-token sample once reported 8.1M
corrections on a file needing 20. It returns `Option` now; `try_predict` falls
back to the stored parse on `None`, and `zprobe` prints the no-fit instead of
hiding it.

### 2. `deflate_fast` was wrong, and three files prove the fix

The greedy loop (levels 1..=3) was running `deflate_slow`'s rules. Three
distinct bugs: it applied the `prev_length < max_lazy` gate, which does not
exist in `deflate_fast`; it applied the TOO_FAR reduction, which is
`deflate_slow`'s alone; and it hashed across every match, where zlib **skips the
inserts entirely** for a match longer than `max_lazy` and re-primes `ins_h` from
two bytes.

| file | was | now |
|---|---|---|
| gz-l1.gz | 64.56% | **100.00000%** (level 1, 0 forced) |
| gz-l2.gz | 67.85% | **100.00000%** |
| gz-l3.gz | 71.62% | **100.00000%** |

**The deflate conservation suite went from 14 files taking the peeled form to
17**, 29/29 EXACT throughout. That is the fix paying for itself on rows that had
been silently falling back.

It does **not** move `python312.zip` (99.805%, unchanged) -- those members are
not written at a fast level -- but ZIP writers commonly are, and a level-1
archive would have fallen back on every member without a word.

### Still open, now measured and LOUD rather than silent

| hole | cost | why it is not fixed here |
|---|---|---|
| `windowBits` unmodelled | smallwindow.gz 59.9% | ZIP mandates a 32 KB window (APPNOTE), so N4 does not meet it. Sweeping it would multiply `infer`'s 81 configs by ~8 |
| `Z_FILTERED` / `HUFFMAN_ONLY` / `Z_RLE` strategies | 83.6% / 57.8% / 57.5% | rare in archives; each needs its own `longest_match` variant |

Both now produce a poor fit that is **reported and falls back**, rather than a
confident wrong answer. `cargo test` **52**, clippy clean, 8 rows byte-identical,
and the save still infers level 4 / memLevel 9 with 0 corrections on its sample
and 20 on the whole stream.

## N4 SHIPPED — the ZIP peel, and two filed thresholds missed by a nameable cause

`src/zip.rs`, `PEEL_ZIP = 3`. A file becomes alternating spans — gap, member,
gap, ..., gap, with `gaps.len() == members.len() + 1`. Gaps are carried
VERBATIM: local headers, the central directory, the EOCD, and **every member
this peel did not take**. A member is taken only if `deflate::peel` reads it AND
re-spells it exactly; anything else stays in the gap and the archive is peeled
around it.

| row | before | after | |
|---|---|---|---|
| python312.zip (599 deflate) | 3,753,980 | **2,027,506** | **-46.0%** |
| ipf-alienware.zip (31 deflate + 3 stored) | 5,653,821 | **2,925,145** | **-48.3%** |
| icu4j.jar (5,826 STORED, 0 deflate) | — | 7,691,219 | **no peel nominated** |

**All three restore EXACT**, read back from disk.

### The filed predictions, judged — misses first

| filed | measured | verdict |
|---|---|---|
| the row lands **under 2,000,000** | **2,027,506** | **MISS** by 27,506 (1.4%) |
| it lands **within 6%** of precomp+zpaq (1,847,299) | **9.8% behind** | **MISS** |
| a fall of **at least 45%** | **-46.0%** | HIT |
| above 2,100,000 and per-member meta is the thing to attack | 2,027,506 | not triggered |

**The cause is one number in the trace, and it is not meta:**

```
zip: 599 members peeled, 8,840,215 B inflated;
     recipe 69,074 B verbatim frame + 415,766 B of member recipes
     (331 predicted, 268 stored)
```

**268 of 599 members ship a STORED parse.** `try_predict` takes the prediction
only when it costs fewer bytes than the streams it replaces, and on a small
member the corrections lose that comparison — 20 corrections at 10 B each is
200 B against a parse that may itself be under 200 B. So the forecast, which
assumed all 599 would predict, undershot the recipe: **489,649 B raw where I
forecast ~297,054**, coding to 156,705 against my ~59,000.

That is a good refusal doing its job — every one of those 268 is a member where
storing genuinely wins — but it is also **the named next lever for N4**: the
per-member recipe on SMALL members. `ipf-alienware.zip`, whose 31 members are
large, predicts **31 of 31** and its recipe is 337,275 B raw for 14.3 MB
inflated.

### What the hostile proved

`icu4j.jar` — 35.6 MB, 5,826 members, all stored, zero deflate — is **not
nominated at all**: `zip::nominates` reads the central directory and finds no
method-8 member, so no peel is attempted, nothing is expanded, and the row goes
through the ordinary ladder and restores EXACT. That was the shape written into
`corpus-zip/suite.txt` as the one that breaks a peel assuming deflate, and it
does not break this one.

### Gates

clippy clean · `cargo test --release` **56** (4 new in `zip.rs`, including a
constructed archive whose LOCAL headers carry different name/extra lengths from
its central directory — the disagreement that makes reading the wrong header
silently read the wrong bytes) · **8 sealed rows byte-identical**, which matters
because `zip::nominates` now runs on every file · deflate suite **29 EXACT, 0
WRONG, 0 LOST**.

### Still not a sealed row

`corpus-zip/` remains its own suite. N4 is proven; **whether a ZIP row joins the
sealed twenty is the next decision**, and it changes every future comparison.

## N4 POST-SEAL — my diagnosis of the 268 was WRONG, and the row breaks the speed floor

Three findings, and the first is a correction to what I published one commit ago.

### 1. The 268 stored members: the cause I named is false

The N4 commit says they ship a stored parse because "on a small member 20
corrections at 10 B each lose to a parse under 200 B". **That was inferred, not
measured, and it is wrong.** A sweep of all 599 members says:

```
  members 599  (no-fit 0, lockstep-fail 0)
  predicted at W=10   599        REFUSED on width   0
  would flip at W=7 : 0     at W=4 : 0     at W=2 : 0
```

**Every one of the 599 wins its size comparison.** None is refused on the
correction width, so a varint record -- the lever I named and the one suggested
back to me -- **buys exactly zero members.** Both of us were aiming at a gate
that never fires.

The real cause is a guard I wrote myself, `deflate.rs:1011`:

```rust
if d.ntok == 0 || d.values.len() < 4096 { return; }
```

**268 of 599 members inflate to under 4,096 B** (median member: 5,254 B), so
`try_predict` returns before it ever tries. Counted independently: exactly 268.

**Priced:**

| | members | stored | predicted | saving |
|---|---|---|---|---|
| below the guard (never attempted) | **268** | 159,366 B | 3,876 B | **155,490 B** |
| above it (predicted today) | 331 | 2,490,966 B | 48,682 B | 2,442,284 B |

Dropping the guard puts the recipe at ~334,159 B raw, ~106,930 coded at the
observed 32%, and the row near **1,977,731** -- under the 2,000,000 I filed and
missed. **268 flips against the "seal instead if under ~100" bar.**

### 2. The peel was never timed, and it is half the row

```
peel 3: [parse 33,679 ms]   THE LAW re-spelled and compared in 30 ms
python312.zip  3,814,526 B -> 2,027,506 armored in 71,457 ms (0.053 MB/s)
```

**The parse is 47% of the row**; the law that validates it costs 30 ms. The
parse is `infer` running up to 81 lockstep passes per member, 599 times, in
series. Parallelising per member is **byte-identical** -- members are
independent and each inference is deterministic -- and is the obvious first cut.

### 3. THE FLOOR IS BROKEN, and promoting this row is what broke it

`python312.zip` runs at **0.053 MB/s against the home floor of 0.25** -- five
times under, measured twice on an idle machine. It is a `corpus-real` row now,
so the floor applies to it.

Even a free parse leaves ~38 s of roster and lands ~0.10 MB/s, because the
roster models the **inflated** 8,840,215 B, not the 3,814,526 B of input. **The
floor counts input bytes per second, and a peel row's work is proportional to
what it inflates to.** `wallpaper.jpg` never exposed this: its JPEG parse is
42 ms and its values ride a dedicated model.

That is a bar question, not a bug, and it wants an explicit answer rather than a
quiet one: move the row to `corpus-big` (where the bar is wall clock), restate
the floor to measure modelled bytes, or record the breach. **Restating a bar to
accommodate the row that broke it is the option that needs the most argument,
and it should not happen silently.**
