// 78.161 — THE MIND'S CHATTER: it says what it stands to other minds, word by
// word as each word's whole motion and hue map; a hearer says each word again
// in its own language and answers with that derivation; a word another mind
// re-derives is where the two meet, and only agreed letters go to the court.
//
//! Hearing is comprehension (1.7): a heard word is read over everything the
//! hearer holds, its genome and its dialect, lane by lane
//! (`Presentation::read_word`), and re-derived exactly or not at all. Two
//! minds reach one word by different derivations (1.5); nothing is compared
//! as bytes, and nothing heard enters the genome or the dialect.
//!
//! 78.163 — SAY IS HEARD, AND THE THOUGHT IS SHARED, BY ITS PHYSICS. Two
//! minds are two charts; carrying a thought across them is transport on a
//! curved space. The round trip said → heard is a loop, and its holonomy
//! (`chatter::loop_of`) is the content: what one mind's language does to the
//! other's thought — split against the word's own mode into DRAG (its plane,
//! another rate) and LIFT (the tilt into another plane, 78.46); still only
//! where the hearer echoed. The hearer answers with whatever its generators
//! reach, never only an exact re-derivation. SAY → HEARD → ACK (the
//! speaker's reading of every loop) → HOLD (the hearer's own reading): each
//! mind verifies the other's by the algebra, never a bool on trust. Where
//! both hold one loop on every word, the thought is SHARED: each mind's ear
//! holds every curved holonomy as a transport between the two
//! (`Presentation::hear_transport`) — the geometry the conversation built is
//! the vernacular — and the thing said is agreed language.

use crate::client::sdk::chatter::{self, AckWord, Chatter, HeardWord, Said, Spoken};
use crate::client::sdk::*;
use crate::network::nerve::Keyed;

/// WHAT THE MIND'S CHATTER CAME TO, cumulative.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChatTally {
    /// Things said (each with its words), and frames sent and not sent.
    pub said: u64,
    pub sent: u64,
    pub unsent: u64,
    /// Words heard; of them, re-derived whole in this mind's language, cited
    /// in part, and not read at all.
    pub heard: u64,
    pub rederived: u64,
    pub partial: u64,
    pub unread: u64,
    /// 78.163 — the loops this mind read of its own words another mind said
    /// back: closed and curved, of them with drag and with lift against the
    /// word's own mode, and those whose mode admits no split; still (an
    /// echo); and open (nothing derived, or no versor or permutation to
    /// close on).
    pub curved: u64,
    pub drag: u64,
    pub lift: u64,
    pub unsplit: u64,
    pub still: u64,
    pub open: u64,
    /// ACKs and HOLDs sent; thoughts SHARED (both minds hold one loop on
    /// every word, read on this side) and REFUTED (the loops differ); and the
    /// transports the shared thoughts filed in this mind's ear.
    pub acked: u64,
    pub held: u64,
    pub shared: u64,
    pub refuted: u64,
    pub transports: u64,
    /// This mind's things some mind shared on every word — agreed language —
    /// and of them the letters that then went to the court.
    pub agreed: u64,
    pub to_court: u64,
    /// Frames that did not open (unsigned, malformed, another tag).
    pub refused: u64,
}

impl ChatTally {
    /// The tally in the order of the Senses' `CHATTER` table.
    pub fn row(&self) -> [u64; crate::observe::CHATTER.len()] {
        [
            self.said,
            self.sent,
            self.unsent,
            self.heard,
            self.rederived,
            self.partial,
            self.unread,
            self.curved,
            self.drag,
            self.lift,
            self.unsplit,
            self.still,
            self.open,
            self.acked,
            self.held,
            self.shared,
            self.refuted,
            self.transports,
            self.agreed,
            self.to_court,
            self.refused,
        ]
    }
}

/// ONE THING THIS MIND SAID: its words as said; per mind it ACKed, the loops
/// it read, awaiting that mind's HOLD; and whether some mind has shared it.
struct Union {
    spoken: Vec<Spoken>,
    acked: Keyed<Vec<AckWord>>,
    shared: bool,
    /// The mind's own K it was said at: a union stands one aperture cycle
    /// of the mind's strokes, as a thought heard and a letter waiting do.
    at: u64,
}

