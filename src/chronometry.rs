// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! CHRONOMETRY — the receipt is the clock (MU-RECURSION 45). K orders events
//! inside one kernel. TUniversal is the height of the receipt ledger. No
//! kernel reads a wall clock.

use crate::network::nerve::Keyed;
use crate::physics::Budget;

/// A peer receipt this act verified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parent {
    pub vk: [u8; 32],
    pub k: u64,
    pub hash: [u8; 32],
}

/// Height of a receipt in the ledger (45.2). Genesis is 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Height(pub u64);

impl Height {
    pub fn get(self) -> u64 {
        self.0
    }

    /// A height reads as a height. This printed `XX:XX:XX:XX:XX:XX` — the
    /// big-endian bytes of the number, formatted to look like a wall clock
    /// (C6). There is no clock (law 9, Part IX): TUniversal is the height of
    /// the receipt ledger, and a reader who sees a timestamp will reason
    /// about elapsed time that nothing in this system measures. `h {n}`, as
    /// 34.2 writes it.
    pub fn stamp(self) -> String {
        Self::stamp_of(self.0)
    }

    pub fn stamp_of(h: u64) -> String {
        format!("h {h}")
    }
}

/// TUniversal is the height of the mesh receipt ledger.
pub type TUniversal = Height;

/// One K-step receipt: `chain_hash = blake3(step ‖ height ‖ parents ‖ delta ‖ op ‖ prev)`.
/// The delta is the Address bytes themselves, never a digest of content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KStepReceipt {
    /// Monotonic step index — this kernel's own K, 0 at genesis.
    pub step_index: u64,
    /// Ledger height (45.2).
    pub height: u64,
    /// Peer receipts this act verified.
    pub parents: Vec<Parent>,
    /// This kernel's verifying key.
    pub vk: [u8; 32],
    /// The committed address, written out.
    pub delta: Vec<u8>,
    /// Which operation was applied (an act digest — never reasoned on).
    pub operator_hash: [u8; 32],
    /// The parent receipt's chain hash (all zeros at genesis).
    pub prev_receipt_hash: [u8; 32],
    /// This receipt's chain hash.
    pub chain_hash: [u8; 32],
}

/// Receipt payload identity of admitted content bytes (Part IX: the
/// object of the hash is the bytes written on the receipt).
pub fn payload_hash(bytes: &[u8]) -> [u8; 32] {
    *blake3::hash(bytes).as_bytes()
}

pub fn operator_hash(voice: &[u8; 32], court_head: &[u8; 32]) -> [u8; 32] {
    let mut buf = [0u8; 64];
    buf[..32].copy_from_slice(voice);
    buf[32..].copy_from_slice(court_head);
    *blake3::hash(&buf).as_bytes()
}

impl KStepReceipt {
    fn put_parents(buf: &mut Vec<u8>, parents: &[Parent]) {
        buf.extend_from_slice(&(parents.len() as u32).to_le_bytes());
        for p in parents {
            buf.extend_from_slice(&p.vk);
            buf.extend_from_slice(&p.k.to_le_bytes());
            buf.extend_from_slice(&p.hash);
        }
    }

    fn bytes(
        step_index: u64,
        height: u64,
        parents: &[Parent],
        delta: &[u8],
        op: &[u8; 32],
        prev: &[u8; 32],
    ) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&step_index.to_le_bytes());
        buf.extend_from_slice(&height.to_le_bytes());
        Self::put_parents(&mut buf, parents);
        buf.extend_from_slice(&(delta.len() as u32).to_le_bytes());
        buf.extend_from_slice(delta);
        buf.extend_from_slice(op);
        buf.extend_from_slice(prev);
        buf
    }

    fn hash(
        step_index: u64,
        height: u64,
        parents: &[Parent],
        delta: &[u8],
        op: &[u8; 32],
        prev: &[u8; 32],
    ) -> [u8; 32] {
        *blake3::hash(&Self::bytes(step_index, height, parents, delta, op, prev)).as_bytes()
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.step_index.to_le_bytes());
        out.extend_from_slice(&self.height.to_le_bytes());
        out.extend_from_slice(&self.vk);
        Self::put_parents(out, &self.parents);
        out.extend_from_slice(&(self.delta.len() as u32).to_le_bytes());
        out.extend_from_slice(&self.delta);
        out.extend_from_slice(&self.operator_hash);
        out.extend_from_slice(&self.prev_receipt_hash);
        out.extend_from_slice(&self.chain_hash);
    }

    pub fn read(b: &[u8], pos: &mut usize) -> Option<Self> {
        let take32 = |p: &mut usize| -> Option<[u8; 32]> {
            let v: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            Some(v)
        };
        let step_index = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
        *pos += 8;
        let height = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
        *pos += 8;
        let vk = take32(pos)?;
        let np = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
        *pos += 4;
        let mut parents = Vec::with_capacity(np.min(1024));
        for _ in 0..np {
            let vk = take32(pos)?;
            let k = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
            let hash = take32(pos)?;
            parents.push(Parent { vk, k, hash });
        }
        let n = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
        *pos += 4;
        let delta = b.get(*pos..*pos + n)?.to_vec();
        *pos += n;
        Some(KStepReceipt {
            step_index,
            height,
            parents,
            vk,
            delta,
            operator_hash: take32(pos)?,
            prev_receipt_hash: take32(pos)?,
            chain_hash: take32(pos)?,
        })
    }
}

