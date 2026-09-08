// AI-GENERATED CONTENT — see DISCLAIMER.md.

//! Chain deconstruction, replay, and location.

use crate::network::nerve::{Keyed, ROOT};
use crate::transducer::element::{self, Element, Versor};
use crate::transducer::beam;

fn index_key(i: usize) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..8].copy_from_slice(&(i as u64).to_le_bytes());
    k
}

/// A chain cell: (row, col, hue before, hue after).
pub type Cell = (i64, i64, u8, u8);

/// A CITATION: the coordinates a sentence speaks for one word — its index
/// in the language, the transport carrying the word's chain into the
/// utterance's frame, and the gauge coordinate (which assignment along the
/// punctured axis). A proof is a sequence of citations; a replay applies
/// them and measures what remains.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cite {
    /// Which word of the language is cited — an index into the snapshot the
    /// reading ran over, resolved to an address at commit.
    pub word: usize,
    /// The transport carrying that word's chain into the utterance's frame.
    pub transport: Versor,
    /// The gauge coordinate: which assignment along the punctured axis. Read
    /// off the target, never trusted as an index into a copy of the word.
    pub gauge: usize,
}

/// A PUNCTURE: a body a held word carries by SHAPE under a gauge the word
/// does not yet hold. It is spoken as a section of that word — the fibre
/// deepens instead of the vocabulary widening — so it is not a citation and
/// not a residual.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Puncture {
    /// The word that carries the body.
    pub word: usize,
    /// The transport taking the word's shape onto the body.
    pub transport: Versor,
    /// The word's cells under the BODY's own transitions — what the section
    /// says, in the body's hues and not the word's.
    pub cells: Vec<Cell>,
}

/// What deconstruction produced: the citations in body order, the
/// connection between consecutive cited bodies (the relative transport
/// `Vᵢ⁻¹ Vⱼ`), the residual no word speaks, and the widest stratum read.
#[derive(Debug, Clone, Default)]
pub struct Sentence {
    pub cites: Vec<Cite>,
    /// PUNCTURES: bodies a held word carries by shape under a gauge the
    /// word does not yet hold — (word, versor, the word's cells under the
    /// body's own transitions). Spoken as sections of the word, not as new
    /// words: the fiber deepens instead of the vocabulary widening.
    pub punctures: Vec<Puncture>,
    pub connection: Vec<Versor>,
    pub residual: Vec<Cell>,
    /// Stratum width per body key, never a maximum over bodies.
    pub multiplicity: Keyed<usize>,
}

impl Sentence {
    /// Closure: every body spoken by held words as held, nothing left over
    /// and no fiber to deepen.
    pub fn is_closed(&self) -> bool {
        self.residual.is_empty() && self.punctures.is_empty()
    }

    /// What the language does not yet hold: the residual, together with the
    /// bodies spoken only as punctures — novelty in the fiber is novelty.
    /// Measured as the replay of the citations alone over the chain.
    pub fn open(&self, words: &[Element], chain: &[Cell]) -> Vec<Cell> {
        match self.punctures.is_empty() {
            true => self.residual.clone(),
            false => replay(words, &self.cites, chain),
        }
    }

    pub fn open_at<'a>(&self, word_at: &impl Fn(usize) -> Option<&'a Element>, chain: &[Cell]) -> Vec<Cell> {
        match self.punctures.is_empty() {
            true => self.residual.clone(),
            false => replay_at(word_at, &self.cites, chain),
        }
    }
}

/// PUNCTURE the held words a sentence's carried bodies name: a carried body
/// on a word's own support under transitions the word does not yet hold
/// becomes a section of that word. Returns how many fibers deepened. The
/// receiving side of speech: the hearer's language deepens on what it
/// verified, never on what it was told.
pub fn puncture_from(
    words: &mut Keyed<std::sync::Arc<Element>>,
    cites: &[crate::anastome::dialogue::CiteWire],
) -> usize {
    let mut deepened = 0usize;
    for cite in cites {
        let (addr, body) = (&cite.addr, &cite.body);
        let Some(word) = words.get_mut(addr) else { continue };
        let mut pos = element::positions(word.sub());
        pos.sort_unstable();
        let mut carried = body.clone();
        carried.sort_unstable();
        if element::positions(&carried) != pos {
            continue; // not this word's support: a citation, not a puncture
        }
        let asg = element::assignment(&carried);
        // Keyed membership on the gauge axis: only the assignments with
        // this charge are compared, never the whole list.
        let same_charge = word.assignments_by_profile(&element::gauge_key(&asg));
        let held = word.assignments();
        let mut held_asg: Keyed<()> = Keyed::new();
        for &k in &same_charge {
            if let Some(a) = held.get(k) {
                held_asg.insert(element::assignment_key(a), ());
            }
        }
        if held_asg.contains(&element::assignment_key(&asg)) {
            continue;
        }
        if std::sync::Arc::make_mut(word).puncture(asg) {
            deepened += 1;
        }
    }
    deepened
}

