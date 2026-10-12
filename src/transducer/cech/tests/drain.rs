// 62.1, T2 — `drain`, THE CONJUGATE OF `condense`, and the pinning rule.
//
//! `ascend` had no inverse and the complex could only GAIN cells: a
//! ratchet, which accretes history rather than finding a ground state.
//! These assert the four outcomes of the release and, above all, that an
//! edge beneath a READING cannot be pulled out from under it.

use super::common::*;
use crate::transducer::alphabet::Rotor;
use crate::transducer::cech::*;
use crate::transducer::element::identity_versor as ident;

/// The cell a fixture built at a link, read by a slice pattern: the
/// fixture holds one, so there is no position to choose among.
fn cell_of(cells: Vec<Vec<u8>>, what: &str) -> Vec<u8> {
    match cells.as_slice() {
        [k, ..] => k.clone(),
        [] => panic!("{what}"),
    }
}

/// Two sectioned vertices and the edge between them.
fn pair(g: Rotor) -> (Complex, Vec<u8>) {
    let mut c = Complex::new();
    for n in [v(1), v(2)] {
        c.set_section(&n, Section::at(n.clone(), Rotor::identity()), vol());
    }
    c.set_edge(v(1), v(2), g, ident(), vol());
    let key = cell_of(c.link(v(1).as_bytes(), 1), "the edge stands");
    (c, key)
}

/// A TETHER CARRYING NOTHING RETURNS TO THE VACUUM.
///
/// Identity rotor, identity carriage: the coupling was real while it stood,
/// and its dissolution has no topological consequence, so nothing is filed
/// and the cell leaves every carrier that named it.
#[test]
fn a_trivial_tether_drains_to_vacuum() {
    let (mut c, key) = pair(Rotor::identity());
    assert!(c.simplex(&key).is_some(), "the edge stands before the drain");

    let out = c.drain(&key);
    assert!(matches!(out, Drained::Vacuum), "identity transport files nothing: {out:?}");
    assert!(out.released(), "and the cell left");
    assert!(out.cauterized().is_none(), "with no class to carry");
    assert!(c.simplex(&key).is_none(), "the 1-cell is gone");
    assert!(
        c.link(v(1).as_bytes(), 1).is_empty(),
        "and gone from the star too, not only from the degree layer"
    );
}

/// A STRAINED TETHER IS CAUTERIZED, NEVER DROPPED.
///
/// A 90° turn between the two ends is a real coupling. When the bodies
/// leave one another's horizon that strain cannot be wiped from the ledger
/// — the reading travels out so the caller can file it as torsion or carry
/// it to the court as a demand (rule 11).
#[test]
fn a_strained_tether_is_cauterized_and_its_class_travels() {
    let (mut c, key) = pair(turn_90());
    let out = c.drain(&key);
    let fill = out.cauterized().expect("a turn is not nothing");
    assert!(
        !matches!(fill, Fill::Open),
        "the severed coupling is classified, not left open: {fill:?}"
    );
    assert!(out.released(), "and the cell still left the complex");
    assert!(c.simplex(&key).is_none(), "a cauterized edge is gone too");
}

/// 78.74 — EVERY SEAL IS BOOKED: a membrane sealed by the climb's own `fill`
/// — not `ascend` — counts on the ledger, once.
#[test]
fn a_seal_by_the_climbs_fill_is_booked() {
    let mut c = Complex::new();
    let vs = [v(1), v(2), v(3)];
    for x in &vs {
        c.set_section(x, Section::at(x.clone(), Rotor::identity()), vol());
    }
    for i in 0..3 {
        for j in i + 1..3 {
            c.set_edge(&vs[i], &vs[j], Rotor::identity(), ident(), vol());
        }
    }
    c.coboundary_on_link(vs[0].as_bytes(), |_| vol());
    let face = cell_of(c.link(vs[0].as_bytes(), 2), "a 2-cell stands");
    let before = crate::physics::metabolism::metabolism().sealed;
    assert!(c.fill(&face).attached().is_some(), "sealed by fill");
    assert_eq!(crate::physics::metabolism::metabolism().sealed, before + 1, "booked once");
    assert_eq!(c.sealed, 1, "and on the complex itself, which any thread can read");
    assert!(matches!(c.fill(&face), CoboundaryOutcome::AlreadyFilled));
    assert_eq!(crate::physics::metabolism::metabolism().sealed, before + 1, "a settled cell is not sealed twice");
}

