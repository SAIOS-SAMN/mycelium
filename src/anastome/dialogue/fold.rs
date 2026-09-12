// 47.3, 45.8 — ACT_FOLD and ACT_ROWS: a home's fold proposal to a witness,
// the witness's answer, and the ledger rows a witness re-derives over.
//
//! The proposal carries the prefix's first K, its root, the epoch rows the
//! home will write and the home's signed position over the root hash; the
//! witness fetches the prefix and answers with its own signed position
//! over the same hash, or a refusal naming a 34.1 constant.

use super::*;
use crate::chronometry::Root;
use crate::economy::Entry;

/// `[ACT_FOLD][k0 u64][root k u64][root hash 32][root height u64][n u32][(who 32)(μ⃗ 4×u64)…][receipt section]`
#[derive(Clone)]
pub struct FoldProposal {
    pub k0: u64,
    pub root: Root,
    pub epochs: Vec<([u8; 32], [u64; 4])>,
    pub section: ReceiptSection,
}

pub fn encode_fold(p: &FoldProposal) -> Option<Vec<u8>> {
    let mut out = vec![ACT_FOLD];
    out.extend_from_slice(&p.k0.to_le_bytes());
    p.root.write(&mut out);
    out.extend_from_slice(&(p.epochs.len() as u32).to_le_bytes());
    for (who, mu) in &p.epochs {
        out.extend_from_slice(who);
        push_mu(&mut out, mu);
    }
    out.extend_from_slice(&super::encode_receipt_section(&p.section.prefix, &p.section.sig, &p.section.receipt)?);
    Some(out)
}

pub fn decode_fold(body: &[u8]) -> Option<FoldProposal> {
    if !act_is(body, ACT_FOLD) {
        return None;
    }
    let mut p = 1usize;
    let k0 = u64::from_le_bytes(body.get(p..p + 8)?.try_into().ok()?);
    p += 8;
    let root = Root::read(body, &mut p)?;
    let n = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    if n > body.len().checked_sub(p)? / 64 {
        return None;
    }
    let mut epochs = Vec::with_capacity(n);
    for _ in 0..n {
        let who: [u8; 32] = body.get(p..p + 32)?.try_into().ok()?;
        p += 32;
        let mu = read_mu(body, &mut p)?;
        epochs.push((who, mu));
    }
    let section = super::decode_receipt_section(body, p)?;
    Some(FoldProposal { k0, root, epochs, section })
}

/// The witness's answer: `[0][vk 32][len16 why]` refused, `[1][vk 32]`
/// unmeasured (the prefix could not be fetched), `[2][section]` co-signed.
#[derive(Clone)]
pub enum FoldWitness {
    Refused { vk: [u8; 32], why: Vec<u8> },
    Unmeasured { vk: [u8; 32] },
    CoSigned { section: Box<ReceiptSection> },
}

pub fn encode_fold_reply(w: &FoldWitness) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    match w {
        FoldWitness::Refused { vk, why } => {
            out.push(0);
            out.extend_from_slice(vk);
            out.extend_from_slice(&(why.len() as u16).to_le_bytes());
            out.extend_from_slice(why);
        }
        FoldWitness::Unmeasured { vk } => {
            out.push(1);
            out.extend_from_slice(vk);
        }
        FoldWitness::CoSigned { section } => {
            out.push(2);
            out.extend_from_slice(&super::encode_receipt_section(&section.prefix, &section.sig, &section.receipt)?);
        }
    }
    Some(out)
}

pub fn decode_fold_reply(body: &[u8]) -> Option<FoldWitness> {
    let arm = act_tag(body)?;
    match arm {
        0 => {
            let vk: [u8; 32] = body.get(1..33)?.try_into().ok()?;
            let n = u16::from_le_bytes(body.get(33..35)?.try_into().ok()?) as usize;
            Some(FoldWitness::Refused { vk, why: body.get(35..35 + n)?.to_vec() })
        }
        1 => {
            let vk: [u8; 32] = body.get(1..33)?.try_into().ok()?;
            Some(FoldWitness::Unmeasured { vk })
        }
        2 => Some(FoldWitness::CoSigned { section: Box::new(super::decode_receipt_section(body, 1)?) }),
        _ => None,
    }
}

/// `[ACT_ROWS][voice 32][signer 32][from u64][to u64][cursor u32]`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowsRequest {
    pub voice: [u8; 32],
    pub signer: [u8; 32],
    pub from: u64,
    pub to: u64,
    pub cursor: u32,
}

pub fn encode_rows_request(voice: &[u8; 32], signer: &[u8; 32], from: u64, to: u64, cursor: u32) -> Vec<u8> {
    let mut out = vec![ACT_ROWS];
    out.extend_from_slice(voice);
    out.extend_from_slice(signer);
    out.extend_from_slice(&from.to_le_bytes());
    out.extend_from_slice(&to.to_le_bytes());
    out.extend_from_slice(&cursor.to_le_bytes());
    out
}

pub fn decode_rows_request(body: &[u8]) -> Option<RowsRequest> {
    if !act_is(body, ACT_ROWS) || body.len() != 85 {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let signer: [u8; 32] = body.get(33..65)?.try_into().ok()?;
    let from = u64::from_le_bytes(body.get(65..73)?.try_into().ok()?);
    let to = u64::from_le_bytes(body.get(73..81)?.try_into().ok()?);
    let cursor = u32::from_le_bytes(body.get(81..85)?.try_into().ok()?);
    Some(RowsRequest { voice, signer, from, to, cursor })
}

/// `[total u32][m u32][row…]`: as many of `rows` as fit `budget`.
pub fn encode_rows_reply(total: u32, rows: &[Entry], budget: usize) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&total.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    let mut m = 0u32;
    for r in rows {
        if out.len() + Entry::TAPE > budget {
            break;
        }
        r.write(&mut out);
        m += 1;
    }
    out[4..8].copy_from_slice(&m.to_le_bytes());
    out
}

pub fn decode_rows_reply(body: &[u8]) -> Option<(u32, Vec<Entry>)> {
    let total = u32::from_le_bytes(body.get(0..4)?.try_into().ok()?);
    let m = u32::from_le_bytes(body.get(4..8)?.try_into().ok()?) as usize;
    let mut p = 8usize;
    if m > body.len().checked_sub(p)? / Entry::TAPE {
        return None;
    }
    let mut rows = Vec::with_capacity(m);
    for _ in 0..m {
        rows.push(Entry::read(body, &mut p)?);
    }
    (p == body.len()).then_some((total, rows))
}
