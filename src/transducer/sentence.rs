// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE SENTENCE — one algebra, two directions.
//!
//! Speaking forms a sentence from geometry: cited words, each carried by a
//! versor and read at a gauge coordinate, and the residual no word speaks.
//! Comprehending runs the same algebra the other way. A received chain is
//! deconstructed into the words that compose it: each body reads its
//! moduli, the stratum over those coordinates names the word, the transport
//! is the quotient of the two bodies' moments, the gauge coordinate is the
//! assignment whose transition profile is the body's own, and the residual
//! is what remains once the transported word is carried onto the body —
//! exactly, on the sensor's own cells. A received sentence is verified by
//! replay: the citations are applied and what remains is measured.
//!
//! Nothing here enumerates the language. A stratum names one word when the
//! moduli are fine enough; when it holds several, that multiplicity is
//! returned as the measurement that the coordinates want an axis. Entity,
//! hypha, and court run these same functions; they differ only in the
//! direction they speak.

use super::blade::Shape;
use super::element::{self, Element, Versor};
use super::stalk::{self, Stalks};
use crate::network::nerve::Keyed;

/// A chain cell: (row, col, hue before, hue after).
pub type Cell = (i64, i64, u8, u8);

/// A CITATION: the coordinates a sentence speaks for one word — its index
/// in the language, the transport carrying the word's chain into the
/// utterance's frame, and the gauge coordinate (which assignment along the
/// punctured axis). A proof is a sequence of citations; a replay applies
/// them and measures what remains.
pub type Cite = (usize, Versor, usize);

