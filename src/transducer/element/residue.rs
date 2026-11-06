// A5, 16.11 — THE RESIDUE: the centred coboundary chain and the words
// built from one.
//
//! A5 centres a chain to its minimum corner and sorts it, so the same
//! deformation at any position is one substance: that canonicalisation, the
//! constructors that read a chain, and the listing key naming it in a frame.

use crate::transducer::germinate::{germ_bytes, ResidueGerm};
use crate::core::Q;
use num_bigint::BigInt;
use num_traits::One;
use crate::transducer::sentence::SectionLink;
use crate::transducer::spelled::SectionSite;
use super::*;

/// The positions skeleton of a centred chain (its base geometry): each
/// link's site of the section, in the chain's order.
pub fn positions(chain: &[SectionLink]) -> Vec<SectionSite> {
    chain.iter().map(|l| l.site).collect()
}

/// The DESCRIPTION of a perceived residue as a germ — post-excision, the ONLY way
/// cognition mints anything: every non-transposition demand IS a coboundary the
/// eye measured, and every "rotation"/"recolour" the mesh will ever speak is a
/// germinated pattern, never a named solver.
///
/// The chain is CENTRED — the changed sites framed with the sites' min-corner at the
/// origin (the substrate's transposition projection, hypha-side per the ruling) —
/// so the same deformation at any position is ONE word, while geometry (the
/// relative offsets between changed sites) is preserved exactly: equal-tally,
/// different-geometry residues stay distinct. The word/identity IS the centred
/// chain; the antisymmetric hue tally is kept only as the δ-algebra lens.
/// A5: translate min-corner to origin, sort, dedup. None when two
/// distinct transitions occupy one site.
/// A5'S CHAIN: centred on its min corner, sorted, and carrying at most
/// one transition per site. [`canonical_chain`] is its only constructor,
/// so a value of this type IS the proof A5 accepted the chain, and every
/// reading that needs one takes it by type rather than re-deriving it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CanonicalChain(Vec<SectionLink>);

impl CanonicalChain {
    /// The links left when the named sites are dropped, re-centred. Total:
    /// a subset of a conflict-free set is conflict-free, and centring and
    /// sorting are total.
    pub fn without(&self, drop: &[SectionSite]) -> CanonicalChain {
        let kept: Vec<SectionLink> = self.0.iter().copied().filter(|l| !drop.contains(&l.site)).collect();
        CanonicalChain(centre(&kept))
    }

    /// Both chains' links. `None` where the two name one site with
    /// different transitions: that is a conflict and A5 refuses it. Equal
    /// transitions at one site are the same link and deduplicate.
    ///
    /// Keeping this one's transition and dropping the other's would be a
    /// member chosen by arrival (rule 15) inventing a consistency the two
    /// chains do not have (rule 11).
    pub fn union(&self, other: &CanonicalChain) -> Option<CanonicalChain> {
        let mut both = self.0.clone();
        both.extend_from_slice(&other.0);
        canonical_chain(&both)
    }

    pub fn as_slice(&self) -> &[SectionLink] {
        &self.0
    }

    pub fn to_vec(&self) -> Vec<SectionLink> {
        self.0.clone()
    }
}

impl std::ops::Deref for CanonicalChain {
    type Target = [SectionLink];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// §2.1 ONE LEVEL DOWN: the canonical object and its carriage. A5's chain
/// is the identity — the same deformation anywhere is one substance — and
/// the translator is where that substance sits in the scene. Two wells
/// holding one chain at two translators are one substance said at two
/// places.
///
/// The corner is measured at the sensor and leaves it as a versor, so the
/// pair never travels (law 11); placements compose by the geometric
/// product where corners would compose by subtraction (3.3).
#[derive(Debug, Clone)]
pub struct PlacedChain {
    chain: CanonicalChain,
    at: Versor,
}

impl Default for PlacedChain {
    /// The empty chain at the origin: no links, and the identity placement
    /// that is true of them rather than standing in for one.
    fn default() -> Self {
        PlacedChain { chain: CanonicalChain::default(), at: identity_versor() }
    }
}

impl PlacedChain {
    pub fn chain(&self) -> &CanonicalChain {
        &self.chain
    }

