// A13 — THE KEYED RELATION: what a mind reads by key, and the reads it
// exposes.

use super::*;

/// XOR ROUTING TRIE — the topological structure the Kademlia metric induces.
/// `closest(key)` follows the key's bits greedily (matching child when present,
/// sibling otherwise): because XOR is dominated by the highest differing bit,
/// agreeing with the key at the earliest bit minimizes XOR — so the greedy leaf
/// IS the XOR-closest, in O(bits), INDEPENDENT of N and never a scan of the peer
/// set. (Correct + simple; path-compression to O(N) nodes is tracked as R5's
/// follow-up — correctness first.)
/// A KEYED RELATION over A21 coordinate bytes: a value reached by its
/// key's bits, O(bits) per read and write, never a scan. A prefix of a
/// A declared coordinate system naming the carrier's mathematical schema (Rule 6).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// THE SYSTEMS, one per A21 tag. This enumerated seven of nineteen, so a
/// carrier over the other twelve could not declare itself even in the
/// runtime form — `for_system` had nothing to name. Every tag `address`
/// writes is a system a carrier can be keyed by, and the two lists are the
/// same list or the declaration is partial by construction.
pub enum CoordinateSystem {
    #[default]
    Any,
    Rotor,
    Versor,
    Word,
    Form,
    Chart,
    Class,
    Cert,
    Listing,
    Sel,
    Hue,
    Assign,
    Gauge,
    Plane,
    Simplex,
    Cocycle,
    Task,
    Demand,
    DemandAt,
    Residual,
    /// §4.4 E8 — the three addresses of a cover (ruled 2026-09-22).
    Piece,
    Carriage,
    Meet,
}

impl CoordinateSystem {
    pub const fn tag(self) -> Option<u8> {
        use crate::transducer::address as a;
        match self {
            CoordinateSystem::Any => None,
            CoordinateSystem::Rotor => Some(a::TAG_ROTOR),
            CoordinateSystem::Versor => Some(a::TAG_VERSOR),
            CoordinateSystem::Word => Some(a::TAG_WORD),
            CoordinateSystem::Form => Some(a::TAG_FORM),
            CoordinateSystem::Chart => Some(a::TAG_CHART),
            CoordinateSystem::Class => Some(a::TAG_CLASS),
            CoordinateSystem::Cert => Some(a::TAG_CERT),
            CoordinateSystem::Listing => Some(a::TAG_LISTING),
            CoordinateSystem::Sel => Some(a::TAG_SEL),
            CoordinateSystem::Hue => Some(a::TAG_HUE),
            CoordinateSystem::Assign => Some(a::TAG_ASSIGN),
            CoordinateSystem::Gauge => Some(a::TAG_GAUGE),
            CoordinateSystem::Plane => Some(a::TAG_PLANE),
            CoordinateSystem::Simplex => Some(a::TAG_SIMPLEX),
            CoordinateSystem::Cocycle => Some(a::TAG_COCYCLE),
            CoordinateSystem::Task => Some(a::TAG_TASK),
            CoordinateSystem::Demand => Some(a::TAG_DEMAND),
            CoordinateSystem::DemandAt => Some(a::TAG_DEMAND_AT),
            CoordinateSystem::Residual => Some(a::TAG_RESIDUAL),
            CoordinateSystem::Piece => Some(a::TAG_PIECE),
            CoordinateSystem::Carriage => Some(a::TAG_CARRIAGE),
            CoordinateSystem::Meet => Some(a::TAG_MEET),
        }
    }
}

/// A refusal returned when an operation breaches the carrier's declared coordinate system (Rule 11).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SchemaRefusal {
    IncompatibleSystem {
        expected: CoordinateSystem,
        actual_tag: u8,
    },
    InvalidDepth {
        max_depth: usize,
        requested: usize,
    },
    /// The key opened with this system's tag and did not spell its
    /// coordinates — the refusal that separates a coarse key from a finer
    /// one written over it, which the tag alone cannot.
    Stratum(Refusal),
}

