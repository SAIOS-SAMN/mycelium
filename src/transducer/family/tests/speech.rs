// 21 speak_rule — SPEECH: what an extracted rule says on the test pair, and
// what it refuses to say rather than guess.

use super::common::*;
use crate::transducer::family::*;

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
f.vertices.push(Vertex::word_on(
    crate::transducer::address::Address::from(vec![1]),
    crate::transducer::alphabet::Derivation::spelled(
        crate::transducer::alphabet::Word::from_letters(&[(
            crate::transducer::alphabet::LetterId::quarter((0, 2)).expect("off-plane"),
            false,
        )]),
    ),
    class,
    Box::new(element::identity_versor()),
    Vec::new(),
    None,
));
let spoken = apply_form(
    &f,
    &x,
    &crate::transducer::alphabet::Word::identity(),
    &NoCrystal,
)
.expect("speech");
assert!(
    spoken.unspoken.contains(&1),
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
let mut f = pair_form_uncrystallized(&x, &y, None);
// MOVING A VERTEX TO ANOTHER CHART IS REBUILDING ITS MEMBER, because the
// chart is the key the member is filed under: the vertex keeps its address,
// its derivation, its carriage and its gauge, and acts on a chart no body
// in the scene carries.
let far = Invariant(vec![0xEEEE_EEEE_EEEE_EEEE]);
for v in &mut f.vertices {
    let moved = match &*v {
        Vertex::Word { address, derivation, acts, contact } => {
            acts.scoped().next().map(|(_, a)| {
                Vertex::word_on(
                    address.clone(),
                    derivation.clone(),
                    far.clone(),
                    a.carriage.clone(),
                    a.gauge.clone(),
                    *contact,
                )
            })
        }
        _ => None,
    };
    if let Some(moved) = moved {
        *v = moved;
    }
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
        .iter()
        .any(|v| matches!(v, Vertex::Unfolded { measure: crate::transducer::sentence::Measure::Occupied })),
    "the rule is the form plus Unfolded Occupied, vertices {:?}",
    rule.form.vertices.len()
);
let e = crate::transducer::sentence::as_element_form(&rule.form, [1, 0, 0, 0]).expect("v2 tape");
let back = crate::transducer::sentence::from_element_form(&e).expect("round trip");
assert!(
    back.vertices
        .iter()
        .any(|v| matches!(v, Vertex::Unfolded { measure: crate::transducer::sentence::Measure::Occupied })),
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
    .iter()
    .filter(|v| matches!(v, Vertex::Word { .. }))
    .count();
assert_eq!(bodies, 1, "one body is one vertex, got {}", rule.form.vertices.len());
assert!(
    matches!(rule.form.vertices.as_slice(), [Vertex::Frame, ..]),
    "a frame vertex is on the form"
);
let frame_face = rule.form.faces.iter().any(|f| f.i == 0 || f.j == 0);
let shifted = rule.form.vertices.iter().any(|v| match v {
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
    "constancy is the word, not the per-pair translation versor; faces {} loops {}",
    rule.form.faces.len(),
    rule.form.loops.len()
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
        frame: (3, 3),
        painted: vec![(2, 2, 5)],
        remainder: Vec::new(),
        unspoken: Vec::new(),
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
    rule.form.vertices.iter().any(|v| matches!(v, Vertex::Word { .. })),
    "the recolour class is one vertex; the chart fiber names the three squares"
);
let minted = mint_sentence(&rule, &family, &NoCrystal, 0).expect("the family mints");
let mu = minted.magnitude();
assert!(mu[1] >= 1, "recolour is gauge content, got {mu:?}");
assert!(mu.iter().any(|&x| x > 0), "T is form_deflation, never a floor on the lane");
assert_eq!(
    minted.description().get(1),
    Some(&crate::transducer::sentence::FORM_TAPE_VERSION),
    "the minted word is the form's tape"
);
assert_eq!(
    minted.address(),
    crate::transducer::sentence::as_element_form(&rule.form, minted.magnitude()).expect("tape").address(),
    "the minted address is the form's address"
);
assert_eq!(
    crate::verdict::blind_replay(&minted),
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
