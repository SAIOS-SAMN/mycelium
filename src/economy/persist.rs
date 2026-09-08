// 28 — THE TREASURY ON DISK: the entries written whole, and the balances
// re-derived from them at load.
//
//! Balances are never restored from the tape: they are recomputed from the
//! entries, so a tape that disagrees with itself is caught by the audit
//! rather than carried forward.

use super::*;

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
            let tag: u8 = match e.kind {
                "mint" => 14,
                "stake" => 4,
                "slash" => 8,
                "transfer" => 15,
                "genesis" => 16,
                "burn" => 17,
                "escrow" => 18,
                "release" => 19,
                "revoke" => 20,
                "unstake" => 21,
                "utility" | "consilience" | "torse" | "assembly" | "solve" | "refine" | "condense" | "rewrite" => 14,
                "spend" | "fee" => 15,
                _ => 255,
            };
            out.push(tag);
            out.extend_from_slice(&e.who);
            for m in &e.mu {
                out.extend_from_slice(&m.to_le_bytes());
            }
            out.extend_from_slice(&e.k_step.to_le_bytes());
            out.extend_from_slice(&e.receipt);
        }
        // ESCROWED AWARDS section (Step 5) — appended after entries so older
        // snapshots (which simply end here) restore with an empty docket.
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
        if ns > b.len().saturating_sub(*pos) / 65 {
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
        if n > b.len().saturating_sub(*pos) / 105 {
            return None; // untrusted count: 105 bytes per entry, refuse before allocating
        }
        let mut entries = Vec::with_capacity(n);
        for _ in 0..n {
            let tag = *b.get(*pos)?;
            *pos += 1;
            let kind = match tag {
                4 => "stake",
                8 => "slash",
                14 => "mint",
                15 => "transfer",
                16 => "genesis",
                17 => "burn",
                18 => "escrow",
                19 => "release",
                20 => "revoke",
                21 => "unstake",
                0 | 1 | 3 | 9 | 10 | 11 | 12 | 13 => "mint",
                5 | 6 => "transfer",
                _ => "unknown",
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
            entries.push(Entry { kind, who, mu, k_step, receipt });
        }
        let mut escrowed = crate::network::nerve::Keyed::new();
        if let Some(c4) = b.get(*pos..*pos + 4) {
            let ne = u32::from_le_bytes(c4.try_into().ok()?) as usize;
            *pos += 4;
            if ne > b.len().saturating_sub(*pos) / 108 {
                return None; // untrusted count: ≥108 bytes per award
            }
            for _ in 0..ne {
                let addr = crate::transducer::address::take_addr(b, pos)?;
                let nv = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
                *pos += 4;
                if nv > b.len().saturating_sub(*pos) / 32 {
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
        // `funded` is rebuilt from the journal by `recompute_balances`, not
        // persisted twice: the escrow and release rows say what was funded.
        let mut t = Treasury {
            entries,
            bond_total,
            stakes,
            escrowed,
            balances: crate::network::nerve::Keyed::new(),
            funded: crate::network::nerve::Keyed::new(),
        };
        t.refresh_bond();
        t.recompute_balances();
        Some(t)
    }

    pub(crate) fn recompute_balances(&mut self) {
        self.balances = crate::network::nerve::Keyed::new();
        let mut i = 0;
        while i < self.entries.len() {
            let who = self.entries[i].who;
            let mu = self.entries[i].mu;
            match self.entries[i].kind {
                "mint" => self.credit(who, mu),
                "transfer" => {
                    self.debit(who, mu);
                    if i + 1 < self.entries.len() {
                        let nxt = &self.entries[i + 1];
                        if nxt.kind == "transfer"
                            && nxt.mu == mu
                            && nxt.k_step == self.entries[i].k_step
                            && nxt.receipt == self.entries[i].receipt
                            && nxt.who != who
                        {
                            self.credit(nxt.who, nxt.mu);
                            i += 2;
                            continue;
                        }
                    }
                }
                _ => {}
            }
            i += 1;
        }
    }

}
