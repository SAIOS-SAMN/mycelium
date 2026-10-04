// 78.69, 78.70, 78.75 — THE RUNGS A MIND CLIMBS: every ring it perceives
// composed by the master sentence equation, every closed sentence extrapolated
// to its theorem, and every closed sentence's open path held as a word of the
// next rung — the grammar applied to its own closures, live.
//
//! A SENTENCE OF RUNG `r + 1` IS ONE WHOSE TERMS ARE WORDS OF RUNG `r`: each
//! term's motion is looked up, by its own coordinates, among the shortcuts
//! this mind already holds, and a sentence composed of rung-`r` words teaches
//! a shortcut of rung `r + 1`. A ring none of whose terms is a held shortcut
//! is a sentence of rung-0 words and teaches rung 1.
//!
//! A SHORTCUT IS SPELLED OR OFFERED. Its motion is peeled against the held
//! alphabet (`peel_versor`): spelled, it is grammar — a composite of held
//! generators, costing the language nothing (78.60) — and is held as this
//! mind's own word of its rung, in its epigenome, volatile (78.49, 80.2);
//! unspelled, it is a generator the language cannot yet say, and it reaches
//! the court only as the mind speaks the form that closes it — the court
//! re-derives the sentence and offers the letter (37.7, 78.61 C2). Nothing
//! crosses between minds except through a court (law 8).
//!
//! RECOGNITION IS COMPREHENSION (80.2): a closed sentence whose shortcut the
//! mind already holds re-derives it — counted, never held twice.
//!
//! The lexicon is one mind's — a voice is one mind per process — and holds
//! one entry per distinct shortcut, by its coordinates: it grows with what
//! the mind has comprehended, never with how often.

use super::compose::{Closure, Composition, RingUnread};
use super::theorem::{Certificate, Open, Theorem};
use crate::network::nerve::Keyed;
use crate::transducer::cga::Mv;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

/// ONE SHORTCUT THE MIND HOLDS: its motion, its rung, and whether the
/// alphabet spells it.
#[derive(Clone, Debug)]
struct Held {
    rung: u64,
    spelled: bool,
    /// 78.98 — the word's motion itself, what its sentences compose with,
    /// and its generators on its own modes once the lexicon has grown with it.
    word: Mv,
    gens: Option<Vec<Mv>>,
}

/// 78.98 — A MIND'S LEXICON, AND THE ALGEBRA IT GENERATES: the words held by
/// their coordinates; the words held since the lexicon last grew, each owed
/// its sentences with every held word; and the basis of
/// `𝔏 = Lie⟨B(W) : W held⟩`, closed under the commutator.
pub struct Lexicon {
    held: Keyed<Held>,
    fresh: Vec<Vec<u8>>,
    basis: Vec<Mv>,
}

impl Default for Lexicon {
    fn default() -> Self {
        Lexicon { held: Keyed::new(), fresh: Vec::new(), basis: Vec::new() }
    }
}

static LEXICON: std::sync::LazyLock<std::sync::Mutex<Lexicon>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Lexicon::default()));

const SLOTS: usize = 19;
static TALLY: [AtomicU64; SLOTS] = [const { AtomicU64::new(0) }; SLOTS];
const RINGS: usize = 0;
const UNREAD: usize = 1;
const CLOSED: usize = 2;
const ROSETTE: usize = 3;
const UNBOUND: usize = 4;
const OPEN_UNREAD: usize = 5;
const PERIODIC: usize = 6;
const NILPOTENT: usize = 7;
const RECOGNIZED: usize = 8;
const SPELLED: usize = 9;
const UNSPELLED: usize = 10;
const TOP_RUNG: usize = 11;
const RANK: usize = 12;
const SOURCES: usize = 13;
const HEARD: usize = 14;
const COMMUTATORS: usize = 15;
const CURVED: usize = 16;
const LEX_RANK: usize = 17;
const GRAMMAR: usize = 18;

