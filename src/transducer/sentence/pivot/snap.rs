// A16, A20 — WHAT A PIVOT PRODUCES: the snapshot of one pivot, the
// discharge it certifies, and the axis it opened.
//
//! SEVERAL CERTIFIED DISCHARGES ARE A VORTEX: closure certifies a pivot
//! after it is made, and no reading of μ⃗ chooses among the candidates that
//! discharged. A vortex records the OBSTRUCTION between its candidates.

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Snap {
    /// A chord the tree gauge closed: its class was trivial (T2), never rank.
    Closed { chord: usize, order: Order },
    /// A held orbit discharged on one occupied axis by a certified pivot.
    Discharged { chord: usize, certificate: Box<PivotCertificate> },
    /// SEVERAL pivots discharge: all are held as candidates of the thought
    /// until a perception, a glue or a ruling collapses the superposition
    /// (12.6).
    /// `between` is the obstruction of each consecutive pair — the reading
    /// that says they are two proofs, recorded and never spent on a choice.
    Vortex { chord: usize, held: Vec<PivotCertificate>, between: Vec<Obstruction> },
    /// Several discharges with no bundle to measure μ⃗, so the obstruction
    /// has no lane reading in it — a third state, not an empty one.
    Unmeasured { chord: usize, held: Vec<PivotCertificate> },
    TorsionFiled { chord: usize, ell: u64 },
    TorsionOpen { chord: usize },
    /// A held orbit of infinite order the basis did not discharge: its
    /// canonical holonomy rotor, carried out as the witness of the standing
    /// demand the form speaks on each chart the orbit stands on (16.16 step
    /// 6, 37.7 at the rotor level).
    Demand {
        chord: usize,
        key: crate::transducer::address::Address,
        rotor: Box<crate::transducer::alphabet::Rotor>,
        chart: crate::transducer::stalk::Invariant,
    },
    /// CS2's DEMAND ARM, 78.37 — an orbit no held word discharged and none
    /// adjoins: its undischarged class vector (whose own plane section names
    /// the mode, 3.9) and the charts it stands on, its cover. Carried to the
    /// contemplation's docket and cited on the orbit's close. 78.47 — with its
    /// RESIDUE: the orbit's whole transport, the one coordinate the demand
    /// carries (`None` where it did not read), cited on the chain by its ray.
    /// Its constructors are a forced function of it and are never carried
    /// beside it: whoever asks derives them (`cga::construct`, law 12).
    Demanded {
        chord: usize,
        class: Box<crate::transducer::sentence::ClassVector>,
        charts: Vec<crate::transducer::stalk::Invariant>,
        residue: Option<crate::transducer::cga::Mv>,
    },
    /// 37.7 FROM AN ORBIT'S OWN RESIDUE: the basis reached no coordinate this
    /// orbit occupies, the class was published and nothing answered, so the
    /// rotor the orbit measured is carried out to be ADJOINED as a generator.
    /// The one move by which a mind constructs the letter it could not be
    /// given, rather than asking again for it.
    ///
    /// It carries no lane measure. Adjoining widens the language; it
    /// discharges nothing on this form and its μ⃗ is not a deflation.
    Adjoin {
        chord: usize,
        rotor: Box<crate::transducer::alphabet::Rotor>,
        chart: crate::transducer::stalk::Invariant,
    },
}

impl Snap {
    /// Lane measure of an A16 snap. Discharged and Vortex read the
    /// certificate μ⃗ (before − after); TorsionFiled has no bundle μ⃗. `None`
    /// where a vortex's sum would pass its carrier (STD1).
    pub fn lanes(&self) -> Option<[u64; 4]> {
        match self {
            Snap::Discharged { certificate, .. } => Some(mu_deflation(&certificate.mu)),
            Snap::Vortex { held, .. } => crate::core::lanes::lanes_sum(held.iter().map(|c| mu_deflation(&c.mu))),
            // `Adjoin` is here for the reason `Demand` is: it widens the
            // language and discharges nothing on this form, so it deflates
            // no lane and its μ⃗ is not a reading to be summed.
            Snap::TorsionFiled { .. }
            | Snap::Unmeasured { .. }
            | Snap::Closed { .. }
            | Snap::TorsionOpen { .. }
            | Snap::Demand { .. }
            | Snap::Demanded { .. }
            | Snap::Adjoin { .. } => Some([0; 4]),
        }
    }
}

pub(crate) fn mu_deflation(mu: &crate::transducer::sentence::form::MeasuredMu) -> [u64; 4] {
    match mu {
        // The DEFLATION a move measured: what each lane fell by, and
        // nothing on a lane that did not fall. A lane that ROSE is not a
        // deflation of zero — `lanes_risen` is where a rise is read — so
        // the empty difference is the reading here.
        crate::transducer::sentence::form::MeasuredMu::Measured { before, after } => {
            crate::economy::shortfall(before, after)
        }
        crate::transducer::sentence::form::MeasuredMu::Unmeasured => [0; 4],
    }
}

