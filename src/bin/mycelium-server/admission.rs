// 47.1, 47.2 — admission: what this court accepts from a peer, in the
// order 47.2 gives, and the tally of what it did with each frame.
//
//! A receipt is admitted against the origin, its signature, its K against
//! the head this court already holds, and its height, in that order. A gap
//! is a gap and not a refusal: the back-fill fetches what makes it re-derive.

use crate::Mesh;
use mycelium::anastome::dialogue;
use mycelium::cognition::Kernel;
use mycelium::network::nerve::Router;
use std::sync::Mutex;

pub(crate) fn room64(r: [usize; 4]) -> [u64; 4] {
    std::array::from_fn(|i| r[i] as u64)
}

pub(crate) fn rebuild_own_cover(kernel: &Mutex<Kernel>, mesh: &Mesh) -> (Vec<(u8, mycelium::transducer::stalk::Invariant)>, [u64; 4]) {
    let (charts, room) = match kernel.lock() {
        Ok(k) => (k.own_cover_charts(), room64(k.lane_room())),
        Err(_) => return (Vec::new(), [0; 4]),
    };
    if let Ok(mut c) = mesh.cover.lock() {
        c.retract_hypha(&mesh.vk);
        for (lane, inv) in &charts {
            c.admit(*lane, inv, mesh.vk);
        }
    }
    if let Ok(mut r) = mesh.rooms.lock() {
        r.insert(mesh.vk, room);
    }
    if let Ok(mut a) = mesh.addrs.lock() {
        a.insert(mesh.vk, mesh.bind.clone());
    }
    (charts, room)
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
    chart: &[u64],
    payload: &[u8],
) -> Option<dialogue::ReceiptSection> {
    let (genesis, epoch) = mesh.origin.lock().ok().and_then(|o| *o)?;
    let k = kernel.lock().ok()?;
    let head = k.chain().receipts().last().cloned()?;
    drop(k);
    let prefix = mycelium::court::admission::SignedPrefix {
        genesis,
        epoch,
        prev_receipt_hash: head.prev_receipt_hash,
        k: head.step_index,
        height: head.height,
        chart: chart.to_vec(),
        payload: payload.to_vec(),
    };
    if !prefix.fits() {
        return None;
    }
    let sig = {
        use ed25519_dalek::Signer;
        mesh.sk.sign(&prefix.bytes()).to_bytes()
    };
    Some(dialogue::ReceiptSection { prefix, sig, receipt: head })
}

pub(crate) fn sign_cover_digest(kernel: &Mutex<Kernel>, mesh: &Mesh) -> Vec<u8> {
    let (charts, room) = rebuild_own_cover(kernel, mesh);
    let ruled_root = match kernel.lock() {
        Ok(k) => k.ruled_root(),
        Err(_) => return Vec::new(),
    };
    let cover_roots = mesh.cover.lock().map(|c| c.roots()).unwrap_or([[0u8; 32]; 4]);
    let prefix = dialogue::cover_signed_prefix(&cover_roots, &ruled_root, &room);
    let sig = {
        use ed25519_dalek::Signer;
        mesh.sk.sign(&prefix).to_bytes()
    };
    let mut out =
        dialogue::encode_cover_digest(&charts, &room, &cover_roots, &ruled_root, &mesh.vk, &mesh.bind, &sig);
    // 47.1: the signer's causal position travels with its signature. The
    // head receipt and the prefix it is signed over are appended, so the
    // peer can run 47.2 and cite this receipt as a parent of its next step.
    // A node with no genesis sends none: it has no origin to claim. The
    // digest is a root over the whole cover, so its prefix names no chart.
    if let Some(bytes) = sign_position(kernel, mesh, &[], &ruled_root)
        .and_then(|sec| dialogue::encode_receipt_section(&sec.prefix, &sec.sig, &sec.receipt))
    {
        out.extend(bytes);
    }
    out
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
    let verified = match mycelium::court::admission::verify_signature(&prefix, &receipt.vk, &sig) {
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
                prefix.genesis[0], prefix.genesis[1], prefix.epoch, receipt.vk[0], receipt.vk[1]
            );
        }
    }
    let origin = mesh
        .origin
        .lock()
        .ok()
        .and_then(|o| *o)
        .ok_or(NoAdmission::NoOrigin)?;
    let mut k = kernel.lock().map_err(|_| NoAdmission::Malformed)?;
    Ok(k.admit_receipt(origin, verified, &receipt))
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
}

impl AdmissionTally {
    pub(crate) fn note(&mut self, r: Result<mycelium::court::admission::Admitted, NoAdmission>) {
        use mycelium::court::admission::Admitted;
        match r {
            Ok(Admitted::Parent(_)) => self.parent += 1,
            Ok(Admitted::Gap { .. }) => self.gap += 1,
            Ok(Admitted::Stale) => self.stale += 1,
            Ok(Admitted::Equivocation { .. }) => self.equivocation += 1,
            Ok(Admitted::Refused(why)) => {
                if why == mycelium::court::admission::REFUSE_ORIGIN {
                    self.origin += 1;
                } else if why == mycelium::court::admission::REFUSE_RECEIPT {
                    self.receipt += 1;
                } else {
                    self.signature += 1;
                }
            }
            Err(NoAdmission::NoSection) => self.no_section += 1,
            Err(NoAdmission::Malformed) => self.malformed += 1,
            Err(NoAdmission::NoOrigin) => self.no_origin += 1,
        }
    }

    pub(crate) fn line(&self) -> String {
        format!(
            "  court receipts parent {} gap {} stale {} equivocation {} \
refused-origin {} refused-signature {} refused-receipt {} \
none {} malformed {} no-origin {}",
            self.parent,
            self.gap,
            self.stale,
            self.equivocation,
            self.origin,
            self.signature,
            self.receipt,
            self.no_section,
            self.malformed,
            self.no_origin
        )
    }
}

pub(crate) fn admit_peer_digest(mesh: &Mesh, body: &[u8], router: &Mutex<Router>) -> bool {
    let Some(dialogue::CoverDigest { charts, room, cover_roots, ruled_root, vk, bind: addr, sig }) =
        dialogue::decode_cover_digest(body)
    else {
        return false;
    };
    let prefix = dialogue::cover_signed_prefix(&cover_roots, &ruled_root, &room);
    if !mycelium::court::identity::verify(&vk, &prefix, &sig) {
        return false;
    }
    if let Ok(mut c) = mesh.cover.lock() {
        c.retract_hypha(&vk);
        for (lane, inv) in &charts {
            c.admit(*lane, inv, vk);
        }
    }
    if let Ok(mut r) = mesh.rooms.lock() {
        r.insert(vk, room);
    }
    if let Ok(mut a) = mesh.addrs.lock() {
        a.insert(vk, addr.clone());
    }
    if let Ok(mut r) = router.lock() {
        r.insert(vk, addr);
    }
    true
}
