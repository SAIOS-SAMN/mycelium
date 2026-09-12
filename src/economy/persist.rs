// 28 — THE TREASURY ON DISK: every carrier written whole, and the supply
// folded back from the rows at load.
//
//! A carrier is persisted and the supply is re-derived, so a tape whose
//! carriers disagree with its journal is caught by the audit (§6) rather
//! than carried forward.

use super::*;

impl Entry {
    /// One row's tape: `[tag u8][who 32][μ⃗ 4×u64][k u64][receipt 32]`,
    /// 105 bytes — the one layout the ledger, the snapshot and the wire
    /// (`ACT_ROWS`) share.
    pub const TAPE: usize = 1 + 32 + 32 + 8 + 32;

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
        out.extend_from_slice(&self.k_step.to_le_bytes());
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
        let k_step = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
        *pos += 8;
        let receipt: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        Some(Entry { kind, who, mu, k_step, receipt })
    }
}

impl Treasury {
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
        // ESCROWED AWARDS section (Step 5).
        out.extend_from_slice(&(self.escrowed.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.escrowed, |addr, row| {
            let (voices, mu, anchor) = (&row.voices, &row.mu, &row.anchor);
            let _ = crate::transducer::address::put_addr(&mut out, addr);
            out.extend_from_slice(&(voices.len() as u32).to_le_bytes());
            for v in voices {
                out.extend_from_slice(v);
            }
            for m in mu {
                out.extend_from_slice(&m.to_le_bytes());
            }
            out.extend_from_slice(&anchor.0.to_le_bytes());
            out.extend_from_slice(&anchor.1);
        });
        // FUNDED ESCROW, written out. It stood as "rebuilt from the journal
        // by `recompute_balances`, not persisted twice" -- but an entry
        // carries a party and a lane vector and NOT the listing it is
        // against, so no rebuild can key it, and `recompute_balances` never
        // touched it. A restored treasury therefore held zero funded escrow
        // against balances that had already paid for it, and §6's second
        // equation failed by exactly the funded total at every restart.
        out.extend_from_slice(&(self.funded.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.funded, |addr, (payer, mu)| {
            let _ = crate::transducer::address::put_addr(&mut out, addr);
            out.extend_from_slice(payer);
            for m in mu {
                out.extend_from_slice(&m.to_le_bytes());
            }
        });
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
        if n > b.len().checked_sub(*pos)? / 105 {
            return None; // untrusted count: 105 bytes per entry, refuse before allocating
        }
        let mut entries = Vec::with_capacity(n);
        for _ in 0..n {
            entries.push(Entry::read(b, pos)?);
        }
        // THE THREE SECTIONS BELOW ARE REQUIRED. Each read as "a snapshot
        // that ends here is an older one and restores with none", and
        // `snapshot` writes all three unconditionally, so no writer emits
        // a tape without them: what that admitted was a TRUNCATED ledger,
        // restored with no escrow, no funded escrow and no balances, which
        // then failed its own audit or passed one it should not have.
        // §10 makes a layout change a recut and deletes the reader for the
        // old layout.
        let mut escrowed = crate::network::nerve::Keyed::new();
        {
            let ne = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
            *pos += 4;
            if ne > b.len().checked_sub(*pos)? / 108 {
                return None; // untrusted count: ≥108 bytes per award
            }
            for _ in 0..ne {
                let addr = crate::transducer::address::take_addr(b, pos)?;
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
                let k = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
                let r: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                *pos += 32;
                escrowed.insert(addr, crate::economy::escrow::Escrowed { voices, mu, anchor: (k, r) });
            }
        }
        // FUNDED ESCROW, read back.
        let mut funded = crate::network::nerve::Keyed::new();
        {
            let nf = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
            *pos += 4;
            if nf > b.len().checked_sub(*pos)? / 66 {
                return None; // untrusted count: ≥66 bytes per funded row
            }
            for _ in 0..nf {
                let addr = crate::transducer::address::take_addr(b, pos)?;
                let payer: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                *pos += 32;
                let mut mu = [0u64; 4];
                for m in &mut mu {
                    *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                }
                funded.insert(addr, (payer, mu));
            }
        }
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
            funded,
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
