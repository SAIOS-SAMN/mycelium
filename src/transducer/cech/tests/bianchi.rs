// 14 — the class at degree three is the Bianchi form, and one fetched
// vertex costs the same whatever the complex already holds.

use super::common::*;
use crate::transducer::alphabet::Rotor;
use crate::transducer::cech::*;

/// K4 with an identity 1-cochain is exact at every degree: the tetra's
/// coboundary coefficient is the identity and it fills by the identity
/// gauge, with no class anywhere on the link.
///
/// Each half is asserted on its own: `class` reads `None` for every fill
/// that is not `Class`, so a disjunction of the two could not fail.
#[test]
fn fill_fills_exact_k4_cochain_in_every_degree() {
    let mut c = exact_k4();
    let tets = c.link(&vs()[0], 3);
    assert_eq!(tets.len(), 1, "the link of one vertex of K4 is one tetra");
    let key = tets[0].clone();
    let coeff = c.simplex(&key).expect("the tetra is held").coefficient.clone();
    assert!(coeff.is_identity(), "an exact cochain's coboundary is the identity: {coeff:?}");
    match c.fill(&key) {
        CoboundaryOutcome::Attached(Fill::Filled(g, carriage)) => {
            assert!(g.is_identity(), "the gauge that closed it is the identity");
            assert!(
                crate::transducer::element::same_versor(
                    &carriage,
                    &crate::transducer::element::identity_versor()
                ),
                "and carries no placement"
            );
        }
        other => panic!("an exact cochain fills by a gauge, not {other:?}"),
    }
    assert!(c.class(&key).is_none(), "a filled simplex carries no class");
    for k in c.link_of(&vs()[0]) {
        assert!(c.class(&k).is_none(), "no simplex of an exact link carries a class");
    }
}

/// An INFINITE-order edge is a class at degree 2, and the class names the
/// rotor that caused it. `pythag5` is the (3,4,5) rotor on the sensor
/// plane: it has no finite order, so the triangle carrying it cannot be
/// congruent and cannot be filled. The claim names the simplex, the class
/// and the order; a congruence anywhere on the link is not it.
#[test]
fn fill_reads_infinite_order_edge_as_class() {
    let mut c = exact_k4();
    let vs = vs();
    c.set_edge(&vs[0], &vs[1], pythag5(), ident(), vol());
    c.coboundary_on_link(&vs[0], |_| vol());
    let key = triangle(&c, &vs[0], &[&vs[0], &vs[1], &vs[2]]);
    match c.fill(&key) {
        CoboundaryOutcome::Attached(Fill::Class(kappa, r)) => {
            assert!(
                r.projective_eq(&pythag5()) || r.reverse().projective_eq(&pythag5()),
                "the class names the rotor that caused it, not some other: {r:?}"
            );
            assert!(
                crate::transducer::element::same_versor(
                    &kappa.carriage,
                    &crate::transducer::element::identity_versor()
                ),
                "the triangle's placements close, so the class carries no placement residue"
            );
        }
        other => panic!("an infinite-order holonomy is a class, not {other:?}"),
    }
    let (_, r, d) = c.class(&key).expect("the filled class is readable back");
    assert_eq!(d, 2, "it is a class at degree two");
    assert_eq!(
        r.order(),
        Some(crate::transducer::alphabet::Order::Infinite),
        "the (3,4,5) rotor is a versor of infinite order — `None` would be no class at all"
    );
}

/// A FINITE-order edge is a congruence at its own period, and the period
/// is the rotor's order and not a number chosen at the call site.
/// `sheet` is a 180° turn on `e₁∧e₂` — a letter of the seventeen acting
/// OFF the sensor plane, which is exactly the transition a versor-only
/// section could not hold at all.
#[test]
fn fill_reads_off_plane_finite_edge_as_congruent() {
    // `sheet()` is the DOUBLED 90° turn on `e₁∧e₂` — a 180° turn, which
    // sends `e₁ → −e₁` and so KEEPS the sensor plane and does have a block
    // there. The single 90° turn is the one that leaves it.
    assert!(
        crate::transducer::alphabet::rotor_block_on(&sheet(), (0, 1)).is_some(),
        "a 180° turn on e₁∧e₂ keeps the sensor plane"
    );
    let s = off_plane_turn_90();
    assert_eq!(
        s.order(),
        Some(crate::transducer::alphabet::Order::Finite(4)),
        "a 90° turn closes at four, on whichever plane it acts"
    );
    assert!(
        crate::transducer::alphabet::rotor_block_on(&s, (0, 1)).is_none(),
        "and it takes e₀ off the sensor plane, so it has no block there and no versor — \
         the transition Cl(3,1) cannot carry"
    );
    let mut c = exact_k4();
    let vs = vs();
    c.set_edge(&vs[1], &vs[2], s.clone(), ident(), vol());
    c.coboundary_on_link(&vs[0], |_| vol());
    let key = triangle(&c, &vs[0], &[&vs[0], &vs[1], &vs[2]]);
    match c.fill(&key) {
        CoboundaryOutcome::Attached(Fill::Congruent(_, g, ell)) => {
            assert_eq!(ell, 4, "the period is the rotor's order");
            assert_eq!(g.order(), Some(crate::transducer::alphabet::Order::Finite(4)));
            assert!(
                g.projective_eq(&s) || g.reverse().projective_eq(&s),
                "and the filed rotor is the one that caused it: {g:?}"
            );
        }
        other => panic!("a 90°-turn holonomy is congruent at four, not {other:?}"),
    }
}