/// ONE THING THIS MIND HEARD AND SAID AGAIN, waiting for the speaker's ACK:
/// what was said and what this mind derived, word by word, and the mind's
/// own K it was heard at.
struct Thought {
    words: Vec<(Spoken, HeardWord)>,
    at: u64,
}

/// THE MIND'S CHATTER: its post, where its peers are found, the union each
/// thing it said has reached, and the thoughts it heard awaiting their
/// ACK.
pub struct Chat {
    post: chatter::Post,
    dir: std::path::PathBuf,
    union: Keyed<Union>,
    /// 78.163 — keyed by the speaker's voice then what it said.
    thoughts: Keyed<Thought>,
    /// Letters stood and said, waiting for a union before the court hears
    /// them (78.161 item 3), by address, with the mind's own K they began
    /// waiting at.
    waiting: Keyed<(crate::transducer::element::Element, u64)>,
    pub tally: ChatTally,
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

impl Entity {
    /// OPEN THE CHATTER: listen on a free local port, and file the address
    /// at the rendezvous `dir` under this mind's voice.
    pub fn open_chatter(&mut self, dir: &std::path::Path) -> std::io::Result<String> {
        let post = chatter::Post::open("127.0.0.1:0")?;
        let addr = post.addr().to_string();
        chatter::publish(dir, &self.session.kernel().voice(), &addr)?;
        self.chat = Some(Chat { post, dir: dir.to_path_buf(), union: Keyed::new(), thoughts: Keyed::new(), waiting: Keyed::new(), tally: ChatTally::default() });
        Ok(addr)
    }

    /// What the chatter came to, where one is open.
    pub fn chat_tally(&self) -> Option<ChatTally> {
        self.chat.as_ref().map(|c| c.tally)
    }

    /// A frame of this mind's, signed by its key: `None` where it holds no key.
    fn chatter_frame(&self, said: Said) -> Option<Vec<u8>> {
        let key = self.session.kernel().signer()?;
        Chatter { speaker: self.session.kernel().voice(), point: self.session.tuniversal(), said }.sign(key)
    }

    /// SEND a frame to every peer at the rendezvous, or to the one named.
    fn chatter_send(&mut self, frame: &[u8], to: Option<&str>) {
        let voice = self.session.kernel().voice();
        let Some(chat) = self.chat.as_mut() else { return };
        let peers = match to {
            Some(a) => vec![a.to_string()],
            None => chatter::peers(&chat.dir, &voice),
        };
        for p in peers {
            match chatter::send(&p, frame) {
                Ok(()) => chat.tally.sent += 1,
                Err(_) => chat.tally.unsent += 1,
            }
        }
    }

    /// SAY WHAT THIS MIND STOOD to every mind it knows: its words, each as
    /// its whole motion and its hue map. Said once; its union is then read.
    pub(crate) fn say(&mut self, e: &crate::transducer::element::Element) {
        let addr = e.address();
        if self.chat.as_ref().is_none_or(|c| c.union.contains(addr.as_bytes())) {
            return;
        }
        let words = match crate::transducer::sentence::from_element_form(e) {
            Some(f) => spoken_of(&f),
            // A word spelled by letters is one word: its spelling at its
            // carriage, its sigma.
            None => match e.letters().and_then(|l| Some((crate::transducer::element::conformal_of(&l.spelling.word, &l.carriage)?, l.sigma))) {
                Some((motion, gauge)) => vec![Spoken { vertex: addr.as_bytes().to_vec(), motion, gauge }],
                None => Vec::new(),
            },
        };
        if words.is_empty() {
            return;
        }
        let Some(frame) = self.chatter_frame(Said::Say { said: addr.as_bytes().to_vec(), words: words.clone() }) else { return };
        let at = self.own_k();
        if let Some(chat) = self.chat.as_mut() {
            chat.tally.said += 1;
            chat.union.insert(&addr, Union { spoken: words, acked: Keyed::new(), shared: false, at });
        }
        self.chatter_send(&frame, None);
    }

