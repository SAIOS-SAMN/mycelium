// 62.5 as amended, AD1 — `ascend` is the inverse of `descend` on content:
// seal, release, re-seal returns the fill and the boundary exactly, and the
// ledger keeps the pair as history.

use super::common::*;
use crate::transducer::alphabet::Rotor;
use crate::transducer::cech::*;
use crate::transducer::element::identity_versor as ident;

/// Three sectioned vertices and their three edges, the third carrying
/// `third`: the identity closes the boundary flat, a quarter turn leaves it
/// carrying a class.
fn triangle(third: Rotor) -> (Complex, Vec<Vec<u8>>) {
    let mut c = Complex::new();
    let vs: Vec<Vec<u8>> = [v(1), v(2), v(3)].iter().map(|a| a.as_bytes().to_vec()).collect();
    for x in &vs {
        c.set_section(x, Section::at(crate::transducer::address::Address::from(x.clone()), Rotor::identity()), vol());
    }
    c.set_edge(&vs[0], &vs[1], Rotor::identity(), ident(), vol());
    c.set_edge(&vs[1], &vs[2], Rotor::identity(), ident(), vol());
    c.set_edge(&vs[0], &vs[2], third, ident(), vol());
    (c, vs)
}

fn edges(c: &Complex, vs: &[Vec<u8>]) -> Vec<String> {
    [(0, 1), (1, 2), (0, 2)].iter().map(|&(i, j)| format!("{:?}", c.edge(&vs[i], &vs[j]))).collect()
}

/// THE ROUND TRIP, ON BOTH POLES: a boundary that closes flat and one that
/// carries a class. Each seals, releases with its fill handed back, and
/// re-seals to the SAME fill over the SAME boundary; the ledger counts two
/// seals and one release, and never nets them.
#[test]
fn ascend_seals_descend_releases_and_ascend_reseals_the_same_content() {
    for third in [Rotor::identity(), quarter()] {
        let _ = crate::physics::metabolism::take_metabolism();
        let (mut c, vs) = triangle(third);
        let boundary = edges(&c, &vs);

        let Ascended::Sealed(first) = c.ascend(&vs) else { panic!("a standing boundary seals") };
        assert!(matches!(c.ascend(&vs), Ascended::AlreadySealed), "a sealed cell is not sealed twice");
        let key = simplex_key(&sorted_verts(vs.clone()));
        let down = c.descend(key.as_bytes());
        let released = down.released().expect("a sealed membrane hands its fill back");
        assert_eq!(format!("{released:?}"), format!("{first:?}"), "descend returns what ascend sealed");
        assert_eq!(edges(&c, &vs), boundary, "the boundary stands with its transports");

        let Ascended::Sealed(again) = c.ascend(&vs) else { panic!("the released boundary still closes") };
        assert_eq!(format!("{again:?}"), format!("{first:?}"), "and re-seals the same content");
        assert_eq!(edges(&c, &vs), boundary, "over the same boundary");

        let m = crate::physics::metabolism::take_metabolism();
        assert_eq!((m.sealed, m.released), (2, 1), "the ledger keeps the pair as history, never netted");
    }
}

/// NO BOUNDARY, NO ASCENT: a missing edge is `Absent` by name, never a cell
/// sealed over a hole.
#[test]
fn ascend_over_a_missing_edge_is_absent() {
    let (mut c, vs) = triangle(Rotor::identity());
    let edge = simplex_key(&sorted_verts([&vs[0], &vs[2]]));
    assert!(matches!(c.drain(edge.as_bytes()), Drained::Vacuum));
    assert!(matches!(c.ascend(&vs), Ascended::Absent), "the boundary does not stand");
}
