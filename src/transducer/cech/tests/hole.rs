// 78.99 (2), (3), 62.1, 63.3 — THE PIVOTS COMPLETED AND THE BOARD: condense
// is drain's inverse on content; puncture keeps the class in as a hole that
// pins its boundary and refuses every pivot but seal; seal caps it; pivots on
// cells sharing no vertex commute; the walk `d, a, p, d` reads as ruled.

use super::common::*;
use crate::transducer::alphabet::Rotor;
use crate::transducer::cech::*;
use crate::transducer::element::identity_versor as ident;

/// Three sectioned vertices `n..n+3` and their edges, the third carrying
/// `third`, its cell sealed: a turn leaves it carrying a class.
fn sealed_triangle(c: &mut Complex, n: u8, third: Rotor) -> (Vec<Vec<u8>>, Vec<u8>) {
    let vs: Vec<Vec<u8>> = [v(n), v(n + 1), v(n + 2)].iter().map(|a| a.as_bytes().to_vec()).collect();
    for x in &vs {
        c.set_section(x, Section::at(crate::transducer::address::Address::from(x.clone()), Rotor::identity()), vol());
    }
    c.set_edge(&vs[0], &vs[1], Rotor::identity(), ident(), vol());
    c.set_edge(&vs[1], &vs[2], Rotor::identity(), ident(), vol());
    c.set_edge(&vs[0], &vs[2], third, ident(), vol());
    let Ascended::Sealed(_) = c.ascend(&vs, |_| vol()) else { panic!("a standing boundary seals") };
    let key = simplex_key(&sorted_verts(vs.clone())).as_bytes().to_vec();
    (vs, key)
}

fn edge_key(a: &[u8], b: &[u8]) -> Vec<u8> {
    simplex_key(&sorted_verts([a, b])).as_bytes().to_vec()
}

/// PUNCTURE KEEPS THE CLASS IN, AND ONLY SEAL CAPS THE HOLE. The sealed cell
/// opens into a hole carrying its fill, and nothing flows out; the hole pins
/// its boundary (no edge beneath drains); `descend`, `ascend` and `puncture`
/// refuse at it by name; `seal` caps it with the same fill, the class it kept
/// killed, and the board is the sealed board again — inverse on content.
#[test]
fn puncture_keeps_the_class_in_and_only_seal_caps_the_hole() {
    let mut c = Complex::new();
    let (vs, key) = sealed_triangle(&mut c, 1, turn_90());
    let sealed = c.board();
    c.arm_outflow();
    let Punctured::Opened(kept) = c.puncture(&key) else { panic!("a sealed cell opens into a hole") };
    assert!(c.take_outflow().is_empty(), "the class stays in: nothing flows out");
    assert!(c.is_hole(&key));
    assert_eq!(c.board().holes, vec![(key.clone(), kept.clone())], "the hole keeps the class");
    assert!(c.board().sealed.is_empty(), "and the cell is gone from the sealed axis");
    assert!(matches!(c.drain(&edge_key(&vs[0], &vs[1])), Drained::Pinned), "a hole pins its boundary");
    assert!(matches!(c.descend(&key), Descended::Hole), "descend refuses at a hole");
    assert!(matches!(c.ascend(&vs, |_| vol()), Ascended::Hole), "ascend refuses at a hole");
    assert!(matches!(c.puncture(&key), Punctured::Hole), "a hole is not punctured twice");
    let Capped::Sealed { cap, killed } = c.seal(&key, |_| vol()) else { panic!("the boundary still closes") };
    assert_eq!(killed, kept, "the class the hole kept is killed");
    assert_eq!(c.board(), sealed, "seal is puncture's inverse on content");
    assert_eq!(c.board().sealed, vec![(key, cap)]);
}

/// CONDENSE IS DRAIN'S INVERSE ON CONTENT. A flat tether drains to the
/// vacuum and condenses back with its transition: the same board. A tether
/// that stands is not condensed twice; a remainder pair refuses.
#[test]
fn condense_is_drains_inverse_on_content() {
    let mut c = Complex::new();
    let vs: Vec<Vec<u8>> = [v(1), v(2)].iter().map(|a| a.as_bytes().to_vec()).collect();
    c.set_edge(&vs[0], &vs[1], Rotor::identity(), ident(), vol());
    let before = c.board();
    assert!(matches!(c.drain(&edge_key(&vs[0], &vs[1])), Drained::Vacuum));
    assert_eq!(c.condense(&vs[0], &vs[1], Rotor::identity(), ident(), vol()), Condensed::Tethered);
    assert_eq!(c.board(), before, "the same tether, the same board");
    assert_eq!(c.condense(&vs[0], &vs[1], Rotor::identity(), ident(), vol()), Condensed::AlreadyTethered);
    let w: Vec<u8> = v(3).as_bytes().to_vec();
    c.set_remainder_edge(&vs[0], &w);
    assert_eq!(c.condense(&vs[0], &w, Rotor::identity(), ident(), vol()), Condensed::Remainder, "a remainder pair refuses");
}

