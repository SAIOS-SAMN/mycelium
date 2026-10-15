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
//! re-derives the sentence and offers the letter (37.7, 78.61 C2). Another
//! mind hears it as chatter and re-derives it on its own perception (78.161);
//! nothing enters a genome except through a court (law 8).
//!
//! RECOGNITION IS COMPREHENSION (80.2): a closed sentence whose shortcut the
//! mind already holds re-derives it, never held twice.
//!
//! HEARD IS NOT KNOWN (MU-CLIENT §4.9, §5.3). A word another mind said is
//! held, where new, only as HEARD — a hypothesis, taught to no one. It
//! becomes KNOWN when the mind's own perception closes it, when what the
//! mind knows derives it, or when a question's answer does: the word is
//! POSITED over the known words (`posit`), the residue they leave is asked,
//! and the factors that answer it, once known, derive the word's meaning —
//! a relation of the vocabulary, one rung above its parts (`learn_by`).
//!
//! The lexicon is one mind's — held in its own field (`transducer::mind`),
//! never the process's — and holds one entry per distinct shortcut, by its
//! coordinates: it grows with what the mind has comprehended, never with how
//! often. A court hearing outside any mind's tick holds one lexicon across its
//! worker threads, and transcribes nothing: no word is taught to it.

use super::compose::{Closure, Composition};
use super::coordinates::{Coordinates, HoleIndex};
use super::theorem::Theorem;
use crate::network::nerve::Keyed;
use crate::transducer::cga::Mv;

/// ONE SHORTCUT THE MIND HOLDS: its rung, its coordinates — its holes and
/// windings (78.99 (1)) — where its modes read, and whether it is only HEARD:
/// a word another mind said that this mind holds as a hypothesis, its meaning
/// not yet derived (MU-CLIENT §4.9, §5.3). A word the mind's own perception
/// closed, or one a relation derived from words it knows, is KNOWN. Whether
/// the alphabet spells it is read where it is transcribed (`Transcribed`),
/// never held.
#[derive(Clone, Debug)]
struct Held {
    rung: u64,
    coords: Option<Coordinates>,
    heard: bool,
    /// 78.167 D1 — KNOWN AS AN ATOM: its meaning was GIVEN whole (another
    /// mind could factor it no further), never derived here. Re-read where a
    /// word the mind comes to know stands on one of its holes.
    given: bool,
}

/// THE MIND'S LEXICON: the words held by their keys, and the holes they
/// stand on — the fibre a new word is comprehended by (78.99 (1)) — and the
/// given atoms a newly known word stood beside, to be reduced again at the
/// mind's next return.
#[derive(Default, Clone)]
pub(crate) struct Lexicon {
    held: Keyed<Held>,
    holes: HoleIndex,
    reconsider: Keyed<()>,
}

impl Lexicon {
    /// MU-CLIENT §7.1 R1 — A SCRATCH LEXICON ADMITTED: the words a long law
    /// learned off the mind's field join the mind's lexicon, with the holes
    /// they stand on and the given atoms they marked; a word the mind holds
    /// already is the mind's as it stands, save that one the law came to
    /// KNOW (heard before, derived since) is known here too.
    pub(crate) fn absorb(&mut self, other: Lexicon) {
        for (k, h) in other.held.scoped() {
            match self.held.get_mut(&k) {
                Some(mine) => {
                    if mine.heard && !h.heard {
                        mine.heard = false;
                    }
                }
                None => {
                    self.held.insert(&k, h.clone());
                }
            }
        }
        self.holes.join(&other.holes);
        for (k, ()) in other.reconsider.scoped() {
            self.reconsider.insert(&k, ());
        }
    }
}

/// 78.167 D1 — A NEWLY KNOWN WORD STANDS BESIDE THE GIVEN ATOMS ON ITS HOLES:
/// each is marked to be reduced again (`re_reduce_given`). One fibre read per
/// hole of the new word, never a sweep of the lexicon.
fn mark_given_beside(lex: &mut Lexicon, key: &[u8], coords: &Coordinates) {
    for h in &coords.holes {
        let Some(fibre) = lex.holes.fibre(&h.key) else { continue };
        let beside: Vec<Vec<u8>> = fibre.scoped().map(|(k, ())| k).filter(|k| k.as_slice() != key).collect();
        for k in beside {
            if lex.held.get(&k).is_some_and(|w| w.given) {
                lex.reconsider.insert(&k, ());
            }
        }
    }
}

