// EC1, 78.33, 45.1 — A PEER'S RULING, APPLIED AT THE NODE THAT ADMITS ITS
// RECEIPT: the arm the Ruling node's kind carries, on the word its first
// citation locates here, through the receipt path and never a message act.
//
//! Upheld, demoted and slashed rule a word on this court's provisional
//! row, through `apply_verdict`. Retired rules a word of this court's
//! ruled language away: the court replays the retirement on its own basis
//! and, where it re-derives, retires the word here too — the ruled cover
//! is federated, and two courts whose languages differ by a word the one
//! retired and the other kept describe two supplies (MU-ECONOMY §3.2, §6).
//! A citation this court does not hold WAITS, keyed by its stamp, and is
//! heard again when a word is filed there; a replay that does not
//! re-derive is REFUSED (I, 78.9), named and counted, never applied and
//! never dropped. Each is counted on the clock by arm:
//! admitted = applied + awaiting + refused.

use crate::chronometry::{KStepReceipt, Ruled, RulingRefused, Stamp};
use crate::cognition::*;

/// What a peer's ruling came to here.
enum Heard {
    /// Nothing here for it to rule: the word is not on the row the arm
    /// rules. Not counted.
    Inert,
    /// A citation this court does not hold, by its stamp.
    Waits(Stamp),
    Applied,
    /// The replay did not re-derive, by its typed reason (F5).
    Refused(RulingRefused),
}

impl Kernel {
    /// EC1 — HEAR A PEER'S RULING whose receipt this court admitted as a
    /// parent, and count what it came to.
    /// Heard inside the act that admitted the receipt, on its stroke (65.5).
    pub(crate) fn hear_ruling(&mut self, r: &KStepReceipt, arm: Ruled, budget: &mut crate::physics::Budget) {
        match self.judge_ruling(r, arm, budget) {
            Heard::Inert => {}
            Heard::Waits(at) => {
                let counts = &mut self.rulings[arm.index()];
                counts[0] += 1;
                counts[2] += 1;
                match self.awaiting_rulings.get_mut(&at.key()) {
                    Some(bucket) => bucket.push(r.clone()),
                    None => {
                        self.awaiting_rulings.insert(at.key(), vec![r.clone()]);
                    }
                }
            }
            Heard::Applied => {
                let counts = &mut self.rulings[arm.index()];
                counts[0] += 1;
                counts[1] += 1;
            }
            // F5 — the refusal by its reason, per arm, on the clock: the
            // reading carries what the print said.
            Heard::Refused(why) => {
                let counts = &mut self.rulings[arm.index()];
                counts[0] += 1;
                counts[3] += 1;
                self.ruling_refusals[arm.index()][why.index()] += 1;
            }
        }
    }

    /// A WORD WAS FILED AT `at`: every peer's ruling that waited on that
    /// stamp is heard again. The bucket is taken before it is walked, so a
    /// ruling that waits on a SECOND citation re-dockets under that one and
    /// a chain of citations resolves one link per filing, never spinning
    /// (rule 10).
    /// On the stroke of the act that filed the word (65.5).
    pub(crate) fn rehear_rulings(&mut self, at: &Stamp, budget: &mut crate::physics::Budget) {
        let Some(waiting) = self.awaiting_rulings.remove(&at.key()) else {
            return;
        };
        for r in waiting {
            let crate::chronometry::ActKind::Ruling(arm) = r.kind else { continue };
            // It leaves the waiting count and is admitted again as whatever
            // it comes to now.
            let counts = &mut self.rulings[arm.index()];
            counts[0] -= 1;
            counts[2] -= 1;
            self.hear_ruling(&r, arm, budget);
        }
    }

    fn judge_ruling(&mut self, r: &KStepReceipt, arm: Ruled, budget: &mut crate::physics::Budget) -> Heard {
        use crate::anastome::dialogue::{RULING_DEMOTED, RULING_SLASHED, RULING_UPHELD};
        // The first citation is the word ruled; a ruling that names none
        // rules nothing here.
        let [ruled, rest @ ..] = r.cites.as_slice() else { return Heard::Inert };
        // A ruling cites words by their stamps; a citation located by its
        // coordinates alone (an orbit's docket) names no word to rule here.
        let Some(ruled_at) = ruled.at() else { return Heard::Inert };
        let Some(addr) = self.word_admitted_at(&ruled_at).cloned() else {
            return Heard::Waits(ruled_at);
        };
        let author = r.stamp.vk;
        let verdict = match arm {
            Ruled::Upheld => RULING_UPHELD,
            Ruled::Demoted => RULING_DEMOTED,
            Ruled::Slashed => RULING_SLASHED,
            Ruled::Retired => return self.replay_retirement(&addr, rest, author, budget),
        };
        if !self.provisional.contains(addr.as_bytes()) {
            return Heard::Inert;
        }
        match self.apply_verdict_by(addr.as_bytes(), verdict, author, budget) {
            true => Heard::Applied,
            false => Heard::Refused(RulingRefused::NoParty),
        }
    }

    /// A PEER RETIRED A WORD THIS COURT HOLDS RULED. Every other citation —
    /// the rewrite the retirement rests on and every word that speaks for
    /// it — must be held here, or the ruling waits on the first that is not.
    /// The replay is this court's own: the word decomposes over its basis
    /// with the word excluded (the replay `retire_rewrite_inputs` runs),
    /// and every speaker is ruled here. Where it re-derives, the word
    /// retires here as at the home, its freed measure goes to the reserve,
    /// and this court seals its own `Ruling(Retired)` with the peer's
    /// receipt among its parents. The replay draws on the stroke of the act
    /// that heard the ruling (65.5), never one of its own.
    fn replay_retirement(
        &mut self,
        addr: &crate::transducer::address::Address,
        rest: &[crate::chronometry::Cite],
        author: [u8; 32],
        budget: &mut crate::physics::Budget,
    ) -> Heard {
        if !self.ruled_contains(addr.as_bytes()) {
            return Heard::Inert;
        }
        let Some(word) = self.basis.get(addr.as_bytes()).map(|w| w.element().clone()) else {
            return Heard::Inert;
        };
        let mut cited = Vec::with_capacity(rest.len());
        for c in rest {
            // Every citation of a retirement is a word by its stamp; one that
            // is not names nothing this court can replay the retirement on.
            let Some(at) = c.at() else { return Heard::Inert };
            match self.word_admitted_at(&at) {
                Some(a) => cited.push(a.clone()),
                None => return Heard::Waits(at),
            }
        }
        // The rewrite is held, never ruled; every other citation speaks for
        // the word and is ruled here.
        for a in &cited {
            let is_rewrite = match self.word_at(a.as_bytes()) {
                Some(w) => w.is_rewrite(),
                None => self.provisional.get(a.as_bytes()).is_some_and(|w| w.is_rewrite()),
            };
            if !is_rewrite && !self.ruled_contains(a.as_bytes()) {
                return Heard::Refused(RulingRefused::SpeakerNotRuled);
            }
        }
        if !matches!(self.reduce_excluding(&word, Some(addr.as_bytes()), budget), NormalForm::Decomposed(_)) {
            return Heard::Refused(RulingRefused::DoesNotDecompose);
        }
        let freed = word.magnitude();
        let retired = self.cite_of(addr.as_bytes());
        let elem = self.basis_remove(addr.as_bytes());
        self.ruled_admit(addr.as_bytes(), budget);
        self.cavity_book.insert(addr, elem);
        self.physics.syntropy(&freed);
        self.seal_retirement(addr.as_bytes(), retired, None, &cited, author);
        self.mark_persist();
        Heard::Applied
    }
}
