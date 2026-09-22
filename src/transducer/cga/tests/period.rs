// 54.7 — the Niven trap: a period is crystallographic or it is infinite.

use super::common::q;
use crate::transducer::cga::*;

/// 54.7 — THE NIVEN TRAP: an open orbit has TWO causes, and reading
/// only the first would report a dense winding as a closed circuit.
#[test]
fn a_period_is_crystallographic_or_it_is_infinite() {
    // A pure transvection has no compact part: open, and no pitch.
    let boost = transvection(&q(2, 1), &q(1, 1), TRANSVECTION_PLANES[0]).expect("a boost");
    assert_eq!(compact_period(&boost), Spectrum::Open, "a dislocation slips and never returns");

    // A quarter turn closes at a crystallographic order, which is ℚ's
    // doing and not a convention.
    let quarter = rotator(&q(0, 1), &q(1, 1));
    let Spectrum::Closes(p) = compact_period(&quarter) else { panic!("a quarter turn closes") };
    let ell = p.plane_1.expect("the turning plane has an order").get();
    assert_eq!(ell, 2, "s = 0 is the half-angle π/2, so the turn is by π");
    assert!(matches!(ell, 1 | 2 | 3 | 4 | 6), "Niven allows only these, got {ell}");

    // THE TRAP ITSELF. `cos θ = (s²−b²)/(s²+b²) = −7/25` is RATIONAL and
    // is not one of Niven's five, so `θ/π` is irrational: the rotor is
    // purely elliptic — a non-zero compact part — and its orbit is dense
    // and closes never. `B_ell ≠ 0` does not imply a finite period.
    let pyth = rotator(&q(3, 1), &q(4, 1));
    let (e, h) = bivector_split(&pyth).expect("a simple rotation splits");
    assert!(!e.is_zero(), "the (3,4,5) rotor HAS a compact part");
    assert!(h.is_zero(), "and no open part at all");
    assert_eq!(
        compact_period(&pyth),
        Spectrum::Open,
        "yet cos θ = −7/25 winds densely: OPEN, not undecomposed — the \
         split exists and the orbit still never closes"
    );
}
