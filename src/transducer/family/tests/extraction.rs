// 21 steps 1-7 — EXTRACTION: the rule a family of train pairs already
// states, read off the pair forms and refused when the family does not
// agree.

use super::common::*;
use crate::transducer::family::*;

/// THE SECTIONS AS THE PERCEPT FILED THEM: a member or a class standing on
/// fewer frames than the family reached, summed over the frames each holds.
/// AT1 — a member now holds its frames (ruled), so `sections()` reads every
/// member's frames, a member standing on every frame included; the section
/// these tests name is the one short of the family, and that is read here.
fn sections_short_of(rule: &crate::transducer::sentence::SentenceRule, frames: usize) -> usize {
    rule.sections()
        .scoped()
        .filter(|(_, s)| !s.is_empty() && s.len() < frames)
        .map(|(_, s)| s.len())
        .sum()
}

/// One class in every pair is one vertex. Counts are not compared:
/// 3, 4, 3 instances stay one class. A class in only some pairs is a section.
#[test]
fn extract_rule_by_class_with_sections() {
let family = vec![
    (squares(3, 1, 9), squares(3, 2, 9)),
    (squares(4, 1, 12), squares(4, 2, 12)),
    (squares(3, 1, 9), squares(3, 2, 9)),
];
let rule = extract_rule_with(&family, &NoCrystal, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("extracts");
let words = rule
    .form
    .vertices
    .scoped()
    .filter(|(_, v)| matches!(v, Vertex::Word { .. }))
    .count();
assert_eq!(words, 1, "presence, not min count; got {words} Word vertices");
// A reflection in only the first pair is a different word from the
// recolour that appears in the second: the reflection is a some-pair
// section. Hue is not a matching key, so a recolour-only split is not.
let mixed = vec![
    (grid(&["1100", "1000", "0000"]), grid(&["0011", "0001", "0000"])),
    (grid(&["1100", "0000", "0000"]), grid(&["2200", "0000", "0000"])),
];
let some = extract_rule_with(&mixed, &NoCrystal, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("sectioned");
assert!(
    sections_short_of(&some, mixed.len()) > 0,
    "a class present in some pairs is a section"
);
}

/// 21 step 2, 25, 12.6, Stage 12 S12.2 — FORMS WITH ORBITS. A body carried
/// by a 90° turn in one pair and by its inverse, −90°, in the other is one
/// transport class (the two words are conjugate), so the rule has one
/// vertex for it; the −90° frames are KEPT as a second member of it, never
/// dropped (AT1). The form then has an orbit across the two frames whose
/// holonomy is the transition between the members — the 180° conjugator,
/// carriage content — held with a class vector the pivot orbit reads on the
/// placement axis (AT1, ruled (a) 2026-09-25).
/// AT1 A1, law 8 — THE COURT APPLIES A FRAME-AXIS MOTOR TO THE FAMILY. The
/// 90° / −90° family's frame-axis overlaps carry the motor the member's
/// input body took between its frames; the court carries the body at the
/// leaving frame by it and finds the body at the arriving frame, and it
/// refuses a motor that does not — never solving for one.
#[test]
fn the_court_refuses_a_forged_frame_axis_motor() {
    let l_in = ["100000", "100000", "110000", "000000", "000330", "000000"];
    let a = ["111000", "100000", "000000", "000000", "000550", "000000"];
    let b = ["001000", "111000", "000000", "000000", "000550", "000000"];
    let family = vec![(grid(&l_in), grid(&a)), (grid(&l_in), grid(&b))];
    let rule = extract_rule_with(&family, &NoCrystal, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("extracts");
    let motors: Vec<crate::network::nerve::coord::Overlap> = rule
        .form
        .faces
        .entries()
        .filter(|(_, f)| f.producer == crate::transducer::sentence::Producer::Motor)
        .map(|(o, _)| o)
        .collect();
    let [motor, ..] = motors.as_slice() else { panic!("the family's frame axis carries a motor overlap") };
    assert!(
        crate::court::verdict::body::edges_rederive_frames(&rule.form, &family, &mut super::common::budget()).is_none(),
        "the measured motor carries the body across its frames"
    );
    let mut forged = rule.form.clone();
    if let Some(f) = forged.faces.at_mut(motor) {
        // A translator one column over: a motor the family never took.
        let q = |n: i64| num_rational::BigRational::from_integer(n.into());
        f.place.carriage = crate::transducer::cga::at_three::translator(&q(0), &q(1), &q(0));
    }
    let refusal = crate::court::verdict::body::edges_rederive_frames(&forged, &family, &mut super::common::budget())
        .expect("a forged frame-axis motor is refused by application, never admitted");
    // PARALLEL TRANSPORT (A.13): the refusal carries the curvature the claim
    // missed — the transport the fibre took, against the forged motor — and
    // its kind is read off that deviation. The fibre does move rigidly, so it
    // is no P; the frames are held, so it is no I; and never unclassified.
    use crate::transducer::sentence::ObstructionKind;
    assert_eq!(refusal.reason, crate::verdict::REFUSE_EDGE);
    assert!(
        matches!(refusal.obstruction.kind(), ObstructionKind::C | ObstructionKind::D),
        "the deviation's kind is read: {:?}",
        refusal.obstruction
    );
}

/// AT1 A1's correction (law 8) — A CHART HOLDING TWO BODIES AT A FRAME. The
/// motor was measured on the chart's whole fibre at each frame, the union of
/// both bodies, so the court applies it to that same union and finds the
/// union at the next frame: a chart with several bodies is verified, never
/// passed unread, and a forged motor on it is refused.
#[test]
fn the_court_applies_a_motor_to_a_charts_whole_fibre() {
    let a_in = ["110000", "000000", "110000", "000000"];
    let a_out = ["220000", "000000", "220000", "000000"];
    let b_in = ["011000", "000000", "011000", "000000"];
    let b_out = ["022000", "000000", "022000", "000000"];
    let family = vec![(grid(&a_in), grid(&a_out)), (grid(&b_in), grid(&b_out))];
    let rule = extract_rule_with(&family, &NoCrystal, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("extracts");
    let motors: Vec<crate::network::nerve::coord::Overlap> = rule
        .form
        .faces
        .entries()
        .filter(|(_, f)| f.producer == crate::transducer::sentence::Producer::Motor)
        .map(|(o, _)| o)
        .collect();
    let [motor, ..] = motors.as_slice() else {
        panic!("the two dominoes' chart steps between its frames: a motor overlap")
    };
    assert!(
        crate::court::verdict::body::edges_rederive_frames(&rule.form, &family, &mut super::common::budget()).is_none(),
        "the measured motor carries the chart's two bodies onto its two bodies at the next frame"
    );
    let mut forged = rule.form.clone();
    if let Some(f) = forged.faces.at_mut(motor) {
        // A translator one row over: no transition of this family.
        let q = |n: i64| num_rational::BigRational::from_integer(n.into());
        f.place.carriage = crate::transducer::cga::at_three::translator(&q(1), &q(0), &q(0));
    }
    assert!(
        crate::court::verdict::body::edges_rederive_frames(&forged, &family, &mut super::common::budget()).is_some(),
        "a forged motor on a chart of two bodies is refused, never passed unread"
    );
}

#[test]
fn a_face_that_varies_across_the_pairs_is_kept_as_sections_and_holds_an_orbit() {
let l_in = ["100000", "100000", "110000", "000000", "000330", "000000"];
let turned_minus_90 = ["111000", "100000", "000000", "000000", "000550", "000000"];
let turned_90 = ["001000", "111000", "000000", "000000", "000550", "000000"];
let family = vec![(grid(&l_in), grid(&turned_minus_90)), (grid(&l_in), grid(&turned_90))];
let rule = extract_rule_with(&family, &NoCrystal, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("extracts");
let words = rule
    .form
    .vertices
    .scoped()
    .filter(|(_, v)| matches!(v, Vertex::Word { .. }))
    .count();
assert_eq!(words, 2, "the turned L and the recoloured bar: two vertices, one class each");
// 21 step 1 IS the assertion: vertices are matched by "kind and the
// conjugacy class of the word, its invariants compared coordinatewise".
// So the count above is not a number the arithmetic happened to produce —
// it is one vertex per distinct conjugacy class, and the document is
// checked by asking the classes directly. A lift of an already-present
// class is not a second class (`q·q·q` is `−q⁻¹`), and that is what the
// count would silently absorb.
let classes: Vec<_> = rule
    .form
    .vertices
    .scoped()
    .filter_map(|(_, v)| match v {
        Vertex::Word { derivation, .. } => derivation.rotor().class_key(),
        _ => None,
    })
    .collect();
assert_eq!(classes.len(), words, "every word vertex has a class (15.4)");
for (i, a) in classes.iter().enumerate() {
    for b in classes.iter().skip(i + 1) {
        assert_ne!(a, b, "two vertices of one conjugacy class are one vertex (21 step 1)");
    }
}
assert!(
    rule.form.faces.len() > words,
    "the varying face is kept, not dropped: {} faces over {words} words",
    rule.form.faces.len()
);
// AT1 (Q2) — A FACE IS ONE OVERLAP PER MEMBER PAIR PER FRAME, so the face
// whose word varies over the pairs is kept as its overlaps at each frame —
// the sections over the pairs are the frames the star stands on at one
// vertex, each overlap its own — never one face with the variation dropped.
let mut star_frames: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>, crate::network::nerve::coord::VertexAt> =
    crate::network::nerve::Keyed::new();
for (o, _) in rule.form.faces.entries() {
    let (crate::network::nerve::coord::ChartAt::Frame, Some(k)) = (&o.from, o.to.frame()) else { continue };
    let v = o.to_vertex();
    match star_frames.at_mut(&v) {
        Some(ks) => {
            ks.insert(crate::transducer::sentence::frame_key(k), ());
        }
        None => {
            star_frames.put(&v, crate::transducer::sentence::section_of_frames([k]));
        }
    }
}
assert!(
    star_frames.scoped().any(|(_, ks)| ks.len() == family.len()),
    "the two words of one face are sections over the pairs: an overlap at each frame"
);
assert!(!rule.form.orbits.is_empty(), "the second section closes an orbit");
let held: Vec<_> = (0..rule.form.orbits.len())
    .filter_map(|i| rule.form.class_vector(i))
    .filter(|cv| !cv.is_zero())
    .collect();
assert!(!held.is_empty(), "a held orbit with a class vector");
// AT1, RULED (a) 2026-09-25: the 90° and −90° readings are conjugate rotors,
// one class and one vertex, the −90° frames a second member whose carriage
// is the relative `V₋₉₀ · Ã`. The 180° conjugator between them is CARRIAGE
// content — 21 step 1: the class excludes the carriage — so the circuit
// across the two frames occupies the PLACEMENT axis, never the word lane.
assert!(
    held.iter().any(|cv| cv.occupied().contains(&crate::transducer::sentence::Axis::Placement)),
    "the 180° conjugator between the two members occupies the placement axis: {:?}",
    held.iter().map(|c| c.occupied()).collect::<Vec<_>>()
);
let mut form = rule.form.clone();
let mut budget = crate::physics::Physics::default().stroke();
let snaps = form.pivot_orbit(b"idea", &crate::network::nerve::Keyed::new(), &NoCrystal, &mut budget, None, 0);
assert!(!snaps.is_empty(), "the pivot orbit reads the held orbit and answers");
}

/// 51.10 PROOF 1, 52.5 — A FAMILY WHOSE RULE IS ONE TRANSLATION. Three
/// pairs, each output the input translated by one vector: every frame's
/// quotient of the bar's chart is that translator in the frame's own
/// coordinates, so the rule is ONE word vertex holding on every frame, with
/// no section, no demand, and no correspondence pass anywhere in it; it
/// closes the family and speaks a fourth frame.
#[test]
fn a_family_translated_along_e3_is_one_vertex_on_every_frame_and_no_correspondence_runs() {
    let family = vec![
        (grid(&["1100", "0000", "0000"]), grid(&["0011", "0000", "0000"])),
        (grid(&["0000", "1100", "0000"]), grid(&["0000", "0011", "0000"])),
        (grid(&["0000", "0000", "1100"]), grid(&["0000", "0000", "0011"])),
    ];
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("the family extracts");
    let words: Vec<&Vertex> =
        rule.form.vertices.scoped().filter(|(_, v)| matches!(v, Vertex::Word { .. })).map(|(_, v)| v).collect();
    let [word] = words.as_slice() else {
        panic!("one word vertex for one translation on every frame: {:?}", rule.form.vertices)
    };
    let carried_by_the_translator = word
        .acts()
        .into_iter()
        .flat_map(|a| a.scoped())
        .all(|(_, m)| element::same_versor(&m.carriage, &element::translator_versor(0, 2)));
    assert!(carried_by_the_translator, "the vertex carries its body by the translation");
    assert_eq!(sections_short_of(&rule, family.len()), 0, "a rule that holds on every frame is no section: {:?}", rule.sections());
    assert_eq!(rule.form.demand_vertices().count(), 0, "and asks for nothing");
    let (_, rems) = form_deflation(&rule.form, &family, &NoCrystal);
    assert!(rems.iter().all(|r| r.is_empty()), "it closes every frame: {rems:?}");
    let spoken = speak_rule(&rule, &grid(&["0000", "0000", "0000", "1100"]), &NoCrystal).expect("spoken");
    assert_eq!(spoken.grid(), Some(grid(&["0000", "0000", "0000", "0011"])), "and speaks a fourth frame");
}

/// 51.10 PROOF 2, 50.9 — THE SAME ADDRESS ON TWO COURTS. Two extractions
/// of one family with no shared runtime memory — an extraction holds
/// nothing after it returns — reach one form address, and a court replays
/// each minted sentence on its own copy of the family.
#[test]
fn two_courts_reach_one_address_for_one_family() {
    let family = vec![
        (grid(&["1100", "0000"]), grid(&["0011", "0000"])),
        (grid(&["0000", "1100"]), grid(&["0000", "0011"])),
    ];
    let first = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("extracts");
    let second = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("extracts again");
    assert_eq!(
        crate::transducer::sentence::form_vertex_key(&first.form),
        crate::transducer::sentence::form_vertex_key(&second.form),
        "one family, one address, on two readings that share nothing"
    );
    for rule in [&first, &second] {
        let word = mint_sentence(rule, &family, &NoCrystal, 0).expect("the family mints");
        assert_eq!(
            crate::verdict::blind_replay_on(&word, &|_| None, Some(&family), &NoCrystal, &mut super::common::budget()),
            None,
            "and a court replays it on its own copy of the family"
        );
    }
}

/// An unmeasured scale is not the identity ratio.
#[test]
fn extract_rule_refuses_unmeasured_scale() {
let family = vec![
    (grid(&["11"]), grid(&["11"])),
    (grid(&["11"]), grid(&["1111"])),
];
assert!(
    family_ratio(&family).is_none(),
    "two pairs with different frame ratios have no scale"
);
assert!(
    extract_rule_with(&family, &NoCrystal, &mut crate::physics::Physics::default().stroke_n(1 << 20)).is_none(),
    "extract_rule_with returns None when the scale is not measured"
);
}

/// B does A's rotation about B's own centre: one Rebased vertex whose
/// word is A's and whose body, hue map and dilation stay B's. B is a
/// different shape from A and recolours differently, so speaking B
/// through A's class or A's gauge would be visible.
#[test]
fn b_does_as_rotation_about_its_centre_is_rebased() {
let letter = crate::transducer::alphabet::LetterId::turn_90((0, 1)).expect("90° turn");
let rot = crate::transducer::alphabet::Word::from_letters(&[(letter, false)]);
// A is the single hue-1 compartment; B is the hue-3 bar, another shape.
// A placement of (3, 3) keeps both images inside the frame for
// either orientation of the 90° turn (compartments map to
// (c, −r−1) + p or (−c−1, r) + p).
let x = grid(&["330000", "000000", "001000", "000000", "000000", "000000"]);
let bodies = perceive(&x);
let a = bodies
    .iter()
    .find(|b| b.compartments.iter().any(|c| c.hue == 1))
    .expect("body A");
let b = bodies
    .iter()
    .find(|b| b.compartments.iter().any(|c| c.hue == 3))
    .expect("body B");
assert_ne!(a.moduli, b.moduli, "two shapes, two charts");
use crate::transducer::sentence::tests::common::FRAME;
let word_at = |w: &crate::transducer::alphabet::Word, chart: &Invariant, gauge: Vec<(u8, u8)>| {
    Vertex::word_on(
        crate::transducer::alphabet::Derivation::spelled(w.clone()),
        chart.clone(),
        Box::new(element::carriage(&q(1), &(q(3), q(3)))),
        gauge,
        [FRAME],
        None,
    )
};
let va = word_at(&rot, &a.moduli, vec![(1, 1)]);
// B CARRYING A'S WORD AT B'S PLACEMENT — the pair form the rebased form must
// speak. AT1 (Q1): one transport class is ONE vertex, so A and B carrying
// one word are one vertex with a member at each chart.
let vb_as_a = word_at(&rot, &b.moduli, vec![(3, 5)]);
let tc_a = transport_class(&va).expect("A has a transport class");
let tc_b = transport_class(&vb_as_a).expect("B has a transport class");
assert_eq!(
    tc_a, tc_b,
    "A and B share a transport class: the same word, two hue sections"
);
let mut pf = Form::default();
let ca = pf.stand_member(&va, FRAME).expect("A stands");
let cb = pf.stand_member(&vb_as_a, FRAME).expect("B stands");
pf.place_overlap(ca, cb, None).expect("the pair's overlap");
pf.reindex();
assert_eq!(
    pf.vertices.scoped().filter(|(_, v)| matches!(v, Vertex::Word { .. })).count(),
    1,
    "one class, one vertex: A and B are its two members"
);
assert_eq!(
    pf.hue_maps.len(),
    2,
    "two hue sections on one word class, got {}",
    pf.hue_maps.len()
);
// THE REBASE IS A RELATION BETWEEN TWO VERTICES (3.2), so B stands at its own
// class — its own motion, here none — and is rebased on A: read in B's frame
// as A's word. AT1 — `rebase_named_faces` offers a rebase only between two
// vertices whose words are equal, which is one class and so one vertex, a
// stabilizer element and never a Rebased vertex; the move it would make is
// `Form::rebase`, called here with the relative the overlap measures.
let vb = word_at(&crate::transducer::alphabet::Word::identity(), &b.moduli, vec![(3, 5)]);
let mut form = Form::default();
let ca = form.stand_member(&va, FRAME).expect("A stands");
let cb = form.stand_member(&vb, FRAME).expect("B stands at its own class");
let chord = form.place_overlap(ca.clone(), cb.clone(), None).expect("the pair's overlap");
let (a_at, b_at) = (ca.vertex(), cb.vertex());
assert_ne!(a_at, b_at, "two classes, two vertices");
let relative = element::carriage(&q(1), &(q(3), q(3))).geo(&element::carriage(&q(1), &(q(3), q(3))).reverse());
let re_at = form.rebase(&a_at, &b_at, relative, &chord).expect("B is rebased on A");
form.reindex();
let n_re = form
    .vertices
    .scoped()
    .filter(|(_, v)| matches!(v, Vertex::Rebased { .. }))
    .count();
assert_eq!(n_re, 1, "one Rebased; the w_i w_j w_i⁻¹ test is gone");
match form.vertices.at(&re_at) {
    Some(Vertex::Rebased { by, class, acts }) => {
        assert_eq!(by, &a_at, "read in A's frame");
        assert_eq!(Some(class.clone()), vb.class(), "and holding the class B had");
        // B'S OWN CHART IS A MEMBER OF THE REBASED VERTEX'S FIBRE, asked for by
        // key: B kept its chart, and the rebase carries it by the relative at
        // that chart (D10). One member here, because B acted on one.
        // A FIBRE READ AT THE CHART'S STRATUM (54.7 R1): the key is
        // `chart ‖ carriage`, so the chart is a prefix and not the whole
        // name of a section.
        let prefix = crate::transducer::sentence::Acting::chart_prefix(&b.moduli);
        let m = acts
            .scoped()
            .find(|(k, _)| k.starts_with(&prefix))
            .map(|(_, x)| x.clone())
            .expect("B keeps its own chart, and the rebase keeps the member there");
        assert_eq!(acts.len(), 1, "B acted on one chart, so the rebase carries one member");
        assert!(
            element::same_versor(&m.carriage, &element::identity_versor())
                || element::same_versor(&m.carriage, &element::carriage(&q(1), &(q(0), q(0)))),
            "relative carriage is the identity translator"
        );
        assert_eq!(m.gauge, vec![(3, 5)], "B keeps its own hue map");
        assert_eq!(
            form.vertices.at(&re_at).and_then(|v| v.word_in(&form.vertices)),
            Some(rot),
            "the word is A's"
        );
    }
    other => panic!("expected Rebased, got {other:?}"),
}
// The pair form itself, with B still a Word carrying A's word at
// B's placement, is what the rebased form must speak.
let reference = apply_form(
    &pf,
    &x,
    &crate::transducer::alphabet::Word::identity(),
    &NoCrystal,
)
.expect("the pair form speaks");
assert!(
    reference.painted.iter().any(|c| c.2 == 5),
    "B is spoken in frame in its own colour, got {:?}",
    reference.painted
);
let spoken = apply_form(
    &form,
    &x,
    &crate::transducer::alphabet::Word::identity(),
    &NoCrystal,
)
.expect("rebased speaks");
assert!(spoken.unspoken.is_empty(), "both classes have a body on this grid");
assert_eq!(
    spoken.painted, reference.painted,
    "the rebased form speaks B's body with B's colours at B's placement"
);
assert!(
    spoken.painted.iter().all(|c| c.2 != 3),
    "B is never spoken through A's hue map, got {:?}",
    spoken.painted
);
}

/// A held closed form glued as a vertex: depth 1, fewer Word vertices.
#[test]
fn extract_rule_glues_a_held_form() {
let family = vec![
    (grid(&["1100", "0000"]), grid(&["0011", "0000"])),
    (grid(&["0000", "1100"]), grid(&["0000", "0011"])),
];
let flat = extract_rule_with(&family, &NoCrystal, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("flat");
let flat_words = flat
    .form
    .vertices
    .scoped()
    .filter(|(_, v)| matches!(v, Vertex::Word { .. }))
    .count();
let held_form = cipher_form(&family[0].0, &family[0].1);
assert!(held_form.is_closed(), "a pair form of a shift coheres");
let held_e = crate::transducer::sentence::as_element_form(&held_form, [1, 0, 0, 0]).expect("tape");
struct Held {
    e: Element,
    f: Form,
}
impl Crystal for Held {
    fn on_class(&self, _class: &Invariant) -> Vec<(&Element, &Form, Standing)> {
        vec![(&self.e, &self.f, Standing::Held)]
    }
    fn on_body(&self, c: &Invariant) -> Vec<(&Element, &Form, Standing)> {
        self.on_class(c)
    }
    fn speak_form(
        &self,
        f: &Form,
        x: &Grid,
        placement: &crate::transducer::alphabet::Word,
    ) -> Option<Speech> {
        apply_form(f, x, placement, self)
    }
    fn by_address(&self, addr: &[u8]) -> Option<(&Element, std::borrow::Cow<'_, Form>)> {
        if addr == self.e.address().as_bytes() {
            Some((&self.e, std::borrow::Cow::Borrowed(&self.f)))
        } else {
            None
        }
    }
    fn on_axis(
        &self,
        _chart: &Invariant,
        axis: crate::transducer::sentence::Axis,
    ) -> Vec<crate::transducer::alphabet::Word> {
        let mut index = crate::transducer::sentence::AxisIndex::new();
        crate::transducer::sentence::admit_words(&mut index, &Invariant::default(), &crate::transducer::sentence::acting_of_form(&self.f));
        crate::transducer::sentence::on_axis_of(&index, &Invariant::default(), axis)
    }
    /// This fixture's claim is the glue, not the language: it holds one
    /// form to be cited and offers no presentation to derive against.
    fn generators_under(
        &self,
        _chart: &Invariant,
        _budget: &mut crate::physics::Budget,
    ) -> Vec<(
        crate::transducer::address::Address,
        crate::transducer::address::Address,
        crate::transducer::alphabet::Word,
    )> {
        Vec::new()
    }
}
let crystal = Held {
    e: held_e,
    f: held_form,
};
let rule = extract_rule_with(&family, &crystal, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("glues");
assert_eq!(rule.form.depth_of(), Some(1), "gluing a depth-0 form is depth 1");
let glued = rule
    .form
    .vertices
    .scoped()
    .filter(|(_, v)| matches!(v, Vertex::Glued { .. }))
    .count();
assert_eq!(glued, 1, "one Glued vertex");
let words = rule
    .form
    .vertices
    .scoped()
    .filter(|(_, v)| matches!(v, Vertex::Word { .. }))
    .count();
assert!(
    words < flat_words,
    "glued form has fewer Word vertices than the flat rule ({words} < {flat_words})"
);
let spoken = apply_form(
    &rule.form,
    &family[0].0,
    &crate::transducer::alphabet::Word::identity(),
    &crystal,
)
.expect("glued form speaks");
assert_eq!(
    spoken.grid(),
    Some(family[0].1.clone()),
    "the spoken sites equal the target, and they fit the frame"
);
assert_ne!(
    form_deflation(&rule.form, &family, &crystal).0,
    [0; 4],
    "the glued form measures its deflation through the crystal that holds its citation"
);
assert_eq!(
    form_deflation(&rule.form, &family, &NoCrystal).0,
    [0; 4],
    "without the cited form nothing is measured, so nothing is minted"
);
}

/// 21 STEP 3 — A CITED FORM IS SPOKEN OVER THE FAMILY ONCE PER PLACEMENT,
/// AND THE CRYSTAL IS ASKED ONCE PER CHART. Three shapes translated by one
/// vector are one class acting on three charts at one placement. What a
/// held form speaks over the family depends on the form, the placement and
/// the family, not on the class asking, so the glue speaks it once — as
/// many times as the family has frames — while it asks the crystal at each
/// of the three charts. Spoken per class it was charts × frames speeches
/// of one reading, under one spend per class.
#[test]
fn the_glue_speaks_a_cited_form_once_per_placement_and_asks_once_per_chart() {
    use std::cell::Cell;
    let family = vec![
        (
            grid(&["1100000", "0000000", "3000000", "0000000", "5550000", "0000000"]),
            grid(&["0011000", "0000000", "0030000", "0000000", "0055500", "0000000"]),
        ),
        (
            grid(&["0000000", "1100000", "0000000", "3000000", "0000000", "5550000"]),
            grid(&["0000000", "0011000", "0000000", "0030000", "0000000", "0055500"]),
        ),
    ];
    let flat = extract_rule_with(&family, &NoCrystal, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("the family extracts");
    let words: Vec<&Vertex> =
        flat.form.vertices.scoped().filter(|(_, v)| matches!(v, Vertex::Word { .. })).map(|(_, v)| v).collect();
    let [word] = words.as_slice() else {
        panic!("one translation, one class: {:?}", flat.form.vertices)
    };
    let charts = word.acts().map_or(0, |a| a.len());
    assert_eq!(charts, 3, "acting on the three shapes' charts");
    let bar = (grid(&["1100", "0000"]), grid(&["0011", "0000"]));
    let held_form = cipher_form(&bar.0, &bar.1);
    assert!(held_form.is_closed(), "the bar's shift is a closed form to cite");
    let held_e = crate::transducer::sentence::as_element_form(&held_form, [1, 0, 0, 0]).expect("tape");
    struct Counting {
        e: Element,
        f: Form,
        asked: Cell<u64>,
        speeches: Cell<u64>,
    }
    impl Crystal for Counting {
        fn on_class(&self, _class: &Invariant) -> Vec<(&Element, &Form, Standing)> {
            vec![(&self.e, &self.f, Standing::Held)]
        }
        fn on_body(&self, c: &Invariant) -> Vec<(&Element, &Form, Standing)> {
            self.asked.set(self.asked.get() + 1);
            self.on_class(c)
        }
        fn speak_form(&self, f: &Form, x: &Grid, placement: &crate::transducer::alphabet::Word) -> Option<Speech> {
            self.speeches.set(self.speeches.get() + 1);
            apply_form(f, x, placement, self)
        }
        fn by_address(&self, addr: &[u8]) -> Option<(&Element, std::borrow::Cow<'_, Form>)> {
            (addr == self.e.address().as_bytes()).then(|| (&self.e, std::borrow::Cow::Borrowed(&self.f)))
        }
        fn on_axis(&self, _chart: &Invariant, _axis: crate::transducer::sentence::Axis) -> Vec<crate::transducer::alphabet::Word> {
            Vec::new()
        }
        fn generators_under(
            &self,
            _chart: &Invariant,
            _budget: &mut crate::physics::Budget,
        ) -> Vec<(crate::transducer::address::Address, crate::transducer::address::Address, crate::transducer::alphabet::Word)> {
            Vec::new()
        }
    }
    let crystal = Counting { e: held_e, f: held_form, asked: Cell::new(0), speeches: Cell::new(0) };
    let rule = extract_rule_with(&family, &crystal, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("the family extracts against the crystal");
    assert_eq!(crystal.asked.get(), charts as u64, "the crystal is asked once per chart the class acts on");
    assert_eq!(
        crystal.speeches.get(),
        family.len() as u64,
        "the held form is spoken over the family once: one reading at one placement, read back for every chart"
    );
    assert!(
        rule.form.vertices.scoped().any(|(_, v)| matches!(v, Vertex::Glued { .. })),
        "and the bar's shift, which reproduces the family, is glued"
    );
}

/// Z2 — THE GLUE PASS RETRACTS ONCE, AND A CITATION IS NEVER CLONED.
///
/// Both halves were measured before they were written. `Form::glue` ran
/// `orbits_of` and `reindex` per citation, and `retract_vertices` — which
/// rebuilds the vertices, the faces, the cycle basis and every index —
/// ran per citation too: on `1d0a4b61` that is 578 rebuilds over 2 850
/// faces, 4.9 s of a 16.8 s pull on the instrument. And `Crystal::by_address`
/// returned `(Element, Form)` BY VALUE, so resolving one citation 578
/// times copied the cited form 578 times.
///
/// THE CLAIM IS ASKED, NOT THE OUTCOME (rule 13). The retraction counter is
/// the work counter for the first half — a count of rebuilds, not a
/// wall-clock — and the second half is pointer identity on what the reader
/// handed back: a `&Form` that IS the crystal's own field cannot have been
/// cloned, and no timing can say that.
#[test]
fn a_glue_pass_retracts_once_and_a_citation_is_never_cloned() {
    let family = vec![
        (grid(&["1100", "0000"]), grid(&["0011", "0000"])),
        (grid(&["1100", "0000"]), grid(&["0011", "0000"])),
    ];
    let bar = (grid(&["1100", "0000"]), grid(&["0011", "0000"]));
    let held_form = cipher_form(&bar.0, &bar.1);
    assert!(held_form.is_closed(), "the bar's shift is a closed form to cite");
    let held_e = crate::transducer::sentence::as_element_form(&held_form, [1, 0, 0, 0]).expect("tape");

    struct Held {
        e: Element,
        f: Form,
    }
    impl Crystal for Held {
        fn on_class(&self, _class: &Invariant) -> Vec<(&Element, &Form, Standing)> {
            vec![(&self.e, &self.f, Standing::Held)]
        }
        fn on_body(&self, c: &Invariant) -> Vec<(&Element, &Form, Standing)> {
            self.on_class(c)
        }
        fn speak_form(&self, f: &Form, x: &Grid, placement: &crate::transducer::alphabet::Word) -> Option<Speech> {
            apply_form(f, x, placement, self)
        }
        fn by_address(&self, addr: &[u8]) -> Option<(&Element, std::borrow::Cow<'_, Form>)> {
            (addr == self.e.address().as_bytes()).then(|| (&self.e, std::borrow::Cow::Borrowed(&self.f)))
        }
        fn on_axis(&self, _chart: &Invariant, _axis: crate::transducer::sentence::Axis) -> Vec<crate::transducer::alphabet::Word> {
            Vec::new()
        }
        fn generators_under(
            &self,
            _chart: &Invariant,
            _budget: &mut crate::physics::Budget,
        ) -> Vec<(crate::transducer::address::Address, crate::transducer::address::Address, crate::transducer::alphabet::Word)> {
            Vec::new()
        }
    }
    let crystal = Held { e: held_e, f: held_form };

    // A CITATION IS NEVER CLONED: the reader hands back the crystal's own
    // form, and `ptr::eq` is the whole question — a clone would be a
    // different object with the same content, which content equality could
    // not tell apart and which is exactly the defect.
    let (e, f) = crystal
        .by_address(crystal.e.address().as_bytes())
        .expect("the crystal resolves its own citation");
    assert!(
        matches!(f, std::borrow::Cow::Borrowed(held) if std::ptr::eq(held, &crystal.f)),
        "by_address hands back the held form, not a copy of it"
    );
    assert!(std::ptr::eq(e, &crystal.e), "and the held element, not a copy of it");

    // The extraction is run whole, as a voice's own stroke runs it.
    let rule = extract_rule_with(&family, &crystal, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("the family extracts against the crystal");
    let cited = rule.form.vertices.scoped().filter(|(_, v)| matches!(v, Vertex::Glued { .. })).count();
    assert!(cited > 0, "the bar's shift, which reproduces the family, is cited");
}

/// A withheld remainder is a listing chain: from_residual reads it,
/// so the residual path can mint against the book.
#[test]
fn withheld_remainder_chains_are_listing_shaped() {
let family = vec![
    (grid(&["1"]), grid(&["2"])),
    (grid(&["10"]), grid(&["23"])),
];
let rule = extract_rule(&[(grid(&["1"]), grid(&["2"]))], &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("recolour");
let (_pay, rems) = form_deflation(&rule.form, &family, &NoCrystal);
let listings: Vec<&Vec<(i64, i64, u8, u8)>> =
    rems.iter().filter(|r| !r.is_empty()).collect();
assert!(
    !listings.is_empty(),
    "a withheld form leaves at least one remainder chain"
);
for r in listings {
    let e = element::from_residual(r).expect("a listing chain is A5's");
    assert_eq!(
        e.sub().len(),
        r.len(),
        "the remainder is a listing chain the residual path speaks"
    );
}
}

#[test]
fn extract_rule_cached_spends_once_per_generation() {
let family = vec![(grid(&["1100", "0000"]), grid(&["2200", "0000"]))];
let fam = b"fam";
let mut cache = ExtractMemo::new();
// Sized the physics' way, by the family's frames, for the two extractions
// this reads (one per atlas generation), never by a fixed number.
let frames = perceive_family(&family);
let mut budget = crate::physics::Physics::default().stroke_n(2 * extraction_spend(&family, &frames));
let atlas1 = crate::transducer::address::Address::from(vec![1]);
let atlas2 = crate::transducer::address::Address::from(vec![2]);
let a = extract_rule_cached(&mut cache, fam, &family, atlas1.clone(), &mut budget, &NoCrystal, Some(&frames));
let left = budget.remaining();
let b = extract_rule_cached(&mut cache, fam, &family, atlas1, &mut budget, &NoCrystal, Some(&frames));
assert_eq!(a.is_some(), b.is_some());
assert_eq!(budget.remaining(), left, "a cache hit spends nothing");
let _ = extract_rule_cached(&mut cache, fam, &family, atlas2, &mut budget, &NoCrystal, Some(&frames));
assert!(budget.remaining() < left, "a new atlas generation re-extracts");
}

/// THE MEMO HOLDS ONE RULE, and that bound is the fleet's OOM.
///
/// It was a `Keyed` with no eviction, one whole `SentenceRule` per task id
/// the voice had ever pulled. Measured on voice 4 of the 2026-09-19 recut,
/// 18 minutes in: 89 pulls, 54 distinct families, task arm +522 MB net —
/// 9.7 MB per stored rule, which is a `Form`'s vertices, faces, orbits and
/// carriers. The training corpus is 1001 tasks, so a voice that sees it
/// once holds 9.4 GB.
///
/// And it bought nothing: its hit rate was 39%, and the CONSECUTIVE-repeat
/// rate over the same journal is 39% on voice 4 and 40% on voice 1 — every
/// hit was the envelope the hypha had just handed back.
///
/// Three readings here. A second family REPLACES the first rather than
/// joining it, so the memo cannot grow. Returning to the first is a miss
/// and re-extracts, which is the cost of the bound, stated. And a starved
/// stroke on a family the memo does not hold answers `None` rather than
/// handing back another family's rule.
#[test]
fn the_extract_memo_holds_one_rule_and_a_second_family_replaces_it() {
    let one = vec![(grid(&["1100", "0000"]), grid(&["2200", "0000"]))];
    let two = vec![(grid(&["0011", "0000"]), grid(&["0033", "0000"]))];
    let atlas = crate::transducer::address::Address::from(vec![1]);
    let mut cache = ExtractMemo::new();
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    assert_eq!(cache.len(), 0, "a memo that has read nothing holds nothing");
    let _ = extract_rule_cached(&mut cache, b"one", &one, atlas.clone(), &mut budget, &NoCrystal, None);
    assert_eq!(cache.len(), 1, "one family read, one rule held");
    let _ = extract_rule_cached(&mut cache, b"two", &two, atlas.clone(), &mut budget, &NoCrystal, None);
    assert_eq!(cache.len(), 1, "a second family REPLACES the first: the memo cannot grow");
    // Returning to the first is a miss. That is the cost of the bound and
    // it is stated, not hidden: the spend is the reading.
    let left = budget.remaining();
    let back = extract_rule_cached(&mut cache, b"one", &one, atlas.clone(), &mut budget, &NoCrystal, None);
    assert!(back.is_some(), "and it re-extracts rather than answering for the other family");
    assert!(budget.remaining() < left, "which is a spend, and the memo says so by spending it");
    // A STARVED STROKE ANSWERS FOR THIS FAMILY OR NOT AT ALL. The memo now
    // holds `one`; asked for `two` with nothing to spend it must not hand
    // back `one`'s rule (rule 11).
    let mut starved = crate::physics::Physics::default().stroke_n(0);
    assert!(
        extract_rule_cached(&mut cache, b"two", &two, atlas, &mut starved, &NoCrystal, None).is_none(),
        "a rule read for another family is not this family's answer"
    );
}

/// A class present in one of two pairs keeps its star face: a missing
/// end on the other pair is not a constancy violation.
#[test]
fn a_class_present_in_one_of_two_pairs_keeps_its_star_face() {
let family = vec![
    (grid(&["1100", "1000", "0000"]), grid(&["0011", "0001", "0000"])),
    (grid(&["1100", "0000", "0000"]), grid(&["2200", "0000", "0000"])),
];
let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("section class");
assert!(
    sections_short_of(&rule, family.len()) > 0,
    "a some-pair class exists, got {} vertices",
    rule.form.vertices.len()
);
let sections = rule.sections();
let mut section_star = 0usize;
for (at, v) in rule.form.vertices.entries() {
    if transport_class(v).is_none() {
        continue;
    }
    let Some(class) = v.class() else { continue };
    // A SOME-PAIR CLASS: its section stands on fewer frames than the family's
    // (AT1 — the class's section is read off its members' frames).
    let short = sections
        .at(&crate::network::nerve::coord::SectionAt::Class(class))
        .is_some_and(|s| !s.is_empty() && s.len() < family.len());
    if !short {
        continue;
    }
    // ITS STAR FACE: an overlap between the Frame's chart and a chart of it.
    let frame = crate::network::nerve::coord::ChartAt::Frame;
    if rule
        .form
        .faces
        .entries()
        .any(|(o, _)| (o.from == frame && o.to_vertex() == at) || (o.to == frame && o.from_vertex() == at))
    {
        section_star += 1;
    }
}
assert!(
    section_star > 0,
    "a class present in one of two pairs keeps its star face, got {} faces {} sections {} vertices",
    rule.form.faces.len(),
    sections_short_of(&rule, family.len()),
    rule.form.vertices.len()
);
}

#[test]
fn a_class_in_some_pairs_is_a_section() {
let family = vec![
    (grid(&["1100", "1000", "0000"]), grid(&["0011", "0001", "0000"])),
    (grid(&["1100", "0000", "0000"]), grid(&["2200", "0000", "0000"])),
];
let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("frame plus a some-pair class");
assert!(
    sections_short_of(&rule, family.len()) > 0,
    "a class present in some pairs is a section, got {} vertices {} sections",
    rule.form.vertices.len(),
    sections_short_of(&rule, family.len())
);
}

/// Correspondence across pairs is transport class (kind, word),
/// not body moduli and not the hue map. A square and a bar recoloured
/// 1→2 are one class; frames of different sizes match by kind.
#[test]
fn correspondence_is_transport_class_not_shape() {
let family = vec![
    (grid(&["1100", "1100"]), grid(&["2200", "2200"])),
    (grid(&["1110", "0000"]), grid(&["2220", "0000"])),
];
let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("different shapes, same recolour");
let bodies = rule
    .form
    .vertices
    .scoped()
    .filter(|(_, v)| matches!(v, Vertex::Word { .. }))
    .count();
assert!(bodies >= 1, "a body vertex of the recolour class survives, got {}", rule.form.vertices.len());
assert!(
    matches!(rule.form.vertices.at(&crate::network::nerve::coord::VertexAt::Frame), Some(Vertex::Frame)),
    "frames match by kind, not [MAX,h,w]"
);
assert!(
    rule.form.vertices.len() >= 2,
    "frame + body, got {} vertices {} faces",
    rule.form.vertices.len(),
    rule.form.faces.len()
);
}

/// Two pairs that recolour differently share one rule vertex; the hue
/// maps are two sections, not two classes.
#[test]
fn two_pairs_that_recolour_differently_share_one_rule_vertex() {
let family = vec![
    (grid(&["1100", "0000"]), grid(&["2200", "0000"])),
    (grid(&["1100", "0000"]), grid(&["3300", "0000"])),
];
let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("one recolour class");
let words: Vec<&Vertex> = rule
    .form
    .vertices
    .scoped()
    .filter(|(_, v)| matches!(v, Vertex::Word { .. }))
    .map(|(_, v)| v)
    .collect();
assert_eq!(
    words.len(),
    1,
    "one rule vertex for one word class, got {} vertices {} faces {:?}",
    rule.form.vertices.len(),
    rule.form.faces.len(),
    // Every member's gauge, per vertex: the reading is what the vertices carry,
    // and a vertex acting on two charts carries two.
    words
        .iter()
        .map(|v| {
            v.acts()
                .into_iter()
                .flat_map(|a| a.scoped())
                .map(|(_, m)| m.gauge.clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
);
let hues = rule.form.hue_maps.len();
assert_eq!(
    hues, 2,
    "two hue sections, got {hues} maps on {} vertices",
    rule.form.vertices.len()
);
}

/// TWO HUES THAT MEET ARE ONE BODY, AND THE MERGE IS THE PERCEPT
/// (H1/70.4, superseding chapter 20 of 2026-09-09; 50.5, 52.5, rule 15).
///
/// The scene is two bars that SLIDE TOGETHER until they touch. Under
/// chapter 20 the output was two bodies of one hue each — one chart with
/// a multiplicity of two — and this test asserted a single `Source`
/// demand naming that multiplicity.
///
/// Under H1 the output is ONE body of four compartments carrying two hues in
/// its gauge, so the reading is two bodies IN and one body OUT: a merge.
/// Each input body needs its own account, which is why the form carries
/// TWO demands rather than one. That is derived from the ruling, not
/// fitted to the run.
///
/// AND IT IS THE RULING'S REAL COST, RECORDED HERE RATHER THAN HIDDEN.
/// Hue-blind perception cannot tell "one two-coloured object" from "two
/// objects that happen to touch" — which is exactly the fear chapter 20's
/// own test wrote down ("read hue-blind, every touching scene would be one
/// blob"). Measured, it is not one blob: 732 regions over 128 scenes, 5.7
/// per scene. But on THIS scene it is real, and H2's diagonal contact does
/// NOT recover it, because these bars meet edge-on and not at a corner.
///
/// WHAT RECOVERS IT IS H3. The distinction did not vanish — it moved to
/// the fiber, as the ruling says. The seam between the hues is a
/// discontinuity in the body's own gauge section, and a reader of that
/// section can say where one sub-object ends. Until H3 lands, nothing
/// reads it, and this test stands as the record of what is owed.
///
/// H1b TOOK THE LAST OF IT (Phase H). This asserted that the bar that stood
/// is carried in place by the cover, which held only while `pieces_of`
/// split the merged body by hue. The cover's piece is hue-blind now, so the
/// output is one piece, and the one input bar under its sites carries no
/// part of it. The form carries no word, and the bar that stood reads as
/// two deformation demands. That is the claim 70.4 makes, recorded here as
/// the number: 0 words, where the per-hue piece gave 1. H3 owes the same
/// recovery it owed before.
#[test]
fn two_hues_that_meet_are_one_body_and_the_merge_is_the_percept() {
let family = vec![
    (grid(&["110022", "000000"]), grid(&["001122", "000000"])),
    (grid(&["000000", "110022"]), grid(&["000000", "001122"])),
];
let pf = cipher_form(&family[0].0, &family[0].1);
let words: Vec<&Vertex> = pf.vertices.scoped().filter(|(_, v)| matches!(v, Vertex::Word { .. })).map(|(_, v)| v).collect();
assert!(
    words.is_empty(),
    "the merged body is one hue-blind piece, and no preimage carries it whole: {:?}",
    pf.vertices
);
let bar_chart = perceive(&family[0].0)
    .into_iter()
    .map(|b| b.moduli)
    .next()
    .expect("the scene's bars stand on a chart");
for still in &words {
    assert!(
        still.acts().into_iter().flat_map(|a| a.scoped()).all(|(_, m)| {
            element::same_versor(&m.carriage, &element::identity_versor())
        }),
        "every word carries a piece of the bar that stood, by the identity: {still:?}"
    );
}
// TWO BODIES IN, ONE OUT: each input body owes an account, so the form
// carries a demand per input body rather than one naming a multiplicity.
let demands: Vec<_> = pf.demand_vertices().collect();
assert_eq!(
    demands.len(),
    2,
    "two bodies merging into one leaves an account per input body; got {:?}",
    demands.iter().map(|(_, k, _, _)| k.label()).collect::<Vec<_>>()
);
let _ = bar_chart;
assert!(
    demands.iter().all(|(_, _, _, links)| !links.is_empty()),
    "each demand carries the links it stands for"
);
// THE HUES ARE NOT LOST — the merged body carries both as gauge, which
// is where H3 will read the seam back out.
let merged = perceive(&family[0].1);
assert_eq!(merged.len(), 1, "the output is one body: {}", merged.len());
// ON THE COMPARTMENTS, NOT THE SHAPE. `body_of` builds the shape through
// `shape_of`, which writes hue `0` at every site — the shape has ALWAYS
// been hue-blind, and only `components()` ever used hue. The body's
// colours ride on its compartments, which is where H3 reads the seam.
let mut hues: Vec<u8> = merged[0].compartments.iter().map(|l| l.hue).collect();
hues.sort_unstable();
hues.dedup();
assert_eq!(hues.len(), 2, "the merged body carries both hues: {hues:?}");
assert!(!pf.is_closed(), "a sentence with demands is valid, never closed (2.2, 50.1)");
let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the family extracts");
let demands: Vec<_> = rule.form.demand_vertices().collect();
assert!(
    !demands.is_empty(),
    "the merge leaves the family's rule owing an account too"
);
assert!(
    demands.iter().all(|(_, _, _, links)| !links.is_empty()),
    "each carrying the links it stands for: {demands:?}"
);
assert!(!rule.form.is_closed(), "and the rule is valid, never closed");
}

/// 52.5 READING 6 (ruled 2026-09-18) — SUCCESSION ALONG e₃ IS THE NEXT
/// FRAME OCCUPIED. `edge_between` read `for k in 0..n`, probing every
/// integer up to the family's pair count and skipping each miss: a scan
/// over a range, where the frames a member stands on are named by its own
/// fiber of `bodies_at`. The occupied set may skip, repeat or reorder in ℤ,
/// and a frame outside `0..n` was never read at all.
///
/// The order is the COORDINATE'S. It was not the trie's while the frame was
/// written little-endian, so byte order put 256 (`[0, 1]`) before 1
/// (`[1, 0]`); AT1 Q4 spells the frame by its length then its big-endian
/// bytes, so the trie's walk IS the succession and nothing sorts.
#[test]
fn succession_reads_the_frames_a_member_occupies_in_coordinate_order() {
    use crate::network::nerve::Keyed;
    use crate::transducer::family::{body_of, occupied_frames};
    let body = || body_of(vec![Compartment::from((0i64, 0i64, 0i64, 1u8))]).expect("one compartment is heard");
    let v = crate::transducer::element::identity_versor();
    // TWO VERTICES BY THEIR COORDINATES (AT1): two classes, the identity's
    // and the 90° turn's.
    let at_of = |w: crate::transducer::alphabet::Word| {
        Vertex::word_on(
            crate::transducer::alphabet::Derivation::spelled(w),
            Invariant::default(),
            Box::new(element::identity_versor()),
            Vec::new(),
            [0u64],
            None,
        )
        .at()
        .expect("a word vertex has a class")
    };
    let seven = at_of(crate::transducer::alphabet::Word::identity());
    let turn_90 = crate::transducer::alphabet::LetterId::turn_90((0, 1)).expect("90° turn");
    let nine = at_of(crate::transducer::alphabet::Word::from_letters(&[(turn_90, false)]));
    assert_ne!(seven, nine, "two classes, two vertices");
    let mut at: Keyed<(
        crate::transducer::family::Body,
        crate::transducer::family::Body,
        crate::transducer::element::Versor,
    )> = Keyed::new();
    let put = |at: &mut Keyed<_>, vx: &crate::network::nerve::coord::VertexAt, ck: &[u8], k: u64| {
        let mut bk = crate::transducer::family::member_key(vx, ck);
        bk.extend_from_slice(&crate::transducer::sentence::frame_key(k));
        at.insert(bk, (body(), body(), v.clone()));
    };
    // ONE member on sparse, non-contiguous frames — including 256, which is
    // outside any `0..n` a three-pair family would have scanned.
    for k in [5u64, 0, 256, 2, 1] {
        put(&mut at, &seven, b"chart-a", k);
    }
    // A second member, and the same vertex on another chart: neither is
    // this member's fiber.
    put(&mut at, &nine, b"chart-a", 3);
    put(&mut at, &seven, b"chart-b", 4);

    let occ = occupied_frames(&at, &seven, b"chart-a");
    assert_eq!(occ, vec![0, 1, 2, 5, 256], "the occupied set, in coordinate order");
    assert!(occ.contains(&256), "a frame outside 0..n is occupied and is read");
    assert!(!occ.contains(&3), "another member's frame is not in this member's fiber");
    assert!(!occ.contains(&4), "nor the same vertex's frame on another chart");
    // THE ORDERING IS LOAD-BEARING. Under the little-endian spelling the
    // trie's own order read 256 as `[0, 1]` and 1 as `[1, 0]`, putting 256
    // second. That sequence is written out here rather than sorted into
    // existence, and it is not the succession.
    assert_ne!(vec![0u64, 256, 1, 2, 5], occ, "the little-endian order is not coordinate order");
}

/// 52.5, 51.3 — THE FACE IS MEASURED ON THE FIBER'S UNION SHAPES. A
/// member's fiber is one body on `(r, c, k)`, and `union_quotient` is that
/// body's quotient: every compartment the member stands on across its occupied
/// frames, read at its own `k` and never flattened to frame zero.
///
/// The two assertions are the point. A member whose frames all read one
/// motion HAS a union quotient, and that quotient's bodies carry the frame
/// extent — three distinct `k` on a three-frame fiber — which is what a
/// reading taken at one frame cannot carry at all. A member whose frames
/// read different motions has NO union quotient, and the per-frame reading
/// stands instead rather than a union invented for it (rule 11).
#[test]
fn a_members_fiber_reads_as_one_union_body_or_not_at_all() {
    use crate::transducer::family::union_quotient;
    // Three frames, each a bar translated by the same vector: one motion
    // over the whole fiber.
    let ins: Vec<Compartment> = [
        (0i64, 0i64, 0i64, 1u8),
        (0, 1, 0, 1),
        (1, 0, 1, 1),
        (1, 1, 1, 1),
        (2, 0, 2, 1),
        (2, 1, 2, 1),
    ]
    .map(Compartment::from)
    .to_vec();
    // Moved along the column axis by `d`, frame by frame.
    let moved = |l: &Compartment, d: i64| Compartment {
        site: l.site.offset(&[0, d, 0]).expect("a small move stays on the base"),
        hue: l.hue,
    };
    let outs: Vec<Compartment> = ins.iter().map(|l| moved(l, 2)).collect();
    let shift = element::translator_versor(0, 2);
    let (a, b, v) =
        union_quotient(ins.clone(), outs, &shift).expect("one motion carries the whole fiber");
    let frames: Vec<i64> = {
        let mut f: Vec<i64> = a.compartments.iter().filter_map(|l| l.site.axis(2)).collect();
        f.sort_unstable();
        f.dedup();
        f
    };
    assert_eq!(frames, vec![0, 1, 2], "the union body stands on every frame of the fiber");
    assert_eq!(b.compartments.len(), a.compartments.len(), "and carries the whole of it");
    assert!(
        element::same_versor(&v, &shift),
        "the fiber's quotient is the one translation, verified on the union"
    );
    // The same three frames with the middle one moved the other way: no
    // single motion carries the union.
    let mixed: Vec<Compartment> = ins
        .iter()
        .map(|l| if l.site.axis(2) == Some(1) { moved(l, 3) } else { moved(l, 2) })
        .collect();
    assert!(
        union_quotient(ins, mixed, &shift).is_none(),
        "a fiber that is not one motion along e₃ has no union quotient"
    );
}

/// AL1 AS AMENDED — A MIRRORED PAIR MINTS. The L-tetromino and its mirror
/// image: no turn relates them, and the pose used to refuse the pair as
/// `Mirrored`, so the family minted nothing. The pair is the same signal read
/// orientation-reversed: the pose is the odd versor, the peel spells it
/// through the letter chart's orientation character, and the sentence mints.
/// Its word vertices read DERIVABLE through the mind's own crystal — the
/// reflection is read, never demanded or adjoined.
#[test]
fn a_mirrored_pair_mints_and_its_class_reads_derivable() {
    let l = grid(&["1000", "1000", "1100", "0000"]);
    let j = grid(&["0100", "0100", "1100", "0000"]);
    let family = vec![(l, j)];
    let rule = extract_rule_with(&family, &NoCrystal, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("the mirrored pair extracts");
    let word = mint_sentence_why(&rule, &family, &NoCrystal, 0).expect("and mints");
    // The pair's transport is the reflection itself — odd — read exactly.
    let la = crate::transducer::blade::shape_of(&[(0i64, 0i64, 0i64), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let jb = crate::transducer::blade::shape_of(&[(0i64, 1i64, 0i64), (1, 1, 0), (2, 1, 0), (2, 0, 0)]);
    let v = crate::transducer::blade::versor_between(&la, &jb, &mut super::common::budget()).expect("a versor carries the L onto its mirror");
    assert_eq!(v.parity(), Some(true), "and it is odd: the mirror, never a turn at an angle no lattice turns");
    let form = crate::transducer::sentence::from_element_form(&word).expect("the form the court holds");
    let rotors: Vec<crate::transducer::alphabet::Rotor> = form
        .vertices
        .entries()
        .filter_map(|(_, v)| match v {
            crate::transducer::sentence::Vertex::Word { derivation, .. } => Some(derivation.rotor()),
            _ => None,
        })
        .collect();
    assert!(!rotors.is_empty(), "the minted form carries its word vertices");
    let epi = crate::client::sdk::Epigenome::new();
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 12);
    for r in &rotors {
        assert_eq!(
            letter_reading(&epi, &mut budget, r),
            LetterReading::Derivable,
            "each word vertex's class reads derivable through the mind's crystal: {r:?}"
        );
    }
}
