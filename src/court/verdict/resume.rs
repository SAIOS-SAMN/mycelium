// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 78.153 — A REPLAY ON THE STROKE: what a court's replay answers when its
//! one stroke runs out, and where the next stroke takes it up.
//!
//! The lock audit (MU-RECURSION 78.153) found the replay unmetered — only
//! `versor_between` spent — and found that the one place that did spend
//! slashed on it: `NoPose::Unbounded` became `REFUSE_EDGE`. A spent stroke
//! is not a refusal. Every input-sized loop of the replay now spends one
//! read per item, and where the stroke runs out the answer is
//! [`Replayed::Unread`]: not ruled this stroke, never a pass, never a slash.
//! The [`ReplayCursor`] keeps the phase and the item it stopped at, with the
//! exact partials a later phase is built from, so the next stroke resumes.

use super::*;

/// WHAT A REPLAY ON ONE STROKE CAME TO. `#[must_use]`: a caller that drops
/// it has neither ruled nor deferred the word.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Replayed {
    /// Every phase re-derived.
    Clean,
    /// Refused, with its reason, coordinate and kind (34.1, 50.9, 78.9).
    Refused(Refusal),
    /// The stroke was spent before the replay ruled. The cursor holds where
    /// it stopped; nothing is ruled, refused or slashed on it.
    Unread,
}

impl Replayed {
    /// The refusal, where the replay refused.
    pub fn refusal(&self) -> Option<&Refusal> {
        match self {
            Replayed::Refused(r) => Some(r),
            Replayed::Clean | Replayed::Unread => None,
        }
    }
}

/// THE PHASES OF A FORM'S REPLAY, in the order `replay_form_on_resume`
/// reads them. The order is the verdict's: a phase is entered only when
/// every one before it passed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase {
    /// Every Glued vertex bonded, its sheets distinct (54.7 B5a).
    #[default]
    Ports,
    /// No cover edge without a port.
    Phantoms,
    /// Every cited form resolves, closed, at an ungauged frame.
    Citations,
    /// Every cite composes to the word its vertex carries (50.2 step 2).
    Derivations,
    /// Every prime a demand names carried by its witness (37.7 P2).
    Witnesses,
    /// Every vertex's tape and moves (23.1 steps 3 and 6).
    Moves,
    /// Every pair of the family perceived, once per replay (law 12).
    Perceive,
    /// Every clause stated on every pair (78.143).
    Clauses,
    /// Every word vertex carries its body on every frame it claims (50.2 G).
    Bodies,
    /// Every face re-derives, or is a certified chord (23.1 step 4).
    Faces,
    /// Every carried edge re-measured on the family (50.3).
    Edges,
    /// Every orbit coheres (54.5).
    Coherence,
    /// The receipts' periods are the terrain's (54.5).
    Terrain,
    /// The rank reads.
    Rank,
    /// Every pair spoken, the remainder what the demands declare (S9.4).
    Deflation,
    /// No pivoted factor without its certificate (AT1).
    Pivots,
    /// Every certificate re-read (A20).
    Certificates,
    /// Past the last phase: every phase passed.
    Ruled,
}

impl Phase {
    fn next(self) -> Phase {
        match self {
            Phase::Ports => Phase::Phantoms,
            Phase::Phantoms => Phase::Citations,
            Phase::Citations => Phase::Derivations,
            Phase::Derivations => Phase::Witnesses,
            Phase::Witnesses => Phase::Moves,
            Phase::Moves => Phase::Perceive,
            Phase::Perceive => Phase::Clauses,
            Phase::Clauses => Phase::Bodies,
            Phase::Bodies => Phase::Faces,
            Phase::Faces => Phase::Edges,
            Phase::Edges => Phase::Coherence,
            Phase::Coherence => Phase::Terrain,
            Phase::Terrain => Phase::Rank,
            Phase::Rank => Phase::Deflation,
            Phase::Deflation => Phase::Pivots,
            Phase::Pivots => Phase::Certificates,
            Phase::Certificates | Phase::Ruled => Phase::Ruled,
        }
    }
}

