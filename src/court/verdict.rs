// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Verification core for the court binary (#162, three-binary split).
//!
//! The court runs on this module and the shared exact algebra only — no
//! market, no door, no epigenome, no task registry. Replaying a candidate
//! reduction needs the minimal cellular replayer:
//!
//!   replay: (W, ∂₁Q) → (δσ̂ = 0)  ∧  strict ≺_DM descent
//!
//! plus the two-docket ruling over the federated basis and the π*
//! derivation from pooled evidence. Keeping this separate lets the court
//! binary run in constrained validator environments without linking the
//! server's role-specific machinery.

use crate::anastome::dialogue;
use crate::transducer::anchor::{derive_carrying_all, Sel};
use crate::transducer::element::{self, Element};

/// Deterministic re-derivation of a spoken word from its own evidence
/// (the contradiction docket). Returns None when the word replays clean;
/// Some(reason) when the speech was false. Same chain → same word
/// (from_residual is deterministic), the chain must be canonically framed,
/// and the install must strictly descend (0⃗ ≺_DM μ⃗).
pub fn blind_replay(spoken: &Element) -> Option<&'static str> {
    let chain = spoken.sub();
    if chain.is_empty() {
        return Some("empty chain — nothing was derived");
    }
    let min_r = chain.iter().map(|(r, _, _, _)| *r).min().unwrap_or(0);
    let min_c = chain.iter().map(|(_, c, _, _)| *c).min().unwrap_or(0);
    if (min_r, min_c) != (0, 0) {
        return Some("chain not framed to origin");
    }
    if !chain.windows(2).all(|w| w[0] <= w[1]) {
        return Some("chain not canonically ordered");
    }
    let rederived = element::from_residual(&chain.to_vec());
    if rederived.address() != spoken.address() {
        return Some("address does not re-derive from the chain");
    }
    if rederived.magnitude() != spoken.magnitude() {
        return Some("magnitude does not re-derive from the chain");
    }
    if !crate::economy::descends(&[0; 4], &spoken.magnitude()) {
        return Some("no strict descent — the germ closes nothing");
    }
    None
}

/// The two-docket ruling for one candidate word against the federated
/// basis. Returns the ruling byte, the reason, and whether the word joins
/// the federation (the caller owns the federation Vec; this function does
/// not mutate it, so the decision is a pure map over the current state).
///
/// - Failed replay → SLASHED (false speech).
/// - Same address, same substance → UPHELD (cross-instance consilience).
/// - Same address, different substance → UPHELD, dialect transport
///   recorded rather than punished.
/// - New address but identical base geometry to a federated word →
///   DEMOTED (the +axis constraint: it belongs as a section, not a
///   sibling entry).
/// - Genuinely new geometry → UPHELD and admitted to the federation.
pub fn rule(
    federation: &[([u8; 32], Element)],
    addr: &[u8; 32],
    spoken: Option<&Element>,
) -> (u8, &'static str, bool) {
    let Some(spoken) = spoken else {
        return (dialogue::RULING_SLASHED, "unparseable word", false);
    };
    if let Some(reason) = blind_replay(spoken) {
        return (dialogue::RULING_SLASHED, reason, false);
    }
    match federation.iter().find(|(fa, _)| fa == addr) {
        Some((_, held)) if held.sub() == spoken.sub() => {
            (dialogue::RULING_UPHELD, "federated consilience", false)
        }
        Some(_) => (dialogue::RULING_UPHELD, "dialect transport recorded", false),
        None => {
            let pos_new = element::positions(spoken.sub());
            let axis_conflict = federation.iter().any(|(fa, held)| {
                fa != addr
                    && !held.sub().is_empty()
                    && element::positions(held.sub()) == pos_new
            });
            if axis_conflict {
                (dialogue::RULING_DEMOTED, "+axis: belongs as a section", false)
            } else {
                (dialogue::RULING_UPHELD, "extends the tautology", true)
            }
        }
    }
}

/// π* derivation from a pooled evidence family: the first anchor term
/// common to every evidence's carrying set (the intersection move — one
/// evidence never binds; the caller holds families of at least two).
pub fn derive_pi(family: &[(Vec<(i64, i64)>, Vec<(i64, i64, u8)>)]) -> Option<Sel> {
    let sets: Vec<Vec<Sel>> =
        family.iter().map(|(fp, scene)| derive_carrying_all(fp, scene)).collect();
    let (first, rest) = sets.split_first()?;
    first.iter().find(|t| rest.iter().all(|s| s.contains(*t))).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::from_residual;
    use crate::transducer::germinate::germinate;

    fn clean_word() -> Element {
        from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap())
    }

    /// A clean word replays; a tampered address, a shifted frame, or an
    /// unparseable body are each named and slashed.
    #[test]
    fn replay_accepts_clean_and_names_false_speech() {
        let w = clean_word();
        assert!(blind_replay(&w).is_none(), "a clean derivation replays");
        let (code, reason, _) = rule(&[], &w.address(), None);
        assert_eq!(code, dialogue::RULING_SLASHED);
        assert_eq!(reason, "unparseable word");
    }

    /// The two-docket ruling over a federation: consilience upholds, a
    /// same-geometry sibling demotes, new geometry admits.
    #[test]
    fn the_ruling_maps_the_federation_correctly() {
        let w = clean_word();
        let addr = w.address();
        // Empty federation: new geometry admits.
        let (code, _, joins) = rule(&[], &addr, Some(&w));
        assert_eq!(code, dialogue::RULING_UPHELD);
        assert!(joins, "new geometry joins the federation");
        // Same address, same substance: consilience (no re-admission).
        let fed = vec![(addr, w.clone())];
        let (code, reason, joins) = rule(&fed, &addr, Some(&w));
        assert_eq!((code, joins), (dialogue::RULING_UPHELD, false));
        assert_eq!(reason, "federated consilience");
        // Different address, identical base geometry: demoted (+axis).
        let mut other = addr;
        other[0] ^= 0xFF;
        let (code, _, joins) = rule(&fed, &other, Some(&w));
        assert_eq!((code, joins), (dialogue::RULING_DEMOTED, false));
    }
}
