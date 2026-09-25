// 28 — THE TREASURY ON DISK: every carrier written whole, and the supply
// folded back from the rows at load.
//
//! A carrier is persisted and the supply is re-derived, so a tape whose
//! carriers disagree with its journal is caught by the audit (§6) rather
//! than carried forward.

use super::*;

impl Entry {
    /// One row's tape: `[tag u8][arm u8][who 32][μ⃗ 4×u64][stamp 56][receipt
    /// 32]`, 154 bytes — the one layout the ledger, the snapshot and the
    /// wire (`ACT_ROWS`) share. The stamp is the act the row stands at
    /// (78.24); the arm the proof of work it pays (78.33).
    pub const TAPE: usize = 1 + 1 + 32 + 32 + crate::chronometry::Stamp::LEN + 32;

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
        out.push(Arm::to_byte(self.arm));
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
        let arm = Arm::from_byte(*b.get(*pos)?)?;
        *pos += 1;
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
        Some(Entry { kind, who, mu, at, receipt, arm })
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

    /// THE TREASURY'S OWN TRIES (78.27, LD3): the stakes and the balances,
    /// each a trie keyed by the identity's `vk` with its μ⃗ as the one leaf.
    /// Its journal rows are not here — each is a leaf of the ledger node it
    /// stands at (T2, the kernel's snapshot) — and its escrowed awards are
    /// not here — each is a leaf of its word's node (T1). The bond total is
    /// re-derived from the stakes at restore.
    ///
    /// BALANCES, written out. They were "recomputed from the entries" at
    /// load, and `recompute_balances` applied two of §2's ten kinds — `mint`
    /// and paired `transfer` — ignoring the rest, so every restored ledger
    /// that had used one of those failed its own audit. And a carrier
    /// derived from the journal agrees with it by construction, which
    /// destroys the cross-check §6's audit exists for: the carrier is
    /// persisted and the supply re-derived, so their disagreement is what
    /// the audit catches.
    pub fn put_tries(&self, out: &mut Vec<u8>) {
        for (name, carrier) in [(STAKES_TRIE, &self.stakes), (BALANCES_TRIE, &self.balances)] {
            let mut nodes: crate::network::nerve::Keyed<crate::snapshot::Leaves> = crate::network::nerve::Keyed::new();
            crate::snapshot::persist(carrier, |who, mu| {
                let mut v = Vec::with_capacity(32);
                for m in mu {
                    v.extend_from_slice(&m.to_le_bytes());
                }
                nodes.insert(who, vec![(LEAF_MU, v)]);
            });
            crate::snapshot::put_trie(out, name, &nodes);
        }
    }

    /// The inverse of [`Treasury::put_tries`]: the stakes and the balances,
    /// each keyed by a `vk` of 32 bytes carrying one μ⃗, or refused whole.
    pub fn take_tries(b: &[u8], pos: &mut usize) -> Option<(ByVk, ByVk)> {
        let mut read = |name: &str| -> Option<crate::network::nerve::Keyed<[u64; 4]>> {
            let nodes = crate::snapshot::take_trie(b, pos, name, &[LEAF_MU])?;
            let mut out = crate::network::nerve::Keyed::new();
            for (who, leaves) in nodes.scoped() {
                let who: [u8; 32] = who.as_slice().try_into().ok()?;
                let v = crate::snapshot::leaf(leaves, LEAF_MU)?;
                let mut mu = [0u64; 4];
                for (l, m) in mu.iter_mut().enumerate() {
                    *m = u64::from_le_bytes(v.get(l * 8..l * 8 + 8)?.try_into().ok()?);
                }
                if v.len() != 32 {
                    return None;
                }
                out.insert(who, mu);
            }
            Some(out)
        };
        let stakes = read(STAKES_TRIE)?;
        let balances = read(BALANCES_TRIE)?;
        Some((stakes, balances))
    }

    /// A TREASURY REBUILT FROM ITS PARTS at restore: the stakes and
    /// balances off their tries, the journal off the ledger's nodes in the
    /// order they stand. The bond total and the supply are re-derived; the
    /// awards come back off their words' nodes (`restore_award`).
    pub fn from_parts(
        stakes: crate::network::nerve::Keyed<[u64; 4]>,
        balances: crate::network::nerve::Keyed<[u64; 4]>,
        entries: Vec<Entry>,
    ) -> Treasury {
        let mut t = Treasury {
            entries,
            bond_total: Some([0; 4]),
            stakes,
            escrowed: crate::network::nerve::Keyed::new(),
            balances,
            overflowed: 0,
            // Folded from the rows, since they arrived off disk and not
            // through `write`. This is the one O(journal) pass.
            issued: None,
            burned: None,
        };
        t.refresh_bond();
        t.refold_supply();
        t
    }
}

/// A carrier keyed by an identity's `vk`, one μ⃗ per identity: the stakes,
/// the balances.
pub type ByVk = crate::network::nerve::Keyed<[u64; 4]>;

/// The stakes trie's name.
pub const STAKES_TRIE: &str = "stakes";
/// The balances trie's name.
pub const BALANCES_TRIE: &str = "balances";
/// The one leaf of a stake or balance node: its μ⃗, four lanes.
pub const LEAF_MU: u8 = 1;
