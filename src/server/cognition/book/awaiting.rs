// 21 step 3, 50.2 step 2 — THE AWAITING DOCKET: a sentence that cites a
// form this court has not ruled waits here instead of being refused, and
// is heard again when that address is ruled.
//
//! A voice one syllabus step ahead of this court's cover is not wrong.
//! Refused, it would seal the address on its own chain (50.9) and lose
//! the sentence permanently for being early. Split from `speech.rs` at
//! the §1.1 600-line reading bound; the items are moved, not rewritten
//! (§1.6).

use crate::cognition::*;
use crate::transducer::element::Element;

impl Kernel {
    /// How many sentences wait on one address. A read for the tests and
    /// for the instrument (45.9): the docket's own size at a coordinate,
    /// never a total and never a branch.
    #[cfg(test)]
    pub(crate) fn awaiting_len(&self, form: &[u8]) -> usize {
        match self.awaiting.get(form) {
            Some(b) => b.len(),
            None => 0,
        }
    }

    /// 21 step 3, 50.2 step 2 — THE FIRST CITED FORM THIS COURT HAS NOT
    /// RULED, or none. Every citation is a keyed read of `ruled_contains`
    /// and the read is over the sentence's OWN vertices — its boundary,
    /// which rule 1 allows — never over the cover. Both levels are read
    /// here: a `Glued` vertex citing a whole form, and a word vertex's
    /// derivation citing a `Word` vertex inside one. They are one
    /// mechanism, so they wait the same way.
    /// THE CITATIONS ARE READ OFF THE ENVELOPE, NOT DECODED (54.5 Road (d),
    /// B3). This parsed the sentence's whole tape to read its vertices for
    /// citations, and the citations were never geometry: the nerve's
    /// 1-simplices are `⟨A_i, A_j, φ⟩` and a vertex's generators are its
    /// presentation. Both ride `Element.receipts`, which is a flat list and
    /// needs no decode at all — so the door's read is now `O(citations)`
    /// with no form built, on the one path every arriving sentence takes.
    pub(crate) fn unruled_citation(
        &self,
        s: &Element,
    ) -> Option<crate::transducer::address::Address> {
        let env = s.receipts();
        for e in &env.nerve {
            if !self.ruled_contains(e.target.as_bytes()) {
                return Some(e.target.clone());
            }
        }
        for cite in &env.derivations {
            for (form, _, _) in &cite.generators {
                if !self.ruled_contains(form.as_bytes()) {
                    return Some(form.clone());
                }
            }
        }
        None
    }

    /// Docket a sentence under the cited address it waits on. Keyed by
    /// the sentence's own address inside that bucket, so a voice that
    /// speaks the same sentence twice waits once.
    pub(crate) fn await_citation(
        &mut self,
        voice: [u8; 32],
        task: &[u8],
        s: &Element,
        form: &crate::transducer::address::Address,
    ) {
        let addr = s.address();
        let entry = (voice, crate::transducer::address::Address::from(task), s.clone());
        match self.awaiting.get_mut(form.as_bytes()) {
            Some(bucket) => {
                if bucket.iter().any(|(_, _, held)| held.address() == addr) {
                    return;
                }
                bucket.push(entry);
            }
            None => {
                self.awaiting.insert(form.clone(), vec![entry]);
            }
        }
        // 76 — A SENTENCE WAITING ON A WORD IS THAT WORD'S PRESSURE: its frame
        // is transported to the word's escrow and the word is carried again,
        // where this court holds it in flight.
        self.press(form.as_bytes(), s.magnitude());
    }

    /// THE WORD THEY WAITED ON LAPSED: every sentence waiting on it is
    /// released as the demand it is — it cites a word this court does not
    /// hold, D (78.9, GLUED_UNRULED) — answered and recorded, so the voice's
    /// next utterance can contract against it (50.9).
    pub(crate) fn release_awaiting(&mut self, form: &[u8]) {
        let Some(waiting) = self.awaiting.remove(form) else {
            return;
        };
        for (voice, _task, s) in waiting {
            let refusal = crate::court::verdict::Refusal::of_tape(crate::verdict::REFUSE_GLUED_UNRULED, &s);
            let _ = self.refuse_sentence(voice, &s.address(), refusal);
        }
    }

    /// A form was ruled: every sentence that waited on it is heard again,
    /// in the order it arrived. A sentence that waits on a SECOND unruled
    /// citation re-dockets under that one, so a chain of citations
    /// resolves one link per ruling and never spins (rule 10: the bucket
    /// is taken before it is walked, so a re-docket under the same
    /// address cannot be re-entered in this pass).
    pub(crate) fn rehear_awaiting(&mut self, form: &[u8]) {
        let Some(waiting) = self.awaiting.remove(form) else {
            return;
        };
        for (voice, task, s) in waiting {
            // Through `adjudicate_one`, not `hear_sentence`. A re-hearing
            // is an adjudication like any other: a refusal on it must be
            // RECORDED in `refusals` with its receipt, or the voice's next
            // utterance cannot cite it as a contraction certificate (50.9)
            // and the refusal exists only in a log line. Dropping the
            // outcome here made the second hearing the one hearing whose
            // verdict nobody kept.
            let _ = self.adjudicate_one(voice, task.as_bytes(), &s, None);
        }
    }

}
