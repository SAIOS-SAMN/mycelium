// LAW-COUPLING v1 (proposal 78.175, F2) with LAW-PASSAGE v3 (78.172):
// the cohesion, the held share `c_k = A/(A + Q) = 1 − ν(t) = ν(1/t)`, the
// viscosity (ruled 2026-11-06), beside the open share `ν(t)`, the fluidity,
// read per turning plane and never summed (MU-ANASTOME §9.4; issue #149).

use super::bodies::{q, turning};
use crate::algebra::{base_axis, Clifford};
use crate::coupling::*;
use crate::passage::{Frame, Poles};
use crate::Q;
use num_traits::One;

/// Ratios across the line: the closed pole, below and above the throat,
/// the throat, and far either side.
fn ratios() -> Vec<Q> {
    vec![q(0, 1), q(1, 3), q(1, 2), q(1, 1), q(2, 1), q(3, 1), q(7, 5), q(1, 1000), q(1000, 1)]
}

/// Frame plane 1's poles, `e₊₁` and `e₋₁` (base axes 1 and 2), under that
/// plane's own signature.
fn poles() -> Poles {
    let negatives = base_axis(2).expect("plane 1's negative unit");
    let unit = |g| Clifford::blade(negatives, base_axis(g).expect("a unit the algebra holds"), Q::one()).expect("a unit");
    Poles::of(&unit(1), &unit(2)).expect("two units of opposite sign")
}

/// THE HELD SHARE IS THE COMPLEMENT AND THE INVERSE: `c(t) = 1 − ν(t)` on
/// every ratio, and `c(t) = ν(1/t)` wherever `1/t` stands; `c(0) = 1`, the
/// closed pole; `c(⅓) = ¾`, `c(3) = ¼`. A negative ratio is refused
/// `NotRatio`, as the open share is.
#[test]
fn held_share_is_open_share_of_inverse() {
    assert_eq!(held_share(&q(0, 1)), Ok(q(1, 1)));
    assert_eq!(held_share(&q(1, 3)), Ok(q(3, 4)));
    assert_eq!(held_share(&q(3, 1)), Ok(q(1, 4)));
    for t in ratios() {
        let (c, nu) = (held_share(&t).expect("a share"), open_share(&t).expect("a share"));
        assert_eq!(&c + &nu, Q::one(), "c(t) + ν(t) = 1 at t = {t}");
        if t != q(0, 1) {
            assert_eq!(Ok(c), open_share(&(Q::one() / &t)), "c(t) = ν(1/t) at t = {t}");
        }
    }
    assert_eq!(held_share(&q(-1, 2)), Err(CouplingRefusal::NotRatio(Box::new(q(-1, 2)))));
}

/// AT THE THROAT BOTH SHARES ARE ½: `t = 1` (`A = Q`) reads
/// `c = ν = ½` from the ratio, from a parcel's frames at three gauges, and
/// from the passage's face `Frame::share` at the throat; no other ratio
/// tried reads them equal.
#[test]
fn cohesion_and_fluidity_half_at_the_throat() {
    let half = q(1, 2);
    assert_eq!((held_share(&q(1, 1)), open_share(&q(1, 1))), (Ok(half.clone()), Ok(half.clone())));
    for g in [q(1, 2), q(1, 1), q(5, 3)] {
        let p = Parcel::of(g.clone(), g.clone()).expect("frames");
        assert_eq!((p.held_share(), p.open_share()), (Ok(half.clone()), Ok(half.clone())));
        assert_eq!(Frame { hold: g.clone(), reach: g }.share(), Ok(half.clone()));
    }
    for t in ratios().into_iter().filter(|t| *t != q(1, 1)) {
        assert_ne!(held_share(&t), open_share(&t), "only the throat reads the two equal, t = {t}");
    }
}

/// THE PASSAGE EXCHANGES THE COHESION AND THE FLUIDITY (78.172; the ruling
/// of 2026-11-06): a frame state `H·n_Z + R·n_Q` past the pole swap
/// `x ↦ e₊·x·e₊` reads, as its cohesion `Frame::share = H/(H + R)`, the
/// fluidity it held before, and the reverse; the parcel's inversion
/// (`t ↦ 1/t`) does the same on its frames.
#[test]
fn passage_exchanges_cohesion_and_fluidity() {
    let poles = poles();
    for (h, r) in [(q(2, 3), q(1, 3)), (q(3, 1), q(1, 1)), (q(1, 1), q(1, 1)), (q(5, 2), q(0, 1)), (q(1, 7), q(9, 4))] {
        let before = Frame { hold: h.clone(), reach: r.clone() };
        let passed = poles.read(&poles.swap(&poles.state(&before).expect("a state")).expect("a swap")).expect("a frame");
        let parcel = Parcel::of(h, r).expect("frames");
        let (c, nu) = (parcel.held_share().expect("a share"), parcel.open_share().expect("a share"));
        assert_eq!(before.share(), Ok(c.clone()), "the face is the held share");
        assert_eq!(passed.share(), Ok(nu.clone()), "past the swap the cohesion is the fluidity before");
        assert_eq!(Q::one() - passed.share().expect("a share"), c, "and the fluidity the cohesion");
        let inverted = parcel.inverted();
        assert_eq!((inverted.held_share(), inverted.open_share()), (Ok(nu), Ok(c)));
    }
}

/// COHESION IS READ PER PLANE, NEVER SUMMED: a body's cohesion is a keyed
/// vector over its turning planes, one component per plane, each its own
/// held share; a bond on τ₁₂ moves τ₁₂'s component alone. Hydrogen's even
/// split on three planes reads `⅔` on each, never `2` and never one mean.
#[test]
fn cohesion_read_per_plane_never_summed() {
    let planes = [turning(2, 3), turning(3, 1), turning(1, 2)];
    let hydrogen = Parcels::founded(&q(1, 1), &q(1, 1), &planes).expect("a body");
    let cohesion = hydrogen.cohesion().expect("a reading");
    assert_eq!(cohesion.components().len(), planes.len());
    for k in &planes {
        assert_eq!(cohesion.get(k), Some(&q(2, 3)), "plane {}", k.written());
    }
    let bonded = hydrogen.bond(&turning(1, 2), &q(1, 6)).expect("a bond").cohesion().expect("a reading");
    assert_eq!(bonded.get(&turning(1, 2)), Some(&q(5, 6)));
    assert_eq!(bonded.get(&turning(2, 3)), Some(&q(2, 3)));
    assert_eq!(bonded.get(&turning(3, 1)), Some(&q(2, 3)));
    let fluidity = hydrogen.fluidity().expect("a reading");
    for (k, c) in cohesion.components() {
        assert_eq!(fluidity.get(k).map(|nu| nu + c), Some(Q::one()), "per plane, c_k + ν_k = 1");
    }
}
