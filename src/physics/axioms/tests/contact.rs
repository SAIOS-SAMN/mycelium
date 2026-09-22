// 69, A.14 — whether the pair reading holds across a contact family.

use crate::transducer::blade;

/// 69 — DOES THE ADJACENCY BAND SURVIVE A SIZE MISMATCH?
///
/// `δ = (d² − r_a² − r_b²)/(2·r_a·r_b)` and the field admits a pair to
/// the frame test only on `1 < δ ≤ 2`. `δ = 2` was measured as the
/// similarity class of edge-adjacency and proved SCALE-FREE — meaning
/// invariant when BOTH bodies are dilated together, which is what a
/// ratio of this shape guarantees.
///
/// THAT IS NOT THE SAME AS RATIO-FREE. `δ` divides by `r_a·r_b`, so
/// two bodies of DIFFERENT extent that are physically touching need
/// not land on the same `δ` as two equal ones. If the band is a
/// property of equal-sized pairs only, the horizon is turning away
/// bodies that are in contact — which is the famine, and it is the
/// same defect class as V1's cardinality gate: a criterion calibrated
/// at one scale and applied across scales.
///
/// This measures it rather than assuming it either way: a single cell
/// placed against bars of growing length, all touching edge-to-edge.
#[test]
fn the_adjacency_band_against_a_size_mismatch() {
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let mut admitted = 0usize;
    let mut rows: Vec<String> = Vec::new();
    for n in 1..=6i64 {
        // A bar of `n` cells along the row, and one cell touching its
        // left end edge-to-edge. Both are genuinely in contact.
        let bar: Vec<(i64, i64, i64)> = (0..n).map(|c| (0, c, 0)).collect();
        let cell = vec![(0, -1, 0)];
        let (a, b) = (blade::shape_of(&cell), blade::shape_of(&bar));
        let Some(face) = blade::Interface::between(&a.blade, &b.blade) else {
            rows.push(format!("n={n}: no interface"));
            continue;
        };
        let sq = &face.strain.num * &face.strain.num;
        let four = q(4);
        let adjacent = face.strain.num > q(0)
            && sq > face.strain.den2
            && sq <= four * &face.strain.den2;
        if adjacent {
            admitted += 1;
        }
        rows.push(format!(
            "n={n}: δ² = {}/{} → {} ({:?})",
            sq,
            face.strain.den2,
            if adjacent { "ADMITTED" } else { "turned away" },
            face.strain.side()
        ));
    }
    for r in &rows {
        eprintln!("  {r}");
    }
    eprintln!("  admitted {admitted} of 6 touching pairs");
    // WHICH BOUND DOES THE CLIQUE TEST ACTUALLY NEED? Eight bodies on
    // ONE SITE overlap, so their nuclei nest and `δ < 1` — the LOWER
    // bound. If the upper bound is not what keeps a site's occupants
    // from cliquing, it protects nothing and is pure famine.
    let shared = vec![(0, 0, 0), (0, 1, 0)];
    let also = vec![(0, 0, 0), (1, 0, 0)];
    let (sa, sb) = (blade::shape_of(&shared), blade::shape_of(&also));
    if let Some(face) = blade::Interface::between(&sa.blade, &sb.blade) {
        let sq = &face.strain.num * &face.strain.num;
        eprintln!(
            "  two bodies SHARING a site: δ² = {}/{} side {:?} — \
             caught by the LOWER bound, not the upper",
            sq,
            face.strain.den2,
            face.strain.side()
        );
    }
    // The equal-sized case must admit, or the band is wrong outright.
    assert!(
        rows[0].contains("ADMITTED"),
        "two unit cells edge-to-edge must be adjacent: {}",
        rows[0]
    );
}

/// A.14 — THE PAIR READING IS DIRECTIONAL, AND THE RANK-1 CARRIER
/// CANNOT CARRY IT (69, ruled 2026-09-22).
///
/// THE OBLIGATION, stated so it can fail: fix a body `A` and let `B`
/// range over a family every member of which is IN CONTACT with `A`.
/// A sound pair reading is CONSTANT on that family. Contact is one
/// relation; a reading that varies across it is reading something
/// else.
///
/// THE CURRENT CARRIER FAILS IT. `radius2() = Tr(I)/M₀` is the MEAN
/// of the extent over all directions — the deviatoric, which is
/// exactly the part saying extent VARIES with direction, is computed,
/// used to orient bodies (V1), and then discarded when they are
/// measured. So `δ` cannot tell a long body from a distant one.
///
/// THE OBVIOUS REPAIR IS RANK-2 AND IT IS MEASURED HERE, not assumed:
/// the directional second moment `r²(v) = (v·I·v)/((v·v)·M₀)` along
/// the separation `v`, exact over ℚ with no root — the ratio of two
/// quadratics never forms `‖v‖`.
#[test]
fn a14_the_pair_reading_must_be_constant_on_a_contact_family() {
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let mut rank1: Vec<String> = Vec::new();
    let mut rank2: Vec<String> = Vec::new();
    for n in 1..=6i64 {
        let bar: Vec<(i64, i64, i64)> = (0..n).map(|c| (0, c, 0)).collect();
        let cell = vec![(0, -1, 0)];
        let (a, b) = (blade::shape_of(&cell), blade::shape_of(&bar));
        let (ba, bb) = (&a.blade, &b.blade);
        // RANK 1 — what the horizon reads today.
        let face = blade::Interface::between(ba, bb).expect("an interface");
        let sq = &face.strain.num * &face.strain.num;
        rank1.push(format!("{}", &sq / &face.strain.den2));
        // RANK 2 — the same reading on the DIRECTIONAL extent.
        let (ca, cb) = (ba.centroid().expect("a"), bb.centroid().expect("b"));
        let (vr, vc) = (&cb.0 - &ca.0, &cb.1 - &ca.1);
        let vv = &vr * &vr + &vc * &vc;
        // `v·I·v` on the sensor plane: Ixx, Iyy, Ixy are inertia[0..3].
        let dir = |bl: &blade::Blade| -> num_rational::BigRational {
            // `M₀` is the body's own lamina count on this base.
            let m0 = q(bl.support.len() as i64);
            let quad = &vr * &vr * &bl.inertia[0]
                + &vc * &vc * &bl.inertia[1]
                + q(2) * &vr * &vc * &bl.inertia[2];
            quad / (&vv * &m0)
        };
        let (ra2, rb2) = (dir(ba), dir(bb));
        let num = &vv - &ra2 - &rb2;
        let den2 = q(4) * &ra2 * &rb2;
        rank2.push(format!("{}", (&num * &num) / &den2));
    }
    eprintln!("  contact family (cell against a bar of n cells, edge to edge):");
    eprintln!("    rank-1 δ² (today, Tr(I)/M₀): {rank1:?}");
    eprintln!("    rank-2 δ² (directional v·I·v): {rank2:?}");
    // THE RANK-1 CARRIER IS NOT CONSTANT — this is the defect, pinned.
    assert!(
        rank1.iter().any(|x| x != &rank1[0]),
        "if this ever passes, the rank-1 reading became constant and A.14 is moot"
    );
    // AND THE OBLIGATION IS RECORDED AS THE THING A.14 OWES: whichever
    // carrier replaces the nucleus must make this vector constant.
    // Asserted here only as the measurement it is, so the row cannot
    // be closed by an implementation that does not achieve it.
    let rank2_constant = rank2.iter().all(|x| x == &rank2[0]);
    eprintln!(
        "    rank-2 constant on the family: {rank2_constant} \
         — A.14 is satisfied only when this reads true"
    );
}

