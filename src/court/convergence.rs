// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 47.3, 13.9 — CONVERGENCE: the home and a witness hypha re-derive one
//! object and reach the SAME M2 address because the algebra is confluent.
//! Nothing is outvoted; the only other outcome is `REFUSE_MV_INEXACT`.

use crate::network::nerve::Keyed;
use crate::transducer::address::Address;

/// One organ's re-derivation of one object: the address it reached, and
/// where it stood when it reached it. The address is the claim — two organs
/// converge iff the two addresses are equal — and K and height place it causally
/// (45.2, 45.4) so a witness's own chain can be checked against it.
#[derive(Debug, Clone)]
pub struct Rederivation {
    /// What the organ RE-DERIVED. Two organs converge iff the two objects are
    /// the same vertex (M1, 17.1), which is a question about the algebra
    /// and not about the encoding: a carriage versor and its negation are
    /// one projective element and two byte strings, so two organs that
    /// derived carriages of opposite sign hold one vertex and comparing
    /// the two addresses would have called them different.
    pub object: crate::transducer::element::Element,
    pub k: u64,
    pub height: u64,
}

impl Rederivation {
    /// The address this re-derivation reaches. A coordinate of the object,
    /// carried for the docket and the log -- never the convergence test.
    pub fn address(&self) -> Address {
        self.object.address()
    }
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

    /// THE ENTRIES CONVERGENCE HOLDS: each organ's re-derivation of each
    /// object, and each use of each.
    pub fn held(&self) -> usize {
        let rederived: usize = self.rederived.scoped().map(|(_, by)| by.len()).sum();
        rederived + self.uses.scoped().map(|(_, by)| by.len()).sum::<usize>()
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
    /// AN OBJECT THE COURT NO LONGER HOLDS: its re-derivations and its uses
    /// leave with it.
    pub fn forget(&mut self, object: &[u8]) {
        self.rederived.remove(object);
        self.uses.remove(object);
    }

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
        // An object no voice has uttered has no parties on it: the reading
        // of absence on a keyed carrier (0.11c).
        match self.uses.get(object) {
            // `get(object)` is the read by the key: what it returns is the voices
            // on THAT object, and all of them are its parties.
            Some(v) => v
                .scoped()
                .filter_map(|(k, _)| <[u8; 32]>::try_from(k.as_slice()).ok())
                .collect(),
            None => Vec::new(),
        }
    }

