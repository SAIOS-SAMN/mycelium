// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 18.2, 6 — THE CHAIN: its links and their incidence, the citations and
//! punctures a deconstruction produces, the residual no word speaks, and the
//! replay that measures what a proof leaves.

use crate::network::nerve::Keyed;
use crate::transducer::element::{self, Element, Versor};
use crate::transducer::beam;

fn index_key(i: usize) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..8].copy_from_slice(&(i as u64).to_le_bytes());
    k
}

/// A chain link: (row, col, hue before, hue after) at a site of one
/// section (16.5).
pub type Link = (i64, i64, u8, u8);

/// A CITATION: the coordinates a sentence speaks for one word — its index
/// in the language, the transport carrying the word's chain into the
/// utterance's frame, and the gauge coordinate (which assignment along the
/// punctured axis). A proof is a sequence of citations; a replay applies
/// them and measures what remains.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cite {
    /// WHICH WORD IS CITED, BY ITS OWN COORDINATE (law 2). It was `usize` —
    /// an offset into the snapshot the reading happened to run over — and
    /// the whole `epi.index`/`Language::index` carrier existed to convert an
    /// address the atlas had already produced into that offset, so a reader
    /// went coordinate → offset → word. `HeldCite` beside this struct has
    /// said the right shape since 24.1: "a held cite is never an index into
    /// a snapshot; the Held stratum is read at the coordinate, so there is
    /// no snapshot to index." A cite is now the same object.
    pub word: crate::transducer::address::Address,
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
    /// The word that carries the body, by its coordinate — the same reading
    /// as `Cite::word`.
    pub word: crate::transducer::address::Address,
    /// The transport taking the word's shape onto the body.
    pub transport: Versor,
    /// The word's links under the BODY's own transitions — what the section
    /// says, in the body's hues and not the word's.
    pub links: Vec<Link>,
}

/// A HELD CITE (24.1 as ruled 2026-09-09): the word at a vertex of a held
/// form, evaluated on the listing body it was asked about. It names the
/// vertex by its address and carries the body the vertex's action
/// reconstructed, in the body's own frame; no element was minted and no
/// chain was decoded to ask the question. A held cite is never an index
/// into a snapshot: the Held stratum is read at the coordinate, so there
/// is no snapshot to index.
#[derive(Debug, Clone)]
pub struct HeldCite {
    /// The vertex's address (M2): the word, never the chain.
    pub address: crate::transducer::address::Address,
    /// The body's links the vertex's action reconstructed, sorted.
    pub body: Vec<Link>,
}

/// What deconstruction produced: the citations in body order, the held
/// cites derived at the bodies' charts, the connection between
/// consecutive cited bodies (the relative transport `Vᵢ⁻¹ Vⱼ`), the
/// residual no word speaks, and the widest stratum read.
#[derive(Debug, Clone, Default)]
pub struct Sentence {
    pub cites: Vec<Cite>,
    /// PUNCTURES: bodies a held word carries by shape under a gauge the
    /// word does not yet hold — (word, versor, the word's links under the
    /// body's own transitions). Spoken as sections of the word, not as new
    /// words: the fiber deepens instead of the vocabulary widening.
    pub punctures: Vec<Puncture>,
    /// HELD CITES: bodies a held form's word reconstructs (24.1), one per
    /// body, each derived at the body's chart when the reading asked.
    pub held: Vec<HeldCite>,
    pub connection: Vec<Versor>,
    pub residual: Vec<Link>,
    /// Stratum width per body key, never a maximum over bodies.
    pub multiplicity: Keyed<usize>,
}

impl Sentence {
    /// Closure: every body spoken by held words as held, nothing left over
    /// and no fiber to deepen.
    pub fn is_closed(&self) -> bool {
        self.residual.is_empty() && self.punctures.is_empty()
    }

