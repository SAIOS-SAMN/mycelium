// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-ECONOMY §5.2 — THE COMPUTE LEASE on the wire: sub-kinds 3 (open) and 4
// (close) of ACT_WALLET, the economy's act (the wire's bytes below the
// court's range are spent; a new act is a sub-kind under an existing one).
//
//! A requester (the PAYER) holds μ against a lease for one PROVIDER over a
//! list of jobs — task citations — for a window of the court's heights. Each
//! job the provider's solve verifies pays the provider that solve's measured
//! discharge out of the hold, capped by what the hold has left; at the
//! window's end, or on the payer's close, the rest returns to the payer.
//! Compute is paid only as its results verify: a lease is escrow against
//! capacity, released by proof.

use super::*;

/// The domain a lease act is signed under.
const LEASE: &[u8] = b"mycelium lease v1";
/// `ACT_WALLET`'s sub-kind for opening a lease.
pub const KIND_OPEN: u8 = 3;
/// `ACT_WALLET`'s sub-kind for closing one.
pub const KIND_CLOSE: u8 = 4;

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

fn take32(b: &[u8], p: &mut usize) -> Option<[u8; 32]> {
    let v: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
    *p += 32;
    Some(v)
}

/// A LEASE OPENED: `payer` holds `mu` for `provider` over `jobs` (task
/// citations, each paid once) for `window` of the court's heights.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeaseAct {
    pub payer: [u8; 32],
    pub provider: [u8; 32],
    pub mu: [u64; 4],
    pub window: u64,
    pub jobs: Vec<[u8; 32]>,
    pub nonce: [u8; 32],
}

impl LeaseAct {
    /// `"mycelium lease v1" ‖ 3 ‖ payer ‖ provider ‖ mu ‖ window ‖ n ‖ jobs ‖
    /// nonce`. `None` where the job count passes its field.
    pub fn signed_bytes(&self) -> Option<Vec<u8>> {
        let mut out = LEASE.to_vec();
        out.push(KIND_OPEN);
        out.extend_from_slice(&self.payer);
        out.extend_from_slice(&self.provider);
        put_lanes(&mut out, &self.mu);
        out.extend_from_slice(&self.window.to_le_bytes());
        crate::transducer::address::put_u8_count(&mut out, self.jobs.len())?;
        for j in &self.jobs {
            out.extend_from_slice(j);
        }
        out.extend_from_slice(&self.nonce);
        Some(out)
    }

