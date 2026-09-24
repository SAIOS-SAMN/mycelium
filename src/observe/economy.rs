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
}

impl Economy {
    pub(crate) fn put(&self, out: &mut Vec<u8>) {
        for arm in self.paid.iter().chain(&self.burned) {
            for lane in arm {
                out.extend_from_slice(&lane.to_le_bytes());
            }
        }
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Economy> {
        let mut e = Economy::default();
        for arm in e.paid.iter_mut().chain(e.burned.iter_mut()) {
            for lane in arm.iter_mut() {
                *lane = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
                *p += 8;
            }
        }
        Some(e)
    }
}
