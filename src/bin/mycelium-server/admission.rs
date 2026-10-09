// 47.1, 47.2 — admission: what this court accepts from a peer, in the
// order 47.2 gives, and the tally of what it did with each frame.
//
//! A receipt is admitted against the origin, its signature, its K against
//! the head this court already holds, and its height, in that order. A gap
//! is a gap and not a refusal: the back-fill fetches what makes it re-derive.

use crate::Mesh;
use mycelium::anastome::dialogue;
use mycelium::cognition::{Kernel, Published};
use mycelium::network::nerve::Router;
use std::sync::Mutex;

/// THIS HYPHA'S OWN COVER AND ROOM onto the mesh, read off the published
/// reading (78.153b): its charts and its room as the court stroke last
/// published them, never the kernel — `lane_room`'s credit walk and the
/// cover trie's walk ran under the kernel on every exchange cycle and every
/// digest asked. The room it admitted, `[0; 4]` where none reads.
pub(crate) fn rebuild_own_cover(read: &Published, mesh: &Mesh) -> [u64; 4] {
    // A room that cannot be read is a broken count, and this hypha admits
    // no cover against one.
    let Some(room) = read.lane_room() else { return [0; 4] };
    if let Ok(mut c) = mesh.cover.lock() {
        c.retract_hypha(&mesh.vk);
        for (lane, inv) in read.own_cover_charts() {
            c.admit(*lane, inv, mesh.vk);
        }
    }
    if let Ok(mut r) = mesh.rooms.lock() {
        r.insert(mesh.vk, room);
    }
    if let Ok(mut a) = mesh.addrs.lock() {
        a.insert(mesh.vk, mesh.bind.clone());
    }
    room
}

/// 47.1: THIS HYPHA'S SIGNED POSITION FOR ONE PAYLOAD — the prefix carrying
/// its genesis, epoch, head receipt, K and height, with `payload` as the
/// address it is signing for, the signature over exactly those bytes, and
/// the head receipt the prefix names.
///
/// "Every signature in the tree is over exactly these bytes, in this
/// order" (47.1), so there is one builder and every act uses it. The
/// payload is an ADDRESS and never a tape: two spellings of one word have
/// one receipt lineage, and a receiver re-derives the address from the tape
/// it was sent, so signing the address signs the form by its content and
/// not by the spelling (law 2).
///
/// `chart` is the FRAME's chart (47.1): the axes of the chart the payload
/// lives on — an offered word's — and none for a transmission digest,
/// whose payload is a root over the whole cover. It wrote the hypha's
/// entire active cover here: with a thousand corpus listings that was
/// 10,855 axes and an 86,926-byte prefix, the section's two-byte length
/// wrapped, and every peer refused every digest of this hypha whole, so no
/// origin was ever adopted and no receipt ever admitted (Stage 11 I33,
/// read on the fleet of 2026-09-08).
///
/// `None` when this hypha has adopted no origin or holds no receipt yet: it
/// has no causal position to sign from, and an act it cannot sign is one it
/// does not make. `None` too when the prefix does not fit the wire
/// (`SignedPrefix::fits`): a position that cannot be framed is not signed.
pub(crate) fn sign_position(
    kernel: &Mutex<Kernel>,
    mesh: &Mesh,
    chart: &mycelium::transducer::stalk::Invariant,
    payload: &[u8],
) -> Option<dialogue::ReceiptSection> {
    let (genesis, epoch) = mesh.origin.lock().ok().and_then(|o| *o)?;
    // Read on the kernel, constant in what it holds (one node unpacked):
    // a signed position is the court's head AT SIGNING, so it stays off the
    // shelf, whose head may stand one stroke behind (78.153b).
    let head = crate::held::held!(kernel).ok()?.run(|k, _| k.chain().head())?;
    let prefix = mycelium::court::admission::SignedPrefix {
        genesis,
        epoch,
        prev_receipt_hash: head.prev_hash(),
        at: head.stamp.place(),
        height: head.stamp.h,
        chart: chart.key().as_bytes().to_vec(),
        payload: payload.to_vec(),
    };
    if !prefix.fits() {
        return None;
    }
    let bytes = prefix.bytes()?;
    let sig = {
        use ed25519_dalek::Signer;
        mesh.sk.sign(&bytes).to_bytes()
    };
    Some(dialogue::ReceiptSection { prefix, sig, receipt: head })
}

