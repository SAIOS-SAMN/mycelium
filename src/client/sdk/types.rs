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
    /// A count of residual links is not what M6 step 5 asks for: "No mind
    /// emits `unknown`; it emits the element it lacks." A count cannot be
    /// docketed by a hypha, cannot be reduced by another mind, and cannot
    /// be replayed — and the remainder A12 computes exactly would be
    /// discarded to produce it.
    Novel { demand: Box<Element>, understood: Vec<(Vec<u8>, Standing)> },
    /// A CONNECTION heard by holding it: the transition to the held word on
    /// its fibre vanished — the same word, glued.
    Held,
    /// A connection heard whose transitions to the words on its fibre did not
    /// vanish: THE OBSTRUCTIONS, keyed by the held word each was measured
    /// against.
    ///
    /// This was one `Box<Element>`. `comprehend` measured a transition against
    /// every word on the fibre and kept the one whose held word had the
    /// LOWEST ADDRESS, discarding the rest — so a mind that could say exactly
    /// how it differed from four held words said how it differed from one of
    /// them, chosen by a byte comparison on an address that carries no
    /// ordering (1.5: an address is an identity, not a rank). Chapter 13 says
    /// the obstructions on a fibre are a COCHAIN: they are all of them, and
    /// which words they stand against is half of what they say.
    Dialect(crate::network::nerve::Keyed<Element>),
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
            // Two cochains are one cochain when they stand against the same
            // words and name the same vertex at each — never when two counts
            // match (rule 2).
            (Comprehension::Dialect(a), Comprehension::Dialect(b)) => {
                a.len() == b.len()
                    && a.scoped().all(|(k, ga)| match b.get(&k) {
                        Some(gb) => ga.same_vertex(gb),
                        None => false,
                    })
            }
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

    /// THE STRONGEST STANDING this mind has on a chart — whether ANY word
    /// on it is ruled, else whether any stands, else held.
    ///
    /// No member is taken. 24.1 puts several words on one stratum as
    /// multiplicity, so the head of the fiber is one of possibly many
    /// ruled words, and taking it would be a choice (rule 15) even though
    /// the value read off it is the same for all of them. The reading is
    /// over the fiber, so the multiplicity never has to be resolved.
    pub fn standing_on(&self, inv: &crate::transducer::stalk::Invariant) -> Standing {
        let on = self.fiber(inv);
        if on.iter().any(|(_, s)| matches!(s, Standing::Ruled)) {
            Standing::Ruled
        } else if on.iter().any(|(_, s)| matches!(s, Standing::Stood)) {
            Standing::Stood
        } else {
            Standing::Held
        }
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
    /// — and `peel` is what reaches it.
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

/// The working atlas read as one atlas (24.1): a thought meets the genome's
/// charts and the dialect's alike, and the members at a chart carry both
/// strata.
///
/// A `children` arm stood here that walked BOTH tries from one cursor under
/// one budget and returned whichever cursor came back last — two
/// independent traversals sharing a position, which is a resume point that
/// belongs to neither. It is gone with the trait method: a chart's members
/// are a keyed read of both fibres, which needs no cursor to be correct
/// across strokes.
impl crate::transducer::stalk::AtlasRead for WorkingAtlas<'_> {
    fn members(&self, inv: &crate::transducer::stalk::Invariant) -> Vec<Vec<u8>> {
        self.fiber(inv).into_iter().map(|(a, _)| a).collect()
    }
}

/// The crystallized language: words on disk. The epigenome is the rest.
pub struct GenomeView<'a> {
    pub words: &'a [Element],
}

