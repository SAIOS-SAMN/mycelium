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
    /// 78.104 — THE HELD ALGEBRA: every held word's generators closed under
    /// the commutator (`cech::quotient::grow`), the algebra a new word is
    /// read against before it is held. At most `dim so(4,1)` = 10 at `n = 3`.
    algebra: Vec<Mv>,
}

static LEXICON: std::sync::LazyLock<std::sync::Mutex<Lexicon>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Lexicon::default()));

/// 78.100 D1 — THE WORDS NEWLY TAUGHT, each with the period its sentence
/// returns at (one where it never returns), waiting for the voice to
/// transcribe them into its language as it transcribes a closed thought
/// (`take_taught`). Drained whole, never read in place.
static TAUGHT: std::sync::LazyLock<std::sync::Mutex<Vec<Taught>>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

/// ONE TAUGHT WORD: the motion, and the period its sentence returns at.
#[derive(Clone, Debug)]
pub struct Taught {
    pub motion: Mv,
    pub period: u64,
}

const SLOTS: usize = 61;
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
const TAUGHT_SPELLED: usize = 23;
const TAUGHT_ADJOINED: usize = 24;
const TAUGHT_INEFFABLE: usize = 25;
const TAUGHT_UNREAD: usize = 26;
const UNREAD_TRANSPORT: usize = 27;
const COORDS_IRRATIONAL: usize = 28;
const COORDS_DEFECTIVE: usize = 29;
const COORDS_SPENT: usize = 30;
const COORDS_GENERATOR: usize = 31;
const UNREAD_NOT_RIGID: usize = 32;
const UNREAD_NO_BASE_ACTION: usize = 33;
const UNREAD_NOT_A_VERSOR: usize = 34;
const UNREAD_UNREFRACTED: usize = 35;
const CONTACT_STATIC: usize = 36;
const CONTACT_COSET: usize = 37;
const UNREAD_COSET_UNBOUNDED: usize = 38;
const EDGES_RELATIVE: usize = 39;
// 78.103 — the bends by kind: contact `CONTACT_BENDS + k`, relative edge
// `RELATIVE_BENDS + k`, `k` in `BEND_KINDS`' order.
const CONTACT_BENDS: usize = 40;
const RELATIVE_BENDS: usize = 44;
const BEND_KINDS: usize = 4;
const UNREAD_STAR: usize = 48;
const UNREAD_STEP: usize = 49;
const EDGE_UNSPELLED: usize = 50;
const EDGE_NO_READING: usize = 51;
const EDGE_TEAR: usize = 52;
const EDGE_NO_BODIES: usize = 53;
const OPEN_SPOKEN: usize = 54;
const EDGELESS_CONTACT: usize = 55;
const EDGELESS_PAIR: usize = 56;
const QUOTIENT_SPANNED: usize = 57;
const QUOTIENT_OUTSIDE: usize = 58;
const QUOTIENT_UNREAD: usize = 59;
const ALGEBRA_RANK: usize = 60;

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

/// One taught word's transcription, counted on the sentences row.
pub fn note_transcribed(t: Transcribed) {
    note(
        match t {
            Transcribed::Spelled => TAUGHT_SPELLED,
            Transcribed::Adjoined => TAUGHT_ADJOINED,
            Transcribed::Ineffable => TAUGHT_INEFFABLE,
            Transcribed::Unread => TAUGHT_UNREAD,
        },
        1,
    );
}

/// 78.100 D1 — THE WORDS TAUGHT SINCE THE LAST TAKE, each with its period
/// the voice transcribes, stands and speaks them.
pub fn take_taught() -> Vec<Taught> {
    TAUGHT.lock().map_or(Vec::new(), |mut held| std::mem::take(&mut *held))
}

fn note(slot: usize, by: u64) {
    if by > 0 && TALLY[slot].fetch_update(Relaxed, Relaxed, |held| held.checked_add(by)).is_err() {
        let mut past = u64::MAX;
        let _ = crate::physics::metabolism::step_or_refuse(&mut past, by);
    }
}

/// WHAT THE MIND'S SENTENCES DID, in this process.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SentenceTally {
    /// Rings composed, and rings whose terms did not read — by why (78.100):
    /// the traversal refused (`unread`), a crossing's transport unread.
    pub rings: u64,
    pub unread: u64,
    pub unread_transport: u64,
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
    /// 78.100 — the words whose modes did not read, by why: no action
    /// (`coords_unread`), a value irrational over ℚ, a defective split, the
    /// stroke spent reading the roots, a generator with no canonical ray.
    pub coords_unread: u64,
    pub coords_irrational: u64,
    pub coords_defective: u64,
    pub coords_spent: u64,
    pub coords_generator: u64,
    pub holes_turned: u64,
    pub holes_counted: u64,
    pub holes_named: u64,
    pub powers: u64,
    pub holes_new: u64,
    pub windings_new: u64,
    /// 78.100 D1 — the taught words' transcriptions into the language.
    pub taught_spelled: u64,
    pub taught_adjoined: u64,
    pub taught_ineffable: u64,
    pub taught_unread: u64,
}

