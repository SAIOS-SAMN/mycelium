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
    /// WHERE AN ADMITTED RECEIPT STANDS AGAINST THIS CHAIN'S OWN ORDER.
    /// One entry per peer receipt held; nothing at all per receipt of our
    /// own, because our own are TOTALLY ORDERED by the prev link and an
    /// index comparison settles any two of them exactly.
    ///
    /// A VECTOR CLOCK PER RECEIPT IS A PLANE. The frontier `F(r)` — signer
    /// ↦ greatest K reachable — is a map as wide as the mesh, and carrying
    /// one per receipt is receipts × peers: at a thousand peers each map is
    /// ~100 KB, one was materialized for every receipt that cited a parent,
    /// and every read of one cloned it whole. That is the Cartesian carrier
    /// the laws forbid, and re-deriving the same map by walking the
    /// ancestry (what stood before it) cost the chain's whole length per
    /// question instead. Neither is needed: see [`Placed`].
    placed: Keyed<Placed>,
}

/// A PEER RECEIPT'S TWO COORDINATES against this chain's own total order.
/// Both are `None` while unmeasured, which is not zero (rule 11).
///
/// Our receipts are a line: `mine[i]` is an ancestor of `mine[j]` exactly
/// when `i < j`. So a foreign receipt `p` needs only to say where it cuts
/// that line — below which of ours it sits, and above which — and every
/// order question against it is a comparison of one integer.
#[derive(Clone, Copy, Debug, Default)]
struct Placed {
    /// The GREATEST index of our own receipts that `p` reaches. Our
    /// receipts being a line, reaching `mine[m]` is reaching every
    /// `mine[i]` with `i ≤ m`, so `mine[i] ≺ p` iff `i ≤ ours_in`.
    ours_in: Option<usize>,
    /// The LEAST index of our own receipts that reaches `p` — the step at
    /// which this chain first cited it, directly or through a parent.
    /// Every later receipt of ours reaches it too, so `p ≺ mine[j]` iff
    /// `reach ≤ j`.
    reach: Option<usize>,
}

/// Where a chain hash stands: on this chain's own line, or beside it.
enum Stands {
    Own(usize),
    Peer(Placed),
}

impl Default for ReceiptChain {
    fn default() -> Self {
        ReceiptChain {
            receipts: Vec::new(),
            root: None,
            vk: [0u8; 32],
            known: Keyed::new(),
            mine: Keyed::new(),
            placed: Keyed::new(),
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

    /// THE RECEIPT AT ONE K, by arithmetic and not by a walk (law 3). The
    /// hot chain is contiguous from one past the fold root, so a K names
    /// its own position: `k − first_k`. The read is verified against the
    /// receipt it lands on, so a chain that is not contiguous answers
    /// nothing rather than the wrong receipt (rule 11).
    ///
    /// `receipts_range` walked `self.receipts` end to end to find one K —
    /// "no loop over peers, pairs, degrees, or planes that the carrier
    /// does not already name", and this carrier names every one of them.
    /// Every back-fill page on the fleet scanned the whole chain: at
    /// epoch 11, two thousand receipts per request, a hundred requests a
    /// minute, for a destination the index already knew.
    pub fn at_k(&self, k: u64) -> Option<&KStepReceipt> {
        let first = self.root.map_or(0, |r| r.k + 1);
        let i = k.checked_sub(first)?;
        let r = self.receipts.get(usize::try_from(i).ok()?)?;
        (r.step_index == k).then_some(r)
    }

    /// THE HEAD RECEIPT: the last one admitted. A chain is an ORDER — each
    /// receipt names its parent's hash and the whole chain re-derives from
    /// that — so its last member is the head by the chain's own structure,
    /// not the one that happened to arrive most recently (rule 15).
    ///
    /// One carrier for it: the three head readers below and `admit` each
    /// reached for the position themselves.
    pub fn head(&self) -> Option<&KStepReceipt> {
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — FIFO drain head: a chain's head is its last receipt by the chain's own order, which every link re-derives"
        )]
        self.receipts.last()
    }

