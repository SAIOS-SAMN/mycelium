// A9 — THE SELECTION TERM: the anchor a family derives, and the key that
// names it.

use super::*;

/// The term's A21 key. One carrier for it: [`Sel::key`], which the
/// derivation orders by as well.
pub(crate) fn sel_key(s: &Sel) -> crate::transducer::address::Address {
    s.key()
}

/// π* derivation from a pooled evidence family: the first anchor term
/// common to every evidence's carrying set (the intersection move — one
/// evidence never binds; the caller holds families of at least two).
pub fn derive_pi(family: &[crate::transducer::anchor::Evidence]) -> Option<Sel> {
    let sets: Vec<Vec<Sel>> =
        family.iter().map(|(fp, scene)| derive_carrying_all(fp, scene)).collect();
    let (first, rest) = sets.split_first()?;
    let rest_tries: Vec<Keyed<()>> = rest
        .iter()
        .map(|s| {
            let mut k = Keyed::new();
            for t in s {
                k.insert(sel_key(t), ());
            }
            k
        })
        .collect();
    let mut common: Vec<Sel> = first
        .iter()
        .filter(|t| {
            let key = sel_key(t);
            rest_tries.iter().all(|s| s.contains(&key))
        })
        .cloned()
        .collect();
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — canonical key order: the terms are ordered by their own A21 key, so every node reads one common carrier from one set"
    )]
    common.sort_by_key(sel_key);
    common.into_iter().next()
}
