// 50.8, A6, 16.4 — Tests for continuous support correspondence and Pythagorean prime rotors.

use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::blade::*;
use crate::transducer::element::versor_of_word;
use crate::transducer::sentence::class::{orbit_row, OrbitRow, FREE_ROW};

/// On a continuous support, a Pythagorean rotor carries no body the 90° turn
/// does not, and its holonomy word reads a free orbit row.
#[test]
fn versor_carrying_refuses_pythagorean_rotor_reads_free_row() {
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
    let mut b = super::common::budget();
    assert!(versor_carrying(&row, &col, &turn_90, &mut b).is_ok(), "the 90° turn carries the row bar onto the column bar");
    assert!(
        matches!(versor_carrying(&row, &col, &p5_turn, &mut b), Err(NoPose::Uncarried(_))),
        "the (3,4,5) turn does not, and the covariance names why"
    );
    assert!(
        matches!(versor_carrying(&row, &row, &p5_turn, &mut b), Err(NoPose::Uncarried(_))),
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
