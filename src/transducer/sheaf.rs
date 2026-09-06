// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE SHEAF OF THE LANGUAGE over the atlas — reconciliation by stratum.
//!
//! Two holders of the language (a hypha and an entity, a court and a
//! hypha) do not page through each other's books. Each measures the same
//! open cover — the atlas strata of its own words — and over every stratum
//! a section digest: the root over (address, fibre volume) of the words
//! held there. The digests are exchanged; a stratum whose roots agree is
//! glued and never spoken of; a discordant stratum names exactly the fibre
//! whose sections differ, and only the words the receiver lacks in that
//! stratum are fetched, with their sections and connection riding along.
//! Cost scales with the divergence, never with the language.

use super::element::Element;
use super::stalk::{invariant_of, Invariant, Stalks};

/// One stratum's section digest: the chart it sits over, how many words
/// stand there, and the root over their (address, fibre volume).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StratumDigest {
    pub inv: Invariant,
    pub count: u32,
    pub root: Vec<u8>,
}

/// The root over a stratum's members: sorted addresses concatenated.
fn root_of(members: &[Vec<u8>]) -> Vec<u8> {
    let mut rows = members.to_vec();
    rows.sort();
    let mut buf = Vec::new();
    for a in &rows {
        crate::transducer::a21::put_bytes(&mut buf, a);
    }
    buf
}

/// Measure the section digests of a language over its atlas. The atlas
/// holds addresses, so the root is over the members themselves: no word is
/// read to measure the cover.
pub fn digests(stalks: &Stalks) -> Vec<StratumDigest> {
    stalks
        .strata()
        .into_iter()
        .map(|(inv, members)| StratumDigest { count: members.len() as u32, root: root_of(&members), inv })
        .collect()
}

/// The remote strata this holder disagrees with: absent here, or a
/// different root. Everything else is glued.
pub fn discordant<'a>(remote: &'a [StratumDigest], local: &[StratumDigest]) -> Vec<&'a StratumDigest> {
    use crate::network::nerve::Keyed;
    let mut here: Keyed<(Vec<u8>, u32)> = Keyed::new();
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
pub fn have(stalks: &Stalks, inv: &Invariant) -> Vec<crate::transducer::a21::Address> {
    stalks.fiber(inv).into_iter().map(crate::transducer::a21::Address::from).collect()
}

/// Serve a fetch: the words in the stratum over `inv` whose address the
/// asker lacks, as lossless bytes, filling the reply up to `budget` bytes
/// (a stratum wider than one reply converges over successive contacts, as
/// the asker's have-list grows). `at` reads a held word by its address.
pub fn serve<'a>(
    stalks: &Stalks,
    at: impl Fn(&[u8]) -> Option<&'a Element>,
    inv: &Invariant,
    have: &[crate::transducer::a21::Address],
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
        let bytes = w.to_bytes();
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
    use crate::transducer::element::from_residual;
    use crate::transducer::germinate::germinate;

    fn word(cells_a: &[(i64, i64, u8)], cells_b: &[(i64, i64, u8)]) -> Element {
        from_residual(&germinate(cells_a, cells_b).unwrap())
    }

    /// Two holders differing by one word in one stratum disagree on exactly
    /// that stratum; the fetch carries only the lacking word; after learning
    /// it the digests agree everywhere.
    #[test]
    fn discordance_names_the_stratum_and_the_fetch_closes_it() {
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
        let mine = have(&es, &d[0].inv);
        assert_eq!(mine, vec![bar.address()]);
        let mut at_index: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
        for (i, w) in hypha.iter().enumerate() {
            at_index.insert(w.address(), i);
        }
        let at = |a: &[u8]| at_index.get(a).copied().and_then(|i| hypha.get(i));
        let page = serve(&hs, at, &d[0].inv, &mine, 1 << 20);
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
        let g = crate::transducer::element::transition(bar_hue.sub(), grown[0].sub()).unwrap();
        grown[0].attach(&bar_hue, &g);
        let gd = digests(&atlas(&grown));
        assert!(discordant(&gd, &digests(&es2)).is_empty(), "the root is over the base");
        // A budget too small for one word serves nothing and never overflows.
        assert!(serve(&hs, at, &d[0].inv, &mine, 8).is_empty());
    }
}
