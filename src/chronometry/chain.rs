// 45, 78.24 — THE LEDGER: a partial order of acts carried as one, keyed by
// the stamp, and the order two receipts stand in.
//
//! Every node this court holds — its own line and the peer receipts it
//! admitted — sits ONCE at its stamp `h‖vk‖K‖i`. `fiber(h)` is the mesh at
//! one tick, `fiber(h‖vk)` one signer at it. A signer's line is read by
//! `vk‖K‖i`, the one relation the range readers ask by (78.24 P1), and the
//! append order is one reader's trajectory through the carrier, never the
//! carrier.

use super::era::Root;
use super::receipt::{ActKind, Cite, KStepReceipt, Parent, Place, Stamp};
use crate::network::nerve::Keyed;
use crate::physics::Budget;

/// THE LEDGER.
#[derive(Debug, Clone, Default)]
pub struct ReceiptChain {
    /// Every node held, keyed by its stamp.
    nodes: Keyed<KStepReceipt>,
    /// `vk‖K‖i → h`: each signer's line, coordinates only.
    lines: Keyed<u64>,
    /// `vk → (K, i)`: the greatest place held on each signer's line —
    /// where 47.2 step 3 reads a signer's head. Kept where a node is held,
    /// rebuilt at restore from the nodes.
    tops: Keyed<Place>,
    /// THE FOLD ROOT (45.8): the archived prefix's head this chain's own
    /// line stands on, or none before the first fold.
    root: Option<Root>,
    vk: [u8; 32],
    /// The stamp of this chain's own last receipt.
    head: Option<Stamp>,
    /// WHERE A PEER NODE STANDS AGAINST THIS CHAIN'S OWN LINE, by our
    /// place, keyed by the peer's stamp (see [`Placed`]).
    placed: Keyed<Placed>,
}

/// A PEER NODE'S TWO COORDINATES against this chain's own line, as our
/// place `(K, i)`. Our receipts are a line — the place orders them exactly
/// — so a peer node needs only say where it cuts it. `None` while
/// unmeasured, which is not zero (rule 11). A fold removes receipts from
/// the line and renumbers nothing: a place stays the coordinate it was.
#[derive(Clone, Copy, Debug, Default)]
struct Placed {
    /// The GREATEST of our places the peer node reaches: `ours(a) ≺ p` iff
    /// `a ≤ ours_in`.
    ours_in: Option<Place>,
    /// The LEAST of our places that reaches the peer node: `p ≺ ours(a)`
    /// iff `reach ≤ a`.
    reach: Option<Place>,
}

/// Where a stamp stands: on this chain's own line, or beside it.
enum Stands {
    Own(Place),
    Peer(Placed),
}

/// The larger of a held reading and a new one; an unmeasured reading
/// takes the new.
fn raise(held: Option<Place>, at: Place) -> Option<Place> {
    Some(match held {
        Some(m) if m >= at => m,
        _ => at,
    })
}

impl ReceiptChain {
    pub fn new() -> Self {
        ReceiptChain::default()
    }

    pub fn with_vk(vk: [u8; 32]) -> Self {
        ReceiptChain { vk, ..ReceiptChain::default() }
    }

    /// The key this chain signs as.
    pub fn vk(&self) -> [u8; 32] {
        self.vk
    }

    /// The place the hot line opens at: genesis, or the one after the root.
    fn first_place(&self) -> Place {
        self.root.map_or(Place::GENESIS, |r| self.after(r.at))
    }

    /// THE PLACE AN ACT TAKES AFTER ONE AT `at`: the first act of the next
    /// orbit — every act closes its orbit until the cycles close by name.
    fn after(&self, at: Place) -> Place {
        at.beyond()
    }

    /// The place the next act of this chain's own line takes.
    pub fn next_place(&self) -> Place {
        match self.head {
            Some(s) => self.after(s.place()),
            None => self.first_place(),
        }
    }

    /// The orbit the next act stands in — the current `K`.
    pub fn next_step_index(&self) -> u64 {
        self.next_place().k
    }

