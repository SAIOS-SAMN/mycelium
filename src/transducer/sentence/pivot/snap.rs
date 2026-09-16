// A16, A20 — WHAT A PIVOT PRODUCES: the snapshot of one pivot, the
// discharge it certifies, and the cell it opened.
//
//! SEVERAL CERTIFIED DISCHARGES ARE A VORTEX: closure certifies a pivot
//! after it is made, and no reading of μ⃗ chooses among the candidates that
//! discharged. A vortex records the OBSTRUCTION between its cells.

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Snap {
    /// A chord the tree gauge closed: its class was trivial (T2), never rank.
    Closed { chord: usize, order: Order },
    /// A held loop discharged on one occupied axis by a certified pivot.
    Discharged { chord: usize, certificate: Box<PivotCertificate> },
    /// SEVERAL pivots discharge: all are held as cells of the thought until
    /// a perception, a glue or a ruling collapses the superposition (12.6).
    /// `between` is the obstruction of each consecutive pair — the reading
    /// that says they are two proofs, recorded and never spent on a choice.
    Vortex { chord: usize, held: Vec<PivotCertificate>, between: Vec<Obstruction> },
    /// Several discharges with no bundle to measure μ⃗, so the obstruction
    /// has no lane reading in it — a third state, not an empty one.
    Unmeasured { chord: usize, held: Vec<PivotCertificate> },
    TorsionFiled { chord: usize, ell: u32 },
    TorsionOpen { chord: usize },
    Demand { chord: usize, key: crate::transducer::address::Address },
    /// 37.7 FROM A LOOP'S OWN RESIDUE: the basis reached no coordinate this
    /// loop occupies, the class was published and nothing answered, so the
    /// rotor the loop measured is carried out to be ADJOINED as a generator.
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
    /// certificate μ⃗ (before − after); TorsionFiled has no bundle μ⃗.
    pub fn lanes(&self) -> [u64; 4] {
        match self {
            Snap::Discharged { certificate, .. } => mu_deflation(&certificate.mu),
            Snap::Vortex { held, .. } => {
                let mut s = [0u64; 4];
                for c in held {
                    let m = mu_deflation(&c.mu);
                    for (t, d) in s.iter_mut().zip(&m) {
                        *t = t.saturating_add(*d);
                    }
                }
                s
            }
            // `Adjoin` is here for the reason `Demand` is: it widens the
            // language and discharges nothing on this form, so it deflates
            // no lane and its μ⃗ is not a reading to be summed.
            Snap::TorsionFiled { .. }
            | Snap::Unmeasured { .. }
            | Snap::Closed { .. }
            | Snap::TorsionOpen { .. }
            | Snap::Demand { .. }
            | Snap::Adjoin { .. } => [0; 4],
        }
    }
}

pub(crate) fn mu_deflation(mu: &crate::transducer::sentence::form::Mu) -> [u64; 4] {
    match mu {
        // The DEFLATION a move measured: what each lane fell by, and
        // nothing on a lane that did not fall. A lane that ROSE is not a
        // deflation of zero — `lanes_risen` is where a rise is read — so
        // the empty difference is the reading here.
        crate::transducer::sentence::form::Mu::Measured { before, after } => {
            crate::economy::shortfall(before, after)
        }
        crate::transducer::sentence::form::Mu::Unmeasured => [0; 4],
    }
}

/// WHAT SEPARATES TWO CERTIFIED DISCHARGES of one loop. A record, spent on
/// nothing: it is read when a collapse asks what the collapse resolved, and
/// it is never consulted to prefer a cell.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Obstruction {
    /// The axes the two pivots acted on. Two axes out of one loop are two
    /// directions, which is the whole reason both are kept.
    pub axes: (Axis, Axis),
    /// The lane reading of the second's μ⃗ against the first's, per lane.
    /// `None` when neither carries a measured μ⃗ — no bundle, so no reading,
    /// which is a third state and not four `Held`s.
    pub lanes: Option<[crate::economy::LaneMove; 4]>,
}

