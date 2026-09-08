// A19, 26 — WHAT THIS COURT SERVES: the ruled words in ruling order, the
// class records that travel as coordinates, and the digests a peer reads.

use crate::cognition::*;

impl Kernel {
    /// Charts this hypha covers: Twisted or Cavity listings, per occupied lane.
    /// Never the whole pending book — that clone was the last deploy's leak.
    pub fn own_cover_charts(&self) -> Vec<(u8, crate::transducer::stalk::Invariant)> {
        // index: this hypha's own cover trie (signed digest payload)
        self.own_cover.all_charts()
    }

    pub fn complexes(&self) -> &crate::network::nerve::Keyed<crate::transducer::cech::Complex> {
        &self.complexes
    }

    /// Ideology boundary on a chart: classes at every degree the mesh holds
    /// there, including coarser prefixes, so a mind can be taught the frame
    /// its bubble sits on.
    pub fn classes_on(
        &self,
        inv: &crate::transducer::stalk::Invariant,
    ) -> Vec<(crate::transducer::alphabet::Rotor, u8)> {
        let mut out = Vec::new();
        for p in inv.prefixes() {
            if let Some(cx) = self.complexes.get(&p.key()) {
                for (r, d) in cx.classes() {
                    out.push((r, d as u8));
                }
            }
        }
        out
    }

    pub fn cochain_bytes(&self) -> usize {
        let mut n = 0usize;
        for (_, c) in self.complexes.fiber(crate::network::nerve::ROOT) {
            n += c.byte_len();
        }
        n
    }

    /// Class records on `inv` (and prefixes), or on every chart when `inv` is empty.
    pub fn class_records(
        &self,
        inv: &crate::transducer::stalk::Invariant,
    ) -> Vec<crate::anastome::dialogue::ClassPage> {
        let mut out = Vec::new();
        let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        let push = |out: &mut Vec<crate::anastome::dialogue::ClassPage>,
                    seen: &mut crate::network::nerve::Keyed<()>,
                    r: &crate::transducer::alphabet::Rotor,
                    order: u32| {
            let Some(k) = r.class_key() else {
                return;
            };
            if seen.contains(&k) {
                return;
            }
            seen.insert(k.clone(), ());
            let mut rotor = Vec::new();
            r.to_bytes(&mut rotor);
            out.push(crate::anastome::dialogue::ClassPage {
                key: k,
                order,
                rotor,
            });
        };
        if inv.0.is_empty() {
            for (_, cx) in self.complexes.fiber(crate::network::nerve::ROOT) {
                for layer in &cx.degrees {
                    for (_, cell) in layer.fiber(crate::network::nerve::ROOT) {
                        match &cell.fill {
                            crate::transducer::cech::Fill::Congruent(r, ell) => {
                                push(&mut out, &mut seen, r, *ell)
                            }
                            crate::transducer::cech::Fill::Class(r) => {
                                push(&mut out, &mut seen, r, 0)
                            }
                            _ => {}
                        }
                    }
                }
            }
        } else {
            for p in inv.prefixes() {
                if let Some(cx) = self.complexes.get(&p.key()) {
                    for layer in &cx.degrees {
                        for (_, cell) in layer.fiber(crate::network::nerve::ROOT) {
                            match &cell.fill {
                                crate::transducer::cech::Fill::Congruent(r, ell) => {
                                    push(&mut out, &mut seen, r, *ell)
                                }
                                crate::transducer::cech::Fill::Class(r) => {
                                    push(&mut out, &mut seen, r, 0)
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        out
    }

    pub fn serve_ruled(
        &self,
        inv: &crate::transducer::stalk::Invariant,
        have: &[crate::transducer::address::Address],
        budget: usize,
    ) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        let mut used = 0usize;
        let mut held: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for h in have {
            held.insert(h, ());
        }
        // An empty mind is seeded from the root: coarsest ruled words first.
        let addrs: Vec<Vec<u8>> = if inv.0.is_empty() {
            self.syllabus_seed()
        } else {
            self.ruled.fiber(inv)
        };
        for addr in addrs {
            if held.contains(&addr) {
                continue;
            }
            // THE GENOME BOUNDARY. A taught word becomes GENOME on the
            // receiver -- crystallized intelligence, the persisted stratum
            // (24.1, chapter 6) -- so only what this court installed may be
            // taught. `word_at` falls through basis -> cavity -> PROVISIONAL,
            // so serving through it hands a voice its own unratified dialect
            // back as language: the epigenome's transmutable stratum filed
            // as the genome's crystallized one. The installed word is the
            // one in `basis`; a ruled address whose word never left
            // provisional is not yet language and is not served.
            let Some(w) = self.installed_word(&addr) else { continue };
            if w.is_connection() && w.letters().is_none() {
                continue; // letterless history is never taught
            }
            let bytes = w.to_bytes();
            if used + bytes.len() + 4 > budget {
                break;
            }
            used += bytes.len() + 4;
            out.push(bytes);
        }
        out
    }

    pub fn ruled_digests(&self) -> Vec<crate::transducer::sheaf::StratumDigest> {
        crate::transducer::sheaf::digests(&self.ruled)
    }

    pub fn ruled_have(&self, inv: &crate::transducer::stalk::Invariant) -> Vec<Vec<u8>> {
        self.ruled.fiber(inv).to_vec()
    }

    /// Ruled addresses, coarsest chart first — the syllabus seed of an
    /// empty atlas.
    pub fn syllabus_seed(&self) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        for (_, members) in self.ruled.strata() {
            out.extend(members);
        }
        out
    }

    pub fn fetch_have(&self, inv: &crate::transducer::stalk::Invariant) -> Vec<Vec<u8>> {
        let mut h: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for a in self.ruled.fiber(inv) {
            h.insert(a, ());
        }
        for (a, _) in self.heard.fiber(crate::network::nerve::ROOT) {
            h.insert(a, ());
        }
        h.fiber(crate::network::nerve::ROOT).map(|(k, _)| k).collect()
    }

}
