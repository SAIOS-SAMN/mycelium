// 10 — THE GAUGE and its coboundary: the hue permutation around an orbit,
// the spanning forest that solves it, and the residue a placement leaves.

use super::common::*;
use crate::network::nerve::{coord, Keyed};
use crate::transducer::alphabet::{self, Word};
use crate::transducer::element::{self};
use crate::transducer::sentence::*;
use crate::transducer::stalk::{self};

/// Every overlap's place, written by `write` — a keyed walk of the faces by
/// their overlaps (AT1 Q2), each written at its own key.
fn write_every_place(f: &mut Form, write: impl Fn(&mut FacePlace)) {
    let overlaps: Vec<coord::Overlap> = f.faces.entries().map(|(o, _)| o).collect();
    for o in overlaps {
        set_face_place(f.faces.at_mut(&o).expect("an overlap just read"), &write);
    }
}

#[test]
fn placement_residue_is_the_affine_translation() {
    let mut f = chord_form(&[Word::identity()]);
    write_every_place(&mut f, |p| p.gauge.clear());
    let chord = f.orbits[0].chord.clone();
    set_face_place(f.faces.at_mut(&chord).expect("the chord"), |p| {
        p.carriage = element::carriage(&q1(), &(q1(), q0()));
    });
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    // THE ORBIT THE FIXTURE SET UP, by the chord it wrote — not index 0.
    // A trivial cycle no longer stands in `orbits`, so the position of an
    // orbit is not its identity and never was (rule 15).
    let at = f.orbit_of_chord(&chord).expect("the chord's orbit");
    let kappa = f.class_vector(at).expect("orbit");
    assert!(
        kappa.occupies(Axis::Placement),
        "a nonzero composed shift occupies Placement"
    );
    assert!(
        element::translator_of(&kappa.carriage)
            .is_some_and(|t| !element::same_versor(&t, &element::identity_versor())),
        "the residue is the translator of the holonomy versor"
    );
    let snaps = f.pivot_orbit(
        [0xa4; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    assert!(
        snaps.iter().any(|s| matches!(s, Snap::Discharged { certificate, .. } if certificate.axis == Axis::Placement)),
        "the residue discharges on Placement: {snaps:?}"
    );
    let after = f.class_vector(at).expect("orbit remains");
    assert!(
        !after.occupies(Axis::Placement),
        "Placement is zero after the pivot"
    );
}

#[test]
fn hue_cycle_type_composes_around_the_orbit() {
    let mut f = chord_form(&[Word::identity()]);
    write_every_place(&mut f, |p| p.gauge.clear());
    let chord = f.orbits[0].chord.clone();
    set_face_place(f.faces.at_mut(&chord).expect("the chord"), |p| p.gauge = vec![(3, 4), (4, 3)]);
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    // THE ORBIT THE FIXTURE SET UP, by the chord it wrote — not index 0.
    // A trivial cycle no longer stands in `orbits`, so the position of an
    // orbit is not its identity and never was (rule 15).
    let at = f.orbit_of_chord(&chord).expect("the chord's orbit");
    let kappa = f.class_vector(at).expect("orbit");
    assert!(
        kappa.occupies(Axis::Hue),
        "a non-identity composed gauge occupies Hue, got {:?}",
        kappa.hue
    );
    let snaps = f.pivot_orbit(
        [0xa5; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    assert!(
        snaps.iter().any(|s| matches!(s, Snap::Discharged { certificate, .. } if certificate.axis == Axis::Hue)),
        "the cycle type discharges on Hue: {snaps:?}"
    );
    let after = f.class_vector(at).expect("orbit remains");
    assert!(!after.occupies(Axis::Hue), "Hue is identity after the pivot");
}

#[test]
fn conjugate_hue_permutations_are_one_cycle_type() {
    let a = CycleType::of(&[(1, 2), (2, 1)]);
    let b = CycleType::of(&[(3, 4), (4, 3)]);
    assert_eq!(a, b, "cycle type is sorted lengths, not the listing");
    assert_eq!(a, CycleType::Type(vec![1]), "one 2-cycle, written ℓ − 1");
    let z = element::identity_versor();
    let ka = ClassVector::read(&Word::identity(), z.clone(), a).expect("identity");
    let kb = ClassVector::read(&Word::identity(), z, b).expect("identity");
    assert!(ka.occupies(Axis::Hue) && kb.occupies(Axis::Hue));
    assert_eq!(
        ka.class_key(),
        kb.class_key(),
        "(1 2) and (3 4) are one Hue coordinate"
    );
    assert!(!ka.is_zero(), "a hue cycle is not closed");
}

/// STD1 — A CYCLE OF EVERY HUE IS READ, NEVER PINNED. Hue 0 is a listed hue,
/// so one cycle can run all 256; its length is no `u8`, and written by its
/// coordinate over the carrier, `ℓ − 1 = 255`, it is exact — where the
/// length it replaced stood pinned at 255, one short, the type of a
/// 255-cycle.
#[test]
fn a_cycle_of_every_hue_is_read_exactly() {
    let every: Vec<(u8, u8)> = (0..=255u8).map(|h| (h, h.wrapping_add(1))).collect();
    assert_eq!(CycleType::of(&every), CycleType::Type(vec![255]), "one 256-cycle");
    let short: Vec<(u8, u8)> = (0..=254u8).map(|h| (h, if h == 254 { 0 } else { h + 1 })).collect();
    assert_eq!(CycleType::of(&short), CycleType::Type(vec![254]), "one 255-cycle, apart from it");
}

#[test]
fn coboundary_gauge_matches_by_address() {
    let q = Word::from_letters(&[(alphabet::LetterId::quarter((0, 1)).unwrap(), false)]);
    // The quarter turn's class, the rebased vertex's own (AT1: a Rebased
    // vertex holds the class it had, beside its base's coordinate).
    let q_class = word_vertex(1, q.clone()).class().expect("a quarter turn has a class");
    let rebased = || {
        Vertex::rebased_on(
            coord::VertexAt::Frame,
            q_class.clone(),
            stalk::Invariant::default(),
            Box::new(element::carriage(&q1(), &(q0(), q1()))),
            Vec::new(),
            [FRAME],
        )
    };
    let mut a = Form::default();
    stand_with_star(&mut a, &word_vertex(1, q.clone()));
    stand_with_star(&mut a, &word_vertex(2, Word::identity()));
    a.put_vertex(rebased()).expect("a rebased vertex has a coordinate");
    a.put_vertex(Vertex::Unfolded {
        measure: Measure::Occupied,
    })
    .expect("an unfolded vertex has a coordinate");
    a.reindex();
    let mut b = Form::default();
    stand_with_star(&mut b, &word_vertex(2, Word::identity()));
    stand_with_star(&mut b, &word_vertex(1, q.clone()));
    b.put_vertex(Vertex::Unfolded {
        measure: Measure::Occupied,
    })
    .expect("an unfolded vertex has a coordinate");
    b.put_vertex(rebased()).expect("a rebased vertex has a coordinate");
    b.reindex();
    assert!(
        coboundary_gauge(&a, &b).is_some(),
        "the same words in a different vertex order are one coboundary"
    );
}

// `solve_coboundary_is_a_spanning_forest` STOOD HERE and is deleted: it
// tested the tree gauge — `solve_coboundary`'s spanning forest and
// `regauge`'s refusal of a short gauge — and the tree gauge RETIRED with
// both (AT1 step 2, ruled (a)); a test of a retired subject retires with it.

#[test]
fn holonomy_is_oriented() {
    let w = order4();
    // TWO ENDS ARE TWO SITES (§1.5), as in `chord_form`: two members of the
    // identity's class on charts `[1]` and `[2]`, each on the Frame's star.
    let bodies = || {
        let mut f = Form::default();
        let a = stand_with_star(&mut f, &word_vertex_on(1, Word::identity(), stalk::Invariant::counts(&[1])));
        let b = stand_with_star(&mut f, &word_vertex_on(2, Word::identity(), stalk::Invariant::counts(&[2])));
        (f, a, b)
    };
    let (mut fwd, a, b) = bodies();
    chord(&mut fwd, &a, &b, w.clone());
    fwd.orbits = Form::orbits_of(&fwd.faces, &fwd.vertices);
    let (mut rev, a, b) = bodies();
    chord(&mut rev, &b, &a, w.inverse());
    rev.orbits = Form::orbits_of(&rev.faces, &rev.vertices);
    assert_eq!(fwd.orbits.len(), 1);
    assert_eq!(rev.orbits.len(), 1);
    // The holonomy is READ along the steps (AT1, ruled), never held.
    let holonomy = |f: &Form| orbit_word_on(&f.faces, &f.vertices, &f.orbits[0]).expect("the orbit's word reads");
    assert_eq!(
        holonomy(&fwd).class_key(),
        holonomy(&rev).class_key(),
        "a reversed chord stores the inverse word and the same class"
    );
}

#[test]
fn relative_gauge_composite_fixes_what_neither_fixes() {
    let t = vec![(1u8, 2), (2, 1)];
    assert!(
        compose_gauge(&t, &t).is_empty(),
        "two (1 2) transpositions compose to the identity"
    );
    let a = vec![(1u8, 2)];
    let b = vec![(2u8, 1)];
    let c = compose_gauge(&b, &a);
    assert!(
        c.iter().all(|&(h, k)| h != 1 || k != 1),
        "composite listing omits fixed points, got {c:?}"
    );
    assert_eq!(
        gauge_image(&gauge_keyed(&c), 1),
        1,
        "1→2 then 2→1 fixes 1, which neither map lists as fixed"
    );
}

/// A GAUGE IS A MAP, AND A LISTING THAT NAMES TWO IMAGES FOR ONE HUE IS NOT
/// ONE. The listing is a `Vec<(from, to)>` deduplicated by the PAIR, so a
/// chain whose links send one input hue to two different output hues yields
/// both entries and neither is the answer. A hue decided by a Vec's order
/// would be a member by position (rule 15), and the court and the speaker
/// would read different images of one lamina; the listing is refused at
/// that hue instead.
#[test]
fn a_gauge_naming_two_images_for_one_hue_is_refused_not_resolved_by_order() {
    // The unambiguous cases first: a listed hue, and one the listing fixes.
    assert_eq!(hue_of(&[(1, 2), (3, 4)], 1), Some(2));
    assert_eq!(hue_of(&[(1, 2), (3, 4)], 3), Some(4));
    assert_eq!(hue_of(&[(1, 2)], 7), Some(7), "a hue the listing omits is fixed (18.2)");
    assert_eq!(hue_of(&[], 5), Some(5), "and an empty gauge fixes everything");
    // A repeated pair is one statement made twice, not two images.
    assert_eq!(hue_of(&[(1, 2), (1, 2)], 1), Some(2));
    // Two images for one hue: refused, and refused whichever order the
    // listing happens to be in — which is the whole point.
    assert_eq!(hue_of(&[(1, 2), (1, 3)], 1), None);
    assert_eq!(hue_of(&[(1, 3), (1, 2)], 1), None);
    assert_eq!(
        hue_of(&[(1, 3), (1, 2)], 9),
        Some(9),
        "and the refusal is at that hue alone, not over the whole gauge"
    );
}

/// 18.2 — THE GAUGE READ AT ITS OWN COORDINATE. A gauge is a partial map on
/// the ten hues and absence is the identity; held as a listing of
/// `(from, to)` it is that map's graph written out as points, and every
/// question about it becomes a scan. The hue is the coordinate.
#[test]
fn a_gauge_is_a_map_on_the_hues_and_absence_is_the_identity() {
    let t = hue_table(&[(1, 4), (9, 9)]).expect("a listing that names each hue once is a map");
    assert_eq!(t[1], Some(4), "a hue the listing moves");
    assert_eq!(t[9], Some(9), "a hue it carries to itself is still named");
    assert_eq!(t[3], None, "and a hue it is silent about is not");

    // The listing comes back in hue order and holds exactly what was named,
    // so two orderings of one map are one map.
    assert_eq!(hue_listing(&t), vec![(1, 4), (9, 9)]);
    assert!(gauge_eq(&[(9, 9), (1, 4)], &[(1, 4), (9, 9)]), "order is not content");

    // A listing that sends one hue two ways is no map, on either side of a
    // comparison (rule 11).
    assert!(hue_table(&[(1, 4), (1, 5)]).is_none());
    assert!(!gauge_eq(&[(1, 4), (1, 5)], &[(1, 4), (1, 5)]));
    // And a hue outside the ten the sensor has is no coordinate of one.
    assert!(hue_table(&[(10, 1)]).is_none());
}

/// TWO GAUGES COMPOSE OR THEY ARE TWO SECTIONS — and which one it is has to
/// be readable, because dropping the difference is what made the largest
/// vertex-body fault on the fleet.
///
/// They compose where their hues do not meet, or meet and agree: one member
/// speaks both listings, which is what 18.2's partial map means. Where they
/// meet and DISAGREE they are two assignments and the class varies across
/// the bundle, exactly as it does when the carriage differs.
#[test]
fn two_gauges_compose_or_name_the_hue_they_send_two_ways() {
    let disjoint_a = hue_table(&[(1, 4)]).expect("map");
    let disjoint_b = hue_table(&[(9, 9)]).expect("map");
    match hue_agreement(&disjoint_a, &disjoint_b) {
        HueAgreement::Agreed(t) => {
            assert_eq!(
                hue_listing(&t),
                vec![(1, 4), (9, 9)],
                "hues that do not meet compose into one map that speaks both"
            );
        }
        HueAgreement::Apart { .. } => panic!("disjoint hues do not disagree"),
    }

    // Meeting and agreeing is still one map.
    let same = hue_table(&[(1, 4), (2, 2)]).expect("map");
    let overlap = hue_table(&[(1, 4), (7, 3)]).expect("map");
    assert!(matches!(
        hue_agreement(&same, &overlap),
        HueAgreement::Agreed(_)
    ));

    // MEETING AND DISAGREEING NAMES THE HUE AND BOTH ANSWERS. This is the
    // reading the union destroyed: it deduped on `from`, kept whichever
    // arrived first, and recorded nothing — so the vertex claimed one map
    // across every pair and the court found the other colour, 880 times,
    // while `FAIL_GAUGE_AMBIGUOUS` sat at 0 because the ambiguous listing
    // was never built for `hue_of` to refuse.
    let here = hue_table(&[(3, 5)]).expect("map");
    let there = hue_table(&[(3, 7)]).expect("map");
    match hue_agreement(&here, &there) {
        HueAgreement::Apart { hue, here, there } => {
            assert_eq!((hue, here, there), (3, 5, 7), "the coordinate and both answers");
        }
        HueAgreement::Agreed(_) => panic!("one hue sent two ways is not one map"),
    }

    // Order does not decide: the disagreement is the same fact read from
    // either side, which is what the arrival-ordered union could not say.
    assert!(matches!(
        hue_agreement(&there, &here),
        HueAgreement::Apart { hue: 3, here: 7, there: 5 }
    ));
}