/// 78.96, 78.97, 78.99 — THE TALLY'S NAMES, in [`SentenceTally::row`]'s
/// order: the typed reading the voice's Senses carry.
pub const SENTENCES: [&str; 32] = [
    "rings",
    "unread: traversal",
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
    "coordinates unread: no action",
    "holes turned",
    "holes counted",
    "holes named",
    "powers recognized",
    "holes new",
    "windings new",
    "taught: spelled and spoken",
    "taught: by an adjoined letter",
    "taught: ineffable",
    "taught: unread",
    "unread: transport",
    "coordinates unread: irrational",
    "coordinates unread: defective",
    "coordinates unread: stroke spent",
    "coordinates unread: generator",
];

/// 78.100 — WHY THE LANGUAGE LOSES WHAT IT PERCEIVES, a row of its own: a
/// ring's crossing unread by its reason (no transport read, no base action
/// of its word, no versor), and the mode reader's rational roots refused for
/// the stroke — how many, the highest degree, the widest coefficient in
/// bits (`cga::root_refusals`). On epoch 124 every unread ring was unread
/// by its transport, and every unread coordinate by the stroke spent.
pub const WHY_UNREAD: [&str; 32] = [
    "transport: member pair, no edge",
    "transport: unrefracted",
    "transport: no base action",
    "transport: not a versor",
    "roots refused",
    "roots refused: highest degree",
    "roots refused: widest coefficient (bits)",
    // 78.6, 78.102 — the contacts the master equation now reads by their
    // refraction, and a held coset whose routes the stroke ran out forming.
    "contacts read: static tether",
    "contacts read: held coset",
    "transport: coset unbounded",
    // 78.103 — the edges of two bodies that moved differentially, and every
    // bend by WHAT IT GENERATES: the planes its modes span by kind (a turn,
    // a null slide, a boost), never a count of axes.
    "edges: relative motion",
    "contact bends: turn planes",
    "contact bends: null planes",
    "contact bends: boost planes",
    "contact bends: unread",
    "relative bends: turn planes",
    "relative bends: null planes",
    "relative bends: boost planes",
    "relative bends: unread",
    // 78.103 — the faces that lost their edge, by the face.
    "transport: star face, no edge",
    "transport: frame step, no edge",
    // 78.103 — every edge refused at perception, by the reader that refused
    // it (per reading: each coset member refuses for its own reason).
    "edges lost: unspelled (a letter candidate)",
    "edges lost: no reading",
    "edges lost: tear",
    "edges lost: no bodies",
    // 78.102 — the letter carrier's voice half: perceived forms spoken as
    // open sentences because a word they taught no held letter spells.
    "letter carrier: open sentences spoken",
    // 78.103 — faces built carrying nothing, by the face.
    "faces edgeless: contact",
    "faces edgeless: member pair",
    // 78.104 — each new word read against the held algebra before it is
    // held: spanned by it, its generators outside it (counted), unread; and
    // the held algebra's rank (its highest).
    "quotient: spanned",
    "quotient: generators outside",
    "quotient: unread",
    "held algebra: rank",
];

/// 78.103 — WHERE A BEND WAS READ: a contact's refraction (its tether, or one
/// candidate of its held coset), or the relative motion of two bodies that
/// moved differentially.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Bend {
    Contact,
    Relative,
}

/// 78.103 — ONE BEND READ BY WHAT IT GENERATES: the plane it spans under its
/// kind — a turn (elliptic), a null slide (parabolic), a boost (hyperbolic);
/// the identity spans none. Read with no root and no stroke; a compound bend
/// is counted unread, by name.
pub(crate) fn note_bends(at: Bend, motion: &Mv) {
    let base = match at {
        Bend::Contact => CONTACT_BENDS,
        Bend::Relative => {
            note(EDGES_RELATIVE, 1);
            RELATIVE_BENDS
        }
    };
    // ONE MODE, READ WITH NO ROOT TAKEN: a versor of grades 0 and 2 alone has
    // `VṼ = a² − B·B − B∧B` scalar, so `B∧B = 0` — its bivector is simple, it
    // spans one plane, and the sign of `B²` is its kind exactly (negative a
    // turn, zero a null slide, positive a boost). The identity spans none.
    let b = motion.grade(2);
    if motion.sub(&motion.grade(0).add(&b)).is_zero() {
        use num_traits::Signed;
        if b.is_zero() {
            return;
        }
        let sq = b.geo(&b).scalar_part();
        let k = match (sq.is_negative(), sq.is_positive()) {
            (true, _) => 0,
            (_, true) => 2,
            _ => 1,
        };
        note(base + k, 1);
        return;
    }
    // A COMPOUND BEND (a grade-4 part, two planes) is counted unread by name
    // and spends nothing: its modes take roots, and the scene's one stroke is
    // comprehension's (epoch 129: `stroke spent` 0–5 while compound bends
    // took `motion_modes`). Its kinds wait for a root-free reading.
    note(base + BEND_KINDS - 1, 1);
}

