// 21 steps 1-7 — EXTRACTION: the rule a family of train pairs already
// states, read off the pair forms and refused when the family does not
// agree.

use super::common::*;
use crate::transducer::family::*;

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
    .iter()
    .filter(|v| matches!(v, Vertex::Word { .. }))
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
    some.section_count() > 0,
    "a class present in some pairs is a section"
);
}

/// 21 step 2, 25, 12.6, Stage 12 S12.2 — FORMS WITH LOOPS. A body carried
/// by a quarter turn in one pair and by its inverse in the other is one
/// transport class (the two words are conjugate), so the rule has one
/// vertex for it; the face word from the Frame differs per pair and is KEPT
/// as two sections over the pairs, never dropped. The form then has a loop
/// whose holonomy is the transition between the sections — a half turn —
/// held with a class vector the pivot loop reads on its unit axis.
#[test]
fn a_face_that_varies_across_the_pairs_is_kept_as_sections_and_holds_a_loop() {
let l_in = ["100000", "100000", "110000", "000000", "000330", "000000"];
let cw = ["111000", "100000", "000000", "000000", "000550", "000000"];
let ccw = ["001000", "111000", "000000", "000000", "000550", "000000"];
let family = vec![(grid(&l_in), grid(&cw)), (grid(&l_in), grid(&ccw))];
let rule = extract_rule_with(&family, &NoCrystal, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("extracts");
let words = rule
    .form
    .vertices
    .iter()
    .filter(|v| matches!(v, Vertex::Word { .. }))
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
    .iter()
    .filter_map(|v| match v {
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
assert!(rule.section_count() > 0, "the two words of one face are sections over the pairs");
assert!(!rule.form.loops.is_empty(), "the second section closes a loop");
let held: Vec<_> = (0..rule.form.loops.len())
    .filter_map(|i| rule.form.class_vector(i))
    .filter(|cv| !cv.is_zero())
    .collect();
assert!(!held.is_empty(), "a held loop with a class vector");
assert!(
    held.iter().any(|cv| cv.occupied().contains(&crate::transducer::sentence::Axis::Unit(0))),
    "the half turn occupies the unit axis of the sensor plane: {:?}",
    held.iter().map(|c| c.occupied()).collect::<Vec<_>>()
);
let mut form = rule.form.clone();
let mut budget = crate::physics::Physics::default().stroke();
let snaps = form.pivot_loop(b"idea", &crate::network::nerve::Keyed::new(), &NoCrystal, &mut budget, None, 0);
assert!(!snaps.is_empty(), "the pivot loop reads the held loop and answers");
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
        rule.form.vertices.iter().filter(|v| matches!(v, Vertex::Word { .. })).collect();
    let [word] = words.as_slice() else {
        panic!("one word vertex for one translation on every frame: {:?}", rule.form.vertices)
    };
    let carried_by_the_translator = word
        .acts()
        .into_iter()
        .flat_map(|a| a.scoped())
        .all(|(_, m)| element::same_versor(&m.carriage, &element::translator_versor(0, 2)));
    assert!(carried_by_the_translator, "the vertex carries its body by the translation");
    assert_eq!(rule.section_count(), 0, "a rule that holds on every frame is no section: {:?}", rule.sections);
    assert_eq!(rule.form.demand_vertices().count(), 0, "and asks for nothing");
    let (_, rems) = form_deflation(&rule.form, &family, &NoCrystal);
    assert!(rems.iter().all(|r| r.is_empty()), "it closes every frame: {rems:?}");
    let spoken = speak_rule(&rule, &grid(&["0000", "0000", "0000", "1100"]), &NoCrystal).expect("spoken");
    assert_eq!(spoken.grid(), Some(grid(&["0000", "0000", "0000", "0011"])), "and speaks a fourth frame");
}

/// 51.10 PROOF 2, 50.9 — THE SAME ADDRESS ON TWO COURTS. Two extractions
/// of one family with no shared runtime memory — the percept memo cleared
/// between them — reach one form address, and a court replays each minted
/// sentence on its own copy of the family.
#[test]
fn two_courts_reach_one_address_for_one_family() {
    let family = vec![
        (grid(&["1100", "0000"]), grid(&["0011", "0000"])),
        (grid(&["0000", "1100"]), grid(&["0000", "0011"])),
    ];
    crate::transducer::family::extract::clear_percept_memo();
    let first = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("extracts");
    crate::transducer::family::extract::clear_percept_memo();
    let second = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("extracts again");
    assert_eq!(
        crate::transducer::sentence::form_vertex_key(&first.form),
        crate::transducer::sentence::form_vertex_key(&second.form),
        "one family, one address, on two readings that share nothing"
    );
    for rule in [&first, &second] {
        let word = mint_sentence(rule, &family, &NoCrystal, 0).expect("the family mints");
        assert_eq!(
            crate::verdict::blind_replay_on(&word, &|_| None, Some(&family), &NoCrystal),
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
let letter = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
let rot = crate::transducer::alphabet::Word::from_letters(&[(letter, false)]);
// A is the single hue-1 lamina; B is the hue-3 bar, another shape.
// A placement of (3, 3) keeps both images inside the frame for
// either orientation of the quarter turn (laminae map to
// (c, −r−1) + p or (−c−1, r) + p).
let x = grid(&["330000", "000000", "001000", "000000", "000000", "000000"]);
let bodies = perceive(&x);
let a = bodies
    .iter()
    .find(|b| b.laminae.iter().any(|c| c.3 == 1))
    .expect("body A");
let b = bodies
    .iter()
    .find(|b| b.laminae.iter().any(|c| c.3 == 3))
    .expect("body B");
assert_ne!(a.moduli, b.moduli, "two shapes, two charts");
// Form::default() carries the Frame at index 0.
let mut pf = Form::default();
pf.vertices.push(Vertex::word_on(
    crate::transducer::address::Address::from(vec![1]),
    crate::transducer::alphabet::Derivation::spelled(rot.clone()),
    a.moduli.clone(),
    Box::new(element::carriage(&q(1), &(q(3), q(3)))),
    vec![(1, 1)],
    None,
));
pf.vertices.push(Vertex::word_on(
    crate::transducer::address::Address::from(vec![2]),
    crate::transducer::alphabet::Derivation::spelled(rot.clone()),
    b.moduli.clone(),
    Box::new(element::carriage(&q(1), &(q(3), q(3)))),
    vec![(3, 5)],
    None,
));
pf.faces.push(pf.face_of(1, 2).expect("pair vertices"));
let mut form = pf.clone();
let tc_a = transport_class(&pf.vertices[1]).expect("A has a transport class");
let tc_b = transport_class(&pf.vertices[2]).expect("B has a transport class");
assert_eq!(
    tc_a, tc_b,
    "A and B share a transport class: the same word, two hue sections"
);
pf.reindex();
assert_eq!(
    pf.hue_maps.len(),
    2,
    "two hue sections on one word class, got {}",
    pf.hue_maps.len()
);
rebase_named_faces(&mut form);
let n_re = form
    .vertices
    .iter()
    .filter(|v| matches!(v, Vertex::Rebased { .. }))
    .count();
assert_eq!(n_re, 1, "one Rebased; the w_i w_j w_i⁻¹ test is gone");
match &form.vertices[2] {
    Vertex::Rebased { by, of, acts } => {
        assert_eq!((*by, *of), (1, 2));
        // B'S OWN CHART IS A MEMBER OF THE REBASED VERTEX'S FIBRE, asked for by
        // key: B kept its chart, and the rebase carries it by the relative at
        // that chart (D10). One member here, because B acted on one.
        let m = acts
            .get(b.moduli.key().as_bytes())
            .expect("B keeps its own chart, and the rebase keeps the member there");
        assert_eq!(acts.len(), 1, "B acted on one chart, so the rebase carries one member");
        assert!(
            element::same_versor(&m.carriage, &element::identity_versor())
                || element::same_versor(&m.carriage, &element::carriage(&q(1), &(q(0), q(0)))),
            "relative carriage is the identity translator"
        );
        assert_eq!(m.gauge, vec![(3, 5)], "B keeps its own hue map");
        assert_eq!(form.vertices[2].word_in(&form), Some(rot), "the word is A's");
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
    .iter()
    .filter(|v| matches!(v, Vertex::Word { .. }))
    .count();
let held_form = one_pair_form(&family[0].0, &family[0].1);
assert!(held_form.is_closed(), "a pair form of a shift coheres");
let held_e = crate::transducer::sentence::as_element_form(&held_form, [1, 0, 0, 0]).expect("tape");
struct Held {
    e: Element,
    f: Form,
}
impl Crystal for Held {
    fn on_class(&self, _class: &Invariant) -> Vec<(Element, Form, Standing)> {
        vec![(self.e.clone(), self.f.clone(), Standing::Held)]
    }
    fn on_body(&self, c: &Invariant) -> Vec<(Element, Form, Standing)> {
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
    fn by_address(&self, addr: &[u8]) -> Option<(Element, Form)> {
        if addr == self.e.address().as_bytes() {
            Some((self.e.clone(), self.f.clone()))
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
assert_eq!(rule.form.depth_of(), 1, "gluing a depth-0 form is depth 1");
let glued = rule
    .form
    .vertices
    .iter()
    .filter(|v| matches!(v, Vertex::Glued { .. }))
    .count();
assert_eq!(glued, 1, "one Glued vertex");
let words = rule
    .form
    .vertices
    .iter()
    .filter(|v| matches!(v, Vertex::Word { .. }))
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
let mut cache = crate::network::nerve::Keyed::new();
let mut budget = crate::physics::Physics::default().stroke_n(8);
let atlas1 = crate::transducer::address::Address::from(vec![1]);
let atlas2 = crate::transducer::address::Address::from(vec![2]);
let a = extract_rule_cached(&mut cache, fam, &family, atlas1.clone(), &mut budget, &NoCrystal);
let left = budget.remaining();
let b = extract_rule_cached(&mut cache, fam, &family, atlas1, &mut budget, &NoCrystal);
assert_eq!(a.is_some(), b.is_some());
assert_eq!(budget.remaining(), left, "a cache hit spends nothing");
let _ = extract_rule_cached(&mut cache, fam, &family, atlas2, &mut budget, &NoCrystal);
assert!(budget.remaining() < left, "a new atlas generation re-extracts");
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
    rule.section_count() > 0,
    "a some-pair class exists, got {} vertices",
    rule.form.vertices.len()
);
let mut section_star = 0usize;
for (i, v) in rule.form.vertices.iter().enumerate() {
    let Some(tc) = transport_class(v) else {
        continue;
    };
    if rule.sections.get(&tc.key()).is_none() {
        continue;
    }
    let ii = i as u16;
    if rule
        .form
        .faces
        .iter()
        .any(|f| (f.i == 0 && f.j == ii) || (f.j == 0 && f.i == ii))
    {
        section_star += 1;
    }
}
assert!(
    section_star > 0,
    "a class present in one of two pairs keeps its star face, got {} faces {} sections {} vertices",
    rule.form.faces.len(),
    rule.section_count(),
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
    rule.section_count() > 0,
    "a class present in some pairs is a section, got {} vertices {} sections",
    rule.form.vertices.len(),
    rule.section_count()
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
    .iter()
    .filter(|v| matches!(v, Vertex::Word { .. }))
    .count();
assert!(bodies >= 1, "a body vertex of the recolour class survives, got {}", rule.form.vertices.len());
assert!(
    matches!(rule.form.vertices.as_slice(), [Vertex::Frame, ..]),
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
    .iter()
    .filter(|v| matches!(v, Vertex::Word { .. }))
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

/// TWO HUES THAT MEET ARE ONE CHART'S MULTIPLICITY (chapter 20 as ruled
/// 2026-09-09, 50.5, 52.5, rule 15). Two bars of different hues that touch
/// in the output are one chart — a single-hue two-lamina bar — with two
/// instances at the frame, one translated and one still: no quotient
/// carries the chart's fiber there, and no instance is paired with another
/// by a search. The cover's pullback carries the bar that stood, in place,
/// as a word vertex; the bar that moved is the multiplicity the chart cannot
/// resolve — a `Source` demand of two, every one standing and none
/// first-found — and the form is valid, never closed. The family's rule
/// reads the same at both frames.
#[test]
fn two_hues_that_meet_are_one_charts_multiplicity() {
let family = vec![
    (grid(&["110022", "000000"]), grid(&["001122", "000000"])),
    (grid(&["000000", "110022"]), grid(&["000000", "001122"])),
];
let pf = one_pair_form(&family[0].0, &family[0].1);
let words: Vec<&Vertex> = pf.vertices.iter().filter(|v| matches!(v, Vertex::Word { .. })).collect();
assert!(!words.is_empty(), "the bar that stood is carried in place by the cover: {:?}", pf.vertices);
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
let demands: Vec<_> = pf.demand_vertices().collect();
let [(_, kind, chart, links)] = demands.as_slice() else {
    panic!("one demand, on the one chart the multiplicity is at; got {}", demands.len())
};
assert!(
    matches!(kind, crate::transducer::sentence::Demand::Source { multiplicity } if *multiplicity == 2),
    "the demand names the multiplicity it could not resolve: {kind:?}"
);
assert_eq!(**chart, bar_chart, "and it names the chart both bars stand on");
assert!(
    !links.is_empty() && links.iter().all(|&(_, _, before, after)| before != after),
    "carrying the links it stands for, each one the frame asks a different hue of"
);
assert!(!pf.is_closed(), "a sentence with demands is valid, never closed (2.2, 50.1)");
let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the family extracts");
let demands: Vec<_> = rule.form.demand_vertices().collect();
let [(_, kind, _, links)] = demands.as_slice() else {
    panic!("one demand over the family, on the one chart; got {}", demands.len())
};
assert!(
    matches!(kind, crate::transducer::sentence::Demand::Source { multiplicity } if *multiplicity == 2),
    "the same multiplicity at both frames: {kind:?}"
);
assert_eq!(links.len(), 8, "both frames' moved bars, before and after, ride the demand: {links:?}");
assert!(!rule.form.is_closed(), "and the rule is valid, never closed");
}

/// 52.5 READING 6 (ruled 2026-09-18) — SUCCESSION ALONG e₃ IS THE NEXT
/// FRAME OCCUPIED. `edge_between` read `for k in 0..n`, probing every
/// integer up to the family's pair count and skipping each miss: a scan
/// over a range, where the frames a member stands on are named by its own
/// fiber of `bodies_at`. The occupied set may skip, repeat or reorder in ℤ,
/// and a frame outside `0..n` was never read at all.
///
/// The order is the COORDINATE'S, not the trie's. The frame is written
/// little-endian, so byte order puts 256 (`[0, 1]`) before 1 (`[1, 0]`);
/// read in that order the succession is not the family's.
#[test]
fn succession_reads_the_frames_a_member_occupies_in_coordinate_order() {
    use crate::network::nerve::Keyed;
    use crate::transducer::family::{body_of, occupied_frames};
    use crate::transducer::sentence::vertex_ix_key;
    let body = || body_of(vec![(0i64, 0i64, 0i64, 1u8)]);
    let v = crate::transducer::element::identity_versor();
    let mut at: Keyed<(
        crate::transducer::family::Body,
        crate::transducer::family::Body,
        crate::transducer::element::Versor,
    )> = Keyed::new();
    let put = |at: &mut Keyed<_>, ix: u16, ck: &[u8], k: u16| {
        let mut bk = vertex_ix_key(ix);
        crate::transducer::address::put_bytes(&mut bk, ck);
        bk.extend_from_slice(&k.to_le_bytes());
        at.insert(bk, (body(), body(), v.clone()));
    };
    // ONE member on sparse, non-contiguous frames — including 256, which is
    // outside any `0..n` a three-pair family would have scanned.
    for k in [5u16, 0, 256, 2, 1] {
        put(&mut at, 7, b"chart-a", k);
    }
    // A second member, and the same vertex on another chart: neither is
    // this member's fiber.
    put(&mut at, 9, b"chart-a", 3);
    put(&mut at, 7, b"chart-b", 4);

    let occ = occupied_frames(&at, 7, b"chart-a");
    assert_eq!(occ, vec![0, 1, 2, 5, 256], "the occupied set, in coordinate order");
    assert!(occ.contains(&256), "a frame outside 0..n is occupied and is read");
    assert!(!occ.contains(&3), "another member's frame is not in this member's fiber");
    assert!(!occ.contains(&4), "nor the same vertex's frame on another chart");
    // THE ORDERING IS LOAD-BEARING. The frame is little-endian, so the
    // trie's own order — the key's bytes — reads 256 as `[0, 1]` and 1 as
    // `[1, 0]`, putting 256 second. That sequence is written out here
    // rather than sorted into existence, and it is not the succession.
    assert_ne!(vec![0u16, 256, 1, 2, 5], occ, "trie order is not coordinate order");
}

/// 52.5, 51.3 — THE FACE IS MEASURED ON THE FIBER'S UNION SHAPES. A
/// member's fiber is one body on `(r, c, k)`, and `union_quotient` is that
/// body's quotient: every lamina the member stands on across its occupied
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
    let ins: Vec<Lamina> = vec![
        (0, 0, 0, 1),
        (0, 1, 0, 1),
        (1, 0, 1, 1),
        (1, 1, 1, 1),
        (2, 0, 2, 1),
        (2, 1, 2, 1),
    ];
    let outs: Vec<Lamina> = ins.iter().map(|&(r, c, k, h)| (r, c + 2, k, h)).collect();
    let (a, b, v) = union_quotient(ins.clone(), outs).expect("one motion carries the whole fiber");
    let frames: Vec<i64> = {
        let mut f: Vec<i64> = a.laminae.iter().map(|&(_, _, k, _)| k).collect();
        f.sort_unstable();
        f.dedup();
        f
    };
    assert_eq!(frames, vec![0, 1, 2], "the union body stands on every frame of the fiber");
    assert_eq!(b.laminae.len(), a.laminae.len(), "and carries the whole of it");
    assert!(
        element::same_versor(&v, &element::translator_versor(0, 2)),
        "the fiber's quotient is the one translation"
    );
    // The same three frames with the middle one moved the other way: no
    // single motion carries the union.
    let mixed: Vec<Lamina> = ins
        .iter()
        .map(|&(r, c, k, h)| if k == 1 { (r, c + 3, k, h) } else { (r, c + 2, k, h) })
        .collect();
    assert!(
        union_quotient(ins, mixed).is_none(),
        "a fiber that is not one motion along e₃ has no union quotient"
    );
}
