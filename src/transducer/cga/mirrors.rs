// 78.14, MG1 — THE BASE'S MIRRORS: the hyperplanes `x_i = k` as constructors
// through `e_∞`, whose affine group is the grid's own Coxeter reading, and
// each nucleus read RELATIVE to the mirrors it stands between.
//
//! A mirror `x_i = k` is the vector `π = e_i + k e_∞`: a point `X` has
//! `X · π = x_i − k`, so the plane is its zero set, and `π² = 1`. Among
//! the base's mirrors `δ²` is `0` across axes (orthogonal) and `1` along
//! one axis (parallel): the lattice's affine group, `Ã₁` per axis, read by
//! GR1/GR2 as any constructors are.
//!
//! A NUCLEUS AGAINST A MIRROR. For a sphere `s` of centre `c` and extent
//! `s² = ρ²` (A.12), `s · π = c_i − k`, so `δ² = (c_i − k)² / ρ²`, the
//! inversive strain's own carrier (65): `0` where the mirror runs through the
//! centre, below `1` where it cuts the sphere, `1` where tangent, above `1`
//! where the body stands apart from it. Read against the mirrors that
//! BRACKET the centre on each axis — `⌊c_i⌋` and `⌈c_i⌉`, one mirror where
//! the centre stands on it — it says where a body sits in the grid, exactly,
//! with no position taken as a key.

use super::multivector::{one, Mv};
use num_rational::BigRational;
use num_traits::Zero;

type Q = BigRational;

/// The mirror `x_axis = k` (`axis` counted from one), as a constructor.
pub fn base_mirror(axis: usize, k: &Q) -> Mv {
    Mv::blade(super::base_axis(axis), one()).add(&super::e_inf().scale(k))
}

/// A sphere's centre on the base's `n` axes, `c_i = (s · e_i) / (−s · e_∞)`;
/// `None` where the sphere has no finite centre (a plane).
pub fn sphere_centre(s: &Mv, n: u8) -> Option<Vec<Q>> {
    let w = -s.geo(&super::e_inf()).scalar_part();
    if w.is_zero() {
        return None;
    }
    Some((1..=usize::from(n)).map(|i| s.geo(&Mv::blade(super::base_axis(i), one())).scalar_part() / &w).collect())
}

/// ONE NUCLEUS AGAINST ONE BRACKETING MIRROR: the axis, the mirror's offset,
/// and the pair's `δ²` and regime by GR1's one reader.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MirrorReading {
    pub axis: usize,
    pub k: Q,
    pub delta2: Q,
    pub regime: super::PairRegime,
}

/// THE MIRRORS A SPHERE STANDS BETWEEN, and its `δ²` against each (MG1),
/// read as the pair `(s, π)` by `pair_regime` — the one producer of `δ²`.
/// Empty where the sphere has no finite centre.
pub fn mirrors_between(s: &Mv, n: u8) -> Vec<MirrorReading> {
    let Some(c) = sphere_centre(s, n) else { return Vec::new() };
    let mut out = Vec::new();
    for (i, ci) in c.iter().enumerate() {
        let (lo, hi) = (ci.floor(), ci.ceil());
        let ks = match lo == hi {
            true => vec![lo],
            false => vec![lo, hi],
        };
        for k in ks {
            let g = super::gram_form(&[s.clone(), base_mirror(i + 1, &k)]);
            let (delta2, regime) = super::pair_regime(&g, 0, 1);
            out.push(MirrorReading { axis: i + 1, k, delta2, regime });
        }
    }
    out
}
