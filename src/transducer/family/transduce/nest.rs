// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! NESTING — AN AXIS TAKEN APART (MU-SENSING §3.6; ROADMAP track 26, S4).
//! A phase of period `e = a · b` may be read as two nested phases: a coarse
//! phase of period `a` and a fine phase of period `b`, site `s` going to
//! the turns `(⌊s / b⌋ / a, (s mod b) / b)`, exact. The turn of the site on
//! the one phase is read back as `coarse + fine / a`, so nothing is lost:
//! the factoring is a coordinate, and speech projects back to the source's
//! own extent exactly.
//!
//! Every factoring of an extent is held together as a vortex
//! (MU-SENSING §3.8): no candidate is chosen by a scalar or by its place in
//! the list. Which factorings the family is invariant under (symbols that
//! separate the blocks, blocks that repeat, motions that act per level) is
//! the reading that keeps or dissolves each candidate; that reading is not
//! built here, and until it is, no axis is taken apart by the transduction.

use super::{TransductionRefusal, Turn};

/// ONE FACTORING OF A PHASE'S PERIOD: `extent = coarse · fine`, both of
/// them neither 1 nor the whole extent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Factoring {
    coarse: usize,
    fine: usize,
}

impl Factoring {
    /// THE FACTORING `extent = coarse · fine`, or `Unfactored` where the
    /// product is not the extent or a factor is not past 1 (a factor of 1
    /// nests nothing: the phase stays one phase).
    pub fn new(extent: usize, coarse: usize, fine: usize) -> Result<Factoring, TransductionRefusal> {
        let product = coarse.checked_mul(fine);
        match product == Some(extent) && coarse > 1 && fine > 1 {
            true => Ok(Factoring { coarse, fine }),
            false => Err(TransductionRefusal::Unfactored { extent, coarse, fine }),
        }
    }

    /// The coarse phase's period `a`.
    pub fn coarse(&self) -> usize {
        self.coarse
    }

    /// The fine phase's period `b`.
    pub fn fine(&self) -> usize {
        self.fine
    }

    /// The period of the phase taken apart, `a · b`. The constructor read
    /// the product without overflow.
    pub fn extent(&self) -> usize {
        self.coarse * self.fine
    }

    /// THE SITE `s` ON THE TWO NESTED PHASES: `(⌊s / b⌋ / a, (s mod b) / b)`,
    /// each a turn of its own period. `None` where `s` is not a site of the
    /// phase.
    pub fn nested(&self, s: usize) -> Option<(Turn, Turn)> {
        (s < self.extent()).then(|| (Turn::new(s / self.fine, self.coarse), Turn::new(s % self.fine, self.fine)))
    }

    /// THE SITE'S TURN ON THE ONE PHASE, read back from the two nested
    /// turns: `s = i · b + j` for the coarse turn `i / a` and the fine turn
    /// `j / b`, as the turn `s / (a · b)`. `None` where a turn does not land
    /// on a site of its phase (a seam's far side, or a turn between sites).
    pub fn joined(&self, coarse: Turn, fine: Turn) -> Option<Turn> {
        let i = coarse.lands(self.coarse).ok().filter(|&i| i < self.coarse)?;
        let j = fine.lands(self.fine).ok().filter(|&j| j < self.fine)?;
        Some(Turn::new(i * self.fine + j, self.extent()))
    }
}

/// THE FACTORINGS OF ONE EXTENT, HELD TOGETHER: every `a · b = e` with
/// neither factor 1, in order of the coarse period. A vortex
/// (MU-SENSING §3.8): the carrier holds every candidate and offers no
/// choice among them. A prime extent, or 1, holds none, and the phase
/// stays one phase.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Factorings {
    extent: usize,
    candidates: Vec<Factoring>,
}

impl Factorings {
    /// Every factoring of `extent`.
    pub fn of(extent: usize) -> Factorings {
        let candidates =
            (1..=extent).filter(|&a| extent % a == 0).filter_map(|a| Factoring::new(extent, a, extent / a).ok()).collect();
        Factorings { extent, candidates }
    }

    /// The extent the candidates factor.
    pub fn extent(&self) -> usize {
        self.extent
    }

    /// The candidates, all held.
    pub fn candidates(&self) -> &[Factoring] {
        &self.candidates
    }
}
