// A19, 26 — WHAT THIS COURT SERVES: the ruled words in ruling order, the
// class records that travel as coordinates, and the digests a peer reads.

use crate::cognition::*;

/// WHERE A SERVED PAGE ENDED, by name (the actions' metabolism).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum PageEnd {
    /// The order after the cursor was exhausted within the window.
    #[default]
    Order,
    /// The frame's bytes would be passed by the next word.
    Frame,
    /// The wire's count (`u8`) was reached.
    Count,
}

/// A SYLLABUS PAGE AS THE COURT SERVED IT: the words taught with their
/// stamps; how many candidates the window consumed (served, missed and
/// deferred); the candidates missed, to re-queue; the words this court's
/// reserve could not fund yet, each with the lane that refused it, to
/// re-queue behind the cursor; what the page drew from the reserve, per
/// lane; and where it ended.
#[derive(Clone, Debug, Default)]
pub struct ServedPage {
    pub words: Vec<crate::anastome::dialogue::Taught>,
    pub consumed: usize,
    pub missed: Vec<Vec<u8>>,
    pub deferred: Vec<(Vec<u8>, crate::physics::Depleted)>,
    pub drawn: [u64; 4],
    pub overflowed: bool,
    pub end: PageEnd,
}

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
    ) -> Vec<(crate::transducer::alphabet::Rotor, usize)> {
        let mut out = Vec::new();
        for p in inv.prefixes() {
            if let Some(cx) = self.complexes.get(&p.key()) {
                for (_kappa, r, d) in cx.classes() {
                    out.push((r, d));
                }
            }
        }
        out
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
            out.push(crate::anastome::dialogue::ClassPage {
                key: k,
                order,
                rotor: r.clone(),
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

    /// A SYLLABUS PAGE, SERVED AND PAID (the actions' metabolism, ruled
    /// 2026-09-29): the candidates are the order after a voice's cursor, in
    /// ruling order; each is served while two walls stand, the first to
    /// fall ending the page:
    /// - the FRAME: the page's bytes, `Stamp::LEN + 4 + tape` per word,
    ///   within `bytes`;
    /// - the COUNT the wire carries (`u8`), so no page is built that the
    ///   encoder then refuses after its receipts are sealed.
    ///
    /// Each is PAID: its magnitude — what it carries — drawn from this
    /// court's reserve per lane, all or nothing (`Reserve::draw`); a word no
    /// lane can fund yet is deferred, its lane named, and the page goes on.
    ///
    /// Only an INSTALLED word is taught (the genome boundary, as
    /// `serve_ruled` reads it); one not installed, letterless or unwritable
    /// is MISSED — consumed from the window and re-queued by the caller. A
    /// word served is sealed as a Syllabus act (45.1) and carries that stamp.
    pub fn serve_syllabus(&mut self, candidates: &[Vec<u8>], voice: [u8; 32], bytes: usize) -> ServedPage {
        use crate::observe::{count, SYLLABUS};
        let page = self.serve_syllabus_page(candidates, voice, bytes);
        // THE ACT, COUNTED WHERE IT HAPPENED: one page, its words, its
        // misses, and where it ended.
        let t = &mut self.acts.syllabus;
        count(t, &SYLLABUS, "pages");
        for _ in &page.words {
            count(t, &SYLLABUS, "words served");
        }
        for _ in &page.missed {
            count(t, &SYLLABUS, "words missed");
        }
        for _ in &page.deferred {
            count(t, &SYLLABUS, "deferred: reserve");
        }
        let end = match page.end {
            PageEnd::Order => "ended: order",
            PageEnd::Frame => "ended: frame",
            PageEnd::Count => "ended: count",
        };
        count(t, &SYLLABUS, end);
        page
    }

    fn serve_syllabus_page(&mut self, candidates: &[Vec<u8>], voice: [u8; 32], bytes: usize) -> ServedPage {
        let mut page = ServedPage::default();
        let mut used = 0usize;
        for addr in candidates {
            if page.words.len() == usize::from(u8::MAX) {
                page.end = PageEnd::Count;
                return page;
            }
            let word = self.installed_word(addr).filter(|w| !(w.is_connection() && w.letters().is_none()));
            let Some((tape, mag)) = word.and_then(|w| w.to_bytes().map(|t| (t, w.magnitude()))) else {
                page.missed.push(addr.clone());
                page.consumed += 1;
                continue;
            };
            let width = crate::chronometry::Stamp::LEN + 4 + tape.len();
            if used + width > bytes {
                page.end = PageEnd::Frame;
                return page;
            }
            // A WORD THE RESERVE CANNOT FUND YET WAITS; THE PAGE GOES ON. It
            // ended the page and left the cursor on it, so every later page
            // to that voice tried the same word first and served nothing
            // behind it: on the fleet (epoch 106) 95–99% of pages ended on
            // the reserve with 0–4 words served against 40–53 ratified. The
            // word is deferred behind the cursor, its refusing lane named,
            // and is taught when the verified work of later orbits funds it
            // (`orbit` regenerates the reserve).
            if let Err(depleted) = self.physics.reserve.draw(mag) {
                page.deferred.push((addr.clone(), depleted));
                page.consumed += 1;
                continue;
            }
            let Some(at) = self.note_served(addr, voice) else {
                // Not sealed: the draw is returned and the word missed.
                self.physics.reserve.restore(mag);
                page.missed.push(addr.clone());
                page.consumed += 1;
                continue;
            };
            used += width;
            // Each draw came out of one reserve, so the page's total is
            // bounded by it; past the carrier it is counted, never wrapped.
            match crate::core::lanes::lanes_add(page.drawn, mag) {
                Some(total) => page.drawn = total,
                None => page.overflowed = true,
            }
            page.words.push(crate::anastome::dialogue::Taught { at, word: tape });
            page.consumed += 1;
        }
        page.end = PageEnd::Order;
        page
    }

    pub fn serve_ruled(
        &mut self,
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
            // A word whose record does not write is not taught (STD1 3c).
            let Some(bytes) = w.to_bytes() else { continue };
            let mag = w.magnitude();
            let width = crate::chronometry::Stamp::LEN + 4 + bytes.len();
            if used + width > budget || out.len() == usize::from(u8::MAX) {
                break;
            }
            // WHAT THE FETCH HANDS OUT IS PAID (the actions' metabolism): the
            // word's magnitude drawn from this court's reserve per lane; a
            // word no lane can fund yet is deferred and the page goes on, as
            // the syllabus does — one costly word first in the stratum
            // starved every word behind it.
            if self.physics.reserve.draw(mag).is_err() {
                crate::observe::count(&mut self.acts.fetch, &crate::observe::FETCH, "deferred: reserve");
                continue;
            }
            used += width;
            crate::observe::count(&mut self.acts.fetch, &crate::observe::FETCH, "words served");
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
        let before = self.chain.next_place();
        self.append_step(crate::chronometry::ActKind::Syllabus, addr, to);
        self.chain.at_place(before).map(|r| r.stamp)
    }

    /// A HELD WORD AS A PAGE CARRIES IT: its tape and the stamp that
    /// admitted it, for a page that seals no act of its own (a genesis
    /// page, a chart fetch). `None` where the word is not held or no act
    /// has admitted it, or its record does not write (STD1 3c).
    pub fn taught(&self, addr: &[u8]) -> Option<crate::anastome::dialogue::Taught> {
        let at = self.admitted_at(addr)?;
        Some(crate::anastome::dialogue::Taught { at, word: self.word_at(addr)?.to_bytes()? })
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
