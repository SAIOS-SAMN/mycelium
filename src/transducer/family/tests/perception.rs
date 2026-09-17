// 20 — PERCEPTION: what one pair form costs the sensor, and what the
// adjacency of a scene's bodies is.

use super::common::*;
use crate::transducer::family::*;

/// Perception writes the form once. A second call on the same grids
/// does not touch a lamina.
#[test]
fn pair_form_reads_laminae_once() {
let a = grid(&["1100", "0000"]);
let b = grid(&["0011", "0000"]);
reset_sensor_reads();
let f1 = pair_form_uncrystallized(&a, &b, None);
let n = sensor_reads();
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
let f2 = pair_form_uncrystallized(&a, &b, None);
assert_eq!(sensor_reads(), n, "second call touches no lamina");
assert_eq!(f1.vertices.len(), f2.vertices.len());
assert_eq!(f1.faces.len(), f2.faces.len());
}

/// 50.3 — A RIGID SCENE HAS IDENTITY EDGES AND A RELATIVE MOTION DOES NOT.
/// Two touching bodies translated together: the pair-body carries rigidly,
/// `V_ij = V_i`, and the edge between them is the identity. One of them
/// moved alone: no versor carries the pair, and the edge is a remainder
/// edge, never a loop's face.
#[test]
fn a_rigid_scene_has_identity_edges_and_a_relative_motion_does_not() {
let body = |rows: &[&str]| perceive(&grid(rows)).into_iter().next().expect("one body");
let (a1, a2) = (body(&["1100", "0000"]), body(&["0000", "2200"]));
let (b1, b2) = (body(&["0110", "0000"]), body(&["0000", "0220"]));
let v1 = carriage(&a1, &b1).expect("a translated bar carries").v;
let edge = edge_transport(&v1, &a1, &a2, &b1, &b2).expect("the pair moved rigidly: an edge transport");
assert!(edge.word.is_identity(), "a rigid motion is the identity on the edge: {:?}", edge.word);
assert!(
    crate::transducer::element::same_versor(&edge.carriage, &crate::transducer::element::identity_versor()),
    "and carries no relative placement"
);
let b2_still = body(&["0000", "2200"]);
assert!(
    matches!(edge_transport(&v1, &a1, &a2, &b1, &b2_still), Err(EdgeRefusal::NotRigid)),
    "a pair no single versor carries is a remainder edge, and says so"
);
}

/// 50.3 — AN EXCHANGE: two congruent bodies that swap places carry as one
/// pair-body by the identity, and the edge transport is the exchange —
/// `V_i⁻¹`, a placement with the identity word, finite on the word.
#[test]
fn edge_transport_lies_in_the_stabilizer_or_the_exchange() {
let body = |rows: &[&str]| perceive(&grid(rows)).into_iter().next().expect("one body");
let (a1, a2) = (body(&["100"]), body(&["002"]));
let (b1, b2) = (body(&["001"]), body(&["200"]));
let v1 = carriage(&a1, &b1).expect("a translated lamina carries").v;
let edge = edge_transport(&v1, &a1, &a2, &b1, &b2).expect("the pair-body is carried by the identity: an edge");
assert!(edge.word.is_identity(), "the exchange has the identity word");
assert!(
    !crate::transducer::element::same_versor(&edge.carriage, &crate::transducer::element::identity_versor()),
    "and a placement: the bodies exchanged"
);
assert!(matches!(edge.word.order(), crate::transducer::alphabet::Order::Finite(_)));
}

/// 50.5 — A CREATED BODY IS A SOURCE OR A CREATION DEMAND, never a word
/// with the identity: a new bar some input chart carries onto is a
/// `Source` with that multiplicity; a new body no input chart carries
/// onto is a `Creation`. Both ride the pair form as demand vertices with
/// the links they stand for, and the form is not closed.
#[test]
fn a_created_body_is_a_source_or_creation_demand() {
use crate::transducer::sentence::Demand;
let f = pair_form_uncrystallized(&grid(&["1100", "0000", "0000"]), &grid(&["1100", "0000", "0220"]), None);
let demands: Vec<_> = f.demand_vertices().collect();
assert_eq!(demands.len(), 1, "one demand: {:?}", f.vertices);
let (_, kind, _, links) = demands[0];
assert_eq!(*kind, Demand::Source { multiplicity: 1 }, "the input bar's chart carries onto the new bar");
assert_eq!(links, &[(2, 1, 0, 2), (2, 2, 0, 2)], "the created links, hue 0 before");
assert!(!f.is_closed(), "a sentence with a demand is not closed");
assert_eq!(
    f.vertices.iter().filter(|v| matches!(v, Vertex::Word { .. })).count(),
    1,
    "the carried bar is the one word vertex; the created bar is no word: {:?}",
    f.vertices
);

let f = pair_form_uncrystallized(&grid(&["1100", "0000", "0000"]), &grid(&["1100", "0000", "0222"]), None);
let demands: Vec<_> = f.demand_vertices().collect();
assert_eq!(demands.len(), 1);
assert_eq!(*demands[0].1, Demand::Creation, "no input chart carries onto a bar of three");
}