    pub fn at(&self) -> &Versor {
        &self.at
    }

    /// The chain as it sits in the scene: each canonical link's site
    /// carried through the translator, its transition unchanged, in the
    /// frame the pose names (16.5). `carry_compartment` is the sensor's, and
    /// this is the one place the pair reappears.
    pub fn placed(&self) -> Vec<SectionLink> {
        let mut out = Vec::new();
        for l in self.chain.as_slice() {
            let [r, c] = l.site.coordinates();
            let Some(sites) = crate::transducer::element::sensor::carry_compartment(&self.at, r, c, 0) else {
                continue;
            };
            for (pr, pc, _) in sites {
                out.push(SectionLink::on_plane([pr, pc], l.before, l.after));
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }
}

impl std::ops::Deref for PlacedChain {
    type Target = CanonicalChain;
    fn deref(&self) -> &Self::Target {
        &self.chain
    }
}

/// A5's chain and the translator that places it, measured from raw scene
/// links. `None` where two transitions occupy one site.
pub fn place_chain(links: &[SectionLink]) -> Option<PlacedChain> {
    let chain = canonical_chain(links)?;
    Some(PlacedChain { chain, at: corner_translator(links) })
}

/// The translator from the origin to the chain's min corner. The corner is
/// read here, at the sensor boundary, and leaves as a versor.
fn corner_translator(links: &[SectionLink]) -> Versor {
    let Some([min_r, min_c]) = corner_of(links) else {
        return identity_versor();
    };
    translator_versor(min_r, min_c)
}

/// THE CHAIN'S LEAST CORNER along the section's plane: the least coordinate
/// on each axis of the plane, in the plane's order. `None` for an empty
/// chain.
#[expect(
    clippy::disallowed_methods,
    reason = "rule 4 — the min corner is the chain's extent, content; §1.4, this is the sensor boundary where a site's coordinates are read"
)]
pub fn corner_of(links: &[SectionLink]) -> Option<[i64; crate::transducer::alphabet::ALPHABET_SAMPLE_RANK]> {
    let mut out = [0i64; crate::transducer::alphabet::ALPHABET_SAMPLE_RANK];
    for (axis, slot) in out.iter_mut().enumerate() {
        *slot = links.iter().filter_map(|l| l.site.coordinates().get(axis).copied()).min()?;
    }
    Some(out)
}

/// Translate to the min corner and sort. Total, and conflict-blind: only
/// [`canonical_chain`] decides whether a chain has one transition per site.
/// The min corner is the chain's extent, content: A5 translates it to the
/// origin so two nodes address one chain alike.
fn centre(links: &[SectionLink]) -> Vec<SectionLink> {
    let Some([min_r, min_c]) = corner_of(links) else {
        return Vec::new();
    };
    let mut out: Vec<SectionLink> = links
        .iter()
        .map(|l| {
            let [r, c] = l.site.coordinates();
            SectionLink::new(SectionSite::on_plane([r - min_r, c - min_c]), l.before, l.after)
        })
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

pub fn canonical_chain(links: &[SectionLink]) -> Option<CanonicalChain> {
    let out = centre(links);
    for w in out.windows(2) {
        if w[0].site == w[1].site && w[0].transition() != w[1].transition() {
            return None;
        }
    }
    Some(CanonicalChain(out))
}

/// The word of a residue germ, or `None` where the germ has no canonical
/// chain: two distinct transitions on one site (A5 step 3).
///
/// A germ `canonical_chain` refuses has no address, because the address IS
/// the centred chain (law 2) and there is no centring of a chain that
/// says two things at one site. The refusal travels.
pub fn from_residue(g: &ResidueGerm) -> Option<Element> {
    Some(from_canonical(&canonical_chain(g)?))
}

/// The word of a chain A5 has already accepted. Total: the type is the
/// proof, so there is nothing left to refuse.
pub fn from_canonical(chain: &CanonicalChain) -> Element {
    let mut e = Element::identity();
    e.description = germ_bytes(chain);
    e.address.take();
    e.mag = measure_lanes(chain);
    e.sub = chain.clone();
    e
}

// ── #97: THE MEASURED FRAME (moved from the former `arc` module unchanged,
// ROADMAP track 26, S1) ──────────────────────────────────────────────────
//
// A pair of samples is related by the re-period of each phase, `p′ᵢ/pᵢ`,
// read on its transduction (`family::transduce`): bodies' bands in turns,
// never a common lattice (MU-SENSING §3.1). Equal enclosures are the
// coboundary, and the re-periods ride as the word's frame fibre.

/// THE MEASURED RE-PERIOD of a pair (MU-SENSING §3.1; track 26 S4): the
/// input's and the output's period on each phase, exact counts, never a
/// ratio and never a guess, at the pair's own rank. A word whose phases
/// re-period alike by `ρ ≠ 1` is a DIFFERENT word from its unframed shadow
/// (`from_canonical_framed` reads the re-period off the counts). It replaces
/// the four planar counts `(h_in, w_in, h_out, w_out)`, which fixed two axes
/// (the defect MU-SENSING §8 named).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Reperiod {
    periods: Vec<(usize, usize)>,
}

impl Reperiod {
    /// Identity: no phase re-periodded. Unframed speech uses this so its
    /// key matches an unframed intake.
    pub const IDENTITY: Reperiod = Reperiod { periods: Vec::new() };