fn q(n: i64, d: i64) -> num_rational::BigRational {
    num_rational::BigRational::new(n.into(), d.into())
}

fn wide() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// THE MOTIONS A FACE CAN HOLD, built by the physics and never by a planar
/// turn: a fibre's CHARGE (a boost fixing its two pivots), a white-hole
/// SLIVER (a null flow out of a point of a geoid, kept inside it), and the
/// LOXODROME (the charge with a twist about the fibre's own axis: one turn,
/// one boost).
fn charge() -> crate::transducer::cga::Mv {
    let pi = crate::transducer::cga::point_at(&[q(0, 1), q(0, 1), q(0, 1)]);
    let pj = crate::transducer::cga::point_at(&[q(3, 1), q(4, 1), q(0, 1)]);
    crate::physics::capacitor::Fibre::new(pi, pj, q(2, 1), q(1, 1)).expect("a fibre").charge(&q(3, 2))
}

fn sliver() -> crate::transducer::cga::Mv {
    use crate::transducer::cga::{self, Mv, E1, E3};
    // Σ about (1, 1, 0) through P = (4, 5, 0); K a turn about Σ's centre in
    // the plane of e₁ and e₃, which keeps Σ and moves P along it.
    let (c, p) = ([q(1, 1), q(1, 1), q(0, 1)], [q(4, 1), q(5, 1), q(0, 1)]);
    let (cc, pp) = (cga::point_at(&c), cga::point_at(&p));
    let sigma = cc.sub(&cga::e_inf().scale(&-cc.geo(&pp).scalar_part()));
    let t = cga::translator_at(&c);
    let k = t.geo(&Mv::blade(E1, q(1, 1)).geo(&Mv::blade(E3, q(1, 1)))).geo(&t.reverse()).grade(2);
    let src = crate::physics::source::Source::of(&sigma, &pp, &k).expect("a source on Σ at P");
    src.sliver(&q(1, 2))
}

fn loxodrome() -> crate::transducer::cga::Mv {
    use crate::transducer::cga::{Mv, E1, E2, E3};
    let normal = Mv::blade(E1, q(4, 1)).sub(&Mv::blade(E2, q(3, 1)));
    let twist = Mv::scalar(q(1, 1)).add(&Mv::blade(E3, q(1, 1)).geo(&normal).grade(2).scale(&q(1, 5)));
    twist.geo(&charge())
}

/// A triangle of three vantages whose edge v1–v2 carries `m` (the others
/// flat): its face's holonomy is `m`.
fn face_over(m: &crate::transducer::cga::Mv) -> (Complex, Vec<u8>) {
    let mut c = Complex::new();
    let vs = [v(1), v(2), v(3)];
    for x in &vs {
        c.set_section(x, Section::at(x.clone(), Rotor::identity()), vol());
    }
    c.set_edge(&vs[0], &vs[1], Rotor::identity(), m.clone(), vol());
    c.set_edge(&vs[0], &vs[2], Rotor::identity(), ident(), vol());
    c.set_edge(&vs[1], &vs[2], Rotor::identity(), ident(), vol());
    c.coboundary_on_link(vs[0].as_bytes(), |_| vol());
    let face = cell_of(c.link(vs[0].as_bytes(), 2), "the face");
    (c, face)
}

