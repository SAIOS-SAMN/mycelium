// 78.99 (2), (3), 62.1, 63.3 — THE PIVOTS COMPLETED AND THE BOARD: condense
// is drain's inverse on content; puncture keeps the class in as a hole that
// pins its boundary and refuses every pivot but seal; seal caps it; a descend
// is the cross, through the puncture (the operator's ruling, 2026-10-20);
// pivots on cells sharing no vertex commute; the walk `d, a, s, d` reads as
// ruled.

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

/// 78.106 — A MEMBRANE'S LIFE BALANCES: SEAL WINDS, THE CROSS UNWINDS. A cell
/// sealed over a 90° turn winds its motion into the owner's field; let go by
/// `descend` — the cross — its content is handed down through the throat: the
/// valve vents the reverse, the same flow unwound, so what the field took in
/// over the cell's life composes to the identity, while its class stays kept
/// as a hole. A turn of finite order unrolls round its own orbit; nothing is
/// stepped.
#[test]
fn a_membrane_winds_at_its_seal_and_unwinds_at_its_release() {
    let mut c = Complex::new();
    c.arm_outflow();
    let (_, key) = sealed_triangle(&mut c, 1, turn_90());
    let wound = c.take_outflow();
    let [w] = wound.as_slice() else { panic!("the seal wound one motion in: {wound:?}") };
    assert!(!w.motion.projective_eq(&ident()), "a turn: pressure held");
    assert_eq!(w.line, Thread::Wound(key.clone()), "wound on the cell's own line");
    assert!(matches!(c.descend(&key), Descended::Throat(_)), "the membrane crosses");
    assert!(c.is_hole(&key), "its class kept as a hole");
    let vented = c.take_outflow();
    let [u] = vented.as_slice() else { panic!("the release vented one motion: {vented:?}") };
    assert!(u.motion.geo(&w.motion).projective_eq(&ident()), "unwound exactly: the cell's life balances");
    assert_eq!(u.line, Thread::Unwound(key), "unwound from the line it was wound on");
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

/// A VERTEX WHOSE WORD HAS LEFT READS NO CLASS BACK, AND CAUTERIZES (the
/// eight-component cross, ruled 2026-10-20). Released under a sealed
/// chamber, it crosses it — the class kept through the throat, its content
/// handed down — and, no meet over it recurring, the hole is cauterized: the
/// class burned, its scar recorded, the boundary unpinned, and the vertex
/// goes. A question standing on it is cauterized first. A vertex under a
/// chamber that sealed nothing keeps nothing, and goes, with no scar.
#[test]
fn a_released_vertex_cauterizes_the_classes_it_cannot_read_back() {
    let mut c = Complex::new();
    c.arm_outflow();
    let (vs, key) = sealed_triangle(&mut c, 1, turn_90());
    let _ = c.take_outflow();
    let r = c.release_vertex(&vs[0]);
    assert_eq!((r.kept, r.scarred, r.pinned), (1, 1, 0), "the chamber crossed and its class was cauterized: {r:?}");
    assert!(!c.is_hole(&key) && c.board().holes.is_empty(), "no wound left standing");
    assert_eq!(c.scars(), 1, "the scar recorded on the complex");
    assert!(c.sections.get(vs[0].as_slice()).is_none(), "and the vertex goes");
    assert!(!c.take_outflow().is_empty(), "its content was handed down at the throat");
    let mut asked = Complex::new();
    let (ws, held) = sealed_triangle(&mut asked, 21, turn_90());
    let Punctured::Opened(_) = asked.puncture(&held) else { panic!("a question opened") };
    let r = asked.release_vertex(&ws[0]);
    assert_eq!((r.scarred, r.pinned), (1, 0), "the question on it cauterized first: {r:?}");
    assert!(asked.sections.get(ws[0].as_slice()).is_none(), "and the vertex goes");
    let mut open = Complex::new();
    let ws: Vec<Vec<u8>> = [v(11), v(12), v(13)].iter().map(|a| a.as_bytes().to_vec()).collect();
    for x in &ws {
        open.set_section(x, Section::at(crate::transducer::address::Address::from(x.clone()), Rotor::identity()), vol());
    }
    for (a, b) in [(0, 1), (1, 2), (0, 2)] {
        open.set_edge(&ws[a], &ws[b], Rotor::identity(), ident(), vol());
    }
    let _ = open.coboundary_on_link(&ws[0], |_| vol());
    let r = open.release_vertex(&ws[0]);
    assert_eq!((r.kept, r.scarred, r.unsealed, r.pinned), (0, 0, 1, 0), "a chamber that sealed nothing keeps nothing: {r:?}");
    assert!(open.sections.get(ws[0].as_slice()).is_none() && open.scars() == 0, "and the vertex goes, unscarred");
}

/// THE READ-BACK THAT CANNOT BE MADE CAUTERIZES (the eight-component
/// cross). A chamber crosses on the mind's complex and the meet over the same
/// vertices reads its class back whole — ascend at the hole, the seal — with
/// no scar. Crossed again, one pair of its boundary then stands as a
/// remainder — a transition that does not realise — so the boundary no
/// longer stands as one and the class cannot be read back: the hole is
/// cauterized, its scar recorded, not left standing.
#[test]
fn a_read_back_that_cannot_be_made_cauterizes() {
    let mut mind = crate::transducer::bubble::contemplation::Contemplation::new();
    let (vs, key) = sealed_triangle(&mut mind.complex, 61, turn_90());
    assert!(matches!(mind.complex.descend(&key), Descended::Throat(_)), "the chamber crosses");
    assert!(
        matches!(mind.ascend(&key), CoboundaryOutcome::Attached(f) if !f.unsettled()),
        "a boundary that still closes reads the class back"
    );
    assert!(mind.complex.simplex(&key).is_some() && mind.complex.scars() == 0, "the chamber returned, no scar");
    assert!(matches!(mind.complex.descend(&key), Descended::Throat(_)), "it crosses again");
    mind.complex.set_remainder_edge(&vs[0], &vs[2]);
    assert!(!matches!(mind.ascend(&key), CoboundaryOutcome::Attached(f) if !f.unsettled()), "the read-back cannot be made");
    assert!(!mind.complex.is_hole(&key), "the hole is cauterized, not left standing");
    assert_eq!(mind.complex.scars(), 1, "its scar recorded");
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

/// MU-CLIENT §4.5, 78.74 — A TETHER IS A FIBRE AND FLEXES. The same tension
/// changes nothing. A 90° turn meeting the identity tether flexes it: the
/// fibre carries the arriving transport, and the composite answered is
/// exactly `old⁻¹ · new`. Two thoughts with no perceived body have no
/// pivots, so no fibre reads the flex as torsion: it works, and winds out on
/// the tether's own line. Under a sealed cell the fibre flexes and the cell
/// re-reads; under a hole it is pinned and carries what it carried.
#[test]
fn a_tether_is_a_fibre_and_flexes() {
    let mut mind = crate::transducer::bubble::contemplation::Contemplation::new();
    mind.complex.arm_outflow();
    let (a, b) = (v(1).as_bytes().to_vec(), v(2).as_bytes().to_vec());
    assert_eq!(mind.tether(&a, &b, Rotor::identity(), ident(), vol()), Condensed::Tethered);
    let _ = mind.complex.take_outflow();
    assert_eq!(mind.tether(&a, &b, Rotor::identity(), ident(), vol()), Condensed::AlreadyTethered, "the same tension changes nothing");
    let Condensed::Flexed(c) = mind.tether(&a, &b, turn_90(), ident(), vol()) else { panic!("it flexes") };
    assert!(c.projective_eq(&ident().geo(&turn_90().lift())), "the composite is old⁻¹ · new");
    assert!(mind.complex.tethered_as(&a, &b, &turn_90(), &ident()), "the fibre carries the arriving transport");
    let wound = mind.complex.take_outflow();
    let line = Thread::Wound(simplex_key(&sorted_verts([a.as_slice(), b.as_slice()])).as_bytes().to_vec());
    assert!(matches!(wound.as_slice(), [f] if f.line == line), "with no pivots the flex works, wound on the tether's own line");
    let (vs, cell) = sealed_triangle(&mut mind.complex, 11, turn_90());
    assert!(matches!(mind.tether(&vs[0], &vs[2], Rotor::identity(), ident(), vol()), Condensed::Flexed(_)), "under a sealed cell it flexes");
    assert!(mind.complex.simplex(&cell).is_some_and(|s| s.fill.unsettled()), "and the cell re-reads");
    let (ws, held) = sealed_triangle(&mut mind.complex, 21, turn_90());
    assert!(matches!(mind.complex.puncture(&held), Punctured::Opened(_)), "a question opened over it");
    assert!(matches!(mind.tether(&ws[0], &ws[2], Rotor::identity(), ident(), vol()), Condensed::Pinned(_)), "a hole pins it");
    assert!(mind.complex.tethered_as(&ws[0], &ws[2], &turn_90(), &ident()), "pinned, it carries what it carried");
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

/// MU-CLIENT §4.5 — THE FIELD RELEASES WHAT RELAXED, AND THE MIND LETS IT GO.
/// Two tethers condense carrying opposite turns of one plane; each winds its
/// strain into the mind's field on its own line, threading the plane's 𝒽
/// orbit. The second brings the orbit back to its return — the tension
/// vented — and the field releases both lines: let go, both drain. A tether
/// under a question (a punctured cell over it) is released and stands: the
/// hole pins it.
#[test]
fn the_field_releases_what_relaxed_and_the_mind_lets_it_go() {
    use crate::transducer::family::pairing::{mark_flux, FieldLine};
    let mut field = crate::transducer::mind::MindField::default();
    field.think(|| {
        let mut mind = crate::transducer::bubble::contemplation::Contemplation::new();
        mind.complex.arm_outflow();
        let vs: Vec<Vec<u8>> = [v(31), v(32), v(33), v(34)].iter().map(|a| a.as_bytes().to_vec()).collect();
        let turn = turn_90();
        assert_eq!(mind.tether(&vs[0], &vs[1], turn.clone(), ident(), vol()), Condensed::Tethered);
        assert_eq!(mind.tether(&vs[2], &vs[3], turn.reverse(), ident(), vol()), Condensed::Tethered);
        let mut budget = crate::physics::Physics::default().stroke_n(1 << 16);
        let lines: Vec<FieldLine> = mind
            .complex
            .take_outflow()
            .into_iter()
            .map(|f| FieldLine { carriage: f.motion, gauge: Vec::new(), mass: 1, free: false, line: f.line })
            .collect();
        assert_eq!(lines.len(), 2, "each tether wound its strain in");
        mark_flux(&[], &lines, &mut budget);
        let Ok(released) = crate::transducer::mind::with_field(|f| f.take_released()) else { panic!("a mind is thinking") };
        let mut expected = vec![edge_key(&vs[0], &vs[1]), edge_key(&vs[2], &vs[3])];
        expected.sort();
        let mut got: Vec<Vec<u8>> = released.iter().map(|(l, _)| l.clone()).collect();
        got.sort();
        assert_eq!(got, expected, "the orbit returned: both lines released");
        assert!(
            released.iter().all(|(_, why)| *why == crate::transducer::cech::orbit::Released::Returned),
            "released because the orbit returned"
        );
        mind.release(&released);
        assert!(mind.complex.board().tethers.is_empty(), "both tethers drained");
    });
    let mut asked = Complex::new();
    let (vs, key) = sealed_triangle(&mut asked, 41, turn_90());
    let Punctured::Opened(_) = asked.puncture(&key) else { panic!("punctured: a question") };
    let mut mind = crate::transducer::bubble::contemplation::Contemplation::new();
    mind.complex = asked;
    mind.release(&[(edge_key(&vs[0], &vs[1]), crate::transducer::cech::orbit::Released::Returned)]);
    assert_eq!(mind.complex.board().tethers.len(), 3, "the question's hole pins its boundary: the tether stands");
}

/// MU-CLIENT §4.5 — FROZEN-IN FLUX: A FIBRE MOVES WITH THE FIELD. Two
/// tethers carrying the same 90° turn wind onto one plane's orbit; the
/// second admission moves the orbit by 90°, and the first fibre — wound
/// there — is carried by exactly that turn: it now carries 180°. The fibre
/// that moved the orbit is not carried by itself.
#[test]
fn a_fibre_moves_with_the_field() {
    use crate::transducer::family::pairing::{mark_flux, FieldLine};
    let mut field = crate::transducer::mind::MindField::default();
    field.think(|| {
        let mut mind = crate::transducer::bubble::contemplation::Contemplation::new();
        mind.complex.arm_outflow();
        let vs: Vec<Vec<u8>> = [v(51), v(52), v(53), v(54)].iter().map(|a| a.as_bytes().to_vec()).collect();
        assert_eq!(mind.tether(&vs[0], &vs[1], turn_90(), ident(), vol()), Condensed::Tethered);
        assert_eq!(mind.tether(&vs[2], &vs[3], turn_90(), ident(), vol()), Condensed::Tethered);
        let lines: Vec<FieldLine> = mind
            .complex
            .take_outflow()
            .into_iter()
            .map(|f| FieldLine { carriage: f.motion, gauge: Vec::new(), mass: 1, free: false, line: f.line })
            .collect();
        mark_flux(&[], &lines, &mut crate::physics::Physics::default().stroke_n(1 << 16));
        let Ok(carried) = crate::transducer::mind::with_field(|f| f.take_carried()) else { panic!("a mind is thinking") };
        let [(line, increment)] = carried.as_slice() else { panic!("one fibre carried: {}", carried.len()) };
        let pairs = [(&vs[0], &vs[1]), (&vs[2], &vs[3])];
        let Some(&(a, b)) = pairs.iter().find(|(a, b)| edge_key(a, b) == *line) else { panic!("a tether's own line") };
        assert!(mind.complex.carry(line, increment), "the fibre moves");
        let half = turn_90().geo(&turn_90());
        assert!(mind.complex.tethered_as(a, b, &half, &ident()), "carried by exactly the orbit's 90°: it carries 180°");
    });
}

/// THE WALK `d, a, s, d` ON ONE CELL (78.99 (3), the cross). `d` crosses
/// through the puncture: the class moves from the sealed axis to the hole
/// axis; `a` refuses at the hole — the dipole reads back by `s`, the seal,
/// alone — and `s` returns the chamber; the last `d` crosses again. The board
/// it leaves is the board `p` alone leaves: the descent IS the puncture,
/// with the content handed down.
#[test]
fn the_walk_descend_ascend_seal_descend_reads_as_ruled() {
    let mut c = Complex::new();
    let (vs, key) = sealed_triangle(&mut c, 1, turn_90());
    let sealed = c.board();
    let walk = [Pivot::Descend(key.clone()), Pivot::Ascend(vs.clone()), Pivot::Seal(key.clone()), Pivot::Descend(key.clone())];
    let read: Vec<Applied> = walk.iter().map(|p| c.apply(p, |_| vol(), vol())).collect();
    assert!(matches!(read[0], Applied::Descended(Descended::Throat(_))), "the descend crosses through the puncture");
    assert!(matches!(read[1], Applied::Ascended(Ascended::Hole)), "ascend refuses at the hole: only the seal reads it back");
    assert!(matches!(read[2], Applied::Capped(Capped::Sealed { .. })), "the seal returns the chamber");
    assert!(matches!(read[3], Applied::Descended(Descended::Throat(_))), "and it crosses again");
    let mut alone = Complex::new();
    let (_, k2) = sealed_triangle(&mut alone, 1, turn_90());
    assert_eq!(alone.board(), sealed);
    let _ = alone.puncture(&k2);
    assert_eq!(c.board(), alone.board(), "the walk leaves what p alone leaves");
}

/// THE LOCALITY THEOREM. Two cells sharing no vertex: a descend of one and a
/// puncture of the other, in either order, leave one board and one outflow.
/// On one cell the cross shows itself: `d` then `p` and `p` then `d` leave one
/// board — the descent is the puncture — and differ in what flowed: `d`
/// first handed the content down, `p` first kept it, and the descend then
/// refuses at the hole.
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
            .map(|f| {
                let mut k = Vec::new();
                f.motion.put_canonical(&mut k);
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
        matches!(first_d.apply(&Pivot::Puncture(ka.clone()), |_| vol(), vol()), Applied::Punctured(Punctured::Hole)),
        "the descend left the hole: a hole is not punctured twice"
    );
    let (mut first_p, ..) = build();
    let _ = first_p.apply(&Pivot::Puncture(ka.clone()), |_| vol(), vol());
    assert!(
        matches!(first_p.apply(&Pivot::Descend(ka.clone()), |_| vol(), vol()), Applied::Descended(Descended::Hole)),
        "the descend refuses at the hole"
    );
    assert_eq!(first_d.board(), first_p.board(), "on one cell the descent is the puncture: one board");
    assert_eq!(first_d.take_outflow().len(), 1, "d first handed the content down");
    assert!(first_p.take_outflow().is_empty(), "p first kept it in");
}
