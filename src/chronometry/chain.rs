// 45, 78.24 — THE LEDGER: a partial order of acts carried as one, keyed by
// the stamp, and the order two receipts stand in.
//
//! Every node this court holds — its own line and the peer receipts it
//! admitted — sits ONCE at its stamp `h‖vk‖K‖i`. `fiber(h)` is the mesh at
//! one tick, `fiber(h‖vk)` one signer at it. A signer's line is read by
//! `vk‖K‖i`, the one relation the range readers ask by (78.24 P1), and the
//! append order is one reader's trajectory through the carrier, never the
//! carrier. A node holds what its act carries and references for what its
//! line already holds (`node.rs`); a receipt is assembled where one is
//! asked for.

use super::era::Root;
use super::receipt::{ActKind, Cite, KStepReceipt, Parent, Place, Stamp};
use crate::network::nerve::Keyed;
use crate::physics::Budget;

/// THE LEDGER.
#[derive(Debug, Clone, Default)]
pub struct ReceiptChain {
    /// Every node held, keyed by its stamp.
    nodes: Keyed<node::Node>,
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
    /// FOLDED RECEIPTS A CITATION RECALLED: `stamp → chain hash`, each one
    /// read back out of an archived era and proved there by its stamp and
    /// hash (`recall_folded`), held while the admission that asked reads
    /// it.
    folded: Keyed<[u8; 32]>,
    /// `vk → root`: THE ROOT EACH PEER'S OWN FOLD ARCHIVED ITS LINE TO, as
    /// its `Fold` node cites it — its stamp and hash — where `shed_folded`
    /// cuts that line and against which a citation into the closed era is
    /// proved. Kept where a peer's fold node is held, rebuilt at restore
    /// from the nodes.
    peer_roots: Keyed<Parent>,
    /// FOLDED PEER RECEIPTS PROVED FROM THEIR ERA (78.65, 78.87 R2): `stamp
    /// → chain hash`, each one a citation into a closed era whose chain was
    /// walked from it up to the peer's fold root (`prove_from_era`). A
    /// closed era returns nothing new, so a proof stands; it is not
    /// persisted, and is proved again where a citation asks after restore.
    proved: Keyed<[u8; 32]>,
    /// AN ERA PROOF IN PROGRESS: `p`'s key and hash → the last receipt of
    /// the signer's line the walk from `p` has re-derived, where the page
    /// that carried it ended short of the root (`walk_era`). The next page
    /// is asked from there, so a proof longer than one frame closes page by
    /// page across hearings, never asked again from `p`. Volatile; an entry
    /// leaves when its proof stands or breaks.
    walking: Keyed<Parent>,
    /// THE OPERATORS the ledger has met, each once, by the index a node
    /// names it with.
    operators: Vec<[u8; 32]>,
    operator_at: Keyed<usize>,
    /// WHAT THE LINE ON DISK DOES NOT YET CARRY (`line`): the nodes filed
    /// and the nodes that left since it was last written.
    journal: line::Journal,
    /// 78.151 — `vk → LineTally`: each signer's line, how many nodes it
    /// holds and the mark of its last change, so a published reading of
    /// the line is re-read only where the line moved.
    tallies: Keyed<LineTally>,
}

/// 78.151 — ONE SIGNER'S LINE AS A PUBLISHER READS IT: the nodes filed new
/// on it and the nodes that left it, each counted since the carrier was
/// built, and the MARK of its last change — drawn from one process-wide
/// count, so no two changes of any line, on any chain, carry the same mark.
/// Between two readings, a line none of whose nodes left and whose new
/// nodes all stand past the first reading's end grew only at its end.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LineTally {
    pub filed: u64,
    pub left: u64,
    pub mark: u64,
}

/// The process-wide count a line's mark is drawn from.
static MARKS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// A fresh mark: one past every mark drawn before it in this process.
pub fn next_mark() -> u64 {
    MARKS.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1
}

mod line;
mod node;
pub use line::{replace_durably, LineWrite};

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

/// WHAT ONE PAGE OF AN ERA PROOF DID (`ReceiptChain::walk_era`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EraWalk {
    /// The walk reached the root: the citation is proved.
    Proved,
    /// The page was a whole chain and ended short of the root: the walk
    /// stands at its last receipt, the next page opens there.
    Continues,
    /// The page moved nothing: the walk keeps its place.
    Stalled,
    /// The page is not the chain the walk stands on, or the citation is no
    /// citation into a closed era.
    Broken,
}

