// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Tests for `transducer::family`.

use super::*;

fn grid(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// Perception writes the form once. A second call on the same grids
/// does not touch a cell.
#[test]
fn pair_form_reads_cells_once() {
    let a = grid(&["1100", "0000"]);
    let b = grid(&["0011", "0000"]);
    reset_cell_reads();
    let f1 = pair_form(&a, &b, None);
    let n = cell_reads();
    assert!(n > 0, "first call reads the grids, got {n}");
    assert!(
        f1.vertices.iter().any(|v| matches!(v, Vertex::Frame)),
        "every pair form has a frame"
    );
    assert!(
        f1.vertices.len() >= 2,
        "frame plus at least one body, got {}",
        f1.vertices.len()
    );
    assert!(
        f1.vertices.iter().any(|v| matches!(v, Vertex::Word { .. })),
        "a carried body is a word vertex"
    );
    let f2 = pair_form(&a, &b, None);
    assert_eq!(cell_reads(), n, "second call touches no cells");
    assert_eq!(f1.vertices.len(), f2.vertices.len());
    assert_eq!(f1.faces.len(), f2.faces.len());
}

/// Three touching after-supports: two adjacency faces besides the
/// Frame star, and the 1-skeleton is not a tree.
#[test]
fn a_row_of_three_touching_bodies_has_two_adjacency_faces_and_a_loop() {
    let supports = vec![
        vec![],
        vec![(0, 0), (0, 1)],
        vec![(0, 2), (0, 3)],
        vec![(0, 4), (0, 5)],
    ];
    let pairs = star_and_cell_faces(4, Some(0), &supports);
    let adj: Vec<_> = pairs
        .iter()
        .copied()
        .filter(|&(i, j)| i != 0 && j != 0)
        .collect();
    assert_eq!(adj.len(), 2, "two adjacency faces, got {adj:?} from {pairs:?}");
    let faces: Vec<FormFace> = pairs
        .iter()
        .map(|&(i, j)| FormFace::new(i, j, crate::transducer::alphabet::Word::identity()))
        .collect();
    let loops = Form::loops_of(&faces, 4);
    assert!(
        !loops.is_empty(),
        "the 1-skeleton is not a star tree; got {} loops",
        loops.len()
    );
}

/// Occupancy + ring probes scale with cells, not with n².
#[test]
fn face_work_is_linear_in_occupied_cells() {
    let run = |n: usize| -> u64 {
        reset_face_work();
        let mut supports = vec![Vec::new()];
        for i in 0..n {
            supports.push(vec![(0, (i as i64) * 3)]);
        }
        let _ = star_and_cell_faces(n + 1, Some(0), &supports);
        face_work()
    };
    let w = run(8);
    let w4 = run(32);
    assert!(w > 0 && w4 > w, "work grows with bodies, got {w} then {w4}");
    assert!(
        w4 <= w.saturating_mul(5),
        "4N work is ~4× N, not n²: {w} vs {w4}"
    );
}

fn squares(n: usize, hue: u8, cols: usize) -> Grid {
    let mut g = vec![vec![0u8; cols]; 2];
    for i in 0..n {
        let c = i * 3;
        if c + 1 < cols {
            g[0][c] = hue;
            g[0][c + 1] = hue;
            g[1][c] = hue;
            g[1][c + 1] = hue;
        }
    }
    g
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
    let rule = extract_rule_with(&family, &NoCrystal).expect("extracts");
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
    let some = extract_rule_with(&mixed, &NoCrystal).expect("sectioned");
    assert!(
        some.section_count() > 0,
        "a class present in some pairs is a section"
    );
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
        extract_rule_with(&family, &NoCrystal).is_none(),
        "extract_rule_with returns None when the scale is not measured"
    );
}