fn note(slot: usize, by: u64) {
    if by > 0 && TALLY[slot].fetch_update(Relaxed, Relaxed, |held| held.checked_add(by)).is_err() {
        let mut past = u64::MAX;
        let _ = crate::physics::metabolism::step_or_refuse(&mut past, by);
    }
}

/// WHAT THE MIND'S SENTENCES DID, in this process.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SentenceTally {
    /// Rings composed, and rings whose terms did not read.
    pub rings: u64,
    pub unread: u64,
    /// Sentences that closed (`Ω^ℓ ~ 1`), and the open ones by why.
    pub closed: u64,
    pub rosette: u64,
    pub unbound: u64,
    pub open_unread: u64,
    /// Theorems extrapolated: periodic, nilpotent — and, of the nilpotent,
    /// those that were white-hole sources (a refusal inverted, 78.75).
    pub periodic: u64,
    pub nilpotent: u64,
    pub sources: u64,
    /// Shortcuts: re-derived (comprehension), held new as spelled words,
    /// and new ones the alphabet does not spell (offered as the mind speaks).
    pub recognized: u64,
    pub spelled: u64,
    pub unspelled: u64,
    /// The highest rung a shortcut has reached, and the summed Lie rank of
    /// the closed sentences' carried words.
    pub top_rung: u64,
    pub rank: u64,
    /// 78.97 — rings that cross a chord: heard by the harmonic eye, never
    /// composed as sentences of rays.
    pub heard: u64,
    /// 78.98 — sentences of two held words composed (their commutators), those
    /// that are curved (the two do not commute), and the rank of the algebra
    /// the lexicon generates.
    pub commutators: u64,
    pub curved: u64,
    pub lexicon_rank: u64,
    /// Words a bracket taught inside the lexicon's span: grammar, derivable at
    /// no cost (78.60) — held, generating no further sentences.
    pub grammar: u64,
}

/// 78.96, 78.97, 78.98 — THE TALLY'S NAMES, in [`SentenceTally::row`]'s
/// order: the typed reading the voice's Senses carry.
pub const SENTENCES: [&str; 19] = [
    "rings",
    "unread",
    "closed",
    "rosette",
    "unbound",
    "open unread",
    "periodic",
    "nilpotent",
    "sources",
    "shortcuts recognized",
    "shortcuts spelled",
    "shortcuts unspelled",
    "top rung",
    "rank",
    "heard through a chord",
    "commutators composed",
    "commutators curved",
    "lexicon rank",
    "commutators grammar",
];

impl SentenceTally {
    /// In [`SENTENCES`]'s order.
    pub fn row(&self) -> [u64; 19] {
        [
            self.rings,
            self.unread,
            self.closed,
            self.rosette,
            self.unbound,
            self.open_unread,
            self.periodic,
            self.nilpotent,
            self.sources,
            self.recognized,
            self.spelled,
            self.unspelled,
            self.top_rung,
            self.rank,
            self.heard,
            self.commutators,
            self.curved,
            self.lexicon_rank,
            self.grammar,
        ]
    }
}

/// The sentences' tally since the process began.
pub fn sentence_tally() -> SentenceTally {
    let r = |i: usize| TALLY[i].load(Relaxed);
    SentenceTally {
        rings: r(RINGS),
        unread: r(UNREAD),
        closed: r(CLOSED),
        rosette: r(ROSETTE),
        unbound: r(UNBOUND),
        open_unread: r(OPEN_UNREAD),
        periodic: r(PERIODIC),
        nilpotent: r(NILPOTENT),
        sources: r(SOURCES),
        recognized: r(RECOGNIZED),
        spelled: r(SPELLED),
        unspelled: r(UNSPELLED),
        top_rung: r(TOP_RUNG),
        rank: r(RANK),
        heard: r(HEARD),
        commutators: r(COMMUTATORS),
        curved: r(CURVED),
        lexicon_rank: r(LEX_RANK),
        grammar: r(GRAMMAR),
    }
}

