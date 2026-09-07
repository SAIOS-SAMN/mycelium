// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! CONVERGENCE — two organs re-derive one object and reach one address.
//!
//! This was `network::consensus`: "two independent mouths must meet",
//! `QUORUM = 2`, `voices(germ) >= QUORUM`. That counts agreement, which is
//! not what this engine does. The condition is convergence: the home
//! re-derives the object, an independent witness hypha re-derives it, and
//! they reach the SAME M2 address because the algebra is confluent. Nothing
//! is outvoted; it either re-derives or it does not, and the only other
//! outcome is `REFUSE_MV_INEXACT` (13.9 step 8). The old code's own comment
//! said so — "agreement is a theorem about the algebra, not a matched card"
//! — and then counted mouths anyway.
//!
//! The two organs are the home and a WITNESS HYPHA (47.3), never two voices
//! on one percept. A second voice uttering the same residual is a USE,
//! credited to both and made a settlement party (24.1), and never a step
//! toward admission: distinct tasks per voice never supply a second mouth,
//! so counting them held every germ pending forever.

use crate::network::nerve::{Keyed, ROOT};
use crate::transducer::a21::Address;

/// One organ's re-derivation of one object: the address it reached, and
/// where it stood when it reached it. The address is the claim — two organs
/// converge iff theirs are equal — and K and height place it causally
/// (45.2, 45.4) so a witness's own chain can be checked against it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rederivation {
    pub address: Address,
    pub k: u64,
    pub height: u64,
}

/// What each organ re-derived, per object. Keyed by the object's address,
/// then by the organ's verifying key: a coordinate read, never a scan.
#[derive(Debug, Default, Clone)]
pub struct Convergence {
    rederived: Keyed<Keyed<Rederivation>>,
    /// Voices that uttered an identical residual: a use and a settlement
    /// party, kept apart from the organs so a mouth can never be mistaken
    /// for a vantage.
    uses: Keyed<Keyed<()>>,
}

/// Whether an object has converged, and what to do when it has not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Converged {
    /// One organ has re-derived it. It is docketed and awaits a witness.
    AwaitingWitness,
    /// Two independent organs reached the same address.
    Witnessed { home: [u8; 32], witness: [u8; 32] },
    /// Two organs re-derived the same object and reached DIFFERENT
    /// addresses. That is not a disagreement to be settled by a third; it is
    /// inconsistent data, and the court refuses it (13.9 step 8).
    Inexact { a: Address, b: Address },
}

impl Convergence {
    pub fn new() -> Self {
        Convergence::default()
    }

    /// Record that `organ` re-derived `object` and reached `r.address`.
    /// Returns whether this organ is new to the object.
    pub fn rederived(&mut self, object: &[u8], organ: [u8; 32], r: Rederivation) -> bool {
        match self.rederived.get_mut(object) {
            Some(by_organ) => {
                if by_organ.contains(&organ) {
                    return false;
                }
                by_organ.insert(organ, r);
                true
            }
            None => {
                let mut by_organ = Keyed::new();
                by_organ.insert(organ, r);
                self.rederived.insert(object, by_organ);
                true
            }
        }
    }

    /// A voice uttered an identical residual: a use, and a party to the
    /// settlement. Never a vantage.
    pub fn used_by(&mut self, object: &[u8], voice: [u8; 32]) {
        match self.uses.get_mut(object) {
            Some(v) => {
                v.insert(voice, ());
            }
            None => {
                let mut v = Keyed::new();
                v.insert(voice, ());
                self.uses.insert(object, v);
            }
        }
    }

