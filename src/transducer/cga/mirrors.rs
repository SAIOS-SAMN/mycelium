// 78.14, MG1 — THE BASE'S MIRRORS: the hyperplanes `x_i = k` as constructors
// through `e_∞`, whose affine group is the grid's own Coxeter reading, and
// each nucleus read RELATIVE to the mirrors it stands between.
//
//! A mirror `x_i = k` is the vector `π = e_i + k e_∞`: a point `X` has
//! `X · π = x_i − k`, so the plane is its zero set, and `π² = 1`. Among
//! the base's mirrors the pairs are orthogonal across axes and parallel
//! along one: the lattice's affine group, `Ã₁` per axis, read by GR1/GR2 as
//! any constructors are.
//!
//! A NUCLEUS AGAINST A MIRROR (78.59: at unit weight, never a squared
//! ratio). The sphere `s` of centre `c`, normalised to unit weight
//! `ŝ = s / (−s · e_∞)`, meets the mirror in its OFFSET `ŝ · π = c_i − k`,
//! first-degree and exact: `0` where the mirror runs through the centre.
//! Whether it cuts, touches or stands apart is the pair's regime (GR1),
//! read in the algebra. Read against the mirrors that
//! BRACKET the centre on each axis — `⌊c_i⌋` and `⌈c_i⌉`, one mirror where
//! the centre stands on it — it says where a body sits in the grid, exactly,
//! with no position taken as a key.

use super::multivector::{one, Mv};
use num_rational::BigRational;
use num_traits::Zero;

type Q = BigRational;

/// The mirror `x_axis = k` (`axis` counted from one), as a constructor;
/// `None` for an axis the algebra does not hold.
pub fn base_mirror(axis: usize, k: &Q) -> Option<Mv> {
    Some(Mv::blade(super::base_axis(axis)?, one()).add(&super::e_inf().scale(k)))
}

/// A sphere's centre on the base's `n` axes, `c_i = (s · e_i) / (−s · e_∞)`;
/// `None` where the sphere has no finite centre (a plane).
pub fn sphere_centre(s: &Mv, n: u8) -> Option<Vec<Q>> {
    let w = -s.geo(&super::e_inf()).scalar_part();
    if w.is_zero() {
        return None;
    }
    Some(super::axes(n).map(|a| s.geo(&Mv::blade(a, one())).scalar_part() / &w).collect())
}

/// ONE NUCLEUS AGAINST ONE BRACKETING MIRROR: the axis, the mirror's offset,
/// the unit-weight sphere's offset from it `c_i − k`, and the pair's regime
/// by GR1's one reader.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MirrorReading {
    pub axis: usize,
    pub k: Q,
    pub offset: Q,
    pub regime: super::PairRegime,
}

/// THE MIRRORS A SPHERE STANDS BETWEEN, its offset from each and the
/// pair's regime (MG1). Empty where the sphere has no finite centre.
pub fn mirrors_between(s: &Mv, n: u8) -> Vec<MirrorReading> {
    let Some(c) = sphere_centre(s, n) else { return Vec::new() };
    let mut out = Vec::new();
    for (i, ci) in c.iter().enumerate() {
        let (lo, hi) = (ci.floor(), ci.ceil());
        let ks = match lo == hi {
            true => vec![lo],
            false => vec![lo, hi],
        };
        // The sphere at unit weight: its offset from the mirror is
        // first degree, `ŝ · π = c_i − k`, never a squared ratio.
        let w = -s.geo(&super::e_inf()).scalar_part();
        if w.is_zero() {
            return Vec::new();
        }
        for k in ks {
            // The centre has one coordinate per axis the algebra holds.
            let Some(mirror) = base_mirror(i + 1, &k) else { continue };
            let offset = s.geo(&mirror).scalar_part() / &w;
            let regime = super::pair_regime(&[s.clone(), mirror], 0, 1);
            out.push(MirrorReading { axis: i + 1, k, offset, regime });
        }
    }
    out
}
