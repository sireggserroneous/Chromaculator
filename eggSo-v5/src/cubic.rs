//! cubic.rs -- the degree-3 geometry, and its honest loss.
//!
//! Cayley, 1879, could state the three-root problem and not see it. We have
//! the computer he did not, so: what does THIS grid look like at degree
//! three?
//!
//! eggSo-v4 placed the fold as the Julia set of `z -> z^2`, with Inner and
//! Outer its two Fatou basins and the anti-transpose the inversion between
//! them. That placement used `rho` alone, because `rho` is a modulus and a
//! degree-2 map needs no more than one. Degree three needs the angle -- and
//! v4 left the angle unspecified, because the coordinate it built was a
//! modulus and nothing else.
//!
//! The angle is not free. `rho` gives one modulus to the `arcs(n)[d]` cells
//! of a band, and the site's own fill order already distinguishes them:
//! `stalk.js:102-110` reads each anti-diagonal "from the bottom-left corner
//! upward". So the `k`-th cell of band `d` takes angle `2*pi*k / arcs(n)[d]`,
//! and `dynamics::z_of` is the whole construction -- band for the radius,
//! Hankel walk for the angle, nothing invented.
//!
//! Classify each cell by which root of `z^3 - 1` Newton's method reaches.
//! That is the degree-3 partition: what the fold WOULD HAVE TO BE if this
//! geometry could carry three basins.
//!
//! THE BAR IS THE PICTURE AND THE NAME, and it is filed here as it was filed
//! in PREDICTIONS.md, before any number:
//!
//!   * it CANNOT beat `(r+c) mod 3`, which already takes 200 of 200 on the
//!     burst channel;
//!   * it will probably be WORSE, and for a reason worth stating: **Fatou
//!     basins have interiors, and interiors concentrate.** v4's own picture
//!     of the cubic shows large solid lobes; a burst landing inside a lobe
//!     lands in one class. A fractal boundary scatters only the cells near
//!     it, and those are a minority;
//!   * its class sizes will be UNBALANCED, because the three basins of
//!     `z^3 - 1` do not have equal measure inside any particular annulus and
//!     `rho` reaches only the radii the grid's bands provide.
//!
//! The name: **the basin decomposition of a cubic Newton map** -- a Newton
//! fractal. Cayley 1879 for the question, Julia 1918 and Fatou 1919-20 for
//! the theory, and the computer era for the picture. v4's lineage audit found
//! zero prior mentions of any of it across nineteen experiments, and Part 1
//! adds no site claim, because a Newton fractal is not the site's geometry --
//! it is what the site's geometry would have to BECOME at degree three, and
//! that distinction is the point of the round.

use crate::dynamics as dy;
use crate::fold;

/// The iteration cap, v4's, so the two rounds agree cell for cell on every
/// point they share.
pub const ITERS: usize = 200;

pub struct Cubic {
    pub n: usize,
    pub class: Vec<u8>,
    pub sizes: [usize; 3],
    /// cells Newton did not settle inside `ITERS`, resolved by nearest root.
    /// A partition must be total, so these are resolved -- and counted, so
    /// the resolution is never mistaken for part of the construction.
    pub unsettled: usize,
    /// the cells of the main anti-diagonal, by class: the one channel where
    /// a fractal boundary can do what a straight seam cannot
    pub fold_band: [usize; 3],
}

pub fn partition(n: usize) -> Cubic {
    let roots = dy::roots3();
    let mut class = vec![0u8; n * n];
    let mut sizes = [0usize; 3];
    let mut unsettled = 0usize;
    for (j, slot) in class.iter_mut().enumerate() {
        let (r, c) = (j / n, j % n);
        let (k, fell_back) = dy::cubic_class_of(r, c, n, &roots, ITERS);
        *slot = k;
        sizes[k as usize] += 1;
        if fell_back {
            unsettled += 1;
        }
    }
    let mut fold_band = [0usize; 3];
    for r in 0..n {
        fold_band[class[r * n + (n - 1 - r)] as usize] += 1;
    }
    Cubic { n, class, sizes, unsettled, fold_band }
}

/// N2, the picture: the GRID coloured by basin, one character per cell.
///
/// v4 drew the cubic in the complex plane, where it is a famous picture. This
/// draws it where the round actually lives, on the site's own square, and the
/// thing to read off it is that it is legibly **not a seam**: no straight
/// line separates these three classes, which is Cayley's wall arriving on the
/// grid rather than in the plane.
pub fn picture(class: &[u8], n: usize) -> Vec<String> {
    let marks = *b".o#";
    (0..n)
        .map(|r| {
            String::from_utf8((0..n).map(|c| marks[class[r * n + c] as usize]).collect()).unwrap()
        })
        .collect()
}

