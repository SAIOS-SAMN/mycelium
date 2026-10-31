// 24 — the genome a client holds: its words, its dialect, its epigenome.

use super::*;

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
#[derive(Clone)]
pub struct GenomeEntry {
    /// How often this mind speaks or hears it — the recollection
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
    /// tail of a vector) with an ordinal one, and that was called a fix. It
    /// was the same linear reflex in a different uniform.
    pub(crate) expression: u64,
}

/// THE PAYLOAD: the decoded word, and what Z11 moves to disk. Resident
/// today; under Z11 this carrier is the bounded escrowed cache and holds
/// only what a reading has reached.
#[derive(Clone)]
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
#[derive(Clone)]
pub struct ExpressedWord {
    pub(crate) word: Element,
    pub(crate) shape: std::sync::OnceLock<Option<crate::transducer::blade::Shape>>,
}

/// AN OPEN QUESTION AS A GENOME FILE HELD IT: the residual chain's address,
/// its links (row, column, hue before, hue after), and the stratum it stands
/// on (its chart invariant's key, the sheaf sync's coordinate).
pub(crate) type OpenResidual = (Vec<u8>, Vec<(i64, i64, u8, u8)>, Vec<u8>);

#[derive(Default, Clone)]
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
    /// the mind's measured discordance with what it was taught.
    pub(crate) refused: u64,
    /// THE EXPRESSED WORDS (ephemeral, never persisted): the residual
    /// bodies this mind germinated itself, before any court ratified
    /// them. They let the mind recognise a residual it has already
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
    /// Held words keyed by (chart, axis). Filled at atlas/dialect admit,
    /// emptied at remove/shed/retire. `on_axis` is one fibre read.
    pub(crate) axis_index: crate::transducer::sentence::AxisIndex,
    /// Missing cited address → citing dialect sentences. Filled when
    /// Withheld is banked; resolve_withheld reads this, never the dialect.
    pub(crate) waiting_on: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// CLOSED FORMS BY THE CHART OF A BODY THEY CARRY (21 step 3, 24.1):
    /// body chart → the addresses of held forms with a `Word` vertex on
    /// that chart. It is the epigenome's `Form.by_class` (18.2) — "the
    /// fibre a listing body's chart reads" — one level up, filled and
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
    /// MU-CLIENT §7.1 — THE RESIDUAL CHAINS A RESTORED GENOME FILE HELD, by
    /// address: a residual is an open question, never a word (ruled
    /// 2026-10-18), so the restore files it here and the boot hands it to the
    /// mind's carrier at once (`Mind::restore_genome`) — empty after.
    pub(crate) restored_residuals: Vec<OpenResidual>,
    /// The rewrites the restore read on the file and left out — acts on the
    /// language, never words of it: the file held more than the genome now
    /// does, and is written again (`Mind::restore_genome`).
    pub(crate) restored_dropped: u64,
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
    /// RM1, 78.37(4) — THE REMAINDERS BY WHERE THEY WERE PERCEIVED: the
    /// residue's chart chain ‖ κ → the word whose torsion record carries it,
    /// over resident and dialect words alike; derived from the records at
    /// every admission and at restore (the word_fibres pattern), and each
    /// word's keys, so it leaves by them.
    pub(crate) remainder_fibres: crate::network::nerve::Keyed<crate::transducer::address::Address>,
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
    /// 78.160 — THE VOCABULARY: every word this mind took in, by the holes
    /// its word vertices' whole motions wind about (`learn_form`), filed where
    /// it is admitted and unfiled where it retires — the language a motion is
    /// deconstructed over with no court asked. Derived, never persisted: a
    /// restore learns each word again as it admits it.
    pub(crate) vocabulary: super::epigenome::Vocabulary,
    /// 21 step 3 — CLOSED FORMS BY WHAT THE FORMS' MEMBERS DO: `act_key` (chart,
    /// whole motion, hue map) → each held word vertex with a member acting
    /// exactly so, by its `generator_key`, as (form, vertex); filled and
    /// emptied beside `by_moduli`.
    #[allow(clippy::type_complexity)]
    pub(crate) by_act: crate::network::nerve::Keyed<
        crate::network::nerve::Keyed<(crate::transducer::address::Address, crate::transducer::address::Address)>,
    >,
}
