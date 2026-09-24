// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 47.2 — ADMISSION: the order of checks a signed frame passes before
//! any carrier is touched, and the per-signer heads they read.

use super::{
    SignedPrefix, REFUSE_EQUIVOCATION, REFUSE_NOT_EQUIVOCATION, REFUSE_ORIGIN, REFUSE_RECEIPT,
    REFUSE_RECEIPT_STALE, REFUSE_SIGNATURE,
};
use crate::chronometry::{KStepReceipt, Parent, Place, ReceiptChain};

/// What 47.2 answers for one signed frame. Every arm is an instruction to
/// the caller, and only `Parent` permits a carrier to be touched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Admitted {
    /// The receipt names the signer's head this node holds as its
    /// predecessor: admit, and cite this receipt as a parent of the node's
    /// next receipt.
    Parent(Parent),
    /// The receipt's predecessor stands past the signer's head this node
    /// holds (or it holds none and the receipt is not the line's first):
    /// the signer's line has a hole from the place after `from` up to
    /// `to`, the predecessor's place — what a back-fill must reach. The
    /// frame dockets as a pending citation does and NOTHING is acted on
    /// until it closes.
    Gap { vk: [u8; 32], from: Option<Place>, to: Place },
    /// A duplicate frame at a place already held with the same hash.
    Stale,
    /// Two hashes at one place on one line (47.4).
    Equivocation { at: Place, held: [u8; 32], claimed: [u8; 32] },
    /// A CAUSAL HOLE ACROSS SIGNERS: the receipt re-derives its height
    /// from the predecessor and every parent it cites, and one of those
    /// citations names a receipt of ANOTHER signer this node does not
    /// hold. That is not a refusal — the frame is well formed and its
    /// author is not at fault — it is the mesh's own shape.
    ///
    /// THE MESH IS A POSET, NOT A LOG. This arm was `Refused(REFUSE_
    /// RECEIPT)`, and `admit_backfill` broke on it, so a back-fill that
    /// paged one signer's linear chain died the moment that chain cited a
    /// third party. Measured on the fleet at epoch 10: `court receipt
    /// backfill ce02… 940..940 admitted 0`, fifty-two times, forever — the
    /// gap could not close by construction, so no peer receipt was ever
    /// admitted as a parent and no docket entry could reach a witness.
    /// Answering the missing citation instead lets the caller chase it.
    MissingParent(Parent),
    /// Refused before any carrier was touched, with the reason.
    Refused(&'static str),
}

impl Admitted {
    /// The 34.1 constant this arm answers a reply with: a duplicate frame
    /// is `REFUSE_RECEIPT_STALE`, an equivocation proof `REFUSE_EQUIVOCATION`,
    /// a refusal its own reason; a parent or a gap refuses nothing.
    pub fn reason(&self) -> Option<&'static str> {
        match self {
            Admitted::Parent(_) | Admitted::Gap { .. } | Admitted::MissingParent(_) => None,
            Admitted::Stale => Some(REFUSE_RECEIPT_STALE),
            Admitted::Equivocation { .. } => Some(REFUSE_EQUIVOCATION),
            Admitted::Refused(why) => Some(why),
        }
    }
}

/// 47.2 STEPS 1 AND 2 for any signed frame: the origin is this node's, and
/// the signature verifies over 47.1's bytes and nothing else. Steps 3 and 4
/// need the signer's chain and are `Admissions::admit`.
///
/// The order is the chapter's and it matters: a frame from another mesh is
/// refused before any bytes are verified, so no carrier and no table is read
/// on behalf of a frame that is not addressed to this mesh at all.
///
/// THERE IS NO "KNOWN SIGNER" TEST HERE. This carried one, refusing a key
/// the mesh had not admitted with a refusal that is not in the chapter. 47.2
/// step 2 verifies under the claimed key; a key this node has never seen is
/// step 3's first sighting, which back-fills from that signer's genesis, and
/// what admits a vantage is room, not identity — identities are free (47.3).
/// Refusing an unknown key here would refuse every new hypha before its
/// first receipt could ever be admitted.
pub fn admit_signature<'a>(
    origin: ([u8; 32], u32),
    prefix: &'a SignedPrefix,
    signer: &[u8; 32],
    sig: &[u8; 64],
) -> Result<Verified<'a>, &'static str> {
    if prefix.genesis != origin.0 || prefix.epoch != origin.1 {
        return Err(REFUSE_ORIGIN);
    }
    verify_signature(prefix, signer, sig)
}

