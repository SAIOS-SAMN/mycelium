// 47.4, XSEPTUM 5.5 — ACT_EQUIVOCATION: two receipt sections a third party
// holds under one key at one K, presented to the court that burns.
//
//! Each section is 47.1's receipt section as a digest carries it, framed by
//! its length; a section that does not parse whole, or a frame with bytes
//! left over, is refused whole (rule 11).

use super::*;

/// The act's layout version, a wire tag.
pub const EQUIVOCATION_VERSION: u8 = 1;

/// `[ACT_EQUIVOCATION][version][len32 section_a][len32 section_b]`, each
/// section `encode_receipt_section`'s bytes: prefix, signature, receipt.
/// `None` when either section cannot be framed.
pub fn encode_equivocation(a: &ReceiptSection, b: &ReceiptSection) -> Option<Vec<u8>> {
    let mut out = vec![ACT_EQUIVOCATION, EQUIVOCATION_VERSION];
    for s in [a, b] {
        let bytes = encode_receipt_section(&s.prefix, &s.sig, &s.receipt)?;
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(&bytes);
    }
    Some(out)
}

/// The two sections, in the order presented. `None` for another act or
/// version, a section that does not parse whole and fill its length, or a
/// frame with bytes after the second section.
pub fn decode_equivocation(body: &[u8]) -> Option<(ReceiptSection, ReceiptSection)> {
    if body.first() != Some(&ACT_EQUIVOCATION) || body.get(1) != Some(&EQUIVOCATION_VERSION) {
        return None;
    }
    let mut p = 2usize;
    let mut take = || -> Option<ReceiptSection> {
        let n = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        let slice = body.get(p..p + n)?;
        p += n;
        if super::receipts::receipt_section_end(slice, 0)? != slice.len() {
            return None;
        }
        decode_receipt_section(slice, 0)
    };
    let a = take()?;
    let b = take()?;
    (p == body.len()).then_some((a, b))
}
