// MU-CLIENT §5.5 — REAL CONVERGENCE: two different constructions reaching one
// conclusion. Two words that are not one vertex, each decompressed onto one
// scene (`family::local::word_local`), say the same thing there: the same
// sites changed, pair by pair, nothing false. Never one object read twice.
//
//! THE SCENES THIS MIND HAS SPOKEN WORDS ON. Every word it speaks under a
//! scene is held by the scene's citation with the scene's family, while its
//! utterance receipt stands (`forget_utterance` lets it go). They are what a
//! heard word is read against.
//!
//! THE EVIDENCE A HEARD WORD CARRIES (`chatter::Evidence`): the scene it was
//! spoken under and its record. The hearer decompresses it onto that scene —
//! held here, or pulled by its citation off the court's registry, the same
//! act a seat pulls a task by — and reads what it does there. A word whose
//! section on its own scene is empty says nothing true there: the hearer does
//! not HOLD it (no agreement on a word the evidence refutes). A word true
//! there, beside a word of this mind's own on that scene that is a different
//! vertex with the same section, is CONVERGED: two orbits, one conclusion.

use crate::client::sdk::*;
use crate::network::nerve::Keyed;
use crate::transducer::family::local::{word_local, Local};

/// WHAT A HEARD WORD'S EVIDENCE READ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Evident {
    /// No evidence carried: no word under a scene (a sentence, a letter, an
    /// answer to a question), heard as it always was.
    Unbound,
    /// Its section on its scene is not empty: true there.
    True,
    /// Its section on its scene is empty, or its record is not the word said.
    Refuted,
    /// The scene could not be read: no evidence read, so no agreement.
    Unread,
}

/// ONE SCENE THIS MIND SPOKE WORDS ON: its family and the words, by address.
struct SceneWords {
    family: Vec<crate::transducer::family::TrainPair>,
    words: Keyed<Element>,
}

/// THE SCENES THIS MIND SPOKE WORDS ON, the scenes each word was spoken
/// under (so a word leaving takes its place with it), what its hearings'
/// evidence read, and the convergences found.
#[derive(Default)]
pub(crate) struct Scenes {
    by_scene: Keyed<SceneWords>,
    of_word: Keyed<Keyed<()>>,
    /// Evidence read, by verdict: true, refuted, unread.
    pub(crate) read: [u64; 3],
    /// Each convergence once, keyed scene ‖ heard word ‖ own word, with the
    /// mind that spoke the heard word.
    pub(crate) converged: Keyed<[u8; 32]>,
    /// THE FAMILIES PULLED AS EVIDENCE, by scene: a scene this mind spoke no
    /// word on, pulled by its citation once and read here for every word
    /// heard on it after — a family does not move (2026-10-22: one family
    /// pulled 49 times from one court as a peer said its word at each
    /// return, and another 18 times for the different words peers said on
    /// it). One entry per scene, the corpus its bound. A pull that read
    /// nothing is not kept: it is asked again.
    pulled: Keyed<Vec<crate::transducer::family::TrainPair>>,
    /// The scenes whose pull is on the court's carrier now: a hearing on one
    /// meanwhile is let go, as a hearing that waited lapses — its speaker
    /// says it again — never pulled twice (8 pulls of one family before the
    /// first landed).
    pulling: Keyed<()>,
}

/// A WORD THAT HAS A SECTION: a connection, not a colour word — the words the
/// court measures by their own section (`Kernel::discharge_on`, W).
pub(crate) fn sectioned(word: &Element) -> bool {
    word.is_connection() && !word.is_colour_word()
}

/// Every pair's section empty: the word changed nothing true there.
fn empty(l: &Local) -> bool {
    l.sites.iter().all(Keyed::is_empty)
}

/// Two sections the same, pair by pair: neither closes a site beyond the
/// other (`Local::beyond`, a difference of sets).
pub(crate) fn same_section(a: &Local, b: &Local, family: &crate::transducer::family::Family) -> bool {
    empty(&a.beyond(b, family)) && empty(&b.beyond(a, family))
}

impl Scenes {
    /// A WORD SPOKEN UNDER A SCENE, held there with the scene's family.
    pub(crate) fn hold(&mut self, scene: &[u8], family: &crate::transducer::family::Family, word: &Element) {
        if scene.is_empty() || family.is_empty() || !sectioned(word) {
            return;
        }
        let addr = word.address();
        match self.by_scene.get_mut(scene) {
            Some(s) => {
                s.words.insert(&addr, word.clone());
            }
            None => {
                let mut words = Keyed::new();
                words.insert(&addr, word.clone());
                self.by_scene.insert(scene, SceneWords { family: family.to_vec(), words });
            }
        }
        self.of_word.nested_insert(addr.as_bytes(), scene, ());
    }

    /// A WORD LEAVING: out of every scene it was held on, and a scene left
    /// with no word leaves with it.
    pub(crate) fn forget(&mut self, addr: &[u8]) {
        let Some(scenes) = self.of_word.remove(addr) else { return };
        for (scene, ()) in scenes.scoped() {
            let gone = match self.by_scene.get_mut(&scene) {
                Some(s) => {
                    s.words.remove(addr);
                    s.words.is_empty()
                }
                None => false,
            };
            if gone {
                self.by_scene.remove(&scene);
            }
        }
    }