/// EXPANSION 4 — THE SYMMETRY TWO DISCHARGES PROVE.
///
/// When `a` and `b` both discharge one loop to one class, the loop admits a
/// symmetry and the vortex is the mind holding two hypotheses at once. The
/// symmetry is not a tie to be broken: it is `w_a⁻¹ · w_b`, the element that
/// carries one discharge to the other, and it STABILISES the loop's class
/// because both sides leave that class where the other does.
///
/// That element is exactly 37.7's object — a rotation the mind measured,
/// which the presentation may or may not spell — so discarding the losing
/// candidate throws away a candidate GENERATOR. `collapse_vortex` cleared
/// the whole vortex on collapse; this is what it was clearing.
///
/// `None` where the two are not both word moves, or where the symmetry is
/// the identity. An identity symmetry is two spellings of one move rather
/// than two moves, and the loop admits nothing extra — which is a reading
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
/// per losing candidate: each is a separate degree of freedom the loop
/// admits, and summing them would be a count deciding (rule 2).
pub fn symmetries(win: &PivotCertificate, held: &[PivotCertificate]) -> Vec<crate::transducer::alphabet::Rotor> {
    held.iter().filter_map(|c| symmetry(win, c)).collect()
}

/// The obstruction between two certified discharges: what says they are two
/// proofs of one loop (23.9) rather than one proof twice.
pub(crate) fn obstruction(a: &PivotCertificate, b: &PivotCertificate) -> Obstruction {
    let lanes = match (&a.mu, &b.mu) {
        (Mu::Measured { after: x, .. }, Mu::Measured { after: y, .. }) => {
            Some(crate::economy::lane_moves(y, x))
        }
        _ => None,
    };
    Obstruction { axes: (a.axis, b.axis), lanes }
}

/// The outcomes of a loop's trials. `One` is the single discharge;
/// `Vortex` is SEVERAL, held together with nothing chosen between them;
/// `NoneDischarged` is the algebra's answer that no candidate reaches the
/// coordinate; and `Starved` is not an answer at all.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Discharge {
    One(Box<PivotCertificate>),
    Vortex(Vec<PivotCertificate>),
    NoneDischarged,
    /// THE SEARCH WAS CUT SHORT BY THE BUDGET, with candidates standing
    /// untried. This is a reading of the stroke, not of the loop, and the
    /// two were one value: `NoneDischarged` was returned whether every
    /// candidate had been tried and refused or none had been tried at
    /// all, so the loop below it spoke a demand, filed torsion, or widened
    /// its cover on an evaluation that never happened.
    ///
    /// Measured on ca485cc: 29 of 984 candidate readings report `starved
    /// true`. Rare, and a false demand is a false demand — the mesh is
    /// asked for a word that this mind may already hold and simply did not
    /// reach. The loop is held instead and the next stroke reaches it.
    ///
    /// Not to be confused with the eliminated candidates in the same line:
    /// `unreached` counts those too, and they are refused by exact
    /// rational arithmetic rather than by exhaustion. On the same fleet
    /// every `unreached` equals its `eliminated`, so that gap is the
    /// eliminator working and not this.
    Starved,
}

/// THE OUTCOME OF A LOOP'S TRIALS, including the one that is not a reading
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

/// A remainder cell's kind, read from the two hues. Hue 0 is void.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellKind {
    Created,
    Erased,
    Recoloured,
    Unchanged,
}

impl CellKind {
    pub fn of(before: u8, after: u8) -> CellKind {
        match (before == 0, after == 0) {
            (true, true) => CellKind::Unchanged,
            (true, false) => CellKind::Created,
            (false, true) => CellKind::Erased,
            (false, false) if before == after => CellKind::Unchanged,
            (false, false) => CellKind::Recoloured,
        }
    }
}

