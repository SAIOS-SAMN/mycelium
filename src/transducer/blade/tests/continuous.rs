// 50.8, A6, 16.4 — Tests for continuous support correspondence and Pythagorean prime rotors.

use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::blade::{continuous::ContinuousSupport, *};
use crate::transducer::sentence::class::{orbit_row, OrbitRow, FREE_ROW};
use num_rational::BigRational;

#[test]
fn continuous_support_invariants_in_closed_form() {
    let bar = super::common::sites(&[(0, 0, 0), (0, 1, 0), (0, 2, 0), (0, 3, 0)]);
    let s = shape_of(&bar);
    let cs = ContinuousSupport::of(&s).expect("continuous support");

    // Shell = 4 in Q
    assert_eq!(cs.shell, BigRational::from_integer(4.into()));

    // An anisotropic bar carries a second harmonic: H2 is not the zero vector.
    assert!(cs.h2.iter().any(|x| *x != BigRational::from_integer(0.into())));

    // Exact similarity under dilation 2: shell scales by 4, H2 by 256
    let dilated = super::common::dilate(&s.blade.support, 2);
    let s_dil = shape_of(&dilated);
    let cs_dil = ContinuousSupport::of(&s_dil).expect("dilated support");

    let rho = BigRational::from_integer(2.into());
    assert!(cs.is_similar_to(&cs_dil, &rho));
}

#[test]
fn continuous_support_pythagorean_rotor_holonomy() {
    // Word carrying pythagorean_5 on plane (0, 1)
    let p5 = LetterId::pythagorean_5((0, 1)).expect("pythagorean 5 on (0, 1)");
    let w = Word::from_letters(&[(p5, false)]);
    let r = w.rotor();

    // SQ1-b4 — THE ROTOR IS APPLIED TO THE HARMONICS, NOT COMPARED BY NORM.
    // A bar along the row and the same bar along the column are one quarter
    // turn apart: the quarter-turn rotor carries the one's harmonics onto
    // the other's. The (3,4,5) rotor does not — and the norm comparison this
    // replaces accepted it (and accepted a body "matching itself" under any
    // turn), because a squared norm cannot see the angle.
    let row_bar = super::common::sites(&[(0, 0, 0), (0, 1, 0), (0, 2, 0)]);
    let col_bar = super::common::sites(&[(0, 0, 0), (1, 0, 0), (2, 0, 0)]);
    let cs = ContinuousSupport::of(&shape_of(&row_bar)).expect("support");
    let turned = ContinuousSupport::of(&shape_of(&col_bar)).expect("support");
    let quarter = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter on (0, 1)"), false)]).rotor();
    assert!(cs.matches_rotor(&turned, &quarter), "the quarter turn carries the row bar onto the column bar");
    assert!(!cs.matches_rotor(&turned, &r), "the (3,4,5) turn does not");
    assert!(!cs.matches_rotor(&cs, &r), "and a body does not match itself under a turn it is not symmetric by");

    // The holonomy word evaluates to OrbitRow::Free with non-zero exponent
    let row = orbit_row(&w).expect("orbit row");
    match row {
        OrbitRow::Free(vec) => {
            assert_ne!(vec, [0i64; FREE_ROW]);
            assert_eq!(vec[0], 1, "m5 exponent on plane (0, 1)");
        }
        other => panic!("expected OrbitRow::Free, got: {other:?}"),
    }
}