    /// The operators this sentence's citations carry: each cited word's
    /// pattern under its transport, and each held cite's body in the frame
    /// it was derived in, which is the chain's own.
    fn operators_at<'a>(&self, word_at: &impl Fn(&[u8]) -> Option<&'a Element>) -> Vec<(Versor, Vec<Link>)> {
        let mut ops: Vec<(Versor, Vec<Link>)> = self
            .cites
            .iter()
            .filter_map(|c| Some((c.transport.clone(), cited_pattern(word_at(c.word.as_bytes())?, c.gauge)?)))
            .collect();
        ops.extend(self.held.iter().map(|h| (element::identity_versor(), h.body.clone())));
        ops
    }

    /// What the language does not yet hold: the residual, together with the
    /// bodies spoken only as punctures — novelty in the fiber is novelty.
    /// Measured as the replay of the citations alone over the chain.
    pub fn open_at<'a>(&self, word_at: &impl Fn(&[u8]) -> Option<&'a Element>, chain: &[Link]) -> Vec<Link> {
        match self.punctures.is_empty() {
            true => self.residual.clone(),
            false => replay_operators(&self.operators_at(word_at), chain),
        }
    }
}

/// PUNCTURE the held words a sentence's carried bodies name: a carried body
/// on a word's own support under transitions the word does not yet hold
/// becomes a section of that word. Returns how many fibers deepened. The
/// receiving side of speech: the hearer's language deepens on what it
/// verified, never on what it was told.
pub fn puncture_from(
    words: &mut Keyed<std::sync::Arc<crate::transducer::element::Held>>,
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
        // Through `Held::update`: the fibre deepens and the form is
        // re-decoded with it, one change to one held word (L16).
        if std::sync::Arc::make_mut(word).update(|e| e.puncture(asg)) {
            deepened += 1;
        }
    }
    deepened
}

/// The connected components of a chain's support under 4-adjacency, each
/// as a sorted chain. The sensor's segmentation, not a search.
pub fn pieces(chain: &[Link]) -> Vec<Vec<Link>> {
    let mut links: Vec<Link> = chain.to_vec();
    links.sort_unstable();
    links.dedup();
    let owner = components(links.len(), &incidence(&links));
    let mut by: Keyed<Vec<Link>> = Keyed::new();
    for (i, link) in links.into_iter().enumerate() {
        let k = index_key(owner[i]);
        match by.get_mut(&k) {
            Some(v) => v.push(link),
            None => {
                by.insert(k, vec![link]);
            }
        }
    }
    // The components this function just grouped, keyed by owner: every
    // piece of the chain is a piece of the answer.
    by.scoped().map(|(_, v)| v.clone()).collect()
}

/// THE INCIDENCE of a support: its 1-skeleton, the pairs of links whose
/// sites share an edge, read off the support itself and never by looking a
/// coordinate up. In row order two sites are edge-adjacent along a row
/// exactly when they are consecutive in the list and differ by one column;
/// in column order, exactly when consecutive and one row apart. Two linear
/// passes over two orderings of the same support. `links` must be sorted
/// and deduplicated; the pairs are indices into it.
pub fn incidence(links: &[Link]) -> Vec<(usize, usize)> {
    let positions: Vec<(i64, i64)> = links.iter().map(|c| (c.0, c.1)).collect();
    incidence_of(&positions)
}

/// The incidence of a support given as sites in row order (sorted,
/// deduplicated): the same two linear passes, for any carrier of sites.
pub fn incidence_of(positions: &[(i64, i64)]) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for i in 1..positions.len() {
        let (p, q) = (positions[i - 1], positions[i]);
        if p.0 == q.0 && p.1 + 1 == q.1 {
            edges.push((i - 1, i));
        }
    }
    let mut by_column: Vec<usize> = (0..positions.len()).collect();
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — canonical key order: column-major over the sites' own coordinates, which is where a vertical adjacency is adjacent"
    )]
    by_column.sort_unstable_by_key(|&i| (positions[i].1, positions[i].0));
    for w in by_column.windows(2) {
        let (p, q) = (positions[w[0]], positions[w[1]]);
        if p.1 == q.1 && p.0 + 1 == q.0 {
            edges.push((w[0], w[1]));
        }
    }
    edges
}