    /// THE RECEIPT THIS HOT CHAIN OPENS AT — its genesis, or the first one
    /// admitted after a fold. The chain's own order again: its first member
    /// is where it begins.
    pub fn opening(&self) -> Option<&KStepReceipt> {
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — canonical key order: a chain opens at its first receipt, which is the chain's own structure"
        )]
        self.receipts.first()
    }

    /// The head's chain hash (the fold root's before the first hot receipt;
    /// all zeros before genesis).
    pub fn head_hash(&self) -> [u8; 32] {
        self.head()
            .map(|r| r.chain_hash)
            .or_else(|| self.root.map(|r| r.hash))
            // A chain before genesis has no head, and the empty hash is
            // what "no head" is on this chain — the reading of absence.
            .map_or([0u8; 32], |h| h)
    }

    /// The head receipt's step index. Zero before genesis.
    pub fn head_k(&self) -> u64 {
        self.head()
            .map(|r| r.step_index)
            .or_else(|| self.root.map(|r| r.k))
            // A chain before genesis stands at step zero.
            .map_or(0, |k| k)
    }

    /// TUniversal: height of the head receipt. Zero before genesis.
    pub fn head_height(&self) -> u64 {
        self.head()
            .map(|r| r.height)
            .or_else(|| self.root.map(|r| r.height))
            // A chain before genesis stands at height zero.
            .map_or(0, |h| h)
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
        self.mine = Keyed::new();
        for (i, r) in self.receipts.iter().enumerate() {
            self.mine.insert(r.chain_hash, i);
        }
        self.reindex_placed(n);
        self.root = Some(root);
        Some((cold, root))
    }

    /// A FOLD RENUMBERS OUR LINE, so every coordinate that cuts it moves
    /// with it (45.8). `n` receipts left the hot chain.
    ///
    /// `ours_in` pointing into the folded prefix becomes UNMEASURED: the
    /// receipts it named are no longer held, and no surviving receipt is
    /// below it. `reach` pointing into the prefix becomes zero: a receipt
    /// this chain reached before the fold is reached by every receipt that
    /// survives it, so the first of them is the answer.
    fn reindex_placed(&mut self, n: usize) {
        // A FOLD RENUMBERS OUR LINE, so every coordinate that cuts it moves
        // — every one, by definition of renumbering. `placed` holds one
        // entry per peer receipt this chain has admitted and its whole
        // content is what the fold moved.
        let keys: Vec<Vec<u8>> = self.placed.scoped().map(|(k, _)| k).collect();
        for k in keys {
            let Some(p) = self.placed.get_mut(&k) else { continue };
            p.ours_in = p.ours_in.and_then(|i| i.checked_sub(n));
            p.reach = match p.reach {
                Some(i) if i >= n => Some(i - n),
                Some(_) => Some(0),
                None => None,
            };
        }
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

    /// Admit a verified peer receipt so later parents can re-derive height,
    /// and PLACE it against this chain's own line while its ancestry is in
    /// hand (see [`Placed`]).
    pub fn admit(&mut self, r: KStepReceipt) {
        let ours_in = self.ours_in_of(&r);
        let held = self.placed.get(&r.chain_hash).copied();
        let reach = match held {
            Some(p) => p.reach,
            None => None,
        };
        self.placed.insert(r.chain_hash, Placed { ours_in, reach });
        self.known.insert(r.chain_hash, r);
    }

    /// THE GREATEST OF OUR OWN RECEIPTS A PEER RECEIPT REACHES.
    ///
    /// The descent runs over the ADMITTED set only, and it stops at two
    /// kinds of node: one of ours, whose index is the answer for that
    /// branch because our receipts are a line; and a peer receipt already
    /// placed, whose `ours_in` already folds in its whole ancestry. Every
    /// admitted receipt is placed once, at admit, so the descent almost
    /// always stops at its parents — the work is amortized to the arity of
    /// a receipt, not to the size of anything.
    ///
    /// The hard bound is the held set: a descent cannot visit more peer
    /// receipts than this chain holds (rule 10 — a real boundary, not a
    /// guess). Ancestry this node does not hold contributes nothing, which
    /// is exactly what an unheld ancestor contributed before.
    fn ours_in_of(&self, r: &KStepReceipt) -> Option<usize> {
        let mut ours_in: Option<usize> = None;
        let mut seen: Keyed<()> = Keyed::new();
        let mut stack: Vec<[u8; 32]> = Vec::new();
        if r.prev_receipt_hash != [0u8; 32] {
            stack.push(r.prev_receipt_hash);
        }
        for p in &r.parents {
            stack.push(p.hash);
        }
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
                ours_in = Some(match ours_in {
                    Some(m) if m >= i => m,
                    _ => i,
                });
                continue;
            }
            if let Some(p) = self.placed.get(&hash) {
                if let Some(i) = p.ours_in {
                    ours_in = Some(match ours_in {
                        Some(m) if m >= i => m,
                        _ => i,
                    });
                }
                continue;
            }
            let Some(k) = self.known.get(&hash) else { continue };
            if k.prev_receipt_hash != [0u8; 32] {
                stack.push(k.prev_receipt_hash);
            }
            for p in &k.parents {
                stack.push(p.hash);
            }
        }
        ours_in
    }

    /// OUR RECEIPT AT `n` NOW REACHES EVERYTHING ITS PARENTS REACH, so
    /// every peer receipt in that ancestry is first reached at `n` unless
    /// it was reached before.
    ///
    /// The descent stops at any receipt already carrying a `reach`: if it
    /// is reached, so is all of its ancestry, and so the stop is exact and
    /// not an economy. Each peer receipt therefore has its `reach` written
    /// once in the life of the chain, which makes the whole of this
    /// amortize to nothing across appends.
    fn mark_reached(&mut self, n: usize, parents: &[Parent]) {
        let mut stack: Vec<[u8; 32]> = parents.iter().map(|p| p.hash).collect();
        let mut left = self.known.len() + 1;
        while let Some(hash) = stack.pop() {
            if left == 0 {
                break;
            }
            left -= 1;
            if self.mine.contains(&hash) {
                continue;
            }
            match self.placed.get_mut(&hash) {
                Some(p) if p.reach.is_some() => continue,
                Some(p) => p.reach = Some(n),
                None => {
                    self.placed.insert(hash, Placed { ours_in: None, reach: Some(n) });
                }
            }
            let Some(k) = self.known.get(&hash) else { continue };
            let (prev, up) = (k.prev_receipt_hash, k.parents.clone());
            if prev != [0u8; 32] {
                stack.push(prev);
            }
            for p in &up {
                stack.push(p.hash);
            }
        }
    }

    /// Where a chain hash stands against this chain's line.
    fn stands(&self, hash: &[u8; 32]) -> Option<Stands> {
        if let Some(&i) = self.mine.get(hash) {
            return Some(Stands::Own(i));
        }
        let p = self.placed.get(hash).copied();
        match (p, self.known.get(hash)) {
            (Some(p), _) => Some(Stands::Peer(p)),
            (None, Some(_)) => Some(Stands::Peer(Placed::default())),
            (None, None) => None,
        }
    }

    /// Does `b` reach `a`, both being peer receipts? The descent is over
    /// the admitted set and stops at our own line, which no peer receipt's
    /// ancestry continues through to another peer receipt of a later
    /// admission. Bounded by the held set.
    fn peer_reaches(&self, from: &[u8; 32], to: &[u8; 32]) -> bool {
        let mut seen: Keyed<()> = Keyed::new();
        let mut stack: Vec<[u8; 32]> = vec![*from];
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
            let Some(k) = self.known.get(&hash) else { continue };
            if k.prev_receipt_hash == *to {
                return true;
            }
            if k.parents.iter().any(|p| p.hash == *to) {
                return true;
            }
            if k.prev_receipt_hash != [0u8; 32] {
                stack.push(k.prev_receipt_hash);
            }
            for p in &k.parents {
                stack.push(p.hash);
            }
        }
        false
    }

    /// A peer receipt this chain admitted, by its chain hash.
    pub fn known(&self, hash: &[u8; 32]) -> Option<&KStepReceipt> {
        self.known.get(hash)
    }

    /// EVERY PEER RECEIPT THIS CHAIN HOLDS, for the ledger to write. They
    /// are not this court's own history but they are part of what its
    /// history MEANS: our own receipts cite them as parents, and a height
    /// that cites an unheld parent cannot be re-derived at all.
    pub fn admitted(&self) -> Vec<&KStepReceipt> {
        // The ledger writes the admitted set WHOLE, as it writes our own
        // line whole: a peer receipt left out is a height that cannot be
        // re-derived at the next boot.
        self.known.scoped().map(|(_, r)| r).collect()
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
        self.mark_reached(self.receipts.len(), &parents);
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
        self.head()
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

    /// `a ≺ b`: is `a` a proper ancestor of `b` in what this chain holds
    /// (45.3)? `None` when the budget is spent or this chain does not hold
    /// one of them — UNMEASURED order, which a caller holds as unordered
    /// rather than reading as "does not precede" (rule 11).
    ///
    /// FOUR CASES, AND THREE OF THEM ARE INTEGER COMPARISONS, because our
    /// own receipts are a line and a peer receipt says where it cuts it
    /// ([`Placed`]). Only two foreign receipts need a descent, and that one
    /// runs over the admitted set.
    ///
    /// This compared two VECTOR CLOCKS, one per receipt, each as wide as
    /// the mesh — and it built them by walking the ancestry from scratch,
    /// twice, one budget spend per visit. The walk cost the chain's length
    /// per question (so on the fleet, at K=1704 against a stroke of 3, it
    /// never once measured); carrying the clocks instead cost receipts ×
    /// peers in memory. The order of two receipts is not a plane.
    pub fn precedes(
        &self,
        a: &KStepReceipt,
        b: &KStepReceipt,
        budget: &mut Budget,
    ) -> Option<bool> {
        if !budget.spend() {
            return None;
        }
        match (self.stands(&a.chain_hash)?, self.stands(&b.chain_hash)?) {
            // Both ours: the prev link IS the order.
            (Stands::Own(i), Stands::Own(j)) => Some(i < j),
            // Ours below a peer's: the peer reaches our line up to
            // `ours_in`, and reaching one of ours reaches all before it.
            (Stands::Own(i), Stands::Peer(p)) => Some(p.ours_in.is_some_and(|m| i <= m)),
            // A peer below ours: we first reached it at `reach`, and every
            // receipt of ours after that reaches it too.
            (Stands::Peer(p), Stands::Own(j)) => Some(p.reach.is_some_and(|r| r <= j)),
            // Two peers: neither cuts the other through our line, so the
            // question is whether one is in the other's held ancestry.
            (Stands::Peer(_), Stands::Peer(_)) => {
                Some(self.peer_reaches(&b.chain_hash, &a.chain_hash))
            }
        }
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

    /// Rehydrate a hot chain standing on a fold root (45.8), or on none,
    /// with NO peer receipts — which is a chain whose own receipts cite no
    /// parent outside itself. A restore reads the admitted set from the
    /// ledger and calls [`ReceiptChain::from_parts`].
    pub fn from_receipts_at(root: Option<Root>, receipts: Vec<KStepReceipt>) -> Option<ReceiptChain> {
        Self::from_parts(root, receipts, Vec::new())
    }

    /// REHYDRATE A CHAIN WHOLE: its own line, and the peer receipts its
    /// line cites as parents.
    ///
    /// A HEIGHT THAT CITES AN UNHELD PARENT IS UNMEASURABLE, and
    /// `verify_chain_integrity` re-derives every height, so a chain whose
    /// receipts cite peers cannot verify without them. The admitted set
    /// used to be dropped at persist and started empty here: on the fleet
    /// at fc75a59 hypha b's 16 MB ledger refused with `ledger refused in
    /// section KCH1 at byte 14324354 of 16736157`
    /// — forty of its receipts named peer parents, `parent_height`
    /// answered `None` for every one of them, and a whole court's memory
    /// was dropped and overwritten with an empty one on the next persist.
    /// No corruption was involved. The ledger was exact and this reader
    /// could not read what it had itself written.
    ///
    /// The peers arrive verified BY CONTENT: a peer receipt's chain hash
    /// covers its own height, parents and prev link, so re-deriving the
    /// hash is the whole check this court can make on its own (law 2). A
    /// peer receipt whose hash does not recompute refuses the tape, the
    /// same as one of ours.
    ///
    /// Then the two coordinates of [`Placed`] are re-derived rather than
    /// persisted, because both are functions of what is held: `ours_in` by
    /// the same descent `admit` runs, in the DAG's own order so every
    /// ancestor is placed before its descendant; `reach` by replaying the
    /// mark each of our own appends made.
    pub fn from_parts(
        root: Option<Root>,
        receipts: Vec<KStepReceipt>,
        peers: Vec<KStepReceipt>,
    ) -> Option<ReceiptChain> {
        // An empty chain names no signer; the empty key is that reading,
        // and every receipt admitted later must agree with it.
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — canonical key order: a chain opens at its genesis receipt, so position zero is the chain's own origin and the signer is read there"
        )]
        let vk = receipts.first().map_or([0u8; 32], |r| r.vk);
        // The index is built before integrity is verified, because
        // verification re-derives every height through it.
        let mut mine = Keyed::new();
        for (i, r) in receipts.iter().enumerate() {
            mine.insert(r.chain_hash, i);
        }
        let mut c = ReceiptChain {
            receipts,
            root,
            vk,
            known: Keyed::new(),
            mine,
            placed: Keyed::new(),
        };
        // The admitted set goes in BEFORE integrity is verified, since
        // verification re-derives a height through every parent.
        let mut order = peers;
        for r in &order {
            if r.chain_hash
                != KStepReceipt::hash(
                    r.step_index,
                    r.height,
                    &r.parents,
                    r.kind,
                    &r.mu,
                    &r.delta,
                    &r.operator_hash,
                    &r.prev_receipt_hash,
                )
            {
                return None;
            }
            c.known.insert(r.chain_hash, r.clone());
        }
        if !c.verify_chain_integrity() {
            return None;
        }
        // THE DAG'S OWN ORDER, and no other: a receipt's height is
        // `1 + max` over its parents', so a parent's height is strictly
        // below its child's and this pass places every ancestor first --
        // which is what makes each descent stop at its parents instead of
        // walking the held set.
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 4 — a sort by a canonical key is the key's own order: height is the receipt's own content coordinate and the partial order of the ancestry, made total"
        )]
        order.sort_unstable_by_key(|r| r.height);
        for r in &order {
            let ours_in = c.ours_in_of(r);
            c.placed.insert(r.chain_hash, Placed { ours_in, reach: None });
        }
        // `reach` as our own line wrote it: each append marked everything
        // its parents reach as first reached at that step.
        for i in 0..c.receipts.len() {
            let parents = c.receipts.get(i).map(|r| r.parents.clone())?;
            c.mark_reached(i, &parents);
        }
        Some(c)
    }
}