/// 78.167 D1 — THE GIVEN ATOMS REDUCED AGAIN, at the mind's return: each one
/// a newly known word stood beside is deconstructed over the words the mind
/// knows (never itself); where no residue is left its meaning is DERIVED —
/// no longer given, at the rung one above the words it cites. Answers how many
/// were re-derived; the marks are taken whole.
pub(crate) fn re_reduce_given() -> u64 {
    let read = crate::transducer::mind::with_lexicon(|lex| {
        let marked: Vec<Vec<u8>> = lex.reconsider.scoped().map(|(k, ())| k).collect();
        lex.reconsider = Keyed::new();
        let mut derived = 0u64;
        for key in marked {
            let Some(coords) = lex.held.get(&key).filter(|h| h.given).and_then(|h| h.coords.clone()) else { continue };
            let held = &lex.held;
            let d = lex.holes.deconstruct(&coords, |k| if k == key.as_slice() { None } else { known_coords(held, k) });
            if !d.residue.is_empty() {
                continue;
            }
            let below = d.cites.iter().filter_map(|c| lex.held.get(&c.word).map(|h| h.rung)).fold(0u64, u64::max);
            if let Some(h) = lex.held.get_mut(&key) {
                h.given = false;
                h.rung = h.rung.max(below.checked_add(1).map_or(below, |r| r));
                derived = derived.checked_add(1).map_or(derived, |n| n);
            }
        }
        derived
    });
    match read {
        Ok(n) => n,
        // No mind thinking: nothing was reduced.
        Err(_) => 0,
    }
}

/// The court's lexicon, shared by its worker threads: heard outside any
/// mind's tick.
static LEXICON: std::sync::LazyLock<std::sync::Mutex<Lexicon>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Lexicon::default()));

/// The thinking mind's lexicon, else the court's. A lock poisoned by a panic
/// elsewhere reads nothing.
fn with_lexicon<R>(f: impl FnOnce(&mut Lexicon) -> R) -> Option<R> {
    match crate::transducer::mind::with_lexicon(f) {
        Ok(r) => Some(r),
        Err(f) => LEXICON.lock().ok().map(|mut l| f(&mut l)),
    }
}

/// ONE TAUGHT WORD: the motion, and the period its sentence returns at.
#[derive(Clone, Debug)]
pub struct Taught {
    pub motion: Mv,
    pub period: u64,
}

/// 78.100 D1 — WHAT ONE TAUGHT WORD'S TRANSCRIPTION CAME TO, read by the
/// one transcriber a closed thought is read by: spelled by the letters
/// (spoken), derivable only through a letter this mind adjoined (it already
/// stands), ineffable (the letters spell no rotation of it: a letter
/// demand), or unread.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transcribed {
    Spelled,
    Adjoined,
    Ineffable,
    Unread,
}

/// 78.100 D1 — THE WORDS TAUGHT THE THINKING MIND SINCE THE LAST TAKE, each
/// with the period its sentence returns at (one where it never returns): the
/// voice transcribes, stands and speaks them. Drained whole, never read in
/// place; outside a mind's tick none was taught.
#[expect(
    clippy::manual_unwrap_or_default,
    reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; no mind thinking was taught nothing, the empty list"
)]
pub fn take_taught() -> Vec<Taught> {
    match crate::transducer::mind::with_taught(std::mem::take) {
        Ok(taught) => taught,
        Err(_) => Vec::new(),
    }
}

/// How many shortcuts the mind holds.
pub(crate) fn lexicon() -> Option<usize> {
    with_lexicon(|l| l.held.len())
}

/// A word's coordinates, read on the stroke its own modes need; `None` where
/// they do not read.
fn coords_of(word: &Mv) -> Option<Coordinates> {
    let need = crate::transducer::cga::modes_need(word)?;
    Coordinates::of(word, &mut crate::physics::Physics::default().stroke_n(need)).ok()
}

