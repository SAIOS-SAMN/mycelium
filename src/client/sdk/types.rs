// 24, 26 — the shapes a client reads and answers with: a verdict, a
// comprehension, a dialect, a working atlas, a genome view, an utterance,
// a quote.

use super::*;

/// The comprehension verdict — the C-STACK's terminal (congruence → coherence →
/// cohesion → converse → COMPREHENSION): the received utterance DECOMPOSED over
/// the local genome toward understanding. Never re-cognition (that is the
/// perception stack's overlay), never recall.
#[derive(Clone, Debug)]
pub enum Comprehension {
    /// The utterance decomposed into held words — address and standing
    /// from the working atlas. Never a sentinel index.
    Understood(Vec<(Vec<u8>, Standing)>),
    /// Partially decomposed: `demand` is the EXACT element still missing —
    /// A12's canonical HNF residue against the span of what this mind holds
    /// on the plane, the whole word off it, or the residual chain a listing
    /// left. `understood` is the part that did factor, kept rather than
    /// thrown away with the rest.
    ///
    /// It carried `residual_cells: usize`. A count is not what M6 step 5
    /// asks for: "No mind emits `unknown`; it emits the element it lacks."
    /// A count cannot be docketed by a hypha, cannot be reduced by another
    /// mind, and cannot be replayed — and the remainder A12 had already
    /// computed exactly was discarded to produce it.
    Novel { demand: Box<Element>, understood: Vec<(Vec<u8>, Standing)> },
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
            // Two demands are one demand when they name the same vertex
            // (M1, rule 3), never when two counts happen to match.
            (
                Comprehension::Novel { demand: a, understood: ua },
                Comprehension::Novel { demand: b, understood: ub },
            ) => a.same_vertex(b) && ua == ub,
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
    /// The sentence AS THIS MIND HOLDS IT (L16): its tape decoded once
    /// when it stood, not once per reader that asks for its form.
    pub sentence: crate::transducer::element::Held,
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
/// never a boolean. Genome-only `atlas()` remains for persistence. The
/// Held stratum is the contemplation's forms, read at the coordinate
/// (24.1 as ruled 2026-09-09): the Word vertices its held forms hold on
/// a chart, through `Contemplation::held_at`, never a list kept beside it.
pub struct WorkingAtlas<'a> {
    pub(crate) genome: &'a crate::transducer::stalk::Stalks,
    pub(crate) dialect: &'a crate::transducer::stalk::Stalks,
    pub(crate) held: Option<&'a crate::transducer::bubble::Contemplation>,
}