/// 78.75 — THE NERVE BY DEGREE, ITS FACES READ BY THEIR MOTION: a triangle of
/// three vantages reads three vertices, three edges and one face, and the
/// face's content is its holonomy as one `Cl(4,1)` motion, read by its modes
/// — never by whether a rotor is the identity. A face holding a fibre's
/// charge holds a BOOST; one holding a white-hole sliver holds a NULL flow;
/// one holding the loxodrome holds a TURN and a BOOST at once; a flat
/// triangle holds no mode at all. Each reading is exact and is the face's
/// own: the motion read back off the cell is the one its edge carried.
#[test]
fn the_census_reads_each_face_by_the_motion_it_holds() {
    let (flat, _) = face_over(&ident());
    let census = flat.census(&mut wide());
    assert!(census.len() >= 3, "vertices, edges and a face: {census:?}");
    assert_eq!((census[1].cells, census[2].cells, census[2].open), (3, 1, 1), "three edges, one face standing open");
    assert_eq!((census[2].flat, census[2].curved()), (1, Some(0)), "a flat triangle holds no mode");
    for (name, m, turn, boost, null) in [
        ("charge", charge(), 0, 1, 0),
        ("sliver", sliver(), 0, 0, 1),
        ("loxodrome", loxodrome(), 1, 1, 0),
    ] {
        let (c, face) = face_over(&m);
        assert!(c.simplex(&face).expect("the face").motion().projective_eq(&m), "{name}: the face's motion is the holonomy its edge carried");
        let d2 = c.census(&mut wide())[2];
        assert_eq!((d2.turn, d2.boost, d2.null, d2.flat), (turn, boost, null, 0), "{name}: {d2:?}");
        assert_eq!(d2.curved(), Some(1), "{name}: the face carries content");
    }
    // A stroke that reaches nothing reads nothing, by name.
    let (c, _) = face_over(&charge());
    let none = c.census(&mut crate::physics::Physics::default().stroke_n(0));
    assert_eq!(none[2].modes_unread, 1, "unread, never called flat");
}

/// 78.75, 78.106 — THE PIVOTS' OUTFLOW: a complex whose owner listens hands
/// back every tether `drain` cauterizes — the strain flows out to the owner's
/// field instead of ending in a counter, AS IT UNWINDS: its reverse, the
/// valve venting the pressure it held (`m̃ · m` the identity) — and nothing
/// while no owner listens; a vacuum tether holds no pressure and vents
/// nothing.
#[test]
fn a_cauterized_tethers_motion_flows_out_to_its_owner() {
    let edge_of = |m: &crate::transducer::cga::Mv| {
        let mut c = Complex::new();
        for n in [v(1), v(2)] {
            c.set_section(&n, Section::at(n.clone(), Rotor::identity()), vol());
        }
        c.set_edge(v(1), v(2), Rotor::identity(), m.clone(), vol());
        let key = cell_of(c.link(v(1).as_bytes(), 1), "the edge stands");
        (c, key)
    };
    let (mut deaf, key) = edge_of(&sliver());
    assert!(matches!(deaf.drain(&key), Drained::Cauterized(_)));
    assert!(deaf.take_outflow().is_empty(), "no owner listening: nothing held");
    let (mut heard, key) = edge_of(&sliver());
    heard.arm_outflow();
    assert!(matches!(heard.drain(&key), Drained::Cauterized(_)), "a sliver is strain");
    let out = heard.take_outflow();
    assert!(matches!(out.as_slice(), [f] if f.motion.projective_eq(&sliver().reverse())), "the sliver unwound: its reverse vented whole: {out:?}");
    assert!(
        matches!(out.as_slice(), [f] if f.motion.geo(&sliver()).projective_eq(&ident())),
        "the vent undoes the strain exactly"
    );
    assert!(matches!(out.as_slice(), [f] if f.line == Thread::Unwound(key.clone())), "on the tether's own line, unwound");
    assert!(heard.take_outflow().is_empty(), "taken once");
    let (mut vacuum, key) = edge_of(&ident());
    vacuum.arm_outflow();
    assert!(matches!(vacuum.drain(&key), Drained::Vacuum));
    assert!(vacuum.take_outflow().is_empty(), "nothing carried, nothing out");
}

