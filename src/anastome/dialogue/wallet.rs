// 78.93 B1 — ACT_WALLET: every party holds a wallet (the operator, 2026-10-04:
// "everyone needs accounts/wallets") — entities, kernels and humans alike,
// each by its 32-byte key.
//
//! A wallet is FLAT by design: per court, a party's balance, bond and holds,
//! each a per-lane vector `S:G:T:H`, never summed across lanes. It is a
//! projection of the record, never a second record: every row it folds stands
//! at an act on that court's line (`economy::Entry::at`), and the rows are
//! served to peers and re-derived at every fold (`ACT_ROWS`, `ACT_FOLD`). So
//! the reading carries where it stands — the court's head stamp when it was
//! read — and a party's wallet across the universe is its account at every
//! court, per lane, summed exactly or not at all.

use super::*;
use crate::chronometry::Stamp;

/// ONE PARTY'S ACCOUNT AT ONE COURT, read at that court's head.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Account {
    pub court: [u8; 32],
    pub balance: [u64; 4],
    pub bond: [u64; 4],
    pub held: [u64; 4],
    /// The court's head when it was read: where the reading stands. `None`
    /// before the court's first act.
    pub at: Option<Stamp>,
}

impl Account {
    /// Everything the party owns at this court, per lane: balance, bond and
    /// holds. `None` where a lane would pass its carrier (rule 11).
    pub fn owned(&self) -> Option<[u64; 4]> {
        crate::core::lanes::lanes_add(crate::core::lanes::lanes_add(self.balance, self.bond)?, self.held)
    }
}

/// A PARTY'S WALLET ACROSS THE COURTS ASKED: each court's account, and the
/// courts that did not answer — counted, never read as an empty account.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Wallet {
    pub who: [u8; 32],
    pub accounts: Vec<Account>,
    pub unanswered: u64,
}

impl Wallet {
    /// What the party owns across the courts that answered, per lane.
    /// `None` where a lane would pass its carrier.
    pub fn owned(&self) -> Option<[u64; 4]> {
        self.accounts.iter().try_fold([0u64; 4], |t, a| crate::core::lanes::lanes_add(t, a.owned()?))
    }
}

pub fn encode_wallet(who: &[u8; 32]) -> Vec<u8> {
    let mut out = vec![ACT_WALLET];
    out.extend_from_slice(who);
    out
}

/// The party asked about, read whole or not at all.
pub fn decode_wallet(body: &[u8]) -> Option<[u8; 32]> {
    if !act_is(body, ACT_WALLET) || body.len() != 33 {
        return None;
    }
    body.get(1..33)?.try_into().ok()
}

fn put_lanes(out: &mut Vec<u8>, v: &[u64; 4]) {
    for x in v {
        out.extend_from_slice(&x.to_le_bytes());
    }
}

fn take_lanes(b: &[u8], p: &mut usize) -> Option<[u64; 4]> {
    let mut v = [0u64; 4];
    for x in v.iter_mut() {
        *x = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        *p += 8;
    }
    Some(v)
}

pub fn encode_wallet_reply(a: &Account) -> Vec<u8> {
    let mut out = vec![ACT_WALLET];
    out.extend_from_slice(&a.court);
    for v in [&a.balance, &a.bond, &a.held] {
        put_lanes(&mut out, v);
    }
    match &a.at {
        None => out.push(0),
        Some(s) => {
            out.push(1);
            s.put_canonical(&mut out);
        }
    }
    out
}

/// The account, read whole or not at all.
pub fn decode_wallet_reply(body: &[u8]) -> Option<Account> {
    if !act_is(body, ACT_WALLET) {
        return None;
    }
    let court: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut p = 33usize;
    let (balance, bond, held) = (take_lanes(body, &mut p)?, take_lanes(body, &mut p)?, take_lanes(body, &mut p)?);
    let flag = *body.get(p)?;
    p += 1;
    let at = match flag {
        0 => None,
        1 => Some(Stamp::take_canonical(body, &mut p)?),
        _ => return None,
    };
    (p == body.len()).then_some(Account { court, balance, bond, held, at })
}
