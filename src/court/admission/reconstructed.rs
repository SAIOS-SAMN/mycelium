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
    /// verifies over them. Refused `Uplinked::Unsigned` where it does not:
    /// no carried commitment stands in for the fold, and nothing admits it.
    ///
    /// MU-HOSTING §5a.3, ROADMAP track 24 B3 — and REBUILT UNDER THE
    /// UNIVERSAL LAWS: every transition the fold states, and every one a
    /// line it carries states (at any depth), is verified under L0 at
    /// `LAW_VERSION` on its own coordinates (`laws::verify_sized`, the
    /// verifier a kernel asks on `ACT_LAWS` mode 1). A transition that
    /// breaks a law is refused `Uplinked::NotLawful`, naming where it stands
    /// and each breach with its law; one with a law unread is answered
    /// `Uplinked::Unread`, neither admitted nor refused. A fold names no law
    /// it was computed under, so none is admitted on a declared name.
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
    ///     if let Ok(r) = mycelium::court::admission::Reconstructed::fold(f) {
    ///         let _ = k.admit_uplink(&r);
    ///     }
    /// }
    /// ```
    pub fn fold(
        f: &crate::anastome::dialogue::uplink::SignedFold,
    ) -> Result<Reconstructed<crate::anastome::dialogue::uplink::SignedFold>, crate::anastome::dialogue::uplink::Uplinked> {
        if !crate::court::identity::verify(&f.fold.tier, &f.fold.signed_bytes(), &f.sig) {
            return Err(crate::anastome::dialogue::uplink::Uplinked::Unsigned);
        }
        lawful(&f.fold, &mut Vec::new())?;
        Ok(Reconstructed { object: f.clone() })
    }
}

/// B3 — EVERY TRANSITION A FOLD STATES, AND ITS CARRIED LINES', reconstructed
/// under L0: the first that breaks a law refused `NotLawful` with every
/// breach it carries, the first with a law unread answered `Unread`; the
/// fold's own before its lines', each line in its order.
fn lawful(fold: &crate::anastome::dialogue::uplink::TierFold, path: &mut Vec<usize>) -> Result<(), crate::anastome::dialogue::uplink::Uplinked> {
    use crate::anastome::dialogue::uplink::{Child, Uplinked};
    for (index, t) in fold.transitions.iter().enumerate() {
        let v = crate::physics::laws::verify_sized(t);
        if !v.breaches.is_empty() {
            return Err(Uplinked::NotLawful { path: path.clone(), index, breaches: v.breaches });
        }
        if !v.unmeasured.is_empty() {
            return Err(Uplinked::Unread { path: path.clone(), index, unmeasured: v.unmeasured });
        }
    }
    for (i, c) in fold.children.iter().enumerate() {
        if let Child::Carried(line) = &c.child {
            path.push(i);
            lawful(&line.fold, path)?;
            path.pop();
        }
    }
    Ok(())
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

impl Reconstructed<crate::physics::laws::clauses::Signing> {
    /// CONCORDANCE §0, ROADMAP track 24 B2 — A KERNEL'S SIGNING ACT
    /// RECONSTRUCTED: the bytes the kernel signs are written out again from
    /// the act's own coordinates (the domain, its key and the clauses;
    /// `Signing::signed_bytes`), and the signature verifies over them under
    /// the kernel's key. Refused `Malformed` where the key is no Ed25519
    /// point, `Unsigned` where the signature does not verify; only a
    /// reconstructed act is judged and recorded (`clauses::judge`).
    pub fn signing(s: &crate::physics::laws::clauses::Signing) -> Result<Reconstructed<crate::physics::laws::clauses::Signing>, crate::physics::laws::clauses::SigningRefusal> {
        use crate::physics::laws::clauses::SigningRefusal;
        use ed25519_dalek::Verifier;
        let key = ed25519_dalek::VerifyingKey::from_bytes(&s.kernel_key).map_err(|_| SigningRefusal::Malformed)?;
        let sig = ed25519_dalek::Signature::from_bytes(&s.sig);
        key.verify(&s.signed_bytes(), &sig).map_err(|_| SigningRefusal::Unsigned)?;
        Ok(Reconstructed { object: s.clone() })
    }
}

impl Reconstructed<crate::cognition::Court> {
    /// MU-ANTI-FLATTENING 14, ROADMAP track 25 X0 (option (a), ruled on #28,
    /// 2026-11-06) — A COURT RECONSTRUCTED FROM ITS SNAPSHOT AND CHECKED
    /// AGAINST ITS RECEIPTS, never replayed and never restored in place.
    /// `held` carries what the snapshot is read against: the line and the
    /// eras bound from disk, or nothing where the line rides the snapshot.
    /// `held` is taken by value: the held line is the evidence the snapshot
    /// is checked against, and a court half-restored by a refused snapshot
    /// is consumed with the refusal, never returned (approved on #190).
    /// The laws, each of which refuses the whole snapshot:
    /// - every receipt the snapshot carries re-derives its own chain hash
    ///   (`KStepReceipt::rederives`), and the line verifies whole on its
    ///   root (`ReceiptChain::from_nodes`);
    /// - a snapshot standing on a held line names a stamp that line holds,
    ///   or one it has folded past;
    /// - every ledger row continues the receipt at its stamp
    ///   (`Predecessor::continues`), the fold root's at the root;
    /// - every word re-derives the citation it is filed under.
    ///
    /// Everything derived (balances, the ruled stalks, the axis index, the
    /// federation's order, the rank's ordinal, the depth index) is
    /// re-derived from what was read, never restored. The claim
    /// `restore_reconstructs_and_checks_receipts`.
    pub fn court(mut held: crate::cognition::Court, b: &[u8]) -> Result<Reconstructed<crate::cognition::Court>, crate::cognition::RestoreRefusal> {
        held.read_snapshot(b)?;
        Ok(Reconstructed { object: held })
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