/// 78.74 — THE NERVE'S ELASTIC VENT: releasing a vertex lets its whole star
/// go through the pivots — the sealed membrane descends first and hands its
/// fill back, then its tethers drain — and leaves the cells not on it
/// standing, with no ghost incidence. A tetrahedron's vertex releases its
/// 3-cell too: `descend` is gated at every degree from two up.
#[test]
fn releasing_a_vertex_vents_its_star_through_the_pivots() {
    let mut c = Complex::new();
    let vs = [v(1), v(2), v(3)];
    for x in &vs {
        c.set_section(x, Section::at(x.clone(), Rotor::identity()), vol());
    }
    for i in 0..3 {
        for j in i + 1..3 {
            c.set_edge(&vs[i], &vs[j], Rotor::identity(), ident(), vol());
        }
    }
    c.coboundary_on_link(vs[0].as_bytes(), |_| vol());
    let face = cell_of(c.link(vs[0].as_bytes(), 2), "a 2-cell stands");
    assert!(c.fill(&face).attached().is_some(), "the membrane is sealed, pinning its boundary");
    let r = c.release_vertex(vs[0].as_bytes());
    assert_eq!((r.released, r.vacuum, r.pinned), (1, 2, 0), "the membrane released with its fill, both tethers to vacuum: {r:?}");
    assert!(c.simplex(&face).is_none() && c.sections.get(vs[0].as_bytes()).is_none(), "the vertex and its cells are gone");
    assert_eq!(c.link(vs[1].as_bytes(), 1).len(), 1, "the edge v2–v3 stands, unpinned");
    for x in &vs[1..] {
        for k in c.link_of(x.as_bytes()) {
            assert!(c.simplex(&k).is_some(), "no ghost incidence");
        }
    }

    let mut c = Complex::new();
    let vs = [v(1), v(2), v(3), v(4)];
    for x in &vs {
        c.set_section(x, Section::at(x.clone(), Rotor::identity()), vol());
    }
    for i in 0..4 {
        for j in i + 1..4 {
            c.set_edge(&vs[i], &vs[j], Rotor::identity(), ident(), vol());
        }
    }
    c.coboundary_on_link(vs[0].as_bytes(), |_| vol());
    let solid = c.link(vs[0].as_bytes(), 3);
    assert!(!solid.is_empty(), "the climb reached a 3-cell");
    for k in c.link(vs[0].as_bytes(), 3).into_iter().chain(c.link(vs[0].as_bytes(), 2)) {
        let _ = c.fill(&k);
    }
    let r = c.release_vertex(vs[0].as_bytes());
    assert_eq!(r.pinned, 0, "highest degree first: nothing pinned: {r:?}");
    assert!(r.released + r.unsealed >= 4, "the 3-cell and its three faces on v1 released: {r:?}");
    assert!(solid.iter().all(|k| c.simplex(k).is_none()), "the 3-cell is gone");
}

