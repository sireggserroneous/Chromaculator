# Chromaculator

An instrument for looking at integers. Lives at **chronochromatic.org**.

Write a number in hex, so its bits arrive padded to a whole nibble. Lay them most
significant first into the smallest square that holds them, and fold that square
along its main anti-diagonal into three regions — **Inner**, **Fold**, **Outer** —
which sum back to the number exactly. Blue is +1, red is −1, green is 0. Negating a
number flips every colour.

Cell *i* weighs 2<sup>−(i+1)</sup>, so the value is *k* / 2<sup>4·nibbles</sup> —
always a dyadic rational strictly inside (−1, 1).

## The pages

| | |
|---|---|
| **Spectrometer** | one integer in full: its stalk, its square, its three regions, its value as a light wave, and its point on the unit sphere |
| **Atlas** | every integer as a ring of dyadics, ordered by value. Ring *r* is the 2<sup>r+1</sup> roots of unity; selecting one draws a radius and lights the path down to it |
| **Wub** | integers as phasors summed tip to tail, each riding an ellipsoid, tracing one closed curve. Torus knots, crossing counts, and a curve you can export |
| **Wub ×** | multiplication as a rectangle; an ordered sequence of operands |
| **Wub ÷** | division kept exact: quotient, a multiplier on the boundary, and the remainder drawn |
| **Wubba Dub** | every operation on one page: each card is plain, pushed, or an operand — + − × ÷ |
| **Spec** | the convention, editable in the page |
| **Inspirations** | who found each piece of this first |

## Running it

```
python3 serve.py        # http://localhost:1338
```

Static files plus one `PUT` endpoint so `spec.md` can be edited from the Spec page.
Only that one file is writable. On a static host the site still works; the Spec page
becomes read-only.

## Checking a change

```
node tools/run.js wub.html      # does the page's JavaScript actually run?
node tools/product.test.js      # the product grid, against the arithmetic
node tools/wubx.test.js         # Wub x, including the parts run.js cannot reach
node tools/gizmo.test.js        # the corner gizmo points where it says
node tools/divide.test.js       # A = 2^e x Q x B + R, at every width
node tools/wubdiv.test.js       # Wub div, including the parts run.js cannot reach
node tools/load.test.js         # all three Wub pages under a full rack
node tools/running.test.js      # the 2^E-times-a-stalk pair all four operations share
node tools/wubbadub.test.js     # the paged cards, and that they reproduce the other three
node tools/bulk.test.js         # 10,000 integers through both grids (~20s)
node tools/chroma-order.test.js # the Chroma Certified Ordering certificate
```

`run.js` runs a page's JavaScript against a stand-in DOM. A 200 from the server says
nothing about whether the page's scripts executed; this does. See `tools/`.

## What is original here

Very little of the mathematics. Signed digits are Booth and Avizienis, the stalks are
Conway's, the bit reversal is van der Corput and Cooley–Tukey, the ring geometry is a
rotary encoder disc, the phasor sum is Fourier epicycles. See **Inspirations** for the
full accounting, with links.

What I have not been able to place is the fold itself: laying a digit string along the
anti-diagonals of a square and cutting it into three regions that sum back to the
number. Held loosely.

## The ordering

`data/chroma-base.tsv` is the Chroma UTF base table: 306 Latin letters and
digits ordered `base | case | accent`, generated from the real DUCET
(`data/allkeys.txt`) with case ranked above accent. It is the one source — the
certificate reads it rather than re-deriving it, because deriving it a second
time with `Intl.Collator` gave a different accent order. Locale collation is
not DUCET.

    tools/fetch-ucd.sh            # allkeys.txt + Unihan, into data/ (not committed)
    python3 tools/chroma_utf.py   # orders every assigned codepoint, certifies, writes data/
    node tools/chroma-order.test.js   # the 8-property certificate on the base table

`tools/chroma_utf.py` extends the ordering over the whole character map. Every
assigned codepoint gets a reading and sorts as if spelled in Chroma UTF; the
reading comes from Unicode's own data rather than a hand table.