/// THE FACE INCIDENCE of a support of sites (51.3, 71.2(2)): two cubes
/// share a face where exactly one coordinate differs, by one. Read one axis
/// at a time: the sites ordered by every other coordinate and then by that
/// axis, so the two ends of every such adjacency are successors. The grid's
/// two axes are read first — columns, then rows — and every axis off the
/// grid after them, so at the corpus's `n = 3` the passes and the edges'
/// order are the ones the three fixed passes made: `(k, r, c)`, `(k, c, r)`,
/// `(r, c, k)`. Two sites of different `n` are never adjacent.
pub fn incidence_of_3d(positions: &[crate::transducer::base::Site]) -> Vec<(usize, usize)> {
    use crate::transducer::base::{is_grid_axis, GRID_AXES};
    let mut edges = Vec::new();
    let n = positions.iter().map(|s| s.n()).fold(0, usize::max);
    let off_grid: Vec<usize> = (0..n).filter(|i| !is_grid_axis(*i)).collect();
    let mut passes: Vec<usize> = vec![GRID_AXES[1], GRID_AXES[0]];
    passes.extend(off_grid.iter().copied());
    for a in passes.into_iter().filter(|&a| a < n) {
        // The order this axis's adjacencies are successors in: the other
        // axes off the grid, then the other grid axes, then this one.
        let mut order: Vec<usize> = off_grid.iter().copied().filter(|&i| i != a).collect();
        order.extend(GRID_AXES.iter().copied().filter(|&i| i != a && i < n));
        order.push(a);
        let key = |s: &crate::transducer::base::Site| -> Vec<i64> {
            order.iter().map(|&i| s.axis(i)).collect::<Option<Vec<i64>>>().into_iter().flatten().collect()
        };
        let mut by: Vec<usize> = (0..positions.len()).collect();
        by.sort_by_cached_key(|&i| (positions[i].n(), key(&positions[i])));
        for w in by.windows(2) {
            let (p, q) = (&positions[w[0]], &positions[w[1]]);
            if p.n() != q.n() {
                continue;
            }
            let adjacent = p.axes().iter().zip(q.axes()).enumerate().all(|(i, (x, y))| match i == a {
                true => x + 1 == *y,
                false => x == y,
            });
            if adjacent {
                edges.push((w[0], w[1]));
            }
        }
    }
    edges
}

/// The connected components of `n` sites under an incidence: each site's
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
pub fn cited_pattern(word: &Element, asg: usize) -> Option<Vec<Link>> {
    let pos = element::positions(word.sub());
    // The citation NAMES the assignment by its index along the word's axis
    // — a coordinate the cite carries on the wire, not a member taken
    // because it came first (rule 15).
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 15 — wire tag offset: the gauge index is the cite's own coordinate along the word's axis"
    )]
    let assignment = word.assignments().into_iter().nth(asg)?;
    let mut pat: Vec<Link> = pos.iter().zip(&assignment).map(|((r, c), (b, a))| (*r, *c, *b, *a)).collect();
    pat.sort_unstable();
    Some(pat)
}

/// THE REPLAY — in the algebra and linear in the proof. For every citation
/// the word's own laminae are carried forward by V, each as its unit
/// square (exact, or refused: a lamina the versor carries onto no block of
/// laminae — a tilt, a shrink — speaks nothing, law 11), and every image
/// lamina is one keyed read of the target: a hit with the same transition
/// is spoken, an image lamina the target lacks is over-explanation. The
/// scene is never walked. The
/// residual is R = Q △ S: the target links no citation speaks, plus the
/// over-explained links. An empty residual is closure; a citation naming
/// nothing held speaks nothing.
pub fn replay_at<'a>(word_at: &impl Fn(&[u8]) -> Option<&'a Element>, cites: &[Cite], target: &[Link]) -> Vec<Link> {
    let ops: Vec<(Versor, Vec<Link>)> = cites
        .iter()
        .filter_map(|c| Some((c.transport.clone(), cited_pattern(word_at(c.word.as_bytes())?, c.gauge)?)))
        .collect();
    replay_operators(&ops, target)
}

