// CS2, 78.32(c) — THE PIVOT READ AGAINST THE CELLS ITS ORBIT CROSSES: each
// orbit's move classified by whether it raises the crossing cell's closed
// algebra, on the orbit's own reading — note only; the gate is 78.32(c)'s,
// after the recut's first reading.
//
//! A move REACHES where its action is not yet in the crossing cell's closed
//! algebra — a new direction; is DERIVABLE where it is (`algebra_contains`,
//! the rank not increasing); a Vortex is derivable only where every
//! certificate is. An orbit no candidate discharged DEMANDS a mode, the
//! adjunction from its residue counted apart; a search the stroke cut short
//! is STARVED; a move with no action to test (a hue move, a fibre move) or a
//! cell whose algebra did not read is UNREAD by which.

use super::*;
use crate::transducer::cech::{algebra_contains, CellAlgebra};

/// THE CELLS A HELD FORM'S PIVOT READS AGAINST: each cell's orbit algebra
/// over the form's own circuits, and, by orbit chord, the cells its circuits
/// cross. Read once per form and memoised on the idea while the form stands.
#[derive(Clone, Debug, Default)]
pub struct PivotCells {
    pub algebras: Vec<CellAlgebra>,
    pub by_chord: Keyed<Vec<usize>>,
}

/// CS2 — THE PIVOT'S OUTCOMES BY CONTENT, counted per orbit per crossing
/// cell (an orbit crossing two cells counted at both).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PivotTally {
    pub reached: u64,
    pub derivable: u64,
    pub demanded: u64,
    pub adjoin: u64,
    pub starved: u64,
    pub unread_fibre: u64,
    pub unread_algebra: u64,
}

impl PivotTally {
    pub fn add(&mut self, o: &PivotTally) {
        self.reached += o.reached;
        self.derivable += o.derivable;
        self.demanded += o.demanded;
        self.adjoin += o.adjoin;
        self.starved += o.starved;
        self.unread_fibre += o.unread_fibre;
        self.unread_algebra += o.unread_algebra;
    }

    /// The contemplation's line segment.
    pub fn line(&self) -> String {
        format!(
            "pivots reached {} derivable {} demanded {} adjoin {} starved {} unread fibre {} algebra {}",
            self.reached, self.derivable, self.demanded, self.adjoin, self.starved, self.unread_fibre, self.unread_algebra
        )
    }
}

/// WHAT ONE MOVE DOES TO ONE CELL'S ALGEBRA: `Some(true)` already in it,
/// `Some(false)` a new direction, `Err(true)` no action (a fibre move),
/// `Err(false)` the cell's algebra or the move's width did not read.
fn move_in_cell(by: &PivotBy, cell: &CellAlgebra) -> Result<bool, bool> {
    let versor = match by {
        PivotBy::Word { word, .. } => element::versor_of_word(word),
        PivotBy::Placement { translator, .. } => Some(translator.clone()),
        PivotBy::OpenAxis { seed, .. } => Some(seed.clone()),
        PivotBy::Hue { .. } => return Err(true),
    };
    let v = versor.ok_or(false)?;
    let [first, ..] = cell.actions.as_deref().ok_or(false)? else { return Err(false) };
    // The cell's algebra acts on `n + 2` vectors; the move is widened to it.
    let n = u8::try_from(first.len().checked_sub(2).ok_or(false)?).map_err(|_| false)?;
    if v.n() > n {
        return Err(false);
    }
    let action = crate::transducer::cga::action_of(&v.widened(n)).ok_or(false)?;
    algebra_contains(cell, &action).ok_or(false)
}

/// NOTE ONE ORBIT'S OUTCOME against the cells its circuit crosses. Nothing is
/// decided here: the pivot applies exactly as it did.
pub(crate) fn note_outcome(
    cells: &PivotCells,
    chord: u16,
    outcome: &Discharge,
    adjoining: bool,
    tally: &mut PivotTally,
) {
    let Some(crossed) = cells.by_chord.get(&chord.to_be_bytes()) else { return };
    for &k in crossed {
        let Some(cell) = cells.algebras.get(k) else { continue };
        let certs: Vec<&PivotCertificate> = match outcome {
            Discharge::One(c) => vec![c.as_ref()],
            Discharge::Vortex(cs) => cs.iter().collect(),
            Discharge::Starved => {
                tally.starved += 1;
                continue;
            }
            Discharge::NoneDischarged => {
                match adjoining {
                    true => tally.adjoin += 1,
                    false => tally.demanded += 1,
                }
                continue;
            }
        };
        let reads: Vec<Result<bool, bool>> = certs.iter().map(|c| move_in_cell(&c.by, cell)).collect();
        // A Vortex is derivable only where every certificate is.
        match (reads.iter().all(|r| *r == Ok(true)), reads.contains(&Ok(false))) {
            (true, _) => tally.derivable += 1,
            (false, true) => tally.reached += 1,
            (false, false) => match reads.iter().all(|r| *r == Err(true) || *r == Ok(true)) {
                true => tally.unread_fibre += 1,
                false => tally.unread_algebra += 1,
            },
        }
    }
}