impl std::fmt::Display for SchemaRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemaRefusal::IncompatibleSystem { expected, actual_tag } => {
                write!(f, "incompatible coordinate system: expected {expected:?}, got tag {actual_tag}")
            }
            SchemaRefusal::InvalidDepth { max_depth, requested } => {
                write!(f, "invalid depth {requested} over max depth {max_depth}")
            }
            SchemaRefusal::Stratum(r) => {
                write!(f, "{} coordinate `{}`: {}", r.system, r.segment, r.why)
            }
        }
    }
}

impl std::error::Error for SchemaRefusal {}

/// A KEYED RELATION over A21 coordinate bytes: a value reached by its
/// key's bits, O(bits) per read and write, never a scan. A prefix of a
/// key is a coarser stratum (`fiber`). Digests are not keys.
///
/// `C` IS THE COORDINATE THIS INDEX ANSWERS (§4.4). It defaults to
/// [`coord::Any`], which is what every carrier in the tree was: undeclared,
/// taking loose bytes, unable to tell a chart key from a member key written
/// over it. A carrier that names its system instead takes only a [`Key`] of
/// that system, and the eight sites of R1 stop being silent wrong answers
/// and become compile errors.
#[derive(Clone, Debug)]
pub struct Keyed<V, C: Coordinate = coord::Any> {
    root: Option<Box<Node<V>>>,
    n: usize,
    system: CoordinateSystem,
    _coord: std::marker::PhantomData<C>,
}

impl<V, C: Coordinate> Default for Keyed<V, C> {
    fn default() -> Self {
        Keyed {
            root: None,
            n: 0,
            system: C::SYSTEM,
            _coord: std::marker::PhantomData,
        }
    }
}

/// TWO KEYED RELATIONS ARE EQUAL WHEN THEY BIND THE SAME KEYS TO THE SAME
/// VALUES, which is what a keyed relation IS — not when their tries have
/// the same shape. A derived equality would compare `root`, and a reader
/// would then be asserting something about branch bits rather than about
/// content (law 2: identity is content). The two happen to agree for a
/// PATRICIA trie, whose shape is determined by its key set, and agreeing
/// by accident is not a reason to assert the wrong one.
///
/// The walk is the left carrier's own, and every key it yields is read
/// from the right by a keyed read — no product of the two.
impl<V: PartialEq, C: Coordinate> PartialEq for Keyed<V, C> {
    fn eq(&self, other: &Self) -> bool {
        if self.n != other.n
            || (self.system != other.system
                && self.system != CoordinateSystem::Any
                && other.system != CoordinateSystem::Any)
        {
            return false;
        }
        self.scoped().all(|(k, v)| other.get_raw(&k) == Some(v))
    }
}

impl<V: Eq, C: Coordinate> Eq for Keyed<V, C> {}

impl<V> Keyed<V, coord::Any> {
    /// AN UNDECLARED CARRIER WITH ITS SYSTEM SET AT RUNTIME.
    ///
    /// The weaker half of the gate, kept for the carriers that have not yet
    /// named their coordinate in the type. A carrier that CAN name it does,
    /// and then this constructor is not reachable for it.
    pub fn for_system(system: CoordinateSystem) -> Self {
        Keyed {
            root: None,
            n: 0,
            system,
            _coord: std::marker::PhantomData,
        }
    }
}

impl<V, C: Coordinate> Keyed<V, C> {
    pub fn new() -> Self {
        Keyed::default()
    }

    pub fn system(&self) -> CoordinateSystem {
        self.system
    }

    /// Read an exact key under the declared coordinate system, refusing on a foreign system (Rule 11).
    pub fn read_exact(&self, key: &[u8]) -> Result<Option<&V>, SchemaRefusal> {
        if self.system != CoordinateSystem::Any {
            if let Some(tag) = crate::transducer::address::tag_of(key) {
                if let Some(expected) = self.system.tag() {
                    if tag != expected {
                        return Err(SchemaRefusal::IncompatibleSystem {
                            expected: self.system,
                            actual_tag: tag,
                        });
                    }
                }
            }
        }
        Ok(self.get_raw(key))
    }

