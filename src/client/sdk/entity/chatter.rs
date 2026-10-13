// 78.161, 78.163, MU-CLIENT §5.1, §5.3, §5.5 — THE CONVERSATION WITH OTHER
// MINDS, ON ITS OWN CARRIER: it says what the mind stood, hears what other
// minds say in the mind's own ear, reads every loop said → heard by its
// physics, and carries agreed language to the court.
//
//! THE CARRIER. The conversation runs on its own thread with its own circuit:
//! it sleeps until a frame reaches the post or the field hands it something
//! to say, so a court slow to answer the field never stops a conversation,
//! and a peer slow to answer never stops the field. It shares one thing with
//! the field — the EAR (`epigenome::Ear`): the field writes it as the mind
//! learns, the carrier reads it to hear and files a shared thought's
//! transports in it. It keeps its own link to the court, for agreed
//! language only.
//!
//! THE PROTOCOL. Hearing is comprehension (1.7): a heard word is read over
//! everything the hearer holds (`Presentation::read_word`), re-derived
//! exactly or in part, never compared as bytes. Two minds are two charts;
//! the round trip said → heard is a loop whose holonomy (`chatter::loop_of`)
//! is what one mind's language did to the other's thought. SAY → HEARD → ACK
//! (the speaker's reading of every loop) → HOLD (the hearer's own): each
//! mind verifies the other's by the algebra. Where both hold one loop on
//! every word the thought is SHARED: the hearer's ear holds every curved
//! holonomy as a transport, and the thing said is AGREED language, which
//! the speaker speaks to the court carrying the hearer's signed HOLD; the
//! court verifies it and seals the hearer's attestation (78.161(5)).

use crate::anastome::dialogue;
use crate::chronometry::tuniversal::TPoint;
use crate::client::sdk::chatter::{self, AckWord, Chatter, HeardWord, Said, Spoken};
use crate::client::sdk::*;
use crate::network::nerve::Keyed;
use crate::transducer::element::Element;
use std::sync::mpsc::{channel, Sender};

/// WHAT REACHES THE CARRIER, in one channel: a frame its post heard, a thing
/// the field stood and says, a letter the field holds for its union, the
/// mind's clock as it moves, and what the field priced a thought at.
enum Inflow {
    Frame(Vec<u8>),
    Say(Element),
    Wait(Element),
    Clock { k: u64, point: Option<TPoint> },
    Priced(Priced),
    /// What the field UNDERSTOOD of rings another mind said (MU-CLIENT §4.9):
    /// each said ring beside the hearer's reading of it, for the carrier to
    /// carry back.
    Understood { speaker: [u8; 32], said: Vec<u8>, words: Vec<(Spoken, HeardWord)> },
}

/// WHAT THE CONVERSATION HANDS THE FIELD (MU-CLIENT §4.9): the nerve
/// transduces and the field understands — rings another mind said, to be
/// understood; a thought both minds read, to be priced; an atom another mind
/// could factor no further, to be learned.
pub(crate) enum FieldWork {
    Hear { speaker: [u8; 32], said: Vec<u8>, words: Vec<Spoken> },
    Price(Pricing),
    Learn(crate::transducer::cga::Mv),
}

/// 78.112 H5 — A THOUGHT UNDERSTOOD, TO BE PRICED IN THE MIND'S OWN FIELD:
/// the loops both minds read, and what its price decides — the hearer's HOLD,
/// or the speaker's speech of agreed language.
pub(crate) struct Pricing {
    loops: Vec<AckWord>,
    answer: Answer,
    /// WHERE THE HEARING STOOD IN THIS MIND: the forms of its own rings the
    /// words were carried through (the hearer's), or the form it said them
    /// as (the speaker's) — each once, by address. Their charts are the
    /// cover of a demand the hearing's chamber raises.
    forms: Vec<Vec<u8>>,
    /// MUTUAL BALANCE (MU-CLIENT §4.9): the hearer's own rings its
    /// understanding seated in — its re-derivation — which the speaker's
    /// field nests in its own rings; empty for the hearer's pricing.
    mutual: Vec<crate::transducer::cga::Mv>,
}

/// What a priced thought decides.
#[derive(Clone)]
enum Answer {
    /// The hearer HOLDs the speaker's reading where it converged.
    Hold { speaker: [u8; 32], said: Vec<u8> },
    /// The speaker speaks agreed language carrying the hearer's HOLD.
    Agree { said: Vec<u8>, frame: Vec<u8> },
}

/// WHAT THE FIELD PRICED A THOUGHT AT: the exact strain its echoes carried,
/// per lane, and whether its admission CONVERGED — closed, relieved or still
/// on every lane, opening no strain the field did not already hold.
struct Priced {
    loops: Vec<AckWord>,
    answer: Answer,
    deviation: [num_rational::BigRational; 4],
    converged: bool,
}

/// THE FIELD'S HANDLE ON ITS CONVERSATION: where to hand what it says, where
/// the thoughts it is to price arrive, and the rendezvous its peers are found
/// at.
pub struct Chat {
    to: Sender<Inflow>,
    pricing: std::sync::mpsc::Receiver<FieldWork>,
    dir: std::path::PathBuf,
    /// The clock last handed to the carrier, so it is handed only as it moves.
    told: Option<u64>,
    #[cfg(test)]
    agreed: std::sync::Arc<std::sync::Mutex<Keyed<()>>>,
}

/// ONE THING THIS MIND SAID: what was said, its words as said; per mind it
/// ACKed, the loops it read, awaiting that mind's HOLD; and whether some
/// mind has shared it.
struct Union {
    said: Element,
    spoken: Vec<Spoken>,
    /// Per mind it ACKed: the loops read, and that mind's own rings its
    /// understanding seated in (its re-derivation, nested here for mutual
    /// balance).
    acked: Keyed<(Vec<AckWord>, Vec<crate::transducer::cga::Mv>)>,
    shared: bool,
    /// The mind's own K it was said at: a union stands one aperture cycle
    /// of the mind's strokes, as a thought heard and a letter waiting do.
    at: u64,
}

