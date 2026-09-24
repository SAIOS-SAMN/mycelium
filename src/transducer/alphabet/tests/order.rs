// 15.3, 9.1 — THE ORDER OF A ROTOR: read from its class invariants, the
// period of its two commuting quaternion factors, never from a spelling
// of its sensor-plane shadow.

use crate::transducer::alphabet::*;
use num_traits::One;

/// A letter reads 1, 4 or infinite (ruled 2026-09-09): the identity, a
/// quarter turn on any plane — the sensor's or not — and a Pythagorean
/// rotor, whose cosine is no root of unity.
#[test]
fn a_letter_reads_one_four_or_infinite() {
    assert_eq!(Word::identity().order(), Order::Finite(1));
    let q01 = LetterId::quarter((0, 1)).expect("the sensor quarter");
    assert_eq!(Word::from_letters(&[(q01, false)]).order(), Order::Finite(4));
    let q23 = LetterId::quarter((2, 3)).expect("a quarter off the sensor plane");
    assert_eq!(
        Word::from_letters(&[(q23, false)]).order(),
        Order::Finite(4),
        "a quarter turn off the sensor plane is a quarter turn, not the sensor's identity"
    );
    let p5 = LetterId::pythagorean_5((0, 1)).expect("a (3,4,5) rotor");
    assert_eq!(Word::from_letters(&[(p5, false)]).order(), Order::Infinite);
    let p13 = LetterId::pythagorean_13((0, 1)).expect("a (5,12,13) rotor");
    assert_eq!(Word::from_letters(&[(p13, false)]).order(), Order::Infinite);
    assert_eq!(
        Word::from_letters(&[(q01, false), (p5, false)]).order(),
        Order::Infinite,
        "a quarter turn beside a Pythagorean rotor on one plane turns forever"
    );
}

/// TWO QUARTER TURNS ON PLANES SHARING AN AXIS CLOSE IN THREE — a third of
/// a turn about the diagonal — where the plane form, reading the identity
/// on the sensor plane, answered four (the residue found under L2). Two
/// quarter turns on complementary planes commute and close in four; a
/// quarter turn beside a half turn on its complement closes in four; the
/// sheet is a half turn; the pseudoscalar is the central inversion, a
/// half turn; the zero rotor has no order; and an even element that is no
/// versor has no scalar power at all.
#[test]
fn two_quarter_turns_on_planes_sharing_an_axis_close_in_three() {
    let q01 = LetterId::quarter((0, 1)).expect("quarter on (0,1)");
    let q02 = LetterId::quarter((0, 2)).expect("quarter on (0,2)");
    let q23 = LetterId::quarter((2, 3)).expect("quarter on (2,3)");
    let diagonal = Word::from_letters(&[(q01, false), (q02, false)]);
    assert_eq!(diagonal.order(), Order::Finite(3), "two quarter turns sharing an axis: a third of a turn");
    let r = diagonal.rotor();
    let r2 = r.geo(&r);
    let r3 = r2.geo(&r);
    assert!(!r.projective_eq(&Rotor::identity()), "the first power is not a scalar");
    assert!(!r2.projective_eq(&Rotor::identity()), "nor the second");
    assert!(r3.projective_eq(&Rotor::identity()), "the third is: the order is exact, not a table");
    let isoclinic = Word::from_letters(&[(q01, false), (q23, false)]);
    assert_eq!(isoclinic.order(), Order::Finite(4), "commuting quarter turns on complementary planes close in four");
    let mixed = Word::from_letters(&[(q01, false), (q23, false), (q23, false)]);
    assert_eq!(mixed.order(), Order::Finite(4), "a quarter turn beside a half turn on its complement closes in four");
    let sheet = PlaneForm { orientation: true, rot: [0; CAYLEY_ALPHABET_LEN] }.to_word();
    assert_eq!(sheet.order(), Order::Finite(2), "the sheet is a half turn through a third axis");
    assert_eq!(
        Rotor::blade(0b1111, Q::one()).order(),
        Some(Order::Finite(2)),
        "the pseudoscalar is the central inversion, a half turn"
    );
    assert_eq!(Rotor::zero().order(), None, "the zero rotor is no versor and has no order");
    let unequal = Rotor::identity()
        .add(&Rotor::blade(0b0011, Q::one()))
        .add(&Rotor::blade(0b1100, Q::one()));
    assert!(!unequal.is_versor(), "1 + e01 + e23 is no versor: the action leaves the pseudoscalar's line");
    assert!(unequal.weight().is_none(), "and a weight is read only for a versor");
    assert_eq!(unequal.order(), None, "so it has no class and no order");
    assert_eq!(
        Word::formed(unequal).order(),
        Order::Infinite,
        "and as a word it turns forever: no power of it is a scalar"
    );
}