/// The connected components of a chain's support under 4-adjacency, each
/// as a sorted chain. The sensor's segmentation, not a search.
pub fn pieces(chain: &[Cell]) -> Vec<Vec<Cell>> {
    let mut cells: Vec<Cell> = chain.to_vec();
    cells.sort_unstable();
    cells.dedup();
    let owner = components(cells.len(), &incidence(&cells));
    let mut by: Keyed<Vec<Cell>> = Keyed::new();
    for (i, cell) in cells.into_iter().enumerate() {
        let k = index_key(owner[i]);
        match by.get_mut(&k) {
            Some(v) => v.push(cell),
            None => {
                by.insert(k, vec![cell]);
            }
        }
    }
    by.fiber(ROOT).map(|(_, v)| v.clone()).collect()
}

/// THE INCIDENCE of a support: its 1-skeleton, the pairs of cells sharing
/// an edge, read off the support itself and never by looking a coordinate
/// up. In row order two cells are edge-adjacent along a row exactly when
/// they are consecutive in the list and differ by one column; in column
/// order, exactly when consecutive and one row apart. Two linear passes
/// over two orderings of the same support. `cells` must be sorted and
/// deduplicated; the pairs are indices into it.
pub fn incidence(cells: &[Cell]) -> Vec<(usize, usize)> {
    let positions: Vec<(i64, i64)> = cells.iter().map(|c| (c.0, c.1)).collect();
    incidence_of(&positions)
}

/// The incidence of a support given as positions in row order (sorted,
/// deduplicated): the same two linear passes, for any carrier of cells.
pub fn incidence_of(positions: &[(i64, i64)]) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for i in 1..positions.len() {
        let (p, q) = (positions[i - 1], positions[i]);
        if p.0 == q.0 && p.1 + 1 == q.1 {
            edges.push((i - 1, i));
        }
    }
    let mut by_column: Vec<usize> = (0..positions.len()).collect();
    by_column.sort_unstable_by_key(|&i| (positions[i].1, positions[i].0));
    for w in by_column.windows(2) {
        let (p, q) = (positions[w[0]], positions[w[1]]);
        if p.1 == q.1 && p.0 + 1 == q.0 {
            edges.push((w[0], w[1]));
        }
    }
    edges
}

/// The connected components of `n` cells under an incidence: each cell's
/// component root (union-find over the edges).
pub fn components(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut parent: Vec<usize> = (0..n).collect();
    for &(i, j) in edges {
        let (ri, rj) = (beam::find(&mut parent, i), beam::find(&mut parent, j));
        if ri != rj {
            parent[ri] = rj;
        }
    }
    (0..n).map(|i| beam::find(&mut parent, i)).collect()
}

/// The transition profile of an assignment: sorted ((before, after), count).
/// The gauge charge of a body.
pub fn profile(asg: &[(u8, u8)]) -> Vec<((u8, u8), u32)> {
    element::gauge_profile(asg)
}

/// Two profiles are the same gauge charge when they hold the same classes
/// in the same proportions. The profile is already reduced to proportions
/// (a dilation by ρ scales every count by ρ², never the proportions), so
/// the same charge is the same profile.
pub fn same_profile(a: &[((u8, u8), u32)], b: &[((u8, u8), u32)]) -> bool {
    a == b
}

/// The chain a citation names in the word's own frame: the word's
/// assignment along its axis over its support.
pub fn cited_pattern(word: &Element, asg: usize) -> Option<Vec<Cell>> {
    let pos = element::positions(word.sub());
    let assignment = word.assignments().into_iter().nth(asg)?;
    let mut pat: Vec<Cell> = pos.iter().zip(&assignment).map(|((r, c), (b, a))| (*r, *c, *b, *a)).collect();
    pat.sort_unstable();
    Some(pat)
}