pub(crate) fn sign_cover_digest(kernel: &Mutex<Kernel>, shelf: &crate::published::Shelf, mesh: &Mesh) -> Vec<u8> {
    // 78.153b — the cover, the room and the ruled root off the published
    // reading; the keeper tick (78.152 TU-1) alone is read on the kernel.
    let read = shelf.take();
    let room = rebuild_own_cover(&read, mesh);
    let ruled_root = read.ruled_root();
    let tick = match crate::held::held!(kernel) {
        Ok(mut h) => h.run(|k, _| k.keeper_tick()),
        Err(_) => return Vec::new(),
    };
    let ring = crate::tshelf::own_ring(mesh, tick);
    // A poisoned cover lock is no reading of the cover; this hypha signs
    // no digest against one (§2.4, rule 11).
    let Ok(mut cover) = mesh.cover.lock() else {
        return Vec::new();
    };
    let cover_roots = cover.roots();
    drop(cover);
    let prefix = dialogue::cover_signed_prefix(&cover_roots, &ruled_root, &room);
    let sig = {
        use ed25519_dalek::Signer;
        mesh.sk.sign(&prefix).to_bytes()
    };
    // A digest whose charts or bind do not fit the lengths it writes is
    // not sent short (47.1): the peer would read a cover this node does
    // not hold and sign its own reading of it.
    // A4: THE DIGEST IS THE ROOTS. The charts are no longer on it — a peer
    // whose roots moved asks for them by page (`ACT_COVER`) — so the only
    // variable field left is the bind, and that is a configuration
    // refusal, named.
    let Some(mut out) =
        dialogue::encode_cover_digest(&room, &cover_roots, &ruled_root, &mesh.vk, &mesh.bind, &ring, &sig)
    else {
        eprintln!(
            "  cover digest refused: the bind does not fit its length, {} bytes",
            mesh.bind.len()
        );
        return Vec::new();
    };
    // 47.1: the signer's causal position travels with its signature. The
    // head receipt and the prefix it is signed over are appended, so the
    // peer can run 47.2 and cite this receipt as a parent of its next step.
    // A node with no genesis sends none: it has no origin to claim. The
    // digest is a root over the whole cover, so its prefix names no chart.
    if let Some(bytes) = sign_position(kernel, mesh, &mycelium::transducer::stalk::Invariant::default(), &ruled_root)
        .and_then(|sec| dialogue::encode_receipt_section(&sec.prefix, &sec.sig, &sec.receipt))
    {
        out.extend(bytes);
    }
    out
}

/// Why one peer's digest was not admitted AT ALL — before any question of
/// a receipt section. `admit_peer_digest` answered a bare `bool`, which is
/// the same rule-11 fault [`NoAdmission`] names one paragraph down: the
/// caller `continue`d on `false` and the exchange tally was never touched,
/// so three courts read `court receipts … all zeros` while every digest
/// was being dropped, and nothing in the journal could tell a refusal
/// from a path that never ran. Measured on the fleet, epoch 10: 480 asks
/// across three hyphae, zero receipts admitted, and no line saying why.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum NoDigest {
    /// The body is not a cover digest — or two readings of it each consume
    /// the buffer, and an ambiguity is a refusal (rule 11).
    Unreadable,
    /// The signature over the cover prefix does not verify under the key
    /// the digest names.
    Signature,
}

/// Why one peer's digest carried no admission. A `None` return with no
/// reason is the rule-11 fault itself: an admission path that cannot say
/// whether it refused, or never ran, is UNMEASURED, and a fleet reading
/// zero admitted receipts cannot tell the two apart.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum NoAdmission {
    /// The peer's digest ends where a section would start: it sent none.
    NoSection,
    /// A section is there and does not parse whole (rule 11: refused).
    Malformed,
    /// This node has adopted no origin, so it has nothing to compare.
    NoOrigin,
}

