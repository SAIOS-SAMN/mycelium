// A6 — THE STRATUM: one level of the chart trie, and the refinement that
// splits it when a new coordinate arrives.

use super::*;

/// One stratum of the atlas: the branches refine by the next axis of the
/// chart, a keyed relation over the axis value (read by its bits, never
/// scanned among siblings); the members are the addresses whose chart
/// ends here.
#[derive(Debug, Clone, Default)]
pub(crate) struct Stratum {
    branches: Keyed<Stratum>,
    members: Keyed<()>,
}

/// An axis value as a key: the coordinate's bytes, big-endian, so the
/// trie's order is the axis's order.
pub(crate) fn axis_key(a: u64) -> [u8; 8] {
    crate::network::nerve::axis_key_le(a)
}

pub(crate) fn axis_of(key: &[u8]) -> u64 {
    let mut b = [0u8; 8];
    let n = key.len().min(8);
    b[..n].copy_from_slice(&key[..n]);
    u64::from_le_bytes(b)
}

impl Stratum {
    pub(crate) fn admit(&mut self, axes: &[u64], addr: &[u8]) {
        match axes.split_first() {
            None => {
                self.members.insert(addr, ());
            }
            Some((&a, rest)) => {
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

    pub(crate) fn fiber(&self, axes: &[u64]) -> Vec<Vec<u8>> {
        match axes.split_first() {
            None => self.members.fiber(ROOT).map(|(k, _)| k).collect(),
            Some((&a, rest)) => self.branches.get(&axis_key(a)).map(|s| s.fiber(rest)).unwrap_or_default(),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.members.is_empty() && self.branches.is_empty()
    }

    /// Drop one member along its own chart path; strata left empty are pruned.
    pub(crate) fn remove(&mut self, axes: &[u64], addr: &[u8]) {
        match axes.split_first() {
            None => {
                self.members.remove(addr);
            }
            Some((&a, rest)) => {
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
        for (_, s) in self.branches.fiber(ROOT) {
            n += s.len();
        }
        n
    }

    pub(crate) fn strata(&self, path: &mut Vec<u64>, out: &mut Vec<(Invariant, Vec<Vec<u8>>)>) {
        if !self.members.is_empty() {
            let members: Vec<Vec<u8>> = self.members.fiber(ROOT).map(|(k, _)| k).collect();
            out.push((Invariant(path.clone()), members));
        }
        for (k, s) in self.branches.fiber(ROOT) {
            path.push(axis_of(&k));
            s.strata(path, out);
            path.pop();
        }
    }

    pub(crate) fn child_axes(&self, axes: &[u64]) -> Vec<u64> {
        match axes.split_first() {
            None => {
                let mut out = Vec::new();
                for (k, _) in self.branches.fiber(ROOT) {
                    out.push(axis_of(&k));
                }
                out
            }
            Some((&a, rest)) => self
                .branches
                .get(&axis_key(a))
                .map(|s| s.child_axes(rest))
                .unwrap_or_default(),
        }
    }

    pub(crate) fn branches_at(&self, axes: &[u64]) -> Option<&Keyed<Stratum>> {
        match axes.split_first() {
            None => Some(&self.branches),
            Some((&a, rest)) => self.branches.get(&axis_key(a)).and_then(|s| s.branches_at(rest)),
        }
    }

    pub(crate) fn collect_members(&self, out: &mut Vec<Vec<u8>>) {
        for (k, _) in self.members.fiber(ROOT) {
            out.push(k);
        }
        for (_, s) in self.branches.fiber(ROOT) {
            s.collect_members(out);
        }
    }

    pub(crate) fn fiber_under(&self, axes: &[u64]) -> Vec<Vec<u8>> {
        match axes.split_first() {
            None => {
                let mut out = Vec::new();
                self.collect_members(&mut out);
                out
            }
            Some((&a, rest)) => self
                .branches
                .get(&axis_key(a))
                .map(|s| s.fiber_under(rest))
                .unwrap_or_default(),
        }
    }
}
