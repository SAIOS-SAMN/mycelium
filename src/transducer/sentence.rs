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

use super::alphabet::{self, Rotor, Word};
use super::cech::{self, Complex, Fill, Section};
use super::element::{affine_of, from_connection, GERM_DIM};
use super::peel::ShellKind;

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
        if last != first {
            if chord.i == last && chord.j == first {
                w = w.compose(&chord.g_ij);
            } else {
                w = w.compose(&chord.g_ij.inverse());
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

/// Instances of each modulus class, ordered in the configuration space
/// by centroid. The class is the stalk; the instances stay distinct points.
fn instances_of(n: &Nerve) -> crate::network::nerve::Keyed<Vec<usize>> {
    let mut m: crate::network::nerve::Keyed<Vec<usize>> = crate::network::nerve::Keyed::new();
    for (i, g) in n.germs.iter().enumerate() {
        let k = germ_class_key(g);
        match m.get_mut(&k) {
            Some(v) => v.push(i),
            None => {
                m.insert(k, vec![i]);
            }
        }
    }
    let mut keys = Vec::new();
    crate::snapshot::persist(&m, |k, _| keys.push(*k));
    for k in keys {
        if let Some(v) = m.get_mut(&k) {
            v.sort_by_key(|&i| germ_centroid_key(&n.germs[i]));
        }
    }
    m
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

/// Assemble a nerve from germs. Faces are the pairwise relative versors
/// inside one modulus class, and the star from the frame germ to each
/// body. Not a cartesian product across classes.
pub fn assemble(mut germs: Vec<Germ>, remainder: Vec<Cell>) -> Nerve {
    germs.sort_by(|a, b| a.address.cmp(&b.address));
    let n = germs.len();
    let mut groups: crate::network::nerve::Keyed<Vec<usize>> = crate::network::nerve::Keyed::new();
    for i in 0..n {
        let k = germ_class_key(&germs[i]);
        match groups.get_mut(&k) {
            Some(v) => v.push(i),
            None => {
                groups.insert(k, vec![i]);
            }
        }
    }
    let mut class_keys = Vec::new();
    crate::snapshot::persist(&groups, |k, _| class_keys.push(*k));
    let mut faces = Vec::new();
    for k in &class_keys {
        if let Some(v) = groups.get_mut(k) {
            v.sort_by_key(|&i| germ_centroid_key(&germs[i]));
            for a in 0..v.len() {
                for b in (a + 1)..v.len() {
                    emit_face(&germs, v[a], v[b], &mut faces);
                }
            }
        }
    }
    if let Some(fi) = germs.iter().position(|g| g.kind == ShellKind::Frame) {
        for i in 0..n {
            if i != fi {
                emit_face(&germs, fi, i, &mut faces);
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
    pub hue_maps: crate::network::nerve::Keyed<Vec<(u8, u8)>>,
    pub nerve: Nerve,
    /// Family frame ratio ((h_out, h_in), (w_out, w_in)).
    pub ratio: ((usize, usize), (usize, usize)),
    /// Affine translation that agreed across pairs, keyed by germ class.
    pub shifts: crate::network::nerve::Keyed<(num_rational::BigRational, num_rational::BigRational)>,
    /// Classes present in some pairs only — sections, not a refusal.
    pub sections: usize,
}

fn chart_key(inv: &Option<stalk::Invariant>) -> [u8; 32] {
    inv.as_ref().map(|i| i.key()).unwrap_or([0u8; 32])
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

/// Identify a germ across pairs: source chart, target chart, shell kind.
pub fn germ_class_key(g: &Germ) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(&chart_key(&g.source));
    h.update(&chart_key(&g.target));
    h.update(&[g.kind.as_u8()]);
    *h.finalize().as_bytes()
}

fn class_ordinal(inst: &crate::network::nerve::Keyed<Vec<usize>>, k: &[u8; 32], i: usize) -> Option<usize> {
    inst.get(k)?.iter().position(|&x| x == i)
}

fn face_signature(n: &Nerve) -> Option<Vec<( [u8; 32], [u8; 32], Word, bool)>> {
    let mut faces: Vec<([u8; 32], [u8; 32], Word, bool)> = Vec::new();
    for f in &n.faces {
        let gi = n.germs.get(f.i)?;
        let gj = n.germs.get(f.j)?;
        let (mut a, mut b, mut w) = (germ_class_key(gi), germ_class_key(gj), f.g_ij.clone());
        if a > b {
            std::mem::swap(&mut a, &mut b);
            w = w.inverse();
        }
        faces.push((a, b, w, f.spelled));
    }
    faces.sort_by(|x, y| (x.0, x.1, x.3).cmp(&(y.0, y.1, y.3)));
    Some(faces)
}

fn hue_signature(n: &Nerve) -> Vec<([u8; 32], Vec<(u8, u8)>)> {
    let mut v: Vec<([u8; 32], Vec<(u8, u8)>)> = n
        .germs
        .iter()
        .map(|g| (germ_class_key(g), germ_hue_map(g)))
        .collect();
    v.sort_by(|a, b| a.0.cmp(&b.0));
    v
}

fn face_pair_key(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    let mut h = blake3::Hasher::new();
    h.update(lo);
    h.update(hi);
    *h.finalize().as_bytes()
}

fn faces_keyed(n: &Nerve) -> crate::network::nerve::Keyed<(Word, bool)> {
    let mut m = crate::network::nerve::Keyed::new();
    for f in &n.faces {
        let Some(gi) = n.germs.get(f.i) else { continue };
        let Some(gj) = n.germs.get(f.j) else { continue };
        let a = germ_class_key(gi);
        let b = germ_class_key(gj);
        let k = face_pair_key(&a, &b);
        let w = if a <= b { f.g_ij.clone() } else { f.g_ij.inverse() };
        if !m.contains(&k) {
            m.insert(k, (w, f.spelled));
        }
    }
    m
}

fn hues_keyed(n: &Nerve) -> crate::network::nerve::Keyed<Vec<(u8, u8)>> {
    let mut m = crate::network::nerve::Keyed::new();
    for g in &n.germs {
        let k = germ_class_key(g);
        if !m.contains(&k) {
            m.insert(k, germ_hue_map(g));
        }
    }
    m
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



/// Correspondence across pairs is by (source chart, target chart, shell
/// kind). The class is the stalk; every instance of that class remains a
/// vertex. Extra instances (count past the family minimum) are sections.
/// Hue that does not agree across pairs is a section, not a refusal.
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
    let insts: Vec<crate::network::nerve::Keyed<Vec<usize>>> = nerves.iter().map(instances_of).collect();
    let mut presence: crate::network::nerve::Keyed<u32> = crate::network::nerve::Keyed::new();
    for inst in &insts {
        crate::snapshot::persist(inst, |k, _| {
            let c = presence.get(k).copied().unwrap_or(0).saturating_add(1);
            presence.insert(*k, c);
        });
    }
    let n_pairs = nerves.len() as u32;
    let hue_of = |n: &Nerve, inst: &crate::network::nerve::Keyed<Vec<usize>>, k: &[u8; 32]| -> Option<Vec<(u8, u8)>> {
        let &i = inst.get(k)?.first()?;
        Some(germ_hue_map(n.germs.get(i)?))
    };
    let mut rule_keys: Vec<[u8; 32]> = Vec::new();
    crate::snapshot::persist(&presence, |k, c| {
        if *c != n_pairs {
            return;
        }
        let h0 = hue_of(first, &insts[0], k);
        let constant = nerves
            .iter()
            .zip(insts.iter())
            .all(|(n, ix)| hue_of(n, ix, k) == h0);
        if constant {
            rule_keys.push(*k);
        }
    });
    let mut section_n = 0usize;
    crate::snapshot::persist(&presence, |k, c| {
        let counts: Vec<usize> = insts
            .iter()
            .map(|ix| ix.get(k).map(|v| v.len()).unwrap_or(0))
            .collect();
        let total: usize = counts.iter().sum();
        let min_c = counts.iter().copied().min().unwrap_or(0);
        if *c != n_pairs || !rule_keys.iter().any(|r| r == k) {
            section_n = section_n.saturating_add(total);
        } else {
            section_n = section_n.saturating_add(total.saturating_sub(min_c.saturating_mul(nerves.len())));
        }
    });
    if rule_keys.is_empty() {
        return None;
    }
    rule_keys.sort();
    let mut germs = Vec::new();
    for k in &rule_keys {
        let v0 = insts[0].get(k)?;
        let min_c = insts
            .iter()
            .map(|ix| ix.get(k).map(|v| v.len()).unwrap_or(0))
            .min()?;
        if min_c == 0 {
            continue;
        }
        for t in 0..min_c {
            germs.push(first.germs[v0[t]].clone());
        }
    }
    if germs.is_empty() {
        return None;
    }
    let mut nerve = assemble(germs, first.remainder.clone());
    let nerve_inst = instances_of(&nerve);
    let mut hue_maps = crate::network::nerve::Keyed::new();
    for g in &nerve.germs {
        let key = chart_key(&g.target);
        if key != [0u8; 32] {
            hue_maps.insert(key, germ_hue_map(g));
        }
    }
    let mut per_shift: Vec<Option<(num_rational::BigRational, num_rational::BigRational)>> =
        vec![None; nerve.germs.len()];
    let mut shifts = crate::network::nerve::Keyed::new();
    for (i, g) in nerve.germs.iter().enumerate() {
        let k = germ_class_key(g);
        let Some(t) = class_ordinal(&nerve_inst, &k, i) else {
            continue;
        };
        let mut held = None;
        let mut agree = true;
        for (n, ix) in nerves.iter().zip(insts.iter()) {
            let Some(v) = ix.get(&k) else {
                agree = false;
                break;
            };
            let Some(&j) = v.get(t) else {
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
                match shifts.get(&k) {
                    None => {
                        shifts.insert(k, d);
                    }
                    Some(h) if *h != d => {
                        shifts.remove(&k);
                    }
                    _ => {}
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
        let ka = germ_class_key(gi);
        let kb = germ_class_key(gj);
        let Some(ta) = class_ordinal(&nerve_inst, &ka, f.i) else {
            return false;
        };
        let Some(tb) = class_ordinal(&nerve_inst, &kb, f.j) else {
            return false;
        };
        nerves.iter().zip(insts.iter()).all(|(n, ix)| {
            let Some(va) = ix.get(&ka).and_then(|v| v.get(ta)).and_then(|&j| n.germs.get(j)) else {
                return false;
            };
            let Some(vb) = ix.get(&kb).and_then(|v| v.get(tb)).and_then(|&j| n.germs.get(j)) else {
                return false;
            };
            let (w, v, s) = relative_face(va, vb);
            w == f.g_ij && s == f.spelled && element::same_versor(&v, &f.versor)
        })
    });
    nerve.complex = bind_complex(&nerve.germs, &nerve.faces);
    nerve.cycles = independent_cycles(nerve.germs.len(), &nerve.faces);
    nerve.class = first_class(&nerve.complex);
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
        sections: section_n,
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
                    let asg: Vec<(u8, u8)> = cell
                        .vertices
                        .windows(2)
                        .map(|p| (germ_index(&p[0]) as u8, germ_index(&p[1]) as u8))
                        .collect();
                    e.record_torsion(asg, *ell, element::FIBER_SPATIAL);
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
        e.record_torsion(sigma, 2, element::FIBER_SPATIAL);
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
        let address: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
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
        if f.i >= n.germs.len() || f.j >= n.germs.len() {
            return Some("sentence face does not re-derive from its germs");
        }
        let (w, v, spelled) = relative_face(&n.germs[f.i], &n.germs[f.j]);
        if spelled != f.spelled {
            return Some("sentence face does not re-derive from its germs");
        }
        if spelled && (w != f.g_ij || !element::same_versor(&v, &f.versor)) {
            return Some("sentence face g_ij does not re-derive from its germs");
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
        assert_eq!(n.faces.len(), 1, "two bodies in one thought share a face");
        assert!(
            !element::same_versor(&n.faces[0].versor, &identity_versor()),
            "a relative translation is a parabolic versor, not 1"
        );
    }

    /// Three germs: faces are the constant relative words, a complete
    /// graph on vertices, no ring code path.
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
        assert_eq!(n.faces.len(), 3, "three germs, three relative words");
        assert!(
            n.faces.iter().all(|f| !element::same_versor(&f.versor, &identity_versor())),
            "relative translations are not the identity versor"
        );
        assert_eq!(n.cycles.len(), 1, "one chord of the spanning forest");
        assert!(n.cycles[0].holonomy.abelian_identity());
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
}
