//! fold.rs -- the fold's coordinate, and the anti-transpose in it.
//!
//! The site draws the anti-transpose inline and has no function for it
//! (`index.html:398`: `const pr = n - 1 - c, pc = n - 1 - r;`), and it
//! characterises the map axiomatically in five places -- "fixes the Fold,
//! swaps Inner with Outer, and undoes itself, which are precisely the three
//! defining properties of inversion in a circle" (`index.html:175-177`,
//! `spec.md:76-77`, `glossary.js:58-62`, `index.html:311-312`,
//! `inspirations.html:413-417`). It never commits to a map.
//!
//! This module supplies the coordinate that turns those three axioms into
//! one. For a cell `(r, c)` of an `n x n` grid let `d = r + c`, the
//! anti-diagonal index -- what the site calls the place-value band
//! (`index.html:170/172/174`, "the low / middle / high place values"). Set
//!
//! ```text
//! rho(r, c) = 2 ^ (d - (n - 1))
//! ```
//!
//! Then Inner is `|rho| < 1`, the Fold is `|rho| = 1`, Outer is `|rho| > 1`,
//! and the anti-transpose sends `rho -> 1/rho`, which IS inversion in the
//! unit circle. The unit circle is the Julia set of `z -> z^2`; its two
//! Fatou basins are the inside, attracted to 0, and the outside, attracted
//! to infinity. See `dynamics.rs` for that half.
//!
//! THE CAVEAT, kept here rather than in the README because it is the one
//! place this argument can be attacked. A cell's weight is `2^-(i+1)` in the
//! STALK index `i` (`spec.md:23`, `glossary.js:48-52`), not in `d`. The
//! site's strong claim that "the anti-diagonals ARE the place values" is
//! exact for the PRODUCT RECTANGLE, where weight is `2^-(r+c+2)`
//! (`spec.md:108-110`; `productRegions` sets `w: r + c + 2` at
//! `stalk.js:229-237`; `squashDiagonals` sums by `r+c` and comments
//! "S[d] rides weight 2^-(d+2)" at `stalk.js:336-341`). On a single folded
//! stalk, anti-diagonal `d` holds `arcs(n)[d]` cells spanning a BAND of
//! place values. So `rho` is an exact normalised place value on the product
//! grid and a magnitude ORDERING on the folded stalk. That is exactly what
//! the site's own bridge sentence says, and this round claims no more.

pub const INNER: u8 = 0;
pub const FOLD: u8 = 1;
pub const OUTER: u8 = 2;
pub const NAMES: [&str; 3] = ["inner", "fold", "outer"];

/// `stalk.js:118-126`'s own comparison: above, on, or below the main
/// anti-diagonal. Pinned against the site's `regions()` by the audit.
#[inline]
pub fn region_of(r: usize, c: usize, n: usize) -> u8 {
    let s = r + c;
    if s + 1 < n {
        INNER
    } else if s + 1 == n {
        FOLD
    } else {
        OUTER
    }
}

/// The anti-diagonal index measured from the Fold: `d - (n - 1)`. Negative
/// inside, zero on the Fold, positive outside. Exact, for exact tests.
#[inline]
pub fn band_of(r: usize, c: usize, n: usize) -> i64 {
    (r + c) as i64 - (n as i64 - 1)
}

/// The coordinate: `2^band`. A power of two, so it is exact in `f64` for
/// every grid this round measures.
#[inline]
pub fn rho_of(r: usize, c: usize, n: usize) -> f64 {
    let b = band_of(r, c, n);
    if b >= 0 {
        (1u128 << b) as f64
    } else {
        1.0 / ((1u128 << (-b)) as f64)
    }
}

/// The anti-transpose, `index.html:398`'s two lines as one function.
#[inline]
pub fn sigma_rc(r: usize, c: usize, n: usize) -> (usize, usize) {
    (n - 1 - c, n - 1 - r)
}

/// The same map on the row-major index the codecs use: `sigma(j) = L - 1 - jT`.
#[inline]
pub fn sigma_idx(j: usize, n: usize) -> usize {
    n * n - 1 - ((j % n) * n + j / n)
}

/// Inversion in the unit circle, in the coordinate. `sigma` induces exactly this.
#[inline]
pub fn sigma_rho(rho: f64) -> f64 {
    1.0 / rho
}

/// `stalk.js:39-44` -- the anti-diagonal lengths, `1, 2, ..., n, ..., 2, 1`.
/// The site claims these as the sphere's structure at `index.html:313-314`:
/// "single-cell poles, widest ring at the Fold, equal hemispheres".
pub fn arcs(n: usize) -> Vec<usize> {
    let mut out = Vec::with_capacity(2 * n - 1);
    for d in 0..(2 * n - 1) {
        let lo = (d + 1).saturating_sub(n);
        let hi = if d + 1 < n { d } else { n - 1 };
        out.push(hi - lo + 1);
    }
    out
}

/// Cells per class, for a whole grid, under an arbitrary assignment.
pub fn class_sizes(n: usize, assign: impl Fn(usize, usize, usize, usize) -> u8) -> [usize; 3] {
    let mut out = [0usize; 3];
    for j in 0..n * n {
        out[assign(j / n, j % n, j, n) as usize] += 1;
    }
    out
}