/// The receipt chain — append-only, parent-linked, integrity-verifiable.
#[derive(Debug, Clone)]
pub struct ReceiptChain {
    receipts: Vec<KStepReceipt>,
    vk: [u8; 32],
    /// Verified peer receipts, keyed by chain hash, so parent heights re-derive.
    known: Keyed<KStepReceipt>,
}

impl Default for ReceiptChain {
    fn default() -> Self {
        ReceiptChain {
            receipts: Vec::new(),
            vk: [0u8; 32],
            known: Keyed::new(),
        }
    }
}

impl ReceiptChain {
    pub fn new() -> Self {
        ReceiptChain::default()
    }

    pub fn with_vk(vk: [u8; 32]) -> Self {
        ReceiptChain {
            vk,
            ..ReceiptChain::default()
        }
    }

    /// The next step index — the current K (0 at genesis).
    pub fn next_step_index(&self) -> u64 {
        self.receipts.len() as u64
    }

    /// The head's chain hash (all zeros before genesis).
    pub fn head_hash(&self) -> [u8; 32] {
        self.receipts.last().map_or([0u8; 32], |r| r.chain_hash)
    }

    /// The head receipt's step index. Zero before genesis.
    pub fn head_k(&self) -> u64 {
        self.receipts.last().map_or(0, |r| r.step_index)
    }

    /// TUniversal: height of the head receipt. Zero before genesis.
    pub fn head_height(&self) -> u64 {
        self.receipts.last().map_or(0, |r| r.height)
    }

    /// Height of a receipt this chain holds, by its chain hash. `None` when
    /// the chain does not hold it: that is UNMEASURED height, and a caller
    /// re-deriving a claimed height must refuse rather than read it as zero.
    pub fn height_of(&self, hash: &[u8; 32]) -> Option<u64> {
        self.parent_height(&Parent { vk: [0u8; 32], k: 0, hash: *hash })
    }

    fn parent_height(&self, p: &Parent) -> Option<u64> {
        self.known.get(&p.hash).map(|r| r.height).or_else(|| {
            self.receipts
                .iter()
                .find(|r| r.chain_hash == p.hash)
                .map(|r| r.height)
        })
    }

    /// Bind this chain to the key that signs for it. The vk is NOT in the
    /// chain hash (it is written and read, never hashed), so binding is
    /// lossless on the history: receipts already sealed under the zero
    /// placeholder are stamped with the signer they always had, and a
    /// receipt naming a real signer is never overwritten -- an admitted
    /// peer receipt keeps its own author.
    ///
    /// Unbound, a chain seals every receipt with `vk: [0u8; 32]`, which is
    /// not a signer: 47.2 step 2 verifies the signature under the CLAIMED
    /// key, so every receipt this node sent was checked against zeros and
    /// refused. It is the same zero-sentinel fault as an unwritten genesis.
    pub fn bind_vk(&mut self, vk: [u8; 32]) {
        self.vk = vk;
        for r in &mut self.receipts {
            if r.vk == [0u8; 32] {
                r.vk = vk;
            }
        }
    }

