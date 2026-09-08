// The shapes a client reads and answers with: a verdict, a comprehension, a
// dialect, a working atlas, a genome view, an utterance, a quote.

use super::*;

/// The judge's answer to a submitted demand, plus any taught ANASTOME element the
/// kernel learns (the mesh's word for what it just described).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// The demand DECOMPOSED into a canonical sentence of installed generators (the
    /// confluent normal form) — the mesh already speaks it, nothing minted.
    Decomposed { taught: Vec<u8> },
    /// A novel generator awaiting a second kernel's voice (consilience pending).
    GermPending,
    /// An irreducible residual that reached consilience and adjoined (KB completion).
    Installed { taught: Vec<u8> },
    /// A section of a held word. Not an install.
    Sectioned,
    /// The scene fragmented into per-body sub-demands.
    Fragmented,
    /// A degenerate demand — nothing to reduce.
    Pivot,
    /// An unrecognized reply code (forward-compat).
    Unknown(u8),
}

impl Verdict {
    pub fn label(&self) -> &'static str {
        match self {
            Verdict::Decomposed { .. } => "decomposed",
            Verdict::GermPending => "germ-pending",
            Verdict::Installed { .. } => "installed",
            Verdict::Sectioned => "section",
            Verdict::Fragmented => "fragmented",
            Verdict::Pivot => "pivot",
            Verdict::Unknown(_) => "unknown",
        }
    }
    /// The taught ANASTOME element the mesh handed back (empty when none).
    pub fn taught(&self) -> &[u8] {
        match self {
            Verdict::Decomposed { taught } | Verdict::Installed { taught } => taught,
            _ => &[],
        }
    }
}

/// The comprehension verdict — the C-STACK's terminal (congruence → coherence →
/// cohesion → converse → COMPREHENSION): the received utterance DECOMPOSED over
/// the local genome toward understanding. Never re-cognition (that is the
/// perception stack's overlay), never recall.
#[derive(Clone, Debug)]
pub enum Comprehension {
    /// The utterance decomposed into held words — address and standing
    /// from the working atlas. Never a sentinel index.
    Understood(Vec<(Vec<u8>, Standing)>),
    /// Partially decomposed: the residual is the not-yet-understood part (a
    /// prospective for the mesh, not a failure of the ear).
    Novel { residual_cells: usize },
    /// A CONNECTION heard by holding it: the transition to the held word on
    /// its fibre vanished — the same word, glued.
    Held,
    /// A connection heard whose transition to the fibre's representative
    /// did not vanish: the obstruction element is the dialect, carried.
    Dialect(Box<Element>),
    /// A connection on a fibre this genome holds no representative on:
    /// admitted as new geometry.
    Admitted,
    /// A sentence cites forms this mind does not hold.
    Withheld { missing: Vec<Vec<u8>> },
}

impl PartialEq for Comprehension {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Comprehension::Understood(a), Comprehension::Understood(b)) => a == b,
            (Comprehension::Novel { residual_cells: a }, Comprehension::Novel { residual_cells: b }) => a == b,
            (Comprehension::Held, Comprehension::Held) | (Comprehension::Admitted, Comprehension::Admitted) => true,
            (Comprehension::Dialect(a), Comprehension::Dialect(b)) => a.same_vertex(b),
            (Comprehension::Withheld { missing: a }, Comprehension::Withheld { missing: b }) => a == b,
            _ => false,
        }
    }
}
impl Eq for Comprehension {}

/// A sentence this mind spoke that the hypha sectioned or settled.
/// Memory only; never serialized.
#[derive(Debug, Clone)]
pub struct Dialect {
    pub sentence: Element,
    pub last_used_k: u64,
    /// TUniversal of last working-atlas read or stand.
    pub last_used_tu: u64,
    pub uses: u64,
    pub(crate) shape: Option<crate::transducer::blade::Shape>,
    /// A withheld sentence: the cited addresses this mind does not hold.
    pub withheld: bool,
    pub missing: Vec<Vec<u8>>,
}

/// Standing of a fiber member on the working atlas.
pub use crate::transducer::stalk::Standing;

/// One working atlas, three strata. A fiber read is `(address, Standing)`,
/// never a boolean. Genome-only `atlas()` remains for persistence.
pub struct WorkingAtlas<'a> {
    pub(crate) genome: &'a crate::transducer::stalk::Stalks,
    pub(crate) dialect: &'a crate::transducer::stalk::Stalks,
    pub(crate) ideas: Option<&'a crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>>,
}