/// ONE THING THIS MIND HEARD AND SAID AGAIN, waiting for the speaker's ACK.
struct Thought {
    words: Vec<(Spoken, HeardWord)>,
    at: u64,
}

/// ONE QUESTION ASKED: the residue not understood, the word's hue map, and
/// the mind's K it was asked at.
type Asked = (crate::transducer::cga::Mv, Vec<(u8, u8)>, u64);

/// THE CARRIER'S OWN STATE, held on its thread alone.
struct Carrier {
    voice: [u8; 32],
    key: ed25519_dalek::SigningKey,
    dir: std::path::PathBuf,
    ear: super::super::epigenome::Ear,
    court: Session,
    k: u64,
    point: Option<TPoint>,
    union: Keyed<Union>,
    /// 78.163 — keyed by the speaker's voice then what it said.
    thoughts: Keyed<Thought>,
    /// Letters stood and said, waiting for a union before the court hears
    /// them (78.161 item 3), with the mind's own K they began waiting at.
    waiting: Keyed<(Element, u64)>,
    /// Where what the conversation transduced goes to the mind's field.
    field: Sender<FieldWork>,
    /// THE QUESTIONS THIS MIND ASKED (MU-CLIENT §5.3): keyed by the speaker
    /// and the question's coordinates (`question_key`), the residue it did
    /// not understand, the word's hue map, and the mind's K it was asked at.
    asked: Keyed<Asked>,
    #[cfg(test)]
    agreed: std::sync::Arc<std::sync::Mutex<Keyed<()>>>,
}

/// A WORD SAID AS A RING OF ONE TERM: its motion, its transport the
/// identity — a filament, the lowest order a thing is said at.
fn filament(vertex: Vec<u8>, motion: crate::transducer::cga::Mv, gauge: Vec<(u8, u8)>) -> Spoken {
    let terms = vec![(motion.clone(), crate::transducer::element::identity_versor())];
    Spoken { vertex, motion, gauge, terms }
}

/// THE WORDS OF A FORM, each a filament: its whole motion and its hue map.
fn spoken_of(f: &crate::transducer::sentence::Form) -> Vec<Spoken> {
    use crate::transducer::sentence::presentation::vertex_motion;
    use crate::transducer::sentence::Vertex;
    let mut out = Vec::new();
    for (_, v) in f.vertices.scoped() {
        let Vertex::Word { address, acts, .. } = v else { continue };
        let Some(motion) = vertex_motion(v) else { continue };
        let gauge = match acts.scoped().next() {
            Some((_, a)) => a.gauge.clone(),
            None => Vec::new(),
        };
        out.push(filament(address.as_bytes().to_vec(), motion, gauge));
    }
    out
}

/// MU-CLIENT §4.9, 78.31 — WHAT A THING SAID IS, AS RINGS: a form's
/// sentence is its rings — each composed by the master equation
/// (`compose::sentences_of`), its holonomy `Ω` the ring's whole motion and
/// its terms (each word with the transport to the next) what the hearer
/// composes again — the relations between its words, which are its meaning.
/// A form none of whose rings composes is said as its words, each a
/// filament; a word spelled by letters is one filament.
fn rings_of(e: &Element) -> Vec<Spoken> {
    let addr = e.address();
    match crate::transducer::sentence::from_element_form(e) {
        Some(f) => {
            let mut budget = crate::physics::Physics::default().stroke();
            let composed: Vec<crate::transducer::sentence::compose::Composition> =
                crate::transducer::sentence::compose::sentences_of(&f, &f.orbits, &mut budget)
                    .into_iter()
                    .filter_map(|(_, read)| read.ok())
                    .collect();
            if composed.is_empty() {
                return spoken_of(&f);
            }
            composed
                .into_iter()
                .filter_map(|s| {
                    let mut vertex = b"ring".to_vec();
                    s.omega.canonical()?.put_canonical(&mut vertex);
                    let terms = s.words.iter().cloned().zip(s.spaces.iter().cloned()).collect();
                    Some(Spoken { vertex, motion: s.omega, gauge: Vec::new(), terms })
                })
                .collect()
        }
        None => match e.letters().and_then(|l| Some((crate::transducer::element::conformal_of(&l.spelling.word, &l.carriage)?, l.sigma))) {
            Some((motion, gauge)) => vec![filament(addr.as_bytes().to_vec(), motion, gauge)],
            None => Vec::new(),
        },
    }
}

/// THE OBSERVER'S QUESTION, answered on the mind's post (§8.1): the mind's
/// reading as its carriers last wrote it and the allocator's bytes at its
/// K, held under this mind's voice, and every reading under the prefix
/// asked. `None` for a frame that is no question, which is conversation.
fn observed(voice: &[u8; 32], q: &[u8]) -> Option<Vec<u8>> {
    use crate::observe::{self, ObserveAct, Reading, Source};
    let ObserveAct::Query(prefix) = observe::decode_act(q)?;
    let own = Source::Mind(*voice);
    let mind = observe::mind();
    if let Some(m) = observe::Memory::now(mind.k) {
        observe::publish(own, Reading::Memory(m));
    }
    observe::replace(own, Reading::Mind(Box::new(mind)));
    let budget = crate::network::wire::frame_budget().checked_sub(64)?;
    observe::encode_reply(&observe::readings(&prefix), budget)
}

/// A RING'S SIZE AS A SOUND: its return, read off its own coordinates on
/// the stroke its modes need — the carrier where it never returns.
fn sound_size(ring: &crate::transducer::cga::Mv) -> Option<crate::transducer::sentence::hearing::Size> {
    let need = crate::transducer::cga::modes_need(ring)?;
    let coords = crate::transducer::sentence::coordinates::Coordinates::of(ring, &mut crate::physics::Physics::default().stroke_n(need)).ok()?;
    crate::transducer::sentence::hearing::Size::of_coordinates(&coords)
}