/// WHAT SEPARATES TWO CERTIFIED DISCHARGES of one orbit. A record, spent on
/// nothing: it is read when a collapse asks what the collapse resolved, and
/// it is never consulted to prefer a candidate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Obstruction {
    /// The axes the two pivots acted on. Two axes out of one orbit are two
    /// directions, which is the whole reason both are kept.
    pub axes: (Axis, Axis),
    /// The lane reading of the second's μ⃗ against the first's, per lane.
    /// `None` when neither carries a measured μ⃗ — no bundle, so no reading,
    /// which is a third state and not four `Held`s.
    pub lanes: Option<[crate::economy::LaneMove; 4]>,
}

/// EXPANSION 4 — THE SYMMETRY TWO DISCHARGES PROVE.
///
/// When `a` and `b` both discharge one orbit to one class, the orbit admits a
/// symmetry and the vortex is the mind holding two hypotheses at once. The
/// symmetry is not a tie to be broken: it is `w_a⁻¹ · w_b`, the element that
/// carries one discharge to the other, and it STABILISES the orbit's class
/// because both sides leave that class where the other does.
///
/// That element is exactly 37.7's object — a rotation the mind measured,
/// which the presentation may or may not spell — so discarding the losing
/// candidate throws away a candidate GENERATOR. `collapse_vortex` cleared
/// the whole vortex on collapse; this is what it was clearing.
///
/// `None` where the two are not both word moves, or where the symmetry is
/// the identity. An identity symmetry is two spellings of one move rather
/// than two moves, and the orbit admits nothing extra — which is a reading
/// and not a failure.
///
/// The precondition is checked and not assumed: the two must discharge to
/// the SAME class after, or `w_a⁻¹·w_b` fixes nothing and is not a symmetry
/// of anything (rule 11).
pub fn symmetry(a: &PivotCertificate, b: &PivotCertificate) -> Option<crate::transducer::alphabet::Rotor> {
    if a.after != b.after {
        return None;
    }
    let (PivotBy::Word { word: wa, .. }, PivotBy::Word { word: wb, .. }) = (&a.by, &b.by) else {
        return None;
    };
    let s = wa.inverse().compose(wb).normal_form();
    (!s.is_identity()).then(|| s.rotor())
}

/// Every symmetry a vortex holds, against the certificate that won it. One
/// per losing candidate: each is a separate degree of freedom the orbit
/// admits, and summing them would be a count deciding (rule 2).
pub fn symmetries(win: &PivotCertificate, held: &[PivotCertificate]) -> Vec<crate::transducer::alphabet::Rotor> {
    held.iter().filter_map(|c| symmetry(win, c)).collect()
}

/// The obstruction between two certified discharges: what says they are two
/// proofs of one orbit (23.9) rather than one proof twice.
pub(crate) fn obstruction(a: &PivotCertificate, b: &PivotCertificate) -> Obstruction {
    let lanes = match (&a.mu, &b.mu) {
        (MeasuredMu::Measured { after: x, .. }, MeasuredMu::Measured { after: y, .. }) => {
            Some(crate::economy::lane_moves(y, x))
        }
        _ => None,
    };
    Obstruction { axes: (a.axis, b.axis), lanes }
}

/// The outcomes of an orbit's trials. `One` is the single discharge;
/// `Vortex` is SEVERAL, held together with nothing chosen between them;
/// `NoneDischarged` is the algebra's answer that no candidate reaches the
/// coordinate; and `Starved` is not an answer at all.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Discharge {
    One(Box<PivotCertificate>),
    Vortex(Vec<PivotCertificate>),
    NoneDischarged,
    /// THE SEARCH WAS CUT SHORT BY THE BUDGET, with candidates standing
    /// untried. This is a reading of the stroke, not of the orbit, and the
    /// two were one value: `NoneDischarged` was returned whether every
    /// candidate had been tried and refused or none had been tried at
    /// all, so the loop below it spoke a demand, filed torsion, or widened
    /// its cover on an evaluation that never happened.
    ///
    /// Measured on ca485cc: 29 of 984 candidate readings report `starved
    /// true`. Rare, and a false demand is a false demand — the mesh is
    /// asked for a word that this mind may already hold and simply did not
    /// reach. The orbit is held instead and the next stroke reaches it.
    ///
    /// Not to be confused with the eliminated candidates in the same line:
    /// `unreached` counts those too, and they are refused by exact
    /// rational arithmetic rather than by exhaustion. On the same deployment
    /// every `unreached` equals its `eliminated`, so that gap is the
    /// eliminator working and not this.
    Starved,
}

