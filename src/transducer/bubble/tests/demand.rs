// 16.16 step 6, 50.5 — THE LOOP'S DEMAND REACHES THE HELD FORM: both of
// the drive's pivot paths write a held loop of infinite order onto the
// bubble's form as the demand the peel types its holonomy as.

use crate::transducer::alphabet::{Rotor, Word, Q};
use crate::transducer::bubble::*;
use crate::transducer::sentence::tests::common::edge_chord_form;
use crate::transducer::sentence::Demand;
use crate::transducer::stalk::{Invariant, Stalks};

/// `3 + e₀₁ + e₀₂`: weight 11, held by no coordinate plane (37.7).
fn eleven() -> Rotor {
    let q = |n: i64| Q::from_integer(num_bigint::BigInt::from(n));
    Rotor::scalar(q(3)).add(&Rotor::blade(0b0011, q(1))).add(&Rotor::blade(0b0101, q(1)))
}

/// The one standing demand a held form carries, with its chart.
fn the_standing_demand(f: &crate::transducer::sentence::Form) -> (Rotor, Invariant) {
    let mut found: Vec<(Rotor, Invariant)> = f
        .demand_vertices()
        .filter_map(|(_, kind, chart, _)| match kind {
            Demand::Standing { rotor, .. } => Some((rotor.as_ref().clone(), chart.clone())),
            _ => None,
        })
        .collect();
    let [one] = found.as_mut_slice() else {
        panic!("one standing demand on the held form, got {}", found.len())
    };
    one.clone()
}

/// THE EXPAND PATH: a bubble holding a form whose one loop carries the
/// weight-11 rotor on its edge, expanded over an empty atlas against a
/// crystal holding nothing, leaves the form it takes back with a `Standing`
/// vertex on the loop's chart, in the holonomy's class.
#[test]
fn an_expanded_bubble_stands_its_loops_demand_on_the_form_it_takes() {
    let form = edge_chord_form(&[Word::formed(eleven())]);
    let mut b = Bubble { form: Some(form), ..Default::default() };
    let atlas = Stalks::default();
    let section_at = |_: &[u8]| None;
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 8);
    let _ = b.expand_with(&atlas, &section_at, 0, &crate::transducer::family::NoCrystal, &mut budget);
    let f = b.form.as_ref().expect("the bubble keeps its form");
    let (rotor, chart) = the_standing_demand(f);
    assert_eq!(rotor.class_key(), eleven().class_key(), "the witness is the loop's holonomy, by class");
    assert_eq!(chart, Invariant::default(), "on the chart the loop's vertices stand at");
    assert!(!f.is_closed(), "a form carrying a demand is not closed");
}

/// THE PIVOT-ON PATH: the same form held by a contemplation and pivoted on
/// by a word at its chart writes the same vertex, so the two paths are one
/// reading (law 12).
#[test]
fn a_pivoted_on_bubble_stands_its_loops_demand_on_the_form_it_takes() {
    use crate::transducer::element::{canonical_chain, from_canonical};
    let form = edge_chord_form(&[Word::formed(eleven())]);
    let rem = vec![(0i64, 0i64, 1u8, 2u8), (0, 1, 1, 2)];
    let mut c = Contemplation::new();
    c.set_now(10);
    c.hold_form(form.clone(), &[], std::slice::from_ref(&rem));
    let word = from_canonical(&canonical_chain(&rem).expect("one entry per position is A5's chain"));
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 8);
    let _ = c.pivot_on(&word, &crate::transducer::family::NoCrystal, &mut budget);
    let held: Vec<&Bubble> = c.bubbles.scoped().map(|(_, b)| b).collect();
    let [b] = held.as_slice() else { panic!("one bubble holds the form, got {}", held.len()) };
    let f = b.form.as_ref().expect("the bubble keeps its form");
    let (rotor, chart) = the_standing_demand(f);
    assert_eq!(rotor.class_key(), eleven().class_key(), "the witness is the loop's holonomy, by class");
    assert_eq!(chart, Invariant::default(), "on the chart the loop's vertices stand at");
}
