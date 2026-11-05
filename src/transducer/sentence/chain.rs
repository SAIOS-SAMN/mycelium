// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 18.2, 6 — THE CHAIN: its links and the links' incidence, the citations and
//! punctures a deconstruction produces, the residue no word speaks, and the
//! reconstruction that measures what a proof leaves.

use crate::network::nerve::Keyed;
use crate::transducer::element::{self, Element, Versor};
use crate::transducer::candidates;
use crate::transducer::spelled::SectionSite;

fn index_key(i: usize) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..8].copy_from_slice(&(i as u64).to_le_bytes());
    k
}

/// A LINK OF THE ALPHABET'S SECTION (16.5; MU-SENSING §3.6, §4): a site
/// of the section ([`SectionSite`], the sample's coordinate vector
/// restricted by its perspective to the letters' plane) and the symbols
/// before and after the change there, one byte each, on the sample of the
/// rank the seventeen letters spell (`alphabet::ALPHABET_SAMPLE_RANK`).
/// The word, chart and field carriers that spell a chain read this
/// projection; a link of any other rank or width reaches them only through
/// [`Link::section`], refused by name ([`SectionRefusal`]), never
/// truncated. The order is the site's, then the two symbols.
///
/// [`Link::section`]: super::Link::section
/// [`SectionRefusal`]: super::SectionRefusal
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SectionLink {
    /// Where the change stands on the section.
    pub site: SectionSite,
    /// The symbol before the change.
    pub before: u8,
    /// The symbol after the change.
    pub after: u8,
}

impl SectionLink {
    /// The link at `site` carrying `before` to `after`.
    pub fn new(site: SectionSite, before: u8, after: u8) -> SectionLink {
        SectionLink { site, before, after }
    }

    /// The link at the site of the section whose coordinates along the
    /// perspective's plane are `coordinates`, in the plane's order.
    pub fn on_plane(coordinates: [i64; crate::transducer::alphabet::ALPHABET_SAMPLE_RANK], before: u8, after: u8) -> SectionLink {
        SectionLink { site: SectionSite::on_plane(coordinates), before, after }
    }

    /// The transition the link carries: the symbol before, then after.
    pub fn transition(&self) -> (u8, u8) {
        (self.before, self.after)
    }
}

/// A CITATION: the coordinates a sentence speaks for one word — its
/// address, the transport carrying the word's chain into the utterance's
/// frame, and the assignment along the punctured axis the body carries. A
/// proof is a sequence of citations; a reconstruction applies them and measures
/// what remains.
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
    /// THE ASSIGNMENT ITSELF along the punctured axis — the hue pairs the
    /// cited body carries, the algebra's domain. It was `gauge: usize`, an
    /// index into the word's assignment list, carried on the wire beside a
    /// body that already says it (law 12), with `u32::MAX` a sentinel for
    /// "a section". The word is asked whether it holds this assignment
    /// ([`cited_pattern`]), never indexed.
    pub assignment: Vec<(u8, u8)>,
}

/// A PUNCTURE: a body a held word carries by SHAPE under a gauge the word
/// does not yet hold. It is spoken as a section of that word — the fibre
/// deepens instead of the vocabulary widening — so it is not a citation and
/// not a residue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Puncture {
    /// The word that carries the body, by its coordinate — the same reading
    /// as `Cite::word`.
    pub word: crate::transducer::address::Address,
    /// The transport taking the word's shape onto the body.
    pub transport: Versor,
    /// The word's links under the BODY's own transitions — what the section
    /// says, in the body's hues and not the word's.
    pub links: Vec<SectionLink>,
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
    pub body: Vec<SectionLink>,
}

