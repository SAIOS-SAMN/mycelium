// 74, 78.9, CV1 — THE KIND OF A REFUSAL, READ OFF ITS CONTENT: whether a
// versor exists, whether the dialect spells it, whether the sections glue.
//
//! 74 defines the three kinds by the CONTENT of the obstruction, so the kind
//! of a closure refusal is read off what the refusing reader carried by ONE
//! reader, [`read_obstruction`], never assigned per code (law 12, rule 2):
//!
//! - **P, permanent** — no versor relates what the closure asks; the proof is
//!   the refusing reader's own name for why.
//! - **D, dialect-relative** — a versor exists and the dialect does not spell
//!   it: its coset (the versor's canonical ray) and the chart it acts on,
//!   offered to a mind whose charts do.
//! - **C, cover-relative** — the sections do not glue: the residue on the
//!   cover, its cocycle, which closes on another mind's cover.
//!
//! Beside them (78.9): **I, integrity** — a refusal that says nothing about
//! a closure (does not parse, a claim that does not re-derive, the chain's
//! own integrity), never offered to another perspective; **N, no question**
//! — no circuit stood to be asked. `Unclassified` is a refusal the reader
//! cannot place: a defect count, never a home.

use super::Link;
use crate::transducer::element::Versor;

/// THE KIND OF AN OBSTRUCTION (74, 78.9).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObstructionKind {
    P,
    D,
    C,
    I,
    N,
    Unclassified,
}

impl ObstructionKind {
    pub const ALL: [ObstructionKind; 6] = [
        ObstructionKind::P,
        ObstructionKind::D,
        ObstructionKind::C,
        ObstructionKind::I,
        ObstructionKind::N,
        ObstructionKind::Unclassified,
    ];

    /// The kind by its letter, as the lines print it.
    pub fn label(self) -> &'static str {
        match self {
            ObstructionKind::P => "P",
            ObstructionKind::D => "D",
            ObstructionKind::C => "C",
            ObstructionKind::I => "I",
            ObstructionKind::N => "N",
            ObstructionKind::Unclassified => "unclassified",
        }
    }

    /// Its place in [`ObstructionKind::ALL`], the counters' axis.
    pub fn index(self) -> usize {
        match self {
            ObstructionKind::P => 0,
            ObstructionKind::D => 1,
            ObstructionKind::C => 2,
            ObstructionKind::I => 3,
            ObstructionKind::N => 4,
            ObstructionKind::Unclassified => 5,
        }
    }
}

/// AN OBSTRUCTION WITH ITS CONTENT (78.9): P with its proof, D with its coset
/// and chart, C with its cocycle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClosureObstruction {
    Permanent { proof: &'static str },
    /// The coset is the versor's canonical ray, each non-zero blade with its
    /// coefficient; the chart is the key of the chart it acts on, where the
    /// refusal names one. A D held by the court itself — a cited word not yet
    /// ruled (CV2) — carries no coset: its content is the cited address,
    /// which rides the court's docket.
    Dialect { coset: Vec<(u16, num_rational::BigRational)>, chart: Option<Vec<u8>> },
    Cover { cocycle: Vec<Link> },
    Integrity,
    NoQuestion,
    Unclassified,
}

impl ClosureObstruction {
    pub fn kind(&self) -> ObstructionKind {
        match self {
            ClosureObstruction::Permanent { .. } => ObstructionKind::P,
            ClosureObstruction::Dialect { .. } => ObstructionKind::D,
            ClosureObstruction::Cover { .. } => ObstructionKind::C,
            ClosureObstruction::Integrity => ObstructionKind::I,
            ClosureObstruction::NoQuestion => ObstructionKind::N,
            ClosureObstruction::Unclassified => ObstructionKind::Unclassified,
        }
    }
}

/// WHAT A REFUSING READER CARRIED, as the one reader takes it.
pub enum Content<'a> {
    /// The holonomy the closure asked for — `None` where no versor lifted,
    /// with the reader's name for why — and the chart it acts on.
    Holonomy { versor: Option<&'a Versor>, chart: Option<&'a [u8]>, proof: &'static str },
    /// A residue on the cover: the links the sections leave.
    Residue(&'a [Link]),
}

/// THE ONE READER OF A CLOSURE REFUSAL'S KIND (78.9): no versor, or one with
/// no inverse, is P; a versor the dialect's peel does not spell is D with its
/// coset and chart; a versor it spells whose closure still refused, or a
/// residue carried, is C with the cocycle; a versor the peel reads nothing
/// from is unclassified, counted as the defect it is.
pub fn read_obstruction(content: Content) -> ClosureObstruction {
    match content {
        Content::Residue(links) => ClosureObstruction::Cover { cocycle: links.to_vec() },
        Content::Holonomy { versor, chart, proof } => {
            let Some(h) = versor.filter(|h| crate::transducer::cga::is_versor(h)) else {
                return ClosureObstruction::Permanent { proof };
            };
            match crate::transducer::family::derivation::peel_versor(h) {
                Some(Err(_)) => ClosureObstruction::Dialect { coset: ray(h), chart: chart.map(<[u8]>::to_vec) },
                Some(Ok(_)) => ClosureObstruction::Cover { cocycle: Vec::new() },
                None => ClosureObstruction::Unclassified,
            }
        }
    }
}

/// A versor's canonical ray, each non-zero blade and its coefficient.
#[expect(
    clippy::cast_possible_truncation,
    reason = "STD1 C1 — the algebra's range: a blade mask below the 2^(n+2) slots of Cl(n+1,1), c.slot_count()"
)]
fn ray(h: &Versor) -> Vec<(u16, num_rational::BigRational)> {
    match h.canonical() {
        Some(c) => (0..c.slot_count())
            .filter(|&b| !num_traits::Zero::is_zero(c.coeff(b)))
            .map(|b| (b as u16, c.coeff(b).clone()))
            .collect(),
        None => Vec::new(),
    }
}