/// THE PINNING RULE — WHAT MAKES THIS A PIVOT AND NOT AN UNDO STACK.
///
/// Once a 2-cell over three vertices has been READ, the class it carries is
/// historical. Draining an edge of its boundary would retroactively
/// invalidate that reading, and observation is a one-way phase transition
/// from geometry into algebra (62.4). The edge refuses, and it refuses by
/// NAMING the reason rather than by answering absent.
#[test]
fn an_edge_beneath_a_reading_is_pinned_until_the_coface_descends() {
    let mut c = Complex::new();
    let vs = [v(1), v(2), v(3)];
    for x in &vs {
        c.set_section(x, Section::at(x.clone(), Rotor::identity()), vol());
    }
    for i in 0..3 {
        for j in i + 1..3 {
            c.set_edge(&vs[i], &vs[j], Rotor::identity(), ident(), vol());
        }
    }
    c.coboundary_on_link(vs[0].as_bytes(), |_| vol());
    let face = cell_of(c.link(vs[0].as_bytes(), 2), "a 2-cell stands");
    let edge = cell_of(c.link(vs[0].as_bytes(), 1), "an edge stands");

    // THE COFACE IS UNREAD, so nothing has been recorded that draining
    // could invalidate and the edge is free.
    assert!(
        matches!(c.simplex(&face).expect("held").fill, Fill::Open),
        "the 2-cell has not been read yet"
    );
    assert!(
        matches!(c.drain(&edge), Drained::Vacuum),
        "an edge under an UNREAD coface drains freely"
    );

    // Rebuild the complex and TAKE the reading this time.
    let mut c = Complex::new();
    for x in &vs {
        c.set_section(x, Section::at(x.clone(), Rotor::identity()), vol());
    }
    for i in 0..3 {
        for j in i + 1..3 {
            c.set_edge(&vs[i], &vs[j], Rotor::identity(), ident(), vol());
        }
    }
    c.coboundary_on_link(vs[0].as_bytes(), |_| vol());
    let face = cell_of(c.link(vs[0].as_bytes(), 2), "a 2-cell stands");
    let edge = cell_of(c.link(vs[0].as_bytes(), 1), "an edge stands");
    let attached = c.fill(&face);
    assert!(attached.attached().is_some(), "the 2-cell is read: {attached:?}");

    let out = c.drain(&edge);
    assert!(
        matches!(out, Drained::Pinned),
        "an edge beneath a recorded class cannot be pulled out: {out:?}"
    );
    assert!(!out.released(), "and the cell is still standing");
    assert!(c.simplex(&edge).is_some(), "the 1-cell survived the refusal");
}

/// T2b — THE LADDER BREATHES: mint, seal, re-open, release.
///
/// The one test that proves the conjugate pairs actually close. A 1-cell
/// enters a 2-cell, is pinned there, and the ONLY named remedy is
/// `descend`; before it existed that edge was immortal and the complex
/// could still accrete at degree 2 with no way back. Here the whole cycle
/// runs and the complex comes back empty of cells, with no ghost incidence
/// left in any star.
#[test]
fn a_cell_ascends_descends_and_then_drains_leaving_no_ghost() {
    let mut c = Complex::new();
    let vs = [v(1), v(2), v(3)];
    for x in &vs {
        c.set_section(x, Section::at(x.clone(), Rotor::identity()), vol());
    }
    for i in 0..3 {
        for j in i + 1..3 {
            c.set_edge(&vs[i], &vs[j], Rotor::identity(), ident(), vol());
        }
    }
    // ASCEND — the boundary closes, so a membrane seals it.
    c.coboundary_on_link(vs[0].as_bytes(), |_| vol());
    let face = cell_of(c.link(vs[0].as_bytes(), 2), "a 2-cell stands");
    assert!(c.fill(&face).attached().is_some(), "the 2-cell is read");

    let edge = cell_of(c.link(vs[0].as_bytes(), 1), "an edge stands");
    assert!(
        matches!(c.drain(&edge), Drained::Pinned),
        "and its boundary is pinned beneath the reading"
    );

    // DESCEND — the membrane releases and hands its class back.
    let down = c.descend(&face);
    assert!(down.opened(), "a 2-cell with no 3-cell above it descends: {down:?}");
    assert!(
        down.released().is_some(),
        "and a SEALED membrane hands its class out rather than erasing it: {down:?}"
    );
    assert!(c.simplex(&face).is_none(), "the membrane is gone");

    // THE BOUNDARY SURVIVED AND IS UNPINNED. This is the whole point of
    // the pair: descending re-opens the cycle without destroying it.
    assert_eq!(
        c.link(vs[0].as_bytes(), 1).len(),
        2,
        "both 1-cells on this vertex are still standing"
    );
    assert!(
        matches!(c.drain(&edge), Drained::Vacuum),
        "and the edge that was pinned now drains cleanly"
    );

    // NO GHOST INCIDENCE anywhere: every star is consistent with what the
    // complex can actually evaluate.
    for x in &vs {
        for k in c.link_of(x.as_bytes()) {
            assert!(
                c.simplex(&k).is_some(),
                "a star names only cells the complex holds"
            );
        }
    }
}