    /// 78.161 item 3 — A LETTER WAITS FOR ITS UNION. Where this mind chatters
    /// and has peers, a letter it coined stands in its dialect and is said to
    /// them, and the court hears it once another mind re-derived it. `false`
    /// where there is no chatter or no peer: the caller speaks it as before.
    pub(crate) fn hold_for_union(&mut self, e: &crate::transducer::element::Element) -> bool {
        let voice = self.session.kernel().voice();
        let has_peers = self.chat.as_ref().is_some_and(|c| !chatter::peers(&c.dir, &voice).is_empty());
        if !has_peers {
            return false;
        }
        let addr = e.address();
        if self.chat.as_ref().is_some_and(|c| c.waiting.contains(addr.as_bytes())) {
            return true;
        }
        let tu = self.now_tu();
        let _ = self.stand(std::slice::from_ref(e), tu);
        let k = self.own_k();
        if let Some(chat) = self.chat.as_mut() {
            chat.waiting.insert(&addr, (e.clone(), k));
        }
        true
    }

    /// A LETTER NO MIND RE-DERIVED IN ONE APERTURE CYCLE of this mind's own
    /// strokes goes to the court on its own: the union decides what is worth
    /// the court's replay first, and never silences a mind that has no union.
    fn release_unmet(&mut self, budget: &mut crate::physics::Budget) {
        let now = self.own_k();
        let cycle = u64::from(crate::physics::aperture_cycle());
        let Some(chat) = self.chat.as_mut() else { return };
        // One circuit of the waiting letters, under the stroke: a wrap past
        // the last key ends it.
        let mut due = Vec::new();
        let mut cursor = chat.waiting.cursor(None, budget);
        while let Some((k, (_, at))) = Iterator::next(&mut cursor) {
            if cursor.circuit() {
                break;
            }
            if now.checked_sub(*at).is_some_and(|waited| waited >= cycle) {
                due.push(k.clone());
            }
        }
        drop(cursor);
        // 78.163 — a thought no ACK reached in one aperture cycle of this
        // mind's strokes is let go: the speaker never closed its loop. And a
        // thing said stands as long: a HOLD after it finds no union, and the
        // carrier holds what the conversation is still about, never all of it.
        let mut lapsed = Vec::new();
        let mut cursor = chat.thoughts.cursor(None, budget);
        while let Some((k, t)) = Iterator::next(&mut cursor) {
            if cursor.circuit() {
                break;
            }
            if now.checked_sub(t.at).is_some_and(|waited| waited >= cycle) {
                lapsed.push(k.clone());
            }
        }
        drop(cursor);
        for k in lapsed {
            chat.thoughts.remove(&k);
        }
        let mut spent = Vec::new();
        let mut cursor = chat.union.cursor(None, budget);
        while let Some((k, u)) = Iterator::next(&mut cursor) {
            if cursor.circuit() {
                break;
            }
            if now.checked_sub(u.at).is_some_and(|stood| stood >= cycle) {
                spent.push(k.clone());
            }
        }
        drop(cursor);
        for k in spent {
            chat.union.remove(&k);
        }
        for k in due {
            let Some(chat) = self.chat.as_mut() else { return };
            let Some((letter, _)) = chat.waiting.remove(&k) else { continue };
            chat.tally.to_court += 1;
            let _ = self.speak(std::slice::from_ref(&letter), budget);
        }
    }

    /// THE CHATTER STROKE: the frames heard, one spend each, earliest first;
    /// what the stroke cannot reach waits in the post for the next.
    pub fn chatter_stroke(&mut self, budget: &mut crate::physics::Budget) {
        let voice = self.session.kernel().voice();
        while budget.spend() {
            let Some(f) = self.chat.as_ref().and_then(|c| c.post.next()) else { break };
            let Some(c) = Chatter::open(&f) else {
                if let Some(chat) = self.chat.as_mut() {
                    chat.tally.refused += 1;
                }
                continue;
            };
            if c.speaker == voice {
                continue;
            }
            match c.said {
                Said::Say { said, words } => self.hear_said(&c.speaker, said, &words),
                Said::Heard { said, words } => self.heard_back(&c.speaker, &said, &words),
                Said::Ack { said, words } => self.acknowledged(&c.speaker, &said, &words),
                Said::Hold { said, words } => self.held_back(&c.speaker, &said, &words, budget),
            }
        }
        self.release_unmet(budget);
    }