| layer | source | characters |
|---|---|---|
| 1 | in Chroma UTF — Latin, digits | 306 |
| 2 | Hangul — the Unicode name *is* the romanisation | 11,172 |
| 3 | Han — Unihan `kMandarin`, then Cantonese, on'yomi, Korean, plus `kTotalStrokes` | 47,063 |
| 3\* | Han with no reading on file — falls back to its name | 51,619 |
| 4 | other script — the letter core of the Unicode name | 15,033 |
| 5 | symbol / emoji — the whole name (`TOP HAT`) | 23,660 |
| 6 | unnamed — private use, controls; sort last by codepoint | 143,678 |

Key, most significant first: `letters… | 0 | language | tone | strokes | codepoint`.
The letter run is 0-terminated, so a reading that is a prefix of another sorts
first. The tail levels are the tie breaks, each a sub-level of consecutive
codes — the same shape as `e < é < è < ê < ë`.

## The phonetic index — multi-listing

We read C as /k/ or /s/, so C belongs in the k path **and** the s path. Every
character is listed in every place its sound can put it, which makes the
phonetic ordering an **index** rather than a permutation: one entry per
(character, reading, language).

    python3 tools/chroma_phonetic.py

    entries            370,571
    multi-listed       27,732 characters sit in more than one place
    widest             阸×16, 啐×14, 噦×13, 噲×13, 僤×13

飼 sits in seven places — `si` (Mandarin), `zi` (Cantonese), `shi` and `ji`
(on'yomi), `kau` and `yashinau` (kun'yomi), `sa` (Korean). 行 sits in ten.

**Language is a filter, not a level.** Declare it and the impossible branches
are pruned; what remains is a *subsequence* of the full order, never a
reordering of it — that is property [4] of the certificate, checked by
comparing sort-then-filter against filter-then-sort. Region narrows a language
rather than siding with another region, so a request for `es-ES` accepts
entries tagged `es` but not `es-419`.

    c in Spanish (Spain)          /θ/  /k/
    c in Spanish (Latin America)  /s/  /k/
    c in Italian                  /tʃ/ /k/
    c in English                  /k/  /s/

Which is what rules out the impossible readings: *Cerveza* in Latin America has
`z` = /s/, so seseo `c` can no longer pair with distinción `z`. Two branches for
the word, not 108.

**The final tie break is the spelling, and the spelling order is the spine.**
Sound discards the accent — á, ä and ǎ all read `a` — so entries that tie on
sound fall back to Chroma UTF's own declared order rather than to the
codepoint. The `a` run reads `a á à ă â ǎ å ǻ ä ǟ ã ȧ ǡ ą ā ȁ ȃ` inside the
phonetic index too.

## Two lists

Same construction twice: position in the sorted list becomes the code, in the
smallest ring that holds the list. Ring *r* gives *r*+1 bit codes.

| | entries | ring | slots | code width | codes | spare |
|---|---|---|---|---|---|---|
| **short** — Chroma UTF base | 306 | 9 | 512 | 10 bits | 512..817 | 206 (40%) |
| **long** — phonetic index | 370,571 | 19 | 524,288 | 20 bits | 524,288..894,858 | 153,717 (29%) |

Ring 18 holds 262,144 — too small — so 19 is the smallest ring that takes the
long list.

The short list is the **spine**: 306 characters in spelling order,
`base | case | accent`. The long list is the **index**: one entry per
(character, reading, language), in sound order.

They are two *orders*, not one nested inside the other. The short list is not a
subsequence of the long one — 3 of its 305 steps run backwards, and all three
are multi-listing doing its job:

    Ư reads 'U' at 204,050   ->   v reads 'b' /b/  at   5,311   (betacism)
    Ŵ reads 'V' at 204,778   ->   x reads 'kh' /x/ at 128,094
    X reads 'KH' at 136,258  ->   y reads 'i' /i/  at 112,255

But the spine is the long list's **final tie break**, which is how the declared
accent run survives inside the sound order:
`a á à ă â ǎ å ǻ ä ǟ ã ȧ ǡ ą ā ȁ ȃ` then every other script's `a`.

## Sorting filenames

    python3 tools/chroma_sort.py shit 飼 this isit
    python3 tools/chroma_sort.py --lang ja-on shit 飼 this isit
    ls ~/Downloads | python3 tools/chroma_sort.py --lang en -

