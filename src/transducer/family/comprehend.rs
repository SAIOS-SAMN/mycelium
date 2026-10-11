// 78.11, 78.14, CG1, CS2, BD2 — EACH CERTIFIED CELL OF A FORM READ AS ITS
// ORBIT ALGEBRA: the circuits that cross it, their holonomies through the
// one composite reader (OR3), the cell's round, constructors and Gram form —
// the cells a held form's pivot reads against.
//
//! A circuit CROSSES a cell where one of its steps crosses a contact face
//! between two different bodies of the cell: it passes through the cell's
//! contact, where its transport meets the cell's round (78.13(3)). The
//! step's bodies are read off the face it crosses — the form's side carrier
//! of each contact face's body pair (BD2) — never derived from its member,
//! whose support can fall into several bodies (checklist (m)). One keyed
//! read per step, never a pairwise read.

use crate::network::nerve::Keyed;
use crate::transducer::cech::CellAlgebra;
use crate::transducer::sentence::Form;

/// THE CIRCUITS CROSSING ONE CELL, by their place in the traversal, each with
/// its order of the cell's bodies (CX1): each body as the circuit's crossings
/// first reach it. A step crosses where the face it crosses joins two
/// different bodies of the cell, read off the face's oriented pair and the
/// step's way (BD2).
fn crossings(
    form: &Form,
    cell: &crate::transducer::sentence::FormCell,
    traversed: &[crate::transducer::sentence::TraversedCircuit],
) -> Vec<(usize, Vec<usize>)> {
    let mut body_of: Keyed<usize> = Keyed::new();
    for (b, piece) in cell.bodies.iter().enumerate() {
        body_of.insert(piece, b);
    }
    let crossing = |face: &crate::network::nerve::coord::Overlap, forward: bool| -> Option<(usize, usize)> {
        let (x, y) = form.face_bodies.at(face)?;
        let (a, b) = (*body_of.get(x)?, *body_of.get(y)?);
        match (a == b, forward) {
            (true, _) => None,
            (false, true) => Some((a, b)),
            (false, false) => Some((b, a)),
        }
    };
    let mut out = Vec::new();
    for (at, t) in traversed.iter().enumerate() {
        let mut order: Vec<usize> = Vec::new();
        for (face, forward) in &t.steps {
            if let Some((from, to)) = crossing(face, *forward) {
                for b in [from, to] {
                    if !order.contains(&b) {
                        order.push(b);
                    }
                }
            }
        }
        if !order.is_empty() {
            out.push((at, order));
        }
    }
    out
}

/// CS2 — THE CELLS A HELD FORM'S PIVOT READS AGAINST: each certified cell's
/// orbit algebra over the form's own circuits (`circuits_of`), and by orbit
/// chord the cells its circuits cross. A reader on the cognition path: it
/// spends the pivot's stroke, once per form per change (memoised on the idea).
pub(crate) fn pivot_cells(form: &Form, budget: &mut crate::physics::Budget) -> crate::transducer::sentence::PivotCells {
    let noted = crate::transducer::sentence::circuits_of(form);
    let traversed: Vec<crate::transducer::sentence::TraversedCircuit> = noted.iter().map(|(_, t)| t.clone()).collect();
    let algebras = cell_algebras(form, &traversed, budget);
    let mut by_chord: Keyed<Vec<usize>, crate::network::nerve::coord::Overlap> = Keyed::new();
    for (k, (_, cell)) in form.cells.scoped().enumerate() {
        for (at, _) in crossings(form, cell, &traversed) {
            let Some((chord, _)) = noted.get(at) else { continue };
            match by_chord.at_mut(chord) {
                Some(cells) if !cells.contains(&k) => cells.push(k),
                Some(_) => {}
                None => {
                    by_chord.put(chord, vec![k]);
                }
            }
        }
    }
    crate::transducer::sentence::PivotCells { algebras, by_chord }
}

/// EVERY CERTIFIED CELL OF `form`, READ AS ITS ORBIT ALGEBRA, under the
/// stroke's budget, in the cells' own key order, off the circuits given.
pub(crate) fn cell_algebras(
    form: &Form,
    traversed: &[crate::transducer::sentence::TraversedCircuit],
    budget: &mut crate::physics::Budget,
) -> Vec<CellAlgebra> {
    // Each cell's crossing circuits and their orders of its bodies.
    type Crossing = (Vec<crate::transducer::cech::CellCircuit>, Vec<Option<Vec<usize>>>);
    let per_cell: Vec<Crossing> = form
        .cells
        .scoped()
        .map(|(_, cell)| {
            let mut circuits: Vec<crate::transducer::cech::CellCircuit> = Vec::new();
            let mut orders: Vec<Option<Vec<usize>>> = Vec::new();
            for (at, order) in crossings(form, cell, traversed) {
                let Some(crate::transducer::sentence::TraversedCircuit { steps, composite, hue, unread }) =
                    traversed.get(at)
                else {
                    continue;
                };
                // OR3's reason rides the circuit into the cell; a composite
                // missing with no reason recorded is the unnamed refusal, by
                // its own name. The ring is the circuit's own steps (78.26(5)).
                use crate::transducer::cech::{CellCircuit, CircuitUnread};
                circuits.push(CellCircuit {
                    ring: CellCircuit::ring_of(steps),
                    holonomy: match (composite, unread) {
                        (Some(h), _) => Ok(h.clone()),
                        (None, Some(why)) => Err(*why),
                        (None, None) => Err(CircuitUnread::Unnamed),
                    },
                    hue: hue.clone(),
                });
                orders.push((order.len() == cell.bodies.len()).then_some(order));
            }
            (circuits, orders)
        })
        .collect();
    // EVERY CELL'S RANK BEFORE ANY CELL'S MODES (78.11 items 4 and 5 across
    // the cells, §6.4, F1b): a stroke that runs out leaves every cell its
    // dimension, never the first cell its modes and the rest nothing.
    let ranks: Vec<crate::transducer::cech::CellRank> =
        per_cell.iter().map(|(circuits, _)| crate::transducer::cech::read_rank(circuits, budget)).collect();
    form.cells
        .scoped()
        .zip(per_cell)
        .zip(ranks)
        .map(|(((_, cell), (circuits, orders)), rank)| {
            crate::transducer::cech::read_cell_after(
                rank,
                cell.grade(),
                cell.meet().as_ref(),
                cell.nuclei.as_deref(),
                &circuits,
                &orders,
                budget,
            )
        })
        .collect()
}