/// THE REPLAY — in the algebra and linear in the proof. For every citation
/// the word's own cells are carried forward by V, each as a blade (exact
/// under dilation), and every image cell is one keyed read of the target:
/// a hit with the same transition is spoken, an image cell the target
/// lacks is over-explanation. The scene is never walked. The residual is
/// R = Q △ S: the target cells no citation speaks, plus the over-explained
/// cells. An empty residual is closure; a citation naming nothing held
/// speaks nothing.
pub fn replay(language: &[Element], cites: &[Cite], target: &[Cell]) -> Vec<Cell> {
    replay_at(&|i| language.get(i), cites, target)
}

pub fn replay_at<'a>(word_at: &impl Fn(usize) -> Option<&'a Element>, cites: &[Cite], target: &[Cell]) -> Vec<Cell> {
    let ops: Vec<(Versor, Vec<Cell>)> = cites
        .iter()
        .filter_map(|c| Some((c.transport.clone(), cited_pattern(word_at(c.word)?, c.gauge)?)))
        .collect();
    replay_operators(&ops, target)
}

/// THE REPLAY OF OPERATORS: each operator is a body (the word's cells in
/// its own frame) with the versor that carries it; nothing is fetched. The
/// utterance is verified as it was spoken.
fn cell_at_key(r: i64, c: i64) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..8].copy_from_slice(&r.to_le_bytes());
    k[8..16].copy_from_slice(&c.to_le_bytes());
    k
}

pub fn replay_operators(ops: &[(Versor, Vec<Cell>)], target: &[Cell]) -> Vec<Cell> {
    let mut at: Keyed<Cell> = Keyed::new();
    for &cell in target {
        at.insert(cell_at_key(cell.0, cell.1), cell);
    }
    let mut spoken: Keyed<()> = Keyed::new();
    let mut over: Vec<Cell> = Vec::new();
    for (v, pat) in ops {
        for &(wr, wc, wb, wa) in pat {
            let Some(image) = element::carry_cell(v, wr, wc) else { continue };
            for (tr, tc) in image {
                let k = cell_at_key(tr, tc);
                match at.get(&k) {
                    Some(t) if (t.2, t.3) == (wb, wa) => {
                        spoken.insert(k, ());
                    }
                    Some(_) => {}
                    None => over.push((tr, tc, wb, wa)),
                }
            }
        }
    }
    let mut residual: Vec<Cell> = at
        .fiber(ROOT)
        .filter(|(k, _)| !spoken.contains(k))
        .map(|(_, t)| *t)
        .collect();
    residual.extend(over);
    residual.sort_unstable();
    residual.dedup();
    residual
}

/// THE GAUGE COORDINATE OF A CITATION, read from the target: the word's
/// assignment whose transitions, carried by `v`, agree with the target on
/// every cell they land on. An index into a word's assignments is not a
/// stable coordinate across copies of the word (a copy that gained a
/// section orders them differently); the target's own transitions are.
/// None when no assignment agrees.
pub fn gauge_of(word: &Element, v: &Versor, target: &[Cell]) -> Keyed<usize> {
    let mut at: Keyed<Cell> = Keyed::new();
    for &cell in target {
        at.insert(cell_at_key(cell.0, cell.1), cell);
    }
    let pos = element::positions(word.sub());
    let landed: Vec<(u8, u8)> = pos
        .iter()
        .filter_map(|&(wr, wc)| {
            let p = element::carry_center(v, wr, wc)?;
            let (tr, tc) = element::cell_of(&p);
            let t = at.get(&cell_at_key(tr, tc))?;
            Some((t.2, t.3))
        })
        .collect();
    let mut agreeing = Keyed::new();
    if landed.is_empty() {
        return agreeing;
    }
    let held = word.assignments();
    for k in word.assignments_by_profile(&element::gauge_key(&landed)) {
        let Some(assignment) = held.get(k) else { continue };
        let ok = pos.iter().zip(assignment).all(|(&(wr, wc), &(wb, wa))| {
            let Some(p) = element::carry_center(v, wr, wc) else { return true };
            let (tr, tc) = element::cell_of(&p);
            match at.get(&cell_at_key(tr, tc)) {
                Some(t) => (t.2, t.3) == (wb, wa),
                None => true,
            }
        });
        if ok {
            agreeing.insert(element::assignment_key(assignment), k);
        }
    }
    agreeing
}