/// IS `δ = 2` THE ADJACENCY SIGNATURE, AND DOES IT SURVIVE SIZE?
///
/// The claim: two unit laminae sharing an edge sit at `δ = 2` exactly,
/// so `δ² ≤ 4` is a DERIVED contact gate rather than a dialled one.
/// `‖ΔP‖² = 1`, `r² = 1/6` each, so
///
/// ```text
///     num  = 1 − 1/6 − 1/6 = 2/3
///     den2 = 4 · (1/6)(1/6) = 1/9
///     δ²   = (2/3)² / (1/9) = 4
/// ```
///
/// The second half is the one that decides whether it is usable: the
/// gyration radius GROWS with the body, so a signature calibrated on
/// single laminae may not hold for adjacent blocks. This measures it
/// rather than assuming either way.
#[test]
fn the_adjacency_signature_and_whether_it_survives_size() {
    use crate::transducer::blade::{self, Strain};
    let q = |n: i64| crate::core::Q::from_integer(n.into());
    let d2 = |a: &blade::Blade, b: &blade::Blade| {
        let s = Strain::between(a, b).expect("centroids");
        &s.num * &s.num / s.den2
    };

    // ONE LAMINA EACH, EDGE-ADJACENT.
    let u0 = blade::shape_of(&[(0, 0, 0)]).blade;
    let u1 = blade::shape_of(&[(0, 1, 0)]).blade;
    assert_eq!(u0.radius2(), q(1) / q(6), "the +1/12 floor, twice");
    assert_eq!(d2(&u0, &u1), q(4), "edge-adjacent unit laminae sit at δ² = 4");

    // DIAGONAL, and SEPARATED BY ONE CELL.
    let diag = blade::shape_of(&[(1, 1, 0)]).blade;
    let gap = blade::shape_of(&[(0, 2, 0)]).blade;
    assert_eq!(d2(&u0, &diag), q(25), "a corner neighbour is δ = 5");
    assert_eq!(d2(&u0, &gap), q(121), "a one-cell gap is δ = 11");

    // So `δ² ≤ 4` admits EDGE adjacency and refuses corner adjacency,
    // which is 51.3's own 4-neighbour convention in the plane arriving
    // from the metric rather than from a stencil.

    // NOW SIZE. Two 3x3 blocks sharing an edge: centroids 3 apart.
    let block = |r0: i64, c0: i64| {
        let mut cells = Vec::new();
        for r in 0..3 {
            for c in 0..3 {
                cells.push((r0 + r, c0 + c, 0));
            }
        }
        blade::shape_of(&cells).blade
    };
    // NOW SIZE — and it is SCALE-FREE, measured across n = 1..5.
    //
    // An n×n block has `r² = n²/6`, so two of them sharing an edge have
    // `‖ΔP‖² = n²` and
    //
    //     num  = n² − n²/6 − n²/6 = 2n²/3
    //     den2 = 4(n²/6)² = n⁴/9
    //     δ²   = (2n²/3)² / (n⁴/9) = 4          for EVERY n
    //
    // `δ² = 4` is therefore not a constant calibrated on unit laminae.
    // It is the SIMILARITY CLASS of edge-adjacency, and `δ` is a
    // similarity invariant, so two adjacent n-blocks are similar to two
    // adjacent unit cells and read identically.
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
        assert_eq!(
            a.radius2(),
            q(n * n) / q(6),
            "an n×n block carries r² = n²/6"
        );
        assert_eq!(d2(&a, &b), q(4), "and edge-adjacency is δ² = 4 at n = {n}");
    }
    let b0 = block(0, 0);
    assert!(b0.radius2() > u0.radius2(), "a block is larger than a lamina");
    assert!(d2(&b0, &block(0, 60)) > q(4), "and distance is still refused");
}
