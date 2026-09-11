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
    /// and the walk is over the sentence's OWN vertices — its boundary,
    /// which rule 1 allows — never over the cover. Both levels are read
    /// here: a `Glued` vertex citing a whole form, and a word vertex's
    /// derivation citing a `Word` vertex inside one. They are one
    /// mechanism, so they wait the same way.
    pub(crate) fn unruled_citation(
        &self,
        s: &Element,
    ) -> Option<crate::transducer::address::Address> {
        use crate::transducer::sentence::Vertex;
        // An ARRIVING sentence, not one this court holds: it has no held
        // word to read a form off, so the decode is named here (L16).
        let f = crate::transducer::sentence::from_element_form(s)?;
        for v in &f.vertices {
            match v {
                Vertex::Glued { address, .. } if !self.ruled_contains(address.as_bytes()) => {
                    return Some(address.clone())
                }
                Vertex::Word { derivation, .. } => {
                    for (form, _, _) in &derivation.generators {
                        if !self.ruled_contains(form.as_bytes()) {
                            return Some(form.clone());
                        }
                    }
                }
                _ => {}
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
            let a = s.address();
            eprintln!(
                "  court speech reheard {:02x}{:02x}…: cited {:02x}{:02x}… is ruled now",
                a[0], a[1], form[0], form[1]
            );
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