/// K4 with every edge the identity, sections placed, and the link climbed.
fn exact_k4() -> Complex {
    let mut c = Complex::new();
    let vs = vs();
    for x in &vs {
        c.set_section(x, Section::at(x.clone(), Rotor::identity()), vol());
    }
    for i in 0..4 {
        for j in i + 1..4 {
            c.set_edge(&vs[i], &vs[j], Rotor::identity(), ident(), vol());
        }
    }
    c.coboundary_on_link(&vs[0], |_| vol());
    c
}

fn vs() -> [crate::transducer::address::Address; 4] {
    [v(1), v(2), v(3), v(4)]
}

fn ident() -> crate::transducer::element::Versor {
    crate::transducer::element::identity_versor()
}

/// The key of the named triangle on `at`'s link, or a panic saying it is
/// absent — never a silent skip that would leave the test asserting
/// nothing.
fn triangle(
    c: &Complex,
    at: &crate::transducer::address::Address,
    want: &[&crate::transducer::address::Address],
) -> Vec<u8> {
    let want = sorted_verts(want.iter().map(|a| a.as_bytes().to_vec()));
    for k in c.link_of(at.as_bytes()) {
        if c.simplex(&k).is_some_and(|s| s.vertices == want) {
            return k;
        }
    }
    panic!("the triangle {want:?} is not on the link of {at:?}");
}

/// Work of filling the link of one vertex does not grow with N isolated
/// sections on the same complex.
#[test]
fn coboundary_on_link_ignores_n() {
    use crate::network::nerve::work;
    let run = |n: u64| -> u64 {
        let mut c = Complex::new();
        let (a, b, cc) = (v(1), v(2), v(3));
        for x in [&a, &b, &cc] {
            c.set_section(x, Section::at((*x).clone(), Rotor::identity()), vol());
        }
        let id = Rotor::identity();
        c.set_edge(&a, &b, id.clone(), ident(), vol());
        c.set_edge(&a, &cc, id.clone(), ident(), vol());
        c.set_edge(&b, &cc, id, ident(), vol());
        for i in 0..n {
            let x = crate::transducer::address::Address::from((1000 + i).to_le_bytes().to_vec());
            c.set_section(&x, Section::at(x.clone(), Rotor::identity()), vol());
        }
        work::reset();
        c.coboundary_on_link(&a, |_| vol());
        for k in c.link(&a, 2) {
            let _ = c.fill(&k);
        }
        work::total()
    };
    let a = run(32);
    let b = run(128);
    eprintln!("work fetched-vertex N=32 {a} N=128 {b}");
    assert_eq!(a, b, "4N differs by zero: {a} vs {b}");
    assert!(a > 0, "the operation does work");
    // thirty isolated bodies: shell cliques would explode here;
    // isolated extras on the same complex must not grow the work.
    let mut thirty = Complex::new();
    for i in 0..30u64 {
        let x = crate::transducer::address::Address::from(i.to_le_bytes().to_vec());
        thirty.set_section(&x, Section::at(x.clone(), Rotor::identity()), vol());
    }
    assert!(
        thirty.degrees.get(3).is_none_or(|l| l.is_empty()),
        "thirty disjoint bodies form no 3-simplex"
    );
    let run_scene = |n: u64| -> u64 {
        let mut c = thirty.clone();
        for i in 0..n {
            let x = crate::transducer::address::Address::from((2000 + i).to_le_bytes().to_vec());
            c.set_section(&x, Section::at(x.clone(), Rotor::identity()), vol());
        }
        let v0 = crate::transducer::address::Address::from(0u64.to_le_bytes().to_vec());
        work::reset();
        c.coboundary_on_link(&v0, |_| [0; 4]);
        work::total()
    };
    let s = run_scene(32);
    let t = run_scene(128);
    eprintln!("work assemble-thirty extras N=32 {s} N=128 {t}");
    assert_eq!(s, t, "4N differs by zero on the assembled scene: {s} vs {t}");
}
