// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 47.2 — ADMISSION: the order of checks a signed frame passes before
//! any carrier is touched, and the per-signer heads they read.

use super::{SignedPrefix, REFUSE_EQUIVOCATION, REFUSE_ORIGIN, REFUSE_RECEIPT, REFUSE_RECEIPT_STALE, REFUSE_SIGNATURE};
use crate::chronometry::{KStepReceipt, Parent};
use crate::network::nerve::{Keyed, ROOT};

/// What 47.2 answers for one signed frame. Every arm is an instruction to
/// the caller, and only `Parent` permits a carrier to be touched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Admitted {
    /// `K = K_h + 1` and `prev = H_h`: admit, and cite this receipt as a
    /// parent of the node's next receipt.
    Parent(Parent),
    /// `K > K_h + 1`: the signer's chain has a hole. The frame dockets as a
    /// pending citation does and NOTHING is acted on until it closes.
    Gap { vk: [u8; 32], from: u64, to: u64 },
    /// A duplicate frame at a K already held with the same hash.
    Stale,
    /// Two hashes at one K on one chain (47.4).
    Equivocation { k: u64, held: [u8; 32], claimed: [u8; 32] },
    /// Refused before any carrier was touched, with the reason.
    Refused(&'static str),
}

impl Admitted {
    /// The 34.1 constant this arm answers a reply with: a duplicate frame
    /// is `REFUSE_RECEIPT_STALE`, an equivocation proof `REFUSE_EQUIVOCATION`,
    /// a refusal its own reason; a parent or a gap refuses nothing.
    pub fn reason(&self) -> Option<&'static str> {
        match self {
            Admitted::Parent(_) | Admitted::Gap { .. } => None,
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

/// THE BINDINGS 47.1 FIXES, for a ruling that travels with its receipt: the
/// prefix's payload is the ruled ADDRESS, its prev hash is the head the
/// ruling binds to, and the receipt names the very key the act claims as
/// its court. Without the first two a signature proves only that the court
/// signed SOMETHING — a frame could carry a valid signature over one
/// position and a ruling on another address under it.
///
/// ONE SIGNER, AND ONE ONLY. Without the third, 47.2 checks two different
/// keys: step 2 verifies "under the claimed key" while step 3 checks "`K`
/// against the signer's known head", and `Admissions::admit` reads that
/// head at `receipt.vk`. A frame carrying a signature that verifies under
/// its own key and a receipt naming a VICTIM's key therefore passed step 2
/// on the sender's key and wrote step 3's state under the victim's, so one
/// frame per victim poisoned that victim's head: its next genuine receipt
/// then reads as a `Gap` that no back-fill can close, or as an
/// `Equivocation`, which 47.4 answers with a bond burn. The prefix and the
/// receipt are one claim, and so is the voice that carries them; a
/// disagreement between halves of one claim is `REFUSE_RECEIPT`, as the
/// prefix-versus-receipt disagreement in `admit` already is.
///
/// What the prefix does not cover is the ruling byte; 47.1 fixes the signed
/// bytes exactly, so a verdict binds the court's position and the word it
/// ruled on, and a court that means to bind the ruling too must move its own
/// K for each one.
pub fn admit_ruled(
    prefix: &SignedPrefix,
    receipt: &KStepReceipt,
    claimed_court: &[u8; 32],
    ruled: &[u8],
    head: [u8; 32],
) -> Result<(), &'static str> {
    if prefix.payload != ruled || prefix.prev_receipt_hash != head {
        return Err(REFUSE_RECEIPT);
    }
    if receipt.vk != *claimed_court {
        return Err(REFUSE_RECEIPT);
    }
    Ok(())
}

/// WHAT A RULING CARRIES BESIDE ITS BYTE (42): the selection a REBIND
/// rebinds to, the base and transport an ATTACH attaches by, or nothing.
/// Read off the act once, by `admit_payload`, so the door matches one
/// total value and no arm is left to answer for a payload that is not
/// there.
#[derive(Debug, Clone)]
pub enum Payload {
    Rebind(crate::transducer::anchor::Sel),
    Attach(crate::transducer::address::Address, Box<crate::transducer::element::Versor>),
    Plain,
}

/// THE ACT'S FIELDS AGREE WITH ITS RULING, or the act is malformed (34.1
/// `REFUSE_MALFORMED`, I18). A `RULING_REBIND` that carries no selection
/// and a `RULING_ATTACHED` that carries no base and transport name a
/// payload the frame does not hold; the door refuses them by that name
/// before any carrier is touched, never as `Inert` (a ruling that found
/// nothing to move) and never as `REFUSE_FORM_TAPE` (a form whose tape does
/// not re-derive), which is what B1 answered with.
pub fn admit_payload(v: &crate::anastome::dialogue::Verdict) -> Result<Payload, &'static str> {
    use crate::anastome::dialogue::{RULING_ATTACHED, RULING_REBIND};
    match (v.ruling, &v.pi, &v.attach) {
        (RULING_REBIND, Some(term), _) => Ok(Payload::Rebind(term.clone())),
        (RULING_REBIND, None, _) => Err(super::REFUSE_MALFORMED),
        (RULING_ATTACHED, _, Some((base, g))) => {
            Ok(Payload::Attach(base.clone(), Box::new(g.clone())))
        }
        (RULING_ATTACHED, _, None) => Err(super::REFUSE_MALFORMED),
        _ => Ok(Payload::Plain),
    }
}

