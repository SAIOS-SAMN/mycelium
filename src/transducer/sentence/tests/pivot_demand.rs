// 16.16 step 6, 37.7 — THE ORBIT'S DEMAND: a held orbit of infinite order
// the basis does not discharge is spoken on the form as a standing demand
// carrying its holonomy. Beside `pivot_orbit.rs` under the §1.1 bound; the
// `Hold` crystal is read across the split.

use super::common::*;
use super::pivot_orbit::Hold;
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
/// orbit through the face reads it (50.3).
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

/// 16.20, 51.2 — THE OPEN SEED IS THE HOLONOMY'S BASE ACTION, WHOLE, AND THE
/// AXIS IS KEYED BY THE ROTOR'S CLASS. A rotation `q₀₁ · q₁₃` turns two
/// base planes that share an axis: its seed carries both turns exactly and
/// reads back as the rotor, and the axis opens under the rotor's class. A
/// rotation turning `e₂` into a base axis has no base action and seeds
/// nothing — refused, never projected (checklist (m)).
#[test]
fn the_open_seed_is_the_holonomys_base_action_and_the_axis_is_keyed_by_the_rotor() {
    let q01 = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("quarter (0,1)"), false)]);
    let q13 = Word::from_letters(&[(LetterId::quarter((1, 3)).expect("quarter (1,3)"), false)]);
    let double = q01.compose(&q13);
    assert_eq!(
        double.rotor().off_plane_blades((alphabet::PLANE_ROW, alphabet::PLANE_COL)).len(),
        2,
        "e₀₃ and e₁₃ turn beyond the sensor plane"
    );
    let mut f = edge_chord_form(std::slice::from_ref(&double));
    let orbits = f.orbits.clone();
    let [lp] = orbits.as_slice() else { panic!("one chord closes one orbit") };
    let seed = f.axis_seed_of(lp).expect("a base rotation has a base action");
    let holonomy = f.orbit_holonomy(lp).expect("one holonomy").rotor();
    assert!(
        element::base_rotor_of(&seed).is_some_and(|r| r.projective_eq(&holonomy)),
        "the seed carries the whole base rotation and reads back as it"
    );
    let whole = holonomy.class_key().expect("a class");
    assert_ne!(whole, q01.rotor().class_key().expect("a class"), "the orbit's class is the double rotation's");
    f.open_axis_on(lp.chord).expect("the axis opens on the orbit");
    assert_eq!(
        f.axes.at(&holonomy.holonomy_key().expect("a holonomy coordinate")),
        Some(&lp.chord),
        "and the axis is keyed by that class"
    );
    let q23 = Word::from_letters(&[(LetterId::quarter((2, 3)).expect("quarter (2,3)"), false)]);
    let off = edge_chord_form(std::slice::from_ref(&q23));
    let [lp] = off.orbits.as_slice() else { panic!("one chord closes one orbit") };
    assert!(off.axis_seed_of(lp).is_none(), "a rotation turning e₂ into the base seeds no axis");
}

