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
    pub(crate) fn stand_calluses(&mut self) {
        for c in self.epigenome.take_calluses() {
            self.stand_callus(c);
        }
    }

    /// THE RAY SPELLED IN THE HELD CONSTRUCTORS — the seventeen and the
    /// letters this mind adjoined (78.29(A)) — as a derivation that SPELLS,
    /// never adjoins. `None` where they do not spell it.
    fn spell_held(&mut self, ray: &crate::transducer::alphabet::Rotor) -> Option<crate::transducer::alphabet::Derivation> {
        let v = crate::transducer::element::identity_versor();
        let mut budget = self.physics.stroke();
        let adjoined = crate::transducer::family::derivation::adjoined_basis_target(
            &self.epigenome,
            &mut budget,
            &v,
            Some(ray),
            &crate::transducer::sentence::CycleType::Identity,
        )?;
        match crate::transducer::family::derivation::peel_versor_with_target(&v, Some(ray), adjoined)? {
            Ok((cited, word)) => Some(crate::transducer::alphabet::Derivation::factored(cited, word, ray.clone())),
            Err(_) => None,
        }
    }

    fn stand_callus(&mut self, c: crate::client::sdk::epigenome::remainder::Callus) {
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
        let Some(derivation) = self.spell_held(&c.ray) else {
            // Two vantages agreeing on a class no held word spells: CS2's
            // demand with its second vantage in hand, up through the docket.
            self.epigenome.calluses.unspelled += 1;
            let charts: Vec<Invariant> = cover.scoped().filter_map(|(k, _)| Invariant::from_key(&k)).collect();
            self.contemplation.docket_demand(&c.class, &charts);
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
            class: c.class,
            rotor: c.ray,
            cover: Some(cover.scoped().map(|(k, _)| k).collect()),
        });
        let Some(at) = self.session.voice().chain().next_stamp(&[]) else {
            self.epigenome.calluses.unminted += 1;
            return;
        };
        cites.insert(0, crate::chronometry::Cite::word(&callus, at));
        let k = self.session.seal(crate::chronometry::ActKind::Stood, callus.magnitude(), cites);
        if self.session.voice().chain().at_place(at.place()).is_none() {
            self.epigenome.calluses.unminted += 1;
            return;
        }
        self.session.voice_mut().file_admitted(&callus.address(), at);
        let tu = self.contemplation.now_k();
        let _ = self.epigenome.stand_dialect_at(callus, k, tu);
        self.epigenome.calluses.stood += 1;
    }
}
