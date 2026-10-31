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
    /// the court holds the ledger that answers it.
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