/// What deconstruction produced: the citations in body order, the held
/// cites derived at the bodies' charts, the connection between
/// consecutive cited bodies (the relative transport `Vᵢ⁻¹ Vⱼ`), the
/// residue no word speaks, and the widest stratum read.
#[derive(Debug, Clone, Default)]
pub struct Sentence {
    pub cites: Vec<Cite>,
    /// PUNCTURES: bodies a held word carries by shape under a gauge the
    /// word does not yet hold — (word, versor, the word's links under the
    /// body's own transitions). Spoken as sections of the word, not as new
    /// words: the fibre deepens instead of the vocabulary widening.
    pub punctures: Vec<Puncture>,
    /// HELD CITES: bodies a held form's word reconstructs (24.1), one per
    /// body, each derived at the body's chart when the reading asked.
    pub held: Vec<HeldCite>,
    pub connection: Vec<Versor>,
    pub residue: Vec<SectionLink>,
    /// Stratum width per body key, never a maximum over bodies.
    pub multiplicity: Keyed<usize>,
}

impl Sentence {
    /// Closure: every body spoken by held words as held, nothing left over
    /// and no fibre to deepen.
    pub fn is_closed(&self) -> bool {
        self.residue.is_empty() && self.punctures.is_empty()
    }

    /// The operators this sentence's citations carry: each cited word's
    /// pattern under its transport, and each held cite's body in the frame
    /// it was derived in, which is the chain's own.
    fn operators_at<'a>(&self, word_at: &impl Fn(&[u8]) -> Option<&'a Element>) -> Vec<(Versor, Vec<SectionLink>)> {
        let mut ops: Vec<(Versor, Vec<SectionLink>)> = self
            .cites
            .iter()
            .filter_map(|c| Some((c.transport.clone(), cited_pattern(word_at(c.word.as_bytes())?, &c.assignment)?)))
            .collect();
        ops.extend(self.held.iter().map(|h| (element::identity_versor(), h.body.clone())));
        ops
    }

    /// What the language does not yet hold: the residue, together with the
    /// bodies spoken only as punctures — novelty in the fibre is novelty.
    /// Measured as the reconstruction of the citations alone over the chain.
    pub fn open_at<'a>(&self, word_at: &impl Fn(&[u8]) -> Option<&'a Element>, chain: &[SectionLink]) -> Vec<SectionLink> {
        match self.punctures.is_empty() {
            true => self.residue.clone(),
            false => reconstruct_operators(&self.operators_at(word_at), chain),
        }
    }
}

/// PUNCTURE the held words a sentence's carried bodies name: a carried body
/// on a word's own support under transitions the word does not yet hold
/// becomes a section of that word. Returns how many fibres deepened. The
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
        let asg = body_assignment(body);
        // An assignment the word already holds is a citation, not a
        // puncture: the one reading, [`holds_assignment`].
        if holds_assignment(word, &asg) {
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

/// The connected components of a chain's support under contact through a
/// shared cell of grade `n − 1` (a face, MU-RECURSION 71.2 (2)), each as a
/// sorted chain. The sensor's segmentation, not a search.
pub fn pieces(chain: &[SectionLink]) -> Vec<Vec<SectionLink>> {
    let mut links: Vec<SectionLink> = chain.to_vec();
    links.sort_unstable();
    links.dedup();
    let owner = components(links.len(), &incidence(&links));
    let mut by: Keyed<Vec<SectionLink>> = Keyed::new();
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
/// sites share a cell of grade `n − 1` (a face), read off the support itself
/// and never by looking a coordinate up ([`incidence_of_sites`]). `links`
/// must be sorted and deduplicated; the pairs are indices into it.
pub fn incidence(links: &[SectionLink]) -> Vec<(usize, usize)> {
    incidence_of(&element::positions(links))
}

/// The incidence of a support of the alphabet's section given as section
/// sites in canonical order (sorted, deduplicated): [`incidence_of_sites`]
/// on the same sites, each the sample's coordinate vector on the plane.
pub fn incidence_of(positions: &[SectionSite]) -> Vec<(usize, usize)> {
    let sites: Vec<crate::transducer::base::Site> =
        positions.iter().map(|s| crate::transducer::base::Site::of(s.coordinates())).collect();
    incidence_of_sites(&sites)
}

/// THE FACE INCIDENCE OF A SUPPORT AT ANY RANK (MU-RECURSION 71.2 (2);
/// MU-SENSING §3.2): the pairs of sites whose unit cubes share a cell of
/// grade `n − 1`, read by [`shared_cells`] and never by a fixed 4- or
/// 6-neighbourhood. Ordered by the axis the two sites differ on, the last
/// axis (the fastest in canonical address order) first, and along each axis
/// in the canonical order of its lines: at rank 2 this is the order of the
/// two linear passes the planar reading took (consecutive along the fastest
/// axis, then along the slower), so a component's root is read as before.
/// `positions` must be sorted and deduplicated; the pairs are indices into
/// it.
pub fn incidence_of_sites(positions: &[crate::transducer::base::Site]) -> Vec<(usize, usize)> {
    let mut faces: Vec<SharedCell> = shared_cells(positions).into_iter().filter(SharedCell::is_face).collect();
    // A stable order by the differing axis, last first: the order within one
    // axis is `shared_cells`' own canonical one, kept.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — canonical key order: the differing axis is the face's own coordinate, the fastest axis first"
    )]
    faces.sort_by_key(|c| std::cmp::Reverse(c.axes.first().copied()));
    faces.into_iter().map(|c| (c.i, c.j)).collect()
}