/// A SIGNATURE THAT VERIFIED, as a value (I17). `Verified` is built here
/// and nowhere else, so a caller holding one has run 47.2 step 2 over
/// exactly these prefix bytes under exactly this key. `Kernel::admit_receipt`
/// takes it rather than the bare prefix: the `Equivocation` arm burns a
/// bond (47.4), and 47.4's condition is two VERIFIED signatures — the held
/// receipt's when it was admitted, the claimed one's now — which used to
/// hold by caller discipline alone, and a test caller reached the burn with
/// no signature at all.
#[derive(Debug, Clone, Copy)]
pub struct Verified<'a> {
    prefix: &'a SignedPrefix,
    signer: [u8; 32],
}

impl<'a> Verified<'a> {
    /// The prefix the signature was verified over.
    pub fn prefix(&self) -> &'a SignedPrefix {
        self.prefix
    }

    /// The key it verified under.
    pub fn signer(&self) -> [u8; 32] {
        self.signer
    }
}

/// 47.2 STEP 2 alone: the signature verifies over 47.1's bytes under the
/// claimed key, or `REFUSE_SIGNATURE`. The digest path runs this before it
/// has an origin to compare — a node with none adopts the one its bootstrap
/// peer signs for (45.5), and only a signed prefix may provision it — and
/// `Admissions::admit` runs step 1 afterwards.
pub fn verify_signature<'a>(
    prefix: &'a SignedPrefix,
    signer: &[u8; 32],
    sig: &[u8; 64],
) -> Result<Verified<'a>, &'static str> {
    if !crate::court::identity::verify(signer, &prefix.bytes(), sig) {
        return Err(REFUSE_SIGNATURE);
    }
    Ok(Verified { prefix, signer: *signer })
}

/// The prefix and the receipt are ONE claim: the prefix's place, height
/// and predecessor are the receipt's stamp and predecessor.
fn one_claim(p: &SignedPrefix, r: &KStepReceipt) -> bool {
    p.at == r.stamp.place() && p.height == r.stamp.h && p.prev_receipt_hash == r.prev_hash()
}

/// 47.4's PROOF, DECIDED: one signer, one place, two chain hashes, both
/// signatures verified. What the court burns against, and all it needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Equivocated {
    pub vk: [u8; 32],
    pub at: Place,
}

/// 47.4 ON A THIRD PARTY'S PRESENTATION (`ACT_EQUIVOCATION`, I21): two
/// receipt sections it holds are an equivocation proof iff both signatures
/// verified (the two `Verified`, 47.2 step 2, the caller's), each receipt
/// is one claim with its prefix and its signer — the key that verified is
/// the key the receipt names, the prefix's place, height and predecessor are
/// the receipt's, and the receipt re-derives its own chain hash — and the
/// two share key and place and differ in hash. Two keys, two places, or one hash (a
/// duplicate frame) are not a proof and are `REFUSE_NOT_EQUIVOCATION`; a
/// half that disagrees with itself is `REFUSE_RECEIPT`, as `admit` refuses
/// the same disagreement on a single frame. The court replays nothing: the
/// signatures decide (T17).
pub fn admit_equivocation(
    a: Verified<'_>,
    ra: &KStepReceipt,
    b: Verified<'_>,
    rb: &KStepReceipt,
) -> Result<Equivocated, &'static str> {
    for (v, r) in [(&a, ra), (&b, rb)] {
        if v.signer() != r.stamp.vk || !one_claim(v.prefix(), r) || !r.rederives() {
            return Err(REFUSE_RECEIPT);
        }
    }
    if ra.stamp.vk != rb.stamp.vk || ra.stamp.place() != rb.stamp.place() || ra.chain_hash == rb.chain_hash {
        return Err(REFUSE_NOT_EQUIVOCATION);
    }
    Ok(Equivocated { vk: ra.stamp.vk, at: ra.stamp.place() })
}

/// 47.2, in order, before any carrier is touched, read against the ledger
/// this node holds (78.24): a signer's head is its line's greatest place,
/// and what is held at a place is the node on its line — so what admission knows
/// is what the ledger holds, persisted with it and never beside it. `origin`
/// is this node's own `(H_genesis, E_origin)`.
///
/// Step 2 (the signature) is the caller's: it holds the key and the bytes.
/// This is steps 1, 3 and 4, and it is deliberately not given a signature
/// to check so that no call site can satisfy admission by verifying bytes
/// alone.
pub fn admit(origin: ([u8; 32], u32), prefix: &SignedPrefix, r: &KStepReceipt, ledger: &ReceiptChain) -> Admitted {
    // 1. Origin, before anything else.
    if prefix.genesis != origin.0 || prefix.epoch != origin.1 {
        return Admitted::Refused(REFUSE_ORIGIN);
    }
    // The prefix and the receipt are ONE claim: a signer may not sign one
    // position and send another.
    if !one_claim(prefix, r) {
        return Admitted::Refused(REFUSE_RECEIPT);
    }
    admit_chain(r, ledger)
}

