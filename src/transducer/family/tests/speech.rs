// 21 speak_rule — SPEECH: what an extracted rule says on the test pair, and
// what it refuses to say rather than guess.

use super::common::*;
use crate::transducer::family::*;

/// LAW 12 — A GLUED VERTEX SPEAKS ITS CITED FORM ONTO THE SCENE THE SPEECH
/// ALREADY PERCEIVED. A form citing the bar's shift twice speaks the grid
/// with the sensor read once — its own perception — and not once more per
/// citation: the bodies are one fact of the grid, and the cited form is
/// spoken onto them at its composed placement.
#[test]
fn a_glued_vertex_speaks_its_cited_form_on_the_one_perception_of_the_scene() {
    let (x, y) = (grid(&["1100", "0000"]), grid(&["0011", "0000"]));
    let held_form = cipher_form(&x, &y);
    assert!(held_form.is_closed(), "the bar's shift is a closed form to cite");
    let held_e = crate::transducer::sentence::as_element_form(&held_form, [1, 0, 0, 0]).expect("tape");
    struct Cited {
        e: Element,
        f: Form,
    }
    impl Crystal for Cited {
        fn on_class(&self, _class: &Invariant) -> Vec<(&Element, &Form, Standing)> {
            Vec::new()
        }
        fn on_body(&self, _chart: &Invariant) -> Vec<(&Element, &Form, Standing)> {
            Vec::new()
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
    let crystal = Cited { e: held_e.clone(), f: held_form.clone() };
    let mut citing = Form::default();
    let identity = crate::transducer::alphabet::Word::identity();
    for _ in 0..2 {
        citing.glue(&held_form, held_e.address(), identity.clone(), held_e.magnitude()).expect("the closed form glues");
    }
    // 54.7 B5a — ONE BOUNDARY, ONE INCIDENCE. The same form cited twice
    // under the same φ at the same chart is one overlap stated twice: one
    // `Glued` vertex at one site, and one edge in the cover. It was two
    // vertices because each carried the cited address, and two vertices at
    // one site have a key that does not round-trip to them.
    let glued = citing.vertices.scoped().filter(|(_, v)| matches!(v, Vertex::Glued { .. })).count();
    assert_eq!(glued, 1, "two statements of one citation are one boundary");
    assert_eq!(citing.cover.len(), 1, "and one incidence");
    let spoken = apply_form(&citing, &x, &identity, &crystal).expect("the citing form speaks");
    assert_eq!(spoken.grid(), Some(y), "and both citations speak the bar's shift onto it");
}

/// An unspellable vertex is unspoken: conformal_of None is not replaced
/// by an invented identity versor.
#[test]
fn unspellable_vertex_is_unspoken() {
let x = grid(&["1100", "0000"]);
let bodies = perceive(&x);
// The fixture is one body.
let [body] = bodies.as_slice() else { panic!("a body") };
let class = body.moduli.clone();
let mut f = Form::default();
let at = f
    .put_vertex(Vertex::word_on(
        crate::transducer::alphabet::Derivation::spelled(
            crate::transducer::alphabet::Word::from_letters(&[(
                crate::transducer::alphabet::LetterId::turn_90((0, 2)).expect("off-plane"),
                false,
            )]),
        ),
        class,
        Box::new(element::identity_versor()),
        Vec::new(),
        [0u64],
        None,
    ))
    .expect("an off-plane 90° turn has a class");
let spoken = apply_form(
    &f,
    &x,
    &crate::transducer::alphabet::Word::identity(),
    &NoCrystal,
)
.expect("speech");
assert!(
    spoken.unspoken.contains(&at),
    "an off-plane word does not spell on the sensor; the vertex is unspoken, got {:?}",
    spoken.unspoken
);
assert!(
    spoken.painted.is_empty(),
    "an invented identity versor would have carried the body, got {:?}",
    spoken.painted
);
}

/// An empty class fiber speaks nothing for that vertex and lists it unspoken.
#[test]
fn empty_fiber_is_unspoken_not_every_body() {
let x = grid(&["1100", "0000"]);
let y = grid(&["0011", "0000"]);
let mut f = cipher_form(&x, &y);
// MOVING A VERTEX TO ANOTHER CHART IS REBUILDING ITS MEMBER, because the
// chart is the key the member is filed under: the vertex keeps its address,
// its derivation, its carriage and its gauge, and acts on a chart no body
// in the scene carries.
let far = Invariant::counts(&[0xEEEE_EEEE_EEEE_EEEE]);
let moved: Vec<(crate::network::nerve::coord::VertexAt, Vertex)> = f
    .vertices
    .entries()
    .filter_map(|(at, v)| match v {
        Vertex::Word { derivation, acts, contact, .. } => acts.scoped().next().map(|(_, a)| {
            (
                at,
                Vertex::word_on(
                    derivation.clone(),
                    far.clone(),
                    a.carriage.clone(),
                    a.gauge.clone(),
                    crate::transducer::sentence::frames_of_section(&a.frames),
                    *contact,
                ),
            )
        }),
        _ => None,
    })
    .collect();
// The class is the key and a moved member keeps its class, so the vertex
// is rewritten under the key it stood at.
for (at, v) in moved {
    f.vertices.put(&at, v);
}
f.reindex();
let spoken = apply_form(
    &f,
    &x,
    &crate::transducer::alphabet::Word::identity(),
    &NoCrystal,
)
.expect("a miss is unspoken, not a refusal");
assert!(
    spoken.painted.is_empty(),
    "empty fiber speaks no site, got {:?}",
    spoken.painted
);
assert!(
    !spoken.unspoken.is_empty(),
    "the Word vertex is listed unspoken"
);
}

/// Grow-to-frame is Unfolded{{Occupied}}; the tape carries no step count.
#[test]
fn unfolding_grow_to_frame() {
let family = vec![
    (grid(&["10", "00"]), grid(&["11", "11"])),
    (grid(&["100", "000", "000"]), grid(&["111", "111", "111"])),
];
let rule = extract_rule_with(&family, &NoCrystal, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("unfolds");
assert!(
    rule.form
        .vertices
        .scoped()
        .any(|(_, v)| matches!(v, Vertex::Unfolded { measure: crate::transducer::sentence::Measure::Occupied })),
    "the rule is the form plus Unfolded Occupied, vertices {:?}",
    rule.form.vertices.len()
);
let e = crate::transducer::sentence::as_element_form(&rule.form, [1, 0, 0, 0]).expect("v2 tape");
let back = crate::transducer::sentence::from_element_form(&e).expect("round trip");
assert!(
    back.vertices
        .scoped()
        .any(|(_, v)| matches!(v, Vertex::Unfolded { measure: crate::transducer::sentence::Measure::Occupied })),
    "no step count on the tape; Occupied is the measure"
);
}

/// A family that shifts every body two columns right and recolours
/// 1→2, a body mirrored in place, and a family that dilates the scene
/// twofold each extract a form and speak a new input exactly.
#[test]
fn a_shift_a_mirror_and_a_dilation_are_comprehended_and_spoken() {
let family = vec![
    (grid(&["1100", "0000", "0000"]), grid(&["0011", "0000", "0000"])),
    (grid(&["0000", "1100", "0000"]), grid(&["0000", "0011", "0000"])),
];
let family: Vec<(Grid, Grid)> = family
    .into_iter()
    .map(|(a, b)| (a, b.into_iter().map(|row| row.into_iter().map(|h| if h == 1 { 2 } else { h }).collect()).collect()))
    .collect();
let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the shift family extracts");
let (_, rems) = form_deflation(&rule.form, &family, &NoCrystal);
assert!(rems.iter().all(|r| r.is_empty()), "the family closes: {:?}", rems);
let spoken = speak_rule(&rule, &grid(&["0000", "0000", "1100"]), &NoCrystal).expect("spoken");
assert!(spoken.remainder.is_empty(), "the test is reached: {:?}", spoken.remainder);
assert_eq!(spoken.grid(), Some(grid(&["0000", "0000", "0022"])));
let family = vec![
    (grid(&["100", "110", "000"]), grid(&["010", "110", "000"])),
    (grid(&["000", "010", "011"]), grid(&["000", "001", "011"])),
];
match extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)) {
    None => {}
    Some(rule) => {
        let (_, rems) = form_deflation(&rule.form, &family, &NoCrystal);
        assert!(
            rems.iter().any(|r| !r.is_empty()),
            "a reflection the form engine does not close is withheld remainder"
        );
    }
}
let family = vec![
    (grid(&["10", "00"]), grid(&["1100", "1100", "0000", "0000"])),
    (grid(&["00", "03"]), grid(&["0000", "0000", "0033", "0033"])),
];
let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the dilation family extracts");
assert_eq!(rule.form.ratio, ((2, 1), (2, 1)));
let (_, rems) = form_deflation(&rule.form, &family, &NoCrystal);
assert!(rems.iter().all(|r| r.is_empty()), "the dilation family closes: {:?}", rems);
let spoken = speak_rule(&rule, &family[0].0, &NoCrystal).expect("dilation spoken");
assert_eq!(spoken.grid(), Some(family[0].1.clone()));
}

/// A family whose rule depends on something the form does not carry
/// is withheld: remainder on the extracted form, or no form when the
/// pairs do not share a frame.
#[test]
fn an_uncomprehended_family_leaves_its_residual_as_the_demand() {
let family = vec![
    (grid(&["100", "000"]), grid(&["001", "000"])),
    (grid(&["100", "000"]), grid(&["000", "001"])),
];
match extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)) {
    None => {}
    Some(rule) => {
        let (_, rems) = form_deflation(&rule.form, &family, &NoCrystal);
        assert!(
            rems.iter().any(|r| !r.is_empty()),
            "a family the form does not close leaves remainder"
        );
        assert!(rems.iter().flatten().all(|&(_, _, s, h)| s != h));
    }
}
let disagree = vec![
    (grid(&["10", "00"]), grid(&["1100", "1100", "0000", "0000"])),
    (grid(&["10", "00"]), grid(&["110", "110", "000"])),
];
assert!(family_ratio(&disagree).is_none(), "frames that disagree are not one form");
assert!(extract_rule(&disagree, &mut crate::physics::Physics::default().stroke_n(1 << 20)).is_none(), "the form engine withholds a family it does not extract");
}

