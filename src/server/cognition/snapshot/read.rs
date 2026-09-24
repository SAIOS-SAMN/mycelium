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
    /// P7 — RESTORE a snapshot into a fresh kernel. Returns `false` on a
    /// bad magic or a section that does not parse, leaving the kernel
    /// empty: nothing is partly restored.
    ///
    /// AND IT SAYS SO, naming the section it stopped in and the byte it
    /// stopped at. The refusal is the caller's to act on and the server's
    /// boot REFUSES on it (X8): a `false` here is a whole court's memory —
    /// its basis, its chain, its treasury — and continuing without it
    /// means the next persist writes an empty court over the ledger that
    /// would not read. Measured on the fleet at fc75a59: hypha b's 16 MB
    /// ledger restored as nothing, ran two epochs reporting `basis=0 K=0`,
    /// and was overwritten, because the only line here was the one printed
    /// when it SUCCEEDED and the only branch taken was the one that
    /// ignored the failure.
    pub fn restore(&mut self, b: &[u8]) -> bool {
        if b.get(0..4) != Some(b"MYC7") {
            // A REFUSAL NAMES WHAT IT READ, not only what it wanted. "The
            // magic is not MYC7" leaves the operator to go and look at the
            // bytes this reader already had in its hand, and a tape that
            // is not a ledger at all reads the same as one written by an
            // older build -- which is the difference that decides whether
            // a recut is the road or the file is corrupt.
            let found = match b.get(0..4) {
                Some(m) => match std::str::from_utf8(m) {
                    Ok(s) => format!("`{s}`"),
                    Err(_) => format!("{m:02x?}, which is not text"),
                },
                None => format!("{} bytes, too short to carry one", b.len()),
            };
            eprintln!("  ledger refused: the magic is {found}, not MYC7");
            return false;
        }
        // WHERE THE READ IS. Set as each section opens, so a refusal names
        // one of the tape's own sections and not a byte offset alone.
        let at = std::cell::Cell::new("basis");
        let mut pos = 4usize;
        let read_u64 = |b: &[u8], p: &mut usize| -> Option<usize> {
            let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            usize::try_from(v).ok()
        };
        // A SECTION'S FRAME: its record count, and the byte its body ends
        // at. The end is read, not inferred from parsing the records, which
        // is what makes a lost frame detectable at all — see
        // `snapshot::framed` for the ledgers that were refused because it
        // was not.
        let section = crate::snapshot::take_frame;
        // A SECTION CLOSES WHERE IT SAID IT WOULD, or it is refused here
        // and says by how much it was out. The old reader could only refuse
        // at whatever field first failed to parse after the frame drifted,
        // which attributed the fault to a section that was intact.
        let closes = |name: &'static str, pos: &usize, start: usize, end: usize, n: usize| -> Option<()> {
            if *pos == end {
                return Some(());
            }
            eprintln!(
                "  section {name}: its {n} records declare the body {start}..{end} and the \
                 reader stopped at {pos} — the frame was lost inside this section"
            );
            None
        };
        // A COUNT THAT CANNOT BE SATISFIED, refused against the tape's own
        // remaining bytes and nothing else. Every record in every section
        // below writes at least one byte, so a count larger than what is
        // left cannot be honoured — which is EXACT and derived from the
        // tape, not estimated from a record's size.
        //
        // A divisor typed per section — a guess at a minimum record — can
        // be wrong in the direction that refuses valid content: a DEF1
        // record is `addr ‖ μ⃗[4] ‖ tag u8`, 35 bytes plus the address, so a
        // record whose address is short — a chart key, a simplex key — is
        // smaller than any divisor assumed for it, and a ledger this build
        // wrote would refuse to read on this build.
        //
        // The bound is the tape's bytes (rule 10) and the count decides
        // nothing else (rule 2). This is not a weaker guard than a divisor:
        // the loops below reserve nothing from `n`, and each refuses at the
        // first field the tape cannot supply, so a count past the end costs
        // one iteration and then refuses.
        let fits = |b: &[u8], pos: &usize, n: usize| -> Option<()> {
            match n > b.len().checked_sub(*pos)? {
                true => None,
                false => Some(()),
            }
        };
        // A WORD ROW'S ADMITTING STAMP (78.24 (2b)): `0`, or `1` and the
        // stamp; any other tag refuses the row.
        let take_admitted = |b: &[u8], p: &mut usize| -> Option<Option<crate::chronometry::Stamp>> {
            let tag = *b.get(*p)?;
            *p += 1;
            match tag {
                0 => Some(None),
                1 => Some(Some(crate::chronometry::Stamp::take_canonical(b, p)?)),
                _ => None,
            }
        };
        // A ROW'S AWARD (LD0 step 4), read beside its admitting stamp.
        let take_award = crate::economy::persist::take_award;
        let load = |kernel: &mut Kernel, b: &[u8], pos: &mut usize| -> Option<()> {
            let (n, sec_end) = section(b, pos)?;
            let start = *pos;
            // The basis is read before the treasury it returns awards to.
            let mut awards: Vec<(Address, crate::economy::escrow::Escrowed)> = Vec::new();
            for _ in 0..n {
                let elen = read_u64(b, pos)?;
                let mut ep = *pos;
                let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                let spoken = Element::from_bytes(&b[..end], &mut ep)?;
                *pos += elen;
                let who: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                *pos += 32;
                let addr = spoken.address();
                if let Some(at) = take_admitted(b, pos)? {
                    kernel.file_admitted(&addr, at);
                }
                if let Some(award) = take_award(b, pos)? {
                    awards.push((addr, award));
                }
                kernel.basis_insert(spoken, who);
            }
            closes("basis", pos, start, sec_end, n)?;
            at.set("treasury");
            kernel.treasury = crate::economy::Treasury::restore(b, pos)?;
            for (addr, award) in awards {
                kernel.treasury.restore_award(&addr, award);
            }
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
            let (np, sec_end) = section(b, pos)?;
            let start = *pos;
            for _ in 0..np {
                let addr = Address::from(crate::snapshot::take_key(b, pos)?);
                let elen = read_u64(b, pos)?;
                let mut ep = *pos;
                let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                let spoken = Element::from_bytes(&b[..end], &mut ep)?;
                *pos += elen;
                if let Some(at) = take_admitted(b, pos)? {
                    kernel.file_admitted(&addr, at);
                }
                if let Some(award) = take_award(b, pos)? {
                    kernel.treasury.restore_award(&addr, award);
                }
                kernel.provisional.insert(addr, spoken);
            }
            closes("provisional", pos, start, sec_end, np)?;
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
            // THE LEDGER (78.24): the court head, then the ledger read as
            // the trie it was written as — its own line and its peers',
            // every node re-deriving its hash, the line verified whole,
            // the peer placements re-derived.
            tag(b, pos, b"LDG1")?;
            let head: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
            *pos += 32;
            let (n, sec_end) = section(b, pos)?;
            let start = *pos;
            let chain = crate::chronometry::ReceiptChain::read(b, pos)?;
            closes("LDG1", pos, start, sec_end, n)?;
            kernel.court_head = head;
            kernel.chain = chain;
            // ARC1: the eras, in K order. Each era's receipts re-verify as
            // a chain of their own below their root.
            tag(b, pos, b"ARC1")?;
            let (ne, sec_end) = section(b, pos)?;
            let start = *pos;
            let mut archive = Vec::new();
            for _ in 0..ne {
                archive.push(crate::chronometry::Era::read(b, pos)?);
            }
            closes("ARC1", pos, start, sec_end, ne)?;
            kernel.archive = archive;
            // The prior measurement, continued incrementally (never read as
            // current: its generations are unknowable here).
            tag(b, pos, b"RNK2")?;
            {
                let basis_seq = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
                let (n, sec_end) = section(b, pos)?;
                let start = *pos;
                let mut rank = Rank::default();
                rank.basis_gen = u64::MAX;
                rank.basis_seq = basis_seq;
                rank.retire_gen = kernel.retire_gen;
                for _ in 0..n {
                    let addr = Address::from(crate::snapshot::take_key(b, pos)?);
                    let closed = crate::snapshot::take_key(b, pos)?;
                    let m = read_u64(b, pos)?;
                    fits(b, pos, m)?;
                    let bytes = b.get(*pos..*pos + m)?;
                    *pos += m;
                    let inv = crate::transducer::stalk::Invariant::from_key(bytes)?;
                    // An empty key is the absent closure; a non-empty one is
                    // the base point's own coordinate.
                    rank.put_closure(
                        &addr,
                        (!closed.is_empty()).then(|| Address::from(closed.clone())),
                    );
                    rank.invariants.insert(&addr, inv);
                }
                closes("RNK2", pos, start, sec_end, n)?;
                kernel.closure = (u64::MAX, u64::MAX, std::sync::Arc::new(rank));
            }
            tag(b, pos, b"DEF2")?;
            {
                // addr ‖ μ⃗[4].
                let (n, sec_end) = section(b, pos)?;
                let start = *pos;
                for i in 0..n {
                    let Some(addr) = crate::snapshot::take_key(b, pos).map(Address::from) else {
                        eprintln!(
                            "  DEF2 record {i} of {n}: the address does not read at byte {} \
                             (body {start}..{sec_end})",
                            *pos
                        );
                        return None;
                    };
                    let mut mag = [0u64; 4];
                    for m in &mut mag {
                        *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                        *pos += 8;
                    }
                    kernel.deflations.insert(&addr, mag);
                }
                closes("DEF2", pos, start, sec_end, n)?;
            }
            tag(b, pos, b"FLD1")?;
            {
                let (n, sec_end) = section(b, pos)?;
                let start = *pos;
                for _ in 0..n {
                    let addr = Address::from(crate::snapshot::take_key(b, pos)?);
                    let ne = read_u64(b, pos)?;
                    fits(b, pos, ne)?;
                    let mut entries = Vec::new();
                    for _ in 0..ne {
                        let m = read_u64(b, pos)?;
                        fits(b, pos, m)?;
                        let chart = crate::transducer::stalk::Invariant::from_key(b.get(*pos..*pos + m)?)?;
                        *pos += m;
                        let tag = *b.get(*pos)?;
                        *pos += 1;
                        let state = match tag {
                            0 => crate::transducer::stalk::Closure::Flat,
                            1 => crate::transducer::stalk::Closure::Twisted(
                                Box::new(element::Versor::from_bytes(b, pos)?),
                            ),
                            3 => {
                                let d = *b.get(*pos)?;
                                *pos += 1;
                                crate::transducer::stalk::Closure::Class(
                                    Box::new(crate::transducer::alphabet::Rotor::from_bytes(b, pos)?),
                                    d,
                                )
                            }
                            _ => crate::transducer::stalk::Closure::Cavity,
                        };
                        let nc = read_u64(b, pos)?;
                        fits(b, pos, nc)?;
                        let mut remainder = Vec::new();
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
                            chart,
                            state,
                            remainder,
                            stagnant,
                        });
                    }
                    let nonflat = entries.iter().filter(|e| !e.state.is_flat()).count() as u32;
                    kernel.field.insert(addr, ListingField { entries, nonflat });
                }
                closes("FLD1", pos, start, sec_end, n)?;
            }
            tag(b, pos, b"BOK1")?;
            {
                let (n, sec_end) = section(b, pos)?;
                let start = *pos;
                for _ in 0..n {
                    let elen = read_u64(b, pos)?;
                    let mut ep = *pos;
                    let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                    let elem = Element::from_bytes(&b[..end], &mut ep)?;
                    *pos += elen;
                    let addr = elem.address();
                    if let Some(at) = take_admitted(b, pos)? {
                        kernel.file_admitted(&addr, at);
                    }
                    if let Some(award) = take_award(b, pos)? {
                        kernel.treasury.restore_award(&addr, award);
                    }
                    kernel.on_page.insert(&addr, ());
                    kernel.listing_atlas.admit(&crate::transducer::stalk::invariant_of(&elem), &addr);
                    kernel.pending.insert(&addr, elem);
                }
                closes("BOK1", pos, start, sec_end, n)?;
            }
            tag(b, pos, b"CAV1")?;
            {
                let (n, sec_end) = section(b, pos)?;
                let start = *pos;
                for _ in 0..n {
                    let addr = Address::from(crate::snapshot::take_key(b, pos)?);
                    let elen = read_u64(b, pos)?;
                    let mut ep = *pos;
                    let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                    let elem = Element::from_bytes(&b[..end], &mut ep)?;
                    *pos += elen;
                    if let Some(at) = take_admitted(b, pos)? {
                        kernel.file_admitted(&addr, at);
                    }
                    if let Some(award) = take_award(b, pos)? {
                        kernel.treasury.restore_award(&addr, award);
                    }
                    kernel.listing_atlas.admit(&crate::transducer::stalk::invariant_of(&elem), &addr);
                    kernel.cavity_book.insert(&addr, elem);
                }
                closes("CAV1", pos, start, sec_end, n)?;
            }
            tag(b, pos, b"CRT1")?;
            {
                let (n, sec_end) = section(b, pos)?;
                let start = *pos;
                for _ in 0..n {
                    let addr = Address::from(crate::snapshot::take_key(b, pos)?);
                    let elen = read_u64(b, pos)?;
                    let mut ep = *pos;
                    let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                    let elem = Element::from_bytes(&b[..end], &mut ep)?;
                    *pos += elen;
                    kernel.court_fed_index.insert(&addr, kernel.court_federation.len());
                    kernel.court_federation.push((addr, elem));
                }
                closes("CRT1", pos, start, sec_end, n)?;
                // A trailing count of a sub-section no longer written, read
                // by peeking at whether the next four bytes were the NEXT
                // section's tag. `snapshot` emitted a constant zero there
                // and nothing else ever will, so both halves are gone.
            }
            tag(b, pos, b"COC1")?;
            {
                let (n, sec_end) = section(b, pos)?;
                let start = *pos;
                for _ in 0..n {
                    let key = Address::from(crate::snapshot::take_key(b, pos)?);
                    let c = crate::transducer::cech::Complex::from_bytes(b, pos)?;
                    kernel.complexes.insert(key, c);
                }
                closes("COC1", pos, start, sec_end, n)?;
            }
            tag(b, pos, b"FLC1")?;
            {
                let (n, sec_end) = section(b, pos)?;
                let start = *pos;
                for _ in 0..n {
                    let addr = Address::from(crate::snapshot::take_key(b, pos)?);
                    let ne = read_u64(b, pos)?;
                    fits(b, pos, ne)?;
                    let mut entries = Vec::new();
                    for _ in 0..ne {
                        let m = read_u64(b, pos)?;
                        fits(b, pos, m)?;
                        let chart = crate::transducer::stalk::Invariant::from_key(b.get(*pos..*pos + m)?)?;
                        *pos += m;
                        let tag = *b.get(*pos)?;
                        *pos += 1;
                        let state = match tag {
                            0 => crate::transducer::stalk::Closure::Flat,
                            1 => crate::transducer::stalk::Closure::Twisted(
                                Box::new(element::Versor::from_bytes(b, pos)?),
                            ),
                            3 => {
                                let d = *b.get(*pos)?;
                                *pos += 1;
                                crate::transducer::stalk::Closure::Class(
                                    Box::new(crate::transducer::alphabet::Rotor::from_bytes(b, pos)?),
                                    d,
                                )
                            }
                            _ => crate::transducer::stalk::Closure::Cavity,
                        };
                        let nc = read_u64(b, pos)?;
                        fits(b, pos, nc)?;
                        let mut remainder = Vec::new();
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
                            chart,
                            state,
                            remainder,
                            stagnant,
                        });
                    }
                    let nonflat = entries.iter().filter(|e| !e.state.is_flat()).count() as u32;
                    kernel.field_cold.insert(addr, ListingField { entries, nonflat });
                }
                closes("FLC1", pos, start, sec_end, n)?;
            }
            // CYC1 was the same section without a stored class key, and its
            // reader re-derived one with `class_key_of_versor`, dropping the
            // row with a log line where the derivation failed. No writer has
            // emitted CYC1 since the key was stored; the key is read (§10).
            tag(b, pos, b"CYC2")?;
            {
                let (n, sec_end) = section(b, pos)?;
                let start = *pos;
                for _ in 0..n {
                    let key = Address::from(crate::snapshot::take_key(b, pos)?);
                    let a: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let bp: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let tu = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let class_key = Address::from(crate::snapshot::take_key(b, pos)?);
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
                closes("CYC2", pos, start, sec_end, n)?;
                let (nc, chain_end) = section(b, pos)?;
                let chain_start = *pos;
                for _ in 0..nc {
                    let key = Address::from(crate::snapshot::take_key(b, pos)?);
                    let tu = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let v = element::Versor::from_bytes(b, pos)?;
                    kernel.dialect_chain.push((key.into_vec(), v, tu));
                }
                closes("CYC2/dialect", pos, chain_start, chain_end, nc)?;
            }
            tag(b, pos, b"SET1")?;
            {
                let (n, sec_end) = section(b, pos)?;
                let start = *pos;
                for _ in 0..n {
                    let listing = Address::from(crate::snapshot::take_key(b, pos)?);
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
                closes("SET1", pos, start, sec_end, n)?;
            }
            // index: keyed field at restore (persistence snapshot, once)
            let field_keys: Vec<Vec<u8>> = kernel.field.scoped().map(|(k, _)| k).collect();
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
            .scoped()
            .map(|(_, e)| e.address())
            .collect();
        for addr in pending_addrs {
            if self.standing_h.get(&addr).is_none() {
                self.standing_h.insert(addr, self.height());
            }
        }
        let prov: Vec<Vec<u8>> = self.provisional.scoped().map(|(a, _)| a).collect();
        for addr in prov {
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
            .scoped()
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
            .scoped()
            .filter(|(_, w)| w.is_connection())
            .map(|(_, w)| w.element().clone())
            .collect();
        // EACH CONNECTION RE-ENTERS UNDER ITS OWN DISCOVERER, read by its
        // address. This took `discoverers.scoped().next()` — the first entry
        // of the trie — and docketed EVERY unruled connection under that one
        // voice, so a boot reattributed the whole docket to whoever happened
        // to sort first, and `settle_use` pays a discoverer. Which voice that
        // was is an insertion order in a trie (rule 15), and provenance is
        // not a thing a carrier's shape may decide.
        //
        // A connection whose discoverer this court does not hold is not
        // docketed under someone else's name: it is left, which is absence
        // and not a default.
        for w in held {
            if self.ruled_contains(&w.address()) {
                continue;
            }
            let Some(ratifier) = self.discoverer_of(&w.address()) else {
                continue;
            };
            self.docket_connection(&w, ratifier);
        }
        true
    }
}
