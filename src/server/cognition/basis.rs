// 23.2, 26 — THE BASIS: how a ruled word enters this mind, what it
// occupies while it is here, and what makes it leave.
//
//! A word admitted to the basis is indexed by its stalk, its axis words and
//! its action; it occupies room on the four lanes and is relocated when its
//! T extent is spent. The holonomy orbit and the germinated fibre grow it (25.1).

use super::*;

impl Kernel {
    /// The heights a listing has stood for. A listing with NO STAMP has
    /// stood for none — 45.4's "accrues nothing", the reading of absence on
    /// a keyed carrier — and that is `Some(0)`.
    ///
    /// `None` where the stamp is ABOVE the current height: a court whose
    /// height went backwards is a broken count, and zero read off one is a
    /// measurement this court cannot make, not a listing stamped just now.
    pub(crate) fn t_extent(&self, addr: &[u8], now: u64) -> Option<u64> {
        let Some(then) = self
            .provisional_h
            .get(addr)
            .copied()
            .or_else(|| self.standing_h.get(addr).copied())
        else {
            return Some(0);
        };
        now.checked_sub(then)
    }

    pub(crate) fn stamp_standing(&mut self, addr: impl AsRef<[u8]>) {
        let addr = addr.as_ref();
        let h = self.height();
        if self.standing_h.get(addr).is_none() {
            self.standing_h.insert(addr, h);
        }
        if self.provisional.contains(addr) && self.provisional_h.get(addr).is_none() {
            self.provisional_h.insert(addr, h);
        }
    }

    /// The root of this court's own stratum over `inv` — the SAME
    /// construction a peer's digest carries, so `confirm_peer_holds` can
    /// compare the two.
    ///
    /// This was a THIRD hand-rolled copy of the sorted concatenation, after
    /// `sheaf::root_of` and the one inside `confirm_peer_holds`, and it is
    /// stored per heard address by `heard_mark` — so every heard address
    /// carried a full copy of its stratum's address list, which is O(C²) in
    /// memory over a growing corpus. Thirty-two bytes now, and one
    /// construction (law 12): when the root became a root, a copy left
    /// behind here would have compared a hash against a concatenation and
    /// dropped every heard address on every confirm.
    pub(crate) fn stratum_root_of(&self, inv: &crate::transducer::stalk::Invariant) -> Vec<u8> {
        crate::transducer::sheaf::stratum_root(&self.ruled.fiber(inv))
    }

    /// Whether this court has marked `addr` heard. A heard mark says the
    /// glue ran once for this address on this court; it is not a verdict,
    /// and a reader that needs one replays (`court_replay`).
    pub fn heard_contains(&self, addr: &[u8]) -> bool {
        self.heard.contains(addr)
    }

    pub(crate) fn heard_mark(&mut self, addr: impl AsRef<[u8]>, inv: &crate::transducer::stalk::Invariant) {
        let addr = addr.as_ref();
        let root = self.stratum_root_of(inv);
        let chart = inv.key().into_vec();
        // Re-marked under another chart, the address leaves the old chart's
        // fiber: an index that only ever grows would answer a stratum with
        // addresses that are no longer heard under it.
        if let Some((was, _)) = self.heard.get(addr) {
            if was != &chart {
                let was = was.clone();
                if let Some(inner) = self.heard_at.get_mut(&was) {
                    inner.remove(addr);
                }
            }
        }
        self.heard_at.nested_insert(chart.clone(), addr, ());
        self.heard.insert(addr, (chart, root));
    }

    /// An address is no longer heard: it leaves both the set and the chart
    /// fiber that indexes it.
    pub(crate) fn heard_forget(&mut self, addr: &[u8]) {
        if let Some((chart, _)) = self.heard.remove(addr) {
            if let Some(inner) = self.heard_at.get_mut(&chart) {
                inner.remove(addr);
            }
        }
    }

    pub(crate) fn chill_field(&mut self, addr: &[u8]) {
        if let Some(listing) = self.field.remove(addr) {
            self.field_cold.insert(addr, listing);
        }
        self.open.remove(addr);
    }