/// THE SEAM CONDITION — the cocycle on overlaps. Where two citations carry
/// their words onto the same target cell, they must speak the same
/// transition; the cells where they disagree are the seam defect, located
/// exactly. A sentence with no overlap has no seam.
pub fn seam_defect(language: &[Element], cites: &[Cite], target: &[Cell]) -> Vec<Cell> {
    let ops: Vec<(Versor, Vec<Cell>)> = cites
        .iter()
        .filter_map(|c| Some((c.transport.clone(), cited_pattern(language.get(c.word)?, c.gauge)?)))
        .collect();
    seam_defect_operators(&ops, target)
}

pub fn seam_defect_operators(ops: &[(Versor, Vec<Cell>)], target: &[Cell]) -> Vec<Cell> {
    let mut first: Keyed<(u8, u8)> = Keyed::new();
    let mut conflict: Keyed<()> = Keyed::new();
    let mut at: Keyed<Cell> = Keyed::new();
    for &cell in target {
        at.insert(cell_at_key(cell.0, cell.1), cell);
    }
    for (v, pat) in ops {
        for &(wr, wc, wb, wa) in pat {
            let Some(p) = element::carry_center(v, wr, wc) else { continue };
            let (tr, tc) = element::cell_of(&p);
            let k = cell_at_key(tr, tc);
            if at.get(&k).is_none() {
                continue;
            }
            match first.get(&k) {
                None => {
                    first.insert(k, (wb, wa));
                }
                Some(&(hb, ha)) if (hb, ha) != (wb, wa) => {
                    conflict.insert(k, ());
                }
                _ => {}
            }
        }
    }
    at.fiber(ROOT)
        .filter(|(k, _)| conflict.contains(k))
        .map(|(_, t)| *t)
        .collect()
}

/// VERIFY a received sentence — the receiving direction of the algebra:
/// the replay's residual together with the seam defect. Empty is closure
/// with a coherent cover.
pub fn verify(language: &[Element], cites: &[Cite], target: &[Cell]) -> Vec<Cell> {
    let mut out = replay(language, cites, target);
    out.extend(seam_defect(language, cites, target));
    out.sort_unstable();
    out.dedup();
    out
}

/// VERIFY AN UTTERANCE as spoken: the operators the citations carry are
/// replayed against the target and checked on their seams, with no copy
/// of any word fetched. The hearer's lexicon is consulted only afterward,
/// to name what was verified.
pub fn verify_utterance(cites: &[crate::anastome::dialogue::CiteWire], target: &[Cell]) -> Vec<Cell> {
    let ops: Vec<(Versor, Vec<Cell>)> =
        cites.iter().map(|c| (c.transport.clone(), c.body.clone())).collect();
    let mut out = replay_operators(&ops, target);
    out.extend(seam_defect_operators(&ops, target));
    out.sort_unstable();
    out.dedup();
    out
}

/// THE HOLONOMY of a cycle of transports: their product around the loop.
/// WHAT LOCATING A BODY RETURNS: the citation if the atlas holds the word,
/// the puncture if it holds the geometry under another assignment, and the
/// stratum's multiplicity — the measurement, never a search width (A8).
pub struct Located {
    /// The citation, when a held word speaks the body as held.
    pub cite: Option<Cite>,
    /// The puncture, when one carries it by shape under another gauge. A
    /// citation is preferred; this stands only when none was found.
    pub puncture: Option<Puncture>,
    /// The stratum's MULTIPLICITY — how many words shared the body's
    /// moduli. A measurement of the atlas, never a search width.
    pub multiplicity: usize,
}

pub fn holonomy(cycle: &[Versor]) -> Versor {
    cycle.iter().fold(element::identity_versor(), |acc, v| acc.geo(v))
}

/// A connection is flat around a cycle when its holonomy is the identity
/// up to the projective scale — the loop closes with no stress.
pub fn is_flat(cycle: &[Versor]) -> bool {
    element::same_versor(&holonomy(cycle), &element::identity_versor())
}