/// How many shortcuts the mind holds: spelled (its own words of their rungs)
/// and unspelled (letters the court is offered as the mind speaks).
pub fn lexicon() -> (usize, usize) {
    LEXICON.lock().map_or((0, 0), |l| {
        let spelled = l.held.scoped().filter(|(_, h)| h.spelled).count();
        (spelled, l.held.len() - spelled)
    })
}

/// 78.98 — THE MIND'S LEXICON GROWS: each word held since it last grew is
/// composed with every word held (`Lexicon::grow`), under the stroke.
pub fn grow(budget: &mut crate::physics::Budget) {
    if let Ok(mut lex) = LEXICON.lock() {
        lex.grow(budget);
    }
}

impl Lexicon {
    /// The rank of the algebra the held words generate.
    pub fn rank(&self) -> usize {
        self.basis.len()
    }

    /// The highest rung a held word stands at.
    pub fn top_rung(&self) -> u64 {
        self.held.scoped().map(|(_, h)| h.rung).fold(0, u64::max)
    }

    /// How many words are held.
    pub fn len(&self) -> usize {
        self.held.len()
    }

    pub fn is_empty(&self) -> bool {
        self.held.is_empty()
    }

    /// 78.98 — ONE SENTENCE HEARD INTO THIS LEXICON (`hear`'s body, on a
    /// lexicon the caller holds).
    pub fn hear(&mut self, s: &Composition) {
        hear_into(self, s);
    }

    /// 78.98 — THE LEXICON GROWS BY ITS OWN SENTENCES. Each word held since it
    /// last grew joins the lexicon's algebra (its generators on their own
    /// modes, closed under the commutator), and meets every word held in the
    /// smallest closed path the two make — `Ω = W̃_b · W̃_a · W_b · W_a`, the
    /// group commutator, a sentence of held words by the master equation.
    ///
    /// BOUNDED BY THE ALGEBRA, NEVER BY A CONSTANT. Two words whose generators
    /// commute (`[B_a, B_b] = 0`) make a flat sentence by construction, so it
    /// is read off the bracket and never composed — every slide against every
    /// slide. A curved sentence is composed and heard, and the word it teaches
    /// is held. The algebra's closure already holds every bracket direction,
    /// so a bracket's word lies in the span: it is GRAMMAR (derivable, 78.60's
    /// zero cost), held as a shortcut and generating no further sentences —
    /// unless joining it opened a dimension. Only words that raise the rank
    /// generate, and the rank is at most `dim so(4,1) = 10` (78.68).
    ///
    /// Each pair is met once (the fresh word against those held before it),
    /// each a read of the stroke; a word whose pairs a spent stroke did not
    /// finish stays fresh for the next.
    pub fn grow(&mut self, budget: &mut crate::physics::Budget) {
        let fresh = std::mem::take(&mut self.fresh);
        let mut left: Vec<Vec<u8>> = Vec::new();
        let mut rest = fresh.as_slice();
        while let [key, later @ ..] = rest {
            rest = later;
            let Some((a, rung)) = self.held.get(key).map(|h| (h.word.clone(), h.rung)) else { continue };
            if !budget.spend() {
                left.push(key.clone());
                continue;
            }
            // Its generators on its own modes, joined into the algebra. A word
            // whose modes do not read has UNREAD generators, never none: it
            // joins nothing and composes nothing, since an unread bracket is
            // not a flat one (rule 11).
            let Some(gens_a) = crate::transducer::cech::flow_generators(&a, budget) else { continue };
            let before = self.basis.len();
            if let Ok(basis) = crate::transducer::cech::lie_close(&self.basis, &gens_a, budget) {
                self.basis = basis;
                TALLY[LEX_RANK].fetch_max(self.basis.len() as u64, Relaxed);
            }
            if let Some(h) = self.held.get_mut(key) {
                h.gens = Some(gens_a.clone());
            }
            // A WORD A BRACKET TAUGHT generates further sentences only where it
            // opened a dimension of the algebra; inside the span it is grammar
            // — held, derivable, generating nothing more (78.60). A word the
            // mind perceived always meets its lexicon.
            if rung > 1 && self.basis.len() <= before {
                note(GRAMMAR, 1);
                continue;
            }
            // Every held word but itself and the fresh words after it, which
            // meet this one when their own turn comes: one pair, once.
            let others: Vec<(Mv, Option<Vec<Mv>>)> = self
                .held
                .scoped()
                .filter(|(k, _)| k != key && !later.contains(k))
                .map(|(_, h)| (h.word.clone(), h.gens.clone()))
                .collect();
            let one = Mv::scalar(num_rational::BigRational::from_integer(1.into()));
            let mut finished = true;
            for (b, gens_b) in others {
                if !budget.spend() {
                    finished = false;
                    break;
                }
                note(COMMUTATORS, 1);
                // FLAT BY THE BRACKET: generators that commute make no curve.
                if let Some(gb) = &gens_b {
                    if commute(&gens_a, gb) {
                        continue;
                    }
                }
                let term = |w: Mv| super::compose::Term { word: w, transport: one.clone() };
                let terms = [term(a.clone()), term(b.clone()), term(a.reverse()), term(b.reverse())];
                let Some(s) = super::compose::compose(&terms, budget) else { continue };
                if is_identity(&s.omega) {
                    continue;
                }
                note(CURVED, 1);
                hear_into(self, &s);
            }
            if !finished {
                left.push(key.clone());
            }
        }
        left.extend(std::mem::take(&mut self.fresh));
        self.fresh = left;
    }
}

