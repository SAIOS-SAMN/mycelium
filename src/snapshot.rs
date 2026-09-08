// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Law 3, 28 — THE SEALED DOOR: the one place a Keyed yields every entry,
//! in key order, for the wire. Cognition names `to_bytes` / `snapshot` on
//! the carrier, never this trait's supertrait.

use crate::network::nerve::Keyed;

mod sealed {
    pub trait Sealed {
        type Value;
        fn walk<F: FnMut(&[u8], &Self::Value)>(&self, f: F);
    }
}

/// The only place a Keyed yields all entries. The supertrait is private
/// to this module; cognition cannot name it.
pub trait Persist: sealed::Sealed {
    fn persist<F: FnMut(&[u8], &Self::Value)>(&self, f: F) {
        sealed::Sealed::walk(self, f);
    }
}

impl<V> sealed::Sealed for Keyed<V> {
    type Value = V;
    fn walk<F: FnMut(&[u8], &V)>(&self, mut f: F) {
        crate::network::nerve::keyed_persist_walk(self, &mut f);
    }
}

impl<V> Persist for Keyed<V> {}

/// Walk `k` in key order. Callers live in this module or in a carrier's
/// `to_bytes` that is the wire of that carrier.
pub fn persist<V>(k: &Keyed<V>, f: impl FnMut(&[u8], &V)) {
    <Keyed<V> as Persist>::persist(k, f);
}
