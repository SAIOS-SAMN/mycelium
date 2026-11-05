// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-ANTI-FLATTENING 13, MU-HOSTING §5a.10 — THE RECONSTRUCTED OBJECT:
//! what admission takes, built only by a law's reconstruction.
//!
//! An object is admitted by reconstructing it from its own evidence under
//! the laws, never because a digest it carries equals one held. So every
//! `admit_*` of the receipt path takes a `Reconstructed<T>` and never the
//! claimed `T`. Its one field is private and its only constructors are the
//! reconstructions below, so a value of the type is the proof that the law
//! ran. Outside this module the struct cannot be written
//! (`reconstructed_is_built_only_by_the_laws`):
//!
//! ```compile_fail,E0451
//! fn claimed(r: mycelium::chronometry::KStepReceipt) -> mycelium::court::admission::Reconstructed<mycelium::chronometry::KStepReceipt> {
//!     mycelium::court::admission::Reconstructed { object: r }
//! }
//! ```
//!
//! The lawful path, which compiles:
//!
//! ```
//! fn reconstructed(r: &mycelium::chronometry::KStepReceipt) -> Option<mycelium::court::admission::Reconstructed<mycelium::chronometry::KStepReceipt>> {
//!     mycelium::court::admission::Reconstructed::receipt(r)
//! }
//! ```

use crate::chronometry::{KStepReceipt, Stamp};

/// AN OBJECT RECONSTRUCTED UNDER THE LAWS (ROADMAP track 25, X0): built
/// only by the reconstructions of this module. It reads as the object
/// (`Deref`) and is never written from one.
#[derive(Debug, Clone)]
pub struct Reconstructed<T> {
    object: T,
}

impl<T> Reconstructed<T> {
    /// The object the law reconstructed.
    pub fn object(&self) -> &T {
        &self.object
    }

    /// The object, taken out of the proof: what a carrier stores once
    /// admission has ruled on it.
    pub fn into_object(self) -> T {
        self.object
    }
}

impl<T> std::ops::Deref for Reconstructed<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.object
    }
}

impl Reconstructed<KStepReceipt> {
    /// 78.24, 47.2 — A RECEIPT RECONSTRUCTED FROM ITS OWN COORDINATES: its
    /// chain hash re-derives from its stamp, kind, frame, predecessor,
    /// parents, citations and operator (`KStepReceipt::rederives`). `None`
    /// where it does not: the receipt claims a hash its coordinates do not
    /// give, and nothing admits it.
    pub fn receipt(r: &KStepReceipt) -> Option<Reconstructed<KStepReceipt>> {
        r.rederives().then(|| Reconstructed { object: r.clone() })
    }

    /// A PAGE OF RECEIPTS, reconstructed in order up to the first that does
    /// not re-derive: the reconstructed run, and the stamp of the receipt
    /// that stopped it where one did. A back-fill admits the run and stops
    /// where the page stops (47.2 step 3).
    pub fn receipts(page: &[KStepReceipt]) -> (Vec<Reconstructed<KStepReceipt>>, Option<Stamp>) {
        let mut run = Vec::with_capacity(page.len());
        for r in page {
            match Reconstructed::receipt(r) {
                Some(rec) => run.push(rec),
                None => return (run, Some(r.stamp)),
            }
        }
        (run, None)
    }
}

