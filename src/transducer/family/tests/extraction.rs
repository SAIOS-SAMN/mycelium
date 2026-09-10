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
// A is the single hue-1 cell; B is the hue-3 bar, another shape.
// A placement of (3, 3) keeps both images inside the frame for
// either orientation of the quarter turn (cells map to
// (c, −r−1) + p or (−c−1, r) + p).
let x = grid(&["330000", "000000", "001000", "000000", "000000", "000000"]);
let bodies = perceive(&x);
let a = bodies
    .iter()
    .find(|b| b.cells.iter().any(|c| c.2 == 1))
    .expect("body A");
let b = bodies
    .iter()
    .find(|b| b.cells.iter().any(|c| c.2 == 3))
    .expect("body B");
assert_ne!(a.moduli, b.moduli, "two shapes, two charts");
// Form::default() carries the Frame at index 0.
let mut pf = Form::default();
pf.vertices.push(Vertex::Word {
    address: crate::transducer::address::Address::from(vec![1]),
    derivation: crate::transducer::alphabet::Derivation::spelled(rot.clone()),
    carriage: Box::new(element::carriage(&q(1), &(q(3), q(3)))),
    gauge: vec![(1, 1)],
    class: a.moduli.clone(),
    contact: None,
});
pf.vertices.push(Vertex::Word {
    address: crate::transducer::address::Address::from(vec![2]),
    derivation: crate::transducer::alphabet::Derivation::spelled(rot.clone()),
    carriage: Box::new(element::carriage(&q(1), &(q(3), q(3)))),
    gauge: vec![(3, 5)],
    class: b.moduli.clone(),
    contact: None,
});
pf.faces.push(FormFace { i: 1, j: 2, transport: pf.face_of(1, 2).expect("pair vertices"), edge: None });
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
rebase_named_faces(&mut form, std::slice::from_ref(&pf));
let n_re = form
    .vertices
    .iter()
    .filter(|v| matches!(v, Vertex::Rebased { .. }))
    .count();