    /// Relocate the largest-T standing carriers on a saturated temporal lane.
    /// T-room is standing extent in heights, never a stored occupancy floor.
    pub(crate) fn relocate_t_exhausted(&mut self) {
        if self.lane_cap.is_none()
            && self.occupancy[2] == 0
            && self.t_leases.scoped().next().is_none()
        {
            return;
        }
        let now = self.height();
        let mut tot = 0u64;
        let mut best: Option<(Vec<u8>, u64)> = None;
        let mut unmeasured = false;
        let mut consider = |addr: Vec<u8>| {
            let Some(t) = self.t_extent(&addr, now) else {
                unmeasured = true;
                return;
            };
            tot = tot.saturating_add(t);
            match best {
                Some((_, bt)) if t <= bt => {}
                _ => best = Some((addr, t)),
            }
        };
        // THE T LANE'S OWN LISTINGS, and the provisionals standing beside
        // them: `lane_listings` is indexed BY LANE, so the first is a
        // coordinate read one level up, and the second is the provisional
        // book, whose whole content stands on the temporal lane by
        // definition. A carrier omitted here is a carrier the saturated
        // lane is never relieved of.
        for (addr, _) in self.lane_listings[2].scoped() {
            consider(addr);
        }
        for (addr, _) in self.provisional.scoped() {
            consider(addr);
        }
        // A lane holding a listing whose extent cannot be measured is not a
        // saturated lane; it is an unread one, and nothing is relocated
        // against it — as with a room that cannot be read, below.
        if unmeasured {
            return;
        }
        // THE CAP IS THE LANE'S COVER, and it was `tot + room`.
        //
        // `room` is `cover − occupancy`, so `tot < tot + room` reduces to
        // `room > 0`: the accumulated extent CANCELS on both sides and the
        // test never reads it. What looked like a saturation measure was a
        // binary switch on whether the lane had any room at all — never
        // relocate while room remains, always relocate the moment it is
        // gone, whatever the standing extent was. The whole `consider` walk
        // above fed a quantity the comparison discarded.
        //
        // The cover is what a saturation test compares against, and
        // `lane_cover_of` reads it directly rather than adding the room back
        // to the occupancy (law 12).
        let Some(cover) = self.lane_cover_of(&self.identity) else {
            // A cover that cannot be read is a broken count, and nothing is
            // relocated against one.
            return;
        };
        let cap = cover[2] as u64;
        // A LANE WITH NO COVER IS UNMEASURED, NOT FULL. `measure_lanes`
        // writes lane 2 as zero — a chain carries no clock — so every
        // family's 𝒯 cover is zero at registration, and the provisional
        // book "stands on the temporal lane by definition". Read as
        // saturation that evicted a listing on every stroke at T=1, and the
        // book drained to nothing: measured, `cavity relocate … T=1` on a
        // continuous loop and `book 0 listings` against clients receiving
        // `0 clusters` to quote.
        //
        // It is the same reading this function already makes twice — an
        // extent that cannot be measured does not saturate a lane, and
        // neither does a room that cannot be read. A cover of zero on a lane
        // nothing ever measured is that case and not a full one (rule 11).
        if cap == 0 {
            return;
        }
        if tot < cap {
            return;
        }
        let Some((addr, t)) = best else {
            return;
        };
        eprintln!(
            "  cavity relocate {:02x}{:02x}… T={t} (T-room exhausted)",
            addr[0], addr[1]
        );
        self.relocate_standing(&addr);
    }

    pub(crate) fn relocate_standing(&mut self, addr: &[u8]) {
        let _ = self.treasury.revoke_award(addr, false);
        if self.pending.contains(addr) {
            self.relocate_cavity(addr);
        } else if let Some(w) = self.provisional.get(addr).cloned() {
            let inv = crate::transducer::stalk::invariant_of(&w);
            self.listing_atlas.admit(&inv, addr);
            self.cavity_book.insert(addr, w.clone());
            self.cavities.push(w);
            self.chill_field(addr);
        }
        self.provisional.remove(addr);
        self.provisional_k.remove(addr);
        self.provisional_h.remove(addr);
        self.offered.remove(addr);
        self.offered_to.remove(addr);
        if self.lane_listings[2].contains(addr) {
            // The lane lists it, so the lane carries it; a lane at zero
            // listing something is a broken count and stays at zero.
            if let Some(next) = self.occupancy[2].checked_sub(1) {
                self.occupancy[2] = next;
            }
            self.lane_listings[2].remove(addr);
        }
        self.standing_h.remove(addr);
    }