A word is not a bag of characters: `shit` is /ʃ/ /ɪ/ /t/, not /s/ /h/ /ɪ/ /t/.
So a string is segmented into graphemes first, longest match wins, and each
grapheme carries the same branches a character does.

    no language declared        Japanese (ja-on)
      isit                        isit
      shit                        飼    shi
      飼    si                     shit
      this                        this

飼 moves because /ʃi/ is a prefix of /ʃit/. Separators (`. _ - /` and spaces)
sort below every letter and spell nothing.

A string with branching graphemes occupies more than one position, exactly as a
character does — `this` has 9, 飼 has 7. To *sort* you need one position per
string, so the key is the **primary** branch of each grapheme, not the minimum:
taking the minimum read `isit` as `ishit`, because `s` has a /ʃ/ branch in
German and `sh` sorts before `si`. A real position for the string, a nonsense
canonical for it.

On a real directory, 5 of 16 positions differ from alphabetical, and every
mover is a `c` that sounds like `k` — `canvas.png` sorts under K.

### Context rules

A branch carries a positional condition, so a word prunes what a character
alone cannot:

    ^        word-initial          $        word-final
    >set     next grapheme starts with one of set     !>set   does not
    <set     previous grapheme ends with one of set
    @a|b|c   the whole word is one of these — the exception hook

    cat -> kat        city -> siti        gem -> jem       game -> game
    knight -> nit     night -> nit        enough -> enouf  ghost -> gost
    xylophone -> zilofone   box -> boks   psalm -> salm    who -> wo
    rose -> roze      canvas -> kanvas    cervezas -> servezas

The **character index still lists every branch** — a character on its own has
no context, so `c` belongs in the k path and the s path both. The word sorter
prunes by condition. Same table, two readers.

The `@` hook exists because some distinctions are not positional at all.
English θ vs ð is **lexical**: a closed list of function words and stems takes
/ð/ and no rule about position separates `this` from `thin`. Rules plus a short
exception list, which is how every real g2p is built. All four minimal pairs
separate:

    this -> dhis /ðis/        thin -> thin /θin/
    mother -> modher /moðeɹ/  month -> month /monθ/
    with -> widh /wið/        width -> width /widθ/
    breathe -> breadhe        breath -> breath

`tools/chroma_sort_test.py` carries the requirement a filename sort has that a
character index does not: **it must be a permutation.** Drop or duplicate a
file and the sort is worse than useless.

## Wub UTF

    python3 serve.py            # then http://localhost:1338/wubutf.html

The ordering, drawn. Every character is a Chroma UTF code in ring 9, so every
code is ten bits in the same four by four frame — ten bits pad to three whole
nibbles, twelve bits lay into a four by four square, and the four leftover
cells are empty in every frame. That is what makes a row read as a row.

A word is a strip of those squares, and **the first square where two names
differ holds the cell that decided the order.** The page marks it.

    dis  | this     decided at square 2, cell 9
    this | thin     decided at square 1, cell 4

So `dis` is *closer* to `this` than `thin` is, which is invisible in an
alphabetical list where d and t are far apart.

The page carries no copy of the tables. The ordering has one implementation, in
Python, and `serve.py` exposes it read-only:

    /api/base            the 306 base characters and their codes, and the ring
    /api/read?q=&lang=   every grapheme of q: its branches, reading and codes
    /api/sort?q=&lang=   q's lines ordered by sound

A change to the tables is a change everywhere. `tools/wubutf.test.js` tests the
seam — the page's own logic against real API output, then the whole stack over
a live socket, including a 404 on an unknown endpoint and a 413 on an oversized
query.

## A word is a polynomial

    node tools/chroma_poly.test.js

Exactly, not by analogy. A word's key is

    value = SUM code_i * x^(i+1),   x = 2^-12

    cerveza = 734x + 574x^2 + 722x^3 + 792x^4 + 574x^5 + 810x^6 + 522x^7

and lexicographic key order equals numeric value order for the same reason the
product rectangle is carry free: the largest coefficient is 810 and the field
is 4096, so no field ever carries into its neighbour.