    /// The voices that uttered this object — the settlement parties.
    pub fn parties(&self, object: &[u8]) -> Vec<[u8; 32]> {
        self.uses
            .get(object)
            .map(|v| {
                v.fiber(ROOT)
                    .filter_map(|(k, _)| <[u8; 32]>::try_from(k.as_slice()).ok())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// The organs that have re-derived this object.
    pub fn organs(&self, object: &[u8]) -> Vec<([u8; 32], Rederivation)> {
        self.rederived
            .get(object)
            .map(|by| {
                by.fiber(ROOT)
                    .filter_map(|(k, r)| {
                        <[u8; 32]>::try_from(k.as_slice()).ok().map(|o| (o, r.clone()))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Has this object converged? `independent` decides whether two organs
    /// are two vantages — 47.3's test: bond outside the home's delegation
    /// closure, room covering the escrow — and is the caller's, because only
    /// the kernel holds the ledger that answers it.
    pub fn state(
        &self,
        object: &[u8],
        independent: &dyn Fn(&[u8; 32], &[u8; 32]) -> bool,
    ) -> Converged {
        let organs = self.organs(object);
        // A cell's own members, walked: the organs that re-derived this one
        // object, never a product over the mesh.
        for (i, (a, ra)) in organs.iter().enumerate() {
            for (b, rb) in organs.iter().skip(i + 1) {
                if ra.address != rb.address {
                    return Converged::Inexact {
                        a: ra.address.clone(),
                        b: rb.address.clone(),
                    };
                }
                if independent(a, b) {
                    return Converged::Witnessed {
                        home: *a,
                        witness: *b,
                    };
                }
            }
        }
        Converged::AwaitingWitness
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addr(n: u8) -> Address {
        Address::from_bytes(&[n])
    }
    fn r(n: u8) -> Rederivation {
        Rederivation { address: addr(n), k: 1, height: 1 }
    }
    fn always(_: &[u8; 32], _: &[u8; 32]) -> bool {
        true
    }
    fn never(_: &[u8; 32], _: &[u8; 32]) -> bool {
        false
    }

    /// One vantage dockets. The object waits for a witness, and waiting is
    /// its own state — not a count that has not reached two.
    #[test]
    fn one_organ_awaits_a_witness() {
        let mut c = Convergence::new();
        assert!(c.rederived(&[9u8], [1; 32], r(9)));
        assert_eq!(c.state(&[9u8], &always), Converged::AwaitingWitness);
    }

    /// The same organ twice is one vantage. It was `a_duplicate_voice_does_
    /// not_reach_quorum`; the claim survives the carrier and the thing that
    /// cannot be duplicated is an ORGAN, not a mouth.
    #[test]
    fn a_duplicate_organ_does_not_converge() {
        let mut c = Convergence::new();
        assert!(c.rederived(&[9u8], [1; 32], r(9)));
        assert!(!c.rederived(&[9u8], [1; 32], r(9)), "one organ, one vantage");
        assert_eq!(c.organs(&[9u8]).len(), 1);
        assert_eq!(c.state(&[9u8], &always), Converged::AwaitingWitness);
    }

    /// Two organs reaching one address is the admission condition.
    #[test]
    fn an_independent_witness_rederivation_installs() {
        let mut c = Convergence::new();
        c.rederived(&[9u8], [1; 32], r(9));
        c.rederived(&[9u8], [2; 32], r(9));
        match c.state(&[9u8], &always) {
            Converged::Witnessed { home, witness } => {
                assert_ne!(home, witness, "two vantages, not one twice");
            }
            other => panic!("two organs at one address converge, got {other:?}"),
        }
        // Independence is 47.3's test and it is the caller's: a witness
        // inside the home's delegation closure is not a second vantage.
        assert_eq!(c.state(&[9u8], &never), Converged::AwaitingWitness);
    }

    /// Two organs re-deriving one object to DIFFERENT addresses is not a
    /// disagreement for a third to settle. It is inconsistent data.
    #[test]
    fn disagreeing_rederivations_refuse_mv_inexact() {
        let mut c = Convergence::new();
        c.rederived(&[9u8], [1; 32], r(9));
        c.rederived(&[9u8], [2; 32], r(8));
        match c.state(&[9u8], &always) {
            Converged::Inexact { a, b } => assert_ne!(a, b),
            other => panic!("differing addresses refuse, got {other:?}"),
        }
    }

    /// A second voice uttering the same residual is a USE and a settlement
    /// party. It never moves the object toward admission.
    #[test]
    fn a_second_voice_is_a_use_never_a_gate() {
        let mut c = Convergence::new();
        c.rederived(&[9u8], [1; 32], r(9));
        c.used_by(&[9u8], [7; 32]);
        c.used_by(&[9u8], [8; 32]);
        c.used_by(&[9u8], [7; 32]);
        let mut p = c.parties(&[9u8]);
        p.sort();
        assert_eq!(p.len(), 2, "distinct voices, counted once each");
        assert_eq!(
            c.state(&[9u8], &always),
            Converged::AwaitingWitness,
            "no number of voices is a second organ"
        );
    }
}
