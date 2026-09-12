// A13 — THE KEYED RELATION: what a mind reads by key, and the reads it
// exposes.

use super::*;

/// XOR ROUTING TRIE — the topological structure the Kademlia metric induces.
/// `closest(key)` descends the key's bits greedily (matching child when present,
/// sibling otherwise): because XOR is dominated by the highest differing bit,
/// agreeing with the key at the earliest bit minimizes XOR — so the greedy leaf
/// IS the XOR-closest, in O(bits), INDEPENDENT of N and never a scan of the peer
/// set. (Correct + simple; path-compression to O(N) nodes is tracked as R5's
/// follow-up — correctness first.)
/// A KEYED RELATION over A21 coordinate bytes: a value reached by its
/// key's bits, O(bits) per read and write, never a scan. A prefix of a
/// key is a coarser stratum (`fiber`). Digests are not keys.
#[derive(Clone, Debug)]
pub struct Keyed<V> {
    root: Option<Box<Node<V>>>,
    n: usize,
}

impl<V> Default for Keyed<V> {
    fn default() -> Self {
        Keyed { root: None, n: 0 }
    }
}

impl<V> Keyed<V> {
    pub fn new() -> Self {
        Keyed::default()
    }

    /// Bind `key` to `value`; a held key is rebound. Returns the value
    /// displaced, if any.
    pub fn insert(&mut self, key: impl AsRef<[u8]>, value: V) -> Option<V> {
        #[cfg(test)]
        work::count_insert();
        let key = key.as_ref().to_vec();
        let Some(root) = self.root.as_deref() else {
            self.root = Some(Box::new(Node::Leaf(key, value)));
            self.n = 1;
            return None;
        };
        let held = root.held_key(&key).to_vec();
        match first_difference(&key, &held) {
            None => self.root.as_deref_mut()?.get_mut(&key).map(|v| std::mem::replace(v, value)),
            Some(d) => {
                insert_at(&mut self.root, key, value, d);
                self.n += 1;
                None
            }
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<&V> {
        #[cfg(test)]
        work::count_get();
        self.root.as_deref()?.get(key)
    }

    pub fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        self.root.as_deref_mut()?.get_mut(key)
    }

    pub fn contains(&self, key: &[u8]) -> bool {
        self.get(key).is_some()
    }

    /// Unbind `key`; the value it held, if any.
    pub fn remove(&mut self, key: &[u8]) -> Option<V> {
        let v = remove_at(&mut self.root, key);
        if v.is_some() {
            // A key was bound, so the count carries it; a carrier at zero
            // holding a key is a broken count and stays at zero.
            if let Some(next) = self.n.checked_sub(1) {
                self.n = next;
            }
        }
        v
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Keys present in both carriers. Simultaneous PATRICIA descent (A13):
    /// subtrees that share no prefix are pruned; cost is the shared
    /// structure, never a product of the two fibers.
    pub fn meet<W>(&self, other: &Keyed<W>) -> Keyed<()> {
        let mut out = Keyed::new();
        meet_nodes(self.root.as_deref(), other.root.as_deref(), &mut out);
        out
    }

    /// Subtree under `prefix`. The empty prefix [`ROOT`] is this carrier's
    /// own fiber — valid when the Keyed is already scoped (one bubble's
    /// charts, one vertex's star, one chart's incidence).
    ///
    /// THE WALK IS THE SUBTREE, NOT THE CARRIER. This pushed the ROOT for
    /// every prefix and filtered the leaves as they came out — `stack` held
    /// `self.root` whatever the prefix was, and `Fiber` carried the prefix
    /// only to test each leaf against it —
    /// so a fiber read of one stratum cost the whole trie — every node of
    /// it, to yield the handful of keys under the prefix. That is rule 1's
    /// whole-carrier walk arriving through the very reader the doctrine
    /// says replaces it, and it is why the rest of X3 exists: `fiber(ROOT)`
    /// was the only honest spelling, because a prefix bought nothing.
    ///
    /// A key is its own path (A13), so the keys carrying a prefix hang
    /// under one node, and `subtree` descends to it in O(bits). The leaf
    /// filter stays: path compression leaves bits between branch points
    /// untested, so the subtree can hold keys that diverge from the prefix
    /// at one of them, and `key_in_fiber` is what decides. What changes is
    /// the extent — the walk visits that subtree and nothing above it.
    pub fn fiber(&self, prefix: &[u8]) -> Fiber<'_, V> {
        let mut stack = Vec::new();
        if let Some(node) = self.subtree(prefix) {
            stack.push(node);
        }
        Fiber { stack, prefix: prefix.to_vec() }
    }

    /// THE WHOLE OF A CARRIER THAT IS ITSELF A FIBER — one bubble's charts,
    /// one vertex's star, one chart's incidence, one form's cited
    /// addresses. Calling this ASSERTS that scoping: the carrier was
    /// reached by a key already, so its whole content is one cell's
    /// reading and walking it is not rule 1's whole-carrier walk.
    ///
    /// It is the other half of [`Prefix`]: a site either has a coordinate
    /// to descend on or is already at one, and which of the two it is
    /// should be legible without reading the carrier's provenance. The
    /// empty prefix said neither.
    pub fn scoped(&self) -> Fiber<'_, V> {
        let mut stack = Vec::new();
        if let Some(root) = self.root.as_deref() {
            stack.push(root);
        }
        Fiber { stack, prefix: Vec::new() }
    }