The multi-listing is the rest of it. A word is a **product of per-grapheme
sums**, and expanding that product enumerates the readings:

    cerveza   branches 4 x 1 x 1 x 2 x 1 x 4 x 1 = 32 terms

Declaring a language is **specialisation** — it sends terms to zero, and the
survivors are always a subset:

    cerveza  32 terms  ->  en 1 (serveza)  ->  es-ES 1 (therbetha)

### Where the push analogy stops

Push and the reading set share a shape — one object, many representations, one
canonical choice — and picking the primary branch really is the analogue of
pushing to the fixpoint. But **push conserves value and phonetic variation does
not.** Cerveza's 32 readings hold 32 different values, so no reading is
reachable from another by pushing. And pushing a word's own bits conserves its
value exactly while introducing 47 negative cells; no Chroma UTF code is
negative, so a pushed word is a value, not a spelling.

## 100,000 words, three filters

    python3 tools/chroma_scale.py en.txt es.txt ja.txt

40,000 English, 40,000 Spanish, 20,000 Japanese, mixed. Each filter holds:
permutation kept, keys nondecreasing, and 100,000/100,000 distinct keys — no
two of a hundred thousand words collide.

**A declared language set is a choice per word, not one rule for the list.** The
detector is the branch set, as Pedro put it: look the word up in each declared
language and the hits name the language. First declared wins a tie.

    detection   claimed by one lexicon 98,142, by several 1,858, by none 0
                overlaps: en+es 1,858

    'en' -> 'en es'          99,889 of 100,000 positions changed, 17,701 readings
                             median displacement 522, max 83,642
    'en' -> 'en ja-on es'    99,895 of 100,000 positions changed, 31,136 readings
                             median displacement 1,392, max 96,224

    亜美      96,511 ->    287    yamei  -> abi
    本文      10,385 -> 38,720    benwen -> honbun

## Two axes, four combinations

Plain or pushed is the **representation** axis. As written or phonetic is the
**reading** axis. They are independent, so:

| | as written | phonetic |
|---|---|---|
| **plain** | `cervezas` | `servezas` |
| **pushed** | 32 shape variants — `c3rv3zas`, `c3rv32as`, … | every reading — `servezas`, `thervezas`, `chervezas`, … |

Pushed stops collapsing a name to one representation and lists every one it
has, each at its own position. On the spelling axis those are the **shape**
variants, so `cervezas` and `c3rv3zas` are one thing written two ways and both
produce the same 32-variant set. On the sound axis they are the readings.

### The shape axis is orthogonal to the sound axis

Not a rival. Declaring `leet` alongside `en` means English sounds over leet
shapes — `c3rv3zas` reads `servezas`, the same key as `cervezas`. Treating
`leet` as another candidate *language* made it compete with `en` for the single
primary slot, `en` won by being declared first, and the shape branches never
got a look in.

Shape is a **pre-pass**, because conditions test their neighbours: `c` before a
literal `3` took the `!>eiy` branch and `c3rv3zas` came out `kervezas` when the
`3` plainly stands for an `e`. Shape resolves first; sound reads what it leaves.

### Two push levels on the sound axis

    in context    32 readings   servezas, thervezas, chervezas, tservezas …
    rules off    120 readings   the same plus kervezas, kerbethas, …

`kervezas` is **not** a reading of `cervezas` in context — the positional rule
says `c` before `e` is never `/k/`, so it is pruned before the branch set is
built. Out of context it is, because a character on its own has no position to
be judged by, which is exactly how the character index already reads one.

## base(chroma-utf)

    python3 serve.py            # then http://localhost:1338/wubutf.html

A string is a number written with Chroma UTF digits. **One card per line,
commas between the items on a card**, the way Wub ± holds a rack:

    card 1 : (hello, 3, 45)

      hello  digits [616, 574, 658, 658, 680]
             616x + 574x² + 658x³ + 658x⁴ + 680x⁵      x = 1/4096
             = 0.616 574 658 658 680 in base 4096

### The value is the fraction, not the integer