/// The number eggSo-v0's verdict rested on: the chance two distinct cells
/// land in different classes. Closed form, from the class sizes alone.
pub fn separation(sizes: &[usize; 3]) -> f64 {
    let l: usize = sizes.iter().sum();
    let pairs = |m: usize| (m * m.saturating_sub(1)) as f64 / 2.0;
    let total = pairs(l);
    1.0 - sizes.iter().map(|&m| pairs(m)).sum::<f64>() / total
}

#[cfg(test)]
mod tests {
    use super::*;

    /// P1, the coordinate. Ground filed in PREDICTIONS.md: 89,439 cells,
    /// n = 2..64, zero exceptions, for both halves of the claim.
    #[test]
    fn rho_reproduces_the_regions_and_sigma_inverts_it() {
        let mut cells = 0usize;
        for n in 2..=64usize {
            for r in 0..n {
                for c in 0..n {
                    cells += 1;
                    let rho = rho_of(r, c, n);
                    let by_rho = if rho < 1.0 {
                        INNER
                    } else if rho == 1.0 {
                        FOLD
                    } else {
                        OUTER
                    };
                    assert_eq!(by_rho, region_of(r, c, n), "region at ({r},{c},{n})");

                    let (pr, pc) = sigma_rc(r, c, n);
                    let rho2 = rho_of(pr, pc, n);
                    assert_eq!(rho2, sigma_rho(rho), "sigma is not 1/rho at ({r},{c},{n})");

                    let fixed = (pr, pc) == (r, c);
                    assert_eq!(fixed, region_of(r, c, n) == FOLD, "fixed set at ({r},{c},{n})");
                    if fixed {
                        assert_eq!(rho, 1.0);
                    }
                    assert_eq!(sigma_rc(pr, pc, n), (r, c));
                }
            }
        }
        assert_eq!(cells, 89_439, "cell count over n = 2..64");
    }

    #[test]
    fn sigma_idx_agrees_with_sigma_rc() {
        for n in 2..=40usize {
            for j in 0..n * n {
                let (r, c) = (j / n, j % n);
                let (pr, pc) = sigma_rc(r, c, n);
                assert_eq!(sigma_idx(j, n), pr * n + pc);
                assert_eq!(sigma_idx(sigma_idx(j, n), n), j);
            }
        }
    }

    /// `region_of(sigma j) = 2 - region_of(j)`, the swap eggSo-v1 pinned.
    #[test]
    fn sigma_swaps_inner_and_outer() {
        for n in 2..=40usize {
            for j in 0..n * n {
                let (r, c) = (j / n, j % n);
                let (pr, pc) = sigma_rc(r, c, n);
                assert_eq!(region_of(pr, pc, n), 2 - region_of(r, c, n));
            }
        }
    }

    /// The site's own sphere claim, `index.html:313-314`.
    #[test]
    fn arcs_give_single_cell_poles_and_equal_hemispheres() {
        for n in 2..=40usize {
            let a = arcs(n);
            assert_eq!(a.len(), 2 * n - 1);
            assert_eq!(a[0], 1, "north pole at n={n}");
            assert_eq!(a[a.len() - 1], 1, "south pole at n={n}");
            assert_eq!(a[n - 1], n, "the Fold is the widest ring at n={n}");
            assert_eq!(a.iter().sum::<usize>(), n * n, "the arcs tile the square at n={n}");
            let north: usize = a[..n - 1].iter().sum();
            let south: usize = a[n..].iter().sum();
            assert_eq!(north, south, "equal hemispheres at n={n}");
            assert_eq!(north, n * (n - 1) / 2);
        }
    }

    /// P6, the correction. eggSo-v0 judged the fold against a fair THREE-way
    /// split and called it sub-optimal. Against a fair TWO-way split -- the
    /// right family if the Fold is a boundary rather than a class -- it wins,
    /// and by a positive margin at every n.
    #[test]
    fn the_fold_beats_a_fair_two_way_split_at_every_n() {
        for n in [4usize, 8, 16, 32, 64, 128] {
            let l = n * n;
            let half = l - l / 2;
            let fold = separation(&[n * (n - 1) / 2, n, n * (n - 1) / 2]);
            let two = separation(&[half, l / 2, 0]);
            assert!(fold > two, "fold {fold} vs fair two-way {two} at n={n}");
        }
        let fold = separation(&[496, 32, 496]);
        let two = separation(&[512, 512, 0]);
        let three = separation(&[341, 342, 341]);
        assert!((fold - 0.5303).abs() < 5e-5, "fold separation {fold}");
        assert!((two - 0.5005).abs() < 5e-5, "two-way separation {two}");
        assert!((three - 0.6673).abs() < 5e-5, "three-way separation {three}");
    }

    /// `(r+c) mod 3` -- the fold's own level sets -- hits the optimum exactly.
    #[test]
    fn diag3_hits_the_optimal_split() {
        let sizes = class_sizes(32, |r, c, _j, _n| ((r + c) % 3) as u8);
        assert_eq!(sizes.iter().sum::<usize>(), 1024);
        let mut sorted = sizes;
        sorted.sort_unstable();
        assert_eq!(sorted, [341, 341, 342]);
        let sep = separation(&sizes);
        assert!((sep - 0.6673).abs() < 5e-5, "diag3 separation {sep}");
    }
}
