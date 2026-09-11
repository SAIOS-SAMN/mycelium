// 45 — THE RECEIPT CHAIN: append-only, parent-linked, and the order two
// receipts stand in.

use super::era::Root;
use super::receipt::{ActKind, KStepReceipt, Parent};
use crate::network::nerve::Keyed;
use crate::physics::Budget;

/// The receipt chain — append-only, parent-linked, integrity-verifiable.
#[derive(Debug, Clone)]
pub struct ReceiptChain {
    pub(super) receipts: Vec<KStepReceipt>,
    /// THE FOLD ROOT (45.8): the archived prefix's head this hot chain
    /// stands on, or none before the first fold. K, the previous hash and
    /// the height continue from it; the archived receipts are held cold in
    /// `ARC1` and are not read through this chain.
    root: Option<Root>,
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
    /// THE FRONTIER IS CARRIED, NOT RE-WALKED (45.3). `F(i)` is
    /// `peers[peer_at[i]]` raised at receipt `i`'s own signer to its step
    /// index: the join over everything receipt `i` reaches. It is monotone
    /// along the chain, so a receipt with no parents and the same signer as
    /// the one before it REUSES that snapshot — a new one is materialized
    /// only where the chain actually gains ancestry (a receipt with
    /// parents) or changes signer.
    ///
    /// Re-deriving `F` by walking the ancestry made causal order cost the
    /// chain's whole length per question, charged to a stroke budget: on a
    /// four-receipt chain with `Budget { remaining: 3 }` the walk ran out,
    /// `precedes` answered `None`, and the transition that demonstrably
    /// followed its incumbent was held concurrent instead of superseding
    /// it. Supersession on the dialect book was decided by how much memory
    /// the machine had free — the same machine-stroke instability that is
    /// not comparable even within one box. A frontier is state of the
    /// chain; carrying it makes `precedes` two keyed reads.
    peer_at: Vec<u32>,
    /// The shared snapshots `peer_at` indexes. `peers[0]` is empty.
    peers: Vec<Keyed<u64>>,
}

/// Raise `f` at `vk` to `k` — a frontier component only ever grows.
fn raise(f: &mut Keyed<u64>, vk: &[u8; 32], k: u64) {
    match f.get(vk) {
        Some(&held) if held >= k => {}
        _ => {
            f.insert(*vk, k);
        }
    }
}

/// Join `src` into `dst` componentwise.
fn join(dst: &mut Keyed<u64>, src: &Keyed<u64>) {
    for (vk, &k) in src.fiber(crate::network::nerve::ROOT) {
        if let Ok(vk) = <[u8; 32]>::try_from(vk.as_slice()) {
            raise(dst, &vk, k);
        }
    }
}