/// Per-signer chain heads: vk ↦ (head K, head chain hash). A coordinate
/// read at the signer's own key, never a scan over receipts.
#[derive(Debug, Default, Clone)]
pub struct Admissions {
    heads: Keyed<(u64, [u8; 32])>,
    /// Receipts held at a signer's K, so a duplicate is told from an
    /// equivocation by the hash rather than by the count.
    at_k: Keyed<Keyed<[u8; 32]>>,
}

impl Admissions {
    pub fn new() -> Self {
        Admissions { heads: Keyed::new(), at_k: Keyed::new() }
    }

    /// The signer's known head, if this node has admitted anything from it.
    pub fn head(&self, vk: &[u8; 32]) -> Option<(u64, [u8; 32])> {
        self.heads.get(vk).copied()
    }

    /// Signers this node has admitted a receipt from.
    pub fn signers(&self) -> Vec<[u8; 32]> {
        self.heads
            .fiber(ROOT)
            .filter_map(|(k, _)| <[u8; 32]>::try_from(k.as_slice()).ok())
            .collect()
    }

    /// 47.2, in order, before any carrier is touched. `origin` is this
    /// node's own `(H_genesis, E_origin)`; `parent_height` resolves a
    /// parent's height on this node's chain, and answers `None` for a
    /// parent it does not hold -- an unheld parent is unmeasured height,
    /// which refuses, and is never read as zero (rule 11).
    ///
    /// Step 2 (the signature) is the caller's: it holds the key and the
    /// bytes. This is steps 1, 3 and 4, which are the ones that were
    /// missing, and it is deliberately not given a signature to check so
    /// that no call site can satisfy admission by verifying bytes alone.
    pub fn admit(
        &mut self,
        origin: ([u8; 32], u32),
        prefix: &SignedPrefix,
        r: &KStepReceipt,
        parent_height: impl Fn(&Parent) -> Option<u64>,
    ) -> Admitted {
        // 1. Origin, before anything else.
        if prefix.genesis != origin.0 || prefix.epoch != origin.1 {
            return Admitted::Refused(REFUSE_ORIGIN);
        }
        // The prefix and the receipt are ONE claim. A prefix that says a
        // different K, height or predecessor than the receipt it travels
        // with is not two opinions to reconcile, and reading either half on
        // its own would let a signer sign one position and send another.
        if prefix.k != r.step_index
            || prefix.height != r.height
            || prefix.prev_receipt_hash != r.prev_receipt_hash
        {
            return Admitted::Refused(REFUSE_RECEIPT);
        }
        // 3. K against the signer's known head. This runs BEFORE the height
        // check, in 47.2's own order: a gapped frame's height cannot
        // re-derive here yet -- the receipts that would make it re-derive
        // are exactly what the back-fill fetches -- so checking height first
        // would report every gap as a refusal and no chain would ever close.
        match self.heads.get(&r.vk).copied() {
            None if r.step_index != 0 => {
                // First sighting of a signer, mid-chain: this node cannot
                // re-derive the height of a chain whose prefix it has never
                // held. Back-fill from the signer's genesis.
                return Admitted::Gap { vk: r.vk, from: 0, to: r.step_index };
            }
            Some((head_k, _)) if r.step_index > head_k + 1 => {
                return Admitted::Gap { vk: r.vk, from: head_k, to: r.step_index };
            }
            Some((head_k, head_hash))
                if !(r.step_index == head_k + 1 && r.prev_receipt_hash == head_hash) =>
            {
                // K ≤ K_h. The hash held at that K tells a duplicate from an
                // equivocation; nothing held there is stale TO US, since a
                // head that moved past it is this node's own bookkeeping and
                // not the signer's fault.
                return match self.at_k.get(&r.vk).and_then(|m| m.get(&r.step_index.to_le_bytes()).copied())
                {
                    Some(h) if h == r.chain_hash => Admitted::Stale,
                    Some(h) => {
                        Admitted::Equivocation { k: r.step_index, held: h, claimed: r.chain_hash }
                    }
                    None => Admitted::Stale,
                };
            }
            _ => {}
        }
        // 4. The height on the frame is a CLAIM, and it re-derives from the
        // predecessor and the parents the receipt names or it is refused.
        let want = if r.step_index == 0 && r.prev_receipt_hash == [0u8; 32] {
            0
        } else {
            // The predecessor's height is one of the maxima, and it is known
            // only through a receipt this node holds; when it does not hold
            // the predecessor the claim is UNMEASURED, which refuses, and is
            // never read as zero (rule 11).
            let prev = Parent {
                vk: r.vk,
                k: r.step_index.saturating_sub(1),
                hash: r.prev_receipt_hash,
            };
            let Some(mut m) = parent_height(&prev) else {
                return Admitted::Refused(REFUSE_RECEIPT);
            };
            for p in &r.parents {
                match parent_height(p) {
                    Some(h) => m = m.max(h),
                    None => return Admitted::Refused(REFUSE_RECEIPT),
                }
            }
            1 + m
        };
        if r.height != want {
            return Admitted::Refused(REFUSE_RECEIPT);
        }
        self.remember(r);
        Admitted::Parent(Parent { vk: r.vk, k: r.step_index, hash: r.chain_hash })
    }

    fn remember(&mut self, r: &KStepReceipt) {
        self.heads.insert(r.vk, (r.step_index, r.chain_hash));
        match self.at_k.get_mut(&r.vk) {
            Some(m) => {
                m.insert(r.step_index.to_le_bytes(), r.chain_hash);
            }
            None => {
                let mut m = Keyed::new();
                m.insert(r.step_index.to_le_bytes(), r.chain_hash);
                self.at_k.insert(r.vk, m);
            }
        }
    }
}
