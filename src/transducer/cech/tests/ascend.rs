// 62.5 as amended, AD1 — `ascend` is the inverse of `descend` on content:
// seal, cross, read back returns the fill and the boundary exactly, and the
// ledger keeps the pair as history.

use super::common::*;
use crate::transducer::alphabet::Rotor;
use crate::transducer::cech::*;
use crate::transducer::element::identity_versor as ident;

/// Three sectioned vertices and the three edges among them, the third carrying
/// `third`: the identity closes the boundary flat, a 90° turn leaves it
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

/// The boundary's transports, compared as the types they are (`PartialEq`),
/// never as a rendering.
fn edges(c: &Complex, vs: &[Vec<u8>]) -> Vec<Option<(Rotor, crate::transducer::element::Versor)>> {
    [(0, 1), (1, 2), (0, 2)].iter().map(|&(i, j)| c.edge(&vs[i], &vs[j])).collect()
}

/// THE ROUND TRIP, ON BOTH POLES, THROUGH THE CROSS: a boundary that closes
/// flat and one that carries a class. Each seals; crosses, its fill kept as
/// a hole over the standing boundary; and is read back by the seal — the
/// dipole's only read-back — to the SAME fill over the SAME boundary; the
/// ledger counts two seals and one kill, and never nets them.
#[test]
fn ascend_then_seal_reads_back_same_content() {
    for third in [Rotor::identity(), turn_90()] {
        let _ = crate::physics::metabolism::take_metabolism();
        let (mut c, vs) = triangle(third);
        let boundary = edges(&c, &vs);

        let Ascended::Sealed(first) = c.ascend(&vs, |_| vol()) else { panic!("a standing boundary seals") };
        assert!(matches!(c.ascend(&vs, |_| vol()), Ascended::AlreadySealed), "a sealed cell is not sealed twice");
        let key = simplex_key(&sorted_verts(vs.clone()));
        let down = c.descend(key.as_bytes());
        let kept = down.kept().expect("a sealed chamber keeps its class through the throat");
        assert_eq!(kept, &first, "descend keeps what ascend sealed");
        assert_eq!(edges(&c, &vs), boundary, "the boundary stands with its transports");
        assert!(matches!(c.ascend(&vs, |_| vol()), Ascended::Hole), "ascend alone does not read it back");

        let Capped::Sealed { cap: again, killed } = c.seal(key.as_bytes(), |_| vol()) else { panic!("the kept boundary still closes") };
        assert_eq!(again, first, "the seal reads back the same content");
        assert_eq!(killed, first, "the class the hole kept");
        assert_eq!(edges(&c, &vs), boundary, "over the same boundary");

        let m = crate::physics::metabolism::take_metabolism();
        assert_eq!((m.sealed, m.killed), (2, 1), "the ledger keeps the pair as history, never netted");
    }
}

/// NO BOUNDARY, NO ASCENT: a missing edge is `Absent` by name, never a cell
/// sealed over a hole.
#[test]
fn ascend_over_missing_edge_is_absent() {
    let (mut c, vs) = triangle(Rotor::identity());
    let edge = simplex_key(&sorted_verts([&vs[0], &vs[2]]));
    assert!(matches!(c.drain(edge.as_bytes()), Drained::Vacuum));
    assert!(matches!(c.ascend(&vs, |_| vol()), Ascended::Absent), "the boundary does not stand");
}

/// A CELL WITH NO VOLUME IS NOT MINTED — by `ascend` as by the climb: the
/// volume is read off the chart, never written as a constant.
#[test]
fn ascend_without_volume_is_absent() {
    let (mut c, vs) = triangle(Rotor::identity());
    assert!(matches!(c.ascend(&vs, |_| [0; 4]), Ascended::Absent), "no volume on any lane, no cell");
    let Ascended::Sealed(_) = c.ascend(&vs, |_| vol()) else { panic!("with its volume read, it seals") };
    let key = simplex_key(&sorted_verts(vs.clone()));
    assert_eq!(c.simplex(key.as_bytes()).map(|s| s.volume), Some(vol()), "the volume the reader read");
}
