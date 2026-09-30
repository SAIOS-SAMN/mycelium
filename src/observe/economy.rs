// 78.33, EC1 — THE ECONOMY BY PROOF OF WORK: what a court's ledger paid and
// burned per arm, per lane, read off its journal at the question.
//
//! POUW, POWC and POWR are named apart and never summed across: a row that
//! pays one carries it (`Entry.arm`), and the reading keeps each arm on its
//! own four lanes. A row that is no proof of work — genesis, a stake, an
//! escrow, a revoke or slash, a fee, a subsidy — is in none of them.

/// ONE COURT'S PAYMENTS BY ARM (`Arm::index`), per lane.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Economy {
    /// What each arm paid out, per lane.
    pub paid: [[u64; 4]; 3],
    /// What each arm burned, per lane.
    pub burned: [[u64; 4]; 3],
    /// STD1 — the treasury's operations refused past their carrier.
    pub overflowed: u64,
    /// E3 — every identity the ledger holds value for: what it can spend,
    /// what it has staked, and what it has set aside, in identity order.
    pub accounts: Vec<Account>,
    /// WHAT MOVED, BY KIND (the operator, 2026-09-30): the journal's rows
    /// since its last fold, one [`Flow`] per name on [`FLOWS`]. A transfer
    /// is one flow per pair of rows, its vector once.
    pub flows: [Flow; FLOWS.len()],
    /// TUITION: the payments this court was paid, a learner's balance to
    /// the court at the Syllabus act that taught it a sentence — of the
    /// flows' payments, those credited to this court.
    pub tuition: Flow,
}

/// The kinds a ledger row is read as on [`Economy::flows`]: a mint or a
/// transfer that pays a proof of work apart from one that pays none (a
/// subsidy, a payment), every other kind as the treasury names it.
pub const FLOWS: [&str; 13] = [
    "genesis",
    "mint (proof of work)",
    "subsidy",
    "stake",
    "unstake",
    "transfer (proof of work)",
    "payment",
    "escrow",
    "release",
    "revoke",
    "slash",
    "burn",
    "epoch",
];

/// ONE KIND OF MOVEMENT: how many rows (pairs, for a transfer) and what
/// they carried, per lane.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Flow {
    pub rows: u64,
    pub mu: [u64; 4],
}

impl Flow {
    /// One more row of `mu`; `None` where a count or a lane passes its u64
    /// — unmeasured, never saturated (rule 11).
    pub fn add(&mut self, mu: &[u64; 4]) -> Option<()> {
        self.rows = self.rows.checked_add(1)?;
        for (t, m) in self.mu.iter_mut().zip(mu) {
            *t = t.checked_add(*m)?;
        }
        Some(())
    }

    fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.rows.to_le_bytes());
        put_lanes(out, &self.mu);
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Flow> {
        let rows = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        *p += 8;
        Some(Flow { rows, mu: take_lanes(b, p)? })
    }
}

/// ONE IDENTITY'S STANDING ON A COURT'S LEDGER, per lane.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Account {
    pub who: [u8; 32],
    pub balance: [u64; 4],
    pub bond: [u64; 4],
    pub held: [u64; 4],
}

fn put_lanes(out: &mut Vec<u8>, v: &[u64; 4]) {
    for lane in v {
        out.extend_from_slice(&lane.to_le_bytes());
    }
}

fn take_lanes(b: &[u8], p: &mut usize) -> Option<[u64; 4]> {
    let mut v = [0u64; 4];
    for lane in v.iter_mut() {
        *lane = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        *p += 8;
    }
    Some(v)
}

impl Economy {
    /// `None` where the accounts' count passes its four bytes (STD1 3c).
    pub(crate) fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        for arm in self.paid.iter().chain(&self.burned) {
            put_lanes(out, arm);
        }
        out.extend_from_slice(&self.overflowed.to_le_bytes());
        crate::transducer::address::put_u32_count(out, self.accounts.len())?;
        for a in &self.accounts {
            out.extend_from_slice(&a.who);
            put_lanes(out, &a.balance);
            put_lanes(out, &a.bond);
            put_lanes(out, &a.held);
        }
        for f in self.flows.iter().chain([&self.tuition]) {
            f.put(out);
        }
        Some(())
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Economy> {
        let mut e = Economy::default();
        for arm in e.paid.iter_mut().chain(e.burned.iter_mut()) {
            *arm = take_lanes(b, p)?;
        }
        e.overflowed = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        *p += 8;
        let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
        *p += 4;
        // Each account is 32 + 3 × 32 bytes: a count the tape cannot hold
        // refuses before anything is allocated for it.
        if n > b.len().checked_sub(*p)? / 128 {
            return None;
        }
        for _ in 0..n {
            let who: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            let (balance, bond, held) = (take_lanes(b, p)?, take_lanes(b, p)?, take_lanes(b, p)?);
            e.accounts.push(Account { who, balance, bond, held });
        }
        for f in e.flows.iter_mut() {
            *f = Flow::take(b, p)?;
        }
        e.tuition = Flow::take(b, p)?;
        Some(e)
    }
}
