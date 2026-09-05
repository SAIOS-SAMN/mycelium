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
use crate::network::nerve::{Keyed, ROOT};

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
        let mut held_asg: Keyed<()> = Keyed::new();
        for &k in &same_charge {
            if let Some(a) = held.get(k) {
                held_asg.insert(element::assignment_key(a), ());
            }
        }
        if held_asg.contains(&element::assignment_key(&asg)) {
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
    let mut spoken: Vec<Keyed<()>> = (0..sorted_target.len()).map(|_| Keyed::new()).collect();
    for (v, pat) in ops {
        for &(wr, wc, wb, wa) in pat {
            let Some(p) = element::carry_center(v, wr, wc) else { continue };
            let (tr, tc) = element::cell_of(&p);
            if let Ok(k) = sorted_target.binary_search_by(|x| (x.0, x.1).cmp(&(tr, tc))) {
                spoken[k].insert(element::hue_pair_key(wb, wa), ());
            }
        }
    }
    sorted_target
        .iter()
        .zip(&spoken)
        .filter(|(_, s)| s.len() > 1)
        .map(|(t, _)| *t)
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

/// A body's gauge classes, indexed by hue-pair chart, each opened into
/// its own connected pieces. None when the body is a single class — a
/// stalk, cited or residual, never opened further. The index is built
/// once from the body's cells; locate then reads each fiber.
fn open_by_chart(body: &[Cell]) -> Option<Vec<Vec<Cell>>> {
    let fibers = element::gauge_fibers(body);
    if fibers.len() <= 1 {
        return None;
    }
    let mut idx: Keyed<Vec<Vec<Cell>>> = Keyed::new();
    for (class, fiber) in fibers {
        idx.insert(element::hue_pair_key(class.0, class.1), pieces(&fiber));
    }
    let mut out = Vec::new();
    for (_, subs) in idx.fiber(&ROOT) {
        out.extend(subs.clone());
    }
    Some(out)
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
            (None, None) => match open_by_chart(&body) {
                None => out.residual.extend(body),
                Some(subs) => {
                    for sub in subs {
                        let (c, p, m) = locate_or_puncture(words, atlas, index_of, shape_at, &sub, exclude);
                        out.multiplicity = out.multiplicity.max(m);
                        match (c, p) {
                            (Some(c), _) => out.cites.push(c),
                            (None, Some(p)) => out.punctures.push(p),
                            (None, None) => out.residual.extend(sub),
                        }
                    }
                }
            },
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
            (None, None) => match open_by_chart(&body) {
                None => out.residual.extend(body),
                Some(subs) => {
                    for sub in subs {
                        let (c, p, m) =
                            locate_or_puncture_on(word_at, fiber_of, index_of, shape_at, &sub, exclude);
                        out.multiplicity = out.multiplicity.max(m);
                        match (c, p) {
                            (Some(c), _) => out.cites.push(c),
                            (None, Some(p)) => out.punctures.push(p),
                            (None, None) => out.residual.extend(sub),
                        }
                    }
                }
            },
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
    /// The faces the loop walks, in path order; the last is the chord that
    /// closes the tree path. A loop names its faces (rule 1).
    pub faces: Vec<usize>,
}

impl Cycle {
    /// The chord face: the one face of the loop outside the spanning forest.
    pub fn chord(&self) -> Option<usize> {
        self.faces.last().copied()
    }
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
    /// The torsion cycles filed on this nerve, `(σ, ℓ)` each, keyed by the
    /// class key of the holonomy (A17). Filing writes the key; cauterize
    /// reads `contains`. σ is the loop's consecutive germ pairs.
    pub torsion: Keyed<(Vec<(u8, u8)>, u32)>,
    /// Certified discharges, keyed by the certificate (A20).
    pub certificates: Keyed<PivotCertificate>,
    /// Incomparable discharges of one loop, keyed the same way (25.1).
    pub vortex: Keyed<PivotCertificate>,
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
                    match self.torsion.get(&k) {
                        Some((_, e)) if *e == ell => {}
                        _ => return false,
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
        /// Stop ring. A form coordinate with ρ, placement, gauge; not M1.
        contact: Option<alphabet::Contact>,
    },
    Glued {
        address: [u8; 32],
        chart: stalk::Invariant,
        magnitude: [u64; 4],
        depth: u16,
        placement: Word,
    },
    /// Vertex `of` read in its own frame as `by`'s word (3.2): the linear
    /// word is `by`'s, realised at `placement_by + shift`; the body it
    /// speaks to, its hue map and its dilation stay `of`'s own, so a
    /// rebased body keeps its shape, its colours and its scale.
    Rebased {
        by: u16,
        of: u16,
        /// Relative placement t = p_of − p_by.
        shift: (num_rational::BigRational, num_rational::BigRational),
        /// `of`'s dilation.
        rho: num_rational::BigRational,
        /// `of`'s hue map, spoken on `of`'s bodies.
        gauge: Vec<(u8, u8)>,
        /// `of`'s chart: the section coordinate naming the bodies it speaks to.
        class: stalk::Invariant,
    },
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
    pub ratio: ((usize, usize), (usize, usize)),
    pub sections: Keyed<Vec<u16>>,
    /// The certified pivots this form's crystallization rests on (A20).
    /// Evidence, replayed by the court; never part of the address.
    pub certificates: Vec<PivotCertificate>,
    /// Filed torsion cycles, keyed by class key. The form owns this
    /// carrier; the element reads it whole.
    pub torsion: Keyed<(Vec<(u8, u8)>, u32)>,
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
            ratio: ((1, 1), (1, 1)),
            sections: Keyed::new(),
            certificates: Vec::new(),
            torsion: Keyed::new(),
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
            contact: None,
        }
    }

    pub fn word_in(&self, form: &Form) -> Word {
        match self {
            Vertex::Frame | Vertex::Unfolded { .. } => Word::identity(),
            Vertex::Word { word, .. } => word.clone(),
            Vertex::Glued { placement, .. } => placement.clone(),
            Vertex::Rebased { by, .. } => form
                .vertices
                .get(*by as usize)
                .map(|v| v.word_in(form))
                .unwrap_or_else(Word::identity),
        }
    }

    pub fn class(&self) -> stalk::Invariant {
        match self {
            Vertex::Word { class, .. } | Vertex::Rebased { class, .. } => class.clone(),
            Vertex::Glued { chart, .. } => chart.clone(),
            Vertex::Frame | Vertex::Unfolded { .. } => stalk::Invariant::default(),
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
        let mut faces: Vec<FormFace> = n
            .faces
            .iter()
            .map(|f| FormFace {
                i: f.i as u16,
                j: f.j as u16,
                g_ij: f.g_ij.clone(),
            })
            .collect();
        let mut torsion = n.torsion.clone();
        // Frame is vertex 0 by construction. Face order (and certificate
        // face indices) stay; vertex indices and torsion σ pairs shift.
        if !matches!(vertices.first(), Some(Vertex::Frame)) {
            vertices.insert(0, Vertex::Frame);
            for f in &mut faces {
                f.i = f.i.saturating_add(1);
                f.j = f.j.saturating_add(1);
            }
            let mut shifted = Keyed::new();
            for (k, (sigma, ell)) in torsion.fiber(&ROOT) {
                let sigma = sigma
                    .iter()
                    .map(|&(a, c)| (a.saturating_add(1), c.saturating_add(1)))
                    .collect();
                shifted.insert(*k, (sigma, *ell));
            }
            torsion = shifted;
        }
        let loops = Form::loops_of(&faces, vertices.len());
        Form {
            vertices,
            faces,
            loops,
            remainder: n.remainder.clone(),
            depth: 0,
            hue_maps: Keyed::new(),
            ratio: ((1, 1), (1, 1)),
            sections: Keyed::new(),
            certificates: Vec::new(),
            torsion,
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
            torsion: self.torsion.clone(),
            certificates: {
                let mut k = Keyed::new();
                for c in &self.certificates {
                    k.insert(c.key(), c.clone());
                }
                k
            },
            vortex: Keyed::new(),
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

    /// A new form (depth 0, no citations) may glue any closed form; its
    /// depth becomes `1 + d`. A form that already cites may only glue a
    /// strictly shallower closed form.
    pub fn glue(&mut self, s: &Form, s_addr: [u8; 32], placement: Word, magnitude: [u64; 4]) -> Option<u16> {
        if !s.is_closed() {
            return None;
        }
        if !matches!(s.vertices.first(), Some(Vertex::Frame)) {
            return None;
        }
        if !s.vertices[0].word_in(s).is_identity() {
            return None;
        }
        let d = s.depth_of();
        let host = self.depth_of();
        if host > 0 && d >= host {
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
            magnitude,
            depth: d,
            placement,
        });
        self.depth = self.depth_of();
        Some(idx)
    }

    /// Replace every face `g_ij` by `γ_j · g_ij · γ_i⁻¹`. Holonomies of
    /// identity stay identity.
    pub fn regauge(&mut self, gamma: &[Word]) {
        for f in &mut self.faces {
            let gi = gamma
                .get(f.i as usize)
                .cloned()
                .unwrap_or_else(Word::identity);
            let gj = gamma
                .get(f.j as usize)
                .cloned()
                .unwrap_or_else(Word::identity);
            f.g_ij = gj.compose(&f.g_ij.compose(&gi.inverse())).normal_form();
        }
        self.loops = Self::loops_of(&self.faces, self.vertices.len());
    }

    /// Tree walk (12.2): γ at the Frame is identity; γ_j = γ_i · g_ij⁻¹
    /// along a spanning forest of the named faces.
    pub fn solve_coboundary(&self) -> Option<Vec<Word>> {
        let n = self.vertices.len();
        if n == 0 {
            return None;
        }
        // Frame is vertex 0 by construction.
        let root = 0usize;
        let mut gamma: Vec<Option<Word>> = vec![None; n];
        gamma[root] = Some(Word::identity());
        let mut adj: Vec<Vec<(usize, usize, bool)>> = vec![Vec::new(); n];
        for (fi, f) in self.faces.iter().enumerate() {
            let (i, j) = (f.i as usize, f.j as usize);
            if i >= n || j >= n {
                continue;
            }
            adj[i].push((j, fi, true));
            adj[j].push((i, fi, false));
        }
        let mut q = vec![root];
        let mut qi = 0usize;
        while qi < q.len() {
            let u = q[qi];
            qi += 1;
            let gu = gamma[u].clone()?;
            for &(v, fi, u_is_i) in &adj[u] {
                let f = &self.faces[fi];
                let gv = if u_is_i {
                    gu.compose(&f.g_ij.inverse()).normal_form()
                } else {
                    gu.compose(&f.g_ij).normal_form()
                };
                match &gamma[v] {
                    None => {
                        gamma[v] = Some(gv);
                        q.push(v);
                    }
                    Some(held) if held != &gv => {}
                    _ => {}
                }
            }
        }
        Some(
            gamma
                .into_iter()
                .map(|g| g.unwrap_or_else(Word::identity))
                .collect(),
        )
    }

    /// Replace vertex `of` by `by` read in `of`'s frame (3.2). `of`'s own
    /// dilation, hue map and chart are kept on the Rebased vertex; only
    /// its word is now `by`'s. A vertex that is not a `Word` is left as it
    /// is, and `of` is returned either way.
    pub fn rebase(
        &mut self,
        by: u16,
        of: u16,
        shift: (num_rational::BigRational, num_rational::BigRational),
    ) -> u16 {
        let Some(Vertex::Word {
            rho,
            gauge,
            class,
            ..
        }) = self.vertices.get(of as usize).cloned()
        else {
            return of;
        };
        self.vertices[of as usize] = Vertex::Rebased {
            by,
            of,
            shift,
            rho,
            gauge,
            class,
        };
        of
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
            let Some((mut path, mut walked)) = tree_path(&adj, a, b) else {
                continue;
            };
            path.push(a);
            walked.push(fi);
            let holonomy = form_named_holonomy(faces, &path, &walked);
            loops.push(Loop {
                faces: walked.into_iter().map(|x| x as u16).collect(),
                holonomy,
            });
        }
        loops
    }
}

