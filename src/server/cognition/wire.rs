// 26 — the mind's own codec: a germ's bytes and address, the frame two
// scenes measure, and the byte an outcome rides the wire as.
//
//! The percept and the demand that were spelled here carried a voice's
//! cells to the court (law 8, ruled 2026-09-10); both are deleted with
//! their acts. A body that does not parse whole is refused; nothing here
//! guesses at a shape.

use super::*;

/// Canonical bytes of an anonymous germ — its coordinate-exact ANASTOME form (the
/// banked word ⊕ the block-diagonal algebra; carries no English, no "kind"). This
/// is the language the hypha teaches back across the nerve; the client treats it
/// opaquely (display only).
pub fn germ_bytes(g: &Element) -> Vec<u8> {
    g.canonical()
}

/// The germ's canonical ADDRESS — a digest of its ANASTOME form. Used to match
/// consilience receipts across voices; a wire/cache identity, never a reasoning
/// operand (the kernel reasons on the element, addresses by this).
pub fn germ_address(g: &Element) -> crate::transducer::address::Address {
    g.address()
}

/// The frame measured from two cell scenes (their 0-cell extents) — the
/// identity frame when both share dims; used where no grid is at hand.
pub fn frame_of(a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) -> crate::arc::Frame {
    let extent = |s: &[(i64, i64, u8)]| {
        (
            s.iter().map(|(r, _, _)| *r + 1).max().unwrap_or(0).max(0) as usize,
            s.iter().map(|(_, c, _)| *c + 1).max().unwrap_or(0).max(0) as usize,
        )
    };
    let (hi, wi) = extent(a);
    let (ho, wo) = extent(b);
    crate::arc::Frame::new(hi, wi, ho, wo)
}

/// One byte on the wire summarising an outcome (server → client reply). A
/// deferral rides as the pending code with no height (23.8, S13.4), which
/// is the shape `dialogue::encode_deferral` writes.
pub fn outcome_code(o: &Outcome) -> u8 {
    match o {
        Outcome::Decomposed(_) => 1,
        Outcome::GermPending | Outcome::Deferred(_) => 2,
        Outcome::Installed(_) => 3,
        Outcome::Sectioned(_) => 6,
        Outcome::Fragmented(_) => 5,
        Outcome::Pivot => 4,
        Outcome::Refused(_) => 7,
    }
}
