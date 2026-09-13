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

/// WRITE A SECTION AND COUNT WHAT IT WROTE, in one act.
///
/// Every section that carries a count wrote `carrier.len()` — a stored
/// counter — and then wrote its content by walking the carrier with
/// [`persist`]. One number, two independent sources, nothing holding them
/// together: law 12 at the door every carrier leaves through.
///
/// They parted in the field. Two of three hyphae held ledgers whose COC1
/// section announced five complexes and carried four; each court's own
/// reader believed the announcement, read a fifth object where the next
/// section began, and refused its whole history. The content was intact.
/// The number written beside it was not a reading of that content.
///
/// So the body is written first and measured as it is written, and the count
/// is that act's own measure. A carrier whose `len` disagrees with its walk
/// can no longer produce a tape that lies about itself — and where those two
/// disagree is a fault in the carrier, to be found there and not papered
/// over by whichever reading the wire happened to take.
pub fn counted<V>(
    out: &mut Vec<u8>,
    carrier: &Keyed<V>,
    mut write: impl FnMut(&mut Vec<u8>, &[u8], &V),
) {
    let mut body = Vec::new();
    let mut n = 0u32;
    persist(carrier, |key, v| {
        n = n.saturating_add(1);
        write(&mut body, key, v);
    });
    out.extend_from_slice(&n.to_le_bytes());
    out.append(&mut body);
}