#[test]
fn a_shift_family_extracts_a_rule_and_speaks_the_test() {
let family = vec![
    (grid(&["1100", "0000"]), grid(&["0011", "0000"])),
    (grid(&["0000", "1100"]), grid(&["0000", "0011"])),
];
let family: Vec<(Grid, Grid)> = family
    .into_iter()
    .map(|(a, b)| {
        (
            a,
            b.into_iter()
                .map(|row| row.into_iter().map(|h| if h == 1 { 2 } else { h }).collect())
                .collect(),
        )
    })
    .collect();
let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the family extracts a rule");
let bodies = rule
    .form
    .vertices
    .scoped()
    .filter(|(_, v)| matches!(v, Vertex::Word { .. }))
    .count();
assert_eq!(bodies, 1, "one body is one vertex, got {}", rule.form.vertices.len());
assert!(
    matches!(rule.form.vertices.at(&crate::network::nerve::coord::VertexAt::Frame), Some(Vertex::Frame)),
    "a frame vertex is on the form"
);
let frame_face = rule.form.faces.entries().any(|(o, _)| {
    o.from == crate::network::nerve::coord::ChartAt::Frame || o.to == crate::network::nerve::coord::ChartAt::Frame
});
let shifted = rule.form.vertices.scoped().any(|(_, v)| match v {
    // ANY MEMBER placed away from the identity is a family-constant shift:
    // the claim is that the rule places something, and a vertex acting on
    // several charts places each of them.
    Vertex::Word { .. } => v.acts().into_iter().flat_map(|a| a.scoped()).any(|(_, m)| {
        !element::same_versor(&m.carriage, &element::identity_versor())
    }),
    _ => false,
});
assert!(frame_face || shifted, "placement is a frame face or a family-constant shift");
assert!(
    !rule.form.faces.is_empty(),
    "constancy is the word, not the per-pair translation versor; faces {} orbits {}",
    rule.form.faces.len(),
    rule.form.orbits.len()
);
let spoken = speak_rule(&rule, &grid(&["1100", "0000"]), &NoCrystal).expect("the test is spoken");
assert!(spoken.remainder.is_empty(), "the test is reached in full: {:?}", spoken.remainder);
assert_eq!(spoken.grid(), Some(grid(&["0022", "0000"])));
}

