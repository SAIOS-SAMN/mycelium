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

/// 78.154 (4) — STAKE OR UNSTAKE: μ moved between a party's balance and its
/// STAKE — non-decaying escrow for traversal and coordinate retrieval, the
/// treasury's own Stake and Unstake rows. An unstake is admitted only once
/// every claim drawn on the stake has settled.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StakeAct {
    pub who: [u8; 32],
    pub unstake: bool,
    pub mu: [u64; 4],
    pub nonce: [u8; 32],
}

const STAKE: &[u8] = b"mycelium stake v1";
const KIND_STAKE: u8 = 1;
const KIND_UNSTAKE: u8 = 2;

impl StakeAct {
    fn kind(&self) -> u8 {
        match self.unstake {
            false => KIND_STAKE,
            true => KIND_UNSTAKE,
        }
    }

    /// `"mycelium stake v1" ‖ kind ‖ who ‖ mu ‖ nonce`.
    pub fn signed_bytes(&self) -> Vec<u8> {
        let mut out = STAKE.to_vec();
        out.push(self.kind());
        out.extend_from_slice(&self.who);
        put_lanes(&mut out, &self.mu);
        out.extend_from_slice(&self.nonce);
        out
    }

    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> SignedStake {
        use ed25519_dalek::Signer;
        let sig = key.sign(&self.signed_bytes()).to_bytes();
        SignedStake { act: self, sig }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedStake {
    pub act: StakeAct,
    pub sig: [u8; 64],
}

/// `[ACT_WALLET][who 32][1 stake | 2 unstake][mu 4×u64][nonce 32][sig 64]`.
pub fn encode_stake(s: &SignedStake) -> Vec<u8> {
    let mut out = encode_wallet(&s.act.who);
    out.push(s.act.kind());
    put_lanes(&mut out, &s.act.mu);
    out.extend_from_slice(&s.act.nonce);
    out.extend_from_slice(&s.sig);
    out
}

pub fn decode_stake(body: &[u8]) -> Option<SignedStake> {
    if !act_is(body, ACT_WALLET) {
        return None;
    }
    let who: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let unstake = match *body.get(33)? {
        KIND_STAKE => false,
        KIND_UNSTAKE => true,
        _ => return None,
    };
    let mut p = 34usize;
    let mu = take_lanes(body, &mut p)?;
    let nonce: [u8; 32] = body.get(p..p + 32)?.try_into().ok()?;
    let sig: [u8; 64] = body.get(p + 32..p + 96)?.try_into().ok()?;
    (p + 96 == body.len()).then_some(SignedStake { act: StakeAct { who, unstake, mu, nonce }, sig })
}

/// WHY A STAKE ACT DID NOT MOVE.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StakeRefusal {
    /// 1 — the party's signature does not verify.
    Unsigned,
    /// 2 — the balance (a stake) or the stake (an unstake) lacks it, on a lane.
    Short,
    /// 3 — a claim drawn on the stake has not settled.
    Claimed,
    /// 4 — zero on every lane.
    Nothing,
    /// 5 — the nonce was used by this party already: a resend moves nothing.
    Spent,
}

impl StakeRefusal {
    fn code(self) -> u8 {
        match self {
            StakeRefusal::Unsigned => 1,
            StakeRefusal::Short => 2,
            StakeRefusal::Claimed => 3,
            StakeRefusal::Nothing => 4,
            StakeRefusal::Spent => 5,
        }
    }
}

/// `[1][stake now 4×u64]` or `[0][reason]`.
pub fn encode_stake_reply(r: &Result<[u64; 4], StakeRefusal>) -> Vec<u8> {
    match r {
        Ok(stake) => {
            let mut out = vec![1];
            put_lanes(&mut out, stake);
            out
        }
        Err(why) => vec![0, why.code()],
    }
}

pub fn decode_stake_reply(b: &[u8]) -> Option<Result<[u64; 4], StakeRefusal>> {
    match b {
        [0, code] => {
            let all = [StakeRefusal::Unsigned, StakeRefusal::Short, StakeRefusal::Claimed, StakeRefusal::Nothing, StakeRefusal::Spent];
            Some(Err(all.into_iter().find(|r| r.code() == *code)?))
        }
        [1, rest @ ..] => {
            let mut p = 0;
            let stake = take_lanes(rest, &mut p)?;
            (p == rest.len()).then_some(Ok(stake))
        }
        _ => None,
    }
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