    pub(crate) fn discoverer_of(&self, addr: &[u8]) -> Option<[u8; 32]> {
        self.discoverers.get(addr).copied()
    }

    /// A held word, to change. `Held::update` is the only way through it:
    /// the element is mutated and the form re-decoded together, so a form
    /// cannot describe a tape the element no longer has (L16).
    pub(crate) fn word_mut(&mut self, addr: &[u8]) -> Option<&mut crate::transducer::element::Held> {
        self.basis.get_mut(addr).map(std::sync::Arc::make_mut)
    }

    pub(crate) fn book_elem(&self, addr: &[u8]) -> Option<&Element> {
        self.pending.get(addr).map(|(p, _)| p)
    }

    pub(crate) fn live_slot(&self, addr: &[u8]) -> Option<usize> {
        self.language.index.get(addr).copied()
    }

    pub(crate) fn index_word(&mut self, w: &Element) {
        let addr = w.address();
        if let Some(k) = action_index_key(w) {
            self.by_action.insert(k, addr.clone());
        }
        if !w.sub().is_empty() {
            let inv = crate::transducer::stalk::invariant_of(w);
            self.by_footprint.nested_insert(chart_footprint_key(&inv, w.sub()), addr, ());
        }
    }

    pub(crate) fn unindex_word(&mut self, w: &Element) {
        let addr = w.address();
        if let Some(k) = action_index_key(w) {
            if self.by_action.get(&k).is_some_and(|a| a.as_bytes() == addr.as_bytes()) {
                self.by_action.remove(&k);
            }
        }
        if !w.sub().is_empty() {
            let inv = crate::transducer::stalk::invariant_of(w);
            let fk = chart_footprint_key(&inv, w.sub());
            if let Some(inner) = self.by_footprint.get_mut(&fk) {
                inner.remove(addr.as_bytes());
            }
        }
    }

    pub(crate) fn forget_spoken(&mut self, listing: &[u8]) {
        if let Some(voices) = self.spoken.remove(listing) {
            for (v, _) in voices.scoped() {
                if let Some(inner) = self.spoken_by.get_mut(&v) {
                    inner.remove(listing);
                }
            }
        }
    }

    pub(crate) fn occupy(&mut self, addr: impl AsRef<[u8]>, mag: &[u64; 4], add: bool) {
        let addr = addr.as_ref();
        let voice = self.drawers.get(addr).copied();
        for (l, &m) in mag.iter().enumerate() {
            if add {
                self.occupancy[l] = self.occupancy[l].saturating_add(m);
                if m > 0 {
                    self.lane_listings[l].insert(addr, ());
                }
            } else {
                // The listing occupied this lane by exactly `m`, so the
                // release is exact; a lane holding less than it releases is
                // a broken count and stays where it is (rule 11).
                if let Some(next) = self.occupancy[l].checked_sub(m) {
                    self.occupancy[l] = next;
                }
                if m > 0 {
                    self.lane_listings[l].remove(addr);
                }
            }
            if add {
                self.pressure_sum[l] = self.pressure_sum[l].saturating_add(m);
            } else if let Some(next) = self.pressure_sum[l].checked_sub(m) {
                self.pressure_sum[l] = next;
            }
        }
        if let Some(v) = voice {
            // A voice that has drawn nothing draws nothing: absence.
            let mut d = match self.drawn.get(&v) {
                Some(d) => *d,
                None => [0; 4],
            };
            for (t, &m) in d.iter_mut().zip(mag) {
                match add {
                    true => *t = t.saturating_add(m),
                    // The voice drew exactly `m` on this lane, so the
                    // return is exact; a draw smaller than its return is a
                    // broken count and stays where it is.
                    false => {
                        if let Some(next) = t.checked_sub(m) {
                            *t = next;
                        }
                    }
                }
            }
            // §3.1 as ruled (S13.15): the voice's claim in flight on a lane
            // closed, so the recall shortfall that closed its room there
            // clears with it.
            if !add {
                if let Some(mut owed) = self.recall_short.get(&v).copied() {
                    for l in 0..4 {
                        if d[l] == 0 {
                            owed[l] = 0;
                        }
                    }
                    if owed == [0; 4] {
                        self.recall_short.remove(&v);
                    } else {
                        self.recall_short.insert(v, owed);
                    }
                }
            }
            self.drawn.insert(v, d);
        }
    }

