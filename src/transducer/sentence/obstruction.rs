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
///
/// A.13 — THE PEEL READS BOTH POLES. A holonomy composed of transvections
/// moves the point at infinity, and the peel read its rotor only where it
/// did not: every such versor fell to unclassified. Its rotation is now
/// read off beside its transversion (`conformal_factors`), and the peel
/// spells it or types its demand. A holonomy in the INVERSION'S CELL — it
/// carries the origin to infinity — is a versor no rotor spells, whatever
/// letters are held: D with its coset, the demand for the letter the
/// inversion is.
pub fn read_obstruction(content: Content) -> ClosureObstruction {
    match content {
        Content::Residue(links) => ClosureObstruction::Cover { cocycle: links.to_vec() },
        Content::Holonomy { versor, chart, proof } => {
            let Some(h) = versor.filter(|h| crate::transducer::cga::is_versor(h)) else {
                return ClosureObstruction::Permanent { proof };
            };
            let dialect = || ClosureObstruction::Dialect { coset: ray(h), chart: chart.map(<[u8]>::to_vec) };
            match crate::transducer::family::derivation::peel_versor(h) {
                Some(Err(_)) => dialect(),
                Some(Ok(_)) => ClosureObstruction::Cover { cocycle: Vec::new() },
                None => match crate::transducer::element::conformal_factors(h) {
                    Ok(f) if f.inverted => dialect(),
                    _ => ClosureObstruction::Unclassified,
                },
            }
        }
    }
}

/// A.17 — A REFUSAL NAMED WHERE IT IS MADE (78.16 C3 as audited): the
/// mind's, the pivot's and the mint's refusals that return no `Refusal`,
/// each by its own name, its kind by 78.9 — read off the content where the
/// content decides it, never assigned per site — and counted once by the one
/// refusal reader ([`crate::observe::note_obstruction`]); never a silent
/// `None`, `false` or `continue`. The trial's per-candidate negatives are
/// returned, never counted: one orbit tried by k candidates is one fact
/// (law 12), counted on the demand arm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// The mint: nothing in the form acts — no question stands (N).
    MintNothingActs,
    /// The mint: a vertex does not carry its family, at the kind the court's
    /// own reader gave that refusal.
    MintVertexNotCarried(ObstructionKind),
    /// The mint: links remain and the form declares no demand — its own
    /// claim does not account for what it read (I).
    MintLinksWithoutDemand,
    /// The mint: links the form's demands do not declare (I).
    MintLinksUndeclared,
    /// The mint: the form has no tape at its measure (I).
    MintNoTape,
    /// A derivation: the citations do not compose to the target (I).
    CitationsDoNotCompose,
    /// 37.7, the Gödel step: the rotor mints no generator, at the kind its
    /// content reads — no versor P, a versor the letters do not spell D (the
    /// letter demand). A spelled one is derivable and never this.
    RotorMintsNoGenerator(ObstructionKind),
    /// The pivot's trial: no orbit, class or measure stood to ask (N).
    TrialNoQuestion,
    /// The trial's arms (C, I) are the candidate's own and returned only.
    /// The trial: the move leaves the coordinate it acted on standing (C).
    TrialNotDischarged,
    /// The trial: the member paths agree on no class after the move (C).
    TrialPathsDisagree,
    /// The trial: the rank moved as no elementary pivot moves it (I).
    TrialRankInadmissible,
    /// The trial: a lane rose, on a pair or on the bundle (C, law 10).
    TrialLaneRose,
    /// The trial: the move split two circuits of a cell that agreed (C,
    /// 78.46(4′)).
    TrialUnclosed,
    /// An owed citation: the cited address is sealed, owed to no court (I:
    /// it says nothing about a closure).
    CitedSealed,
    /// An owed citation: the cited form is not held here (I).
    CitedNotHeld,
    /// An owed citation: every court this mind knows was taught it and none
    /// rules it — it waits for its family (I).
    OwedToEveryCourt,
    /// The court: an end of the face is not a vertex of the form (I).
    FaceEndNotVertex,
    /// The court: the face's place does not re-derive (I).
    FacePlaceNotRederived,
    /// The court: no place derives at the face at all — a claim that does
    /// not re-derive (I, 78.9's face row).
    FacePlaceUnderived,
}

impl Refused {
    /// Its kind (78.9).
    pub fn kind(self) -> ObstructionKind {
        use ObstructionKind::{C, I, N};
        match self {
            Refused::MintVertexNotCarried(k) | Refused::RotorMintsNoGenerator(k) => k,
            Refused::MintNothingActs | Refused::TrialNoQuestion => N,
            Refused::TrialNotDischarged
            | Refused::TrialPathsDisagree
            | Refused::TrialLaneRose
            | Refused::TrialUnclosed => C,
            Refused::MintLinksWithoutDemand
            | Refused::MintLinksUndeclared
            | Refused::MintNoTape
            | Refused::CitationsDoNotCompose
            | Refused::TrialRankInadmissible
            | Refused::CitedSealed
            | Refused::CitedNotHeld
            | Refused::OwedToEveryCourt
            | Refused::FaceEndNotVertex
            | Refused::FacePlaceNotRederived
            | Refused::FacePlaceUnderived => I,
        }
    }

    /// Its name, the proof a permanent obstruction carries.
    pub fn proof(self) -> &'static str {
        match self {
            Refused::MintNothingActs => "nothing in the form acts",
            Refused::MintVertexNotCarried(_) => "a vertex does not carry its family",
            Refused::MintLinksWithoutDemand => "links remain and the form declares no demand",
            Refused::MintLinksUndeclared => "links undeclared by the form's demands",
            Refused::MintNoTape => "the form has no tape at its measure",
            Refused::CitationsDoNotCompose => "the citations do not compose to the target",
            Refused::RotorMintsNoGenerator(_) => "the rotor mints no generator",
            Refused::TrialNoQuestion => "no orbit, class or measure stood to ask",
            Refused::TrialNotDischarged => "the move leaves its coordinate standing",
            Refused::TrialPathsDisagree => "the member paths agree on no class",
            Refused::TrialRankInadmissible => "the rank moved as no elementary pivot moves it",
            Refused::TrialLaneRose => "a lane rose",
            Refused::TrialUnclosed => "the move splits two circuits of a cell that agreed",
            Refused::CitedSealed => "the cited address is sealed, owed to no court",
            Refused::CitedNotHeld => "the cited form is not held here",
            Refused::OwedToEveryCourt => "owed to every court and none rules it",
            Refused::FaceEndNotVertex => "an end is not a vertex of the form",
            Refused::FacePlaceNotRederived => "the place does not re-derive",
            Refused::FacePlaceUnderived => "the place does not derive at all",
        }
    }

    /// The obstruction it is, for a refusal that carries one (78.9).
    pub fn obstruction(self) -> ClosureObstruction {
        match self.kind() {
            ObstructionKind::P => ClosureObstruction::Permanent { proof: self.proof() },
            ObstructionKind::D => ClosureObstruction::Dialect { coset: Vec::new(), chart: None },
            ObstructionKind::C => ClosureObstruction::Cover { cocycle: Vec::new() },
            ObstructionKind::I => ClosureObstruction::Integrity,
            ObstructionKind::N => ClosureObstruction::NoQuestion,
            ObstructionKind::Unclassified => ClosureObstruction::Unclassified,
        }
    }

    /// Counted by the one refusal reader, and returned.
    pub fn noted(self) -> Refused {
        crate::observe::note_obstruction(self.kind());
        self
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
