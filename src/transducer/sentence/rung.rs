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

/// ONE SHORTCUT THE MIND HOLDS: its rung, and its coordinates — its holes
/// and windings (78.99 (1)) — where its modes read. Whether the alphabet
/// spells it is read where it is transcribed (`Transcribed`), never held.
#[derive(Clone, Debug)]
struct Held {
    rung: u64,
    coords: Option<Coordinates>,
    /// The shortcut's own motion: the ring a heard ring is echoed against
    /// where it seats in this one's layer (MU-CLIENT §4.9).
    word: Mv,
}

/// THE MIND'S LEXICON: the words held by their keys, and the holes they
/// stand on — the fibre a new word is comprehended by (78.99 (1)).
#[derive(Default)]
pub(crate) struct Lexicon {
    held: Keyed<Held>,
    holes: HoleIndex,
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

/// MU-CLIENT §4.9 — A HEARD RING UNDERSTOOD AGAINST THIS MIND'S OWN RINGS:
/// the shortcuts its closed sentences taught it, by size — each its return,
/// the carrier where it never returns (`hearing::Size`) — layered in the
/// order sizes nest, and the ring seated in the next larger layer it divides
/// (`hearing::hear_across`): WHOLE where it resonates with a held ring,
/// TILTED with its echoes, OUTER where no held ring is larger, LEFTOVER where
/// it divides none — not understood. Read in a mind's tick only: a court's
/// lexicon is not a mind's understanding, and outside one there is none.
/// Answers the seat and the held rings of the layer it seated in, by their
/// motions — the hearer's re-derivation. `None` where no mind is thinking or
/// the ring's modes do not read.
pub(crate) fn seat(ring: &Mv) -> Option<(super::hearing::Echoed, Vec<Mv>)> {
    use super::hearing::{hear_across, Echoed, Seat, Size};
    if super::presentation::is_still(ring) {
        return Some((Echoed { seat: Seat::Whole, carried: Some(ring.clone()), through: Vec::new() }, Vec::new()));
    }
    let need = crate::transducer::cga::modes_need(ring)?;
    let coords = Coordinates::of(ring, &mut crate::physics::Physics::default().stroke_n(need)).ok()?;
    let size = Size::of_coordinates(&coords)?;
    crate::transducer::mind::with_lexicon(|lex| {
        // The held rings by their size's coordinate, which orders them as
        // sizes nest.
        let mut layers: Keyed<Vec<(Vec<u8>, Mv)>> = Keyed::new();
        for (k, h) in lex.held.scoped() {
            let Some(s) = h.coords.as_ref().and_then(Size::of_coordinates) else { continue };
            let at = s.key();
            match layers.get_mut(&at) {
                Some(members) => members.push((k, h.word.clone())),
                None => {
                    layers.insert(&at, vec![(k, h.word.clone())]);
                }
            }
        }
        let read: Vec<(Size, Vec<(Vec<u8>, Mv)>)> =
            layers.scoped().filter_map(|(at, members)| Some((Size::from_key(&at)?, members.clone()))).collect();
        let echoed =
            hear_across(ring, size, read.iter().map(|(s, members)| (*s, members.iter().map(|(k, m)| (k.clone(), m)).collect())));
        let through: Vec<Mv> = echoed.through.iter().filter_map(|k| lex.held.get(k).map(|h| h.word.clone())).collect();
        (echoed, through)
    })
    .ok()
}

/// MU-CLIENT §5.3 — AN ATOM LEARNED: a word another mind could factor no
/// further, held in this mind's lexicon at the first rung, by its own
/// coordinates — so a ring of it seats from here on. A word held already is
/// held once. Answers whether it was newly held; outside a mind's tick
/// nothing is learned.
pub(crate) fn learn(word: &Mv) -> bool {
    if is_identity(word) {
        return false;
    }
    let Some(key) = key_of(word) else { return false };
    let coords = crate::transducer::cga::modes_need(word)
        .and_then(|need| Coordinates::of(word, &mut crate::physics::Physics::default().stroke_n(need)).ok());
    crate::transducer::mind::with_lexicon(|lex| {
        if lex.held.get(&key).is_some() {
            return false;
        }
        if let Some(c) = &coords {
            lex.holes.file(&key, c);
        }
        lex.held.insert(&key, Held { rung: 1, coords, word: word.clone() });
        true
    })
    .is_ok_and(|learned| learned)
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
    let below = words.iter().filter_map(|m| held.get(&key_of(m)?).map(|h| h.rung)).fold(0u64, u64::max);
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

/// ONE COMPOSED SENTENCE, HEARD BY THE MIND: its theorem extrapolated where a
/// certificate stands, and its shortcut recognized, held at its rung, or left
/// for the court to offer.
pub fn hear(s: &Composition, budget: &mut crate::physics::Budget) {
    // 78.97 — THE WORD A THEOREM TEACHES: a returning sentence its open path (the
    // closing transport's inverse, one word for the whole path, 78.69); a
    // nilpotent one its holonomy itself, `Ω = 1 + N` — it never returns, so
    // what it says per traversal is the one null motion, counted by its
    // winding (78.68). Its open path is the identity wherever the slip rides
    // the closing transport alone.
    // 78.100 D2 — THE ONE TEACHING RULE (`compose::teaches`).
    let theorem = Theorem::of(s);
    let Some(word) = super::compose::teaches(s, &theorem) else { return };
    let taught = with_lexicon(|lex| hold_shortcut(lex, s, word, budget)).flatten();
    // 78.100 D1 — AND A NEW SHORTCUT IS HANDED TO THE THINKING MIND, to be
    // transcribed into its language, stood and spoken; a court transcribes
    // nothing.
    if let Some(t) = taught {
        let _ = crate::transducer::mind::with_taught(|held| held.push(t));
    }
}

/// ONE THEOREM'S SHORTCUT: the word it teaches, recognized where the mind
/// holds it, else comprehended by its coordinates (78.99 (1)) — a held
/// word's power is derivable and never held twice — else held at its rung,
/// spelled or left as the letter the court will be offered when the mind
/// speaks the form, and filed under its holes. A shortcut newly held is
/// returned, taught.
fn hold_shortcut(lex: &mut Lexicon, s: &Composition, word: &Mv, budget: &mut crate::physics::Budget) -> Option<Taught> {
    if is_identity(word) {
        return None;
    }
    let key = key_of(word)?;
    if lex.held.get(&key).is_some() {
        return None;
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
        let d = lex.holes.deconstruct(c, |k| held.get(k).and_then(|h| h.coords.as_ref()));
        if d.derived() {
            return None;
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
    }
    lex.held.insert(&key, Held { rung, coords, word: word.clone() });
    // The period it returns at, one where it never returns (a null or named
    // word carries no return).
    let period = match s.closure {
        Closure::Closes(l) => l,
        Closure::Rosette | Closure::Unbound | Closure::Unread => 1,
    };
    Some(Taught { motion: word.clone(), period })
}