    /// The node every key carrying `prefix` hangs under, by descent on the
    /// prefix's own bits. `None` where no key carries it — an empty fiber,
    /// which is the reading of absence and not an empty walk of the trie.
    ///
    /// The empty prefix is the root node: a carrier that is itself a fiber
    /// has its whole content under one node, and that is the one case where
    /// walking everything is the reading (the rest of X3 names those sites).
    fn subtree(&self, prefix: &[u8]) -> Option<&Node<V>> {
        let mut node = self.root.as_deref()?;
        let want = Node::<V>::bits(prefix)?;
        loop {
            match node {
                Node::Leaf(..) => return Some(node),
                Node::Branch { bit, children, .. } => {
                    // At or past the prefix's last bit, this node's whole
                    // subtree is the candidate set; above it, the prefix's
                    // own bit says which child can hold it.
                    if *bit >= want {
                        return self.carries(node, prefix).then_some(node);
                    }
                    node = children[key_bit(prefix, *bit)].as_deref()?;
                }
            }
        }
    }

    /// CAN ANY KEY UNDER THIS NODE CARRY `prefix`? Below a branch at bit
    /// `b` every key agrees on bits `[0, b)` — a key that differed earlier
    /// would have split above it — so where `b` is at or past the prefix's
    /// last bit, one key of the subtree answers for all of them, and the
    /// representative is that key.
    ///
    /// A representative SHORTER than the prefix answers for nothing: it
    /// cannot carry a longer prefix itself, while a longer key beside it
    /// still might, so the subtree stands and the leaf filter decides. The
    /// walk is wider in that case and never wrong.
    fn carries(&self, node: &Node<V>, prefix: &[u8]) -> bool {
        let rep = node.representative();
        rep.len() < prefix.len() || rep.starts_with(prefix)
    }

