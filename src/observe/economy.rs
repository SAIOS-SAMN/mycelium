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
        Some(e)
    }
}
