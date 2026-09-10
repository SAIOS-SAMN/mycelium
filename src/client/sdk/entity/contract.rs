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

fn hx(a: &[u8]) -> String {
    let mut s: String = a.iter().take(2).map(|b| format!("{b:02x}")).collect();
    s.push('…');
    s
}

impl Entity {
    /// RECEIVE A REFUSAL (50.9). Seals the receipt and the address, executes
    /// the move on the spoken form, holds the contracted form as an idea
    /// with its certificate, and queues the contracted sentence for the
    /// next utterance. `Quiescent` when nothing can be uttered: a malformed
    /// complex, a coordinate the form does not hold, or a form no vertex of
    /// which still acts — held inert until this mind's language moves.
    pub fn receive_adjudication(
        &mut self,
        spoken: &Element,
        kind: RefusalKind,
        at: At,
        receipt: [u8; 32],
        train: &crate::transducer::family::Family,
        task: &[u8],
    ) -> Contracted {
        use crate::transducer::family::{form_descent, mint_sentence};
        use crate::transducer::sentence::{form_vertex_key, from_element_form, rule_of_form};
        let addr = spoken.address();
        eprintln!(
            "  adjudicated {}: refused {} at {} receipt {}",
            hx(addr.as_bytes()),
            kind.label(),
            at.label(),
            hx(&receipt)
        );
        // 45.1: the refusal is an admitted act of this mind, sealed on its
        // own chain; the address is sealed in the utterance index.
        let k = self.session.seal(crate::chronometry::ActKind::Refused, addr.as_bytes());
        self.sealed.insert(&addr, k);
        let Some(mut form) = from_element_form(spoken) else {
            return self.quiescent(spoken, train);
        };
        if kind == RefusalKind::MalformedComplex {
            return self.quiescent(spoken, train);
        }
        // The move, twice: once to read the remainder the contracted form
        // leaves on this mind's own family, once with that remainder as the
        // bodies a refused word's demand stands for.
        let mut probe = form.clone();
        if probe.contract(kind, &at, &[]).is_none() {
            return self.quiescent(spoken, train);
        }
        let mut remainder: Vec<(i64, i64, u8, u8)> =
            form_descent(&probe, train, &self.epigenome).1.into_iter().flatten().collect();
        remainder.sort_unstable();
        remainder.dedup();
        if form.contract(kind, &at, &remainder) != Some(Contracted::Moved) {
            return self.quiescent(spoken, train);
        }
        let after = form_vertex_key(&form);
        let certificate = Contraction { kind, at: at.clone(), before: addr.clone(), after: after.clone(), receipt };
        eprintln!(
            "  contracted {} → {} by {} at {}",
            hx(addr.as_bytes()),
            hx(after.as_bytes()),
            kind.label(),
            at.label()
        );
        let rems = form_descent(&form, train, &self.epigenome).1;
        self.hold_form(&form, train, &rems);
        self.contractions.insert(&after, certificate.clone());
        if !form.acts() {
            let chart = crate::transducer::stalk::invariant(&remainder);
            eprintln!(
                "  quiescent {}: held inert on chart {}",
                hx(after.as_bytes()),
                hx(chart.key().as_bytes())
            );
            self.contemplation.dirty.remove(&after);
            return Contracted::Quiescent;
        }
        match mint_sentence(&rule_of_form(form), train, &self.epigenome) {
            Some(sentence) => {
                self.contracted.push(Pending { sentence, train: train.to_vec(), task: task.to_vec(), certificate });
            }
            None => eprintln!(
                "  contracted {} withheld: no measured descent and no demand",
                hx(after.as_bytes())
            ),
        }
        Contracted::Moved
    }

    /// Structural collapse (50.9 move 4): the form is held as an inert
    /// idea, not dirty, and nothing is re-spoken until the language moves.
    fn quiescent(&mut self, spoken: &Element, train: &crate::transducer::family::Family) -> Contracted {
        use crate::transducer::sentence::{form_vertex_key, from_element_form};
        let addr = spoken.address();
        let chart = crate::transducer::stalk::invariant_of(spoken);
        eprintln!(
            "  quiescent {}: held inert on chart {}",
            hx(addr.as_bytes()),
            hx(chart.key().as_bytes())
        );
        if let Some(f) = from_element_form(spoken) {
            self.hold_form(&f, train, &[]);
            self.contemplation.dirty.remove(&form_vertex_key(&f));
        }
        Contracted::Quiescent
    }

    /// UTTER THE CONTRACTED SENTENCES (50.9): each one spoken to the court
    /// with its certificate, over the family and task it was minted for.
    /// One this mind could not speak stays pending.
    pub fn speak_contracted(&mut self) -> Vec<(crate::transducer::address::Address, u8)> {
        let pending = std::mem::take(&mut self.contracted);
        let mut out = Vec::with_capacity(pending.len());
        for p in pending {
            let addr = p.sentence.address();
            match self.speak_with(std::slice::from_ref(&p.sentence), &p.train, &p.task) {
                Ok(codes) => {
                    if let [code] = codes.as_slice() {
                        out.push((addr, *code));
                    }
                }
                Err(_) => self.contracted.push(p),
            }
        }
        out
    }

    /// UTTER WHAT THIS MIND OWES (21 step 3, chapter 6's depth order): for
    /// every form a court answered `Awaiting` on, the cited form first and
    /// the citer after it, in the same frame and in that order. A cited
    /// form must precede its citer, so one frame carries both and the
    /// court rules the generator before it hears the sentence that
    /// derives through it.
    pub fn speak_owed(&mut self) -> Vec<(crate::transducer::address::Address, u8)> {
        let owed = std::mem::take(&mut self.owed);
        let mut out = Vec::with_capacity(owed.len());
        for o in owed {
            let citer = o.citer.address();
            let both = [o.form.clone(), o.citer.clone()];
            match self.speak_with(&both, &o.train, &o.task) {
                Ok(codes) => {
                    if let [_, code] = codes.as_slice() {
                        out.push((citer, *code));
                    }
                }
                // The court is not reachable this stroke: still owed.
                Err(_) => self.owed.push(o),
            }
        }
        out
    }

    /// How many cited forms wait to be taught to a court.
    pub fn owed_pending(&self) -> usize {
        self.owed.len()
    }

    /// The certificate a sentence cites: the contraction that produced the
    /// form it was minted from, read by the form's vertex key.
    pub(crate) fn certificate_for(&self, e: &Element) -> Option<Contraction> {
        let f = crate::transducer::sentence::from_element_form(e)?;
        self.contractions.get(&crate::transducer::sentence::form_vertex_key(&f)).cloned()
    }

    /// The K at which this mind sealed a refused address, if it did.
    pub fn sealed_at(&self, addr: &[u8]) -> Option<u64> {
        self.sealed.get(addr).copied()
    }

    /// The contracted sentences waiting to be uttered, taken.
    pub fn take_contracted(&mut self) -> Vec<Pending> {
        std::mem::take(&mut self.contracted)
    }

    /// How many contracted sentences wait to be uttered.
    pub fn contracted_pending(&self) -> usize {
        self.contracted.len()
    }
}
