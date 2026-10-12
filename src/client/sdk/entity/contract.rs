// 50.9 — THE CONTRACTION ON THE MIND: what a court's refusal does to the
// sentence this mind spoke, the address it seals, and the sentence it
// utters next.
//
//! A refusal is an admitted act of the mind (45.1): a receipt is sealed on
//! its own chain and the refused address is sealed in the utterance index,
//! so the identical tape cannot leave again. The held form is contracted
//! by the move the refusal names (`Form::contract`), held as an idea with
//! its certificate, and — where a vertex still acts — minted as the next
//! utterance, citing the receipt.

use crate::client::sdk::*;
use crate::transducer::sentence::{At, Contracted, Contraction, RefusalKind};

impl Entity {
    /// RECEIVE A REFUSAL (50.9). Seals the receipt and the address, executes
    /// the move on the spoken form, holds the contracted form as an idea
    /// with its certificate, and queues the contracted sentence for the
    /// next utterance. `Quiescent` when nothing can be uttered: a malformed
    /// complex, a coordinate the form does not hold, or a form no vertex of
    /// which still acts — held inert until this mind's language moves.
    #[expect(clippy::too_many_arguments, reason = "the refusal's three coordinates, the sentence, its family and task, and the stroke of the act that heard it (65.5)")]
    pub(crate) fn receive_adjudication(
        &mut self,
        spoken: &Element,
        kind: RefusalKind,
        at: At,
        receipt: [u8; 32],
        train: &crate::transducer::family::Family,
        task: &[u8],
        budget: &mut crate::physics::Budget,
    ) -> Contracted {
        use crate::transducer::family::{form_deflation, mint_sentence};
        use crate::transducer::sentence::{form_vertex_key, rule_of_form};
        let addr = spoken.address();
        // 78.49: the refusal is an act of this mind at its own K; the address
        // is sealed in the utterance index. The court sealed the refusal.
        let k = self.act();
        self.sealed.insert(&addr, k);
        // 24.2: the dialect is what this mind spoke THAT STOOD. The refused
        // sentence stood there before it was uttered and did not stand at
        // the court; it leaves, so no form glues it and no sentence cites
        // it again — a citer of a sealed address waits at every court.
        self.epigenome.drop_dialect(addr.as_bytes());
        self.owed_tries.remove(addr.as_bytes());
        let Some(mut form) = crate::transducer::sentence::from_element_form(spoken) else {
            return self.quiescent(spoken, train, budget);
        };
        if kind == RefusalKind::MalformedComplex {
            return self.quiescent(spoken, train, budget);
        }
        // The move, twice: once to read the remainder the contracted form
        // leaves on this mind's own family, once with that remainder as the
        // bodies a refused word's demand stands for.
        let mut probe = form.clone();
        if probe.contract(kind, &at, &[]).is_none() {
            return self.quiescent(spoken, train, budget);
        }
        let mut remainder: Vec<(i64, i64, u8, u8)> =
            form_deflation(&probe, train, &self.epigenome).1.into_iter().flatten().collect();
        remainder.sort_unstable();
        remainder.dedup();
        if form.contract(kind, &at, &remainder) != Some(Contracted::Moved) {
            return self.quiescent(spoken, train, budget);
        }
        let after = form_vertex_key(&form);
        let certificate = Contraction { kind, at: at.clone(), before: addr.clone(), after: after.clone(), receipt };
        let rems = form_deflation(&form, train, &self.epigenome).1;
        // The form is handed over (Z4 item 1) and what this arm needs of it
        // afterwards is read from the value it still has here, before the
        // move: `acts` and the remainder are read below off `form`, so the
        // hold takes a clone at this one site and the other does not.
        self.hold_form(form.clone(), train, &rems, budget);
        self.contractions.insert(&after, certificate.clone());
        if !form.acts() {
            self.contemplation.dirty.remove(&after);
            return Contracted::Quiescent;
        }
        // An unmeasurable span is not a zero: the contraction is not minted
        // against an extent this mind cannot read (rule 11).
        let minted = self
            .contemplation
            .span_of(&form)
            .and_then(|d| mint_sentence(&rule_of_form(form), train, &self.epigenome, d.span()));
        // No measured trajectory and no demand: the contraction is held with
        // its certificate and nothing is uttered.
        if let Some(sentence) = minted {
            let minted_addr = sentence.address();
            self.contractions.insert(&minted_addr, certificate.clone());
            self.contracted.push(Pending { sentence, train: train.to_vec(), task: task.to_vec(), certificate });
        }
        Contracted::Moved
    }

    /// Structural collapse (50.9 move 4): the form is held as an inert
    /// idea, not dirty, and nothing is re-spoken until the language moves.
    fn quiescent(
        &mut self,
        spoken: &Element,
        train: &crate::transducer::family::Family,
        budget: &mut crate::physics::Budget,
    ) -> Contracted {
        use crate::transducer::sentence::form_vertex_key;
        if let Some(f) = crate::transducer::sentence::from_element_form(spoken) {
            // The key is the form's own coordinate and is taken BEFORE the
            // form is handed over, so nothing is cloned to read it back.
            let key = form_vertex_key(&f);
            self.hold_form(f, train, &[], budget);
            self.contemplation.dirty.remove(&key);
        }
        Contracted::Quiescent
    }