    /// THE COVER MOVED. The root over it is not rebuilt here.
    ///
    /// This called `sheaf::digests`, which materialises every stratum with
    /// every member and sorts each one — on EVERY ruling. Admitting the Nth
    /// word cost O(N log N) and admitting N words cost O(N² log N), on the
    /// one path the whole engine exists to reach. It is a walk of an
    /// append-only carrier driven by that carrier's own growth.
    ///
    /// The root is a reading of the cover, and a reading is taken when it
    /// is asked for (law 12). It is asked for once per exchange cycle, by
    /// `sign_cover_digest`; it was being computed once per ruling.
    pub(crate) fn refresh_ruled_root(&mut self) {
        self.ruled_root_due = true;
    }

    /// The root over the ruled cover, measured if the cover has moved
    /// since the last reading.
    pub(crate) fn measure_ruled_root(&mut self) -> [u8; 32] {
        if self.ruled_root_due {
            let digests = crate::transducer::sheaf::digests(&self.ruled);
            let mut buf = Vec::new();
            for d in &digests {
                crate::transducer::address::put_bytes(&mut buf, &d.root);
            }
            self.ruled_root = crate::chronometry::payload_hash(&buf);
            self.ruled_root_due = false;
        }
        self.ruled_root
    }

    /// Place a word on the basis with no syntropic stroke: restore and the
    /// fallback cursor walk insert this way so a snapshot does not condense
    /// the language it is loading.
    pub(crate) fn basis_insert(&mut self, word: Element, discoverer: [u8; 32]) {
        let addr = word.address();
        self.index_word(&word);
        // ONE DECODE, HERE, where the court places the word (L16).
        let arc = std::sync::Arc::new(crate::transducer::element::Held::of(word));
        self.basis.insert(&addr, std::sync::Arc::clone(&arc));
        self.note_discovered(&addr, discoverer);
        self.basis_gen += 1;
        self.language_append_arc(arc);
    }

    /// Install a word and pivot Path 2 on the axis it just moved: condense
    /// a congruent sibling, probe the stratum for a derivable generator,
    /// pay each yield on its own lane. A word is named by its address; the
    /// slot index these returned was a position in the language snapshot,
    /// read back through `live_slot` and defaulted to 0 when the snapshot
    /// lagged the carrier -- slot 0 is a real word, and it is not this one.
    /// No caller read either return (rule 11).
    pub(crate) fn basis_push(&mut self, word: Element, discoverer: [u8; 32]) {
        let addr = word.address();
        self.basis_insert(word, discoverer);
        self.syntropic_on_admit(addr);
    }

    /// Path 2 at admit: the atlas names the sibling a versor relates, the
    /// stratum names a word the new generator derives. Fallback consolidate
    /// still walks the cursor when these reads are empty.
    pub(crate) fn syntropic_on_admit(&mut self, addr: Address) {
        let survivor = self.condense_on(&addr);
        self.refine_stratum(&survivor);
        self.holonomy_orbit(&survivor);
        self.drain_unreachable();
    }

