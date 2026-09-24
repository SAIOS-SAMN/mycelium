// 28 — THE SNAPSHOT: every carrier of a mind written whole, section by
// section.
//
//! Each section carries its own four-byte tag and its own count, so a
//! reader knows what it is looking at before it reads it and a section it
//! does not recognise is a refusal rather than a guess.

use crate::cognition::*;

use crate::snapshot::{counted, framed, put_key};

impl Kernel {
    pub fn snapshot(&self) -> Vec<u8> {
        let mut out = Vec::new();
        // MYC7: every key carries a `u64` length, so no address on this tape
        // can be too long to write; MYC6 wrote the `u16` citation prefix of
        // the wire and a form's address outgrows it.
        // Every section states its own BYTE LENGTH beside its count,
        // so a reader knows where a section ends before it reads a record
        // and a lost frame refuses at the section that lost it. MYC5 wrote
        // the count alone; see `snapshot::framed` for what that cost on the
        // fleet. A layout change is a recut (§10), and this one is the
        // recut's reason.
        out.extend_from_slice(b"MYC7");
        let items: Vec<_> = self
            .basis
            .scoped()
            .filter_map(|(_, g)| self.discoverer_of(&g.address()).map(|who| (std::sync::Arc::clone(g), who)))
            .collect();
        let mut body = Vec::new();
        for (g, who) in &items {
            let eb = g.to_bytes();
            body.extend_from_slice(&(eb.len() as u64).to_le_bytes());
            body.extend_from_slice(&eb);
            body.extend_from_slice(who);
        }
        framed(&mut out, items.len() as u64, &mut body);
        out.extend_from_slice(&self.treasury.snapshot());
        // PROVISIONAL section (Step 5).
        counted(&mut out, &self.provisional, |body, addr, spoken| {
            put_key(body, addr);
            let eb = spoken.to_bytes();
            body.extend_from_slice(&(eb.len() as u64).to_le_bytes());
            body.extend_from_slice(&eb);
            Some(())
        });
        // THE CHAIN section: the court head and every k-step receipt, so
        // the hypha's clock continues across boots instead of restarting.
        out.extend_from_slice(b"KCH1");
        out.extend_from_slice(&self.court_head);
        // The fold root before the hot receipts (45.8, S13.9), so a chain
        // that begins at a fold re-verifies from it.
        crate::chronometry::Root::write_opt(self.chain.root().as_ref(), &mut out);
        let receipts = self.chain.receipts();
        let mut body = Vec::new();
        for r in receipts {
            r.write(&mut body);
        }
        framed(&mut out, receipts.len() as u64, &mut body);
        // PRC1: THE PEER RECEIPTS THIS COURT ADMITTED AS PARENTS. Our own
        // receipts cite them, and a height over an unheld parent is
        // unmeasurable, so a ledger without them is a ledger this reader
        // refuses — which is what it did to hypha b's 16 MB of history.
        // They are written whole and read back verified by content; the
        // two coordinates of where each stands against our line are
        // re-derived at restore and never persisted.
        out.extend_from_slice(b"PRC1");
        let admitted = self.chain.admitted();
        let mut body = Vec::new();
        for r in &admitted {
            r.write(&mut body);
        }
        framed(&mut out, admitted.len() as u64, &mut body);
        // ARC1: the eras this court folded, in K order, cold.
        out.extend_from_slice(b"ARC1");
        let mut body = Vec::new();
        for era in &self.archive {
            era.write(&mut body);
        }
        framed(&mut out, self.archive.len() as u64, &mut body);
        // THE RANK section: each listing's closure and moduli as last
        // measured, so a boot continues the measurement instead of reading
        // the whole book from nothing.
        let rank = &self.closure.2;
        // RNK2: the closure is the base point's ADDRESS, not a slot into a
        // basis snapshot that has since moved. An absent closure is the
        // empty key, which `put_key` writes as a zero length — a slot could
        // only spell absence as a sentinel (`u64::MAX`).
        out.extend_from_slice(b"RNK2");
        out.extend_from_slice(&rank.basis_seq.to_le_bytes());
        counted(&mut out, rank.closure_carrier(), |body, addr, closed| {
            put_key(body, addr);
            match closed {
                Some(c) => put_key(body, c.as_bytes()),
                None => put_key(body, []),
            }
            // A listing the reading holds no invariant for sits at the
            // root stratum, which the empty axis list is.
            let root = crate::transducer::stalk::Invariant::default();
            let inv = match rank.invariants.get(addr) {
                Some(i) => i,
                None => &root,
            };
            let key = inv.key();
            let bytes = key.as_bytes();
            body.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
            body.extend_from_slice(bytes);
            Some(())
        });
        // DEFLATION section: each provisional's measured deflation, so a
        // restart does not docket Δ⃗ = 0 and feed the court a slash list.
        out.extend_from_slice(b"DEF1");
        // DEF1 record: addr ‖ μ⃗[4] ‖ tag u8
        //   tag 0 = K and height absent (never written as 0)
        //   tag 1 = K u64 ‖ tu u64
        // Restore refuses any other tag (a missing K is not clock-zero).
        counted(&mut out, &self.deflations, |body, addr, mag| {
            put_key(body, addr);
            for m in mag {
                body.extend_from_slice(&m.to_le_bytes());
            }
            match (
                self.provisional_k.get(addr).copied(),
                self.provisional_h.get(addr).copied(),
            ) {
                (Some(k), Some(tu)) => {
                    body.push(1);
                    body.extend_from_slice(&k.to_le_bytes());
                    body.extend_from_slice(&tu.to_le_bytes());
                }
                _ => body.push(0),
            }
            Some(())
        });
        // Coherence field beside DEF1: which angles each thought has been
        // probed through. A restart must not forget the field.
        out.extend_from_slice(b"FLD1");
        counted(&mut out, &self.field, |body, addr, listing| {
            put_key(body, addr);
            body.extend_from_slice(&(listing.entries.len() as u64).to_le_bytes());
            for e in &listing.entries {
                let key = e.chart.key();
                body.extend_from_slice(&(key.as_bytes().len() as u64).to_le_bytes());
                body.extend_from_slice(key.as_bytes());
                match &e.state {
                    crate::transducer::stalk::Closure::Flat => body.push(0),
                    crate::transducer::stalk::Closure::Twisted(v) => {
                        body.push(1);
                        v.to_bytes(body);
                    }
                    crate::transducer::stalk::Closure::Cavity => body.push(2),
                    crate::transducer::stalk::Closure::Class(r, d) => {
                        body.push(3);
                        body.push(*d);
                        r.to_bytes(body);
                    }
                }
                body.extend_from_slice(&(e.remainder.len() as u64).to_le_bytes());
                for &(r, c, hb, ha) in &e.remainder {
                    body.extend_from_slice(&r.to_le_bytes());
                    body.extend_from_slice(&c.to_le_bytes());
                    body.push(hb);
                    body.push(ha);
                }
                for s in e.stagnant {
                    body.extend_from_slice(&s.to_le_bytes());
                }
            }
            Some(())
        });
        out.extend_from_slice(b"BOK1");
        counted(&mut out, &self.pending, |body, _addr, (elem, k)| {
            let eb = elem.to_bytes();
            body.extend_from_slice(&(eb.len() as u64).to_le_bytes());
            body.extend_from_slice(&eb);
            body.extend_from_slice(&k.to_le_bytes());
            Some(())
        });
        out.extend_from_slice(b"CAV1");
        counted(&mut out, &self.cavity_book, |body, addr, elem| {
            put_key(body, addr);
            let eb = elem.to_bytes();
            body.extend_from_slice(&(eb.len() as u64).to_le_bytes());
            body.extend_from_slice(&eb);
            Some(())
        });
        out.extend_from_slice(b"CRT1");
        out.push(self.crt_imported as u8);
        let mut body = Vec::new();
        for (addr, elem) in &self.court_federation {
            put_key(&mut body, addr);
            let eb = elem.to_bytes();
            body.extend_from_slice(&(eb.len() as u64).to_le_bytes());
            body.extend_from_slice(&eb);
        }
        framed(&mut out, self.court_federation.len() as u64, &mut body);
        // A constant zero stood here: the count of a sub-section this
        // writer stopped emitting, which the reader skipped by peeking at
        // whether the next four bytes were COC1's tag. Both are gone (§10).
        out.extend_from_slice(b"COC1");
        counted(&mut out, &self.complexes, |body, k, c| {
            put_key(body, k);
            c.to_bytes(body);
            Some(())
        });
        out.extend_from_slice(b"FLC1");
        counted(&mut out, &self.field_cold, |body, addr, listing| {
            put_key(body, addr);
            body.extend_from_slice(&(listing.entries.len() as u64).to_le_bytes());
            for e in &listing.entries {
                let key = e.chart.key();
                body.extend_from_slice(&(key.as_bytes().len() as u64).to_le_bytes());
                body.extend_from_slice(key.as_bytes());
                match &e.state {
                    crate::transducer::stalk::Closure::Flat => body.push(0),
                    crate::transducer::stalk::Closure::Twisted(v) => {
                        body.push(1);
                        v.to_bytes(body);
                    }
                    crate::transducer::stalk::Closure::Cavity => body.push(2),
                    crate::transducer::stalk::Closure::Class(r, d) => {
                        body.push(3);
                        body.push(*d);
                        r.to_bytes(body);
                    }
                }
                body.extend_from_slice(&(e.remainder.len() as u64).to_le_bytes());
                for &(r, c, hb, ha) in &e.remainder {
                    body.extend_from_slice(&r.to_le_bytes());
                    body.extend_from_slice(&c.to_le_bytes());
                    body.push(hb);
                    body.push(ha);
                }
                for s in e.stagnant {
                    body.extend_from_slice(&s.to_le_bytes());
                }
            }
            Some(())
        });
        out.extend_from_slice(b"CYC2");
        counted(&mut out, &self.cocycle_at, |body, key, row| {
            put_key(body, key);
            body.extend_from_slice(&row.a);
            body.extend_from_slice(&row.b);
            body.extend_from_slice(&row.t_universal.to_le_bytes());
            put_key(body, &row.class_key);
            row.gauge.to_bytes(body);
            Some(())
        });
        let mut body = Vec::new();
        for (key, v, tu) in &self.dialect_chain {
            put_key(&mut body, key);
            body.extend_from_slice(&tu.to_le_bytes());
            v.to_bytes(&mut body);
        }
        framed(&mut out, self.dialect_chain.len() as u64, &mut body);
        out.extend_from_slice(b"SET1");
        let mut all = self.settlements_cold.clone();
        all.extend(self.settlements.iter().cloned());
        let mut body = Vec::new();
        let mut kept = 0u64;
        for s in &all {
            // LD0 — a settlement whose citations cannot be framed is not
            // written, named and left out of the count, never a partial row.
            let mut row = Vec::new();
            put_key(&mut row, &s.listing);
            crate::anastome::dialogue::push_chain(&mut row, &s.chain);
            row.extend_from_slice(&s.voice);
            if crate::anastome::dialogue::push_cites(&mut row, &s.cites).is_none() {
                eprintln!("  snapshot settlement refused: a citation's address cannot be framed");
                continue;
            }
            for m in &s.before {
                row.extend_from_slice(&m.to_le_bytes());
            }
            for m in &s.after {
                row.extend_from_slice(&m.to_le_bytes());
            }
            row.extend_from_slice(&s.k.to_le_bytes());
            row.extend_from_slice(&s.t_universal.to_le_bytes());
            body.append(&mut row);
            kept += 1;
        }
        framed(&mut out, kept, &mut body);
        out
    }

}