/// 47.2 on the voice: run admission over the receipt section a peer
/// appended to its digest, and cite what admits.
///
/// The signature (step 2) is checked HERE, over exactly `prefix.bytes()`,
/// before the mind is asked: `Kernel::admit_receipt` is deliberately not
/// given a key, so no call site can satisfy admission by verifying bytes
/// and skipping the chain.
pub(crate) fn admit_peer_receipt(
    kernel: &Mutex<Kernel>,
    mesh: &Mesh,
    body: &[u8],
) -> Result<mycelium::court::admission::Admitted, NoAdmission> {
    use mycelium::court::admission::Admitted;
    let at = dialogue::cover_digest_end(body).ok_or(NoAdmission::Malformed)?;
    if at >= body.len() {
        return Err(NoAdmission::NoSection);
    }
    let dialogue::ReceiptSection { prefix, sig, receipt } =
        dialogue::decode_receipt_section(body, at).ok_or(NoAdmission::Malformed)?;
    // Step 2, as a VALUE the mind is handed (I17): `admit_receipt` takes
    // the verified prefix and not the bare one, so the chain checks and
    // 47.4's burn cannot be reached with bytes nobody verified.
    let verified = match mycelium::court::admission::verify_signature(&prefix, &receipt.stamp.vk, &sig) {
        Ok(v) => v,
        Err(why) => return Ok(Admitted::Refused(why)),
    };
    // 45.5: a node with NO origin adopts the one its bootstrap peer signs
    // for. That is the provisioning channel the chapter names -- the peer
    // list is how a node is told which mesh it belongs to, and the genesis
    // travels the same way. A node that already holds an origin never
    // adopts another: a peer claiming a different one is REFUSE_ORIGIN
    // below, which is the whole point of the check.
    {
        let mut o = mesh.origin.lock().map_err(|_| NoAdmission::Malformed)?;
        if o.is_none() {
            *o = Some((prefix.genesis, prefix.epoch));
            eprintln!(
                "  court genesis {:02x}{:02x}… epoch {} adopted from {:02x}{:02x}…",
                prefix.genesis[0], prefix.genesis[1], prefix.epoch, receipt.stamp.vk[0], receipt.stamp.vk[1]
            );
        }
    }
    let origin = mesh
        .origin
        .lock()
        .ok()
        .and_then(|o| *o)
        .ok_or(NoAdmission::NoOrigin)?;
    let mut h = crate::held::held!(kernel).map_err(|_| NoAdmission::Malformed)?;
    // OWED (78.153b): `recall_cited` reads a whole era per missing parent
    // (the lock audit, row 15), unmetered.
    Ok(h.run(|k, _| k.admit_receipt(origin, verified, &receipt)))
}

/// One exchange cycle's admission tally, written with the 34.2 organ line.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AdmissionTally {
    pub(crate) parent: usize,
    pub(crate) gap: usize,
    pub(crate) stale: usize,
    pub(crate) origin: usize,
    pub(crate) signature: usize,
    pub(crate) receipt: usize,
    pub(crate) equivocation: usize,
    pub(crate) no_section: usize,
    pub(crate) malformed: usize,
    pub(crate) no_origin: usize,
    /// The digest itself was refused, so no receipt was ever looked for.
    pub(crate) digest_unreadable: usize,
    pub(crate) digest_signature: usize,
}

