// 78.2, EN1 — CONTAINMENT IS THE SIGN OF THE INVERSIVE PRODUCT: the
// enclosure a cell's bodies share is the JOIN of their centres as null
// points — the round through them — and which body it wraps, and which
// enclosure wraps which, is read off the pair's inversive product through
// GR1's one pair reader, never off a Euclidean inside or outside.
//
//! THE CENTRE of a nucleus is where its inversion sends infinity, `s e_∞ s`
//! — read by the sandwich, never by a radius.
//!
//! THE ENCLOSURE of `k + 1` bodies is the round `R = X₀ ∧ … ∧ X_k` their
//! centres join (a point pair, a circle, a sphere as `k` rises). Its
//! CARRIER is the sphere the round is a great round of, centred in the
//! round's own span: the contraction of `R` by its flat `F = R ∧ e_∞`,
//! `σ = ⟨R F⁻¹⟩₁`. At `k = n` the round is itself a sphere and the carrier
//! is its dual. Centres on one flat (three on a line) span no round: the
//! enclosure is FLAT, by name — there is nothing to be inside.
//!
//! CONTAINMENT of a sphere `s` against an enclosure `e`, both written with
//! the weight infinity reads (`s · e_∞ = −1`), is read in two steps:
//!
//! 1. GR1's `pair_regime` on the pair's Gram form. Tangent, orthogonal or
//!    cutting at any angle (`δ² ≤ 1`): the body stands ON the enclosure —
//!    by name, neither inside nor out. A body of the cell stands on its own
//!    enclosure by construction: its centre is on the round.
//! 2. Apart (`δ² > 1`): the sign of the inversive product `s · e`. Negative:
//!    APART, each outside the other. Positive: NESTED — and which of the two
//!    wraps is the side of the pair's radical plane `π = s − e` (the pencil's
//!    member of weight zero) the enclosure stands on, the sign of `e · π`:
//!    negative, the body is INSIDE; positive, the body WRAPS the enclosure.
//!
//! A point (a volumeless nucleus, `s² = 0`) is read by the sign of `X · e`
//! alone. An imaginary sphere (`s² < 0`) is UNREAD: its product carries a
//! sign it does not mean.

use super::multivector::{half, one, Mv};
use super::{e_inf, gram_form, pair_regime, PairRegime};
use num_rational::BigRational;
use num_traits::{Signed, Zero};

type Q = BigRational;

/// WHERE A SPHERE STANDS AGAINST AN ENCLOSURE, by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Containment {
    /// Nested, the sphere within the enclosure.
    Inside,
    /// Touching or cutting it: on it, neither inside nor out.
    On,
    /// Each outside the other.
    Apart,
    /// Nested, the enclosure within the sphere.
    Wraps,
    /// An imaginary sphere or enclosure: no containment reads.
    Unread,
}

/// A sphere written with the weight infinity reads, `s · e_∞ = −1`; `None`
/// for a flat (weight zero), which has no inside.
fn weighted(s: &Mv) -> Option<Mv> {
    let w = -s.geo(&e_inf()).scalar_part();
    if w.is_zero() {
        return None;
    }
    Some(s.scale(&(one() / w)))
}

/// THE CENTRE OF A NUCLEUS as a null point: where its inversion sends
/// infinity, `−½ s e_∞ s` at weight one. `None` for a flat.
pub fn centre_point(s: &Mv) -> Option<Mv> {
    let s = weighted(s)?;
    let x = s.geo(&e_inf()).geo(&s).grade(1).scale(&-half());
    weighted(&x)
}

/// THE ENCLOSURE OF A CELL'S BODIES: the carrier of the round their
/// centres join, at weight one. `None` where the centres span no round —
/// they coincide, or stand on one flat.
pub fn enclosure(nuclei: &[Mv]) -> Option<Mv> {
    let centres = nuclei.iter().map(centre_point).collect::<Option<Vec<Mv>>>()?;
    let round = super::wedge(&centres);
    if round.is_zero() {
        return None;
    }
    let k = centres.len() as u32;
    let flat = round.wedge_vector(k, &e_inf());
    let carrier = round.geo(&flat.versor_inverse()?).grade(1);
    weighted(&carrier)
}

/// CONTAINMENT OF `s` AGAINST THE ENCLOSURE `e` (EN1).
pub fn containment(s: &Mv, e: &Mv) -> Containment {
    let (Some(s), Some(e)) = (weighted(s), weighted(e)) else { return Containment::Unread };
    let g = gram_form(&[s.clone(), e.clone()]);
    if g[0][0].is_negative() || g[1][1].is_negative() {
        return Containment::Unread;
    }
    let sign = |q: &Q| match () {
        _ if q.is_positive() => Containment::Inside,
        _ if q.is_negative() => Containment::Apart,
        _ => Containment::On,
    };
    match pair_regime(&g, 0, 1).1 {
        // A point against the enclosure, or the enclosure a point: the sign
        // of the one product there is.
        PairRegime::Null => match (g[0][0].is_zero(), sign(&g[0][1])) {
            (true, c) => c,
            (false, Containment::Inside) => Containment::Wraps,
            (false, c) => c,
        },
        PairRegime::Finite(_) | PairRegime::Parabolic | PairRegime::Rosette => Containment::On,
        PairRegime::Boost => match sign(&g[0][1]) {
            Containment::Inside => {
                let radical = s.sub(&e);
                match e.geo(&radical).scalar_part().is_negative() {
                    true => Containment::Inside,
                    false => Containment::Wraps,
                }
            }
            c => c,
        },
    }
}

/// NESTING IS THE PARSE (50.10): for each enclosure, the one that wraps it
/// most closely — the wrapper every other wrapper of it also wraps — or
/// `None` at a root. Read by containment alone.
pub fn nesting(enclosures: &[Mv]) -> Vec<Option<usize>> {
    let inside = |a: usize, b: usize| a != b && containment(&enclosures[a], &enclosures[b]) == Containment::Inside;
    (0..enclosures.len())
        .map(|i| {
            let wrappers: Vec<usize> = (0..enclosures.len()).filter(|&j| inside(i, j)).collect();
            wrappers.iter().copied().find(|&j| wrappers.iter().all(|&w| w == j || inside(j, w)))
        })
        .collect()
}
