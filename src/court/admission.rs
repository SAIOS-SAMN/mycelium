// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 47.1, 47.2 — ADMISSION: the one byte string every signature in the tree
//! is over, carrying the signer's causal position, and the order of checks
//! a signed frame passes before any carrier is touched.

use crate::chronometry::{KStepReceipt, Parent};
use crate::network::nerve::{Keyed, ROOT};

/// The frame's genesis or epoch is not this node's (47.2 step 1).
pub const REFUSE_ORIGIN: &str = "genesis or epoch is not this node's";
/// The claimed height does not re-derive from the parents named (47.2 step 4).
pub const REFUSE_RECEIPT: &str = "claimed height does not re-derive from its parents";
/// A receipt already held at this K with this chain hash (47.2 step 3).
pub const REFUSE_RECEIPT_STALE: &str = "receipt already held at this K";
/// Two receipts at one K on one chain with different hashes (47.4).
pub const REFUSE_EQUIVOCATION: &str = "two receipts at one K with different chain hashes";

/// 47.1's signed prefix, held as its fields so a reader takes coordinates
/// off it rather than a digest. `bytes()` is the ONLY thing ever signed,
/// in this order and no other:
///
/// ```text
/// H_genesis 32 ‖ E_origin u32 ‖ prev_receipt_hash 32 ‖ K u64 ‖ height u64
///   ‖ chart len16 + axes u64… ‖ payload address len16 + bytes
/// ```
///
/// The payload is an ADDRESS -- the M2 composite (A21) for a word or form,
/// a root for a transmission digest, a listing key for a settlement act --
/// and never a digest of frame bytes, so two spellings of one word share
/// one receipt lineage and a receiver reads coordinates off the address
/// (law 2). `prev_receipt_hash` is a digest of receipt bytes because the
/// receipt bytes are the object there (Part IX).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedPrefix {
    pub genesis: [u8; 32],
    pub epoch: u32,
    pub prev_receipt_hash: [u8; 32],
    pub k: u64,
    pub height: u64,
    /// The chart's axes, written out. Never a fingerprint of the chart.
    pub chart: Vec<u64>,
    /// The payload's ADDRESS bytes (A21), written out and decodable.
    pub payload: Vec<u8>,
}

