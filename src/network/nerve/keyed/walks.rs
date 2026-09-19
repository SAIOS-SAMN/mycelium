// A13 — THE WALKS: a fiber, a link, a cursor and a shell, each a read of
// structure rather than a search over a list.

use super::*;

/// Subtree under a key prefix. The only read of more than one entry that
/// is not a link, a peel, or a budgeted cursor.
pub struct Fiber<'a, V> {
    pub(crate) stack: Vec<&'a Node<V>>,
    pub(crate) prefix: Vec<u8>,
}

impl<'a, V> Fiber<'a, V> {
    fn empty() -> Self {
        Fiber { stack: Vec::new(), prefix: Vec::new() }
    }
}

impl<'a, V> Iterator for Fiber<'a, V> {
    type Item = (Vec<u8>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            #[cfg(test)]
            work::count_visit();
            match node {
                Node::Leaf(k, v) => {
                    if key_in_fiber(k, &self.prefix) {
                        return Some((k.to_vec(), v));
                    }
                }
                Node::Branch { children, end, .. } => {
                    if let Some(c) = children[1].as_deref() {
                        self.stack.push(c);
                    }
                    if let Some(c) = children[0].as_deref() {
                        self.stack.push(c);
                    }
                    if let Some((k, v)) = end.as_deref() {
                        if key_in_fiber(k, &self.prefix) {
                            return Some((k.to_vec(), v));
                        }
                    }
                }
            }
        }
        None
    }
}

/// Entries incident to one key: the inner trie at that key.
pub struct Link<'a, V> {
    pub(crate) fiber: Fiber<'a, V>,
}

impl<'a, V> Iterator for Link<'a, V> {
    type Item = (Vec<u8>, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        self.fiber.next()
    }
}

impl<T> Keyed<Keyed<T>> {
    /// The inner trie at `key` — simplices on a vertex, bubbles on a chart.
    pub fn link(&self, key: &[u8]) -> Link<'_, T> {
        Link {
            fiber: match self.get(key) {
                // The inner trie AT a key: `get` is the descent, so what it
                // returns is already one stratum's fiber.
                Some(inner) => inner.scoped(),
                None => Fiber::empty(),
            },
        }
    }
}

/// Budgeted ORBIT in key order from `from`, consuming a
/// [`crate::physics::Budget`] (45.7, S9.14). The walk continues past the
/// last key from the first and ends where it began, so a carrier larger
/// than one budget is walked whole across strokes and a key admitted below
/// the resume point is reached on the next circuit; a cursor that stopped
/// at the end of the key order had walked off the edge of its book.
/// `circuit` reports that the walk passed the end of the order — one
/// completed circuit of the carrier, which the stroke counts as its orbit.
pub struct Cursor<'a, V> {
    pub(crate) keyed: &'a Keyed<V>,
    pub(crate) fiber: Fiber<'a, V>,
    pub(crate) budget: &'a mut crate::physics::Budget,
    /// The key the walk began after; `None` began at the first key.
    pub(crate) from: Option<Vec<u8>>,
    pub(crate) last: Option<Vec<u8>>,
    pub(crate) wrapped: bool,
    pub(crate) done: bool,
}

impl<'a, V> Cursor<'a, V> {
    /// Last key yielded, so the next stroke resumes after it.
    pub fn last_key(&self) -> Option<&[u8]> {
        self.last.as_deref()
    }

    /// The walk passed the last key of the order and continued from the
    /// first: one circuit of the carrier completed under this cursor.
    pub fn circuit(&self) -> bool {
        self.wrapped
    }
}

impl<'a, V> Iterator for Cursor<'a, V> {
    type Item = (Vec<u8>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.done || self.budget.remaining() == 0 {
                return None;
            }
            let Some((k, v)) = self.fiber.next() else {
                // The end of the key order. An empty carrier has no
                // circuit; a walk from the first key has completed one and
                // ends; a walk from `from` continues from the first key.
                if self.wrapped || self.keyed.is_empty() {
                    self.done = true;
                    return None;
                }
                self.wrapped = true;
                if self.from.is_none() {
                    self.done = true;
                    return None;
                }
                self.fiber = self.keyed.scoped();
                continue;
            };
            match (&self.from, self.wrapped) {
                // Before the wrap: the keys after `from` only.
                (Some(f), false) if &k[..] <= f.as_slice() => continue,
                // After it: the keys skipped, up to and including `from` —
                // the walk ends where it began.
                (Some(f), true) if &k[..] > f.as_slice() => {
                    self.done = true;
                    return None;
                }
                _ => {}
            }
            if !self.budget.spend() {
                return None;
            }
            self.last = Some(k.clone());
            return Some((k, v));
        }
    }
}

/// One shell: the parent prefix of a peel, and the entries at that branch.
pub struct Shell<'a, V> {
    pub(crate) fiber: Fiber<'a, V>,
    pub(crate) prefix: Vec<u8>,
}

impl<'a, V> Shell<'a, V> {
    pub fn key(&self) -> &[u8] {
        &self.prefix
    }
}

impl<'a, V> Iterator for Shell<'a, V> {
    type Item = (Vec<u8>, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        self.fiber.next()
    }
}