    /// WHAT ANOTHER MIND SAID, SAID AGAIN IN THIS ONE'S LANGUAGE: each word
    /// read over all this mind holds, lane by lane, and the derivation — the
    /// motion and hue map its citations compose to — sent back to the
    /// speaker; the thought kept until the speaker acknowledges it.
    fn hear_said(&mut self, speaker: &[u8; 32], said: Vec<u8>, words: &[Spoken]) {
        let mut out = Vec::with_capacity(words.len());
        let mut t = ChatTally::default();
        for w in words {
            t.heard += 1;
            match self.epigenome.ear.read_word(&w.motion, &w.gauge, false) {
                Some(r) => {
                    match (r.composes, r.cites.is_empty()) {
                        (true, _) => t.rederived += 1,
                        (false, false) => t.partial += 1,
                        (false, true) => {}
                    }
                    out.push(HeardWord {
                        vertex: w.vertex.clone(),
                        derived: r.derived,
                        gauge: r.derived_gauge,
                        cites: r
                            .cites
                            .into_iter()
                            .filter_map(|(f, v, l, e)| Some((f.into_vec(), v.into_vec(), u8::try_from(l).ok()?, e)))
                            .collect(),
                    });
                }
                None => {
                    t.unread += 1;
                    out.push(HeardWord { vertex: w.vertex.clone(), derived: None, gauge: Vec::new(), cites: Vec::new() });
                }
            }
        }
        let k = self.own_k();
        let Some(frame) = self.chatter_frame(Said::Heard { said: said.clone(), words: out.clone() }) else { return };
        let Some(chat) = self.chat.as_mut() else { return };
        chat.tally.heard += t.heard;
        chat.tally.rederived += t.rederived;
        chat.tally.partial += t.partial;
        chat.tally.unread += t.unread;
        let mut key = speaker.to_vec();
        key.extend_from_slice(&said);
        chat.thoughts.insert(&key, Thought { words: words.iter().cloned().zip(out).collect(), at: k });
        if let Some(to) = chatter::peer_of(&chat.dir, speaker) {
            self.chatter_send(&frame, Some(&to));
        }
    }

    /// ANOTHER MIND SAID BACK WHAT IT DERIVED of something this one said:
    /// every word's loop said → heard is read by its physics — curved, split
    /// into drag and lift against the word's own mode; still (an echo); or
    /// open — and the reading is sent back as the ACK, kept until that mind
    /// HOLDs the same loops.
    fn heard_back(&mut self, who: &[u8; 32], said: &[u8], words: &[HeardWord]) {
        let Some(spoken) = self.chat.as_ref().and_then(|c| c.union.get(said)).map(|u| u.spoken.clone()) else { return };
        let mut t = ChatTally::default();
        let mut acks = Vec::with_capacity(words.len());
        for h in words {
            let Some(s) = spoken.iter().find(|s| s.vertex == h.vertex) else { continue };
            let a = chatter::loop_of(s, h);
            match (&a.motion, chatter::still(&a)) {
                (_, true) => t.still += 1,
                (None, false) => t.open += 1,
                (Some(holonomy), false) => {
                    t.curved += 1;
                    match crate::transducer::cga::lift_drag(&s.motion, holonomy) {
                        Some(ld) => {
                            t.drag += u64::from(!ld.drag.is_zero());
                            t.lift += u64::from(!ld.lift.is_zero());
                        }
                        None => t.unsplit += 1,
                    }
                }
            }
            acks.push(a);
        }
        let frame = self.chatter_frame(Said::Ack { said: said.to_vec(), words: acks.clone() });
        let Some(chat) = self.chat.as_mut() else { return };
        chat.tally.curved += t.curved;
        chat.tally.drag += t.drag;
        chat.tally.lift += t.lift;
        chat.tally.unsplit += t.unsplit;
        chat.tally.still += t.still;
        chat.tally.open += t.open;
        let to = chatter::peer_of(&chat.dir, who);
        let Some(u) = chat.union.get_mut(said) else { return };
        u.acked.insert(who, acks);
        if let (Some(frame), Some(to)) = (frame, to) {
            self.chatter_send(&frame, Some(&to));
            if let Some(chat) = self.chat.as_mut() {
                chat.tally.acked += 1;
            }
        }
    }