/// What deconstruction produced: the citations in body order, the
/// connection between consecutive cited bodies (the relative transport
/// `Vᵢ⁻¹ Vⱼ`), the residual no word speaks, and the widest stratum read.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Sentence {
    pub cites: Vec<Cite>,
    /// PUNCTURES: bodies a held word carries by shape under a gauge the
    /// word does not yet hold — (word, versor, the word's cells under the
    /// body's own transitions). Spoken as sections of the word, not as new
    /// words: the fiber deepens instead of the vocabulary widening.
    pub punctures: Vec<(usize, Versor, Vec<Cell>)>,
    pub connection: Vec<Versor>,
    pub residual: Vec<Cell>,
    /// 1 when every stratum read named one word; more is the measurement
    /// that the moduli want another axis (never a list to test through).
    pub multiplicity: usize,
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
    words: &mut [std::sync::Arc<Element>],
    index_of: impl Fn(&[u8; 32]) -> Option<usize>,
    cites: &[crate::anastome::dialogue::CiteWire],
) -> usize {
    let mut deepened = 0usize;
    for (addr, _, _, body) in cites {
        let Some(i) = index_of(addr) else { continue };
        let mut pos = element::positions(words[i].sub());
        pos.sort_unstable();
        let mut carried = body.clone();
        carried.sort_unstable();
        if element::positions(&carried) != pos {
            continue; // not this word's support: a citation, not a puncture
        }
        let asg = element::assignment(&carried);
        // Keyed membership on the gauge axis: only the assignments with
        // this charge are compared, never the whole list.
        let same_charge = words[i].assignments_by_profile(&element::gauge_key(&asg));
        let held = words[i].assignments();
        if same_charge.iter().any(|&k| held.get(k) == Some(&asg)) {
            continue;
        }
        if std::sync::Arc::make_mut(&mut words[i]).puncture(asg) {
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
    // Group by component root: one ordering, consecutive runs are the
    // pieces — no bucket is searched for.
    let mut keyed: Vec<(usize, Cell)> = cells.into_iter().enumerate().map(|(i, c)| (owner[i], c)).collect();
    keyed.sort_unstable();
    let mut out: Vec<Vec<Cell>> = Vec::new();
    let mut last: Option<usize> = None;
    for (root, cell) in keyed {
        if last != Some(root) {
            out.push(Vec::new());
            last = Some(root);
        }
        out.last_mut().unwrap().push(cell);
    }
    out
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
        let (ri, rj) = (super::spectral::find(&mut parent, i), super::spectral::find(&mut parent, j));
        if ri != rj {
            parent[ri] = rj;
        }
    }
    (0..n).map(|i| super::spectral::find(&mut parent, i)).collect()
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
        .filter_map(|(i, v, asg)| Some((v.clone(), cited_pattern(word_at(*i)?, *asg)?)))
        .collect();
    replay_operators(&ops, target)
}

/// THE REPLAY OF OPERATORS: each operator is a body (the word's cells in
/// its own frame) with the versor that carries it; nothing is fetched. The
/// utterance is verified as it was spoken.
pub fn replay_operators(ops: &[(Versor, Vec<Cell>)], target: &[Cell]) -> Vec<Cell> {
    let mut sorted_target = target.to_vec();
    sorted_target.sort_unstable();
    let mut spoken = vec![false; sorted_target.len()];
    let mut over: Vec<Cell> = Vec::new();
    // APPLY g, never hunt the scene: each operator carries its own cells
    // forward (each cell as a blade, exact under dilation), and every image
    // cell is one keyed read of the target. A hit with the same transition
    // is spoken; an image cell the target lacks is over-explained. Linear
    // in the proof, never the scene times the word.
    for (v, pat) in ops {
        for &(wr, wc, wb, wa) in pat {
            let Some(image) = element::carry_cell(v, wr, wc) else { continue };
            for (tr, tc) in image {
                match sorted_target.binary_search_by(|x| (x.0, x.1).cmp(&(tr, tc))) {
                    Ok(k) => {
                        if (sorted_target[k].2, sorted_target[k].3) == (wb, wa) {
                            spoken[k] = true;
                        }
                    }
                    Err(_) => over.push((tr, tc, wb, wa)),
                }
            }
        }
    }
    let mut residual: Vec<Cell> =
        sorted_target.iter().zip(&spoken).filter(|(_, s)| !**s).map(|(t, _)| *t).collect();
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
pub fn gauge_of(word: &Element, v: &Versor, target: &[Cell]) -> Option<usize> {
    let mut sorted_target = target.to_vec();
    sorted_target.sort_unstable();
    // The charge the target speaks where the word lands: the word's cells
    // carried forward by V, each a keyed read of the target. That charge's
    // key names the word's candidate assignments in one read; each is
    // confirmed by one forward replay. 0 / 1 / many is the measurement.
    let pos = element::positions(word.sub());
    let landed: Vec<(u8, u8)> = pos
        .iter()
        .filter_map(|&(wr, wc)| {
            let p = element::carry_center(v, wr, wc)?;
            let (tr, tc) = element::cell_of(&p);
            let k = sorted_target.binary_search_by(|x| (x.0, x.1).cmp(&(tr, tc))).ok()?;
            Some((sorted_target[k].2, sorted_target[k].3))
        })
        .collect();
    if landed.is_empty() {
        return None;
    }
    let held = word.assignments();
    word.assignments_by_profile(&element::gauge_key(&landed)).into_iter().find(|&k| {
        let Some(assignment) = held.get(k) else { return false };
        pos.iter().zip(assignment).all(|(&(wr, wc), &(wb, wa))| {
            let Some(p) = element::carry_center(v, wr, wc) else { return true };
            let (tr, tc) = element::cell_of(&p);
            match sorted_target.binary_search_by(|x| (x.0, x.1).cmp(&(tr, tc))) {
                Ok(j) => (sorted_target[j].2, sorted_target[j].3) == (wb, wa),
                Err(_) => true,
            }
        })
    })
}

/// THE SEAM CONDITION — the cocycle on overlaps. Where two citations carry
/// their words onto the same target cell, they must speak the same
/// transition; the cells where they disagree are the seam defect, located
/// exactly. A sentence with no overlap has no seam.
pub fn seam_defect(language: &[Element], cites: &[Cite], target: &[Cell]) -> Vec<Cell> {
    let ops: Vec<(Versor, Vec<Cell>)> = cites
        .iter()
        .filter_map(|(i, v, asg)| Some((v.clone(), cited_pattern(language.get(*i)?, *asg)?)))
        .collect();
    seam_defect_operators(&ops, target)
}

pub fn seam_defect_operators(ops: &[(Versor, Vec<Cell>)], target: &[Cell]) -> Vec<Cell> {
    let mut sorted_target = target.to_vec();
    sorted_target.sort_unstable();
    let mut spoken: Vec<Vec<u16>> = vec![Vec::new(); sorted_target.len()];
    for (v, pat) in ops {
        for &(wr, wc, wb, wa) in pat {
            let Some(p) = element::carry_center(v, wr, wc) else { continue };
            let (tr, tc) = element::cell_of(&p);
            if let Ok(k) = sorted_target.binary_search_by(|x| (x.0, x.1).cmp(&(tr, tc))) {
                let pair = ((wb as u16) << 8) | wa as u16;
                if !spoken[k].iter().any(|&p| p == pair) {
                    spoken[k].push(pair);
                }
            }
        }
    }
    sorted_target.iter().zip(&spoken).filter(|(_, s)| s.len() > 1).map(|(t, _)| *t).collect()
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
    let ops: Vec<(Versor, Vec<Cell>)> = cites.iter().map(|(_, v, _, body)| (v.clone(), body.clone())).collect();
    let mut out = replay_operators(&ops, target);
    out.extend(seam_defect_operators(&ops, target));
    out.sort_unstable();
    out.dedup();
    out
}

/// THE HOLONOMY of a cycle of transports: their product around the loop.
pub fn holonomy(cycle: &[Versor]) -> Versor {
    cycle.iter().fold(element::identity_versor(), |acc, v| acc.geo(v))
}

/// A connection is flat around a cycle when its holonomy is the identity
/// up to the projective scale — the loop closes with no stress.
pub fn is_flat(cycle: &[Versor]) -> bool {
    element::same_versor(&holonomy(cycle), &element::identity_versor())
}

/// LOCATE one body: read its moduli, take the stratum, and for the word
/// there whose gauge charge is the body's, solve the transport from the
/// moments and verify it by replay. Returns the citation and the stratum's
/// multiplicity (the measurement, not a search width).
pub fn locate<'a>(
    words: &'a [Element],
    atlas: &Stalks,
    index_of: &impl Fn(&[u8; 32]) -> Option<usize>,
    shape_at: &impl Fn(usize) -> Option<&'a Shape>,
    piece: &[Cell],
    exclude: Option<usize>,
) -> (Option<Cite>, usize) {
    let (cite, _, m) = locate_or_puncture_at(&|i| words.get(i), atlas, index_of, shape_at, piece, exclude);
    (cite, m)
}

/// LOCATE, and when a word carries the body by shape under a gauge it does
/// not yet hold, the PUNCTURE: the word's cells under the body's own
/// transitions pulled back through the transport, verified by replay. A
/// citation is preferred; a puncture is returned only when no word speaks
/// the body as held.
pub fn locate_or_puncture<'a>(
    words: &'a [Element],
    atlas: &Stalks,
    index_of: &impl Fn(&[u8; 32]) -> Option<usize>,
    shape_at: &impl Fn(usize) -> Option<&'a Shape>,
    piece: &[Cell],
    exclude: Option<usize>,
) -> (Option<Cite>, Option<(usize, Versor, Vec<Cell>)>, usize) {
    locate_or_puncture_at(&|i| words.get(i), atlas, index_of, shape_at, piece, exclude)
}