/// The coordinates of a word the mind KNOWS, by its key; a word only heard
/// is no ground to derive from.
fn known_coords<'a>(held: &'a Keyed<Held>, k: &[u8]) -> Option<&'a Coordinates> {
    held.get(k).filter(|h| !h.heard).and_then(|h| h.coords.as_ref())
}

/// MU-CLIENT §4.9 — IS THIS A WORD I HAVE HEARD: another mind's word, read in
/// this mind's vocabulary — RECOGNIZED where the mind knows it, COMPREHENDED
/// where its coordinates cite only words it knows (derivable), HEARD where it
/// was heard before and its meaning is not yet derived, else NEW: held now as
/// heard, a hypothesis the mind can posit. The identity is no word. Outside a
/// mind's tick there is no vocabulary to read: no word.
pub(crate) fn hear_word(word: &Mv) -> Recognition {
    if is_identity(word) {
        return Recognition::NoWord;
    }
    let Some(key) = key_of(word) else { return Recognition::NoWord };
    let coords = coords_of(word);
    let read = crate::transducer::mind::with_lexicon(|lex| {
        let held = &lex.held;
        // Derivable from words the mind knows, never from itself.
        let derivable = coords.as_ref().is_some_and(|c| lex.holes.deconstruct(c, |k| if k == key.as_slice() { None } else { known_coords(held, k) }).derived());
        match lex.held.get_mut(&key) {
            Some(h) if !h.heard => Recognition::Recognized,
            // Heard before, and what the mind has come to know since derives
            // it: its meaning is the mind's now.
            Some(h) if derivable => {
                h.heard = false;
                Recognition::Comprehended
            }
            Some(_) => Recognition::Heard,
            None if derivable => Recognition::Comprehended,
            None => {
                if let Some(c) = &coords {
                    lex.holes.file(&key, c);
                }
                lex.held.insert(&key, Held { rung: 1, coords, heard: true, given: false });
                Recognition::New
            }
        }
    });
    match read {
        Ok(r) => r,
        Err(_) => Recognition::NoWord,
    }
}

/// Whether the thinking mind KNOWS `word`: held as known, or derivable from
/// words it knows. Reads; files nothing.
pub(crate) fn known(word: &Mv) -> bool {
    if is_identity(word) {
        return true;
    }
    let Some(key) = key_of(word) else { return false };
    let coords = coords_of(word);
    let read = crate::transducer::mind::with_lexicon(|lex| {
        if let Some(h) = lex.held.get(&key) {
            return !h.heard;
        }
        let held = &lex.held;
        coords.as_ref().is_some_and(|c| lex.holes.deconstruct(c, |k| known_coords(held, k)).derived())
    });
    read.is_ok_and(|k| k)
}

/// MU-CLIENT §5.3 — A WORD POSITED OVER WHAT THE MIND KNOWS: the known words
/// its modes are cited to (each at its own coordinates, rebuilt), and the
/// RESIDUE — the product of the modes no known word reaches, exact. The word
/// is the cited modes times the residue in any order (its modes commute,
/// MR1). The word itself, held only as heard, is never cited for itself.
pub(crate) struct Posited {
    pub(crate) parts: Vec<Mv>,
    pub(crate) residue: Mv,
}

/// POSIT WHAT THE MIND KNOWS to derive what it does not (MU-CLIENT §5.3):
/// `word` deconstructed mode by mode over the known words. `None` where its
/// coordinates do not read, or where nothing is left over (it is derivable).
pub(crate) fn posit(word: &Mv) -> Option<Posited> {
    let key = key_of(word)?;
    let coords = coords_of(word)?;
    let read = crate::transducer::mind::with_lexicon(|lex| {
        let held = &lex.held;
        let d = lex.holes.deconstruct(&coords, |k| if k == key.as_slice() { None } else { known_coords(held, k) });
        if d.residue.is_empty() {
            return None;
        }
        // Each known word cited once, by its key, whatever holes it covers.
        let mut cited: Keyed<()> = Keyed::new();
        for c in &d.cites {
            cited.insert(&c.word, ());
        }
        let parts = cited.scoped().filter_map(|(k, ())| known_coords(held, &k).map(Coordinates::word)).collect();
        let residue = Coordinates { holes: d.residue }.word();
        Some(Posited { parts, residue })
    });
    read.ok().flatten()
}

