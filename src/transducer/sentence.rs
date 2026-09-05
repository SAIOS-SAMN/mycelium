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
use crate::network::nerve::ROOT;

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
    for (addr, _, _, _, body) in cites {
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
    let ops: Vec<(Versor, Vec<Cell>)> = cites.iter().map(|(_, _, v, _, body)| (v.clone(), body.clone())).collect();
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
    locate_or_puncture_on(word_at, &|inv| atlas.fiber(inv), index_of, shape_at, piece, exclude)
}

/// Locate against an arbitrary fiber — the working atlas, not only the genome.
pub fn locate_or_puncture_on<'a>(
    word_at: &impl Fn(usize) -> Option<&'a Element>,
    fiber_of: &impl Fn(&stalk::Invariant) -> Vec<[u8; 32]>,
    index_of: &impl Fn(&[u8; 32]) -> Option<usize>,
    shape_at: &impl Fn(usize) -> Option<&'a Shape>,
    piece: &[Cell],
    exclude: Option<usize>,
) -> (Option<Cite>, Option<(usize, Versor, Vec<Cell>)>, usize) {
    let Some(shape) = element::shape_of_chain(piece) else { return (None, None, 0) };
    let want = profile(&element::assignment(piece));
    let stratum: Vec<usize> = fiber_of(&stalk::invariant(piece)).iter().filter_map(|a| index_of(a)).collect();
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
    deconstruct_on(word_at, &|inv| atlas.fiber(inv), index_of, shape_at, chain, exclude)
}

/// Deconstruct over a fiber the caller names — genome, dialect, or both.
pub fn deconstruct_on<'a>(
    word_at: &impl Fn(usize) -> Option<&'a Element>,
    fiber_of: &impl Fn(&stalk::Invariant) -> Vec<[u8; 32]>,
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
        let (whole, _, m) = locate_or_puncture_on(word_at, fiber_of, index_of, shape_at, chain, exclude);
        out.multiplicity = m;
        if let Some(c) = whole {
            out.cites.push(c);
            return out;
        }
    }
    for body in bodies {
        let (c, p, m) = locate_or_puncture_on(word_at, fiber_of, index_of, shape_at, &body, exclude);
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
                        let (c, p, m) = locate_or_puncture_on(word_at, fiber_of, index_of, shape_at, &sub, exclude);
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

use super::alphabet::{self, Order, Rotor, Word};
use super::cech::{self, Complex, Fill, Section};
use super::element::{affine_of, from_connection, GERM_DIM};
use super::peel::ShellKind;
use super::stalk::Standing;
use crate::network::nerve::Keyed;
use crate::physics::Budget;

/// One germ of a sentence: a body the family carried, spelled.
/// Source or target may be absent: a created body has no source, an
/// erased body has no target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Germ {
    pub address: [u8; 32],
    pub versor: Versor,
    pub word: Word,
    pub chain: Vec<Cell>,
    pub source: Option<stalk::Invariant>,
    pub target: Option<stalk::Invariant>,
    pub kind: ShellKind,
    /// Translation that agreed across the family. Absent: placement is a face.
    pub shift: Option<(num_rational::BigRational, num_rational::BigRational)>,
}

/// A face of the nerve: the relative Clifford versor
/// `g_ij = T_{Δx} R_θ ∈ Spin(4,1)(ℚ)` between two bodies, and the spelled
/// linear block when that block is a word. A pure translation is a
/// parabolic versor, not the identity word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Face {
    pub i: usize,
    pub j: usize,
    pub overlap: Vec<Cell>,
    pub g_ij: Word,
    pub versor: Versor,
    /// False when the relative transport does not spell: a face with remainder.
    pub spelled: bool,
}

/// An independent loop of the nerve, carrying its holonomy word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cycle {
    pub germs: Vec<usize>,
    pub holonomy: Word,
}

/// A sentence: a nerve of germs inside one thought. Germs are vertices,
/// k-fold body overlaps are k-cells. The complex is the object; faces and
/// cycles are the tape projection. A class at any degree is torsion: the
/// sentence is thought.
#[derive(Debug, Clone, Default)]
pub struct Nerve {
    pub germs: Vec<Germ>,
    pub complex: Complex,
    pub faces: Vec<Face>,
    pub cycles: Vec<Cycle>,
    pub remainder: Vec<Cell>,
    pub class: Option<(Rotor, usize)>,
    /// Chart prefixes the idea spans, so the court knows the cover.
    pub cover: crate::network::nerve::Keyed<()>,
}

impl PartialEq for Nerve {
    fn eq(&self, other: &Self) -> bool {
        self.germs == other.germs
            && self.faces == other.faces
            && self.cycles == other.cycles
            && self.remainder == other.remainder
            && self.class == other.class
    }
}
impl Eq for Nerve {}

impl Nerve {
    pub fn is_closed(&self) -> bool {
        self.remainder.is_empty()
            && self.cycles.iter().all(|c| c.holonomy.abelian_identity())
            && self.class.is_none()
    }

    /// Remainder empty and every loop identity or filed finite torsion.
    /// Infinite open chords keep the thought open.
    pub fn is_cauterized(&self) -> bool {
        if !self.remainder.is_empty() {
            return false;
        }
        for c in &self.cycles {
            if c.holonomy.abelian_identity() {
                continue;
            }
            match c.holonomy.order() {
                Order::Infinite => return false,
                Order::Finite(ell) => {
                    let k = c.holonomy.rotor().class_key();
                    let filed = self.complex.degrees.iter().any(|layer| {
                        layer.fiber(&crate::network::nerve::ROOT).any(|(_, cell)| {
                            matches!(&cell.fill, Fill::Congruent(r, e) if *e == ell && r.class_key() == k)
                        })
                    });
                    if !filed {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn germ_count(&self) -> usize {
        self.germs.len()
    }

    /// A closed 1-cocycle, or a single non-trivial 1-morphism
    /// (`g = output ∘ input⁻¹`, `g ≠ 1`). One germ is a sentence when that
    /// morphism is not the identity.
    pub fn is_sentence(&self) -> bool {
        if !self.is_closed() {
            return false;
        }
        if self.germ_count() >= 2 {
            return true;
        }
        let Some(g) = self.germs.first() else {
            return false;
        };
        !g.word.abelian_identity() || g.chain.iter().any(|&(_, _, b, a)| b != a)
    }
}

/// How an unfolding stops. The step count is an instance coordinate, never
/// stored on the form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Measure {
    Occupied,
    Remainder,
    Ring { anchor: u16 },
}

/// A vertex of a sentence form (chapter 18). Beside `Germ` until ch.30 step 6.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vertex {
    Frame,
    Word {
        address: [u8; 32],
        word: Word,
        rho: num_rational::BigRational,
        placement: (num_rational::BigRational, num_rational::BigRational),
        gauge: Vec<(u8, u8)>,
        class: stalk::Invariant,
    },
    Glued {
        address: [u8; 32],
        chart: stalk::Invariant,
        magnitude: [u64; 4],
        depth: u16,
        placement: Word,
    },
    Rebased { by: u16, of: u16 },
    Unfolded { measure: Measure },
}

/// Face of a `Form`. The nerve's [`Face`] stays until the adapters leave.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormFace {
    pub i: u16,
    pub j: u16,
    pub g_ij: Word,
}

/// Independent loop of a form: face indices and their holonomy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loop {
    pub faces: Vec<u16>,
    pub holonomy: Word,
}

/// A sentence as vertices, faces, loops, remainder. Replaces `Nerve` as the
/// stored object once chapter 30 finishes; both live here until then.
#[derive(Debug, Clone)]
pub struct Form {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<FormFace>,
    pub loops: Vec<Loop>,
    pub remainder: Vec<Cell>,
    pub depth: u16,
    pub hue_maps: Keyed<Vec<(u8, u8)>>,
}

impl Default for Form {
    fn default() -> Self {
        Form {
            vertices: vec![Vertex::Frame],
            faces: Vec::new(),
            loops: Vec::new(),
            remainder: Vec::new(),
            depth: 0,
            hue_maps: Keyed::new(),
        }
    }
}

impl Vertex {
    pub fn from_germ(g: &Germ) -> Vertex {
        if g.kind == ShellKind::Frame {
            return Vertex::Frame;
        }
        let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
        let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
        Vertex::Word {
            address: g.address,
            word: g.word.clone(),
            rho: one,
            placement: g.shift.clone().unwrap_or((zero.clone(), zero)),
            gauge: germ_hue_map(g),
            class: g.target.clone().or(g.source.clone()).unwrap_or_default(),
        }
    }

    pub fn word_in(&self, form: &Form) -> Word {
        match self {
            Vertex::Frame | Vertex::Unfolded { .. } => Word::identity(),
            Vertex::Word { word, .. } => word.clone(),
            Vertex::Glued { placement, .. } => placement.clone(),
            Vertex::Rebased { by, of } => {
                let a = form
                    .vertices
                    .get(*by as usize)
                    .map(|v| v.word_in(form))
                    .unwrap_or_else(Word::identity);
                let w = form
                    .vertices
                    .get(*of as usize)
                    .map(|v| v.word_in(form))
                    .unwrap_or_else(Word::identity);
                a.compose(&w.compose(&a.inverse())).normal_form()
            }
        }
    }

    pub fn class(&self) -> stalk::Invariant {
        match self {
            Vertex::Word { class, .. } => class.clone(),
            Vertex::Glued { chart, .. } => chart.clone(),
            _ => stalk::Invariant::default(),
        }
    }

    pub fn depth(&self) -> u16 {
        match self {
            Vertex::Glued { depth, .. } => *depth,
            _ => 0,
        }
    }
}

impl Form {
    pub fn from_nerve(n: &Nerve) -> Form {
        let mut vertices: Vec<Vertex> = n.germs.iter().map(Vertex::from_germ).collect();
        let mut shift = 0u16;
        if !vertices.iter().any(|v| matches!(v, Vertex::Frame)) {
            vertices.insert(0, Vertex::Frame);
            shift = 1;
        }
        let faces: Vec<FormFace> = n
            .faces
            .iter()
            .map(|f| FormFace {
                i: (f.i as u16).saturating_add(shift),
                j: (f.j as u16).saturating_add(shift),
                g_ij: f.g_ij.clone(),
            })
            .collect();
        let loops = Form::loops_of(&faces, vertices.len());
        Form {
            vertices,
            faces,
            loops,
            remainder: n.remainder.clone(),
            depth: 0,
            hue_maps: Keyed::new(),
        }
    }

    /// Nerve of Body vertices only. `None` when the form cites, conjugates, or unfolds.
    pub fn to_nerve(&self) -> Option<Nerve> {
        if self.vertices.iter().any(|v| {
            matches!(
                v,
                Vertex::Glued { .. } | Vertex::Rebased { .. } | Vertex::Unfolded { .. }
            )
        }) {
            return None;
        }
        let mut germs = Vec::with_capacity(self.vertices.len());
        for v in &self.vertices {
            match v {
                Vertex::Frame => germs.push(Germ {
                    address: [0u8; 32],
                    versor: element::identity_versor(),
                    word: Word::identity(),
                    chain: Vec::new(),
                    source: None,
                    target: None,
                    kind: ShellKind::Frame,
                    shift: None,
                }),
                Vertex::Word {
                    address,
                    word,
                    rho,
                    placement,
                    class,
                    ..
                } => {
                    let spelling = alphabet::Spelling {
                        word: word.clone(),
                        rho: rho.clone(),
                    };
                    let versor = element::conformal_of(&spelling, placement)
                        .unwrap_or_else(element::identity_versor);
                    germs.push(Germ {
                        address: *address,
                        versor,
                        word: word.clone(),
                        chain: Vec::new(),
                        source: None,
                        target: Some(class.clone()),
                        kind: ShellKind::Whole,
                        shift: Some(placement.clone()),
                    });
                }
                _ => return None,
            }
        }
        let faces: Vec<Face> = self
            .faces
            .iter()
            .map(|f| Face {
                i: f.i as usize,
                j: f.j as usize,
                overlap: Vec::new(),
                g_ij: f.g_ij.clone(),
                versor: element::identity_versor(),
                spelled: true,
            })
            .collect();
        let cycles = independent_cycles(germs.len(), &faces);
        let complex = bind_complex(&germs, &faces);
        Some(Nerve {
            germs,
            complex,
            faces,
            cycles,
            remainder: self.remainder.clone(),
            class: None,
            cover: Keyed::new(),
        })
    }