`0.hello`, in (0,1) like every other stalk on the site. Leading with the
integer was wrong: `SUM code_i · B^(n-1-i)` grows with the digit count, so **it
sorts short words first whatever they say** — `he` beats `hell` on length alone.
Measured against the real key:

    0.word    he hell helllo hello helo   == the sort
    integer   he hell helo hello helllo   != the sort

The fraction also gets prefix ordering **for free**: a missing digit reads as
zero, zero is below every code, so `hell` is smaller than `hello` with no
terminator rule at all. The integer is kept because Wub ± takes integers, and
`tools/wubutf.test.js` keeps it as a live control — if it ever starts agreeing
with the key, the check has stopped meaning anything.

Each card pages through **Bodies** (one 4×4 square per digit), **Polynomial**,
**Facts** and **Rack**.

### Why 12 bits a digit and not 10

A code only needs ten bits, but **12 is three whole nibbles**. So a character
never straddles a nibble, and laid four cells wide every character is exactly
three rows. At ten bits nothing lines up — five characters is fifty bits, which
is twelve and a half nibbles.

### One integer construction, both axes

The digits change with the reading axis; the construction never does. That is
what keeps a spelling and its phonetic reading comparable pictures, and it is
the polynomial `tools/chroma_poly.test.js` already certifies — the integer and
the picture are the same object.

## Three alphabets

    python3 tools/chroma_ipa.py

| | symbols | ring | digit | arithmetic | nibbles |
|---|---|---|---|---|---|
| **Chroma UTF** — the base table | 306 | 9 | 12 bits | — | 3 |
| **the phonetic index** — multi-listed | 370,571 | 19 | 20 bits | — | 5 |
| **Chroma IPA** — the chart | 126 | 7 | **8 bits** | **mod 127, a field** | **2** |

126 symbols: 108 chart letters, 13 prosody marks, and 5 addresses. **Eight bits
is two whole nibbles**, which neither of the other two can reach.

### Rank 0 is the zero and stays empty

A trailing rank-0 digit is invisible — `0.5` and `0.50` are one number — so a
symbol parked there cannot be told from not being there. `/sit/` and `/sitˈ/`
were the same value until this was fixed. Ranks start at 1.

That makes **127 elements**, and 127 is prime, so **every nonzero digit has an
inverse** and division is total. Storage is nibble-aligned at 8 bits; the
arithmetic is mod 127. Two roles for one digit.

    Z/123  = 3 x 41     80 of 122 divide    65.6%
    Z/127  PRIME       126 of 126 divide   100.0%   <- a field
    Z/128  = 2^7        64 of 127 divide    50.4%

Filling all 128 slots looks tidier and is strictly worse to compute with: only
odd digits invert. The linking mark and half-long were dropped to land on 127.

### The five addresses

    ⦾ tiny-on < ⦿ tiny < ↑ up < ○ over < ε epsilon < every sound

Pedro's anatomy of 0, at ranks 1..5 — below every mark and every letter.
**Each is one digit, not two:** a stalk carries the sign, so `under` is `over`
negated, `down` is `up` negated, `miny` is `tiny` negated. Five digits, ten
addresses.

They sit at the bottom because **appending any digit is already the
"infinitesimally above" move** — every appended digit lands between `hello` and
`hellp`, whatever it is. The rank does not decide *whether* you went up, it
decides *how far*. These are the smallest increments there are.

Their spellings begin with `0` where prosody begins with `1`, so the order still
derives from Chroma UTF rather than being declared.

### The order is derived, not declared

Not place|manner|voice. Every symbol already carries a Chroma UTF spelling, so
**the IPA order is those spellings in the order the base table already
declares.** Nothing new is judged, and the adjacency falls out:

    /s/ s and /ʃ/ sh   +1   immediate
    /d/ d and /ð/ dh   +2   retroflex /ɖ/ spells dd and sorts between them

All ten stop/fricative pairs share a first digit; five are immediate. The ones
that are not have a sibling phoneme sitting between, which is a fact about the
chart rather than noise.

**Diacritics are not digits.** They modify a segment the way an accent modifies
a letter, so they are a sub-level, the same shape as `base | case | accent`.