/// A QUESTION'S COORDINATES: the thing said and the word in it the question
/// is about, each by its length — the address the speaker's clarification is
/// said under, so both minds name it alike.
fn question_key(said: &[u8], vertex: &[u8]) -> Vec<u8> {
    let mut k = vec![b'?'];
    crate::transducer::address::put_key_bytes(&mut k, said);
    crate::transducer::address::put_key_bytes(&mut k, vertex);
    k
}

/// A RESIDUE FACTORED INTO WHAT IT IS MADE OF (MU-CLIENT §5.3): its
/// carriage, where it carries, and its rotor peeled into the alphabet's
/// letters, each a word said on its own, the hue map riding the first. A
/// residue that splits into no more than itself — no carriage, one letter, or
/// no peel — is an ATOM, said whole.
pub(crate) fn factors_of(residue: &crate::transducer::cga::Mv, gauge: &[(u8, u8)], vertex: &[u8]) -> Vec<Spoken> {
    use crate::transducer::element::{conformal_factors, conformal_of, identity_versor, same_versor};
    let atom = || vec![filament(vertex.to_vec(), residue.clone(), gauge.to_vec())];
    let Ok(f) = conformal_factors(residue) else { return atom() };
    let carriage = f.carriage();
    let letters: Vec<(crate::transducer::alphabet::LetterId, bool)> =
        match crate::transducer::family::derivation::peel_versor(residue) {
            Some(Ok(word)) => word.letters().to_vec(),
            _ => Vec::new(),
        };
    let carries = !same_versor(&carriage, &identity_versor());
    if letters.len() + usize::from(carries) < 2 {
        return atom();
    }
    let mut out: Vec<Spoken> = Vec::new();
    if carries {
        let mut key = b"carriage".to_vec();
        if let Some(ray) = carriage.canonical() {
            ray.put_canonical(&mut key);
        }
        out.push(filament(key, carriage, Vec::new()));
    }
    for l in letters {
        let word = crate::transducer::alphabet::Word::from_letters(&[l]);
        let Some(motion) = conformal_of(&word, &identity_versor()) else { return atom() };
        out.push(filament(word.key().as_bytes().to_vec(), motion, Vec::new()));
    }
    if let Some(first) = out.first_mut() {
        first.gauge = gauge.to_vec();
    }
    out
}

/// Two readings of one thought's loops are one: word for word, the same
/// loop (`chatter::same_loop`, projectively).
fn one_reading(a: &[AckWord], b: &[AckWord]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| chatter::same_loop(x, y))
}

/// Every word of the thought closed its loop on the motion: heard whole or
/// shifted, none leftover.
fn understood(loops: &[AckWord]) -> bool {
    !loops.is_empty() && loops.iter().all(|a| a.motion.is_some())
}

