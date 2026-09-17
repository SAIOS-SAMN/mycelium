// A5, 16.11 — THE RESIDUAL: the centered coboundary chain and the words
// built from one.
//
//! A5 centers a chain to its minimum corner and sorts it, so the same
//! deformation at any position is one substance: that canonicalisation, the
//! constructors that read a chain, and the listing key naming it in a frame.

// `germ_bytes as residual_word` was the alias here. It renamed a correctly
// named function — bytes are bytes — to the document's noun for a Cayley-graph
// path, at the import. The alias is dropped rather than renamed: the right
// name already existed upstream.
use crate::transducer::germinate::{germ_bytes, ResidualGerm};
use crate::core::Q;
use num_bigint::BigInt;
use num_traits::One;
use super::*;

/// The positions skeleton of a centered chain (its base geometry).
pub fn positions(chain: &[(i64, i64, u8, u8)]) -> Vec<(i64, i64)> {
    chain.iter().map(|(r, c, _, _)| (*r, *c)).collect()
}

/// The DESCRIPTION of a perceived residual as a germ — post-excision, the ONLY way
/// cognition mints anything: every non-transposition demand IS a coboundary the
/// eye measured, and every "rotation"/"recolour" the mesh will ever speak is a
/// germinated pattern, never a named solver.
///
/// The chain is CENTERED — the changed-cell set framed with its min-corner at the
/// origin (the substrate's transposition projection, hypha-side per the ruling) —
/// so the same deformation at any position is ONE word, while geometry (the
/// relative offsets between changed cells) is preserved exactly: equal-tally,
/// different-geometry residuals stay distinct. The word/identity IS the centered
/// chain; the antisymmetric hue tally is kept only as the δ-algebra lens.
/// A5: translate min-corner to origin, sort, dedup. None when two
/// distinct transitions occupy one site.
/// A5'S CHAIN: centered on its min corner, sorted, and carrying at most
/// one transition per site. [`canonical_chain`] is its only constructor,
/// so a value of this type IS the proof A5 accepted the chain, and every
/// reading that needs one takes it by type rather than re-deriving it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CanonicalChain(Vec<(i64, i64, u8, u8)>);

impl CanonicalChain {
    /// The cells this chain dropped, re-centered. Total: a subset of a
    /// conflict-free set is conflict-free, and centering and sorting are
    /// total.
    pub fn without(&self, drop: &[(i64, i64)]) -> CanonicalChain {
        let kept: Vec<(i64, i64, u8, u8)> = self
            .0
            .iter()
            .copied()
            .filter(|(r, c, _, _)| !drop.contains(&(*r, *c)))
            .collect();
        CanonicalChain(center(&kept))
    }

    /// Both chains' cells. `None` where the two name one site with
    /// different transitions: that is a conflict and A5 refuses it. Equal
    /// transitions at one site are the same cell and deduplicate.
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
    /// The empty chain at the origin: no cells, and the identity placement
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

    /// The chain as it sits in the scene: each canonical cell carried
    /// through the translator, its transition unchanged, in the frame the
    /// pose names (16.5). `carry_cell` is the sensor's, and this is the one
    /// place the pair reappears.
    pub fn placed(&self) -> Vec<(i64, i64, u8, u8)> {
        let mut out = Vec::new();
        for &(r, c, hb, ha) in self.chain.as_slice() {
            let Some(cells) = crate::transducer::element::sensor::carry_cell(&self.at, r, c, 0) else {
                continue;
            };
            for (pr, pc, _) in cells {
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
/// cells. `None` where two transitions occupy one site.
pub fn place_chain(cells: &[(i64, i64, u8, u8)]) -> Option<PlacedChain> {
    let chain = canonical_chain(cells)?;
    Some(PlacedChain { chain, at: corner_translator(cells) })
}

/// The translator from the origin to the chain's min corner. The corner is
/// read here, at the sensor boundary, and leaves as a versor.
#[expect(
    clippy::disallowed_methods,
    reason = "rule 4 — the min corner is the chain's extent, content; §1.4, this is the sensor boundary where a cell coordinate is read"
)]
fn corner_translator(cells: &[(i64, i64, u8, u8)]) -> Versor {
    let (Some(min_r), Some(min_c)) = (
        cells.iter().map(|(r, _, _, _)| *r).min(),
        cells.iter().map(|(_, c, _, _)| *c).min(),
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
fn center(cells: &[(i64, i64, u8, u8)]) -> Vec<(i64, i64, u8, u8)> {
    let (Some(min_r), Some(min_c)) = (
        cells.iter().map(|(r, _, _, _)| *r).min(),
        cells.iter().map(|(_, c, _, _)| *c).min(),
    ) else {
        return Vec::new();
    };
    let mut out: Vec<(i64, i64, u8, u8)> = cells
        .iter()
        .map(|(r, c, b, a)| (r - min_r, c - min_c, *b, *a))
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

pub fn canonical_chain(cells: &[(i64, i64, u8, u8)]) -> Option<CanonicalChain> {
    let out = center(cells);
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
/// the centered chain (law 2) and there is no centering of a chain that
/// says two things at one cell. The refusal travels.
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

/// A residual germ with its FRAME FIBER: the exact frame map
/// ρ = (h_out/h_in, w_out/w_in) ∈ ℚ² enters the spatial generator's
/// diagonal, so a word at ρ ≠ (1,1) is a different word from its planar
/// shadow (identity and substance both carry the fiber; only translation is
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
/// and the dilation that do not.
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
    for (g, c) in e.mag.iter_mut().zip(carried) {
        *g = g.saturating_add(c);
    }
    Some(e)
}

/// The description of a connection: its spelling in the alphabet at its
/// dilation, the placement, the gauge map, and the centered remainder. The
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
    out.extend_from_slice(&(sigma.len() as u16).to_le_bytes());
    for &(f, t) in sigma {
        out.push(f);
        out.push(t);
    }
    let germ = germ_bytes(remainder);
    out.extend_from_slice(&(germ.len() as u32).to_le_bytes());
    out.extend_from_slice(&germ);
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
    crate::transducer::address::put_addr(&mut d, a)?;
    crate::transducer::address::put_addr(&mut d, b)?;
    e.set_description(d);
    Some(e)
}