    /// Read a fiber under the declared coordinate system, refusing on a foreign system (Rule 11).
    pub fn read_fiber<'a>(&'a self, prefix: Prefix<'a>) -> Result<Fiber<'a, V>, SchemaRefusal> {
        if self.system != CoordinateSystem::Any {
            if let Some(tag) = crate::transducer::address::tag_of(prefix.as_bytes()) {
                if let Some(expected) = self.system.tag() {
                    if tag != expected {
                        return Err(SchemaRefusal::IncompatibleSystem {
                            expected: self.system,
                            actual_tag: tag,
                        });
                    }
                }
            }
        }
        Ok(self.fiber(prefix))
    }

    /// Truncate a canonical coordinate key by depth (Rule 6).
    pub fn truncate_key(key: &[u8], depth: usize) -> Option<&[u8]> {
        if depth == 0 {
            return Some(&key[..0]);
        }
        let tag = crate::transducer::address::tag_of(key)?;
        match tag {
            crate::transducer::address::TAG_CHART => {
                let max_depth = (key.len() - 1) / 8;
                if depth > max_depth {
                    return None;
                }
                Some(&key[..1 + depth * 8])
            }
            _ => {
                if depth > 1 {
                    None
                } else {
                    Some(key)
                }
            }
        }
    }