    /// THE LEASE'S NAME: the payload hash of its signed bytes. `None` where
    /// they do not write.
    pub fn citation(&self) -> Option<[u8; 32]> {
        Some(crate::chronometry::payload_hash(&self.signed_bytes()?))
    }

    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> Option<SignedLease> {
        use ed25519_dalek::Signer;
        let sig = key.sign(&self.signed_bytes()?).to_bytes();
        Some(SignedLease { act: self, sig })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedLease {
    pub act: LeaseAct,
    pub sig: [u8; 64],
}

/// `[ACT_WALLET][payer 32][3][provider 32][mu 4×u64][window u64][n u8][job 32·n][nonce 32][sig 64]`.
pub fn encode_lease(s: &SignedLease) -> Option<Vec<u8>> {
    let a = &s.act;
    let mut out = vec![ACT_WALLET];
    out.extend_from_slice(&a.payer);
    out.push(KIND_OPEN);
    out.extend_from_slice(&a.provider);
    put_lanes(&mut out, &a.mu);
    out.extend_from_slice(&a.window.to_le_bytes());
    crate::transducer::address::put_u8_count(&mut out, a.jobs.len())?;
    for j in &a.jobs {
        out.extend_from_slice(j);
    }
    out.extend_from_slice(&a.nonce);
    out.extend_from_slice(&s.sig);
    Some(out)
}

/// A lease opened, read whole or not at all.
pub fn decode_lease(body: &[u8]) -> Option<SignedLease> {
    if !act_is(body, ACT_WALLET) || *body.get(33)? != KIND_OPEN {
        return None;
    }
    let payer: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut p = 34usize;
    let provider = take32(body, &mut p)?;
    let mu = take_lanes(body, &mut p)?;
    let window = u64::from_le_bytes(body.get(p..p + 8)?.try_into().ok()?);
    p += 8;
    let n = usize::from(*body.get(p)?);
    p += 1;
    let mut jobs = Vec::with_capacity(n);
    for _ in 0..n {
        jobs.push(take32(body, &mut p)?);
    }
    let nonce = take32(body, &mut p)?;
    let sig: [u8; 64] = body.get(p..p + 64)?.try_into().ok()?;
    (p + 64 == body.len()).then_some(SignedLease { act: LeaseAct { payer, provider, mu, window, jobs, nonce }, sig })
}

/// A LEASE CLOSED BY ITS PAYER: what it holds returns to the payer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CloseLease {
    pub payer: [u8; 32],
    pub lease: [u8; 32],
    pub nonce: [u8; 32],
}

impl CloseLease {
    /// `"mycelium lease v1" ‖ 4 ‖ payer ‖ lease ‖ nonce`.
    pub fn signed_bytes(&self) -> Vec<u8> {
        let mut out = LEASE.to_vec();
        out.push(KIND_CLOSE);
        out.extend_from_slice(&self.payer);
        out.extend_from_slice(&self.lease);
        out.extend_from_slice(&self.nonce);
        out
    }

    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> SignedClose {
        use ed25519_dalek::Signer;
        let sig = key.sign(&self.signed_bytes()).to_bytes();
        SignedClose { act: self, sig }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedClose {
    pub act: CloseLease,
    pub sig: [u8; 64],
}

/// `[ACT_WALLET][payer 32][4][lease 32][nonce 32][sig 64]`.
pub fn encode_close(s: &SignedClose) -> Vec<u8> {
    let mut out = vec![ACT_WALLET];
    out.extend_from_slice(&s.act.payer);
    out.push(KIND_CLOSE);
    out.extend_from_slice(&s.act.lease);
    out.extend_from_slice(&s.act.nonce);
    out.extend_from_slice(&s.sig);
    out
}

/// A close, read whole or not at all.
pub fn decode_close(body: &[u8]) -> Option<SignedClose> {
    if !act_is(body, ACT_WALLET) || *body.get(33)? != KIND_CLOSE {
        return None;
    }
    let payer: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut p = 34usize;
    let lease = take32(body, &mut p)?;
    let nonce = take32(body, &mut p)?;
    let sig: [u8; 64] = body.get(p..p + 64)?.try_into().ok()?;
    (p + 64 == body.len()).then_some(SignedClose { act: CloseLease { payer, lease, nonce }, sig })
}

/// WHY A LEASE ACT DID NOT MOVE.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeaseRefusal {
    /// 1 — the payer's signature does not verify.
    Unsigned,
    /// 2 — the nonce was used by this payer already: a resend moves nothing.
    Spent,
    /// 3 — zero on every lane, no job, no window, or a job named twice.
    Malformed,
    /// 4 — the payer's balance lacks the hold, on a lane.
    Short,
    /// 5 — payer and provider are one identity.
    Oneself,
    /// 6 — a job is already under an open lease to this provider.
    Leased,
    /// 7 — no open lease of this payer by that name.
    UnknownLease,
}

const REFUSALS: [LeaseRefusal; 7] = [
    LeaseRefusal::Unsigned,
    LeaseRefusal::Spent,
    LeaseRefusal::Malformed,
    LeaseRefusal::Short,
    LeaseRefusal::Oneself,
    LeaseRefusal::Leased,
    LeaseRefusal::UnknownLease,
];

impl LeaseRefusal {
    fn byte(self) -> u8 {
        match self {
            LeaseRefusal::Unsigned => 1,
            LeaseRefusal::Spent => 2,
            LeaseRefusal::Malformed => 3,
            LeaseRefusal::Short => 4,
            LeaseRefusal::Oneself => 5,
            LeaseRefusal::Leased => 6,
            LeaseRefusal::UnknownLease => 7,
        }
    }
}

/// WHAT A LEASE ACT CAME TO: the lease and its lanes (opened: what it holds;
/// closed: what returned to the payer), or the refusal.
pub type LeaseReply = Result<([u8; 32], [u64; 4]), LeaseRefusal>;

/// `[1][lease 32][held 4×u64]` or `[0][reason]`.
pub fn encode_lease_reply(r: &LeaseReply) -> Vec<u8> {
    match r {
        Ok((lease, mu)) => {
            let mut out = vec![1];
            out.extend_from_slice(lease);
            put_lanes(&mut out, mu);
            out
        }
        Err(why) => vec![0, why.byte()],
    }
}

pub fn decode_lease_reply(b: &[u8]) -> Option<LeaseReply> {
    match b {
        [0, code] => Some(Err(REFUSALS.into_iter().find(|r| r.byte() == *code)?)),
        [1, rest @ ..] => {
            let mut p = 0;
            let lease = take32(rest, &mut p)?;
            let mu = take_lanes(rest, &mut p)?;
            (p == rest.len()).then_some(Ok((lease, mu)))
        }
        _ => None,
    }
}
