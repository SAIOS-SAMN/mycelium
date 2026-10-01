// 69, A.14 — whether the pair reading holds across a contact family, and the
// tether's reading of adjacency, recorded.

use crate::transducer::blade;

/// THE TETHER'S CLASS: its conjugacy class as coordinates (72.1), the
/// scale-free reading of `V` — a similarity conjugates the tether, so its
/// class stands — read in the algebra, never the squared ratio
/// `⟨V⟩₀² / ⟨VṼ⟩₀` this read before 78.59.
fn tether_class(a: &blade::Blade, b: &blade::Blade) -> crate::transducer::address::Address {
    blade::tether(a, b).expect("both nuclei").class_key().expect("a versor has a class")
}

// `the_adjacency_band_against_a_size_mismatch` STOOD HERE and is retired
// with the band (BQ5a, SQ1 item 2): it measured whether `1 < δ ≤ 2` turned
// away touching bodies of different extent, and it did. Whether two bodies
// touch is now the lattice's alone, and the obligation below asserts that
// reading constant on the same family.

/// A.14 — THE PAIR READING IS DIRECTIONAL, AND THE RANK-1 CARRIER
/// CANNOT CARRY IT (69, ruled 2026-09-22).
///
/// THE OBLIGATION, stated so it can fail: fix a body `A` and let `B`
/// range over a family every member of which is IN CONTACT with `A`.
/// A sound pair reading is CONSTANT on that family. Contact is one
/// relation; a reading that varies across it is reading something
/// else.
///
/// THE RANK-1 READING FAILS IT: `radius2() = Tr(I)/M₀` is the mean of the
/// extent over all directions, so the tether's class cannot tell a long body
/// from a distant one. (The rank-2 record of 69.3 — a Euclidean quadratic
/// form, `Σ vᵢ²` and a squared ratio — is retired with δ², 78.59.)
///
/// PH3, 69.3 as ruled — THE OBLIGATION IS MET BY THE LATTICE: contact is
/// the cell the two cubes share, read by grade (BQ2), and that reading is
/// asserted constant on the family.
#[test]
fn a14_the_pair_reading_must_be_constant_on_a_contact_family() {
    let mut rank1: Vec<crate::transducer::address::Address> = Vec::new();
    let mut by_grade: Vec<Vec<usize>> = Vec::new();
    for n in 1..=6i64 {
        let bar: Vec<_> = (0..n).map(|c| (0i64, c, 0i64)).collect();
        let cell = vec![(0i64, -1i64, 0i64)];
        let (a, b) = (blade::shape_of(&cell), blade::shape_of(&bar));
        let union: Vec<crate::transducer::base::Site> =
            cell.iter().chain(bar.iter()).map(|&t| t.into()).collect();
        let in_a = |i: usize| i < cell.len();
        let mut grades: Vec<usize> = crate::transducer::sentence::shared_cells(&union)
            .into_iter()
            .filter(|c| in_a(c.i) != in_a(c.j))
            .map(|c| c.grade)
            .collect();
        grades.sort_unstable();
        by_grade.push(grades);
        // RANK 1 — the tether's class, its scale-free reading.
        rank1.push(tether_class(&a.blade, &b.blade));
    }
    eprintln!("    shared cells by grade (the lattice's reading): {by_grade:?}");
    assert!(
        rank1.iter().any(|x| x != &rank1[0]),
        "if this ever passes, the rank-1 reading became constant and A.14 is moot"
    );
    assert!(!by_grade[0].is_empty(), "a cell edge to edge with a bar shares a cell with it");
    assert!(
        by_grade.iter().all(|g| g == &by_grade[0]),
        "A.14's obligation: the pair reading is constant on the contact family: {by_grade:?}"
    );
}

/// THE TETHER'S READING OF ADJACENCY, AND THAT IT IS SCALE-FREE — kept as
/// a record, gating nothing. Edge-adjacent unit compartments, a corner
/// neighbour and a one-cell gap read three tether classes; two `n×n` blocks
/// sharing an edge read the edge's class at every `n`, because a similarity
/// conjugates the tether and its class stands. Read in the algebra — the
/// squared ratio `δ²` this recorded as 4, 25 and 121 is retired (78.59).
#[test]
fn the_tether_reads_adjacency_scale_free() {
    let q = |n: i64| crate::core::Q::from_integer(n.into());
    let u0 = blade::shape_of(&[(0, 0, 0)]).blade;
    let u1 = blade::shape_of(&[(0, 1, 0)]).blade;
    assert_eq!(u0.radius2(), q(1) / q(6), "the +1/12 floor, twice");
    let edge = tether_class(&u0, &u1);
    let diag = tether_class(&u0, &blade::shape_of(&[(1, 1, 0)]).blade);
    let gap = tether_class(&u0, &blade::shape_of(&[(0, 2, 0)]).blade);
    assert!(edge != diag && edge != gap && diag != gap, "edge, corner and gap read three classes");
    for n in 1..=5i64 {
        let cells = |r0: i64, c0: i64| {
            let mut v = Vec::new();
            for r in 0..n {
                for c in 0..n {
                    v.push((r0 + r, c0 + c, 0));
                }
            }
            blade::shape_of(&v).blade
        };
        let (a, b) = (cells(0, 0), cells(0, n));
        assert_eq!(a.radius2(), q(n * n) / q(6), "an n×n block carries r² = n²/6");
        assert_eq!(tether_class(&a, &b), edge, "and edge-adjacency reads the edge's class at n = {n}");
    }
}