/// THE FACE INCIDENCE of a support of sites (51.3, 71.2(2)): the pairs of
/// [`shared_cells`] of grade `n − 1`, whose cubes share a face — exactly one
/// coordinate differs, by one. This is the relation that makes a body, and
/// it feeds [`components`]; every lower grade is contact.
pub fn face_incidence(positions: &[crate::transducer::base::Site]) -> Vec<(usize, usize)> {
    shared_cells(positions).into_iter().filter(SharedCell::is_face).map(|c| (c.i, c.j)).collect()
}

/// The connected components of `n` sites under an incidence: each site's
/// component root (union-find over the edges).
pub fn components(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut parent: Vec<usize> = (0..n).collect();
    for &(i, j) in edges {
        let (ri, rj) = (candidates::find(&mut parent, i), candidates::find(&mut parent, j));
        if ri != rj {
            parent[ri] = rj;
        }
    }
    (0..n).map(|i| candidates::find(&mut parent, i)).collect()
}

/// The transition profile of an assignment: sorted ((before, after), count).
/// The gauge charge of a body.
pub fn profile(asg: &[(u8, u8)]) -> Vec<((u8, u8), u32)> {
    element::gauge_profile(asg)
}

/// THE ASSIGNMENT A CITED BODY CARRIES, read off it in its support's
/// order — the order a word's canonical chain stands in, so it compares
/// with the word's own assignments.
pub fn body_assignment(body: &[SectionLink]) -> Vec<(u8, u8)> {
    let mut carried = body.to_vec();
    carried.sort_unstable();
    element::assignment(&carried)
}

/// DOES THE WORD HOLD THIS ASSIGNMENT — the one reading of it. The charge's
/// keyed read (`assignments_by_profile`), then exact equality among the
/// assignments of that charge; never the whole axis, never an index.
pub fn holds_assignment(word: &Element, asg: &[(u8, u8)]) -> bool {
    word.assignments_by_profile(&element::gauge_key(asg)).iter().any(|a| a.as_slice() == asg)
}

/// The chain a citation names in the word's own frame: the word's support
/// under the assignment the cite names. `None` where the word does not hold
/// that assignment (rule 11): the word is ASKED — the charge's keyed read,
/// then exact equality among the assignments of that charge — never
/// indexed, so no position along the axis stands for an assignment.
pub fn cited_pattern(word: &Element, asg: &[(u8, u8)]) -> Option<Vec<SectionLink>> {
    holds_assignment(word, asg).then_some(())?;
    let pos = element::positions(word.sub());
    // One transition per site: an assignment of another length is not this
    // word's, whatever its charge.
    (pos.len() == asg.len()).then_some(())?;
    let mut pat: Vec<SectionLink> = pos.iter().zip(asg).map(|(&site, &(b, a))| SectionLink::new(site, b, a)).collect();
    pat.sort_unstable();
    Some(pat)
}

