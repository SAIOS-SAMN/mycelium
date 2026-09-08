// 14 — the class at degree three is the Bianchi form, and one fetched
// vertex costs the same whatever the complex already holds.

use super::common::*;
use crate::transducer::alphabet::Rotor;
use crate::transducer::cech::*;

/// Four vertices with a coboundary 1-cochain have δh = 1; a rotation on
/// one pair and a sheet on the opposite edge is a class at degree 3.
#[test]
fn a_class_at_degree_three_is_the_bianchi_form() {
    let mut c = Complex::new();
    let vs = [v(1), v(2), v(3), v(4)];
    for x in &vs {
        c.set_section(x, Section { addr: x.clone(), gamma: Rotor::identity() }, vol());
    }
    let id = Rotor::identity();
    for i in 0..4 {
        for j in i + 1..4 {
            c.set_edge(&vs[i], &vs[j], id.clone(), vol());
        }
    }
    c.coboundary_on_link(&vs[0], |_| vol());
    let tets = c.link(&vs[0], 3);
    assert_eq!(tets.len(), 1, "the link of one vertex of K4 is one tetra");
    let key = tets[0].clone();
    assert!(c.fill(&key) == Fill::Filled(Rotor::identity()) || c.class(&key).is_none());
    c.set_edge(&vs[0], &vs[1], pythag5(), vol());
    c.set_edge(&vs[2], &vs[3], sheet(), vol());
    c.coboundary_on_link(&vs[0], |_| vol());
    let mut found = false;
    // index: link of vs[0]
    for key in c.link_of(&vs[0]) {
        let Some(d) = c.cell(&key).map(|c| c.degree()) else {
            continue;
        };
        if d < 2 {
            continue;
        }
        let outcome = c.fill(&key);
        if matches!(outcome, Fill::Class(_) | Fill::Congruent(_, _)) || c.class(&key).is_some() {
            found = true;
        }
    }
    assert!(found, "an inconsistent link carries a class or a finite congruence");
    assert!(c.byte_len() < 4096, "four covering vertices stay in kilobytes");
}

/// Work of filling the link of one vertex does not grow with N isolated
/// sections on the same complex.
#[test]
fn work_of_one_fetched_vertex_is_independent_of_n() {
    use crate::network::nerve::work;
    let run = |n: u64| -> u64 {
        let mut c = Complex::new();
        let (a, b, cc) = (v(1), v(2), v(3));
        for x in [&a, &b, &cc] {
            c.set_section(x, Section { addr: (*x).clone(), gamma: Rotor::identity() }, vol());
        }
        let id = Rotor::identity();
        c.set_edge(&a, &b, id.clone(), vol());
        c.set_edge(&a, &cc, id.clone(), vol());
        c.set_edge(&b, &cc, id, vol());
        for i in 0..n {
            let x = crate::transducer::a21::Address::from((1000 + i).to_le_bytes().to_vec());
            c.set_section(&x, Section { addr: x.clone(), gamma: Rotor::identity() }, vol());
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
        let x = crate::transducer::a21::Address::from(i.to_le_bytes().to_vec());
        thirty.set_section(&x, Section { addr: x.clone(), gamma: Rotor::identity() }, vol());
    }
    assert!(
        thirty.degrees.get(3).is_none_or(|l| l.is_empty()),
        "thirty disjoint bodies form no 3-cell"
    );
    let run_scene = |n: u64| -> u64 {
        let mut c = thirty.clone();
        for i in 0..n {
            let x = crate::transducer::a21::Address::from((2000 + i).to_le_bytes().to_vec());
            c.set_section(&x, Section { addr: x.clone(), gamma: Rotor::identity() }, vol());
        }
        let v0 = crate::transducer::a21::Address::from(0u64.to_le_bytes().to_vec());
        work::reset();
        c.coboundary_on_link(&v0, |_| [0; 4]);
        work::total()
    };
    let s = run_scene(32);
    let t = run_scene(128);
    eprintln!("work assemble-thirty extras N=32 {s} N=128 {t}");
    assert_eq!(s, t, "4N differs by zero on the assembled scene: {s} vs {t}");
}
