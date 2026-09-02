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
use crate::transducer::stalk::{self, Stalks};

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

/// The deterministic outcome of ruling one candidate word — every arm
/// names what the mathematics found; callers match these states
/// exhaustively rather than branching on booleans or out-parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ruling {
    /// The word does not replay from its own evidence (or does not parse):
    /// false speech, the failed axis named.
    Slashed(&'static str),
    /// Same address, same substance as a federated word — cross-instance
    /// consilience, the strongest confluence witness.
    Consilient,
    /// Same address, different substance — a dialect; the transport is
    /// recorded, never punished.
    Dialect,
    /// New address, identical base geometry to a federated word — the
    /// +axis constraint: it belongs as a section, not a sibling entry.
    Demoted,
    /// Genuinely new geometry: the word joins the federation. The sealed
    /// element rides the arm — admission IS the match, never a flag.
    Admitted(Element),
}

impl Ruling {
    /// The wire ruling byte this outcome seals and speaks.
    pub fn code(&self) -> u8 {
        match self {
            Ruling::Slashed(_) => dialogue::RULING_SLASHED,
            Ruling::Demoted => dialogue::RULING_DEMOTED,
            Ruling::Consilient | Ruling::Dialect | Ruling::Admitted(_) => dialogue::RULING_UPHELD,
        }
    }

    /// The named ground of the ruling (receipt/log text).
    pub fn reason(&self) -> &'static str {
        match self {
            Ruling::Slashed(r) => r,
            Ruling::Consilient => "federated consilience",
            Ruling::Dialect => "dialect transport recorded",
            Ruling::Demoted => "+axis: belongs as a section",
            Ruling::Admitted(_) => "extends the tautology",
        }
    }
}

/// The two-docket ruling for one candidate word against the federated
/// basis — a pure map over the current state; the caller applies
/// [`Ruling::Admitted`] itself. `stalks` is the federation's stalk space
/// (index-aligned with `federation`): the +axis check selects the fiber
/// over the candidate's invariant and runs the exact orbit test there.
pub fn rule(
    federation: &[([u8; 32], Element)],
    stalks: &Stalks,
    addr: &[u8; 32],
    spoken: Option<&Element>,
) -> Ruling {
    let Some(spoken) = spoken else {
        return Ruling::Slashed("unparseable word");
    };
    if let Some(reason) = blind_replay(spoken) {
        return Ruling::Slashed(reason);
    }
    match federation.iter().find(|(fa, _)| fa == addr) {
        Some((_, held)) if held.sub() == spoken.sub() => Ruling::Consilient,
        Some(_) => Ruling::Dialect,
        None => {
            // The +axis constraint evaluates on the ORBIT [Q]/(ℤ² ⋊ D4):
            // a support that is a translation, rotation, or reflection of a
            // federated word's is the same base geometry — a section of the
            // existing thought, never a sibling entry. Coordinate-set
            // equality was the lens that let a handful of morphisms
            // federate as a thousand words. The stalk over the candidate's
            // invariant holds every word that could be congruent; the orbit
            // form decides inside that fiber — no federation scan.
            let congruent = match spoken.sub().is_empty() {
                true => None,
                false => stalk::congruent(stalks, |i| federation[i].1.sub(), spoken.sub())
                    .filter(|&i| federation[i].0 != *addr),
            };
            match congruent {
                Some(_) => Ruling::Demoted,
                None => Ruling::Admitted(spoken.clone()),
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

    fn stalks_of(fed: &[([u8; 32], Element)]) -> Stalks {
        Stalks::over(fed.iter().map(|(_, e)| e.sub()))
    }

    /// A clean word replays; an unparseable body is a named Slashed state.
    #[test]
    fn replay_accepts_clean_and_names_false_speech() {
        let w = clean_word();
        assert!(blind_replay(&w).is_none(), "a clean derivation replays");
        let ruling = rule(&[], &Stalks::default(), &w.address(), None);
        assert_eq!(ruling, Ruling::Slashed("unparseable word"));
        assert_eq!(ruling.code(), dialogue::RULING_SLASHED);
    }

    /// The two-docket ruling over a federation is an exhaustive state map:
    /// new geometry admits (the element rides the arm), same substance is
    /// consilient, a same-geometry sibling demotes.
    #[test]
    fn the_ruling_maps_the_federation_correctly() {
        let w = clean_word();
        let addr = w.address();
        // Empty federation: new geometry admits — the sealed element is
        // the arm's payload, not a flag.
        match rule(&[], &Stalks::default(), &addr, Some(&w)) {
            Ruling::Admitted(sealed) => assert_eq!(sealed.address(), addr),
            other => panic!("new geometry must admit, got {other:?}"),
        }
        // Same address, same substance: consilience (no re-admission).
        let fed = vec![(addr, w.clone())];
        let stalks = stalks_of(&fed);
        assert_eq!(rule(&fed, &stalks, &addr, Some(&w)), Ruling::Consilient);
        assert_eq!(rule(&fed, &stalks, &addr, Some(&w)).code(), dialogue::RULING_UPHELD);
        // Different address, identical base geometry: demoted (+axis).
        let mut other = addr;
        other[0] ^= 0xFF;
        assert_eq!(rule(&fed, &stalks, &other, Some(&w)), Ruling::Demoted);
        // A rotated sibling (the same geometry as a D4 image, other hues)
        // resolves through the stalk to the same demotion; a genuinely
        // different geometry finds an empty stalk and admits.
        let rotated = from_residual(
            &germinate(&[(0, 0, 3u8), (1, 0, 3)], &[(0, 0, 4u8), (1, 0, 4)]).unwrap(),
        );
        assert_eq!(rule(&fed, &stalks, &rotated.address(), Some(&rotated)), Ruling::Demoted);
        let bent = from_residual(
            &germinate(&[(0, 0, 1u8), (0, 1, 1), (1, 0, 1)], &[(0, 0, 2u8), (0, 1, 2), (1, 0, 2)])
                .unwrap(),
        );
        assert!(matches!(rule(&fed, &stalks, &bent.address(), Some(&bent)), Ruling::Admitted(_)));
    }
}