/// The members of one stratum split by lane (42.6 as amended): four
/// independent lists and never their concatenation.
pub type LaneMembers = [Vec<(Vec<u8>, Standing)>; 4];

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
        if let Some(held) = self.held {
            // index: the Word vertices the held forms hold on this chart
            for v in held.held_at(inv) {
                let crate::transducer::sentence::Vertex::Word { address, .. } = v else {
                    continue;
                };
                if have.contains(address) {
                    continue;
                }
                have.insert(address, ());
                out.push((address.clone().into_vec(), Standing::Held));
            }
        }
        out
    }

    /// THE GENERATORS OF ONE STRATUM (42.6, 16.12, A21 §2): every address
    /// under a prefix of `inv`, coarsened one axis at a time until the read
    /// is not empty, with the stratum it was found at.
    ///
    /// A12's basis is "the held words on the chart's fiber", and read at the
    /// EXACT chart that basis is almost always empty or a single word: a
    /// connection's chart is its rotor's trace and determinant (M4 step 2),
    /// so every distinct rotation is its own chart and the only words a mind
    /// could factor through were the powers of one word. The lattice could
    /// never have more than one generator and the whole Hermite computation
    /// was decorative. A stratum is the fiber under a PREFIX — "a demand is
    /// reduced against the generators of one stratum, never the language"
    /// — and `peel` is what walks out to it.
    ///
    /// It coarsens no further than it must: the first non-empty stratum is
    /// the one returned, so a mind factors against the nearest generators it
    /// holds and never against everything it holds.
    pub fn stratum(
        &self,
        inv: &crate::transducer::stalk::Invariant,
    ) -> (crate::transducer::stalk::Invariant, Vec<(Vec<u8>, Standing)>) {
        let mut at = inv.clone();
        loop {
            let held = self.fiber_under(&at);
            if !held.is_empty() || at.0.is_empty() {
                return (at, held);
            }
            at = at.peel();
        }
    }

    /// THE STRATUM AS A BASIS PER LANE (42.6 as amended, L12).
    ///
    /// The cover of the language is the PRODUCT of four covers, one per
    /// lane, and a reduction is bounded by a presentation only if it runs
    /// against the lane's own. `stratum` answers the whole stratum, which
    /// is the union; this splits it, so lane `l`'s basis is the members
    /// whose class vector occupies an axis on `l`.
    ///
    /// A member may stand in more than one lane's basis, and that is not a
    /// double count: a generator that turns AND recolours is a generator
    /// on `𝒽` and a generator on `𝒢`, and each lane's reduction is
    /// entitled to it. What is refused is the other direction — reducing a
    /// target that occupies `𝒢` against a generator that occupies only
    /// `𝒽`, which is a basis no lane has.
    ///
    /// A member whose form this mind no longer holds, or whose word has no
    /// class vector, stands in no lane's basis. It is dropped rather than
    /// placed in all four (rule 11).
    /// `class_of` is the caller's: this carrier holds the chart INDEXES
    /// and not the words, so the class vector of a member is resolved by
    /// whoever holds them. Passing it in rather than reaching for a second
    /// carrier is what keeps the atlas one object.
    pub fn stratum_by_lane(
        &self,
        inv: &crate::transducer::stalk::Invariant,
        class_of: &dyn Fn(&[u8]) -> Option<crate::transducer::sentence::ClassVector>,
    ) -> (crate::transducer::stalk::Invariant, LaneMembers) {
        let (at, held) = self.stratum(inv);
        let mut lanes: LaneMembers = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for (addr, st) in held {
            let Some(cv) = class_of(&addr) else { continue };
            for (l, occupied) in cv.lanes().into_iter().enumerate() {
                if occupied {
                    lanes[l].push((addr.clone(), st));
                }
            }
        }
        (at, lanes)
    }

    /// Every address under a chart prefix, not only those exactly at it:
    /// `fiber` reads one node's own members, and a stratum is the subtree
    /// (A21 §2). Ruled first, then Stood, then Held, each address once.
    pub fn fiber_under(
        &self,
        inv: &crate::transducer::stalk::Invariant,
    ) -> Vec<(Vec<u8>, Standing)> {
        let mut out: Vec<(Vec<u8>, Standing)> = Vec::new();
        let mut have: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for (addrs, st) in [
            (self.genome.fiber_under(inv), Standing::Ruled),
            (self.dialect.fiber_under(inv), Standing::Stood),
        ] {
            for a in addrs {
                if have.contains(&a) {
                    continue;
                }
                have.insert(&a, ());
                out.push((a, st));
            }
        }
        out
    }

    /// The standing of an address on a chart: the chart is the coordinate
    /// the read happens at, in every stratum.
    pub fn standing_of(&self, addr: &[u8], inv: &crate::transducer::stalk::Invariant) -> Option<Standing> {
        if self.genome.chart_of(addr) == Some(inv) {
            return Some(Standing::Ruled);
        }
        if self.dialect.chart_of(addr) == Some(inv) {
            return Some(Standing::Stood);
        }
        let on_chart = |v: &crate::transducer::sentence::Vertex| {
            matches!(v, crate::transducer::sentence::Vertex::Word { address, .. } if address.as_bytes() == addr)
        };
        if self.held.is_some_and(|held| held.held_at(inv).into_iter().any(on_chart)) {
            return Some(Standing::Held);
        }
        None
    }
}

/// The working atlas read as one atlas (24.1): a thought expands over the
/// genome's charts and the dialect's alike, each trie walked from the same
/// cursor under one budget; members carry both strata.
impl crate::transducer::stalk::AtlasRead for WorkingAtlas<'_> {
    fn children(
        &self,
        inv: &crate::transducer::stalk::Invariant,
        after: Option<&[u8]>,
        budget: &mut crate::physics::Budget,
        f: &mut dyn FnMut(crate::transducer::stalk::Invariant),
    ) -> Option<Vec<u8>> {
        let a = self.genome.each_child(inv, after, budget, &mut *f);
        let b = self.dialect.each_child(inv, after, budget, &mut *f);
        b.or(a)
    }

    fn members(&self, inv: &crate::transducer::stalk::Invariant) -> Vec<Vec<u8>> {
        self.fiber(inv).into_iter().map(|(a, _)| a).collect()
    }
}