/// 78.103 — WHY A MEMBER PAIR'S EDGE DID NOT FORM, by the reader that refused
/// it: the letters spell no rotor of it (37.7's candidate), no reading of its
/// transition, a tear, or no bodies to read it from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EdgeLost {
    Unspelled,
    NoReading,
    Tear,
    NoBodies,
}

/// 78.102 — one perceived form spoken as an open sentence by the letter
/// carrier's voice half.
pub fn note_open_spoken() {
    note(OPEN_SPOKEN, 1);
}

/// 78.103 — one face built with no edge and no refraction, by whether it is
/// a contact.
pub(crate) fn note_edgeless_face(contact: bool) {
    note(
        match contact {
            true => EDGELESS_CONTACT,
            false => EDGELESS_PAIR,
        },
        1,
    );
}

/// One member pair's edge lost, counted on the unread row by why.
pub(crate) fn note_edge_lost(why: EdgeLost) {
    note(
        match why {
            EdgeLost::Unspelled => EDGE_UNSPELLED,
            EdgeLost::NoReading => EDGE_NO_READING,
            EdgeLost::Tear => EDGE_TEAR,
            EdgeLost::NoBodies => EDGE_NO_BODIES,
        },
        1,
    );
}

/// The [`WHY_UNREAD`] row since the process began.
pub fn unread_tally() -> [u64; 32] {
    let r = |i: usize| TALLY[i].load(Relaxed);
    let (refused, degree, bits) = crate::transducer::cga::root_refusals();
    [
        r(UNREAD_NOT_RIGID),
        r(UNREAD_UNREFRACTED),
        r(UNREAD_NO_BASE_ACTION),
        r(UNREAD_NOT_A_VERSOR),
        refused,
        degree,
        bits,
        r(CONTACT_STATIC),
        r(CONTACT_COSET),
        r(UNREAD_COSET_UNBOUNDED),
        r(EDGES_RELATIVE),
        r(CONTACT_BENDS),
        r(CONTACT_BENDS + 1),
        r(CONTACT_BENDS + 2),
        r(CONTACT_BENDS + 3),
        r(RELATIVE_BENDS),
        r(RELATIVE_BENDS + 1),
        r(RELATIVE_BENDS + 2),
        r(RELATIVE_BENDS + 3),
        r(UNREAD_STAR),
        r(UNREAD_STEP),
        r(EDGE_UNSPELLED),
        r(EDGE_NO_READING),
        r(EDGE_TEAR),
        r(EDGE_NO_BODIES),
        r(OPEN_SPOKEN),
        r(EDGELESS_CONTACT),
        r(EDGELESS_PAIR),
        r(QUOTIENT_SPANNED),
        r(QUOTIENT_OUTSIDE),
        r(QUOTIENT_UNREAD),
        r(ALGEBRA_RANK),
    ]
}

/// 78.6, 78.102 — HOW A CONTACT WITH NO ONE TRANSPORT WAS READ by the master
/// equation: its static tether, or its held coset's candidates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ContactRead {
    Static,
    Coset,
}

/// One contact read by its refraction, counted on the unread row.
pub(crate) fn note_contact(read: ContactRead) {
    note(
        match read {
            ContactRead::Static => CONTACT_STATIC,
            ContactRead::Coset => CONTACT_COSET,
        },
        1,
    );
}