    /// Read the holonomy group on the admitted word: a closed orbit of
    /// gauge transitions condenses through torse_at; an open orbit stays
    /// free; an orbit that fails to close on a Twisted chart germinates a
    /// fibre on a fresh axis seeded with the failing transition.
    pub(crate) fn holonomy_orbit(&mut self, addr: &[u8]) {
        let Some(word) = self.basis.get(addr) else {
            return;
        };
        let trans = word.transitions().to_vec();
        if trans.is_empty() {
            return;
        }
        let inv = crate::transducer::stalk::invariant_of(word);
        let twisted = self.field.get(&word.address()).is_some_and(|f| {
            f.nonflat > 0
                && f.entries.iter().any(|e| {
                    e.chart == inv
                        && matches!(
                            e.state,
                            crate::transducer::stalk::Closure::Twisted(_)
                                | crate::transducer::stalk::Closure::Class(..)
                        )
                })
        });
        let mut asgs: crate::network::nerve::Keyed<Vec<element::Versor>> =
            crate::network::nerve::Keyed::new();
        for crate::transducer::element::Transition { asg, gauge: g, .. } in trans {
            let mut k = Vec::with_capacity(asg.len() * 2);
            for &(a, b) in &asg {
                k.push(a);
                k.push(b);
            }
            match asgs.get_mut(&k) {
                Some(vs) => vs.push(g),
                None => {
                    asgs.insert(k, vec![g]);
                }
            }
        }
        let mut germinate: Option<element::Versor> = None;
        let mut torse = false;
        for (_, vs) in asgs.scoped() {
            match fiber_holonomy(vs) {
                // Closed: the loop is torsion the language already carries.
                Some(h) if h.projective_eq(&element::identity_versor()) => torse = true,
                // Held: the germ is the holonomy itself, the exact thing the
                // loop failed to close by, not a transition read off the end
                // of the fiber.
                Some(h) if twisted => germinate = Some(h),
                _ => {}
            }
        }
        if torse {
            self.torse_at(addr);
        }
        if let Some(fail) = germinate {
            self.germinate_fibre(addr, &fail);
        }
    }

    pub(crate) fn germinate_fibre(&mut self, addr: &[u8], fail: &element::Versor) {
        let Some(word) = self.basis.get(addr).map(|w| (**w).clone()) else {
            return;
        };
        let Some(seed) = word.versor() else {
            let asg = element::assignment(word.sub());
            if let Some(w) = self.word_mut(addr) {
                w.update(|e| e.record_dislocation(asg, fail.clone(), element::FIBER_SPATIAL));
            }
            self.touch_at(addr);
            return;
        };
        let asg = element::assignment(word.sub());
        let axis = match self.word_mut(addr) {
            // ONE `update`, not two: opening the axis and recording the
            // dislocation on it are one change to the word, and a re-decode
            // between them would file the dislocation against a form that
            // does not yet have the axis.
            Some(w) => w.update(|e| {
                let axis = e.open_axis(seed);
                e.record_dislocation(asg, fail.clone(), axis);
                axis
            }),
            None => return,
        };
        self.touch_at(addr);
        eprintln!(
            "  germinated fibre axis {axis} from a failed orbit on a Twisted chart"
        );
    }

