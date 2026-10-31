// 78.151 — RUNS: an ordered carrier held as immutable runs behind shared
// pointers, so two published readings share every run neither changed.

use std::sync::Arc;

/// AN ORDERED SEQUENCE AS RUNS. Each run is in order and every run stands
/// wholly after the one before it. A new run joins at the end, and two
/// runs merge where the newer is at least as long as the one before it —
/// so the runs' lengths fall strictly from first to last, there are at most
/// one more runs than the bits of the length, and a member is copied again
/// once per merge it takes part in. A clone copies the run pointers, never
/// a member.
#[derive(Debug)]
pub(crate) struct Runs<T> {
    runs: Vec<Arc<[T]>>,
}

/// WHAT ONE EXTENSION COPIED: members, and the members' weight summed. A sum past
/// its carrier is not summed further and is named (`passed`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Copied {
    pub(crate) members: u64,
    pub(crate) weight: u64,
    pub(crate) passed: bool,
}

impl Copied {
    fn add<T>(&mut self, members: &[T], weight: &impl Fn(&T) -> u64) {
        for m in members {
            match (self.members.checked_add(1), self.weight.checked_add(weight(m))) {
                (Some(n), Some(w)) => {
                    self.members = n;
                    self.weight = w;
                }
                _ => self.passed = true,
            }
        }
    }
}

impl<T> Clone for Runs<T> {
    fn clone(&self) -> Self {
        Runs { runs: self.runs.clone() }
    }
}

impl<T> Default for Runs<T> {
    fn default() -> Self {
        Runs { runs: Vec::new() }
    }
}

impl<T: Clone> Runs<T> {
    /// THE SEQUENCE EXTENDED by `next`, which stands wholly after every
    /// member held: one run joined, merged down while it is at least as long
    /// as the one before it. Answers the members copied — `next` itself, and
    /// every member a merge copied again — and the members' `weight` summed.
    pub(crate) fn extend(&mut self, next: Vec<T>, weight: impl Fn(&T) -> u64) -> Copied {
        let mut copied = Copied::default();
        if next.is_empty() {
            return copied;
        }
        copied.add(&next, &weight);
        self.runs.push(Arc::from(next));
        while let [.., before, newest] = self.runs.as_slice() {
            if newest.len() < before.len() {
                break;
            }
            let mut joined: Vec<T> = Vec::with_capacity(before.len() + newest.len());
            joined.extend(before.iter().cloned());
            joined.extend(newest.iter().cloned());
            copied.add(&joined, &weight);
            self.runs.truncate(self.runs.len() - 2);
            self.runs.push(Arc::from(joined));
        }
        copied
    }

    /// The members whose coordinate (`at`) stands strictly past `after`
    /// (every member where `None`), in order: each run is entered at its
    /// first such member, found by bisection.
    pub(crate) fn past<'a, K: Ord + Copy + 'a>(&'a self, at: impl Fn(&T) -> K + Copy + 'a, after: Option<K>) -> impl Iterator<Item = &'a T> + 'a {
        self.runs.iter().flat_map(move |run| {
            let from = match after {
                Some(a) => run.partition_point(|m| at(m) <= a),
                None => 0,
            };
            run.get(from..).into_iter().flatten()
        })
    }

    /// The member at the end of the sequence.
    pub(crate) fn end(&self) -> Option<&T> {
        match self.runs.as_slice() {
            [.., newest] => match &**newest {
                [.., m] => Some(m),
                [] => None,
            },
            [] => None,
        }
    }
}