/// An era proof's key: the cited receipt's place key, then its hash.
fn era_walk_key(p: &Parent) -> Vec<u8> {
    let mut k = p.at.key();
    k.extend_from_slice(&p.hash);
    k
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

    /// The place the hot line opens at: genesis, or the first of the orbit
    /// after the root's — a fold cuts only at an orbit's close.
    fn first_place(&self) -> Place {
        self.root.map_or(Place::GENESIS, |r| r.at.beyond())
    }

    /// This chain's own head as it is held: its stamp and its node.
    fn head_node(&self) -> Option<(Stamp, &node::Node)> {
        let at = self.head?;
        Some((at, self.node_at(&at)?))
    }

    /// The place the next act of this chain's own line takes: after its
    /// head as the head's kind says (a close opens the next orbit).
    pub fn next_place(&self) -> Place {
        match self.head_node() {
            Some((at, n)) => at.place().after(n.kind),
            None => self.first_place(),
        }
    }

    /// THE ACTS OF ONE ORBIT of a signer's line, in line order: the line
    /// relation read by the prefix `vk‖K`, never by counting places.
    fn orbit_acts(&self, vk: &[u8; 32], k: u64) -> Vec<(Stamp, &node::Node)> {
        let mut prefix = vk.to_vec();
        prefix.extend_from_slice(&k.to_be_bytes());
        let Some(orbit) = self.lines.fiber_at(&prefix) else { return Vec::new() };
        orbit
            .filter_map(|(key, &h)| {
                let mut p = 32usize;
                let at = Stamp::new(h, *vk, Place::take_canonical(&key, &mut p)?);
                Some((at, self.node_at(&at)?))
            })
            .collect()
    }

    /// THE FRAME OF AN ORBIT (78.23 as amended): the per-lane sum of the
    /// frames its acts measured before `before` — lanes never mixed (law
    /// 5), a close's own frame never counted in. `None` where a lane passes
    /// its u64: an unmeasured frame, never a saturated one (rule 11).
    pub fn orbit_frame(&self, vk: &[u8; 32], k: u64, before: Place) -> Option<[u64; 4]> {
        let mut frame = [0u64; 4];
        for (at, n) in self.orbit_acts(vk, k) {
            if at.place() >= before || n.kind == ActKind::Orbit {
                continue;
            }
            for (f, m) in frame.iter_mut().zip(n.mu) {
                *f = f.checked_add(m)?;
            }
        }
        Some(frame)
    }

    /// THIS CHAIN'S OPEN ORBIT: the place its close would take and the
    /// frame it would carry, or `None` where no act stands since the last
    /// close — a close with no act seals nothing (OK1).
    pub fn open_orbit(&self) -> Option<(Place, [u64; 4])> {
        let (_, head) = self.head_node()?;
        if head.kind == ActKind::Orbit {
            return None;
        }
        let at = self.next_place();
        Some((at, self.orbit_frame(&self.vk, at.k, at)?))
    }

    /// EVERY ORBIT CLOSED AT ONE TICK, read off the close nodes in
    /// `fiber(h)`: the signer, its `K` and the orbit's frame as the close
    /// carries it. An orbit whose close this ledger does not hold is not
    /// here: unmeasured, never compared (rule 11).
    pub fn orbits_closed_at(&self, h: u64) -> Vec<([u8; 32], u64, [u64; 4])> {
        let Some(tick) = self.nodes.fiber_at(&h.to_be_bytes()) else { return Vec::new() };
        tick.filter(|(_, n)| n.kind == ActKind::Orbit)
            .filter_map(|(key, n)| Some((Stamp::from_key(&key)?, n.mu)))
            .map(|(at, mu)| (at.vk, at.k, mu))
            .collect()
    }

    /// THIS CHAIN'S LAST CLOSE: the head where the head is a close, or the
    /// node the open orbit's first act names as its predecessor. `None`
    /// before the first close.
    pub fn last_close(&self) -> Option<KStepReceipt> {
        let (at, head) = self.head_node()?;
        if head.kind == ActKind::Orbit {
            return self.unpack(at, head);
        }
        let orbit = self.orbit_acts(&self.vk, at.k);
        let (first, n) = orbit.into_iter().next()?;
        let prev = self.prev_of(&first, n)??;
        self.node(&prev.at).filter(|r| r.kind == ActKind::Orbit)
    }

    /// WOULD A CLOSE AT `tick` CARRYING `frame` COLLIDE (78.23 as amended):
    /// another signer's orbit closed at that tick with a frame equal on
    /// every lane, compared exactly — no constant, no threshold.
    pub fn collides(&self, tick: u64, frame: [u64; 4]) -> bool {
        self.orbits_closed_at(tick).into_iter().any(|(vk, _, f)| vk != self.vk && f == frame)
    }

    /// THE SEPARATION AT THIS CHAIN'S LAST CLOSE: per peer orbit closed at
    /// the same tick, the per-lane signed distance from our frame to its —
    /// sign first, exact, never a root and never summed (law 5).
    pub fn separation(&self) -> Vec<([u8; 32], [i128; 4])> {
        let Some(ours) = self.last_close() else { return Vec::new() };
        self.orbits_closed_at(ours.stamp.h)
            .into_iter()
            .filter(|(vk, _, _)| *vk != self.vk)
            .map(|(vk, _, f)| (vk, std::array::from_fn(|l| i128::from(ours.mu[l]) - i128::from(f[l]))))
            .collect()
    }

    /// DOES A CLOSE RE-DERIVE (OK1): a node of kind `Orbit` stands after at
    /// least one act of its orbit and carries the per-lane sum of their
    /// frames, re-derived here from the acts this ledger holds, exactly as
    /// a height is. Any other kind is not a close and passes.
    pub fn close_rederives(&self, r: &KStepReceipt) -> bool {
        r.kind != ActKind::Orbit
            || (r.stamp.i > 0 && self.orbit_frame(&r.stamp.vk, r.stamp.k, r.stamp.place()) == Some(r.mu))
    }

    /// The orbit the next act stands in — the current `K`.
    pub fn next_step_index(&self) -> u64 {
        self.next_place().k
    }

    /// The node at one stamp, as its receipt.
    pub fn node(&self, at: &Stamp) -> Option<KStepReceipt> {
        self.unpack(*at, self.node_at(at)?)
    }

    /// Whether a node stands at this stamp with this hash.
    pub fn holds(&self, p: &Parent) -> bool {
        self.node_at(&p.at).is_some_and(|n| n.hash == p.hash)
    }

    /// The node a signer holds at one place, through the line relation.
    pub fn at(&self, vk: &[u8; 32], at: Place) -> Option<KStepReceipt> {
        self.node(&self.stamp_at(vk, at)?)
    }

    /// This chain's own receipt at one place.
    pub fn at_place(&self, at: Place) -> Option<KStepReceipt> {
        self.at(&self.vk, at)
    }

    /// The stamps of a signer's line between two places: strictly after
    /// `after` (from the line's start where `None`) up to and including
    /// `upto`, in line order, at most `n`.
    fn line_stamps(&self, vk: &[u8; 32], after: Option<Place>, upto: Place, n: usize) -> Vec<Stamp> {
        let Some(line) = self.lines.fiber_at(vk) else { return Vec::new() };
        line.filter_map(|(key, &h)| {
            let mut p = 32usize;
            let at = Place::take_canonical(&key, &mut p)?;
            (after.is_none_or(|a| a < at) && at <= upto).then_some(Stamp::new(h, *vk, at))
        })
        .take(n)
        .collect()
    }

    /// A SIGNER'S LINE BETWEEN TWO PLACES: the nodes strictly after
    /// `after` (from the line's start where `None`) up to and including
    /// `upto`, in line order, at most `n` — read off the line relation by
    /// the signer's prefix, never by counting places.
    pub fn line_between(&self, vk: &[u8; 32], after: Option<Place>, upto: Place, n: usize) -> Vec<KStepReceipt> {
        self.line_stamps(vk, after, upto, n).iter().filter_map(|at| self.node(at)).collect()
    }

    /// 78.153b — THE FIRST NODE `pred` HOLDS OF on a signer's line strictly
    /// after `after` up to and including `upto`, in line order: the nodes
    /// unpacked one at a time until it is met, never the span read whole
    /// first ([`ReceiptChain::line_between`]'s nodes, the first that holds).
    pub fn first_between(&self, vk: &[u8; 32], after: Option<Place>, upto: Place, pred: impl Fn(&KStepReceipt) -> bool) -> Option<KStepReceipt> {
        let line = self.lines.fiber_at(vk)?;
        line.filter_map(|(key, &h)| {
            let mut p = 32usize;
            let at = Place::take_canonical(&key, &mut p)?;
            (after.is_none_or(|a| a < at) && at <= upto).then_some(Stamp::new(h, *vk, at))
        })
        .filter_map(|at| self.node(&at))
        .find(|r| pred(r))
    }

    /// A SIGNER'S HEAD as this ledger holds it: the node at the greatest
    /// place on its line.
    pub fn line_head(&self, vk: &[u8; 32]) -> Option<KStepReceipt> {
        self.at(vk, *self.tops.get(vk)?)
    }

    /// Every signer this ledger holds a line of, this chain's own
    /// included.
    pub fn signers(&self) -> Vec<[u8; 32]> {
        self.tops.scoped().filter_map(|(k, _)| <[u8; 32]>::try_from(k.as_slice()).ok()).collect()
    }

    /// File one node's stamp on its signer's line, raising the line's top.
    fn file_on_line(&mut self, at: &Stamp) {
        let new = self.lines.insert(Stamp::line_key(&at.vk, at.place()), at.h).is_none();
        let top = raise(self.tops.get(&at.vk).copied(), at.place());
        if let Some(top) = top {
            self.tops.insert(at.vk, top);
        }
        self.tally(&at.vk, new, false);
    }

    /// 78.151 — A LINE MOVED: a node filed new on it or a node gone from
    /// it, counted, and the line marked. A count past 2⁶⁴ is not counted
    /// further; the mark still moves.
    fn tally(&mut self, vk: &[u8; 32], filed: bool, left: bool) {
        let mut t = match self.tallies.get(vk) {
            Some(t) => *t,
            None => LineTally::default(),
        };
        if filed {
            if let Some(n) = t.filed.checked_add(1) {
                t.filed = n;
            }
        }
        if left {
            if let Some(n) = t.left.checked_add(1) {
                t.left = n;
            }
        }
        t.mark = next_mark();
        self.tallies.insert(vk, t);
    }

    /// 78.151 — a signer's line as a publisher reads it; the default (no
    /// node, no mark) where the line was never filed.
    pub fn line_tally(&self, vk: &[u8; 32]) -> LineTally {
        match self.tallies.get(vk) {
            Some(t) => *t,
            None => LineTally::default(),
        }
    }

    /// 78.151 — every signer whose fold root this chain holds, read by a
    /// publisher beside `signers`: a root stands where no node of its line
    /// may be held.
    pub fn rooted_signers(&self) -> Vec<[u8; 32]> {
        self.peer_roots.scoped().filter_map(|(k, _)| <[u8; 32]>::try_from(k.as_slice()).ok()).collect()
    }

    /// THE HEAD RECEIPT: this chain's own last.
    pub fn head(&self) -> Option<KStepReceipt> {
        self.node(&self.head?)
    }

    /// THE RECEIPT THIS HOT LINE OPENS AT — genesis, or the first after a
    /// fold.
    pub fn opening(&self) -> Option<KStepReceipt> {
        self.at_place(self.first_place())
    }

    /// The head's chain hash (the root's before the first hot receipt; the
    /// empty hash before genesis — the reading of absence).
    pub fn head_hash(&self) -> [u8; 32] {
        self.head_node()
            .map(|(_, n)| n.hash)
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

    /// THE ROOT A PEER'S OWN FOLD ARCHIVED ITS LINE TO, as this ledger holds
    /// its `Fold` node — the place at or below which `shed_folded` cut that
    /// signer's line. `None` where no fold of the signer is held.
    pub fn peer_root(&self, vk: &[u8; 32]) -> Option<Place> {
        self.peer_roots.get(vk).map(|r| r.at.place())
    }

    /// A CITATION INTO A PEER'S CLOSED ERA, PROVED BY ITS CHAIN (78.65,
    /// 78.87 R2): `page` is the signer's line from `p` up to the root its
    /// fold archived the line to. Walked from `p`: every receipt re-derives
    /// its own hash and names the one before it as its predecessor, and the
    /// walk ends exactly at the root's stamp and hash. Only then is `p` held
    /// as proved, and its height reads. Nothing is trusted of who sent the
    /// page: the chain is the proof. `false` where `p` stands past the root,
    /// no root is held, or the page does not prove it.
    pub fn prove_from_era(&mut self, p: &Parent, page: &[KStepReceipt]) -> bool {
        matches!(self.walk_era(p, page), EraWalk::Proved)
    }

    /// WHERE THE NEXT PAGE OF `p`'S ERA PROOF OPENS: the last receipt a
    /// walk from `p` re-derived, or `p` itself where none has begun.
    pub fn era_walk_from(&self, p: &Parent) -> Parent {
        match self.walking.get(&era_walk_key(p)) {
            Some(at) => *at,
            None => *p,
        }
    }

    /// 78.65, 78.87 R2 — ONE PAGE OF AN ERA PROOF, walked from where the
    /// proof stands (`era_walk_from`): the page opens at that receipt, and
    /// every receipt after it re-derives its own hash and names the one
    /// before it, up to the root the signer's fold archived the line to.
    /// PROVED where the walk reaches the root: `p` is held as proved and the
    /// walk leaves. CONTINUES where the page is a whole chain from where it
    /// opened and ends short of the root, having moved: the walk stands at
    /// its last receipt and the next page opens there — a proof longer than
    /// one frame closes page by page, and a root a later fold moved further
    /// is walked to as well. STALLED where the page moved nothing (the walk
    /// keeps its place). BROKEN where the page is not the chain the walk
    /// stands on, or `p` is no citation into a closed era: the walk leaves.
    pub fn walk_era(&mut self, p: &Parent, page: &[KStepReceipt]) -> EraWalk {
        let key = era_walk_key(p);
        let broken = |c: &mut Self| {
            c.walking.remove(&key);
            EraWalk::Broken
        };
        let Some(root) = self.peer_roots.get(&p.at.vk).copied() else { return broken(self) };
        if p.at.place() > root.at.place() {
            return broken(self);
        }
        let from = self.era_walk_from(p);
        let Some(start) = page.iter().position(|r| r.stamp == from.at && r.chain_hash == from.hash && r.rederives()) else {
            return broken(self);
        };
        let mut last = from;
        let mut reached = last == root;
        for r in page.iter().skip(start + 1) {
            if reached {
                break;
            }
            if !r.rederives() || r.prev != Some(last) {
                return broken(self);
            }
            last = Parent { at: r.stamp, hash: r.chain_hash };
            reached = last == root;
        }
        if reached {
            self.walking.remove(&key);
            self.proved.insert(p.at.key(), p.hash);
            return EraWalk::Proved;
        }
        if last == from {
            return EraWalk::Stalled;
        }
        self.walking.insert(key, last);
        EraWalk::Continues
    }

    /// 78.147 — WHERE A SIGNER'S LINE STANDS IN THIS LEDGER, as the next
    /// receipt of it must name its predecessor: the node at the line's top,
    /// or the root its signer's fold archived the line to where that root
    /// stands past every node held and its height reads (an adopted root,
    /// `adopt_root`). `None` where neither is held.
    pub fn line_tip(&self, vk: &[u8; 32]) -> Option<Parent> {
        let head = self.line_head(vk).map(|r| Parent { at: r.stamp, hash: r.chain_hash });
        let root = self.peer_roots.get(vk).copied().filter(|r| self.height_of(r).is_some());
        match (head, root) {
            (Some(h), Some(r)) if r.at.place() > h.at.place() => Some(r),
            (Some(h), _) => Some(h),
            (None, r) => r,
        }
    }

    /// 78.147 — A PEER'S FOLD ROOT ADOPTED, its two signatures verified by
    /// the caller (`Kernel::adopt_root`): the signer's line stands on it here
    /// as the signer's own line stands on it — the next receipt names it as
    /// its predecessor and its height reads off its stamp — and nothing below
    /// it is asked again. `false` for this chain's own key, or where the line
    /// already stands at or past the root.
    pub fn adopt_root(&mut self, root: Parent) -> bool {
        let vk = root.at.vk;
        if vk == self.vk || self.line_tip(&vk).is_some_and(|t| t.at.place() >= root.at.place()) {
            return false;
        }
        self.peer_roots.insert(vk, root);
        self.proved.insert(root.at.key(), root.hash);
        true
    }

    /// The stamps of this chain's own line, in line order.
    fn own_stamps(&self) -> Vec<Stamp> {
        let vk = self.vk;
        self.lines
            .fiber_at(&vk)
            .into_iter()
            .flatten()
            .filter_map(|(key, &h)| {
                let mut p = 32usize;
                Some(Stamp::new(h, vk, Place::take_canonical(&key, &mut p)?))
            })
            .collect()
    }

    /// THIS CHAIN'S OWN LINE, in line order: the line relation read by the
    /// signer's prefix.
    pub fn own(&self) -> impl Iterator<Item = KStepReceipt> + '_ {
        self.own_stamps().into_iter().filter_map(move |at| self.node(&at))
    }

    /// How many receipts the hot line holds: its nodes counted, never a
    /// difference of places.
    pub fn own_len(&self) -> u64 {
        self.own_stamps().len() as u64
    }

    /// THE LEDGER'S CARRIERS counted, each by its own members.
    pub fn census(&self, c: &mut crate::observe::Carriers) {
        let ReceiptChain {
            nodes,
            lines,
            tops,
            root: _,
            vk: _,
            head: _,
            placed,
            folded,
            peer_roots,
            proved,
            walking,
            operators,
            operator_at: _,
            journal: _,
            tallies,
        } = self;
        c.count("chain.tallies", tallies.len());
        c.count("chain.nodes", nodes.len());
        c.count("chain.lines", lines.len());
        c.count("chain.tops", tops.len());
        c.count("chain.placed", placed.len());
        c.count("chain.folded", folded.len());
        c.count("chain.peer_roots", peer_roots.len());
        c.count("chain.proved", proved.len());
        c.count("chain.walking", walking.len());
        c.count("chain.operators", operators.len());
    }

    /// How many nodes the ledger holds, its own and its peers'.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Whether the ledger holds no node.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// EVERY NODE AT ONE TICK — the mesh's width at `h`.
    pub fn at_tick(&self, h: u64) -> Vec<KStepReceipt> {
        let Some(tick) = self.nodes.fiber_at(&h.to_be_bytes()) else { return Vec::new() };
        tick.filter_map(|(key, n)| self.unpack(Stamp::from_key(&key)?, n)).collect()
    }

    /// Every node held, in stamp order: the ledger as it is persisted.
    pub fn nodes(&self) -> impl Iterator<Item = KStepReceipt> + '_ {
        self.nodes.scoped().filter_map(|(key, n)| self.unpack(Stamp::from_key(&key)?, n))
    }

    /// FOLD (45.8): this chain's own receipts at or below `upto` leave the
    /// ledger and are returned, cold, in line order, with the root they
    /// leave the line standing on. A fold cuts only at an orbit's close, so
    /// every orbit is held whole on one side of it and a close re-derives
    /// its frame from what the hot line holds: `None` where the node at
    /// `upto` is not a close, or no hot receipt sits at or below it.
    pub fn fold_before(&mut self, upto: Place) -> Option<(Vec<KStepReceipt>, Root)> {
        self.head?;
        let at = self.stamp_at(&self.vk, upto)?;
        if self.node_at(&at)?.kind != ActKind::Orbit {
            return None;
        }
        let (cold, top) = self.remove_own_upto(upto);
        let root = top?;
        self.stand_on(root);
        // The line on disk cuts its own nodes at the root: none is named.
        for r in &cold {
            self.journal.cut(&r.stamp.key());
        }
        Some((cold, root))
    }

    /// THIS CHAIN'S OWN NODES AT OR BELOW `upto` leave, returned in line
    /// order with the root the last of them makes.
    fn remove_own_upto(&mut self, upto: Place) -> (Vec<KStepReceipt>, Option<Root>) {
        let vk = self.vk;
        let mut cold = Vec::new();
        let mut top = None;
        for at in self.line_stamps(&vk, None, upto, usize::MAX) {
            if let Some(r) = self.take_node(&at) {
                top = Some(Root { at: r.stamp.place(), hash: r.chain_hash, height: r.stamp.h });
                cold.push(r);
            }
        }
        (cold, top)
    }

    /// THIS CHAIN'S OWN LINE STANDS ON `root`: the head leaves where the
    /// root reaches it.
    fn stand_on(&mut self, root: Root) {
        if self.head.is_some_and(|h| h.place() <= root.at) {
            self.head = None;
        }
        self.root = Some(root);
        self.journal.rooted();
    }

    /// THE HEIGHT A PARENT NAMES, where this chain holds the node at its
    /// stamp with its hash. `None` otherwise: UNMEASURED height, which a
    /// caller re-deriving a claim refuses rather than reads as zero.
    pub fn height_of(&self, p: &Parent) -> Option<u64> {
        let hash = match self.node_at(&p.at) {
            Some(held) => held.hash,
            None => match self.folded.get(&p.at.key()) {
                Some(h) => *h,
                None => *self.proved.get(&p.at.key())?,
            },
        };
        (hash == p.hash).then_some(p.at.h)
    }

    /// A folded receipt, read back from its era and proved there, held by
    /// its stamp and hash so a citation of it re-derives its height.
    pub fn recall_folded(&mut self, p: Parent) {
        self.folded.insert(p.at.key(), p.hash);
    }

    /// The citation it was recalled for is read: it leaves, and is read
    /// back from its era again where another cites it.
    pub fn forget_folded(&mut self, p: &Parent) {
        self.folded.remove(&p.at.key());
    }

    /// THE PREDECESSOR A ROOT OR A HEAD IS, as the next receipt names it.
    fn line_prev(&self) -> Option<Parent> {
        match self.head_node() {
            Some((at, n)) => Some(Parent { at, hash: n.hash }),
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
        let key = r.stamp.key();
        // 78.94 — a peer's node of an action no court seals is no node.
        if r.stamp.vk == self.vk || self.nodes.contains(&key) || !r.kind.sealed() {
            return;
        }
        let ours_in = self.ours_in_of(&r);
        let reach = self.placed.get(&key).and_then(|p| p.reach);
        self.placed.insert(&key, Placed { ours_in, reach });
        self.file_on_line(&r.stamp);
        self.file_peer_root(&r);
        self.journal.filed(&key);
        let n = self.pack(r);
        self.nodes.insert(key, n);
    }

    /// A PEER'S FOLD ROOT, read off its `Fold` node: the root it cites on
    /// its own signer's line, raising that signer's entry. Any other node
    /// names no root.
    fn file_peer_root(&mut self, r: &KStepReceipt) {
        if r.kind != ActKind::Fold {
            return;
        }
        for c in &r.cites {
            let Cite::Root { at, hash } = c else { continue };
            if at.vk != r.stamp.vk {
                continue;
            }
            // The later root stands: a fold archives everything below it.
            let later = self.peer_roots.get(&at.vk).is_none_or(|held| held.at.place() < at.place());
            if later {
                self.peer_roots.insert(at.vk, Parent { at: *at, hash: *hash });
            }
        }
    }

    /// THE INVERSE OF `admit` (45.8): a peer's receipts at or below the
    /// root its own fold archived leave the hot ledger — its era holds
    /// them, on the signer and on the witness that co-signed it. Two kinds
    /// stay: what this chain's own hot line cites, through which its
    /// heights re-derive at restore, and what `keep` names. The signer's
    /// fold node stands above its root, so its line keeps the head
    /// admission reads. Answers how many nodes left.
    pub fn shed_folded(&mut self, keep: impl Fn(&KStepReceipt) -> bool) -> u64 {
        let mut cited: Keyed<()> = Keyed::new();
        for at in self.own_stamps() {
            for p in self.node_at(&at).into_iter().flat_map(|n| n.parents.iter()) {
                cited.insert(p.at.key(), ());
            }
        }
        let roots: Vec<([u8; 32], Place)> = self
            .peer_roots
            .scoped()
            .filter_map(|(vk, root)| Some((<[u8; 32]>::try_from(vk.as_slice()).ok()?, root.at.place())))
            .collect();
        let mut shed = 0u64;
        for (vk, upto) in roots {
            let leaving: Vec<Stamp> = self
                .line_between(&vk, None, upto, usize::MAX)
                .into_iter()
                .filter(|r| !cited.contains(&r.stamp.key()) && !keep(r))
                .map(|r| r.stamp)
                .collect();
            for at in leaving {
                if self.take_node(&at).is_some() {
                    self.journal.left(&at.key());
                    shed += 1;
                }
            }
        }
        // A PLACEMENT OF A NODE NOT HELD waits for that node to arrive; at or
        // below its signer's root it never will (admission reads it stale),
        // and the placement leaves with the line.
        let unheld: Vec<Vec<u8>> = self
            .placed
            .scoped()
            .map(|(key, _)| key)
            .filter(|key| !self.nodes.contains(key))
            .filter(|key| {
                Stamp::from_key(key).is_some_and(|at| self.peer_roots.get(&at.vk).is_some_and(|root| at.place() <= root.at.place()))
            })
            .collect();
        for key in unheld {
            self.placed.remove(&key);
        }
        shed
    }

    /// The edges out of a receipt: its predecessor and its parents.
    fn edges(r: &KStepReceipt) -> impl Iterator<Item = Stamp> + '_ {
        r.prev.iter().chain(r.parents.iter()).map(|p| p.at)
    }

    /// The edges out of the node held at a stamp; none where none is held.
    fn held_edges(&self, at: &Stamp) -> Vec<Stamp> {
        match self.node_at(at) {
            Some(n) => self.edges_of(at, n),
            None => Vec::new(),
        }
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
            stack.extend(self.held_edges(&at));
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
            let up = self.held_edges(&at);
            stack.extend(up);
        }
    }

    fn stands(&self, at: &Stamp) -> Option<Stands> {
        if at.vk == self.vk {
            return self.node_at(at).map(|_| Stands::Own(at.place()));
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
            for up in self.held_edges(&at) {
                if up == *to {
                    return true;
                }
                stack.push(up);
            }
        }
        false
    }

    /// EVERY PEER NODE THIS CHAIN HOLDS.
    pub fn admitted(&self) -> Vec<KStepReceipt> {
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
    /// a parent is not held or a citation cannot be written (LD0), and
    /// where a close is asked with no act since the last close or with a
    /// frame other than its orbit's (OK1).
    pub fn append(
        &mut self,
        kind: ActKind,
        mu: [u64; 4],
        cites: Vec<Cite>,
        operator_hash: [u8; 32],
        parents: Vec<Parent>,
    ) -> Option<KStepReceipt> {
        // 78.94 — THE ONE SEALING DOOR READS THE PHYSICS: an action no court
        // seals — an observation, a request, a retired kind — writes no node.
        if !kind.sealed() {
            return None;
        }
        if kind == ActKind::Orbit && self.open_orbit().map(|(_, frame)| frame) != Some(mu) {
            return None;
        }
        let stamp = self.next_stamp(&parents)?;
        let k = stamp.place();
        let prev = self.line_prev();
        let r = KStepReceipt::seal(stamp, kind, mu, prev, parents, cites, operator_hash)?;
        self.mark_reached(k, &r.parents);
        self.file_on_line(&stamp);
        self.journal.filed(&stamp.key());
        let n = self.pack(r.clone());
        self.nodes.insert(stamp.key(), n);
        self.head = Some(stamp);
        Some(r)
    }

    /// Verify the hot line: contiguous from its opening — every node's
    /// predecessor the one before it (or the root), and its place the one
    /// the predecessor's kind says follows it — every height re-derived
    /// from its predecessor and parents, every close's frame re-derived
    /// from its orbit's acts, every hash re-derived from its coordinates.
    pub fn verify_chain_integrity(&self) -> bool {
        let mut prev = self.root.map(|r| Parent { at: Stamp::new(r.height, self.vk, r.at), hash: r.hash });
        // The root is a close: a fold cuts nowhere else.
        let mut prev_kind = ActKind::Orbit;
        let mut last = None;
        for at in self.own_stamps() {
            let Some(r) = self.node(&at) else { return false };
            let follows = match prev {
                Some(p) => p.at.place().after(prev_kind) == r.stamp.place(),
                None => r.stamp.place() == Place::GENESIS,
            };
            if !follows || r.prev != prev || !self.close_rederives(&r) {
                return false;
            }
            prev_kind = r.kind;
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
    /// persisted. `None` refuses the whole ledger, and a stamp met twice
    /// refuses it.
    pub fn from_nodes(root: Option<Root>, vk: [u8; 32], nodes: Vec<KStepReceipt>) -> Option<ReceiptChain> {
        let mut c = ReceiptChain::with_vk(vk);
        c.root = root;
        for r in nodes {
            c.hold(r)?;
        }
        c.settle()
    }

    /// ONE PERSISTED NODE HELD at its stamp, on its signer's line: it
    /// re-derives its own hash, and no other node stands at its stamp.
    /// Nothing is placed here; `settle` places the whole.
    fn hold(&mut self, r: KStepReceipt) -> Option<()> {
        let key = r.stamp.key();
        if !r.rederives() || self.nodes.contains(&key) {
            return None;
        }
        self.file_on_line(&r.stamp);
        if r.stamp.vk == self.vk {
            self.head = Some(match self.head {
                Some(h) if h.place() >= r.stamp.place() => h,
                _ => r.stamp,
            });
        } else {
            self.file_peer_root(&r);
        }
        let n = self.pack(r);
        self.nodes.insert(key, n);
        Some(())
    }

    /// THE HELD NODES SETTLED: this chain's own line verifies whole on its
    /// root, and every peer node is placed against it. The nodes are read
    /// in key order, which is height first, and a parent's height is below
    /// its child's, so every ancestor is placed before its descendant.
    fn settle(mut self) -> Option<ReceiptChain> {
        if !self.verify_chain_integrity() {
            return None;
        }
        let vk = self.vk;
        let peers: Vec<Stamp> =
            self.nodes.scoped().filter_map(|(key, _)| Stamp::from_key(&key)).filter(|at| at.vk != vk).collect();
        for at in peers {
            let Some(r) = self.node(&at) else { continue };
            let ours_in = self.ours_in_of(&r);
            self.placed.insert(at.key(), Placed { ours_in, reach: None });
        }
        let own: Vec<(Place, Vec<Parent>)> =
            self.own_stamps().into_iter().filter_map(|at| Some((at.place(), self.node_at(&at)?.parents.to_vec()))).collect();
        for (k, parents) in own {
            self.mark_reached(k, &parents);
        }
        Some(self)
    }

    /// A chain rehydrated from its own line alone, in `K` order, on no
    /// root — a line whose receipts cite no peer.
    pub fn from_receipts(receipts: Vec<KStepReceipt>) -> Option<ReceiptChain> {
        let vk = receipts.iter().map(|r| r.stamp.vk).next().map_or([0u8; 32], |v| v);
        Self::from_nodes(None, vk, receipts)
    }
}

/// WHAT A CYCLE'S CLOSE BY NAME DID (OK1): the orbit closed at its stamp;
/// it was WITHHELD, its frame coinciding with a peer's at its tick, and
/// stays open for the separating transport; or nothing stood to close.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Close {
    Sealed(Stamp),
    Withheld,
    Nothing,
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