impl Entity {
    /// OPEN THE CONVERSATION: a post on a free local port filed at the
    /// rendezvous `dir` under this mind's voice, and the carrier that
    /// converses on its own thread, with its own link to this mind's court.
    pub fn open_chatter(&mut self, dir: &std::path::Path) -> std::io::Result<String> {
        let Some(key) = self.hypha.kernel().signer().cloned() else {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "a mind that signs nothing says nothing"));
        };
        let (to, rx) = channel::<Inflow>();
        let deliver = to.clone();
        let voice = self.hypha.kernel().voice();
        let post = chatter::Post::open(
            "127.0.0.1:0",
            move |f| {
                let _ = deliver.send(Inflow::Frame(f));
            },
            move |q: &[u8]| observed(&voice, q),
        )?;
        let addr = post.addr().to_string();
        chatter::publish(dir, &voice, &addr)?;
        #[cfg(test)]
        let agreed = std::sync::Arc::new(std::sync::Mutex::new(Keyed::new()));
        let (field, pricing) = channel::<FieldWork>();
        let mut carrier = Carrier {
            voice,
            key,
            dir: dir.to_path_buf(),
            ear: self.epigenome.ear.clone(),
            court: Session::open(self.hypha.addr(), self.hypha.kernel().clone()),
            k: self.own_k(),
            point: self.hypha.tuniversal(),
            union: Keyed::new(),
            thoughts: Keyed::new(),
            waiting: Keyed::new(),
            field,
            asked: Keyed::new(),
            #[cfg(test)]
            agreed: std::sync::Arc::clone(&agreed),
        };
        std::thread::Builder::new().name("conversation".into()).spawn(move || {
            let _post = post;
            carrier.court.discover();
            for inflow in rx {
                carrier.take(inflow);
            }
        })?;
        self.chat = Some(Chat {
            to,
            pricing,
            dir: dir.to_path_buf(),
            told: None,
            #[cfg(test)]
            agreed,
        });
        Ok(addr)
    }

    /// 78.112 H5 — THE FIELD PRICES WHAT ITS CONVERSATION UNDERSTOOD: each
    /// thought's echoes read for their exact strain, per lane (`strain` on the
    /// motion, `hue_strain` on 𝒢), and admitted into this mind's own field
    /// as field lines (`mark_flux`, the one reader, its lanes booked as every
    /// flux is: the DRAG, the rate it moved at). Its echoes are closed loops —
    /// each a commutator, a holonomy — and together they are ONE CELL of this
    /// mind's orbit algebra (78.46, 78.26(5)): read by the one cell reader,
    /// their bracket closure the LIFT, their coherence the thought's. Where
    /// every echo is resonant the cell CLOSES — at its joint period, a delay
    /// and not a loss, a tilted reading as much agreed as a whole one — and
    /// the thought converged. Where an echo never returns, the cell is OPEN
    /// and that echo is a DEMAND on the charts where the hearing stood, its
    /// holonomy the residue (78.114): deviation feeds construction, and the
    /// mind's constructors (`take_demands`) expand its thought there until a
    /// later hearing closes. The price goes back to the conversation, which
    /// acts on it (`Carrier::priced`). Same word, two minds, two prices: each
    /// field is its own.
    pub(crate) fn price_hearings(&mut self) {
        let Some(chat) = self.chat.as_ref() else { return };
        let works: Vec<FieldWork> = chat.pricing.try_iter().collect();
        for w in works {
            match w {
                FieldWork::Hear { speaker, said, words } => self.understand(speaker, said, words),
                FieldWork::Learn(atom) => {
                    if crate::transducer::sentence::rung::learn(&atom) {
                        crate::observe::note_mind(|m| m.asks[2] += 1);
                    }
                }
                FieldWork::Price(p) => self.price(p),
            }
        }
    }

    /// MU-CLIENT §4.9 — RINGS ANOTHER MIND SAID, UNDERSTOOD IN THIS MIND'S
    /// FIELD: each composed again from its terms (`compose`), recognized
    /// against this mind's own rings (`rung::hear` — recognized, comprehended
    /// by its coordinates, or held new at its rung: the heard thought grows
    /// the mind's manifold), and seated among them by size (`rung::seat`):
    /// WHOLE where it resonates with a held ring, TILTED with its echoes,
    /// OUTER where no held ring is larger, LEFTOVER — not understood, and
    /// asked for. Each reading carries the held rings it sounded with — this
    /// mind's re-derivation, which the speaker nests for mutual balance. The
    /// understanding goes back to the conversation to be carried.
    fn understand(&mut self, speaker: [u8; 32], said: Vec<u8>, words: Vec<Spoken>) {
        use crate::transducer::sentence::compose::{compose, Term};
        use crate::transducer::sentence::hearing::Seat;
        use crate::transducer::sentence::rung::Recognition;
        let mut budget = self.stroke();
        let before = crate::transducer::sentence::rung::lexicon();
        let mut heard: Vec<(Spoken, HeardWord)> = Vec::with_capacity(words.len());
        for w in words {
            let terms: Vec<Term> = w.terms.iter().map(|(word, transport)| Term { word: word.clone(), transport: transport.clone() }).collect();
            // THE WORD: is this a word I have heard? Recognized or derivable
            // in the mind's own vocabulary (`rung::hear`), or new to it.
            // A sentence by the word its theorem teaches; one that teaches
            // none, or a lone word, by the word itself.
            let (ring, word) = match compose(&terms, &mut budget) {
                Some(s) => match crate::transducer::sentence::rung::hear(&s, &mut budget) {
                    Recognition::NoWord => {
                        let r = crate::transducer::sentence::rung::hear_word(&s.omega);
                        (s.omega, r)
                    }
                    r => (s.omega, r),
                },
                None => (w.motion.clone(), crate::transducer::sentence::rung::hear_word(&w.motion)),
            };
            // THE SOUND: does it sound like something I have heard? Read
            // against what this mind REMEMBERS hearing, never a vocabulary —
            // and remembered now, so it is part of that memory from here on.
            let sound = self.sound_of(&ring);
            self.remember_sound(&ring);
            let familiar = matches!(&sound, Some((e, _)) if matches!(e.seat, Seat::Whole | Seat::Tilted(_)));
            let (derived, asks, held, slot) = match (&sound, familiar, word.known()) {
                (None, ..) => (None, None, Vec::new(), 4),
                // A familiar sound and a known word: UNDERSTOOD, carried
                // round what it sounded like.
                (Some((e, through)), true, true) => (e.carried.clone(), None, through.clone(), 0),
                // It sounds like something heard, and is no word I know:
                // WHAT DOES THAT MEAN — the question.
                (Some(_), true, false) => (None, Some(ring.clone()), Vec::new(), 1),
                // A word I know, said in a way I have not heard: understood
                // SHIFTED, as itself.
                (Some(_), false, true) => (Some(ring.clone()), None, Vec::new(), 2),
                // A new sound and no word: noise not yet signal — LISTENED
                // to, held, not asked about.
                (Some(_), false, false) => (None, None, Vec::new(), 3),
            };
            crate::observe::note_mind(|m| m.seats[slot] += 1);
            let h = HeardWord { vertex: w.vertex.clone(), derived, gauge: w.gauge.clone(), cites: Vec::new(), asks, held };
            heard.push((w, h));
        }
        let after = crate::transducer::sentence::rung::lexicon();
        if let (Some(b), Some(a)) = (before, after) {
            crate::observe::note_mind(|m| {
                m.lexicon = a as u64;
                if let Some(g) = a.checked_sub(b) {
                    m.grown += g as u64;
                }
            });
        }
        if let Some(chat) = self.chat.as_ref() {
            let _ = chat.to.send(Inflow::Understood { speaker, said, words: heard });
        }
    }

    /// MU-CLIENT §4.9 — DOES THIS SOUND LIKE SOMETHING I HAVE HEARD: a ring
    /// read against what this mind remembers hearing, by its size, in the order sizes
    /// nest (`hearing::hear_across`) — WHOLE or TILTED where it sounds inside
    /// a heard ring (familiar), OUTER or LEFTOVER where it does not (a new
    /// sound) — with the heard rings it sounded with. `None` where its modes
    /// do not read.
    pub(crate) fn sound_of(&self, ring: &crate::transducer::cga::Mv) -> Option<(crate::transducer::sentence::hearing::Echoed, Vec<crate::transducer::cga::Mv>)> {
        use crate::transducer::sentence::hearing::{hear_across, Echoed, Seat, Size};
        if crate::transducer::sentence::presentation::is_still(ring) {
            return Some((Echoed { seat: Seat::Whole, carried: Some(ring.clone()), through: Vec::new() }, Vec::new()));
        }
        let size = sound_size(ring)?;
        // A SOUND HEARD AT ITS OWN SIZE: a held ring of the same return it
        // passes through whole — it commutes with it — sounds like it. The
        // nesting below reads only the layers above.
        if let Some(members) = self.remembered.get(&size.key()) {
            let alike: Vec<crate::transducer::cga::Mv> = members
                .iter()
                .filter(|(_, m)| crate::transducer::sentence::hearing::echo_sized(ring, m) == crate::transducer::sentence::hearing::Echo::Whole)
                .map(|(_, m)| m.clone())
                .collect();
            if !alike.is_empty() {
                return Some((Echoed { seat: Seat::Whole, carried: Some(ring.clone()), through: Vec::new() }, alike));
            }
        }
        let layers: Vec<(Size, Vec<_>)> = self
            .remembered
            .scoped()
            .filter_map(|(at, members)| Some((Size::from_key(&at)?, members.iter().map(|(k, m)| (k.clone(), m)).collect())))
            .collect();
        let echoed = hear_across(ring, size, layers.into_iter());
        let through = echoed
            .through
            .iter()
            .filter_map(|k| self.remembered.scoped().find_map(|(_, members)| members.iter().find(|(m, _)| m == k).map(|(_, r)| r.clone())))
            .collect();
        Some((echoed, through))
    }

    /// A RING HEARD, REMEMBERED by the mind under its size, by its
    /// coordinates — once.
    pub(crate) fn remember_sound(&mut self, ring: &crate::transducer::cga::Mv) {
        let (Some(size), Some(ray)) = (sound_size(ring), ring.canonical()) else { return };
        let mut key = Vec::new();
        ray.put_canonical(&mut key);
        let at = size.key();
        match self.remembered.get_mut(&at) {
            Some(members) => {
                if !members.iter().any(|(k, _)| *k == key) {
                    members.push((key, ring.clone()));
                }
            }
            None => {
                self.remembered.insert(&at, vec![(key, ring.clone())]);
            }
        }
    }

    /// ONE THOUGHT PRICED in this mind's field (78.112 H5): its echoes'
    /// strain, its chamber's coherence, and — at the speaker — MUTUAL
    /// balance: the hearer's own rings its understanding seated in, each
    /// seated among this mind's rings; any left over is a hearing this mind
    /// does not resonate with, and no agreement (MU-CLIENT §4.9).
    fn price(&mut self, p: Pricing) {
        use crate::transducer::cech::{ChamberCircuit, Coherence};
        use crate::transducer::family::pairing::{mark_flux, FieldLine, FluxReading};
        use crate::transducer::sentence::hearing::{hue_strain, strain, Seat};
        use crate::transducer::sentence::presentation::{is_still, LANE_G};
        // MUTUAL: what the hearer's understanding sounded like must sound
        // like something this mind has heard too.
        let mutual = p.mutual.iter().all(|r| matches!(self.sound_of(r), Some((e, _)) if matches!(e.seat, Seat::Whole | Seat::Tilted(_))));
        if !p.mutual.is_empty() {
            crate::observe::note_mind(|m| m.mutual[usize::from(!mutual)] += 1);
        }
        {
            let mut deviation: [num_rational::BigRational; 4] = std::array::from_fn(|_| num_rational::BigRational::from_integer(0.into()));
            let mut lines = Vec::new();
            let mut circuits = Vec::new();
            for a in &p.loops {
                if let Some(m) = &a.motion {
                    circuits.push(ChamberCircuit { ring: a.vertex.clone(), holonomy: Ok(m.clone()), hue: a.hue.clone() });
                }
                if let Some(s) = a.motion.as_ref().and_then(strain) {
                    for (d, x) in deviation.iter_mut().zip(s) {
                        *d += x;
                    }
                }
                if let Some(h) = &a.hue {
                    deviation[LANE_G] += hue_strain(h);
                }
                let gauge = match &a.hue {
                    Some(h) => h.clone(),
                    None => Vec::new(),
                };
                match &a.motion {
                    Some(m) if !is_still(m) || gauge.iter().any(|(x, y)| x != y) => {
                        lines.push(FieldLine { carriage: m.clone(), gauge, mass: 1, free: false, line: crate::transducer::cech::Thread::Free });
                    }
                    _ => {}
                }
            }
            let mut budget = self.stroke();
            if !lines.is_empty() {
                let _: FluxReading = mark_flux(&[], &lines, &mut budget);
            }
            // `k + 1` bodies make a `k`-cell: the thought's words and the
            // mind that heard them.
            let orders = vec![None; circuits.len()];
            let cell = crate::transducer::cech::read_chamber(circuits.len(), None, None, &circuits, &orders, &mut budget);
            let (mut demanded, mut answered) = (0u64, 0u64);
            let converged = match &cell.coherence {
                // A held hearing comes round precessing: understood, its
                // torsion its content, the deviation it carries the price.
                Coherence::Closes(_) | Coherence::Held { .. } => true,
                Coherence::Unread => false,
                Coherence::Open { escapes: incoherent, .. } => {
                    let charts: Vec<crate::transducer::stalk::Invariant> = p
                        .forms
                        .iter()
                        .filter_map(|f| self.epigenome.working_word(f))
                        .map(|h| crate::transducer::stalk::invariant_of(h.element()))
                        .collect();
                    for inc in incoherent {
                        let Ok(class) = &inc.class else { continue };
                        match self.contemplation.holds_own_letter(&inc.residue) {
                            true => answered += 1,
                            false => {
                                self.contemplation.docket_demand(class, &charts, Some(&inc.residue));
                                demanded += 1;
                            }
                        }
                    }
                    false
                }
            };
            crate::observe::note_mind(|m| {
                let (slot, sum) = match &cell.coherence {
                    Coherence::Closes(_) => (0, &mut m.closed),
                    Coherence::Held { .. } => (1, &mut m.closed),
                    Coherence::Open { .. } => (2, &mut m.open),
                    Coherence::Unread => (3, &mut m.open),
                };
                for (s, d) in sum.iter_mut().zip(&deviation) {
                    *s += d;
                }
                m.priced[slot] += 1;
                m.demanded += demanded;
                m.constructed[crate::observe::Constructed::Held as usize] += answered;
            });
            let converged = converged && mutual;
            if let Some(chat) = self.chat.as_ref() {
                let _ = chat.to.send(Inflow::Priced(Priced { loops: p.loops, answer: p.answer, deviation, converged }));
            }
        }
    }

    /// SAY WHAT THIS MIND STOOD to every mind it knows (78.161): handed to
    /// the conversation's carrier, never waited on.
    pub(crate) fn say(&mut self, e: &Element) {
        if let Some(chat) = self.chat.as_ref() {
            let _ = chat.to.send(Inflow::Say(e.clone()));
        }
    }

    /// 78.161 item 3 — A LETTER WAITS FOR ITS UNION. Where this mind
    /// converses and has peers, a letter it coined stands in its dialect and
    /// is said to them, and the court hears it once another mind agreed it
    /// (or once it has waited an aperture cycle unmet). `false` where there
    /// is no conversation or no peer: the caller speaks it as before.
    pub(crate) fn hold_for_union(&mut self, e: &Element) -> bool {
        let voice = self.hypha.kernel().voice();
        let has_peers = self.chat.as_ref().is_some_and(|c| !chatter::peers(&c.dir, &voice).is_empty());
        if !has_peers {
            return false;
        }
        let tu = self.now_tu();
        let _ = self.stand(std::slice::from_ref(e), tu);
        if let Some(chat) = self.chat.as_ref() {
            let _ = chat.to.send(Inflow::Wait(e.clone()));
        }
        true
    }

    /// THE MIND'S CLOCK, handed to its conversation as it moves: its own K
    /// (what a lapse is read against) and the final point its frames carry.
    pub(crate) fn tell_conversation_the_clock(&mut self) {
        let k = self.own_k();
        let point = self.hypha.tuniversal();
        let Some(chat) = self.chat.as_mut() else { return };
        if chat.told == Some(k) {
            return;
        }
        chat.told = Some(k);
        let _ = chat.to.send(Inflow::Clock { k, point });
    }
}

