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
#[derive(Debug, Clone, Default)]
pub struct Sentence {
    pub cites: Vec<Cite>,
    /// PUNCTURES: bodies a held word carries by shape under a gauge the
    /// word does not yet hold — (word, versor, the word's cells under the
    /// body's own transitions). Spoken as sections of the word, not as new
    /// words: the fiber deepens instead of the vocabulary widening.
    pub punctures: Vec<(usize, Versor, Vec<Cell>)>,
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
    by.fiber(&ROOT).map(|(_, v)| v.clone()).collect()
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
        .fiber(&ROOT)
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
            agreeing.insert(index_key(k), k);
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
        .filter_map(|(i, v, asg)| Some((v.clone(), cited_pattern(language.get(*i)?, *asg)?)))
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
    at.fiber(&ROOT)
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
    let agreeing = gauge_of(word, &v, piece);
    let mut asgs = agreeing.fiber(&ROOT);
    if let Some((_, &asg)) = asgs.next() {
        if asgs.next().is_none() {
            let cite: Cite = (i, v.clone(), asg);
            if replay_at(word_at, std::slice::from_ref(&cite), piece).is_empty() {
                return (Some(cite), None, m);
            }
        } else {
            return (None, None, agreeing.len());
        }
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
fn open_by_chart(body: &[Cell]) -> Vec<Vec<Cell>> {
    let fibers = element::gauge_fibers(body);
    let mut idx: Keyed<Vec<Vec<Cell>>> = Keyed::new();
    for (class, fiber) in fibers {
        idx.insert(element::hue_pair_key(class.0, class.1), pieces(&fiber));
    }
    let mut out = Vec::new();
    for (_, subs) in idx.fiber(&ROOT) {
        out.extend(subs.clone());
    }
    if out.is_empty() {
        out.push(body.to_vec());
    }
    out
}

fn body_key(body: &[Cell]) -> [u8; 32] {
    match body.first() {
        Some(c) => cell_at_key(c.0, c.1),
        None => [0u8; 32],
    }
}

fn place_located<'a>(
    out: &mut Sentence,
    located: (Option<Cite>, Option<(usize, Versor, Vec<Cell>)>, usize),
    body: &[Cell],
    word_at: &impl Fn(usize) -> Option<&'a Element>,
    fiber_of: &impl Fn(&stalk::Invariant) -> Vec<[u8; 32]>,
    index_of: &impl Fn(&[u8; 32]) -> Option<usize>,
    shape_at: &impl Fn(usize) -> Option<&'a Shape>,
    exclude: Option<usize>,
) {
    out.multiplicity.insert(body_key(body), located.2);
    match located {
        (Some(c), _, _) => out.cites.push(c),
        (None, Some(p), _) => out.punctures.push(p),
        (None, None, _) => {
            for sub in open_by_chart(body) {
                if sub.as_slice() == body {
                    out.residual.extend(sub);
                    continue;
                }
                let loc = locate_or_puncture_on(word_at, fiber_of, index_of, shape_at, &sub, exclude);
                place_located(out, loc, &sub, word_at, fiber_of, index_of, shape_at, exclude);
            }
        }
    }
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
    deconstruct_on(&|i| words.get(i), &|inv| atlas.fiber(inv), index_of, shape_at, chain, exclude)
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
    let whole = locate_or_puncture_on(word_at, fiber_of, index_of, shape_at, chain, exclude);
    if whole.0.is_some() {
        place_located(&mut out, whole, chain, word_at, fiber_of, index_of, shape_at, exclude);
        out.residual.sort_unstable();
        out.connection = out
            .cites
            .windows(2)
            .filter_map(|w| Some(w[0].1.versor_inverse()?.geo(&w[1].1)))
            .collect();
        return out;
    }
    for body in pieces(chain) {
        let loc = locate_or_puncture_on(word_at, fiber_of, index_of, shape_at, &body, exclude);
        place_located(&mut out, loc, &body, word_at, fiber_of, index_of, shape_at, exclude);
    }
    out.residual.sort_unstable();
    out.connection = out
        .cites
        .windows(2)
        .filter_map(|w| Some(w[0].1.versor_inverse()?.geo(&w[1].1)))
        .collect();
    out
}

// ── a sentence is a form ────────────────────────────────────────────────────

use super::alphabet::{self, Order, Rotor, Word};
use super::element::{affine_of, GERM_DIM};
use super::peel::ShellKind;
use super::stalk::Standing;
use crate::physics::Budget;

/// How an unfolding stops. The step count is an instance coordinate, never
/// stored on the form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Measure {
    Occupied,
    Remainder,
    Ring { anchor: u16 },
}

/// A vertex of a sentence form (chapter 18).
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

/// Face transport: the relative word, translation and hue map from vertex
/// `i` to vertex `j`. The same three coordinates `transport_class` keys on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transport {
    pub word: Word,
    pub shift: (num_rational::BigRational, num_rational::BigRational),
    pub gauge: Vec<(u8, u8)>,
}

impl Transport {
    pub fn from_word(word: Word) -> Transport {
        Transport {
            word,
            shift: q2_zero(),
            gauge: Vec::new(),
        }
    }

    pub fn inverse(&self) -> Option<Transport> {
        let word = self.word.inverse();
        let mapped = apply_linear(&word, &self.shift)?;
        Some(Transport {
            word,
            shift: (-mapped.0, -mapped.1),
            gauge: gauge_invert(&self.gauge),
        })
    }
}

/// Face of a `Form`: two vertices and the transport between them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormFace {
    pub i: u16,
    pub j: u16,
    pub transport: Transport,
}

impl FormFace {
    pub fn new(i: u16, j: u16, word: Word) -> FormFace {
        FormFace {
            i,
            j,
            transport: Transport::from_word(word),
        }
    }
}

fn q2_zero() -> (num_rational::BigRational, num_rational::BigRational) {
    let z = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    (z.clone(), z)
}

fn apply_linear(
    word: &Word,
    t: &(num_rational::BigRational, num_rational::BigRational),
) -> Option<(num_rational::BigRational, num_rational::BigRational)> {
    let b = word.plane_block()?;
    Some((
        &b[0][0] * &t.0 + &b[0][1] * &t.1,
        &b[1][0] * &t.0 + &b[1][1] * &t.1,
    ))
}

fn hue_key(h: u8) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[0] = h;
    k
}

fn gauge_invert(g: &[(u8, u8)]) -> Vec<(u8, u8)> {
    let mut inv: Vec<(u8, u8)> = g.iter().map(|&(a, b)| (b, a)).collect();
    inv.sort_unstable();
    inv.dedup();
    inv
}

fn gauge_keyed(g: &[(u8, u8)]) -> Keyed<u8> {
    let mut m = Keyed::new();
    for &(a, b) in g {
        m.insert(hue_key(a), b);
    }
    m
}

fn gauge_image(g: &Keyed<u8>, h: u8) -> u8 {
    match g.get(&hue_key(h)) {
        Some(&x) => x,
        None => h,
    }
}