### What it buys

    word      chroma utf   digits  bits   ipa       digits  bits   saving
    shit      shit              4    48   ʃit            3    24    50%
    this      dhis              4    48   ðis            3    24    50%
    think     think             5    60   θink           4    32    46%
    cervezas  servezas          8    96   seɹvezas       8    64    33%

Two savings compound: one sound is one digit where `sh`, `th`, `dh` cost two,
and the digit is eight bits rather than twelve. `/api/ipa` serves the alphabet
and a query's reading in it.

## Alphabets as tie-breakers

Stack them and each one resolves what the last could not. 20,000 English words:

    1. IPA alone              19,852 distinct,   293 tied (1.47%)
    2. + Chroma UTF reading   19,937 distinct,   124 tied (0.62%)
    3. + the spelling itself  20,000 distinct,     0 tied (0.00%)

145 homophone groups survive alphabet 1 — `sicks sics six`, `right wright writ`,
`Toni Tony tony`. Alphabet 2 splits most of them because the *reading* differs
even when the sound does not, and alphabet 3 is total by construction: distinct
words have distinct spellings.

Levels 2 and 3 are already the key. **Chroma UTF comes first and IPA is its own
kind of sorting**, so the two are *siblings, not levels*: pick one with
`alphabet=chroma` (the default) or `alphabet=ipa`. Within the IPA sort, ties
fall back to the Chroma key — the cascade again.

An alphabet is **one choice**: it fixes the digits, the base, the frame they
draw in *and* the order. Sorting by one while drawing the other would put the
divergence marker on a digit that decides nothing.

| | digits | base | frame | padding |
|---|---|---|---|---|
| Chroma UTF | 12 bits | 4,096 | 4×4 | 4 of 16 |
| Chroma IPA | 8 bits | 256 | 3×3 | 1 of 9 |

    shit   chroma  [734, 616, 622, 746]  48 bits
           ipa     [100, 58, 104]        24 bits

The frame has to come from the alphabet's **declared width**, not from the
value. Chroma UTF codes are `2^9 + i`, so the leading 1 is always there and
every code is ten significant bits — the frame comes out uniform for free.
Chroma IPA uses bare ranks, so rank 1 has one significant bit and rank 126 has
seven, and sizing to the value drew small ranks 2×2 and large ones 3×3. Same
alphabet, two frames, and a row stops reading as a row.

    chroma   church  cat  shoe  sun  top
             /tʃuɹtʃ/ /kat/ /ʃoe/ /sun/ /top/

    ipa      cat  sun  shoe  top  church
             /kat/ /sun/ /ʃoe/ /top/ /tʃuɹtʃ/

`church` moves behind `top` because an affricate is **two** IPA symbols and
/tʃ/ opens with /t/. Over 20,000 words that relocates 99.7% of positions — which
is exactly why it is a separate sort rather than the default.

## Rank orders, code draws

    python3 tools/chroma_utf.py

Rank and code stopped being the same number. **Rank** is the position in the
order and what mod-arithmetic runs on. **Code** is where a symbol sits in the
digit space, and it decides two things rank cannot: how much of every square is
dead, and where a character points when it is drawn as a phasor.

Packed, all 306 codes sat in 512..817 — **a 27° wedge out of 360**. Every
character in the alphabet pointed the same way, so a word's phasors landed on
top of each other and a rack was a smear rather than a shape. So each type gets
a share of the ring and is strided to fill it:

    digit    10 symbols  0x100..0x2ff  stride 51
    letter  296 symbols  0x400..0xfff  stride 10

    codes 256..3974, spanning 327° of 360     (was 27°)
    4 of 16 cells dark in every body          (was 6)

The type comes from Unicode (`Nd` is a digit); the block bounds are **declared** —
the same split as "DUCET orders, we tailor". Blocks land on nibble boundaries, so
the leading nibble — the top row of a body — identifies the type by range.

**Blocks alone are barely better than packed**, because inside a block every
member shares its leading bits. The stride *within* the block is what kills them.

### The invariant that made it safe

**The code rises with the rank.** Comparing tuples of codes is identical to
comparing tuples of ranks while that holds, so every code could be respaced
without moving a single ordering — the 100k run, the collision cascade and the
homophone groups all stand unchanged. Only the values moved.

