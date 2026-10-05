// 78.99 (4), M8 — GLUING PRODUCES MEANING: the non-abelian connecting map
// reads a class neither exact part held where the overlap is disconnected,
// none where it is connected or the loop is flat, refuses an inexact glue,
// and reads a hole one part caps.

use super::common::*;
use crate::transducer::alphabet::Rotor;
use crate::transducer::cech::*;
use crate::transducer::element::identity_versor as ident;

fn key(n: u8) -> Vec<u8> {
    v(n).as_bytes().to_vec()
}

/// A PATH `a — mid — b` with its two tethers, the second carrying `g`.
fn arc(a: u8, mid: u8, b: u8, g: Rotor) -> Complex {
    let mut c = Complex::new();
    for n in [a, mid, b] {
        c.set_section(key(n), Section::at(crate::transducer::address::Address::from(key(n)), Rotor::identity()), vol());
    }
    c.set_edge(key(a), key(mid), Rotor::identity(), ident(), vol());
    c.set_edge(key(mid), key(b), g, ident(), vol());
    c
}

/// TWO ARCS GLUED AT THEIR ENDS MAKE A RING, and its holonomy is a class
/// neither arc held: each arc is a tree, exact on its own; the overlap is the
/// two ends, two components; the loop out through one arc and back through
/// the other carries the turn. With both arcs flat the loop is flat and
/// makes nothing.
#[test]
fn two_exact_arcs_glued_at_their_ends_make_a_class_neither_held() {
    let (u, w) = (arc(1, 2, 3, turn_90()), arc(1, 4, 3, Rotor::identity()));
    let g = connecting(&u, &w).expect("two exact parts glue");
    assert_eq!(g.components, 2, "the overlap is the two ends");
    assert_eq!(g.made.len(), 1, "one loop through the overlap, one class made: {g:?}");
    assert!(matches!(g.made[0].2, Fill::Class(..) | Fill::Congruent(..)), "a curved class: {:?}", g.made[0].2);
    let flat = connecting(&arc(1, 2, 3, Rotor::identity()), &w).expect("glues");
    assert!(flat.made.is_empty() && flat.flat == 1, "a flat loop makes nothing: {flat:?}");
}

/// A CONNECTED OVERLAP MAKES NOTHING, AND AN INEXACT ONE REFUSES. Two parts
/// sharing a tether with one transition overlap in one component; sharing it
/// with two transitions is the Mayer–Vietoris inexactness, refused by name.
#[test]
fn a_connected_overlap_makes_nothing_and_an_inexact_one_refuses() {
    let u = arc(1, 2, 3, turn_90());
    let same = arc(5, 1, 2, Rotor::identity());
    let g = connecting(&u, &same).expect("glues");
    assert_eq!((g.components, g.made.len()), (1, 0), "{g:?}");
    let other = arc(5, 2, 3, Rotor::identity());
    assert!(matches!(connecting(&u, &other), Err(Unglued::Inexact(_))), "one tether, two transitions");
    assert!(matches!(connecting(&u, &arc(7, 8, 9, Rotor::identity())), Err(Unglued::NoOverlap)));
}

/// GLUING CAPS: a hole of one part whose cell the other holds sealed is read
/// capped, with the class it kept. (The triangles are flat, so each part is
/// exact and the glue reads; a part carrying holonomy of its own refuses by
/// name, below.)
#[test]
fn a_hole_one_part_holds_sealed_is_capped_by_the_glue() {
    let triangle = |third: Rotor| {
        let mut c = arc(1, 2, 3, Rotor::identity());
        c.set_edge(key(1), key(3), third, ident(), vol());
        let Ascended::Sealed(_) = c.ascend(&[key(1), key(2), key(3)], |_| vol()) else { panic!("seals") };
        c
    };
    let (mut u, w) = (triangle(Rotor::identity()), triangle(Rotor::identity()));
    let k = simplex_key(&sorted_verts(vec![key(1), key(2), key(3)])).as_bytes().to_vec();
    let Punctured::Opened(kept) = u.puncture(&k) else { panic!("punctured") };
    let g = connecting(&u, &w).expect("two exact parts glue");
    assert_eq!(g.capped, vec![(k, kept)], "the hole is capped by the other part's cell");
    let curved = triangle(turn_90());
    assert!(matches!(connecting(&curved, &w), Err(Unglued::Inexact(_))), "sharing the turned tether with another transition: inexact");
    let ends = arc(1, 4, 3, Rotor::identity());
    assert_eq!(connecting(&curved, &ends).err(), Some(Unglued::PartNotExact(0)), "a part with holonomy of its own refuses by name");
}