    /// THE RE-PERIOD OF A PAIR OF EXTENTS, phase by phase, or `None` where
    /// the two are of different rank: a section is never padded or cut.
    pub fn of_extents(input: &[usize], output: &[usize]) -> Option<Reperiod> {
        (input.len() == output.len()).then(|| Reperiod { periods: input.iter().copied().zip(output.iter().copied()).collect() })
    }

    /// Each phase's period on the input and on the output, in axis order.
    pub fn periods(&self) -> &[(usize, usize)] {
        &self.periods
    }

    /// THE ONE RE-PERIOD EVERY PHASE SHARES, `p′/p`, where every input
    /// period is past 0 and every phase re-periods alike; `None` otherwise
    /// (no phase, an empty input, or phases re-periodded apart: an
    /// anisotropic re-period, which no dilator carries).
    pub fn shared(&self) -> Option<Q> {
        let mut ratios = self.periods.iter().map(|&(p, q)| (p > 0).then(|| Q::new(BigInt::from(q as u64), BigInt::from(p as u64))));
        let first = ratios.next()??;
        ratios.all(|r| r.as_ref() == Some(&first)).then_some(first)
    }
}

/// A residue germ with its FRAME FIBRE: the exact re-period `ρ` every phase
/// shares enters the spatial generator's diagonal, so a word at `ρ ≠ 1` is a
/// different word from its unframed shadow (identity and substance both
/// carry the fibre; only translation is quotiented).
/// The unique listing key of a demand: the address of the framed residue.
/// Intake, the book, quote, attest, probe, settle, task, and the nerve
/// route all use this, never an unframed shadow.
/// `None` where the chain has no canonical form: a listing that cannot be
/// addressed is a refusal that travels, never listed and never counted
/// (U14).
pub fn listing_key(
    chain: &[SectionLink],
    frame: &Reperiod,
) -> Option<crate::transducer::address::Address> {
    Some(from_residue_framed(&chain.to_vec(), frame)?.address())
}

/// Identity frame: no scale. Unframed speech uses this so its key matches
/// a planar intake.
pub const IDENTITY_FRAME: Reperiod = Reperiod::IDENTITY;

pub fn from_residue_framed(g: &ResidueGerm, frame: &Reperiod) -> Option<Element> {
    Some(from_canonical_framed(&canonical_chain(g)?, frame))
}

/// The framed word of a chain A5 has already accepted. Total, as
/// [`from_canonical`] is.
pub fn from_canonical_framed(chain: &CanonicalChain, frame: &Reperiod) -> Element {
    let mut e = from_canonical(chain);
    if let Some(rho) = frame.shared().filter(|r| !r.is_one()) {
        e.action.spatial = Some(crate::transducer::cga::dilator(&rho));
        e.address.take();
    }
    e
}

/// MINT THE CONNECTION: the ORIGIN-FIXING word, the A4 carriage, the gauge
/// map, and the remainder chain. `V = carriage · sensor_action_of(word)`. A
/// word that does not keep the plane is not a connection.
///
/// The word was called `linear`. It is not linear and there is nothing
/// linear in the engine: it is the part of the transport that fixes the
/// origin — the rotor — as against the carriage, which is the translation
/// and the dilation that do not. `None` too where the magnitude it carries
/// would pass its carrier on any lane (STD1): a fresh word holds no section
/// or cycle, so the lane that fits is the one that holds `holonomic_fits`.
pub fn from_connection(
    rotor_word: &crate::transducer::alphabet::Word,
    carriage: &Versor,
    sigma: &[(u8, u8)],
    remainder: &ResidueGerm,
    carried: [u64; 4],
) -> Option<Element> {
    let v = conformal_of(rotor_word, carriage)?;
    let spelling = crate::transducer::alphabet::Spelling { word: rotor_word.clone() };
    let mut e = from_residue(remainder)?;
    e.description = connection_description(&spelling, carriage, sigma, &e.sub.to_vec());
    e.action.spatial = Some(v);
    e.action.gauge = sigma.to_vec();
    e.address.take();
    e.mag = crate::core::lanes::lanes_add(e.mag, carried)?;
    Some(e)
}

/// 78.53 — A COLOUR WORD: an automorphism of the colour fibre and nothing
/// else — a connection that moves no body (the identity rotor, the identity
/// carriage), carries no chain, and whose gauge map is the fibre relation σ
/// (`family::fibre_relation`), in the sensor's labels. Its measure is on
/// 𝒢 alone: the hues it moves. `None` for a relation that moves no hue.
///
/// TRACK 26 S5 — THE RECORDS TRAVEL AS A TYPED SECTION, never as symbol
/// bytes: the description is the connection's (the rows its gauge map), and
/// where the relation records any distinction in e_∅ the record section
/// follows the remainder, `[n][c·n]` (n ≥ 1; `Element::letters` reads it).
/// A relation with no record is written exactly as before.
pub fn colour_word(fibre: &FibreRelation) -> Option<Element> {
    if !is_fibre_relation(fibre) {
        return None;
    }
    let mut e = from_connection(
        &crate::transducer::alphabet::Word::identity(),
        &identity_versor(),
        &fibre.rows,
        &Vec::new(),
        // The hues it moves: a still row moves none, and a record is no row.
        [0, fibre.rows.iter().filter(|&&(f, t)| f != t).count() as u64, 0, 0],
    )?;
    if !fibre.records.is_empty() {
        let mut d = e.description().to_vec();
        crate::transducer::address::put_key_count(&mut d, fibre.records.len());
        d.extend_from_slice(&fibre.records);
        e.set_description(d);
    }
    Some(e)
}

/// 78.53, 78.58 (Closure I) — A FIBRE RELATION: its ROWS on the family's
/// symbols, and its RECORDS in the reservoir ray e_∅. The fibre is the
/// family's symbols extended by e_∅, `E = span{e_σ : σ ∈ Σ} ⊕ ℚe_∅`, and
/// e_∅'s index is `|Σ|` (`symbols::SymbolSet::reservoir`, MU-SENSING
/// §3.3); the sensor's projection Π never sees it, so no scene can paint
/// it, and a word writes to it and nothing else does.
///
/// TRACK 26 S5 — e_∅ IS A TYPED TAG, NEVER A SYMBOL CODE: a record is an
/// entry of `records`, apart from the rows, so every byte is a symbol a row
/// may name (a family whose symbols hold the byte 10 has its colour word).
/// The former record row `(RESERVOIR, c)`, with `RESERVOIR = 10` beside
/// one-byte symbols, is retired; a word's description and the law
/// register's wire (`physics::laws::codec`) carry the records as their own
/// section.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FibreRelation {
    /// The rows: each a MOVE `(f, t)` or a merged target's STILL `(t, t)`,
    /// sorted by the hue they leave.
    pub rows: Vec<(u8, u8)>,
    /// The merged hues whose distinction rotates into e_∅, sorted, each
    /// once.
    pub records: Vec<u8>,
}