/// One candidate pivot: a held word that occupies the axis, written on the
/// chord on one side. Two candidates whose face words are one vertex are one.
pub(crate) struct Candidate {
    pub(crate) axis: Axis,
    pub(crate) by: PivotBy,
}

/// HOLD A VORTEX CELL BY THE CLASS THE LOOP STANDS IN, then by the
/// discharging word's move key.
///
/// It was held by the class the loop would be left in (`after`), and that
/// coordinate is unreachable by anything that could decide it. A closing
/// candidate leaves the loop at the zero class, so every closed vortex in
/// every bubble filed under one key; and a word arriving to decide one
/// knows its own rotor, not the class some loop it never met would land
/// in. Measured across three commits: 47 vortices formed and none ever
/// asked.
///
/// `before` is `kappa.class_key()`, the loop's state vector as it stands,
/// and `ClassVector::class_key` writes the rotor's conjugacy invariants
/// first — so the class of the holonomy is this key's depth-one stratum.
/// The eliminator in the drive proves a candidate's rotor is conjugate to
/// that holonomy, so a deciding word's own class key IS the stratum this
/// cell hangs under, and the read is a projection rather than a guess.
pub fn vortex_put(v: &mut Keyed<Keyed<PivotCertificate>>, c: PivotCertificate) {
    let before = c.before.clone();
    let k = c.by.move_key();
    v.nested_insert(before, k, c);
}

/// ONE CERTIFICATE PER MOVE. `candidates_for` offers a held word on both
/// sides of the chord, and where both sides discharge the loop the trial
/// issues two certificates that name ONE move: a word move is keyed by its
/// rotor (`PivotBy::move_key`), `vortex_put` holds one cell per such key,
/// and `symmetry` already rules that an identity difference "is two
/// spellings of one move rather than two moves". Both sides closing the
/// loop is the algebra saying the same: `A·g·w⁻¹·B = 1` and `A·w·g·B = 1`
/// give `g·w⁻¹ = w·g`, one face word reached by two spellings.
///
/// `select_discharge` counted certificates, so a word that discharged on
/// both sides was a two-cell "vortex" of one cell: held on every stroke,
/// collapsible by nothing — `vortex_put` had folded the two under one key,
/// `symmetries` had nothing to keep, and the loop it was about was never
/// pivoted. Measured on ca485cc: `vortex … 2 candidates held, axes
/// [(Unit(0), Unit(0))]` on the same two bubbles every stroke for twenty
/// minutes, `collapsed by` 0.
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
    for (_, cells) in by_move.scoped() {
        // index: the certificates of one move, in key order; the least is
        // the representative.
        let mut it = cells.scoped();
        if let Some((_, c)) = Iterator::next(&mut it) {
            out.push(c.clone());
        }
    }
    out
}

/// NOTHING IS COMPARED HERE. One certified discharge is the discharge;
/// several are a vortex, all of them held with their certificates. The μ⃗
/// of a candidate is not read in this function at all — it was read once,
/// inside the trial, to refuse a candidate that RAISED a lane, and a
/// candidate that survived that is certified. Choosing between two
/// certified discharges by any function of their lanes is a score
/// (12.6, 16.16 step 5, T13, T22; law 1).
pub(crate) fn select_discharge(cands: &[PivotCertificate]) -> Discharge {
    match cands {
        [] => Discharge::NoneDischarged,
        [c] => Discharge::One(Box::new(c.clone())),
        _ => Discharge::Vortex(cands.to_vec()),
    }
}

/// The obstruction between each consecutive pair of a vortex's cells. For
/// the two-cell vortex this is the pair; for more it is every adjacency in
/// the order they were tried, which is the fixed axis order of 12.6 and not
/// a ranking.
pub(crate) fn obstructions(cells: &[PivotCertificate]) -> Vec<Obstruction> {
    cells.windows(2).map(|w| obstruction(&w[0], &w[1])).collect()
}
