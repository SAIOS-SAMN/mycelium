// 69, A.14 — whether the pair reading holds across a contact family, and the
// tether's reading of adjacency, recorded.

use crate::transducer::blade;

/// `δ²` READ OFF THE TETHER: `⟨V⟩₀² / ⟨VṼ⟩₀`, the projective reading of `V`'s
/// class (72.1). Equal, value for value, to the `num²/den2` the split
/// `Strain` carried.
fn delta2(a: &blade::Blade, b: &blade::Blade) -> crate::core::Q {
    let v = blade::tether(a, b).expect("both nuclei");
    &v.scalar_part() * &v.scalar_part() / v.geo(&v.reverse()).scalar_part()
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
/// extent over all directions, so `δ` cannot tell a long body from a
/// distant one. The rank-2 reading along the separation is printed beside
/// it as the record of 69.3.
///
/// PH3, 69.3 as ruled — THE OBLIGATION IS MET BY THE LATTICE: contact is
/// the cell the two cubes share, read by grade (BQ2), and that reading is
/// asserted constant on the family.
#[test]
fn a14_the_pair_reading_must_be_constant_on_a_contact_family() {
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let mut rank1: Vec<String> = Vec::new();
    let mut rank2: Vec<String> = Vec::new();
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
        let (ba, bb) = (&a.blade, &b.blade);
        // RANK 1 — the tether's scale-free reading.
        rank1.push(format!("{}", delta2(ba, bb)));
        // RANK 2 — the same reading on the DIRECTIONAL extent.
        let (ca, cb) = (ba.centroid().expect("a"), bb.centroid().expect("b"));
        let v: Vec<num_rational::BigRational> = cb.iter().zip(&ca).map(|(x, y)| x - y).collect();
        let vv = v.iter().fold(q(0), |s, x| s + x * x);
        let dir = |bl: &blade::Blade| -> num_rational::BigRational {
            let m0 = q(bl.support.len() as i64);
            let mut quad = q(0);
            for (i, vi) in v.iter().enumerate() {
                for (j, vj) in v.iter().enumerate() {
                    if let Some(m) = bl.moment2(i, j) {
                        quad += vi * vj * m;
                    }
                }
            }
            quad / (&vv * &m0)
        };
        let (ra2, rb2) = (dir(ba), dir(bb));
        let num = &vv - &ra2 - &rb2;
        let den2 = q(4) * &ra2 * &rb2;
        rank2.push(format!("{}", (&num * &num) / &den2));
    }
    eprintln!("  contact family (cell against a bar of n cells, edge to edge):");
    eprintln!("    rank-1 δ² (the tether, Tr(I)/M₀): {rank1:?}");
    eprintln!("    rank-2 δ² (directional v·I·v): {rank2:?}");
    assert!(
        rank1.iter().any(|x| x != &rank1[0]),
        "if this ever passes, the rank-1 reading became constant and A.14 is moot"
    );
    let rank2_constant = rank2.iter().all(|x| x == &rank2[0]);
    eprintln!("    rank-2 constant on the family: {rank2_constant} (the record of 69.3, not the obligation)");
    eprintln!("    shared cells by grade (the lattice's reading): {by_grade:?}");
    assert!(!by_grade[0].is_empty(), "a cell edge to edge with a bar shares a cell with it");
    assert!(
        by_grade.iter().all(|g| g == &by_grade[0]),
        "A.14's obligation: the pair reading is constant on the contact family: {by_grade:?}"
    );
}

/// THE TETHER'S READING OF ADJACENCY, AND THAT IT IS SCALE-FREE — kept as
/// a record, gating nothing.
///
/// Two unit laminae sharing an edge read `δ² = 4`, a corner neighbour 25,
/// a one-cell gap 121; two `n×n` blocks sharing an edge read 4 at every `n`,
/// because `δ` is a similarity invariant. The band that turned these values
/// into a gate is retired (BQ5a, SQ1 item 2); the values stand as what the
/// tether's class reads.
#[test]
fn the_tether_reads_adjacency_scale_free() {
    let q = |n: i64| crate::core::Q::from_integer(n.into());
    let u0 = blade::shape_of(&[(0, 0, 0)]).blade;
    let u1 = blade::shape_of(&[(0, 1, 0)]).blade;
    assert_eq!(u0.radius2(), q(1) / q(6), "the +1/12 floor, twice");
    assert_eq!(delta2(&u0, &u1), q(4), "edge-adjacent unit laminae read δ² = 4");
    let diag = blade::shape_of(&[(1, 1, 0)]).blade;
    let gap = blade::shape_of(&[(0, 2, 0)]).blade;
    assert_eq!(delta2(&u0, &diag), q(25), "a corner neighbour is δ = 5");
    assert_eq!(delta2(&u0, &gap), q(121), "a one-cell gap is δ = 11");
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
        assert_eq!(delta2(&a, &b), q(4), "and edge-adjacency is δ² = 4 at n = {n}");
    }
}