/// The same picture for any partition, so the cubic can be printed beside
/// `diag3` and the difference read rather than described. `diag3`'s picture
/// is stripes; that is what a seam looks like.
pub fn picture_of(n: usize, assign: impl Fn(usize, usize, usize, usize) -> u8) -> Vec<String> {
    let class: Vec<u8> = (0..n * n).map(|j| assign(j / n, j % n, j, n)).collect();
    picture(&class, n)
}

impl Cubic {
    pub fn separation(&self) -> f64 {
        fold::separation(&self.sizes)
    }
    /// How far the largest class sits from a third of the square, in points.
    /// The filed prediction is that no class comes within 5% of a third.
    pub fn imbalance(&self) -> f64 {
        let third = (self.n * self.n) as f64 / 3.0;
        self.sizes
            .iter()
            .map(|&m| (m as f64 - third).abs() / third)
            .fold(0.0f64, f64::max)
    }
    pub fn closest_to_a_third(&self) -> f64 {
        let third = (self.n * self.n) as f64 / 3.0;
        self.sizes
            .iter()
            .map(|&m| (m as f64 - third).abs() / third)
            .fold(f64::INFINITY, f64::min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimum::{floor_of, worst_all};
    use crate::seam;

    /// The partition is TOTAL and its sizes add back to the square, which is
    /// the only thing a class assignment must do to be usable at all.
    #[test]
    fn the_partition_is_total() {
        for n in [8usize, 16, 32] {
            let cu = partition(n);
            assert_eq!(cu.class.len(), n * n);
            assert_eq!(cu.sizes.iter().sum::<usize>(), n * n);
            assert!(cu.class.iter().all(|&k| k < 3));
            assert_eq!(cu.fold_band.iter().sum::<usize>(), n);
        }
    }

    /// It is deterministic. A partition that moved between runs could not be
    /// pinned, printed, or compared with anything.
    #[test]
    fn the_partition_is_deterministic() {
        let a = partition(16);
        let b = partition(16);
        assert_eq!(a.class, b.class);
        assert_eq!(a.sizes, b.sizes);
        assert_eq!(a.unsettled, b.unsettled);
    }

    /// N3, the honest half, asserted rather than hoped for: the cubic arm is
    /// WORSE than `diag3` on a row burst, because lobes have interiors. If
    /// this test ever fails the round has found something and must say so.
    #[test]
    fn the_cubic_arm_loses_the_row_burst_to_diag3() {
        let n = 32usize;
        let l = 12usize;
        let cu = partition(n);
        let d3: Vec<u8> = (0..n * n).map(|j| seam::a_diag3(j / n, j % n, j, n)).collect();
        let wc = worst_all(&cu.class, n, l).per[0].unwrap();
        let wd = worst_all(&d3, n, l).per[0].unwrap();
        assert_eq!(wd, floor_of(l), "diag3 should be at the floor on a row burst");
        assert!(wc > wd, "the cubic arm managed {wc} against diag3's {wd}");
    }

    /// N3, the other half: the one channel it wins. `diag3` puts the whole
    /// main anti-diagonal in one class because the anti-diagonal is its level
    /// set; a fractal boundary crosses that band and splits it.
    #[test]
    fn the_cubic_arm_splits_the_anti_diagonal_that_diag3_concentrates() {
        for n in [16usize, 32] {
            let cu = partition(n);
            let mut d3 = [0usize; 3];
            for r in 0..n {
                d3[seam::a_diag3(r, n - 1 - r, r * n + (n - 1 - r), n) as usize] += 1;
            }
            assert_eq!(*d3.iter().max().unwrap(), n, "diag3 concentrates the band at n={n}");
            assert!(
                *cu.fold_band.iter().max().unwrap() < n,
                "the cubic arm concentrated it too at n={n}: {:?}",
                cu.fold_band
            );
        }
    }

    /// Every band's first cell lands in class 0, which is the stated
    /// mechanism for the filed imbalance: `2n-1` cells are forced there
    /// before the geometry gets a say.
    #[test]
    fn the_bands_first_cells_are_all_class_zero() {
        for n in [8usize, 16, 32] {
            let cu = partition(n);
            for d in 0..2 * n - 1 {
                let r = (n - 1).min(d);
                let c = d - r;
                assert_eq!(cu.class[r * n + c], 0, "band {d} at n={n}");
            }
        }
    }

    /// The picture is the right shape and uses all three marks, or it is not
    /// a picture of three basins.
    #[test]
    fn the_picture_is_n_by_n_and_shows_three_classes() {
        let n = 32usize;
        let cu = partition(n);
        let p = picture(&cu.class, n);
        assert_eq!(p.len(), n);
        assert!(p.iter().all(|row| row.len() == n));
        for mark in ['.', 'o', '#'] {
            assert!(p.iter().any(|row| row.contains(mark)), "no {mark} in the picture");
        }
    }
}