#[cfg(test)]
impl Entity {
    /// What the conversation transduced, taken in by this mind's field as
    /// its tick's admission does: inside the mind's own field, where its
    /// lexicon is.
    pub(crate) fn take_in_conversation(&mut self) {
        let mut field = std::mem::take(&mut self.field);
        field.think(|| self.price_hearings());
        self.field = field;
    }
}

#[cfg(test)]
impl Chat {
    /// Whether the thing this mind said at `addr` is agreed language.
    pub(crate) fn agreed(&self, addr: &[u8]) -> bool {
        self.agreed.lock().is_ok_and(|a| a.contains(addr))
    }
}

impl Carrier {
    /// ONE INFLOW TAKEN, and what has stood its aperture cycle let go.
    fn take(&mut self, inflow: Inflow) {
        match inflow {
            Inflow::Frame(f) => self.hear_frame(&f),
            Inflow::Say(e) => self.say(&e),
            Inflow::Wait(e) => {
                let addr = e.address();
                if !self.waiting.contains(addr.as_bytes()) {
                    self.waiting.insert(&addr, (e.clone(), self.k));
                }
                self.say(&e);
            }
            Inflow::Clock { k, point } => {
                self.k = k;
                if point.is_some() {
                    self.point = point;
                }
            }
            Inflow::Priced(p) => self.priced(p),
            Inflow::Understood { speaker, said, words } => self.understood(&speaker, said, words),
        }
        self.release_unmet();
        let ear = self.ear.read().len() as u64;
        let (thoughts, unions, waiting) = (self.thoughts.len() as u64, self.union.len() as u64, self.waiting.len() as u64);
        crate::observe::note_mind(|m| {
            m.k = self.k;
            m.ear = ear;
            m.thoughts = thoughts;
            m.unions = unions;
            m.waiting = waiting;
        });
    }