    pub fn prune_basis_to_ruled(&mut self) -> usize {
        let mut nb = crate::network::nerve::Keyed::new();
        let mut nd = crate::network::nerve::Keyed::new();
        let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        // Walk the federation once through the fed index; peel the live word.
        for (addr, elem) in &self.court_federation {
            if elem.is_connection() && elem.letters().is_none() {
                continue;
            }
            if seen.contains(addr) {
                continue;
            }
            seen.insert(addr, ());
            if let Some(w) = self.basis.get(addr) {
                nb.insert(addr, std::sync::Arc::clone(w));
                if let Some(d) = self.discoverer_of(addr) {
                    nd.insert(addr, d);
                }
            } else {
                nb.insert(
                    addr,
                    std::sync::Arc::new(crate::transducer::element::Held::of(elem.clone())),
                );
            }
        }
        if seen.is_empty() {
            // No court cover: the restored or local language stands.
            return 0;
        }
        // WHAT LEFT, COUNTED WHERE IT LEFT: the words the old basis held
        // that the new one does not. A difference of the two sizes is not
        // that quantity — this prune also ADOPTS federation words the basis
        // never held, so the sizes can move the other way while words are
        // still dropping.
        let dropped = self
            .basis
            .scoped()
            .filter(|(addr, _)| nb.get(addr).is_none())
            .count();
        self.basis = nb;
        self.discoverers = nd;
        self.by_action = crate::network::nerve::Keyed::new();
        self.by_footprint = crate::network::nerve::Keyed::new();
        let mut stalks = crate::transducer::stalk::Stalks::default();
        let mut ruled = crate::transducer::stalk::Stalks::default();
        self.ruled_addrs = crate::network::nerve::Keyed::new();
        // REINDEXING IS OVER THE BASIS, all of it: the stalks, the ruled
        // cover and the axis index are being rebuilt from the words that
        // survived the prune, and a word not visited here is a word the
        // new index does not know. The carrier is the object of the pass.
        let held: Vec<(Vec<u8>, std::sync::Arc<crate::transducer::element::Held>)> = self
            .basis
            .scoped()
            .map(|(a, w)| (a, std::sync::Arc::clone(w)))
            .collect();
        for (addr, w) in &held {
            self.index_word(w);
            let inv = crate::transducer::stalk::invariant_of(w);
            stalks.admit(&inv, addr);
            ruled.admit(&inv, addr);
            self.ruled_addrs.insert(addr, ());
        }
        self.stalks = std::sync::Arc::new(stalks);
        self.ruled = ruled;
        self.refill_axis_index();
        self.refresh_ruled_root();
        self.basis_gen += 1;
        self.retire_gen += 1;
        self.language_rebuild();
        dropped
    }

    /// Restore fills the axis index through the same admits as `ruled_admit`.
    pub(crate) fn refill_axis_index(&mut self) {
        self.axis_index = crate::transducer::sentence::AxisIndex::new();
        let strata = self.ruled.strata();
        let mut held = Vec::new();
        for (inv, addrs) in strata {
            for addr in addrs {
                if let Some(w) = self.word_at(&addr) {
                    held.push((
                        inv.clone(),
                        crate::transducer::sentence::words_of_element(w),
                    ));
                }
            }
        }
        for (inv, words) in held {
            crate::transducer::sentence::admit_words(&mut self.axis_index, &inv, &words);
        }
    }

    /// A FIXTURE, NOT AN ORGAN: identity-glue every provisional as if an
    /// overlapping hypha had fetched the same word, so a test can reach a
    /// basis without standing up a second court. The two vantages are the
    /// constants `[2; 32]` and `[1; 32]`, which is why this is compiled
    /// only under test: a hypha that glued its own provisionals under a
    /// pair of made-up keys would be manufacturing its own second vantage
    /// (47.3), and rule 5 does not let a constant stand for a measurement
    /// in shipped code. The two keys are wire identifiers, which is §3.4's
    /// one exception; every test that reaches a basis through this says so
    /// and claims nothing about 47.3's witness (Stage 11 I24).
    #[cfg(test)]
    pub fn glue_local(&mut self) {
        let words: Vec<Element> = self.provisional.scoped().map(|(_, e)| e.clone()).collect();
        let mut cover = crate::network::nerve::Cover::new();
        for w in words {
            let _ = self.glue_fetched(w, [2; 32], [1; 32], &mut cover);
        }
    }

    #[cfg(test)]
    pub(crate) fn leak_unruled(&mut self, w: Element) {
        self.basis_insert(w, [0; 32]);
    }

    #[cfg(test)]
    pub(crate) fn rule_all_provisionals(&mut self) {
        let addrs: Vec<Vec<u8>> = self.provisional.scoped().map(|(a, _)| a).collect();
        for a in addrs {
            self.ruled_admit(&a);
        }
    }

    pub(crate) fn fed_admit(&mut self, addr: &[u8], spoken: Element) {
        if self.court_fed_index.get(addr).is_some() {
            return;
        }
        self.court_fed_index.insert(addr, self.court_federation.len());
        self.court_federation.push((Address::from(addr), spoken));
    }