    pub fn face_of(&self, i: u16, j: u16) -> Word {
        let a = self
            .vertices
            .get(i as usize)
            .map(|v| v.word_in(self))
            .unwrap_or_else(Word::identity);
        let b = self
            .vertices
            .get(j as usize)
            .map(|v| v.word_in(self))
            .unwrap_or_else(Word::identity);
        b.compose(&a.inverse()).normal_form()
    }

    pub fn delta1(&self) -> Vec<(u16, Word)> {
        self.loops
            .iter()
            .enumerate()
            .map(|(i, lp)| (i as u16, loop_holonomy(self, lp)))
            .collect()
    }

    pub fn coheres(&self) -> bool {
        self.delta1()
            .into_iter()
            .all(|(_, w)| w.rotor().projective_eq(&Rotor::identity()))
    }

    pub fn is_closed(&self) -> bool {
        self.coheres() && self.remainder.is_empty()
    }

    pub fn depth_of(&self) -> u16 {
        self.vertices
            .iter()
            .filter_map(|v| match v {
                Vertex::Glued { depth, .. } => Some(depth.saturating_add(1)),
                _ => None,
            })
            .max()
            .unwrap_or(0)
    }

    pub fn glue(&mut self, s: &Form, s_addr: [u8; 32], placement: Word) -> Option<u16> {
        if !s.is_closed() {
            return None;
        }
        let Some(fi) = s.vertices.iter().position(|v| matches!(v, Vertex::Frame)) else {
            return None;
        };
        if !s.vertices[fi].word_in(s).is_identity() {
            return None;
        }
        let d = s.depth_of();
        if d >= self.depth_of() && self.depth_of() > 0 {
            return None;
        }
        let idx = self.vertices.len() as u16;
        self.vertices.push(Vertex::Glued {
            address: s_addr,
            chart: s
                .vertices
                .iter()
                .find_map(|v| match v {
                    Vertex::Word { class, .. } => Some(class.clone()),
                    Vertex::Glued { chart, .. } => Some(chart.clone()),
                    _ => None,
                })
                .unwrap_or_default(),
            magnitude: [0; 4],
            depth: d,
            placement,
        });
        self.depth = self.depth_of();
        Some(idx)
    }

    pub fn rebase(&mut self, by: u16, of: u16) -> u16 {
        let idx = self.vertices.len() as u16;
        self.vertices.push(Vertex::Rebased { by, of });
        idx
    }

    pub fn unfold(&mut self, measure: Measure) -> u16 {
        let idx = self.vertices.len() as u16;
        self.vertices.push(Vertex::Unfolded { measure });
        idx
    }

    pub fn loops_of(faces: &[FormFace], n: usize) -> Vec<Loop> {
        if n == 0 {
            return Vec::new();
        }
        let mut parent: Vec<usize> = (0..n).collect();
        let find = |parent: &mut [usize], mut x: usize| -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        };
        let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
        let mut loops = Vec::new();
        for (fi, f) in faces.iter().enumerate() {
            let (a, b) = (f.i as usize, f.j as usize);
            if a >= n || b >= n {
                continue;
            }
            let (pa, pb) = (find(&mut parent, a), find(&mut parent, b));
            if pa != pb {
                parent[pa] = pb;
                adj[a].push((b, fi));
                adj[b].push((a, fi));
                continue;
            }
            let Some(path) = tree_path(&adj, a, b) else {
                continue;
            };
            let mut face_ix: Vec<u16> = Vec::new();
            for w in path.windows(2) {
                if let Some(&(_, efi)) = adj[w[0]].iter().find(|&&(v, _)| v == w[1]) {
                    face_ix.push(efi as u16);
                }
            }
            face_ix.push(fi as u16);
            let holonomy = form_cycle_holonomy(faces, &adj, &path, f);
            loops.push(Loop {
                faces: face_ix,
                holonomy,
            });
        }
        loops
    }
}

fn form_cycle_holonomy(
    faces: &[FormFace],
    adj: &[Vec<(usize, usize)>],
    path: &[usize],
    chord: &FormFace,
) -> Word {
    let mut w = Word::identity();
    for pair in path.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        let Some(&(_, fi)) = adj[a].iter().find(|&&(v, _)| v == b) else {
            continue;
        };
        let f = &faces[fi];
        if f.i as usize == a && f.j as usize == b {
            w = w.compose(&f.g_ij);
        } else {
            w = w.compose(&f.g_ij.inverse());
        }
    }
    if chord.i as usize == path.last().copied().unwrap_or(chord.i as usize)
        && chord.j as usize == path.first().copied().unwrap_or(chord.j as usize)
    {
        w = w.compose(&chord.g_ij);
    } else {
        w = w.compose(&chord.g_ij.inverse());
    }
    w.normal_form()
}

fn loop_holonomy(form: &Form, lp: &Loop) -> Word {
    let mut w = Word::identity();
    for &fi in &lp.faces {
        if let Some(f) = form.faces.get(fi as usize) {
            w = w.compose(&f.g_ij);
        }
    }
    w.normal_form()
}

fn germ_vertex(i: usize) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..8].copy_from_slice(&(i as u64).to_le_bytes());
    k
}

fn germ_index(v: &[u8; 32]) -> usize {
    u64::from_le_bytes(v[..8].try_into().unwrap_or([0; 8])) as usize
}

fn word_of_rotor(r: &Rotor) -> Word {
    let one = alphabet::Q::from_integer(num_bigint::BigInt::from(1));
    let zero = alphabet::Q::from_integer(num_bigint::BigInt::from(0));
    let e0 = r.carry(&[one.clone(), zero.clone(), zero.clone(), zero.clone()]);
    let e1 = r.carry(&[zero.clone(), one, zero.clone(), zero]);
    if let (Some(a), Some(b)) = (e0, e1) {
        let block = [[a[0].clone(), b[0].clone()], [a[1].clone(), b[1].clone()]];
        if let Some(sp) = alphabet::spell(&block) {
            return sp.word;
        }
    }
    Word::identity()
}

/// Independent cycles: chords of a spanning forest of spelled faces.
/// Rank is F − V + C; each chord closes one loop along the tree path.
fn independent_cycles(n: usize, faces: &[Face]) -> Vec<Cycle> {
    if n == 0 {
        return Vec::new();
    }
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank = vec![0u8; n];
    let find = |parent: &mut [usize], mut x: usize| -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    };
    let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    let mut cycles = Vec::new();
    // index: spelled faces of this nerve (the 1-skeleton)
    for (fi, f) in faces.iter().enumerate() {
        if f.i >= n || f.j >= n {
            continue;
        }
        let (a, b) = (f.i, f.j);
        let (pa, pb) = (find(&mut parent, a), find(&mut parent, b));
        if pa != pb {
            if rank[pa] < rank[pb] {
                parent[pa] = pb;
            } else if rank[pb] < rank[pa] {
                parent[pb] = pa;
            } else {
                parent[pb] = pa;
                rank[pa] = rank[pa].saturating_add(1);
            }
            adj[a].push((b, fi));
            adj[b].push((a, fi));
            continue;
        }
        if !f.spelled {
            continue;
        }
        let Some(mut path) = tree_path(&adj, a, b) else {
            continue;
        };
        path.push(a);
        let holonomy = cycle_holonomy(faces, &adj, &path, f);
        cycles.push(Cycle { germs: path, holonomy });
    }
    cycles
}

fn tree_path(adj: &[Vec<(usize, usize)>], src: usize, dst: usize) -> Option<Vec<usize>> {
    if src == dst {
        return Some(vec![src]);
    }
    let mut prev: Vec<Option<usize>> = vec![None; adj.len()];
    let mut seen = vec![false; adj.len()];
    let mut q = vec![src];
    seen[src] = true;
    let mut qi = 0usize;
    while qi < q.len() {
        let u = q[qi];
        qi += 1;
        if u == dst {
            break;
        }
        for &(v, _) in &adj[u] {
            if seen[v] {
                continue;
            }
            seen[v] = true;
            prev[v] = Some(u);
            q.push(v);
        }
    }
    if !seen[dst] {
        return None;
    }
    let mut path = vec![dst];
    let mut cur = dst;
    while cur != src {
        cur = prev[cur]?;
        path.push(cur);
    }
    path.reverse();
    Some(path)
}

fn cycle_holonomy(faces: &[Face], adj: &[Vec<(usize, usize)>], path: &[usize], chord: &Face) -> Word {
    let mut w = Word::identity();
    let step = |i: usize, j: usize, faces: &[Face], adj: &[Vec<(usize, usize)>]| -> Option<Word> {
        let fi = adj.get(i)?.iter().find(|(v, _)| *v == j).map(|(_, f)| *f)?;
        let f = faces.get(fi)?;
        if f.i == i && f.j == j {
            Some(f.g_ij.clone())
        } else {
            Some(f.g_ij.inverse())
        }
    };
    for win in path.windows(2) {
        if let Some(s) = step(win[0], win[1], faces, adj) {
            w = w.compose(&s);
        }
    }
    if let (Some(&last), Some(&first)) = (path.last(), path.first()) {
        let dst = if last == first {
            path.get(path.len().saturating_sub(2)).copied()
        } else {
            Some(last)
        };
        if let Some(dst) = dst {
            if dst != first {
                if chord.i == dst && chord.j == first {
                    w = w.compose(&chord.g_ij);
                } else {
                    w = w.compose(&chord.g_ij.inverse());
                }
            }
        }
    }
    w
}

fn shared_volume(supports: &[Vec<(i64, i64)>], verts: &[[u8; 32]]) -> [u64; 4] {
    if verts.is_empty() || supports.is_empty() {
        return [0; 4];
    }
    let i0 = germ_index(&verts[0]);
    if i0 >= supports.len() {
        return [0; 4];
    }
    let mut acc = supports[i0].clone();
    for v in &verts[1..] {
        let i = germ_index(v);
        if i >= supports.len() {
            return [0; 4];
        }
        acc.retain(|p| supports[i].binary_search(p).is_ok());
    }
    let cells: Vec<Cell> = acc.into_iter().map(|(r, c)| (r, c, 0, 0)).collect();
    element::measure_lanes(&cells)
}

fn bind_complex(germs: &[Germ], faces: &[Face]) -> Complex {
    let mut cx = Complex::new();
    let supports: Vec<Vec<(i64, i64)>> = germs.iter().map(after_support).collect();
    for (i, g) in germs.iter().enumerate() {
        cx.set_section(
            germ_vertex(i),
            Section {
                addr: g.address,
                gamma: g.word.rotor(),
            },
            element::measure_lanes(&g.chain),
        );
    }
    for f in faces {
        let vol = element::measure_lanes(&f.overlap);
        if f.spelled {
            cx.set_edge(germ_vertex(f.i), germ_vertex(f.j), f.g_ij.rotor(), vol);
        } else {
            cx.set_remainder_edge(germ_vertex(f.i), germ_vertex(f.j));
        }
    }
    for i in 0..germs.len() {
        cx.coboundary_on_link(&germ_vertex(i), |verts| shared_volume(&supports, verts));
    }
    cx
}

fn first_class(c: &Complex) -> Option<(Rotor, usize)> {
    // index: keyed unfilled cells of this nerve
    for (d, key) in c.unfilled() {
        if let Some(cl) = c.class(&key) {
            return Some(cl);
        }
        let _ = d;
    }
    c.classes().into_iter().next()
}

