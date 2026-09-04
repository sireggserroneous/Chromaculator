
### M3c CORRECTION, filed 2026-09-03 BEFORE the S2a lever was written

The census ran first, as filed, and **it killed my own sharpened reading before
the plan's**. `EGG_JSTATS=1` on `wallpaper.jpg`:

```
jstats: 432000 blocks, 1608209 nonzero ACs (3.72/block),
        1608209 mag codings over 768 contexts (2094.0 each),
        1535236 mbits bits over 768 contexts (1999.0 each),
        tables 53544 Pr = 214176 B
```

**Where my arithmetic was wrong:** I counted `mag`'s 12,288 `Pr` as 12,288
CONTEXTS. They are **768 contexts x a 16-node tree**. The dilution figure I filed
(`~6 codings per context after the change`) is off by exactly the tree width:
after the change `mag` has 768 x 4 x 4 = **12,288 contexts** carrying 1,608,209
codings, i.e. **131 each**, and `mbits` 12,288 contexts carrying 1,535,236, i.e.
**125 each**. Both are healthy. The nonzero count I estimated at "roughly 1.3M"
is **1,608,209** -- 24% higher, which helps further.

**So the sharpened reading is WITHDRAWN and re-filed, before any lever code:**

- **S2a-1 `mag` += (nzb, qb[k])**: called **-1% to -3%**. The "one chance in three
  of losing outright" is withdrawn: at 131 codings per context there is no
  dilution cliff. A small loss is still possible if `qb[k]` is redundant with
  `kb` -- the quantisation table is a function of the band on most encoders.
- **S2a-2 `mbits` += (ba, bl)**: called **-0.5% to -1.5%**, unchanged.
- **S2a-3 both**: called **-1.5% to -4%** -- which is now exactly the plan's own
  range. **I withdraw the claim that -4% is out of reach.**

The record keeps both columns: the first sharpened reading MISSED on its input
arithmetic, and the instrument that was filed to check it is what found the miss.
