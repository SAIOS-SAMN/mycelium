// 28 — THE TREASURY ON DISK: every carrier written whole, and the supply
// folded back from the rows at load.
//
//! A carrier is persisted and the supply is re-derived, so a tape whose
//! carriers disagree with its journal is caught by the audit (§6) rather
//! than carried forward.

use super::*;

impl Entry {
    /// One row's tape: `[tag u8][who 32][μ⃗ 4×u64][stamp 48][receipt 32]`,
    /// 145 bytes — the one layout the ledger, the snapshot and the wire
    /// (`ACT_ROWS`) share. The stamp is the act the row stands at (78.24).
    pub const TAPE: usize = 1 + 32 + 32 + crate::chronometry::Stamp::LEN + 32;

    /// One tag per kind, and the match is exhaustive: a kind that could not
    /// be written would not compile, where a string match once fell
    /// through to `255` and wrote a row nothing could read back.
    pub fn write(&self, out: &mut Vec<u8>) {
        let tag: u8 = match self.kind {
            Kind::Stake => 4,
            Kind::Slash => 8,
            Kind::Mint => 14,
            Kind::Transfer => 15,
            Kind::Genesis => 16,
            Kind::Burn => 17,
            Kind::Escrow => 18,
            Kind::Release => 19,
            Kind::Revoke => 20,
            Kind::Unstake => 21,
            Kind::Epoch => 22,
        };
        out.push(tag);
        out.extend_from_slice(&self.who);
        for m in &self.mu {
            out.extend_from_slice(&m.to_le_bytes());
        }
        self.at.put_canonical(out);
        out.extend_from_slice(&self.receipt);
    }

    /// A tag the layout does not define REFUSES the tape (rule 11, A21 1:
    /// the decoder is total on well-formed keys and refuses otherwise);
    /// a row read as an unknown kind was once dropped from the supply and
    /// the audit then compared carriers against a journal it had not read
    /// whole.
    pub fn read(b: &[u8], pos: &mut usize) -> Option<Entry> {
        let tag = *b.get(*pos)?;
        *pos += 1;
        let kind = match tag {
            4 => Kind::Stake,
            8 => Kind::Slash,
            14 => Kind::Mint,
            15 => Kind::Transfer,
            16 => Kind::Genesis,
            17 => Kind::Burn,
            18 => Kind::Escrow,
            19 => Kind::Release,
            20 => Kind::Revoke,
            21 => Kind::Unstake,
            22 => Kind::Epoch,
            _ => return None,
        };
        let who: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        let mut mu = [0u64; 4];
        for m in &mut mu {
            *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
        let at = crate::chronometry::Stamp::take_canonical(b, pos)?;
        let receipt: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        Some(Entry { kind, who, mu, at, receipt })
    }
}

/// AN AWARD AS A FIELD OF ITS WORD'S ROW (78.22, LD0 step 4): `0`, or `1`
/// then `[nv u32][voice 32·nv][μ⃗ 4×u64][stamp 48][receipt 32]`. The word's
/// row already carries its address, once; the award carries none. The one
/// writing of it, for the snapshot and for LD1's measure.
pub fn put_award(out: &mut Vec<u8>, award: Option<&super::escrow::Escrowed>) {
    let Some(row) = award else {
        out.push(0);
        return;
    };
    out.push(1);
    out.extend_from_slice(&(row.voices.len() as u32).to_le_bytes());
    for v in &row.voices {
        out.extend_from_slice(v);
    }
    for m in &row.mu {
        out.extend_from_slice(&m.to_le_bytes());
    }
    row.anchor.0.put_canonical(out);
    out.extend_from_slice(&row.anchor.1);
}

/// The inverse of [`put_award`]: none, one award, or a refusal.
pub fn take_award(b: &[u8], pos: &mut usize) -> Option<Option<super::escrow::Escrowed>> {
    let tag = *b.get(*pos)?;
    *pos += 1;
    if tag == 0 {
        return Some(None);
    }
    if tag != 1 {
        return None;
    }
    let nv = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
    *pos += 4;
    if nv > b.len().checked_sub(*pos)? / 32 {
        return None;
    }
    let mut voices = Vec::with_capacity(nv);
    for _ in 0..nv {
        voices.push(b.get(*pos..*pos + 32)?.try_into().ok()?);
        *pos += 32;
    }
    let mut mu = [0u64; 4];
    for m in &mut mu {
        *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
        *pos += 8;
    }
    let at = crate::chronometry::Stamp::take_canonical(b, pos)?;
    let r: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
    *pos += 32;
    Some(Some(super::escrow::Escrowed { voices, mu, anchor: (at, r) }))
}

impl Treasury {
    /// LD1 — THE BYTES EACH ESCROWED AWARD CARRIES as a field of its word's
    /// row: the award alone, flat in the size of the word it is on.
    pub fn carriage(&self, c: &mut crate::observe::Carriage) {
        crate::snapshot::persist(&self.escrowed, |_, row| {
            let mut field = Vec::new();
            put_award(&mut field, Some(row));
            c.add(crate::observe::CARRIER_AWARD, 0, field.len() as u64);
        });
    }