impl SentenceTally {
    /// In [`SENTENCES`]'s order.
    pub fn row(&self) -> [u64; 32] {
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
            self.taught_spelled,
            self.taught_adjoined,
            self.taught_ineffable,
            self.taught_unread,
            self.unread_transport,
            self.coords_irrational,
            self.coords_defective,
            self.coords_spent,
            self.coords_generator,
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
        taught_spelled: r(TAUGHT_SPELLED),
        taught_adjoined: r(TAUGHT_ADJOINED),
        taught_ineffable: r(TAUGHT_INEFFABLE),
        taught_unread: r(TAUGHT_UNREAD),
        unread_transport: r(UNREAD_TRANSPORT),
        coords_irrational: r(COORDS_IRRATIONAL),
        coords_defective: r(COORDS_DEFECTIVE),
        coords_spent: r(COORDS_SPENT),
        coords_generator: r(COORDS_GENERATOR),
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
    // 78.103 — the contacts' bends are read after every ring is heard.
    let (sentences, bends) = super::compose::sentences_and_bends_of(form, rings, budget);
    for (steps, read) in sentences {
        note(RINGS, 1);
        let s = match read {
            Ok(s) => s,
            // 78.100 — UNREAD BY WHY: the ring's traversal refused, or a
            // crossing's transport not read.
            Err(RingUnread::Traversal) => {
                note(UNREAD, 1);
                continue;
            }
            Err(RingUnread::Transport(why)) => {
                use super::compose::TermUnread;
                note(UNREAD_TRANSPORT, 1);
                note(
                    match why {
                        TermUnread::NotRigid => UNREAD_NOT_RIGID,
                        TermUnread::StarUnread => UNREAD_STAR,
                        TermUnread::StepUnread => UNREAD_STEP,
                        TermUnread::Unrefracted => UNREAD_UNREFRACTED,
                        TermUnread::NoBaseAction => UNREAD_NO_BASE_ACTION,
                        TermUnread::NotAVersor => UNREAD_NOT_A_VERSOR,
                        // `sentences_of` composes every route of a held coset;
                        // where it asks one sequence it is the stroke's limit.
                        TermUnread::HeldCoset | TermUnread::CosetUnbounded => UNREAD_COSET_UNBOUNDED,
                    },
                    1,
                );
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
    super::compose::read_bends(&bends);
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
    // 78.100 — BY WHY: no action to read, a mode's value irrational over ℚ,
    // a defective split, the stroke spent reading the roots, or a mode's
    // generator with no canonical ray.
    let coords = match Coordinates::of(word, budget) {
        Ok(c) => Some(c),
        Err(why) => {
            use super::coordinates::Unread;
            use crate::transducer::cga::Undecomposed;
            note(
                match why {
                    Unread::NoAction => COORDS_UNREAD,
                    Unread::Modes(Undecomposed::Irrational(_)) => COORDS_IRRATIONAL,
                    Unread::Modes(Undecomposed::Defective) => COORDS_DEFECTIVE,
                    Unread::Modes(Undecomposed::Unbounded(_)) => COORDS_SPENT,
                    Unread::Generator => COORDS_GENERATOR,
                },
                1,
            );
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
    // 78.104 — THE QUOTIENT READING: the word read against the algebra this
    // mind holds, before it is held — every generator spanned by it, a
    // composition of what is held; any outside it by its coset; or unread by
    // name. A static, one-way reading (never lift and drag, 78.46), and the
    // algebra reads the planes, not the rates.
    //
    // A READING, NEVER A GATE (epoch 132): the held algebra reached rank 10 —
    // all of so(4,1) — on every voice after three words outside it, and from
    // there every word is spanned and the reading says nothing. Gating the
    // letter carrier on it silenced the carrier (open sentences spoken 0,
    // ineffable 10–70 a voice). A word reaching here is already new — neither
    // recognized nor a power of a held word — so every ineffable one is sent.
    match crate::transducer::cech::quotient(&lex.algebra, word, budget) {
        Ok(r) if r.spanned() => note(QUOTIENT_SPANNED, 1),
        Ok(r) => note(QUOTIENT_OUTSIDE, r.outside.len() as u64),
        Err(_) => note(QUOTIENT_UNREAD, 1),
    }
    // And the held algebra grows by what is now held.
    if let Some(grown) = crate::transducer::cech::quotient::grow(&lex.algebra, word, budget) {
        lex.algebra = grown;
        TALLY[ALGEBRA_RANK].fetch_max(lex.algebra.len() as u64, Relaxed);
    }
    let rung = rung_of(&s.motions, &lex.held);
    let spelled = matches!(crate::transducer::family::derivation::peel_versor(word), Some(Ok(_)));
    note(if spelled { SPELLED } else { UNSPELLED }, 1);
    TALLY[TOP_RUNG].fetch_max(rung, Relaxed);
    if let Some(c) = &coords {
        lex.holes.file(&key, c);
    }
    lex.held.insert(&key, Held { rung, spelled, coords });
    // 78.100 D1 — AND IT IS HANDED TO THE VOICE, to be transcribed into its
    // language, stood and spoken: the period it returns at, one where it
    // never returns (a null or named word carries no return).
    let period = match s.closure {
        Closure::Closes(l) => l,
        Closure::Rosette | Closure::Unbound | Closure::Unread => 1,
    };
    if let Ok(mut taught) = TAUGHT.lock() {
        taught.push(Taught { motion: word.clone(), period });
    }
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
