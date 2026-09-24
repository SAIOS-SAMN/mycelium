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
                for (_kappa, r, d) in cx.classes() {
                    out.push((r, d as u8));
                }
            }
        }
        out
    }

    pub fn cochain_bytes(&self) -> usize {
        let mut n = 0usize;
        for (_, c) in self.complexes.scoped() {
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
        // THE ROOT STRATUM IS THE WHOLE COVER, and this is the one place in
        // the tree where an empty coordinate legitimately means everything:
        // a voice that names no axes is asking what this court holds at the
        // coarsest stratum, which is every complex. The arm below follows
        // the invariant's prefixes instead, one keyed read per prefix, and
        // reaches a scoped layer the same way.
        if inv.0.is_empty() {
            for (_, cx) in self.complexes.scoped() {
                for layer in &cx.degrees {
                    for (_, simplex) in layer.scoped() {
                        match &simplex.fill {
                            crate::transducer::cech::Fill::Congruent(_, r, ell) => {
                                push(&mut out, &mut seen, r, *ell)
                            }
                            crate::transducer::cech::Fill::Class(_, r) => {
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
                        for (_, simplex) in layer.scoped() {
                            match &simplex.fill {
                                crate::transducer::cech::Fill::Congruent(_, r, ell) => {
                                    push(&mut out, &mut seen, r, *ell)
                                }
                                crate::transducer::cech::Fill::Class(_, r) => {
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
    ) -> Vec<crate::anastome::dialogue::Taught> {
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
            // The word with the stamp that admitted it (78.24 (2b)): a word
            // no act admitted has no locator and is not taught.
            let Some(at) = self.admitted_at(&addr) else { continue };
            let bytes = w.to_bytes();
            let width = crate::chronometry::Stamp::LEN + 4 + bytes.len();
            if used + width > budget {
                break;
            }
            used += width;
            out.push(crate::anastome::dialogue::Taught { at, word: bytes });
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

    /// 45.1: A SYLLABUS PAGE SERVED IS AN ADMITTED ACT. One page of the
    /// order went out to one voice, so this court's clock steps once for it
    /// and the receipt names the address served and the voice it went to.
    /// The door already wrote `receipt K … h …` beside every page it served
    /// — a receipt nothing sealed, so the K it named was whatever the last
    /// unrelated act had left there.
    ///
    /// Answers the stamp the Syllabus node took — the stamp the page
    /// carries beside the word (78.24 (2b)) — or `None` where it could not
    /// be sealed, and the word is not served.
    pub fn note_served(&mut self, addr: &[u8], to: [u8; 32]) -> Option<crate::chronometry::Stamp> {
        let before = self.chain.next_step_index();
        self.append_step(crate::chronometry::ActKind::Syllabus, addr, to);
        self.chain.at_k(before).map(|r| r.stamp)
    }

    /// A HELD WORD AS A PAGE CARRIES IT: its tape and the stamp that
    /// admitted it, for a page that seals no act of its own (a genesis
    /// page, a chart fetch). `None` where the word is not held or no act
    /// has admitted it.
    pub fn taught(&self, addr: &[u8]) -> Option<crate::anastome::dialogue::Taught> {
        let at = self.admitted_at(addr)?;
        Some(crate::anastome::dialogue::Taught { at, word: self.word_at(addr)?.to_bytes() })
    }

    /// What this court already holds in the stratum over `inv`, sent with a
    /// fetch so only the lacking words travel.
    ///
    /// THE HEARD ADDRESSES ARE THIS CHART'S, NOT ALL OF THEM. This unioned
    /// the WHOLE of `heard` into every chart's have-list. A peer serving
    /// the fetch skips an address only when it stands in the fiber over
    /// `inv` (`sheaf::serve`), so an address heard under any other chart
    /// could never match: it filtered nothing and was pure wire, pure
    /// allocation, and pure read.
    ///
    /// It is called once per stratum, per peer, per exchange cycle, and
    /// `heard` grows with peers × corpus — so the mesh paid O(P² · S · C)
    /// for addresses that could not be used. Read from the chart's own
    /// fiber, the cost is the stratum.
    pub fn fetch_have(&self, inv: &crate::transducer::stalk::Invariant) -> Vec<Vec<u8>> {
        let mut h: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for a in self.ruled.fiber(inv) {
            h.insert(a, ());
        }
        if let Some(here) = self.heard_at.get(inv.key().as_bytes()) {
            for (a, _) in here.scoped() {
                h.insert(a, ());
            }
        }
        h.scoped().map(|(k, _)| k).collect()
    }

}
