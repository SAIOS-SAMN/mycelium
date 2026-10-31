// 47.1, 45.5 — ACT_FETCH, and the receipt section a cover
// digest carries behind it.
//
//! The section is framed, so its start is derived from the digest frame
//! that precedes it rather than guessed at.

use super::*;
use crate::chronometry::Place;

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
///
/// `None` when the prefix does not fit the two-byte length that frames it
/// (`SignedPrefix::fits`): the length was written `as u16` and wrapped, so
/// a hypha whose prefix carried its whole cover sent a section no peer
/// could read and every one of its digests was refused whole (I33). An act
/// whose section cannot be framed is not spoken (rule 11).
pub fn encode_receipt_section(
    prefix: &crate::court::admission::SignedPrefix,
    sig: &[u8; 64],
    receipt: &crate::chronometry::KStepReceipt,
) -> Option<Vec<u8>> {
    if !prefix.fits() {
        return None;
    }
    let pb = prefix.bytes()?;
    let plen = u16::try_from(pb.len()).ok()?;
    let mut rb = Vec::new();
    receipt.write(&mut rb)?;
    let rlen = u32::try_from(rb.len()).ok()?;
    let mut out = Vec::with_capacity(pb.len() + rb.len() + 70);
    out.extend_from_slice(&plen.to_le_bytes());
    out.extend_from_slice(&pb);
    out.extend_from_slice(sig);
    out.extend_from_slice(&rlen.to_le_bytes());
    out.extend_from_slice(&rb);
    Some(out)
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
        // One chart framed whole or not at all: a chart or an address that
        // reaches its u16 is refused by name and left out of the count.
        let mut rec = Vec::new();
        let framed = push_invariant(&mut rec, inv)
            .and_then(|()| crate::transducer::address::put_u32_count(&mut rec, have.len()))
            .and_then(|()| have.iter().try_for_each(|a| put_address(&mut rec, a)));
        if framed.is_none() {
            eprintln!("  fetch chart refused: a chart or an address it holds cannot be framed");
            continue;
        }
        out.append(&mut rec);
        k += 1;
    }
    out[count_at..count_at + 4].copy_from_slice(&k.to_le_bytes());
    out
}

pub fn decode_fetch(body: &[u8]) -> Option<([u8; 32], Vec<Chart>)> {
    if !act_is(body, ACT_FETCH) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let mut p = 33usize;
    let k = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    if k > body.len().checked_sub(p)? / 5 {
        return None;
    }
    let mut charts = Vec::with_capacity(k);
    for _ in 0..k {
        let inv = read_invariant(body, &mut p)?;
        let n = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        if n > body.len().checked_sub(p)? / 2 {
            return None;
        }
        let mut have = Vec::with_capacity(n);
        for _ in 0..n {
            have.push(take_address(body, &mut p)?);
        }
        charts.push(Chart { inv, have });
    }
    Some((voice, charts))
}

/// A SPAN OF A SIGNER'S LINE ON THE WIRE: the place it opens after — a tag
/// byte, `0` from the line's start or `1` and the place — then the place it
/// closes at, each place `K‖i`.
pub(crate) fn put_span(out: &mut Vec<u8>, after: Option<Place>, upto: Place) {
    match after {
        None => out.push(0),
        Some(a) => {
            out.push(1);
            a.put_canonical(out);
        }
    }
    upto.put_canonical(out);
}

/// The inverse of [`put_span`], whole or refused.
pub(crate) fn take_span(b: &[u8], p: &mut usize) -> Option<(Option<Place>, Place)> {
    let tag = *b.get(*p)?;
    *p += 1;
    let after = match tag {
        0 => None,
        1 => Some(Place::take_canonical(b, p)?),
        _ => return None,
    };
    Some((after, Place::take_canonical(b, p)?))
}

/// 47.2's PLACE-RANGE REQUEST (S13.10): which signer, and which span of
/// its line — strictly after `after` up to `upto`. `replay` (78.147) is an
/// explicit ask for an era's own receipts — a citation proved by its era, a
/// ledger read — the one ask the cold archive answers; a catch-up opening
/// below the signer's fold root is answered with the root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptsRequest {
    pub voice: [u8; 32],
    pub signer: [u8; 32],
    pub after: Option<Place>,
    pub upto: Place,
    pub replay: bool,
}

/// The replay ask's trailing tag; a catch-up carries none.
const REPLAY: u8 = 1;

pub fn encode_receipts_request(voice: &[u8; 32], signer: &[u8; 32], after: Option<Place>, upto: Place) -> Vec<u8> {
    let mut out = vec![ACT_RECEIPTS];
    out.extend_from_slice(voice);
    out.extend_from_slice(signer);
    put_span(&mut out, after, upto);
    out
}

/// The same span asked as an ERA REPLAY (78.147): the catch-up ask and the
/// replay tag after it.
pub fn encode_replay_request(voice: &[u8; 32], signer: &[u8; 32], after: Option<Place>, upto: Place) -> Vec<u8> {
    let mut out = encode_receipts_request(voice, signer, after, upto);
    out.push(REPLAY);
    out
}