    /// Bind `key` to `value`; a held key is rebound. Returns the value
    /// displaced, if any.
    /// Z4c — FILE AN OBJECT UNDER THE COPY IT ALREADY HOLDS.
    ///
    /// `insert` copies, which is right for a key built at the site: the
    /// caller's buffer is scratch and the trie must own what it keys by.
    /// A key that IS an object's address is different — the object holds
    /// it, several carriers name it, and the address of a form is
    /// megabytes (7.3 MB on the 590-vertex rule). Handing the trie that
    /// one copy makes every carrier naming the form point at the same
    /// bytes.
    ///
    /// Content, trie order and identity are unchanged; only who owns the
    /// bytes moves.
    pub(crate) fn insert_shared_raw(&mut self, key: std::sync::Arc<[u8]>, value: V) -> Option<V> {
        #[cfg(test)]
        work::count_insert();
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
                #[expect(clippy::disallowed_methods, reason = "rule 11 — one per resident entry: a count of what this trie holds cannot pass a 64-bit usize")]
                let n = self.n.saturating_add(1);
                self.n = n;
                None
            }
        }
    }

    pub(crate) fn insert_raw(&mut self, key: impl AsRef<[u8]>, value: V) -> Option<V> {
        #[cfg(test)]
        work::count_insert();
        let key: std::sync::Arc<[u8]> = std::sync::Arc::from(key.as_ref());
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

    pub(crate) fn get_raw(&self, key: &[u8]) -> Option<&V> {
        #[cfg(test)]
        work::count_get();
        self.root.as_deref()?.get(key)
    }

    pub(crate) fn get_mut_raw(&mut self, key: &[u8]) -> Option<&mut V> {
        self.root.as_deref_mut()?.get_mut(key)
    }

    /// BIND A COORDINATE OF THIS CARRIER'S OWN SYSTEM.
    ///
    /// The gate, and it takes the coordinate VALUE, never bytes. There is
    /// no way to reach this with a key of another system: `C` encodes
    /// itself, so `by_class` cannot be handed a member coordinate at all,
    /// and the eight sites of R1 are not wrong answers but programs that
    /// do not exist.
    pub fn put(&mut self, key: &C, value: V) -> Option<V> {
        self.insert_shared_raw(key.encode().arc(), value)
    }

    /// The exact read at a coordinate of this carrier's system.
    pub fn at(&self, key: &C) -> Option<&V> {
        self.get_raw(key.encode().as_bytes())
    }

    pub fn at_mut(&mut self, key: &C) -> Option<&mut V> {
        let k = key.encode();
        self.get_mut_raw(k.as_bytes())
    }

    pub fn holds(&self, key: &C) -> bool {
        self.at(key).is_some()
    }

    /// Unbind a coordinate of this carrier's system.
    pub fn drop_at(&mut self, key: &C) -> Option<V> {
        let k = key.encode();
        self.remove_raw(k.as_bytes())
    }

    /// THE FIBER OVER A DECLARED COARSER COORDINATE.
    ///
    /// `D: Stratum<C>` is the declaration that `D`'s opening bytes are the
    /// opening of a `C` key — a chart under a carrier of sections. Without
    /// the bound there is no fibre read, so a byte-prefix coincidence
    /// cannot be mistaken for a stratum.
    pub fn fiber_over<D: Stratum<C>>(&self, coarse: &D) -> Result<Fiber<'_, V>, CoordinateError> {
        let opening = coarse.opening();
        match Prefix::new(opening.as_bytes()) {
            Some(p) => Ok(self.fiber(p)),
            None => Err(D::refuse("the coordinate is empty, which names no stratum")),
        }
    }

    /// THE CARRIER'S ENTRIES, each key DECODED back to the coordinate it is.
    ///
    /// This is where invertibility earns its place: a walk hands back the
    /// chart, the section, the class — not bytes that a reader has to
    /// believe a comment about. A key that does not decode is a refusal
    /// carrying which coordinate failed, never an entry silently skipped.
    /// A KEY THIS CARRIER WROTE DECODES, because `put` is the only writer
    /// and it took the coordinate itself. A refusal here is a key that did
    /// NOT come through `put` — bytes off a tape, a trie built by hand —
    /// and [`Keyed::refusals`] counts them so the absence is a reading and
    /// not a silence (rule 11).
    pub fn entries(&self) -> impl Iterator<Item = (C, &V)> {
        self.scoped().filter_map(|(k, v)| C::decode(&k).ok().map(|c| (c, v)))
    }

    /// The keys of this carrier that do NOT decode to its coordinate, each
    /// naming what it failed to spell. Zero on a carrier only `put` wrote.
    pub fn refusals(&self) -> Vec<CoordinateError> {
        self.scoped().filter_map(|(k, _)| C::decode(&k).err()).collect()
    }

    /// Both readings at once, for a caller that must be total.
    pub fn decoded(&self) -> (Vec<(C, &V)>, Vec<CoordinateError>) {
        let mut ok = Vec::new();
        let mut bad = Vec::new();
        for (k, v) in self.scoped() {
            match C::decode(&k) {
                Ok(c) => ok.push((c, v)),
                Err(e) => bad.push(e),
            }
        }
        (ok, bad)
    }

    pub(crate) fn contains_raw(&self, key: &[u8]) -> bool {
        self.get_raw(key).is_some()
    }

    /// Unbind `key`; the value it held, if any.
    pub(crate) fn remove_raw(&mut self, key: &[u8]) -> Option<V> {
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

    /// THE BYTES THIS CARRIER'S KEYS OCCUPY, walked (§8.1).
    ///
    /// A key is the object's coordinates written out (law 2) and for a form
    /// it is megabytes, so "how many entries" says nothing about what a
    /// carrier costs and "how many bytes" says it exactly. Shared keys
    /// (Z4c) are counted where they are HELD, so a carrier naming a form by
    /// the one copy reports the same bytes as one that owns it — the
    /// reading is what this carrier reaches, not what it alone allocated,
    /// and the vector of them names the grower without any of them
    /// claiming the whole.
    ///
    /// Values are not counted: their sizes differ by type and summing them
    /// with the keys would make one number out of two facts (law 5's habit,
    /// one level down). A carrier whose VALUES grow is named by its entry
    /// count beside this, never folded into it.
    /// The trie's root, for the persist walk that writes it out as the
    /// trie it is (Z11). Sealed to the nerve: a reader outside it holds a
    /// carrier and asks at a coordinate, never a node.
    pub(crate) fn root_node(&self) -> Option<&Node<V>> {
        self.root.as_deref()
    }

    pub fn key_bytes(&self) -> usize {
        self.scoped().map(|(k, _)| k.len()).sum()
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// KEYS PRESENT IN BOTH CARRIERS — their meet, read along the
    /// trajectory the two keys' own coordinates trace together (A13). A key
    /// is its path, so where the two tries' paths part they share nothing
    /// below and that branch is not read; the cost is the structure the two
    /// share, never a product of the two fibers. The trajectory has no
    /// preferred direction and chooses nothing: it follows whichever branches
    /// both carriers hold, and the meet is the same set in any order of
    /// reading.
    pub fn meet<W, D: Coordinate>(&self, other: &Keyed<W, D>) -> Keyed<()> {
        let mut out = Keyed::new();
        meet_nodes(self.root.as_deref(), other.root.as_deref(), &mut out);
        out
    }

    /// SUBTREE UNDER A COORDINATE. The prefix cannot be empty: an empty
    /// prefix is not a coordinate, it is the whole carrier, and that
    /// reading is [`Keyed::scoped`], which says so at its site.
    ///
    /// THE WALK IS THE SUBTREE, NOT THE CARRIER. This pushed the root for
    /// every prefix and filtered the leaves as they came out — `stack` held
    /// `self.root` whatever the prefix was, and `Fiber` carried the prefix
    /// only to test each leaf against it —
    /// so a fiber read of one stratum cost the whole trie — every node of
    /// it, to yield the handful of keys under the prefix. That is rule 1's
    /// whole-carrier read arriving through the very reader the doctrine
    /// says replaces it, and it is why the rest of X3 existed: the empty
    /// prefix was the only honest spelling, because a prefix bought nothing.
    ///
    /// A key is its own path (A13), so the keys carrying a prefix hang
    /// under one node, and `subtree` reaches it by the prefix in O(bits). The leaf
    /// filter stays: path compression leaves bits between branch points
    /// untested, so the subtree can hold keys that diverge from the prefix
    /// at one of them, and `key_in_fiber` is what decides. What changes is
    /// the extent — the walk visits that subtree and nothing above it.
    pub fn fiber(&self, prefix: Prefix<'_>) -> Fiber<'_, V> {
        let prefix = prefix.as_bytes();
        let mut stack = Vec::new();
        if let Some(node) = self.subtree(prefix) {
            stack.push(node);
        }
        Fiber { stack, prefix: prefix.to_vec() }
    }

    /// The fiber under `prefix`, or the reading of an EMPTY prefix, which
    /// is not a fiber: `None`. Sites that build a key from coordinates and
    /// cannot prove it non-empty at the type level ask here and refuse the
    /// empty case themselves (rule 11), rather than reading the whole carrier.
    pub fn fiber_at(&self, prefix: &[u8]) -> Option<Fiber<'_, V>> {
        Prefix::new(prefix).map(|p| self.fiber(p))
    }

    /// THE WHOLE OF A CARRIER THAT IS ITSELF A FIBER — one bubble's charts,
    /// one vertex's star, one chart's incidence, one form's cited
    /// addresses. Calling this ASSERTS that scoping: the carrier was
    /// reached by a key already, so its whole content is one stratum's
    /// reading and reading it whole is not rule 1's whole-carrier read.
    ///
    /// It is the other half of [`Prefix`]: a site either has a coordinate
    /// to follow or is already at one, and which of the two it is
    /// should be legible without reading the carrier's provenance. The
    /// empty prefix said neither.
    pub fn scoped(&self) -> Fiber<'_, V> {
        let mut stack = Vec::new();
        if let Some(root) = self.root.as_deref() {
            stack.push(root);
        }
        Fiber { stack, prefix: Vec::new() }
    }

    /// The node every key carrying `prefix` hangs under, by following the
    /// prefix's own bits. `None` where no key carries it — an empty fiber,
    /// which is the reading of absence and not an empty walk of the trie.
    ///
    /// The empty prefix is the root node: a carrier that is itself a fiber
    /// has its whole content under one node, and that is the one case where
    /// reading the whole carrier is the reading (the rest of X3 names those sites).
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
                    if &k[..] != key {
                        return None;
                    }
                    let p = parent?;
                    return Some(Shell {
                        fiber: Fiber { stack: vec![p], prefix: Vec::new() },
                        prefix: clear_after_bit(key, parent_bit),
                    });
                }
                Node::Branch { bit, children, end } => {
                    if Node::<V>::bits(key) == Some(*bit) {
                        let (k, _) = end.as_deref()?;
                        if &k[..] != key {
                            return None;
                        }
                        return Some(Shell {
                            fiber: Fiber { stack: vec![node], prefix: Vec::new() },
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
    /// the last key from the first and ending where it began (45.7). `None`
    /// starts at the first key. The last yielded key is the resume cursor;
    /// `Cursor::circuit` reports a completed circuit.
    ///
    /// X3: A WALK THAT HAS NOT STARTED HAS NO CURSOR, and that is an
    /// absence, not an empty key. This took `&[u8]` and spelled the
    /// absence `ROOT` — then converted it to `Option` on its own first
    /// line, which is the sentinel being translated one step later than it
    /// should be (rule 11). Eight resume cursors in the kernel held a
    /// `Vec<u8>` that was empty until the first yield for the same reason.
    pub fn cursor<'a>(&'a self, from: Option<&[u8]>, budget: &'a mut crate::physics::Budget) -> Cursor<'a, V, C> {
        let from = from.map(|f| f.to_vec());
        Cursor {
            keyed: self,
            // THE CIRCUIT IS THE WHOLE CARRIER BY DESIGN (45.7) and its bound
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

/// THE UNDECLARED CARRIER — loose bytes in, loose bytes out.
///
/// Every reader and writer that has not yet named the coordinate it answers
/// lives here. These are the SAME operations as [`Keyed::put`] and
/// [`Keyed::at`] with the check removed, and the count of carriers still
/// spelled `Keyed<V>` is the remaining work of §4.4 — not a permanent half
/// of the API. A carrier that names its system in the type loses access to
/// this block entirely, which is what makes the naming a gate rather than a
/// suggestion.
impl<V> Keyed<V, coord::Any> {
    /// Z4c — FILE AN OBJECT UNDER THE COPY IT ALREADY HOLDS.
    pub fn insert_shared(&mut self, key: std::sync::Arc<[u8]>, value: V) -> Option<V> {
        self.insert_shared_raw(key, value)
    }

    pub fn insert(&mut self, key: impl AsRef<[u8]>, value: V) -> Option<V> {
        self.insert_raw(key, value)
    }

    pub fn get(&self, key: &[u8]) -> Option<&V> {
        self.get_raw(key)
    }

    pub fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        self.get_mut_raw(key)
    }

    pub fn contains(&self, key: &[u8]) -> bool {
        self.contains_raw(key)
    }

    pub fn remove(&mut self, key: &[u8]) -> Option<V> {
        self.remove_raw(key)
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

/// A NON-EMPTY KEY PREFIX — the coordinate a fiber read follows.
///
/// The empty slice is not a coordinate: it names the whole carrier, and
/// the whole carrier is not a fiber unless the carrier is one already.
/// Keeping the two apart in the TYPE is what makes rule 1 readable at a
/// site: `fiber(Prefix)` reaches somewhere by a prefix, `scoped()` says this
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
pub(crate) fn keyed_persist_walk<V, C: Coordinate>(k: &Keyed<V, C>, f: &mut impl FnMut(&[u8], &V)) {
    // A SNAPSHOT IS THE CARRIER. Persisting less than all of it is a
    // ledger that loses what it does not name, which is X8's row from the
    // other side; there is no prefix here and there should not be one.
    for (key, v) in k.scoped() {
        f(&key, v);
    }
}
