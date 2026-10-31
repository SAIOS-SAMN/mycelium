// A6 — THE STRATUM: one level of the chart trie, and the refinement that
// splits it when a new coordinate arrives.

use super::*;

/// One stratum of the atlas: the branches refine by the next axis of the
/// chart, a keyed relation over the axis value (read by its bits, never
/// scanned among siblings); the members are the addresses whose chart
/// ends here.
///
/// X3: BOTH CARRIERS ARE ALREADY ONE NODE'S FIBRE, and every iteration
/// below reads them with `scoped`. A `Stratum` is reached by following the
/// chart's axes — `branches.get(&axis_key(a))`, one keyed read per axis —
/// so `members` is the addresses at THIS chart and `branches` is this
/// node's own children. There is no prefix to follow inside either: the
/// prefix was already followed, above.
#[derive(Debug, Clone, Default)]
pub(crate) struct Stratum {
    branches: Keyed<Stratum>,
    members: Keyed<()>,
}

/// An axis value as a key: the coordinate's bytes, big-endian, so the
/// trie's order is the axis's order.
/// A STRATUM BRANCHES ON A WHOLE COORDINATE (A21 as ruled 2026-09-18).
/// The branch key is the coordinate's own kind-tagged, length-prefixed
/// bytes — the same unit `Invariant::key` writes — so the chart a word is
/// filed under and the path a fibre follows are one construction (law 12).
/// It was eight bare bytes, which is one `u64` limb and not one coordinate.
pub(crate) fn axis_key(a: &ChartAxis) -> Vec<u8> {
    a.key_bytes()
}

/// The coordinate a branch key names. `None` where the bytes are not one:
/// a branch that does not decode contributes no axis rather than an
/// invented one (rule 11).
pub(crate) fn axis_of(key: &[u8]) -> Option<ChartAxis> {
    ChartAxis::from_key_bytes(key)
}

impl Stratum {
    /// Z10 — the bytes this stratum and everything under it reach: its
    /// members' addresses and its branches' coordinates, all the way down.
    /// An atlas is a trie of tries and a reading that stopped at the root
    /// would report a chart hierarchy as one entry.
    pub(crate) fn bytes(&self) -> usize {
        let mut n = self.members.key_bytes();
        for (k, b) in self.branches.scoped() {
            n += k.len() + b.bytes();
        }
        n
    }

    pub(crate) fn admit(&mut self, axes: &[ChartAxis], addr: &[u8]) {
        match axes.split_first() {
            None => {
                self.members.insert(addr, ());
            }
            Some((a, rest)) => {
                let key = axis_key(a);
                match self.branches.get_mut(&key) {
                    Some(s) => s.admit(rest, addr),
                    None => {
                        let mut s = Stratum::default();
                        s.admit(rest, addr);
                        self.branches.insert(key, s);
                    }
                }
            }
        }
    }

    pub(crate) fn fibre(&self, axes: &[ChartAxis]) -> Vec<Vec<u8>> {
        match axes.split_first() {
            None => self.members.scoped().map(|(k, _)| k).collect(),
            // A stratum with no branch on this axis holds no members
            // under it: the empty fibre is that reading (0.11c).
            Some((a, rest)) => match self.branches.get(&axis_key(a)) {
                Some(s) => s.fibre(rest),
                None => Vec::new(),
            },
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.members.is_empty() && self.branches.is_empty()
    }

    /// Drop one member along its own chart path; strata left empty are pruned.
    pub(crate) fn remove(&mut self, axes: &[ChartAxis], addr: &[u8]) {
        match axes.split_first() {
            None => {
                self.members.remove(addr);
            }
            Some((a, rest)) => {
                let key = axis_key(a);
                let empty = {
                    let Some(s) = self.branches.get_mut(&key) else { return };
                    s.remove(rest, addr);
                    s.is_empty()
                };
                if empty {
                    self.branches.remove(&key);
                }
            }
        }
    }

    pub(crate) fn len(&self) -> usize {
        let mut n = self.members.len();
        for (_, s) in self.branches.scoped() {
            n += s.len();
        }
        n
    }

    pub(crate) fn strata(&self, path: &mut Vec<ChartAxis>, out: &mut Vec<(Invariant, Vec<Vec<u8>>)>) {
        if !self.members.is_empty() {
            let members: Vec<Vec<u8>> = self.members.scoped().map(|(k, _)| k).collect();
            out.push((Invariant(path.clone()), members));
        }
        for (k, s) in self.branches.scoped() {
            let Some(a) = axis_of(&k) else { continue };
            path.push(a);
            s.strata(path, out);
            path.pop();
        }
    }

    /// 78.153 — every populated stratum with HOW MANY words it holds, and
    /// not the words: one read per stratum visited. False where the stroke
    /// ran out first; what was read is then not a reading of the cover.
    pub(crate) fn strata_counted(
        &self,
        path: &mut Vec<ChartAxis>,
        out: &mut Vec<(Invariant, usize)>,
        spend: &mut dyn FnMut() -> bool,
    ) -> bool {
        if !spend() {
            return false;
        }
        if !self.members.is_empty() {
            out.push((Invariant(path.clone()), self.members.len()));
        }
        for (k, s) in self.branches.scoped() {
            let Some(a) = axis_of(&k) else { continue };
            path.push(a);
            let read = s.strata_counted(path, out, spend);
            path.pop();
            if !read {
                return false;
            }
        }
        true
    }

    pub(crate) fn child_axes(&self, axes: &[ChartAxis]) -> Vec<ChartAxis> {
        match axes.split_first() {
            None => {
                let mut out = Vec::new();
                for (k, _) in self.branches.scoped() {
                    if let Some(a) = axis_of(&k) {
                        out.push(a);
                    }
                }
                out
            }
            // No branch on this axis, no axes under it.
            Some((a, rest)) => match self.branches.get(&axis_key(a)) {
                Some(s) => s.child_axes(rest),
                None => Vec::new(),
            },
        }
    }

    // `branches_at(axes)` stood here and handed out a stratum's whole branch
    // carrier so `each_child` could page it. Both are gone with the outward
    // read they existed for: an idea meets the charts its own vertices stand
    // on, and a chart's members are `fibre`, not a page of its siblings.
    // Nothing else in the tree ever asked a stratum for its branches.

    pub(crate) fn collect_members(&self, out: &mut Vec<Vec<u8>>) {
        for (k, _) in self.members.scoped() {
            out.push(k);
        }
        for (_, s) in self.branches.scoped() {
            s.collect_members(out);
        }
    }

    pub(crate) fn fibre_under(&self, axes: &[ChartAxis]) -> Vec<Vec<u8>> {
        match axes.split_first() {
            None => {
                let mut out = Vec::new();
                self.collect_members(&mut out);
                out
            }
            // No branch on this axis, no members under it.
            Some((a, rest)) => match self.branches.get(&axis_key(a)) {
                Some(s) => s.fibre_under(rest),
                None => Vec::new(),
            },
        }
    }
}