/// Whether every generator of one word commutes with every generator of
/// another: `B_a B_b − B_b B_a = 0`, the bracket read exactly.
fn commute(a: &[Mv], b: &[Mv]) -> bool {
    a.iter().all(|x| b.iter().all(|y| x.geo(y).sub(&y.geo(x)).is_zero()))
}

/// A motion's coordinates, as the lexicon keys it: its canonical ray.
fn key_of(m: &Mv) -> Option<Vec<u8>> {
    let mut k = Vec::new();
    m.canonical()?.put_canonical(&mut k);
    Some(k)
}

fn is_identity(m: &Mv) -> bool {
    m.projective_eq(&Mv::scalar(num_rational::BigRational::from_integer(1.into())))
}

/// THE RUNG OF A SENTENCE: one above the highest rung among its terms'
/// motions that the mind holds as shortcuts; rung 1 for a sentence of
/// perceived words alone. A term held as its inverse stands at that word's
/// rung (a commutator's back-steps are its words reversed).
fn rung_of(terms: &[Mv], held: &Keyed<Held>) -> u64 {
    let at = |m: &Mv| held.get(&key_of(m)?).or_else(|| held.get(&key_of(&m.reverse())?)).map(|h| h.rung);
    let below = terms.iter().filter_map(at).fold(0u64, u64::max);
    below.checked_add(1).map_or(below, |r| r)
}

/// 78.75 — A MIND SPEAKS ITS RINGS: each composed (`sentences_of`), each
/// closed one extrapolated to its theorem (`Theorem::of`) and its shortcut
/// held, recognized, or offered; each open one named by why. Returns the
/// compositions read, with the overlaps each crossed.
pub fn speak(
    form: &super::Form,
    rings: &[super::Orbit],
    budget: &mut crate::physics::Budget,
) -> Vec<(Vec<crate::network::nerve::coord::Overlap>, Composition)> {
    let mut out = Vec::new();
    for (steps, read) in super::compose::sentences_of(form, rings, budget) {
        note(RINGS, 1);
        let s = match read {
            Ok(s) => s,
            Err(RingUnread::Traversal | RingUnread::Transport) => {
                note(UNREAD, 1);
                continue;
            }
            // 78.97 — a chord's ring is the harmonic eye's, counted apart.
            Err(RingUnread::Heard) => {
                note(HEARD, 1);
                continue;
            }
        };
        hear(&s);
        out.push((steps, s));
    }
    // 78.98 — and the lexicon grows by the sentences its new words make.
    grow(budget);
    out
}