assert_eq!(n_re, 1, "one Rebased; the w_i w_j w_i⁻¹ test is gone");
match &form.vertices[2] {
    Vertex::Rebased {
        by,
        of,
        carriage,
        gauge,
        class,
    } => {
        assert_eq!((*by, *of), (1, 2));
        assert!(
            element::same_versor(carriage, &element::identity_versor())
                || element::same_versor(carriage, &element::carriage(&q(1), &(q(0), q(0)))),
            "relative carriage is the identity translator"
        );
        assert_eq!(gauge, &vec![(3, 5)], "B keeps its own hue map");
        assert_eq!(class, &b.moduli, "B keeps its own chart");
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
    reference.cells.iter().any(|c| c.2 == 5),
    "B is spoken in frame in its own colour, got {:?}",
    reference.cells
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
    spoken.cells, reference.cells,
    "the rebased form speaks B's body with B's colours at B's placement"
);
assert!(
    spoken.cells.iter().all(|c| c.2 != 3),
    "B is never spoken through A's hue map, got {:?}",
    spoken.cells
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
let held_form = pair_form_uncrystallized(&family[0].0, &family[0].1, None);
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
        crate::transducer::sentence::admit_words(&mut index, &Invariant::default(), &crate::transducer::sentence::words_of_form(&self.f));
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
    family[0].1,
    "spoken cells equal the target"
);
assert_ne!(
    form_descent(&rule.form, &family, &crystal).0,
    [0; 4],
    "the glued form measures its descent through the crystal that holds its citation"
);
assert_eq!(
    form_descent(&rule.form, &family, &NoCrystal).0,
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
let (_pay, rems) = form_descent(&rule.form, &family, &NoCrystal);
let listings: Vec<&Vec<(i64, i64, u8, u8)>> =
    rems.iter().filter(|r| !r.is_empty()).collect();
assert!(
    !listings.is_empty(),
    "a withheld form leaves at least one remainder chain"
);
for r in listings {
    let e = element::from_residual(r);
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
    matches!(rule.form.vertices.first(), Some(Vertex::Frame)),
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
    words.iter().map(|v| match v {
        Vertex::Word { gauge, .. } => gauge.clone(),
        _ => Vec::new(),
    }).collect::<Vec<_>>()
);
let hues = rule.form.hue_maps.len();
assert_eq!(
    hues, 2,
    "two hue sections, got {hues} maps on {} vertices",
    rule.form.vertices.len()
);
}

/// TWO HUES THAT MEET ARE TWO BODIES WITH A CONTACT FACE (chapter 20 as
/// ruled 2026-09-09, 50.1, 50.3). Two bars of different hues that touch
/// in the output are two bodies each carried by its own versor — one
/// translated, one still — so the pair form holds two word vertices, no
/// demand, and a face between the touching bodies; the pair did not move
/// rigidly, so that face's edge transport is a remainder edge. Read
/// hue-blind, the joined output was one blob no versor carried and every
/// such scene became a deformation demand with no contact face at all.
#[test]
fn two_hues_that_meet_are_two_bodies_with_a_contact_face() {
let family = vec![
    (grid(&["110022", "000000"]), grid(&["001122", "000000"])),
    (grid(&["000000", "110022"]), grid(&["000000", "001122"])),
];
let pf = pair_form_uncrystallized(&family[0].0, &family[0].1, None);
let words = pf.vertices.iter().filter(|v| matches!(v, Vertex::Word { .. })).count();
assert_eq!(words, 2, "two bodies of one hue each, both carried: two word vertices, got {words}");
assert_eq!(pf.demand_vertices().count(), 0, "nothing the eyes could not pair");
let contact: Vec<&FormFace> = pf.faces.iter().filter(|f| f.i != 0 && f.j != 0).collect();
assert_eq!(contact.len(), 1, "one face between the two touching bodies, got {}", contact.len());
assert!(
    contact[0].edge.is_none(),
    "one bar moved and the other stood: no single versor carries the pair, a remainder edge (50.3)"
);
let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the family extracts");
let words = rule
    .form
    .vertices
    .iter()
    .filter(|v| matches!(v, Vertex::Word { .. }))
    .count();
assert_eq!(
    words, 1,
    "correspondence is transport class (21 step 1): both bodies carry the identity word, one rule vertex, their two carriages two sections; got {words}"
);
assert_eq!(rule.form.demand_vertices().count(), 0, "and no demand");
assert!(
    rule.form.loops.iter().all(|c| c.holonomy.is_identity()),
    "the placements' coboundary telescopes around the contact loop"
);
let e = crate::transducer::sentence::as_element_form(&rule.form, [1, 0, 0, 0]).expect("element");
assert!(
    crate::verdict::blind_replay(&e).is_none() || e.magnitude().iter().all(|&x| x == 0),
    "a form tape parses"
);
}

/// L8w, rule 11, 50.2: AN EXTRACTION THE BUDGET CUTS OFF MEASURED
/// NOTHING. A truncated bundle is not a smaller rule — what two pairs of
/// four agree on is not what the family states — so the extraction says
/// nothing and the voice withholds; the next stroke asks again with a
/// budget of its own. Returned as a rule and minted, half of what the
/// fleet says would be a shape the mind never measured.
#[test]
fn an_extraction_cut_off_by_its_budget_measures_nothing() {
    let family = vec![
        (grid(&["1100", "0000"]), grid(&["0011", "0000"])),
        (grid(&["0000", "1100"]), grid(&["0000", "0011"])),
    ];
    let mut ph = crate::physics::Physics::default();
    // One spend reaches the first pair form and no further.
    let mut one = ph.stroke_n(1);
    assert!(
        extract_rule_with(&family, &NoCrystal, &mut one).is_none(),
        "a bundle the budget cannot reach the end of is unmeasured, never a rule over the pairs it saw"
    );
    // The same family with a budget that reaches the end is a rule, and it
    // is the rule of the WHOLE bundle.
    let mut whole = ph.stroke_n(1 << 20);
    let full = extract_rule_with(&family, &NoCrystal, &mut whole)
        .expect("the bundle read end to end is a rule");
    assert_eq!(
        full.form.ratio,
        family_ratio(&family).expect("scale"),
        "and it carries the family's own frame ratio"
    );
}

/// The cache never holds an unmeasured reading: a family too large for one
/// stroke would otherwise be "no rule" for as long as the language stood
/// still, and would never be spoken (L8w).
#[test]
fn an_unmeasured_extraction_is_not_cached() {
    let family = vec![
        (grid(&["1100", "0000"]), grid(&["0011", "0000"])),
        (grid(&["0000", "1100"]), grid(&["0000", "0011"])),
    ];
    let mut ph = crate::physics::Physics::default();
    let mut cache = crate::network::nerve::Keyed::new();
    let root = crate::transducer::address::Address::new(Vec::new());
    let mut small = ph.stroke_n(2);
    assert!(
        extract_rule_cached(&mut cache, b"fam", &family, root.clone(), &mut small, &NoCrystal)
            .is_none(),
        "the small budget measures nothing"
    );
    let mut large = ph.stroke_n(1 << 20);
    assert!(
        extract_rule_cached(&mut cache, b"fam", &family, root, &mut large, &NoCrystal).is_some(),
        "the next stroke asks again and reads the family"
    );
}
