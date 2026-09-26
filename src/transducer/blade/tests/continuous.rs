// 50.8, A6, 16.4 — Tests for continuous support correspondence and Pythagorean prime rotors.

use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::blade::{continuous::ContinuousSupport, *};
use crate::transducer::element::versor_of_word;
use crate::transducer::sentence::class::{orbit_row, OrbitRow, FREE_ROW};

#[test]
fn continuous_support_invariants_in_closed_form() {
    let bar = super::common::sites(&[(0, 0, 0), (0, 1, 0), (0, 2, 0), (0, 3, 0)]);
    let s = shape_of(&bar);

    // SQ1-b5 — similarity is the dilation APPLIED, not a scalar prefilter:
    // the coset carrying the bar onto its double is non-empty, and each of
    // its versors carries the one tower onto the other at ρ = 2.
    let dilated = super::common::dilate(&s.blade.support, 2);
    let s_dil = shape_of(&dilated);
    let coset = versor_coset(&s, &s_dil);
    assert!(!coset.is_empty(), "D(2) carries the bar onto its double");
    let (sa, sb) = (
        ContinuousSupport::of(&s.blade, Tower::Region).expect("support"),
        ContinuousSupport::of(&s_dil.blade, Tower::Region).expect("support"),
    );
    for (_, v) in &coset {
        assert!(sa.matches(&sb, v), "the member carries the tower");
        assert!(!sb.matches(&sa, v), "and not back: its inverse does");
    }
}

#[test]
fn continuous_support_pythagorean_rotor_holonomy() {
    // Word carrying pythagorean_5 on plane (0, 1)
    let p5 = LetterId::pythagorean_5((0, 1)).expect("pythagorean 5 on (0, 1)");
    let w = Word::from_letters(&[(p5, false)]);

    // SQ1-b4 — THE ROTOR IS APPLIED TO THE HARMONICS, NOT COMPARED BY NORM.
    // A bar along the row and the same bar along the column are one 90°
    // turn apart: the 90°-turn rotor carries the one's harmonics onto
    // the other's. The (3,4,5) rotor does not — and the norm comparison this
    // replaces accepted it (and accepted a body "matching itself" under any
    // turn), because a squared norm cannot see the angle.
    let row_bar = super::common::sites(&[(0, 0, 0), (0, 1, 0), (0, 2, 0)]);
    let col_bar = super::common::sites(&[(0, 0, 0), (1, 0, 0), (2, 0, 0)]);
    let (row, col) = (shape_of(&row_bar), shape_of(&col_bar));
    let turn_90 =
        versor_of_word(&Word::from_letters(&[(LetterId::turn_90((0, 1)).expect("90° turn on (0, 1)"), false)]))
            .expect("the 90° turn's base action");
    let p5_turn = versor_of_word(&w).expect("the (3,4,5) turn's base action");
    assert!(versor_carrying(&row, &col, &turn_90).is_some(), "the 90° turn carries the row bar onto the column bar");
    assert!(versor_carrying(&row, &col, &p5_turn).is_none(), "the (3,4,5) turn does not");
    assert!(
        versor_carrying(&row, &row, &p5_turn).is_none(),
        "and a body does not match itself under a turn it is not symmetric by"
    );

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
