// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A GROUND'S VORTEX COLLAPSES BY A RELATION (MU-SENSING §3.4, §3.8; ruled
//! 2026-11-06 on #32 (3)), as the nesting's does (`nesting`, point 5). Every
//! candidate of a vortex is ambient, enclosing and never carried as a body,
//! so the three conditions cannot tell them apart. Two relations between the
//! bodies and the ground can, each read in perception at any rank with no
//! threshold:
//!
//! 1. **Stands** ([`GroundReading::Stands`]): on every train input, a body
//!    stands on the candidate: the candidate's one body covers a whole seam
//!    and a body lies inside it (a level read past its membrane,
//!    `membrane::levels`).
//! 2. **Painted on** ([`GroundReading::PaintedOn`]): every pair's residue
//!    changes only the candidate's sites: at every site where a held
//!    factorisation's speech and the output differ, the symbol before or
//!    after the change is the candidate. A body appearing on the ground, or
//!    lifted off it, changes the ground's sites; nothing else is read. A
//!    pair whose factorisations hold no motion reads nothing here.
//!
//! **The collapse.** A candidate collapses the vortex where one of the two
//! holds for it on every pair and for no other kept candidate. Where both
//! hold alike, or neither holds, or two candidates each hold one, the
//! candidates stay superposed ([`Collapse::Superposed`]): none is chosen by
//! a scalar, an order or a count.
//!
//! **A closing rule never collapses the ground.** Whether the family's rule
//! closes under a candidate decides perception by the solve, which inverts
//! description and solution; it may be kept as an observation per
//! candidate, and is never read here.

use super::{factorisations, Factored};
use crate::transducer::sample::Sample;

/// A RELATION THAT HOLDS A GROUND CANDIDATE (points 1 and 2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GroundReading {
    /// On every train input, a body stands on the candidate.
    Stands,
    /// Every pair's residue changes only the candidate's sites.
    PaintedOn,
}

/// ONE CANDIDATE READ: the relations that hold for it on every pair, in
/// reading order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CandidateReading {
    pub symbol: Vec<u8>,
    pub readings: Vec<GroundReading>,
}

/// WHETHER A VORTEX COLLAPSES.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Collapse {
    /// The candidate `candidate` (its index in the vortex's order) collapsed
    /// it by the reading `by`, which holds for no other candidate.
    Collapsed { candidate: usize, by: GroundReading },
    /// No candidate collapses it: the candidates stay superposed.
    Superposed,
}

/// THE CANDIDATES OF A VORTEX READ BY THE TWO RELATIONS, in the vortex's
/// order, and whether one collapses it. A family with no pair reads no
/// relation, and its candidates stay superposed.
pub fn collapse_of(family: &[(Sample, Sample)], candidates: &[Vec<u8>]) -> (Vec<CandidateReading>, Collapse) {
    let stands: Vec<Vec<Vec<u8>>> = family.iter().map(|(x, _)| standing_grounds(x)).collect();
    let residues: Vec<Option<Vec<(Vec<u8>, Vec<u8>)>>> = family.iter().map(|(x, y)| residue_changes(x, y)).collect();
    let read: Vec<CandidateReading> = candidates
        .iter()
        .map(|g| {
            let mut readings = Vec::new();
            if !family.is_empty() && stands.iter().all(|on| on.contains(g)) {
                readings.push(GroundReading::Stands);
            }
            let painted = |changes: &Option<Vec<(Vec<u8>, Vec<u8>)>>| {
                changes.as_ref().is_some_and(|c| c.iter().all(|(before, after)| before == g || after == g))
            };
            if !family.is_empty() && residues.iter().all(painted) {
                readings.push(GroundReading::PaintedOn);
            }
            CandidateReading { symbol: g.clone(), readings }
        })
        .collect();
    let collapse = collapse_by(&read);
    (read, collapse)
}

/// The one candidate a reading holds for and no other, or none.
fn collapse_by(read: &[CandidateReading]) -> Collapse {
    let mut collapsing = read.iter().enumerate().filter_map(|(i, own)| {
        let by = own.readings.iter().copied().find(|r| read.iter().enumerate().all(|(j, other)| j == i || !other.readings.contains(r)))?;
        Some((i, by))
    });
    match (collapsing.next(), collapsing.next()) {
        (Some((candidate, by)), None) => Collapse::Collapsed { candidate, by },
        _ => Collapse::Superposed,
    }
}

/// THE SYMBOLS A BODY STANDS ON IN `x`: each membrane of its levels with a
/// level read inside it.
fn standing_grounds(x: &Sample) -> Vec<Vec<u8>> {
    let levels = super::membrane::levels(x);
    levels
        .iter()
        .enumerate()
        .filter(|(d, _)| levels.len() > d + 1)
        .filter_map(|(_, l)| l.membrane.clone())
        .collect()
}

/// THE CHANGES EVERY HELD FACTORISATION OF THE PAIR LEAVES: each residue
/// link's symbol before and after, over every factorisation held (none
/// chosen among them). `None` where the pair's factorisations do not read
/// or hold no motion: such a pair holds no residue to read.
fn residue_changes(x: &Sample, y: &Sample) -> Option<Vec<(Vec<u8>, Vec<u8>)>> {
    // One stroke per signed permutation of the phases: each motion's own
    // reading draws a stroke of its own inside `factorisations`.
    let need = super::factorisations_need(x.rank(), 1)?;
    let factored = factorisations(x, y, &mut crate::physics::Physics::default().stroke_n(need)).ok()?;
    if let Factored::None(_) = factored {
        return None;
    }
    Some(factored.held().iter().flat_map(|f| f.residue.iter().map(|l| (l.before.clone(), l.after.clone()))).collect())
}