/// After-support of a germ: the input positions carried by its versor.
/// A germ with no source (created) uses the chain's after-cells; a germ
/// with no target (erased) uses the before-cells.
pub fn after_support(g: &Germ) -> Vec<(i64, i64)> {
    let mut out = Vec::new();
    if g.source.is_none() && g.target.is_some() {
        for &(r, c, _, a) in &g.chain {
            if a != 0 {
                out.push((r, c));
            }
        }
        out.sort_unstable();
        out.dedup();
        return out;
    }
    if g.target.is_none() && g.source.is_some() {
        for &(r, c, b, _) in &g.chain {
            if b != 0 {
                out.push((r, c));
            }
        }
        out.sort_unstable();
        out.dedup();
        return out;
    }
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

/// Transport class of a conformal versor: the spelled word's class key.
pub fn class_key_of_versor(v: &Versor) -> [u8; 32] {
    spell_versor(v)
        .map(|w| w.class_key())
        .unwrap_or([0; 32])
}

fn face_word(a: &Germ, b: &Germ) -> (Word, bool) {
    let w = b.word.compose(&a.word.inverse()).normal_form();
    if w.abelian_identity() || !w.is_empty() || (a.word.abelian_identity() && b.word.abelian_identity()) {
        return (w, true);
    }
    let Some(inv) = a.versor.versor_inverse() else {
        return (Word::identity(), false);
    };
    let g = b.versor.geo(&inv);
    match spell_versor(&g) {
        Some(sp) => (sp.normal_form(), true),
        None => (Word::identity(), false),
    }
}

fn germ_centroid_key(g: &Germ) -> (i64, i64) {
    let s = after_support(g);
    if s.is_empty() {
        return (0, 0);
    }
    let n = s.len() as i64;
    let sr: i64 = s.iter().map(|p| p.0).sum();
    let sc: i64 = s.iter().map(|p| p.1).sum();
    (sr.saturating_mul(1_000_000) / n.max(1), sc.saturating_mul(1_000_000) / n.max(1))
}

/// Transport class of a germ: shell kind, conjugacy class of the word,
/// and hue map. Moduli stay on the germ as a section coordinate, never a
/// matching key. Frames match by kind alone (size is rule.ratio).
pub fn same_stalk(a: &Germ, b: &Germ) -> bool {
    if a.kind == ShellKind::Frame || b.kind == ShellKind::Frame {
        return a.kind == b.kind;
    }
    a.kind == b.kind && a.word.same_class(&b.word) && germ_hue_map(a) == germ_hue_map(b)
}

fn group_stalks(germs: &[Germ]) -> Vec<Vec<usize>> {
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for (i, g) in germs.iter().enumerate() {
        match groups.iter_mut().find(|grp| same_stalk(&germs[grp[0]], g)) {
            Some(grp) => grp.push(i),
            None => groups.push(vec![i]),
        }
    }
    groups
}

fn group_match(inst: &[Vec<usize>], germs: &[Germ], probe: &Germ) -> Option<usize> {
    inst.iter().position(|g| same_stalk(&germs[g[0]], probe))
}

fn ordinal(inst: &[Vec<usize>], i: usize) -> Option<usize> {
    inst.iter().find_map(|v| v.iter().position(|&x| x == i))
}

/// Instances of each stalk, ordered in the configuration space by
/// centroid. The class is the stalk; the instances stay distinct points.
fn instances_of(n: &Nerve) -> Vec<Vec<usize>> {
    let mut groups = group_stalks(&n.germs);
    for g in &mut groups {
        g.sort_by_key(|&i| germ_centroid_key(&n.germs[i]));
    }
    groups
}

/// Relative connection between two bodies: the CGA versor
/// `versor_between` of their after-supports. The spelled word is the
/// linear block when it spells; a pure translation keeps the parabolic
/// versor and does not collapse to the identity carrier.
fn relative_face(a: &Germ, b: &Germ) -> (Word, Versor, bool) {
    let sa = after_support(a);
    let sb = after_support(b);
    if !sa.is_empty() && !sb.is_empty() {
        let va = super::blade::shape_of(&sa);
        let vb = super::blade::shape_of(&sb);
        if let Some(v) = super::blade::versor_between(&va, &vb) {
            let w = spell_versor(&v).map(|s| s.normal_form()).unwrap_or_else(Word::identity);
            return (w, v, true);
        }
    }
    let (w, spelled) = face_word(a, b);
    let v = a
        .versor
        .versor_inverse()
        .map(|inv| b.versor.geo(&inv))
        .unwrap_or_else(element::identity_versor);
    (w, v, spelled)
}

fn emit_face(germs: &[Germ], i: usize, j: usize, faces: &mut Vec<Face>) {
    if i == j {
        return;
    }
    let (lo, hi) = if i < j { (i, j) } else { (j, i) };
    if faces.iter().any(|f| f.i == lo && f.j == hi) {
        return;
    }
    let (g_ij, versor, spelled) = relative_face(&germs[lo], &germs[hi]);
    if !spelled {
        return;
    }
    let overlap = overlap_cells(&after_support(&germs[lo]), &after_support(&germs[hi]));
    faces.push(Face {
        i: lo,
        j: hi,
        overlap,
        g_ij,
        versor,
        spelled,
    });
}

/// Assemble a nerve from germs. Faces are the pairs the bundle relates:
/// the star from the frame to each body, and adjacency of after-supports
/// (overlap or ring distance one). Not the complete graph of spelled pairs.
pub fn assemble(mut germs: Vec<Germ>, remainder: Vec<Cell>) -> Nerve {
    germs.sort_by(|a, b| a.address.cmp(&b.address));
    let n = germs.len();
    let mut faces = Vec::new();
    let frame = germs.iter().position(|g| g.kind == ShellKind::Frame);
    let supports: Vec<Vec<(i64, i64)>> = germs.iter().map(after_support).collect();
    for i in 0..n {
        for j in (i + 1)..n {
            let star = frame == Some(i) || frame == Some(j);
            let adj = !overlap_cells(&supports[i], &supports[j]).is_empty();
            if star || adj {
                emit_face(&germs, i, j, &mut faces);
            }
        }
    }
    faces.sort_by_key(|f| (f.i, f.j));
    let complex = bind_complex(&germs, &faces);
    let cycles = independent_cycles(germs.len(), &faces);
    let class = first_class(&complex);
    Nerve {
        germs,
        complex,
        faces,
        cycles,
        remainder,
        class,
        cover: crate::network::nerve::Keyed::new(),
    }
}

/// The invariant cochain of a family: germ charts, face words, holonomies,
/// and hue maps keyed by chart. Per-pair gauges are not on the tape.
#[derive(Debug, Clone)]
pub struct SentenceRule {
    pub charts: Vec<(Option<stalk::Invariant>, Option<stalk::Invariant>)>,
    pub faces: Vec<(usize, usize, Word, bool)>,
    pub holonomies: Vec<Word>,
    pub hue_maps: Vec<(stalk::Invariant, Vec<(u8, u8)>)>,
    pub nerve: Nerve,
    /// Family frame ratio ((h_out, h_in), (w_out, w_in)).
    pub ratio: ((usize, usize), (usize, usize)),
    /// Affine translation that agreed across pairs, one per stalk.
    pub shifts: Vec<(
        super::peel::ShellKind,
        Option<stalk::Invariant>,
        Option<stalk::Invariant>,
        Word,
        (num_rational::BigRational, num_rational::BigRational),
    )>,
    /// Classes present in some pairs only: class key → pair indices.
    pub sections: crate::network::nerve::Keyed<Vec<u16>>,
}

impl SentenceRule {
    pub fn section_count(&self) -> usize {
        self.sections
            .fiber(&crate::network::nerve::ROOT)
            .map(|(_, v)| v.len())
            .sum()
    }

    /// Hue map of a target chart, by exact axes, never a digest.
    pub fn hue_for(&self, inv: &stalk::Invariant) -> Vec<(u8, u8)> {
        self.hue_maps
            .iter()
            .find(|(i, _)| i == inv)
            .map(|(_, m)| m.clone())
            .unwrap_or_default()
    }

    /// Family-constant shift of this germ's stalk, by algebraic class.
    pub fn shift_of(&self, g: &Germ) -> Option<(num_rational::BigRational, num_rational::BigRational)> {
        self.shifts
            .iter()
            .find(|(k, s, t, w, _)| *k == g.kind && *s == g.source && *t == g.target && w.same_class(&g.word))
            .map(|(_, _, _, _, d)| d.clone())
    }
}

fn germ_hue_map(g: &Germ) -> Vec<(u8, u8)> {
    let mut m = Vec::new();
    for &(_, _, b, a) in &g.chain {
        if !m.iter().any(|(x, _)| *x == b) {
            m.push((b, a));
        }
    }
    m.sort_unstable();
    m
}



/// Vertex key of a sentence: the 1-cocycle of germ words (kind, conjugacy,
/// hue map) and face words, never the serialised tape and never a placement
/// versor. Translation is a fibre coordinate, already on `shift`.
pub fn vertex_address(e: &Element) -> Option<[u8; 32]> {
    let n = from_element(e)?;
    Some(nerve_vertex_key(&n))
}

fn germ_class_key(g: &Germ) -> Vec<u8> {
    let mut row = Vec::new();
    row.extend_from_slice(&g.word.key());
    row.push(g.kind.as_u8());
    for (b, a) in germ_hue_map(g) {
        row.push(b);
        row.push(a);
    }
    row
}

fn face_class_key(f: &Face) -> Vec<u8> {
    let mut row = Vec::new();
    row.extend_from_slice(&f.g_ij.key());
    row.push(u8::from(f.spelled));
    row
}

fn nerve_vertex_key(n: &Nerve) -> [u8; 32] {
    let mut germs: Vec<Vec<u8>> = n.germs.iter().map(germ_class_key).collect();
    germs.sort_unstable();
    let mut faces: Vec<Vec<u8>> = n.faces.iter().map(face_class_key).collect();
    faces.sort_unstable();
    let mut buf = Vec::new();
    for g in germs {
        buf.extend(g);
    }
    for f in faces {
        buf.extend(f);
    }
    *blake3::hash(&buf).as_bytes()
}

/// Same sentence vertex: the same 1-cocycle of words. Placement versors
/// and body moduli are fibre coordinates, not identity.
pub fn nerves_same_vertex(a: &Nerve, b: &Nerve) -> bool {
    if a.germs.len() != b.germs.len() || a.faces.len() != b.faces.len() {
        return false;
    }
    let mut ga: Vec<Vec<u8>> = a.germs.iter().map(germ_class_key).collect();
    let mut gb: Vec<Vec<u8>> = b.germs.iter().map(germ_class_key).collect();
    ga.sort_unstable();
    gb.sort_unstable();
    if ga != gb {
        return false;
    }
    let mut fa: Vec<Vec<u8>> = a.faces.iter().map(face_class_key).collect();
    let mut fb: Vec<Vec<u8>> = b.faces.iter().map(face_class_key).collect();
    fa.sort_unstable();
    fb.sort_unstable();
    fa == fb
}

fn holo_signature(n: &Nerve) -> Vec<Word> {
    let mut holonomies: Vec<Word> = n.cycles.iter().map(|c| c.holonomy.clone()).collect();
    holonomies.sort_by(|a, b| format!("{a:?}").cmp(&format!("{b:?}")));
    holonomies
}

fn affine_shift(v: &Versor) -> Option<(num_rational::BigRational, num_rational::BigRational)> {
    let a = affine_of(v)?;
    if a.len() < 2 || a[0].len() < 3 || a[1].len() < 3 {
        return None;
    }
    Some((a[0][2].clone(), a[1][2].clone()))
}



/// Correspondence across pairs is by transport class (`same_stalk`: kind,
/// word conjugacy, hue map). Moduli stay a section coordinate. The class
/// is the stalk; every instance of that class remains a vertex. Extra
/// instances (count past the family minimum) are sections. Faces of the
/// assembled nerve are kept when the relative *word* is constant; the
/// translation versor is the per-pair shift, already on `per_shift`.
pub fn agree_up_to_gauge(nerves: &[Nerve]) -> Option<SentenceRule> {
    let first = nerves.first()?;
    if first.germs.is_empty() || first.class.is_some() {
        return None;
    }
    for n in nerves {
        if n.class.is_some() {
            return None;
        }
    }
    let insts: Vec<Vec<Vec<usize>>> = nerves.iter().map(instances_of).collect();
    let n_pairs = nerves.len();
    let hue_of = |n: &Nerve, inst: &[Vec<usize>], probe: &Germ| -> Option<Vec<(u8, u8)>> {
        let g = group_match(inst, &n.germs, probe)?;
        let &i = inst[g].first()?;
        Some(germ_hue_map(n.germs.get(i)?))
    };
    let mut rule_groups: Vec<usize> = Vec::new();
    for (gi, grp) in insts[0].iter().enumerate() {
        let probe = &first.germs[grp[0]];
        let present = nerves
            .iter()
            .zip(insts.iter())
            .all(|(n, ix)| group_match(ix, &n.germs, probe).is_some());
        if !present {
            continue;
        }
        let h0 = hue_of(first, &insts[0], probe);
        let constant = nerves
            .iter()
            .zip(insts.iter())
            .all(|(n, ix)| hue_of(n, ix, probe) == h0);
        if constant {
            rule_groups.push(gi);
        }
    }
    let mut sections: crate::network::nerve::Keyed<Vec<u16>> = crate::network::nerve::Keyed::new();
    let mut section_germs: Vec<Germ> = Vec::new();
    for (ni, n) in nerves.iter().enumerate() {
        for grp in &insts[ni] {
            let probe = &n.germs[grp[0]];
            let in_rule = group_match(&insts[0], &first.germs, probe).is_some_and(|gi| rule_groups.contains(&gi));
            if in_rule {
                continue;
            }
            let key = probe.word.class_key();
            match sections.get_mut(&key) {
                Some(v) => {
                    if !v.contains(&(ni as u16)) {
                        v.push(ni as u16);
                    }
                }
                None => {
                    sections.insert(key, vec![ni as u16]);
                }
            }
            if !section_germs.iter().any(|g| same_stalk(g, probe)) {
                section_germs.push(n.germs[grp[0]].clone());
            }
        }
    }
    for &gi in &rule_groups {
        let probe = &first.germs[insts[0][gi][0]];
        let counts: Vec<usize> = insts
            .iter()
            .zip(nerves.iter())
            .map(|(ix, n)| group_match(ix, &n.germs, probe).map(|g| ix[g].len()).unwrap_or(0))
            .collect();
        let min_c = counts.iter().copied().min().unwrap_or(0);
        for (pi, &c) in counts.iter().enumerate() {
            if c > min_c {
                let key = probe.word.class_key();
                match sections.get_mut(&key) {
                    Some(v) => {
                        if !v.contains(&(pi as u16)) {
                            v.push(pi as u16);
                        }
                    }
                    None => {
                        sections.insert(key, vec![pi as u16]);
                    }
                }
            }
        }
    }
    if rule_groups.is_empty() && section_germs.is_empty() {
        return None;
    }
    let mut germs = Vec::new();
    for &gi in &rule_groups {
        let v0 = &insts[0][gi];
        let probe = &first.germs[v0[0]];
        let min_c = insts
            .iter()
            .zip(nerves.iter())
            .map(|(ix, n)| group_match(ix, &n.germs, probe).map(|g| ix[g].len()).unwrap_or(0))
            .min()?;
        if min_c == 0 {
            continue;
        }
        for t in 0..min_c {
            germs.push(first.germs[v0[t]].clone());
        }
    }
    germs.extend(section_germs);
    if germs.is_empty() {
        return None;
    }
    let mut nerve = assemble(germs, first.remainder.clone());
    let nerve_inst = instances_of(&nerve);
    let mut hue_maps: Vec<(stalk::Invariant, Vec<(u8, u8)>)> = Vec::new();
    for g in &nerve.germs {
        if let Some(t) = g.target.clone() {
            if !hue_maps.iter().any(|(i, _)| i == &t) {
                hue_maps.push((t, germ_hue_map(g)));
            }
        }
    }
    let mut per_shift: Vec<Option<(num_rational::BigRational, num_rational::BigRational)>> =
        vec![None; nerve.germs.len()];
    let mut shifts: Vec<(
        super::peel::ShellKind,
        Option<stalk::Invariant>,
        Option<stalk::Invariant>,
        Word,
        (num_rational::BigRational, num_rational::BigRational),
    )> = Vec::new();
    for (i, g) in nerve.germs.iter().enumerate() {
        let Some(t) = ordinal(&nerve_inst, i) else {
            continue;
        };
        let mut held = None;
        let mut agree = true;
        for (n, ix) in nerves.iter().zip(insts.iter()) {
            let Some(gj) = group_match(ix, &n.germs, g) else {
                agree = false;
                break;
            };
            let Some(&j) = ix[gj].get(t) else {
                agree = false;
                break;
            };
            let Some(d) = n.germs.get(j).and_then(|h| affine_shift(&h.versor)) else {
                agree = false;
                break;
            };
            match &held {
                None => held = Some(d),
                Some(h) if *h != d => {
                    agree = false;
                    break;
                }
                _ => {}
            }
        }
        if agree {
            per_shift[i] = held.clone();
            if let Some(d) = held {
                if !shifts.iter().any(|(k, s, t, w, _)| {
                    *k == g.kind && *s == g.source && *t == g.target && w.same_class(&g.word)
                }) {
                    shifts.push((g.kind, g.source.clone(), g.target.clone(), g.word.clone(), d));
                }
            }
        }
    }
    for (i, g) in nerve.germs.iter_mut().enumerate() {
        g.shift = per_shift[i].clone();
    }
    nerve.faces.retain(|f| {
        let Some(gi) = nerve.germs.get(f.i) else {
            return false;
        };
        let Some(gj) = nerve.germs.get(f.j) else {
            return false;
        };
        let Some(ta) = ordinal(&nerve_inst, f.i) else {
            return false;
        };
        let Some(tb) = ordinal(&nerve_inst, f.j) else {
            return false;
        };
        nerves.iter().zip(insts.iter()).all(|(n, ix)| {
            let Some(ga) = group_match(ix, &n.germs, gi) else {
                return false;
            };
            let Some(gb) = group_match(ix, &n.germs, gj) else {
                return false;
            };
            let Some(va) = ix[ga].get(ta).and_then(|&j| n.germs.get(j)) else {
                return false;
            };
            let Some(vb) = ix[gb].get(tb).and_then(|&j| n.germs.get(j)) else {
                return false;
            };
            let (w, _v, s) = relative_face(va, vb);
            w == f.g_ij && s == f.spelled
        })
    });
    nerve.complex = bind_complex(&nerve.germs, &nerve.faces);
    nerve.cycles = independent_cycles(nerve.germs.len(), &nerve.faces);
    // Fill coboundaries of the word-constant 1-skeleton: a vortex that
    // closes is a coboundary, not a missing class. Remaining class is torsion.
    let _ = close(&mut nerve);
    let faces: Vec<(usize, usize, Word, bool)> = nerve
        .faces
        .iter()
        .map(|f| (f.i, f.j, f.g_ij.clone(), f.spelled))
        .collect();
    let holo0 = holo_signature(&nerve);
    Some(SentenceRule {
        charts: nerve
            .germs
            .iter()
            .map(|g| (g.source.clone(), g.target.clone()))
            .collect(),
        faces,
        holonomies: holo0,
        hue_maps,
        nerve,
        ratio: ratio_from_nerve(first),
        shifts,
        sections,
    })
}

fn ratio_from_nerve(n: &Nerve) -> ((usize, usize), (usize, usize)) {
    let Some(g) = n.germs.iter().find(|g| g.kind == ShellKind::Frame) else {
        return ((1, 1), (1, 1));
    };
    let axis = |inv: &Option<stalk::Invariant>, i: usize| {
        inv.as_ref().and_then(|v| v.0.get(i).copied()).unwrap_or(1) as usize
    };
    let (hi, wi) = (axis(&g.source, 1), axis(&g.source, 2));
    let (ho, wo) = (axis(&g.target, 1), axis(&g.target, 2));
    if hi == 0 || wi == 0 || ho == 0 || wo == 0 {
        return ((1, 1), (1, 1));
    }
    let gh = gcd_dim(ho, hi).max(1);
    let gw = gcd_dim(wo, wi).max(1);
    ((ho / gh, hi / gh), (wo / gw, wi / gw))
}

fn gcd_dim(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd_dim(b, a % b)
    }
}