/// Two pairs whose bodies differ in shape but share the carriage are
/// one rule vertex. Matching by moduli would split them and leave a
/// frame-only form.
#[test]
fn different_shapes_with_one_carriage_are_one_vertex() {
    let family = vec![
        (grid(&["1100"]), grid(&["0011"])),
        (grid(&["111000"]), grid(&["000111"])),
    ];
    let rule = extract_rule_with(&family, &NoCrystal).expect("extracts");
    let words = rule
        .form
        .vertices
        .iter()
        .filter(|v| matches!(v, Vertex::Word { .. }))
        .count();
    assert_eq!(
        words, 1,
        "one carriage is one vertex; moduli are a section coordinate, got {words}"
    );
    assert!(
        rule.form.vertices.iter().any(|v| matches!(v, Vertex::Frame)),
        "the frame stays; the form is not frame-only"
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
        address: crate::transducer::a21::Address::from(vec![1]),
        word: rot.clone(),
        carriage: element::carriage(&q(1), &(q(3), q(3))),
        gauge: vec![(1, 1)],
        class: a.moduli.clone(),
        contact: None,
    });
    pf.vertices.push(Vertex::Word {
        address: crate::transducer::a21::Address::from(vec![2]),
        word: rot.clone(),
        carriage: element::carriage(&q(1), &(q(3), q(3))),
        gauge: vec![(3, 5)],
        class: b.moduli.clone(),
        contact: None,
    });
    pf.faces.push(FormFace { i: 1, j: 2, transport: pf.face_of(1, 2).expect("pair vertices"), });
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
    let flat = extract_rule_with(&family, &NoCrystal).expect("flat");
    let flat_words = flat
        .form
        .vertices
        .iter()
        .filter(|v| matches!(v, Vertex::Word { .. }))
        .count();
    let held_form = pair_form(&family[0].0, &family[0].1, None);
    assert!(held_form.is_closed(), "a pair form of a shift coheres");
    let held_e = super::super::sentence::as_element_form(&held_form, [1, 0, 0, 0]).expect("tape");
    struct Held {
        e: Element,
        f: Form,
    }
    impl Crystal for Held {
        fn on_class(&self, _class: &Invariant) -> Vec<(Element, Form, Standing)> {
            vec![(self.e.clone(), self.f.clone(), Standing::Held)]
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
    }
    let crystal = Held {
        e: held_e,
        f: held_form,
    };
    let rule = extract_rule_with(&family, &crystal).expect("glues");
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

/// An unspellable vertex is unspoken: conformal_of None is not replaced
/// by an invented identity versor.
#[test]
fn unspellable_vertex_is_unspoken() {
    let x = grid(&["1100", "0000"]);
    let bodies = perceive(&x);
    let class = bodies
        .first()
        .map(|b| b.moduli.clone())
        .expect("a body");
    let mut f = Form::default();
    f.vertices.push(Vertex::Word {
        address: crate::transducer::a21::Address::from(vec![1]),
        word: crate::transducer::alphabet::Word::from_letters(&[(
            crate::transducer::alphabet::LetterId::quarter((0, 2)).expect("off-plane"),
            false,
        )]),
        carriage: element::identity_versor(),
        gauge: Vec::new(),
        class,
        contact: None,
    });
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
        spoken.cells.is_empty(),
        "an invented identity versor would have carried the body, got {:?}",
        spoken.cells
    );
}