/// The crystallized language: words on disk. The epigenome is the rest.
pub struct GenomeView<'a> {
    pub words: &'a [Element],
}

#[derive(Default)]
pub struct Epigenome {
    /// THE CRYSTALLIZED LANGUAGE AS THIS MIND HOLDS IT (L16): each word
    /// with the form its tape decodes to, decoded once at the admission.
    /// Held as `Element`, every reader that wanted a form decoded the tape
    /// behind a `OnceLock` that a clone did not carry.
    pub(crate) genome: Vec<crate::transducer::element::Held>,
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
    /// CLOSED FORMS BY THE CHART OF A BODY THEY CARRY (21 step 3, 24.1):
    /// body chart → the addresses of held forms with a `Word` vertex on
    /// that chart. It is the epigenome's `Form.by_class` (18.2) — "the
    /// fiber a listing body's chart reads" — one level up, filled and
    /// emptied beside `axis_index`.
    ///
    /// The glue asks the crystal at a perceived body's moduli, and the
    /// atlas files a sentence under `chart_of_form`: `[2, vertices, faces,
    /// loops, …]` against `[components, …harmonics…]`, two coordinate
    /// systems that meet only at the empty root. Read there, `on_class`
    /// answered the empty vector to every question the glue could ask, so
    /// no rule ever cited a held form, no citation was ever a use, and
    /// nothing was ever paid for one (26, 27).
    pub(crate) by_moduli: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// 1.8, 50.2 step 2 — THE GENERATORS this mind holds, by the chart of
    /// the versor each carries: versor chart → (vertex address → form
    /// address). A ruled word is a form since L5, so the generators of the
    /// presentation are its WORD VERTICES; read off `Element::letters`,
    /// which answers `None` for every sentence, a factorisation saw only
    /// the v1 connection words the utterance path stopped producing.
    pub(crate) generators: crate::transducer::sentence::GeneratorIndex,
    /// 42.6 AS AMENDED, L12 — THE SAME GENERATORS, ONE COVER PER LANE.
    /// The cover of the language is the product of four, and a reduction
    /// is bounded by a presentation only when it runs against the lane's
    /// own basis. `generators` above is the `𝒽` cover under another name
    /// and is kept as the reader every existing caller takes; this is the
    /// four, and `lanes[3]` holds the same members under the same chart.
    pub(crate) lanes: crate::transducer::sentence::LaneIndex,
}

