// 16.16 step 6, 37.7 — THE LOOP'S DEMAND: a held loop of infinite order
// the basis does not discharge is spoken on the form as a standing demand
// carrying its holonomy. Beside `pivot_loop.rs` under the §1.1 bound; the
// `Hold` crystal is read across the split.

use super::common::*;
use super::pivot_loop::Hold;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{factor_rotor, Rotor, Word, Q};
use crate::transducer::sentence::*;
use crate::transducer::stalk;

/// A held loop of infinite order the basis does not discharge is spoken as
/// a standing demand on each chart the loop stands on, carrying the loop's
/// holonomy rotor as its witness — here `3 + e₀₁ + e₀₂`, weight 11, held by
/// no coordinate plane. The rotor stands on the chord's EDGE (50.3): a face
/// word with no sensor action places no member pair, so the loop is judged
/// by its edge composite.
#[test]
fn a_free_part_loop_is_spoken_as_a_standing_demand_carrying_its_holonomy() {
    let q = |n: i64| Q::from_integer(num_bigint::BigInt::from(n));
    let r = Rotor::scalar(q(3))
        .add(&Rotor::blade(0b0011, q(1)))
        .add(&Rotor::blade(0b0101, q(1)));
    let w = Word::formed(r.clone());
    let mut f = edge_chord_form(std::slice::from_ref(&w));
    let snaps = f.pivot_loop(
        [0x33; 32],
        &Keyed::new(),
        &Hold(Vec::new()),
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    let demands: Vec<(Rotor, stalk::Invariant)> = snaps
        .iter()
        .filter_map(|s| match s {
            Snap::Demand { rotor, chart, .. } => Some((rotor.as_ref().clone(), chart.clone())),
            _ => None,
        })
        .collect();
    let [(rotor, chart)] = demands.as_slice() else {
        panic!("one standing demand, on the one chart the loop stands on: {snaps:?}")
    };
    // The loop runs the chord backward, so the holonomy it carries is
    // `r⁻¹`: the same class as `r` (15.3 reads the ray, and inversion
    // fixes both invariants), and the class is what the loop is judged by.
    assert_eq!(
        rotor.class_key(),
        r.class_key(),
        "the demand carries the loop's holonomy as its witness, in the class the loop is judged by"
    );
    assert!(
        rotor.projective_eq(&r) || rotor.projective_eq(&r.reverse()),
        "and it is that holonomy, read along the loop"
    );
    assert_eq!(*chart, stalk::Invariant::default(), "on the chart the loop's vertices stand at");
    assert!(
        matches!(factor_rotor(rotor), Some(Err(Demand::Standing { .. }))),
        "and no coordinate plane holds it"
    );
}