/// A SPEECH THAT DOES NOT FIT ITS OWN FRAME LAYS DOWN NO GRID.
///
/// A transported site outside the output rectangle is the condition the
/// court names `FAIL_PAST_FRAME` — a site landing past the image frame's
/// extent — and a voice that indexed the rectangle by it would die where
/// the court refuses.
///
/// Refused, not dropped: laying the rest of the sites down would produce a
/// grid that silently is not what the form said.
#[test]
fn a_speech_whose_site_is_outside_its_frame_refuses_the_grid() {
    use crate::transducer::family::Speech;
    let fits = Speech {
        unresolved: Vec::new(),
        cycled: Vec::new(),
        frame: (3, 3),
        painted: vec![(2, 2, 5)],
        remainder: Vec::new(),
        unspoken: Vec::new(),
        unreached: Vec::new(),
        demanded: [0; 6],
    };
    assert_eq!(
        fits.grid(),
        Some(grid(&["000", "000", "005"])),
        "a site inside the frame is laid down"
    );
    // Row 9 on a 3-row frame.
    let past = Speech { painted: vec![(9, 0, 5)], ..fits.clone() };
    assert_eq!(past.grid(), None, "a row past the frame's extent refuses");
    let wide = Speech { painted: vec![(0, 9, 5)], ..fits.clone() };
    assert_eq!(wide.grid(), None, "and so does a column past it");
    // Before the origin is the other half of the same fault.
    let before = Speech { painted: vec![(-1, 0, 5)], ..fits.clone() };
    assert_eq!(before.grid(), None, "a negative coordinate is not a site of the frame");
    // One bad site refuses the whole grid; the good ones are not laid down
    // to stand in for what the form actually said.
    let mixed = Speech { painted: vec![(0, 0, 5), (9, 0, 5)], ..fits };
    assert_eq!(mixed.grid(), None, "one site outside refuses the speech, not just that site");
}