    /// Admit a verified peer receipt so later parents can re-derive height.
    pub fn admit(&mut self, r: KStepReceipt) {
        self.known.insert(r.chain_hash, r);
    }

    /// Append the next receipt. Height is 0 at genesis, else
    /// `1 + max(h(prev), max parent heights)`.
    pub fn append(
        &mut self,
        delta: impl AsRef<[u8]>,
        operator_hash: [u8; 32],
        parents: Vec<Parent>,
    ) -> Option<&KStepReceipt> {
        let step_index = self.next_step_index();
        let prev = self.head_hash();
        let delta = delta.as_ref().to_vec();
        let height = if self.receipts.is_empty() {
            0
        } else {
            let mut m = self.head_height();
            for p in &parents {
                m = m.max(self.parent_height(p)?);
            }
            1 + m
        };
        let chain_hash = KStepReceipt::hash(step_index, height, &parents, &delta, &operator_hash, &prev);
        self.receipts.push(KStepReceipt {
            step_index,
            height,
            parents,
            vk: self.vk,
            delta,
            operator_hash,
            prev_receipt_hash: prev,
            chain_hash,
        });
        Some(self.receipts.last().unwrap())
    }

    /// Verify the whole chain: indices monotone from 0, every link's parent hash
    /// matches, every chain hash recomputes, every height re-derives.
    pub fn verify_chain_integrity(&self) -> bool {
        let mut prev = [0u8; 32];
        let mut prev_h = 0u64;
        for (i, r) in self.receipts.iter().enumerate() {
            if r.step_index != i as u64 || r.prev_receipt_hash != prev {
                return false;
            }
            let expect_h = if i == 0 {
                0
            } else {
                let mut m = prev_h;
                for p in &r.parents {
                    let Some(ph) = self.parent_height(p) else {
                        return false;
                    };
                    m = m.max(ph);
                }
                1 + m
            };
            if r.height != expect_h {
                return false;
            }
            let expect = KStepReceipt::hash(
                r.step_index,
                r.height,
                &r.parents,
                &r.delta,
                &r.operator_hash,
                &prev,
            );
            if r.chain_hash != expect {
                return false;
            }
            prev = r.chain_hash;
            prev_h = r.height;
        }
        true
    }

    pub fn receipts(&self) -> &[KStepReceipt] {
        &self.receipts
    }

    fn by_hash(&self, hash: &[u8]) -> Option<&KStepReceipt> {
        self.known.get(hash).or_else(|| {
            self.receipts.iter().find(|r| r.chain_hash.as_slice() == hash)
        })
    }

    /// Frontier reachable from one receipt: vk ↦ greatest K reachable through
    /// prev and parents. Each reached receipt is visited once, one spend per
    /// visit. `None` when the budget is spent with the walk unfinished: a
    /// truncated frontier is UNMEASURED, never a smaller one. Reporting a
    /// short frontier as the frontier is what let `precedes` answer `false`
    /// for receipts that do precede (rule 11: exact or refused).
    fn frontier_from(&self, head: [u8; 32], budget: &mut Budget) -> Option<Keyed<u64>> {
        let mut f = Keyed::new();
        let mut seen: Keyed<()> = Keyed::new();
        let mut stack: Vec<[u8; 32]> = vec![head];
        while let Some(hash) = stack.pop() {
            if !budget.spend() {
                return None;
            }
            if seen.contains(&hash) {
                continue;
            }
            seen.insert(hash, ());
            let Some(r) = self.by_hash(&hash) else {
                continue;
            };
            let cur = f.get(&r.vk).copied().unwrap_or(0);
            if r.step_index >= cur {
                f.insert(r.vk, r.step_index);
            }
            if r.prev_receipt_hash != [0u8; 32] {
                stack.push(r.prev_receipt_hash);
            }
            for p in &r.parents {
                stack.push(p.hash);
            }
        }
        Some(f)
    }

    /// Frontier of the head. An empty chain has an empty frontier; a walk
    /// the budget could not finish is unmeasured.
    pub fn frontier(&self, budget: &mut Budget) -> Option<Keyed<u64>> {
        match self.receipts.last() {
            None => Some(Keyed::new()),
            Some(h) => self.frontier_from(h.chain_hash, budget),
        }
    }