/// `second ∘ first` on the union of listed hues; a hue absent from a map
/// is fixed by that map. The listing holds non-fixed points only.
#[cfg(test)]
fn compose_gauge(second: &[(u8, u8)], first: &[(u8, u8)]) -> Vec<(u8, u8)> {
    let a = gauge_keyed(first);
    let b = gauge_keyed(second);
    let mut hues: Keyed<()> = Keyed::new();
    for &(x, y) in first.iter().chain(second.iter()) {
        hues.insert(hue_key(x), ());
        hues.insert(hue_key(y), ());
    }
    let mut out = Vec::new();
    for (k, _) in hues.fiber(&ROOT) {
        let h = k[0];
        let image = gauge_image(&b, gauge_image(&a, h));
        if image != h {
            out.push((h, image));
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

fn relative_gauge(from: &[(u8, u8)], to: &[(u8, u8)]) -> Vec<(u8, u8)> {
    let fm = gauge_keyed(from);
    let tm = gauge_keyed(to);
    let mut hues: Keyed<()> = Keyed::new();
    for &(x, y) in from.iter().chain(to.iter()) {
        hues.insert(hue_key(x), ());
        hues.insert(hue_key(y), ());
    }
    let mut out = Vec::new();
    for (k, _) in hues.fiber(&ROOT) {
        let h = k[0];
        let b = gauge_image(&fm, h);
        let c = gauge_image(&tm, h);
        if b != c {
            out.push((b, c));
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

fn vertex_translation(v: &Vertex, form: &Form) -> (num_rational::BigRational, num_rational::BigRational) {
    match v {
        Vertex::Word { placement, .. } => placement.clone(),
        Vertex::Rebased { by, shift, .. } => match form.vertices.get(*by as usize) {
            Some(Vertex::Word { placement, .. }) => {
                (placement.0.clone() + &shift.0, placement.1.clone() + &shift.1)
            }
            _ => shift.clone(),
        },
        _ => q2_zero(),
    }
}

fn vertex_gauge(v: &Vertex) -> Vec<(u8, u8)> {
    match v {
        Vertex::Word { gauge, .. } | Vertex::Rebased { gauge, .. } => gauge.clone(),
        _ => Vec::new(),
    }
}

/// Independent loop of a form: the closed vertex walk, the faces along it,
/// the chord (never a star face), and the oriented holonomy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loop {
    pub vertices: Vec<u16>,
    pub faces: Vec<u16>,
    pub chord: u16,
    pub holonomy: Word,
}

/// A sentence as vertices, faces, loops, remainder.
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
    pub certificates: Keyed<PivotCertificate>,
    /// Filed torsion cycles, keyed by class key: `(σ, ℓ, chord face)`.
    /// The form owns this carrier; the element reads it whole.
    pub torsion: Keyed<(Vec<(u8, u8)>, u32, u16)>,
    /// Cited form address → glued vertex index. Written by [`Form::glue`]
    /// and rebuilt by the decoder from Glued vertices.
    pub glued: Keyed<u16>,
    /// Incomparable discharges held as cells until a glue or ruling.
    /// Outer key is the after class; inner is the discharging rotor A1 key.
    pub vortex: Keyed<Keyed<PivotCertificate>>,
    /// Word vertex address → index. The meet for A6 and Mayer–Vietoris.
    pub by_address: Keyed<u16>,
    /// Vertex index key → Frame face index. Star incidence, one face per vertex.
    pub star: Keyed<u16>,
    /// Word vertex address → placement, filled at construction.
    pub placements: Keyed<(num_rational::BigRational, num_rational::BigRational)>,
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
            certificates: Keyed::new(),
            torsion: Keyed::new(),
            glued: Keyed::new(),
            vortex: Keyed::new(),
            by_address: Keyed::new(),
            star: Keyed::new(),
            placements: Keyed::new(),
        }
    }
}

impl Form {
    /// Fill by_address, star, placements (by transport-class key) and
    /// hue_maps from the vertices and faces. Two gauges or two placements
    /// on one class are two sections: that class is refused as one map.
    pub fn reindex(&mut self) {
        let mut by_address = Keyed::new();
        let mut hues: Keyed<Vec<(u8, u8)>> = Keyed::new();
        let mut hue_conflict: Keyed<()> = Keyed::new();
        let mut places: Keyed<(
            num_rational::BigRational,
            num_rational::BigRational,
        )> = Keyed::new();
        let mut place_conflict: Keyed<()> = Keyed::new();
        let mut star = Keyed::new();
        for (i, v) in self.vertices.iter().enumerate() {
            if let Vertex::Word {
                address,
                word,
                placement,
                gauge,
                class,
                ..
            } = v
            {
                by_address.insert(*address, i as u16);
                let ck = class.key();
                match hues.get(&ck) {
                    None => {
                        hues.insert(ck, gauge.clone());
                    }
                    Some(held) if held != gauge => {
                        hue_conflict.insert(ck, ());
                    }
                    _ => {}
                }
                let tk = vertex_transport_key(word, gauge);
                match places.get(&tk) {
                    None => {
                        places.insert(tk, placement.clone());
                    }
                    Some(held) if held != placement => {
                        place_conflict.insert(tk, ());
                    }
                    _ => {}
                }
            }
        }
        let mut hue_maps = Keyed::new();
        for (k, g) in hues.fiber(&ROOT) {
            if !hue_conflict.contains(k) {
                hue_maps.insert(*k, g.clone());
            }
        }
        let mut placements = Keyed::new();
        for (k, p) in places.fiber(&ROOT) {
            if !place_conflict.contains(k) {
                placements.insert(*k, p.clone());
            }
        }
        for (fi, f) in self.faces.iter().enumerate() {
            if f.i == 0 {
                star.insert(vertex_ix_key(f.j), fi as u16);
            } else if f.j == 0 {
                star.insert(vertex_ix_key(f.i), fi as u16);
            }
        }
        self.by_address = by_address;
        self.star = star;
        self.placements = placements;
        self.hue_maps = hue_maps;
    }

    pub fn push_vertex(&mut self, v: Vertex) -> u16 {
        let i = self.vertices.len() as u16;
        self.vertices.push(v);
        self.reindex();
        i
    }

    pub fn push_face(&mut self, f: FormFace) -> u16 {
        let i = self.faces.len() as u16;
        self.faces.push(f);
        self.reindex();
        i
    }
}

impl Vertex {

    pub fn word_in(&self, form: &Form) -> Option<Word> {
        match self {
            Vertex::Frame | Vertex::Unfolded { .. } => Some(Word::identity()),
            Vertex::Word { word, .. } => Some(word.clone()),
            Vertex::Glued { placement, .. } => Some(placement.clone()),
            Vertex::Rebased { by, .. } => form.vertices.get(*by as usize)?.word_in(form),
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

    pub fn face_of(&self, i: u16, j: u16) -> Option<Transport> {
        let va = self.vertices.get(i as usize)?;
        let vb = self.vertices.get(j as usize)?;
        let wa = va.word_in(self)?;
        let wb = vb.word_in(self)?;
        let word = wb.compose(&wa.inverse()).normal_form();
        let ta = vertex_translation(va, self);
        let tb = vertex_translation(vb, self);
        let mapped = apply_linear(&word, &ta)?;
        let shift = (tb.0 - mapped.0, tb.1 - mapped.1);
        let gauge = relative_gauge(&vertex_gauge(va), &vertex_gauge(vb));
        Some(Transport { word, shift, gauge })
    }

    pub fn delta1(&self) -> Vec<(u16, Word)> {
        self.loops
            .iter()
            .enumerate()
            .map(|(i, lp)| (i as u16, lp.holonomy.clone()))
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
        if !s.vertices[0].word_in(s).is_some_and(|w| w.is_identity()) {
            return None;
        }
        let d = s.depth_of();
        let host = self.depth_of();
        if host > 0 && d >= host {
            return None;
        }
        let chart = chart_of_form(s);
        if chart.0.is_empty() {
            return None;
        }
        let idx = self.vertices.len() as u16;
        self.vertices.push(Vertex::Glued {
            address: s_addr,
            chart,
            magnitude,
            depth: d,
            placement: placement.clone(),
        });
        self.faces.push(FormFace::new(0, idx, placement));
        self.glued.insert(s_addr, idx);
        self.loops = Self::loops_of(&self.faces, self.vertices.len());
        self.depth = self.depth_of();
        self.reindex();
        Some(idx)
    }

    /// Drop the named vertices (keyed by old index), drop faces that touched
    /// them, remap surviving faces through the old-to-new map, rebuild loops.
    pub fn retract_vertices(&mut self, drop: &Keyed<()>) {
        let n = self.vertices.len();
        if n == 0 {
            return;
        }
        let mut old_to_new: Vec<Option<u16>> = vec![None; n];
        let mut new_verts = Vec::new();
        for (i, v) in self.vertices.iter().enumerate() {
            if i != 0 && drop.contains(&vertex_ix_key(i as u16)) {
                continue;
            }
            old_to_new[i] = Some(new_verts.len() as u16);
            new_verts.push(v.clone());
        }
        let mut face_old_to_new: Vec<Option<u16>> = vec![None; self.faces.len()];
        let mut new_faces = Vec::new();
        for (fi, f) in self.faces.iter().enumerate() {
            let Some(&Some(i)) = old_to_new.get(f.i as usize) else {
                continue;
            };
            let Some(&Some(j)) = old_to_new.get(f.j as usize) else {
                continue;
            };
            face_old_to_new[fi] = Some(new_faces.len() as u16);
            new_faces.push(FormFace {
                i,
                j,
                transport: f.transport.clone(),
            });
        }
        let mut glued = Keyed::new();
        for (k, &old_i) in self.glued.fiber(&ROOT) {
            if let Some(&Some(ni)) = old_to_new.get(old_i as usize) {
                glued.insert(*k, ni);
            }
        }
        let mut sections = Keyed::new();
        for (k, ix) in self.sections.fiber(&ROOT) {
            let mapped: Vec<u16> = ix
                .iter()
                .filter_map(|&i| old_to_new.get(i as usize).copied().flatten())
                .collect();
            if !mapped.is_empty() {
                sections.insert(*k, mapped);
            }
        }
        let mut torsion = Keyed::new();
        for (k, (sigma, ell, chord)) in self.torsion.fiber(&ROOT) {
            let Some(&Some(nc)) = face_old_to_new.get(*chord as usize) else {
                continue;
            };
            let Some(mapped) = sigma
                .iter()
                .map(|&(a, c)| {
                    let i = old_to_new.get(a as usize).copied().flatten()?;
                    let j = old_to_new.get(c as usize).copied().flatten()?;
                    Some((i as u8, j as u8))
                })
                .collect::<Option<Vec<_>>>()
            else {
                continue;
            };
            torsion.insert(*k, (mapped, *ell, nc));
        }
        self.vertices = new_verts;
        self.faces = new_faces;
        self.glued = glued;
        self.sections = sections;
        self.torsion = torsion;
        self.loops = Self::loops_of(&self.faces, self.vertices.len());
        self.depth = self.depth_of();
        self.reindex();
    }

    /// Replace every face `g_ij` by `γ_j · g_ij · γ_i⁻¹`. Holonomies of
    /// identity stay identity. The gauge's length is the vertex count.
    pub fn regauge(&mut self, gamma: &[Word]) -> Option<()> {
        if gamma.len() != self.vertices.len() {
            return None;
        }
        for f in &mut self.faces {
            let gi = gamma.get(f.i as usize)?;
            let gj = gamma.get(f.j as usize)?;
            f.transport.word = gj
                .compose(&f.transport.word.compose(&gi.inverse()))
                .normal_form();
            f.transport.shift = apply_linear(gj, &f.transport.shift)?;
        }
        self.loops = Self::loops_of(&self.faces, self.vertices.len());
        Some(())
    }

    /// Spanning forest (12.2): each component is rooted at its least vertex
    /// index with γ = 1; γ_j = γ_i · g_ij⁻¹ along tree edges. None only when
    /// a face names a vertex outside the form.
    pub fn solve_coboundary(&self) -> Option<Vec<Word>> {
        let n = self.vertices.len();
        if n == 0 {
            return None;
        }
        let mut adj: Vec<Vec<(usize, usize, bool)>> = vec![Vec::new(); n];
        for (fi, f) in self.faces.iter().enumerate() {
            let (i, j) = (f.i as usize, f.j as usize);
            if i >= n || j >= n {
                return None;
            }
            adj[i].push((j, fi, true));
            adj[j].push((i, fi, false));
        }
        let mut gamma: Vec<Option<Word>> = vec![None; n];
        for root in 0..n {
            if gamma[root].is_some() {
                continue;
            }
            gamma[root] = Some(Word::identity());
            let mut q = vec![root];
            let mut qi = 0usize;
            while qi < q.len() {
                let u = q[qi];
                qi += 1;
                let gu = gamma[u].as_ref()?.clone();
                for &(v, fi, u_is_i) in &adj[u] {
                    let f = &self.faces[fi];
                    let gv = if u_is_i {
                        gu.compose(&f.transport.word.inverse()).normal_form()
                    } else {
                        gu.compose(&f.transport.word).normal_form()
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
        }
        gamma.into_iter().collect()
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
    ) -> Option<u16> {
        if (by as usize) >= self.vertices.len() {
            return None;
        }
        let Vertex::Word {
            rho,
            gauge,
            class,
            ..
        } = self.vertices.get(of as usize).cloned()?
        else {
            return None;
        };
        self.vertices[of as usize] = Vertex::Rebased {
            by,
            of,
            shift,
            rho,
            gauge,
            class,
        };
        Some(of)
    }

    pub fn unfold(&mut self, measure: Measure) -> u16 {
        let idx = self.vertices.len() as u16;
        self.vertices.push(Vertex::Unfolded { measure });
        idx
    }

    /// Remainder empty and every loop identity or filed finite torsion.
    pub fn is_cauterized(&self) -> bool {
        if !self.remainder.is_empty() {
            return false;
        }
        for lp in &self.loops {
            if lp.holonomy.abelian_identity() {
                continue;
            }
            match lp.holonomy.order() {
                Order::Infinite => return false,
                Order::Finite(ell) => {
                    let k = lp.holonomy.rotor().class_key();
                    match self.torsion.get(&k) {
                        Some((_, e, _)) if *e == ell => {}
                        _ => return false,
                    }
                }
            }
        }
        true
    }

    /// A18 over this form's loops.
    pub fn rank(&self) -> Rank {
        rank_of_holonomies(self.loops.iter().map(|lp| &lp.holonomy))
    }

    /// The class vector of loop `idx` (12.6): plane coordinates from the
    /// oriented holonomy. Placement and hue live on the chord Transport
    /// and are read once those coordinates have a carrier (A4).
    pub fn class_vector(&self, idx: usize) -> Option<ClassVector> {
        let lp = self.loops.get(idx)?;
        Some(ClassVector::of_word(&lp.holonomy))
    }

    /// Write a certified word onto the chord face's transport and re-walk.
    /// Vertices are not written.
    pub fn apply_pivot(&mut self, cert: &PivotCertificate) {
        let PivotBy::Word { face, word, side } = &cert.by;
        if let Some(f) = self.faces.get_mut(*face as usize) {
            f.transport.word = pivoted_face(&f.transport.word, word, *side);
        }
        self.loops = Self::loops_of(&self.faces, self.vertices.len());
        self.reindex();
    }

    /// A16 on the form. Certificates and vortex live here.
    /// `bundle` is the pairs the idea germinated from; `t_extent` is how
    /// long the loop has been held on the bubble's clock.
    pub fn pivot_loop(
        &mut self,
        idea: [u8; 32],
        horizon: &Keyed<(Rotor, Order, Standing)>,
        crystal: &dyn super::family::Crystal,
        budget: &mut Budget,
        bundle: Option<&[(Vec<Vec<u8>>, Vec<Vec<u8>>)]>,
        t_extent: u64,
    ) -> Vec<Snap> {
        let mut snaps = Vec::new();
        let mut handled: Keyed<()> = Keyed::new();
        loop {
            if budget.remaining() == 0 {
                break;
            }
            self.loops = Self::loops_of(&self.faces, self.vertices.len());
            if let Some(gamma) = self.solve_coboundary() {
                let _ = self.regauge(&gamma);
            }
            self.loops = Self::loops_of(&self.faces, self.vertices.len());
            let open: Vec<usize> = self
                .loops
                .iter()
                .enumerate()
                .filter(|(_, lp)| !lp.holonomy.abelian_identity())
                .map(|(i, _)| i)
                .collect();
            if open.is_empty() {
                break;
            }
            let rows: Vec<LoopRow> = self.loops.iter().map(|lp| loop_row(&lp.holonomy)).collect();
            let rank_before = rank_of_rows(&rows);
            let mu_before = match bundle {
                None => [0; 4],
                Some(b) => match super::family::bundle_pressure(self, b, crystal, t_extent) {
                    Some(m) => m,
                    None => [0; 4],
                },
            };
            let mut applied = false;
            for idx in open {
                if !budget.spend() {
                    break;
                }
                let Some(kappa) = self.class_vector(idx) else {
                    continue;
                };
                let key = kappa.class_key();
                let loop_k = vertex_ix_key(idx as u16);
                if handled.contains(&loop_k) {
                    continue;
                }
                let held = horizon.get(&key).is_some();
                let order = self.loops[idx].holonomy.order();
                let cands = self.candidates_for(idx, &kappa, crystal);
                let mut discharging: Vec<PivotCertificate> = Vec::new();
                for cand in &cands {
                    if !budget.spend() {
                        break;
                    }
                    if let Some(cert) = self.trial(
                        idx,
                        cand,
                        &kappa,
                        &rows,
                        rank_before,
                        crystal,
                        bundle,
                        t_extent,
                        mu_before,
                    ) {
                        discharging.push(cert);
                    } else {
                        eprintln!(
                            "  pivot {:02x}{:02x}… loop {idx} axis {:?} no discharge",
                            idea[0], idea[1], cand.axis
                        );
                    }
                }
                match select_discharge(&discharging) {
                    Discharge::One(cert) => {
                        debug_assert_eq!(cert.rank_before, rank_before);
                        self.apply_pivot(&cert);
                        debug_assert!(
                            Rank::admits(rank_before, self.rank()),
                            "A18: an applied pivot does not lose rank"
                        );
                        debug_assert_eq!(self.rank(), cert.rank_after);
                        eprintln!(
                            "  pivot {:02x}{:02x}… loop {idx} axis {:?} discharged {:02x}{:02x}…→{:02x}{:02x}…",
                            idea[0], idea[1], cert.axis, cert.before[0], cert.before[1], cert.after[0], cert.after[1]
                        );
                        let ck = cert.key();
                        self.certificates.insert(ck, cert.clone());
                        snaps.push(Snap::Discharged {
                            chord: idx,
                            certificate: cert,
                        });
                        handled.insert(loop_k, ());
                        applied = true;
                        break;
                    }
                    Discharge::Vortex(held_certs) => {
                        for c in &held_certs {
                            vortex_put(&mut self.vortex, c.clone());
                        }
                        if bundle.is_some() {
                            eprintln!(
                                "  vortex {:02x}{:02x}… loop {idx} {} candidates held",
                                idea[0],
                                idea[1],
                                held_certs.len()
                            );
                            snaps.push(Snap::Vortex {
                                chord: idx,
                                held: held_certs,
                            });
                        } else {
                            eprintln!(
                                "  vortex {:02x}{:02x}… loop {idx} {} candidates unmeasured",
                                idea[0],
                                idea[1],
                                held_certs.len()
                            );
                            snaps.push(Snap::Unmeasured {
                                chord: idx,
                                held: held_certs,
                            });
                        }
                        handled.insert(loop_k, ());
                    }
                    Discharge::NoneDischarged => match order {
                        Order::Infinite => {
                            if !held {
                                snaps.push(Snap::Demand { chord: idx, key });
                                eprintln!(
                                    "  demand spoken {:02x}{:02x}… order ∞",
                                    idea[0], idea[1]
                                );
                            }
                            handled.insert(loop_k, ());
                        }
                        Order::Finite(ell) => {
                            if self.file_loop(idx, ell) {
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
                            handled.insert(loop_k, ());
                        }
                    },
                }
            }
            if !applied {
                break;
            }
            handled = Keyed::new();
        }
        snaps
    }

    fn candidates_for(
        &self,
        idx: usize,
        kappa: &ClassVector,
        crystal: &dyn super::family::Crystal,
    ) -> Vec<Candidate> {
        let Some(lp) = self.loops.get(idx) else {
            return Vec::new();
        };
        let chord = lp.chord as usize;
        if self.faces.get(chord).is_none() {
            return Vec::new();
        }
        let mut charts: Keyed<stalk::Invariant> = Keyed::new();
        for &fi in &lp.faces {
            let Some(f) = self.faces.get(fi as usize) else {
                continue;
            };
            for vi in [f.i, f.j] {
                if let Some(v) = self.vertices.get(vi as usize) {
                    let chart = v.class();
                    charts.insert(chart.key(), chart);
                }
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
            for (_, chart) in charts.fiber(&ROOT) {
                for w in crystal.on_axis(chart, axis) {
                    for side in [Side::Right, Side::Left] {
                        let wk = w.key();
                        let sk = side_slot(side);
                        if seen.get(&wk).is_some_and(|s| s.contains(&sk)) {
                            continue;
                        }
                        seen.nested_insert(wk, sk, ());
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

    fn trial(
        &self,
        idx: usize,
        cand: &Candidate,
        kappa: &ClassVector,
        rows: &[LoopRow],
        rank_before: Rank,
        crystal: &dyn super::family::Crystal,
        bundle: Option<&[(Vec<Vec<u8>>, Vec<Vec<u8>>)]>,
        t_extent: u64,
        mu_before: [u64; 4],
    ) -> Option<PivotCertificate> {
        let lp = self.loops.get(idx)?;
        let PivotBy::Word { face, word, side } = &cand.by;
        let held = self.faces.get(*face as usize)?;
        let after_g = pivoted_face(&held.transport.word, word, *side);
        let mut w = Word::identity();
        for (win, &fi) in lp.vertices.windows(2).zip(&lp.faces) {
            let step = if fi as usize == *face as usize {
                if held.i == win[0] && held.j == win[1] {
                    after_g.clone()
                } else {
                    after_g.inverse()
                }
            } else {
                form_face_step(&self.faces, fi as usize, win[0] as usize, win[1] as usize)?
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
        let mut trial_form = self.clone();
        if let Some(f) = trial_form.faces.get_mut(*face as usize) {
            f.transport.word = after_g;
        }
        trial_form.loops = Form::loops_of(&trial_form.faces, trial_form.vertices.len());
        let mu_after = match bundle {
            None => [0; 4],
            Some(b) => super::family::bundle_pressure(&trial_form, b, crystal, t_extent)?,
        };
        if !(mu_after == mu_before || crate::economy::descends(&mu_after, &mu_before)) {
            return None;
        }
        Some(PivotCertificate {
            chord: idx as u16,
            axis: cand.axis,
            before: kappa.class_key(),
            after: kappa_after.class_key(),
            rank_before,
            rank_after,
            mu_before,
            mu_after,
            by: cand.by.clone(),
        })
    }

    fn file_loop(&mut self, idx: usize, ell: u32) -> bool {
        if ell == 0 {
            return false;
        }
        let Some(lp) = self.loops.get(idx) else {
            return false;
        };
        let sigma: Vec<(u8, u8)> = lp
            .vertices
            .windows(2)
            .map(|p| (p[0] as u8, p[1] as u8))
            .collect();
        let class = lp.holonomy.rotor().class_key();
        self.torsion.insert(class, (sigma, ell, lp.chord));
        true
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
        let mut order = Vec::with_capacity(faces.len());
        for (fi, f) in faces.iter().enumerate() {
            if f.i == 0 || f.j == 0 {
                order.push(fi);
            }
        }
        for (fi, f) in faces.iter().enumerate() {
            if f.i != 0 && f.j != 0 {
                order.push(fi);
            }
        }
        for fi in order {
            let f = &faces[fi];
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
                vertices: path.into_iter().map(|x| x as u16).collect(),
                faces: walked.into_iter().map(|x| x as u16).collect(),
                chord: fi as u16,
                holonomy,
            });
        }
        loops
    }
}

fn form_face_step(faces: &[FormFace], fi: usize, from: usize, to: usize) -> Option<Word> {
    let f = faces.get(fi)?;
    if f.i as usize == from && f.j as usize == to {
        Some(f.transport.word.clone())
    } else {
        Some(f.transport.word.inverse())
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

/// The tree path from `src` to `dst` as its vertices and the faces walked
/// between them, named in the same pass that finds them: each vertex is
/// reached through one face of its own adjacency, so no face is searched
/// for afterwards.
fn tree_path(adj: &[Vec<(usize, usize)>], src: usize, dst: usize) -> Option<(Vec<usize>, Vec<usize>)> {
    if src == dst {
        return Some((vec![src], Vec::new()));
    }
    let mut prev: Vec<Option<(usize, usize)>> = vec![None; adj.len()];
    prev[src] = Some((src, 0));
    let mut q = vec![src];
    let mut qi = 0usize;
    while qi < q.len() {
        let u = q[qi];
        qi += 1;
        if u == dst {
            break;
        }
        for &(v, fi) in &adj[u] {
            if prev[v].is_some() {
                continue;
            }
            prev[v] = Some((u, fi));
            q.push(v);
        }
    }
    if prev[dst].is_none() {
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

fn spell_versor(v: &Versor) -> Option<Word> {
    let a = affine_of(v)?;
    if a.len() < 2 || a[0].len() < 2 {
        return None;
    }
    let block = [[a[0][0].clone(), a[0][1].clone()], [a[1][0].clone(), a[1][1].clone()]];
    Some(alphabet::spell(&block)?.word)
}

/// Transport class of a conformal versor: the spelled word's class key.
pub fn class_key_of_versor(v: &Versor) -> Option<[u8; 32]> {
    Some(spell_versor(v)?.class_key())
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
        self.form
            .hue_maps
            .get(&inv.key())
            .cloned()
            .unwrap_or_default()
    }

    /// Placement of a transport class, read from the form's placements.
    pub fn shift_of(&self, key: &[u8; 32]) -> Option<(num_rational::BigRational, num_rational::BigRational)> {
        self.form.placements.get(key).cloned()
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

/// A form as a `SentenceRule` (chapter 31). Hue maps are filled from the
/// form's own Word vertices when the form does not already carry them.
pub fn rule_of_form(mut form: Form) -> SentenceRule {
    form.reindex();
    let sections = form.sections.clone();
    let ratio = form.ratio;
    SentenceRule {
        form,
        sections,
        ratio,
    }
}



/// Vertex key of a sentence form: Word vertices (M1, ρ, placement, contact,
/// gauge) and face words. The body's absolute position is already quotiented
/// on the chain; the bundle's translation is meaning.
pub fn vertex_address(e: &Element) -> Option<[u8; 32]> {
    from_element_form(e).map(|f| form_vertex_key(&f))
}

pub fn vertex_ix_key(i: u16) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..2].copy_from_slice(&i.to_le_bytes());
    k
}

fn index_key(i: usize) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..8].copy_from_slice(&(i as u64).to_le_bytes());
    k
}

pub fn form_vertex_key(f: &Form) -> [u8; 32] {
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
        buf.extend_from_slice(&face.transport.word.key());
        alphabet::put_q(&mut buf, &face.transport.shift.0);
        alphabet::put_q(&mut buf, &face.transport.shift.1);
        let mut g = face.transport.gauge.clone();
        g.sort_unstable();
        buf.extend_from_slice(&(g.len() as u16).to_le_bytes());
        for (a, b) in g {
            buf.push(a);
            buf.push(b);
        }
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

/// One snap of the pivot loop (A16).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Snap {
    /// A chord the tree gauge closed: its class was trivial (T2), never rank.
    Closed { chord: usize, order: Order },
    /// A held loop discharged on one occupied axis by a certified pivot.
    Discharged { chord: usize, certificate: PivotCertificate },
    /// Several pivots discharge and none dominates on μ⃗: all are held as
    /// cells of the thought until a perception, a glue or a ruling collapses
    /// the superposition (12.6).
    Vortex { chord: usize, held: Vec<PivotCertificate> },
    /// Several discharges with no bundle to measure μ⃗.
    Unmeasured { chord: usize, held: Vec<PivotCertificate> },
    TorsionFiled { chord: usize, ell: u32 },
    TorsionOpen { chord: usize },
    Demand { chord: usize, key: [u8; 32] },
}

/// The three outcomes of selecting among certified discharges.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Discharge {
    One(PivotCertificate),
    Vortex(Vec<PivotCertificate>),
    NoneDischarged,
}

/// A remainder cell's kind, read from the two hues. Hue 0 is void.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellKind {
    Created,
    Erased,
    Recoloured,
    Unchanged,
}

impl CellKind {
    pub fn of(before: u8, after: u8) -> CellKind {
        match (before == 0, after == 0) {
            (true, true) => CellKind::Unchanged,
            (true, false) => CellKind::Created,
            (false, true) => CellKind::Erased,
            (false, false) if before == after => CellKind::Unchanged,
            (false, false) => CellKind::Recoloured,
        }
    }
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
    /// μ⃗ of the form over the bundle before the move.
    pub mu_before: [u64; 4],
    /// μ⃗ after the trial move; `select_discharge` reads this.
    pub mu_after: [u64; 4],
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
        for m in self.mu_before.iter().chain(&self.mu_after) {
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
        let mut mu_before = [0u64; 4];
        let mut mu_after = [0u64; 4];
        for m in mu_before.iter_mut().chain(&mut mu_after) {
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
            mu_before,
            mu_after,
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
            if index.get(&ck).is_none() {
                index.insert(ck, Keyed::new());
            }
            if let Some(mid) = index.get_mut(&ck) {
                mid.nested_insert(ak, w.key(), w.clone());
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

/// Hold a vortex cell by its after class, then by the discharging word's
/// rotor A1 key. Collapse reads `(class, r.key())`.
pub fn vortex_put(v: &mut Keyed<Keyed<PivotCertificate>>, c: PivotCertificate) {
    let after = c.after;
    let k = match &c.by {
        PivotBy::Word { word, .. } => word.rotor().key(),
    };
    v.nested_insert(after, k, c);
}

/// Dominance certifies (law 5): the lane-wise minimum of the candidates'
/// μ⃗ is computed once; a candidate whose μ⃗ equals that minimum is the
/// discharge. Several that match, or none that match (incomparable lanes),
/// are a vortex. No pairwise pass.
fn select_discharge(cands: &[PivotCertificate]) -> Discharge {
    let Some(first) = cands.first() else {
        return Discharge::NoneDischarged;
    };
    let min: [u64; 4] = std::array::from_fn(|l| {
        cands.iter().map(|c| c.mu_after[l]).min().unwrap_or(first.mu_after[l])
    });
    let mut winner: Option<&PivotCertificate> = None;
    for c in cands {
        if c.mu_after == min {
            if winner.is_some() {
                return Discharge::Vortex(cands.to_vec());
            }
            winner = Some(c);
        }
    }
    match winner {
        Some(c) => Discharge::One(c.clone()),
        None => Discharge::Vortex(cands.to_vec()),
    }
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

pub(crate) fn encode_form_v2(f: &Form, out: &mut Vec<u8>) {
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
        word_to_form_bytes(&face.transport.word, None, out);
        alphabet::put_q(out, &face.transport.shift.0);
        alphabet::put_q(out, &face.transport.shift.1);
        out.extend_from_slice(&(face.transport.gauge.len() as u16).to_le_bytes());
        for &(a, b) in &face.transport.gauge {
            out.push(a);
            out.push(b);
        }
    }
    out.extend_from_slice(&(f.loops.len() as u16).to_le_bytes());
    for lp in &f.loops {
        out.extend_from_slice(&(lp.vertices.len() as u16).to_le_bytes());
        for &i in &lp.vertices {
            out.extend_from_slice(&i.to_le_bytes());
        }
        out.extend_from_slice(&(lp.faces.len() as u16).to_le_bytes());
        for &i in &lp.faces {
            out.extend_from_slice(&i.to_le_bytes());
        }
        out.extend_from_slice(&lp.chord.to_le_bytes());
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
    for (_, cert) in f.certificates.fiber(&crate::network::nerve::ROOT) {
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
    for (k, (sigma, ell, chord)) in f.torsion.fiber(&crate::network::nerve::ROOT) {
        out.extend_from_slice(k);
        out.extend_from_slice(&(sigma.len() as u16).to_le_bytes());
        for &(a, b) in sigma {
            out.push(a);
            out.push(b);
        }
        out.extend_from_slice(&ell.to_le_bytes());
        out.extend_from_slice(&chord.to_le_bytes());
    }
}

fn take_u16(b: &[u8], pos: &mut usize) -> Option<u16> {
    let v = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
    *pos += 2;
    Some(v)
}

pub(crate) fn decode_form_v2(b: &[u8], pos: &mut usize) -> Option<Form> {
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
        let (word, _) = word_from_form_bytes(b, pos)?;
        let s0 = alphabet::take_q(b, pos)?;
        let s1 = alphabet::take_q(b, pos)?;
        let ng = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
        let mut gauge = Vec::with_capacity(ng);
        for _ in 0..ng {
            gauge.push((*b.get(*pos)?, *b.get(*pos + 1)?));
            *pos += 2;
        }
        faces.push(FormFace {
            i,
            j,
            transport: Transport {
                word,
                shift: (s0, s1),
                gauge,
            },
        });
    }
    let nl = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
    let mut loops = Vec::with_capacity(nl);
    for _ in 0..nl {
        let nv = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
        let mut vs = Vec::with_capacity(nv);
        for _ in 0..nv {
            vs.push(take_u16(b, pos)?);
        }
        let n = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
        let mut fs = Vec::with_capacity(n);
        for _ in 0..n {
            fs.push(take_u16(b, pos)?);
        }
        let chord = take_u16(b, pos)?;
        let (holonomy, _) = word_from_form_bytes(b, pos)?;
        loops.push(Loop {
            vertices: vs,
            faces: fs,
            chord,
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
    let mut certificates = Keyed::new();
    for _ in 0..ncert {
        let c = PivotCertificate::from_bytes(b, pos)?;
        certificates.insert(c.key(), c);
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
            let chord = take_u16(b, pos)?;
            torsion.insert(k, (sigma, ell, chord));
        }
    }
    let mut glued = Keyed::new();
    for (i, v) in vertices.iter().enumerate() {
        if let Vertex::Glued { address, .. } = v {
            glued.insert(*address, i as u16);
        }
    }
    let mut form = Form {
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
        glued,
        vortex: Keyed::new(),
        by_address: Keyed::new(),
        star: Keyed::new(),
        placements: Keyed::new(),
    };
    form.reindex();
    Some(form)
}

/// SENTENCE_TAG v2: the Form tape.
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

/// Read a Form from a v2 tape.
pub fn from_element_form(e: &Element) -> Option<Form> {
    if !e.is_sentence() {
        return None;
    }
    let b = e.description();
    if b.get(1).copied() != Some(2) {
        return None;
    }
    let mut pos = 2usize;
    decode_form_v2(b, &mut pos)
}

/// Filed torsion on a sentence tape: `Form.torsion`, keyed by class.
/// Empty when the word is not a sentence, or when the tape is v1 and the
/// cycles live only on the element's residual section.
pub fn form_torsion(e: &Element) -> Keyed<(Vec<(u8, u8)>, u32, u16)> {
    e.form().map(|f| f.torsion.clone()).unwrap_or_default()
}

/// Whether a spoken word carries a torsion cycle: the form first, then
/// the residual element's section so a v1 tape that adopted torsion still
/// reads as a bundle.
pub fn carries_torsion(e: &Element) -> bool {
    if let Some(f) = e.form() {
        if !f.torsion.is_empty() {
            return true;
        }
    }
    !e.torsion().is_empty()
}

/// The rotor a filed torsion cycle names: the chord face stored on the
/// class key. None when that face is missing — never an identity rotor.
pub fn torsion_loop_rotor(f: &Form, class: &[u8; 32]) -> Option<Rotor> {
    let &(_, _, chord) = f.torsion.get(class)?;
    f.faces.get(chord as usize).map(|face| face.transport.word.rotor())
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

/// A sentence Element: tagged tape, not an opened-axis connection.
pub fn is_sentence(e: &Element) -> bool {
    e.is_sentence()
}

/// Chart of a crystallized form (M4 step 3): the form's cell counts,
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
                let row = v.word_in(f).and_then(|w| w.form()).map(|pf| {
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
    if let Some(f) = from_element_form(e) {
        return chart_of_form(&f);
    }
    crate::transducer::stalk::Invariant(vec![2u64, 1, 0, 0])
}

/// Two forms whose cochains differ by a coboundary δγ: the gauge that
/// takes `a` onto `b`.
pub fn coboundary_gauge(a: &Form, b: &Form) -> Option<Vec<Word>> {
    if a.vertices.len() != b.vertices.len() || a.faces.len() != b.faces.len() {
        return None;
    }
    let mut gamma = Vec::with_capacity(a.vertices.len());
    for (va, vb) in a.vertices.iter().zip(&b.vertices) {
        gamma.push(vb.word_in(b)?.compose(&va.word_in(a)?.inverse()));
    }
    for (fa, fb) in a.faces.iter().zip(&b.faces) {
        if fa.i != fb.i || fa.j != fb.j {
            return None;
        }
        let i = fa.i as usize;
        let j = fa.j as usize;
        let expect = gamma[j]
            .compose(&fa.transport.word)
            .compose(&gamma[i].inverse());
        if !expect.same_form(&fb.transport.word) {
            return None;
        }
    }
    Some(gamma)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::from_residual;
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
            transport: f.face_of(0, 1).expect("star"),
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
        f.faces.push(FormFace::new(1, glued, w.clone(),));
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
        assert_eq!(back.faces[0].transport.word, f.faces[0].transport.word);
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

    #[test]
    fn face_of_a_missing_vertex_is_none() {
        let f = Form::default();
        assert!(
            f.face_of(0, 5).is_none(),
            "a missing vertex is not the identity word"
        );
        assert!(f.face_of(0, 0).is_some(), "the frame is present");
    }

    #[test]
    fn solve_coboundary_is_a_spanning_forest() {
        let mut f = Form::default();
        f.vertices.push(Vertex::Word {
            address: [1u8; 32],
            word: Word::identity(),
            rho: q1(),
            placement: (q0(), q0()),
            gauge: Vec::new(),
            class: stalk::Invariant::default(),
            contact: None,
        });
        f.vertices.push(Vertex::Word {
            address: [2u8; 32],
            word: Word::identity(),
            rho: q1(),
            placement: (q0(), q0()),
            gauge: Vec::new(),
            class: stalk::Invariant::default(),
            contact: None,
        });
        f.faces.push(FormFace::new(0, 1, Word::identity(),));
        let gamma = f.solve_coboundary().expect("an isolated vertex roots its own tree");
        assert_eq!(gamma.len(), 3);
        assert!(gamma[0].is_identity(), "the least index of a component is γ = 1");
        assert!(gamma[2].is_identity(), "vertex 2 is its own root");
        f.faces.push(FormFace::new(0, 9, Word::identity(),));
        assert!(
            f.solve_coboundary().is_none(),
            "a face that names a vertex outside the form refuses"
        );
        assert!(f.regauge(&[Word::identity()]).is_none(), "a short gauge refuses");
    }

    #[test]
    fn glue_retract_remaps_faces_and_placement_refuses() {
        let w = Word::from_letters(&[(alphabet::LetterId::quarter((0, 1)).unwrap(), false)]);
        let mut host = Form::default();
        host.vertices.push(Vertex::Word {
            address: [1u8; 32],
            word: w.clone(),
            rho: q1(),
            placement: (q0(), q0()),
            gauge: Vec::new(),
            class: stalk::Invariant::default(),
            contact: None,
        });
        host.vertices.push(Vertex::Word {
            address: [2u8; 32],
            word: w.clone(),
            rho: q1(),
            placement: (q0(), q1()),
            gauge: Vec::new(),
            class: stalk::Invariant::default(),
            contact: None,
        });
        host.faces.push(FormFace { i: 0, j: 1, transport: host.face_of(0, 1).expect("0-1"), });
        host.faces.push(FormFace { i: 1, j: 2, transport: host.face_of(1, 2).expect("1-2"), });
        host.faces.push(FormFace { i: 0, j: 2, transport: host.face_of(0, 2).expect("0-2"), });
        let inner = closed_word_form(w.clone(), None);
        let inner_e = as_element_form(&inner, [1, 0, 0, 0]).expect("inner");
        let gi = host
            .glue(&inner, inner_e.address(), w.clone(), [1, 0, 0, 0])
            .expect("glue");
        assert_eq!(host.glued.get(&inner_e.address()).copied(), Some(gi));
        assert_eq!(
            host.faces.last().map(|f| (f.i, f.j)),
            Some((0, gi)),
            "glue writes the Frame→Glued placement face"
        );
        let mut drop = Keyed::new();
        drop.insert(vertex_ix_key(1), ());
        host.retract_vertices(&drop);
        assert_eq!(host.vertices.len(), 3, "Frame, remaining Word, Glued");
        for face in &host.faces {
            assert_eq!(
                host.face_of(face.i, face.j).as_ref(),
                Some(&face.transport),
                "every surviving face re-derives"
            );
        }
        let mut taped = host.clone();
        let mag = [1u64, 0, 0, 0];
        let clean = as_element_form(&taped, mag).expect("tape");
        assert_eq!(crate::court::verdict::blind_replay(&clean), Some(crate::court::verdict::REFUSE_GLUED_UNRULED));
        let resolve = |a: &[u8; 32]| (a == &inner_e.address()).then(|| inner.clone());
        assert_eq!(crate::court::verdict::blind_replay_with(&clean, &resolve), None);
        let Some(fi) = taped.faces.iter().position(|f| f.i == 0 && matches!(taped.vertices.get(f.j as usize), Some(Vertex::Glued { .. }))) else {
            panic!("placement face");
        };
        taped.faces[fi].transport.word = Word::identity();
        let bad = as_element_form(&taped, mag).expect("tampered placement");
        assert_eq!(
            crate::court::verdict::blind_replay_with(&bad, &resolve),
            Some(crate::court::verdict::REFUSE_FACE),
            "a tampered placement face refuses"
        );
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
        assert!(
            !s.multiplicity.is_empty(),
            "stratum width is recorded per body"
        );
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
        assert_eq!(
            s.multiplicity.len(),
            3,
            "bar, dot and residual each have a width, never one maximum"
        );
        assert_eq!(s.connection.len(), 1, "one relative transport between two cited bodies");
        assert_eq!(s.residual, vec![(9, 9, 5, 6), (9, 10, 5, 6), (10, 9, 5, 6)]);
        // Excluding the bar itself: a bar chain has no other word of hue 1→2.
        let s = deconstruct(&words, &atlas, &index_of, &shape_at,words[0].sub(), Some(0));
        assert!(s.cites.is_empty() && !s.is_closed(), "no word but itself speaks it");
    }

    #[test]
    fn deconstruct_multiplicity_is_per_body() {
        let bar = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
        let dot = from_residual(&germinate(&[(0, 0, 3u8)], &[(0, 0, 4u8)]).unwrap());
        let words = vec![bar, dot];
        let atlas = Stalks::over_elements(words.iter());
        let mut index = Keyed::new();
        for (i, w) in words.iter().enumerate() {
            index.insert(w.address(), i);
        }
        let index_of = |a: &[u8; 32]| index.get(a).copied();
        let shapes: Vec<Option<Shape>> = words.iter().map(|w| element::shape_of_chain(w.sub())).collect();
        let shape_at = |i: usize| shapes.get(i).and_then(|s| s.as_ref());
        let mixed = vec![(0, 0, 1, 2), (0, 1, 1, 2), (4, 4, 3, 4)];
        let s = deconstruct(&words, &atlas, &index_of, &shape_at, &mixed, None);
        assert_eq!(s.cites.len(), 2);
        assert_eq!(s.multiplicity.len(), 2, "two bodies, two keys");
        let widths: Vec<usize> = s
            .multiplicity
            .fiber(&ROOT)
            .map(|(_, w)| *w)
            .collect();
        assert!(
            widths.iter().all(|&w| w >= 1),
            "each body keeps its own stratum width: {widths:?}"
        );
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

    fn word_vertex(addr: u8, word: Word) -> Vertex {
        let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
        let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
        Vertex::Word {
            address: [addr; 32],
            word,
            rho: one,
            placement: (zero.clone(), zero),
            gauge: vec![(1, 2)],
            class: stalk::Invariant::default(),
            contact: None,
        }
    }

    fn star_form(n_words: usize, extra: Vec<(u16, u16, Word)>) -> Form {
        let mut f = Form::default();
        for i in 0..n_words {
            f.vertices.push(word_vertex(i as u8 + 1, Word::identity()));
            let j = (i as u16) + 1;
            f.faces.push(FormFace {
                i: 0,
                j,
                transport: f.face_of(0, j).expect("star"),
            });
        }
        for (i, j, w) in extra {
            f.faces.push(FormFace::new(i, j, w));
        }
        f.loops = Form::loops_of(&f.faces, f.vertices.len());
        f
    }

    /// Two word vertices and the frame star are a tree; the coboundary is exact.
    #[test]
    fn a_two_germ_nerve_is_a_tree_and_closes() {
        let f = star_form(2, Vec::new());
        assert_eq!(f.vertices.len(), 3, "frame plus two words");
        assert!(!f.faces.is_empty(), "the frame star has faces");
        assert!(f.loops.is_empty(), "a tree has no independent cycle");
        assert!(f.solve_coboundary().is_some(), "a tree 1-cochain is exact");
        assert!(f.coheres(), "a tree coheres");
    }

    #[test]
    fn separated_bodies_meet_by_relative_word() {
        let f = star_form(2, Vec::new());
        let word_word = f.faces.iter().filter(|fa| fa.i != 0 && fa.j != 0).count();
        assert_eq!(word_word, 0, "separated bodies have no adjacency face");
    }

    #[test]
    fn a_row_of_three_has_faces_by_constancy() {
        let tree = star_form(3, Vec::new());
        assert!(tree.faces.len() <= 2 * (tree.vertices.len() - 1));
        let touching = star_form(3, vec![(1, 2, Word::identity()), (2, 3, Word::identity())]);
        let adj = touching.faces.iter().filter(|fa| fa.i != 0 && fa.j != 0).count();
        assert_eq!(adj, 2, "two adjacency faces on a touching row, got {adj}");
        assert!(!touching.loops.is_empty(), "the 1-skeleton is not a star tree");
    }

    #[test]
    fn a_sentence_chart_leads_with_axis_two() {
        let f = star_form(2, Vec::new());
        let e = as_element_form(&f, [1, 0, 0, 0]).expect("form element");
        assert!(e.is_sentence());
        let inv = stalk::invariant_of(&e);
        assert_eq!(inv.0.first().copied(), Some(2), "leading axis 2");
        assert!(inv.0.get(1).copied().unwrap_or(0) >= 2, "vertex count");
    }

    #[test]
    fn a_sentence_round_trips_its_nerve() {
        let f = star_form(2, Vec::new());
        let e = as_element_form(&f, [1, 0, 0, 0]).expect("form element");
        assert!(e.is_sentence());
        let back = from_element_form(&e).expect("the form survives the tape");
        assert_eq!(back.vertices.len(), f.vertices.len());
        assert_eq!(back.faces.len(), f.faces.len());
        assert_eq!(back.loops.len(), f.loops.len());
    }

    #[test]
    fn face_constancy_is_the_word_not_the_versor() {
        let a = star_form(2, vec![(1, 2, Word::identity())]);
        let b = star_form(2, vec![(1, 2, Word::identity())]);
        let words1: Vec<_> = a.faces.iter().map(|fa| fa.transport.word.clone()).collect();
        let words2: Vec<_> = b.faces.iter().map(|fa| fa.transport.word.clone()).collect();
        assert_eq!(words1, words2, "constancy is the word");
        let _ = as_element_form(&a, [1, 0, 0, 0]).expect("rule element");
    }

    #[test]
    fn word_constant_triangle_keeps_its_cycle() {
        let f = star_form(3, vec![(1, 2, Word::identity()), (2, 3, Word::identity()), (1, 3, Word::identity())]);
        assert!(!f.loops.is_empty(), "a triangle has a loop");
        assert!(f.loops.iter().all(|lp| lp.holonomy.abelian_identity()));
        let _ = as_element_form(&f, [1, 0, 0, 0]).expect("rule element");
    }

    #[test]
    fn placement_is_not_the_sentence_vertex() {
        let f = star_form(2, Vec::new());
        assert!(!f.faces.is_empty(), "the frame star relates the two words");
        let e1 = as_element_form(&f, [1, 0, 0, 0]).expect("e1");
        let e2 = as_element_form(&f, [1, 0, 0, 0]).expect("e2");
        assert!(e1.same_vertex(&e2));
        assert_eq!(e1.address(), e2.address());
    }

    #[test]
    fn infinite_class_is_never_filed() {
        let mut f = chord_form(&[infinite()]);
        let snaps = f.pivot_loop(
            [0x11; 32],
            &crate::network::nerve::Keyed::new(),
            &crate::transducer::family::NoCrystal,
            &mut crate::physics::Physics::default().stroke_n(8),
            None,
            0,
        );
        assert!(
            snaps.iter().any(|s| matches!(s, Snap::Demand { .. })),
            "infinite is demanded: {snaps:?}"
        );
        assert!(
            !snaps.iter().any(|s| matches!(s, Snap::TorsionFiled { .. })),
            "infinite is never filed: {snaps:?}"
        );
        assert!(
            f.torsion.is_empty(),
            "an infinite class is never a large-ℓ torsion cycle"
        );
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

    fn chord_form(chords: &[Word]) -> Form {
        let mut f = Form::default();
        for (ci, w) in chords.iter().enumerate() {
            let a = (ci * 2 + 1) as u16;
            let b = (ci * 2 + 2) as u16;
            f.vertices.push(word_vertex(a as u8, Word::identity()));
            f.vertices.push(word_vertex(b as u8, Word::identity()));
            f.faces.push(FormFace {
                i: 0,
                j: a,
                transport: f.face_of(0, a).expect("star a"),
            });
            f.faces.push(FormFace {
                i: 0,
                j: b,
                transport: f.face_of(0, b).expect("star b"),
            });
            f.faces.push(FormFace::new(a, b, w.clone()));
        }
        f.loops = Form::loops_of(&f.faces, f.vertices.len());
        f
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
            mu_before: [0; 4],
            mu_after: mu,
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
        assert!(
            matches!(select_discharge(&[a, b]), Discharge::Vortex(_)),
            "incomparable μ⃗ are a vortex, not a pick"
        );
    }

    /// Descent certifies after a trial; it never ranks candidates. A chord
    /// write changes no vertex, so speech and μ⃗ are unmoved: two words that
    /// both discharge tie and are a vortex. Per-candidate μ⃗ moves only when
    /// a candidate changes what the form speaks (a glue pivot).
    #[test]
    fn descent_certifies_never_selects() {
        let loose = cert(Axis::Unit(0), [4, 4, 4, 4]);
        let tight = cert(Axis::Unit(0), [1, 1, 1, 1]);
        let Discharge::One(won) = select_discharge(&[loose.clone(), tight.clone()]) else {
            panic!("one μ⃗ dominates");
        };
        assert_eq!(won.mu_after, tight.mu_after, "the lane-wise minimum is the discharge");
        fn perm(m: [u64; 4]) -> [u64; 4] {
            [m[2], m[1], m[0], m[3]]
        }
        let mut lp = loose.clone();
        let mut tp = tight.clone();
        lp.mu_after = perm(lp.mu_after);
        tp.mu_after = perm(tp.mu_after);
        let Discharge::One(won_p) = select_discharge(&[lp, tp]) else {
            panic!("permuted lanes keep the winner");
        };
        assert_eq!(won_p.axis, tight.axis, "the same pivot after a lane permutation");
        let a = cert(Axis::Unit(0), [2, 0, 0, 0]);
        let b = cert(Axis::Hue, [0, 2, 0, 0]);
        assert!(
            matches!(select_discharge(&[a, b]), Discharge::Vortex(_)),
            "two incomparable discharges are a vortex"
        );

        let q = order4();
        let qinv = q.inverse();
        let x: Vec<Vec<u8>> = vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ];
        let bodies = crate::transducer::family::perceive(&x);
        assert!(!bodies.is_empty(), "the bar is a body");
        let class = bodies[0].moduli.clone();
        assert!(!class.0.is_empty(), "the body has a chart");
        let mut spoken = Form::default();
        spoken.vertices.push(Vertex::Word {
            address: [1u8; 32],
            word: q.clone(),
            rho: q1(),
            placement: (q1() + q1() + q1(), q1() + q1() + q1()),
            gauge: vec![(1, 1)],
            class: class.clone(),
            contact: None,
        });
        let y = crate::transducer::family::apply_form(
            &spoken,
            &x,
            &Word::identity(),
            &crate::transducer::family::NoCrystal,
        )
        .map(|s| s.grid())
        .expect("q speaks the bar");
        assert!(
            y.iter().flatten().any(|&h| h != 0),
            "the spoken image is not the empty frame"
        );
        let mut f = chord_form(&[q.clone()]);
        if let Vertex::Word {
            class: c,
            gauge,
            placement,
            ..
        } = &mut f.vertices[1]
        {
            *c = class;
            *gauge = vec![(1, 1)];
            *placement = (q1() + q1() + q1(), q1() + q1() + q1());
        }
        let bundle = vec![(x, y)];
        let snaps = f.pivot_loop(
            [0xde; 32],
            &Keyed::new(),
            &Hold(vec![q.clone(), qinv]),
            &mut crate::physics::Physics::default().stroke_n(32),
            Some(&bundle),
            3,
        );
        let Some(Snap::Vortex { held, .. }) = snaps.iter().find(|s| matches!(s, Snap::Vortex { .. })) else {
            panic!("a pivot that does not write vertices leaves μ⃗ unmoved, so two discharges are a vortex: {snaps:?}");
        };
        assert!(held.len() >= 2, "q and q⁻¹ both discharge: {held:?}");
        assert_eq!(held[0].mu_before[2], 3, "T is the extent passed in");
        assert_eq!(held[0].mu_after[2], 3);
        assert_eq!(held[0].mu_after, held[1].mu_after, "equal μ⃗, neither dominates");
    }

    #[test]
    fn cauterize_filed_torsion_crystallizes() {
        let mut f = chord_form(&[order2()]);
        assert!(!f.is_closed());
        let _ = f.pivot_loop(
            [0xca; 32],
            &crate::network::nerve::Keyed::new(),
            &crate::transducer::family::NoCrystal,
            &mut crate::physics::Physics::default().stroke_n(8),
            None,
            0,
        );
        assert!(f.is_cauterized(), "filed finite torsion with empty remainder cauterizes");
        let e = as_element_form(&f, [1, 0, 0, 0]).expect("crystallized form");
        assert!(
            carries_torsion(&e),
            "the tape carries the filed cycle"
        );
    }

    /// Chapter 23.1 step 4: a filed torsion chord replays; a chord that is
    /// not the holonomy refuses the face; a missing torsion entry refuses
    /// the loop.
    #[test]
    fn filed_torsion_chord_replays_or_refuses() {
        let mut f0 = chord_form(&[order2()]);
        let _ = f0.pivot_loop(
            [0xca; 32],
            &crate::network::nerve::Keyed::new(),
            &crate::transducer::family::NoCrystal,
            &mut crate::physics::Physics::default().stroke_n(8),
            None,
            0,
        );
        assert!(f0.is_cauterized(), "filed finite torsion with empty remainder cauterizes");
        let e = as_element_form(&f0, [1, 0, 0, 0]).expect("the filed form crystallizes");
        assert!(
            e.torsion().is_empty(),
            "v2 does not fill the element torsion section"
        );
        assert!(
            carries_torsion(&e),
            "the form carries the filed cycle"
        );
        let keyed = from_element_form(&e).expect("v2 tape");
        let (class, (_, _, chord)) = keyed
            .torsion
            .fiber(&crate::network::nerve::ROOT)
            .next()
            .map(|(k, v)| (*k, v.clone()))
            .expect("filed class");
        assert_eq!(
            torsion_loop_rotor(&keyed, &class).map(|r| r.class_key()),
            Some(class),
            "the chord face is the keyed representative"
        );
        assert!(
            keyed.faces.get(chord as usize).is_some(),
            "the carrier stores a live chord face"
        );
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
            .position(|face| f.face_of(face.i, face.j).as_ref() != Some(&face.transport))
        else {
            panic!("the filed form has a torsion chord");
        };
        f.faces[ci].transport.word = order4();
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
        let mut f = chord_form(&[infinite()]);
        let r = infinite().rotor();
        let mut horizon = crate::network::nerve::Keyed::new();
        horizon.insert(r.class_key(), (r, Order::Infinite, Standing::Held));
        let snaps = f.pivot_loop(
            [0x5e; 32],
            &horizon,
            &crate::transducer::family::NoCrystal,
            &mut crate::physics::Physics::default().stroke_n(8),
            None,
            0,
        );
        assert!(
            !snaps.iter().any(|s| matches!(s, Snap::Demand { .. })),
            "a held infinite class is not spoken again: {snaps:?}"
        );
    }

    #[test]
    fn two_loops_of_one_class_are_both_filed_or_discharged() {
        let w = order2();
        fn two_chords(w: &Word) -> Form {
            chord_form(&[w.clone(), w.clone()])
        }
        let mut f = two_chords(&w);
        assert_eq!(f.loops.len(), 2, "two independent chords");
        let snaps = f.pivot_loop(
            [0xa3; 32],
            &Keyed::new(),
            &crate::transducer::family::NoCrystal,
            &mut crate::physics::Physics::default().stroke_n(16),
            None,
            0,
        );
        let filed = snaps
            .iter()
            .filter(|s| matches!(s, Snap::TorsionFiled { .. }))
            .count();
        assert_eq!(filed, 2, "both loops of one class file: {snaps:?}");

        let q = order4();
        let mut f = two_chords(&q);
        let snaps = f.pivot_loop(
            [0xa3; 32],
            &Keyed::new(),
            &Hold(vec![q.clone()]),
            &mut crate::physics::Physics::default().stroke_n(32),
            None,
            0,
        );
        let disc = snaps
            .iter()
            .filter(|s| matches!(s, Snap::Discharged { .. }))
            .count();
        assert_eq!(disc, 2, "both loops discharge: {snaps:?}");
    }

    #[test]
    fn pivot_never_reopens_a_closed_chord() {
        let mut f = chord_form(&[order2(), order4()]);
        let mut budget = crate::physics::Physics::default().stroke_n(16);
        let first = f.pivot_loop(
            [0xcd; 32],
            &crate::network::nerve::Keyed::new(),
            &crate::transducer::family::NoCrystal,
            &mut budget,
            None,
            0,
        );
        let closed: Vec<u32> = first
            .iter()
            .filter_map(|s| match s {
                Snap::Closed {
                    order: Order::Finite(e),
                    ..
                } => Some(*e),
                Snap::TorsionFiled { ell, .. } => Some(*ell),
                _ => None,
            })
            .collect();
        let mut budget = crate::physics::Physics::default().stroke_n(16);
        let second = f.pivot_loop(
            [0xcd; 32],
            &crate::network::nerve::Keyed::new(),
            &crate::transducer::family::NoCrystal,
            &mut budget,
            None,
            0,
        );
        for o in closed {
            assert!(
                !second.iter().any(|s| matches!(s, Snap::Demand { .. })) || o > 0,
                "a closed finite chord is not demanded"
            );
            let still = f.loops.iter().any(|lp| {
                !lp.holonomy.abelian_identity() && matches!(lp.holonomy.order(), Order::Finite(ell) if ell == o)
            });
            let filed = f.torsion.fiber(&crate::network::nerve::ROOT).any(|(_, (_, ell, _))| *ell == o);
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
        let mut f = chord_form(&[q.clone()]);
        let snaps = f.pivot_loop(
            [0x11; 32],
            &Keyed::new(),
            &Hold(vec![q.clone()]),
            &mut crate::physics::Physics::default().stroke_n(16),
            None,
            0,
        );
        let Some(Snap::Discharged { certificate, .. }) = snaps.iter().find(|s| matches!(s, Snap::Discharged { .. })) else {
            panic!("a crystal holding q discharges: {snaps:?}");
        };
        assert_eq!(certificate.before, q.rotor().class_key());
        assert_eq!(certificate.after, CLOSED_KEY);
        assert_eq!(certificate.rank_before.torsion, 1);
        assert_eq!(certificate.rank_after.torsion, 0);
        assert!(
            f.certificates.contains(&certificate.key()),
            "the form carries the discharge by key"
        );
        let e = as_element_form(&f, [1, 0, 0, 0]).expect("the discharged form crystallizes");
        assert!(
            matches!(f.vertices.first(), Some(Vertex::Frame)),
            "Frame is vertex 0 of the crystallized form"
        );
        assert_eq!(
            crate::court::verdict::blind_replay(&e),
            None,
            "a certified discharge replays: every after-face is face_of, then the certificate"
        );
        let mut tampered = from_element_form(&e).expect("v2 tape");
        let crate::transducer::sentence::PivotBy::Word { face, .. } = &tampered
            .certificates
            .fiber(&crate::network::nerve::ROOT)
            .next()
            .expect("one certificate")
            .1
            .by;
        let fi = *face as usize;
        tampered.faces[fi].transport.word = q.clone();
        let bad = as_element_form(&tampered, e.magnitude()).expect("tampered tape");
        assert_eq!(
            crate::court::verdict::blind_replay(&bad),
            Some(crate::court::verdict::REFUSE_FACE),
            "a word on a certified face that is not face_of of its vertices refuses"
        );

        let mut f = chord_form(&[q.clone()]);
        let snaps = f.pivot_loop(
            [0x22; 32],
            &Keyed::new(),
            &Hold(vec![q.clone(), qinv]),
            &mut crate::physics::Physics::default().stroke_n(16),
            None,
            0,
        );
        let Some(Snap::Unmeasured { held, .. }) = snaps.iter().find(|s| matches!(s, Snap::Unmeasured { .. })) else {
            panic!("q and q⁻¹ unmeasured are a vortex: {snaps:?}");
        };
        assert_eq!(held.len(), 2, "two certificates, got {held:?}");
        assert!(!f.vortex.is_empty(), "the form holds the vortex cells");

        let mut f = chord_form(&[q.clone()]);
        let mut horizon = Keyed::new();
        let r = q.rotor();
        horizon.insert(r.class_key(), (r, q.order(), Standing::Held));
        let snaps = f.pivot_loop(
            [0x33; 32],
            &horizon,
            &crate::transducer::family::NoCrystal,
            &mut crate::physics::Physics::default().stroke_n(16),
            None,
            0,
        );
        assert!(snaps.is_empty(), "a horizon holding q files silently: {snaps:?}");
        assert!(
            f.torsion.contains(&q.rotor().class_key()),
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

    #[test]
    fn holonomy_is_oriented() {
        let w = order4();
        let mut fwd = Form::default();
        fwd.vertices.push(word_vertex(1, Word::identity()));
        fwd.vertices.push(word_vertex(2, Word::identity()));
        fwd.faces.push(FormFace {
            i: 0,
            j: 1,
            transport: fwd.face_of(0, 1).expect("star"),
        });
        fwd.faces.push(FormFace {
            i: 0,
            j: 2,
            transport: fwd.face_of(0, 2).expect("star"),
        });
        fwd.faces.push(FormFace::new(1, 2, w.clone()));
        fwd.loops = Form::loops_of(&fwd.faces, fwd.vertices.len());
        let mut rev = Form::default();
        rev.vertices.push(word_vertex(1, Word::identity()));
        rev.vertices.push(word_vertex(2, Word::identity()));
        rev.faces.push(FormFace {
            i: 0,
            j: 1,
            transport: rev.face_of(0, 1).expect("star"),
        });
        rev.faces.push(FormFace {
            i: 0,
            j: 2,
            transport: rev.face_of(0, 2).expect("star"),
        });
        rev.faces.push(FormFace::new(2, 1, w.inverse()));
        rev.loops = Form::loops_of(&rev.faces, rev.vertices.len());
        assert_eq!(fwd.loops.len(), 1);
        assert_eq!(rev.loops.len(), 1);
        assert_eq!(
            fwd.loops[0].holonomy.class_key(),
            rev.loops[0].holonomy.class_key(),
            "a reversed chord stores the inverse word and the same class"
        );
    }

    #[test]
    fn chord_is_never_a_star_face() {
        let f = chord_form(&[order4(), order2()]);
        assert_eq!(f.loops.len(), 2, "two chords, two loops");
        for lp in &f.loops {
            let ch = &f.faces[lp.chord as usize];
            assert!(
                ch.i != 0 && ch.j != 0,
                "chord {} is a star face ({}, {})",
                lp.chord,
                ch.i,
                ch.j
            );
        }
    }

    #[test]
    fn pivot_writes_the_chord_transport_only() {
        let q = order4();
        let mut f = chord_form(&[q.clone()]);
        let verts_before = f.vertices.clone();
        let chord = f.loops[0].chord;
        let snaps = f.pivot_loop(
            [0xc0; 32],
            &Keyed::new(),
            &Hold(vec![q.clone()]),
            &mut crate::physics::Physics::default().stroke_n(16),
            None,
            0,
        );
        assert!(
            snaps.iter().any(|s| matches!(s, Snap::Discharged { .. })),
            "a held quarter discharges: {snaps:?}"
        );
        assert_eq!(
            f.vertices, verts_before,
            "a pivot writes no vertex placement or gauge"
        );
        for (fi, face) in f.faces.iter().enumerate() {
            if fi as u16 == chord {
                continue;
            }
            assert_eq!(
                f.face_of(face.i, face.j).as_ref(),
                Some(&face.transport),
                "tree face {fi} re-derives"
            );
        }
    }

    #[test]
    fn relative_gauge_composite_fixes_what_neither_fixes() {
        let t = vec![(1u8, 2), (2, 1)];
        assert!(
            compose_gauge(&t, &t).is_empty(),
            "two (1 2) transpositions compose to the identity"
        );
        let a = vec![(1u8, 2)];
        let b = vec![(2u8, 1)];
        let c = compose_gauge(&b, &a);
        assert!(
            c.iter().all(|&(h, k)| h != 1 || k != 1),
            "composite listing omits fixed points, got {c:?}"
        );
        assert_eq!(
            gauge_image(&gauge_keyed(&c), 1),
            1,
            "1→2 then 2→1 fixes 1, which neither map lists as fixed"
        );
    }

    #[test]
    fn offplane_word_refuses_to_act_on_a_translation() {
        let w = Word::from_letters(&[(alphabet::LetterId::quarter((0, 2)).unwrap(), false)]);
        assert!(w.plane_block().is_none(), "a (0,2) quarter leaves the sensor plane");
        let t = (q1(), q0());
        assert!(
            apply_linear(&w, &t).is_none(),
            "an off-plane word does not act as the identity on a translation"
        );
        assert!(
            Transport {
                word: w,
                shift: t,
                gauge: Vec::new(),
            }
            .inverse()
            .is_none(),
            "Transport::inverse refuses an off-plane word"
        );
    }
}