    /// WHAT THE FIELD PRICED A THOUGHT AT, acted on. Where its admission
    /// converged in this mind's field, the hearer HOLDs — carrying the exact
    /// deviation, so the speaker and the court read how far it moved — and
    /// the speaker speaks the thing said as agreed language, once. Where it
    /// opened strain, the thought stays understood and unagreed: its echoes
    /// held as transports, its strain the field's open debt until later
    /// hearing closes it.
    fn priced(&mut self, p: Priced) {
        if !p.converged {
            return;
        }
        match p.answer {
            Answer::Hold { speaker, said } => {
                let to = chatter::peer_of(&self.dir, &speaker);
                if let (Some(frame), Some(to)) = (self.frame(Said::Hold { said, words: p.loops, deviation: p.deviation }), to) {
                    self.send(&frame, Some(&to));
                    crate::observe::note_mind(|m| m.held += 1);
                }
            }
            Answer::Agree { said, frame } => {
                let Some(u) = self.union.get_mut(&said) else { return };
                if u.shared {
                    return;
                }
                u.shared = true;
                let agreed = u.said.clone();
                self.waiting.remove(&said);
                #[cfg(test)]
                if let Ok(mut a) = self.agreed.lock() {
                    a.insert(&said, ());
                }
                if self.speak_agreed(&agreed, Some(&frame)).is_ok() {
                    crate::observe::note_mind(|m| m.agreed += 1);
                }
            }
        }
    }

    /// A frame of this mind's, signed by its key.
    fn frame(&self, said: Said) -> Option<Vec<u8>> {
        Chatter { speaker: self.voice, point: self.point, said }.sign(&self.key)
    }

    /// SEND a frame to every peer at the rendezvous, or to the one named; a
    /// rendezvous entry that refuses its connection names a mind gone, and
    /// leaves.
    fn send(&self, frame: &[u8], to: Option<&str>) {
        let peers = match to {
            Some(a) => vec![a.to_string()],
            None => chatter::peers(&self.dir, &self.voice),
        };
        for p in peers {
            if let Err(e) = chatter::send(&p, frame) {
                if e.kind() == std::io::ErrorKind::ConnectionRefused {
                    chatter::forget(&self.dir, &p);
                }
            }
        }
    }