/// THE WALK `d, a, p, d` ON ONE CELL (78.99 (3)). `d·a` cancels on the board;
/// `p` moves the class from the sealed axis to the hole axis; the last `d`
/// refuses at the hole. The board it leaves is the board `p` alone leaves.
#[test]
fn the_walk_descend_ascend_puncture_descend_reads_as_ruled() {
    let mut c = Complex::new();
    let (vs, key) = sealed_triangle(&mut c, 1, turn_90());
    let sealed = c.board();
    let walk = [Pivot::Descend(key.clone()), Pivot::Ascend(vs.clone()), Pivot::Puncture(key.clone()), Pivot::Descend(key.clone())];
    let read: Vec<Applied> = walk.iter().map(|p| c.apply(p, |_| vol(), vol())).collect();
    assert!(matches!(read[0], Applied::Descended(Descended::Released(_))));
    assert!(matches!(read[1], Applied::Ascended(Ascended::Sealed(_))));
    assert!(matches!(read[2], Applied::Punctured(Punctured::Opened(_))));
    assert!(matches!(read[3], Applied::Descended(Descended::Hole)) && read[3].refused(), "the last descend refuses at the hole");
    let mut alone = Complex::new();
    let (_, k2) = sealed_triangle(&mut alone, 1, turn_90());
    assert_eq!(alone.board(), sealed);
    let _ = alone.puncture(&k2);
    assert_eq!(c.board(), alone.board(), "d·a cancelled on the board: the walk leaves what p alone leaves");
}

/// THE LOCALITY THEOREM. Two cells sharing no vertex: a descend of one and a
/// puncture of the other, in either order, leave one board and one outflow.
/// On one cell the order is content: `d` then `p` leaves no hole (the puncture
/// finds no cell), `p` then `d` leaves the hole (the descend refuses).
#[test]
fn pivots_sharing_no_vertex_commute_and_on_one_cell_the_order_is_content() {
    let build = || {
        let mut c = Complex::new();
        let (a, ka) = sealed_triangle(&mut c, 1, turn_90());
        let (b, kb) = sealed_triangle(&mut c, 4, turn_90());
        c.arm_outflow();
        (c, a, ka, b, kb)
    };
    let (mut x, a, ka, b, kb) = build();
    assert!(independent(&a, &b), "the two cells share no vertex");
    let (d, p) = (Pivot::Descend(ka.clone()), Pivot::Puncture(kb.clone()));
    let _ = x.apply(&d, |_| vol(), vol());
    let _ = x.apply(&p, |_| vol(), vol());
    let (mut y, ..) = build();
    let _ = y.apply(&p, |_| vol(), vol());
    let _ = y.apply(&d, |_| vol(), vol());
    assert_eq!(x.board(), y.board(), "independent pivots commute on the board");
    let bytes = |c: &mut Complex| {
        let mut out: Vec<Vec<u8>> = c
            .take_outflow()
            .iter()
            .map(|m| {
                let mut k = Vec::new();
                m.put_canonical(&mut k);
                k
            })
            .collect();
        out.sort();
        out
    };
    assert_eq!(bytes(&mut x), bytes(&mut y), "and on the outflow, as a multiset");
    let (mut first_d, ..) = build();
    let _ = first_d.apply(&Pivot::Descend(ka.clone()), |_| vol(), vol());
    assert!(first_d.apply(&Pivot::Puncture(ka.clone()), |_| vol(), vol()).refused(), "no cell left to puncture");
    let (mut first_p, ..) = build();
    let _ = first_p.apply(&Pivot::Puncture(ka.clone()), |_| vol(), vol());
    assert!(first_p.apply(&Pivot::Descend(ka.clone()), |_| vol(), vol()).refused(), "the descend refuses at the hole");
    assert_ne!(first_d.board(), first_p.board(), "on one cell the order is content: the hole moved");
}
