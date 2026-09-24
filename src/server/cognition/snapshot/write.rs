// 28 — THE SNAPSHOT: every carrier of a mind written whole, section by
// section.
//
//! Each section carries its own four-byte tag and its own count, so a
//! reader knows what it is looking at before it reads it and a section it
//! does not recognise is a refusal rather than a guess.

use crate::cognition::*;

use crate::snapshot::{counted, framed, put_key};

/// A WORD ROW'S ADMITTING STAMP (78.24 (2b)): `0`, or `1` and the stamp. A
/// row no act has admitted says so rather than carrying a stamp of zeros.
fn put_admitted(out: &mut Vec<u8>, at: Option<&crate::chronometry::Stamp>) {
    match at {
        None => out.push(0),
        Some(s) => {
            out.push(1);
            s.put_canonical(out);
        }
    }
}

/// A LISTING'S COHERENCE FIELD as a row field (LD0 step 5): `0`, or `1`
/// and its entries — each chart as its own coordinates, each closure as
/// its canonical ray, the remainder and the stagnation counts.
pub(super) fn put_field(out: &mut Vec<u8>, field: Option<&ListingField>) {
    let Some(listing) = field else {
        out.push(0);
        return;
    };
    out.push(1);
    out.extend_from_slice(&(listing.entries.len() as u64).to_le_bytes());
    for e in &listing.entries {
        e.chart.put_canonical(out);
        match &e.state {
            crate::transducer::stalk::Closure::Flat => out.push(0),
            crate::transducer::stalk::Closure::Twisted(v) => {
                out.push(1);
                v.put_canonical(out);
            }
            crate::transducer::stalk::Closure::Cavity => out.push(2),
            crate::transducer::stalk::Closure::Class(r, d) => {
                out.push(3);
                out.push(*d);
                r.put_canonical(out);
            }
        }
        out.extend_from_slice(&(e.remainder.len() as u64).to_le_bytes());
        for &(r, c, hb, ha) in &e.remainder {
            out.extend_from_slice(&r.to_le_bytes());
            out.extend_from_slice(&c.to_le_bytes());
            out.push(hb);
            out.push(ha);
        }
        for s in e.stagnant {
            out.extend_from_slice(&s.to_le_bytes());
        }
    }
}

/// A WORD'S DEFLATION as a row field (LD0 step 5): `0`, or `1` and μ⃗.
fn put_deflation(out: &mut Vec<u8>, mag: Option<&[u64; 4]>) {
    let Some(mag) = mag else {
        out.push(0);
        return;
    };
    out.push(1);
    for m in mag {
        out.extend_from_slice(&m.to_le_bytes());
    }
}

/// THE ROWS A WORD CAN STAND ON, in the order its per-word fields are
/// written on the first that holds it (LD0 steps 4 and 5): an award, a
/// deflation and a coherence field are each a field of ONE row.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Row {
    Provisional,
    Listing,
    Cavity,
    Basis,
}

impl Kernel {
    /// The row a word's per-word fields are written on: the first of
    /// provisional, the listing book, the cavity and the basis that holds
    /// the word.
    pub(super) fn home_row(&self, addr: &[u8]) -> Option<Row> {
        if self.provisional.contains(addr) {
            Some(Row::Provisional)
        } else if self.pending.contains(addr) {
            Some(Row::Listing)
        } else if self.cavity_book.contains(addr) {
            Some(Row::Cavity)
        } else if self.basis.contains(addr) {
            Some(Row::Basis)
        } else {
            None
        }
    }

    /// A row's own fields after its word: the admitting stamp, then — where
    /// this row is the word's home — its award, its deflation, and its
    /// coherence field hot and cold.
    fn put_row_fields(&self, out: &mut Vec<u8>, addr: &[u8], row: Row) {
        put_admitted(out, self.admitted_at.get(addr));
        let home = self.home_row(addr) == Some(row);
        crate::economy::persist::put_award(out, self.treasury.escrow_of(addr).filter(|_| home));
        put_deflation(out, self.deflations.get(addr).filter(|_| home));
        put_field(out, self.field.get(addr).filter(|_| home));
        put_field(out, self.field_cold.get(addr).filter(|_| home));
    }

    /// A WORD ROW'S SECTION, WRITTEN FOR THE WORDS AN ACT ADMITTED (ACT1-hold,
    /// 78.27 Q1): a word no act admitted has no coordinates and is not
    /// written; `snapshot` names how many each row refused, once.
    fn stamped_rows<V>(
        &self,
        out: &mut Vec<u8>,
        carrier: &crate::network::nerve::Keyed<V>,
        mut write: impl FnMut(&mut Vec<u8>, &[u8], &V),
    ) {
        let mut body = Vec::new();
        let mut n = 0u64;
        crate::snapshot::persist(carrier, |key, v| {
            if self.admitted_at.contains(key) {
                write(&mut body, key, v);
                n += 1;
            }
        });
        framed(out, n, &mut body);
    }

