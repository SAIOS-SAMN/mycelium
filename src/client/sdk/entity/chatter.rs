// 78.161 — THE MIND'S CHATTER: it says what it stands to other minds, hears
// what they say, re-derives each word on its own perception lane by lane, and
// answers AGREE or DIFFER; a word another mind agreed to is agreed language,
// and only agreed letters go to the court.
//
//! A heard word is never taken: it is lifted (78.61 C1) and asked of this
//! mind's own held language at the lifted coordinates, one keyed read
//! (`Epigenome::by_perception`). Coherent on every lane, the mind names its own
//! word; otherwise it names the lanes it holds nothing alike on
//! (`Epigenome::by_lane`). Nothing heard enters the genome or the dialect.

use crate::client::sdk::chatter::{self, Chatter, Said};
use crate::client::sdk::*;
use crate::network::nerve::Keyed;

/// WHAT THE MIND'S CHATTER CAME TO, cumulative.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChatTally {
    /// Words said, and frames sent (a word to every peer).
    pub said: u64,
    pub sent: u64,
    /// Frames that did not reach a peer.
    pub unsent: u64,
    /// Words heard; of them, agreed to and differed on; heard words this mind
    /// could not perceive (no form, nothing lifted, or unread).
    pub heard: u64,
    pub agreed_to: u64,
    pub differed_on: u64,
    pub unperceived: u64,
    /// Agreements and divergences this mind's own words received.
    pub agreements: u64,
    pub divergences: u64,
    /// This mind's words that became agreed language, and of them the letters
    /// that then went to the court.
    pub agreed_words: u64,
    pub to_court: u64,
    /// Frames that did not open (unsigned, malformed, another tag).
    pub refused: u64,
}

impl ChatTally {
    pub fn line(&self) -> String {
        format!(
            "chatter: said {} (sent {}, unsent {}) · heard {} (agreed {}, differed {}, unperceived {}) · received agree {} differ {} · agreed words {} · to court {} · refused {}",
            self.said,
            self.sent,
            self.unsent,
            self.heard,
            self.agreed_to,
            self.differed_on,
            self.unperceived,
            self.agreements,
            self.divergences,
            self.agreed_words,
            self.to_court,
            self.refused
        )
    }
}

/// THE MIND'S CHATTER: its post, where its peers are found, and the union
/// each of its words has reached.
pub struct Chat {
    post: chatter::Post,
    dir: std::path::PathBuf,
    /// Each of this mind's words said, by address → the minds that agreed to
    /// it (their voice → their own coherent word).
    union: Keyed<Keyed<Vec<u8>>>,
    /// Letters stood and said, waiting for a union before the court hears
    /// them (78.161 item 3), by address.
    waiting: Keyed<crate::transducer::element::Element>,
    pub tally: ChatTally,
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

    /// SAY A WORD THIS MIND STOOD to every mind it knows.
    pub(crate) fn say(&mut self, e: &crate::transducer::element::Element) {
        if self.chat.is_none() {
            return;
        }
        // A word already said is not said again: its union is being read.
        if self.chat.as_ref().is_some_and(|c| c.union.contains(e.address().as_bytes())) {
            return;
        }
        let Some(tape) = e.to_bytes() else { return };
        let Some(frame) = self.chatter_frame(Said::Say { tape }) else { return };
        if let Some(chat) = self.chat.as_mut() {
            chat.tally.said += 1;
            chat.union.insert(e.address(), Keyed::new());
        }
        self.chatter_send(&frame, None);
    }

    /// 78.161 item 3 — A LETTER WAITS FOR ITS UNION. Where this mind chatters
    /// and has peers, a letter it coined stands in its dialect and is said to
    /// them, and the court hears it once another mind agreed. `false` where
    /// there is no chatter or no peer: the caller speaks it as before.
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
        if let Some(chat) = self.chat.as_mut() {
            chat.waiting.insert(&addr, e.clone());
        }
        true
    }

    /// THE CHATTER STROKE: every frame heard since the last, one spend each.
    /// A word is re-derived and answered; an answer to this mind's own word is
    /// filed on its union, and a waiting letter another mind agreed to goes
    /// to the court.
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
                Said::Say { tape } => self.hear_word(&c.speaker, &tape),
                Said::Agree { word, own } => self.agreed(&c.speaker, &word, own, budget),
                Said::Differ { .. } => {
                    if let Some(chat) = self.chat.as_mut() {
                        chat.tally.divergences += 1;
                    }
                }
            }
        }
    }

    /// A WORD HEARD, RE-DERIVED: lifted and asked of this mind's own held
    /// language. The answer goes to the speaker at the rendezvous.
    fn hear_word(&mut self, speaker: &[u8; 32], tape: &[u8]) {
        let mut pos = 0usize;
        let heard = crate::transducer::element::Element::from_bytes(tape, &mut pos)
            .map(crate::transducer::element::Held::of);
        let keys = heard.as_ref().and_then(|h| h.form()).and_then(crate::client::sdk::epigenome::atlas::perception_keys);
        let Some(chat) = self.chat.as_mut() else { return };
        chat.tally.heard += 1;
        let (Some(heard), Some((whole, lanes))) = (heard, keys) else {
            chat.tally.unperceived += 1;
            return;
        };
        let word = heard.address().as_bytes().to_vec();
        // Coherent on every lane: the least of this mind's own words perceived
        // so — content order (rule 15), any one of them is the agreement.
        let own: Option<Vec<u8>> = self
            .epigenome
            .by_perception
            .get(&whole)
            .and_then(|bucket| bucket.scoped().map(|(a, ())| a.to_vec()).next());
        let said = match own {
            Some(own) => {
                chat.tally.agreed_to += 1;
                Said::Agree { word, own }
            }
            None => {
                chat.tally.differed_on += 1;
                let mut bits = 0u8;
                for (l, k) in lanes.iter().enumerate() {
                    let held = match (k, self.epigenome.by_lane.get(l)) {
                        (Some(k), Some(ix)) => ix.get(k).is_some(),
                        (None, _) => true,
                        _ => false,
                    };
                    if !held {
                        bits |= 1 << l;
                    }
                }
                Said::Differ { word, lanes: bits }
            }
        };
        let to = chatter::peer_of(&chat.dir, speaker);
        let Some(frame) = self.chatter_frame(said) else { return };
        if let Some(to) = to {
            self.chatter_send(&frame, Some(&to));
        }
    }

    /// ANOTHER MIND AGREED TO A WORD OF THIS MIND'S: filed on its union. The
    /// first agreement makes it agreed language, and a letter waiting for its
    /// union goes to the court.
    fn agreed(&mut self, who: &[u8; 32], word: &[u8], own: Vec<u8>, budget: &mut crate::physics::Budget) {
        let Some(chat) = self.chat.as_mut() else { return };
        chat.tally.agreements += 1;
        let first = match chat.union.get_mut(word) {
            Some(u) => {
                let first = u.is_empty();
                u.insert(who, own);
                first
            }
            None => return,
        };
        if !first {
            return;
        }
        chat.tally.agreed_words += 1;
        let Some(letter) = chat.waiting.remove(word) else { return };
        chat.tally.to_court += 1;
        let _ = self.speak(std::slice::from_ref(&letter), budget);
    }
}