    /// SAY WHAT THE MIND STOOD: its words, each as its whole motion and hue
    /// map, to every mind at the rendezvous. Said once; its union is then
    /// read.
    fn say(&mut self, e: &Element) {
        let addr = e.address();
        if self.union.contains(addr.as_bytes()) {
            return;
        }
        let words = rings_of(e);
        if words.is_empty() {
            return;
        }
        let Some(frame) = self.frame(Said::Say { said: addr.as_bytes().to_vec(), words: words.clone() }) else { return };
        self.union.insert(&addr, Union { said: e.clone(), spoken: words, acked: Keyed::new(), shared: false, at: self.k });
        self.send(&frame, None);
    }

    /// A FRAME HEARD: opened under its speaker's signature, and taken by
    /// what it says. A frame that does not open, or this mind's own, is
    /// nothing.
    fn hear_frame(&mut self, f: &[u8]) {
        let Some(c) = Chatter::open(f) else { return };
        if c.speaker == self.voice {
            return;
        }
        match c.said {
            Said::Say { said, words } => self.hear_said(&c.speaker, said, &words),
            Said::Heard { said, words } => self.heard_back(&c.speaker, &said, &words),
            Said::Ack { said, words } => self.acknowledged(&c.speaker, &said, &words),
            Said::Hold { said, words, .. } => self.held_back(&c.speaker, &said, &words, f),
        }
    }

    /// WHAT ANOTHER MIND SAID, TRANSDUCED (MU-CLIENT §4.9): the nerve
    /// judges nothing. The said rings go to the mind's field, which
    /// understands them against its own rings (`Entity::understand`), and
    /// the understanding comes back to be carried (`understood`). An answer
    /// to a question this mind asked that is the residue itself, whole — the
    /// speaker could factor it no further — is an ATOM, handed to the field
    /// to be learned.
    fn hear_said(&mut self, speaker: &[u8; 32], said: Vec<u8>, words: &[Spoken]) {
        crate::observe::note_mind(|m| m.heard += 1);
        let mut asked_key = speaker.to_vec();
        asked_key.extend_from_slice(&said);
        if let Some((residue, _, _)) = self.asked.remove(&asked_key) {
            if let [only] = words {
                if only.motion.projective_eq(&residue) {
                    let _ = self.field.send(FieldWork::Learn(residue));
                    return;
                }
            }
        }
        let _ = self.field.send(FieldWork::Hear { speaker: *speaker, said, words: words.to_vec() });
    }

    /// WHAT THE FIELD UNDERSTOOD, CARRIED BACK: every question it raised
    /// remembered, the thought kept until the speaker acknowledges it, and
    /// the hearing said back to the speaker.
    fn understood(&mut self, speaker: &[u8; 32], said: Vec<u8>, heard: Vec<(Spoken, HeardWord)>) {
        let (words, out): (Vec<Spoken>, Vec<HeardWord>) = heard.into_iter().unzip();
        // Every question asked is remembered by its coordinates, until it is
        // answered or stands an aperture cycle unmet.
        for (h, w) in out.iter().zip(&words) {
            if let Some(residue) = &h.asks {
                let mut k = speaker.to_vec();
                k.extend_from_slice(&question_key(&said, &h.vertex));
                self.asked.insert(&k, (residue.clone(), w.gauge.clone(), self.k));
                crate::observe::note_mind(|m| m.asks[0] += 1);
            }
        }
        let Some(frame) = self.frame(Said::Heard { said: said.clone(), words: out.clone() }) else { return };
        let mut key = speaker.to_vec();
        key.extend_from_slice(&said);
        self.thoughts.insert(&key, Thought { words: words.into_iter().zip(out).collect(), at: self.k });
        if let Some(to) = chatter::peer_of(&self.dir, speaker) {
            self.send(&frame, Some(&to));
        }
    }

    /// ANOTHER MIND SAID BACK WHAT IT DERIVED of something this one said:
    /// every word's loop said → heard read by its physics, sent back as the
    /// ACK, and kept until that mind HOLDs the same loops.
    fn heard_back(&mut self, who: &[u8; 32], said: &[u8], words: &[HeardWord]) {
        let Some(spoken) = self.union.get(said).map(|u| u.spoken.clone()) else { return };
        // A QUESTION ASKED OF WHAT THIS MIND SAID: a ring the hearer did not
        // understand, answered ONE ORDER DOWN (MU-CLIENT §5.3) — a ring of
        // several terms by its terms, each word and each transport between
        // them said as a filament of its own; a filament by its letters and
        // carriage (`factors_of`) — said back to the asker alone, under the
        // question's coordinates.
        for h in words {
            let Some(residue) = &h.asks else { continue };
            let ring = spoken.iter().find(|s| s.vertex == h.vertex);
            let gauge = match ring {
                Some(s) => s.gauge.clone(),
                None => Vec::new(),
            };
            let factors = match ring {
                Some(s) if s.terms.len() > 1 => s
                    .terms
                    .iter()
                    .enumerate()
                    .flat_map(|(i, (word, transport))| {
                        let at = |part: u8| {
                            let mut v = h.vertex.clone();
                            v.extend_from_slice(&(i as u64).to_le_bytes());
                            v.push(part);
                            v
                        };
                        [filament(at(0), word.clone(), Vec::new()), filament(at(1), transport.clone(), Vec::new())]
                    })
                    .filter(|f| !crate::transducer::sentence::presentation::is_still(&f.motion))
                    .collect(),
                _ => factors_of(residue, &gauge, &h.vertex),
            };
            if let (Some(frame), Some(to)) = (
                self.frame(Said::Say { said: question_key(said, &h.vertex), words: factors }),
                chatter::peer_of(&self.dir, who),
            ) {
                self.send(&frame, Some(&to));
                crate::observe::note_mind(|m| m.asks[1] += 1);
            }
        }
        let acks: Vec<AckWord> =
            words.iter().filter_map(|h| spoken.iter().find(|s| s.vertex == h.vertex).map(|s| chatter::loop_of(s, h))).collect();
        let frame = self.frame(Said::Ack { said: said.to_vec(), words: acks.clone() });
        let to = chatter::peer_of(&self.dir, who);
        let Some(u) = self.union.get_mut(said) else { return };
        let held: Vec<crate::transducer::cga::Mv> = words.iter().flat_map(|h| h.held.iter().cloned()).collect();
        u.acked.insert(who, (acks, held));
        if let (Some(frame), Some(to)) = (frame, to) {
            self.send(&frame, Some(&to));
        }
    }