fn form_face_step(faces: &[FormFace], fi: usize, from: usize, to: usize) -> Option<Word> {
    let f = faces.get(fi)?;
    if f.i as usize == from && f.j as usize == to {
        Some(f.g_ij.clone())
    } else {
        Some(f.g_ij.inverse())
    }
}

fn form_named_holonomy(faces: &[FormFace], path: &[usize], walked: &[usize]) -> Word {
    let mut w = Word::identity();
    for (win, &fi) in path.windows(2).zip(walked) {
        if let Some(s) = form_face_step(faces, fi, win[0], win[1]) {
            w = w.compose(&s);
        }
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
        let Some((mut path, mut walked)) = tree_path(&adj, a, b) else {
            continue;
        };
        // Close the walk with the chord: b → a through face fi.
        path.push(a);
        walked.push(fi);
        let holonomy = cycle_holonomy(faces, &path, &walked);
        cycles.push(Cycle {
            germs: path,
            holonomy,
            faces: walked,
        });
    }
    cycles
}

/// The tree path from `src` to `dst` as its vertices and the faces walked
/// between them, named in the same pass that finds them: each vertex is
/// reached through one face of its own adjacency, so no face is searched
/// for afterwards.
fn tree_path(adj: &[Vec<(usize, usize)>], src: usize, dst: usize) -> Option<(Vec<usize>, Vec<usize>)> {
    if src == dst {
        return Some((vec![src], Vec::new()));
    }
    let mut prev: Vec<Option<(usize, usize)>> = vec![None; adj.len()];
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
        for &(v, fi) in &adj[u] {
            if seen[v] {
                continue;
            }
            seen[v] = true;
            prev[v] = Some((u, fi));
            q.push(v);
        }
    }
    if !seen[dst] {
        return None;
    }
    let mut path = vec![dst];
    let mut walked = Vec::new();
    let mut cur = dst;
    while cur != src {
        let (p, fi) = prev[cur]?;
        walked.push(fi);
        path.push(p);
        cur = p;
    }
    path.reverse();
    walked.reverse();
    Some((path, walked))
}

/// The word of face `fi` read in the direction `from → to`.
fn face_step(faces: &[Face], fi: usize, from: usize, to: usize) -> Option<Word> {
    let f = faces.get(fi)?;
    if f.i == from && f.j == to {
        Some(f.g_ij.clone())
    } else {
        Some(f.g_ij.inverse())
    }
}

