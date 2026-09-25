// 26 — the mind's own codec: a germ's bytes and address, the frame two
// scenes measure, and the byte an outcome rides the wire as.
//
//! The percept and the demand that were spelled here carried a voice's
//! painted sites to the court (law 8, ruled 2026-09-10); both are deleted with
//! their acts. A body that does not parse whole is refused; nothing here
//! guesses at a shape.

use super::*;

/// The germ's canonical ADDRESS — a digest of its ANASTOME form. Used to match
/// consilience receipts across voices; a wire/cache identity, never a reasoning
/// operand (the kernel reasons on the element, addresses by this).
pub fn germ_address(g: &Element) -> crate::transducer::address::Address {
    g.address()
}

/// The frame measured from two painted scenes (their site extents) — the
/// identity frame when both share dims; used where no grid is at hand.
#[expect(
    clippy::disallowed_methods,
    reason = "rule 4 — a scene's extent is the largest coordinate its painted sites occupy, content"
)]
pub fn frame_of(a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) -> crate::arc::Frame {
    // A scene's extent is its largest occupied coordinate plus one, which
    // the painted set determines (rule 4). An empty scene occupies no sites
    // and its extent is zero by measurement, not by default.
    let extent = |s: &[(i64, i64, u8)]| {
        let axis = |m: Option<i64>| match m {
            Some(x) if x > 0 => x as usize,
            _ => 0,
        };
        (
            axis(s.iter().map(|(r, _, _)| *r + 1).max()),
            axis(s.iter().map(|(_, c, _)| *c + 1).max()),
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
        // A wait on a citation reads as pending, like the door's
        // deferral: nothing was ruled and the voice speaks again. The
        // per-sentence shape that carries the cited address is
        // `Adjudication::Awaiting` on the speech reply, not this byte.
        Outcome::GermPending | Outcome::Deferred(_) | Outcome::Awaiting(_) => 2,
        Outcome::Installed(_) => 3,
        Outcome::Sectioned(_) => 6,
        Outcome::Fragmented(_) => 5,
        Outcome::Pivot => 4,
        Outcome::Refused(_) => 7,
    }
}