/// ONE GENOME WORD AND EVERYTHING KEPT ABOUT IT, AT ITS OWN COORDINATE.
///
/// It was four carriers side by side — `genome: Vec<Held>`, `expression:
/// Vec<u64>`, `shapes: Vec<OnceLock<..>>` and `index: Keyed<usize>` — three
/// of them documented "parallel to the genome" and the fourth a stored
/// derivation of the first, rebuilt from nothing by `reindex()` and holding
/// 11.5 MB of duplicated address on a voice eleven minutes into a soak.
/// Parallel vectors are a Cartesian frame: they make POSITION the identity
/// of a word, so a removal has to shift three carriers and re-key a fourth,
/// and the word's own coordinate — which law 2 says IS its identity — is
/// carried beside it instead of under it.
///
/// One word, one entry, filed at the address the word derives for itself.
/// Nothing is parallel, nothing shifts, and there is no index because the
/// trie's prefix read IS the lookup.
/// Z11's FIRST STEP — THE INDEX IS A SEPARATE OBJECT FROM THE PAYLOAD.
///
/// The row rules that the genome persists on disk and a voice keeps
/// resident "an index by address", decoding a word only when cited or
/// spoken. That target is not measurable while the index and the word are
/// one struct: the carrier line can only report `epi.genome` as one number
/// and cannot say which half Z11 removes. Split here first, with the
/// payload still resident, so the vector reads them apart and the row
/// lands against a reading rather than against a hope.
///
/// Both are keyed by the word's own address (law 2). Nothing is parallel:
/// an entry and its word are two carriers at ONE coordinate, which is not
/// the parallel-vector defect Z9 removed — that made POSITION the identity
/// and made a removal shift three carriers. Here a removal is two keyed
/// removes at the same key, and neither can name a word the other does not.
pub struct GenomeEntry {
    /// How often this entity speaks or hears it — the recollection
    /// efficiency. Content: how often a word was used is not derivable
    /// from the word.
    ///
    /// THE ADMISSION ORDINAL STOOD HERE and is gone. It was `seq`, a
    /// counter handed out per admission, read by `thought_gen` as "every
    /// word added since". That is an append-only timeline: it answers
    /// "which words arrived after the n-th" when the question the pivot
    /// actually asks is "which words has it not reached". Whether a word
    /// arrived at stroke 2 or stroke 200 is not a property of the word;
    /// whether its obstruction is still undischarged is. The pending work
    /// is a set of COORDINATES (`unpivoted`), drained at the address.
    ///
    /// It replaced a positional watermark (`genome()[thought_gen..]`, a
    /// tail of a vector) with an ordinal one and I called that a fix. It
    /// was the same linear reflex in a different uniform.
    pub(crate) expression: u64,
}

/// THE PAYLOAD: the decoded word, and what Z11 moves to disk. Resident
/// today; under Z11 this carrier is the bounded escrowed cache and holds
/// only what a reading has reached.
pub struct GenomeWord {
    /// The word, with the form its tape decodes to (L16).
    pub(crate) held: crate::transducer::element::Held,
    /// This word's shape, lifted once on first use. A measurement
    /// memoized under the word it measures, never persisted.
    pub(crate) shape: std::sync::OnceLock<Option<crate::transducer::blade::Shape>>,
}

/// An expressed word — a residual this mind germinated and has not had
/// ratified — at its own coordinate, for the same reason.
#[cfg(test)]
pub struct ExpressedWord {
    pub(crate) word: Element,
    pub(crate) shape: std::sync::OnceLock<Option<crate::transducer::blade::Shape>>,
}