/// THE RECONSTRUCTION — in the algebra and linear in the proof. For every citation
/// the word's own compartments are carried forward by V, each as its unit
/// square (exact, or refused: a compartment the versor carries onto no block of
/// compartments — a tilt, a shrink — speaks nothing, law 11), and every image
/// compartment is one keyed read of the target: a hit with the same transition
/// is spoken, an image compartment the target lacks is over-explanation. The
/// scene is never read whole. The
/// residue is R = Q △ S: the target links no citation speaks, plus the
/// over-explained links. An empty residue is closure; a citation naming
/// nothing held speaks nothing.
pub fn reconstruct_at<'a>(word_at: &impl Fn(&[u8]) -> Option<&'a Element>, cites: &[Cite], target: &[SectionLink]) -> Vec<SectionLink> {
    let ops: Vec<(Versor, Vec<SectionLink>)> = cites
        .iter()
        .filter_map(|c| Some((c.transport.clone(), cited_pattern(word_at(c.word.as_bytes())?, &c.assignment)?)))
        .collect();
    reconstruct_operators(&ops, target)
}

/// The key of a section's site: a section's sites stand in frame `0`, the
/// sites' own (16.5), and the key is that site's own ([`Site::key`], `n` first). A
/// site of any other base is keyed by `Site::key` directly.
pub(crate) fn section_site_key(site: crate::transducer::spelled::SectionSite) -> Vec<u8> {
    let [r, c] = site.coordinates();
    crate::transducer::base::Site::from((r, c, 0)).key()
}

/// THE RECONSTRUCTION OF OPERATORS: each operator is a body (the word's links in
/// its own frame) with the versor that carries it; nothing is fetched. The
/// utterance is verified as it was spoken. The target is one section, in
/// its own frame: a word's compartment carried into another frame speaks
/// nothing of it, and the section's site it was to cover stays unspoken.
pub fn reconstruct_operators(ops: &[(Versor, Vec<SectionLink>)], target: &[SectionLink]) -> Vec<SectionLink> {
    let mut at: Keyed<SectionLink> = Keyed::new();
    for &link in target {
        at.insert(section_site_key(link.site), link);
    }
    let mut spoken: Keyed<()> = Keyed::new();
    let mut over: Vec<SectionLink> = Vec::new();
    for (v, pat) in ops {
        for w in pat {
            let [wr, wc] = w.site.coordinates();
            let Some(image) = element::carry_compartment(v, wr, wc, 0) else { continue };
            for (tr, tc, tk) in image {
                if tk != 0 {
                    continue;
                }
                let landed = SectionSite::on_plane([tr, tc]);
                let k = section_site_key(landed);
                match at.get(&k) {
                    Some(t) if t.transition() == w.transition() => {
                        spoken.insert(k, ());
                    }
                    Some(_) => {}
                    None => over.push(SectionLink::new(landed, w.before, w.after)),
                }
            }
        }
    }
    // ONE CHAIN'S LINKS, keyed by site: the residue is what the word did
    // not speak, so every link is asked and none may be skipped.
    let mut residue: Vec<SectionLink> = at
        .scoped()
        .filter(|(k, _)| !spoken.contains(k))
        .map(|(_, t)| *t)
        .collect();
    residue.extend(over);
    residue.sort_unstable();
    residue.dedup();
    residue
}

