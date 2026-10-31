// A9 — THE SELECTION TERM: the anchor a family derives, and the key that
// names it.

use crate::network::nerve::Keyed;
use crate::transducer::anchor::{derive_carrying_all, Selection};

/// The term's A21 key. One carrier for it: [`Selection::key`], which the
/// derivation orders by as well.
pub(crate) fn selection_key(s: &Selection) -> crate::transducer::address::Address {
    s.key()
}

/// π* derivation from a pooled evidence family: the first anchor term
/// common to every evidence's carrying set (the intersection move — one
/// evidence never binds; the caller holds families of at least two).
pub fn derive_pi(family: &[crate::transducer::anchor::Evidence]) -> Option<Selection> {
    let sets: Vec<Vec<Selection>> =
        family.iter().map(|(fp, scene)| derive_carrying_all(fp, scene)).collect();
    let (first, rest) = sets.split_first()?;
    let rest_tries: Vec<Keyed<()>> = rest
        .iter()
        .map(|s| {
            let mut k = Keyed::new();
            for t in s {
                k.insert(selection_key(t), ());
            }
            k
        })
        .collect();
    let mut common: Vec<Selection> = first
        .iter()
        .filter(|t| {
            let key = selection_key(t);
            rest_tries.iter().all(|s| s.contains(&key))
        })
        .cloned()
        .collect();
    // The order is the terms' own A21 keys, so every node sorts the common
    // set alike — and then the HEAD IS TAKEN. Where several terms carry
    // every evidence, the one with the least key is bound as π* and the
    // others are dropped: a choice by the sort key, not a reading of it.
    // L9a's row, open since Stage 14.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — a choice by the term's A21 key, consumed by next(); owed to L9a"
    )]
    common.sort_by_key(selection_key);
    common.into_iter().next()
}
