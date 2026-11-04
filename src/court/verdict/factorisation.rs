// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-SENSING §3.9 (2) — A CLAIM OF FACTORISATION ADJUDICATED: a claim
//! states a pair's motion and the residue it leaves; the court reconstructs
//! the motion on its own copy of the posted samples, DERIVES THE RESIDUE
//! ITSELF through the one function the mind read it through
//! (`transduce::Factorisation::of`), and admits the claim where the two
//! residues are one. It never searches for a motion, never proposes one,
//! and never reads the claimed residue into its own: a disagreement is
//! returned as the residue of the residues, the links each side holds that
//! the other does not (MU-HOSTING §5a.10), never a flag. At any rank: the
//! samples, the motion and the links carry their own.

use crate::transducer::family::transduce::{need_of_motion, Factorisation, Motion, TransductionRefusal};
use crate::transducer::sample::Sample;
use crate::transducer::sentence::Link;

/// A CLAIM OF FACTORISATION: the motion stated, and the residue stated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorClaim {
    pub motion: Motion,
    pub residue: Vec<Link>,
}

/// THE CLAIM ADMITTED: the factorisation the court reconstructed, whose
/// residue is the claim's.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorAdmitted {
    pub reconstructed: Factorisation,
}

/// WHY A CLAIM OF FACTORISATION IS REFUSED, named.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FactorRefusal {
    /// The stated motion is no motion of the posted pair: refused by the
    /// transduction's own name (`Rank`, `OffSite`, `OntoAnother`, `Spent`).
    Motion(TransductionRefusal),
    /// The residue the court derived is not the residue claimed: `unclaimed`
    /// holds the links the court derived and the claim left out, `uncarried`
    /// the links the claim stated and the motion does not leave.
    Residue { unclaimed: Vec<Link>, uncarried: Vec<Link> },
}

/// ADJUDICATE A CLAIM OF FACTORISATION on the posted pair `(x, y)`: the
/// stated motion reconstructed under the stroke its need sizes, the residue
/// derived, and the claim admitted on a match.
pub fn admit_factorisation(x: &Sample, y: &Sample, claim: &FactorClaim) -> Result<FactorAdmitted, FactorRefusal> {
    let need = need_of_motion(&claim.motion).ok_or(FactorRefusal::Motion(TransductionRefusal::Spent { need: None }))?;
    let mut stroke = crate::physics::Physics::default().stroke_n(need);
    let reconstructed = Factorisation::of(x, y, &claim.motion, &mut stroke).map_err(FactorRefusal::Motion)?;
    let mut claimed = claim.residue.clone();
    claimed.sort_unstable();
    claimed.dedup();
    let derived = &reconstructed.residue;
    let unclaimed: Vec<Link> = derived.iter().filter(|l| claimed.binary_search(l).is_err()).cloned().collect();
    let uncarried: Vec<Link> = claimed.iter().filter(|l| !derived.contains(l)).cloned().collect();
    match (unclaimed.is_empty(), uncarried.is_empty()) {
        (true, true) => Ok(FactorAdmitted { reconstructed }),
        _ => Err(FactorRefusal::Residue { unclaimed, uncarried }),
    }
}