    /// THE SPEAKER READ THIS MIND'S THOUGHT (its ACK): every loop read again
    /// here by the one reader. Where the two minds hold one loop on every
    /// word the thought is SHARED: this mind HOLDs it back, signed, and its
    /// ear holds every curved holonomy. Loops read differently hold nothing.
    fn acknowledged(&mut self, speaker: &[u8; 32], said: &[u8], acks: &[AckWord]) {
        let mut key = speaker.to_vec();
        key.extend_from_slice(said);
        let Some(thought) = self.thoughts.remove(&key) else { return };
        let mine: Vec<AckWord> = thought.words.iter().map(|(s, h)| chatter::loop_of(s, h)).collect();
        // A word not understood closes no loop, and two readings of nothing
        // are no shared thought.
        if !one_reading(&mine, acks) || !understood(&mine) {
            return;
        }
        crate::observe::note_mind(|m| m.understood += 1);
        // Its price is the field's: the HOLD waits on it (`priced`). What the
        // two minds' resonance leaves is held in the field and the lexicon,
        // never filed into the ear (S6).
        let pricing = Pricing { loops: mine, answer: Answer::Hold { speaker: *speaker, said: said.to_vec() }, forms: Vec::new(), mutual: Vec::new() };
        let _ = self.field.send(FieldWork::Price(pricing));
    }

    /// THE HEARER HOLDS THE LOOPS THIS MIND READ (its HOLD, `frame`):
    /// verified against the ACK sent it. Where the two readings are one, the
    /// thought is shared on this side too, and every echo it carried — what
    /// the hearer's language did to it — is held as a transport between the
    /// two perspectives. Whether it is AGREED LANGUAGE is this mind's field's
    /// to price, as the hearer's priced it (`priced`): where the echoes
    /// converge here too, it is spoken to the court carrying the hearer's
    /// signed HOLD, once.
    fn held_back(&mut self, who: &[u8; 32], said: &[u8], holds: &[AckWord], frame: &[u8]) {
        let Some(u) = self.union.get_mut(said) else { return };
        let Some((sent, mutual)) = u.acked.remove(who) else { return };
        if !one_reading(&sent, holds) || !understood(&sent) {
            return;
        }
        // Where it stood in this mind: the thing said; and the hearer's own
        // rings its understanding seated in, nested here for MUTUAL balance.
        let forms = vec![said.to_vec()];
        crate::observe::note_mind(|m| m.returned += 1);
        let pricing = Pricing { loops: sent, answer: Answer::Agree { said: said.to_vec(), frame: frame.to_vec() }, forms, mutual };
        let _ = self.field.send(FieldWork::Price(pricing));
    }

    /// SPEAK TO THE COURT what the conversation carries there: an agreed
    /// thing with the hearer's signed HOLD, or a letter that waited its
    /// cycle unmet with none. The court's reply is its own; the sentence's
    /// standing returns to the mind through the syllabus.
    fn speak_agreed(&self, e: &Element, agreement: Option<&[u8]>) -> std::io::Result<Vec<u8>> {
        let Some(tape) = e.to_bytes() else {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "the sentence's record does not write"));
        };
        let spoken = [dialogue::Spoken {
            tape,
            standing: crate::transducer::stalk::Standing::Stood.as_u8(),
            contraction: None,
            agreement: agreement.map(<[u8]>::to_vec),
        }];
        let tu = self.point.map_or(0, |p| p.h);
        let Some(body) = dialogue::encode_speech_spoken(&self.voice, [], tu, &spoken) else {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "the speech does not frame"));
        };
        let reply = self.court.speak_nerve(body)?;
        match dialogue::decode_speech_reply(&reply) {
            Some(adjudicated) => Ok(adjudicated.iter().map(|a| a.code()).collect()),
            None => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "speech reply does not decode")),
        }
    }

    /// WHAT HAS STOOD AN APERTURE CYCLE of the mind's own strokes is let go:
    /// a thought no ACK reached, a thing said no HOLD met; and a letter no
    /// mind agreed goes to the court on its own — the union decides what is
    /// worth the court's replay first, and never silences a mind that has
    /// none.
    fn release_unmet(&mut self) {
        let now = self.k;
        let cycle = u64::from(crate::physics::aperture_cycle());
        let stood = |at: u64| now.checked_sub(at).is_some_and(|waited| waited >= cycle);
        let lapsed: Vec<Vec<u8>> = self.thoughts.scoped().filter(|(_, t)| stood(t.at)).map(|(k, _)| k).collect();
        for k in lapsed {
            self.thoughts.remove(&k);
        }
        let unanswered: Vec<Vec<u8>> = self.asked.scoped().filter(|(_, (_, _, at))| stood(*at)).map(|(k, _)| k).collect();
        for k in unanswered {
            self.asked.remove(&k);
        }
        let spent: Vec<Vec<u8>> = self.union.scoped().filter(|(_, u)| stood(u.at)).map(|(k, _)| k).collect();
        for k in spent {
            self.union.remove(&k);
        }
        let due: Vec<Vec<u8>> = self.waiting.scoped().filter(|(_, (_, at))| stood(*at)).map(|(k, _)| k).collect();
        for k in due {
            if let Some((letter, _)) = self.waiting.remove(&k) {
                let _ = self.speak_agreed(&letter, None);
            }
        }
    }
}