#[derive(Default)]
pub struct Epigenome {
    /// THE CRYSTALLIZED LANGUAGE AS THIS MIND HOLDS IT (L16): each word
    /// with the form its tape decodes to, decoded once at the admission.
    /// Held as `Element`, every reader that wanted a form decoded the tape
    /// behind a `OnceLock` that a clone did not carry.
    pub(crate) genome: crate::network::nerve::Keyed<GenomeEntry>,
    /// The decoded words, at the same coordinates as `genome`. Z11 backs
    /// this with the genome file and bounds it by escrow.
    pub(crate) resident: crate::network::nerve::Keyed<GenomeWord>,
    /// Locally comprehended sentences, keyed by the sentence's word addresses
    /// (not by basis indices that shift on retire).
    pub(crate) recollections: crate::network::nerve::Keyed<(Vec<Vec<u8>>, u64)>,
    /// THE ATLAS of the genome — admitted in place as words are learned
    /// or restored; rebuilt only on retire. Never persisted.
    pub(crate) atlas: crate::transducer::stalk::Stalks,
    pub(crate) atlas_len: usize,
    /// THE WORDS THE PIVOT HAS NOT REACHED, as coordinates. A word's
    /// address enters on admission and leaves when the pivot has taken
    /// it; nothing here says when anything arrived, and nothing counts.
    pub(crate) unpivoted: crate::network::nerve::Keyed<()>,
    /// Taught words refused because they did not verify (a transition
    /// that is not a versor, a section that does not span the support) —
    /// the entity's measured discordance with what it was taught.
    pub(crate) refused: u64,
    /// THE EXPRESSED WORDS (ephemeral, never persisted): the residual
    /// bodies this entity germinated itself, before any court ratified
    /// them. They let the entity recognise a residual it has already
    /// claimed, so a claim is spoken once; a ratified word arrives by
    /// teaching and enters the genome.
    #[cfg(test)]
    pub(crate) expressed: crate::network::nerve::Keyed<ExpressedWord>,
    #[cfg(test)]
    pub(crate) expressed_atlas: crate::transducer::stalk::Stalks,
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
    /// orbits, …]` against `[components, …harmonics…]`, two coordinate
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
    /// FX1's reading, alternative B (78.37(3)) — the dialect words a sealed
    /// form's circuit ran through their vertex: the voice's K it last did,
    /// and the ones this orbit (volatile, never persisted).
    pub(crate) traversed_k: crate::network::nerve::Keyed<u64>,
    pub(crate) traversed_orbit: crate::network::nerve::Keyed<()>,
    /// RM1, 78.37(4) — THE REMAINDERS BY WHERE THEY WERE PERCEIVED: the
    /// residue's chart chain ‖ κ → the word whose torsion record carries it,
    /// over resident and dialect words alike; derived from the records at
    /// every admission and at restore (the word_fibers pattern), and each
    /// word's keys, so it leaves by them.
    pub(crate) remainder_fibers: crate::network::nerve::Keyed<crate::transducer::address::Address>,
    pub(crate) remainder_keys: crate::network::nerve::Keyed<Vec<Vec<u8>>>,
    pub(crate) remainders_coincident: u64,
    /// RM1's RECEPTOR: a remainder arriving where one of its class stands at
    /// a NESTED chain (one vantage at a finer depth), at an incomparable one
    /// with a differing ray (DISCORDANT: agreeing on the class, not on the
    /// twist); the calluses the receptor raised and awaits the voice to
    /// stand, keyed per κ per pair of chains so each pair raises one; and
    /// what the voice made of them.
    pub(crate) remainders_nested: u64,
    pub(crate) remainders_discordant: u64,
    pub(crate) calluses_pending: Vec<super::epigenome::remainder::Callus>,
    pub(crate) calloused: crate::network::nerve::Keyed<()>,
    pub(crate) calluses: super::epigenome::remainder::CallusTally,
    /// 78.49 — THE COURT'S STAMP EACH WORD WAS ADMITTED AT, keyed by the
    /// word and sharing its address's bytes: the court seals and
    /// adjudicates, so a word is crystallized knowledge exactly where a
    /// court admitted it, and this is its citation's coordinate. A word the
    /// mind only stood itself has none until a court rules it.
    pub(crate) admitted_at: crate::network::nerve::Keyed<crate::chronometry::Stamp>,
    /// 78.49 — THE REMAINDERS THIS MIND RE-SPELLED (RM1 (c)), by the word and
    /// the remainder's own key → the word that spells it: the mind's act,
    /// volatile, never a stamp — a remainder is ANSWERED on the record only
    /// by a court's seal. The receptor and the open read skip both.
    pub(crate) refilled: crate::network::nerve::Keyed<crate::transducer::address::Address>,
    /// 78.160 P2 — WHAT ITS COURT ANSWERED IT, by the motion's canonical key
    /// (`presentation::motion_key`): the generators whose powers compose to
    /// the motion exactly — minerals, volatile, never persisted. One entry per
    /// motion asked and answered whole. OWED: its bound beside the genome's.
    pub(crate) presented: crate::network::nerve::Keyed<Vec<(crate::transducer::address::Address, crate::transducer::address::Address, i64)>>,
    /// The motions asked of the court in this language — cleared when the
    /// atlas root it was asked at moves, so a motion is asked again once the
    /// mind holds more than it did (`asked_root`).
    pub(crate) asked: crate::network::nerve::Keyed<()>,
    pub(crate) asked_root: Option<crate::transducer::address::Address>,
    /// 78.160 — THE EAR: every genome word this mind took in, heard by the
    /// holes its word vertices' whole motions wind about (`hear_form`), filed
    /// where it is admitted and unfiled where it retires — the language a
    /// motion is deconstructed over with no court asked. Derived, never
    /// persisted: a restore hears each word again as it admits it.
    pub(crate) ear: super::epigenome::Ear,
    /// 21 step 3 — CLOSED FORMS BY WHAT THEIR MEMBERS DO: `act_key` (chart,
    /// whole motion, hue map) → each held word vertex with a member acting
    /// exactly so, by its `generator_key`, as (form, vertex); filled and
    /// emptied beside `by_moduli`.
    #[allow(clippy::type_complexity)]
    pub(crate) by_act: crate::network::nerve::Keyed<
        crate::network::nerve::Keyed<(crate::transducer::address::Address, crate::transducer::address::Address)>,
    >,
}

