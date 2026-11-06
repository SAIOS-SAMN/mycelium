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
//! **Separated blocks** (ruled 2026-11-06 on #32): a separator is part of
//! the fine period. Blocks of `b` sites, each followed by one separator
//! site, have the fine period `b + 1` and the extent `a · (b + 1) − 1`: the
//! far seam closes the last block, which has no separator after it. The
//! carrier is the same [`Factoring`], its [`Joint`] `Separated`; the
//! separator is the site at the fine phase `b`, and which symbol stands
//! there is deduced ([`super::nesting`]), never declared.
//!
//! Every factoring of an extent is held together as a vortex
//! (MU-SENSING §3.8): no candidate is chosen by a scalar or by its place in
//! the list. Which factorings the family is invariant under is read in
//! [`super::nesting`].

use super::{TransductionRefusal, Turn};

/// HOW A FACTORING'S BLOCKS MEET.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Joint {
    /// Block abuts block: the extent is `a · b`, every fine phase a site of
    /// a block.
    Abutting,
    /// A separator site follows every block but the last: the fine period
    /// is `b + 1`, the extent `a · (b + 1) − 1`.
    Separated,
}

/// ONE FACTORING OF A PHASE'S PERIOD: `coarse` blocks of fine period
/// `fine`, meeting by `joint`; both periods past 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Factoring {
    coarse: usize,
    fine: usize,
    joint: Joint,
}

impl Factoring {
    /// THE FACTORING `extent = coarse · fine`, or `Unfactored` where the
    /// product is not the extent or a factor is not past 1 (a factor of 1
    /// nests nothing: the phase stays one phase).
    pub fn new(extent: usize, coarse: usize, fine: usize) -> Result<Factoring, TransductionRefusal> {
        let product = coarse.checked_mul(fine);
        match product == Some(extent) && coarse > 1 && fine > 1 {
            true => Ok(Factoring { coarse, fine, joint: Joint::Abutting }),
            false => Err(TransductionRefusal::Unfactored { extent, coarse, fine }),
        }
    }

    /// THE SEPARATED FACTORING `extent = coarse · fine − 1`: `coarse` blocks
    /// of `fine − 1` sites, each but the last followed by its separator.
    /// `Unfactored` where the extent is not that, or a period is not past 1.
    pub fn separated(extent: usize, coarse: usize, fine: usize) -> Result<Factoring, TransductionRefusal> {
        let product = coarse.checked_mul(fine).and_then(|p| p.checked_sub(1));
        match product == Some(extent) && coarse > 1 && fine > 1 {
            true => Ok(Factoring { coarse, fine, joint: Joint::Separated }),
            false => Err(TransductionRefusal::Unfactored { extent, coarse, fine }),
        }
    }

    /// The coarse phase's period `a`.
    pub fn coarse(&self) -> usize {
        self.coarse
    }

    /// The fine phase's period: `b`, or `b + 1` where separated.
    pub fn fine(&self) -> usize {
        self.fine
    }

    /// How the blocks meet.
    pub fn joint(&self) -> Joint {
        self.joint
    }

    /// The sites of one block along the phase: the fine period, less the
    /// separator where separated.
    pub fn block(&self) -> usize {
        match self.joint {
            Joint::Abutting => self.fine,
            Joint::Separated => self.fine - 1,
        }
    }

    /// The period of the phase taken apart: `a · b`, or `a · (b + 1) − 1`.
    /// The constructor read the product without overflow.
    pub fn extent(&self) -> usize {
        match self.joint {
            Joint::Abutting => self.coarse * self.fine,
            Joint::Separated => self.coarse * self.fine - 1,
        }
    }

    /// THE SITE `s` ON THE TWO NESTED PHASES: `(⌊s / f⌋ / a, (s mod f) / f)`
    /// for the fine period `f`, each a turn of its own period. `None` where
    /// `s` is not a site of the phase.
    pub fn nested(&self, s: usize) -> Option<(Turn, Turn)> {
        (s < self.extent()).then(|| (Turn::of(s / self.fine, self.coarse), Turn::of(s % self.fine, self.fine)))
    }

    /// THE SITE'S TURN ON THE ONE PHASE, read back from the two nested
    /// turns: `s = i · f + j` for the coarse turn `i / a` and the fine turn
    /// `j / f`, as the turn `s / e`. `None` where a turn does not land on a
    /// site of its phase (a seam's far side, a turn between sites, or the
    /// separator the far seam closes).
    pub fn joined(&self, coarse: Turn, fine: Turn) -> Option<Turn> {
        let i = coarse.lands(self.coarse).ok().filter(|&i| i < self.coarse)?;
        let j = fine.lands(self.fine).ok().filter(|&j| j < self.fine)?;
        let s = i * self.fine + j;
        (s < self.extent()).then(|| Turn::of(s, self.extent()))
    }

    /// THE SITE OF THE PHASE at block `i`, fine coordinate `j` within the
    /// block, or `None` past either.
    pub fn site(&self, i: usize, j: usize) -> Option<usize> {
        (i < self.coarse && j < self.block()).then(|| i * self.fine + j)
    }
}

/// THE FACTORINGS OF ONE EXTENT, HELD TOGETHER: every `a · b = e` with
/// neither factor 1, in order of the coarse period, then every separated
/// `a · (b + 1) − 1 = e`, in order of the coarse period. A vortex
/// (MU-SENSING §3.8): the carrier holds every candidate and offers no
/// choice among them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Factorings {
    extent: usize,
    candidates: Vec<Factoring>,
}

impl Factorings {
    /// Every abutting factoring of `extent`. A prime extent, or 1, holds
    /// none, and the phase stays one phase.
    pub fn of(extent: usize) -> Factorings {
        let candidates =
            (1..=extent).filter(|&a| extent % a == 0).filter_map(|a| Factoring::new(extent, a, extent / a).ok()).collect();
        Factorings { extent, candidates }
    }

    /// EVERY FACTORING OF `extent`, abutting and separated: the
    /// candidates a family's nesting is read over.
    pub fn of_axis(extent: usize) -> Factorings {
        let mut held = Factorings::of(extent);
        let Some(past) = extent.checked_add(1) else { return held };
        held.candidates.extend(
            (2..=past).filter(|&a| past % a == 0).filter_map(|a| Factoring::separated(extent, a, past / a).ok()),
        );
        held
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
