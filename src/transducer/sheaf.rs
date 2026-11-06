// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 13, 26 — THE SHEAF OF THE LANGUAGE over the atlas: each holder measures a
//! section digest per stratum of its own words. Strata whose roots agree glue;
//! a discordant one names exactly the fibre whose sections differ.

use super::element::Element;
use super::stalk::{invariant_of, Invariant, Stalks};
use crate::court::admission::Locator;

/// One stratum's section digest: the chart it sits over, how many words
/// stand there, and the root over the words' addresses.
///
/// Its root is a [`Locator`], so the digest has no `==`
/// (`stratum_digest_has_no_equality`); two digests are read by their
/// charts and by a root that locates the other's:
///
/// ```compile_fail,E0369
/// use mycelium::transducer::sheaf::StratumDigest;
/// fn same(a: &StratumDigest, b: &StratumDigest) -> bool {
///     a == b
/// }
/// ```
///
/// The named comparison, which compiles:
///
/// ```
/// use mycelium::transducer::sheaf::stratum_root;
/// let members = vec![vec![1u8, 2], vec![3u8]];
/// assert!(stratum_root(&members).locates(&stratum_root(&members)));
/// ```
#[derive(Debug, Clone)]
pub struct StratumDigest {
    pub inv: Invariant,
    /// The stratum's members: the mesh's count, `u64` (3b's rule).
    pub count: u64,
    /// The root over the members, a digest in the address sense
    /// (MU-ANTI-FLATTENING 13, MU-HOSTING §5a.10): it locates the stratum a
    /// holder stands on and never admits a word, so it is typed at its
    /// source and compared only by [`Locator::locates`].
    pub root: Locator,
}

/// THE ROOT OVER A STRATUM'S MEMBERS. Thirty-two bytes, whatever the
/// stratum holds.
///
/// This returned the sorted addresses CONCATENATED — the whole population
/// of the stratum, not a root over it — and `digests` returns one per
/// stratum, so a digest exchange carried the entire ruled set. Every court
/// ships this to every peer on every exchange cycle, which is O(P² · C) on
/// the wire before a single word is asked for. A root is what a peer needs
/// to decide whether it disagrees; the members are what it asks for only
/// once it does.
///
/// The bytes ARE the object here, as they are for a receipt chain: what is
/// being identified is a SET, and a set has no canonical coordinate of its
/// own to be written out (law 2 is about a word's identity, and no word's
/// identity passes through this). `payload_hash` is the same construction
/// `measure_ruled_root` already folds these roots with.
pub fn stratum_root(members: &[Vec<u8>]) -> Locator {
    let mut rows = members.to_vec();
    rows.sort();
    let mut buf = Vec::new();
    for a in &rows {
        crate::transducer::address::put_key_bytes(&mut buf, a);
    }
    Locator::of_bytes(crate::chronometry::payload_hash(&buf))
}

/// Measure the section digests of a language over its atlas. The atlas
/// holds addresses, so the root is over the members themselves: no word is
/// read to measure the cover.
pub fn digests(stalks: &Stalks) -> Vec<StratumDigest> {
    stalks
        .strata()
        .into_iter()
        .map(|(inv, members)| StratumDigest {
            count: members.len() as u64,
            root: stratum_root(&members),
            inv,
        })
        .collect()
}

/// The remote strata this holder disagrees with: absent here, or a root
/// that does not locate this holder's stratum. Everything else is glued.
pub fn discordant<'a>(remote: &'a [StratumDigest], local: &[StratumDigest]) -> Vec<&'a StratumDigest> {
    use crate::network::nerve::Keyed;
    let mut here: Keyed<(Locator, u64)> = Keyed::new();
    for l in local {
        here.insert(l.inv.key(), (l.root, l.count));
    }
    remote
        .iter()
        .filter(|r| match here.get(&r.inv.key()) {
            Some((root, _)) => !root.locates(&r.root),
            None => true,
        })
        .collect()
}

/// The addresses this holder already has in the stratum over `inv`, sent
/// with a fetch so only the lacking words travel.
pub fn have(stalks: &Stalks, inv: &Invariant) -> Vec<crate::transducer::address::Address> {
    stalks.fibre(inv).into_iter().map(crate::transducer::address::Address::from).collect()
}

/// Serve a fetch: the words in the stratum over `inv` whose address the
/// asker lacks, as lossless bytes, filling the reply up to `budget` bytes
/// (a stratum wider than one reply converges over successive contacts, as
/// the asker's have-list grows). `at` reads a held word by its address.
pub fn serve<'a>(
    stalks: &Stalks,
    at: impl Fn(&[u8]) -> Option<&'a Element>,
    inv: &Invariant,
    have: &[crate::transducer::address::Address],
    budget: usize,
) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let mut used = 0usize;
    use crate::network::nerve::Keyed;
    let mut held: Keyed<()> = Keyed::new();
    for h in have {
        held.insert(h, ());
    }
    for a in stalks.fibre(inv) {
        if held.contains(&a) {
            continue;
        }
        let Some(w) = at(&a) else { continue };
        // A word whose record does not write is not served (STD1 3c).
        let Some(bytes) = w.to_bytes() else { continue };
        if used + bytes.len() + 4 > budget {
            break;
        }
        used += bytes.len() + 4;
        out.push(bytes);
    }
    out
}

/// The atlas of a bare word list (an entity's genome has no kernel).
pub fn atlas(words: &[Element]) -> Stalks {
    Stalks::over_elements(words.iter())
}

/// The invariant of one word (the chart it sits over).
pub fn chart(word: &Element) -> Invariant {
    invariant_of(word)
}
