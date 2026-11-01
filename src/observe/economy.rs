// 78.33, EC1 — THE ECONOMY BY PROOF OF WORK: what a court's ledger paid and
// burned per arm, per lane, read off its journal at the question.
//
//! POUW, POWC, POWR and POWV are named apart and never summed across: a row that
//! pays one carries it (`Entry.arm`), and the reading keeps each arm on its
//! own four lanes. A row that is no proof of work — genesis, a stake, an
//! escrow, a revoke or slash, a fee, a subsidy — is in none of them. Every
//! amount is an exact [`Mu`], written by its rational codec.

use crate::economy::Mu;

/// ONE COURT'S PAYMENTS BY ARM (`Arm::index`), per lane.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Economy {
    /// What each arm paid out, per lane.
    pub paid: [Mu; crate::economy::Arm::ALL.len()],
    /// What each arm burned, per lane.
    pub burned: [Mu; crate::economy::Arm::ALL.len()],
    /// E3 — every identity the ledger holds value for: what it can spend,
    /// what it has staked, and what it has set aside, in identity order.
    pub accounts: Vec<Account>,
    /// WHAT MOVED, BY KIND (ruled 2026-09-30): the journal's rows
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
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Flow {
    pub rows: u64,
    pub mu: Mu,
}

impl Flow {
    /// One more row of `mu`; `None` where the row count passes its u64 —
    /// unmeasured, never saturated (rule 11).
    pub fn add(&mut self, mu: &Mu) -> Option<()> {
        self.rows = self.rows.checked_add(1)?;
        self.mu.step(mu);
        Some(())
    }

    fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.rows.to_le_bytes());
        self.mu.put(out);
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Flow> {
        let rows = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        *p += 8;
        Some(Flow { rows, mu: Mu::take_from(b, p)? })
    }
}

/// ONE IDENTITY'S STANDING ON A COURT'S LEDGER, per lane.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Account {
    pub who: [u8; 32],
    pub balance: Mu,
    pub bond: Mu,
    pub held: Mu,
}

/// The fewest bytes an account takes: its key and three amounts of eleven
/// bytes a lane at least (`Mu::put` of zero).
/// FORMAT: an account's fewest bytes, 164.
const ACCOUNT_MIN: usize = 32 + 3 * 44;

impl Economy {
    /// `None` where the accounts' count passes its four bytes (STD1 3c).
    pub(crate) fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        for arm in self.paid.iter().chain(&self.burned) {
            arm.put(out);
        }
        crate::transducer::address::put_u32_count(out, self.accounts.len())?;
        for a in &self.accounts {
            out.extend_from_slice(&a.who);
            a.balance.put(out);
            a.bond.put(out);
            a.held.put(out);
        }
        for f in self.flows.iter().chain([&self.tuition]) {
            f.put(out);
        }
        Some(())
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Economy> {
        let mut e = Economy::default();
        for arm in e.paid.iter_mut().chain(e.burned.iter_mut()) {
            *arm = Mu::take_from(b, p)?;
        }
        let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
        *p += 4;
        // A count the tape cannot hold at the fewest bytes an account takes
        // refuses before anything is allocated for it.
        if n > b.len().checked_sub(*p)? / ACCOUNT_MIN {
            return None;
        }
        for _ in 0..n {
            let who: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            let (balance, bond, held) = (Mu::take_from(b, p)?, Mu::take_from(b, p)?, Mu::take_from(b, p)?);
            e.accounts.push(Account { who, balance, bond, held });
        }
        for f in e.flows.iter_mut() {
            *f = Flow::take(b, p)?;
        }
        e.tuition = Flow::take(b, p)?;
        Some(e)
    }
}
