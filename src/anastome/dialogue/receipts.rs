// 47.1, 45.5 — ACT_GENESIS and ACT_FETCH, and the receipt section a cover
// digest carries behind it.
//
//! The section is framed, so its start is derived from the digest frame
//! that precedes it rather than guessed at.

use super::*;

/// 47.1's RECEIPT SECTION, appended after a digest's signature: the
/// sender's head receipt, the 47.1 prefix it was signed over, and the
/// signature over exactly those prefix bytes.
///
/// `[plen u16][prefix][sig 64][rlen u32][receipt]`
///
/// The section is what carries a signer's causal position with its
/// signature (47.1). Without it a receiver could verify that bytes came
/// from a key and learn nothing about where in that key's chain they came
/// from, so no receipt was ever admitted and no height ever climbed on
/// anything but the receiver's own strokes (45.2).
///
/// It is APPENDED, so a peer that does not send one still decodes: the
/// digest's own carriers and signature are unchanged by it. Migrating
/// `cover_signed_prefix` itself onto 47.1 -- which moves what the digest's
/// primary signature covers -- is its own task and is not this.
pub fn encode_receipt_section(
    prefix: &crate::court::admission::SignedPrefix,
    sig: &[u8; 64],
    receipt: &crate::chronometry::KStepReceipt,
) -> Vec<u8> {
    let pb = prefix.bytes();
    let mut rb = Vec::new();
    receipt.write(&mut rb);
    let mut out = Vec::with_capacity(pb.len() + rb.len() + 70);
    out.extend_from_slice(&(pb.len() as u16).to_le_bytes());
    out.extend_from_slice(&pb);
    out.extend_from_slice(sig);
    out.extend_from_slice(&(rb.len() as u32).to_le_bytes());
    out.extend_from_slice(&rb);
    out
}

/// Read a receipt section from `body` at `at`, the offset one past the
/// digest's signature. `None` when the peer sent none, or when the section
/// does not parse whole -- a truncated section is refused, never a partial
/// admission (rule 11).
pub fn decode_receipt_section(body: &[u8], at: usize) -> Option<ReceiptSection> {
    let mut p = at;
    let plen = u16::from_le_bytes(body.get(p..p + 2)?.try_into().ok()?) as usize;
    p += 2;
    let prefix = crate::court::admission::SignedPrefix::read(body.get(p..p + plen)?)?;
    p += plen;
    let sig: [u8; 64] = body.get(p..p + 64)?.try_into().ok()?;
    p += 64;
    let rlen = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    let rb = body.get(p..p + rlen)?;
    let mut q = 0usize;
    let receipt = crate::chronometry::KStepReceipt::read(rb, &mut q)?;
    if q != rlen {
        return None;
    }
    Some(ReceiptSection { prefix, sig, receipt })
}

/// Offset one past a receipt section starting at `at`, or `None` when
/// there is none there or it does not parse whole.
pub(crate) fn receipt_section_end(body: &[u8], at: usize) -> Option<usize> {
    let mut p = at;
    let plen = u16::from_le_bytes(body.get(p..p + 2)?.try_into().ok()?) as usize;
    p += 2;
    crate::court::admission::SignedPrefix::read(body.get(p..p + plen)?)?;
    p += plen + 64;
    let rlen = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    let rb = body.get(p..p + rlen)?;
    let mut q = 0usize;
    crate::chronometry::KStepReceipt::read(rb, &mut q)?;
    (q == rlen).then_some(p + rlen)
}

pub fn encode_fetch<A: AsRef<[u8]>>(
    voice: &[u8; 32],
    charts: &[ChartOf<A>],
    budget: usize,
) -> Vec<u8> {
    let mut out = vec![ACT_FETCH];
    out.extend_from_slice(voice);
    let count_at = out.len();
    out.extend_from_slice(&0u32.to_le_bytes());
    let mut k = 0u32;
    for ChartOf { inv, have } in charts {
        let width = 1 + 8 * inv.0.len() + 4 + have.iter().map(|a| 2 + a.as_ref().len()).sum::<usize>();
        if out.len() + width > budget {
            break;
        }
        push_invariant(&mut out, inv);
        out.extend_from_slice(&(have.len() as u32).to_le_bytes());
        for a in have {
            let _ = put_addr(&mut out, a);
        }
        k += 1;
    }
    out[count_at..count_at + 4].copy_from_slice(&k.to_le_bytes());
    out
}

pub fn decode_fetch(body: &[u8]) -> Option<([u8; 32], Vec<Chart>)> {
    if body.first() != Some(&ACT_FETCH) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut p = 33usize;
    let k = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    if k > body.len().saturating_sub(p) / 5 {
        return None;
    }
    let mut charts = Vec::with_capacity(k);
    for _ in 0..k {
        let inv = read_invariant(body, &mut p)?;
        let n = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        if n > body.len().saturating_sub(p) / 2 {
            return None;
        }
        let mut have = Vec::with_capacity(n);
        for _ in 0..n {
            have.push(take_addr(body, &mut p)?);
        }
        charts.push(Chart { inv, have });
    }
    Some((voice, charts))
}

pub fn encode_genesis_request(voice: &[u8; 32], cursor: u32) -> Vec<u8> {
    let mut out = vec![ACT_GENESIS];
    out.extend_from_slice(voice);
    out.extend_from_slice(&cursor.to_le_bytes());
    out
}

pub fn decode_genesis_request(body: &[u8]) -> Option<([u8; 32], u32)> {
    if body.first() != Some(&ACT_GENESIS) || body.len() != 37 {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let cursor = u32::from_le_bytes(body.get(33..37)?.try_into().ok()?);
    Some((voice, cursor))
}
