// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-SENSING §3.9 (2), §6.2; ROADMAP track 26 S4c — THE HYPHA HEARS A CLAIM
//! OF FACTORISATION: the act read (`dialogue::decode_factorisation`), the
//! posted pair it names found on the book, the claim's stated ground read
//! against the task's train pairs (`cognition::stated_ground_of`), and the
//! claim adjudicated by reconstruction alone
//! (`court::verdict::factorisation::admit_factorisation`): the stated motion
//! rebuilt through the function the mind reads its residue through, the
//! residue derived, and the claim admitted on a match. The court never
//! searches for a factorisation of its own and never proposes one.
//!
//! At any rank: nothing here reads the rank, and nothing passes through the
//! alphabet's section.
//!
//! AN ADMITTED FACTORISATION IS RECORDED, NEVER PAID (ruled 2026-11-04;
//! MU-SENSING §3.9). μ is the metabolic cost of traversal. A factorisation
//! is an adjudicated perception, and its residue is meaning; meaning is
//! content, never paid (§3.7). The book records it
//! (`TaskRegistry::record_factorisation`) and the verdict carries no μ.

use crate::anastome::dialogue::{FactorVerdict, FactorisationAct, PostedTask};
use crate::court::verdict::factorisation::{admit_factorisation, FactorClaim, FactorRefusal};
use crate::court::verdict::{REFUSE_FACTOR_MOTION, REFUSE_FACTOR_NO_PAIR, REFUSE_FACTOR_UNSIGNED};

/// THE VERDICT ON A CLAIM OF FACTORISATION, read on `task`, the posted task
/// the claim's citation names at this court (`None` where it names none).
/// Refused by name where the claim is unsigned, names no task or no train
/// pair, or states a ground the court does not read; otherwise the court's
/// reconstruction decides: admitted, or the residue of the residues.
pub fn judge_factorisation(task: Option<&PostedTask>, act: &FactorisationAct) -> FactorVerdict {
    let refused = |reason: &str| FactorVerdict::Refused { reason: reason.as_bytes().to_vec() };
    if !act.signed {
        return refused(REFUSE_FACTOR_UNSIGNED);
    }
    let Some(task) = task else { return refused("no such task at this court") };
    let Some((x, y)) = task.train.get(act.pair) else { return refused(REFUSE_FACTOR_NO_PAIR) };
    if let Err(why) = crate::cognition::stated_ground_of(&task.train, act.ground.as_deref()) {
        return refused(why.reason());
    }
    let claim = FactorClaim { motion: act.motion.clone(), residue: act.residue.clone() };
    match admit_factorisation(x, y, &claim) {
        Ok(_) => FactorVerdict::Admitted,
        Err(FactorRefusal::Residue { unclaimed, uncarried }) => FactorVerdict::Residue { unclaimed, uncarried },
        Err(FactorRefusal::Motion(why)) => refused(&format!("{REFUSE_FACTOR_MOTION}: {why:?}")),
    }
}