pub fn locate_or_puncture_at<'a>(
    word_at: &impl Fn(usize) -> Option<&'a Element>,
    atlas: &Stalks,
    index_of: &impl Fn(&[u8; 32]) -> Option<usize>,
    shape_at: &impl Fn(usize) -> Option<&'a Shape>,
    piece: &[Cell],
    exclude: Option<usize>,
) -> (Option<Cite>, Option<(usize, Versor, Vec<Cell>)>, usize) {
    let Some(shape) = element::shape_of_chain(piece) else { return (None, None, 0) };
    let want = profile(&element::assignment(piece));
    let stratum: Vec<usize> = atlas.fiber(&stalk::invariant(piece)).iter().filter_map(|a| index_of(a)).collect();
    let key = element::gauge_key(&element::assignment(piece));
    let stratum: Vec<usize> = stratum
        .into_iter()
        .filter(|&i| Some(i) != exclude)
        .filter(|&i| word_at(i).is_some_and(|w| !w.assignments_by_profile(&key).is_empty()))
        .collect();
    let m = stratum.len();
    let [i] = stratum.as_slice() else { return (None, None, m) };
    let i = *i;
    let Some(word) = word_at(i) else { return (None, None, m) };
    let Some(held) = shape_at(i) else { return (None, None, m) };
    let Some(to_word) = super::blade::versor_between(&shape, held) else { return (None, None, m) };
    let Some(v) = to_word.versor_inverse() else { return (None, None, m) };
    let _ = &want;
    let gauge: Vec<usize> = word.assignments_by_profile(&element::gauge_key(&element::assignment(piece)));
    match gauge.as_slice() {
        [asg] => {
            let cite: Cite = (i, v.clone(), *asg);
            if replay_at(word_at, std::slice::from_ref(&cite), piece).is_empty() {
                return (Some(cite), None, m);
            }
        }
        [] => {}
        _ => return (None, None, gauge.len()),
    }
    let pos = element::positions(word.sub());
    if let Some(asg) = element::pullback(&pos, piece, &v) {
        let mut body: Vec<Cell> = pos.iter().zip(&asg).map(|(&(r, c), &(b, a))| (r, c, b, a)).collect();
        body.sort_unstable();
        if replay_operators(&[(v.clone(), body.clone())], piece).is_empty() {
            return (None, Some((i, v, body)), m);
        }
    }
    (None, None, m)
}

/// DECONSTRUCT a chain over a language: the whole chain located first (a
/// word of several bodies), then each body; the connection between
/// consecutive cited bodies is their relative transport; the residual is
/// what no word speaks. `exclude` keeps a word from citing itself.
pub fn deconstruct<'a>(
    words: &'a [Element],
    atlas: &Stalks,
    index_of: &impl Fn(&[u8; 32]) -> Option<usize>,
    shape_at: &impl Fn(usize) -> Option<&'a Shape>,
    chain: &[Cell],
    exclude: Option<usize>,
) -> Sentence {
    let mut out = Sentence::default();
    if chain.is_empty() {
        return out;
    }
    let bodies = pieces(chain);
    if bodies.len() > 1 {
        let (whole, m) = locate(words, atlas, index_of, shape_at, chain, exclude);
        out.multiplicity = m;
        if let Some(c) = whole {
            out.cites.push(c);
            return out;
        }
    }
    for body in bodies {
        let (c, p, m) = locate_or_puncture(words, atlas, index_of, shape_at,&body, exclude);
        out.multiplicity = out.multiplicity.max(m);
        match (c, p) {
            (Some(c), _) => out.cites.push(c),
            (None, Some(p)) => out.punctures.push(p),
            (None, None) => {
                // RECURSION: a body no word carries whole opens along its
                // gauge classes (the cells of one transition), and each
                // class's components is a sub-body located on its own. The
                // recursion ends at a single component of a single class,
                // which is a stalk: cited or residual, never opened further.
                let fibers = element::gauge_fibers(&body);
                if fibers.len() <= 1 {
                    out.residual.extend(body);
                    continue;
                }
                for (_, fiber) in &fibers {
                    for sub in pieces(fiber) {
                        let (c, p, m) = locate_or_puncture(words, atlas, index_of, shape_at,&sub, exclude);
                        out.multiplicity = out.multiplicity.max(m);
                        match (c, p) {
                            (Some(c), _) => out.cites.push(c),
                            (None, Some(p)) => out.punctures.push(p),
                            (None, None) => out.residual.extend(sub),
                        }
                    }
                }
            }
        }
    }
    out.residual.sort_unstable();
    out.connection = out
        .cites
        .windows(2)
        .filter_map(|w| Some(w[0].1.versor_inverse()?.geo(&w[1].1)))
        .collect();
    out
}

/// Deconstruct over a getter, so a language snapshot of `Arc<Element>`
/// never flattens into a cloned slice.
pub fn deconstruct_at<'a>(
    word_at: &impl Fn(usize) -> Option<&'a Element>,
    atlas: &Stalks,
    index_of: &impl Fn(&[u8; 32]) -> Option<usize>,
    shape_at: &impl Fn(usize) -> Option<&'a Shape>,
    chain: &[Cell],
    exclude: Option<usize>,
) -> Sentence {
    let mut out = Sentence::default();
    if chain.is_empty() {
        return out;
    }
    let bodies = pieces(chain);
    if bodies.len() > 1 {
        let (whole, _, m) = locate_or_puncture_at(word_at, atlas, index_of, shape_at, chain, exclude);
        out.multiplicity = m;
        if let Some(c) = whole {
            out.cites.push(c);
            return out;
        }
    }
    for body in bodies {
        let (c, p, m) = locate_or_puncture_at(word_at, atlas, index_of, shape_at, &body, exclude);
        out.multiplicity = out.multiplicity.max(m);
        match (c, p) {
            (Some(c), _) => out.cites.push(c),
            (None, Some(p)) => out.punctures.push(p),
            (None, None) => {
                let fibers = element::gauge_fibers(&body);
                if fibers.len() <= 1 {
                    out.residual.extend(body);
                    continue;
                }
                for (_, fiber) in &fibers {
                    for sub in pieces(fiber) {
                        let (c, p, m) = locate_or_puncture_at(word_at, atlas, index_of, shape_at, &sub, exclude);
                        out.multiplicity = out.multiplicity.max(m);
                        match (c, p) {
                            (Some(c), _) => out.cites.push(c),
                            (None, Some(p)) => out.punctures.push(p),
                            (None, None) => out.residual.extend(sub),
                        }
                    }
                }
            }
        }
    }
    out.residual.sort_unstable();
    out.connection = out
        .cites
        .windows(2)
        .filter_map(|w| Some(w[0].1.versor_inverse()?.geo(&w[1].1)))
        .collect();
    out
}

// ── a sentence is a nerve of germs ──────────────────────────────────────────

use super::alphabet::{self, Word};
use super::element::{affine_of, from_connection, GERM_DIM};

/// One germ of a sentence: a body the family carried, spelled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Germ {
    pub address: [u8; 32],
    pub versor: Versor,
    pub word: Word,
    pub chain: Vec<Cell>,
}

