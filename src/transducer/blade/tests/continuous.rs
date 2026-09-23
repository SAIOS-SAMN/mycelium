// 50.8, A6, 16.4 — Tests for continuous support correspondence and Pythagorean prime rotors.

use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::blade::{continuous::ContinuousSupport, *};
use crate::transducer::sentence::class::{loop_row, LoopRow, FREE_ROW};
use num_rational::BigRational;

#[test]
fn continuous_support_invariants_in_closed_form() {
    let bar = super::common::sites(&[(0, 0, 0), (0, 1, 0), (0, 2, 0), (0, 3, 0)]);
    let s = shape_of(&bar);
    let cs = ContinuousSupport::of(&s).expect("continuous support");

    // Shell = 4 in Q
    assert_eq!(cs.shell, BigRational::from_integer(4.into()));

    // Casimir invariant |H2|^2 must be non-zero for anisotropic bar
    assert!(cs.h2_casimir > BigRational::from_integer(0.into()));

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

    let bar = super::common::sites(&[(0, 0, 0), (0, 1, 0), (0, 2, 0)]);
    let s = shape_of(&bar);
    let cs = ContinuousSupport::of(&s).expect("support");

    // Rotated continuous support preserves Casimir invariants and volume
    assert!(cs.matches_rotor(&cs, &r));

    // The holonomy word evaluates to LoopRow::Free with non-zero exponent
    let row = loop_row(&w).expect("loop row");
    match row {
        LoopRow::Free(vec) => {
            assert_ne!(vec, [0i64; FREE_ROW]);
            assert_eq!(vec[0], 1, "m5 exponent on plane (0, 1)");
        }
        other => panic!("expected LoopRow::Free, got: {other:?}"),
    }
}