    /// Active cover: the identity fibre (so Čech overlap is non-empty after
    /// the first Cover), plus charts of open listings and of the ruled language.
    pub fn active_cover(&self) -> Vec<(u8, crate::transducer::stalk::Invariant)> {
        let mut out = vec![(0, crate::transducer::stalk::invariant_of(&Element::identity()))];
        for (inv, _) in self.listing_atlas.strata() {
            out.push((0, inv));
        }
        for (inv, _) in self.ruled.strata() {
            out.push((0, inv));
        }
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 4 — canonical key order: a chart IS its coordinate vector, and the cover is written in it"
        )]
        out.sort_by(|a, b| a.1 .0.cmp(&b.1 .0));
        out.dedup();
        out
    }

    /// Whether this address still stands on the quote book.
    pub fn has_listing(&self, addr: &[u8]) -> bool {
        self.pending.contains(addr)
    }

    /// Relocate one listing onto the cavity atlas. The address is kept.
    pub fn relocate_listing(&mut self, addr: &[u8]) {
        self.relocate_cavity(addr);
    }

    /// Relocate restored demands whose chart no longer homes here. Never dropped.
    pub fn unlist_foreign(&mut self, home: impl Fn(&Element) -> bool) {
        let keys: Vec<Address> = self
            .pending
            .scoped()
            .filter(|(_, (p, _))| !home(p))
            .map(|(_, (p, _))| p.address())
            .collect();
        for k in keys {
            self.relocate_cavity(&k);
        }
    }

    /// Import a court.federation once as the initial ruled cover.
    /// Letterless words are history and are never taught (not on the basis).
    pub fn import_crt(&mut self, state: crate::court::ledger::CourtState) {
        if self.crt_imported {
            return;
        }
        self.crt_imported = true;
        for (addr, elem) in state.federation {
            let letterless = elem.is_connection() && elem.letters().is_none();
            if !letterless {
                if self.basis.get(&addr).is_none() {
                    self.basis_insert(elem.clone(), [0; 32]);
                }
                self.ruled_admit(&addr);
            }
            self.fed_admit(&addr, elem);
        }
        if !state.chain.receipts().is_empty() && self.chain.receipts().is_empty() {
            self.chain = state.chain;
        }
    }

    /// The germs this court has Offered and still holds provisionally, with
    /// their tapes — what a second organ needs in order to replay them.
    /// Bounded by the caller's `Budget`: an offer is work like any other and
    /// a stroke sends what it can pay for, never the whole docket.
    pub fn offers_pending(
        &self,
        budget: &mut crate::physics::Budget,
    ) -> Vec<(crate::transducer::address::Address, Vec<u8>)> {
        let mut out = Vec::new();
        for (addr, _) in self.offered.scoped() {
            if !budget.spend() {
                break;
            }
            let Some(w) = self.provisional.get(&addr) else {
                continue;
            };
            out.push((crate::transducer::address::Address::from(addr), w.to_bytes()));
        }
        out
    }

    /// What the offer of `addr` stands at for `witness` (S12.4d): `None`
    /// until it has been offered to that witness.
    pub fn offer_standing(&self, addr: &[u8], witness: &[u8; 32]) -> Option<crate::cognition::OfferStanding> {
        self.offered_to.get(addr).and_then(|w| w.get(witness)).copied()
    }

    /// The offer of `addr` went to `witness` at this court's K `k`; it
    /// stands pending there until the witness answers or its Λ elapses.
    pub fn note_offered(&mut self, addr: &[u8], witness: [u8; 32], k: u64) {
        self.offered_to.nested_insert(addr, witness, crate::cognition::OfferStanding::Pending { k });
    }

    /// `witness` answered the offer of `addr` with a vantage of its own —
    /// the section it reached, or the refusal it named — and is not asked
    /// again for this word.
    pub fn note_witness_answered(&mut self, addr: &[u8], witness: [u8; 32]) {
        self.offered_to.nested_insert(addr, witness, crate::cognition::OfferStanding::Answered);
    }

}