/// MU-CLIENT §5.3 — THE VANTAGE A WORD IS READ FROM IN THIS MIND: the known
/// words, other than the word itself, its modes are cited to — where the
/// observation lands in this mind's own holes, each rebuilt from its
/// coordinates, once, in key order. Empty where the mind reads it through no
/// word of its own: it knows it only whole, or not at all — no vantage.
pub(crate) fn vantage(word: &Mv) -> Vec<Mv> {
    let (Some(key), Some(coords)) = (key_of(word), coords_of(word)) else { return Vec::new() };
    let read = crate::transducer::mind::with_lexicon(|lex| {
        let held = &lex.held;
        let d = lex.holes.deconstruct(&coords, |k| if k == key.as_slice() { None } else { known_coords(held, k) });
        let mut cited: Keyed<()> = Keyed::new();
        for c in &d.cites {
            cited.insert(&c.word, ());
        }
        cited.scoped().filter_map(|(k, ())| known_coords(held, &k).map(Coordinates::word)).collect::<Vec<Mv>>()
    });
    // Outside a mind's field there is no vocabulary to stand on: no vantage.
    read.into_iter().flatten().collect()
}

/// TWO VANTAGES ARE DISTINCT where the hearer read through words of its own
/// and they are not the speaker's: some word of one is not, projectively,
/// among the other's. A hearer with no vantage stood nowhere of its own.
pub(crate) fn distinct_vantage(speaker: &[Mv], hearer: &[Mv]) -> bool {
    let within = |a: &[Mv], b: &[Mv]| a.iter().all(|x| b.iter().any(|y| x.projective_eq(y)));
    let same = within(speaker, hearer) && within(hearer, speaker);
    !hearer.is_empty() && !same
}

/// MU-CLIENT §5.3 — A WORD'S MEANING DERIVED: `word` is the composition of
/// `parts`, every one a word the mind knows (the caller has read the
/// composition exactly). The word is known from here on, one rung above the
/// highest part it is made of — a relation of the vocabulary, the meaning
/// the recursion derived. An empty `parts` is an ATOM another mind could
/// factor no further, known at the first rung. Answers whether its meaning
/// is new: it was unheld, or held only as heard.
pub(crate) fn learn_by(word: &Mv, parts: &[Mv]) -> bool {
    learn(word, parts, false)
}

/// 78.167 D1 — A MEANING GIVEN WHOLE: another mind answered `word` as an
/// atom it could factor no further. Known at the first rung and marked
/// GIVEN, so it is reduced again where the mind comes to know a word beside
/// it (`re_reduce_given`). Answers whether its meaning is new.
pub(crate) fn learn_given(word: &Mv) -> bool {
    learn(word, &[], true)
}