/// Three identical squares in one family are a section: the test
/// recolours all three.
#[test]
fn three_identical_squares_recolour_all_three_on_the_test() {
let train_in = grid(&[
    "11011",
    "11011",
    "00000",
    "11000",
    "11000",
]);
let train_out: Grid = train_in
    .iter()
    .map(|row| row.iter().map(|&h| if h == 1 { 2 } else { h }).collect())
    .collect();
let family = vec![(train_in.clone(), train_out.clone())];
let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("three squares extract as a section");
let spoken = speak_rule(&rule, &train_in, &NoCrystal).expect("the test is spoken");
assert!(spoken.remainder.is_empty(), "all three squares are reached: {:?}", spoken.remainder);
assert_eq!(spoken.grid(), Some(train_out));
assert!(
    rule.form.vertices.scoped().any(|(_, v)| matches!(v, Vertex::Word { .. })),
    "the recolour class is one vertex; the chart fiber names the three squares"
);
let minted = mint_sentence(&rule, &family, &NoCrystal, 0).expect("the family mints");
let mu = minted.magnitude();
assert!(mu[1] >= 1, "recolour is gauge content, got {mu:?}");
assert!(mu.iter().any(|&x| x > 0), "T is form_deflation, never a floor on the lane");
// 54.7 B6 — THE MINTED WORD IS THE FORM'S ADDRESS, and no longer its tape.
// This asserted `description()[1] == FORM_TAPE_VERSION`: that the mint had
// written a byte buffer carrying a version. A sentence mints no buffer, so
// the claim is the one below, which was always the real one — the minted
// word stands at the coordinate the form itself keys to.
assert!(minted.description().is_empty(), "a sentence mints no tape");
assert_eq!(
    minted.address().tag(),
    Some(crate::transducer::address::TAG_FORM),
    "it is a sentence by the grade-0 coordinate its address opens with"
);
assert_eq!(
    minted.address(),
    crate::transducer::sentence::as_element_form(&rule.form, minted.magnitude()).expect("tape").address(),
    "the minted address is the form's address"
);
assert_eq!(
    crate::verdict::blind_replay(&minted, &mut super::common::budget()),
    None,
    "the court replays the minted form from its tape"
);
// The hypha reads the form back from the tape and measures the same
// deflation the voice measured.
let back = crate::transducer::sentence::from_element_form(&minted).expect("v2 form");
assert_eq!(back.vertices, rule.form.vertices, "vertices survive the tape");
assert_eq!(back.faces, rule.form.faces, "faces survive the tape");
assert_eq!(back.ratio, rule.form.ratio);
assert_eq!(
    form_deflation(&back, &family, &NoCrystal),
    form_deflation(&rule.form, &family, &NoCrystal),
    "the deflation re-measures from the tape"
);
}

