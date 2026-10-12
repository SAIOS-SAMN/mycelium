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
/// the field stood and says, a letter the field holds for its union, and the
/// mind's clock as it moves.
enum Inflow {
    Frame(Vec<u8>),
    Say(Element),
    Wait(Element),
    Clock { k: u64, point: Option<TPoint> },
}

/// THE FIELD'S HANDLE ON ITS CONVERSATION: where to hand what it says, and
/// the rendezvous its peers are found at.
pub struct Chat {
    to: Sender<Inflow>,
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
    acked: Keyed<Vec<AckWord>>,
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
    #[cfg(test)]
    agreed: std::sync::Arc<std::sync::Mutex<Keyed<()>>>,
}

/// THE WORDS OF A FORM, each as its whole motion and its hue map.
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
        out.push(Spoken { vertex: address.as_bytes().to_vec(), motion, gauge });
    }
    out
}

/// THE WORDS OF A THING SAID: a form's word vertices, or a word spelled by
/// letters as one word — its spelling at its carriage, its sigma.
fn words_of(e: &Element) -> Vec<Spoken> {
    let addr = e.address();
    match crate::transducer::sentence::from_element_form(e) {
        Some(f) => spoken_of(&f),
        None => match e.letters().and_then(|l| Some((crate::transducer::element::conformal_of(&l.spelling.word, &l.carriage)?, l.sigma))) {
            Some((motion, gauge)) => vec![Spoken { vertex: addr.as_bytes().to_vec(), motion, gauge }],
            None => Vec::new(),
        },
    }
}

/// Two readings of one thought's loops are one: word for word, the same
/// loop (`chatter::same_loop`, projectively).
fn one_reading(a: &[AckWord], b: &[AckWord]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| chatter::same_loop(x, y))
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
        let post = chatter::Post::open("127.0.0.1:0", move |f| {
            let _ = deliver.send(Inflow::Frame(f));
        })?;
        let addr = post.addr().to_string();
        let voice = self.hypha.kernel().voice();
        chatter::publish(dir, &voice, &addr)?;
        #[cfg(test)]
        let agreed = std::sync::Arc::new(std::sync::Mutex::new(Keyed::new()));
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
            dir: dir.to_path_buf(),
            told: None,
            #[cfg(test)]
            agreed,
        });
        Ok(addr)
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
        }
        self.release_unmet();
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
        let words = words_of(e);
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
            Said::Hold { said, words } => self.held_back(&c.speaker, &said, &words, f),
        }
    }

    /// WHAT ANOTHER MIND SAID, SAID AGAIN IN THIS ONE'S LANGUAGE: each word
    /// read over all this mind holds, and the derivation — the motion and
    /// hue map its citations compose to — sent back to the speaker; the
    /// thought kept until the speaker acknowledges it.
    fn hear_said(&mut self, speaker: &[u8; 32], said: Vec<u8>, words: &[Spoken]) {
        let out: Vec<HeardWord> = {
            let ear = self.ear.read();
            words
                .iter()
                .map(|w| match ear.read_word(&w.motion, &w.gauge, false) {
                    Some(r) => HeardWord {
                        vertex: w.vertex.clone(),
                        derived: r.derived,
                        gauge: r.derived_gauge,
                        cites: r.cites.into_iter().filter_map(|(f, v, l, e)| Some((f.into_vec(), v.into_vec(), u8::try_from(l).ok()?, e))).collect(),
                    },
                    None => HeardWord { vertex: w.vertex.clone(), derived: None, gauge: Vec::new(), cites: Vec::new() },
                })
                .collect()
        };
        let Some(frame) = self.frame(Said::Heard { said: said.clone(), words: out.clone() }) else { return };
        let mut key = speaker.to_vec();
        key.extend_from_slice(&said);
        self.thoughts.insert(&key, Thought { words: words.iter().cloned().zip(out).collect(), at: self.k });
        if let Some(to) = chatter::peer_of(&self.dir, speaker) {
            self.send(&frame, Some(&to));
        }
    }

    /// ANOTHER MIND SAID BACK WHAT IT DERIVED of something this one said:
    /// every word's loop said → heard read by its physics, sent back as the
    /// ACK, and kept until that mind HOLDs the same loops.
    fn heard_back(&mut self, who: &[u8; 32], said: &[u8], words: &[HeardWord]) {
        let Some(spoken) = self.union.get(said).map(|u| u.spoken.clone()) else { return };
        let acks: Vec<AckWord> =
            words.iter().filter_map(|h| spoken.iter().find(|s| s.vertex == h.vertex).map(|s| chatter::loop_of(s, h))).collect();
        let frame = self.frame(Said::Ack { said: said.to_vec(), words: acks.clone() });
        let to = chatter::peer_of(&self.dir, who);
        let Some(u) = self.union.get_mut(said) else { return };
        u.acked.insert(who, acks);
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
        if !one_reading(&mine, acks) {
            return;
        }
        self.hold_transports(&mine);
        let to = chatter::peer_of(&self.dir, speaker);
        if let (Some(frame), Some(to)) = (self.frame(Said::Hold { said: said.to_vec(), words: mine }), to) {
            self.send(&frame, Some(&to));
        }
    }

    /// THE HEARER HOLDS THE LOOPS THIS MIND READ (its HOLD, `frame`):
    /// verified against the ACK sent it. Where the two readings are one, the
    /// thought is shared on this side too and the thing said is AGREED
    /// LANGUAGE: spoken to the court carrying the hearer's signed HOLD, once.
    fn held_back(&mut self, who: &[u8; 32], said: &[u8], holds: &[AckWord], frame: &[u8]) {
        let Some(u) = self.union.get_mut(said) else { return };
        let Some(sent) = u.acked.remove(who) else { return };
        if !one_reading(&sent, holds) {
            return;
        }
        let first = !u.shared;
        u.shared = true;
        let agreed = u.said.clone();
        self.hold_transports(&sent);
        if !first {
            return;
        }
        self.waiting.remove(said);
        #[cfg(test)]
        if let Ok(mut a) = self.agreed.lock() {
            a.insert(said, ());
        }
        let _ = self.speak_agreed(&agreed, Some(frame));
    }

    /// A SHARED THOUGHT FEEDS THE EAR: every curved loop's holonomy held as
    /// a transport through which later words from that perspective are
    /// deconstructed. Still and open loops carry nothing.
    fn hold_transports(&self, loops: &[AckWord]) {
        let mut ear = self.ear.write();
        for a in loops.iter().filter(|a| !chatter::still(a)) {
            let Some(motion) = &a.motion else { continue };
            let hue = match &a.hue {
                Some(h) => h.as_slice(),
                None => &[],
            };
            let _ = ear.hear_transport(motion, hue);
        }
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