/// 50.5 — A DEFORMED BODY IS A DEFORMATION DEMAND: an input body no versor
/// carries onto the output body over it names that image's chart and the
/// axes they differ on, and the image is consumed by the deformation — it
/// is not a second demand, a creation; a body that vanished names the
/// empty chart on every axis.
#[test]
fn a_deformed_body_is_a_deformation_demand() {
use crate::transducer::sentence::Demand;
let f = pair_form_uncrystallized(&grid(&["1100"]), &grid(&["1000"]), None);
let kinds: Vec<&Demand> = f.demand_vertices().map(|(_, k, _, _)| k).collect();
assert_eq!(kinds.len(), 1, "one demand: the deformation, its image consumed: {kinds:?}");
assert!(matches!(kinds[0], Demand::TopologicalDeformation { image, axes } if !image.0.is_empty() && !axes.is_empty()), "{kinds:?}");
let (_, _, _, links) = f.demand_vertices().next().expect("the demand");
assert_eq!(links, &[(0, 0, 1, 1), (0, 1, 1, 0)], "the deformation stands for the union of both bodies, hue before and after");

let f = pair_form_uncrystallized(&grid(&["1100", "0000", "0220"]), &grid(&["1100", "0000", "0000"]), None);
let kinds: Vec<&Demand> = f.demand_vertices().map(|(_, k, _, _)| k).collect();
assert_eq!(kinds.len(), 1, "{kinds:?}");
assert!(matches!(kinds[0], Demand::TopologicalDeformation { image, axes } if image.0.is_empty() && !axes.is_empty()), "a vanished body deforms to the empty chart on every axis: {kinds:?}");
for (_, kind, _, _) in f.demand_vertices() {
    assert_eq!(kind.typed(), Ok(()), "every demand the eyes emit is typed");
}
}

/// 50.5 — DEMANDS RIDE THE SENTENCE ONE PER (KIND, CHART): two created bars
/// of one chart are one demand vertex carrying both bodies' links.
#[test]
fn demands_ride_the_sentence_one_per_chart() {
use crate::transducer::sentence::Demand;
let f = pair_form_uncrystallized(&grid(&["1100000", "0000000", "0000000"]), &grid(&["1100000", "0000000", "0220022"]), None);
let demands: Vec<_> = f.demand_vertices().collect();
assert_eq!(demands.len(), 1, "one vertex for one (kind, chart): {:?}", f.vertices);
let (idx, kind, chart, links) = demands[0];
assert_eq!(*kind, Demand::Source { multiplicity: 1 });
assert_eq!(links.len(), 4, "both bars' links ride it");
assert_eq!(f.demands.get(&crate::transducer::sentence::demand_key(kind, chart)), Some(&idx), "and it is read by (kind, chart)");
}