    /// The node at one stamp.
    pub fn node(&self, at: &Stamp) -> Option<&KStepReceipt> {
        self.nodes.get(&at.key())
    }

    /// The node a signer holds at one place, through the line relation.
    pub fn at(&self, vk: &[u8; 32], at: Place) -> Option<&KStepReceipt> {
        let h = *self.lines.get(&Stamp::line_key(vk, at))?;
        self.node(&Stamp::new(h, *vk, at))
    }

    /// This chain's own receipt at one place.
    pub fn at_place(&self, at: Place) -> Option<&KStepReceipt> {
        self.at(&self.vk, at)
    }

    /// A SIGNER'S LINE BETWEEN TWO PLACES: the nodes strictly after
    /// `after` (from the line's start where `None`) up to and including
    /// `upto`, in line order, at most `n` — read off the line relation by
    /// the signer's prefix, never by counting places.
    pub fn line_between(&self, vk: &[u8; 32], after: Option<Place>, upto: Place, n: usize) -> Vec<&KStepReceipt> {
        let Some(line) = self.lines.fiber_at(vk) else { return Vec::new() };
        line.filter_map(|(key, &h)| {
            let mut p = 32usize;
            let at = Place::take_canonical(&key, &mut p)?;
            (after.is_none_or(|a| a < at) && at <= upto).then_some((at, h))
        })
        .take(n)
        .filter_map(|(at, h)| self.node(&Stamp::new(h, *vk, at)))
        .collect()
    }

    /// A SIGNER'S HEAD as this ledger holds it: the node at the greatest
    /// place on its line.
    pub fn line_head(&self, vk: &[u8; 32]) -> Option<&KStepReceipt> {
        self.at(vk, *self.tops.get(vk)?)
    }

    /// Every signer this ledger holds a line of, this chain's own
    /// included.
    pub fn signers(&self) -> Vec<[u8; 32]> {
        self.tops.scoped().filter_map(|(k, _)| <[u8; 32]>::try_from(k.as_slice()).ok()).collect()
    }

    /// File one node's stamp on its signer's line, raising the line's top.
    fn file_on_line(&mut self, at: &Stamp) {
        self.lines.insert(Stamp::line_key(&at.vk, at.place()), at.h);
        let top = raise(self.tops.get(&at.vk).copied(), at.place());
        if let Some(top) = top {
            self.tops.insert(at.vk, top);
        }
    }

    /// THE HEAD RECEIPT: this chain's own last.
    pub fn head(&self) -> Option<&KStepReceipt> {
        self.node(&self.head?)
    }

    /// THE RECEIPT THIS HOT LINE OPENS AT — genesis, or the first after a
    /// fold.
    pub fn opening(&self) -> Option<&KStepReceipt> {
        self.at_place(self.first_place())
    }

    /// The head's chain hash (the root's before the first hot receipt; the
    /// empty hash before genesis — the reading of absence).
    pub fn head_hash(&self) -> [u8; 32] {
        self.head()
            .map(|r| r.chain_hash)
            .or_else(|| self.root.map(|r| r.hash))
            .map_or([0u8; 32], |h| h)
    }

    /// The head's stamp, or the root's before the first hot receipt.
    pub fn head_stamp(&self) -> Option<Stamp> {
        self.head.or_else(|| self.root.map(|r| Stamp::new(r.height, self.vk, r.at)))
    }

    /// The head receipt's place, or none before genesis.
    pub fn head_place(&self) -> Option<Place> {
        self.head_stamp().map(|s| s.place())
    }

    /// The head receipt's place as a line prints it, `K.i`, or `none`
    /// before genesis.
    pub fn head_label(&self) -> String {
        self.head_place().map_or_else(|| "none".to_string(), Place::print)
    }

    /// The head receipt's orbit `K`. Zero before genesis.
    pub fn head_k(&self) -> u64 {
        self.head_stamp().map_or(0, |s| s.k)
    }

    /// TUniversal: the head's height. Zero before genesis.
    pub fn head_height(&self) -> u64 {
        self.head_stamp().map_or(0, |s| s.h)
    }

