// MU-CLIENT §4.10, ROADMAP track 10, 78.73, 78.32(3)(f) — HISTORY IS AMENDED BY EXPERIENCE:
// every experience against a held thought deforms it:
// 1. Elastic: confirmed. Residue returns within held constructors; thought renewed.
// 2. Plastic: amended, by addition, never by overwrite. Residue does not return:
//    constructors re-derived from it; amended constructor enters BESIDE the one it amends,
//    one class with two twists held at conjugate grade, separated by commutator.
// 3. The commutator reads what the amendment was: g₁ g₂ g̃₁ g̃₂.
//    Identity: old retires by derivability.
//    Anything else: experience has taught the thought a DISTINCTION both keep.
// 5. Discarded: fracture of the whole thought; leaves active field to cavity.

use crate::transducer::cga::multivector::{one, Mv};
use crate::transducer::cga::transvection_commutator;
use super::Mind;

/// The constitutive response of a thought's constructor under experience (MU-CLIENT §4.10).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Amendment {
    /// Elastic: the experience's residue returns within the thought's held constructors.
    Elastic,
    /// Plastic (identity commutator): the amendment is derivable; old retires by derivability.
    Derivable {
        amended: Mv,
    },
    /// Plastic (non-trivial commutator): amended constructor enters BESIDE the one it amends,
    /// separated by their commutator C = g₁ g₂ g̃₁ g̃₂.
    HeldBeside {
        original: Mv,
        amended: Mv,
        commutator: Mv,
    },
    /// Fracture: nothing in the thought is renewed; leaves active field.
    Fracture,
}

/// Evaluates the amendment between a held constructor and an amended constructor
/// re-derived from experience residue.
pub fn evaluate_amendment(original: &Mv, amended: &Mv) -> Option<Amendment> {
    let commutator = transvection_commutator(original, amended)?;
    if commutator.projective_eq(&Mv::scalar(one())) {
        Some(Amendment::Derivable { amended: amended.clone() })
    } else {
        Some(Amendment::HeldBeside {
            original: original.clone(),
            amended: amended.clone(),
            commutator,
        })
    }
}

impl Mind {
    /// MU-CLIENT §4.10 — AMEND CONSTRUCTOR: evaluate and hold an amendment beside.
    pub fn amend_constructor(&mut self, original: &Mv, amended: &Mv) -> Option<Amendment> {
        let amendment = evaluate_amendment(original, amended)?;
        match &amendment {
            Amendment::HeldBeside { .. } | Amendment::Derivable { .. } => {
                self.amendments.push(amendment.clone());
            }
            _ => {}
        }
        Some(amendment)
    }

    /// MU-CLIENT §4.10, §7.1 — REVISE PERCEPTION ON A COURT RULING BY AMENDMENT:
    /// when a court ruling or experience indicates the perception does not close,
    /// the constructor is amended beside the original.
    pub fn revise_perception_by_amendment(&mut self, original: &Mv, obstruction: &Mv) -> Option<Amendment> {
        self.amend_constructor(original, obstruction)
    }

    /// The amendments held beside.
    pub fn held_amendments(&self) -> &[Amendment] {
        &self.amendments
    }
}