/// The key of a section's site: a section's sites stand in frame `0`, their
/// own (16.5), and the key is that site's own ([`Site::key`], `n` first). A
/// site of any other base is keyed by `Site::key` directly.
pub(crate) fn section_site_key(r: i64, c: i64) -> Vec<u8> {
    crate::transducer::base::Site::from((r, c, 0)).key()
}

/// THE REPLAY OF OPERATORS: each operator is a body (the word's links in
/// its own frame) with the versor that carries it; nothing is fetched. The
/// utterance is verified as it was spoken. The target is one section, in
/// its own frame: a word's lamina carried into another frame speaks
/// nothing of it, and the section's site it was to cover stays unspoken.
pub fn replay_operators(ops: &[(Versor, Vec<Link>)], target: &[Link]) -> Vec<Link> {
    let mut at: Keyed<Link> = Keyed::new();
    for &link in target {
        at.insert(section_site_key(link.0, link.1), link);
    }
    let mut spoken: Keyed<()> = Keyed::new();
    let mut over: Vec<Link> = Vec::new();
    for (v, pat) in ops {
        for &(wr, wc, wb, wa) in pat {
            let Some(image) = element::carry_lamina(v, wr, wc, 0) else { continue };
            for (tr, tc, tk) in image {
                if tk != 0 {
                    continue;
                }
                let k = section_site_key(tr, tc);
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
    // ONE CHAIN'S LINKS, keyed by site: the residual is what the word did
    // not speak, so every link is asked and none may be skipped.
    let mut residual: Vec<Link> = at
        .scoped()
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
/// every site they land on. An index into a word's assignments is not a
/// stable coordinate across copies of the word (a copy that gained a
/// section orders them differently); the target's own transitions are.
/// None when no assignment agrees.
pub fn gauge_of(word: &Element, v: &Versor, target: &[Link]) -> Keyed<usize> {
    let mut at: Keyed<Link> = Keyed::new();
    for &link in target {
        at.insert(section_site_key(link.0, link.1), link);
    }
    let pos = element::positions(word.sub());
    let landed: Vec<(u8, u8)> = pos
        .iter()
        .filter_map(|&(wr, wc)| {
            let p = element::carry_center(v, wr, wc, 0)?;
            let (tr, tc, tk) = element::site_of(&p)?;
            if tk != 0 {
                return None;
            }
            let t = at.get(&section_site_key(tr, tc))?;
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
            let Some(p) = element::carry_center(v, wr, wc, 0) else { return true };
            // A centre off the base, or off the section's frame, lands on
            // no site to disagree with.
            let Some((tr, tc, tk)) = element::site_of(&p) else { return true };
            if tk != 0 {
                return true;
            }
            match at.get(&section_site_key(tr, tc)) {
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
/// their words onto the same target site, they must speak the same
/// transition; the links where they disagree are the seam defect, located
/// exactly. A sentence with no overlap has no seam.
pub fn seam_defect_at<'a>(
    word_at: &impl Fn(&[u8]) -> Option<&'a Element>,
    cites: &[Cite],
    target: &[Link],
) -> Vec<Link> {
    let ops: Vec<(Versor, Vec<Link>)> = cites
        .iter()
        .filter_map(|c| Some((c.transport.clone(), cited_pattern(word_at(c.word.as_bytes())?, c.gauge)?)))
        .collect();
    seam_defect_operators(&ops, target)
}

pub fn seam_defect_operators(ops: &[(Versor, Vec<Link>)], target: &[Link]) -> Vec<Link> {
    let mut first: Keyed<(u8, u8)> = Keyed::new();
    let mut conflict: Keyed<()> = Keyed::new();
    let mut at: Keyed<Link> = Keyed::new();
    for &link in target {
        at.insert(section_site_key(link.0, link.1), link);
    }
    for (v, pat) in ops {
        for &(wr, wc, wb, wa) in pat {
            let Some(p) = element::carry_center(v, wr, wc, 0) else { continue };
            let Some((tr, tc, tk)) = element::site_of(&p) else { continue };
            if tk != 0 {
                continue;
            }
            let k = section_site_key(tr, tc);
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
    // The same chain, asked for the links two words disagree on.
    at.scoped()
        .filter(|(k, _)| conflict.contains(k))
        .map(|(_, t)| *t)
        .collect()
}

/// VERIFY a received sentence — the receiving direction of the algebra:
/// the replay's residual together with the seam defect. Empty is closure
/// with a coherent cover.
pub fn verify_at<'a>(
    word_at: &impl Fn(&[u8]) -> Option<&'a Element>,
    cites: &[Cite],
    target: &[Link],
) -> Vec<Link> {
    let mut out = replay_at(word_at, cites, target);
    out.extend(seam_defect_at(word_at, cites, target));
    out.sort_unstable();
    out.dedup();
    out
}

/// VERIFY AN UTTERANCE as spoken: the operators the citations carry are
/// replayed against the target and checked on their seams, with no copy
/// of any word fetched. The hearer's lexicon is consulted only afterward,
/// to name what was verified.
pub fn verify_utterance(cites: &[crate::anastome::dialogue::CiteWire], target: &[Link]) -> Vec<Link> {
    let ops: Vec<(Versor, Vec<Link>)> =
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
    /// The held cite, when no ruled or stood word is on the body's chart
    /// and exactly one held form's word reconstructs the body (24.1):
    /// Ruled first, then Stood, then Held.
    pub held: Option<HeldCite>,
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

/// H2, 70.4 — THE DIAGONAL INCIDENCE, WHICH IS INTER-BODY CONTACT.
///
/// Under H1 a body is a maximal HUE-BLIND connected region, so two
/// distinct bodies can never be 4-adjacent: if they were, they would be
/// ONE body. 4-adjacency therefore stops being the inter-body relation
/// and becomes strictly interior, and contact needs its own primitive.
///
/// It is the shared boundary that is not a face. Two maximal regions CAN
/// meet at an edge or a corner of their cubes without merging, because
/// neither is in the relation that formed them, so this is exactly the
/// relation the six face-adjacencies leave available.
///
/// KEPT APART FROM [`incidence_of_3d`] ON PURPOSE. If the diagonal ever
/// entered the component relation it would merge bodies that only touch
/// at a corner, and the base would swallow the contact it is meant to
/// expose. One primitive defines bodies; the other relates them.
///
/// C2, 70.6 — THE WHOLE SHARED BOUNDARY OF A CUBE COMPLEX ON `(r, c, k)`.
/// The corner in one frame is four of the twenty. Two cubes that share no
/// face can still share an EDGE or a CORNER. The twelve edge-adjacencies
/// are the four in the frame, `(r ± 1, c ± 1, k)`, and the eight across
/// consecutive frames, `(r ± 1, c, k ± 1)` and `(r, c ± 1, k ± 1)`. The
/// eight corner-adjacencies differ by one in all three coordinates. The six
/// face-adjacencies are [`incidence_of_3d`]'s, and they make a body.
///
/// The twenty neighbours are ten DIRECTIONS up to sign, and each direction
/// is one canonical order, as in the rest of this file. `d` is written with
/// its first non-zero coordinate `+1`. Along `d` that coordinate, `t`,
/// is the parameter of the line, and the other two less `d`'s share of `t`
/// stay constant. So ordering by those two invariants and then by `t` puts the
/// two ends of every such adjacency next to each other, and the test is
/// `t + 1`. The result is exact and integer, with no extent and no
/// distance test against every lamina.
///
/// EACH PAIR CARRIES ITS DIRECTION, `q − p = d`. That is content, the
/// coordinates of the adjacency itself, and not a label: `d` zero on the
/// frame axis is contact within one frame, and `±1` there is the frame axis
/// entering contact.
///
/// THE DIRECTIONS ARE GENERATED FROM `n` (71.2(2)): every `d ∈ {−1,0,1}ⁿ`
/// with two or more non-zero coordinates, the first of them `+1` — by how
/// many differ, then by which axes, then by the signs. At the corpus's
/// `n = 3` that is the ten, in the order the fixed list had them.
pub fn incidence_diagonal_3d(positions: &[crate::transducer::base::Site]) -> Vec<(usize, usize, Vec<i64>)> {
    let mut edges = Vec::new();
    let n = positions.iter().map(|s| s.n()).fold(0, usize::max);
    for d in shared_boundary_directions(n) {
        // `t` is the coordinate `d` leads with. The invariants are the
        // coordinates less `d`'s share of `t`, so they are constant along
        // `d`.
        let lead = match d.iter().position(|&x| x != 0) {
            Some(i) => i,
            None => continue,
        };
        let line = |p: &crate::transducer::base::Site| -> (usize, Vec<i64>, i64) {
            #[expect(
                clippy::manual_unwrap_or_default,
                reason = "the inclusion ℚᵐ ⊂ ℚⁿ (71): an axis a smaller base lacks is zero in the larger, measured and not defaulted"
            )]
            let t = match p.axis(lead) {
                Some(t) => t,
                None => 0,
            };
            let inv = p.axes().iter().zip(&d).map(|(x, dx)| x - dx * t).collect();
            (p.n(), inv, t)
        };
        let mut along: Vec<usize> = (0..positions.len()).collect();
        // Canonical key order (rule 4): the line along d, then the
        // parameter on it, is where a neighbour along d is a successor.
        along.sort_by_cached_key(|&i| line(&positions[i]));
        for w in along.windows(2) {
            let ((np, lp, tp), (nq, lq, tq)) = (line(&positions[w[0]]), line(&positions[w[1]]));
            if np == n && nq == n && lp == lq && tp + 1 == tq {
                edges.push((w[0], w[1], d.clone()));
            }
        }
    }
    edges
}

/// The directions of the shared boundary of a cube of ℤⁿ that is not a
/// face: every `d ∈ {−1,0,1}ⁿ` with two or more non-zero coordinates, the
/// first of them `+1`, one per pair of opposite neighbours. By how many
/// coordinates differ, then by which axes (in order), then by the signs of
/// the axes after the first, `+` before `−`.
pub fn shared_boundary_directions(n: usize) -> Vec<Vec<i64>> {
    let mut out = Vec::new();
    for m in 2..=n {
        // The axis sets of size m, in lexicographic order.
        let mut sets: Vec<Vec<usize>> = vec![Vec::new()];
        for i in 0..n {
            let mut next = Vec::new();
            for s in &sets {
                next.push(s.clone());
                if s.len() < m {
                    let mut t = s.clone();
                    t.push(i);
                    next.push(t);
                }
            }
            sets = next;
        }
        let mut sets: Vec<Vec<usize>> = sets.into_iter().filter(|s| s.len() == m).collect();
        sets.sort_unstable();
        for axes in sets {
            // The signs of the axes after the first: m − 1 of them, + first.
            for code in 0..(1usize << (m - 1)) {
                let mut d = vec![0i64; n];
                d[axes[0]] = 1;
                for (b, &ax) in axes[1..].iter().enumerate() {
                    let minus = (code >> (m - 2 - b)) & 1 == 1;
                    d[ax] = if minus { -1 } else { 1 };
                }
                out.push(d);
            }
        }
    }
    out
}
