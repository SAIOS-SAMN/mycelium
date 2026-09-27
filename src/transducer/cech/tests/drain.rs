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