    /// Parent shell of `key`: the branch that split this leaf. None when
    /// the key is unbound or is the sole leaf.
    pub fn peel(&self, key: &[u8]) -> Option<Shell<'_, V>> {
        let mut node = self.root.as_deref()?;
        let mut parent: Option<&Node<V>> = None;
        let mut parent_bit = 0usize;
        loop {
            match node {
                Node::Leaf(k, _) => {
                    if k.as_slice() != key {
                        return None;
                    }
                    let p = parent?;
                    return Some(Shell {
                        fiber: Fiber { stack: vec![p], prefix: ROOT.to_vec() },
                        prefix: clear_after_bit(key, parent_bit),
                    });
                }
                Node::Branch { bit, children, end } => {
                    if Node::<V>::bits(key) == Some(*bit) {
                        let (k, _) = end.as_deref()?;
                        if k.as_slice() != key {
                            return None;
                        }
                        return Some(Shell {
                            fiber: Fiber { stack: vec![node], prefix: ROOT.to_vec() },
                            prefix: clear_after_bit(key, *bit),
                        });
                    }
                    parent = Some(node);
                    parent_bit = *bit;
                    node = children[key_bit(key, *bit)].as_deref()?;
                }
            }
        }
    }

    /// At most `budget` entries in key order after `from`, continuing past
    /// the last key from the first and ending where it began (45.7).
    /// [`ROOT`] starts at the first key. The last yielded key is the
    /// resume cursor; `Cursor::circuit` reports a completed circuit.
    pub fn cursor<'a>(&'a self, from: &[u8], budget: &'a mut crate::physics::Budget) -> Cursor<'a, V> {
        let from = (!from.is_empty()).then(|| from.to_vec());
        Cursor {
            keyed: self,
            // THE ORBIT IS THE WHOLE CARRIER BY DESIGN (45.7) and its bound
            // is the Budget in its signature, not a prefix: that is what
            // makes a carrier larger than one stroke legible across strokes
            // instead of walked at once (rule 10).
            fiber: self.scoped(),
            budget,
            last: from.clone(),
            from,
            wrapped: false,
            done: false,
        }
    }
}

impl<V> Keyed<Keyed<V>> {
    /// Insert at `(a, b)` without rebuilding the inner trie when `a` is held.
    pub fn nested_insert(&mut self, a: impl AsRef<[u8]>, b: impl AsRef<[u8]>, value: V) {
        let a = a.as_ref();
        if let Some(inner) = self.get_mut(a) {
            inner.insert(b, value);
            return;
        }
        let mut inner = Keyed::new();
        inner.insert(b, value);
        self.insert(a, inner);
    }
}

/// The empty prefix: the root fiber of a keyed that is already a fiber.
///
/// X3 RETIRES THIS. It is the spelling of "walk everything", and it is
/// written in 280 places because a prefix used to buy nothing (c483bab).
/// Each of those sites is one of two readings, and the reader says which:
/// [`Prefix`] where a coordinate is in hand, [`Keyed::scoped`] where the
/// carrier is ALREADY one cell's fiber. When no site names it, the
/// constant goes and `fiber` takes a `Prefix` that cannot be empty.
pub const ROOT: &[u8] = &[];

/// A NON-EMPTY KEY PREFIX — the coordinate a fiber read descends on.
///
/// The empty slice is not a coordinate: it names the whole carrier, and
/// the whole carrier is not a fiber unless the carrier is one already.
/// Keeping the two apart in the TYPE is what makes rule 1 readable at a
/// site: `fiber(Prefix)` descends to somewhere, `scoped()` says this
/// carrier is somewhere.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Prefix<'a>(&'a [u8]);

impl<'a> Prefix<'a> {
    /// `None` for the empty prefix. A caller holding a key it built from
    /// coordinates gets a `Prefix`; a caller whose "prefix" turned out
    /// empty has no coordinate and must say which reading it wanted, which
    /// is the refusal doing its work (rule 11).
    pub fn new(bytes: &'a [u8]) -> Option<Self> {
        (!bytes.is_empty()).then_some(Prefix(bytes))
    }

    pub fn as_bytes(&self) -> &'a [u8] {
        self.0
    }
}

/// Sealed persist walk: only [`crate::snapshot`] names this.
pub(crate) fn keyed_persist_walk<V>(k: &Keyed<V>, f: &mut impl FnMut(&[u8], &V)) {
    // A SNAPSHOT IS THE CARRIER. Persisting less than all of it is a
    // ledger that loses what it does not name, which is X8's row from the
    // other side; there is no prefix here and there should not be one.
    for (key, v) in k.scoped() {
        f(&key, v);
    }
}
