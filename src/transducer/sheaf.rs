// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 13, 26 — THE SHEAF OF THE LANGUAGE over the atlas: each holder measures a
//! section digest per stratum of its own words. Strata whose roots agree glue;
//! a discordant one names exactly the fibre whose sections differ.

use super::element::Element;
use super::stalk::{invariant_of, Invariant, Stalks};

/// One stratum's section digest: the chart it sits over, how many words
/// stand there, and the root over their addresses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StratumDigest {
    pub inv: Invariant,
    /// The stratum's members: the mesh's count, `u64` (3b's rule).
    pub count: u64,
    pub root: Vec<u8>,
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
pub fn stratum_root(members: &[Vec<u8>]) -> Vec<u8> {
    let mut rows = members.to_vec();
    rows.sort();
    let mut buf = Vec::new();
    for a in &rows {
        crate::transducer::address::put_key_bytes(&mut buf, a);
    }
    crate::chronometry::payload_hash(&buf).to_vec()
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

/// The remote strata this holder disagrees with: absent here, or a
/// different root. Everything else is glued.
pub fn discordant<'a>(remote: &'a [StratumDigest], local: &[StratumDigest]) -> Vec<&'a StratumDigest> {
    use crate::network::nerve::Keyed;
    let mut here: Keyed<(Vec<u8>, u64)> = Keyed::new();
    for l in local {
        here.insert(l.inv.key(), (l.root.clone(), l.count));
    }
    remote
        .iter()
        .filter(|r| match here.get(&r.inv.key()) {
            Some((root, _)) => *root != r.root,
            None => true,
        })
        .collect()
}

/// The addresses this holder already has in the stratum over `inv`, sent
/// with a fetch so only the lacking words travel.
pub fn have(stalks: &Stalks, inv: &Invariant) -> Vec<crate::transducer::address::Address> {
    stalks.fiber(inv).into_iter().map(crate::transducer::address::Address::from).collect()
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
    for a in stalks.fiber(inv) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::from_canonical;
    use crate::transducer::germinate::germinate;

    fn word(painted_a: &[(i64, i64, u8)], painted_b: &[(i64, i64, u8)]) -> Element {
        from_canonical(&germinate(painted_a, painted_b).unwrap())
    }

    /// A ROOT IS NOT ITS POPULATION. This returned the stratum's sorted
    /// addresses concatenated, and `digests` returns one per stratum, so
    /// every exchange cycle shipped the whole ruled set to every peer —
    /// O(P²·C) on the wire before a word was asked for. Thirty-two bytes
    /// whatever the stratum holds, over the SET and not the order, and it
    /// separates: one member added moves it, which is what discordance
    /// reads.
    #[test]
    fn stratum_root_is_fixed_width_over_set() {
        let small: Vec<Vec<u8>> = (0..2u16).map(|i| i.to_le_bytes().to_vec()).collect();
        let large: Vec<Vec<u8>> = (0..2000u16).map(|i| i.to_le_bytes().to_vec()).collect();
        assert_eq!(stratum_root(&small).len(), 32, "a root is thirty-two bytes");
        assert_eq!(
            stratum_root(&large).len(),
            32,
            "and a stratum a thousand times wider is the same thirty-two"
        );
        let mut reversed = large.clone();
        reversed.reverse();
        assert_eq!(
            stratum_root(&large),
            stratum_root(&reversed),
            "the root is over the set; arrival order is not part of it"
        );
        assert_ne!(stratum_root(&small), stratum_root(&large), "and it separates strata");
        let mut plus = small.clone();
        plus.push(vec![9u8; 4]);
        assert_ne!(
            stratum_root(&small),
            stratum_root(&plus),
            "one member added moves the root — this is what discordance reads"
        );
    }

    /// Two holders differing by one word in one stratum disagree on exactly
    /// that stratum; the fetch carries only the lacking word; after learning
    /// it the digests agree everywhere.
    #[test]
    fn discordant_names_stratum_and_serve_closes_it() {
        let bar = word(&[(0, 0, 1), (0, 1, 1)], &[(0, 0, 2), (0, 1, 2)]);
        let bar_hue = word(&[(0, 0, 3), (0, 1, 3)], &[(0, 0, 4), (0, 1, 4)]);
        let square = word(
            &[(0, 0, 1), (0, 1, 1), (1, 0, 1), (1, 1, 1)],
            &[(0, 0, 5), (0, 1, 5), (1, 0, 5), (1, 1, 5)],
        );
        let hypha = vec![bar.clone(), bar_hue.clone(), square.clone()];
        let mut entity = vec![bar.clone(), square.clone()];
        let (hs, es) = (atlas(&hypha), atlas(&entity));
        let (hd, ed) = (digests(&hs), digests(&es));
        let d = discordant(&hd, &ed);
        assert_eq!(d.len(), 1, "one stratum disagrees");
        assert_eq!(d[0].inv, chart(&bar), "the bar's chart");
        assert_eq!(d[0].count, 2);
        let entity_has = have(&es, &d[0].inv);
        assert_eq!(entity_has, vec![bar.address()]);
        let mut at_index: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
        for (i, w) in hypha.iter().enumerate() {
            at_index.insert(w.address(), i);
        }
        let at = |a: &[u8]| at_index.get(a).copied().and_then(|i| hypha.get(i));
        let page = serve(&hs, at, &d[0].inv, &entity_has, 1 << 20);
        assert_eq!(page.len(), 1, "only the lacking word travels");
        let mut pos = 0usize;
        let learned = Element::from_bytes(&page[0], &mut pos).unwrap();
        assert_eq!(learned.address(), bar_hue.address());
        entity.push(learned);
        let es2 = atlas(&entity);
        assert!(discordant(&hd, &digests(&es2)).is_empty(), "glued everywhere");
        // A grown fibre does not change the root: the base is what glues;
        // a section attached on one side travels by teaching, never as
        // discordance.
        let mut grown = hypha.clone();
        let g = crate::transducer::element::transition(bar_hue.sub(), grown[0].sub(), &mut crate::physics::Physics::default().stroke_n(1 << 20)).unwrap();
        grown[0].attach(&bar_hue, &g);
        let gd = digests(&atlas(&grown));
        assert!(discordant(&gd, &digests(&es2)).is_empty(), "the root is over the base");
        // A budget too small for one word serves nothing and never overflows.
        assert!(serve(&hs, at, &d[0].inv, &entity_has, 8).is_empty());
    }
}