    /// Every award this treasury holds and the address of the word it is
    /// on, for the rows that carry them to name those no row holds.
    pub fn award_addresses(&self) -> Vec<Vec<u8>> {
        self.escrowed.scoped().map(|(a, _)| a).collect()
    }

    /// An award read back off its word's row at restore.
    pub fn restore_award(&mut self, addr: &crate::transducer::address::Address, award: super::escrow::Escrowed) {
        self.escrowed.insert_shared(addr.shared(), award);
    }

    /// Serialize the dimensional ledger: `[n][tag u8][who 32][μ⃗ 4×u64][k u64]
    /// [receipt 32]…` — vector amounts and chronometric anchors survive restart.
    pub fn snapshot(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for p in &self.bond_total {
            out.extend_from_slice(&p.to_le_bytes());
        }
        out.extend_from_slice(&(self.stakes.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.stakes, |who, mu| {
            out.extend_from_slice(who);
            out.push(0); // former tier discriminant; unused
            for m in mu {
                out.extend_from_slice(&m.to_le_bytes());
            }
        });
        out.extend_from_slice(&(self.entries.len() as u32).to_le_bytes());
        for e in &self.entries {
            e.write(&mut out);
        }
        // THE ESCROWED AWARDS ARE NOT HERE (78.22, LD0 step 4): each is a
        // field of its word's row, written where the word is (the kernel's
        // snapshot), so no address is written twice.
        // BALANCES, written out. They were "recomputed from the entries" at
        // load, and `recompute_balances` applied two of §2's ten kinds --
        // `mint` and paired `transfer` -- ignoring genesis, burn, stake,
        // unstake, escrow, release, revoke and slash. Every restored ledger
        // that had used one of those failed its own audit, which §6
        // requires before every snapshot: the mesh could not persist.
        //
        // A better walk was not the fix. A `burn` writes ONE row for what
        // it took from the bond and the balance together, so the balance
        // half cannot be recovered from the journal at all. And recomputing
        // balances FROM the journal makes them agree with it by
        // construction, which destroys the cross-check the audit exists for
        // -- §6 compares `genesis + mint − burn` against the carriers, and
        // a carrier derived from the first can never disagree with it. The
        // carrier is persisted and the supply re-derived, so the two are
        // independent and their disagreement is what the audit catches.
        out.extend_from_slice(&(self.balances.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.balances, |who, mu| {
            out.extend_from_slice(who);
            for m in mu {
                out.extend_from_slice(&m.to_le_bytes());
            }
        });
        out
    }

    /// Reload a ledger written by [`Treasury::snapshot`]. `None` on corruption.
    pub fn restore(b: &[u8], pos: &mut usize) -> Option<Treasury> {
        let mut bond_total = [0u64; 4];
        for p in &mut bond_total {
            *p = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
        let ns = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
        *pos += 4;
        if ns > b.len().checked_sub(*pos)? / 65 {
            return None; // untrusted count: 65 bytes per stake
        }
        let mut stakes = crate::network::nerve::Keyed::new();
        for _ in 0..ns {
            let who: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
            *pos += 32;
            *pos += 1; // former tier discriminant
            let mut mu = [0u64; 4];
            for m in &mut mu {
                *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
            }
            stakes.insert(who, mu);
        }
        let n = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
        *pos += 4;
        if n > b.len().checked_sub(*pos)? / Entry::TAPE {
            return None; // untrusted count: one tape per entry, refuse before allocating
        }
        let mut entries = Vec::with_capacity(n);
        for _ in 0..n {
            entries.push(Entry::read(b, pos)?);
        }
        // THE SECTION BELOW IS REQUIRED: a tape without it is a TRUNCATED
        // ledger, and §10 makes a layout change a recut. The awards are not
        // read here: each comes back off its word's row (`restore_award`).
        let escrowed = crate::network::nerve::Keyed::new();
        // BALANCES, read back. Not recomputed: see `snapshot`.
        let mut balances = crate::network::nerve::Keyed::new();
        {
            let nb = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
            *pos += 4;
            if nb > b.len().checked_sub(*pos)? / 64 {
                return None; // untrusted count: 64 bytes per balance row
            }
            for _ in 0..nb {
                let who: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                *pos += 32;
                let mut mu = [0u64; 4];
                for m in &mut mu {
                    *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                }
                balances.insert(who, mu);
            }
        }
        let mut t = Treasury {
            entries,
            bond_total,
            stakes,
            escrowed,
            balances,
            // Folded from the rows, since they arrived off disk and not
            // through `write`. This is the one O(journal) pass.
            issued: None,
            burned: None,
        };
        t.refresh_bond();
        t.refold_supply();
        Some(t)
    }

}