    /// A PER-WORD FIELD NO ROW HOLDS is a quantity on a word this court no
    /// longer has: named, per carrier, and not written — never a row of
    /// its own that copies the address (R6).
    fn name_orphans(&self) {
        let award = self.treasury.award_addresses().iter().filter(|a| self.home_row(a).is_none()).count();
        let orphans = |keys: Vec<Vec<u8>>| keys.iter().filter(|a| self.home_row(a).is_none()).count();
        let deflation = orphans(self.deflations.scoped().map(|(a, _)| a).collect());
        let hot = orphans(self.field.scoped().map(|(a, _)| a).collect());
        let cold = orphans(self.field_cold.scoped().map(|(a, _)| a).collect());
        if award + deflation + hot + cold > 0 {
            eprintln!(
                "  snapshot fields refused on words no row holds: awards {award} deflations {deflation} \
                 fields {hot} cold fields {cold}"
            );
        }
    }
}

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
        // ACT1-hold (78.27 Q1): A WORD NO ACT ADMITTED IS NOT WRITTEN. It has
        // no coordinates — no stamp to locate it — so each row below writes
        // only the words an act admitted, and what each refused is named
        // here, per row, from the reading the clock carries.
        let [b, p, l, c, f] = self.unstamped();
        if b + p + l + c + f > 0 {
            eprintln!(
                "  snapshot refused words held without a stamp: basis {b} provisional {p} \
                 listing {l} cavity {c} federation {f}"
            );
        }
        let items: Vec<_> = self
            .basis
            .scoped()
            .filter(|(a, _)| self.admitted_at.contains(a))
            .filter_map(|(_, g)| self.discoverer_of(&g.address()).map(|who| (std::sync::Arc::clone(g), who)))
            .collect();
        let mut body = Vec::new();
        for (g, who) in &items {
            let eb = g.to_bytes();
            body.extend_from_slice(&(eb.len() as u64).to_le_bytes());
            body.extend_from_slice(&eb);
            body.extend_from_slice(who);
            self.put_row_fields(&mut body, g.address().as_bytes(), Row::Basis);
        }
        framed(&mut out, items.len() as u64, &mut body);
        out.extend_from_slice(&self.treasury.snapshot());
        // PROVISIONAL section (Step 5): each row with its admitting stamp.
        self.stamped_rows(&mut out, &self.provisional, |body, addr, spoken| {
            put_key(body, addr);
            let eb = spoken.to_bytes();
            body.extend_from_slice(&(eb.len() as u64).to_le_bytes());
            body.extend_from_slice(&eb);
            self.put_row_fields(body, addr, Row::Provisional);
        });
        self.name_orphans();
        // THE LEDGER (78.24): the court head, then the ledger as its trie —
        // the key it signs as, the fold root, and every node it holds, its
        // own line and its peers', each once at its stamp. Where each peer
        // node stands against our line is re-derived at restore, never
        // persisted.
        out.extend_from_slice(b"LDG1");
        out.extend_from_slice(&self.court_head);
        let mut body = Vec::new();
        if self.chain.write(&mut body).is_none() {
            eprintln!("  snapshot ledger refused: a node cannot be written");
            body.clear();
            // The empty ledger under this court's key: a restore reads a
            // court with no history, which it names, rather than a tape
            // cut in the middle of a node.
            if crate::chronometry::ReceiptChain::with_vk(self.identity).write(&mut body).is_none() {
                body.clear();
            }
        }
        framed(&mut out, 1, &mut body);
        // ARC1: the eras this court folded, in K order, cold.
        out.extend_from_slice(b"ARC1");
        let mut body = Vec::new();
        let mut kept = 0u64;
        for era in &self.archive {
            if era.write(&mut body).is_none() {
                eprintln!("  snapshot era refused: a receipt of its payload cannot be written");
                continue;
            }
            kept += 1;
        }
        framed(&mut out, kept, &mut body);
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
            // The chart as its own coordinates, never its key nested.
            inv.put_canonical(body);
            Some(())
        });
        // DEF2, FLD1 AND FLC1 ARE GONE (LD0 step 5): a deflation and a
        // coherence field are per-word quantities, fields of the word's
        // row, each written where the word is and nowhere else.
        out.extend_from_slice(b"BOK1");
        self.stamped_rows(&mut out, &self.pending, |body, addr, elem| {
            let eb = elem.to_bytes();
            body.extend_from_slice(&(eb.len() as u64).to_le_bytes());
            body.extend_from_slice(&eb);
            self.put_row_fields(body, addr, Row::Listing);
        });
        out.extend_from_slice(b"CAV1");
        self.stamped_rows(&mut out, &self.cavity_book, |body, addr, elem| {
            put_key(body, addr);
            let eb = elem.to_bytes();
            body.extend_from_slice(&(eb.len() as u64).to_le_bytes());
            body.extend_from_slice(&eb);
            self.put_row_fields(body, addr, Row::Cavity);
        });
        out.extend_from_slice(b"CRT1");
        let mut body = Vec::new();
        let mut ruled = 0u64;
        for (addr, elem) in &self.court_federation {
            if !self.admitted_at.contains(addr) {
                continue;
            }
            put_key(&mut body, addr);
            let eb = elem.to_bytes();
            body.extend_from_slice(&(eb.len() as u64).to_le_bytes());
            body.extend_from_slice(&eb);
            ruled += 1;
        }
        framed(&mut out, ruled, &mut body);
        // A constant zero stood here: the count of a sub-section this
        // writer stopped emitting, which the reader skipped by peeking at
        // whether the next four bytes were COC1's tag. Both are gone (§10).
        out.extend_from_slice(b"COC1");
        counted(&mut out, &self.complexes, |body, k, c| {
            put_key(body, k);
            c.to_bytes(body);
            Some(())
        });
        // The cocycle row's gauge and the dialect chain's transitions as
        // their canonical rays (LD0 step 5, A4).
        out.extend_from_slice(b"CYC2");
        counted(&mut out, &self.cocycle_at, |body, key, row| {
            put_key(body, key);
            body.extend_from_slice(&row.a);
            body.extend_from_slice(&row.b);
            body.extend_from_slice(&row.t_universal.to_le_bytes());
            put_key(body, &row.class_key);
            row.gauge.put_canonical(body);
            Some(())
        });
        let mut body = Vec::new();
        for (key, v, tu) in &self.dialect_chain {
            put_key(&mut body, key);
            body.extend_from_slice(&tu.to_le_bytes());
            v.put_canonical(&mut body);
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