/// THE ENTITY — a kernel identity with its genome/epigenome, conversing through
/// a session: it speaks sentences, learns what the mesh teaches (comprehending,
/// not storing), asks quotes, and probes listings.
/// Why an axis did not open (24.3), written by the stroke as `axis refused`.
pub const AXIS_ONE_BODY: &str = "one body deflated the listing";
pub const AXIS_NO_BODY: &str = "a held word has no body";
pub const AXIS_NO_VERSOR: &str = "no rational versor carries one body onto the other";
pub const AXIS_TRIED: &str = "the listing was tried in this language";
pub const AXIS_ALREADY: &str = "the axis germ was already spoken";
pub const AXIS_REFUSED_SPEECH: &str = "the axis germ was refused on the wire";

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
    /// `taught_up` INDEXED BY VERTEX STRATUM: the M1 key of a word's rotor
    /// (or its canonical chain, where it has no letters) to the addresses
    /// uttered at it.
    ///
    /// `already_taught` walked the whole of `taught_up` doing an exact
    /// `same_vertex` against every word this mind had ever uttered, on
    /// every sentence of every speak — law 3's "no loop over pairs the
    /// carrier does not already name", and its answer: "the carrier is
    /// missing a key and that key is the first thing to write". On a mesh
    /// whose voices all produce language, that walk grew with the mesh's
    /// whole output.
    ///
    /// The key is NECESSARY and not sufficient: `same_vertex` requires the
    /// rotors to be projectively equal, so two words that are one vertex
    /// share this key, and the exact test still decides over the fiber it
    /// names. That is A21 §3's own shape — "equal keys are necessary for
    /// two classes to be the same; sufficiency … is `solve_gauge_between`".
    pub(crate) taught_at: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
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
    /// OPEN THOUGHTS: the contemplation over bubbles, the one carrier of
    /// held forms (24.1 as ruled 2026-09-09); a cite reads their words at
    /// the listing body's chart. Remainders persist; residuals are never
    /// lost on restart.
    pub(crate) contemplation: crate::transducer::bubble::Contemplation,
    /// Genome length last seen by the pivot; words added since dirty their charts.
    pub(crate) thought_gen: usize,
    /// Listings this mind has opened an axis on (Vec-of-relations, keyed by
    /// listing address on read): an axis is opened once per listing.
    pub(crate) opened: crate::network::nerve::Keyed<()>,
    /// Axes this mind has spoken (24.3): the count the stroke line reads,
    /// kept apart from `opened`, which is cleared when the language moves.
    pub(crate) axes_spoken: usize,
    pub(crate) physics: crate::physics::Physics,
    pub(crate) height: crate::chronometry::Height,
    /// THE SEALED SET (50.9): addresses a court refused, each with the K
    /// of this mind's own receipt of the refusal (45.1). Read before any
    /// frame is built; a sealed tape is refused by the mind itself.
    pub(crate) sealed: crate::network::nerve::Keyed<u64>,
    /// Contraction certificates by the contracted form's vertex key, cited
    /// on the next utterance of that form (50.9).
    pub(crate) contractions: crate::network::nerve::Keyed<crate::transducer::sentence::Contraction>,
    /// The contracted sentences waiting to be uttered, each with the family
    /// and the task it was minted for.
    pub(crate) contracted: Vec<Pending>,
    /// 21 step 3, chapter 6's depth order — WHAT THIS MIND OWES A COURT
    /// BEFORE IT SPEAKS AGAIN. A court that answered `Awaiting` has not
    /// ruled a form this mind cited; the mind HOLDS that form, so it
    /// utters the cited form first and the citer after it. Nothing was
    /// sealed and nothing was refused: this is the mind teaching the
    /// court what it was speaking from, in the order a cited form must
    /// precede its citer.
    pub(crate) owed: Vec<Owed>,
}

/// A cited form owed to a court, with the citer that waits on it and the
/// family and task both were spoken over.
pub struct Owed {
    pub form: Element,
    pub citer: Element,
    pub train: Vec<crate::transducer::family::TrainPair>,
    pub task: Vec<u8>,
}

/// A CONTRACTED SENTENCE WAITING TO BE UTTERED (50.9): the sentence the
/// move produced, the family it was minted over, the task it answers, and
/// the certificate it cites.
#[derive(Clone, Debug)]
pub struct Pending {
    pub sentence: Element,
    pub train: Vec<crate::transducer::family::TrainPair>,
    pub task: Vec<u8>,
    pub certificate: crate::transducer::sentence::Contraction,
}

/// WHAT THE MIND'S OWN REDUCTION DID, per lane: the residual measured
/// against the chain it came from. A reading — the caller decides what to
/// do with a residual that raised a lane or moved nothing, and 24.2 says
/// what that is: hold it as thought rather than speak it.
pub(crate) fn residual_move(
    residual: &[(i64, i64, u8, u8)],
    chain: &[(i64, i64, u8, u8)],
) -> [crate::economy::LaneMove; 4] {
    crate::economy::lane_moves(
        &crate::transducer::element::measure_lanes(residual),
        &crate::transducer::element::measure_lanes(chain),
    )
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
    pub address: crate::transducer::address::Address,
    pub mu: [u64; 4],
    pub chain: Vec<(i64, i64, u8, u8)>,
    /// Open (non-flat) charts of the listing's coherence field. The entity
    /// reads these against its own atlas; several may close, none is ranked.
    pub open: Vec<crate::transducer::stalk::Invariant>,
}

/// THE DEFERRAL a judge answered with (23.8, S13.4): the door's per-lane
/// reserve deficit, carried on the `WouldBlock` refusal so the arm dwells
/// on the lanes the judge named.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Deferral {
    pub lanes: [u64; 4],
}

impl std::fmt::Display for Deferral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "deferred by the judge: deficit {:?}", self.lanes)
    }
}

impl std::error::Error for Deferral {}