impl SignedPrefix {
    pub fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(84 + self.chart.len() * 8 + self.payload.len());
        out.extend_from_slice(&self.genesis);
        out.extend_from_slice(&self.epoch.to_le_bytes());
        out.extend_from_slice(&self.prev_receipt_hash);
        out.extend_from_slice(&self.k.to_le_bytes());
        out.extend_from_slice(&self.height.to_le_bytes());
        out.extend_from_slice(&(self.chart.len() as u16).to_le_bytes());
        for a in &self.chart {
            out.extend_from_slice(&a.to_le_bytes());
        }
        out.extend_from_slice(&(self.payload.len() as u16).to_le_bytes());
        out.extend_from_slice(&self.payload);
        out
    }

    /// Read the prefix back. Length-prefixed and decodable end to end (A21):
    /// a prefix that does not parse whole is refused, never truncated.
    pub fn read(b: &[u8]) -> Option<Self> {
        let mut p = 0usize;
        let mut take = |n: usize| -> Option<&[u8]> {
            let s = b.get(p..p + n)?;
            p += n;
            Some(s)
        };
        let genesis: [u8; 32] = take(32)?.try_into().ok()?;
        let epoch = u32::from_le_bytes(take(4)?.try_into().ok()?);
        let prev_receipt_hash: [u8; 32] = take(32)?.try_into().ok()?;
        let k = u64::from_le_bytes(take(8)?.try_into().ok()?);
        let height = u64::from_le_bytes(take(8)?.try_into().ok()?);
        let n = u16::from_le_bytes(take(2)?.try_into().ok()?) as usize;
        let mut chart = Vec::with_capacity(n);
        for _ in 0..n {
            chart.push(u64::from_le_bytes(take(8)?.try_into().ok()?));
        }
        let m = u16::from_le_bytes(take(2)?.try_into().ok()?) as usize;
        let payload = take(m)?.to_vec();
        if p != b.len() {
            return None;
        }
        Some(SignedPrefix { genesis, epoch, prev_receipt_hash, k, height, chart, payload })
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chronometry::ReceiptChain;

    const G: [u8; 32] = [7; 32];
    const E: u32 = 2;

    /// A chain of `n` steps under one key, each step naming the parents given
    /// for it. Returns the chain so its receipts are real ones: heights that
    /// re-derive, hashes that link.
    fn chain(vk: [u8; 32], steps: &[(&[u8], Vec<Parent>)]) -> ReceiptChain {
        let mut c = ReceiptChain::with_vk(vk);
        for (delta, parents) in steps {
            c.append(delta, [0; 32], parents.clone()).expect("appends");
        }
        c
    }

    fn prefix_for(r: &KStepReceipt) -> SignedPrefix {
        SignedPrefix {
            genesis: G,
            epoch: E,
            prev_receipt_hash: r.prev_receipt_hash,
            k: r.step_index,
            height: r.height,
            chart: vec![1, 2, 3],
            payload: r.delta.clone(),
        }
    }

    /// The prefix is 47.1's bytes in 47.1's order, and it reads back whole.
    /// Nothing in it is a digest of frame bytes: the payload is the address.
    #[test]
    fn the_signed_prefix_is_written_out_and_decodes() {
        let p = SignedPrefix {
            genesis: G,
            epoch: E,
            prev_receipt_hash: [9; 32],
            k: 41,
            height: 77,
            chart: vec![5, 0, 12],
            payload: b"an address, written".to_vec(),
        };
        let b = p.bytes();
        assert_eq!(&b[..32], &G, "H_genesis first");
        assert_eq!(&b[32..36], &E.to_le_bytes(), "then E_origin");
        assert_eq!(&b[36..68], &[9u8; 32], "then prev_receipt_hash");
        assert_eq!(&b[68..76], &41u64.to_le_bytes(), "then K");
        assert_eq!(&b[76..84], &77u64.to_le_bytes(), "then height");
        assert_eq!(SignedPrefix::read(&b).as_ref(), Some(&p), "and it reads back");
        assert!(SignedPrefix::read(&b[..b.len() - 1]).is_none(), "a short prefix refuses whole");
    }

    /// 47.2 step 1 runs BEFORE any carrier is touched: a frame from another
    /// genesis or another epoch is refused on the origin, not on its content,
    /// and the signer's head is not remembered from it.
    #[test]
    fn prefix_refuses_foreign_genesis_or_epoch_before_carriers() {
        let c = chain([1; 32], &[(b"a".as_slice(), vec![])]);
        let r = c.receipts()[0].clone();
        let mut a = Admissions::new();

        let mut foreign = prefix_for(&r);
        foreign.genesis = [8; 32];
        assert_eq!(
            a.admit((G, E), &foreign, &r, |_| Some(0)),
            Admitted::Refused(REFUSE_ORIGIN)
        );
        let mut old_epoch = prefix_for(&r);
        old_epoch.epoch = E - 1;
        assert_eq!(
            a.admit((G, E), &old_epoch, &r, |_| Some(0)),
            Admitted::Refused(REFUSE_ORIGIN)
        );
        assert!(a.head(&[1; 32]).is_none(), "no carrier was touched");

        assert!(
            matches!(a.admit((G, E), &prefix_for(&r), &r, |_| Some(0)), Admitted::Parent(_)),
            "the same receipt on this node's own origin admits"
        );
    }

    /// 47.2 step 3: K = K_h + 1 with the matching prev is a parent; K beyond
    /// that is a GAP, and nothing is acted on until the missing receipts are
    /// fetched. The back-fill closes it and the next step admits normally.
    #[test]
    fn receipt_gap_backfills_before_acting() {
        let c = chain(
            [1; 32],
            &[(b"a".as_slice(), vec![]), (b"b".as_slice(), vec![]), (b"c".as_slice(), vec![])],
        );
        let (r0, r1, r2) = (c.receipts()[0].clone(), c.receipts()[1].clone(), c.receipts()[2].clone());
        let h = |p: &Parent| c.receipts().iter().find(|x| x.chain_hash == p.hash).map(|x| x.height);
        let mut a = Admissions::new();

        assert!(matches!(a.admit((G, E), &prefix_for(&r0), &r0, h), Admitted::Parent(_)));
        assert_eq!(
            a.admit((G, E), &prefix_for(&r2), &r2, h),
            Admitted::Gap { vk: [1; 32], from: 0, to: 2 },
            "K jumps the head: docket and back-fill, act on nothing"
        );
        assert_eq!(a.head(&[1; 32]), Some((0, r0.chain_hash)), "the gapped frame moved no head");

        assert!(matches!(a.admit((G, E), &prefix_for(&r1), &r1, h), Admitted::Parent(_)), "back-fill");
        assert!(
            matches!(a.admit((G, E), &prefix_for(&r2), &r2, h), Admitted::Parent(_)),
            "the chain closed, so the frame admits"
        );
    }

    /// A first sighting of a signer at K > 0 is a gap back to its genesis,
    /// not an admission: this node cannot re-derive the height of a chain
    /// whose prefix it has never held.
    #[test]
    fn a_signer_first_seen_mid_chain_is_a_gap_to_its_genesis() {
        let c = chain([2; 32], &[(b"a".as_slice(), vec![]), (b"b".as_slice(), vec![])]);
        let r1 = c.receipts()[1].clone();
        let h = |p: &Parent| c.receipts().iter().find(|x| x.chain_hash == p.hash).map(|x| x.height);
        let mut a = Admissions::new();
        assert_eq!(
            a.admit((G, E), &prefix_for(&r1), &r1, h),
            Admitted::Gap { vk: [2; 32], from: 0, to: 1 }
        );
    }

    /// 47.2 step 4. The height on the frame is a CLAIM and is re-derived
    /// from the parents the receipt names. A parent this node does not hold
    /// leaves the height unmeasured, which refuses; a height that re-derives
    /// to something else refuses on the same code.
    #[test]
    fn claimed_height_that_does_not_rederive_is_refused() {
        let c = chain([1; 32], &[(b"a".as_slice(), vec![]), (b"b".as_slice(), vec![])]);
        let r1 = c.receipts()[1].clone();
        let held = |p: &Parent| c.receipts().iter().find(|x| x.chain_hash == p.hash).map(|x| x.height);
        let mut a = Admissions::new();
        a.admit((G, E), &prefix_for(&c.receipts()[0]), &c.receipts()[0], held);

        let mut lying = r1.clone();
        lying.height = r1.height + 5;
        assert_eq!(
            a.admit((G, E), &prefix_for(&lying), &lying, held),
            Admitted::Refused(REFUSE_RECEIPT),
            "a height that does not re-derive is refused"
        );
        assert_eq!(
            a.admit((G, E), &prefix_for(&r1), &r1, |_| None),
            Admitted::Refused(REFUSE_RECEIPT),
            "a parent this node does not hold is UNMEASURED, never height 0"
        );
        // The prefix and the receipt are one claim, not two opinions.
        let mut split = prefix_for(&r1);
        split.height = r1.height + 1;
        assert_eq!(
            a.admit((G, E), &split, &r1, held),
            Admitted::Refused(REFUSE_RECEIPT),
            "a prefix that disagrees with the receipt it travels with"
        );
        assert!(matches!(a.admit((G, E), &prefix_for(&r1), &r1, held), Admitted::Parent(_)));
    }

    /// 47.2 step 3's last two arms. The same receipt twice is STALE, and is
    /// ignored. A different chain hash at a K already held is not a later
    /// opinion to prefer: it is an equivocation proof (47.4).
    #[test]
    fn a_duplicate_is_stale_and_a_second_hash_at_one_k_is_equivocation() {
        let c = chain([1; 32], &[(b"a".as_slice(), vec![]), (b"b".as_slice(), vec![])]);
        let d = chain([1; 32], &[(b"z".as_slice(), vec![])]);
        let (r0, r1) = (c.receipts()[0].clone(), c.receipts()[1].clone());
        let held = |p: &Parent| {
            c.receipts()
                .iter()
                .chain(d.receipts())
                .find(|x| x.chain_hash == p.hash)
                .map(|x| x.height)
        };
        let mut a = Admissions::new();
        assert!(matches!(a.admit((G, E), &prefix_for(&r0), &r0, held), Admitted::Parent(_)));
        assert!(matches!(a.admit((G, E), &prefix_for(&r1), &r1, held), Admitted::Parent(_)));

        assert_eq!(a.admit((G, E), &prefix_for(&r0), &r0, held), Admitted::Stale);

        let fork = d.receipts()[0].clone();
        assert_ne!(fork.chain_hash, r0.chain_hash, "one K, two hashes");
        assert_eq!(
            a.admit((G, E), &prefix_for(&fork), &fork, held),
            Admitted::Equivocation { k: 0, held: r0.chain_hash, claimed: fork.chain_hash }
        );
    }
}
