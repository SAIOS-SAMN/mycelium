// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 19 — THE FORM TAPE'S PRIMITIVES: the readers and writers `tape.rs`
//! composes a sentence out of. Split from `tape.rs` at the §1.1 600-line
//! reading bound when v7 added the derivation codec (50.2 step 2); the
//! items are moved, not rewritten (§1.6).

use super::chain::Cell;
use crate::transducer::alphabet::{self, Word};
use crate::transducer::stalk;

pub(crate) fn word_to_form_bytes(w: &Word, contact: Option<alphabet::Contact>, out: &mut Vec<u8>) {
    w.to_bytes(out);
    put_contact(contact, out);
}

/// A word vertex's derivation and its contact ring (50.2 step 2, v7):
/// `[n u16][(form addr)(vertex addr)(exp i64)]…[residue][product when n>0]`
/// then the contact byte, where the plain word stood at v6.
pub(crate) fn derivation_to_form_bytes(
    d: &alphabet::Derivation,
    contact: Option<alphabet::Contact>,
    out: &mut Vec<u8>,
) -> Option<()> {
    d.to_bytes(out)?;
    put_contact(contact, out);
    Some(())
}

pub(crate) fn derivation_from_form_bytes(
    b: &[u8],
    pos: &mut usize,
) -> Option<(alphabet::Derivation, Option<alphabet::Contact>)> {
    let d = alphabet::Derivation::from_bytes(b, pos)?;
    let c = take_contact(b, pos)?;
    Some((d, c))
}

fn put_contact(contact: Option<alphabet::Contact>, out: &mut Vec<u8>) {
    match contact {
        None => out.push(0),
        Some(c) => {
            out.push(1);
            out.extend_from_slice(&c.anchor.to_le_bytes());
            out.push(c.ring);
        }
    }
}

fn take_contact(b: &[u8], pos: &mut usize) -> Option<Option<alphabet::Contact>> {
    let tag = *b.get(*pos)?;
    *pos += 1;
    if tag == 0 {
        return Some(None);
    }
    if tag != 1 {
        return None;
    }
    let anchor = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
    *pos += 2;
    let ring = *b.get(*pos)?;
    *pos += 1;
    Some(Some(alphabet::Contact { anchor, ring }))
}

pub(crate) fn word_from_form_bytes(
    b: &[u8],
    pos: &mut usize,
) -> Option<(Word, Option<alphabet::Contact>)> {
    let w = Word::from_bytes(b, pos)?;
    let c = take_contact(b, pos)?;
    Some((w, c))
}

fn bytes_left(b: &[u8], pos: usize) -> usize {
    b.len().saturating_sub(pos)
}

pub(crate) fn take_count(b: &[u8], pos: usize, n: usize, item: usize) -> Option<usize> {
    if item == 0 {
        return Some(n);
    }
    if n > bytes_left(b, pos) / item {
        return None;
    }
    Some(n)
}

pub(crate) fn put_inv(inv: &stalk::Invariant, out: &mut Vec<u8>) -> Option<()> {
    let n = u16::try_from(inv.0.len()).ok()?;
    out.extend_from_slice(&n.to_le_bytes());
    for a in &inv.0 {
        out.extend_from_slice(&a.to_le_bytes());
    }
    Some(())
}

pub(crate) fn take_inv(b: &[u8], pos: &mut usize) -> Option<stalk::Invariant> {
    let n = take_u16(b, pos)? as usize;
    let n = take_count(b, *pos, n, 8)?;
    let mut axes = Vec::with_capacity(n);
    for _ in 0..n {
        let v = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
        *pos += 8;
        axes.push(v);
    }
    Some(stalk::Invariant(axes))
}

pub(crate) fn take_u16(b: &[u8], pos: &mut usize) -> Option<u16> {
    let v = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
    *pos += 2;
    Some(v)
}

pub(crate) fn take_u32(b: &[u8], pos: &mut usize) -> Option<u32> {
    let v = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
    *pos += 4;
    Some(v)
}

pub(crate) fn take_cell(b: &[u8], pos: &mut usize) -> Option<Cell> {
    let r = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
    *pos += 8;
    let c = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
    *pos += 8;
    let before = *b.get(*pos)?;
    *pos += 1;
    let after = *b.get(*pos)?;
    *pos += 1;
    Some((r, c, before, after))
}
