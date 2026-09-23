// 78.11, CG1 — EACH CERTIFIED CELL OF A FORM READ AS ITS ORBIT ALGEBRA: the
// circuits that cross it, their holonomies through the one composite reader
// (OR3), the cell's round and constructors — and the census's reading of
// them by grade.
//
//! A circuit CROSSES a cell where one of its steps runs between members
//! standing in two different bodies of the cell: it passes through the
//! cell's contact, which is where its transport meets the cell's round
//! (78.13(3): the circuits through a cell are the generators its algebra
//! combines). A circuit that only visits one body of the cell does not
//! cross it. Read by the cell's own member keys (NB1's return), one keyed
//! read per member, never a pair walk.

use crate::network::nerve::Keyed;
use crate::transducer::cech::{CellAlgebra, Coherence, Congruence};
use crate::transducer::cga::{ConstructorOrbit, Mv};
use crate::transducer::sentence::Form;

/// EVERY CERTIFIED CELL OF `form`, READ AS ITS ORBIT ALGEBRA, under the
/// stroke's budget, in the cells' own key order. The circuits are the ones
/// the percept's own walk kept — `(members, composite)` — never walked again.
pub(crate) fn cell_algebras(
    form: &Form,
    walked: &[(Vec<Vec<u8>>, Option<Mv>)],
    budget: &mut crate::physics::Budget,
) -> Vec<CellAlgebra> {
    form.cells
        .scoped()
        .map(|(_, cell)| {
            let mut body_of: Keyed<usize> = Keyed::new();
            for (b, members) in cell.bodies.iter().enumerate() {
                for m in members {
                    body_of.insert(m, b);
                }
            }
            let circuits: Vec<Option<Mv>> = walked
                .iter()
                .filter(|(members, _)| {
                    members.windows(2).any(|w| match (body_of.get(&w[0]), body_of.get(&w[1])) {
                        (Some(a), Some(b)) => a != b,
                        _ => false,
                    })
                })
                .map(|(_, composite)| composite.clone())
                .collect();
            crate::transducer::cech::read_cell(
                cell.grade(),
                cell.meet().as_ref(),
                cell.nuclei.as_deref(),
                &circuits,
                budget,
            )
        })
        .collect()
}

/// ONE GRADE'S ROW: the grade; congruence `(congruent, incongruent, unread,
/// no round, no circuit)`; coherence `(closes, held, open, unread)`; the
/// capacities read; the constructors' orbits `(closed, open, unbounded,
/// unread)`.
type GradeRow = (usize, [u64; 5], [u64; 4], Vec<usize>, [u64; 4]);

/// THE CELLS' READING BY GRADE, one row per grade, ascending.
#[derive(Default)]
pub(crate) struct CellTally {
    rows: Vec<GradeRow>,
}

impl CellTally {
    pub(crate) fn of(cells: &[CellAlgebra]) -> CellTally {
        let mut t = CellTally::default();
        for c in cells {
            let at = match t.rows.binary_search_by_key(&c.grade, |r| r.0) {
                Ok(at) => at,
                Err(at) => {
                    t.rows.insert(at, (c.grade, [0; 5], [0; 4], Vec::new(), [0; 4]));
                    at
                }
            };
            let row = &mut t.rows[at];
            row.1[match c.congruence {
                Congruence::Congruent => 0,
                Congruence::Incongruent { .. } => 1,
                Congruence::Unread { .. } => 2,
                Congruence::NoRound => 3,
                Congruence::NoCircuit => 4,
            }] += 1;
            row.2[match c.coherence {
                Coherence::Closes(_) => 0,
                Coherence::Held => 1,
                Coherence::Open(_) => 2,
                Coherence::Unread => 3,
            }] += 1;
            if let Some(k) = c.capacity {
                row.3.push(k);
            }
            row.4[match c.constructors {
                ConstructorOrbit::Closed { .. } => 0,
                ConstructorOrbit::Open { .. } => 1,
                ConstructorOrbit::Unbounded { .. } => 2,
                ConstructorOrbit::Unread => 3,
            }] += 1;
        }
        t
    }

    /// The census line's segment.
    pub(crate) fn line(&self) -> String {
        let rows: Vec<String> = self
            .rows
            .iter()
            .map(|(g, cg, ch, cap, co)| {
                format!(
                    "grade {g}: congruent {} incongruent {} unread {} no-round {} no-circuit {} | coherence closes {} held {} open {} unread {} | capacity {cap:?} | constructors closed {} open {} unbounded {} unread {}",
                    cg[0], cg[1], cg[2], cg[3], cg[4], ch[0], ch[1], ch[2], ch[3], co[0], co[1], co[2], co[3]
                )
            })
            .collect();
        match rows.is_empty() {
            true => "cells: none certified".into(),
            false => format!("cells: {}", rows.join(" ; ")),
        }
    }
}