/// `descend` IS GATED AT DEGREE 2, as `drain` is at degree 1. Releasing a
/// 1-cell is `drain`'s and a 0-cell is neither's; answering here would
/// scramble the ladder.
#[test]
fn descend_answers_only_at_dimension_two() {
    let (mut c, edge) = pair(Rotor::identity());
    assert!(matches!(c.descend(&edge), Descended::Absent), "an edge is drained");
    assert!(
        matches!(c.descend(v(1).as_bytes()), Descended::Absent),
        "a 0-cell is not a membrane"
    );
}

/// AN UNREAD MEMBRANE SEALED NOTHING, so it hands nothing back. The two
/// successful descents are different facts and are kept apart, as
/// `Vacuum` and `Cauterized` are at degree 1.
#[test]
fn an_unread_membrane_descends_without_a_class() {
    let mut c = Complex::new();
    let vs = [v(1), v(2), v(3)];
    for x in &vs {
        c.set_section(x, Section::at(x.clone(), Rotor::identity()), vol());
    }
    for i in 0..3 {
        for j in i + 1..3 {
            c.set_edge(&vs[i], &vs[j], Rotor::identity(), ident(), vol());
        }
    }
    c.coboundary_on_link(vs[0].as_bytes(), |_| vol());
    let face = cell_of(c.link(vs[0].as_bytes(), 2), "a 2-cell stands");

    let down = c.descend(&face);
    assert!(matches!(down, Descended::Unsealed), "it recorded nothing: {down:?}");
    assert!(down.released().is_none(), "so there is no class to hand back");
    assert!(down.opened(), "and the membrane still left");
}

/// A CELL THAT IS NOT THERE IS NOT A REFUSAL. `Absent` and `Pinned` are
/// different facts, and answering one for both is the defect
/// `CoboundaryOutcome` was split to avoid.
#[test]
fn draining_nothing_is_absent_and_not_pinned() {
    let mut c = Complex::new();
    let out = c.drain(&[9, 9, 9]);
    assert!(matches!(out, Drained::Absent), "no such 1-cell: {out:?}");
    assert!(!out.released());
}

/// A 0-CELL AND A 2-CELL ARE NOT TETHERS. `drain` is the conjugate of
/// `condense`, which operates between dimensions 1 and 0; releasing a
/// 2-cell is `descend`, a different pivot on a different pair, and
/// answering here would scramble the dimensional hierarchy.
#[test]
fn drain_answers_only_at_dimension_one() {
    let mut c = Complex::new();
    let vs = [v(1), v(2), v(3)];
    for x in &vs {
        c.set_section(x, Section::at(x.clone(), Rotor::identity()), vol());
    }
    for i in 0..3 {
        for j in i + 1..3 {
            c.set_edge(&vs[i], &vs[j], Rotor::identity(), ident(), vol());
        }
    }
    c.coboundary_on_link(vs[0].as_bytes(), |_| vol());
    let face = cell_of(c.link(vs[0].as_bytes(), 2), "a 2-cell stands");

    assert!(
        matches!(c.drain(&face), Drained::Absent),
        "a 2-cell is descended, never drained"
    );
    assert!(
        matches!(c.drain(vs[0].as_bytes()), Drained::Absent),
        "a 0-cell is not a tether"
    );
}

/// A DROPPED SIMPLEX TAKES ITS ORBITS WITH IT (78.48(3)). The orbits of a
/// simplex stand at coordinates its key prefixes; a vertex dropped from the
/// complex drops every simplex on it, and the orbits filed under them leave
/// too — they stood under every dropped ring for as long as the mind lived.
#[test]
fn a_dropped_vertex_takes_its_simplices_orbits() {
    let (mut c, key) = pair(Rotor::identity());
    let orbit = crate::transducer::cech::orbit_key(&key, v(1).as_bytes(), v(2).as_bytes());
    c.orbits.insert(orbit, Fill::Open);
    assert_eq!(c.orbits_of(&key).len(), 1, "the orbit stands under its simplex");
    c.drop_vertex(v(1).as_bytes());
    assert!(c.orbits_of(&key).is_empty(), "and leaves with it");
}
