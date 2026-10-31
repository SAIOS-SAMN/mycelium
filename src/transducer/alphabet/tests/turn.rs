// DG1, 78.45 — THE TURN, read exactly: degrees where the order is finite,
// the point and its ℚ-protractor sector where not, never a decimal.

use crate::transducer::alphabet::*;

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

fn turns(w: &Word) -> [Turn; 2] {
    w.rotor().turns().expect("a letter's rotor is a versor")
}

/// The identity turns 0°, and the letter on the sensor plane turns 90° in
/// its plane — exact, 90° of the full 360°, `360·1/4`.
#[test]
fn degrees_exact_for_finite_turn() {
    let [a, b] = turns(&Word::identity());
    assert_eq!((a.degrees(), b.degrees()), (Some(q(0, 1)), Some(q(0, 1))), "the identity turns 0°");
    let l = LetterId::turn_90((0, 1)).expect("the sensor letter");
    let [a, b] = turns(&Word::from_letters(&[(l, false)]));
    assert_eq!(a.degrees(), Some(q(90, 1)), "the letter turns 90° in its plane");
    assert_eq!(b.degrees(), Some(q(90, 1)), "in both factors: one plane, one turn");
    assert_eq!(a.to_string(), "90°");
    assert_eq!(a.sector(), None, "a finite turn sits on its degrees");
}

/// 78.45(3)'s worked example: the (3,4,5) letter `3 + 4B` has half-angle
/// point (3/5, 4/5), and it turns by the DOUBLE angle, whose point is
/// (−7/25, 24/25); `cos² = 49/625` lies between `cos² 120° = 1/4` and
/// `cos² 90° = 0` with `cos < 0`, so the turn lies strictly between 90° and
/// 120°. No decimal exists in the reading.
#[test]
fn turns_reads_infinite_turn_as_point_and_sector() {
    let l = LetterId::pythagorean_5((0, 1)).expect("the (3,4,5) letter");
    let [a, _] = turns(&Word::from_letters(&[(l, false)]));
    assert_eq!(a, Turn::Point { cos: q(-7, 25), sin_sq: q(576, 625) });
    assert_eq!(a.sin(), Some(q(24, 25)), "the sine is rational here: 576/625 is a square");
    assert_eq!(a.degrees(), None, "no exact degrees exist for it");
    assert_eq!(a.sector(), Some((90, 120)), "strictly between 90° and 120°");
    assert_eq!(a.to_string(), "(-7/25, 24/25) in (90°, 120°)");
    assert!(!a.to_string().contains('.'), "no decimal anywhere in the reading (78.45)");
}

/// The sector is decided on both sides of 90° by the sign of the cosine: the
/// (5,12,13) letter's half-angle point is (5/13, 12/13), so its turn's
/// cosine is 2·25/169 − 1 = −119/169, `cos² = 14161/28561` below 1/2
/// (2·14161 = 28322 < 28561) and above 1/4, with `cos < 0` — between 120°
/// and 135°.
#[test]
fn sector_reads_cosine_sign_and_square() {
    let l = LetterId::pythagorean_13((0, 1)).expect("the (5,12,13) letter");
    let [a, _] = turns(&Word::from_letters(&[(l, false)]));
    let Turn::Point { cos, .. } = &a else { panic!("an infinite-order turn is a point: {a:?}") };
    assert_eq!(cos, &q(-119, 169));
    assert_eq!(a.sector(), Some((120, 135)));
}