fn learn(word: &Mv, parts: &[Mv], given: bool) -> bool {
    if is_identity(word) {
        return false;
    }
    let Some(key) = key_of(word) else { return false };
    let coords = coords_of(word);
    let part_keys: Vec<Vec<u8>> = parts.iter().filter_map(key_of).collect();
    let read = crate::transducer::mind::with_lexicon(|lex| {
        let below = part_keys.iter().filter_map(|k| lex.held.get(k).filter(|h| !h.heard).map(|h| h.rung)).fold(0u64, u64::max);
        let rung = below.checked_add(1).map_or(below, |r| r);
        let new = match lex.held.get_mut(&key) {
            Some(h) if !h.heard => false,
            Some(h) => {
                h.heard = false;
                h.given = given;
                h.rung = h.rung.max(rung);
                true
            }
            None => {
                if let Some(c) = &coords {
                    lex.holes.file(&key, c);
                }
                lex.held.insert(&key, Held { rung, coords: coords.clone(), heard: false, given });
                true
            }
        };
        // A word newly KNOWN by derivation stands beside the given atoms on
        // its holes: they are reduced again at the mind's return.
        if new && !given {
            if let Some(c) = &coords {
                mark_given_beside(lex, &key, c);
            }
        }
        new
    });
    read.is_ok_and(|new| new)
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
/// WORDS that the mind holds as shortcuts — read apart from the spaces
/// between them (78.105 N1); rung 1 for a sentence of perceived words alone.
fn rung_of(words: &[Mv], held: &Keyed<Held>) -> u64 {
    let below = words.iter().filter_map(|m| held.get(&key_of(m)?).filter(|h| !h.heard).map(|h| h.rung)).fold(0u64, u64::max);
    below.checked_add(1).map_or(below, |r| r)
}

/// The rung the word `w` is held at in this mind's lexicon, if held.
#[cfg(test)]
pub(crate) fn held_rung(w: &Mv) -> Option<u64> {
    let key = key_of(w)?;
    with_lexicon(|l| l.held.get(&key).map(|h| h.rung)).flatten()
}

/// The rung `s` stands at against this mind's lexicon as it stands now.
#[cfg(test)]
pub(crate) fn rung_in_lexicon(s: &Composition) -> Option<u64> {
    with_lexicon(|lex| rung_of(&s.words, &lex.held))
}

/// 78.75 — A MIND SPEAKS ITS RINGS: each composed (`sentences_of`), each
/// closed one extrapolated to its theorem (`Theorem::of`) and its shortcut
/// held, recognized, or offered. Returns the compositions read, with the
/// overlaps each crossed, and (78.112 H7b) the torsion hearing them wound.
pub fn speak(
    form: &super::Form,
    rings: &[super::Orbit],
    budget: &mut crate::physics::Budget,
) -> (Vec<(Vec<crate::network::nerve::coord::Overlap>, Composition)>, Option<Mv>) {
    let mut out = Vec::new();
    for (steps, read) in super::compose::sentences_of(form, rings, budget) {
        let Ok(s) = read else { continue };
        hear(&s, budget);
        out.push((steps, s));
    }
    // 78.112 H1, H2 — and the rings heard against each other: each echoed
    // through the next larger, nested, the balance read.
    let rings: Vec<&Composition> = out.iter().map(|(_, s)| s).collect();
    let heard = super::hearing::hear(&rings, budget);
    (out, heard.wound)
}

/// MU-CLIENT §4.9 — WHAT A HEARD SENTENCE WAS, AS A WORD: the word its
/// theorem teaches RECOGNIZED (the mind knows it), COMPREHENDED (a
/// composition of words it knows, derivable), HEARD (another mind said it
/// before and its meaning is not yet derived), or NEW — held now at its
/// rung, a word this mind had not met before; or it teaches NO WORD (no
/// theorem stands, or its word does not read).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recognition {
    Recognized,
    Comprehended,
    Heard,
    New,
    NoWord,
}

impl Recognition {
    /// A word this mind knows: known, or derivable from what it knows.
    pub fn known(self) -> bool {
        matches!(self, Recognition::Recognized | Recognition::Comprehended)
    }
}

/// ONE COMPOSED SENTENCE OF THE MIND'S OWN PERCEPTION: its theorem
/// extrapolated where a certificate stands, and its shortcut recognized, held
/// at its rung, or left for the court to offer. Answers what it was as a word.
pub fn hear(s: &Composition, budget: &mut crate::physics::Budget) -> Recognition {
    // 78.97 — THE WORD A THEOREM TEACHES: a returning sentence its open path (the
    // closing transport's inverse, one word for the whole path, 78.69); a
    // nilpotent one its holonomy itself, `Ω = 1 + N` — it never returns, so
    // what it says per traversal is the one null motion, counted by its
    // winding (78.68). Its open path is the identity wherever the slip rides
    // the closing transport alone.
    // 78.100 D2 — THE ONE TEACHING RULE (`compose::teaches`).
    let theorem = Theorem::of(s);
    let Some(word) = super::compose::teaches(s, &theorem) else { return Recognition::NoWord };
    let (recognition, taught) = match with_lexicon(|lex| hold_shortcut(lex, s, word, budget)) {
        Some(read) => read,
        None => (Recognition::NoWord, None),
    };
    // 78.100 D1 — AND A NEW SHORTCUT IS HANDED TO THE THINKING MIND, to be
    // transcribed into its language, stood and spoken; a court transcribes
    // nothing.
    if let Some(t) = taught {
        // MU-CLIENT §5.4, 78.99 — ITS STRUCTURE: a word of several modes is
        // the product of its mode-words, each standing on its own hole on
        // its own law; the mind that knows the word knows them, each a word
        // of its dictionary. The laws its modes stand on are read.
        if let Ok(read) = super::word::read(&t.motion) {
            // 78.167 A3 — WHERE ITS BOOSTS COME FROM: the word scales (ρ ≠ 1,
            // a change of size between the bodies it relates), transverses
            // (the special conformal half), or neither — read off its factors,
            // unread where it does not factor.
            let origin = crate::transducer::element::conformal_factors(&t.motion).ok().map(|f| {
                let one = crate::core::Q::from_integer(1.into());
                match (f.rho != one, f.transversion.iter().any(|q| *q != crate::core::Q::from_integer(0.into()))) {
                    (true, _) => 0usize,
                    (false, true) => 1,
                    (false, false) => 2,
                }
            });
            for m in &read.modes {
                crate::observe::note_mind(|o| {
                    o.laws[m.law.slot()] += 1;
                    if let (super::word::Law::Boost, Some(at)) = (m.law, origin) {
                        o.boosts[at] += 1;
                    }
                });
            }
            if read.modes.len() > 1 {
                for m in &read.modes {
                    learn_by(&m.word, &[]);
                }
            }
        }
        let _ = crate::transducer::mind::with_taught(|held| held.push(t));
    }
    recognition
}