impl<'a> WorkingAtlas<'a> {
    /// Addresses on this chart, each carrying the stratum it came from.
    /// Ruled first, then Stood, then Held; an address appears once, at its
    /// strongest standing.
    pub fn fiber(&self, inv: &crate::transducer::stalk::Invariant) -> Vec<(Vec<u8>, Standing)> {
        let mut out: Vec<(Vec<u8>, Standing)> = Vec::new();
        let mut have: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for a in self.genome.fiber(inv) {
            have.insert(&a, ());
            out.push((a, Standing::Ruled));
        }
        for a in self.dialect.fiber(inv) {
            if have.contains(&a) {
                continue;
            }
            have.insert(&a, ());
            out.push((a, Standing::Stood));
        }
        if let Some(ideas) = self.ideas {
            if let Some(m) = ideas.get(inv.key().as_bytes()) {
                for (a, _) in m.fiber(crate::network::nerve::ROOT) {
                    if have.contains(&a) {
                        continue;
                    }
                    have.insert(&a, ());
                    out.push((a, Standing::Held));
                }
            }
        }
        out
    }

    pub fn standing_of(&self, addr: &[u8], inv: &crate::transducer::stalk::Invariant) -> Option<Standing> {
        if self.genome.chart_of(addr) == Some(inv) {
            return Some(Standing::Ruled);
        }
        if self.dialect.chart_of(addr) == Some(inv) {
            return Some(Standing::Stood);
        }
        if self
            .ideas
            .and_then(|m| m.get(inv.key().as_bytes()))
            .is_some_and(|inner| inner.contains(addr))
        {
            return Some(Standing::Held);
        }
        None
    }

    pub fn standing_addr(&self, addr: &[u8]) -> Option<Standing> {
        if self.genome.contains(addr) {
            return Some(Standing::Ruled);
        }
        if self.dialect.contains(addr) {
            return Some(Standing::Stood);
        }
        if let Some(ideas) = self.ideas {
            if ideas.fiber(crate::network::nerve::ROOT).any(|(_, m)| m.contains(addr)) {
                return Some(Standing::Held);
            }
        }
        None
    }
}

/// The crystallized language: words on disk. The epigenome is the rest.
pub struct GenomeView<'a> {
    pub words: &'a [Element],
}

#[derive(Default)]
pub struct Epigenome {
    pub(crate) genome: Vec<Element>,
    /// Expression weighting per genome word — how often this entity speaks/hears
    /// it (the recollection efficiency; parallel to `genome`).
    pub(crate) expression: Vec<u64>,
    /// Locally comprehended sentences, keyed by the sentence's word addresses
    /// (not by basis indices that shift on retire).
    pub(crate) recollections: crate::network::nerve::Keyed<(Vec<Vec<u8>>, u64)>,
    /// THE ATLAS of the genome — admitted in place as words are learned
    /// or restored; rebuilt only on retire. Never persisted.
    pub(crate) atlas: crate::transducer::stalk::Stalks,
    pub(crate) atlas_len: usize,
    /// Each word's shape, lifted once on first use (parallel to the genome).
    pub(crate) shapes: Vec<std::sync::OnceLock<Option<crate::transducer::blade::Shape>>>,
    /// Taught words refused because they did not verify (a transition
    /// that is not a versor, a section that does not span the support) —
    /// the entity's measured discordance with what it was taught.
    pub(crate) refused: u64,
    /// THE EXPRESSED WORDS (ephemeral, never persisted): the residual
    /// bodies this entity germinated itself, before any court ratified
    /// them. They let the entity recognise a residual it has already
    /// claimed, so a claim is spoken once; a ratified word arrives by
    /// teaching and enters the genome.
    pub(crate) expressed: Vec<Element>,
    pub(crate) expressed_atlas: crate::transducer::stalk::Stalks,
    pub(crate) expressed_shapes: Vec<std::sync::OnceLock<Option<crate::transducer::blade::Shape>>>,
    pub(crate) expressed_index: crate::network::nerve::Keyed<usize>,
    /// Every genome word reached by its address (index-aligned with
    /// `genome`): a keyed relation, never a walk of the genome.
    pub(crate) index: crate::network::nerve::Keyed<usize>,
    /// Sentences this mind spoke that stood. Memory only.
    pub(crate) dialect: crate::network::nerve::Keyed<Dialect>,
    /// Charts of stood sentences, admitted in stand_dialect_at, retracted in shed.
    pub(crate) dialect_atlas: crate::transducer::stalk::Stalks,
    /// Addresses promoted from dialect into the genome this learn.
    pub(crate) promoted: Vec<Vec<u8>>,
    /// Dialects dropped this process.
    pub(crate) shed_n: u64,
    /// TUniversal at each of the last aperture_cycle strokes.
    pub(crate) stroke_k: Vec<u64>,
    /// Held words keyed by (chart, axis). Filled at atlas/dialect admit,
    /// emptied at remove/shed/retire. `on_axis` is one fiber read.
    pub(crate) axis_index: crate::transducer::sentence::AxisIndex,
    /// Missing cited address → citing dialect sentences. Filled when
    /// Withheld is banked; resolve_withheld reads this, never the dialect.
    pub(crate) waiting_on: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
}

