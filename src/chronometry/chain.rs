// 45 — THE RECEIPT CHAIN: append-only, parent-linked, and the order two
// receipts stand in.

use super::receipt::{KStepReceipt, Parent};
use crate::network::nerve::Keyed;
use crate::physics::Budget;

/// The receipt chain — append-only, parent-linked, integrity-verifiable.
#[derive(Debug, Clone)]
pub struct ReceiptChain {
    pub(super) receipts: Vec<KStepReceipt>,
    vk: [u8; 32],
    /// Verified peer receipts, keyed by chain hash, so parent heights re-derive.
    known: Keyed<KStepReceipt>,
    /// THIS CHAIN'S OWN RECEIPTS, keyed by chain hash — the step each hash
    /// sits at. `by_hash` and `parent_height` fell back to
    /// `receipts.iter().find(…)`, a walk of the whole chain for one lookup
    /// (rule 10: a loop whose bound is neither a `Budget` nor one cell's
    /// boundary). Every admitted receipt re-derives a height through it
    /// (47.2 step 4) and `frontier_from` pays it once per VISIT, so
    /// ordering two receipts cost the walk's depth times the chain — on a
    /// carrier that grows for as long as the node runs. A chain hash names
    /// one receipt, and that is a keyed read.
    mine: Keyed<usize>,
}

impl Default for ReceiptChain {
    fn default() -> Self {
        ReceiptChain {
            receipts: Vec::new(),
            vk: [0u8; 32],
            known: Keyed::new(),
            mine: Keyed::new(),
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
        self.by_hash(&p.hash).map(|r| r.height)
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
        self.mine.insert(chain_hash, self.receipts.len());
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
        self.receipts.last()
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

    /// The receipt a chain hash names — this chain's own, else one it has
    /// admitted. Two keyed reads and no walk.
    fn by_hash(&self, hash: &[u8]) -> Option<&KStepReceipt> {
        match self.mine.get(hash) {
            Some(&i) => self.receipts.get(i),
            None => self.known.get(hash),
        }
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
        // The index is built before integrity is verified, because
        // verification re-derives every height through it.
        let mut mine = Keyed::new();
        for (i, r) in receipts.iter().enumerate() {
            mine.insert(r.chain_hash, i);
        }
        let c = ReceiptChain {
            receipts,
            vk,
            known: Keyed::new(),
            mine,
        };
        if c.verify_chain_integrity() {
            Some(c)
        } else {
            None
        }
    }
}