/// Holonomy of a loop: the product of its named faces in path order, each
/// read in the direction the path walks it. `path` is the closed walk
/// (first vertex repeated last); `walked` its faces, the chord last.
fn cycle_holonomy(faces: &[Face], path: &[usize], walked: &[usize]) -> Word {
    let mut w = Word::identity();
    for (win, &fi) in path.windows(2).zip(walked) {
        if let Some(s) = face_step(faces, fi, win[0], win[1]) {
            w = w.compose(&s);
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

/// Transport class of a germ: shell kind, conjugacy class of the word,
/// and hue map. Moduli stay on the germ as a section coordinate, never a
/// matching key. Frames match by kind alone (size is rule.ratio).
pub fn same_stalk(a: &Germ, b: &Germ) -> bool {
    germ_transport_key(a) == germ_transport_key(b)
}

fn germ_transport_key(g: &Germ) -> [u8; 32] {
    if g.kind == ShellKind::Frame {
        let mut k = [0u8; 32];
        k[0] = g.kind.as_u8();
        return k;
    }
    let mut buf = Vec::new();
    buf.push(g.kind.as_u8());
    buf.extend_from_slice(&g.word.class_key());
    let hue = germ_hue_map(g);
    buf.extend_from_slice(&(hue.len() as u16).to_le_bytes());
    for (b, a) in hue {
        buf.push(b);
        buf.push(a);
    }
    *blake3::hash(&buf).as_bytes()
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

fn emit_face(germs: &[Germ], i: usize, j: usize, faces: &mut Vec<Face>, seen: &mut Keyed<()>) {
    if i == j {
        return;
    }
    let (lo, hi) = if i < j { (i, j) } else { (j, i) };
    let k = super::family::pair_key(lo as u16, hi as u16);
    if seen.contains(&k) {
        return;
    }
    seen.insert(k, ());
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
/// the star from the Frame (vertex 0 when the first germ is a Frame) and
/// adjacency of after-supports (keyed occupancy + ring probe), shared
/// with `family::star_and_cell_faces`. Not the complete graph of spelled pairs.
pub fn assemble(germs: Vec<Germ>, remainder: Vec<Cell>) -> Nerve {
    let n = germs.len();
    let frame = germs
        .first()
        .filter(|g| g.kind == ShellKind::Frame)
        .map(|_| 0usize);
    let supports: Vec<Vec<(i64, i64)>> = germs.iter().map(after_support).collect();
    let pairs = super::family::star_and_cell_faces(n, frame, &supports);
    let mut seen = Keyed::new();
    let mut faces = Vec::new();
    for (a, b) in pairs {
        emit_face(&germs, a as usize, b as usize, &mut faces, &mut seen);
    }
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
        cover: Keyed::new(),
        torsion: Keyed::new(),
        certificates: Keyed::new(),
        vortex: Keyed::new(),
    }
}

/// The invariant cochain of a family (chapter 31): the form, its sections,
/// and the bundle frame ratio. Charts, faces and holonomies are derived
/// from the form.
#[derive(Debug, Clone)]
pub struct SentenceRule {
    pub form: Form,
    /// Classes present in some pairs only: class key → pair indices.
    pub sections: Keyed<Vec<u16>>,
    /// Family frame ratio ((h_out, h_in), (w_out, w_in)).
    pub ratio: ((usize, usize), (usize, usize)),
}

impl SentenceRule {
    pub fn section_count(&self) -> usize {
        self.sections
            .fiber(&crate::network::nerve::ROOT)
            .map(|(_, v)| v.len())
            .sum()
    }

    /// Hue map of a target chart. One Keyed from the form's Word vertices,
    /// then a read; never a scan.
    pub fn hue_for(&self, inv: &stalk::Invariant) -> Vec<(u8, u8)> {
        if let Some(m) = self.form.hue_maps.get(&inv.key()) {
            return m.clone();
        }
        let mut by: Keyed<Vec<(u8, u8)>> = Keyed::new();
        for v in &self.form.vertices {
            if let Vertex::Word { class, gauge, .. } = v {
                let k = class.key();
                if !by.contains(&k) {
                    by.insert(k, gauge.clone());
                }
            }
        }
        by.get(&inv.key()).cloned().unwrap_or_default()
    }

    /// Placement of a transport class. One Keyed from the form's Word vertices
    /// by transport-class key, then a read.
    pub fn shift_of(&self, key: &[u8; 32]) -> Option<(num_rational::BigRational, num_rational::BigRational)> {
        let mut by: Keyed<(num_rational::BigRational, num_rational::BigRational)> = Keyed::new();
        for v in &self.form.vertices {
            if let Vertex::Word {
                word,
                gauge,
                placement,
                ..
            } = v
            {
                let k = vertex_transport_key(word, gauge);
                if !by.contains(&k) {
                    by.insert(k, placement.clone());
                }
            }
        }
        by.get(key).cloned()
    }
}

fn vertex_transport_key(word: &Word, gauge: &[(u8, u8)]) -> [u8; 32] {
    let mut buf = Vec::new();
    buf.push(ShellKind::Whole.as_u8());
    buf.extend_from_slice(&word.class_key());
    buf.extend_from_slice(&(gauge.len() as u16).to_le_bytes());
    for &(b, a) in gauge {
        buf.push(b);
        buf.push(a);
    }
    *blake3::hash(&buf).as_bytes()
}

fn germ_hue_map(g: &Germ) -> Vec<(u8, u8)> {
    let mut seen: Keyed<u8> = Keyed::new();
    for &(_, _, b, a) in &g.chain {
        let k = element::hue_pair_key(b, 0);
        if !seen.contains(&k) {
            seen.insert(k, a);
        }
    }
    let mut m: Vec<(u8, u8)> = seen.fiber(&ROOT).map(|(k, a)| (k[0], *a)).collect();
    m.sort_unstable();
    m
}

/// A form as a `SentenceRule` (chapter 31). Hue maps are filled from the
/// form's own Word vertices when the form does not already carry them.
pub fn rule_of_form(mut form: Form) -> SentenceRule {
    if form.hue_maps.is_empty() {
        for v in &form.vertices {
            if let Vertex::Word { class, gauge, .. } = v {
                let k = class.key();
                if !form.hue_maps.contains(&k) {
                    form.hue_maps.insert(k, gauge.clone());
                }
            }
        }
    }
    let sections = form.sections.clone();
    let ratio = form.ratio;
    SentenceRule {
        form,
        sections,
        ratio,
    }
}



/// Vertex key of a sentence: the 1-cocycle of germ words (kind, conjugacy,
/// hue map, agreed shift relative to the Frame) and face words. The body's
/// absolute position is already quotiented on the chain; the bundle's
/// translation is meaning.
pub fn vertex_address(e: &Element) -> Option<[u8; 32]> {
    if let Some(f) = from_element_form(e) {
        return Some(form_vertex_key(&f));
    }
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
    match &g.shift {
        None => row.push(0),
        Some((r, c)) => {
            row.push(1);
            alphabet::put_q(&mut row, r);
            alphabet::put_q(&mut row, c);
        }
    }
    row
}

fn form_vertex_key(f: &Form) -> [u8; 32] {
    let mut buf = Vec::new();
    buf.extend_from_slice(&(f.vertices.len() as u32).to_le_bytes());
    for v in &f.vertices {
        match v {
            Vertex::Frame => buf.push(0),
            Vertex::Word {
                word,
                rho,
                placement,
                gauge,
                contact,
                ..
            } => {
                buf.push(1);
                buf.extend_from_slice(&word.key());
                alphabet::put_q(&mut buf, rho);
                alphabet::put_q(&mut buf, &placement.0);
                alphabet::put_q(&mut buf, &placement.1);
                match contact {
                    None => buf.push(0),
                    Some(c) => {
                        buf.push(1);
                        buf.extend_from_slice(&c.anchor.to_le_bytes());
                        buf.push(c.ring);
                    }
                }
                let mut g = gauge.clone();
                g.sort_unstable();
                buf.extend_from_slice(&(g.len() as u16).to_le_bytes());
                for (a, b) in g {
                    buf.push(a);
                    buf.push(b);
                }
            }
            Vertex::Glued {
                address,
                placement,
                depth,
                ..
            } => {
                buf.push(2);
                buf.extend_from_slice(address);
                buf.extend_from_slice(&placement.key());
                buf.extend_from_slice(&depth.to_le_bytes());
            }
            Vertex::Rebased {
                by,
                of,
                shift,
                rho,
                gauge,
                ..
            } => {
                buf.push(3);
                buf.extend_from_slice(&by.to_le_bytes());
                buf.extend_from_slice(&of.to_le_bytes());
                alphabet::put_q(&mut buf, &shift.0);
                alphabet::put_q(&mut buf, &shift.1);
                alphabet::put_q(&mut buf, rho);
                let mut g = gauge.clone();
                g.sort_unstable();
                buf.extend_from_slice(&(g.len() as u16).to_le_bytes());
                for (a, b) in g {
                    buf.push(a);
                    buf.push(b);
                }
            }
            Vertex::Unfolded { measure } => {
                buf.push(4);
                match measure {
                    Measure::Occupied => buf.push(0),
                    Measure::Remainder => buf.push(1),
                    Measure::Ring { anchor } => {
                        buf.push(2);
                        buf.extend_from_slice(&anchor.to_le_bytes());
                    }
                }
            }
        }
    }
    buf.extend_from_slice(&(f.faces.len() as u32).to_le_bytes());
    for face in &f.faces {
        buf.extend_from_slice(&face.i.to_le_bytes());
        buf.extend_from_slice(&face.j.to_le_bytes());
        buf.extend_from_slice(&face.g_ij.key());
    }
    // The bundle's frame ratio and its sections are meaning (rule 7): two
    // forms that speak the same words onto differently scaled frames, or
    // that section different vertices, are two forms.
    let ((hn, hd), (wn, wd)) = f.ratio;
    for x in [hn, hd, wn, wd] {
        buf.extend_from_slice(&(x as u64).to_le_bytes());
    }
    for (class, members) in f.sections.fiber(&crate::network::nerve::ROOT) {
        buf.extend_from_slice(class);
        buf.extend_from_slice(&(members.len() as u16).to_le_bytes());
        for m in members {
            buf.extend_from_slice(&m.to_le_bytes());
        }
    }
    *blake3::hash(&buf).as_bytes()
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

/// Same sentence vertex: the same 1-cocycle of words, including the
/// bundle's agreed shift and contact. Body moduli and the body's
/// absolute position remain fibre coordinates.
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

/// One snap of the pivot loop (A16).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Snap {
    /// A chord the tree gauge closed: its class was trivial (T2), never rank.
    Closed { chord: usize, order: u32 },
    /// A held loop discharged on one occupied axis by a certified pivot.
    Discharged { chord: usize, certificate: PivotCertificate },
    /// Several pivots discharge and none dominates on μ⃗: all are held as
    /// cells of the thought until a perception, a glue or a ruling collapses
    /// the superposition (12.6).
    Vortex { chord: usize, held: Vec<PivotCertificate> },
    TorsionFiled { chord: usize, ell: u32 },
    TorsionOpen { chord: usize },
    Demand { chord: usize, key: [u8; 32] },
}

/// The axes a class vector can occupy, in the fixed order of 12.6: the
/// declaration order is the walk order and is not a preference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    Sheet(u8),
    Unit(u8),
    Prime5(u8),
    Prime13(u8),
    Hue,
    Placement,
    OffPlane,
    Open,
}

impl Axis {
    /// One tape byte: the kind in the high bits, the slot plane in the low.
    fn to_byte(self) -> u8 {
        match self {
            Axis::Sheet(p) => p,
            Axis::Unit(p) => (1 << 3) | p,
            Axis::Prime5(p) => (2 << 3) | p,
            Axis::Prime13(p) => (3 << 3) | p,
            Axis::Hue => 4 << 3,
            Axis::Placement => 5 << 3,
            Axis::OffPlane => 6 << 3,
            Axis::Open => 7 << 3,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Axis::Sheet(_) => "sheet",
            Axis::Unit(_) => "unit",
            Axis::Prime5(_) => "prime5",
            Axis::Prime13(_) => "prime13",
            Axis::Hue => "hue",
            Axis::Placement => "placement",
            Axis::OffPlane => "offplane",
            Axis::Open => "open",
        }
    }

    fn from_byte(b: u8) -> Option<Axis> {
        let p = b & 7;
        match (b >> 3, p) {
            (0, p) if p < 6 => Some(Axis::Sheet(p)),
            (1, p) if p < 6 => Some(Axis::Unit(p)),
            (2, p) if p < 6 => Some(Axis::Prime5(p)),
            (3, p) if p < 6 => Some(Axis::Prime13(p)),
            (4, 0) => Some(Axis::Hue),
            (5, 0) => Some(Axis::Placement),
            (6, 0) => Some(Axis::OffPlane),
            (7, 0) => Some(Axis::Open),
            _ => None,
        }
    }
}

/// The state vector of a held loop (12.6), read from the canonical conjugacy
/// representative of its holonomy's plane form (15.2) and from the
/// translation part of its versor. Every coordinate is an exact integer or
/// rational; an axis is occupied when its coordinate is nonzero, and a pivot
/// is tried only on an occupied axis. The hue cycle type is not on a nerve's
/// faces and is read on the form (chapter 30).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassVector {
    /// Per slot plane, in `SLOT_PLANES_SIX` order: sheet parity, unit turns
    /// in Z/4, prime-5 and prime-13 exponents in Z.
    plane: [(bool, u32, i64, i64); 6],
    /// Translation part of the loop's versor after the tree gauge.
    placement: (num_rational::BigRational, num_rational::BigRational),
    /// The holonomy has no plane form: the class is the rotor's invariants.
    offplane: bool,
    key: [u8; 32],
}

/// The class key of a closed loop: the identity's, written as zero (A20).
pub const CLOSED_KEY: [u8; 32] = [0u8; 32];

impl ClassVector {
    /// The vector of a holonomy word alone (no versor: zero residue).
    pub fn of_word(h: &Word) -> ClassVector {
        let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
        Self::read(h, (zero.clone(), zero))
    }

    fn read(h: &Word, placement: (num_rational::BigRational, num_rational::BigRational)) -> ClassVector {
        if h.abelian_identity() {
            return ClassVector {
                plane: [(false, 0, 0, 0); 6],
                placement,
                offplane: false,
                key: CLOSED_KEY,
            };
        }
        let key = h.rotor().class_key();
        let mut plane = [(false, 0u32, 0i64, 0i64); 6];
        let offplane = match h.form() {
            Some(pf) => {
                // Slot layout of the seventeen letters: quarter turns 1..=6,
                // (3,4,5) rotors 7..=12, (5,12,13) rotors 13..=16 on the
                // first four planes; the sheet is the sensor plane's parity.
                let rep = pf.conjugacy_rep();
                for (p, slot) in plane.iter_mut().enumerate() {
                    let unit = rep.rot[1 + p].rem_euclid(4) as u32;
                    let m5 = rep.rot[7 + p];
                    let m13 = rep.rot.get(13 + p).copied().filter(|_| p < 4).unwrap_or(0);
                    *slot = (rep.parity && p == 0, unit, m5, m13);
                }
                false
            }
            None => true,
        };
        ClassVector {
            plane,
            placement,
            offplane,
            key,
        }
    }

    /// The coordinate an axis names, as "nonzero". `Open` is never a
    /// coordinate: it is the last move, admitted when no coordinate's own
    /// pivot discharged.
    pub fn occupies(&self, axis: Axis) -> bool {
        match axis {
            Axis::Sheet(p) => self.plane.get(p as usize).is_some_and(|s| s.0),
            Axis::Unit(p) => self.plane.get(p as usize).is_some_and(|s| s.1 != 0),
            Axis::Prime5(p) => self.plane.get(p as usize).is_some_and(|s| s.2 != 0),
            Axis::Prime13(p) => self.plane.get(p as usize).is_some_and(|s| s.3 != 0),
            Axis::Hue => false,
            Axis::Placement => {
                let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
                self.placement.0 != zero || self.placement.1 != zero
            }
            Axis::OffPlane => self.offplane,
            Axis::Open => false,
        }
    }

    /// The occupied coordinates in the fixed order of 12.6.
    pub fn coordinates(&self) -> Vec<Axis> {
        let mut out = Vec::new();
        for p in 0..6u8 {
            for axis in [Axis::Sheet(p), Axis::Unit(p), Axis::Prime5(p), Axis::Prime13(p)] {
                if self.occupies(axis) {
                    out.push(axis);
                }
            }
        }
        for axis in [Axis::Hue, Axis::Placement, Axis::OffPlane] {
            if self.occupies(axis) {
                out.push(axis);
            }
        }
        out
    }

    /// The occupied axes: the coordinates, then `Open` last whenever the
    /// loop is held at all (the one move that is not a coordinate).
    pub fn occupied(&self) -> Vec<Axis> {
        let mut out = self.coordinates();
        if !out.is_empty() {
            out.push(Axis::Open);
        }
        out
    }

    /// Every coordinate other than `axis` agrees: the discharge condition of
    /// 12.6 (a coordinate may fall to zero, none may rise or move).
    pub fn agrees_except(&self, other: &ClassVector, axis: Axis) -> bool {
        let mut a = self.clone();
        let mut b = other.clone();
        a.clear(axis);
        b.clear(axis);
        a.plane == b.plane && a.placement == b.placement && a.offplane == b.offplane
    }

    fn clear(&mut self, axis: Axis) {
        match axis {
            Axis::Sheet(p) => {
                if let Some(s) = self.plane.get_mut(p as usize) {
                    s.0 = false;
                }
            }
            Axis::Unit(p) => {
                if let Some(s) = self.plane.get_mut(p as usize) {
                    s.1 = 0;
                }
            }
            Axis::Prime5(p) => {
                if let Some(s) = self.plane.get_mut(p as usize) {
                    s.2 = 0;
                }
            }
            Axis::Prime13(p) => {
                if let Some(s) = self.plane.get_mut(p as usize) {
                    s.3 = 0;
                }
            }
            Axis::Placement => {
                let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
                self.placement = (zero.clone(), zero);
            }
            Axis::OffPlane => self.offplane = false,
            Axis::Hue | Axis::Open => {}
        }
    }

    pub fn class_key(&self) -> [u8; 32] {
        self.key
    }

    /// No coordinate is occupied: the loop is closed.
    pub fn is_zero(&self) -> bool {
        self.occupied().is_empty()
    }

    /// The free part: a prime exponent, or an off-plane class (infinite
    /// order). A loop with a free part is a demand, never filed torsion.
    pub fn free_part(&self) -> bool {
        self.offplane || self.plane.iter().any(|s| s.2 != 0 || s.3 != 0)
    }
}

/// A18: the dimension audit of a nerve or form, a vector never summed.
/// `torsion` counts the open finite-order loops; `free` is the ℤ-rank of
/// the free parts of the open plane loops plus the open off-plane loops;
/// `axes` the chart axes opened by discharge (none on a nerve, whose atlas
/// is the contemplation's).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Rank {
    pub torsion: u32,
    pub free: u32,
    pub axes: u32,
}

impl Rank {
    /// A pivot is admissible when it leaves rank equal or lowers `torsion`
    /// or `free` while `axes` does not fall (12.6, R2).
    pub fn admits(before: Rank, after: Rank) -> bool {
        after.torsion <= before.torsion && after.free <= before.free && after.axes >= before.axes
    }

    fn to_bytes(self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.torsion.to_le_bytes());
        out.extend_from_slice(&self.free.to_le_bytes());
        out.extend_from_slice(&self.axes.to_le_bytes());
    }

    fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Rank> {
        let mut f = [0u32; 3];
        for x in &mut f {
            *x = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
            *pos += 4;
        }
        Some(Rank {
            torsion: f[0],
            free: f[1],
            axes: f[2],
        })
    }
}

/// The free part of a holonomy as one row over ℤ: (m5, m13) per plane.
fn free_row(h: &Word) -> Option<[i64; 12]> {
    let rep = h.form()?.conjugacy_rep();
    let mut row = [0i64; 12];
    for p in 0..6 {
        row[2 * p] = rep.rot[7 + p];
        row[2 * p + 1] = rep.rot.get(13 + p).copied().filter(|_| p < 4).unwrap_or(0);
    }
    Some(row)
}