    /// `a ≺ b` iff F(a) ≤ F(b) componentwise and the frontiers differ.
    /// `None` when either frontier is unmeasured under this budget — the
    /// caller holds both receipts as concurrent (45.3), which is what an
    /// unordered pair is, rather than reading an exhausted walk as evidence
    /// that `a` does not precede `b`.
    ///
    /// The two frontiers are walked from `a` and `b` over this chain's own
    /// receipts, so nothing is copied to compare two of its members: the
    /// walk from a receipt reaches exactly its ancestors, which is what the
    /// prefix-plus-self copy used to construct (rule 12).
    pub fn precedes(
        &self,
        a: &KStepReceipt,
        b: &KStepReceipt,
        budget: &mut Budget,
    ) -> Option<bool> {
        let fa = self.frontier_from(a.chain_hash, budget)?;
        let fb = self.frontier_from(b.chain_hash, budget)?;
        let mut equal = true;
        let mut le = true;
        for (vk, &ka) in fa.fiber(crate::network::nerve::ROOT) {
            let kb = fb.get(&vk).copied().unwrap_or(0);
            if ka > kb {
                le = false;
            }
            if ka != kb {
                equal = false;
            }
        }
        for (vk, &kb) in fb.fiber(crate::network::nerve::ROOT) {
            if fa.get(&vk).is_none() && kb > 0 {
                equal = false;
            }
        }
        Some(le && !equal)
    }

    /// `a ≺ b` for two receipts named by their chain hashes. `None` when
    /// this chain does not hold one of them, or when the budget cannot
    /// finish a walk: both are UNMEASURED order, which a caller holds as
    /// unordered (45.3) rather than reading as "does not precede".
    pub fn precedes_hashes(
        &self,
        a: &[u8; 32],
        b: &[u8; 32],
        budget: &mut Budget,
    ) -> Option<bool> {
        let ra = self.by_hash(a)?.clone();
        let rb = self.by_hash(b)?.clone();
        self.precedes(&ra, &rb, budget)
    }