/// An empty class fiber speaks nothing for that vertex and lists it unspoken.
#[test]
fn empty_fiber_is_unspoken_not_every_body() {
    let x = grid(&["1100", "0000"]);
    let y = grid(&["0011", "0000"]);
    let mut f = pair_form(&x, &y, None);
    for v in &mut f.vertices {
        if let Vertex::Word { class, .. } = v {
            *class = Invariant(vec![0xEEEE_EEEE_EEEE_EEEE]);
        }
    }
    let spoken = apply_form(
        &f,
        &x,
        &crate::transducer::alphabet::Word::identity(),
        &NoCrystal,
    )
    .expect("a miss is unspoken, not a refusal");
    assert!(
        spoken.cells.is_empty(),
        "empty fiber speaks no cells, got {:?}",
        spoken.cells
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
    let rule = extract_rule_with(&family, &NoCrystal).expect("unfolds");
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

/// Contact when the first step moves the ring; NoOrbit when it does not.
#[test]
fn orbit_contact_and_no_orbit() {
    let zero = q(0);
    let one = q(1);
    let moving = Vertex::Word {
        address: crate::transducer::a21::Address::from(vec![1]),
        word: crate::transducer::alphabet::Word::identity(),
        carriage: element::carriage(&one.clone(), &(zero.clone(), one.clone())),
        gauge: Vec::new(),
        class: Invariant::default(),
        contact: None,
    };
    let still = Vertex::Word {
        address: crate::transducer::a21::Address::from(vec![2]),
        word: crate::transducer::alphabet::Word::identity(),
        carriage: element::carriage(&one, &(zero.clone(), zero)),
        gauge: Vec::new(),
        class: Invariant::default(),
        contact: None,
    };
    let cells = vec![(0i64, 0i64, 1u8)];
    let anchor = vec![(0i64, 3i64)];
    let frame = (5usize, 5usize);
    match orbit(&moving, &cells, &anchor, frame) {
        OrbitEnd::Contact { steps, .. } => assert!(steps >= 1, "contact takes at least one step"),
        other => panic!("expected Contact, got {other:?}"),
    }
    assert_eq!(
        orbit(&still, &cells, &anchor, frame),
        OrbitEnd::NoOrbit,
        "a still word has no orbit"
    );
    let with_c = Vertex::Word {
        address: crate::transducer::a21::Address::from(vec![3]),
        word: crate::transducer::alphabet::Word::identity(),
        carriage: element::carriage(&q(1), &(q(0), q(1))),
        gauge: Vec::new(),
        class: Invariant::default(),
        contact: Some(crate::transducer::alphabet::Contact { anchor: 0, ring: 0 }),
    };
    match orbit(&with_c, &cells, &anchor, frame) {
        OrbitEnd::Contact { .. } | OrbitEnd::LeftFrame { .. } | OrbitEnd::Torsion { .. } => {}
        OrbitEnd::NoOrbit => panic!("contact word must orbit"),
    }
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
    let rule = extract_rule(&family).expect("the shift family extracts");
    let (_, rems) = form_descent(&rule.form, &family, &NoCrystal);
    assert!(rems.iter().all(|r| r.is_empty()), "the family closes: {:?}", rems);
    let spoken = speak_rule(&rule, &grid(&["0000", "0000", "1100"]), &NoCrystal).expect("spoken");
    assert!(spoken.remainder.is_empty(), "the test is reached: {:?}", spoken.remainder);
    assert_eq!(spoken.grid(), grid(&["0000", "0000", "0022"]));
    let family = vec![
        (grid(&["100", "110", "000"]), grid(&["010", "110", "000"])),
        (grid(&["000", "010", "011"]), grid(&["000", "001", "011"])),
    ];
    match extract_rule(&family) {
        None => {}
        Some(rule) => {
            let (_, rems) = form_descent(&rule.form, &family, &NoCrystal);
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
    let rule = extract_rule(&family).expect("the dilation family extracts");
    assert_eq!(rule.form.ratio, ((2, 1), (2, 1)));
    let (_, rems) = form_descent(&rule.form, &family, &NoCrystal);
    assert!(rems.iter().all(|r| r.is_empty()), "the dilation family closes: {:?}", rems);
    let spoken = speak_rule(&rule, &family[0].0, &NoCrystal).expect("dilation spoken");
    assert_eq!(spoken.grid(), family[0].1);
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
    match extract_rule(&family) {
        None => {}
        Some(rule) => {
            let (_, rems) = form_descent(&rule.form, &family, &NoCrystal);
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
    assert!(extract_rule(&disagree).is_none(), "the form engine withholds a family it does not extract");
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
    let rule = extract_rule(&family).expect("the family extracts a rule");
    let bodies = rule
        .form
        .vertices
        .iter()
        .filter(|v| matches!(v, Vertex::Word { .. }))
        .count();
    assert_eq!(bodies, 1, "one body is one vertex, got {}", rule.form.vertices.len());
    assert!(
        matches!(rule.form.vertices.first(), Some(Vertex::Frame)),
        "a frame vertex is on the form"
    );
    let frame_face = rule.form.faces.iter().any(|f| f.i == 0 || f.j == 0);
    let shifted = rule.form.vertices.iter().any(|v| match v {
        Vertex::Word { carriage, .. } => {
            !element::same_versor(carriage, &element::identity_versor())
        }
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
    assert_eq!(spoken.grid(), grid(&["0022", "0000"]));
}

/// Pairing is ∃V on the shapes, not a digest of the moduli axes.
#[test]
fn pairing_is_carriage_not_a_moduli_digest() {
    let family = vec![(grid(&["1100", "0000"]), grid(&["0011", "0000"]))];
    let rule = extract_rule(&family).expect("a translated bar extracts");
    let bodies = rule
        .form
        .vertices
        .iter()
        .filter(|v| matches!(v, Vertex::Word { .. }))
        .count();
    assert_eq!(bodies, 1, "one body is one vertex under ∃V, got {}", rule.form.vertices.len());
    let a = perceive(&family[0].0);
    let b = perceive(&family[0].1);
    assert_eq!(a[0].moduli, b[0].moduli, "the chart is exact axes, not a hash");
    assert!(carriage(&a[0], &b[0]).is_some(), "a versor carries the bar");
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
    let rule = extract_rule(&family).expect("three squares extract as a section");
    let spoken = speak_rule(&rule, &train_in, &NoCrystal).expect("the test is spoken");
    assert!(spoken.remainder.is_empty(), "all three squares are reached: {:?}", spoken.remainder);
    assert_eq!(spoken.grid(), train_out);
    assert!(
        rule.form.vertices.iter().any(|v| matches!(v, Vertex::Word { .. })),
        "the recolour class is one vertex; the chart fiber names the three squares"
    );
    let minted = mint_sentence(&rule, &family, &NoCrystal).expect("the family mints");
    let mu = minted.magnitude();
    assert!(mu[1] >= 1, "recolour is gauge content, got {mu:?}");
    assert!(mu.iter().any(|&x| x > 0), "T is form_descent, never a floor on the lane");
    assert_eq!(minted.description().get(1), Some(&4), "the minted word is the form's v4 tape");
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
    // descent the voice measured.
    let back = crate::transducer::sentence::from_element_form(&minted).expect("v2 form");
    assert_eq!(back.vertices, rule.form.vertices, "vertices survive the tape");
    assert_eq!(back.faces, rule.form.faces, "faces survive the tape");
    assert_eq!(back.ratio, rule.form.ratio);
    assert_eq!(
        form_descent(&back, &family, &NoCrystal),
        form_descent(&rule.form, &family, &NoCrystal),
        "the descent re-measures from the tape"
    );
}

/// Deflation is per-pair measure_lanes(delta(x,y)) − measure_lanes(delta(spoken,y)),
/// accepted only if that pair descends. A raise or a miss on one pair
/// does not zero the others; a form is spoken only when every remainder
/// is empty.
#[test]
fn form_descent_measures_deflation_per_pair() {
    let x = grid(&["1"]);
    let y = grid(&["2"]);
    let family_exact = vec![(x.clone(), y.clone())];
    let rule = extract_rule(&family_exact).expect("recolour");
    let (pay, rems) = form_descent(&rule.form, &family_exact, &NoCrystal);
    assert!(
        rems.iter().all(|r| r.is_empty()),
        "an exact speak leaves empty remainders, got {rems:?}"
    );
    assert_ne!(pay, [0; 4], "exact speak measures the closed delta");
    let before = element::measure_lanes(
        &crate::arc::refine_delta(&x, &y)
            .expect("delta")
            .0,
    );
    assert_eq!(pay, before, "deflation of an empty remainder is the closed delta");
    let x2 = grid(&["10"]);
    let y2 = grid(&["23"]);
    let family = vec![(x, y), (x2, y2)];
    let (pay2, rems2) = form_descent(&rule.form, &family, &NoCrystal);
    assert!(rems2[0].is_empty(), "pair one closes");
    assert!(
        !rems2[1].is_empty(),
        "pair two leaves a remainder, got {:?}",
        rems2[1]
    );
    assert_ne!(
        pay2, [0; 4],
        "a miss on pair two does not zero pair one's deflation"
    );
    assert!(
        mint_sentence(&rule, &family, &NoCrystal).is_none(),
        "a form is spoken only when every remainder is empty"
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
    let rule = extract_rule(&[(grid(&["1"]), grid(&["2"]))]).expect("recolour");
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
    let atlas1 = crate::transducer::a21::Address::from(vec![1]);
    let atlas2 = crate::transducer::a21::Address::from(vec![2]);
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
    let rule = extract_rule(&family).expect("section class");
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
    let rule = extract_rule(&family).expect("frame plus a some-pair class");
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
    let rule = extract_rule(&family).expect("different shapes, same recolour");
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
    let rule = extract_rule(&family).expect("one recolour class");
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

/// Two hues of one linear word are one transport class; the maps are
/// hue sections, not two vertices. A vortex needs two words, not two hues.
#[test]
fn two_hues_form_a_cycle_the_word_not_the_versor() {
    let family = vec![
        (grid(&["110022", "000000"]), grid(&["001122", "000000"])),
        (grid(&["000000", "110022"]), grid(&["000000", "001122"])),
    ];
    let rule = extract_rule(&family).expect("two bodies extract");
    let words = rule
        .form
        .vertices
        .iter()
        .filter(|v| matches!(v, Vertex::Word { .. }))
        .count();
    assert_eq!(
        words, 1,
        "two hues of one word class are one vertex, got {words} words, {} vertices",
        rule.form.vertices.len()
    );
    assert!(
        rule.form.hue_maps.len() >= 2,
        "two hue sections, got {}",
        rule.form.hue_maps.len()
    );
    assert!(
        rule.form.loops.iter().all(|c| c.holonomy.is_identity()),
        "holonomy of identity-word faces is the identity"
    );
    let e = crate::transducer::sentence::as_element_form(&rule.form, [1, 0, 0, 0]).expect("element");
    assert!(
        crate::verdict::blind_replay(&e).is_none() || e.magnitude().iter().all(|&x| x == 0),
        "a form tape parses"
    );
}

#[test]
fn bundle_pressure_is_additive_and_refuses_a_raise_on_one_pair() {
    let x = grid(&["1"]);
    let y = grid(&["2"]);
    let f = Form::default();
    let bundle = [(x.clone(), y.clone()), (x.clone(), y.clone())];
    let (pairs, sum) = bundle_pressure(&f, &bundle, &NoCrystal, 0).expect("two pairs");
    assert_eq!(pairs.len(), 2);
    assert_eq!(
        sum[0],
        pairs[0][0] + pairs[1][0],
        "S is the sum of the pair residuals, got {sum:?} from {pairs:?}"
    );
    let spoken = apply_form(&f, &x, &crate::transducer::alphabet::Word::identity(), &NoCrystal);
    let g = spoken.as_ref().map(|s| s.grid());
    let r1 = residual_of(g.as_ref(), &y);
    let mut cat = r1.clone();
    cat.extend(r1);
    let merged = element::measure_lanes(&cat);
    assert_ne!(
        sum[3], merged[3],
        "H of a concatenated chain at shared coordinates is a merge, sum {sum:?} merged {merged:?}"
    );
    let pair_one_down = [1u64, 0, 0, 2];
    let pair_two_raise = [0u64, 0, 0, 3];
    let before_pairs = [[3u64, 0, 0, 5], [0, 0, 0, 1]];
    let after_pairs = [pair_one_down, pair_two_raise];
    let before_sum = [3u64, 0, 0, 6];
    let after_sum = [1u64, 0, 0, 5];
    assert!(
        crate::economy::descends(&after_sum, &before_sum),
        "the summed μ⃗ descends"
    );
    assert!(
        !pairs_descend(&before_pairs, &after_pairs),
        "a raise on pair two is refused even though the sum descends"
    );
}