    /// The organs that have re-derived this object.
    pub fn organs(&self, object: &[u8]) -> Vec<([u8; 32], Rederivation)> {
        // As above: no re-derivation, no organs.
        match self.rederived.get(object) {
            // As above: `get` scoped it to this object's re-derivations.
            Some(by) => by
                .scoped()
                .filter_map(|(k, r)| <[u8; 32]>::try_from(k.as_slice()).ok().map(|o| (o, r.clone())))
                .collect(),
            None => Vec::new(),
        }
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
        // One object's own organs, walked: the organs that re-derived this
        // one object, never a product over the mesh.
        //
        // AGREEMENT IS ASKED OF EVERY PAIR BEFORE ANY PAIR IS WITNESSED.
        // These two questions shared one walk, and `Witnessed` returned from
        // inside it — so the first independent pair ended the walk and every
        // pair after it went unexamined. A court could witness an object two
        // organs agreed on while a third stood there disagreeing, which is
        // exactly the re-derivation 13.9 step 8 says it refuses. Whether the
        // disagreement was seen depended on where the disagreeing organ sat
        // in the walk.
        //
        // A1: the test is RE-DERIVATION, never bytes. `same_vertex` compares
        // decoded coordinates with each algebra's own predicate — M1 on the
        // letters, projective equality on the carriage versors, A5 on the
        // chain — and takes address equality only as a fast path inside
        // itself. Only a genuinely differing re-derivation is Inexact.
        for (i, (_, ra)) in organs.iter().enumerate() {
            for (_, rb) in organs.iter().skip(i + 1) {
                if !ra.object.same_vertex(&rb.object) {
                    return Converged::Inexact {
                        a: ra.address(),
                        b: rb.address(),
                    };
                }
            }
        }
        // Every organ re-derived one object. Now a witness is an independent
        // pair among them, and any such pair is one — nothing here prefers
        // the pair that happens to come first, because they all say the same
        // thing about the same object.
        for (i, (a, _)) in organs.iter().enumerate() {
            for (b, _) in organs.iter().skip(i + 1) {
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

    use crate::transducer::element::{self, Element};

    /// A distinct object per `n`: the residual of one compartment recoloured to
    /// hue `n`, which is a different vertex for each n.
    fn obj(n: u8) -> Element {
        element::from_canonical(
            &element::canonical_chain(&[(0i64, 0i64, 1u8, n)]).expect("one link is A5's chain"),
        )
    }
    fn r(n: u8) -> Rederivation {
        Rederivation { object: obj(n), k: 1, height: 1 }
    }
    fn always(_: &[u8; 32], _: &[u8; 32]) -> bool {
        true
    }
    fn never(_: &[u8; 32], _: &[u8; 32]) -> bool {
        false
    }

    /// A DISAGREEING THIRD ORGAN REFUSES, however late in the walk it sits.
    ///
    /// `state` asked agreement and independence in ONE walk and returned
    /// `Witnessed` from inside it, so the first independent pair ended the
    /// walk and no pair after it was examined. Two organs that agree and are
    /// independent witnessed the object while a third stood there
    /// disagreeing — the re-derivation 13.9 step 8 says the court refuses —
    /// and whether that was caught depended on where the disagreeing organ
    /// happened to sit.
    #[test]
    fn state_refuses_disagreeing_organ_after_independent_pair() {
        let mut c = Convergence::new();
        // The first two agree and are independent: under the old walk this
        // returned `Witnessed` before the third was ever compared.
        assert!(c.rederived(&[9u8], [1; 32], r(9)));
        assert!(c.rederived(&[9u8], [2; 32], r(9)));
        assert!(c.rederived(&[9u8], [3; 32], r(7)));
        assert_eq!(c.organs(&[9u8]).len(), 3);
        assert!(
            matches!(c.state(&[9u8], &always), Converged::Inexact { .. }),
            "the third organ derived another vertex, and the court refuses the object"
        );
    }

    /// One vantage dockets. The object waits for a witness, and waiting is
    /// its own state — not a count that has not reached two.
    #[test]
    fn state_awaits_witness_for_one_organ() {
        let mut c = Convergence::new();
        assert!(c.rederived(&[9u8], [1; 32], r(9)));
        assert_eq!(c.state(&[9u8], &always), Converged::AwaitingWitness);
    }

    /// The same organ twice is one vantage. It was `a_duplicate_voice_does_
    /// not_reach_quorum`; the claim survives the carrier and the thing that
    /// cannot be duplicated is an ORGAN, not a mouth.
    #[test]
    fn rederived_counts_duplicate_organ_once() {
        let mut c = Convergence::new();
        assert!(c.rederived(&[9u8], [1; 32], r(9)));
        assert!(!c.rederived(&[9u8], [1; 32], r(9)), "one organ, one vantage");
        assert_eq!(c.organs(&[9u8]).len(), 1);
        assert_eq!(c.state(&[9u8], &always), Converged::AwaitingWitness);
    }

    /// A1. TWO ORGANS THAT SPELL ONE VERTEX DIFFERENTLY CONVERGE. The test
    /// is re-derivation, not bytes: a versor and its negation are ONE
    /// projective element and two coefficient lists. `form_key_bytes` once
    /// wrote the list, so the two reached different M2 addresses and the
    /// court had to read M1 behind the bytes; it writes the A4 output now
    /// (17.2), so the two presentations reach ONE address and the court's
    /// M1 test and the address agree. An organ that derived the negated
    /// sign holds the same vertex and is not called inexact.
    #[test]
    fn state_converges_differently_spelled_rederivations() {
        use num_rational::BigRational;
        use num_traits::One;
        let v = element::translator_versor(2, 3);
        let neg = v.scale(&-BigRational::one());
        let positive = element::of_versor(v);
        let negated = element::of_versor(neg);
        assert_eq!(
            positive.address(),
            negated.address(),
            "one address (17.2): the codec writes A4, not the sign"
        );
        assert!(positive.same_vertex(&negated), "and M1 agrees with it");

        let mut c = Convergence::new();
        let key = positive.address();
        c.rederived(key.as_bytes(), [1; 32], Rederivation { object: positive, k: 1, height: 1 });
        c.rederived(key.as_bytes(), [2; 32], Rederivation { object: negated, k: 1, height: 1 });
        assert_eq!(
            c.state(key.as_bytes(), &always),
            Converged::Witnessed { home: [1; 32], witness: [2; 32] },
            "two organs, one vertex, two spellings: converged"
        );

        // And a genuinely different vertex still refuses.
        let mut d = Convergence::new();
        d.rederived(&[9u8], [1; 32], r(3));
        d.rederived(&[9u8], [2; 32], r(4));
        assert!(matches!(d.state(&[9u8], &always), Converged::Inexact { .. }));
    }

    /// Two organs reaching one address is the admission condition.
    #[test]
    fn state_witnesses_independent_rederivation() {
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
    fn state_refuses_disagreeing_rederivations_as_inexact() {
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
    fn used_by_records_party_never_witness() {
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
