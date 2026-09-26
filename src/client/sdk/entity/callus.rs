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

impl Entity {
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
        // once, in their own key order.
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
            // Its residue is the two remainders' ray, already on RM1's leaf
            // the chain reaches: the citation carries none beside it (78.47(6)).
            self.contemplation.docket_demand(&c.class, &charts, None);
            return;
        };
        let Some(callus) = crate::transducer::sentence::callus_form(derivation, &tops) else {
            self.epigenome.calluses.unminted += 1;
            return;
        };
        // The act cites the callus at its own stamp, the two remainder words
        // at theirs, and the class with its ray and both chains' charts.
        let mut cites = Vec::new();
        for w in &c.words {
            let cite = self
                .epigenome
                .held_element(w.as_bytes())
                .and_then(|e| self.session.voice().cite_of(w.as_bytes(), &e));
            match cite {
                Some(ci) => cites.push(ci),
                None => {
                    self.epigenome.calluses.uncited += 1;
                    return;
                }
            }
        }
        cites.push(crate::chronometry::Cite::Class {
            class: c.class.clone(),
            rotor: c.ray.clone(),
            cover: Some(cover.scoped().map(|(k, _)| k).collect()),
        });
        let Some(callus) = self.stand_word(callus, cites) else {
            self.epigenome.calluses.unminted += 1;
            return;
        };
        self.epigenome.calluses.stood += 1;
        // RM1 (c), door (a) — THE CALLUS SPELLS THE TWIST: its two remainder
        // words re-fill, each at the chain it was perceived on.
        for (w, chain) in c.words.iter().zip(&c.chains) {
            self.refill(w, &c.class, &c.ray, chain, &callus);
        }
    }

    /// A MIND'S OWN WORD STOOD: sealed Stood at its own stamp beside `cites`,
    /// admitted at that stamp and stood in the dialect. `None` where the
    /// line refused the act.
    fn stand_word(
        &mut self,
        word: crate::transducer::element::Element,
        mut cites: Vec<crate::chronometry::Cite>,
    ) -> Option<crate::transducer::element::Element> {
        let at = self.session.voice().chain().next_stamp(&[])?;
        cites.insert(0, crate::chronometry::Cite::word(&word, at));
        let k = self.session.seal(crate::chronometry::ActKind::Stood, word.magnitude(), cites);
        self.session.voice().chain().at_place(at.place())?;
        self.session.voice_mut().file_admitted(&word.address(), at);
        let tu = self.contemplation.now_k();
        let _ = self.epigenome.stand_dialect_at(word.clone(), k, tu);
        Some(word)
    }

    /// RM1 (c) — THE RE-FILL, an act on the voice's line: SETTLED — the
    /// ledger's closure kind — citing the rewrite word, its remainder
    /// (`Cite::Class {κ, g, cover: its chain}`) and the word that spells the
    /// twist; its μ⃗ the pressure answered. The record's remainder is
    /// answered at the act's stamp, so its pressure reads zero from there.
    /// Refused, counted, where the record carries no open remainder there,
    /// a word is not cited, or the line refused the act.
    pub(crate) fn refill(
        &mut self,
        word: &crate::transducer::address::Address,
        class: &crate::transducer::sentence::ClassVector,
        ray: &crate::transducer::alphabet::Rotor,
        at: &[Vec<u8>],
        spelling: &crate::transducer::element::Element,
    ) -> bool {
        let rewrite = self.epigenome.held_element(word.as_bytes());
        let mu = rewrite.as_ref().and_then(|e| e.open_pressure(class, at));
        let cites = rewrite.as_ref().and_then(|e| {
            let voice = self.session.voice();
            Some([voice.cite_of(word.as_bytes(), e)?, voice.cite_of(spelling.address().as_bytes(), spelling)?])
        });
        let (Some(mu), Some([cw, cs])) = (mu, cites) else {
            self.epigenome.calluses.refill_refused += 1;
            return false;
        };
        let class_cite = crate::chronometry::Cite::Class {
            class: Box::new(class.clone()),
            rotor: Box::new(ray.clone()),
            cover: Some(at.to_vec()),
        };
        let before = self.session.voice().chain().head_stamp();
        self.session.seal(crate::chronometry::ActKind::Settled, mu, vec![cw, class_cite, cs]);
        let Some(stamp) = self.session.voice().chain().head_stamp().filter(|s| Some(*s) != before) else {
            self.epigenome.calluses.refill_refused += 1;
            return false;
        };
        match self.epigenome.answer_remainder(word.as_bytes(), class, at, stamp) {
            Some(_) => {
                self.epigenome.calluses.refilled += 1;
                true
            }
            None => {
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
                let cite = crate::chronometry::Cite::Class {
                    class: Box::new(class.clone()),
                    rotor: Box::new(ray.clone()),
                    cover: Some(chain.clone()),
                };
                let Some(spelling) = self.stand_word(form, vec![cite]) else {
                    self.epigenome.calluses.unminted += 1;
                    continue;
                };
                self.refill(&word, &class, &ray, &chain, &spelling);
            }
        }
    }
}