/// 52.5, 16.5 — THE OPEN SECTION IS SPOKEN THROUGH `carry_compartment`, AT ITS
/// OWN FRAME. `emit_carried` is the one carry on the speaking path — the
/// only other writer of `painted` is 16.16's unfold, which grows from
/// neighbours and carries nothing — and it hands the sensor the body's own
/// `k`, never a zero substituted for it.
///
/// The two readings are the assertion. The frame RIDES ON THE POSE: a body
/// at frame `n` speaks the same `(r, c)` as the same body at frame 0 and
/// stays at frame `n`, which is what makes the test input an open section
/// of the family's one body rather than a separate scene. And the sensor
/// REFUSES rather than rounds: a versor whose image does not land on the
/// base emits no paint at all, and the body leaves as remainder.
#[test]
fn the_open_section_is_carried_at_its_own_frame_and_a_tilt_is_refused() {
    use crate::transducer::family::{body_of, emit_carried};
    let bar = |k: i64| {
        body_of(vec![Compartment::from((0i64, 0i64, k, 1u8)), Compartment::from((0i64, 1i64, k, 1u8))])
            .expect("a bar is heard at n = 3")
    };
    let shift = element::translator_versor(2, 0);
    let mut at_zero = Vec::new();
    let mut rem_zero = Vec::new();
    emit_carried(&bar(0), &shift, &[(1, 1)], (6, 6), &mut at_zero, &mut rem_zero);
    let mut at_three = Vec::new();
    let mut rem_three = Vec::new();
    emit_carried(&bar(3), &shift, &[(1, 1)], (6, 6), &mut at_three, &mut rem_three);
    assert_eq!(at_zero, vec![(2, 0, 1), (2, 1, 1)], "the bar is carried two rows down");
    assert_eq!(
        at_three, at_zero,
        "an open section at frame 3 speaks what the same body at frame 0 speaks"
    );
    assert!(rem_zero.is_empty() && rem_three.is_empty(), "and neither leaves remainder");
    // A rotation by an angle the base does not close on: the sensor refuses
    // the image rather than rounding it onto a site, and the whole body is
    // remainder.
    let letter = crate::transducer::alphabet::LetterId::pythagorean_5((0, 1))
        .expect("the (3,4,5) rotor on the sensor plane");
    let tilt = element::versor_of_word(&crate::transducer::alphabet::Word::from_letters(&[(letter, false)]))
        .expect("a base action");
    let mut painted = Vec::new();
    let mut remainder = Vec::new();
    emit_carried(&bar(3), &tilt, &[(1, 1)], (6, 6), &mut painted, &mut remainder);
    assert!(
        painted.is_empty() && remainder.len() == 2,
        "a coordinate that does not fit the base is refused, not rounded: {painted:?} {remainder:?}"
    );
}