/// THE GAUGE COORDINATE OF A CITATION, read from the target: the word's
/// assignment whose transitions, carried by `v`, agree with the target on
/// every site they land on. An index into a word's assignments is not a
/// stable coordinate across copies of the word (a copy that gained a
/// section orders them differently); the target's own transitions are.
/// None when no assignment agrees.
pub fn gauge_of(word: &Element, v: &Versor, target: &[SectionLink]) -> Keyed<Vec<(u8, u8)>> {
    let mut at: Keyed<SectionLink> = Keyed::new();
    for &link in target {
        at.insert(section_site_key(link.site), link);
    }
    let pos = element::positions(word.sub());
    let landed: Vec<(u8, u8)> = pos
        .iter()
        .filter_map(|w| {
            let [wr, wc] = w.coordinates();
            let p = element::carry_centre(v, wr, wc, 0)?;
            let (tr, tc, tk) = element::site_of(&p)?;
            if tk != 0 {
                return None;
            }
            let t = at.get(&section_site_key(SectionSite::on_plane([tr, tc])))?;
            Some(t.transition())
        })
        .collect();
    let mut agreeing = Keyed::new();
    if landed.is_empty() {
        return agreeing;
    }
    for assignment in word.assignments_by_profile(&element::gauge_key(&landed)) {
        let ok = pos.iter().zip(&assignment).all(|(w, &(wb, wa))| {
            let [wr, wc] = w.coordinates();
            let Some(p) = element::carry_centre(v, wr, wc, 0) else { return true };
            // A centre off the base, or off the section's frame, lands on
            // no site to disagree with.
            let Some((tr, tc, tk)) = element::site_of(&p) else { return true };
            if tk != 0 {
                return true;
            }
            match at.get(&section_site_key(SectionSite::on_plane([tr, tc]))) {
                Some(t) => t.transition() == (wb, wa),
                None => true,
            }
        });
        if ok {
            agreeing.insert(element::assignment_key(&assignment), assignment);
        }
    }
    agreeing
}

/// THE SEAM CONDITION — the cocycle on overlaps. Where two citations carry
/// the words they cite onto the same target site, they must speak the same
/// transition; the links where they disagree are the seam defect, located
/// exactly. A sentence with no overlap has no seam.
pub fn seam_defect_at<'a>(
    word_at: &impl Fn(&[u8]) -> Option<&'a Element>,
    cites: &[Cite],
    target: &[SectionLink],
) -> Vec<SectionLink> {
    let ops: Vec<(Versor, Vec<SectionLink>)> = cites
        .iter()
        .filter_map(|c| Some((c.transport.clone(), cited_pattern(word_at(c.word.as_bytes())?, &c.assignment)?)))
        .collect();
    seam_defect_operators(&ops, target)
}