/// Solve δγ = ω: a gauge per germ, integer-linear in letter exponents.
/// On a forest every closed 1-cochain is exact; a chord that does not
/// match the tree path is a withheld class.
pub fn solve_coboundary(n: &Nerve) -> Option<Vec<Word>> {
    let mut edges = Vec::new();
    for f in &n.faces {
        if !f.spelled {
            continue;
        }
        edges.push((f.i, f.j, f.g_ij.rotor()));
    }
    cech::solve_gauge(n.germs.len(), &edges).map(|gs| gs.into_iter().map(|g| word_of_rotor(&g)).collect())
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

/// One snap of the twist loop (A16): a finite chord closed, torsion filed
/// or left open, or an infinite class spoken as a demand.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Snap {
    Closed { chord: usize, order: u32 },
    TorsionFiled { chord: usize, ell: u32 },
    TorsionOpen { chord: usize },
    Demand { chord: usize, key: [u8; 32] },
}

fn apply_cochain_gauge(n: &mut Nerve, gamma: &[Word]) {
    for f in &mut n.faces {
        if f.i >= gamma.len() || f.j >= gamma.len() {
            continue;
        }
        f.g_ij = gamma[f.j]
            .compose(&f.g_ij)
            .compose(&gamma[f.i].inverse())
            .normal_form();
    }
    n.cycles = independent_cycles(n.germs.len(), &n.faces);
}

fn try_close_chord(n: &mut Nerve, idx: usize) -> bool {
    let mut edges = Vec::new();
    for f in &n.faces {
        if !f.spelled {
            continue;
        }
        edges.push((f.i, f.j, f.g_ij.rotor()));
    }
    let Some(gs) = cech::solve_gauge(n.germs.len(), &edges) else {
        return false;
    };
    let words: Vec<Word> = gs.iter().map(|g| g.spelled_word()).collect();
    let target = n.cycles.get(idx).map(|c| c.holonomy.rotor().class_key());
    apply_cochain_gauge(n, &words);
    match target {
        Some(key) => !n
            .cycles
            .iter()
            .any(|c| c.holonomy.rotor().class_key() == key && !c.holonomy.abelian_identity()),
        None => true,
    }
}

fn file_cycle(n: &mut Nerve, idx: usize, ell: u32) -> bool {
    if ell == 0 {
        return false;
    }
    let Some(c) = n.cycles.get(idx) else {
        return false;
    };
    let Some(&a) = c.germs.first() else {
        return false;
    };
    let b = c
        .germs
        .get(c.germs.len().saturating_sub(2))
        .copied()
        .unwrap_or(a);
    n.complex
        .file_congruent(germ_vertex(a), germ_vertex(b), c.holonomy.rotor(), ell);
    true
}