    /// The fold root this chain stands on, if it was ever folded.
    pub fn root(&self) -> Option<Root> {
        self.root
    }

    /// THIS CHAIN'S OWN LINE, in line order: the line relation read by the
    /// signer's prefix.
    pub fn own(&self) -> impl Iterator<Item = &KStepReceipt> + '_ {
        let vk = self.vk;
        self.lines.fiber_at(&vk).into_iter().flatten().filter_map(move |(key, &h)| {
            let mut p = 32usize;
            let at = Place::take_canonical(&key, &mut p)?;
            self.node(&Stamp::new(h, vk, at))
        })
    }

    /// How many receipts the hot line holds: its nodes counted, never a
    /// difference of places.
    pub fn own_len(&self) -> u64 {
        self.own().count() as u64
    }

    /// EVERY NODE AT ONE TICK — the mesh's width at `h`.
    pub fn at_tick(&self, h: u64) -> Vec<&KStepReceipt> {
        match self.nodes.fiber_at(&h.to_be_bytes()) {
            Some(f) => f.map(|(_, r)| r).collect(),
            None => Vec::new(),
        }
    }

    /// Every node held, in stamp order: the ledger as it is persisted.
    pub fn nodes(&self) -> impl Iterator<Item = &KStepReceipt> + '_ {
        self.nodes.scoped().map(|(_, r)| r)
    }

    /// FOLD (45.8): this chain's own receipts at or below `upto` leave the
    /// ledger and are returned, cold, in line order, with the root they
    /// leave the line standing on. `None` when no hot receipt sits at or
    /// below `upto`.
    pub fn fold_before(&mut self, upto: Place) -> Option<(Vec<KStepReceipt>, Root)> {
        self.head?;
        let vk = self.vk;
        let places: Vec<Place> = self.line_between(&vk, None, upto, usize::MAX).iter().map(|r| r.stamp.place()).collect();
        let mut cold = Vec::new();
        let mut top = None;
        for at in places {
            let Some(h) = self.lines.remove(&Stamp::line_key(&vk, at)) else { continue };
            if let Some(r) = self.nodes.remove(&Stamp::new(h, vk, at).key()) {
                top = Some(Root { at: r.stamp.place(), hash: r.chain_hash, height: r.stamp.h });
                cold.push(r);
            }
        }
        let root = top?;
        if self.head.is_some_and(|h| h.place() <= root.at) {
            self.head = None;
        }
        self.root = Some(root);
        Some((cold, root))
    }

    /// THE HEIGHT A PARENT NAMES, where this chain holds the node at its
    /// stamp with its hash. `None` otherwise: UNMEASURED height, which a
    /// caller re-deriving a claim refuses rather than reads as zero.
    pub fn height_of(&self, p: &Parent) -> Option<u64> {
        let held = self.node(&p.at)?;
        (held.chain_hash == p.hash).then_some(p.at.h)
    }

    /// THE PREDECESSOR A ROOT OR A HEAD IS, as the next receipt names it.
    fn line_prev(&self) -> Option<Parent> {
        match self.head() {
            Some(r) => Some(Parent { at: r.stamp, hash: r.chain_hash }),
            None => self.root.map(|r| Parent { at: Stamp::new(r.height, self.vk, r.at), hash: r.hash }),
        }
    }

    /// Bind this chain to the key that signs for it. The key is a
    /// coordinate of every stamp and in every hash, so a chain that has
    /// sealed under another key cannot be rebound: `false` then, and the
    /// chain keeps the key it has.
    pub fn bind_vk(&mut self, vk: [u8; 32]) -> bool {
        if self.vk == vk {
            return true;
        }
        if self.head.is_some() || self.root.is_some() {
            return false;
        }
        self.vk = vk;
        true
    }

    /// ADMIT a verified peer node: held once at its stamp, entered on its
    /// signer's line, and PLACED against this chain's own line.
    pub fn admit(&mut self, r: KStepReceipt) {
        if r.stamp.vk == self.vk || self.nodes.contains(&r.stamp.key()) {
            return;
        }
        let ours_in = self.ours_in_of(&r);
        let reach = self.placed.get(&r.stamp.key()).and_then(|p| p.reach);
        self.placed.insert(r.stamp.key(), Placed { ours_in, reach });
        self.file_on_line(&r.stamp);
        self.nodes.insert(r.stamp.key(), r);
    }

    /// The edges out of a node: its predecessor and its parents.
    fn edges(r: &KStepReceipt) -> impl Iterator<Item = Stamp> + '_ {
        r.prev.iter().chain(r.parents.iter()).map(|p| p.at)
    }

    /// THE GREATEST OF OUR OWN PLACES A PEER NODE REACHES. The descent runs
    /// over the held nodes and stops at one of ours (the line settles it)
    /// or at a peer node already placed; the held set bounds it (rule 10).
    fn ours_in_of(&self, r: &KStepReceipt) -> Option<Place> {
        let mut ours_in = None;
        let mut seen: Keyed<()> = Keyed::new();
        let mut stack: Vec<Stamp> = Self::edges(r).collect();
        let mut left = self.nodes.len() + 1;
        while let Some(at) = stack.pop() {
            if left == 0 {
                break;
            }
            left -= 1;
            let key = at.key();
            if seen.contains(&key) {
                continue;
            }
            seen.insert(&key, ());
            if at.vk == self.vk {
                ours_in = raise(ours_in, at.place());
                continue;
            }
            if let Some(p) = self.placed.get(&key) {
                if let Some(i) = p.ours_in {
                    ours_in = raise(ours_in, i);
                }
                continue;
            }
            let Some(held) = self.nodes.get(&key) else { continue };
            stack.extend(Self::edges(held));
        }
        ours_in
    }

    /// OUR RECEIPT AT `k` REACHES EVERYTHING ITS PARENTS REACH: each peer
    /// node in that ancestry is first reached at `k` unless it was before.
    fn mark_reached(&mut self, k: Place, parents: &[Parent]) {
        let mut stack: Vec<Stamp> = parents.iter().map(|p| p.at).collect();
        let mut left = self.nodes.len() + 1;
        while let Some(at) = stack.pop() {
            if left == 0 {
                break;
            }
            left -= 1;
            if at.vk == self.vk {
                continue;
            }
            let key = at.key();
            match self.placed.get_mut(&key) {
                Some(p) if p.reach.is_some() => continue,
                Some(p) => p.reach = Some(k),
                None => {
                    self.placed.insert(&key, Placed { ours_in: None, reach: Some(k) });
                }
            }
            let Some(held) = self.nodes.get(&key) else { continue };
            let up: Vec<Stamp> = Self::edges(held).collect();
            stack.extend(up);
        }
    }

    fn stands(&self, at: &Stamp) -> Option<Stands> {
        if at.vk == self.vk {
            return self.node(at).map(|_| Stands::Own(at.place()));
        }
        let key = at.key();
        match (self.placed.get(&key).copied(), self.nodes.contains(&key)) {
            (Some(p), _) => Some(Stands::Peer(p)),
            (None, true) => Some(Stands::Peer(Placed::default())),
            (None, false) => None,
        }
    }

    /// Does `from` reach `to`, both peer nodes? Bounded by the held set.
    fn peer_reaches(&self, from: &Stamp, to: &Stamp) -> bool {
        let mut seen: Keyed<()> = Keyed::new();
        let mut stack = vec![*from];
        let mut left = self.nodes.len() + 1;
        while let Some(at) = stack.pop() {
            if left == 0 {
                break;
            }
            left -= 1;
            let key = at.key();
            if seen.contains(&key) {
                continue;
            }
            seen.insert(&key, ());
            let Some(held) = self.nodes.get(&key) else { continue };
            for up in Self::edges(held) {
                if up == *to {
                    return true;
                }
                stack.push(up);
            }
        }
        false
    }

    /// EVERY PEER NODE THIS CHAIN HOLDS.
    pub fn admitted(&self) -> Vec<&KStepReceipt> {
        self.nodes().filter(|r| r.stamp.vk != self.vk).collect()
    }

    /// THE STAMP THE NEXT RECEIPT CITING `parents` WILL TAKE: this chain's
    /// key, the next place, and the height `1 + max` over the predecessor and
    /// the parents. An act that admits what it names cites itself by it.
    /// `None` where a parent is not held.
    pub fn next_stamp(&self, parents: &[Parent]) -> Option<Stamp> {
        let mut heights = Vec::with_capacity(parents.len() + 1);
        if let Some(p) = self.line_prev() {
            heights.push(p.at.h);
        }
        for q in parents {
            heights.push(self.height_of(q)?);
        }
        Some(Stamp::new(height_over(&heights), self.vk, self.next_place()))
    }

    /// Append the next receipt (45.1): its stamp is this chain's key at
    /// the next place and the height `1 + max` over its predecessor and the
    /// parents it cites, each of which this chain must hold (an unheld
    /// parent is unmeasured height, and the append refuses). `None` where
    /// a parent is not held or a citation cannot be written (LD0).
    pub fn append(
        &mut self,
        kind: ActKind,
        mu: [u64; 4],
        cites: Vec<Cite>,
        operator_hash: [u8; 32],
        parents: Vec<Parent>,
    ) -> Option<&KStepReceipt> {
        let stamp = self.next_stamp(&parents)?;
        let k = stamp.place();
        let prev = self.line_prev();
        let r = KStepReceipt::seal(stamp, kind, mu, prev, parents, cites, operator_hash)?;
        self.mark_reached(k, &r.parents);
        self.file_on_line(&stamp);
        self.nodes.insert(stamp.key(), r);
        self.head = Some(stamp);
        self.head()
    }

    /// Verify the hot line: contiguous from its opening — every node's
    /// predecessor the one before it (or the root), and its place one that
    /// follows the predecessor's — every height re-derived from its
    /// predecessor and parents, every hash re-derived from its coordinates.
    pub fn verify_chain_integrity(&self) -> bool {
        let mut prev = self.root.map(|r| Parent { at: Stamp::new(r.height, self.vk, r.at), hash: r.hash });
        let mut last = None;
        for r in self.own() {
            let follows = match prev {
                Some(p) => p.at.place().is_followed_by(r.stamp.place()),
                None => r.stamp.place() == Place::GENESIS,
            };
            if !follows || r.prev != prev {
                return false;
            }
            let mut heights: Vec<u64> = prev.iter().map(|p| p.at.h).collect();
            for q in &r.parents {
                let Some(ph) = self.height_of(q) else { return false };
                heights.push(ph);
            }
            let want_h = height_over(&heights);
            if r.stamp.h != want_h || !r.rederives() {
                return false;
            }
            prev = Some(Parent { at: r.stamp, hash: r.chain_hash });
            last = Some(r.stamp);
        }
        self.head == last
    }

    /// `a ≺ b` (45.3): is `a` a proper ancestor of `b` in what this chain
    /// holds? `None` when the budget is spent or a stamp is not held —
    /// UNMEASURED order, held as unordered, never read as "does not
    /// precede" (rule 11). Three of the four cases are comparisons of our
    /// own `K`; only two peer nodes need a descent.
    pub fn precedes(&self, a: &Stamp, b: &Stamp, budget: &mut Budget) -> Option<bool> {
        if !budget.spend() {
            return None;
        }
        match (self.stands(a)?, self.stands(b)?) {
            (Stands::Own(i), Stands::Own(j)) => Some(i < j),
            (Stands::Own(i), Stands::Peer(p)) => Some(p.ours_in.is_some_and(|m| i <= m)),
            (Stands::Peer(p), Stands::Own(j)) => Some(p.reach.is_some_and(|r| r <= j)),
            (Stands::Peer(_), Stands::Peer(_)) => Some(self.peer_reaches(b, a)),
        }
    }

    /// REHYDRATE THE LEDGER from its persisted nodes: this chain's own
    /// line (the nodes signed by `vk`) and the peer nodes it admitted, on
    /// the root it stands on. Every node re-derives its own hash, the line
    /// verifies whole, and the peer placements are re-derived rather than
    /// persisted — in height order, so every ancestor is placed first.
    /// `None` refuses the whole ledger.
    pub fn from_nodes(root: Option<Root>, vk: [u8; 32], nodes: Vec<KStepReceipt>) -> Option<ReceiptChain> {
        let mut c = ReceiptChain::with_vk(vk);
        c.root = root;
        let mut peers = Vec::new();
        for r in nodes {
            if !r.rederives() {
                return None;
            }
            c.file_on_line(&r.stamp);
            if r.stamp.vk == vk {
                c.head = Some(match c.head {
                    Some(h) if h.place() >= r.stamp.place() => h,
                    _ => r.stamp,
                });
            } else {
                peers.push(r.stamp);
            }
            c.nodes.insert(r.stamp.key(), r);
        }
        if !c.verify_chain_integrity() {
            return None;
        }
        // THE DAG'S OWN ORDER: a parent's height is strictly below its
        // child's, and the stamps were read in key order, which is height
        // first — so every ancestor is placed before its descendant.
        for at in peers {
            let r = c.node(&at)?.clone();
            let ours_in = c.ours_in_of(&r);
            c.placed.insert(at.key(), Placed { ours_in, reach: None });
        }
        let own: Vec<(Place, Vec<Parent>)> = c.own().map(|r| (r.stamp.place(), r.parents.clone())).collect();
        for (k, parents) in own {
            c.mark_reached(k, &parents);
        }
        Some(c)
    }

    /// A chain rehydrated from its own line alone, in `K` order, on no
    /// root — a line whose receipts cite no peer.
    pub fn from_receipts(receipts: Vec<KStepReceipt>) -> Option<ReceiptChain> {
        let vk = receipts.iter().map(|r| r.stamp.vk).next().map_or([0u8; 32], |v| v);
        Self::from_nodes(None, vk, receipts)
    }

    /// THE LEDGER WRITTEN AS THE TRIE (78.24 (c)): the key this chain signs
    /// as, the root, then every node in stamp order, each once. `None`
    /// where a node cannot be written (LD0).
    #[must_use = "a ledger that cannot be written is refused by name, never dropped (LD0)"]
    pub fn write(&self, out: &mut Vec<u8>) -> Option<()> {
        let mut rec = Vec::new();
        rec.extend_from_slice(&self.vk);
        super::era::Root::write_opt(self.root.as_ref(), &mut rec);
        rec.extend_from_slice(&(self.nodes.len() as u64).to_le_bytes());
        for r in self.nodes() {
            r.write(&mut rec)?;
        }
        out.append(&mut rec);
        Some(())
    }

    /// The inverse of [`ReceiptChain::write`], whole or refused.
    pub fn read(b: &[u8], pos: &mut usize) -> Option<ReceiptChain> {
        let vk: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        let root = super::era::Root::read_opt(b, pos)?;
        let n = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?) as usize;
        *pos += 8;
        if n > b.len().checked_sub(*pos)? / KStepReceipt::MIN_TAPE {
            return None;
        }
        let mut nodes = Vec::with_capacity(n);
        for _ in 0..n {
            nodes.push(KStepReceipt::read(b, pos)?);
        }
        Self::from_nodes(root, vk, nodes)
    }
}

/// 45.2 — THE HEIGHT A RECEIPT TAKES over the heights it cites, its
/// predecessor's and its parents': `1 + max`, and `0` only where it cites
/// neither — the mesh genesis alone. A line's FIRST receipt that cites
/// parents stands above them; it was 0 wherever the line had no
/// predecessor (HT1). One statement for the writer (`next_stamp`), the
/// admission's re-derivation and the ledger's integrity check (law 12).
pub fn height_over(cited: &[u64]) -> u64 {
    match cited.iter().fold(None, |m: Option<u64>, &h| Some(m.map_or(h, |m| m.max(h)))) {
        None => 0,
        Some(m) => 1 + m,
    }
}
