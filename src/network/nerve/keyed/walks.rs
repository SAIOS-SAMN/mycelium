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
        Fiber { stack: Vec::new(), prefix: ROOT.to_vec() }
    }
}

impl<'a, V> Iterator for Fiber<'a, V> {
    type Item = (Vec<u8>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            match node {
                Node::Leaf(k, v) => {
                    if key_in_fiber(k, &self.prefix) {
                        return Some((k.clone(), v));
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
                            return Some((k.clone(), v));
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
    /// The inner trie at `key` — cells on a vertex, bubbles on a chart.
    pub fn link(&self, key: &[u8]) -> Link<'_, T> {
        Link {
            fiber: match self.get(key) {
                Some(inner) => inner.fiber(ROOT),
                None => Fiber::empty(),
            },
        }
    }
}

/// Budgeted walk in key order from `from`, consuming a [`crate::physics::Budget`].
pub struct Cursor<'a, V> {
    pub(crate) fiber: Fiber<'a, V>,
    pub(crate) budget: &'a mut crate::physics::Budget,
    pub(crate) from: Vec<u8>,
    pub(crate) last: Option<Vec<u8>>,
}

impl<'a, V> Cursor<'a, V> {
    /// Last key yielded, so the next stroke resumes after it.
    pub fn last_key(&self) -> Option<&[u8]> {
        self.last.as_deref()
    }
}

impl<'a, V> Iterator for Cursor<'a, V> {
    type Item = (Vec<u8>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.budget.remaining() == 0 {
                return None;
            }
            let (k, v) = self.fiber.next()?;
            if !self.from.is_empty() && k.as_slice() <= self.from.as_slice() {
                continue;
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