    /// The family of a scene this mind spoke words on.
    pub(crate) fn family(&self, scene: &[u8]) -> Option<&[crate::transducer::family::TrainPair]> {
        self.by_scene.get(scene).map(|s| s.family.as_slice())
    }

    /// How many scenes and words are held.
    pub(crate) fn held(&self) -> (usize, usize) {
        (self.by_scene.len(), self.by_scene.scoped().map(|(_, s)| s.words.len()).sum())
    }

    /// WHAT `word`, SAID BY `speaker` UNDER `scene`, DOES ON `family`: refuted
    /// where its section is empty; else true, and every word of this mind's
    /// own on that scene that is another vertex with the same section is a
    /// convergence, recorded once.
    pub(crate) fn read(&mut self, scene: &[u8], word: &Element, family: &crate::transducer::family::Family, speaker: [u8; 32]) -> Evident {
        let heard = word_local(word, family);
        if empty(&heard) {
            self.read[1] += 1;
            return Evident::Refuted;
        }
        self.read[0] += 1;
        let addr = word.address();
        let own: Vec<(Vec<u8>, Element)> = match self.by_scene.get(scene) {
            Some(s) => s.words.scoped().map(|(a, w)| (a, w.clone())).collect(),
            None => Vec::new(),
        };
        for (at, mine) in own {
            if mine.same_vertex(word) || !same_section(&heard, &word_local(&mine, family), family) {
                continue;
            }
            let mut key = Vec::new();
            crate::transducer::address::put_key_bytes(&mut key, scene);
            crate::transducer::address::put_key_bytes(&mut key, addr.as_bytes());
            key.extend_from_slice(&at);
            self.converged.insert(key, speaker);
        }
        Evident::True
    }
}

/// THE CHARTS A HEARING IS HEARD BY IN THE GROUND STATE: the charts its
/// evidence scene's bodies stand on (`Offer::of`), where the word is true
/// there — a refuted or unread word wakes no resting mind.
fn charts_heard(evident: Evident, family: &crate::transducer::family::Family) -> Vec<Vec<u8>> {
    match evident {
        Evident::True => super::offer::Offer::of(family).charts,
        _ => Vec::new(),
    }
}

impl Entity {
    /// RINGS ANOTHER MIND SAID, WITH THEIR EVIDENCE: read on their scene
    /// before they are understood, so the understanding knows whether the
    /// word is true there. A record that is not the word said, or not a word
    /// with a section, is refuted; a scene this mind holds is read at once;
    /// any other is pulled by its citation on the court's carrier and read
    /// where it lands — unread where no court answers it.
    pub(crate) fn hear_with_evidence(&mut self, speaker: [u8; 32], said: Vec<u8>, words: Vec<crate::client::sdk::chatter::Spoken>, evidence: Option<crate::client::sdk::chatter::Evidence>) {
        let Some(ev) = evidence else {
            self.perceive_hearing(speaker, said, words, Evident::Unbound);
            return;
        };
        let mut p = 0usize;
        let word = Element::from_bytes(&ev.word, &mut p).filter(|w| p == ev.word.len() && w.address().as_bytes() == said.as_slice() && sectioned(w));
        let (Some(word), Ok(id)) = (word, <[u8; 32]>::try_from(ev.scene.as_slice())) else {
            self.scenes.read[1] += 1;
            self.perceive_hearing(speaker, said, words, Evident::Refuted);
            return;
        };
        // A scene it spoke on, or one pulled before: read here.
        let held = self.scenes.family(&ev.scene).or_else(|| self.scenes.pulled.get(&ev.scene).map(Vec::as_slice)).map(<[_]>::to_vec);
        if let Some(family) = held {
            let evident = self.scenes.read(&ev.scene, &word, &family, speaker);
            let charts = charts_heard(evident, &family);
            self.perceive_hearing_on(speaker, said, words, evident, charts);
            return;
        }
        if self.scenes.pulling.contains(&id) {
            return;
        }
        self.scenes.pulling.insert(id, ());
        let act = dialogue::encode_task_by_id(&self.hypha.kernel().voice(), &id);
        self.ask(move |s| s.speak_keyed(&id, act), move |me, body: io::Result<Vec<u8>>| {
            me.scenes.pulling.remove(&id);
            let family = body.ok().filter(|b| !b.is_empty()).and_then(|b| dialogue::decode_task_envelope(&b)).map(|env| env.train);
            let (evident, charts) = match family {
                Some(f) => {
                    let evident = me.scenes.read(&id, &word, &f, speaker);
                    let charts = charts_heard(evident, &f);
                    me.scenes.pulled.insert(id, f);
                    (evident, charts)
                }
                None => {
                    me.scenes.read[2] += 1;
                    (Evident::Unread, Vec::new())
                }
            };
            me.perceive_hearing_on(speaker, said, words, evident, charts);
        });
    }
}