pub fn seam_defect_operators(ops: &[(Versor, Vec<SectionLink>)], target: &[SectionLink]) -> Vec<SectionLink> {
    let mut first: Keyed<(u8, u8)> = Keyed::new();
    let mut conflict: Keyed<()> = Keyed::new();
    let mut at: Keyed<SectionLink> = Keyed::new();
    for &link in target {
        at.insert(section_site_key(link.site), link);
    }
    for (v, pat) in ops {
        for w in pat {
            let ([wr, wc], (wb, wa)) = (w.site.coordinates(), w.transition());
            let Some(p) = element::carry_centre(v, wr, wc, 0) else { continue };
            let Some((tr, tc, tk)) = element::site_of(&p) else { continue };
            if tk != 0 {
                continue;
            }
            let k = section_site_key(SectionSite::on_plane([tr, tc]));
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
/// the reconstruction's residue together with the seam defect. Empty is closure
/// with a coherent cover.
pub fn verify_at<'a>(
    word_at: &impl Fn(&[u8]) -> Option<&'a Element>,
    cites: &[Cite],
    target: &[SectionLink],
) -> Vec<SectionLink> {
    let mut out = reconstruct_at(word_at, cites, target);
    out.extend(seam_defect_at(word_at, cites, target));
    out.sort_unstable();
    out.dedup();
    out
}

/// VERIFY AN UTTERANCE as spoken: the operators the citations carry are
/// reconstructed against the target and checked on the seams, with no copy
/// of any word fetched. The hearer's lexicon is consulted only afterward,
/// to name what was verified.
pub fn verify_utterance(cites: &[crate::anastome::dialogue::CiteWire], target: &[SectionLink]) -> Vec<SectionLink> {
    let ops: Vec<(Versor, Vec<SectionLink>)> =
        cites.iter().map(|c| (c.transport.clone(), c.body.clone())).collect();
    let mut out = reconstruct_operators(&ops, target);
    out.extend(seam_defect_operators(&ops, target));
    out.sort_unstable();
    out.dedup();
    out
}

/// THE HOLONOMY of a cycle of transports: the transports' product around the orbit.
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
/// up to the projective scale — the orbit closes with no stress.
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
/// meet at an edge or a corner of the regions' cubes without merging, because
/// neither is in the relation that formed them, so this is exactly the
/// relation the six face-adjacencies leave available.
///
/// KEPT APART FROM [`face_incidence`] ON PURPOSE. If the diagonal ever
/// entered the component relation it would merge bodies that only touch
/// at a corner, and the base would swallow the contact it is meant to
/// expose. One relation defines bodies; the other relates them — and both
/// are read off ONE reader, [`shared_cells`], by grade (71.2(2), BQ2).
///
/// These are the shared cells of grade below `n − 1`: an edge, a corner,
/// and at `n > 3` every cell between.
pub fn contact_cells(positions: &[crate::transducer::base::Site]) -> Vec<SharedCell> {
    shared_cells(positions).into_iter().filter(|c| !c.is_face()).collect()
}

/// 71.2(2), BQ2 — ONE CELL TWO CUBES SHARE: the two sites, the offset
/// `q − p` between them, the AXES ON WHICH THEY DIFFER, and the GRADE of the
/// cell they share, `n − m` for `m` differing axes. Two unit cubes of ℤⁿ
/// whose sites differ by one on `m` axes and agree on the rest share exactly
/// the cell spanned by the `n − m` axes they agree on: a face at `m = 1`
/// (grade `n − 1`), an edge at `n − 2`, down to a corner at grade `0`.
///
/// THE GRADE IS THE CONTACT (71.2(2)). It is read off the pair, never
/// assigned by a table of neighbour kinds: at `n = 3` the six faces, twelve
/// edges and eight corners are what the grades come to, and at `n = 4` the
/// same reader gives the eight faces, twenty-four 2-cells, thirty-two edges
/// and sixteen corners without a line changing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SharedCell {
    pub i: usize,
    pub j: usize,
    /// `q − p`, its first non-zero coordinate `+1`.
    pub d: Vec<i64>,
    /// The axes the two sites differ on, ascending: `m` of them.
    pub axes: Vec<usize>,
    /// The dimension of the shared cell, `n − m`.
    pub grade: usize,
}

impl SharedCell {
    /// A face: one differing axis. The cell that makes a body.
    pub fn is_face(&self) -> bool {
        matches!(self.axes.as_slice(), [_])
    }

    /// Does the shared cell lie across axis `a` — do the two sites differ
    /// on it? At the frame axis, a contact between two frames (51.3).
    #[cfg(test)]
    pub fn crosses(&self, a: usize) -> bool {
        self.axes.contains(&a)
    }
}

/// THE ONE READER OF THE NEIGHBOURHOOD (71.2(2), BQ2): every pair of sites
/// whose unit cubes share a cell of the boundary, with its offset, its
/// differing axes and its grade.
///
/// The neighbourhood is GENERATED: [`neighbourhood`]`(n)` is `{−1,0,1}ⁿ ∖ 0`
/// up to sign, one direction per pair of opposite neighbours. Each direction
/// is one canonical order, as in the rest of this file: along `d` its first
/// non-zero coordinate `t` is the parameter of the line and the rest less
/// `d`'s share of `t` stay constant, so ordering by those invariants and
/// then by `t` puts the two ends of every adjacency along `d` next to each
/// other, and the test is `t + 1`. Exact and integer, with no extent and no
/// distance test against every compartment — the directions are read in turn, the
/// sites are ordered, and nothing is searched.
///
/// Two sites of different `n` share no cell: a cell is of one base. EVERY
/// BASE PRESENT IS READ AT ITS OWN `n`: the sites of each base meet each
/// other under that base's own neighbourhood, so a support holding two
/// bases loses the cells of neither.
pub fn shared_cells(positions: &[crate::transducer::base::Site]) -> Vec<SharedCell> {
    let mut edges = Vec::new();
    let mut bases: Vec<usize> = positions.iter().map(|s| s.n()).collect();
    bases.sort_unstable();
    bases.dedup();
    for n in bases {
        let of_base: Vec<usize> = (0..positions.len()).filter(|&i| positions[i].n() == n).collect();
        for d in neighbourhood(n) {
            let axes: Vec<usize> = d.iter().enumerate().filter(|(_, x)| **x != 0).map(|(i, _)| i).collect();
            let grade = n - axes.len();
            // `t` is the coordinate `d` leads with — its first axis — and the
            // invariants are the coordinates less `d`'s share of `t`, so they
            // are constant along `d`.
            let [lead, ..] = axes.as_slice() else { continue };
            // THE CANONICAL KEY ITSELF, written beside each site's index
            // (rule 4): the line along d, then the parameter on it, is where
            // a neighbour along d is a successor. The key is the order, so
            // the sort is the key's own and chooses nothing.
            // Every site of this base has the axis `d` leads with, `lead < n`.
            let mut along: Vec<(Vec<i64>, i64, usize)> = of_base
                .iter()
                .map(|&i| {
                    let p = positions[i].axes();
                    let t = p[*lead];
                    (p.iter().zip(&d).map(|(x, dx)| x - dx * t).collect(), t, i)
                })
                .collect();
            along.sort_unstable();
            for w in along.windows(2) {
                let [(lp, tp, i), (lq, tq, j)] = w else { continue };
                if lp == lq && tp + 1 == *tq {
                    edges.push(SharedCell { i: *i, j: *j, d: d.clone(), axes: axes.clone(), grade });
                }
            }
        }
    }
    edges
}

/// NB1, 78.2 — THE ELEMENTS OF THE BASE AND WHAT STANDS IN EACH: every cell
/// of every unit cube's closure, named by its centre doubled, with the
/// positions whose cube holds it, ascending, in the elements' own
/// coordinate order.
///
/// A cube at `p` covers `[p, p + 1]ⁿ`; its closure's cells are the doubled
/// centres `2p + o` for `o ∈ {0, 1, 2}ⁿ` — generated from `n`, `3ⁿ` of them
/// — the interior at `o = 1ⁿ`, a face where one axis is `0` or `2`, down to
/// the corners. One read of the base: an element standing in several cubes
/// is ONE element, which the pairwise [`shared_cells`] cannot say, since it
/// names each pair by its own largest shared cell. Three cubes meeting only
/// at a corner stand together here, at that corner.
pub fn elements_standing(positions: &[crate::transducer::base::Site]) -> Vec<(crate::transducer::base::Site, Vec<usize>)> {
    let mut held: crate::network::nerve::Keyed<(crate::transducer::base::Site, Vec<usize>)> =
        crate::network::nerve::Keyed::new();
    for (ix, p) in positions.iter().enumerate() {
        let n = p.n();
        // `o` counts through `{0,1,2}ⁿ` as the digits of `0 .. 3ⁿ`, least
        // axis first: generated, never tabled.
        #[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — the algebra's range: n is the base's n, a site's coordinate count and the n of Cl(n+1,1) (the algebra's u8); 3ⁿ overflows usize at n = 41 and is refused by checked_pow")]
        let Some(count) = 3usize.checked_pow(n as u32) else { continue };
        for code in 0..count {
            let mut rest = code;
            // The doubled centre, exact or refused: a coordinate whose double
            // passes `i64` stands at no element of this base.
            let axes: Option<Vec<i64>> = p
                .axes()
                .iter()
                .map(|&x| {
                    let o = (rest % 3) as i64;
                    rest /= 3;
                    x.checked_mul(2)?.checked_add(o)
                })
                .collect();
            // The element has the position's own arity, which the base held.
            let Some(element) = axes.and_then(crate::transducer::base::Site::new) else { continue };
            let key = element.key();
            match held.get_mut(&key) {
                Some((_, standing)) => {
                    if let Err(at) = standing.binary_search(&ix) {
                        standing.insert(at, ix);
                    }
                }
                None => {
                    held.insert(&key, (element, vec![ix]));
                }
            }
        }
    }
    held.scoped().map(|(_, v)| v.clone()).collect()
}

/// EVERY NEIGHBOUR OF A CUBE OF ℤⁿ, BOTH SENSES, GENERATED (71.2 (2)):
/// `d ∈ {−1,0,1}ⁿ ∖ 0`, `3ⁿ − 1` of them, in lexicographic order with `−1`
/// first, axis 0 the most significant. A contact at every grade, never a
/// list of offsets written for one rank.
pub fn surrounding(n: usize) -> Vec<Vec<i64>> {
    #[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — the algebra's range: n is a base's n, the n of Cl(n+1,1); 3ⁿ is refused past usize by checked_pow")]
    let Some(count) = 3usize.checked_pow(n as u32) else { return Vec::new() };
    let mut out = Vec::with_capacity(count);
    for code in 0..count {
        let mut rest = code;
        let mut d = vec![0i64; n];
        for slot in d.iter_mut().rev() {
            *slot = match rest % 3 {
                0 => -1,
                1 => 0,
                _ => 1,
            };
            rest /= 3;
        }
        if d.iter().any(|x| *x != 0) {
            out.push(d);
        }
    }
    out
}

/// [`surrounding`] ORDERED BY THE GRADE OF CONTACT: the faces (one axis
/// differs, the cell of grade `n − 1`) first, then each lower grade; within
/// a grade by the set of differing axes, then by their senses, `−` before
/// `+`. The faces alone ([`faces_of`]) are the contact that makes a body.
pub fn by_grade(n: usize) -> Vec<Vec<i64>> {
    let mut all = surrounding(n);
    let key = |d: &Vec<i64>| {
        let axes: Vec<usize> = (0..d.len()).filter(|&i| d.get(i).is_some_and(|x| *x != 0)).collect();
        let senses: Vec<i64> = axes.iter().filter_map(|&i| d.get(i).copied()).collect();
        (axes.len(), axes, senses)
    };
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — canonical key order: the grade, the differing axes and the senses are the direction's own coordinates"
    )]
    all.sort_by_key(key);
    all
}