    /// Rehydrate a chain from persisted receipts. Integrity is verified
    /// before acceptance — a tampered journal refuses whole.
    pub fn from_receipts(receipts: Vec<KStepReceipt>) -> Option<ReceiptChain> {
        let vk = receipts.first().map(|r| r.vk).unwrap_or([0u8; 32]);
        let c = ReceiptChain {
            receipts,
            vk,
            known: Keyed::new(),
        };
        if c.verify_chain_integrity() {
            Some(c)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::Physics;

    /// A budget of exactly `n` visits. The walks below are bounded by the
    /// receipts they reach, not by this box's reserve: `stroke()` is
    /// `(reserve_max()/8).max(2)`, so a hardware-derived budget made these
    /// claims pass on a large host and fail on a small one.
    fn budget(n: usize) -> Budget {
        Physics::default().stroke_n(n)
    }

    #[test]
    fn height_is_one_plus_max_of_prev_and_parents() {
        let mut a = ReceiptChain::with_vk([1; 32]);
        a.append([1u8], [9; 32], Vec::new()).expect("genesis");
        assert_eq!(a.head_height(), 0, "h(genesis) = 0");
        a.append([2u8], [9; 32], Vec::new()).expect("second");
        assert_eq!(a.head_height(), 1, "h = 1 + h(prev)");
        let mut b = ReceiptChain::with_vk([2; 32]);
        b.append([1u8], [9; 32], Vec::new()).expect("b genesis");
        b.append([2u8], [9; 32], Vec::new()).expect("b second");
        b.append([3u8], [9; 32], Vec::new()).expect("b third");
        assert_eq!(b.head_height(), 2);
        let parent = Parent {
            vk: [2; 32],
            k: b.head_k(),
            hash: b.head_hash(),
        };
        a.admit(b.receipts().last().unwrap().clone());
        a.append([3u8], [9; 32], vec![parent]).expect("cite b");
        assert_eq!(
            a.head_height(),
            1 + 2,
            "h = 1 + max(h(prev)=1, h(parent)=2)"
        );
    }

    #[test]
    fn precedes_implies_lower_height_and_concurrency_is_kept() {
        let mut c = ReceiptChain::with_vk([1; 32]);
        c.append([1u8], [9; 32], Vec::new()).unwrap();
        c.append([2u8], [9; 32], Vec::new()).unwrap();
        c.append([3u8], [9; 32], Vec::new()).unwrap();
        let r0 = &c.receipts()[0];
        let r1 = &c.receipts()[1];
        let r2 = &c.receipts()[2];
        // Each pair pays for the two walks it reaches: 1+2, 2+3, 3+1 visits.
        let mut b = budget(8);
        assert_eq!(c.precedes(r0, r1, &mut b), Some(true), "earlier ≺ later");
        let mut b = budget(8);
        assert_eq!(c.precedes(r1, r2, &mut b), Some(true));
        let mut b = budget(8);
        assert_eq!(
            c.precedes(r2, r0, &mut b),
            Some(false),
            "later does not precede earlier"
        );
        assert!(r0.height < r1.height && r1.height < r2.height);
        let mut d = ReceiptChain::with_vk([3; 32]);
        d.append([1u8], [9; 32], Vec::new()).unwrap();
        let mut e = ReceiptChain::with_vk([4; 32]);
        e.append([1u8], [9; 32], Vec::new()).unwrap();
        let mut mix = ReceiptChain::with_vk([1; 32]);
        mix.append([0u8], [9; 32], Vec::new()).unwrap();
        mix.admit(d.receipts()[0].clone());
        mix.admit(e.receipts()[0].clone());
        mix.append(
            [9u8],
            [9; 32],
            vec![
                Parent {
                    vk: [3; 32],
                    k: 0,
                    hash: d.head_hash(),
                },
                Parent {
                    vk: [4; 32],
                    k: 0,
                    hash: e.head_hash(),
                },
            ],
        )
        .unwrap();
        let mut b = budget(8);
        assert_eq!(
            mix.precedes(&d.receipts()[0], &e.receipts()[0], &mut b),
            Some(false),
            "two geneses are concurrent"
        );
    }

    /// An exhausted walk is unmeasured, never "does not precede": a budget
    /// that cannot reach the second frontier refuses, and the caller holds
    /// the pair as concurrent (45.3) instead of ordering it on a short read.
    #[test]
    fn a_frontier_the_budget_cannot_finish_is_refused_not_shortened() {
        let mut c = ReceiptChain::with_vk([1; 32]);
        c.append([1u8], [9; 32], Vec::new()).unwrap();
        c.append([2u8], [9; 32], Vec::new()).unwrap();
        c.append([3u8], [9; 32], Vec::new()).unwrap();
        let (r0, r2) = (&c.receipts()[0], &c.receipts()[2]);
        // r0 costs one visit, r2 three: one unit reaches neither pair.
        let mut short = budget(1);
        assert_eq!(c.precedes(r0, r2, &mut short), None, "unmeasured, not false");
        let mut short = budget(2);
        assert!(c.frontier(&mut short).is_none(), "the head walk is three deep");
        let mut enough = budget(3);
        assert!(c.frontier(&mut enough).is_some(), "measured at its own size");
    }

    #[test]
    fn frontier_visits_each_receipt_once() {
        let mut c = ReceiptChain::with_vk([7; 32]);
        c.append([1u8], [9; 32], Vec::new()).unwrap();
        c.append([2u8], [9; 32], Vec::new()).unwrap();
        let mut b = budget(8);
        let f = c.frontier(&mut b).expect("measured");
        assert_eq!(f.get(&[7; 32]).copied(), Some(1), "greatest K of this vk");
        assert_eq!(f.len(), 1, "one vk, visited once");
    }

    #[test]
    fn the_chain_is_temporal_authority_and_tamper_evident() {
        let mut c = ReceiptChain::new();
        assert_eq!(c.next_step_index(), 0, "genesis K = 0");
        c.append([1u8], [9; 32], Vec::new()).unwrap();
        c.append([2u8], [9; 32], Vec::new()).unwrap();
        let r = c.append([3u8], [8; 32], Vec::new()).unwrap();
        assert_eq!(r.step_index, 2, "K is the algebraic clock");
        assert_eq!(c.head_k(), 2, "head_k is the last receipt");
        assert_eq!(c.next_step_index(), 3, "next is one past the head");
        assert!(c.verify_chain_integrity());
        let mut t = c.clone();
        t.receipts[1].delta[0] ^= 1;
        assert!(!t.verify_chain_integrity(), "a single byte breaks the continuity law");
    }
}
