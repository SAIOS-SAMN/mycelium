// 16.16 step 6, 50.5 — THE ORBIT'S DEMAND REACHES THE HELD FORM: both of
// the drive's pivot paths write a held orbit of infinite order onto the
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

/// THE STANDING DEMANDS A HELD FORM CARRIES, with the chart each stands on.
///
/// It asserted exactly ONE, and that was a reading of a broken fixture
/// (54.5). 50.5 stands one demand per `(kind, chart)`, and `edge_chord_form`
/// built its two chord ends with identical coordinates — one rotor, one
/// chart, one carriage, one gauge — so §1.5 made them ONE vertex and the
/// orbit stood on ONE chart. Two ends of a chord are two SITES, which is the
/// note `chord_form` has carried since 54.2, and with the ends separated the
/// orbit runs through two charts and stands a demand at each. That is the
/// doctrine, not a regression: what the claim is about is that every demand
/// carries the orbit's holonomy, on a chart the orbit actually stands on.
fn standing_demands(f: &crate::transducer::sentence::Form) -> Vec<(Rotor, Invariant)> {
    let found: Vec<(Rotor, Invariant)> = f
        .demand_vertices()
        .filter_map(|(_, kind, chart, _)| match kind {
            Demand::Standing { rotor, .. } => Some((rotor.as_ref().clone(), chart.clone())),
            _ => None,
        })
        .collect();
    assert!(!found.is_empty(), "the held form carries a standing demand");
    found
}

/// Every demand the orbit stood, checked against the holonomy and the charts
/// its own vertices stand at.
fn each_demand_carries(
    f: &crate::transducer::sentence::Form,
    rotor: &Rotor,
) {
    let charts: Vec<Invariant> = f
        .vertices
        .iter()
        .filter_map(|v| v.face_chart())
        .collect();
    for (got, chart) in standing_demands(f) {
        assert_eq!(
            got.class_key(),
            rotor.class_key(),
            "the witness is the orbit's holonomy, by class"
        );
        assert!(
            charts.contains(&chart),
            "on a chart the orbit's vertices stand at, got {chart:?} of {charts:?}"
        );
    }
}

/// THE EXPAND PATH: a bubble holding a form whose one orbit carries the
/// weight-11 rotor on its edge, expanded over an empty atlas against a
/// crystal holding nothing, leaves the form it takes back with a `Standing`
/// vertex on the orbit's chart, in the holonomy's class.
#[test]
fn an_expanded_bubble_stands_its_orbits_demand_on_the_form_it_takes() {
    let form = edge_chord_form(&[Word::formed(eleven())]);
    let mut b = Bubble { form: Some(form), ..Default::default() };
    let atlas = Stalks::default();
    let section_at = |_: &[u8]| None;
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 8);
    let _ = b.expand_with(&atlas, &section_at, 0, &crate::transducer::family::NoCrystal, &mut budget);
    let f = b.form.as_ref().expect("the bubble keeps its form");
    each_demand_carries(f, &eleven());
    assert!(!f.is_closed(), "a form carrying a demand is not closed");
}

/// THE PIVOT-ON PATH: the same form held by a contemplation and pivoted on
/// by a word at its chart writes the same vertex, so the two paths are one
/// reading (law 12).
#[test]
fn a_pivoted_on_bubble_stands_its_orbits_demand_on_the_form_it_takes() {
    use crate::transducer::element::{canonical_chain, from_canonical};
    let form = edge_chord_form(&[Word::formed(eleven())]);
    let rem = vec![(0i64, 0i64, 1u8, 2u8), (0, 1, 1, 2)];
    let mut c = Contemplation::new();
    c.set_now(10);
    c.hold_form(form.clone(), &[], std::slice::from_ref(&rem), [0; 4]);
    let word = from_canonical(&canonical_chain(&rem).expect("one entry per position is A5's chain"));
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 8);
    let _ = c.pivot_on(&word, &crate::transducer::family::NoCrystal, &mut budget);
    let held: Vec<&Bubble> = c.bubbles.scoped().map(|(_, b)| b).collect();
    let [b] = held.as_slice() else { panic!("one bubble holds the form, got {}", held.len()) };
    let f = b.form.as_ref().expect("the bubble keeps its form");
    each_demand_carries(f, &eleven());
}