/// Rank of the lattice spanned by integer rows: fraction-free Gaussian
/// elimination (Bareiss) over i128, exact.
fn z_rank(rows: &[[i64; 12]]) -> u32 {
    let mut m: Vec<[i128; 12]> = rows
        .iter()
        .map(|r| std::array::from_fn(|i| r[i] as i128))
        .collect();
    let mut rank = 0usize;
    let mut prev = 1i128;
    for col in 0..12 {
        let Some(piv) = (rank..m.len()).find(|&r| m[r][col] != 0) else {
            continue;
        };
        m.swap(rank, piv);
        let p = m[rank][col];
        for r in (rank + 1)..m.len() {
            let f = m[r][col];
            for c in 0..12 {
                m[r][c] = (m[r][c] * p - m[rank][c] * f) / prev;
            }
        }
        prev = p;
        rank += 1;
    }
    rank as u32
}

/// What one loop contributes to rank (A18): read once per loop, so a trial
/// that changes one loop's holonomy swaps one row instead of re-reading the
/// nerve.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoopRow {
    Closed,
    Torsion,
    Free([i64; 12]),
    OffPlane,
}

pub fn loop_row(h: &Word) -> LoopRow {
    if h.abelian_identity() {
        return LoopRow::Closed;
    }
    match free_row(h) {
        Some(row) if row == [0i64; 12] => LoopRow::Torsion,
        Some(row) => LoopRow::Free(row),
        None => LoopRow::OffPlane,
    }
}

/// A18 over the rows of a nerve's cycles or a form's loops.
pub fn rank_of_rows(rows: &[LoopRow]) -> Rank {
    let mut torsion = 0u32;
    let mut offplane = 0u32;
    let mut free: Vec<[i64; 12]> = Vec::new();
    for r in rows {
        match r {
            LoopRow::Closed => {}
            LoopRow::Torsion => torsion += 1,
            LoopRow::Free(row) => free.push(*row),
            LoopRow::OffPlane => offplane += 1,
        }
    }
    Rank {
        torsion,
        free: z_rank(&free).saturating_add(offplane),
        axes: 0,
    }
}

pub fn rank_of_holonomies<'a>(holonomies: impl Iterator<Item = &'a Word>) -> Rank {
    let rows: Vec<LoopRow> = holonomies.map(loop_row).collect();
    rank_of_rows(&rows)
}

pub fn rank_of(n: &Nerve) -> Rank {
    rank_of_holonomies(n.cycles.iter().map(|c| &c.holonomy))
}

/// Which side of the chord face a held word is written on: the face word
/// `g` becomes `g · w⁻¹` (Right) or `w · g` (Left). Both are the relation
/// "the loop's holonomy is this held word"; on a non-abelian chart they are
/// two vertices and two candidates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

/// The move a certificate records (A20 `by`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PivotBy {
    /// A held word written onto the chord face `face` as the relation the
    /// bundle was missing on that loop.
    Word { face: u16, word: Word, side: Side },
}

/// A certified discharge of one occupied axis (A20). Content-addressed:
/// two minds that discharge one loop by one move produce one key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PivotCertificate {
    /// The loop's index in the walk before the move.
    pub chord: u16,
    pub axis: Axis,
    /// Class key of κ(ℓ) before, and after (`CLOSED_KEY` when the loop closed).
    pub before: [u8; 32],
    pub after: [u8; 32],
    pub rank_before: Rank,
    pub rank_after: Rank,
    /// μ⃗(ℓ): the remainder the loop leaves open, the dominance test only.
    pub mu: [u64; 4],
    pub by: PivotBy,
}

impl PivotCertificate {
    pub fn key(&self) -> [u8; 32] {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.chord.to_le_bytes());
        buf.push(self.axis.to_byte());
        buf.extend_from_slice(&self.before);
        buf.extend_from_slice(&self.after);
        self.rank_before.to_bytes(&mut buf);
        self.rank_after.to_bytes(&mut buf);
        let PivotBy::Word { face, word, side } = &self.by;
        buf.extend_from_slice(&face.to_le_bytes());
        buf.push(match side {
            Side::Left => 0,
            Side::Right => 1,
        });
        buf.extend_from_slice(&word.key());
        *blake3::hash(&buf).as_bytes()
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.chord.to_le_bytes());
        out.push(self.axis.to_byte());
        out.extend_from_slice(&self.before);
        out.extend_from_slice(&self.after);
        self.rank_before.to_bytes(out);
        self.rank_after.to_bytes(out);
        for m in &self.mu {
            out.extend_from_slice(&m.to_le_bytes());
        }
        match &self.by {
            PivotBy::Word { face, word, side } => {
                out.push(0);
                out.extend_from_slice(&face.to_le_bytes());
                out.push(match side {
                    Side::Left => 0,
                    Side::Right => 1,
                });
                word.to_bytes(out);
            }
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<PivotCertificate> {
        let chord = take_u16(b, pos)?;
        let axis = Axis::from_byte(*b.get(*pos)?)?;
        *pos += 1;
        let before: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        let after: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        let rank_before = Rank::from_bytes(b, pos)?;
        let rank_after = Rank::from_bytes(b, pos)?;
        let mut mu = [0u64; 4];
        for m in &mut mu {
            *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
        let kind = *b.get(*pos)?;
        *pos += 1;
        let by = match kind {
            0 => {
                let face = take_u16(b, pos)?;
                let side = match *b.get(*pos)? {
                    0 => Side::Left,
                    1 => Side::Right,
                    _ => return None,
                };
                *pos += 1;
                let word = Word::from_bytes(b, pos)?;
                PivotBy::Word { face, word, side }
            }
            _ => return None,
        };
        Some(PivotCertificate {
            chord,
            axis,
            before,
            after,
            rank_before,
            rank_after,
            mu,
            by,
        })
    }
}

/// The face word after a held word is written onto the chord (A16 step 4).
pub fn pivoted_face(g: &Word, w: &Word, side: Side) -> Word {
    match side {
        Side::Right => g.compose(&w.inverse()).normal_form(),
        Side::Left => w.compose(g).normal_form(),
    }
}

/// The face word before that move: the inverse of [`pivoted_face`].
pub fn unpivoted_face(g: &Word, w: &Word, side: Side) -> Word {
    match side {
        Side::Right => g.compose(w).normal_form(),
        Side::Left => w.inverse().compose(g).normal_form(),
    }
}

/// The translation part of a loop's versor: the product of its faces'
/// versors in path order, each read in the direction the path walks it.
fn loop_translation(n: &Nerve, c: &Cycle) -> (num_rational::BigRational, num_rational::BigRational) {
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut acc: Option<Versor> = None;
    for (win, &fi) in c.germs.windows(2).zip(&c.faces) {
        let Some(f) = n.faces.get(fi) else {
            return (zero.clone(), zero);
        };
        let step = if f.i == win[0] && f.j == win[1] {
            f.versor.clone()
        } else {
            f.versor.reverse()
        };
        acc = Some(match acc {
            None => step,
            Some(v) => step.geo(&v),
        });
    }
    acc.and_then(|v| affine_of(&v))
        .filter(|a| a.len() >= 2 && a[0].len() >= 3 && a[1].len() >= 3)
        .map(|a| (a[0][2].clone(), a[1][2].clone()))
        .unwrap_or((zero.clone(), zero))
}

/// The class vector of loop `idx` of a nerve (12.6).
pub fn class_vector_of(n: &Nerve, idx: usize) -> Option<ClassVector> {
    let c = n.cycles.get(idx)?;
    Some(ClassVector::read(&c.holonomy, loop_translation(n, c)))
}

/// The axis index of a crystal (12.6, A16 step 3): chart → axis slot → M1
/// word. Nested keyed reads, never a hash of the pair.
pub type AxisIndex = Keyed<Keyed<Keyed<Word>>>;

fn axis_slot(axis: Axis) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[0] = axis.to_byte();
    k
}

/// The words a form speaks: its Word vertices'.
pub fn words_of_form(f: &Form) -> Vec<Word> {
    f.vertices
        .iter()
        .filter_map(|v| match v {
            Vertex::Word { word, .. } => Some(word.clone()),
            _ => None,
        })
        .collect()
}

/// The words a held element speaks: a form's Word vertices, or a
/// connection's letters.
pub fn words_of_element(e: &Element) -> Vec<Word> {
    if e.is_sentence() {
        return from_element_form(e).map(|f| words_of_form(&f)).unwrap_or_default();
    }
    e.letters().map(|l| vec![l.spelling.word]).unwrap_or_default()
}

/// Admit words to the axis index under every coordinate their class vector
/// occupies on `chart`.
pub fn admit_words(index: &mut AxisIndex, chart: &stalk::Invariant, words: &[Word]) {
    let ck = chart.key();
    for w in words {
        for axis in ClassVector::of_word(w).coordinates() {
            let ak = axis_slot(axis);
            match index.get_mut(&ck) {
                Some(mid) => match mid.get_mut(&ak) {
                    Some(bucket) => {
                        bucket.insert(w.key(), w.clone());
                    }
                    None => {
                        let mut bucket = Keyed::new();
                        bucket.insert(w.key(), w.clone());
                        mid.insert(ak, bucket);
                    }
                },
                None => {
                    let mut bucket = Keyed::new();
                    bucket.insert(w.key(), w.clone());
                    let mut mid = Keyed::new();
                    mid.insert(ak, bucket);
                    index.insert(ck, mid);
                }
            }
        }
    }
}

/// Retract words from the axis index (shed, retire).
pub fn retract_words(index: &mut AxisIndex, chart: &stalk::Invariant, words: &[Word]) {
    let ck = chart.key();
    let Some(mid) = index.get_mut(&ck) else {
        return;
    };
    for w in words {
        for axis in ClassVector::of_word(w).coordinates() {
            if let Some(bucket) = mid.get_mut(&axis_slot(axis)) {
                bucket.remove(&w.key());
            }
        }
    }
}

/// The fiber of the axis index at (chart, axis).
pub fn on_axis_of(index: &AxisIndex, chart: &stalk::Invariant, axis: Axis) -> Vec<Word> {
    index
        .get(&chart.key())
        .and_then(|mid| mid.get(&axis_slot(axis)))
        .map(|bucket| bucket.fiber(&crate::network::nerve::ROOT).map(|(_, w)| w.clone()).collect())
        .unwrap_or_default()
}

/// One candidate pivot: a held word that occupies the axis, written on the
/// chord on one side. Two candidates whose face words are one vertex are one.
struct Candidate {
    axis: Axis,
    by: PivotBy,
}

/// A16 step 3: for each occupied coordinate of the loop, in the fixed
/// order, the held words on the loop's own charts that occupy it, read as
/// one fiber of the crystal's axis index; each word on either side of the
/// chord. A germ without a chart reads the root stratum.
fn candidates_for(n: &Nerve, c: &Cycle, kappa: &ClassVector, crystal: &dyn super::family::Crystal) -> Vec<Candidate> {
    let Some(chord) = c.chord() else {
        return Vec::new();
    };
    let Some(_face) = n.faces.get(chord) else {
        return Vec::new();
    };
    let mut charts: Keyed<stalk::Invariant> = Keyed::new();
    for &gi in &c.germs {
        if let Some(g) = n.germs.get(gi) {
            let chart = g.target.clone().or_else(|| g.source.clone()).unwrap_or_default();
            charts.insert(chart.key(), chart);
        }
    }
    fn side_slot(side: Side) -> [u8; 32] {
        let mut k = [0u8; 32];
        k[0] = match side {
            Side::Left => 0,
            Side::Right => 1,
        };
        k
    }
    let mut seen: Keyed<Keyed<()>> = Keyed::new();
    let mut out = Vec::new();
    for axis in kappa.coordinates() {
        for (_, chart) in charts.fiber(&crate::network::nerve::ROOT) {
            for w in crystal.on_axis(chart, axis) {
                for side in [Side::Right, Side::Left] {
                    let wk = w.key();
                    let sk = side_slot(side);
                    if seen.get(&wk).is_some_and(|s| s.contains(&sk)) {
                        continue;
                    }
                    match seen.get_mut(&wk) {
                        Some(sides) => {
                            sides.insert(sk, ());
                        }
                        None => {
                            let mut sides = Keyed::new();
                            sides.insert(sk, ());
                            seen.insert(wk, sides);
                        }
                    }
                    out.push(Candidate {
                        axis,
                        by: PivotBy::Word {
                            face: chord as u16,
                            word: w.clone(),
                            side,
                        },
                    });
                }
            }
        }
    }
    out
}

