// 78.37(4), RM1 — THE CALLUS STOOD: one class perceived at two incomparable
// vantages with one ray, spelled in the held constructors and stood by the
// voice; where nothing held spells it, its class goes up as a demand.

use crate::client::sdk::*;
use crate::transducer::stalk::Invariant;

/// THE TOP OF A CHART CHAIN: the member every other member is a prefix of —
/// the deepest by Invariant prefix depth, read off the prefixes and never
/// off key order. `None` where a member does not read or no member is the
/// top.
fn top_of(chain: &[Vec<u8>]) -> Option<Invariant> {
    let invs: Vec<Invariant> = chain.iter().map(|k| Invariant::from_key(k)).collect::<Option<_>>()?;
    let tops: Vec<&Invariant> = invs
        .iter()
        .filter(|m| {
            let ps: Vec<Vec<u8>> = m.prefixes().iter().map(|p| p.key().into_vec()).collect();
            invs.iter().all(|o| ps.contains(&o.key().into_vec()))
        })
        .collect();
    let [top] = tops.as_slice() else { return None };
    Some((*top).clone())
}

impl Mind {
    /// RM1 — THE CALLUSES THE RECEPTOR RAISED, each stood or refused by name.
    /// Run at the orbit's close, before its docket is cited, so a class no
    /// held word spells goes up in this orbit's plan.
    ///
    /// ONE STROKE'S BUDGET FOR THE CLOSE, as every arm of the stroke draws
    /// its own (`pivot_thoughts`, the glue): the peels of all the calluses
    /// spend it together, never a fresh budget per callus. A callus the
    /// spent budget cannot spell waits, pending, for the next close.
    pub(crate) fn stand_calluses(&mut self) {
        let mut budget = self.physics.stroke();
        let mut pending = self.epigenome.take_calluses().into_iter();
        for c in pending.by_ref() {
            if budget.remaining() == 0 {
                self.epigenome.calluses_pending.push(c);
                break;
            }
            self.stand_callus(c, &mut budget);
        }
        self.epigenome.calluses_pending.extend(pending);
        // Door (b), on the same budget: the demands a discharge answered.
        self.refill_answered(&mut budget);
    }

    /// THE RAY SPELLED IN THE HELD CONSTRUCTORS — the seventeen and the
    /// letters this mind adjoined (78.29(A)) — as a derivation that SPELLS,
    /// never adjoins, on the close's budget. `None` where they do not spell
    /// it.
    fn spell_held(
        &mut self,
        ray: &crate::transducer::alphabet::Rotor,
        budget: &mut crate::physics::Budget,
    ) -> Option<crate::transducer::alphabet::Derivation> {
        let v = crate::transducer::element::identity_versor();
        let adjoined = crate::transducer::family::derivation::adjoined_basis_target(
            &self.epigenome,
            budget,
            &v,
            Some(ray),
            &crate::transducer::sentence::CycleType::Identity,
        )?;
        match crate::transducer::family::derivation::peel_versor_with_target(&v, Some(ray), adjoined)? {
            Ok((cited, word)) => Some(crate::transducer::alphabet::Derivation::factored(cited, word, ray.clone())),
            Err(_) => None,
        }
    }

    fn stand_callus(&mut self, c: crate::client::sdk::epigenome::remainder::Callus, budget: &mut crate::physics::Budget) {
        let Some(tops) = c.chains.iter().map(|ch| top_of(ch)).collect::<Option<Vec<Invariant>>>() else {
            self.epigenome.calluses.untopped += 1;
            return;
        };
        // One cover for the act and the demand: both chains' charts, each
        // once, in key order.
        let mut cover: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for ch in &c.chains {
            for k in ch {
                cover.insert(k, ());
            }
        }
        let Some(derivation) = self.spell_held(&c.ray, budget) else {
            // Two vantages agreeing on a class no held word spells: CS2's
            // demand with its second vantage in hand, up through the docket.
            self.epigenome.calluses.unspelled += 1;
            let charts: Vec<Invariant> = cover.scoped().filter_map(|(k, _)| Invariant::from_key(&k)).collect();
            // Its residue is the two remainders' ray. 78.47(6) kept it off the
            // CITATION because a chain reached it on RM1's leaf; the plan is
            // volatile and written nowhere since 78.49, so withheld here it
            // reached nothing — 141–380 demands a voice the taker could not
            // construct (read live, 2026-10-06). The plan carries the ray.
            let residue = crate::transducer::element::versor_of_word(&crate::transducer::alphabet::Word::formed(
                c.ray.as_ref().clone(),
            ));
            self.contemplation.docket_demand(&c.class, &charts, residue.as_ref());
            return;
        };
        let Some(callus) = crate::transducer::sentence::callus_form(derivation, &tops) else {
            self.epigenome.calluses.unminted += 1;
            return;
        };
        let callus = self.stand_word(callus);
        self.epigenome.calluses.stood += 1;
        // RM1 (c), door (a) — THE CALLUS SPELLS THE TWIST: its two remainder
        // words re-fill, each at the chain it was perceived on.
        for (w, chain) in c.words.iter().zip(&c.chains) {
            self.refill(w, &c.class, chain, &callus);
        }
    }

    /// A MIND'S OWN WORD STOOD (78.49): an act of this mind, stood in the
    /// dialect at its own K. Nothing is sealed — a court admits it, or not,
    /// when it is spoken.
    fn stand_word(&mut self, word: crate::transducer::element::Element) -> crate::transducer::element::Element {
        let k = self.act();
        let tu = self.contemplation.now_k();
        let _ = self.epigenome.stand_dialect_at(word.clone(), k, tu);
        word
    }

    /// RM1 (c), 78.49 — THE RE-FILL, an act of this mind: the held word's
    /// open remainder of `class` at `at` is RE-SPELLED by `spelling`, marked
    /// in the mind by the word that spells it — the record's `answered` is a
    /// court's seal and is left to the court. Refused, counted, where the
    /// record carries no open remainder there.
    pub(crate) fn refill(
        &mut self,
        word: &crate::transducer::address::Address,
        class: &crate::transducer::sentence::ClassVector,
        at: &[Vec<u8>],
        spelling: &crate::transducer::element::Element,
    ) -> bool {
        match self.epigenome.mark_refilled(word.as_bytes(), class, at, &spelling.address()) {
            true => {
                self.epigenome.calluses.refilled += 1;
                true
            }
            false => {
                self.epigenome.calluses.refill_refused += 1;
                false
            }
        }
    }

    /// RM1 (c), door (b) — A DEMAND ANSWERED BY CLASS KEY: every open
    /// remainder of that class is re-spelled in the held constructors (the
    /// letter now among them), and each that spells re-fills, citing the
    /// spelling stood as the mind's own word at the remainder's chart.
    fn refill_answered(&mut self, budget: &mut crate::physics::Budget) {
        let classes = std::mem::take(&mut self.contemplation.demands_answered);
        for class in classes {
            for (word, chain, ray) in self.epigenome.open_remainders_of(&class) {
                if budget.remaining() == 0 {
                    return;
                }
                let Some(derivation) = self.spell_held(&ray, budget) else { continue };
                let Some(top) = top_of(&chain) else {
                    self.epigenome.calluses.untopped += 1;
                    continue;
                };
                let Some(form) = crate::transducer::sentence::callus_form(derivation, std::slice::from_ref(&top)) else {
                    self.epigenome.calluses.unminted += 1;
                    continue;
                };
                let spelling = self.stand_word(form);
                self.refill(&word, &class, &chain, &spelling);
            }
        }
    }
}