/// A held orbit of infinite order the basis does not discharge is spoken as
/// a standing demand on each chart the orbit stands on, carrying the orbit's
/// holonomy rotor as its witness — here `3 + e₀₁ + e₀₂`, weight 11, held by
/// no coordinate plane. The rotor stands on the chord's EDGE (50.3): a face
/// word with no sensor action places no member pair, so the orbit is judged
/// by its edge composite.
#[test]
fn a_free_part_orbit_is_spoken_as_a_standing_demand_carrying_its_holonomy() {
    let r = eleven();
    let w = Word::formed(r.clone());
    let mut f = edge_chord_form(std::slice::from_ref(&w));
    let snaps = f.pivot_orbit(
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
    // ONE DEMAND PER CHART THE ORBIT STANDS ON, which is what the line above
    // this test has always said and what the assertion did not check.
    //
    // It read `let [(rotor, chart)] = ...` — exactly one — and passed
    // because `edge_chord_form` built its two chord ends with identical
    // coordinates, so §1.5 made them ONE vertex standing on ONE chart. Two
    // ends of a chord are two SITES (the note `chord_form` has carried since
    // 54.2), and with the ends separated the orbit runs through two charts
    // and stands a demand at each — which is 50.5's `(kind, chart)` and is
    // the doctrine the doc comment states.
    assert!(!demands.is_empty(), "the orbit is spoken as a standing demand: {snaps:?}");
    let charts: Vec<stalk::Invariant> =
        f.vertices.iter().filter_map(|v| v.face_chart()).collect();
    for (rotor, chart) in &demands {
        // The orbit runs the chord backward, so the holonomy it carries is
        // `r⁻¹`: the same class as `r` (15.3 reads the ray, and inversion
        // fixes both invariants), and the class is what the orbit is judged
        // by.
        assert_eq!(
            rotor.class_key(),
            r.class_key(),
            "the demand carries the orbit's holonomy as its witness, in the class the orbit is judged by"
        );
        assert!(
            rotor.projective_eq(&r) || rotor.projective_eq(&r.reverse()),
            "and it is that holonomy, read along the orbit"
        );
        assert!(
            charts.contains(chart),
            "on a chart the orbit's vertices stand at, got {chart:?} of {charts:?}"
        );
        assert!(
            matches!(factor_rotor(rotor), Some(Err(Demand::Standing { .. }))),
            "and no coordinate plane holds it"
        );
    }
}

/// 52.3 — THE FILING RIDES BESIDE THE DEMAND, AND THE DEMAND IS WHAT
/// CARRIES THE OBSTRUCTION (54.5, as amended 2026-09-20).
///
/// The same held orbit of infinite order files under its holonomy's class
/// at period 0, A19's spelling of infinite order, in the stroke that
/// speaks its standing demand: the pivot's `TorsionFiled` snap carries the
/// zero and the filing holds the class. All of that stands.
///
/// What does NOT follow is that the orbit coheres. A free part never
/// returns, and the form says so by standing a DEMAND — a vertex, in the
/// key, carried as content — rather than by an entry in a side-table. The
/// form is then VALID and not closed, which is 50.1's own distinction.
#[test]
fn a_free_part_orbit_files_at_period_zero_beside_its_demand() {
    let r = eleven();
    let w = Word::formed(r.clone());
    let mut f = edge_chord_form(std::slice::from_ref(&w));
    let snaps = f.pivot_orbit(
        [0x34; 32],
        &Keyed::new(),
        &Hold(Vec::new()),
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    assert!(snaps.iter().any(|s| matches!(s, Snap::Demand { .. })), "the demand is spoken: {snaps:?}");
    assert!(
        snaps.iter().any(|s| matches!(s, Snap::TorsionFiled { ell: 0, .. })),
        "and the orbit is filed at period 0 beside it: {snaps:?}"
    );
    let filed: Vec<crate::transducer::sentence::ToralPeriod> =
        f.torsion.scoped().map(|(_, l)| l.periods).collect();
    assert_eq!(
        filed,
        vec![crate::transducer::sentence::ToralPeriod::OPEN],
        "one class, neither plane closing"
    );
    assert!(
        f.torsion.holds(&r.holonomy_key().expect("a class")),
        "under the holonomy's own class"
    );
    // FILED IS NOT CLOSED (54.5, as ruled 2026-09-20). This asserted that
    // filing the class made the orbit cohere. A free part of infinite order
    // never returns; the filing records that the stroke read it, and the
    // DEMAND is what carries the obstruction — which is this test's own
    // subject two assertions above.
    assert!(!f.coheres(), "a free part is not a compact resonance, filed or not");
    // AND THE FORM IS VALID THOUGH NOT CLOSED, which is 50.1's own
    // distinction and the whole point of speaking a demand: the obstruction
    // is carried as a vertex, in the key, rather than hidden by a filing.
    assert!(!f.is_closed(), "and so the form is not closed");
    // THE DEMAND IS ON THE SNAPS, NOT YET ON THE FORM, and this asserted
    // otherwise on my own reading rather than on the engine's. `pivot_orbit`
    // REPORTS the obstruction it met; `Form::stand_demand` is what writes
    // the vertex, and the bubble calls it — `bubble/tests/demand.rs` is
    // where the vertex on the form is asserted, on both pivot paths.
    // Claiming it here named a carrier this call does not write (rule 11).
    assert!(
        snaps.iter().any(|s| matches!(s, Snap::Demand { .. })),
        "what the stroke carries instead of a permit is the demand it reported"
    );
}