## Wub UTF

    python3 serve.py            # then http://localhost:1338/wubutf.html

Cards on the left, sphere on the right. **Each character is a phasor**: its code
is the angle, its place in the word is the weight (`2^-(i+1)`), and its own
square gives it a height — Fold the equator, Inner north, Outer south, the same
convention Wub ± and Wub ÷ use. A word is a rack, summed tip to tail.

Click a string and it goes to the sphere, with its **1D X/Y/Z** and **2D
XY/XZ/YZ** projections below. Drag to turn, or click an axis on the gizmo.

## Chromaculator

    python3 serve.py            # then http://localhost:1338/chromaculator.html

A Desmos-shaped list on the left, a field of black bodies on the right. **A card
is a black body**, and what you type in it becomes phasors spaced evenly around
it:

    3            1 phasor
    3, 5         2 at 180°
    3, 5, 7      3 at 120°
    1,2,3,4,5,6  6 at 60°

Cards take expressions, evaluated as **exact rationals in BigInt** — never
floats, because the whole system's claim is that the value is exact:

    47*127          = 5969        dyadic, lands exactly
    (13*3*127/2^4)  = 4953/16     dyadic
    (1/3)           = 1/3         not dyadic — cut to 16 cells, carries its remainder

Only a power-of-two denominator can end, so `1/3` is `0101…` for ever. It is cut
at a declared width and says what it dropped, which is the same answer Wub ÷
gives when only 430 of its 2178 quotients came out exact. A malformed item is
reported in place and the good ones still draw.

### `{:}` — the shared midpoint

A body does not sit still. **A card is itself a phasor one level up**, swinging
about the common point `{:}`, and its items orbit its moving tip. Two cards are
180° apart about `{:}`, three are 120°, four are 90° — the same n-gon step the
items use inside a card, which is `360/n`, the **exterior** angle of an n-gon.
Tip to tail, twice over.

A body's own phasor is what its card **sums to**, so `3, 5, 7` swings as 15 and
the three points hanging off it are how it got there. Each swing is normalised,
so `3` travels as far as `47*127` and neither drowns the other.

### The card is a Wubba Dub card

Not a lookalike — the same renderer. `dominoesHTML`, `boxes`, `factsHTML` and
`nmOf` moved out of `wubbadub.html` into `stalk.js`, so a Chromaculator card
draws what a Wubba Dub card draws:

    3   Inner 0.000000  Fold 0.125000  Outer 0.062500
        Value 3/16 · 446 nm · 2 green
        Cells 0011 · 4 in 2x2    Commas 0,01,1
        Push  1--1 = 3/16        Spread no trailing greens to fill

Page 0 is the number — dominoes, the boxed grid, the facts, one block per item
on the card. Pages 1 and 2 are its 1D and 2D projections. Dots at the bottom of
each row page through.

`nmOf` had five copies across the pages before this and now has one.

### Variables

A card that reads `name = expr` is a **knob and a body**: it takes a slider,
every card after it can use the name, and it still draws — it has a value.
(Typing `a = 3` and getting an empty field was the page refusing to show the one
thing on it.) Slide `a` and everything downstream moves.
Definitions read in order, so one can build on another — and an unknown name is
an **error**, not a zero, because a typo should say so rather than quietly draw
the wrong thing.

### Collapse to `{:}`

Every point, **at time t**, measured from `{:}` instead of from its own body.
The bodies stop swinging apart and their point groups sit on the one common
origin — each card keeps its own phase step, so `3, 5, 7` is still three waves
120° apart, they are just drawn against the shared midpoint.

It is **not** a sum. `alignByWeight` will add `3, 5, 7` into a single 15, cells
`[1111]` on ring 4 — that is true, and it is a different question. Collapsing
keeps all three waves and only moves where they are measured from.

Where a body starts is still the **Fibonacci lattice** — evenly spread with no
relaxation to run and no two landing on each other, checked at 1, 2, 3, 8 and 30.

`math.js` would have done the parsing, and it evaluates to doubles. The parser
is forty lines in `stalk.js` instead.