    /// UTTER THE CONTRACTED SENTENCES (50.9): each one spoken to the court
    /// with its certificate, over the family and task it was minted for.
    /// One the court could not be reached for is pending again when the
    /// answer lands.
    pub(crate) fn speak_contracted(&mut self) {
        let pending = std::mem::take(&mut self.contracted);
        for p in pending {
            let addr = p.sentence.address();
            self.contractions.insert(&addr, p.certificate.clone());
            let (sentence, train, task) = (p.sentence.clone(), p.train.clone(), p.task.clone());
            self.speak_with(std::slice::from_ref(&sentence), &train, &task, move |me, heard| {
                if heard.is_err() {
                    me.contracted.push(p);
                }
            });
        }
    }

    /// UTTER WHAT THIS MIND OWES (21 step 3, chapter 6's depth order): for
    /// every form a court answered `Awaiting` on, the cited form first and
    /// the citer after it, in the same frame and in that order. A cited
    /// form must precede its citer, so one frame carries both and the
    /// court rules the generator before it hears the sentence that
    /// derives through it.
    ///
    /// ONE STROKE'S WORTH PER PASS. `mem::take` drained the whole queue,
    /// and each entry is a frame to a court carrying two elements and a
    /// family — on a backlog that is one stroke spending the thread on
    /// network round trips and nothing else. The queue is durable across
    /// strokes, so what is not taught this pass is taught next (rule 10).
    pub(crate) fn speak_owed(&mut self) {
        let mut budget = self.physics.stroke();
        let mut owed = std::mem::take(&mut self.owed);
        let keep = owed.split_off(owed.len().min(budget.remaining()));
        self.owed = keep;
        for o in owed {
            if !budget.spend() {
                self.owed.push(o);
                continue;
            }
            // A cited form this mind sealed cannot be taught to any court:
            // the frame would refuse it here and carry the citer alone.
            if self.sealed.get(o.form.address().as_bytes()).is_some() {
                let _ = crate::transducer::sentence::Refused::CitedSealed.noted();
                continue;
            }
            let both = [o.form.clone(), o.citer.clone()];
            let (train, task) = (o.train.clone(), o.task.clone());
            self.speak_with(&both, &train, &task, move |me, heard| {
                // The court was not reachable: still owed.
                if heard.is_err() {
                    me.owed.push(o);
                }
            });
        }
    }

    /// How many cited forms wait to be taught to a court.
    #[cfg(test)]
    pub(crate) fn owed_pending(&self) -> usize {
        self.owed.len()
    }

    /// A COURT ANSWERED `Awaiting` (21 step 3, 50.9): it has not ruled the
    /// form the citer cites. The mind owes that court the cited form before
    /// the citer, and queues both — unless the cited address is one this
    /// mind sealed, which no court will rule; unless the mind does not hold
    /// the cited form, which is a fault of the utterance; or unless the
    /// citer has already been owed to every court this mind knows, since
    /// the owed frame goes to the courts in turn and a citer no court can
    /// stand waits for its family to come round again. Queued, or the
    /// refusal by name, counted where it forms (A.17, 78.9).
    pub(crate) fn owe_citation(
        &mut self,
        citer: &Element,
        form: &crate::transducer::address::Address,
        train: &crate::transducer::family::Family,
        task: &[u8],
    ) -> Result<(), crate::transducer::sentence::Refused> {
        use crate::transducer::sentence::Refused;
        let addr = citer.address();
        if self.sealed.get(form.as_bytes()).is_some() {
            return Err(Refused::CitedSealed.noted());
        }
        let Some(cited) = self.epigenome.working_word(form.as_bytes()).map(|h| h.element().clone()) else {
            return Err(Refused::CitedNotHeld.noted());
        };
        let courts = self.hypha.courts();
        let tried = match self.owed_tries.get(addr.as_bytes()) {
            Some(n) => *n,
            None => 0,
        };
        if tried >= courts {
            self.owed_tries.remove(addr.as_bytes());
            return Err(Refused::OwedToEveryCourt.noted());
        }
        self.owed_tries.insert(addr.as_bytes(), tried + 1);
        self.owed.push(crate::client::sdk::Owed {
            form: cited,
            citer: citer.clone(),
            train: train.to_vec(),
            task: task.to_vec(),
        });
        Ok(())
    }

    /// The certificate a sentence cites: the contraction that produced the
    /// form it was minted from, read by the minted sentence's address or its
    /// form's vertex key.
    pub(crate) fn certificate_for(&self, e: &Element) -> Option<Contraction> {
        let addr = e.address();
        if let Some(c) = self.contractions.get(&addr).cloned() {
            return Some(c);
        }
        let k = crate::transducer::sentence::form_vertex_key(
            &crate::transducer::sentence::from_element_form(e)?,
        );
        self.contractions.get(&k).cloned()
    }

    /// The K at which this mind sealed a refused address, if it did.
    #[cfg(test)]
    pub(crate) fn sealed_at(&self, addr: &[u8]) -> Option<u64> {
        self.sealed.get(addr).copied()
    }

    /// The contracted sentences waiting to be uttered, taken.
    #[cfg(test)]
    pub(crate) fn take_contracted(&mut self) -> Vec<Pending> {
        std::mem::take(&mut self.contracted)
    }

    /// How many contracted sentences wait to be uttered.
    #[cfg(test)]
    pub(crate) fn contracted_pending(&self) -> usize {
        self.contracted.len()
    }
}