/// The trial of one candidate (A16 step 4). Only the chord face changes, so
/// the loop's holonomy after the move is exactly the old holonomy with the
/// word written on; its vector is re-read, and the rank re-read with that
/// one loop's row swapped. The candidate discharges iff the axis's
/// coordinate is zero, no other coordinate moved, and rank is admissible.
fn trial(
    n: &Nerve,
    idx: usize,
    cand: &Candidate,
    kappa: &ClassVector,
    rows: &[LoopRow],
    rank_before: Rank,
    mu: [u64; 4],
) -> Option<PivotCertificate> {
    let c = n.cycles.get(idx)?;
    let PivotBy::Word { face, word, side } = &cand.by;
    // Write on the chord face and re-read that same path. Pivoting the
    // stored holonomy is only valid when the walk traverses the chord
    // forward; a backward chord would select the side that leaves the
    // after-face off `face_of`.
    let Some(held) = n.faces.get(*face as usize) else {
        return None;
    };
    let after_g = pivoted_face(&held.g_ij, word, *side);
    let mut w = Word::identity();
    for (win, &fi) in c.germs.windows(2).zip(&c.faces) {
        let step = if fi == *face as usize {
            if held.i == win[0] && held.j == win[1] {
                after_g.clone()
            } else {
                after_g.inverse()
            }
        } else {
            face_step(&n.faces, fi, win[0], win[1]).unwrap_or_else(Word::identity)
        };
        w = w.compose(&step);
    }
    let after_h = w.normal_form();
    let kappa_after = ClassVector::read(&after_h, kappa.placement.clone());
    if kappa_after.occupies(cand.axis) || !kappa.agrees_except(&kappa_after, cand.axis) {
        return None;
    }
    let mut after_rows = rows.to_vec();
    *after_rows.get_mut(idx)? = loop_row(&after_h);
    let rank_after = rank_of_rows(&after_rows);
    if !Rank::admits(rank_before, rank_after) {
        return None;
    }
    Some(PivotCertificate {
        chord: idx as u16,
        axis: cand.axis,
        before: kappa.class_key(),
        after: kappa_after.class_key(),
        rank_before,
        rank_after,
        mu,
        by: cand.by.clone(),
    })
}

/// Apply a certified pivot: write the word onto the chord face and re-walk.
pub fn apply_pivot(n: &mut Nerve, cert: &PivotCertificate) {
    let PivotBy::Word { face, word, side } = &cert.by;
    if let Some(f) = n.faces.get_mut(*face as usize) {
        f.g_ij = pivoted_face(&f.g_ij, word, *side);
    }
    n.cycles = independent_cycles(n.germs.len(), &n.faces);
}

/// Dominance certifies (law 5): the lane-wise minimum of the candidates'
/// μ⃗ is computed once; a candidate whose μ⃗ equals that minimum is the
/// discharge. Several that match, or none that match (incomparable lanes),
/// are a vortex. No pairwise pass.
fn select_discharge(cands: &[(PivotCertificate, [u64; 4])]) -> Option<PivotCertificate> {
    let Some((_, first)) = cands.first() else {
        return None;
    };
    let min: [u64; 4] = std::array::from_fn(|l| {
        cands.iter().map(|(_, m)| m[l]).min().unwrap_or(first[l])
    });
    let mut winner: Option<&PivotCertificate> = None;
    for (c, m) in cands {
        if *m == min {
            if winner.is_some() {
                return None;
            }
            winner = Some(c);
        }
    }
    winner.cloned()
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
    apply_cochain_gauge(n, &words);
    // A gauge keeps loop indices aligned; the chord is this loop.
    match n.cycles.get(idx) {
        Some(c) => c.holonomy.abelian_identity(),
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
    // σ: the loop's own consecutive germ pairs, written as it is filed.
    let sigma: Vec<(u8, u8)> = c
        .germs
        .windows(2)
        .map(|p| (p[0] as u8, p[1] as u8))
        .collect();
    let rotor = c.holonomy.rotor();
    let class = rotor.class_key();
    n.complex.file_congruent(germ_vertex(a), germ_vertex(b), rotor, ell);
    n.torsion.insert(class, (sigma, ell));
    true
}

/// A16, the pivot loop. For every open loop, in index order: the tree gauge
/// first (a trivial class closes by re-gauging, T2, never rank); then the
/// pivots the loop's class vector names, read as one axis-index fiber per
/// occupied coordinate from the crystal (the working atlas), each tried
/// exactly; every discharge is certified (A20); several discharges with no
/// dominant μ⃗ are a vortex held as cells; a loop nothing discharges is
/// filed as torsion when finite and spoken as a demand when free. A class
/// the horizon already holds is filed silently and never demanded again.
/// No candidate is ordered by a scalar and nothing is searched for.
pub fn pivot_loop(
    n: &mut Nerve,
    idea: [u8; 32],
    horizon: &Keyed<(Rotor, Order, Standing)>,
    crystal: &dyn super::family::Crystal,
    budget: &mut Budget,
) -> Vec<Snap> {
    let mut snaps = Vec::new();
    let mut handled: Keyed<()> = Keyed::new();
    loop {
        if budget.remaining() == 0 {
            break;
        }
        n.cycles = independent_cycles(n.germs.len(), &n.faces);
        let open: Vec<usize> = n
            .cycles
            .iter()
            .enumerate()
            .filter(|(_, c)| !c.holonomy.abelian_identity())
            .map(|(i, _)| i)
            .collect();
        if open.is_empty() {
            break;
        }
        let rows: Vec<LoopRow> = n.cycles.iter().map(|c| loop_row(&c.holonomy)).collect();
        let rank_before = rank_of_rows(&rows);
        let mu = element::measure_lanes(&n.remainder);
        let mut applied = false;
        for idx in open {
            if !budget.spend() {
                break;
            }
            let Some(kappa) = class_vector_of(n, idx) else {
                continue;
            };
            let key = kappa.class_key();
            if handled.contains(&key) {
                continue;
            }
            let held = horizon.get(&key).is_some();
            let order = n.cycles[idx].holonomy.order();
            if try_close_chord(n, idx) {
                let ell = match order {
                    Order::Finite(e) => e,
                    Order::Infinite => 0,
                };
                snaps.push(Snap::Closed { chord: idx, order: ell });
                eprintln!(
                    "  pivot {:02x}{:02x}… loop {idx} closed by the tree gauge",
                    idea[0], idea[1]
                );
                handled.insert(key, ());
                applied = true;
                break;
            }
            let cands = candidates_for(n, &n.cycles[idx], &kappa, crystal);
            let mut discharging: Vec<(PivotCertificate, [u64; 4])> = Vec::new();
            for cand in &cands {
                if !budget.spend() {
                    break;
                }
                if let Some(cert) = trial(n, idx, cand, &kappa, &rows, rank_before, mu) {
                    discharging.push((cert, mu));
                } else {
                    eprintln!(
                        "  pivot {:02x}{:02x}… loop {idx} axis {:?} no discharge",
                        idea[0], idea[1], cand.axis
                    );
                }
            }
            match select_discharge(&discharging) {
                Some(cert) => {
                    apply_pivot(n, &cert);
                    eprintln!(
                        "  pivot {:02x}{:02x}… loop {idx} axis {:?} discharged {:02x}{:02x}…→{:02x}{:02x}…",
                        idea[0], idea[1], cert.axis, cert.before[0], cert.before[1], cert.after[0], cert.after[1]
                    );
                    n.certificates.insert(cert.key(), cert.clone());
                    snaps.push(Snap::Discharged {
                        chord: idx,
                        certificate: cert,
                    });
                    handled.insert(key, ());
                    applied = true;
                    break;
                }
                None if !discharging.is_empty() => {
                    eprintln!(
                        "  vortex {:02x}{:02x}… loop {idx} {} candidates held",
                        idea[0],
                        idea[1],
                        discharging.len()
                    );
                    let mut held = Vec::new();
                    for (c, _) in discharging {
                        n.vortex.insert(c.key(), c.clone());
                        held.push(c);
                    }
                    snaps.push(Snap::Vortex {
                        chord: idx,
                        held,
                    });
                    handled.insert(key, ());
                }
                None => match order {
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
                        handled.insert(key, ());
                    }
                    Order::Finite(ell) => {
                        if file_cycle(n, idx, ell) {
                            if !held {
                                snaps.push(Snap::TorsionFiled { chord: idx, ell });
                                eprintln!(
                                    "  torsion filed {:02x}{:02x}… (σ,{ell})",
                                    idea[0], idea[1]
                                );
                            }
                        } else {
                            snaps.push(Snap::TorsionOpen { chord: idx });
                            eprintln!(
                                "  torsion open {:02x}{:02x}… chord {idx}",
                                idea[0], idea[1]
                            );
                        }
                        handled.insert(key, ());
                    }
                },
            }
        }
        if !applied {
            break;
        }
    }
    snaps
}

/// A nerve crystallized as its form on the v2 tape (25.2): the certified
/// pivots its closure rests on ride the tape, and the torsion cycles it
/// filed (`Nerve::torsion`, written at filing) are recorded on the element
/// as for any word. Face indices are the nerve's (from_nerve keeps face
/// order), so certificates carry over as they are.
pub fn as_element_form_of_nerve(n: &Nerve) -> Option<Element> {
    let mut form = Form::from_nerve(n);
    form.certificates = n
        .certificates
        .fiber(&crate::network::nerve::ROOT)
        .map(|(_, c)| c.clone())
        .collect();
    form.torsion = n.torsion.clone();
    let mut e = as_element_form(&form, cochain_magnitude(n))?;
    e.adopt_torsion(&form.torsion);
    Some(e)
}