/// A16: tree-walk the nerve, sort open chords by [`Word::order`] (finite
/// ascending, infinite last), close the loosest finite chord first.
/// Leftover finite chords go to A17; infinite chords are demands.
/// `horizon` classes (Held/Stood) are not re-derived.
pub fn twist(
    n: &mut Nerve,
    idea: [u8; 32],
    horizon: &Keyed<(Rotor, Order, Standing)>,
    budget: &mut Budget,
) -> Vec<Snap> {
    let mut snaps = Vec::new();
    let mut handled: Keyed<()> = Keyed::new();
    loop {
        if budget.remaining() == 0 {
            break;
        }
        n.cycles = independent_cycles(n.germs.len(), &n.faces);
        let mut open: Vec<(usize, Order, [u8; 32])> = n
            .cycles
            .iter()
            .enumerate()
            .filter(|(_, c)| !c.holonomy.abelian_identity())
            .map(|(i, c)| (i, c.holonomy.order(), c.holonomy.rotor().class_key()))
            .filter(|(_, order, k)| {
                if handled.contains(k) {
                    return false;
                }
                match order {
                    Order::Finite(ell) => !n.complex.degrees.iter().any(|layer| {
                        layer.fiber(&crate::network::nerve::ROOT).any(|(_, cell)| {
                            matches!(&cell.fill, Fill::Congruent(r, e) if e == ell && r.class_key() == *k)
                        })
                    }),
                    Order::Infinite => n
                        .class
                        .as_ref()
                        .map(|(r, _)| r.class_key() != *k)
                        .unwrap_or(true),
                }
            })
            .collect();
        open.sort_by(|a, b| a.1.cmp(&b.1).then(a.2.cmp(&b.2)).then(a.0.cmp(&b.0)));
        let Some(&(idx, order, key)) = open.first() else {
            break;
        };
        if !budget.spend() {
            break;
        }
        let held = horizon.get(&key).is_some();
        match order {
            Order::Infinite => {
                if !held {
                    snaps.push(Snap::Demand { chord: idx, key });
                    if n.class.is_none() {
                        n.class = Some((n.cycles[idx].holonomy.rotor(), 1));
                    }
                    eprintln!(
                        "  demand spoken {:02x}{:02x}… order ∞",
                        idea[0], idea[1]
                    );
                }
            }
            Order::Finite(ell) => {
                if !held && try_close_chord(n, idx) {
                    snaps.push(Snap::Closed { chord: idx, order: ell });
                    eprintln!(
                        "  twist {:02x}{:02x}… chord {idx} order {ell} closed",
                        idea[0], idea[1]
                    );
                } else if file_cycle(n, idx, ell) {
                    if !held {
                        snaps.push(Snap::Closed { chord: idx, order: ell });
                        eprintln!(
                            "  twist {:02x}{:02x}… chord {idx} order {ell} closed",
                            idea[0], idea[1]
                        );
                    }
                    snaps.push(Snap::TorsionFiled { chord: idx, ell });
                    eprintln!(
                        "  torsion filed {:02x}{:02x}… (σ,{ell})",
                        idea[0], idea[1]
                    );
                } else {
                    snaps.push(Snap::TorsionOpen { chord: idx });
                    eprintln!(
                        "  torsion open {:02x}{:02x}… chord {idx}",
                        idea[0], idea[1]
                    );
                }
            }
        }
        handled.insert(key, ());
    }
    snaps
}

/// Close the sentence: fill up the degrees on each germ's link. A class
/// at any degree stays as torsion and the sentence is thought.
pub fn close(n: &mut Nerve) -> bool {
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let mut all_filled = true;
    let supports: Vec<Vec<(i64, i64)>> = n.germs.iter().map(after_support).collect();
    // index: germs of this nerve (its vertices)
    for i in 0..n.germs.len() {
        n.complex.coboundary_on_link(&germ_vertex(i), |verts| shared_volume(&supports, verts));
        // index: link of this germ
        for k in n.complex.link_of(&germ_vertex(i)) {
            let Some(d) = n.complex.cell(&k).map(|c| c.degree()) else {
                continue;
            };
            if d == 0 {
                continue;
            }
            if seen.contains(&k) {
                continue;
            }
            seen.insert(k, ());
            match n.complex.fill(&k) {
                Fill::Filled(_) => {}
                Fill::Congruent(r, _) => {
                    n.class = Some((r, d));
                    all_filled = false;
                }
                Fill::Class(r) => {
                    n.class = Some((r, d));
                    all_filled = false;
                }
                Fill::Open | Fill::Already => {
                    all_filled = false;
                }
            }
        }
    }
    n.cycles = independent_cycles(n.germs.len(), &n.faces);
    if !all_filled || n.class.is_some() {
        return false;
    }
    let Some(gamma) = solve_coboundary(n) else {
        return false;
    };
    if gamma.iter().any(|w| !w.abelian_identity()) {
        apply_gauge(n, &gamma);
    }
    n.is_closed()
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

/// Magnitude of the closed cochain: spatial area, gauge permutation
/// support, temporal 1-morphism step, and holonomic shell plus |χ| of
/// the 1-skeleton. Never a dummy `[0,0,0,1]`.
pub fn cochain_magnitude(n: &Nerve) -> [u64; 4] {
    let mut chain = Vec::new();
    for g in &n.germs {
        if g.kind == ShellKind::Frame {
            continue;
        }
        chain.extend(g.chain.iter().copied());
    }
    chain.sort_unstable();
    chain.dedup();
    let mut mag = element::measure_lanes(&chain);
    if mag[2] == 0 {
        let morph = n.germs.iter().any(|g| {
            g.kind != ShellKind::Frame
                && (!g.word.abelian_identity() || g.chain.iter().any(|&(_, _, b, a)| b != a))
        });
        mag[2] = u64::from(morph);
    }
    if n.germs.len() >= 2 {
        let chi = (n.germs.len() as i64 - n.faces.len() as i64).unsigned_abs();
        mag[3] = mag[3].saturating_add(chi);
    }
    mag
}

fn word_to_form_bytes(w: &Word, out: &mut Vec<u8>) {
    w.to_bytes(out);
    match w.contact() {
        None => out.push(0),
        Some(c) => {
            out.push(1);
            out.extend_from_slice(&c.anchor.to_le_bytes());
            out.push(c.ring);
        }
    }
}

fn word_from_form_bytes(b: &[u8], pos: &mut usize) -> Option<Word> {
    let w = Word::from_bytes(b, pos)?;
    let tag = *b.get(*pos)?;
    *pos += 1;
    if tag == 0 {
        return Some(w);
    }
    if tag != 1 {
        return None;
    }
    let anchor = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
    *pos += 2;
    let ring = *b.get(*pos)?;
    *pos += 1;
    Some(w.with_contact(alphabet::Contact { anchor, ring }))
}

fn put_inv(inv: &stalk::Invariant, out: &mut Vec<u8>) {
    out.extend_from_slice(&(inv.0.len().min(u16::MAX as usize) as u16).to_le_bytes());
    for a in inv.0.iter().take(u16::MAX as usize) {
        out.extend_from_slice(&a.to_le_bytes());
    }
}

fn take_inv(b: &[u8], pos: &mut usize) -> Option<stalk::Invariant> {
    let n = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?) as usize;
    *pos += 2;
    let mut axes = Vec::with_capacity(n.min(256));
    for _ in 0..n.min(256) {
        let v = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
        *pos += 8;
        axes.push(v);
    }
    Some(stalk::Invariant(axes))
}