/// 47.2 STEPS 3 AND 4 for a receipt fetched by place range (S13.10): it
/// carries no signature of its own and is admitted by its chain — it
/// re-derives its own hash, links to the head this node holds for the
/// signer, and its height re-derives. Nothing admitted here is cited as a
/// parent; the signed frame that closes the gap is.
pub fn admit_backfilled(r: &KStepReceipt, ledger: &ReceiptChain) -> Admitted {
    if !r.rederives() {
        return Admitted::Refused(REFUSE_RECEIPT);
    }
    admit_chain(r, ledger)
}

fn admit_chain(r: &KStepReceipt, ledger: &ReceiptChain) -> Admitted {
    let (vk, at) = (r.stamp.vk, r.stamp.place());
    // The line's continuity is the predecessor the receipt NAMES: its own
    // signer's, at a place this one follows — the next act of its orbit or
    // the first of the next. Only the line's first receipt names none.
    match &r.prev {
        None if at != Place::GENESIS => return Admitted::Refused(REFUSE_RECEIPT),
        Some(prev) if prev.at.vk != vk || !prev.at.place().is_followed_by(at) => {
            return Admitted::Refused(REFUSE_RECEIPT);
        }
        _ => {}
    }
    // 3. The place against the signer's head. This runs BEFORE the height
    // check, in 47.2's own order: a gapped frame's height cannot re-derive
    // here yet — what would make it re-derive is exactly what the back-fill
    // fetches.
    match (ledger.line_head(&vk), &r.prev) {
        (None, Some(prev)) => {
            // First sighting of a signer, mid-line: back-fill from its
            // genesis up to the predecessor.
            return Admitted::Gap { vk, from: None, to: prev.at.place() };
        }
        (Some(head), Some(prev)) if prev.at.place() > head.stamp.place() => {
            return Admitted::Gap { vk, from: Some(head.stamp.place()), to: prev.at.place() };
        }
        (Some(head), _) if r.prev != Some(Parent { at: head.stamp, hash: head.chain_hash }) => {
            // At or below the head. The node held at that place tells a
            // duplicate from an equivocation; nothing held there is stale
            // TO US.
            return match ledger.at(&vk, at) {
                Some(held) if held.chain_hash == r.chain_hash => Admitted::Stale,
                Some(held) => Admitted::Equivocation { at, held: held.chain_hash, claimed: r.chain_hash },
                None => Admitted::Stale,
            };
        }
        _ => {}
    }
    // The receipt names the head this node holds: its place is the one the
    // head's kind says follows it — the next orbit after a close, the next
    // act of the orbit otherwise — and a close carries its orbit's frame,
    // re-derived from the acts held, as the height is (OK1).
    if let Some(held) = r.prev.and_then(|p| ledger.node(&p.at)) {
        if held.stamp.place().after(held.kind) != at {
            return Admitted::Refused(REFUSE_RECEIPT);
        }
    }
    if !ledger.close_rederives(r) {
        return Admitted::Refused(REFUSE_RECEIPT);
    }
    // 4. The height on the frame is a CLAIM: it re-derives from the
    // predecessor and the parents the receipt names, each of which this
    // node must hold at its stamp with its hash. An unheld one is a HOLE,
    // not a fault — the answer names it so it can be fetched (47.2 step 3).
    // 45.2 (HT1): `1 + max` over the predecessor and the parents, `0` only
    // with neither — a line's first receipt citing parents stands above them.
    let mut cited: Vec<u64> = Vec::with_capacity(r.parents.len() + 1);
    match &r.prev {
        None => {}
        Some(prev) => {
            let Some(h) = ledger.height_of(prev) else {
                return Admitted::MissingParent(*prev);
            };
            cited.push(h);
        }
    }
    for p in &r.parents {
        match ledger.height_of(p) {
            Some(h) => cited.push(h),
            None => return Admitted::MissingParent(*p),
        }
    }
    let want = crate::chronometry::height_over(&cited);
    if r.stamp.h != want {
        return Admitted::Refused(REFUSE_RECEIPT);
    }
    Admitted::Parent(Parent { at: r.stamp, hash: r.chain_hash })
}