/// A face of the nerve: two germs whose after-supports meet, with the
/// relative word `g_ij = V_j V_i⁻¹` spelled, or a remainder face.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Face {
    pub i: usize,
    pub j: usize,
    pub overlap: Vec<Cell>,
    pub g_ij: Word,
    /// False when the relative transport does not spell: a face with remainder.
    pub spelled: bool,
}

/// An independent loop of the nerve, carrying its holonomy word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cycle {
    pub germs: Vec<usize>,
    pub holonomy: Word,
}

/// A sentence: a nerve of germs inside one thought. Its linguistics is the
/// 1-cochain of relative words on that nerve. Not a list of citations.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Nerve {
    pub germs: Vec<Germ>,
    pub faces: Vec<Face>,
    pub cycles: Vec<Cycle>,
    pub remainder: Vec<Cell>,
}

impl Nerve {
    pub fn is_closed(&self) -> bool {
        self.remainder.is_empty()
            && self.faces.iter().all(|f| f.spelled)
            && self.cycles.iter().all(|c| c.holonomy.abelian_identity())
    }

    pub fn germ_count(&self) -> usize {
        self.germs.len()
    }
}

/// After-support of a germ: the input positions carried by its versor.
pub fn after_support(g: &Germ) -> Vec<(i64, i64)> {
    let mut out = Vec::new();
    for &(r, c, _, _) in &g.chain {
        if let Some(image) = element::carry_cell(&g.versor, r, c) {
            out.extend(image);
        } else {
            out.push((r, c));
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

fn overlap_positions<'a>(a: &'a [(i64, i64)], b: &'a [(i64, i64)]) -> impl Iterator<Item = (i64, i64)> + 'a {
    a.iter().copied().filter(|p| b.binary_search(p).is_ok())
}

fn overlap_cells(a: &[(i64, i64)], b: &[(i64, i64)]) -> Vec<Cell> {
    let mut u: Vec<Cell> = overlap_positions(a, b).map(|(r, c)| (r, c, 0, 0)).collect();
    if u.is_empty() {
        // Shared boundary: cells of a that are adjacent to b.
        u = a
            .iter()
            .copied()
            .filter(|&(r, c)| {
                [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
                    .iter()
                    .any(|p| b.binary_search(p).is_ok())
            })
            .map(|(r, c)| (r, c, 0, 0))
            .collect();
    }
    u.sort_unstable();
    u.dedup();
    u
}

fn spell_versor(v: &Versor) -> Option<Word> {
    let a = affine_of(v)?;
    if a.len() < 2 || a[0].len() < 2 {
        return None;
    }
    let block = [[a[0][0].clone(), a[0][1].clone()], [a[1][0].clone(), a[1][1].clone()]];
    Some(alphabet::spell(&block)?.word)
}

fn face_word(a: &Germ, b: &Germ) -> (Word, bool) {
    let Some(inv) = a.versor.versor_inverse() else {
        return (Word::identity(), false);
    };
    let g = b.versor.geo(&inv);
    match spell_versor(&g) {
        Some(w) => (w, true),
        None => (Word::identity(), false),
    }
}

fn cell_key(r: i64, c: i64) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..8].copy_from_slice(&r.to_le_bytes());
    k[8..16].copy_from_slice(&c.to_le_bytes());
    k
}

fn germ_pair_key(i: usize, j: usize) -> [u8; 32] {
    let (a, b) = if i < j { (i, j) } else { (j, i) };
    let mut k = [0u8; 32];
    k[..8].copy_from_slice(&(a as u64).to_le_bytes());
    k[8..16].copy_from_slice(&(b as u64).to_le_bytes());
    k
}

/// Assemble a nerve from germs: faces from meeting after-supports, cycles
/// from a spanning tree plus chords. Canonical germ order is by address.
pub fn assemble(mut germs: Vec<Germ>, remainder: Vec<Cell>) -> Nerve {
    germs.sort_by(|a, b| a.address.cmp(&b.address));
    let supports: Vec<Vec<(i64, i64)>> = germs.iter().map(after_support).collect();
    let mut at: Keyed<Vec<usize>> = Keyed::new();
    for (i, sup) in supports.iter().enumerate() {
        for &(r, c) in sup {
            let k = cell_key(r, c);
            match at.get_mut(&k) {
                Some(v) => {
                    if !v.contains(&i) {
                        v.push(i);
                    }
                }
                None => {
                    at.insert(k, vec![i]);
                }
            }
        }
    }
    let mut meet: Keyed<()> = Keyed::new();
    at.for_each(|_, germs_here| {
        for a in 0..germs_here.len() {
            for b in a + 1..germs_here.len() {
                meet.insert(germ_pair_key(germs_here[a], germs_here[b]), ());
            }
        }
    });
    for (i, sup) in supports.iter().enumerate() {
        for &(r, c) in sup {
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if let Some(others) = at.get(&cell_key(r + dr, c + dc)) {
                    for &j in others {
                        if j != i {
                            meet.insert(germ_pair_key(i, j), ());
                        }
                    }
                }
            }
        }
    }
    let mut faces = Vec::new();
    let pairs: Vec<(usize, usize)> = meet
        .entries()
        .into_iter()
        .map(|(k, _)| {
            let i = u64::from_le_bytes(k[..8].try_into().unwrap()) as usize;
            let j = u64::from_le_bytes(k[8..16].try_into().unwrap()) as usize;
            (i, j)
        })
        .collect();
    for (i, j) in pairs {
        let overlap = overlap_cells(&supports[i], &supports[j]);
        let (g_ij, spelled) = face_word(&germs[i], &germs[j]);
        faces.push(Face { i, j, overlap, g_ij, spelled });
    }
    let cycles = independent_cycles(germs.len(), &faces);
    Nerve { germs, faces, cycles, remainder }
}

fn independent_cycles(n: usize, faces: &[Face]) -> Vec<Cycle> {
    if n == 0 {
        return Vec::new();
    }
    let mut parent: Vec<usize> = (0..n).collect();
    let find = |parent: &mut [usize], mut x: usize| {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    };
    let mut tree: Vec<bool> = vec![false; faces.len()];
    for (k, f) in faces.iter().enumerate() {
        let (a, b) = (find(&mut parent, f.i), find(&mut parent, f.j));
        if a != b {
            parent[a] = b;
            tree[k] = true;
        }
    }
    let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for (k, f) in faces.iter().enumerate() {
        if tree[k] {
            adj[f.i].push((f.j, k));
            adj[f.j].push((f.i, k));
        }
    }
    let mut by_pair: Keyed<usize> = Keyed::new();
    for (fi, face) in faces.iter().enumerate() {
        by_pair.insert(germ_pair_key(face.i, face.j), fi);
    }
    let mut cycles = Vec::new();
    for (k, f) in faces.iter().enumerate() {
        if tree[k] {
            continue;
        }
        // Path f.i ↝ f.j on the tree, then the chord.
        let mut prev = vec![None; n];
        let mut q = std::collections::VecDeque::new();
        q.push_back(f.i);
        prev[f.i] = Some(f.i);
        while let Some(u) = q.pop_front() {
            if u == f.j {
                break;
            }
            for &(v, _) in &adj[u] {
                if prev[v].is_none() {
                    prev[v] = Some(u);
                    q.push_back(v);
                }
            }
        }
        if prev[f.j].is_none() {
            continue;
        }
        let mut nodes = vec![f.j];
        let mut cur = f.j;
        while cur != f.i {
            cur = prev[cur].unwrap();
            nodes.push(cur);
        }
        nodes.reverse();
        let mut hol = Word::identity();
        for w in nodes.windows(2) {
            if let Some(&fi) = by_pair.get(&germ_pair_key(w[0], w[1])) {
                let face = &faces[fi];
                hol = if face.i == w[0] {
                    hol.compose(&face.g_ij)
                } else {
                    hol.compose(&face.g_ij.inverse())
                };
            }
        }
        hol = hol.compose(&f.g_ij.inverse());
        cycles.push(Cycle { germs: nodes, holonomy: hol });
    }
    cycles
}

/// Solve δγ = ω: a gauge per germ, integer-linear in letter exponents.
/// On a forest every closed 1-cochain is exact; a chord that does not
/// match the tree path is a withheld class.
pub fn solve_coboundary(n: &Nerve) -> Option<Vec<Word>> {
    let g = n.germs.len();
    if g == 0 {
        return Some(Vec::new());
    }
    let mut parent: Vec<usize> = (0..g).collect();
    let find = |parent: &mut [usize], mut x: usize| {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    };
    let mut tree_faces: Vec<&Face> = Vec::new();
    let mut chords: Vec<&Face> = Vec::new();
    for f in &n.faces {
        let (a, b) = (find(&mut parent, f.i), find(&mut parent, f.j));
        if a != b {
            parent[a] = b;
            tree_faces.push(f);
        } else {
            chords.push(f);
        }
    }
    let mut gamma: Vec<Word> = vec![Word::identity(); g];
    let mut placed = vec![false; g];
    placed[0] = true;
    let mut progressed = true;
    while progressed {
        progressed = false;
        for f in &tree_faces {
            match (placed[f.i], placed[f.j]) {
                (true, false) => {
                    gamma[f.j] = f.g_ij.compose(&gamma[f.i]);
                    placed[f.j] = true;
                    progressed = true;
                }
                (false, true) => {
                    gamma[f.i] = f.g_ij.inverse().compose(&gamma[f.j]);
                    placed[f.i] = true;
                    progressed = true;
                }
                _ => {}
            }
        }
    }
    for f in chords {
        let expect = gamma[f.j].compose(&gamma[f.i].inverse());
        if !expect.same_form(&f.g_ij) {
            return None;
        }
    }
    Some(gamma)
}

/// Apply a gauge: each germ's tape is re-anchored by γ_i, then the nerve
/// is reassembled so faces and cycles are the gauged 1-cochain.
pub fn apply_gauge(n: &mut Nerve, gamma: &[Word]) {
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    for (i, g) in n.germs.iter_mut().enumerate() {
        let Some(w) = gamma.get(i) else { continue };
        if w.abelian_identity() {
            continue;
        }
        g.word = w.compose(&g.word);
        if let Some(v) = element::conformal_of(&alphabet::Spelling { word: g.word.clone(), rho: one.clone() }, &(zero.clone(), zero.clone())) {
            g.versor = v;
        }
    }
    *n = assemble(std::mem::take(&mut n.germs), std::mem::take(&mut n.remainder));
}

/// Close the sentence: trivial 2-cocycle and a gauge that kills the faces.
/// No solution withholds the sentence (caller mints ω onto torsion).
pub fn close(n: &mut Nerve) -> bool {
    if n.cycles.iter().any(|c| !c.holonomy.abelian_identity()) {
        return false;
    }
    let Some(gamma) = solve_coboundary(n) else {
        return false;
    };
    if gamma.iter().any(|w| !w.abelian_identity()) {
        apply_gauge(n, &gamma);
    }
    n.is_closed() || (n.faces.iter().all(|f| f.spelled) && n.cycles.iter().all(|c| c.holonomy.abelian_identity()))
}

fn embed_affine(a: &crate::core::engine::QMatrix) -> crate::core::engine::QMatrix {
    use crate::core::matrix::identity;
    let mut m = identity(GERM_DIM);
    let n = a.len().min(3);
    for r in 0..n {
        for c in 0..a[r].len().min(3) {
            m[r][c] = a[r][c].clone();
        }
    }
    m
}

/// The sentence as one Element: first germ is the base connection; every
/// further germ occupies a germinated axis; faces are transitions; a
/// withheld cycle is torsion; the remainder is `sub`.
pub fn as_element(n: &Nerve) -> Option<Element> {
    if n.germs.is_empty() {
        return None;
    }
    let g0 = &n.germs[0];
    let affine = affine_of(&g0.versor)?;
    let sigma = element::assignment(&g0.chain);
    let carried = element::measure_lanes(&n.remainder);
    let mut e = from_connection(&affine, &sigma, &n.remainder, carried)?;
    for (k, g) in n.germs.iter().enumerate().skip(1) {
        let Some(a) = affine_of(&g.versor) else { continue };
        e.set_fiber(element::FIBER_GERMINATED + (k as u16 - 1), embed_affine(&a));
    }
    for f in &n.faces {
        if !f.spelled {
            continue;
        }
        let asg = element::assignment(&f.overlap);
        if let Some(v) = element::conformal_of(
            &alphabet::Spelling {
                word: f.g_ij.clone(),
                rho: num_rational::BigRational::from_integer(num_bigint::BigInt::from(1)),
            },
            &(num_rational::BigRational::from_integer(num_bigint::BigInt::from(0)), num_rational::BigRational::from_integer(num_bigint::BigInt::from(0))),
        ) {
            e.record_dislocation(asg, v);
        }
    }
    if !n.is_closed() {
        for c in &n.cycles {
            if c.holonomy.abelian_identity() {
                continue;
            }
            if let Some(ell) = c.holonomy.period() {
                let sigma: Vec<(u8, u8)> = c.germs.windows(2).map(|w| (w[0] as u8, w[1] as u8)).collect();
                e.record_torsion(sigma, ell);
            }
        }
    }
    let tagged = encode_sentence_word(n, e.description());
    e.set_description(tagged);
    Some(e)
}

fn encode_sentence_word(n: &Nerve, connection: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(element::SENTENCE_TAG);
    if connection.first() == Some(&element::CONNECTION_TAG) && connection.len() > 1 {
        out.extend_from_slice(&connection[1..]);
    }
    encode_nerve_tail(n, &mut out);
    out
}

fn encode_nerve_tail(n: &Nerve, out: &mut Vec<u8>) {
    out.extend_from_slice(&(n.germs.len() as u32).to_le_bytes());
    for g in &n.germs {
        out.extend_from_slice(&g.address);
        g.versor.to_bytes(out);
        g.word.to_bytes(out);
        out.extend_from_slice(&(g.chain.len() as u32).to_le_bytes());
        for &(r, c, b, a) in &g.chain {
            out.extend_from_slice(&r.to_le_bytes());
            out.extend_from_slice(&c.to_le_bytes());
            out.push(b);
            out.push(a);
        }
    }
    out.extend_from_slice(&(n.faces.len() as u32).to_le_bytes());
    for f in &n.faces {
        out.extend_from_slice(&(f.i as u32).to_le_bytes());
        out.extend_from_slice(&(f.j as u32).to_le_bytes());
        out.extend_from_slice(&(f.overlap.len() as u32).to_le_bytes());
        for &(r, c, b, a) in &f.overlap {
            out.extend_from_slice(&r.to_le_bytes());
            out.extend_from_slice(&c.to_le_bytes());
            out.push(b);
            out.push(a);
        }
        f.g_ij.to_bytes(out);
        out.push(f.spelled as u8);
    }
    out.extend_from_slice(&(n.cycles.len() as u32).to_le_bytes());
    for c in &n.cycles {
        out.extend_from_slice(&(c.germs.len() as u32).to_le_bytes());
        for &i in &c.germs {
            out.extend_from_slice(&(i as u32).to_le_bytes());
        }
        c.holonomy.to_bytes(out);
    }
    out.extend_from_slice(&(n.remainder.len() as u32).to_le_bytes());
    for &(r, c, b, a) in &n.remainder {
        out.extend_from_slice(&r.to_le_bytes());
        out.extend_from_slice(&c.to_le_bytes());
        out.push(b);
        out.push(a);
    }
}

fn take_u32(b: &[u8], pos: &mut usize) -> Option<u32> {
    let v = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
    *pos += 4;
    Some(v)
}

fn take_cell(b: &[u8], pos: &mut usize) -> Option<Cell> {
    let r = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
    *pos += 8;
    let c = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
    *pos += 8;
    let before = *b.get(*pos)?;
    *pos += 1;
    let after = *b.get(*pos)?;
    *pos += 1;
    Some((r, c, before, after))
}

fn decode_nerve_tail(b: &[u8], pos: &mut usize) -> Option<Nerve> {
    let ng = take_u32(b, pos)? as usize;
    let mut germs = Vec::with_capacity(ng.min(1024));
    for _ in 0..ng.min(1024) {
        let address: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        let versor = Versor::from_bytes(b, pos)?;
        let word = Word::from_bytes(b, pos)?;
        let nc = take_u32(b, pos)? as usize;
        let mut chain = Vec::with_capacity(nc.min(1 << 16));
        for _ in 0..nc.min(1 << 16) {
            chain.push(take_cell(b, pos)?);
        }
        germs.push(Germ { address, versor, word, chain });
    }
    let nf = take_u32(b, pos)? as usize;
    let mut faces = Vec::with_capacity(nf.min(1024));
    for _ in 0..nf.min(1024) {
        let i = take_u32(b, pos)? as usize;
        let j = take_u32(b, pos)? as usize;
        let no = take_u32(b, pos)? as usize;
        let mut overlap = Vec::with_capacity(no.min(1 << 16));
        for _ in 0..no.min(1 << 16) {
            overlap.push(take_cell(b, pos)?);
        }
        let g_ij = Word::from_bytes(b, pos)?;
        let spelled = *b.get(*pos)? != 0;
        *pos += 1;
        faces.push(Face { i, j, overlap, g_ij, spelled });
    }
    let ncyc = take_u32(b, pos)? as usize;
    let mut cycles = Vec::with_capacity(ncyc.min(1024));
    for _ in 0..ncyc.min(1024) {
        let nk = take_u32(b, pos)? as usize;
        let mut gs = Vec::with_capacity(nk.min(1024));
        for _ in 0..nk.min(1024) {
            gs.push(take_u32(b, pos)? as usize);
        }
        let holonomy = Word::from_bytes(b, pos)?;
        cycles.push(Cycle { germs: gs, holonomy });
    }
    let nr = take_u32(b, pos).unwrap_or(0) as usize;
    let mut remainder = Vec::with_capacity(nr.min(1 << 16));
    for _ in 0..nr.min(1 << 16) {
        remainder.push(take_cell(b, pos)?);
    }
    Some(Nerve { germs, faces, cycles, remainder })
}

/// A sentence Element: tagged tape, not an opened-axis connection.
pub fn is_sentence(e: &Element) -> bool {
    e.is_sentence()
}

/// Chart of a sentence Element: leading axis 2, then germ/face/cycle
/// counts from the tape, then each germ's linear invariants.
pub fn chart_of(e: &Element) -> crate::transducer::stalk::Invariant {
    let n = from_element(e);
    let (germs, faces, cycles) = n.as_ref().map(|n| (n.germs.len() as u64, n.faces.len() as u64, n.cycles.len() as u64)).unwrap_or((1, 0, 0));
    let mut axes = vec![2u64, germs, faces, cycles];
    if let Some(n) = n {
        let mut extra: Vec<Vec<u64>> = Vec::new();
        for g in &n.germs {
            extra.push(g.word.form().map(|f| {
                let mut row = vec![f.parity as u64];
                row.extend(f.rot.iter().map(|&x| x.unsigned_abs()));
                row
            }).unwrap_or_default());
        }
        extra.sort();
        for row in extra {
            axes.extend(row);
        }
    }
    crate::transducer::stalk::Invariant(axes)
}

/// Read germs, faces and cycles back off a sentence Element. The cochain
/// is the one that was sent, not a re-assembly from empty after-supports.
pub fn from_element(e: &Element) -> Option<Nerve> {
    if !e.is_sentence() {
        return None;
    }
    let b = e.description();
    let mut pos = 1usize;
    let _ = alphabet::Spelling::from_bytes(b, &mut pos)?;
    let _ = alphabet::take_q(b, &mut pos)?;
    let _ = alphabet::take_q(b, &mut pos)?;
    let ns = u16::from_le_bytes(b.get(pos..pos + 2)?.try_into().ok()?) as usize;
    pos += 2;
    pos = pos.checked_add(ns.saturating_mul(2))?;
    let rlen = u32::from_le_bytes(b.get(pos..pos + 4)?.try_into().ok()?) as usize;
    pos += 4;
    pos = pos.checked_add(rlen)?;
    decode_nerve_tail(b, &mut pos)
}

/// Replay a closed sentence onto a test grid: transports compose along a
/// spanning tree of the nerve; cycle words are checked, not assumed.
pub fn speak_along_tree(n: &Nerve, cells: &[(i64, i64, u8)]) -> Vec<(i64, i64, u8)> {
    let mut out = Vec::new();
    for g in &n.germs {
        for &(r, c, h) in cells {
            if let Some(image) = element::carry_cell(&g.versor, r, c) {
                for (i, j) in image {
                    out.push((i, j, h));
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// Blind replay of a sentence: re-derive every germ's block from its tape,
/// every face g_ij from the two germs, δg on every triangle, and require
/// stored torsion to close under `toroid::witness_stored`.
pub fn replay_nerve(e: &Element) -> Option<&'static str> {
    let n = from_element(e)?;
    for g in &n.germs {
        if spell_versor(&g.versor).as_ref() != Some(&g.word) && !g.word.abelian_identity() {
            return Some("sentence germ does not re-derive from its letters");
        }
    }
    for f in &n.faces {
        let (w, spelled) = face_word(&n.germs[f.i], &n.germs[f.j]);
        if spelled != f.spelled {
            return Some("sentence face does not re-derive from its germs");
        }
        if spelled && !w.same_form(&f.g_ij) {
            return Some("sentence face g_ij does not re-derive from its germs");
        }
    }
    for c in &n.cycles {
        if !c.holonomy.abelian_identity() && c.holonomy.period().is_none() {
            return Some("sentence cycle holonomy does not close under repetition");
        }
    }
    for (stored, witness) in super::toroid::witness_stored(e) {
        match witness {
            super::toroid::HolonomicLoopWitness::Closed { period, .. } if period == stored => {}
            super::toroid::HolonomicLoopWitness::Closed { .. } => {
                return Some("sentence torsion period does not re-derive")
            }
            super::toroid::HolonomicLoopWitness::Open { .. } => {
                return Some("sentence torsion cycle does not close")
            }
        }
    }
    None
}

/// Two sentences on the same nerve chart whose cochains differ by a
/// coboundary δγ: the gauge that takes `a` onto `b`.
pub fn coboundary_gauge(a: &Nerve, b: &Nerve) -> Option<Vec<Word>> {
    if a.germs.len() != b.germs.len() || a.faces.len() != b.faces.len() {
        return None;
    }
    let mut gamma = Vec::with_capacity(a.germs.len());
    for (ga, gb) in a.germs.iter().zip(&b.germs) {
        gamma.push(gb.word.compose(&ga.word.inverse()));
    }
    for (fa, fb) in a.faces.iter().zip(&b.faces) {
        if fa.i != fb.i || fa.j != fb.j {
            return None;
        }
        let expect = gamma[fa.j].compose(&fa.g_ij).compose(&gamma[fa.i].inverse());
        if !expect.same_form(&fb.g_ij) {
            return None;
        }
    }
    Some(gamma)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::{from_residual, identity_versor};
    use crate::transducer::germinate::germinate;

    /// A language of a bar and a dot deconstructs a rotated, translated,
    /// and dilated bar through its moduli with a verified transport, speaks
    /// a chain of two bodies as two words with their relative transport,
    /// leaves the unknown body as the residual, and refuses to cite a word
    /// with itself.
    #[test]
    fn a_chain_deconstructs_into_its_words_and_residual() {
        let bar = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
        let dot = from_residual(&germinate(&[(0, 0, 3u8)], &[(0, 0, 4u8)]).unwrap());
        let bar_other = from_residual(&germinate(&[(0, 0, 7u8), (0, 1, 7)], &[(0, 0, 8u8), (0, 1, 8)]).unwrap());
        let words = vec![bar, dot, bar_other];
        let atlas = Stalks::over_elements(words.iter());
        let mut index = crate::network::nerve::Keyed::new();
        for (i, w) in words.iter().enumerate() {
            index.insert(w.address(), i);
        }
        let index_of = |a: &[u8; 32]| index.get(a).copied();
        let shapes: Vec<Option<Shape>> = words.iter().map(|w| element::shape_of_chain(w.sub())).collect();
        let shape_at = |i: usize| shapes.get(i).and_then(|s| s.as_ref());
        // A vertical bar far away: the bar rotated and translated.
        let target = vec![(5, 5, 1, 2), (6, 5, 1, 2)];
        let s = deconstruct(&words, &atlas, &index_of, &shape_at,&target, None);
        assert_eq!(s.cites.len(), 1);
        assert_eq!(s.cites[0].0, 0, "the bar, not the other-hued bar");
        assert!(s.is_closed());
        assert!(replay(&words, &s.cites, &target).is_empty(), "the citation replays to closure");
        assert!(s.multiplicity >= 1);
        // A 2× dilated bar with the other hues: the other bar at ρ = 2.
        let big = vec![(0, 0, 7, 8), (0, 1, 7, 8), (0, 2, 7, 8), (0, 3, 7, 8), (1, 0, 7, 8), (1, 1, 7, 8), (1, 2, 7, 8), (1, 3, 7, 8)];
        let s = deconstruct(&words, &atlas, &index_of, &shape_at,&big, None);
        assert_eq!(s.cites.len(), 1);
        assert_eq!(s.cites[0].0, 2);
        assert!(s.is_closed());
        // Two bodies and an unknown: two citations with one relative
        // transport between them, one residual body.
        let mixed = vec![(0, 0, 1, 2), (0, 1, 1, 2), (4, 4, 3, 4), (9, 9, 5, 6), (9, 10, 5, 6), (10, 9, 5, 6)];
        let s = deconstruct(&words, &atlas, &index_of, &shape_at,&mixed, None);
        assert_eq!(s.cites.len(), 2);
        assert_eq!(s.connection.len(), 1, "one relative transport between two cited bodies");
        assert_eq!(s.residual, vec![(9, 9, 5, 6), (9, 10, 5, 6), (10, 9, 5, 6)]);
        // Excluding the bar itself: a bar chain has no other word of hue 1→2.
        let s = deconstruct(&words, &atlas, &index_of, &shape_at,words[0].sub(), Some(0));
        assert!(s.cites.is_empty() && !s.is_closed(), "no word but itself speaks it");
    }

    /// The receiving direction: two citations carrying different words
    /// onto the same cells are a seam defect the replay alone would miss;
    /// a transport and its inverse are a flat loop, two translations are
    /// not.
    #[test]
    fn seams_and_holonomy_are_located_exactly() {
        let bar = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
        let bar_other = from_residual(&germinate(&[(0, 0, 7u8), (0, 1, 7)], &[(0, 0, 8u8), (0, 1, 8)]).unwrap());
        let words = vec![bar, bar_other];
        let target = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
        let id = element::identity_versor();
        let cites: Vec<Cite> = vec![(0, id.clone(), 0), (1, id.clone(), 0)];
        assert!(replay(&words, &cites[..1], &target).is_empty());
        assert_eq!(seam_defect(&words, &cites, &target), target, "both cells spoken twice, differently");
        assert_eq!(verify(&words, &cites, &target), target);
        assert!(verify(&words, &cites[..1], &target).is_empty(), "one word, no seam");
        let t = element::translator_versor(3, 0);
        assert!(is_flat(&[t.clone(), t.versor_inverse().unwrap()]));
        assert!(!is_flat(&[t.clone(), element::translator_versor(0, 2)]));
        assert!(is_flat(&[]), "the empty loop is flat");
    }

    /// Two adjacent germs form a nerve with a face and no cycle; the
    /// coboundary is exact on a tree.
    #[test]
    fn a_two_germ_nerve_is_a_tree_and_closes() {
        let chain_a = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
        let chain_b = vec![(0, 1, 1, 2), (0, 2, 1, 2)];
        let ga = Germ {
            address: from_residual(&chain_a).address(),
            versor: identity_versor(),
            word: Word::identity(),
            chain: chain_a,
        };
        let gb = Germ {
            address: from_residual(&chain_b).address(),
            versor: identity_versor(),
            word: Word::identity(),
            chain: chain_b,
        };
        let mut n = assemble(vec![ga, gb], Vec::new());
        assert_eq!(n.germs.len(), 2);
        assert!(!n.faces.is_empty(), "adjacent after-supports meet");
        assert!(n.cycles.is_empty(), "a tree has no independent cycle");
        assert!(solve_coboundary(&n).is_some(), "a tree 1-cochain is exact");
        assert!(close(&mut n));
    }

    /// A sentence Element is charted on leading axis 2, disjoint from
    /// residual and connection charts.
    #[test]
    fn a_sentence_chart_leads_with_axis_two() {
        let chain_a = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
        let chain_b = vec![(0, 1, 1, 2), (0, 2, 1, 2)];
        let ga = Germ {
            address: from_residual(&chain_a).address(),
            versor: identity_versor(),
            word: Word::identity(),
            chain: chain_a,
        };
        let gb = Germ {
            address: from_residual(&chain_b).address(),
            versor: identity_versor(),
            word: Word::identity(),
            chain: chain_b,
        };
        let e = as_element(&assemble(vec![ga, gb], Vec::new())).expect("nerve element");
        assert!(e.is_sentence());
        let inv = stalk::invariant_of(&e);
        assert_eq!(inv.0.first().copied(), Some(2), "leading axis 2");
        assert!(inv.0.get(1).copied().unwrap_or(0) >= 2, "germ count");
    }

    /// Faces and cycles ride the tape: from_element does not re-assemble
    /// from empty after-supports.
    #[test]
    fn a_sentence_round_trips_its_nerve() {
        let chain_a = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
        let chain_b = vec![(0, 1, 1, 2), (0, 2, 1, 2)];
        let ga = Germ {
            address: from_residual(&chain_a).address(),
            versor: identity_versor(),
            word: Word::identity(),
            chain: chain_a,
        };
        let gb = Germ {
            address: from_residual(&chain_b).address(),
            versor: identity_versor(),
            word: Word::identity(),
            chain: chain_b,
        };
        let n = assemble(vec![ga, gb], Vec::new());
        let e = as_element(&n).expect("a two-germ nerve is an element");
        assert!(e.is_sentence(), "the tape tag is SENTENCE, not an opened axis");
        let back = from_element(&e).expect("the nerve survives the tape");
        assert_eq!(back.germs.len(), n.germs.len());
        assert_eq!(back.faces.len(), n.faces.len());
        assert_eq!(back.cycles.len(), n.cycles.len());
        assert_eq!(back.germs[0].address, n.germs[0].address);
        assert_eq!(back.germs[1].address, n.germs[1].address);
        for (a, b) in n.faces.iter().zip(&back.faces) {
            assert_eq!((a.i, a.j, a.spelled), (b.i, b.j, b.spelled));
            assert!(a.g_ij.same_form(&b.g_ij));
        }
    }
}