impl FibreRelation {
    /// A relation of rows alone, recording nothing in e_∅ (an injection, or
    /// rows a law reads).
    #[cfg(test)]
    pub fn of_rows(rows: &[(u8, u8)]) -> FibreRelation {
        FibreRelation { rows: rows.to_vec(), records: Vec::new() }
    }
}

/// WHAT A COLOUR WORD MAY CARRY (78.53, 78.58): a UNITARY relation of the
/// fibre extended by the reservoir. Three kinds of entry:
///
///   * a MOVE `(f, t)`: a hue the scenes showed going to another, both in
///     the sensor's channels, each hue once;
///   * a STILL `(t, t)`: a merge's target the scenes showed standing still —
///     written only where it merges, since a still hue is otherwise no
///     transition;
///   * a RECORD `c`: the distinction of a merged hue `c`, rotated into e_∅
///     (Closure I: `(c₁ ∧ c₂) ⊗ e_∅ ↦ c₃ ⊗ (Δc ∧ e_∅)`), a typed entry
///     apart from the rows.
///
/// A target is MERGED where two or more hues go to it, itself standing
/// still counted among them. It is unitary when every moved preimage of a
/// merged target is recorded, and nothing else is: no preimage is singled
/// out to stay unrecorded, since any one would be chosen by its label (rule
/// 4). So the word inverts exactly on the extended fibre — each recorded
/// distinction returns to the hue it names, and a target standing still
/// to itself. An injection has no records and no stills and is the 78.53
/// partial injection unchanged. It is NOT completed to a permutation of the
/// family's symbols: a transition no scene showed would be a forward guess
/// (78.45). A row's symbols are labels compared by equality, of any count.
pub fn is_fibre_relation(fibre: &FibreRelation) -> bool {
    let rows = &fibre.rows;
    let recorded = |c: u8| fibre.records.contains(&c);
    let merged = |t: u8| rows.iter().filter(|&&(_, u)| u == t).count() > 1;
    rows.iter().any(|&(f, t)| f != t)
        && rows.iter().all(|&(f, t)| f != t || merged(t))
        && rows.iter().all(|&(f, t)| f == t || recorded(f) == merged(t))
        && fibre.records.iter().all(|&c| rows.iter().any(|&(f, t)| f == c && f != t))
        && rows.windows(2).all(|w| w[0].0 < w[1].0)
        && fibre.records.windows(2).all(|w| w[0] < w[1])
}