/// THE OUTCOME OF AN ORBIT'S TRIALS, including the one that is not a reading
/// of the algebra.
///
/// A discharge that HAPPENED is real however the stroke ended, so a
/// non-empty set answers as it always did. Only the empty set is
/// ambiguous, and the budget is what separates its two meanings.
pub(crate) fn outcome_of(moves: &[PivotCertificate], starved: bool) -> Discharge {
    match (moves.is_empty(), starved) {
        (true, true) => Discharge::Starved,
        _ => select_discharge(moves),
    }
}

/// A remainder link's kind, read from its two hues. Hue 0 is void.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkKind {
    Created,
    Erased,
    Recoloured,
    Unchanged,
}

impl LinkKind {
    pub fn of(before: u8, after: u8) -> LinkKind {
        match (before == 0, after == 0) {
            (true, true) => LinkKind::Unchanged,
            (true, false) => LinkKind::Created,
            (false, true) => LinkKind::Erased,
            (false, false) if before == after => LinkKind::Unchanged,
            (false, false) => LinkKind::Recoloured,
        }
    }
}

/// One candidate pivot: a held word that occupies the axis, written on the
/// chord on one side. Two candidates whose face words are one vertex are one.
pub(crate) struct Candidate {
    pub(crate) axis: Axis,
    pub(crate) by: PivotBy,
}

/// HOLD A VORTEX CANDIDATE BY THE CLASS THE ORBIT STANDS IN, then by the
/// discharging word's move key.
///
/// `before` is `kappa.class_key()`, the orbit's state vector as it stands,
/// and `ClassVector::class_key` writes the rotor's conjugacy invariants
/// first — so the class of the holonomy is this key's depth-one stratum.
/// The eliminator in the drive proves a candidate's rotor is conjugate to
/// that holonomy, so a deciding word's own class key IS the stratum this
/// candidate hangs under, and the read is a projection rather than a
/// guess.
pub fn vortex_put(v: &mut Keyed<Keyed<PivotCertificate>>, c: PivotCertificate) {
    let before = c.before.clone();
    let k = c.by.move_key();
    v.nested_insert(before, k, c);
}

/// ONE CERTIFICATE PER MOVE. `candidates_for` offers a held word on both
/// sides of the chord, and where both sides discharge the orbit the trial
/// issues two certificates that name ONE move: a word move is keyed by its
/// rotor (`PivotBy::move_key`), `vortex_put` holds one candidate per such
/// key, and `symmetry` already rules that an identity difference "is two
/// spellings of one move rather than two moves". Both sides closing the
/// orbit is the algebra saying the same: `A·g·w⁻¹·B = 1` and `A·w·g·B = 1`
/// give `g·w⁻¹ = w·g`, one face word reached by two spellings.
///
/// The representative of a move is the certificate of least key —
/// canonical key order, the order every keyed carrier reads in — and not
/// the one the trial happened to issue first.
pub(crate) fn one_per_move(certs: &[PivotCertificate]) -> Vec<PivotCertificate> {
    let mut by_move: Keyed<Keyed<PivotCertificate>> = Keyed::new();
    for c in certs {
        by_move.nested_insert(c.by.move_key(), c.key(), c.clone());
    }
    let mut out = Vec::new();
    for (_, of_move) in by_move.scoped() {
        // index: the certificates of one move, in key order; the least is
        // the representative.
        let mut it = of_move.scoped();
        if let Some((_, c)) = Iterator::next(&mut it) {
            out.push(c.clone());
        }
    }
    out
}

/// NOTHING IS COMPARED HERE. One certified discharge is the discharge;
/// several are a vortex, each held with its certificate. The μ⃗
/// of a candidate is not read in this function at all — it was read once,
/// inside the trial, to refuse a candidate that RAISED a lane, and a
/// candidate that survived that is certified. Choosing between two
/// certified discharges by any function of the discharges' lanes is a score
/// (12.6, 16.16 step 5, T13, T22; law 1).
pub(crate) fn select_discharge(cands: &[PivotCertificate]) -> Discharge {
    match cands {
        [] => Discharge::NoneDischarged,
        [c] => Discharge::One(Box::new(c.clone())),
        _ => Discharge::Vortex(cands.to_vec()),
    }
}

/// The obstruction between each consecutive pair of a vortex's candidates.
/// For the two-candidate vortex this is the pair; for more it is every
/// adjacency in the order they were tried, which is the fixed axis order
/// of 12.6 and not a ranking.
pub(crate) fn obstructions(candidates: &[PivotCertificate]) -> Vec<Obstruction> {
    candidates.windows(2).map(|w| obstruction(&w[0], &w[1])).collect()
}