impl AdmissionTally {
    /// THE ONE CLASSIFICATION OF A PEER'S FRAME: counted on this cycle's
    /// line and, on the same arm, on the observer's replication reading
    /// under the signer the digest named (CV5) — one match, so the two
    /// cannot read the frame differently (law 12). The axes are named in
    /// `observe::rep`, beside the one statement of their order.
    pub(crate) fn note(
        &mut self,
        signer: Option<[u8; 32]>,
        r: Result<mycelium::court::admission::Admitted, NoAdmission>,
    ) {
        use mycelium::court::admission::Admitted;
        use mycelium::observe::rep;
        let axis = match r {
            Ok(Admitted::Parent(_)) => {
                self.parent += 1;
                rep::PARENT
            }
            // A gap in one signer's chain and a citation of another signer
            // this node does not hold are both holes, and the back-fill
            // chases both; the cycle's line reads them as one, and the
            // observer apart (2026-10-08, no silent verdict: the merged
            // count hid hypha c's isolation).
            Ok(Admitted::Gap { .. }) => {
                self.gap += 1;
                rep::GAP
            }
            Ok(Admitted::MissingParent(_)) => {
                self.gap += 1;
                rep::MISSING_PARENT
            }
            Ok(Admitted::Stale) => {
                self.stale += 1;
                rep::STALE
            }
            Ok(Admitted::Equivocation { .. }) => {
                self.equivocation += 1;
                rep::EQUIVOCATION
            }
            Ok(Admitted::Refused(why)) => {
                if why == mycelium::court::admission::REFUSE_ORIGIN {
                    self.origin += 1;
                    rep::REFUSED_ORIGIN
                } else if why == mycelium::court::admission::REFUSE_RECEIPT {
                    self.receipt += 1;
                    rep::REFUSED_RECEIPT
                } else {
                    self.signature += 1;
                    rep::REFUSED_SIGNATURE
                }
            }
            Err(NoAdmission::NoSection) => {
                self.no_section += 1;
                rep::NO_SECTION
            }
            Err(NoAdmission::Malformed) => {
                self.malformed += 1;
                rep::MALFORMED
            }
            Err(NoAdmission::NoOrigin) => {
                self.no_origin += 1;
                rep::NO_ORIGIN
            }
        };
        mycelium::observe::note_replication(signer, axis, 1);
    }

    /// A digest refused before any receipt could be read out of it. No
    /// signer can be read off a digest that did not open, so it is counted
    /// unattributed.
    pub(crate) fn note_digest(&mut self, r: Result<(), NoDigest>) {
        use mycelium::observe::rep;
        let axis = match r {
            Ok(()) => return,
            Err(NoDigest::Unreadable) => {
                self.digest_unreadable += 1;
                rep::DIGEST_UNREADABLE
            }
            Err(NoDigest::Signature) => {
                self.digest_signature += 1;
                rep::DIGEST_SIGNATURE
            }
        };
        mycelium::observe::note_replication(None, axis, 1);
    }

    pub(crate) fn line(&self) -> String {
        format!(
            "  court receipts parent {} gap {} stale {} equivocation {} \
refused-origin {} refused-signature {} refused-receipt {} \
none {} malformed {} no-origin {} digest-unreadable {} digest-signature {}",
            self.parent,
            self.gap,
            self.stale,
            self.equivocation,
            self.origin,
            self.signature,
            self.receipt,
            self.no_section,
            self.malformed,
            self.no_origin,
            self.digest_unreadable,
            self.digest_signature
        )
    }
}

pub(crate) fn admit_peer_digest(
    mesh: &Mesh,
    body: &[u8],
    router: &Mutex<Router>,
) -> Result<(), NoDigest> {
    let Some(dialogue::CoverDigest { room, cover_roots, ruled_root, vk, bind: addr, ring, sig }) =
        dialogue::decode_cover_digest(body)
    else {
        return Err(NoDigest::Unreadable);
    };
    let prefix = dialogue::cover_signed_prefix(&cover_roots, &ruled_root, &room);
    if !mycelium::court::identity::verify(&vk, &prefix, &sig) {
        return Err(NoDigest::Signature);
    }
    // 78.152 TU-2: the signer's keeper ring, verified on admission —
    // signature, parent chain, cites — and the door's final point raised
    // where the quorum moved it (TU-3). Each reading is signed on its own,
    // so the ring stands apart from the digest's signature. 78.152b: the
    // ring also travels on its own act (`ACT_TICK`); this one stays beside
    // it, and its outcome is counted and journalled where it is heard.
    let _ = crate::tshelf::hear_ring(mesh, vk, ring);
    // The digest names the peer's room, its address and its ROOTS. Its
    // charts are read by page when those roots move (`admit_peer_cover`),
    // so nothing here retracts a cover this reading cannot replace.
    if let Ok(mut r) = mesh.rooms.lock() {
        r.insert(vk, room);
    }
    if let Ok(mut a) = mesh.addrs.lock() {
        a.insert(vk, addr.clone());
    }
    if let Ok(mut r) = router.lock() {
        r.insert(vk, addr);
    }
    Ok(())
}