/// THE OBSERVED FUNCTION of a colour word, Π∘U on the empty reservoir: each
/// moved hue to its target and each still target to itself — the hues the
/// scenes showed, and no others. Sorted by hue.
pub fn fibre_function(fibre: &FibreRelation) -> Vec<(u8, u8)> {
    fibre.rows.clone()
}

/// 78.58 — THE UNITARY LIFT of a function on the hues the scenes showed
/// (`(hue, hue it became)`, a hue standing still included): the moves; a
/// merged target's still row where it stands still; and every moved
/// preimage of a merged target rotated into the reservoir, as a record.
/// A still hue no other hue merges into is no transition and is not
/// written. `None` when the function moves no hue or is not a function.
/// Every byte is a symbol: the record is a typed entry (track 26 S5).
pub fn lift_fibre(function: &[(u8, u8)]) -> Option<FibreRelation> {
    let mut out = FibreRelation::default();
    for &(f, t) in function {
        if function.iter().any(|&(g, u)| g == f && u != t) {
            return None;
        }
        let merged = function.iter().any(|&(g, u)| u == t && g != f);
        if f != t || merged {
            out.rows.push((f, t));
        }
        if f != t && merged {
            out.records.push(f);
        }
    }
    out.rows.sort_unstable();
    out.rows.dedup();
    out.records.sort_unstable();
    out.records.dedup();
    is_fibre_relation(&out).then_some(out)
}

