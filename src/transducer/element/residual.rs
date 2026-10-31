// A5, 16.11 — THE RESIDUAL: the centred coboundary chain and the words
// built from one.
//
//! A5 centres a chain to its minimum corner and sorts it, so the same
//! deformation at any position is one substance: that canonicalisation, the
//! constructors that read a chain, and the listing key naming it in a frame.

use crate::transducer::germinate::{germ_bytes, ResidualGerm};
use crate::core::Q;
use num_bigint::BigInt;
use num_traits::One;
use super::*;

/// The positions skeleton of a centred chain (its base geometry).
pub fn positions(chain: &[(i64, i64, u8, u8)]) -> Vec<(i64, i64)> {
    chain.iter().map(|(r, c, _, _)| (*r, *c)).collect()
}

/// The DESCRIPTION of a perceived residual as a germ — post-excision, the ONLY way
/// cognition mints anything: every non-transposition demand IS a coboundary the
/// eye measured, and every "rotation"/"recolour" the mesh will ever speak is a
/// germinated pattern, never a named solver.
///
/// The chain is CENTRED — the changed sites framed with the sites' min-corner at the
/// origin (the substrate's transposition projection, hypha-side per the ruling) —
/// so the same deformation at any position is ONE word, while geometry (the
/// relative offsets between changed sites) is preserved exactly: equal-tally,
/// different-geometry residuals stay distinct. The word/identity IS the centred
/// chain; the antisymmetric hue tally is kept only as the δ-algebra lens.
/// A5: translate min-corner to origin, sort, dedup. None when two
/// distinct transitions occupy one site.
/// A5'S CHAIN: centred on its min corner, sorted, and carrying at most
/// one transition per site. [`canonical_chain`] is its only constructor,
/// so a value of this type IS the proof A5 accepted the chain, and every
/// reading that needs one takes it by type rather than re-deriving it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CanonicalChain(Vec<(i64, i64, u8, u8)>);