/// 50.5 (F step 5) — THE DEMANDS RIDE THE RULE: over a bundle of two pairs
/// that each create a bar of one chart, the rule form carries one `Source`
/// demand vertex with both pairs' links, and the sentence mints whole —
/// its measure the deflation its words closed and the links its demand
/// asks for — where a form with a remainder and no demand mints nothing.
#[test]
fn demands_ride_the_rule_form_and_the_sentence_mints_whole() {
use crate::transducer::sentence::Demand;
let family = vec![
    (grid(&["1100", "0000", "0000", "0000"]), grid(&["1100", "0000", "0220", "0000"])),
    (grid(&["0000", "1100", "0000", "0000"]), grid(&["0000", "1100", "0000", "0220"])),
];
let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the bundle extracts");
let demands: Vec<_> = rule.form.demand_vertices().collect();
assert_eq!(demands.len(), 1, "one (kind, chart) over the bundle: {:?}", rule.form.vertices);
let (_, kind, _, links) = demands[0];
assert_eq!(*kind, Demand::Source { multiplicity: 1 });
assert_eq!(links.len(), 4, "both pairs' created links ride it: {links:?}");
let word = mint_sentence(&rule, &family, &NoCrystal, 0).expect("a sentence with demands mints whole");
assert!(word.is_sentence());
assert!(word.magnitude().iter().any(|&m| m > 0), "it measures what it asks for: {:?}", word.magnitude());
let back = crate::transducer::sentence::from_element_form(&word).expect("its tape decodes");
assert_eq!(back.demand_vertices().count(), 1, "the demand rides the tape");
assert!(!back.is_closed());

// A DEMAND DECLARES LINKS, NOT AN EXEMPTION. The mint above is spoken
// because its demand names every remainder link — the court asks
// `undeclared_remainder`, and on this family there is none. A voice that
// asked only whether ANY demand vertex existed would mint a form declaring
// one link and leaving twenty, and the court would refuse it.
assert!(
    crate::court::verdict::body::undeclared_remainder(
        &form_deflation(&rule.form, &family, &NoCrystal).1,
        &rule.form
    )
    .is_empty(),
    "this form's demand covers what it leaves, which is why it mints"
);
// Strip the links the demand declares. The form still HOLDS a demand
// vertex — so the old test, which asked only whether one existed, still
// passes it — and it now declares nothing, so the remainder is undeclared
// and the voice withholds.
let mut bare = rule.clone();
for v in &mut bare.form.vertices {
    if let Vertex::Demand { bodies, .. } = v {
        bodies.clear();
    }
}
assert_eq!(
    bare.form.demand_vertices().count(),
    1,
    "it still has a demand vertex, which is all the old clause asked"
);
assert!(
    !crate::court::verdict::body::undeclared_remainder(
        &form_deflation(&bare.form, &family, &NoCrystal).1,
        &bare.form
    )
    .is_empty(),
    "and the court would now find the remainder undeclared"
);
// 50.5 — THE MINT DECLARES WHAT WAS LEFT UNSPOKEN. The empty demand exempts
// nothing, as before; what changed is that the voice no longer withholds
// the form for it — the unspoken links are declared as the demand they
// are, on the chart they stand on, extending the demand already there, and
// the court's own reader then finds nothing undeclared. A demand that
// declares nothing still exempts nothing: it is filled, not honoured.
let spoken = mint_sentence(&bare, &family, &NoCrystal, 0).expect("spoken with the remainder declared");
let back = crate::transducer::sentence::from_element_form(&spoken).expect("its tape decodes");
assert_eq!(back.demand_vertices().count(), 1, "the existing demand is extended, not doubled");
assert!(
    crate::court::verdict::body::undeclared_remainder(
        &form_deflation(&back, &family, &NoCrystal).1,
        &back
    )
    .is_empty(),
    "the court finds nothing undeclared on the spoken form"
);
let declared: Vec<_> = back.demand_vertices().flat_map(|(_, _, _, c)| c.to_vec()).collect();
assert_eq!(declared.len(), 4, "the four created links are the declared remainder: {declared:?}");
}

/// The supports a form hands `star_and_site_faces`, keyed by each
/// vertex's own coordinate. A test writes them in vertex order because
/// that is what it is asserting about; the reader is keyed, so a vertex
/// with no support simply has no entry.
fn supports_at(rows: &[Vec<(i64, i64)>]) -> crate::network::nerve::Keyed<Vec<(i64, i64, i64)>> {
    let mut k = crate::network::nerve::Keyed::new();
    // KEYED BY MEMBER — vertex and chart (D10). These fixtures put every body
    // on the root stratum, which is one chart per vertex, so the member key is
    // the vertex's with that chart's; every support stands in frame 0.
    for (i, s) in rows.iter().enumerate() {
        if !s.is_empty() {
            k.insert(root_member(i as u16), s.iter().map(|&(r, c)| (r, c, 0)).collect());
        }
    }
    k
}

/// The member key of vertex `i` standing at the root stratum.
fn root_member(i: u16) -> Vec<u8> {
    crate::transducer::family::member_key(
        i,
        crate::transducer::stalk::Invariant::default().key().as_bytes(),
    )
}