/// THE ENTITY — a kernel identity with its genome/epigenome, conversing through
/// a session: it speaks sentences, learns what the mesh teaches (comprehending,
/// not storing), asks quotes, and probes listings.
pub struct Entity {
    pub(crate) session: Session,
    pub epigenome: Epigenome,
    /// Rotating cursor for the refinement probe — one genome word examined
    /// per stroke, so the compression search is metabolic, never a sweep.
    /// WHERE THE REFINEMENT CIRCUIT STOOD, AS A COORDINATE. It was a `usize`
    /// advanced by `% genome.len()`: modular arithmetic over a length that
    /// changes whenever a word is admitted or retired, so the cursor
    /// silently skipped and re-visited words as the genome moved under it.
    /// The court's own refine cursor has been an address since 45.7; this
    /// is the same object. `None` is a circuit that has not started (X3) and
    /// is an absence, not an empty key.
    pub(crate) refine_cursor: Option<Vec<u8>>,
    /// The rule the court last refused for each task, by task id — so the
    /// same rule is not sent again (`Entity::solve_task`). One per task.
    pub(crate) solve_refused: crate::network::nerve::Keyed<Vec<u8>>,
    /// Words this voice has spoken up for adoption: said once, the judge
    /// decided (installed, derivable, or refused). Keyed by transport
    /// address; membership is vertex equality, never the digest alone.
    pub(crate) taught_up: crate::network::nerve::Keyed<Element>,
    /// `taught_up` INDEXED BY VERTEX STRATUM: the M1 key of a word's rotor
    /// (or its canonical chain, where it has no letters) to the addresses
    /// uttered at it.
    ///
    /// `already_taught` read the whole of `taught_up` doing an exact
    /// `same_vertex` against every word this mind had ever uttered, on
    /// every sentence of every speak — law 3's "no loop over pairs the
    /// carrier does not already name", and its answer: "the carrier is
    /// missing a key and that key is the first thing to write". On a mesh
    /// whose voices all produce language, that read grew with the mesh's
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
    /// WHAT THIS MIND LEARNED SINCE THE STROKE LAST TOOK IT (24.3): the
    /// measure, per lane, of every page `absorb_taught` admitted — the
    /// outcome's `learned` mode, accumulated at the admission and taken
    /// once per stroke by `take_learned_mu`.
    pub(crate) learned_mu: [u64; 4],
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
    pub(crate) physics: crate::physics::Physics,
    pub(crate) height: crate::chronometry::Height,
    /// 78.49, 78.23 — THIS MIND'S OWN K: the orbits it has closed, one per
    /// stroke in which it acted — it stood, uttered, learned or settled —
    /// counted at the close. The mind's clock is its own motion; the ledger
    /// is the court's (law 8), and no receipt of the mind's exists.
    pub(crate) k: u64,
    /// Whether this orbit has acted yet: the close steps `k` only then.
    pub(crate) acted: bool,
    /// ACT1 (78.21(3)) — the court's height this voice had heard, and its
    /// own `K`, when its genome was last written whole; none before.
    pub(crate) genome_persisted: Option<(u64, u64)>,
    /// THE SEALED SET (50.9): addresses a court refused, each with the mind's
    /// own K at which it heard the refusal (78.49). Read before any frame is
    /// built; a sealed tape is refused by the mind itself.
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
    /// HOW MANY COURTS A WAITING CITER HAS BEEN OWED TO, by its address:
    /// an owed frame goes to the courts in turn, so a citer is owed at
    /// most once per court this mind knows and then waits for the family
    /// to come round again. Cleared when the citer is admitted or sealed.
    pub(crate) owed_tries: crate::network::nerve::Keyed<usize>,
    /// 37.7 — WHAT THIS MIND SPOKE AND NO COURT HAS TAUGHT YET, by address:
    /// a letter or crystallized idea a court deferred stands in the dialect,
    /// and the guard that kept it from being spoken twice kept it from ever
    /// being spoken again (courts deferred 211/126/442 acts, epoch 146). Said
    /// again each stroke until a court stands or ratifies it, or seals it;
    /// at most an aperture of them (`physics::aperture_cycle`).
    pub(crate) unheard: crate::network::nerve::Keyed<Element>,
    /// 78.153 — THE SOLVES THE COURT HAS NOT JUDGED YET (`SOLVE_UNREAD`), by
    /// task: the rule and the outputs, asked again each stroke until the
    /// court rules, its replay resuming where its stroke ran out. Without it
    /// the task arm moved to the next task and the solve was never asked
    /// again: 38 of 41 verdicts on the fleet were "unread" (2026-10-10). At
    /// most an aperture of them (`physics::aperture_cycle`).
    pub(crate) unread_solves: crate::network::nerve::Keyed<(Element, Vec<Grid>)>,
    /// The task coordinate the last unread solve was asked again at: the
    /// next asking takes the least after it, so every one is asked in turn.
    pub(crate) unread_after: Vec<u8>,
    /// 78.146 — THE SEATS THIS MIND SITS IN A GAME, by domain ‖ seat: what
    /// it perceives there and what its calls were seen to do (volatile, the
    /// mind's own, never persisted).
    pub(crate) seats: crate::network::nerve::Keyed<super::seat_mind::SeatMind>,
    /// 78.161 — this mind's chatter with other minds, where it opened one.
    pub(crate) chat: Option<super::entity::chatter::Chat>,
    /// 78.164 — the boundaries this mind is open on, and what its field
    /// keeps from tick to tick to meet them.
    pub(crate) board: super::entity::tick::Board,
    /// MU-CLIENT §4.2 — THIS MIND'S OWN FIELD: its tubes and orbits, its
    /// lanes, its lexicon, the words taught it, the covers it parked; it
    /// thinks in it for the length of each tick.
    pub(crate) field: crate::transducer::mind::MindField,
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
#[cfg(test)]
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
    /// back when the residual raised a lane or moved nothing and is held
    /// as thought.
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
    /// The listing WHOLE, as the judge served it (L8q). Served as the
    /// coboundary links alone, a sentence would reach the voice with its
    /// demand vertices stripped and the only move left would be to subtract
    /// links; `Held::of` rebuilds the form and `demand_vertices` names what
    /// it could not spell.
    pub element: Vec<u8>,
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