impl Default for ReceiptChain {
    fn default() -> Self {
        ReceiptChain {
            receipts: Vec::new(),
            root: None,
            vk: [0u8; 32],
            known: Keyed::new(),
            mine: Keyed::new(),
            peer_at: Vec::new(),
            peers: vec![Keyed::new()],
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

    /// The next step index — the current K (0 at genesis; one past the
    /// fold root and the hot receipts after a fold).
    pub fn next_step_index(&self) -> u64 {
        self.root.map_or(0, |r| r.k + 1) + self.receipts.len() as u64
    }

    /// The head's chain hash (the fold root's before the first hot receipt;
    /// all zeros before genesis).
    pub fn head_hash(&self) -> [u8; 32] {
        self.receipts
            .last()
            .map(|r| r.chain_hash)
            .or_else(|| self.root.map(|r| r.hash))
            .unwrap_or([0u8; 32])
    }

    /// The head receipt's step index. Zero before genesis.
    pub fn head_k(&self) -> u64 {
        self.receipts
            .last()
            .map(|r| r.step_index)
            .or_else(|| self.root.map(|r| r.k))
            .unwrap_or(0)
    }

    /// TUniversal: height of the head receipt. Zero before genesis.
    pub fn head_height(&self) -> u64 {
        self.receipts
            .last()
            .map(|r| r.height)
            .or_else(|| self.root.map(|r| r.height))
            .unwrap_or(0)
    }

    /// The fold root this chain stands on, if it was ever folded.
    pub fn root(&self) -> Option<Root> {
        self.root
    }

    /// FOLD (45.8): the receipts at or below `k` leave the hot chain and
    /// are returned, cold, with the root they leave it standing on. `None`
    /// when no hot receipt sits at or below `k`: nothing to fold.
    pub fn fold_before(&mut self, k: u64) -> Option<(Vec<KStepReceipt>, Root)> {
        let n = self.receipts.iter().take_while(|r| r.step_index <= k).count();
        let last = self.receipts.get(n.checked_sub(1)?)?;
        let root = Root { k: last.step_index, hash: last.chain_hash, height: last.height };
        let tail = self.receipts.split_off(n);
        let cold = std::mem::replace(&mut self.receipts, tail);
        // The snapshots the hot receipts stand on are kept: each was built
        // over the whole history, so a folded prefix does not lower any
        // frontier that already reads it (45.8).
        self.peer_at = self.peer_at.split_off(n);
        self.mine = Keyed::new();
        for (i, r) in self.receipts.iter().enumerate() {
            self.mine.insert(r.chain_hash, i);
        }
        self.root = Some(root);
        Some((cold, root))
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

    /// The frontier of the receipt at `i`: the snapshot it stands on,
    /// raised at its own signer to its own step index. Two reads, no walk.
    fn frontier_of_index(&self, i: usize) -> Option<Keyed<u64>> {
        let r = self.receipts.get(i)?;
        let mut f = self.peers.get(*self.peer_at.get(i)? as usize)?.clone();
        raise(&mut f, &r.vk, r.step_index);
        Some(f)
    }

    /// Record the snapshot the receipt about to be appended stands on.
    /// Called with `self.receipts` still ending at the PREVIOUS receipt.
    ///
    /// A receipt with no parents whose signer is the one before it reaches
    /// exactly what that one reached, plus itself — so it reuses the
    /// snapshot and the join of its own `(vk, k)` is left to the read. A
    /// receipt that names parents gains ancestry, and that is where a new
    /// snapshot is materialized: the previous receipt's whole frontier
    /// joined with each parent's.
    fn extend_frontier(&mut self, vk: [u8; 32], parents: &[Parent]) {
        let prev_i = self.receipts.len().checked_sub(1);
        let same_signer = prev_i
            .and_then(|i| self.receipts.get(i))
            .is_none_or(|p| p.vk == vk);
        // THE FIRST RECEIPT OF A HOT CHAIN STANDS ON THE EMPTY SNAPSHOT:
        // it reaches nothing but itself, and that is exact, not a default.
        if parents.is_empty() && same_signer {
            let mut at = 0u32;
            if let Some(&held) = prev_i.and_then(|i| self.peer_at.get(i)) {
                at = held;
            }
            self.peer_at.push(at);
            return;
        }
        let mut snap = Keyed::new();
        if let Some(f) = prev_i.and_then(|i| self.frontier_of_index(i)) {
            snap = f;
        }
        for p in parents {
            join(&mut snap, &self.frontier_of_admitted(&p.hash));
        }
        self.peers.push(snap);
        self.peer_at.push((self.peers.len() - 1) as u32);
    }

    /// The frontier of an ADMITTED peer receipt: what its held ancestry
    /// supports. The descent runs over the admitted set only — it splices
    /// a stored frontier and stops the moment it reaches a receipt of this
    /// chain's own, so it is bounded by the peer receipts held and never
    /// by the chain's length. Ancestry this node does not hold contributes
    /// nothing, which is what an unheld receipt contributed before.
    fn frontier_of_admitted(&self, head: &[u8; 32]) -> Keyed<u64> {
        let mut f = Keyed::new();
        let mut seen: Keyed<()> = Keyed::new();
        let mut stack: Vec<[u8; 32]> = vec![*head];
        let mut left = self.known.len() + 1;
        while let Some(hash) = stack.pop() {
            if left == 0 {
                break;
            }
            left -= 1;
            if seen.contains(&hash) {
                continue;
            }
            seen.insert(hash, ());
            if let Some(&i) = self.mine.get(&hash) {
                if let Some(stored) = self.frontier_of_index(i) {
                    join(&mut f, &stored);
                }
                continue;
            }
            let Some(r) = self.known.get(&hash) else { continue };
            raise(&mut f, &r.vk, r.step_index);
            if r.prev_receipt_hash != [0u8; 32] {
                stack.push(r.prev_receipt_hash);
            }
            for p in &r.parents {
                stack.push(p.hash);
            }
        }
        f
    }

    /// A peer receipt this chain admitted, by its chain hash.
    pub fn known(&self, hash: &[u8; 32]) -> Option<&KStepReceipt> {
        self.known.get(hash)
    }

    /// Append the next receipt, naming the act it admits (45.1). Height is
    /// 0 at genesis, else `1 + max(h(prev), max parent heights)`.
    /// `mu` is WHAT THE ACT MEASURED, per lane (45.1, 45.9), and it is in
    /// the chain hash: an act's measure is part of what the receipt
    /// commits to, so a ledger read by kind and by lane reads one object.
    pub fn append(
        &mut self,
        kind: ActKind,
        mu: [u64; 4],
        delta: impl AsRef<[u8]>,
        operator_hash: [u8; 32],
        parents: Vec<Parent>,
    ) -> Option<&KStepReceipt> {
        let step_index = self.next_step_index();
        let prev = self.head_hash();
        let delta = delta.as_ref().to_vec();
        let height = if self.receipts.is_empty() && self.root.is_none() {
            0
        } else {
            let mut m = self.head_height();
            for p in &parents {
                m = m.max(self.parent_height(p)?);
            }
            1 + m
        };
        let chain_hash = KStepReceipt::hash(
            step_index,
            height,
            &parents,
            kind,
            &mu,
            &delta,
            &operator_hash,
            &prev,
        );
        self.mine.insert(chain_hash, self.receipts.len());
        self.extend_frontier(self.vk, &parents);
        self.receipts.push(KStepReceipt {
            step_index,
            height,
            parents,
            kind,
            mu,
            vk: self.vk,
            delta,
            operator_hash,
            prev_receipt_hash: prev,
            chain_hash,
        });
        self.receipts.last()
    }

    /// Verify the whole hot chain: indices monotone from 0 (or from one past
    /// the fold root), every link's parent hash matches, every chain hash
    /// recomputes, every height re-derives.
    pub fn verify_chain_integrity(&self) -> bool {
        let mut prev = self.root.map_or([0u8; 32], |r| r.hash);
        let mut prev_h = self.root.map_or(0, |r| r.height);
        let first_k = self.root.map_or(0, |r| r.k + 1);
        for (i, r) in self.receipts.iter().enumerate() {
            if r.step_index != first_k + i as u64 || r.prev_receipt_hash != prev {
                return false;
            }
            let expect_h = if i == 0 && self.root.is_none() {
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
                r.kind,
                &r.mu,
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

    /// Frontier reachable from one receipt: vk ↦ greatest K reachable
    /// through prev and parents. One spend: a receipt of this chain's own
    /// reads the frontier it carries, and a peer receipt's is the join over
    /// the admitted ancestry that stands behind it. `None` when the budget
    /// is spent — an unmeasured frontier, never a short one (rule 11).
    fn frontier_from(&self, head: [u8; 32], budget: &mut Budget) -> Option<Keyed<u64>> {
        if !budget.spend() {
            return None;
        }
        match self.mine.get(&head) {
            Some(&i) => self.frontier_of_index(i),
            None => Some(self.frontier_of_admitted(&head)),
        }
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
        Self::from_receipts_at(None, receipts)
    }

    /// Rehydrate a hot chain standing on a fold root (45.8), or on none.
    pub fn from_receipts_at(root: Option<Root>, receipts: Vec<KStepReceipt>) -> Option<ReceiptChain> {
        let vk = receipts.first().map(|r| r.vk).unwrap_or([0u8; 32]);
        // The index is built before integrity is verified, because
        // verification re-derives every height through it.
        let mut mine = Keyed::new();
        for (i, r) in receipts.iter().enumerate() {
            mine.insert(r.chain_hash, i);
        }
        // The carried frontiers are rebuilt by the same rule that appends
        // them, receipt by receipt in chain order: a rehydrated chain and a
        // chain that ran answer causal order identically. Parents whose
        // receipts this node has not readmitted contribute nothing, which
        // is what an unheld ancestor contributed while it ran.
        let mut c = ReceiptChain {
            receipts: Vec::new(),
            root,
            vk,
            known: Keyed::new(),
            mine,
            peer_at: Vec::new(),
            peers: vec![Keyed::new()],
        };
        for r in receipts {
            c.extend_frontier(r.vk, &r.parents);
            c.receipts.push(r);
        }
        if c.verify_chain_integrity() {
            Some(c)
        } else {
            None
        }
    }
}