/// THE ENTITY — a kernel identity with its genome/epigenome, conversing through
/// a session: it submits percepts, learns what the mesh teaches (comprehending,
/// not storing), asks quotes, and attests demands.
pub struct Entity {
    pub(crate) session: Session,
    pub epigenome: Epigenome,
    /// Rotating cursor for the refinement probe — one genome word examined
    /// per stroke, so the compression search is metabolic, never a sweep.
    pub(crate) refine_cursor: usize,
    /// Words this voice has spoken up for adoption: said once, the judge
    /// decided (installed, derivable, or refused). Keyed by transport
    /// address; membership is vertex equality, never the digest alone.
    pub(crate) taught_up: crate::network::nerve::Keyed<Element>,
    /// The discordance measured at the last sync (strata whose roots differ
    /// from the hypha's); zero is a glued genome — the condition under which
    /// this entity speaks proofs. Unmeasured: the open thought's boundary
    /// cochain norm, never usize::MAX.
    pub(crate) discordance: usize,
    pub(crate) discordance_measured: bool,
    /// ČECH AFTER FETCH: the obstructions measured on the fibre when a
    /// fetched word met the words already held there — (the fetched
    /// word's address, the transition that did not vanish). The digest
    /// only indexed the wire; this is the glue, on the elements.
    pub(crate) obstructions: crate::network::nerve::Keyed<Element>,
    /// Strata the fibre Čech has already glued (a fetched word's transition
    /// to the held representative vanished): the outer index is not walked
    /// there again. The digest names what differs by address; the geometry
    /// decides what is fetched.
    pub(crate) glued: crate::network::nerve::Keyed<()>,
    /// OPEN THOUGHTS: the contemplation over bubbles. Remainders persist;
    /// residuals are never lost on restart.
    pub(crate) contemplation: crate::transducer::bubble::Contemplation,
    /// Genome length last seen by the pivot; words added since dirty their charts.
    pub(crate) thought_gen: usize,
    /// Listings this mind has opened an axis on (Vec-of-relations, keyed by
    /// listing address on read): an axis is opened once per listing.
    pub(crate) opened: crate::network::nerve::Keyed<()>,
    /// Ideas keyed by address: a cite is a read, never a walk of bubbles.
    pub(crate) ideas: crate::network::nerve::Keyed<Element>,
    pub(crate) physics: crate::physics::Physics,
    pub(crate) height: crate::chronometry::Height,
}

/// Residual strictly below the chain on a lane, and never a raise on any
/// lane. Equal measures do not descend: the mind did not reduce.
pub(crate) fn residual_descends(residual: &[(i64, i64, u8, u8)], chain: &[(i64, i64, u8, u8)]) -> bool {
    let r = crate::transducer::element::measure_lanes(residual);
    let c = crate::transducer::element::measure_lanes(chain);
    let mut below = false;
    for i in 0..4 {
        if r[i] > c[i] {
            return false;
        }
        if r[i] < c[i] {
            below = true;
        }
    }
    below
}

/// ONE CONTEMPLATED BUBBLE, as the quote reads it: the remainder it holds
/// and the volume that remainder stands at.
pub type Bubbled = (Vec<(i64, i64, u8, u8)>, [u64; 4]);

/// WHAT CITING A CHAIN YIELDS: the words it cited, and the part of the chain
/// no held word covers.
pub type Cited = (Vec<crate::anastome::dialogue::CiteWire>, Vec<(i64, i64, u8, u8)>);