/// Three touching after-supports: two adjacency faces besides the
/// Frame star, and the 1-skeleton is not a tree.
#[test]
fn a_row_of_three_touching_bodies_has_two_adjacency_faces_and_a_loop() {
let supports = supports_at(&[
    vec![],
    vec![(0, 0), (0, 1)],
    vec![(0, 2), (0, 3)],
    vec![(0, 4), (0, 5)],
]);
let pairs = star_and_site_faces(Some(&root_member(0)), &supports);
// The pairs are MEMBER pairs, so the vertex each names is read back out of
// its key rather than being the whole of it.
let ends: Vec<(u16, u16)> = pairs
    .iter()
    .filter_map(|(a, b)| {
        let (i, _) = crate::transducer::family::read_member_key(a)?;
        let (j, _) = crate::transducer::family::read_member_key(b)?;
        Some((i, j))
    })
    .collect();
let adj: Vec<_> = ends.iter().copied().filter(|&(i, j)| i != 0 && j != 0).collect();
assert_eq!(adj.len(), 2, "two adjacency faces, got {adj:?} from {ends:?}");
let faces: Vec<FormFace> = ends
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

/// Occupancy + ring probes scale with sites, not with n².
#[test]
fn face_work_is_linear_in_occupied_sites() {
let run = |n: usize| -> u64 {
    reset_face_work();
    let mut supports = vec![Vec::new()];
    for i in 0..n {
        supports.push(vec![(0i64, (i as i64) * 3)]);
    }
    let _ = star_and_site_faces(Some(&root_member(0)), &supports_at(&supports));
    face_work()
};
let w = run(8);
let w4 = run(32);
assert!(w > 0 && w4 > w, "work grows with bodies, got {w} then {w4}");
assert!(
    w.checked_mul(5).is_some_and(|cap| w4 <= cap),
    "4N work is ~4× N, not n²: {w} vs {w4}"
);
}

/// A SITE'S OCCUPANTS ARE CHAINED, NOT CLIQUED. The test above gives every
/// body a site of its own, so no site ever holds two occupants and a
/// quadratic in `star_and_site_faces` would never fire under it: a pairing
/// of a site's own vertices as a complete graph, uncounted by
/// `note_face_work`, is invisible to that gauge.
///
/// k bodies on one site are a simplex. The complex needs k−1 faces to
/// connect them; the k(k−1)/2 − (k−1) chords are cycles that bound by
/// construction. Both the face count and the work are asserted here, so
/// neither can go quadratic without a red test.
#[test]
fn one_site_chains_its_occupants_and_does_not_clique_them() {
let run = |k: usize| -> (usize, usize, u64) {
    reset_face_work();
    let mut supports = vec![Vec::new()];
    for _ in 0..k {
        supports.push(vec![(0i64, 0i64)]);
    }
    let pairs = star_and_site_faces(Some(&root_member(0)), &supports_at(&supports));
    let work = face_work();
    // The vertex each member key names, read back out of it.
    let adj = pairs
        .iter()
        .filter(|(a, b)| {
            let ends = (
                crate::transducer::family::read_member_key(a),
                crate::transducer::family::read_member_key(b),
            );
            matches!(ends, (Some((i, _)), Some((j, _))) if i != 0 && j != 0)
        })
        .count();
    (pairs.len(), adj, work)
};
let (all, adj, _) = run(8);
assert_eq!(adj, 7, "eight bodies on one site chain in seven faces, not 28: {adj}");
assert_eq!(all, 15, "plus the eight-body Frame star: {all}");
let (_, adj4, w4) = run(32);
assert_eq!(adj4, 31, "and thirty-two chain in thirty-one, not 496: {adj4}");
// THE LOOPS ARE LINEAR IN THE OCCUPANTS, WHICH IS THE WHOLE POINT. The
// Frame star already spans every vertex, so each intra-site face is a
// chord and carries one loop: Frame → a → b → Frame, whose reading is the
// cocycle condition between two placements — `edge_holonomy` skips the
// star faces, so what it measures is the meet, and that is a real
// obstruction the pivot exists to discharge.
//
// So the chain does not abolish this site's loops; it abolishes the
// DEPENDENT ones. k(k−1)/2 chords carry k(k−1)/2 loops of which only k−1
// are independent, and the rest are sums of those — fake homology inside
// one contractible site. k−1 in, k−1 out.
assert_eq!(Form::loops_of(&run_faces(8), 9).len(), 7, "eight occupants carry seven loops, not 28");
assert_eq!(Form::loops_of(&run_faces(32), 33).len(), 31, "and thirty-two carry thirty-one, not 496");
let (_, _, w) = run(8);
assert!(
    w.checked_mul(5).is_some_and(|cap| w4 <= cap),
    "the work is counted and stays linear in the occupants: {w} vs {w4}"
);
}

/// The faces eight bodies on one site name, as a form would hold them.
fn run_faces(k: usize) -> Vec<FormFace> {
let mut supports = vec![Vec::new()];
for _ in 0..k {
    supports.push(vec![(0i64, 0i64)]);
}
star_and_site_faces(Some(&root_member(0)), &supports_at(&supports))
    .iter()
    .filter_map(|(a, b)| {
        let (i, _) = crate::transducer::family::read_member_key(a)?;
        let (j, _) = crate::transducer::family::read_member_key(b)?;
        Some(FormFace::new(i, j, crate::transducer::alphabet::Word::identity()))
    })
    .collect()
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
let rule = extract_rule_with(&family, &NoCrystal, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("extracts");
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

/// Pairing is ∃V on the shapes, not a digest of the moduli axes.
#[test]
fn pairing_is_carriage_not_a_moduli_digest() {
let family = vec![(grid(&["1100", "0000"]), grid(&["0011", "0000"]))];
let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("a translated bar extracts");
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

/// Y4 (24.4, 20, 21 step 1, W10, Rule 2):
/// A scene of five bodies on three charts answers "the chart whose fiber holds one"
/// without a count compared to a literal.
#[test]
fn a_scene_of_five_bodies_on_three_charts_answers_the_chart_whose_fiber_holds_one() {
    let g = grid(&[
        "110333",
        "000000",
        "220444",
        "000000",
        "005000",
    ]);
    let bodies = perceive(&g);
    assert_eq!(bodies.len(), 5, "five bodies perceived: {bodies:?}");
    let fibers = crate::transducer::family::perceive::bodies_by_chart(&bodies);
    assert_eq!(fibers.len(), 3, "three charts represented: {fibers:?}");

    let dot = bodies.iter().find(|b| b.laminae == vec![(4, 2, 0, 5)]).expect("the dot");
    let chart = crate::transducer::family::perceive::chart_holding_one(&bodies)
        .expect("unique chart holding one body");
    assert_eq!(chart, dot.moduli, "answers the chart whose fiber holds one");
}

/// Y4: A family whose rule selects that body extracts a rule vertex where it extracted a demand before.
#[test]
fn a_family_whose_rule_selects_that_body_extracts_a_rule_vertex_where_it_extracted_a_demand_before() {
    let family = vec![
        (
            grid(&[
                "110304",
                "000304",
                "220000",
                "000005",
            ]),
            grid(&[
                "000000",
                "000000",
                "000000",
                "000005",
            ]),
        ),
        (
            grid(&[
                "110304",
                "000304",
                "220000",
                "000006",
            ]),
            grid(&[
                "000000",
                "000000",
                "000000",
                "000006",
            ]),
        ),
    ];
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let rule = extract_rule(&family, &mut budget).expect("the family extracts");
    let words = rule
        .form
        .vertices
        .iter()
        .filter(|v| matches!(v, Vertex::Word { .. }))
        .count();
    assert_eq!(words, 1, "extracts a rule vertex, got {words}");
    assert!(
        rule.form.demand_vertices().next().is_none(),
        "zero demands where it extracted demands before: {:?}",
        rule.form.vertices
    );
    assert!(rule.form.is_closed(), "the extracted rule form is closed");
}

/// 51.3 — A BODY CARRIES ITS FRAME ON ITS SUPPORT, one carrier: a square
/// perceived in frame 1 has its laminae at `k = 1`, its centroid there,
/// and the same chart as the square in frame 0.
#[test]
fn perception_3d_coordinates_preserve_frame_and_moments() {
    let laminae: Vec<Lamina> = vec![
        (0, 0, 1, 1),
        (0, 1, 1, 1),
        (1, 0, 1, 1),
        (1, 1, 1, 1),
    ];
    let b3d = body_of(laminae);
    assert_eq!(b3d.laminae.len(), 4);
    assert!(b3d.laminae.iter().all(|c| c.2 == 1), "the support stands in its frame");
    assert_eq!(b3d.shape.blade.inertia.len(), 6);
    let c3 = b3d.shape.blade.centroid().expect("3d centroid");
    assert_eq!(c3.2, num_rational::BigRational::from_integer(num_bigint::BigInt::from(1)));
    let at_zero = body_of(vec![(0, 0, 0, 1), (0, 1, 0, 1), (1, 0, 0, 1), (1, 1, 0, 1)]);
    assert_eq!(at_zero.moduli, b3d.moduli, "a frame carries no meaning on the chart (51.1)");
    assert!(carriage(&at_zero, &b3d).is_some(), "and the translation along e₃ carries one onto the other");
}

#[test]
fn six_adjacency_connects_bodies_across_frames_into_one_streamline() {
    let frame0 = grid(&["10", "00"]);
    let frame1 = grid(&["10", "00"]);
    let bodies = perceive_frames(&[frame0, frame1]);
    assert_eq!(bodies.len(), 1, "two adjacent laminae across k form one 3D body streamline");
    assert_eq!(
        bodies[0].shape.blade.shell,
        num_rational::BigRational::from_integer(num_bigint::BigInt::from(2))
    );
}