pub fn decode_receipts_request(body: &[u8]) -> Option<ReceiptsRequest> {
    if !act_is(body, ACT_RECEIPTS) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let signer: [u8; 32] = body.get(33..65)?.try_into().ok()?;
    let mut p = 65usize;
    let (after, upto) = take_span(body, &mut p)?;
    let replay = match body.get(p..) {
        Some([]) => false,
        Some([REPLAY]) => true,
        _ => return None,
    };
    Some(ReceiptsRequest { voice, signer, after, upto, replay })
}

/// The K-range reply: as many of `receipts`, in order, as fit `budget`
/// bytes — a page, never a receipt cut short.
///
/// A PAGE ALWAYS CARRIES ITS FIRST RECEIPT, whatever the budget says. The
/// fit test ran on every receipt including the first, so a receipt larger
/// than one frame was never served — and `break`, not `continue`, meant
/// every page that STARTED at that K came back empty, forever. A paging
/// protocol that cannot guarantee one item per page does not page, it
/// stops: the reader asks for the K it is missing, is told nothing, and
/// has no way to ask differently.
///
/// Measured on the development deployment at epoch 11: hypha a held receipts 0..2000 and
/// answered `1111..1111` with count 0, `1110..1112` with count 1 and
/// `1109..1112` with count 2 — the page walking up to the oversized
/// receipt and halting on it, so the back-fill could never pass K 1111 and
/// no peer receipt was ever admitted as a parent.
///
/// The only bound left is `BODY_BUDGET`, which the frame codec refuses
/// past: a receipt over the protocol ceiling cannot ride any wire, and
/// that is a refusal rather than a silent short page (rule 11).
pub fn encode_receipts_reply(receipts: &[crate::chronometry::KStepReceipt], budget: usize) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&0u32.to_le_bytes());
    let mut n = 0u32;
    // THE WALK IS BOUNDED BY THE PAGE, not by a `break` it hopes to reach:
    // a page cannot carry more than its budget divided by the smallest a
    // receipt can be, so that is how far this reads (rule 10). `receipts`
    // is a slice the caller chose the length of, and nothing in this
    // function's own shape stopped it.
    let cap = budget / crate::chronometry::KStepReceipt::MIN_TAPE;
    // THE SHAPE SAYS WHICH RECEIPT IS THE FIRST, not a tally of how many
    // have been written (rule 2): the head of the slice always rides, and
    // the tail rides only while it fits.
    // A node that cannot be written ends the page there, named: the pages
    // are a signer's line in K order, and none is skipped past (LD0).
    if let Some((head, rest)) = receipts.split_first() {
        let mut rb = Vec::new();
        let size = match head.write(&mut rb) {
            Some(()) => out.len() + rb.len(),
            None => {
                eprintln!("  receipts page refused: its first node cannot be written");
                usize::MAX
            }
        };
        if size <= crate::network::wire::BODY_BUDGET {
            out.extend_from_slice(&rb);
            n += 1;
            for r in rest.iter().take(cap) {
                let mut rb = Vec::new();
                if r.write(&mut rb).is_none() {
                    eprintln!("  receipts page ends: a node cannot be written");
                    break;
                }
                let size = out.len() + rb.len();
                if size > budget {
                    break;
                }
                out.extend_from_slice(&rb);
                n += 1;
            }
        }
    }
    out[..4].copy_from_slice(&n.to_le_bytes());
    out
}

/// The page read back whole, or refused: a reply that does not parse to
/// its end is not a shorter page (rule 11).
pub fn decode_receipts_reply(body: &[u8]) -> Option<Vec<crate::chronometry::KStepReceipt>> {
    decode_receipts_reply_at(body).map(|(page, _)| page)
}

/// 78.151 (WIRE-CHANGE) — THE HEIGHT A READ WAS ANSWERED AT: a door that
/// serves a read off its published reading writes that reading's height
/// behind the answer, eight bytes, so the asker knows how fresh it is.
pub const READING_HEIGHT_LEN: usize = 8;

/// The published reading's height, written behind a read's answer.
pub fn put_reading_height(out: &mut Vec<u8>, height: u64) {
    out.extend_from_slice(&height.to_le_bytes());
}

/// What stands behind an answer that ends at `end`: nothing (a door that
/// writes no height), or the height, whole — anything else refuses.
pub(crate) fn take_reading_height(body: &[u8], end: usize) -> Option<Option<u64>> {
    match body.len().checked_sub(end)? {
        0 => Some(None),
        READING_HEIGHT_LEN => Some(Some(u64::from_le_bytes(body.get(end..)?.try_into().ok()?))),
        _ => None,
    }
}

/// The page read back whole with the height it was answered at, if the
/// door wrote one (78.151), or refused.
pub fn decode_receipts_reply_at(body: &[u8]) -> Option<(Vec<crate::chronometry::KStepReceipt>, Option<u64>)> {
    let n = u32::from_le_bytes(body.get(0..4)?.try_into().ok()?) as usize;
    let mut p = 4usize;
    if n > body.len().checked_sub(p)? / crate::chronometry::KStepReceipt::MIN_TAPE {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(crate::chronometry::KStepReceipt::read(body, &mut p)?);
    }
    Some((out, take_reading_height(body, p)?))
}
