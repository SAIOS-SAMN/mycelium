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