/// 78.55 — COLOUR WORDS COMPOSE. `after ∘ first` is the composite of the two words'
/// observed functions (`fibre_function`), defined on a hue exactly where
/// `first` says what it becomes and `after` says what that becomes — never
/// on a hue either word says nothing about, so nothing is guessed (78.45)
/// — then lifted unitarily (`lift_fibre`), so a merge the composite makes
/// is carried into the reservoir and not lost. For injections this is the
/// symmetric inverse monoid's product. `None` where the composite moves no
/// hue. Exact, on the sensor's labels.
pub fn compose_fibre(after: &FibreRelation, first: &FibreRelation) -> Option<FibreRelation> {
    let (f, a) = (fibre_function(first), fibre_function(after));
    let out: Vec<(u8, u8)> = f
        .iter()
        .filter_map(|&(h, t)| a.iter().find(|(g, _)| *g == t).map(|&(_, u)| (h, u)))
        .collect();
    lift_fibre(&out)
}

/// 78.55 — A DERIVED COLOUR WORD: the composite of two ruled colour words
/// (`after ∘ first`), carrying HOW it was reached as a citation in its
/// receipts envelope — the two words by the words' coordinates, in application
/// order. The envelope is outside the address: the word is its relation,
/// and a derivation is a presentation of it (§1.5), so a word derived is
/// the same word as the one a scene witnessed. `None` where either is not a
/// colour word, the composite is not a colour relation, or an address does
/// not fit its citation.
pub fn derived_colour_word(first: &Element, after: &Element) -> Option<Element> {
    let (f, a) = (first.letters()?, after.letters()?);
    if !(first.is_colour_word() && after.is_colour_word()) {
        return None;
    }
    let fibre = compose_fibre(&a.fibre(), &f.fibre())?;
    let mut word = colour_word(&fibre)?;
    let derivation = super::DerivationCite {
        vertex: word.address(),
        // Each generator is cited as (form, vertex, exponent): a colour word
        // is its own form and vertex, applied once.
        generators: vec![(first.address(), first.address(), 1), (after.address(), after.address(), 1)],
        residue: crate::transducer::alphabet::Word::identity(),
        conjugator: None,
    };
    word.set_receipts(super::ReceiptsEnvelope { derivations: vec![derivation], ..Default::default() });
    Some(word)
}

impl Element {
    /// The two ruled words a derived colour word cites (`first`, `after`),
    /// read off its receipts envelope; `None` for a word that cites none.
    pub fn colour_citation(&self) -> Option<(crate::transducer::address::Address, crate::transducer::address::Address)> {
        let own = self.address();
        let d = self.receipts().derivations.iter().find(|d| d.vertex == own)?;
        match d.generators.as_slice() {
            [(first, _, 1), (after, _, 1)] => Some((first.clone(), after.clone())),
            _ => None,
        }
    }
}

/// The inverse of a colour relation on its image, read on the EMPTY
/// reservoir: each unmerged move sent back. A recorded hue returns only
/// with its record, which no scene carries, so its row is not among them;
/// a still target returns to itself and moves nothing.
#[cfg(test)]
pub fn invert_fibre(fibre: &FibreRelation) -> Vec<(u8, u8)> {
    let recorded = |c: u8| fibre.records.contains(&c);
    let mut out: Vec<(u8, u8)> = fibre
        .rows
        .iter()
        .filter(|&&(f, t)| f != t && !recorded(f))
        .map(|&(f, t)| (t, f))
        .collect();
    out.sort_unstable();
    out
}

