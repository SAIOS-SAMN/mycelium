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
use super::coordinates::{Comprehended, Coordinates, HoleIndex, Winding};
use super::theorem::{Certificate, Open, Theorem};
use crate::network::nerve::Keyed;
use crate::transducer::cga::Mv;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

/// ONE SHORTCUT THE MIND HOLDS: its rung, whether the alphabet spells it,
/// and its coordinates — its holes and windings (78.99 (1)) — where its
/// modes read.
#[derive(Clone, Debug)]
struct Held {
    rung: u64,
    spelled: bool,
    coords: Option<Coordinates>,
}

/// THE MIND'S LEXICON: the words held by their keys, and the holes they
/// stand on — the fibre a new word is comprehended by (78.99 (1)).
#[derive(Default)]
struct Lexicon {
    held: Keyed<Held>,
    holes: HoleIndex,
}

static LEXICON: std::sync::LazyLock<std::sync::Mutex<Lexicon>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Lexicon::default()));

const SLOTS: usize = 23;
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
const NAMED: usize = 15;
const COORDS_UNREAD: usize = 16;
const HOLES_TURNED: usize = 17;
const HOLES_COUNTED: usize = 18;
const HOLES_NAMED: usize = 19;
const POWERS: usize = 20;
const HOLES_NEW: usize = 21;
const WINDINGS_NEW: usize = 22;

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
    /// 78.68, 78.99 — open sentences (hyperbolic, rosette) whose holonomy is
    /// named and held as a word.
    pub named: u64,
    /// 78.99 (1) — the coordinates of the words taught: words whose modes did
    /// not read; holes read as turns, counts and named points; words
    /// recognized as a held word's power through their windings; and, of the
    /// new words, the holes no held word stood on and the held holes at a
    /// winding no power reached.
    pub coords_unread: u64,
    pub holes_turned: u64,
    pub holes_counted: u64,
    pub holes_named: u64,
    pub powers: u64,
    pub holes_new: u64,
    pub windings_new: u64,
}

/// 78.96, 78.97, 78.99 — THE TALLY'S NAMES, in [`SentenceTally::row`]'s
/// order: the typed reading the voice's Senses carry.
pub const SENTENCES: [&str; 23] = [
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
    "named (hyperbolic, rosette)",
    "coordinates unread",
    "holes turned",
    "holes counted",
    "holes named",
    "powers recognized",
    "holes new",
    "windings new",
];

impl SentenceTally {
    /// In [`SENTENCES`]'s order.
    pub fn row(&self) -> [u64; 23] {
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
            self.named,
            self.coords_unread,
            self.holes_turned,
            self.holes_counted,
            self.holes_named,
            self.powers,
            self.holes_new,
            self.windings_new,
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
        named: r(NAMED),
        coords_unread: r(COORDS_UNREAD),
        holes_turned: r(HOLES_TURNED),
        holes_counted: r(HOLES_COUNTED),
        holes_named: r(HOLES_NAMED),
        powers: r(POWERS),
        holes_new: r(HOLES_NEW),
        windings_new: r(WINDINGS_NEW),
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
/// perceived words alone.
fn rung_of(terms: &[Mv], held: &Keyed<Held>) -> u64 {
    let below = terms.iter().filter_map(|m| held.get(&key_of(m)?).map(|h| h.rung)).fold(0u64, u64::max);
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
        hear(&s, budget);
        out.push((steps, s));
    }
    out
}

/// ONE COMPOSED SENTENCE, HEARD BY THE MIND: its closure named, its theorem
/// extrapolated where a certificate stands, and — closed — its rank read and
/// its shortcut recognized, held at its rung, or left for the court to offer.
pub fn hear(s: &Composition, budget: &mut crate::physics::Budget) {
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
    // 78.100 D2 — THE ONE TEACHING RULE (`compose::teaches`), each theorem
    // counted by its kind.
    let theorem = Theorem::of(s);
    match &theorem {
        Ok(Theorem { certificate: Certificate::Periodic { .. }, .. }) => note(PERIODIC, 1),
        Ok(Theorem { certificate: Certificate::Nilpotent { .. }, .. }) => note(NILPOTENT, 1),
        Err(Open::Rosette | Open::Unbound) => note(NAMED, 1),
        Ok(Theorem { certificate: Certificate::Saturated { .. }, .. }) | Err(Open::Unread) => {}
    }
    let Some(word) = super::compose::teaches(s, &theorem) else { return };
    if let Ok(r) = &s.rank {
        note(RANK, *r as u64);
    }
    if let Ok(mut lex) = LEXICON.lock() {
        hold_shortcut(&mut lex, s, word, budget);
    }
}

/// ONE THEOREM'S SHORTCUT: the word it teaches, recognized where the mind
/// holds it, else comprehended by its coordinates (78.99 (1)) — a held
/// word's power is derivable and never held twice — else held at its rung,
/// spelled or counted as the letter the court will be offered when the mind
/// speaks the form, and filed under its holes.
fn hold_shortcut(lex: &mut Lexicon, s: &Composition, word: &Mv, budget: &mut crate::physics::Budget) {
    if is_identity(word) {
        return;
    }
    let Some(key) = key_of(word) else { return };
    if lex.held.get(&key).is_some() {
        note(RECOGNIZED, 1);
        return;
    }
    // ITS COORDINATES, read off its own modes under the stroke; unread by
    // name where they do not read, and the word is held without them.
    let coords = match Coordinates::of(word, budget) {
        Ok(c) => Some(c),
        Err(_) => {
            note(COORDS_UNREAD, 1);
            None
        }
    };
    if let Some(c) = &coords {
        for h in &c.holes {
            note(
                match h.winding {
                    Winding::Turn { .. } => HOLES_TURNED,
                    Winding::Count(_) => HOLES_COUNTED,
                    Winding::Named { .. } => HOLES_NAMED,
                },
                1,
            );
        }
        let held = &lex.held;
        match lex.holes.comprehend(c, |k| held.get(k).and_then(|h| h.coords.as_ref())) {
            Comprehended::Power { .. } => {
                note(POWERS, 1);
                return;
            }
            Comprehended::New { holes, windings } => {
                note(HOLES_NEW, holes as u64);
                note(WINDINGS_NEW, windings as u64);
            }
        }
    }
    let rung = rung_of(&s.motions, &lex.held);
    let spelled = matches!(crate::transducer::family::derivation::peel_versor(word), Some(Ok(_)));
    note(if spelled { SPELLED } else { UNSPELLED }, 1);
    TALLY[TOP_RUNG].fetch_max(rung, Relaxed);
    if let Some(c) = &coords {
        lex.holes.file(&key, c);
    }
    lex.held.insert(&key, Held { rung, spelled, coords });
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