/// ONE COMPOSED SENTENCE, HEARD BY THE MIND: its closure named, its theorem
/// extrapolated where a certificate stands, and — closed — its rank read and
/// its shortcut recognized, held at its rung, or left for the court to offer.
pub fn hear(s: &Composition) {
    if let Ok(mut lex) = LEXICON.lock() {
        hear_into(&mut lex, s);
    }
}

/// [`hear`]'s body, on the lexicon the caller holds.
fn hear_into(lex: &mut Lexicon, s: &Composition) {
    match s.closure {
        Closure::Closes(_) => note(CLOSED, 1),
        Closure::Rosette => note(ROSETTE, 1),
        Closure::Unbound => note(UNBOUND, 1),
        Closure::Unread => note(OPEN_UNREAD, 1),
    }
    // 78.97 — THE WORD A THEOREM TEACHES: a returning sentence its open path (the
    // closing transport's inverse, one word for the whole path, 78.69); a
    // nilpotent one its holonomy itself, `Ω = 1 + N` — it never returns, so
    // what it says per traversal is the one null motion, counted by its
    // winding (78.68). Its open path is the identity wherever the slip rides
    // the closing transport alone.
    let taught = match Theorem::of(s) {
        Ok(Theorem { certificate: Certificate::Periodic { .. }, .. }) => {
            note(PERIODIC, 1);
            Some(&s.path)
        }
        Ok(Theorem { certificate: Certificate::Nilpotent { .. }, .. }) => {
            note(NILPOTENT, 1);
            Some(&s.omega)
        }
        Ok(Theorem { certificate: Certificate::Saturated { .. }, .. }) | Err(Open::Rosette | Open::Unbound | Open::Unread) => None,
    };
    let word = match (s.closure.closes(), taught) {
        (_, Some(w)) => w,
        (true, None) => &s.path,
        (false, None) => return,
    };
    if let Ok(r) = &s.rank {
        note(RANK, *r as u64);
    }
    hold_shortcut(lex, s, word);
}

/// ONE THEOREM'S SHORTCUT: the word it teaches, recognized where the mind
/// holds it, else held at its rung — spelled, or counted as the letter the
/// court will be offered when the mind speaks the form — and owed its
/// sentences with every held word (78.98).
fn hold_shortcut(lex: &mut Lexicon, s: &Composition, word: &Mv) {
    if is_identity(word) {
        return;
    }
    let Some(key) = key_of(word) else { return };
    if lex.held.get(&key).is_some() {
        note(RECOGNIZED, 1);
        return;
    }
    let rung = rung_of(&s.motions, &lex.held);
    let spelled = matches!(crate::transducer::family::derivation::peel_versor(word), Some(Ok(_)));
    note(if spelled { SPELLED } else { UNSPELLED }, 1);
    TALLY[TOP_RUNG].fetch_max(rung, Relaxed);
    lex.held.insert(&key, Held { rung, spelled, word: word.clone(), gens: None });
    lex.fresh.push(key);
}

/// 78.75 — A REFUSAL INVERTED IS A THEOREM: a white-hole source's sliver `1 +
/// N`, `N² = 0`, composed as a one-word sentence, extrapolates to the
/// nilpotent certificate `Ωᵐ ∝ 1 + m·N` — its ellipsis closed. Counted.
pub fn hold_sources(sources: &[Mv], budget: &mut crate::physics::Budget) {
    let one = Mv::scalar(num_rational::BigRational::from_integer(1.into()));
    for m in sources {
        let terms = [super::compose::Term { word: m.clone(), transport: one.clone() }];
        let Some(s) = super::compose::compose(&terms, budget) else { continue };
        if let Ok(Theorem { certificate: Certificate::Nilpotent { .. }, .. }) = Theorem::of(&s) {
            note(SOURCES, 1);
        }
    }
}