/// ONE THEOREM'S SHORTCUT: the word it teaches, recognized where the mind
/// holds it, else comprehended by its coordinates (78.99 (1)) — a held
/// word's power is derivable and never held twice — else held at its rung,
/// spelled or left as the letter the court will be offered when the mind
/// speaks the form, and filed under its holes. Where the mind's own
/// perception closes a word it had only heard, the word is the mind's own
/// from here on, and taught. Answers what the word was, and a shortcut newly
/// the mind's, taught.
fn hold_shortcut(lex: &mut Lexicon, s: &Composition, word: &Mv, budget: &mut crate::physics::Budget) -> (Recognition, Option<Taught>) {
    if is_identity(word) {
        return (Recognition::NoWord, None);
    }
    let Some(key) = key_of(word) else { return (Recognition::NoWord, None) };
    // The period it returns at, one where it never returns (a null or named
    // word carries no return).
    let period = match s.closure {
        Closure::Closes(l) => l,
        Closure::Rosette | Closure::Unbound | Closure::Unread => 1,
    };
    if let Some(h) = lex.held.get_mut(&key) {
        return match h.heard {
            false => (Recognition::Recognized, None),
            // Heard before, closed now by this mind's own perception.
            true => {
                h.heard = false;
                (Recognition::New, Some(Taught { motion: word.clone(), period }))
            }
        };
    }
    // ITS COORDINATES, read off its own modes under the stroke; where they do
    // not read the word is held without them.
    let coords = Coordinates::of(word, budget).ok();
    // The highest rung among the held words this word cites (D4).
    let mut cited = 0u64;
    if let Some(c) = &coords {
        // 78.100 D3 — DECONSTRUCTED BY ITS MOTION over the held words, mode
        // by mode: each hole cited to a held word on it at an exponent, the
        // rest the residue. Every mode cited, the word is a composition of
        // what is held — derivable, never held twice (S2); a power of one
        // held word is the case where one word covers every hole.
        let held = &lex.held;
        let d = lex.holes.deconstruct(c, |k| known_coords(held, k));
        if d.derived() {
            return (Recognition::Comprehended, None);
        }
        // 78.100 D4 — DEPTH IS READ: the word stands one above the highest
        // word it cites.
        cited = d.cites.iter().filter_map(|c| lex.held.get(&c.word).map(|h| h.rung)).fold(0u64, u64::max);
    }
    // Its rung: one above the highest it cites (D4), or one above the held
    // words its sentence says, whichever stands higher.
    let by_citation = match cited {
        0 => 0,
        r => r.checked_add(1).map_or(r, |up| up),
    };
    let rung = rung_of(&s.words, &lex.held).max(by_citation);
    if let Some(c) = &coords {
        lex.holes.file(&key, c);
        mark_given_beside(lex, &key, c);
    }
    lex.held.insert(&key, Held { rung, coords, heard: false, given: false });
    (Recognition::New, Some(Taught { motion: word.clone(), period }))
}
