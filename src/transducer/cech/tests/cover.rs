// 78.123 A — ONE COVER READ, TWO COEFFICIENT GROUPS: the lane group a
// domain's ledger tree carries, and the versors the minds' complex holds —
// the same cycle read the same way, every holonomy exact, an obstruction
// localized to its triangle.

use crate::transducer::alphabet::Rotor;
use crate::transducer::cech::coefficient::{Coefficient, Lanes, Turned};
use crate::transducer::cech::cover::{CellReading, Cover, CycleUnread};
use num_rational::BigRational;

fn v(b: u8) -> Vec<u8> {
    vec![b; 4]
}

fn who(b: u8) -> [u8; 32] {
    [b; 32]
}

fn lanes(nets: &[([u8; 32], [i128; 4])]) -> Lanes {
    Lanes::of(nets).expect("within the carrier")
}

/// THE LANE GROUP: exact sums per identity per lane, zeros leaving the one
/// writing, the inverse the negation — and four lanes never summed into one.
#[test]
fn the_lane_group_sums_exactly_per_identity_per_lane() {
    let a = lanes(&[(who(1), [5, 0, 2, 0]), (who(2), [-5, 1, 0, 0])]);
    let b = lanes(&[(who(2), [5, -1, 0, 0]), (who(3), [0, 0, 0, 7])]);
    let sum = a.compose(&b).expect("within the carrier");
    assert_eq!(sum.nets(), &[(who(1), [5, 0, 2, 0]), (who(3), [0, 0, 0, 7])], "identity 2 cancels and leaves the writing");
    assert!(a.compose(&a.inverse().expect("negated")).expect("summed").is_identity(), "g · g⁻¹ is the empty sum");
    let past = lanes(&[(who(1), [i128::MAX, 0, 0, 0])]);
    assert_eq!(past.compose(&lanes(&[(who(1), [1, 0, 0, 0])])), None, "a lane past its carrier is refused, never wrapped");
}

/// A LEDGER TRIANGLE: what crosses `a → b → c` and back nets to nothing — the
/// cell glues; one fold that made value from nowhere and the triangle is
/// obstructed by exactly that value, on exactly that lane, for exactly that
/// identity.
#[test]
fn a_ledger_triangle_glues_and_an_obstruction_names_what_was_made() {
    let mut c: Cover<Lanes> = Cover::new();
    c.set_edge(&v(1), &v(2), lanes(&[(who(7), [5, 0, 0, 0])])).expect("set");
    c.set_edge(&v(2), &v(3), lanes(&[(who(7), [3, 0, 0, 0])])).expect("set");
    c.set_edge(&v(3), &v(1), lanes(&[(who(7), [-8, 0, 0, 0])])).expect("set");
    assert!(matches!(c.fill(&[v(1), v(2), v(3)]), CellReading::Filled));
    c.set_edge(&v(3), &v(1), lanes(&[(who(7), [-8, 0, 0, 0]), (who(8), [0, 2, 0, 0])])).expect("set");
    let CellReading::Obstructed { holonomy, triangle } = c.fill(&[v(3), v(2), v(1)]) else { panic!("obstructed") };
    assert_eq!(triangle, [v(1), v(2), v(3)], "named in its vertices' order");
    assert_eq!(holonomy.nets(), &[(who(8), [0, 2, 0, 0])], "exactly what was made, where");
}

/// A CELL OF HIGHER DEGREE is read on every triangle it holds: a tetrahedron
/// of ledgers glues where all four triangles close; changing one edge
/// obstructs a triangle that holds it — and only a missing edge leaves a
/// cycle unread, by name.
#[test]
fn a_tetrahedron_glues_on_all_its_triangles_and_one_edge_obstructs_it() {
    // A potential on four ledgers: the edge a → b carries φ(b) − φ(a), so
    // every cycle closes.
    let phi = |b: u8| -> i128 { i128::from(b) * 10 };
    let mut c: Cover<Lanes> = Cover::new();
    for (a, b) in [(1u8, 2u8), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)] {
        c.set_edge(&v(a), &v(b), lanes(&[(who(1), [phi(b) - phi(a), 0, 0, 0])])).expect("set");
    }
    let cell = [v(1), v(2), v(3), v(4)];
    assert!(matches!(c.fill(&cell), CellReading::Filled), "every triangle closes");
    c.set_edge(&v(2), &v(4), lanes(&[(who(1), [phi(4) - phi(2), 0, 0, 3])])).expect("set");
    let CellReading::Obstructed { triangle, .. } = c.fill(&cell) else { panic!("obstructed") };
    assert!(triangle.contains(&v(2)) && triangle.contains(&v(4)), "a triangle holding the changed edge: {triangle:?}");
    let mut sparse: Cover<Lanes> = Cover::new();
    sparse.set_edge(&v(1), &v(2), Lanes::identity()).expect("set");
    assert!(matches!(sparse.fill(&[v(1), v(2), v(3)]), CellReading::Unread(CycleUnread::EdgeAbsent { .. })));
}