/// WHAT CONCLUDING A LISTING RETURNS (24.2).
pub struct Concluded {
    /// The words this entity cited to speak the chain.
    pub cites: Vec<crate::anastome::dialogue::CiteWire>,
    /// The chain it could NOT reduce — its own claim, or the whole chain
    /// back when the residual does not descend and is held as thought.
    pub residual: Vec<(i64, i64, u8, u8)>,
    /// Whether the genome already held that residual. A familiar residual
    /// is not spoken again.
    pub familiar: bool,
}

/// What `speak_word` did with a closed mint. Only [`Utterance::Spoken`]
/// is a fresh frame; the memo and the identity gate are not speech.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Utterance {
    /// Adopted up; the address is now on the memo.
    Spoken,
    /// This address was already uttered; the judge has the word.
    Already,
    /// Identity, or a mint with no group component: not a message.
    Silent,
}

/// The quoted yield: the per-lane surface and the open-demand market's top
/// clusters (address, priced μ⃗, and the chain itself — real, attemptable work).
#[derive(Clone, Debug)]
pub struct YieldQuote {
    /// Book generation at this quote.
    pub height: u64,
    pub surface: [u64; 4],
    /// The judge's k-step at the quote — the temporal coordinate of the
    /// book this quote reads.
    pub k: u64,
    pub clusters: Vec<Cluster>,
}

#[derive(Clone, Debug)]
pub struct Cluster {
    pub address: crate::transducer::a21::Address,
    pub mu: [u64; 4],
    pub chain: Vec<(i64, i64, u8, u8)>,
    /// Open (non-flat) charts of the listing's coherence field. The entity
    /// reads these against its own atlas; several may close, none is ranked.
    pub open: Vec<crate::transducer::stalk::Invariant>,
}

/// A settle receipt riding an install reply: the germ, its consilience parties,
/// and the polytonic μ⃗ award priced at the moment's yield surface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SettleReceipt {
    pub germ: crate::transducer::a21::Address,
    pub voices: Vec<[u8; 32]>,
    pub mu: [u64; 4],
}

pub(crate) fn decode_reply(body: &[u8]) -> Reply {
    // `[code][height u64][pressure 4×u64][taught_len u16][taught…][settle?]`
    let settle = {
        let tl = body.get(41..43).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize).unwrap_or(0);
        body.get(43 + tl..)
            .and_then(dialogue::decode_settle)
            .map(|(germ, voices, mu)| SettleReceipt { germ, voices, mu })
    };
    Reply {
        verdict: decode_verdict(body),
        height: decode_height(body),
        pressure: decode_pressure(body),
        settle,
    }
}

/// A hypha's answer: the verdict, the book height, and the per-lane
/// pressure as four exact u64 lanes — never a max, never a saturated byte.
#[derive(Clone, Debug)]
pub struct Reply {
    pub verdict: Verdict,
    pub height: u64,
    pub pressure: [u64; 4],
    /// The settle receipt when this exchange completed a consilience install.
    pub settle: Option<SettleReceipt>,
}

pub(crate) fn decode_height(body: &[u8]) -> u64 {
    body.get(1..9)
        .and_then(|b| b.try_into().ok())
        .map(u64::from_le_bytes)
        .unwrap_or(0)
}

pub(crate) fn decode_pressure(body: &[u8]) -> [u64; 4] {
    std::array::from_fn(|i| {
        let o = 9 + i * 8;
        body.get(o..o + 8)
            .and_then(|b| b.try_into().ok())
            .map(u64::from_le_bytes)
            .unwrap_or(0)
    })
}

/// Decode a hypha reply: `[code][height u64][pressure 4×u64][taught_len u16][taught…]`.
pub(crate) fn decode_verdict(body: &[u8]) -> Verdict {
    let taught = {
        let tl = body.get(41..43).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize).unwrap_or(0);
        body.get(43..43 + tl).map(<[u8]>::to_vec).unwrap_or_default()
    };
    match body.first() {
        Some(1) => Verdict::Decomposed { taught },
        Some(2) => Verdict::GermPending,
        Some(3) => Verdict::Installed { taught },
        Some(6) => Verdict::Sectioned,
        Some(4) => Verdict::Pivot,
        Some(5) => Verdict::Fragmented,
        Some(&c) => Verdict::Unknown(c),
        None => Verdict::Unknown(0),
    }
}
