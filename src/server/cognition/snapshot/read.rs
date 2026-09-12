// 28 — THE RESTORE: a mind read back from its sections, with everything
// derived re-derived rather than restored.
//
//! Balances, the ruled stalks, the axis index and the deflations are
//! recomputed from the carriers that were written. A tape that does not
//! parse whole is refused: nothing is partly restored.

use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::element::Element;

impl Kernel {
    /// P7 — RESTORE a snapshot into a fresh kernel. Returns `false` (leaving the
    /// kernel empty) on a bad magic or corruption — a corrupt ledger never crashes
    /// a boot, it just starts clean.
    ///
    /// AND IT SAYS SO, naming the section it stopped in and the byte it
    /// stopped at. Starting clean is a whole court's memory dropped — its
    /// basis, its chain, its treasury — and the next persist writes the
    /// empty one over the ledger that would not read. Measured on the fleet
    /// at fc75a59: hypha b's 16 MB ledger, written by the build that could
    /// not read it back, restored as nothing and the boot line said
    /// nothing either, because the only line here was the one printed when
    /// it SUCCEEDED.
    pub fn restore(&mut self, b: &[u8]) -> bool {
        if b.get(0..4) != Some(b"MYC5") {
            eprintln!("  ledger refused: the magic is not MYC5");
            return false;
        }
        // WHERE THE READ IS. Set as each section opens, so a refusal names
        // one of the tape's own sections and not a byte offset alone.
        let at = std::cell::Cell::new("basis");
        let mut pos = 4usize;
        let read_u32 = |b: &[u8], p: &mut usize| -> Option<u32> {
            let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
            *p += 4;
            Some(v)
        };
        let load = |kernel: &mut Kernel, b: &[u8], pos: &mut usize| -> Option<()> {
            let n = read_u32(b, pos)? as usize;
            for _ in 0..n {
                let elen = read_u32(b, pos)? as usize;
                let mut ep = *pos;
                let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                let spoken = Element::from_bytes(&b[..end], &mut ep)?;
                *pos += elen;
                let who: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                *pos += 32;
                kernel.basis_insert(spoken, who);
            }
            at.set("treasury");
            kernel.treasury = crate::economy::Treasury::restore(b, pos)?;
            at.set("provisional");
            // EVERY SECTION BELOW IS REQUIRED. Each read as "absent in an
            // earlier snapshot → carry on with nothing", and `snapshot`
            // writes every one of them unconditionally, so no writer in
            // the tree emits a tape that omits one. What those arms
            // actually admitted was a TRUNCATED tape: a mind restored with
            // an empty docket, a clock at zero, an unmeasured book, and
            // said nothing. §10 rules a persisted-layout change a recut
            // and deletes the reader for the old layout; a tag that is not
            // the tag refuses the tape whole, as the header already does.
            let np = read_u32(b, pos)? as usize;
            if np > b.len().checked_sub(*pos)? / 6 {
                return None; // untrusted count: len16 + elem len at least
            }
            for _ in 0..np {
                let addr = crate::transducer::address::take_addr(b, pos)?;
                let elen = read_u32(b, pos)? as usize;
                let mut ep = *pos;
                let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                let spoken = Element::from_bytes(&b[..end], &mut ep)?;
                *pos += elen;
                kernel.provisional.insert(addr, spoken);
            }
            // The chain: the court head and every k-step receipt, so the
            // hypha's clock continues across a boot.
            let tag = |b: &[u8], p: &mut usize, want: &'static [u8; 4]| -> Option<()> {
                // The section being opened, named by its own tag: a tape
                // whose tags are not text is a tape this reader cannot
                // name, and it says that rather than a default.
                at.set(match std::str::from_utf8(want) {
                    Ok(s) => s,
                    Err(_) => "a section whose tag is not text",
                });
                match b.get(*p..*p + 4) == Some(want.as_slice()) {
                    true => {
                        *p += 4;
                        Some(())
                    }
                    false => None,
                }
            };
            tag(b, pos, b"KCH1")?;
            let head: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
            *pos += 32;
            let root = crate::chronometry::Root::read_opt(b, pos)?;
            let n = read_u32(b, pos)? as usize;
            if n > b.len().checked_sub(*pos)? / 106 {
                return None;
            }
            let mut receipts = Vec::with_capacity(n);
            for _ in 0..n {
                receipts.push(crate::chronometry::KStepReceipt::read(b, pos)?);
            }
            // PRC1: the peer receipts this court admitted as parents. The
            // chain is built from BOTH lists at once, because verifying
            // the line re-derives a height through every parent and forty
            // of hypha b's named a peer.
            tag(b, pos, b"PRC1")?;
            let np = read_u32(b, pos)? as usize;
            if np > b.len().checked_sub(*pos)? / 106 {
                return None;
            }
            let mut peers = Vec::with_capacity(np);
            for _ in 0..np {
                peers.push(crate::chronometry::KStepReceipt::read(b, pos)?);
            }
            kernel.court_head = head;
            kernel.chain = crate::chronometry::ReceiptChain::from_parts(root, receipts, peers)?;
            // ARC1: the eras, in K order. Each era's receipts re-verify as
            // a chain of their own below their root.
            tag(b, pos, b"ARC1")?;
            let ne = read_u32(b, pos)? as usize;
            if ne > b.len().checked_sub(*pos)? / 56 {
                return None;
            }
            let mut archive = Vec::with_capacity(ne);
            for _ in 0..ne {
                archive.push(crate::chronometry::Era::read(b, pos)?);
            }
            kernel.archive = archive;
            // The prior measurement, continued incrementally (never read as
            // current: its generations are unknowable here).
            tag(b, pos, b"RNK1")?;
            {
                let basis_len = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?) as usize;
                *pos += 8;
                let n = read_u32(b, pos)? as usize;
                if n > b.len().checked_sub(*pos)? / 14 {
                    return None;
                }
                let mut rank = Rank { basis_gen: u64::MAX, basis_len, retire_gen: kernel.retire_gen, ..Rank::default() };
                for _ in 0..n {
                    let addr = crate::transducer::address::take_addr(b, pos)?;
                    let closed = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let m = read_u32(b, pos)? as usize;
                    if m > b.len().checked_sub(*pos)? / 8 {
                        return None;
                    }
                    let mut axes = Vec::with_capacity(m);
                    for _ in 0..m {
                        axes.push(u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?));
                        *pos += 8;
                    }
                    rank.closure.insert(&addr, (closed != u64::MAX).then_some(closed as usize));
                    rank.invariants.insert(&addr, crate::transducer::stalk::Invariant(axes));
                }
                kernel.closure = (u64::MAX, u64::MAX, std::sync::Arc::new(rank));
            }
            tag(b, pos, b"DEF1")?;
            {
                let n = read_u32(b, pos)? as usize;
                // addr ‖ μ⃗[4] ‖ tag u8 [‖ K u64 ‖ tu u64]; refuse a count that cannot fit.
                if n > b.len().checked_sub(*pos)? / 50 {
                    return None;
                }
                for _ in 0..n {
                    let addr = crate::transducer::address::take_addr(b, pos)?;
                    let mut mag = [0u64; 4];
                    for m in &mut mag {
                        *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                        *pos += 8;
                    }
                    let row = *b.get(*pos)?;
                    *pos += 1;
                    match row {
                        0 => {
                            kernel.deflations.insert(&addr, mag);
                        }
                        1 => {
                            let k = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                            *pos += 8;
                            let tu = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                            *pos += 8;
                            kernel.deflations.insert(&addr, mag);
                            kernel.provisional_k.insert(&addr, k);
                            kernel.provisional_h.insert(&addr, tu);
                            kernel.standing_h.insert(&addr, tu);
                        }
                        _ => return None,
                    }
                }
            }
            tag(b, pos, b"FLD1")?;
            {
                let n = read_u32(b, pos)? as usize;
                for _ in 0..n {
                    let addr = crate::transducer::address::take_addr(b, pos)?;
                    let ne = read_u32(b, pos)? as usize;
                    let mut entries = Vec::with_capacity(ne.min(64));
                    for _ in 0..ne {
                        let m = read_u32(b, pos)? as usize;
                        let mut axes = Vec::with_capacity(m);
                        for _ in 0..m {
                            axes.push(u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?));
                            *pos += 8;
                        }
                        let tag = *b.get(*pos)?;
                        *pos += 1;
                        let state = match tag {
                            0 => crate::transducer::stalk::Closure::Flat,
                            1 => crate::transducer::stalk::Closure::Twisted(
                                element::Versor::from_bytes(b, pos)?,
                            ),
                            3 => {
                                let d = *b.get(*pos)?;
                                *pos += 1;
                                crate::transducer::stalk::Closure::Class(
                                    crate::transducer::alphabet::Rotor::from_bytes(b, pos)?,
                                    d,
                                )
                            }
                            _ => crate::transducer::stalk::Closure::Cavity,
                        };
                        let nc = read_u32(b, pos)? as usize;
                        let mut remainder = Vec::with_capacity(nc.min(1 << 16));
                        for _ in 0..nc {
                            let r = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                            *pos += 8;
                            let c = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                            *pos += 8;
                            let hb = *b.get(*pos)?;
                            *pos += 1;
                            let ha = *b.get(*pos)?;
                            *pos += 1;
                            remainder.push((r, c, hb, ha));
                        }
                        let mut stagnant = [0u32; 4];
                        for s in &mut stagnant {
                            *s = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
                            *pos += 4;
                        }
                        entries.push(FieldEntry {
                            chart: crate::transducer::stalk::Invariant(axes),
                            state,
                            remainder,
                            stagnant,
                        });
                    }
                    let nonflat = entries.iter().filter(|e| !e.state.is_flat()).count() as u32;
                    kernel.field.insert(addr, ListingField { entries, nonflat });
                }
            }
            tag(b, pos, b"BOK1")?;
            {
                let n = read_u32(b, pos)? as usize;
                for _ in 0..n {
                    let elen = read_u32(b, pos)? as usize;
                    let mut ep = *pos;
                    let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                    let elem = Element::from_bytes(&b[..end], &mut ep)?;
                    *pos += elen;
                    let k = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let addr = elem.address();
                    kernel.on_page.insert(&addr, ());
                    kernel.listing_atlas.admit(&crate::transducer::stalk::invariant_of(&elem), &addr);
                    kernel.pending.insert(&addr, (elem, k));
                }
            }
            tag(b, pos, b"CAV1")?;
            {
                let n = read_u32(b, pos)? as usize;
                for _ in 0..n {
                    let addr = crate::transducer::address::take_addr(b, pos)?;
                    let elen = read_u32(b, pos)? as usize;
                    let mut ep = *pos;
                    let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                    let elem = Element::from_bytes(&b[..end], &mut ep)?;
                    *pos += elen;
                    kernel.listing_atlas.admit(&crate::transducer::stalk::invariant_of(&elem), &addr);
                    kernel.cavity_book.insert(&addr, elem);
                }
            }
            tag(b, pos, b"CRT1")?;
            {
                kernel.crt_imported = *b.get(*pos)? != 0;
                *pos += 1;
                let n = read_u32(b, pos)? as usize;
                for _ in 0..n {
                    let addr = crate::transducer::address::take_addr(b, pos)?;
                    let elen = read_u32(b, pos)? as usize;
                    let mut ep = *pos;
                    let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                    let elem = Element::from_bytes(&b[..end], &mut ep)?;
                    *pos += elen;
                    kernel.court_fed_index.insert(&addr, kernel.court_federation.len());
                    kernel.court_federation.push((addr, elem));
                }
                // A trailing count of a sub-section no longer written, read
                // by peeking at whether the next four bytes were the NEXT
                // section's tag. `snapshot` emitted a constant zero there
                // and nothing else ever will, so both halves are gone.
            }
            tag(b, pos, b"COC1")?;
            {
                let n = read_u32(b, pos)? as usize;
                if n > b.len().checked_sub(*pos)? {
                    return None;
                }
                for _ in 0..n {
                    let key = take_addr(b, pos)?;
                    let c = crate::transducer::cech::Complex::from_bytes(b, pos)?;
                    kernel.complexes.insert(key, c);
                }
            }
            tag(b, pos, b"FLC1")?;
            {
                let n = read_u32(b, pos)? as usize;
                if n > b.len().checked_sub(*pos)? {
                    return None;
                }
                for _ in 0..n {
                    let addr = crate::transducer::address::take_addr(b, pos)?;
                    let ne = read_u32(b, pos)? as usize;
                    let mut entries = Vec::with_capacity(ne.min(64));
                    for _ in 0..ne {
                        let m = read_u32(b, pos)? as usize;
                        let mut axes = Vec::with_capacity(m);
                        for _ in 0..m {
                            axes.push(u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?));
                            *pos += 8;
                        }
                        let tag = *b.get(*pos)?;
                        *pos += 1;
                        let state = match tag {
                            0 => crate::transducer::stalk::Closure::Flat,
                            1 => crate::transducer::stalk::Closure::Twisted(
                                element::Versor::from_bytes(b, pos)?,
                            ),
                            3 => {
                                let d = *b.get(*pos)?;
                                *pos += 1;
                                crate::transducer::stalk::Closure::Class(
                                    crate::transducer::alphabet::Rotor::from_bytes(b, pos)?,
                                    d,
                                )
                            }
                            _ => crate::transducer::stalk::Closure::Cavity,
                        };
                        let nc = read_u32(b, pos)? as usize;
                        let mut remainder = Vec::with_capacity(nc.min(1 << 16));
                        for _ in 0..nc {
                            let r = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                            *pos += 8;
                            let c = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                            *pos += 8;
                            let hb = *b.get(*pos)?;
                            *pos += 1;
                            let ha = *b.get(*pos)?;
                            *pos += 1;
                            remainder.push((r, c, hb, ha));
                        }
                        let mut stagnant = [0u32; 4];
                        for s in &mut stagnant {
                            *s = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
                            *pos += 4;
                        }
                        entries.push(FieldEntry {
                            chart: crate::transducer::stalk::Invariant(axes),
                            state,
                            remainder,
                            stagnant,
                        });
                    }
                    let nonflat = entries.iter().filter(|e| !e.state.is_flat()).count() as u32;
                    kernel.field_cold.insert(addr, ListingField { entries, nonflat });
                }
            }
            // CYC1 was the same section without a stored class key, and its
            // reader re-derived one with `class_key_of_versor`, dropping the
            // row with a log line where the derivation failed. No writer has
            // emitted CYC1 since the key was stored; the key is read (§10).
            tag(b, pos, b"CYC2")?;
            {
                let n = read_u32(b, pos)? as usize;
                if n > b.len().checked_sub(*pos)? {
                    return None;
                }
                for _ in 0..n {
                    let key = take_addr(b, pos)?;
                    let a: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let bp: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let tu = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let class_key = take_addr(b, pos)?;
                    let v = element::Versor::from_bytes(b, pos)?;
                    kernel.cocycle_at.insert(
                        key,
                        crate::cognition::CocycleRow {
                            gauge: v,
                            t_universal: tu,
                            a,
                            b: bp,
                            class_key,
                        },
                    );
                    // Chart index is rebuilt on next glue; hot row is enough.
                }
                let nc = read_u32(b, pos)? as usize;
                if nc > b.len().checked_sub(*pos)? {
                    return None;
                }
                for _ in 0..nc {
                    let key = take_addr(b, pos)?;
                    let tu = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let v = element::Versor::from_bytes(b, pos)?;
                    kernel.dialect_chain.push((key.into_vec(), v, tu));
                }
            }
            tag(b, pos, b"SET1")?;
            {
                let n = read_u32(b, pos)? as usize;
                if n > b.len().checked_sub(*pos)? {
                    return None;
                }
                for _ in 0..n {
                    let listing = crate::transducer::address::take_addr(b, pos)?;
                    let chain = crate::anastome::dialogue::read_chain(b, pos)?;
                    let voice: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let cites = crate::anastome::dialogue::read_cites(b, pos)?;
                    let mut before = [0u64; 4];
                    for m in &mut before {
                        *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                        *pos += 8;
                    }
                    let mut after = [0u64; 4];
                    for m in &mut after {
                        *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                        *pos += 8;
                    }
                    let k = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let tu = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    kernel.settlements_cold.push(crate::anastome::dialogue::Settlement {
                        listing,
                        chain,
                        voice,
                        cites,
                        before,
                        after,
                        k,
                        t_universal: tu,
                    });
                }
            }
            // index: keyed field at restore (persistence snapshot, once)
            let field_keys: Vec<Vec<u8>> = kernel.field.fiber(crate::network::nerve::ROOT).map(|(k, _)| k).collect();
            for addr in field_keys {
                let Some(listing) = kernel.field.get(&addr).cloned() else { continue };
                let mag = element::measure_lanes(&union_remainders(&listing.entries));
                let lanes = crate::network::nerve::occupied_lanes(&mag);
                for e in &listing.entries {
                    if !e.state.is_flat() {
                        kernel.listing_atlas.admit_also(&e.chart, &addr);
                        for lane in &lanes {
                            kernel.own_cover.admit(*lane, &e.chart, &addr);
                        }
                        let ck = e.chart.key();
                        match kernel.chart_listings.get_mut(&ck) {
                            Some(m) => {
                                m.insert(&addr, ());
                            }
                            None => {
                                let mut m = crate::network::nerve::Keyed::new();
                                m.insert(&addr, ());
                                kernel.chart_listings.insert(ck, m);
                            }
                        }
                    }
                }
                if listing.nonflat > 0 {
                    kernel.open.insert(&addr, ());
                    kernel.tension.insert(&addr, mag);
                } else {
                    kernel.field.remove(&addr);
                    kernel.field_cold.insert(&addr, listing);
                    kernel.tension.insert(&addr, [0; 4]);
                }
            }
            Some(())
        };
        if load(self, b, &mut pos).is_none() {
            eprintln!(
                "  ledger refused in section {} at byte {pos} of {}: the court starts clean",
                at.get(),
                b.len()
            );
            self.basis_clear();
            return false;
        }
        self.occupancy = [0; 4];
        self.drawers = crate::network::nerve::Keyed::new();
        self.drawn = crate::network::nerve::Keyed::new();
        self.t_leases = crate::network::nerve::Keyed::new();
        self.lane_listings = Default::default();
        let pending_addrs: Vec<Address> = self
            .pending
            .fiber(crate::network::nerve::ROOT)
            .map(|(_, (e, _))| e.address())
            .collect();
        for addr in pending_addrs {
            if self.standing_h.get(&addr).is_none() {
                self.standing_h.insert(addr, self.height());
            }
        }
        let prov: Vec<Vec<u8>> = self.provisional.fiber(crate::network::nerve::ROOT).map(|(a, _)| a).collect();
        for addr in prov {
            if self.provisional_h.get(&addr).is_none() {
                self.provisional_h.insert(&addr, self.height());
            }
            self.stamp_standing(&addr);
        }
        // Ruled cover is the lettered federation, not the leaked basis.
        self.ruled = crate::transducer::stalk::Stalks::over_elements(
            self.court_federation.iter().filter_map(|(_, w)| {
                if w.is_connection() && w.letters().is_none() {
                    None
                } else {
                    Some(w)
                }
            }),
        );
        let dropped = self.prune_basis_to_ruled();
        if dropped > 0 {
            eprintln!("  pruned {dropped} leaked basis words; ruled cover remains");
        }
        self.refill_axis_index();
        // A snapshot without DEF1 (or a zero record) re-measures from the
        // spoken word: presence on the docket is not Δ⃗ = 0.
        let need: Vec<(Address, [u64; 4])> = self
            .provisional
            .fiber(crate::network::nerve::ROOT)
            .filter_map(|(addr, spoken)| {
                if self.deflations.get(&addr).is_some_and(|d| d.iter().any(|&x| x > 0)) {
                    return None;
                }
                // The listing's own magnitude where the book still holds it,
                // the spoken word's where it does not. Both are measured on
                // a word; neither stands in for the other (rule 11).
                let mag = match self.book_elem(&addr) {
                    Some(p) => p.magnitude(),
                    None => spoken.magnitude(),
                };
                mag.iter().any(|&x| x > 0).then(|| (Address::from(addr.clone()), mag))
            })
            .collect();
        for (addr, mag) in need {
            if mag.iter().any(|&x| x > 0) {
                self.deflations.insert(addr, mag);
            }
        }
        // L15: the depth index is DERIVED and never persisted, so a
        // restored court reads it back off its own chain — the receipts
        // it wrote and the peer receipts it admitted — exactly as it read
        // them when they arrived. Left out, every restored court read
        // depth zero for every address it had ever ruled.
        self.rebuild_attestors();
        // Unruled connections that survived the prune re-enter the docket.
        let held: Vec<Element> = self
            .basis
            .fiber(crate::network::nerve::ROOT)
            .filter(|(_, w)| w.is_connection())
            .map(|(_, w)| w.element().clone())
            .collect();
        let Some((_, &ratifier)) = self.discoverers.fiber(crate::network::nerve::ROOT).next() else {
            return true;
        };
        for w in held {
            if !self.ruled_contains(&w.address()) {
                self.docket_connection(&w, ratifier);
            }
        }
        true
    }
}
