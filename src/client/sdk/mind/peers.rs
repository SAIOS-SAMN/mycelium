// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-CLIENT §5.3(5) (#145, ruled 2026-11-06) — A PEER IS REMEMBERED BY ITS
//! RECEIPTS, NEVER BY A LABEL. A peer mind keeps no receipt chain (78.49):
//! its signed receipts are its verified chatter frames. A frame a peer said
//! under the coordinates of a question this mind asked it is held with that
//! question (`Question::frames`), and lapses with it by the question's own
//! physics (its line let go, FX1); a closed question's frames stay while the
//! closure is cited, its residue known in the vocabulary (`Closed`).
//!
//! THE RECORD IS A FOLD, NEVER A STORE. Per peer key it is rebuilt by
//! verifying and reading the held frames again: each answer taken (its
//! factors re-derive the residue exactly) or inexact, filed by the class
//! asked, the residue's conjugacy invariants read off the motion itself
//! (segment 1, §4.6; `Conjugacy::class_key`), never the class key of the
//! word the question was raised in; and the vantages the peer said with it.
//! A question is addressed by that record: a class the peer answered
//! inexactly is not asked of it again. No posture, label or score is made
//! of it (78.168 §6); it is detection only, any consequence the game
//! kernel's (78.165).

use crate::anastome::dialogue::chatter::{Chatter, Said};
use crate::client::sdk::Mind;
use crate::transducer::address::Address;
use crate::transducer::cga::{Conjugacy, Mv};

/// HOW A PEER'S ANSWER STOOD AGAINST THE RESIDUE ASKED: a typed reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Answer {
    /// Its factors, composed in the order said, re-derive the residue.
    Taken,
    /// They do not.
    Inexact,
}

/// ONE RECEIPT OF A PEER, READ: the class asked, how the answer stood, and
/// the vantages the peer said with it.
#[derive(Debug, Clone)]
pub(crate) struct PeerReading {
    pub(crate) class: Address,
    pub(crate) answer: Answer,
    #[cfg_attr(not(test), expect(dead_code, reason = "the vantages are part of the record (§5.3(5)); their reader is the game kernel's (78.165), not yet on the mind"))]
    pub(crate) vantage: Vec<Mv>,
}

/// WHETHER A QUESTION IS ADDRESSED TO A PEER, read off its record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Addressed {
    /// Nothing in the record stands against asking it.
    Ask,
    /// It answered this class inexactly: not asked it again.
    Inexact,
}

/// A CLOSED QUESTION'S RECEIPTS: its residue, and the frames said under it.
#[derive(Debug, Clone)]
pub(crate) struct Closed {
    pub(crate) residue: Mv,
    pub(crate) frames: Vec<Vec<u8>>,
}

/// ONE FRAME READ AGAINST THE RESIDUE IT ANSWERED: its speaker and the
/// reading, or `None` where the frame does not verify, says no words, or
/// the residue has no class.
pub(crate) fn read_frame(frame: &[u8], residue: &Mv) -> Option<([u8; 32], PeerReading)> {
    let c = Chatter::verify(frame)?;
    let Said::Say { words, .. } = c.said else { return None };
    let class = residue.class_key()?;
    let one = Mv::scalar(num_rational::BigRational::from_integer(1.into()));
    let composed = words.iter().fold(one, |acc, w| acc.geo(&w.motion));
    let answer = match composed.projective_eq(residue) {
        true => Answer::Taken,
        false => Answer::Inexact,
    };
    let vantage = words.iter().flat_map(|w| w.vantage.iter().cloned()).collect();
    Some((c.speaker, PeerReading { class, answer, vantage }))
}

impl Mind {
    /// A PEER'S FRAME UNDER A QUESTION'S COORDINATES, held with the question
    /// open at `key`; with none open there, it answers nothing and is not held.
    pub(crate) fn hold_answer_frame(&mut self, key: &[u8], frame: Vec<u8>) {
        if let Some(q) = self.questions.get_mut(key) {
            q.frames.push(frame);
        }
    }

    /// A CLOSED QUESTION'S FRAMES, kept under its key beside its residue.
    pub(crate) fn keep_closed_frames(&mut self, key: &[u8], residue: &Mv, frames: Vec<Vec<u8>>) {
        if !frames.is_empty() {
            self.closed_answers.insert(key, Closed { residue: residue.clone(), frames });
        }
    }

    /// THE PEER'S RECORD, FOLDED FROM ITS RECEIPTS: every held frame `peer`
    /// signed, read against the residue it answered — the open questions'
    /// and the cited closures'. A closure no longer cited lets its frames go
    /// here, where the fold finds it.
    pub(crate) fn peer_record(&mut self, peer: &[u8; 32]) -> Vec<PeerReading> {
        let uncited: Vec<Vec<u8>> = self
            .closed_answers
            .scoped()
            .filter(|(_, c)| !crate::transducer::sentence::rung::known(&c.residue))
            .map(|(k, _)| k)
            .collect();
        for k in uncited {
            self.closed_answers.remove(&k);
        }
        let open = self.questions.scoped().flat_map(|(_, q)| q.frames.iter().map(move |f| (f, &q.residue)));
        let closed = self.closed_answers.scoped().flat_map(|(_, c)| c.frames.iter().map(move |f| (f, &c.residue)));
        open.chain(closed)
            .filter_map(|(f, r)| read_frame(f, r))
            .filter(|(speaker, _)| speaker == peer)
            .map(|(_, reading)| reading)
            .collect()
    }

    /// WHETHER `residue` IS ASKED OF `peer`: not where its record holds an
    /// inexact answer on the residue's class.
    pub(crate) fn addressed(&mut self, peer: &[u8; 32], residue: &Mv) -> Addressed {
        let Some(class) = residue.class_key() else { return Addressed::Ask };
        match self.peer_record(peer).iter().any(|r| r.answer == Answer::Inexact && r.class == class) {
            true => Addressed::Inexact,
            false => Addressed::Ask,
        }
    }
}