/// WHERE A REPLAY STOPPED (78.153, option (A) as ruled): the phase,
/// the item inside it (which glued vertex, which edge, which pair, which
/// certificate — the item's position in the form's own order), and the
/// exact partials a later phase is built from — the pairs as the sensor read
/// them, the undeclared links of the pairs already spoken, and the terrain's
/// class rows already derived.
///
/// VOLATILE. Held on the docket in memory; a restart starts the replay over.
/// It holds no copy of the word and no cited form: those are content and
/// are read again, by address, at the next stroke.
#[derive(Clone, Debug, Default)]
pub struct ReplayCursor {
    pub(crate) phase: Phase,
    pub(crate) at: usize,
    pub(crate) frames: Vec<super::body::Perceived>,
    pub(crate) undeclared: Vec<sentence::Link>,
    pub(crate) derived: Vec<(crate::network::nerve::coord::Holonomy, sentence::ToralPeriod)>,
}

impl ReplayCursor {
    /// HOW FAR THE REPLAY HAS COME: the phase and the item, and the pairs
    /// perceived. Two cursors at one progress hold the same partials, since
    /// every partial is a function of the items read; a stroke that leaves
    /// the progress where it found it moved the replay nowhere.
    pub fn progress(&self) -> (Phase, usize, usize) {
        (self.phase, self.at, self.frames.len())
    }

    /// The phase the replay stands in.
    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// The item of that phase it resumes at.
    pub fn at(&self) -> usize {
        self.at
    }

    /// Where a phase is entered: `None` where the cursor is past it, the
    /// item it stopped at where it stopped inside it, the first otherwise.
    pub(crate) fn enter(&self, phase: Phase) -> Option<usize> {
        match self.phase.cmp(&phase) {
            std::cmp::Ordering::Greater => None,
            std::cmp::Ordering::Equal => Some(self.at),
            std::cmp::Ordering::Less => Some(0),
        }
    }

    /// A phase settled: passed (the cursor moves past it), refused, or spent
    /// (the cursor keeps the item and the replay is unread).
    pub(crate) fn settle(&mut self, phase: Phase, step: Step) -> Option<Replayed> {
        match step {
            Ok(()) => {
                self.phase = phase.next();
                self.at = 0;
                None
            }
            Err(Halt::Refused(r)) => Some(Replayed::Refused(r)),
            Err(Halt::Spent(at)) => {
                self.phase = phase;
                self.at = at;
                Some(Replayed::Unread)
            }
        }
    }

    /// Unread between two phases: the cursor stays where it stands.
    pub(crate) fn unread_here(&self) -> Replayed {
        Replayed::Unread
    }
}

/// How a metered phase stops.
#[derive(Debug)]
pub(crate) enum Halt {
    Refused(Refusal),
    /// The stroke ran out at this item.
    Spent(usize),
}

/// One phase's reading.
pub(crate) type Step = Result<(), Halt>;

/// How a reader that answers with a reason alone stops.
#[derive(Debug)]
pub(crate) enum Cut {
    Refused(&'static str),
    Spent(usize),
}

/// The stroke was spent inside a sub-read (`NoPose::Unbounded`).
#[derive(Debug)]
pub(crate) struct Unbounded;

/// WHAT A METERED READER SPENDS: the court's stroke, or nothing — the
/// readers the voice runs off a court's lock (the mint's own body check).
pub(crate) enum Meter<'b> {
    Free,
    Stroke(&'b mut crate::physics::Budget),
}

impl Meter<'_> {
    /// One read; false once the stroke is spent.
    pub(crate) fn spend(&mut self) -> bool {
        match self {
            Meter::Free => true,
            Meter::Stroke(b) => b.spend(),
        }
    }
}
