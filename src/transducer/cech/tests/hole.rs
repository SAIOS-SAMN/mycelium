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

/// 78.106 — A MEMBRANE'S LIFE BALANCES: SEAL WINDS, RELEASE UNWINDS. A cell
/// sealed over a 90° turn winds its motion into the owner's field; let go by
/// `descend`, the valve vents the reverse — the same flow unwound — so what
/// the field took in over the cell's life composes to the identity. A turn
/// of finite order unrolls round its own orbit; nothing is stepped.
#[test]
fn a_membrane_winds_at_its_seal_and_unwinds_at_its_release() {
    let mut c = Complex::new();
    c.arm_outflow();
    let (_, key) = sealed_triangle(&mut c, 1, turn_90());
    let wound = c.take_outflow();
    let [w] = wound.as_slice() else { panic!("the seal wound one motion in: {wound:?}") };
    assert!(!w.projective_eq(&ident()), "a turn: pressure held");
    assert!(matches!(c.descend(&key), Descended::Released(_)), "the membrane releases");
    let vented = c.take_outflow();
    let [u] = vented.as_slice() else { panic!("the release vented one motion: {vented:?}") };
    assert!(u.geo(w).projective_eq(&ident()), "unwound exactly: the cell's life balances");
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

/// A RELEASED VERTEX CAPS ITS HOLES FIRST (78.99 (2), the voice's vent): the
/// hole is sealed at the volume its cell was read with, its class returned to
/// a cell, and the release then hands that class out by `descend` with the
/// rest of the star — no hole left naming a boundary that is gone.
#[test]
fn a_released_vertex_caps_its_holes_and_hands_their_classes_out() {
    let mut c = Complex::new();
    let (vs, key) = sealed_triangle(&mut c, 1, turn_90());
    let Punctured::Opened(_) = c.puncture(&key) else { panic!("punctured") };
    let r = c.release_vertex(&vs[0]);
    assert!(!c.is_hole(&key), "the hole was capped");
    assert_eq!(r.released, 1, "and its class handed out by the descend: {r:?}");
    assert_eq!(r.pinned, 0, "nothing stands pinned");
    assert!(c.board().holes.is_empty() && c.board().sealed.is_empty());
}

/// 78.100 — WHERE THE CLIMB STOPS, read on the board it leaves. Three
/// vertices on a path: the third face does not stand and no cell is minted;
/// with the face and no volume on any lane, none again; with volume, one.
#[test]
fn the_climb_mints_a_cell_only_over_standing_faces_with_volume() {
    let mut c = Complex::new();
    let vs: Vec<Vec<u8>> = [v(41), v(42), v(43)].iter().map(|a| a.as_bytes().to_vec()).collect();
    for x in &vs {
        c.set_section(x, Section::at(crate::transducer::address::Address::from(x.clone()), Rotor::identity()), vol());
    }
    let cells = |c: &Complex| c.board().open.len() + c.board().sealed.len();
    c.set_edge(&vs[0], &vs[1], Rotor::identity(), ident(), vol());
    c.set_edge(&vs[1], &vs[2], Rotor::identity(), ident(), vol());
    let _ = c.coboundary_on_link(&vs[0], |_| vol());
    assert_eq!(cells(&c), 0, "the third face does not stand");
    c.set_edge(&vs[0], &vs[2], Rotor::identity(), ident(), vol());
    let _ = c.coboundary_on_link(&vs[0], |_| [0; 4]);
    assert_eq!(cells(&c), 0, "no volume on any lane");
    let _ = c.coboundary_on_link(&vs[0], |_| vol());
    assert_eq!(cells(&c), 1, "with volume, one 2-cell stands");
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

/// MU-CLIENT §4.5 — THE MIND CONDENSES, AND RE-TETHERS BY THE CONJUGATE
/// PAIR. Tension between two thoughts condenses a tether; the same tension
/// again changes nothing; tension re-formed on another transport drains the
/// standing tether and condenses the new one. Beneath a sealed cell the
/// tether is pinned: the new transport is not written over the class the
/// cell read.
#[test]
fn the_mind_condenses_and_retethers_by_drain_then_condense() {
    let mut mind = crate::transducer::bubble::contemplation::Contemplation::new();
    let (a, b) = (v(1).as_bytes().to_vec(), v(2).as_bytes().to_vec());
    assert_eq!(mind.tether(&a, &b, Rotor::identity(), ident(), vol()), Condensed::Tethered);
    assert_eq!(mind.tether(&a, &b, Rotor::identity(), ident(), vol()), Condensed::AlreadyTethered, "the same tension changes nothing");
    assert_eq!(mind.tether(&a, &b, turn_90(), ident(), vol()), Condensed::Tethered, "re-formed on a turn: drained, then condensed");
    assert!(mind.complex.tethered_as(&a, &b, &turn_90(), &ident()), "the tether carries the new transport");
    let (vs, _) = sealed_triangle(&mut mind.complex, 11, turn_90());
    assert_eq!(mind.tether(&vs[0], &vs[2], Rotor::identity(), ident(), vol()), Condensed::AlreadyTethered, "pinned beneath the sealed cell");
    assert!(mind.complex.tethered_as(&vs[0], &vs[2], &turn_90(), &ident()), "the read cell's tether stands as it was");
}

/// MU-CLIENT §4.5 — THE MIND'S CELL ASCENDS BY ITS OWN BOUNDARY. The climb
/// mints a cell open over three standing tethers; one tether drains (an open
/// cell pins nothing), and the cell no longer ascends — its boundary is not
/// whole. Condensed back, the boundary stands and the cell seals.
#[test]
fn the_minds_cell_ascends_only_over_its_standing_boundary() {
    let mut mind = crate::transducer::bubble::contemplation::Contemplation::new();
    let vs: Vec<Vec<u8>> = [v(21), v(22), v(23)].iter().map(|a| a.as_bytes().to_vec()).collect();
    for x in &vs {
        mind.complex.set_section(x, Section::at(crate::transducer::address::Address::from(x.clone()), Rotor::identity()), vol());
    }
    for (a, b) in [(0, 1), (1, 2), (0, 2)] {
        assert_eq!(mind.tether(&vs[a], &vs[b], Rotor::identity(), ident(), vol()), Condensed::Tethered);
    }
    let _ = mind.complex.coboundary_on_link(&vs[0], |_| vol());
    let cell = simplex_key(&sorted_verts(vs.clone())).as_bytes().to_vec();
    assert!(mind.complex.simplex(&cell).is_some_and(|s| s.fill.unsettled()), "the climb minted the cell open");
    assert!(matches!(mind.complex.drain(&edge_key(&vs[0], &vs[2])), Drained::Vacuum), "an open cell pins nothing");
    assert!(matches!(mind.ascend(&cell), CoboundaryOutcome::Absent), "a boundary missing a tether does not ascend");
    assert_eq!(mind.tether(&vs[0], &vs[2], Rotor::identity(), ident(), vol()), Condensed::Tethered);
    assert!(matches!(mind.ascend(&cell), CoboundaryOutcome::Attached(Fill::Filled(..))), "the standing boundary seals flat");
}

/// MU-CLIENT §4.5 — WHERE THE TENSION RELAXED, THE MIND DESCENDS AND DRAINS.
/// A sealed triangle of tethers that carried no flux over a whole aperture:
/// its cell descends and its tethers drain — the complex is empty of both.
/// The same triangle with one tether carrying flux since the aperture began:
/// the cell does not stand on relaxed tethers alone, so it stays sealed, and
/// it pins the two relaxed tethers beneath it.
#[test]
fn where_the_tension_relaxed_the_mind_descends_and_drains() {
    let triangle = |flowing: bool| {
        let mut mind = crate::transducer::bubble::contemplation::Contemplation::new();
        let vs: Vec<Vec<u8>> = [v(31), v(32), v(33)].iter().map(|a| a.as_bytes().to_vec()).collect();
        for x in &vs {
            mind.complex.set_section(x, Section::at(crate::transducer::address::Address::from(x.clone()), Rotor::identity()), vol());
        }
        mind.now_k = 1;
        for (a, b) in [(0, 1), (1, 2), (0, 2)] {
            assert_eq!(mind.tether(&vs[a], &vs[b], Rotor::identity(), ident(), vol()), Condensed::Tethered);
        }
        let _ = mind.complex.coboundary_on_link(&vs[0], |_| vol());
        let cell = simplex_key(&sorted_verts(vs.clone())).as_bytes().to_vec();
        assert!(matches!(mind.ascend(&cell), CoboundaryOutcome::Attached(_)), "the cell seals");
        mind.now_k = 10;
        if flowing {
            mind.flux_at.insert(edge_key(&vs[0], &vs[1]), 7);
        }
        mind.release_relaxed(5);
        mind
    };
    let relaxed = triangle(false);
    assert!(relaxed.complex.board().sealed.is_empty(), "the cell over relaxed tethers descended");
    assert!(relaxed.complex.board().tethers.is_empty(), "and its tethers drained");
    let held = triangle(true);
    assert_eq!(held.complex.board().sealed.len(), 1, "a cell one of whose tethers carried flux stands");
    assert_eq!(held.complex.board().tethers.len(), 3, "and pins its relaxed tethers beneath it");
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
    assert!(matches!(read[3], Applied::Descended(Descended::Hole)), "the last descend refuses at the hole");
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
    assert!(
        matches!(first_d.apply(&Pivot::Puncture(ka.clone()), |_| vol(), vol()), Applied::Punctured(Punctured::Absent)),
        "no cell left to puncture"
    );
    let (mut first_p, ..) = build();
    let _ = first_p.apply(&Pivot::Puncture(ka.clone()), |_| vol(), vol());
    assert!(
        matches!(first_p.apply(&Pivot::Descend(ka.clone()), |_| vol(), vol()), Applied::Descended(Descended::Hole)),
        "the descend refuses at the hole"
    );
    assert_ne!(first_d.board(), first_p.board(), "on one cell the order is content: the hole moved");
}