impl CanonicalChain {
    /// The links left when the named sites are dropped, re-centred. Total:
    /// a subset of a conflict-free set is conflict-free, and centring and
    /// sorting are total.
    pub fn without(&self, drop: &[(i64, i64)]) -> CanonicalChain {
        let kept: Vec<(i64, i64, u8, u8)> = self
            .0
            .iter()
            .copied()
            .filter(|(r, c, _, _)| !drop.contains(&(*r, *c)))
            .collect();
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

    pub fn as_slice(&self) -> &[(i64, i64, u8, u8)] {
        &self.0
    }

    pub fn to_vec(&self) -> Vec<(i64, i64, u8, u8)> {
        self.0.clone()
    }
}

impl std::ops::Deref for CanonicalChain {
    type Target = [(i64, i64, u8, u8)];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// §2.1 ONE LEVEL DOWN: the canonical object and its carriage. A5's chain
/// is the identity — the same deformation anywhere is one substance — and
/// the translator is where that substance sits in the scene. Two bubbles
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
    pub fn placed(&self) -> Vec<(i64, i64, u8, u8)> {
        let mut out = Vec::new();
        for &(r, c, hb, ha) in self.chain.as_slice() {
            let Some(sites) = crate::transducer::element::sensor::carry_compartment(&self.at, r, c, 0) else {
                continue;
            };
            for (pr, pc, _) in sites {
                out.push((pr, pc, hb, ha));
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
pub fn place_chain(links: &[(i64, i64, u8, u8)]) -> Option<PlacedChain> {
    let chain = canonical_chain(links)?;
    Some(PlacedChain { chain, at: corner_translator(links) })
}

/// The translator from the origin to the chain's min corner. The corner is
/// read here, at the sensor boundary, and leaves as a versor.
#[expect(
    clippy::disallowed_methods,
    reason = "rule 4 — the min corner is the chain's extent, content; §1.4, this is the sensor boundary where a site's coordinates are read"
)]
fn corner_translator(links: &[(i64, i64, u8, u8)]) -> Versor {
    let (Some(min_r), Some(min_c)) = (
        links.iter().map(|(r, _, _, _)| *r).min(),
        links.iter().map(|(_, c, _, _)| *c).min(),
    ) else {
        return identity_versor();
    };
    translator_versor(min_r, min_c)
}

/// Translate to the min corner and sort. Total, and conflict-blind: only
/// [`canonical_chain`] decides whether a chain has one transition per site.
#[expect(
    clippy::disallowed_methods,
    reason = "rule 4 — the min corner is the chain's extent, content: A5 translates it to the origin so two nodes address one chain alike"
)]
fn centre(links: &[(i64, i64, u8, u8)]) -> Vec<(i64, i64, u8, u8)> {
    let (Some(min_r), Some(min_c)) = (
        links.iter().map(|(r, _, _, _)| *r).min(),
        links.iter().map(|(_, c, _, _)| *c).min(),
    ) else {
        return Vec::new();
    };
    let mut out: Vec<(i64, i64, u8, u8)> = links
        .iter()
        .map(|(r, c, b, a)| (r - min_r, c - min_c, *b, *a))
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

pub fn canonical_chain(links: &[(i64, i64, u8, u8)]) -> Option<CanonicalChain> {
    let out = centre(links);
    for w in out.windows(2) {
        if w[0].0 == w[1].0 && w[0].1 == w[1].1 && (w[0].2, w[0].3) != (w[1].2, w[1].3) {
            return None;
        }
    }
    Some(CanonicalChain(out))
}

/// The word of a residual germ, or `None` where the germ has no canonical
/// chain: two distinct transitions on one site (A5 step 3).
///
/// A germ `canonical_chain` refuses has no address, because the address IS
/// the centred chain (law 2) and there is no centring of a chain that
/// says two things at one site. The refusal travels.
pub fn from_residual(g: &ResidualGerm) -> Option<Element> {
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

/// A residual germ with its FRAME FIBRE: the exact frame map
/// ρ = (h_out/h_in, w_out/w_in) ∈ ℚ² enters the spatial generator's
/// diagonal, so a word at ρ ≠ (1,1) is a different word from its planar
/// shadow (identity and substance both carry the fibre; only translation is
/// quotiented). `frame` is (h_in, w_in, h_out, w_out).
/// The unique listing key of a demand: the address of the framed residual.
/// Intake, the book, quote, attest, probe, settle, task, and the nerve
/// route all use this, never an unframed shadow.
/// `None` where the chain has no canonical form: a listing that cannot be
/// addressed is a refusal that travels, never listed and never counted
/// (U14).
pub fn listing_key(
    chain: &[(i64, i64, u8, u8)],
    frame: &crate::arc::Frame,
) -> Option<crate::transducer::address::Address> {
    Some(from_residual_framed(&chain.to_vec(), frame)?.address())
}

/// Identity frame: no scale. Unframed speech uses this so its key matches
/// a planar intake.
pub const IDENTITY_FRAME: crate::arc::Frame = crate::arc::Frame::IDENTITY;

pub fn from_residual_framed(g: &ResidualGerm, frame: &crate::arc::Frame) -> Option<Element> {
    Some(from_canonical_framed(&canonical_chain(g)?, frame))
}

/// The framed word of a chain A5 has already accepted. Total, as
/// [`from_canonical`] is.
pub fn from_canonical_framed(chain: &CanonicalChain, frame: &crate::arc::Frame) -> Element {
    let mut e = from_canonical(chain);
    let crate::arc::Frame { h_in: hi, w_in: wi, h_out: ho, w_out: wo } = *frame;
    if hi > 0 && wi > 0 && (hi, wi) != (ho, wo) {
        let rh = Q::new(BigInt::from(ho as u64), BigInt::from(hi as u64));
        let rw = Q::new(BigInt::from(wo as u64), BigInt::from(wi as u64));
        if rh == rw && !rh.is_one() {
            e.action.spatial = Some(crate::transducer::cga::dilator(&rh));
            e.address.take();
        }
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
    remainder: &ResidualGerm,
    carried: [u64; 4],
) -> Option<Element> {
    let v = conformal_of(rotor_word, carriage)?;
    let spelling = crate::transducer::alphabet::Spelling { word: rotor_word.clone() };
    let mut e = from_residual(remainder)?;
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
pub fn colour_word(sigma: &[(u8, u8)]) -> Option<Element> {
    if !is_fibre_relation(sigma) {
        return None;
    }
    from_connection(
        &crate::transducer::alphabet::Word::identity(),
        &identity_versor(),
        sigma,
        &Vec::new(),
        // The hues it moves: a still row and a record move none.
        [0, sigma.iter().filter(|&&(f, t)| f != t && f != RESERVOIR).count() as u64, 0, 0],
    )
}

/// 78.58 (Closure I) — THE RESERVOIR: the ancilla ray e_∅ of the colour
/// fibre, the one coordinate outside the detector's channels
/// (E = span{e_0..e_9} ⊕ ℚe_∅). The sensor's projection Π never sees it,
/// so no scene can paint it; a word writes to it and nothing else does.
#[expect(
    clippy::cast_possible_truncation,
    reason = "STD1 C1 — the sensor's: PALETTE = 10 channels, and the reservoir is the coordinate after the last"
)]
pub const RESERVOIR: u8 = PALETTE as u8;

/// WHAT A COLOUR WORD MAY CARRY (78.53, 78.58): a UNITARY relation of the
/// fibre extended by the reservoir. Three kinds of row:
///
///   * a MOVE `(f, t)`: a hue the scenes showed going to another, both in
///     the sensor's channels, each hue once;
///   * a STILL `(t, t)`: a merge's target the scenes showed standing still —
///     written only where it merges, since a still hue is otherwise no
///     transition;
///   * a RECORD `(RESERVOIR, c)`: the distinction of a merged hue `c`,
///     rotated into e_∅ (Closure I: `(c₁ ∧ c₂) ⊗ e_∅ ↦ c₃ ⊗ (Δc ∧ e_∅)`).
///
/// A target is MERGED where two or more hues go to it, itself standing
/// still counted among them. It is unitary when every moved preimage of a
/// merged target is recorded, and nothing else is: no preimage is singled
/// out to stay unrecorded, since any one would be chosen by its label (rule
/// 4). So the word inverts exactly on the extended fibre — each recorded
/// distinction returns to the hue it names, and a target standing still
/// to itself. An injection has no records and no stills and is the 78.53
/// partial injection unchanged. It is NOT completed to a permutation of the
/// palette: a transition no scene showed would be a forward guess (78.45).
pub fn is_fibre_relation(sigma: &[(u8, u8)]) -> bool {
    let within = |h: u8| usize::from(h) < PALETTE;
    let rows: Vec<(u8, u8)> = sigma.iter().copied().filter(|&(f, _)| f != RESERVOIR).collect();
    let recorded = |c: u8| sigma.contains(&(RESERVOIR, c));
    let merged = |t: u8| rows.iter().filter(|&&(_, u)| u == t).count() > 1;
    rows.iter().any(|&(f, t)| f != t)
        && rows.iter().all(|&(f, t)| within(f) && within(t) && (f != t || merged(t)))
        && rows.iter().all(|&(f, t)| f == t || recorded(f) == merged(t))
        && sigma.iter().filter(|&&(f, _)| f == RESERVOIR).all(|&(_, c)| rows.iter().any(|&(f, t)| f == c && f != t))
        && sigma.windows(2).all(|w| w[0].0 < w[1].0 || (w[0].0 == RESERVOIR && w[0] < w[1]))
}

/// THE OBSERVED FUNCTION of a colour word, Π∘U on the empty reservoir: each
/// moved hue to its target and each still target to itself — the hues the
/// scenes showed, and no others. Sorted by hue.
pub fn fibre_function(sigma: &[(u8, u8)]) -> Vec<(u8, u8)> {
    sigma.iter().copied().filter(|&(f, _)| f != RESERVOIR).collect()
}

/// 78.58 — THE UNITARY LIFT of a function on the hues the scenes showed
/// (`(hue, hue it became)`, a hue standing still included): the moves; a
/// merged target's still row where it stands still; and every moved
/// preimage of a merged target rotated into the reservoir. A still hue no
/// other hue merges into is no transition and is not written. `None` when
/// the function moves no hue, or is not a function.
pub fn lift_fibre(function: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let mut out: Vec<(u8, u8)> = Vec::new();
    for &(f, t) in function {
        if function.iter().any(|&(g, u)| g == f && u != t) {
            return None;
        }
        let merged = function.iter().any(|&(g, u)| u == t && g != f);
        if f != t || merged {
            out.push((f, t));
        }
        if f != t && merged {
            out.push((RESERVOIR, f));
        }
    }
    out.sort_unstable();
    out.dedup();
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
pub fn compose_fibre(after: &[(u8, u8)], first: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
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
    let sigma = compose_fibre(&a.sigma, &f.sigma)?;
    let mut word = colour_word(&sigma)?;
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
pub fn invert_fibre(sigma: &[(u8, u8)]) -> Vec<(u8, u8)> {
    let recorded = |c: u8| sigma.contains(&(RESERVOIR, c));
    let mut out: Vec<(u8, u8)> = sigma
        .iter()
        .filter(|&&(f, t)| f != RESERVOIR && f != t && !recorded(f))
        .map(|&(f, t)| (t, f))
        .collect();
    out.sort_unstable();
    out
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
            && l.sigma.iter().any(|&(f, t)| f != t && f != RESERVOIR)
    }
}

/// The description of a connection: its spelling in the alphabet at its
/// dilation, the placement, the gauge map, and the centred remainder. The
/// doc said "description" and the name said `word`; the doc was right.
pub(crate) fn connection_description(
    spelling: &crate::transducer::alphabet::Spelling,
    carriage: &Versor,
    sigma: &[(u8, u8)],
    remainder: &ResidualGerm,
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
/// gauge map and tagged so the court replays it as a translation.
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
