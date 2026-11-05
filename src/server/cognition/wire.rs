// 26 — the mind's own codec: the frame two
// scenes measure, and the byte an outcome rides the wire as.
//
//! The percept and the demand that were spelled here carried a voice's
//! painted sites to the court (law 8, ruled 2026-09-10); both are deleted with
//! the acts that carried them. A body that does not parse whole is refused; nothing here
//! guesses at a shape.

use super::*;

/// The frame measured from two painted scenes (the scenes' site extents) — the
/// identity frame when both share dims; used where no sample is at hand.
#[expect(
    clippy::disallowed_methods,
    reason = "rule 4 — a scene's extent is the largest coordinate its painted sites occupy, content"
)]
pub fn frame_of(
    a: &[crate::transducer::spelled::PaintedSite],
    b: &[crate::transducer::spelled::PaintedSite],
) -> Option<crate::transducer::element::Frame> {
    // A scene's extent is its largest occupied coordinate plus one, which
    // the painted set determines (rule 4). An empty scene occupies no sites
    // and its extent is zero by measurement, not by default. An extent no
    // `usize` holds is refused, never truncated (STD1 3c-5).
    let extent = |s: &[crate::transducer::spelled::PaintedSite]| -> Option<(usize, usize)> {
        let axis = |m: Option<i64>| match m {
            Some(x) if x > 0 => usize::try_from(x).ok(),
            _ => Some(0),
        };
        // The frame's two counts are the plane's two axes in the plane's
        // order (`Frame`, a defect named in MU-SENSING §8).
        let reach = |on: usize| s.iter().filter_map(|p| p.site.coordinates().get(on).map(|x| x + 1)).max();
        Some((axis(reach(0))?, axis(reach(1))?))
    };
    let (hi, wi) = extent(a)?;
    let (ho, wo) = extent(b)?;
    Some(crate::transducer::element::Frame::new(hi, wi, ho, wo))
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
        Outcome::GermPending | Outcome::Deferred(_) | Outcome::Awaiting(_) => crate::anastome::dialogue::PENDING_CODE,
        Outcome::Installed(_) => 3,
        Outcome::Sectioned(_) => 6,
        Outcome::Fragmented(_) => 5,
        Outcome::Pivot => 4,
        Outcome::Refused(_) => 7,
    }
}
