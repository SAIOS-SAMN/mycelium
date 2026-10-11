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

use crate::client::sdk::chatter::{self, Chatter, HeardWord, Said, Spoken};
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
    /// This mind's words another mind re-derived, and those it did not.
    pub met: u64,
    pub unmet: u64,
    /// This mind's things another mind re-derived word for word — agreed
    /// language — and of them the letters that then went to the court.
    pub agreed: u64,
    pub to_court: u64,
    /// Frames that did not open (unsigned, malformed, another tag).
    pub refused: u64,
}

impl ChatTally {
    pub fn line(&self) -> String {
        format!(
            "chatter: said {} (sent {}, unsent {}) · heard words {} (re-derived {}, in part {}, unread {}) · own words met {} unmet {} · agreed {} · to court {} · refused {}",
            self.said, self.sent, self.unsent, self.heard, self.rederived, self.partial, self.unread, self.met, self.unmet, self.agreed, self.to_court, self.refused
        )
    }
}

/// ONE THING THIS MIND SAID: how many words it has, and per mind that answered,
/// how many of them that mind re-derived.
struct Union {
    words: u64,
    by: Keyed<u64>,
}

/// THE MIND'S CHATTER: its post, where its peers are found, and the union
/// each thing it said has reached.
pub struct Chat {
    post: chatter::Post,
    dir: std::path::PathBuf,
    union: Keyed<Union>,
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
        self.chat = Some(Chat { post, dir: dir.to_path_buf(), union: Keyed::new(), waiting: Keyed::new(), tally: ChatTally::default() });
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
        let n = words.len() as u64;
        let Some(frame) = self.chatter_frame(Said::Say { said: addr.as_bytes().to_vec(), words }) else { return };
        if let Some(chat) = self.chat.as_mut() {
            chat.tally.said += 1;
            chat.union.insert(&addr, Union { words: n, by: Keyed::new() });
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
        for k in due {
            let Some(chat) = self.chat.as_mut() else { return };
            let Some((letter, _)) = chat.waiting.remove(&k) else { continue };
            chat.tally.to_court += 1;
            let _ = self.speak(std::slice::from_ref(&letter), budget);
        }
    }

    /// THE CHATTER STROKE: every frame heard since the last, one spend each.
    pub fn chatter_stroke(&mut self, budget: &mut crate::physics::Budget) {
        let Some(frames) = self.chat.as_ref().map(|c| c.post.take()) else { return };
        let voice = self.session.kernel().voice();
        for f in frames {
            if !budget.spend() {
                break;
            }
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
                Said::Heard { said, words } => self.heard_back(&c.speaker, &said, &words, budget),
            }
        }
        self.release_unmet(budget);
    }

    /// WHAT ANOTHER MIND SAID, SAID AGAIN IN THIS ONE'S LANGUAGE: each word
    /// read over all this mind holds, lane by lane, and the derivation sent
    /// back to the speaker.
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
                        composes: r.composes,
                        residue: r.residue,
                        cites: r
                            .cites
                            .into_iter()
                            .filter_map(|(f, v, l, e)| Some((f.into_vec(), v.into_vec(), u8::try_from(l).ok()?, e)))
                            .collect(),
                    });
                }
                None => {
                    t.unread += 1;
                    out.push(HeardWord { vertex: w.vertex.clone(), composes: false, residue: [0; 4], cites: Vec::new() });
                }
            }
        }
        let Some(chat) = self.chat.as_mut() else { return };
        chat.tally.heard += t.heard;
        chat.tally.rederived += t.rederived;
        chat.tally.partial += t.partial;
        chat.tally.unread += t.unread;
        let to = chatter::peer_of(&chat.dir, speaker);
        let Some(frame) = self.chatter_frame(Said::Heard { said, words: out }) else { return };
        if let Some(to) = to {
            self.chatter_send(&frame, Some(&to));
        }
    }

    /// ANOTHER MIND SAID BACK WHAT IT RE-DERIVED of something this one said:
    /// every word it re-derived is a word the two minds meet on. Once one mind
    /// has re-derived every word of it, it is agreed language, and a letter
    /// waiting for its union goes to the court.
    fn heard_back(&mut self, who: &[u8; 32], said: &[u8], words: &[HeardWord], budget: &mut crate::physics::Budget) {
        let Some(chat) = self.chat.as_mut() else { return };
        let met = words.iter().filter(|w| w.composes).count() as u64;
        chat.tally.met += met;
        chat.tally.unmet += words.len() as u64 - met;
        let Some(u) = chat.union.get_mut(said) else { return };
        let before = u.by.scoped().any(|(_, &n)| n >= u.words);
        let n = match u.by.get(who) {
            Some(&held) => held.max(met),
            None => met,
        };
        u.by.insert(who, n);
        let agreed = n >= u.words;
        if before || !agreed {
            return;
        }
        chat.tally.agreed += 1;
        let Some((letter, _)) = chat.waiting.remove(said) else { return };
        chat.tally.to_court += 1;
        let _ = self.speak(std::slice::from_ref(&letter), budget);
    }
}
