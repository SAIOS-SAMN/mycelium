// A9 — THE SELECTION TERM: the anchor a family derives, and the key that
// names it.

use super::*;

/// π* derivation from a pooled evidence family: the first anchor term
/// common to every evidence's carrying set (the intersection move — one
/// evidence never binds; the caller holds families of at least two).
pub(crate) fn sel_key(s: &Sel) -> crate::transducer::a21::Address {
    let mut buf = vec![crate::transducer::a21::TAG_SEL];
    s.to_bytes(&mut buf);
    crate::transducer::a21::Address::new(buf)
}

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
    common.sort_by_key(sel_key);
    common.into_iter().next()
}
