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
use super::stalk::{invariant, Invariant, Stalks};

/// One stratum's section digest: the chart it sits over, how many words
/// stand there, and the root over their (address, fibre volume).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StratumDigest {
    pub inv: Invariant,
    pub count: u32,
    pub root: [u8; 32],
}

/// The root over a stratum's members: sorted by address so two holders of
/// the same words measure the same root; the fibre volume (sections and
/// connection relations) enters so a bundle that grew reads as changed.
fn root_of(members: &[usize], words: &[Element]) -> [u8; 32] {
    let mut rows: Vec<([u8; 32], u32, u32)> = members
        .iter()
        .filter_map(|&i| words.get(i))
        .map(|w| (w.address(), w.sections().len() as u32, w.transitions().len() as u32))
        .collect();
    rows.sort_unstable();
    let mut h = blake3::Hasher::new();
    for (a, s, t) in &rows {
        h.update(a);
        h.update(&s.to_le_bytes());
        h.update(&t.to_le_bytes());
    }
    *h.finalize().as_bytes()
}

/// Measure the section digests of a language over its atlas.
pub fn digests(stalks: &Stalks, words: &[Element]) -> Vec<StratumDigest> {
    stalks
        .strata()
        .into_iter()
        .map(|(inv, members)| StratumDigest {
            count: members.len() as u32,
            root: root_of(&members, words),
            inv,
        })
        .collect()
}

/// The remote strata this holder disagrees with: absent here, or a
/// different root. Everything else is glued.
pub fn discordant<'a>(remote: &'a [StratumDigest], local: &[StratumDigest]) -> Vec<&'a StratumDigest> {
    remote
        .iter()
        .filter(|r| !local.iter().any(|l| l.inv == r.inv && l.root == r.root))
        .collect()
}

/// The addresses this holder already has in the stratum over `inv`, sent
/// with a fetch so only the lacking words travel.
pub fn have(stalks: &Stalks, words: &[Element], inv: &Invariant) -> Vec<[u8; 32]> {
    stalks.fiber(inv).iter().filter_map(|&i| words.get(i)).map(|w| w.address()).collect()
}

/// Serve a fetch: the words in the stratum over `inv` whose address the
/// asker lacks, as lossless bytes, filling the reply up to `budget` bytes
/// (a stratum wider than one reply converges over successive contacts, as
/// the asker's have-list grows).
pub fn serve(stalks: &Stalks, words: &[Element], inv: &Invariant, have: &[[u8; 32]], budget: usize) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let mut used = 0usize;
    for &i in stalks.fiber(inv) {
        let Some(w) = words.get(i) else { continue };
        if have.contains(&w.address()) {
            continue;
        }
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
    Stalks::over(words.iter().map(|w| w.sub()))
}

/// The invariant of one word (the chart it sits over).
pub fn chart(word: &Element) -> Invariant {
    invariant(word.sub())
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
        let (hd, ed) = (digests(&hs, &hypha), digests(&es, &entity));
        let d = discordant(&hd, &ed);
        assert_eq!(d.len(), 1, "one stratum disagrees");
        assert_eq!(d[0].inv, chart(&bar), "the bar's chart");
        assert_eq!(d[0].count, 2);
        let mine = have(&es, &entity, &d[0].inv);
        assert_eq!(mine, vec![bar.address()]);
        let page = serve(&hs, &hypha, &d[0].inv, &mine, 1 << 20);
        assert_eq!(page.len(), 1, "only the lacking word travels");
        let mut pos = 0usize;
        let learned = Element::from_bytes(&page[0], &mut pos).unwrap();
        assert_eq!(learned.address(), bar_hue.address());
        entity.push(learned);
        let es2 = atlas(&entity);
        assert!(discordant(&hd, &digests(&es2, &entity)).is_empty(), "glued everywhere");
        // A grown fibre changes the root: attaching a section re-opens the stratum.
        let mut grown = hypha.clone();
        let g = crate::transducer::element::transition(bar_hue.sub(), grown[0].sub()).unwrap();
        grown[0].attach(&bar_hue, &g);
        let gd = digests(&atlas(&grown), &grown);
        assert_eq!(discordant(&gd, &digests(&es2, &entity)).len(), 1, "fibre volume is in the root");
        // A budget too small for one word serves nothing and never overflows.
        assert!(serve(&hs, &hypha, &d[0].inv, &mine, 8).is_empty());
    }
}