/// THE SAME READING OVER THE VERSORS: rotors `R`, `S` and the inverse of `S·R`
/// around a triangle compose to the identity ray — the cell glues; another
/// third side obstructs it. One cover, two coefficient groups.
#[test]
fn the_versor_group_reads_the_same_cover_the_same_way() {
    let q = |n: i64| BigRational::from_integer(n.into());
    let turned = |s: i64, b: i64| Turned { rotor: Rotor::scalar(q(s)).add(&Rotor::blade(0b0011, q(b))), carriage: crate::transducer::element::identity_versor() };
    let (r, s) = (turned(2, 1), turned(3, 4));
    let closing = r.compose(&s).and_then(|sr| sr.inverse()).expect("composed");
    let mut c: Cover<Turned> = Cover::new();
    c.set_edge(&v(1), &v(2), r.clone()).expect("set");
    c.set_edge(&v(2), &v(3), s.clone()).expect("set");
    c.set_edge(&v(3), &v(1), closing).expect("set");
    assert!(matches!(c.fill(&[v(1), v(2), v(3)]), CellReading::Filled), "R, S and (S·R)⁻¹ close");
    c.set_edge(&v(3), &v(1), turned(1, 1)).expect("set");
    assert!(matches!(c.fill(&[v(1), v(2), v(3)]), CellReading::Obstructed { .. }));
}

/// 78.123 C — THE MINDS' COMPLEX READS ITS CYCLES BY THE ONE HOLONOMY: a
/// mind's triangle of three non-commuting turns, read by `Complex::holonomy`,
/// is exactly the path product `r₁₂ · r₂₃ · r₃₁` on both halves — and the
/// same element a `Cover<Turned>` holding those edges reads. One machinery,
/// the minds' versors and the ledger's lanes and rates.
#[test]
fn the_minds_complex_reads_its_cycles_by_the_one_holonomy() {
    use crate::transducer::cech::Complex;
    let q = |n: i64| BigRational::from_integer(n.into());
    let turn = |s: i64, blade: usize, b: i64| Rotor::scalar(q(s)).add(&Rotor::blade(blade, q(b)));
    let one = crate::transducer::element::identity_versor;
    let mut mind = Complex::new();
    mind.set_edge(v(1), v(2), turn(2, 0b0011, 1), one(), [1, 0, 0, 0]);
    mind.set_edge(v(2), v(3), turn(3, 0b0110, 4), one(), [1, 0, 0, 0]);
    mind.set_edge(v(3), v(1), turn(1, 0b0101, 2), one(), [1, 0, 0, 0]);
    let edge = |a: u8, b: u8| mind.edge(&v(a), &v(b)).expect("an edge stands");
    let (r12, r23, r31) = (edge(1, 2).0, edge(2, 3).0, edge(3, 1).0);
    let (h, _) = mind.holonomy(&v(1), &v(2), &v(3)).expect("the triangle reads");
    assert!(h.projective_eq(&r12.geo(&r23).geo(&r31)), "path order, as the minds have always composed");
    assert!(!h.projective_eq(&r31.geo(&r23).geo(&r12)), "the other order is another element");
    let mut cover: Cover<Turned> = Cover::new();
    for (a, b) in [(1, 2), (2, 3), (3, 1)] {
        let (rotor, carriage) = edge(a, b);
        cover.set_edge(&v(a), &v(b), Turned { rotor, carriage }).expect("set");
    }
    let g = cover.holonomy(&[&v(1), &v(2), &v(3)]).expect("read");
    assert!(g.rotor.projective_eq(&h), "the cover in the versor group reads the mind's own holonomy");
}
