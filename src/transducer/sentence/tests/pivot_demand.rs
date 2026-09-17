// 16.16 step 6, 37.7 — THE LOOP'S DEMAND: a held loop of infinite order
// the basis does not discharge is spoken on the form as a standing demand
// carrying its holonomy. Beside `pivot_loop.rs` under the §1.1 bound; the
// `Hold` crystal is read across the split.

use super::common::*;
use super::pivot_loop::Hold;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{self, factor_rotor, LetterId, Rotor, Word, Q};
use crate::transducer::element;
use crate::transducer::sentence::*;
use crate::transducer::stalk;

/// `3 + e₀₁ + e₀₂`: weight 11, held by no coordinate plane (37.7).
fn eleven() -> Rotor {
    let q = |n: i64| Q::from_integer(num_bigint::BigInt::from(n));
    Rotor::scalar(q(3)).add(&Rotor::blade(0b0011, q(1))).add(&Rotor::blade(0b0101, q(1)))
}

/// 51.2 — A FACE WHOSE WORD HAS NO SENSOR ACTION PLACES NO MEMBER PAIR: the
/// pair's carriage is derived through `versor_of_word`, one plane by
/// construction, so the incidence names a pair and the face carries none.
/// The same two vertices under the identity word carry their one pair, and
/// the off-plane rotor stands on the edge transport instead, where every
/// loop through the face reads it (50.3).
#[test]
fn a_face_whose_word_has_no_sensor_action_places_no_member_pair() {
    let mut f = Form::default();
    f.vertices.push(word_vertex(1, Word::identity()));
    f.vertices.push(word_vertex(2, Word::identity()));
    let off = f.face_carrying(1, 2, Word::formed(eleven())).expect("a face between two bodies");
    assert_eq!(off.at.len(), 0, "the incidence named one pair and the word places none of it");
    let on = f.face_carrying(1, 2, Word::identity()).expect("a face between two bodies");
    assert_eq!(on.at.len(), 1, "the identity word places the pair the incidence named");
    let edged = f
        .face_carrying_with_edge(1, 2, Word::identity(), Some(Transport::from_word(Word::formed(eleven()))))
        .expect("a face between two bodies");
    assert_eq!(edged.at.len(), 1, "an edge transport rides the placed pair");
    assert!(
        edged.at.scoped().all(|(_, p)| p.edge.as_ref().is_some_and(|t| t.word.rotor().projective_eq(&eleven()))),
        "and carries the rotor whole"
    );
}

/// 16.20, 51.2 — THE OPEN SEED IS THE HOLONOMY'S SENSOR ACTION AND THE AXIS
/// IS KEYED BY THE WHOLE ROTOR. A double rotation `q₀₁ · q₂₃` keeps the
/// sensor plane and turns another; its seed is the `(0,1)` quarter alone,
/// its class key is the double rotation's, and the two differ. This is the
/// `Cl(3,1)` bridge read exactly, so the widening to `Cl(4,1)` (N6) has the
/// claim to rewrite.
#[test]
fn the_open_seed_carries_the_sensor_action_while_the_axis_is_keyed_by_the_rotor() {
    let q01 = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter (0,1)"), false)]);
    let q23 = Word::from_letters(&[(LetterId::quarter((2, 3)).expect("quarter (2,3)"), false)]);
    let double = q01.compose(&q23);
    assert_eq!(
        double.rotor().off_plane_blades((alphabet::PLANE_ROW, alphabet::PLANE_COL)).len(),
        2,
        "e₂₃ and the pseudoscalar turn beyond the sensor plane"
    );
    let mut f = edge_chord_form(std::slice::from_ref(&double));
    let loops = f.loops.clone();
    let [lp] = loops.as_slice() else { panic!("one chord closes one loop") };
    let seed = f.axis_seed_of(lp).expect("the holonomy keeps the sensor plane, so it has a sensor action");
    assert!(
        element::same_versor(&seed, &element::versor_of_word(&q01.inverse()).expect("a quarter's sensor action")),
        "the seed is the (0,1) quarter the loop runs, and nothing of (2,3)"
    );
    let whole = f.loop_holonomy(lp).expect("one holonomy").rotor().class_key().expect("a class");
    assert_ne!(whole, q01.rotor().class_key().expect("a class"), "the loop's class is the double rotation's");
    f.open_axis_on(lp.chord).expect("the axis opens on the loop");
    assert_eq!(f.axes.get(&whole), Some(&lp.chord), "and the axis is keyed by that class, not the seed's");
}

/// A held loop of infinite order the basis does not discharge is spoken as
/// a standing demand on each chart the loop stands on, carrying the loop's
/// holonomy rotor as its witness — here `3 + e₀₁ + e₀₂`, weight 11, held by
/// no coordinate plane. The rotor stands on the chord's EDGE (50.3): a face
/// word with no sensor action places no member pair, so the loop is judged
/// by its edge composite.
#[test]
fn a_free_part_loop_is_spoken_as_a_standing_demand_carrying_its_holonomy() {
    let r = eleven();
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