impl Reconstructed<crate::anastome::dialogue::uplink::SignedFold> {
    /// 78.119, ROADMAP track 25 X2 — A TIER'S FOLD RECONSTRUCTED AT THE
    /// VERTEX: the bytes the tier signs are written out again from the
    /// fold's own coordinates (its tier, prefix head, and the commitments
    /// to its rows, children and handoffs, each re-derived from what the
    /// fold carries; `TierFold::signed_bytes`), and the tier's signature
    /// verifies over them. `None` where it does not: no carried commitment
    /// stands in for the fold, and nothing admits it
    /// (`Court::hear_uplink` answers `Uplinked::Unsigned`).
    ///
    /// The vertex admits only the reconstructed fold; a claimed one does
    /// not compile (`vertex_admits_only_a_reconstructed_fold`):
    ///
    /// ```compile_fail,E0308
    /// fn admitted(k: &mut mycelium::cognition::Court, f: &mycelium::anastome::dialogue::uplink::SignedFold) {
    ///     let _ = k.admit_uplink(f);
    /// }
    /// ```
    ///
    /// The lawful path, which compiles:
    ///
    /// ```
    /// fn admitted(k: &mut mycelium::cognition::Court, f: &mycelium::anastome::dialogue::uplink::SignedFold) {
    ///     if let Some(r) = mycelium::court::admission::Reconstructed::fold(f) {
    ///         let _ = k.admit_uplink(&r);
    ///     }
    /// }
    /// ```
    pub fn fold(f: &crate::anastome::dialogue::uplink::SignedFold) -> Option<Reconstructed<crate::anastome::dialogue::uplink::SignedFold>> {
        crate::court::identity::verify(&f.fold.tier, &f.fold.signed_bytes(), &f.sig).then(|| Reconstructed { object: f.clone() })
    }
}

impl Reconstructed<crate::anastome::dialogue::domain::SignedDomain> {
    /// 78.120, ROADMAP track 25 X2 — A DOMAIN'S GENESIS RECONSTRUCTED AT
    /// THE VERTEX: the bytes the domain signs are written out again from
    /// the genesis's own coordinates (the domain and its tier keys;
    /// `DomainGenesis::signed_bytes`), and the domain's signature verifies
    /// over them. `None` where it does not (`Court::hear_domain` answers
    /// `Founded::Unsigned`).
    pub fn domain(w: &crate::anastome::dialogue::domain::SignedDomain) -> Option<Reconstructed<crate::anastome::dialogue::domain::SignedDomain>> {
        crate::court::identity::verify(&w.genesis.domain, &w.genesis.signed_bytes(), &w.sig).then(|| Reconstructed { object: w.clone() })
    }
}

impl Reconstructed<crate::anastome::dialogue::RootProof> {
    /// 78.147, 47.3 — A FOLD ROOT RECONSTRUCTED FROM THE SIGNATURES OVER
    /// IT, as the fold was sealed: the home's signed position verifies
    /// under the signer's key at `origin` (47.2 steps 1 and 2), the
    /// witness's under another key (`REFUSE_WITNESS_DEPENDENT` otherwise).
    ///
    /// #91: each position signs the root written out (`Root::signed`), so
    /// all three of its coordinates are reconstructed from the signatures.
    /// The place and the height each position states are the proof's,
    /// compared as coordinates, and each position's hash binds the root
    /// (`Commitment::binds`). A payload that is no root, or a root restated
    /// at another place or height or with another hash than the one
    /// signed, is refused `REFUSE_RECEIPT` (a prefix and the receipt it
    /// travels with disagree). A root claimed without both is refused by
    /// name and nothing adopts it.
    pub fn root(
        proof: &crate::anastome::dialogue::RootProof,
        origin: ([u8; 32], u32),
    ) -> Result<Reconstructed<crate::anastome::dialogue::RootProof>, &'static str> {
        use crate::chronometry::Root;
        super::admit_signature(origin, &proof.home.prefix, &proof.signer, &proof.home.sig)?;
        let wk = proof.witness.receipt.stamp.vk;
        if wk == proof.signer {
            return Err(crate::verdict::REFUSE_WITNESS_DEPENDENT);
        }
        super::verify_signature(&proof.witness.prefix, &wk, &proof.witness.sig)?;
        let home = Root::of_signed(&proof.home.prefix.payload).ok_or(crate::verdict::REFUSE_RECEIPT)?;
        let witness = Root::of_signed(&proof.witness.prefix.payload).ok_or(crate::verdict::REFUSE_RECEIPT)?;
        let stated = (proof.root.at, proof.root.height);
        if (home.at, home.height) != stated || (witness.at, witness.height) != stated {
            return Err(crate::verdict::REFUSE_RECEIPT);
        }
        let signed = Reconstructed { object: proof.clone() };
        let binds = |r: &Root| super::Commitment::of_bytes(r.hash.bytes()).binds(&signed);
        match binds(&home) && binds(&witness) {
            true => Ok(signed),
            false => Err(crate::verdict::REFUSE_RECEIPT),
        }
    }
}