/// DIRECTIONS OF THE ALPHABET'S SECTION as coordinate pairs: each direction
/// of a base of rank 2 (`alphabet::ALPHABET_SAMPLE_RANK`) as its two
/// coordinates, in the order given. A direction of another rank has no pair
/// and is left out, which no direction generated at rank 2 is.
pub fn on_section(directions: Vec<Vec<i64>>) -> Vec<(i64, i64)> {
    directions
        .into_iter()
        .filter_map(|d| match d.as_slice() {
            &[a, b] => Some((a, b)),
            _ => None,
        })
        .collect()
}

/// THE FACE NEIGHBOURS OF A CUBE OF ℤⁿ, both senses, in [`by_grade`]'s
/// order: `2n` of them, the contact of grade `n − 1`.
pub fn faces_of(n: usize) -> Vec<Vec<i64>> {
    by_grade(n).into_iter().filter(|d| d.iter().filter(|x| **x != 0).count() == 1).collect()
}

/// THE NEIGHBOURHOOD OF A CUBE OF ℤⁿ, GENERATED (71.2(2)): every
/// `d ∈ {−1,0,1}ⁿ ∖ 0` with its first non-zero coordinate `+1`, one per pair
/// of opposite neighbours — `(3ⁿ − 1)/2` of them. In canonical order: by how
/// many coordinates differ (`m`, so the faces first), then by which axes (in
/// order), then by the signs of the axes after the first, `+` before `−`.
pub fn neighbourhood(n: usize) -> Vec<Vec<i64>> {
    let mut out = Vec::new();
    for m in 1..=n {
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
