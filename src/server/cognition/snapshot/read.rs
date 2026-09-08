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
    pub fn restore(&mut self, b: &[u8]) -> bool {
        if b.get(0..4) != Some(b"MYC3") {
            return false;
        }
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
            kernel.treasury = crate::economy::Treasury::restore(b, pos)?;
            // Provisional section (absent in pre-court snapshots → empty docket).
            if let Some(np) = read_u32(b, pos) {
                let np = np as usize;
                if np > b.len().saturating_sub(*pos) / 6 {
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
            }
            // Chain section (absent in earlier snapshots → the clock starts).
            if b.get(*pos..*pos + 4) == Some(b"KCH1") {
                *pos += 4;
                let head: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                *pos += 32;
                let n = read_u32(b, pos)? as usize;
                if n > b.len().saturating_sub(*pos) / 106 {
                    return None;
                }
                let mut receipts = Vec::with_capacity(n);
                for _ in 0..n {
                    receipts.push(crate::chronometry::KStepReceipt::read(b, pos)?);
                }
                kernel.court_head = head;
                kernel.chain = crate::chronometry::ReceiptChain::from_receipts(receipts)?;
            }
            // Rank section: the prior measurement, continued incrementally
            // (never read as current: its generations are unknowable here).
            if b.get(*pos..*pos + 4) == Some(b"RNK1") {
                *pos += 4;
                let basis_len = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?) as usize;
                *pos += 8;
                let n = read_u32(b, pos)? as usize;
                if n > b.len().saturating_sub(*pos) / 14 {
                    return None;
                }
                let mut rank = Rank { basis_gen: u64::MAX, basis_len, retire_gen: kernel.retire_gen, ..Rank::default() };
                for _ in 0..n {
                    let addr = crate::transducer::address::take_addr(b, pos)?;
                    let closed = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let m = read_u32(b, pos)? as usize;
                    if m > b.len().saturating_sub(*pos) / 8 {
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
            if b.get(*pos..*pos + 4) == Some(b"DEF1") {
                *pos += 4;
                let n = read_u32(b, pos)? as usize;
                // addr ‖ μ⃗[4] ‖ tag u8 [‖ K u64 ‖ tu u64]; refuse a count that cannot fit.
                if n > b.len().saturating_sub(*pos) / 50 {
                    return None;
                }
                for _ in 0..n {
                    let addr = crate::transducer::address::take_addr(b, pos)?;
                    let mut mag = [0u64; 4];
                    for m in &mut mag {
                        *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                        *pos += 8;
                    }
                    let tag = *b.get(*pos)?;
                    *pos += 1;
                    match tag {
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
            if b.get(*pos..*pos + 4) == Some(b"FLD1") {
                *pos += 4;
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
            if b.get(*pos..*pos + 4) == Some(b"BOK1") {
                *pos += 4;
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
            if b.get(*pos..*pos + 4) == Some(b"CAV1") {
                *pos += 4;
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
            if b.get(*pos..*pos + 4) == Some(b"CRT1") {
                *pos += 4;
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
                if *pos + 4 <= b.len() && b.get(*pos..*pos + 4) != Some(b"COC1") {
                    let n = read_u32(b, pos)? as usize;
                    if n > b.len().saturating_sub(*pos) {
                        return None;
                    }
                    for _ in 0..n {
                        let nax = read_u32(b, pos)? as usize;
                        for _ in 0..nax {
                            *pos = pos.checked_add(8).filter(|e| *e <= b.len())?;
                        }
                        *pos = pos.checked_add(64).filter(|e| *e <= b.len())?;
                        let _ = element::Versor::from_bytes(b, pos)?;
                    }
                }
            }
            if b.get(*pos..*pos + 4) == Some(b"COC1") {
                *pos += 4;
                let n = read_u32(b, pos)? as usize;
                if n > b.len().saturating_sub(*pos) {
                    return None;
                }
                for _ in 0..n {
                    let key = take_addr(b, pos)?;
                    let c = crate::transducer::cech::Complex::from_bytes(b, pos)?;
                    kernel.complexes.insert(key, c);
                }
            }
            if b.get(*pos..*pos + 4) == Some(b"FLC1") {
                *pos += 4;
                let n = read_u32(b, pos)? as usize;
                if n > b.len().saturating_sub(*pos) {
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
            let cyc2 = b.get(*pos..*pos + 4) == Some(b"CYC2");
            if cyc2 || b.get(*pos..*pos + 4) == Some(b"CYC1") {
                *pos += 4;
                let n = read_u32(b, pos)? as usize;
                if n > b.len().saturating_sub(*pos) {
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
                    let ck_stored = if cyc2 { Some(take_addr(b, pos)?) } else { None };
                    let v = element::Versor::from_bytes(b, pos)?;
                    let class_key = match ck_stored {
                        Some(k) => k,
                        None => match crate::transducer::sentence::class_key_of_versor(&v) {
                            Some(k) => k,
                            None => {
                                eprintln!(
                                    "  court cocycle row refused {:02x}{:02x}… no class key",
                                    key[0], key[1]
                                );
                                continue;
                            }
                        },
                    };
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
                if nc > b.len().saturating_sub(*pos) {
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
            if b.get(*pos..*pos + 4) == Some(b"SET1") {
                *pos += 4;
                let n = read_u32(b, pos)? as usize;
                if n > b.len().saturating_sub(*pos) {
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
                let mag = self.book_elem(&addr).map(|p| p.magnitude()).unwrap_or_else(|| spoken.magnitude());
                mag.iter().any(|&x| x > 0).then(|| (Address::from(addr.clone()), mag))
            })
            .collect();
        for (addr, mag) in need {
            if mag.iter().any(|&x| x > 0) {
                self.deflations.insert(addr, mag);
            }
        }
        // Unruled connections that survived the prune re-enter the docket.
        let held: Vec<Element> = self
            .basis
            .fiber(crate::network::nerve::ROOT)
            .filter(|(_, w)| w.is_connection())
            .map(|(_, w)| (**w).clone())
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