    /// THE SPEAKER READ THIS MIND'S THOUGHT (its ACK): every word's loop is
    /// read again here, from what was said and what this mind derived, by
    /// the one reader. Where the two minds hold one loop on every word the
    /// thought is SHARED — this mind HOLDs it back, with its own reading for
    /// the speaker to verify, and its ear holds every curved holonomy as a
    /// transport between the two. A loop the two read differently is
    /// refuted, and nothing is held.
    fn acknowledged(&mut self, speaker: &[u8; 32], said: &[u8], acks: &[AckWord]) {
        let Some(chat) = self.chat.as_mut() else { return };
        let mut key = speaker.to_vec();
        key.extend_from_slice(said);
        let Some(thought) = chat.thoughts.remove(&key) else { return };
        let mine: Vec<AckWord> = thought.words.iter().map(|(s, h)| chatter::loop_of(s, h)).collect();
        if !one_reading(&mine, acks) {
            chat.tally.refuted += 1;
            return;
        }
        chat.tally.shared += 1;
        let to = chatter::peer_of(&chat.dir, speaker);
        self.hold_transports(&mine);
        let Some(frame) = self.chatter_frame(Said::Hold { said: said.to_vec(), words: mine }) else { return };
        if let Some(to) = to {
            self.chatter_send(&frame, Some(&to));
            if let Some(chat) = self.chat.as_mut() {
                chat.tally.held += 1;
            }
        }
    }

    /// THE HEARER HOLDS THE LOOPS THIS MIND READ (its HOLD): verified against
    /// the ACK this mind sent it, by the algebra. Where the two readings are
    /// one, the thought is SHARED on this side too — this mind's ear holds
    /// every curved holonomy — and the thing said is agreed language: a
    /// letter waiting for its union goes to the court.
    fn held_back(&mut self, who: &[u8; 32], said: &[u8], holds: &[AckWord], budget: &mut crate::physics::Budget) {
        let Some(chat) = self.chat.as_mut() else { return };
        let Some(u) = chat.union.get_mut(said) else { return };
        let Some(sent) = u.acked.remove(who) else { return };
        if !one_reading(&sent, holds) {
            chat.tally.refuted += 1;
            return;
        }
        chat.tally.shared += 1;
        let first = !u.shared;
        u.shared = true;
        self.hold_transports(&sent);
        if !first {
            return;
        }
        let Some(chat) = self.chat.as_mut() else { return };
        chat.tally.agreed += 1;
        let Some((letter, _)) = chat.waiting.remove(said) else { return };
        chat.tally.to_court += 1;
        let _ = self.speak(std::slice::from_ref(&letter), budget);
    }

    /// A SHARED THOUGHT FEEDS THE EAR: every curved loop's holonomy — what one
    /// mind's language does to the other's thought — held as a transport, a
    /// generator of this mind's dialect, through which later words from that
    /// perspective are deconstructed. Still and open loops carry nothing.
    fn hold_transports(&mut self, loops: &[AckWord]) {
        let mut filed = 0u64;
        for a in loops.iter().filter(|a| !chatter::still(a)) {
            let (Some(motion), hue) = (&a.motion, &a.hue) else { continue };
            let hue = match hue {
                Some(h) => h.as_slice(),
                None => &[],
            };
            filed += u64::from(self.epigenome.ear.hear_transport(motion, hue));
        }
        if let Some(chat) = self.chat.as_mut() {
            chat.tally.transports += filed;
        }
    }
}

/// Two readings of one thought's loops are one: word for word, the same
/// loop (`chatter::same_loop`, projectively).
fn one_reading(a: &[AckWord], b: &[AckWord]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| chatter::same_loop(x, y))
}
