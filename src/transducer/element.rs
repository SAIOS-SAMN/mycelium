// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE MULTI-AXIAL ELEMENT — the uniform carrier of an ANASTOME germ. NOT an
//! enum of "kinds" (that would cap dimensional capacity and force kind-branching
//! — the flat-taxonomy sin). It is a sparse element over an EXTENSIBLE set of
//! fibers: `(fiber_index → exact component bytes)`. Conformal content, gauge
//! relabel, temporal/shell, and germinated residual fibers are all just FIBERS
//! of the one element — never variants. A pivot germinates by allocating a NEW
//! fiber index (dimensional resolution grows the axis count). The kernel treats
//! it OPAQUELY: equality + `address()` + the demand it reproduces; the component
//! bytes are ANASTOME, not reasoned-on English.

/// Reserved base fibers (the four strata); germinated fibers take indices ≥ 4.
pub const FIBER_SPATIAL: u16 = 0;
pub const FIBER_GAUGE: u16 = 1;
pub const FIBER_TEMPORAL: u16 = 2;
pub const FIBER_SHELL: u16 = 3;
/// First index available to germinated (residual/new-axis) fibers.
pub const FIBER_GERMINATED: u16 = 4;

/// A germ as a sparse multi-axial element: fibers sorted by index, each carrying
/// its exact component bytes. Empty fibers are simply absent (sparse).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Element {
    fibers: Vec<(u16, Vec<u8>)>,
}

impl Element {
    pub fn new() -> Self {
        Element::default()
    }

    /// Set (or replace) a fiber's exact component. An empty component clears it,
    /// keeping the element sparse. Kept sorted by fiber index (canonical order).
    pub fn set_fiber(&mut self, index: u16, bytes: Vec<u8>) {
        self.fibers.retain(|(i, _)| *i != index);
        if !bytes.is_empty() {
            self.fibers.push((index, bytes));
            self.fibers.sort_by_key(|(i, _)| *i);
        }
    }

    /// A builder-style set.
    pub fn with_fiber(mut self, index: u16, bytes: Vec<u8>) -> Self {
        self.set_fiber(index, bytes);
        self
    }

    pub fn fiber(&self, index: u16) -> Option<&[u8]> {
        self.fibers.iter().find(|(i, _)| *i == index).map(|(_, b)| b.as_slice())
    }

    /// The fibers this element carries content on — its occupied axes.
    pub fn axes(&self) -> Vec<u16> {
        self.fibers.iter().map(|(i, _)| *i).collect()
    }

    /// The highest germinated fiber in use (for allocating the next new axis).
    pub fn max_axis(&self) -> Option<u16> {
        self.fibers.iter().map(|(i, _)| *i).max()
    }

    pub fn is_empty(&self) -> bool {
        self.fibers.is_empty()
    }

    /// Canonical ANASTOME bytes: `[fiber_id:2][len:4][bytes]` per fiber, in fiber
    /// order. Order-independent by construction (fibers are kept sorted).
    pub fn canonical(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for (i, b) in &self.fibers {
            out.extend_from_slice(&i.to_le_bytes());
            out.extend_from_slice(&(b.len() as u32).to_le_bytes());
            out.extend_from_slice(b);
        }
        out
    }

    /// The element's canonical ADDRESS — a digest of its ANASTOME form. Wire/cache
    /// identity for matching consilience receipts; never a reasoning operand.
    pub fn address(&self) -> [u8; 32] {
        *blake3::hash(&self.canonical()).as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibers_are_sparse_sorted_and_order_independent() {
        let a = Element::new().with_fiber(FIBER_GAUGE, vec![9]).with_fiber(FIBER_SPATIAL, vec![1, 2]);
        let b = Element::new().with_fiber(FIBER_SPATIAL, vec![1, 2]).with_fiber(FIBER_GAUGE, vec![9]);
        assert_eq!(a.address(), b.address(), "insertion order does not change identity");
        assert_eq!(a.axes(), vec![FIBER_SPATIAL, FIBER_GAUGE]);
        assert_eq!(a.fiber(FIBER_SPATIAL), Some(&[1, 2][..]));
    }

    /// Dimensional resolution: germinating a NEW fiber extends the element
    /// without disturbing existing fibers or their address-of-the-rest.
    #[test]
    fn a_new_fiber_extends_without_touching_the_others() {
        let base = Element::new().with_fiber(FIBER_SPATIAL, vec![1, 2]);
        let grown = base.clone().with_fiber(FIBER_GERMINATED, vec![7, 7, 7]);
        assert_eq!(grown.fiber(FIBER_SPATIAL), base.fiber(FIBER_SPATIAL), "old axis intact");
        assert_ne!(grown.address(), base.address(), "the new axis changes the whole germ's identity");
        assert_eq!(grown.max_axis(), Some(FIBER_GERMINATED));
    }

    #[test]
    fn distinct_content_distinct_address_same_content_same() {
        let x = Element::new().with_fiber(FIBER_SPATIAL, vec![1]);
        let y = Element::new().with_fiber(FIBER_SPATIAL, vec![2]);
        let z = Element::new().with_fiber(FIBER_SPATIAL, vec![1]);
        assert_ne!(x.address(), y.address());
        assert_eq!(x.address(), z.address());
    }

    #[test]
    fn clearing_a_fiber_keeps_it_sparse() {
        let mut e = Element::new().with_fiber(FIBER_SPATIAL, vec![1]).with_fiber(FIBER_GAUGE, vec![2]);
        e.set_fiber(FIBER_GAUGE, vec![]); // empty clears
        assert_eq!(e.axes(), vec![FIBER_SPATIAL]);
    }
}
