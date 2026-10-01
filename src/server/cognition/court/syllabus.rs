// A19, 26 — WHAT THIS COURT SERVES: the ruled words in ruling order, the
// class records that travel as coordinates, and the digests a peer reads.

use crate::cognition::*;

/// WHAT THIS COURT TAUGHT A VOICE, keyed by the word then the voice: the
/// word's marks are its own fiber, and leave with it.
pub(crate) fn taught_key(addr: &[u8], voice: &[u8; 32]) -> Vec<u8> {
    let mut key = addr.to_vec();
    key.extend_from_slice(voice);
    key
}

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
            // THE COST IS READ BEFORE THE WORD IS WRITTEN: a word the reserve
            // cannot fund is never serialized. Every page to every voice
            // wrote each deferred word out whole and threw the tape away.
            let Some((mag, kind)) = self
                .installed_word(addr)
                .filter(|w| !(w.is_connection() && w.letters().is_none()))
                .map(|w| (w.magnitude(), deferred_kind(w)))
            else {
                page.missed.push(addr.clone());
                page.consumed += 1;
                continue;
            };
            // A SENTENCE IS CHARGED ITS GLUING STEP, NOT ITS MASS (the
            // operator, 2026-09-30): what it cites that this voice already
            // holds from this court is not paid for again.
            let mag = self.gluing_cost(addr, mag, voice);
            // A SENTENCE IS PAID BY ITS LEARNER (ruled 2026-09-30, the
            // operator's word): its gluing step comes out of the voice's own
            // balance, never the court's reserve — the sentences starved
            // there behind cheaper words that spent every refill first (38–54
            // at each court, epoch 115). A voice that cannot pay yet waits,
            // its own shortfall named; a word is still paid by the reserve.
            if kind == SENTENCE {
                match self.serve_paid_by_learner(addr, mag, voice, bytes, used) {
                    Learner::Served(taught, width) => {
                        used += width;
                        page.words.push(taught);
                    }
                    Learner::Short(depleted) => {
                        self.note_deferred(addr, kind, mag, &depleted, Some(voice));
                        page.deferred.push((addr.clone(), depleted));
                    }
                    Learner::Frame => {
                        page.end = PageEnd::Frame;
                        return page;
                    }
                    Learner::Missed => page.missed.push(addr.clone()),
                }
                page.consumed += 1;
                continue;
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
                self.note_deferred(addr, kind, mag, &depleted, None);
                page.deferred.push((addr.clone(), depleted));
                page.consumed += 1;
                continue;
            }
            // Paid: now the tape, and the frame's room for it.
            let Some(tape) = self.installed_word(addr).and_then(|w| w.to_bytes()) else {
                self.physics.reserve.restore(mag);
                page.missed.push(addr.clone());
                page.consumed += 1;
                continue;
            };
            let width = crate::chronometry::Stamp::LEN + 4 + tape.len();
            if used + width > bytes {
                self.physics.reserve.restore(mag);
                page.end = PageEnd::Frame;
                return page;
            }
            self.deferred_words.remove(addr);
            let Some(at) = self.note_served(addr, voice) else {
                // Not sealed: the draw is returned and the word missed.
                self.physics.reserve.restore(mag);
                page.missed.push(addr.clone());
                page.consumed += 1;
                continue;
            };
            used += width;
            self.taught_voice.insert(taught_key(addr, &voice), ());
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

    /// ONE SENTENCE TAUGHT AT ITS LEARNER'S EXPENSE: the voice's balance
    /// read first — a short voice waits, nothing written — then the tape and
    /// the frame's room, the Syllabus act sealed, and the gluing step paid
    /// from the voice to this court at that act. A step of nothing is free.
    fn serve_paid_by_learner(&mut self, addr: &[u8], cost: [u64; 4], voice: [u8; 32], bytes: usize, used: usize) -> Learner {
        let short = crate::economy::shortfall(&cost, &self.treasury.balance_mu(&voice));
        // The short lane named is the first in (S, G, T, H) order — content
        // order, as `Depleted` names it, never a priority.
        if let [lane, ..] = crate::economy::lanes_short(&short).as_slice() {
            let lane = *lane;
            let held = self.treasury.balance_mu(&voice)[lane];
            return Learner::Short(crate::physics::Depleted { lane, asked: cost[lane], held });
        }
        let Some(tape) = self.installed_word(addr).and_then(|w| w.to_bytes()) else { return Learner::Missed };
        let width = crate::chronometry::Stamp::LEN + 4 + tape.len();
        if used + width > bytes {
            return Learner::Frame;
        }
        self.deferred_words.remove(addr);
        let Some(at) = self.note_served(addr, voice) else { return Learner::Missed };
        if cost.iter().any(|&c| c > 0) {
            let anchor = self.anchor();
            if let Err(why) = self.treasury.pay(voice, self.identity, cost, anchor) {
                // Read as affordable above; a refusal here is counted, and
                // the word is taught — the act is sealed, the page stands.
                eprintln!("  syllabus: a learner's payment refused after its balance was read: {why:?}");
            }
        }
        // 78.60 — what the sentence stands to among the words this court
        // already taught this learner, read before it joins them.
        let anchor = self.anchor_of(addr, voice);
        crate::observe::count(&mut self.acts.syllabus, &crate::observe::SYLLABUS, anchor);
        self.taught_voice.insert(taught_key(addr, &voice), ());
        Learner::Served(crate::anastome::dialogue::Taught { at, word: tape }, width)
    }

    /// 78.60 — WHAT A TAUGHT SENTENCE STANDS TO IN ITS LEARNER'S
    /// VOCABULARY, of the words this court taught that learner on the
    /// sentence's chart, by the strongest relation any of them stands in:
    /// refracted — `held.refract(Γ)` is the sentence, so it could have
    /// travelled as (held, Γ); conjugate — one spelled word conjugates every
    /// vertex, the carriage, gauge or a citation apart; an unspellable
    /// conjugator — a generator's obstruction (37.7); a rotor per vertex (a
    /// coboundary δγ, `coboundary_gauge`); the chart alone; or none. Exact
    /// (the conjugators solved over ℚ, the refraction's address compared),
    /// never a byte count; a lower bound, since the learner's own speech and
    /// other courts' words are not read.
    fn anchor_of(&self, addr: &[u8], voice: [u8; 32]) -> &'static str {
        use crate::transducer::sentence::chart::{chart_of, coboundary_gauge};
        use crate::transducer::sentence::Relation;
        const READS: [&str; 6] = [
            "sentences: refracted (held, Γ)",
            "sentences: conjugate, carriage apart",
            "sentences: unspellable conjugator",
            "sentences: rotor per vertex δγ",
            "sentences: chart only",
            "sentences: unanchored",
        ];
        let Some(word) = self.installed_word(addr) else { return READS[5] };
        let (Some(inv), Some(form)) = (chart_of(word), crate::transducer::sentence::from_element_form(word)) else {
            return READS[5];
        };
        // The strongest relation met so far, as its place on READS.
        let mut best = 5usize;
        for held in self.ruled.fiber(&inv) {
            if held.as_slice() == addr || !self.taught_voice.contains(&taught_key(&held, &voice)) {
                continue;
            }
            best = best.min(4);
            let Some(h) = self.installed_word(&held).and_then(crate::transducer::sentence::from_element_form) else {
                continue;
            };
            let read = match crate::transducer::sentence::relation(&h, &form) {
                Relation::Refracted(_) => return READS[0],
                Relation::Conjugate(_) => 1,
                Relation::Unspellable(_) => 2,
                Relation::Unrelated => match coboundary_gauge(&h, &form) {
                    Some(_) => 3,
                    None => 4,
                },
            };
            best = best.min(read);
        }
        READS[best]
    }

    /// WHAT TEACHING `addr` TO `voice` COSTS: a word its magnitude whole; a
    /// sentence its magnitude less that of every distinct word it cites
    /// which this court has already served this voice, lane by lane — the
    /// gluing step. The syllabus measured on epoch 115 deferred about twenty
    /// sentences at every court, costing 25–540 against a reserve of 5–40 a
    /// lane, charged their whole mass on every page, and taught nothing
    /// composite. On a lane the held words already carry whole, the gluing
    /// adds nothing and is charged nothing.
    #[expect(
        clippy::manual_unwrap_or_default,
        reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; a lane the held words carry whole has no gluing step, named at the arm"
    )]
    fn gluing_cost(&self, addr: &[u8], mag: [u64; 4], voice: [u8; 32]) -> [u64; 4] {
        let Some(w) = self.installed_word(addr).filter(|w| w.is_sentence()) else { return mag };
        let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        let mut held = [0u64; 4];
        for c in w.receipts().cited_addresses() {
            if c.as_bytes() == addr || seen.contains(c.as_bytes()) {
                continue;
            }
            seen.insert(c.as_bytes(), ());
            if !self.taught_voice.contains(&taught_key(c.as_bytes(), &voice)) {
                continue;
            }
            let Some(m) = self.installed_word(c.as_bytes()).map(|x| x.magnitude()) else { continue };
            match crate::core::lanes::lanes_add(held, m) {
                Some(total) => held = total,
                // Held words past the carrier carry every lane whole.
                None => return [0; 4],
            }
        }
        std::array::from_fn(|l| match mag[l].checked_sub(held[l]) {
            Some(step) => step,
            // The held words already carry this lane whole: nothing to glue.
            None => 0,
        })
    }

    /// A word a syllabus page could not pay for, filed under its address:
    /// its kind and cost, the lane that fell short, and one more try.
    fn note_deferred(&mut self, addr: &[u8], kind: u8, mag: [u64; 4], depleted: &crate::physics::Depleted, learner: Option<[u8; 32]>) {
        let mut head = [0u8; 8];
        for (h, b) in head.iter_mut().zip(addr) {
            *h = *b;
        }
        // A draw is refused only where the lane held less than was asked, so
        // the shortfall exists; a lane past the four names none.
        let mut short = [0u64; 4];
        if let (Some(s), Some(gap)) = (short.get_mut(depleted.lane), depleted.asked.checked_sub(depleted.held)) {
            *s = gap;
        }
        // A count past the carrier stands where it was (never wrapped).
        let tries = match self.deferred_words.get(addr).map(|d| d.tries.checked_add(1)) {
            Some(Some(t)) => t,
            Some(None) => u64::MAX,
            None => 1,
        };
        // The learner named by the head of its key, as the word is.
        let learner = learner.map(|v| std::array::from_fn(|i| v[i]));
        self.deferred_words.insert(addr, crate::observe::Deferred { head, kind, mag, short, tries, learner });
    }

    /// The deferred words in the order their addresses have — never ordered
    /// by how often they were tried (rule 4) — as many as the reading's
    /// count carries; the record is bounded by the words installed.
    pub(crate) fn deferred_reading(&self) -> Vec<crate::observe::Deferred> {
        self.deferred_words.scoped().map(|(_, d)| d.clone()).take(crate::observe::DEFERRED_SHOWN).collect()
    }
}

/// What serving one sentence at its learner's expense came to.
enum Learner {
    Served(crate::anastome::dialogue::Taught, usize),
    Short(crate::physics::Depleted),
    Frame,
    Missed,
}

/// A sentence's kind on `observe::DEFERRED_KINDS`.
const SENTENCE: u8 = 2;

/// A word's kind on `observe::DEFERRED_KINDS`.
fn deferred_kind(w: &crate::transducer::element::Element) -> u8 {
    match () {
        _ if w.is_colour_word() => 3,
        _ if w.is_sentence() => SENTENCE,
        _ if w.is_connection() => 1,
        _ => 0,
    }
}
