// 20 — PERCEPTION: what one pair form costs the sensor, and what the
// adjacency of a scene's bodies is.

use super::common::*;
use crate::transducer::family::*;

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
    edge_transport(&v1, &a1, &a2, &b1, &b2_still).is_none(),
    "a pair no single versor carries is a remainder edge"
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
let v1 = carriage(&a1, &b1).expect("a translated cell carries").v;
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
/// the cells they stand for, and the form is not closed.
#[test]
fn a_created_body_is_a_source_or_creation_demand() {
use crate::transducer::sentence::Demand;
let f = pair_form(&grid(&["1100", "0000", "0000"]), &grid(&["1100", "0000", "0220"]), None);
let demands: Vec<_> = f.demand_vertices().collect();
assert_eq!(demands.len(), 1, "one demand: {:?}", f.vertices);
let (_, kind, _, cells) = demands[0];
assert_eq!(*kind, Demand::Source { multiplicity: 1 }, "the input bar's chart carries onto the new bar");
assert_eq!(cells, &[(2, 1, 0, 2), (2, 2, 0, 2)], "the created cells, hue 0 before");
assert!(!f.is_closed(), "a sentence with a demand is not closed");
assert_eq!(
    f.vertices.iter().filter(|v| matches!(v, Vertex::Word { .. })).count(),
    1,
    "the carried bar is the one word vertex; the created bar is no word: {:?}",
    f.vertices
);

let f = pair_form(&grid(&["1100", "0000", "0000"]), &grid(&["1100", "0000", "0222"]), None);
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
let f = pair_form(&grid(&["1100"]), &grid(&["1000"]), None);
let kinds: Vec<&Demand> = f.demand_vertices().map(|(_, k, _, _)| k).collect();
assert_eq!(kinds.len(), 1, "one demand: the deformation, its image consumed: {kinds:?}");
assert!(matches!(kinds[0], Demand::TopologicalDeformation { image, axes } if !image.0.is_empty() && !axes.is_empty()), "{kinds:?}");
let (_, _, _, cells) = f.demand_vertices().next().expect("the demand");
assert_eq!(cells, &[(0, 0, 1, 1), (0, 1, 1, 0)], "the deformation stands for the union of both bodies, hue before and after");

let f = pair_form(&grid(&["1100", "0000", "0220"]), &grid(&["1100", "0000", "0000"]), None);
let kinds: Vec<&Demand> = f.demand_vertices().map(|(_, k, _, _)| k).collect();
assert_eq!(kinds.len(), 1, "{kinds:?}");
assert!(matches!(kinds[0], Demand::TopologicalDeformation { image, axes } if image.0.is_empty() && !axes.is_empty()), "a vanished body deforms to the empty chart on every axis: {kinds:?}");
for (_, kind, _, _) in f.demand_vertices() {
    assert_eq!(kind.typed(), Ok(()), "every demand the eyes emit is typed");
}
}

/// 50.5 — DEMANDS RIDE THE SENTENCE ONE PER (KIND, CHART): two created bars
/// of one chart are one demand vertex carrying both bodies' cells.
#[test]
fn demands_ride_the_sentence_one_per_chart() {
use crate::transducer::sentence::Demand;
let f = pair_form(&grid(&["1100000", "0000000", "0000000"]), &grid(&["1100000", "0000000", "0220022"]), None);
let demands: Vec<_> = f.demand_vertices().collect();
assert_eq!(demands.len(), 1, "one vertex for one (kind, chart): {:?}", f.vertices);
let (idx, kind, chart, cells) = demands[0];
assert_eq!(*kind, Demand::Source { multiplicity: 1 });
assert_eq!(cells.len(), 4, "both bars' cells ride it");
assert_eq!(f.demands.get(&crate::transducer::sentence::demand_key(kind, chart)), Some(&idx), "and it is read by (kind, chart)");
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