/// 78.58, MU-SENSING §3.3 — A FIBRE RELATION READ ON THE FAMILY'S OWN
/// FIBRE `E = span{e_σ : σ ∈ Σ} ⊕ ℚe_∅`: each row's symbols at their
/// indices into Σ, and each record as the row from the reservoir's index
/// `|Σ|` to the recorded symbol's index. The tape's labels become the
/// family's addressing order, so two families that differ by a relabelling
/// of their symbols read one relation here. `None` where an entry names a
/// symbol the family never showed.
#[cfg(test)]
pub fn fibre_on_symbols(fibre: &FibreRelation, symbols: &crate::transducer::symbols::SymbolSet) -> Option<Vec<(usize, usize)>> {
    let at = |h: u8| symbols.index(std::slice::from_ref(&h));
    let rows = fibre.rows.iter().map(|&(f, t)| Some((at(f)?, at(t)?)));
    let records = fibre.records.iter().map(|&c| Some((symbols.reservoir(), at(c)?)));
    rows.chain(records).collect()
}

impl Element {
    /// Whether this word is a colour word (`colour_word`): a connection
    /// whose only content is a fibre relation — no motion, no chain, a
    /// gauge map that moves a hue. Read off the word itself.
    pub fn is_colour_word(&self) -> bool {
        let Some(l) = self.letters() else { return false };
        self.sub().is_empty()
            && l.spelling.word.is_identity()
            && l.carriage.projective_eq(&identity_versor())
            && l.sigma.iter().any(|&(f, t)| f != t)
    }
}

/// The description of a connection: its spelling in the alphabet at its
/// dilation, the placement, the gauge map, and the centred remainder. The
/// doc said "description" and the name said `word`; the doc was right.
pub(crate) fn connection_description(
    spelling: &crate::transducer::alphabet::Spelling,
    carriage: &Versor,
    sigma: &[(u8, u8)],
    remainder: &ResidueGerm,
) -> Vec<u8> {
    let mut out = vec![CONNECTION_TAG];
    spelling.to_bytes(&mut out);
    carriage.to_bytes(&mut out);
    // Each count by its own coordinate (STD1 3c-2, ruled (d)): the
    // description is keyed whole into the word's address.
    crate::transducer::address::put_key_count(&mut out, sigma.len());
    for &(f, t) in sigma {
        out.push(f);
        out.push(t);
    }
    crate::transducer::address::put_key_bytes(&mut out, &germ_bytes(remainder));
    out
}

/// THE DEMAND A MIND SPEAKS (A12 step 5, M6 step 5): the exact element it
/// lacks, spoken as a connection with no chain. On the plane this is the
/// canonical HNF residue of the incoming word against the span of what the
/// mind holds — unique for that lattice, so two minds holding the same
/// words speak the same demand, with no choice in it; off the plane there
/// is no HNF and no metric to say which partial is nearer, so the whole
/// word is the demand.
///
/// Its μ⃗ is read off the content by `reduction_magnitude` — what the
/// connection carries — and never handed in as a constant. "No mind emits
/// `unknown`; it emits the element it lacks."
pub fn from_demand(rotor_word: &crate::transducer::alphabet::Word) -> Option<Element> {
    let mut e = from_connection(rotor_word, &identity_versor(), &[], &Vec::new(), [0; 4])?;
    e.mag = e.reduction_magnitude();
    e.address.take();
    Some(e)
}

/// A gauge between two sections, spelled as a connection with identity
/// gauge map and tagged so the court reconstructs it as a translation.
pub fn from_translation(
    rotor: &crate::transducer::alphabet::Rotor,
    a: impl AsRef<[u8]>,
    b: impl AsRef<[u8]>,
) -> Option<Element> {
    let word = rotor.spelled_word()?;
    let mut e = from_connection(&word, &identity_versor(), &[], &Vec::new(), [1, 0, 0, 0])?;
    let mut d = e.description().to_vec();
    if crate::transducer::address::tag_of(&d) == Some(CONNECTION_TAG) {
        d[0] = TRANSLATION_TAG;
    }
    crate::transducer::address::put_address(&mut d, a)?;
    crate::transducer::address::put_address(&mut d, b)?;
    e.set_description(d);
    Some(e)
}