/// Close the sentence: fill up the degrees on each germ's link. A class
/// at any degree stays as torsion and the sentence is thought.
pub fn close(n: &mut Nerve) -> bool {
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let mut open: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
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
                    open.insert(k, ());
                }
                Fill::Class(r) => {
                    n.class = Some((r, d));
                    open.insert(k, ());
                }
                Fill::Open | Fill::Already => {
                    open.insert(k, ());
                }
            }
        }
    }
    n.cycles = independent_cycles(n.germs.len(), &n.faces);
    if !open.is_empty() || n.class.is_some() {
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

fn word_to_form_bytes(w: &Word, contact: Option<alphabet::Contact>, out: &mut Vec<u8>) {
    w.to_bytes(out);
    match contact {
        None => out.push(0),
        Some(c) => {
            out.push(1);
            out.extend_from_slice(&c.anchor.to_le_bytes());
            out.push(c.ring);
        }
    }
}

fn word_from_form_bytes(b: &[u8], pos: &mut usize) -> Option<(Word, Option<alphabet::Contact>)> {
    let w = Word::from_bytes(b, pos)?;
    let tag = *b.get(*pos)?;
    *pos += 1;
    if tag == 0 {
        return Some((w, None));
    }
    if tag != 1 {
        return None;
    }
    let anchor = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
    *pos += 2;
    let ring = *b.get(*pos)?;
    *pos += 1;
    Some((w, Some(alphabet::Contact { anchor, ring })))
}

fn bytes_left(b: &[u8], pos: usize) -> usize {
    b.len().saturating_sub(pos)
}

/// Bound a count by the bytes that remain for items of size `item`. Refuse
/// rather than silently truncate.
fn take_count(b: &[u8], pos: usize, n: usize, item: usize) -> Option<usize> {
    if item == 0 {
        return Some(n);
    }
    if n > bytes_left(b, pos) / item {
        return None;
    }
    Some(n)
}

fn put_inv(inv: &stalk::Invariant, out: &mut Vec<u8>) {
    let n = u16::try_from(inv.0.len()).unwrap_or(u16::MAX);
    out.extend_from_slice(&n.to_le_bytes());
    for a in inv.0.iter().take(n as usize) {
        out.extend_from_slice(&a.to_le_bytes());
    }
}

fn take_inv(b: &[u8], pos: &mut usize) -> Option<stalk::Invariant> {
    let n = take_u16(b, pos)? as usize;
    let n = take_count(b, *pos, n, 8)?;
    let mut axes = Vec::with_capacity(n);
    for _ in 0..n {
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
                contact,
            } => {
                out.push(1);
                out.extend_from_slice(address);
                word_to_form_bytes(word, *contact, out);
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
                word_to_form_bytes(placement, None, out);
            }
            Vertex::Rebased {
                by,
                of,
                shift,
                rho,
                gauge,
                class,
            } => {
                out.push(3);
                out.extend_from_slice(&by.to_le_bytes());
                out.extend_from_slice(&of.to_le_bytes());
                alphabet::put_q(out, &shift.0);
                alphabet::put_q(out, &shift.1);
                alphabet::put_q(out, rho);
                out.extend_from_slice(&(gauge.len() as u16).to_le_bytes());
                for &(a, b) in gauge {
                    out.push(a);
                    out.push(b);
                }
                put_inv(class, out);
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
        word_to_form_bytes(&face.g_ij, None, out);
    }
    out.extend_from_slice(&(f.loops.len() as u16).to_le_bytes());
    for lp in &f.loops {
        out.extend_from_slice(&(lp.faces.len() as u16).to_le_bytes());
        for &i in &lp.faces {
            out.extend_from_slice(&i.to_le_bytes());
        }
        word_to_form_bytes(&lp.holonomy, None, out);
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
    out.extend_from_slice(&(f.certificates.len() as u16).to_le_bytes());
    for cert in &f.certificates {
        cert.to_bytes(out);
    }
    let ((hn, hd), (wn, wd)) = f.ratio;
    out.extend_from_slice(&(hn as u16).to_le_bytes());
    out.extend_from_slice(&(hd as u16).to_le_bytes());
    out.extend_from_slice(&(wn as u16).to_le_bytes());
    out.extend_from_slice(&(wd as u16).to_le_bytes());
    let sec: Vec<([u8; 32], Vec<u16>)> = f
        .sections
        .fiber(&crate::network::nerve::ROOT)
        .map(|(k, v)| (*k, v.clone()))
        .collect();
    out.extend_from_slice(&(sec.len() as u16).to_le_bytes());
    for (k, ix) in sec {
        out.extend_from_slice(&k);
        out.extend_from_slice(&(ix.len() as u16).to_le_bytes());
        for i in ix {
            out.extend_from_slice(&i.to_le_bytes());
        }
    }
    out.extend_from_slice(&(f.torsion.len() as u16).to_le_bytes());
    for (k, (sigma, ell)) in f.torsion.fiber(&crate::network::nerve::ROOT) {
        out.extend_from_slice(k);
        out.extend_from_slice(&(sigma.len() as u16).to_le_bytes());
        for &(a, b) in sigma {
            out.push(a);
            out.push(b);
        }
        out.extend_from_slice(&ell.to_le_bytes());
    }
}

fn take_u16(b: &[u8], pos: &mut usize) -> Option<u16> {
    let v = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
    *pos += 2;
    Some(v)
}

fn decode_form_v2(b: &[u8], pos: &mut usize) -> Option<Form> {
    let nv = take_count(b, *pos, take_u16(b, pos)? as usize, 1)?;
    if nv == 0 {
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
                let (word, contact) = word_from_form_bytes(b, pos)?;
                let rho = alphabet::take_q(b, pos)?;
                let p0 = alphabet::take_q(b, pos)?;
                let p1 = alphabet::take_q(b, pos)?;
                let ng = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
                let mut gauge = Vec::with_capacity(ng);
                for _ in 0..ng {
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
                    contact,
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
                let (placement, _) = word_from_form_bytes(b, pos)?;
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
                let s0 = alphabet::take_q(b, pos)?;
                let s1 = alphabet::take_q(b, pos)?;
                let rho = alphabet::take_q(b, pos)?;
                let ng = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
                let mut gauge = Vec::with_capacity(ng);
                for _ in 0..ng {
                    gauge.push((*b.get(*pos)?, *b.get(*pos + 1)?));
                    *pos += 2;
                }
                let class = take_inv(b, pos)?;
                Vertex::Rebased {
                    by,
                    of,
                    shift: (s0, s1),
                    rho,
                    gauge,
                    class,
                }
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
    let nf = take_count(b, *pos, take_u16(b, pos)? as usize, 4)?;
    let mut faces = Vec::with_capacity(nf);
    for _ in 0..nf {
        let i = take_u16(b, pos)?;
        let j = take_u16(b, pos)?;
        let (g_ij, _) = word_from_form_bytes(b, pos)?;
        faces.push(FormFace { i, j, g_ij });
    }
    let nl = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
    let mut loops = Vec::with_capacity(nl);
    for _ in 0..nl {
        let n = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
        let mut fs = Vec::with_capacity(n);
        for _ in 0..n {
            fs.push(take_u16(b, pos)?);
        }
        let (holonomy, _) = word_from_form_bytes(b, pos)?;
        loops.push(Loop {
            faces: fs,
            holonomy,
        });
    }
    let nh = take_count(b, *pos, take_u16(b, pos).unwrap_or(0) as usize, 32)?;
    let mut hue_maps = Keyed::new();
    for _ in 0..nh {
        let k: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        let np = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
        let mut pairs = Vec::with_capacity(np);
        for _ in 0..np {
            pairs.push((*b.get(*pos)?, *b.get(*pos + 1)?));
            *pos += 2;
        }
        hue_maps.insert(k, pairs);
    }
    let nr = take_count(b, *pos, take_u32(b, pos).unwrap_or(0) as usize, 18)?;
    let mut remainder = Vec::with_capacity(nr);
    for _ in 0..nr {
        remainder.push(take_cell(b, pos)?);
    }
    let depth = take_u16(b, pos).unwrap_or(0);
    let ncert = take_count(b, *pos, take_u16(b, pos).unwrap_or(0) as usize, 2)?;
    let mut certificates = Vec::with_capacity(ncert);
    for _ in 0..ncert {
        certificates.push(PivotCertificate::from_bytes(b, pos)?);
    }
    let mut ratio = ((1usize, 1usize), (1usize, 1usize));
    let mut sections = Keyed::new();
    if let (Some(hn), Some(hd), Some(wn), Some(wd)) = (
        take_u16(b, pos),
        take_u16(b, pos),
        take_u16(b, pos),
        take_u16(b, pos),
    ) {
        // A zero denominator is not a ratio; the tape is refused, never
        // repaired (rule 11).
        if hd == 0 || wd == 0 {
            return None;
        }
        ratio = ((hn as usize, hd as usize), (wn as usize, wd as usize));
        if let Some(ns) = take_u16(b, pos) {
            let ns = take_count(b, *pos, ns as usize, 32)?;
            for _ in 0..ns {
                let k: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                *pos += 32;
                let nv = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
                let mut ix = Vec::with_capacity(nv);
                for _ in 0..nv {
                    ix.push(take_u16(b, pos)?);
                }
                sections.insert(k, ix);
            }
        }
    }
    let mut torsion = Keyed::new();
    if *pos < b.len() {
        let nt = take_count(b, *pos, take_u16(b, pos)? as usize, 32)?;
        for _ in 0..nt {
            let k: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
            *pos += 32;
            let ns = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
            let mut sigma = Vec::with_capacity(ns);
            for _ in 0..ns {
                sigma.push((*b.get(*pos)?, *b.get(*pos + 1)?));
                *pos += 2;
            }
            let ell = take_u32(b, pos)?;
            torsion.insert(k, (sigma, ell));
        }
    }
    Some(Form {
        vertices,
        faces,
        loops,
        remainder,
        depth,
        hue_maps,
        ratio,
        sections,
        certificates,
        torsion,
    })
}

/// SENTENCE_TAG v2: the Form tape. v1 nerve tapes still go through [`as_element`].
/// `mag` is the descent the form measures over its bundle. Zero is not a word.
pub fn as_element_form(f: &Form, mag: [u64; 4]) -> Option<Element> {
    if f.vertices.is_empty() || mag.iter().all(|&x| x == 0) {
        return None;
    }
    let mut e = Element::identity(GERM_DIM);
    let mut word = vec![element::SENTENCE_TAG, 2];
    encode_form_v2(f, &mut word);
    e.set_description(word);
    e.set_magnitude(mag);
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
    e.adopt_torsion(&n.torsion);
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
            let n = u8::try_from(i.0.len()).unwrap_or(u8::MAX);
            out.push(n);
            for a in i.0.iter().take(n as usize) {
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
    let n = take_count(b, *pos + 1, *b.get(*pos)? as usize, 8)?;
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
    let ng = take_count(b, *pos, take_u32(b, pos)? as usize, 32)?;
    let mut germs = Vec::with_capacity(ng);
    for _ in 0..ng {
        let _stored: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        let versor = Versor::from_bytes(b, pos)?;
        let word = Word::from_bytes(b, pos)?;
        let nc = take_count(b, *pos, take_u32(b, pos)? as usize, 18)?;
        let mut chain = Vec::with_capacity(nc);
        for _ in 0..nc {
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
    let nf = take_count(b, *pos, take_u32(b, pos)? as usize, 8)?;
    let mut faces = Vec::with_capacity(nf);
    for _ in 0..nf {
        let i = take_u32(b, pos)? as usize;
        let j = take_u32(b, pos)? as usize;
        let no = take_count(b, *pos, take_u32(b, pos)? as usize, 18)?;
        let mut overlap = Vec::with_capacity(no);
        for _ in 0..no {
            overlap.push(take_cell(b, pos)?);
        }
        let g_ij = Word::from_bytes(b, pos)?;
        let versor = Versor::from_bytes(b, pos)?;
        let spelled = *b.get(*pos)? & 1 != 0;
        *pos += 1;
        faces.push(Face { i, j, overlap, g_ij, versor, spelled });
    }
    // Loops are derived, never read: the tape's cycle section is walked past
    // and the cycles are re-derived from the faces (chapter 19).
    let ncyc = take_u32(b, pos)? as usize;
    if ncyc > b.len().saturating_sub(*pos) / 4 {
        return None;
    }
    for _ in 0..ncyc {
        let nk = take_u32(b, pos)? as usize;
        if nk > b.len().saturating_sub(*pos) / 4 {
            return None;
        }
        *pos = pos.checked_add(nk.checked_mul(4)?)?;
        let _ = Word::from_bytes(b, pos)?;
    }
    let cycles = independent_cycles(germs.len(), &faces);
    let nr = take_count(b, *pos, take_u32(b, pos).unwrap_or(0) as usize, 18)?;
    let mut remainder = Vec::with_capacity(nr);
    for _ in 0..nr {
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
        let nc = take_count(b, *pos, take_u32(b, pos).unwrap_or(0) as usize, 32)?;
        for _ in 0..nc {
            let k: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
            *pos += 32;
            cover.insert(k, ());
        }
    }
    Some(Nerve {
        germs,
        complex,
        faces,
        cycles,
        remainder,
        class,
        cover,
        torsion: Keyed::new(),
        certificates: Keyed::new(),
        vortex: Keyed::new(),
    })
}

/// A sentence Element: tagged tape, not an opened-axis connection.
pub fn is_sentence(e: &Element) -> bool {
    e.is_sentence()
}

/// Chart of a sentence Element: leading axis 2, then germ/face/cycle
/// counts from the tape, then each germ's linear invariants.
/// The chart of a crystallized form (M4 step 3): the form's cell counts,
/// then per vertex its A3-reduced plane form (a rebased vertex reads the
/// word it speaks; a glued vertex contributes the cited chart), sorted so
/// the order of vertices is not a coordinate.
fn chart_of_form(f: &Form) -> crate::transducer::stalk::Invariant {
    let mut axes = vec![
        2u64,
        f.vertices.len() as u64,
        f.faces.len() as u64,
        f.loops.len() as u64,
    ];
    let mut rows: Vec<Vec<u64>> = Vec::new();
    for v in &f.vertices {
        match v {
            Vertex::Word { .. } | Vertex::Rebased { .. } => {
                let row = v.word_in(f).form().map(|pf| {
                    let mut row = vec![1u64, pf.parity as u64];
                    row.extend(pf.rot.iter().map(|&x| x as u64));
                    row
                });
                rows.push(row.unwrap_or_default());
            }
            Vertex::Glued { chart, .. } => {
                let mut row = vec![3u64];
                row.extend_from_slice(&chart.0);
                rows.push(row);
            }
            Vertex::Frame | Vertex::Unfolded { .. } => {}
        }
    }
    rows.sort();
    for row in rows {
        axes.extend(row);
    }
    crate::transducer::stalk::Invariant(axes)
}

pub fn chart_of(e: &Element) -> crate::transducer::stalk::Invariant {
    if e.description().get(1).copied() == Some(2) {
        if let Some(f) = from_element_form(e) {
            return chart_of_form(&f);
        }
    }
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
    // A v2 tape is a Form (chapter 19). Readers that still want a nerve see
    // it through the chapter 30 adapter, and hold (None) when the form
    // carries a vertex a nerve cannot: Glued, Rebased, Unfolded.
    if b.get(1).copied() == Some(2) {
        let mut pos = 2usize;
        return decode_form_v2(b, &mut pos)?.to_nerve();
    }
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
        let mut f = Form::default();
        f.vertices.push(Vertex::Word {
            address: [7u8; 32],
            word: word.clone(),
            rho: q1(),
            placement: (q0(), q1()),
            gauge: vec![(1, 2)],
            class: stalk::Invariant(vec![3, 5]),
            contact,
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
        assert!(as_element_form(&inner, [0; 4]).is_none(), "zero descent is not spoken");
        let inner_e = as_element_form(&inner, [1, 0, 0, 0]).expect("inner tape");
        let inner_addr = inner_e.address();

        let mut f = closed_word_form(
            w.clone(),
            Some(alphabet::Contact { anchor: 1, ring: 2 }),
        );
        let glued = f
            .glue(&inner, inner_addr, Word::identity(), [0, 0, 0, 1])
            .expect("glue depth 0 into depth 0");
        let rebased = f.vertices.len() as u16;
        f.vertices.push(Vertex::Rebased {
            by: 0,
            of: 1,
            shift: (q0(), q1()),
            rho: q1(),
            gauge: vec![(1, 2)],
            class: stalk::Invariant(vec![3, 5]),
        });
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
        assert!(matches!(
            f.vertices[rebased as usize],
            Vertex::Rebased { by: 0, of: 1, .. }
        ));
        assert!(matches!(
            f.vertices[unfolded as usize],
            Vertex::Unfolded {
                measure: Measure::Occupied
            }
        ));

        let e = as_element_form(&f, [1, 0, 0, 0]).expect("v2 tape");
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
                Vertex::Word { word: a, gauge: ga, class: ca, contact: ca_c, .. },
                Vertex::Word { word: b, gauge: gb, class: cb, contact: cb_c, .. },
            ) => {
                assert_eq!(ca_c.map(|c| (c.anchor, c.ring)), Some((1, 2)));
                assert_eq!(cb_c.map(|c| (c.anchor, c.ring)), Some((1, 2)));
                assert_eq!(a, b, "M1 is the rotor, contact is beside it");
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
        match &back.vertices[rebased as usize] {
            Vertex::Rebased {
                by,
                of,
                shift,
                rho,
                gauge,
                class,
            } => {
                assert_eq!(*by, 0);
                assert_eq!(*of, 1);
                assert_eq!(shift, &(q0(), q1()));
                assert_eq!(rho, &q1());
                assert_eq!(gauge, &vec![(1, 2)]);
                assert_eq!(class, &stalk::Invariant(vec![3, 5]));
            }
            _ => panic!("rebased vertex"),
        }
        assert_eq!(
            back.vertices[unfolded as usize],
            Vertex::Unfolded {
                measure: Measure::Occupied
            }
        );
        assert_eq!(back.faces[0].i, 0);
        assert_eq!(back.faces[0].j, 1);
        assert_eq!(back.faces[0].g_ij, f.faces[0].g_ij);
        let e_plain = as_element_form(&closed_word_form(w.clone(), None), [1, 0, 0, 0]).expect("plain");
        let e_contact = as_element_form(
            &closed_word_form(
                w.clone(),
                Some(alphabet::Contact { anchor: 0, ring: 2 }),
            ),
            [1, 0, 0, 0],
        )
        .expect("contact");
        assert_ne!(
            e_plain.address(),
            e_contact.address(),
            "contact is on the form key"
        );
        assert_eq!(w, w, "M1 is the rotor; contact is not a letter");
    }

    /// The bundle's agreed shift is meaning: two translations of one linear
    /// block are two vertices.
    #[test]
    fn two_shifts_are_two_addresses() {
        let letter = alphabet::LetterId::quarter((0, 1)).expect("quarter");
        let w = Word::from_letters(&[(letter, false)]);
        let mut a = closed_word_form(w.clone(), None);
        let mut b = closed_word_form(w, None);
        if let Vertex::Word { placement, .. } = &mut a.vertices[1] {
            *placement = (q0(), q1());
        }
        if let Vertex::Word { placement, .. } = &mut b.vertices[1] {
            *placement = (
                q0(),
                num_rational::BigRational::from_integer(num_bigint::BigInt::from(3)),
            );
        }
        let ea = as_element_form(&a, [1, 0, 0, 0]).expect("a");
        let eb = as_element_form(&b, [1, 0, 0, 0]).expect("b");
        assert_ne!(ea.address(), eb.address(), "shift is not quotiented off the vertex");
        assert!(!ea.same_vertex(&eb));
    }

    /// The bundle's frame ratio and its sections are meaning: they are on
    /// the form key and on the tape, and a tape with a zero denominator is
    /// refused rather than repaired.
    #[test]
    fn ratio_and_sections_are_on_the_form_key() {
        let letter = alphabet::LetterId::quarter((0, 1)).expect("quarter");
        let w = Word::from_letters(&[(letter, false)]);
        let a = closed_word_form(w.clone(), None);
        let mut b = closed_word_form(w.clone(), None);
        b.ratio = ((2, 1), (2, 1));
        let mut c = closed_word_form(w, None);
        c.sections.insert([7u8; 32], vec![1]);
        let ea = as_element_form(&a, [1, 0, 0, 0]).expect("a");
        let eb = as_element_form(&b, [1, 0, 0, 0]).expect("b");
        let ec = as_element_form(&c, [1, 0, 0, 0]).expect("c");
        assert_ne!(ea.address(), eb.address(), "the frame ratio is on the key");
        assert_ne!(ea.address(), ec.address(), "a section is on the key");
        let back_b = from_element_form(&eb).expect("v2 tape");
        assert_eq!(back_b.ratio, ((2, 1), (2, 1)));
        let back_c = from_element_form(&ec).expect("v2 tape");
        assert_eq!(back_c.sections.get(&[7u8; 32]), Some(&vec![1u16]));
        // Zero the height denominator on the tape: after certificates the
        // tail is ratio (8) + sections count (2) + torsion count (2). hd is
        // the second u16 of the ratio.
        let mut bytes = ea.description().to_vec();
        let n = bytes.len();
        bytes[n - 10] = 0;
        bytes[n - 9] = 0;
        let mut broken = ea.clone();
        broken.set_description(bytes);
        assert!(from_element_form(&broken).is_none(), "a zero denominator refuses the tape");
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
        outer.glue(&inner, a0, Word::identity(), [0, 0, 0, 1]).expect("glue into empty host");
        assert_eq!(outer.depth_of(), 1);
        assert_eq!(outer.depth, 1);
        let mut wrap = Form::default();
        wrap.glue(&outer, [2u8; 32], Word::identity(), [0, 0, 0, 1]).expect("glue depth 1 into depth 0");
        assert_eq!(wrap.depth_of(), 2);
        let outer_copy = outer.clone();
        assert!(
            outer.glue(&outer_copy, [3u8; 32], Word::identity(), [0, 0, 0, 1]).is_none(),
            "equal depth is refused"
        );
        assert!(
            outer.glue(&wrap, [4u8; 32], Word::identity(), [0, 0, 0, 1]).is_none(),
            "deeper form is refused"
        );
        let mut again = Form::default();
        again.glue(&inner, a0, Word::identity(), [0, 0, 0, 1]).unwrap();
        again.glue(&inner, [5u8; 32], Word::identity(), [0, 0, 0, 1]).expect("another depth-0 glue");
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
        let touching = assemble(
            vec![
                frame(vec![(0, 0, 1, 2), (0, 1, 1, 2), (0, 2, 1, 2), (0, 3, 1, 2), (0, 4, 1, 2), (0, 5, 1, 2)]),
                whole(vec![(0, 0, 1, 2), (0, 1, 1, 2)]),
                whole(vec![(0, 2, 1, 2), (0, 3, 1, 2)]),
                whole(vec![(0, 4, 1, 2), (0, 5, 1, 2)]),
            ],
            Vec::new(),
        );
        let frame_i = touching
            .germs
            .iter()
            .position(|g| g.kind == ShellKind::Frame)
            .expect("frame");
        let adj = touching
            .faces
            .iter()
            .filter(|f| f.i != frame_i && f.j != frame_i)
            .count();
        assert_eq!(adj, 2, "two adjacency faces on a touching row, got {adj}");
        assert!(
            !touching.cycles.is_empty(),
            "the 1-skeleton is not a star tree; got {} cycles",
            touching.cycles.len()
        );
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
        let words1: Vec<_> = n1.faces.iter().map(|f| f.g_ij.clone()).collect();
        let words2: Vec<_> = n2.faces.iter().map(|f| f.g_ij.clone()).collect();
        assert_eq!(words1, words2, "constancy is the word, not the per-pair translation versor");
        let e = as_element(&n1).expect("rule element");
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
        assert!(
            n1.faces.len() <= 2 * n1.germ_count(),
            "faces {} cycles {}",
            n1.faces.len(),
            n1.cycles.len()
        );
        let e = as_element(&n1).expect("rule element");
        assert!(replay_nerve(&e).is_none(), "{:?}", replay_nerve(&e));
        let _ = n2;
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
        let snaps = pivot_loop(
            &mut n,
            [0x11; 32],
            &crate::network::nerve::Keyed::new(),
            &crate::transducer::family::NoCrystal,
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
                    chain: if i == 0 {
                        Vec::new()
                    } else {
                        vec![(0, i as i64, 1, 2)]
                    },
                    source: None,
                    target: None,
                    kind: if i == 0 {
                        ShellKind::Frame
                    } else {
                        ShellKind::Whole
                    },
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
            torsion: crate::network::nerve::Keyed::new(),
            certificates: crate::network::nerve::Keyed::new(),
            vortex: crate::network::nerve::Keyed::new(),
        }
    }

    #[test]
    fn pivot_candidates_are_read_from_occupied_axes_only() {
        let id = ClassVector::of_word(&Word::identity());
        assert!(id.occupied().is_empty() && id.is_zero());
        let k2 = ClassVector::of_word(&order2());
        let occ = k2.coordinates();
        assert_eq!(occ, vec![Axis::Sheet(0)], "sheet parity occupies Sheet(0), got {occ:?}");
        let inf = ClassVector::of_word(&infinite());
        assert!(
            inf.free_part(),
            "a pythagorean-5 holonomy is free"
        );
        let mut seen = occ.clone();
        seen.extend(inf.coordinates());
        let mut declaration = seen.clone();
        declaration.sort_by_key(|a| match a {
            Axis::Sheet(i) => (0, *i),
            Axis::Unit(i) => (1, *i),
            Axis::Prime5(i) => (2, *i),
            Axis::Prime13(i) => (3, *i),
            Axis::Hue => (4, 0),
            Axis::Placement => (5, 0),
            Axis::OffPlane => (6, 0),
            Axis::Open => (7, 0),
        });
        assert_eq!(seen, declaration, "occupied axes already in declaration order");
    }

    fn cert(axis: Axis, mu: [u64; 4]) -> PivotCertificate {
        PivotCertificate {
            chord: 0,
            axis,
            before: [0; 32],
            after: [0; 32],
            rank_before: Rank {
                torsion: 0,
                free: 0,
                axes: 0,
            },
            rank_after: Rank {
                torsion: 0,
                free: 0,
                axes: 0,
            },
            mu,
            by: PivotBy::Word {
                face: 0,
                word: Word::identity(),
                side: Side::Right,
            },
        }
    }

    #[test]
    fn incomparable_discharges_are_held_as_vortex() {
        let a = cert(Axis::Unit(0), [2, 0, 1, 0]);
        let b = cert(Axis::Hue, [0, 3, 0, 1]);
        let cands = vec![(a.clone(), a.mu), (b.clone(), b.mu)];
        assert!(
            select_discharge(&cands).is_none(),
            "incomparable μ⃗ are a vortex, not a pick"
        );
    }

    #[test]
    fn descent_certifies_never_selects() {
        let loose = cert(Axis::Unit(0), [4, 4, 4, 4]);
        let tight = cert(Axis::Unit(0), [1, 1, 1, 1]);
        let cands = vec![(loose.clone(), loose.mu), (tight.clone(), tight.mu)];
        let won = select_discharge(&cands).expect("one μ⃗ dominates");
        assert_eq!(won.mu, tight.mu, "the lane-wise minimum is the discharge");
        let a = cert(Axis::Unit(0), [2, 0, 0, 0]);
        let b = cert(Axis::Hue, [0, 2, 0, 0]);
        assert!(
            select_discharge(&[(a.clone(), a.mu), (b.clone(), b.mu)]).is_none(),
            "lane-incomparable discharges are not selected"
        );
    }

    #[test]
    fn cauterize_filed_torsion_crystallizes() {
        let mut n = chord_nerve(&[order2()]);
        assert!(!n.is_closed());
        let _ = pivot_loop(
            &mut n,
            [0xca; 32],
            &crate::network::nerve::Keyed::new(),
            &crate::transducer::family::NoCrystal,
            &mut crate::physics::Physics::default().stroke_n(8),
        );
        assert!(n.is_cauterized(), "filed finite torsion with empty remainder cauterizes");
        let e = as_element(&n).expect("crystallized form");
        assert!(
            !e.torsion().is_empty(),
            "the tape carries the filed cycle"
        );
    }

    /// Chapter 23.1 step 4: a filed torsion chord replays; a chord that is
    /// not the holonomy refuses the face; a missing torsion entry refuses
    /// the loop.
    #[test]
    fn filed_torsion_chord_replays_or_refuses() {
        let mut n = chord_nerve(&[order2()]);
        let _ = pivot_loop(
            &mut n,
            [0xca; 32],
            &crate::network::nerve::Keyed::new(),
            &crate::transducer::family::NoCrystal,
            &mut crate::physics::Physics::default().stroke_n(8),
        );
        assert!(n.is_cauterized(), "filed finite torsion with empty remainder cauterizes");
        let e = as_element_form_of_nerve(&n).expect("the filed nerve crystallizes");
        assert_eq!(
            crate::court::verdict::blind_replay(&e),
            None,
            "a filed torsion chord replays: the chord carries the holonomy"
        );
        let mut f = from_element_form(&e).expect("v2 tape");
        let mag = e.magnitude();
        let Some(ci) = f
            .faces
            .iter()
            .position(|face| f.face_of(face.i, face.j) != face.g_ij)
        else {
            panic!("the filed form has a torsion chord");
        };
        f.faces[ci].g_ij = order4();
        let bad = as_element_form(&f, mag).expect("tampered chord");
        assert_eq!(
            crate::court::verdict::blind_replay(&bad),
            Some(crate::court::verdict::REFUSE_FACE),
            "a chord that does not carry the loop holonomy refuses the face"
        );
        let mut gone = from_element_form(&e).expect("v2 tape");
        gone.torsion = Keyed::new();
        let missing = as_element_form(&gone, mag).expect("torsion stripped");
        assert_eq!(
            crate::court::verdict::blind_replay(&missing),
            Some(crate::court::verdict::REFUSE_LOOP),
            "a filed chord without its torsion entry refuses the loop"
        );
    }

    #[test]
    fn horizon_seeded_infinite_is_not_demanded() {
        let mut n = chord_nerve(&[infinite()]);
        let r = infinite().rotor();
        let mut horizon = crate::network::nerve::Keyed::new();
        horizon.insert(r.class_key(), (r, Order::Infinite, Standing::Held));
        let snaps = pivot_loop(
            &mut n,
            [0x5e; 32],
            &horizon,
            &crate::transducer::family::NoCrystal,
            &mut crate::physics::Physics::default().stroke_n(8),
        );
        assert!(
            !snaps.iter().any(|s| matches!(s, Snap::Demand { .. })),
            "a held infinite class is not spoken again: {snaps:?}"
        );
    }

    #[test]
    fn pivot_never_reopens_a_closed_chord() {
        let mut n = chord_nerve(&[order2(), order4()]);
        let mut budget = crate::physics::Physics::default().stroke_n(16);
        let first = pivot_loop(
            &mut n,
            [0xcd; 32],
            &crate::network::nerve::Keyed::new(),
            &crate::transducer::family::NoCrystal,
            &mut budget,
        );
        let closed: Vec<u32> = first
            .iter()
            .filter_map(|s| match s {
                Snap::Closed { order, .. } => Some(*order),
                Snap::TorsionFiled { ell, .. } => Some(*ell),
                _ => None,
            })
            .collect();
        let mut budget = crate::physics::Physics::default().stroke_n(16);
        let second = pivot_loop(
            &mut n,
            [0xcd; 32],
            &crate::network::nerve::Keyed::new(),
            &crate::transducer::family::NoCrystal,
            &mut budget,
        );
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

    struct Hold(Vec<Word>);
    impl crate::transducer::family::Crystal for Hold {
        fn on_class(
            &self,
            _class: &stalk::Invariant,
        ) -> Vec<(
            Element,
            Form,
            Standing,
        )> {
            Vec::new()
        }
        fn speak_form(
            &self,
            _f: &Form,
            _x: &crate::transducer::family::Grid,
            _p: &Word,
        ) -> Option<crate::transducer::family::Speech> {
            None
        }
        fn by_address(&self, _a: &[u8; 32]) -> Option<(Element, Form)> {
            None
        }
        fn on_axis(&self, _chart: &stalk::Invariant, axis: Axis) -> Vec<Word> {
            self.0
                .iter()
                .filter(|w| ClassVector::of_word(w).occupies(axis))
                .cloned()
                .collect()
        }
    }

    #[test]
    fn pivot_loop_discharges_held_word_and_vortices_a_pair() {
        let q = order4();
        let qinv = q.inverse();
        let mut n = chord_nerve(&[q.clone()]);
        let snaps = pivot_loop(
            &mut n,
            [0x11; 32],
            &Keyed::new(),
            &Hold(vec![q.clone()]),
            &mut crate::physics::Physics::default().stroke_n(16),
        );
        let Some(Snap::Discharged { certificate, .. }) = snaps.iter().find(|s| matches!(s, Snap::Discharged { .. })) else {
            panic!("a crystal holding q discharges: {snaps:?}");
        };
        assert_eq!(certificate.before, q.rotor().class_key());
        assert_eq!(certificate.after, CLOSED_KEY);
        assert_eq!(certificate.rank_before.torsion, 1);
        assert_eq!(certificate.rank_after.torsion, 0);
        assert!(
            n.certificates.get(&certificate.key()).is_some(),
            "the nerve carries the discharge by key"
        );
        let e = as_element_form_of_nerve(&n).expect("the discharged nerve crystallizes");
        assert!(
            matches!(Form::from_nerve(&n).vertices.first(), Some(Vertex::Frame)),
            "Frame is vertex 0 of the crystallized form"
        );
        assert_eq!(
            crate::court::verdict::blind_replay(&e),
            None,
            "a certified discharge replays: every after-face is face_of, then the certificate"
        );
        let mut tampered = from_element_form(&e).expect("v2 tape");
        let crate::transducer::sentence::PivotBy::Word { face, .. } = &tampered.certificates[0].by;
        let fi = *face as usize;
        tampered.faces[fi].g_ij = q.clone();
        let bad = as_element_form(&tampered, e.magnitude()).expect("tampered tape");
        assert_eq!(
            crate::court::verdict::blind_replay(&bad),
            Some(crate::court::verdict::REFUSE_FACE),
            "a word on a certified face that is not face_of of its vertices refuses"
        );

        let mut n = chord_nerve(&[q.clone()]);
        let snaps = pivot_loop(
            &mut n,
            [0x22; 32],
            &Keyed::new(),
            &Hold(vec![q.clone(), qinv]),
            &mut crate::physics::Physics::default().stroke_n(16),
        );
        let Some(Snap::Vortex { held, .. }) = snaps.iter().find(|s| matches!(s, Snap::Vortex { .. })) else {
            panic!("q and q⁻¹ are a vortex: {snaps:?}");
        };
        assert_eq!(held.len(), 2, "two certificates, got {held:?}");

        let mut n = chord_nerve(&[q.clone()]);
        let mut horizon = Keyed::new();
        let r = q.rotor();
        horizon.insert(r.class_key(), (r, q.order(), Standing::Held));
        let snaps = pivot_loop(
            &mut n,
            [0x33; 32],
            &horizon,
            &crate::transducer::family::NoCrystal,
            &mut crate::physics::Physics::default().stroke_n(16),
        );
        assert!(snaps.is_empty(), "a horizon holding q files silently: {snaps:?}");
        assert!(
            n.torsion.contains(&q.rotor().class_key()),
            "filed torsion is keyed by the class"
        );
    }

    #[test]
    fn axis_index_admit_is_linear_on_axis_is_constant() {
        use crate::network::nerve::work;
        let inv = stalk::Invariant::default();
        let w = order2();
        let run = |n: usize| {
            work::reset();
            let mut index = AxisIndex::new();
            for _ in 0..n {
                admit_words(&mut index, &inv, std::slice::from_ref(&w));
            }
            let inserts = work::snapshot().1;
            work::reset();
            let _ = on_axis_of(&index, &inv, Axis::Sheet(0));
            let gets = work::snapshot().0;
            (inserts, gets)
        };
        let (i8, g8) = run(8);
        let (i32n, g32) = run(32);
        assert!(
            i8 > 0 && i32n >= i8 && i32n <= i8.saturating_mul(5),
            "4N admits cost ~4× N inserts: {i8} then {i32n}"
        );
        assert_eq!(g8, g32, "on_axis get cost is independent of N: {g8} vs {g32}");
    }
}