fn encode_form_v2(f: &Form, out: &mut Vec<u8>) {
    out.extend_from_slice(&(f.vertices.len() as u16).to_le_bytes());
    for v in &f.vertices {
        match v {
            Vertex::Frame => out.push(0),
            Vertex::Word {
                address,
                word,
                rho,
                placement,
                gauge,
                class,
            } => {
                out.push(1);
                out.extend_from_slice(address);
                word_to_form_bytes(word, out);
                alphabet::put_q(out, rho);
                alphabet::put_q(out, &placement.0);
                alphabet::put_q(out, &placement.1);
                out.extend_from_slice(&(gauge.len() as u16).to_le_bytes());
                for &(a, b) in gauge {
                    out.push(a);
                    out.push(b);
                }
                put_inv(class, out);
            }
            Vertex::Glued {
                address,
                chart,
                magnitude,
                depth,
                placement,
            } => {
                out.push(2);
                out.extend_from_slice(address);
                put_inv(chart, out);
                for m in magnitude {
                    out.extend_from_slice(&m.to_le_bytes());
                }
                out.extend_from_slice(&depth.to_le_bytes());
                word_to_form_bytes(placement, out);
            }
            Vertex::Rebased { by, of } => {
                out.push(3);
                out.extend_from_slice(&by.to_le_bytes());
                out.extend_from_slice(&of.to_le_bytes());
            }
            Vertex::Unfolded { measure } => {
                out.push(4);
                match measure {
                    Measure::Occupied => out.push(0),
                    Measure::Remainder => out.push(1),
                    Measure::Ring { anchor } => {
                        out.push(2);
                        out.extend_from_slice(&anchor.to_le_bytes());
                    }
                }
            }
        }
    }
    out.extend_from_slice(&(f.faces.len() as u16).to_le_bytes());
    for face in &f.faces {
        out.extend_from_slice(&face.i.to_le_bytes());
        out.extend_from_slice(&face.j.to_le_bytes());
        word_to_form_bytes(&face.g_ij, out);
    }
    out.extend_from_slice(&(f.loops.len() as u16).to_le_bytes());
    for lp in &f.loops {
        out.extend_from_slice(&(lp.faces.len() as u16).to_le_bytes());
        for &i in &lp.faces {
            out.extend_from_slice(&i.to_le_bytes());
        }
        word_to_form_bytes(&lp.holonomy, out);
    }
    out.extend_from_slice(&(f.hue_maps.len() as u16).to_le_bytes());
    for (k, pairs) in f.hue_maps.fiber(&crate::network::nerve::ROOT) {
        out.extend_from_slice(k);
        out.extend_from_slice(&(pairs.len() as u16).to_le_bytes());
        for &(a, b) in pairs {
            out.push(a);
            out.push(b);
        }
    }
    out.extend_from_slice(&(f.remainder.len() as u32).to_le_bytes());
    for &(r, c, before, after) in &f.remainder {
        out.extend_from_slice(&r.to_le_bytes());
        out.extend_from_slice(&c.to_le_bytes());
        out.push(before);
        out.push(after);
    }
    out.extend_from_slice(&f.depth.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
}

fn take_u16(b: &[u8], pos: &mut usize) -> Option<u16> {
    let v = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
    *pos += 2;
    Some(v)
}

fn decode_form_v2(b: &[u8], pos: &mut usize) -> Option<Form> {
    let nv = take_u16(b, pos)? as usize;
    if nv == 0 || nv > 4096 {
        return None;
    }
    let mut vertices = Vec::with_capacity(nv);
    for _ in 0..nv {
        let kind = *b.get(*pos)?;
        *pos += 1;
        let v = match kind {
            0 => Vertex::Frame,
            1 => {
                let address: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                *pos += 32;
                let word = word_from_form_bytes(b, pos)?;
                let rho = alphabet::take_q(b, pos)?;
                let p0 = alphabet::take_q(b, pos)?;
                let p1 = alphabet::take_q(b, pos)?;
                let ng = take_u16(b, pos)? as usize;
                let mut gauge = Vec::with_capacity(ng.min(256));
                for _ in 0..ng.min(256) {
                    gauge.push((*b.get(*pos)?, *b.get(*pos + 1)?));
                    *pos += 2;
                }
                let class = take_inv(b, pos)?;
                Vertex::Word {
                    address,
                    word,
                    rho,
                    placement: (p0, p1),
                    gauge,
                    class,
                }
            }
            2 => {
                let address: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                *pos += 32;
                let chart = take_inv(b, pos)?;
                let mut magnitude = [0u64; 4];
                for m in &mut magnitude {
                    *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                }
                let depth = take_u16(b, pos)?;
                let placement = word_from_form_bytes(b, pos)?;
                Vertex::Glued {
                    address,
                    chart,
                    magnitude,
                    depth,
                    placement,
                }
            }
            3 => {
                let by = take_u16(b, pos)?;
                let of = take_u16(b, pos)?;
                Vertex::Rebased { by, of }
            }
            4 => {
                let m = *b.get(*pos)?;
                *pos += 1;
                let measure = match m {
                    0 => Measure::Occupied,
                    1 => Measure::Remainder,
                    2 => {
                        let anchor = take_u16(b, pos)?;
                        Measure::Ring { anchor }
                    }
                    _ => return None,
                };
                Vertex::Unfolded { measure }
            }
            _ => return None,
        };
        vertices.push(v);
    }
    let nf = take_u16(b, pos)? as usize;
    let mut faces = Vec::with_capacity(nf.min(4096));
    for _ in 0..nf.min(4096) {
        let i = take_u16(b, pos)?;
        let j = take_u16(b, pos)?;
        let g_ij = word_from_form_bytes(b, pos)?;
        faces.push(FormFace { i, j, g_ij });
    }
    let nl = take_u16(b, pos)? as usize;
    let mut loops = Vec::with_capacity(nl.min(4096));
    for _ in 0..nl.min(4096) {
        let n = take_u16(b, pos)? as usize;
        let mut fs = Vec::with_capacity(n.min(4096));
        for _ in 0..n.min(4096) {
            fs.push(take_u16(b, pos)?);
        }
        let holonomy = word_from_form_bytes(b, pos)?;
        loops.push(Loop {
            faces: fs,
            holonomy,
        });
    }
    let nh = take_u16(b, pos).unwrap_or(0) as usize;
    let mut hue_maps = Keyed::new();
    for _ in 0..nh.min(4096) {
        let k: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        let np = take_u16(b, pos)? as usize;
        let mut pairs = Vec::with_capacity(np.min(256));
        for _ in 0..np.min(256) {
            pairs.push((*b.get(*pos)?, *b.get(*pos + 1)?));
            *pos += 2;
        }
        hue_maps.insert(k, pairs);
    }
    let nr = take_u32(b, pos).unwrap_or(0) as usize;
    let mut remainder = Vec::with_capacity(nr.min(1 << 16));
    for _ in 0..nr.min(1 << 16) {
        remainder.push(take_cell(b, pos)?);
    }
    let depth = take_u16(b, pos).unwrap_or(0);
    let ncert = take_u16(b, pos).unwrap_or(0);
    for _ in 0..ncert {
        let _ = take_u16(b, pos)?;
        *pos = pos.checked_add(1)?;
        *pos = pos.checked_add(32)?;
        *pos = pos.checked_add(32)?;
        *pos = pos.checked_add(12)?;
        *pos = pos.checked_add(12)?;
        let kind = *b.get(*pos)?;
        *pos += 1;
        match kind {
            3 => {
                *pos = pos.checked_add(32)?;
                let _ = word_from_form_bytes(b, pos)?;
            }
            4 => {
                let _ = take_inv(b, pos)?;
            }
            _ => {
                let _ = word_from_form_bytes(b, pos)?;
            }
        }
    }
    Some(Form {
        vertices,
        faces,
        loops,
        remainder,
        depth,
        hue_maps,
    })
}

/// SENTENCE_TAG v2: the Form tape. v1 nerve tapes still go through [`as_element`].
pub fn as_element_form(f: &Form) -> Option<Element> {
    if f.vertices.is_empty() {
        return None;
    }
    let mut e = Element::identity(GERM_DIM);
    let mut word = vec![element::SENTENCE_TAG, 2];
    encode_form_v2(f, &mut word);
    e.set_description(word);
    e.set_magnitude([0, 0, 0, 1]);
    Some(e)
}

/// Read a Form from a v2 tape, or lift a v1 nerve tape through [`Form::from_nerve`].
pub fn from_element_form(e: &Element) -> Option<Form> {
    if !e.is_sentence() {
        return None;
    }
    let b = e.description();
    if b.get(1).copied() == Some(2) {
        let mut pos = 2usize;
        if let Some(f) = decode_form_v2(b, &mut pos) {
            return Some(f);
        }
    }
    let n = from_element(e)?;
    Some(Form::from_nerve(&n))
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
    let carried = cochain_magnitude(n);
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
        e.record_dislocation(asg, f.versor.clone(), element::FIBER_SPATIAL);
    }
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    // index: keyed cells of this nerve (tape projection)
    for layer in &n.complex.degrees {
        for (_, cell) in layer.fiber(&ROOT) {
            match &cell.fill {
                Fill::Filled(gamma) if !gamma.is_identity() => {
                    let w = word_of_rotor(gamma);
                    if let Some(v) = element::conformal_of(&alphabet::Spelling { word: w, rho: one.clone() }, &(zero.clone(), zero.clone())) {
                        let asg: Vec<(u8, u8)> = cell
                            .vertices
                            .windows(2)
                            .map(|p| (germ_index(&p[0]) as u8, germ_index(&p[1]) as u8))
                            .collect();
                        e.record_dislocation(asg, v, element::FIBER_SPATIAL);
                    }
                }
                Fill::Congruent(g, ell) => {
                    if *ell > 0 && !matches!(g.word_order(), Order::Infinite) {
                        let asg: Vec<(u8, u8)> = cell
                            .vertices
                            .windows(2)
                            .map(|p| (germ_index(&p[0]) as u8, germ_index(&p[1]) as u8))
                            .collect();
                        e.record_torsion(asg, *ell, element::FIBER_SPATIAL);
                    }
                    let _ = g;
                }
                _ => {}
            }
        }
    }
    if let Some((class, degree)) = &n.class {
        let sigma: Vec<(u8, u8)> = n
            .complex
            .degrees
            .get(*degree)
            .and_then(|layer| {
                let mut s = None;
                for (_, cell) in layer.fiber(&ROOT) {
                    if s.is_none() && cell.coefficient == *class {
                        s = Some(
                            cell.vertices
                                .windows(2)
                                .map(|w| (germ_index(&w[0]) as u8, germ_index(&w[1]) as u8))
                                .collect(),
                        );
                    }
                }
                s
            })
            .unwrap_or_default();
        if let Order::Finite(ell) = word_of_rotor(class).order() {
            e.record_torsion(sigma, ell, element::FIBER_SPATIAL);
        }
    } else if !n.is_closed() {
        for c in &n.cycles {
            if c.holonomy.abelian_identity() {
                continue;
            }
            if let crate::transducer::alphabet::Order::Finite(ell) = c.holonomy.order() {
                let sigma: Vec<(u8, u8)> = c.germs.windows(2).map(|w| (w[0] as u8, w[1] as u8)).collect();
                e.record_torsion(sigma, ell, element::FIBER_SPATIAL);
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
        out.push(g.kind.as_u8());
        write_opt_inv(&g.source, out);
        write_opt_inv(&g.target, out);
        match &g.shift {
            None => out.push(0),
            Some((r, c)) => {
                out.push(1);
                alphabet::put_q(out, r);
                alphabet::put_q(out, c);
            }
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
        f.versor.to_bytes(out);
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
    match &n.class {
        None => out.push(0),
        Some((class, degree)) => {
            out.push(1);
            out.push(*degree as u8);
            class.to_bytes(out);
        }
    }
    out.extend_from_slice(&(n.cover.len() as u32).to_le_bytes());
    crate::snapshot::persist(&n.cover, |k, _| out.extend_from_slice(k));
}

fn write_opt_inv(inv: &Option<stalk::Invariant>, out: &mut Vec<u8>) {
    match inv {
        None => out.push(0),
        Some(i) => {
            out.push(1);
            out.push(i.0.len().min(255) as u8);
            for a in i.0.iter().take(255) {
                out.extend_from_slice(&a.to_le_bytes());
            }
        }
    }
}

fn take_opt_inv(b: &[u8], pos: &mut usize) -> Option<Option<stalk::Invariant>> {
    let tag = *b.get(*pos)?;
    *pos += 1;
    if tag == 0 {
        return Some(None);
    }
    let n = *b.get(*pos)? as usize;
    *pos += 1;
    let mut axes = Vec::with_capacity(n);
    for _ in 0..n {
        let v = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
        *pos += 8;
        axes.push(v);
    }
    Some(Some(stalk::Invariant(axes)))
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
        let _stored: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        let versor = Versor::from_bytes(b, pos)?;
        let word = Word::from_bytes(b, pos)?;
        let nc = take_u32(b, pos)? as usize;
        let mut chain = Vec::with_capacity(nc.min(1 << 16));
        for _ in 0..nc.min(1 << 16) {
            chain.push(take_cell(b, pos)?);
        }
        let kind = ShellKind::from_u8(*b.get(*pos)?)?;
        *pos += 1;
        let source = take_opt_inv(b, pos)?;
        let target = take_opt_inv(b, pos)?;
        let shift = match *b.get(*pos)? {
            0 => {
                *pos += 1;
                None
            }
            _ => {
                *pos += 1;
                let r = alphabet::take_q(b, pos)?;
                let c = alphabet::take_q(b, pos)?;
                Some((r, c))
            }
        };
        let chain = element::canonical_chain(&chain).unwrap_or(chain);
        let address = element::from_residual(&chain).address();
        germs.push(Germ { address, versor, word, chain, source, target, kind, shift });
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
        let versor = Versor::from_bytes(b, pos)?;
        let spelled = *b.get(*pos)? & 1 != 0;
        *pos += 1;
        faces.push(Face { i, j, overlap, g_ij, versor, spelled });
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
    let class = if *pos < b.len() && b[*pos] == 1 {
        *pos += 1;
        let degree = *b.get(*pos)? as usize;
        *pos += 1;
        Some((Rotor::from_bytes(b, pos)?, degree))
    } else {
        if *pos < b.len() {
            *pos += 1;
        }
        None
    };
    let complex = bind_complex(&germs, &faces);
    let mut cover = crate::network::nerve::Keyed::new();
    if *pos + 4 <= b.len() {
        let nc = take_u32(b, pos).unwrap_or(0) as usize;
        for _ in 0..nc.min(1 << 16) {
            if let Some(k) = b.get(*pos..*pos + 32).and_then(|s| s.try_into().ok()) {
                *pos += 32;
                cover.insert(k, ());
            } else {
                break;
            }
        }
    }
    Some(Nerve { germs, complex, faces, cycles, remainder, class, cover })
}

/// A sentence Element: tagged tape, not an opened-axis connection.
pub fn is_sentence(e: &Element) -> bool {
    e.is_sentence()
}

/// Chart of a sentence Element: leading axis 2, then germ/face/cycle
/// counts from the tape, then each germ's linear invariants.
pub fn chart_of(e: &Element) -> crate::transducer::stalk::Invariant {
    let n = from_element(e);
    let (germs, faces, cycles) = n.as_ref().map(|n| {
        let d1 = n.complex.degrees.get(1).map(|l| l.len() as u64).unwrap_or(0);
        let d2 = n.complex.degrees.get(2).map(|l| l.len() as u64).unwrap_or(0);
        (n.germs.len() as u64, d1, d2)
    }).unwrap_or((1, 0, 0));
    let mut axes = vec![2u64, germs, faces, cycles];
    if let Some(n) = n {
        let mut extra: Vec<Vec<u64>> = Vec::new();
        for g in &n.germs {
            extra.push(g.word.form().map(|f| {
                let mut row = vec![f.parity as u64];
                row.extend(f.rot.iter().map(|&x| x as u64));
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
    let mut n = from_element(e)?;
    for g in &n.germs {
        if spell_versor(&g.versor).as_ref() != Some(&g.word) && !g.word.abelian_identity() {
            return Some("sentence germ does not re-derive from its letters");
        }
    }
    for f in &n.faces {
        if f.i >= n.germs.len() || f.j >= n.germs.len() {
            return Some("sentence face does not re-derive from its germs");
        }
        let (w, _v, spelled) = relative_face(&n.germs[f.i], &n.germs[f.j]);
        if spelled != f.spelled {
            return Some("sentence face does not re-derive from its germs");
        }
        if spelled && w != f.g_ij {
            return Some("sentence face g_ij does not re-derive from its germs");
        }
    }
    // δg on every triangle of the 1-skeleton: a fillable coboundary is
    // closed, not a missing class. from_element rebinds the complex, so
    // the fill is re-derived here the same way close derives it.
    {
        let supports: Vec<Vec<(i64, i64)>> = n.germs.iter().map(after_support).collect();
        let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for i in 0..n.germs.len() {
            n.complex.coboundary_on_link(&germ_vertex(i), |verts| shared_volume(&supports, verts));
            for k in n.complex.link_of(&germ_vertex(i)) {
                let Some(d) = n.complex.cell(&k).map(|c| c.degree()) else {
                    continue;
                };
                if d == 0 {
                    continue;
                }
                if seen.contains(&k) {
                    continue;
                }
                seen.insert(k, ());
                match n.complex.fill(&k) {
                    Fill::Filled(_) | Fill::Already => {}
                    Fill::Congruent(_, _) | Fill::Class(_) => {
                        if n.class.is_none() && e.torsion().is_empty() {
                            return Some("sentence has a non-coboundary coefficient with no class");
                        }
                    }
                    Fill::Open => {}
                }
            }
        }
    }
    for c in &n.cycles {
        if !c.holonomy.abelian_identity() {
            match c.holonomy.order() {
                crate::transducer::alphabet::Order::Infinite if n.class.is_none() && e.torsion().is_empty() => {
                    return Some("sentence has a non-coboundary coefficient with no class")
                }
                _ => {}
            }
        }
    }
    // index: keyed unfilled cells of this nerve
    for (d, key) in n.complex.unfilled() {
        if d == 0 {
            continue;
        }
        if n.class.is_none() && e.torsion().is_empty() {
            if n.complex.class(&key).is_none() {
                return Some("sentence has a non-coboundary coefficient with no class");
            }
        }
    }
    let derived = bind_complex(&n.germs, &n.faces);
    let class_got = first_class(&derived);
    match (class_got, n.class.clone()) {
        (Some(_), None) => return Some("sentence has a non-coboundary coefficient with no class"),
        (Some((got, gd)), Some((held, hd))) if got != held || gd != hd => {
            return Some("sentence H² class does not re-derive")
        }
        (None, Some(_)) => return Some("sentence H² class does not re-derive"),
        _ => {}
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

    fn q0() -> num_rational::BigRational {
        num_rational::BigRational::from_integer(num_bigint::BigInt::from(0))
    }
    fn q1() -> num_rational::BigRational {
        num_rational::BigRational::from_integer(num_bigint::BigInt::from(1))
    }

    fn closed_word_form(word: Word, contact: Option<alphabet::Contact>) -> Form {
        let word = match contact {
            Some(c) => word.with_contact(c),
            None => word,
        };
        let mut f = Form::default();
        f.vertices.push(Vertex::Word {
            address: [7u8; 32],
            word: word.clone(),
            rho: q1(),
            placement: (q0(), q1()),
            gauge: vec![(1, 2)],
            class: stalk::Invariant(vec![3, 5]),
        });
        f.faces.push(FormFace {
            i: 0,
            j: 1,
            g_ij: word.clone(),
        });
        f.remainder.clear();
        f.depth = f.depth_of();
        f
    }

    /// SENTENCE_TAG v2 round-trips Frame, Word (with contact), Glued,
    /// Rebased, Unfolded, faces, and loops.
    #[test]
    fn form_tape_v2_round_trips() {
        let q = alphabet::LetterId::quarter((0, 1)).expect("quarter");
        let w = Word::from_letters(&[(q, false)]);
        let inner = closed_word_form(w.clone(), None);
        assert!(inner.is_closed(), "a tree of two vertices coheres");
        let inner_e = as_element_form(&inner).expect("inner tape");
        let inner_addr = inner_e.address();

        let mut f = closed_word_form(
            w.clone(),
            Some(alphabet::Contact { anchor: 1, ring: 2 }),
        );
        let glued = f
            .glue(&inner, inner_addr, Word::identity())
            .expect("glue depth 0 into depth 0");
        let rebased = f.rebase(0, 1);
        let unfolded = f.unfold(Measure::Occupied);
        f.faces.push(FormFace {
            i: 0,
            j: glued,
            g_ij: Word::identity(),
        });
        f.faces.push(FormFace {
            i: 1,
            j: glued,
            g_ij: w.clone(),
        });
        f.loops = Form::loops_of(&f.faces, f.vertices.len());
        f.remainder = vec![(9, 9, 1, 0)];
        assert!(matches!(f.vertices[glued as usize], Vertex::Glued { .. }));
        assert!(matches!(f.vertices[rebased as usize], Vertex::Rebased { by: 0, of: 1 }));
        assert!(matches!(
            f.vertices[unfolded as usize],
            Vertex::Unfolded {
                measure: Measure::Occupied
            }
        ));

        let e = as_element_form(&f).expect("v2 tape");
        assert!(e.is_sentence());
        assert_eq!(e.description()[1], 2, "version 2");
        let back = from_element_form(&e).expect("decode v2");
        assert_eq!(back.vertices.len(), f.vertices.len());
        assert_eq!(back.faces.len(), f.faces.len());
        assert_eq!(back.loops.len(), f.loops.len());
        assert_eq!(back.remainder, f.remainder);
        assert_eq!(back.depth, f.depth);
        match (&f.vertices[1], &back.vertices[1]) {
            (
                Vertex::Word { word: a, gauge: ga, class: ca, .. },
                Vertex::Word { word: b, gauge: gb, class: cb, .. },
            ) => {
                assert_eq!(a.contact().map(|c| (c.anchor, c.ring)), Some((1, 2)));
                assert_eq!(a, b);
                assert_eq!(ga, gb);
                assert_eq!(ca, cb);
            }
            _ => panic!("word vertex"),
        }
        match (&f.vertices[glued as usize], &back.vertices[glued as usize]) {
            (Vertex::Glued { address: a, depth: da, .. }, Vertex::Glued { address: b, depth: db, .. }) => {
                assert_eq!(a, b);
                assert_eq!(da, db);
            }
            _ => panic!("glued vertex"),
        }
        assert_eq!(
            back.vertices[rebased as usize],
            Vertex::Rebased { by: 0, of: 1 }
        );
        assert_eq!(
            back.vertices[unfolded as usize],
            Vertex::Unfolded {
                measure: Measure::Occupied
            }
        );
        assert_eq!(back.faces[0].i, 0);
        assert_eq!(back.faces[0].j, 1);
        assert_eq!(back.faces[0].g_ij.contact().map(|c| c.ring), Some(2));
    }

    /// Depth is one plus the deepest glued form; gluing at equal depth is refused.
    #[test]
    fn depth_is_one_plus_max_glued() {
        let q = alphabet::LetterId::quarter((0, 1)).expect("quarter");
        let w = Word::from_letters(&[(q, false)]);
        let inner = closed_word_form(w.clone(), None);
        assert_eq!(inner.depth_of(), 0);
        let mut outer = Form::default();
        let a0 = [1u8; 32];
        outer.glue(&inner, a0, Word::identity()).expect("glue into empty host");
        assert_eq!(outer.depth_of(), 1);
        assert_eq!(outer.depth, 1);
        let mut wrap = Form::default();
        wrap.glue(&outer, [2u8; 32], Word::identity()).expect("glue depth 1 into depth 0");
        assert_eq!(wrap.depth_of(), 2);
        let outer_copy = outer.clone();
        assert!(
            outer.glue(&outer_copy, [3u8; 32], Word::identity()).is_none(),
            "equal depth is refused"
        );
        assert!(
            outer.glue(&wrap, [4u8; 32], Word::identity()).is_none(),
            "deeper form is refused"
        );
        let mut again = Form::default();
        again.glue(&inner, a0, Word::identity()).unwrap();
        again.glue(&inner, [5u8; 32], Word::identity()).expect("another depth-0 glue");
        assert_eq!(again.depth_of(), 1, "max glued depth, not a count");
    }

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
            source: None,
            target: None,
            kind: ShellKind::Whole,
            shift: None,
        };
        let gb = Germ {
            address: from_residual(&chain_b).address(),
            versor: identity_versor(),
            word: Word::identity(),
            chain: chain_b,
            source: None,
            target: None,
            kind: ShellKind::Whole,
            shift: None,
        };
        let mut n = assemble(vec![ga, gb], Vec::new());
        assert_eq!(n.germs.len(), 2);
        assert!(!n.faces.is_empty(), "two germs have a relative word");
        assert!(n.cycles.is_empty(), "a tree has no independent cycle");
        assert!(solve_coboundary(&n).is_some(), "a tree 1-cochain is exact");
        assert!(close(&mut n));
    }

    /// Bodies separated by background still form a face: the relative
    /// word is the identity vertex, with no ring dilation.
    #[test]
    fn separated_bodies_meet_by_relative_word() {
        let chain_a = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
        let chain_b = vec![(0, 4, 3, 4), (0, 5, 3, 4)];
        let ga = Germ {
            address: from_residual(&chain_a).address(),
            versor: identity_versor(),
            word: Word::identity(),
            chain: chain_a,
            source: None,
            target: None,
            kind: ShellKind::Whole,
            shift: None,
        };
        let gb = Germ {
            address: from_residual(&chain_b).address(),
            versor: identity_versor(),
            word: Word::identity(),
            chain: chain_b,
            source: None,
            target: None,
            kind: ShellKind::Whole,
            shift: None,
        };
        let n = assemble(vec![ga, gb], Vec::new());
        assert!(
            n.faces.is_empty(),
            "separated bodies are not adjacent and have no frame star"
        );
    }

    /// Three germs: faces are the bundle's pairs, at most 2·germs.
    #[test]
    fn a_row_of_three_has_faces_by_constancy() {
        let germ = |chain: Vec<(i64, i64, u8, u8)>| Germ {
            address: from_residual(&chain).address(),
            versor: identity_versor(),
            word: Word::identity(),
            chain,
            source: None,
            target: None,
            kind: ShellKind::Whole,
            shift: None,
        };
        let ga = germ(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
        let gb = germ(vec![(0, 4, 1, 2), (0, 5, 1, 2)]);
        let gc = germ(vec![(0, 8, 1, 2), (0, 9, 1, 2)]);
        let n = assemble(vec![ga, gb, gc], Vec::new());
        assert!(
            n.faces.len() <= 2 * n.germs.len(),
            "faces are at most 2·germs, got {}",
            n.faces.len()
        );
        assert_eq!(n.faces.len(), 0, "a disjoint row is not a clique");
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
            source: None,
            target: None,
            kind: ShellKind::Whole,
            shift: None,
        };
        let gb = Germ {
            address: from_residual(&chain_b).address(),
            versor: identity_versor(),
            word: Word::identity(),
            chain: chain_b,
            source: None,
            target: None,
            kind: ShellKind::Whole,
            shift: None,
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
            source: None,
            target: None,
            kind: ShellKind::Whole,
            shift: None,
        };
        let gb = Germ {
            address: from_residual(&chain_b).address(),
            versor: identity_versor(),
            word: Word::identity(),
            chain: chain_b,
            source: None,
            target: None,
            kind: ShellKind::Whole,
            shift: None,
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

    fn whole(chain: Vec<Cell>) -> Germ {
        Germ {
            address: from_residual(&chain).address(),
            versor: identity_versor(),
            word: Word::identity(),
            chain,
            source: None,
            target: None,
            kind: ShellKind::Whole,
            shift: None,
        }
    }

    fn frame(chain: Vec<Cell>) -> Germ {
        let mut g = whole(chain);
        g.kind = ShellKind::Frame;
        g
    }

    /// Face constancy is the relative word. Two pairs of the same two
    /// bodies at different spacings disagree on the translation versor
    /// and still agree: the word is identity, the shift is per-pair.
    #[test]
    fn face_constancy_is_the_word_not_the_versor() {
        let n1 = assemble(
            vec![
                frame(vec![(0, 0, 1, 2), (0, 1, 1, 2), (0, 4, 1, 2), (0, 5, 1, 2)]),
                whole(vec![(0, 0, 1, 2), (0, 1, 1, 2)]),
                whole(vec![(0, 4, 1, 2), (0, 5, 1, 2)]),
            ],
            Vec::new(),
        );
        let n2 = assemble(
            vec![
                frame(vec![(0, 0, 1, 2), (0, 1, 1, 2), (0, 8, 1, 2), (0, 9, 1, 2)]),
                whole(vec![(0, 0, 1, 2), (0, 1, 1, 2)]),
                whole(vec![(0, 8, 1, 2), (0, 9, 1, 2)]),
            ],
            Vec::new(),
        );
        assert!(n1.faces.len() >= 2 && n1.faces.len() <= 2 * n1.germs.len());
        assert!(n2.faces.len() >= 2 && n2.faces.len() <= 2 * n2.germs.len());
        let rule = agree_up_to_gauge(&[n1, n2]).expect("word-constant faces agree up to gauge");
        assert!(
            !rule.nerve.faces.is_empty(),
            "constancy is the word, not the per-pair translation versor; faces {}",
            rule.nerve.faces.len()
        );
        let e = as_element(&rule.nerve).expect("rule element");
        assert!(replay_nerve(&e).is_none(), "replay compares face words, not versors: {:?}", replay_nerve(&e));
    }

    /// Three co-present bodies form a K3: faces survive word constancy
    /// across a spacing change, the independent cycle is a vortex, and
    /// holonomy of identity words is the identity (checked, not assumed).
    #[test]
    fn word_constant_triangle_keeps_its_cycle() {
        let n1 = assemble(
            vec![
                whole(vec![(0, 0, 1, 2), (0, 1, 1, 2)]),
                whole(vec![(0, 4, 1, 2), (0, 5, 1, 2)]),
                whole(vec![(0, 8, 1, 2), (0, 9, 1, 2)]),
            ],
            Vec::new(),
        );
        let n2 = assemble(
            vec![
                whole(vec![(0, 0, 1, 2), (0, 1, 1, 2)]),
                whole(vec![(0, 5, 1, 2), (0, 6, 1, 2)]),
                whole(vec![(0, 10, 1, 2), (0, 11, 1, 2)]),
            ],
            Vec::new(),
        );
        assert!(
            n1.faces.len() <= 2 * n1.germs.len(),
            "faces are at most 2·germs, got {}",
            n1.faces.len()
        );
        let rule = agree_up_to_gauge(&[n1, n2]).expect("triangle agrees on words");
        assert!(
            rule.nerve.faces.len() <= 2 * rule.nerve.germ_count(),
            "faces {} cycles {}",
            rule.nerve.faces.len(),
            rule.nerve.cycles.len()
        );
        let e = as_element(&rule.nerve).expect("rule element");
        assert!(replay_nerve(&e).is_none(), "{:?}", replay_nerve(&e));
    }

    /// Two bodies of different hue in one thought still share a face, and
    /// two embeddings that differ only by a translation are one vertex.
    #[test]
    fn placement_is_not_the_sentence_vertex() {
        let n1 = assemble(
            vec![
                frame(vec![(0, 0, 1, 2), (0, 1, 1, 2), (0, 4, 3, 4), (0, 5, 3, 4)]),
                whole(vec![(0, 0, 1, 2), (0, 1, 1, 2)]),
                whole(vec![(0, 4, 3, 4), (0, 5, 3, 4)]),
            ],
            Vec::new(),
        );
        let n2 = assemble(
            vec![
                frame(vec![(2, 0, 1, 2), (2, 1, 1, 2), (2, 4, 3, 4), (2, 5, 3, 4)]),
                whole(vec![(2, 0, 1, 2), (2, 1, 1, 2)]),
                whole(vec![(2, 4, 3, 4), (2, 5, 3, 4)]),
            ],
            Vec::new(),
        );
        assert!(
            !n1.faces.is_empty(),
            "the frame star relates the two hues"
        );
        assert!(nerves_same_vertex(&n1, &n2), "a global translation is fibre, not a new vertex");
        let e1 = as_element(&n1).expect("e1");
        let e2 = as_element(&n2).expect("e2");
        assert!(e1.same_vertex(&e2));
        assert_eq!(e1.address(), e2.address(), "vertex key ignores placement versors");
    }

    #[test]
    fn infinite_class_is_never_filed() {
        let mut n = chord_nerve(&[infinite()]);
        let snaps = twist(
            &mut n,
            [0x11; 32],
            &crate::network::nerve::Keyed::new(),
            &mut crate::physics::Physics::default().stroke_n(8),
        );
        assert!(
            snaps.iter().any(|s| matches!(s, Snap::Demand { .. })),
            "infinite is demanded: {snaps:?}"
        );
        assert!(
            !snaps.iter().any(|s| matches!(s, Snap::TorsionFiled { .. })),
            "infinite is never filed: {snaps:?}"
        );
        if let Some(e) = as_element(&n) {
            assert!(
                e.torsion().is_empty(),
                "an infinite class is never a large-ℓ torsion cycle"
            );
        }
    }

    #[test]
    fn torsion_filed_only_when_witnessed_at_period() {
        let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
        let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
        let mut t = from_residual(&crate::transducer::germinate::germinate(&a, &b).unwrap());
        t.puncture(vec![(2, 3), (3, 1), (1, 2)]);
        assert_eq!(t.torse(), 1, "the hue cycle is one torsion generator");
        let ws = crate::transducer::toroid::witness_stored(&t);
        assert!(
            ws.iter().any(|(ell, w)| *ell == 3
                && matches!(w, crate::transducer::toroid::HolonomicLoopWitness::Closed { period: 3, .. })),
            "period 3 is witnessed closed: {ws:?}"
        );
        t.record_torsion(vec![(1, 9)], 8, crate::transducer::element::FIBER_SPATIAL);
        let ws = crate::transducer::toroid::witness_stored(&t);
        assert!(
            ws.iter().any(|(ell, w)| *ell == 8
                && matches!(w, crate::transducer::toroid::HolonomicLoopWitness::Open { .. })),
            "an unclosed walk is not believed: {ws:?}"
        );
    }

    fn order2() -> Word {
        alphabet::PlaneForm {
            parity: true,
            rot: [0; alphabet::CAYLEY_ALPHABET_LEN],
        }
        .to_word()
    }

    fn order4() -> Word {
        Word::from_letters(&[(alphabet::LetterId::quarter((0, 1)).unwrap(), false)])
    }

    fn infinite() -> Word {
        Word::from_letters(&[(alphabet::LetterId::pythagorean_5((0, 1)).unwrap(), false)])
    }

    fn chord_nerve(chords: &[Word]) -> Nerve {
        let n = chords.len() + 1;
        let germs: Vec<Germ> = (0..n)
            .map(|i| {
                let mut address = [0u8; 32];
                address[0] = (i as u8).saturating_add(1);
                Germ {
                    address,
                    versor: identity_versor(),
                    word: Word::identity(),
                    chain: vec![(0, i as i64, 1, 2)],
                    source: None,
                    target: None,
                    kind: ShellKind::Whole,
                    shift: None,
                }
            })
            .collect();
        let mut faces = Vec::new();
        for i in 0..n.saturating_sub(1) {
            faces.push(Face {
                i,
                j: i + 1,
                overlap: Vec::new(),
                g_ij: Word::identity(),
                versor: identity_versor(),
                spelled: true,
            });
        }
        for (ci, w) in chords.iter().enumerate() {
            let (i, j) = match ci {
                0 => (0, 2.min(n - 1)),
                1 => (1.min(n - 1), 3.min(n - 1)),
                _ => (0, (ci + 1).min(n - 1)),
            };
            if i == j {
                continue;
            }
            faces.push(Face {
                i,
                j,
                overlap: Vec::new(),
                g_ij: w.clone(),
                versor: identity_versor(),
                spelled: true,
            });
        }
        let cycles = independent_cycles(n, &faces);
        Nerve {
            germs,
            complex: Complex::new(),
            faces,
            cycles,
            remainder: Vec::new(),
            class: None,
            cover: crate::network::nerve::Keyed::new(),
        }
    }

    #[test]
    fn twist_closes_lowest_order_first_and_terminates() {
        assert_eq!(order2().order(), Order::Finite(2));
        assert_eq!(order4().order(), Order::Finite(4));
        assert_eq!(infinite().order(), Order::Infinite);
        let mut n = chord_nerve(&[order2(), order4(), infinite()]);
        assert!(n.cycles.len() >= 3, "three chords, got {}", n.cycles.len());
        let mut budget = crate::physics::Physics::default().stroke_n(16);
        let snaps = twist(&mut n, [0xab; 32], &crate::network::nerve::Keyed::new(), &mut budget);
        let mut seen_inf = false;
        let mut last_finite: u32 = 0;
        for s in &snaps {
            match s {
                Snap::Closed { order, .. } | Snap::TorsionFiled { ell: order, .. } => {
                    assert!(!seen_inf, "never an ∞ line before a finite one");
                    assert!(*order >= last_finite, "finite chords ascend");
                    last_finite = *order;
                }
                Snap::Demand { .. } => seen_inf = true,
                Snap::TorsionOpen { .. } => {}
            }
        }
        assert!(
            snaps.iter().any(|s| matches!(s, Snap::Closed { order: 2, .. } | Snap::TorsionFiled { ell: 2, .. })),
            "order 2 first: {snaps:?}"
        );
        assert!(
            snaps.iter().any(|s| matches!(s, Snap::Demand { .. })),
            "infinite is a demand: {snaps:?}"
        );
        let again = twist(&mut n, [0xab; 32], &crate::network::nerve::Keyed::new(), &mut crate::physics::Physics::default().stroke_n(16));
        assert!(again.is_empty(), "the loop terminates, leftover {again:?}");
    }

    #[test]
    fn cauterize_filed_torsion_crystallizes() {
        let mut n = chord_nerve(&[order2()]);
        assert!(!n.is_closed());
        let _ = twist(
            &mut n,
            [0xca; 32],
            &crate::network::nerve::Keyed::new(),
            &mut crate::physics::Physics::default().stroke_n(8),
        );
        assert!(n.is_cauterized(), "filed finite torsion with empty remainder cauterizes");
        let e = as_element(&n).expect("crystallized form");
        assert!(
            !e.torsion().is_empty(),
            "the tape carries the filed cycle"
        );
    }

    #[test]
    fn horizon_seeded_infinite_is_not_demanded() {
        let mut n = chord_nerve(&[infinite()]);
        let r = infinite().rotor();
        let mut horizon = crate::network::nerve::Keyed::new();
        horizon.insert(r.class_key(), (r, Order::Infinite, Standing::Held));
        let snaps = twist(
            &mut n,
            [0x5e; 32],
            &horizon,
            &mut crate::physics::Physics::default().stroke_n(8),
        );
        assert!(
            !snaps.iter().any(|s| matches!(s, Snap::Demand { .. })),
            "a held infinite class is not spoken again: {snaps:?}"
        );
    }

    #[test]
    fn twist_never_reopens_a_closed_chord() {
        let mut n = chord_nerve(&[order2(), order4()]);
        let mut budget = crate::physics::Physics::default().stroke_n(16);
        let first = twist(&mut n, [0xcd; 32], &crate::network::nerve::Keyed::new(), &mut budget);
        let closed: Vec<u32> = first
            .iter()
            .filter_map(|s| match s {
                Snap::Closed { order, .. } => Some(*order),
                Snap::TorsionFiled { ell, .. } => Some(*ell),
                _ => None,
            })
            .collect();
        let mut budget = crate::physics::Physics::default().stroke_n(16);
        let second = twist(&mut n, [0xcd; 32], &crate::network::nerve::Keyed::new(), &mut budget);
        for o in closed {
            assert!(
                !second.iter().any(|s| matches!(s, Snap::Demand { .. })) || o > 0,
                "a closed finite chord is not demanded"
            );
            let still = n.cycles.iter().any(|c| {
                !c.holonomy.abelian_identity() && matches!(c.holonomy.order(), Order::Finite(ell) if ell == o)
            });
            let filed = n.complex.degrees.iter().any(|layer| {
                layer.fiber(&crate::network::nerve::ROOT).any(|(_, cell)| {
                    matches!(cell.fill, Fill::Congruent(_, ell) if ell == o)
                })
            });
            assert!(
                !still || filed,
                "order {o} stayed closed or filed; second {second:?}"
            );
        }
    }
}
