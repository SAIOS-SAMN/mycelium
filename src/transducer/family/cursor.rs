// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! L20, S13.3 — WHERE A COVER STOPPED. A stroke that ends mid-cover leaves
//! the cover PENDING and the next stroke resumes where it stopped, rather
//! than re-covering from the first piece. That is what turns the cover's
//! cost into strokes.
//!
//! THE MECHANISM ALREADY EXISTS IN THE TREE and this is its second use:
//! `stroke_fills` resumes from `fill_cursor` under S13.3 and never
//! restarts at the first key. Measured at 754d192, the cover needs 4 374
//! reads over twelve tasks and the machine's stroke gives it 16, so
//! covers that cannot complete were re-doing their first pieces every
//! stroke and reaching no further.
//!
//! IT IS NOT A SHARE AND NOT A RATIO. `cover_share` was struck for hiding
//! starvation behind a constant and `Budget::halve` after it for putting a
//! chosen proportion in the same place; `physics/budget` records both. The
//! cover spends the caller's budget and its cost shows in `exhausted` and
//! `unmeasured` with no number in front of it. What a cursor changes is
//! that the spend ACCUMULATES across strokes instead of being paid again.
//!
//! WHAT IS NOT ROLLED BACK ANY MORE, said plainly: a cover that ends used
//! to discard everything it had placed, and the caller read `Unmeasured`.
//! The caller still reads `Unmeasured` and the demand still stands exactly
//! as F typed it — nothing a half-run cover placed reaches a form (rule
//! 11) — but the placements are KEPT here, off the form, to be finished.

use super::cover::{Cover, Supports};
use crate::network::nerve::Keyed;
use std::cell::RefCell;

/// WHICH STAGE OF THE COVER IS OWED, and the last key that stage finished.
/// The key is the piece's own address, so a resume skips what is already
/// behind it in key order and never counts how many were done.
#[derive(Clone)]
pub(crate) enum Stage {
    /// (a) the pullback, over the image's pieces.
    Pullback(Option<Vec<u8>>),
    /// (b) the structural pass, over the input's unconsumed pieces. TWO
    /// keys, because the charge there is per carriage ATTEMPT and one
    /// input piece is tried against every output of its kind and hue: the
    /// input piece and the output piece within it. The charge and the
    /// cursor are the same unit here as everywhere else, or a stroke that
    /// pays for one attempt does not advance by one.
    Structure(Option<(Vec<u8>, Vec<u8>)>),
    /// (d) the meets, over the occupancy of the placed supports.
    Meets(Option<Vec<u8>>),
}

/// A COVER PART-WAY THROUGH, with everything the next stroke needs to go
/// on: what it has placed, what it has consumed, what it has reached, and
/// where it stopped. `occ` is NOT kept — it is re-derived from `placed` on
/// the resume, because it is a reading of `placed` and law 12 keeps one
/// fact on one carrier.
pub(crate) struct Pending {
    pub(crate) cover: Cover,
    pub(crate) placed: Keyed<Supports>,
    pub(crate) consumed: Keyed<()>,
    pub(crate) reached: Keyed<()>,
    pub(crate) stage: Stage,
}

impl Pending {
    pub(crate) fn new() -> Pending {
        Pending {
            cover: Cover::new(),
            placed: Keyed::new(),
            consumed: Keyed::new(),
            reached: Keyed::new(),
            stage: Stage::Pullback(None),
        }
    }
}

thread_local! {
    static PENDING: RefCell<Keyed<Pending>> = RefCell::new(Keyed::new());
}

/// THE ADDRESS OF A DEFORMATION, as content: the two bodies' cells written
/// out and the frame ratio that was asked for. Sensor bytes at the sensor
/// boundary, which is the one place bytes may be a key (rule 6) and is
/// where the percept memo beside this one is already keyed.
pub(crate) fn deformation_key(
    input: &[(i64, i64, u8)],
    image: &[(i64, i64, u8)],
    scale: Option<((usize, usize), (usize, usize))>,
) -> Vec<u8> {
    let mut k = Vec::with_capacity((input.len() + image.len()) * 17 + 40);
    for (tag, cells) in [(0u8, input), (1u8, image)] {
        k.push(tag);
        k.extend_from_slice(&(cells.len() as u32).to_le_bytes());
        for &(r, c, h) in cells {
            k.extend_from_slice(&r.to_le_bytes());
            k.extend_from_slice(&c.to_le_bytes());
            k.push(h);
        }
    }
    match scale {
        None => k.push(0),
        Some(((a, b), (c, d))) => {
            k.push(1);
            for n in [a, b, c, d] {
                k.extend_from_slice(&(n as u64).to_le_bytes());
            }
        }
    }
    k
}

/// Take the pending cover at this deformation, if a stroke parked one. It
/// is REMOVED on the take: a cover is worked on by one stroke at a time,
/// and a stroke that takes it either finishes it or parks it again.
pub(crate) fn take(key: &[u8]) -> Option<Pending> {
    PENDING.with(|p| p.borrow_mut().remove(key))
}

/// Park a cover that ran out of budget, and say so on the line.
pub(crate) fn park(key: &[u8], pending: Pending) {
    // The line names the STAGE and what is placed so far, not a prefix of
    // the key: the key is a deformation's cells written out, and two bytes
    // of it name nothing a reader can use.
    let stage = match &pending.stage {
        Stage::Pullback(_) => "pullback",
        Stage::Structure(_) => "structure",
        Stage::Meets(_) => "meets",
    };
    eprintln!(
        "  cover pending at {stage} placed {} meets {}",
        pending.placed.fiber(crate::network::nerve::ROOT).count(),
        pending.cover.meets.fiber(crate::network::nerve::ROOT).count()
    );
    PENDING.with(|p| p.borrow_mut().insert(key, pending));
}

/// How many covers are parked. A reading for the soak line; nothing
/// branches on it.
pub(crate) fn parked() -> usize {
    PENDING.with(|p| p.borrow().fiber(crate::network::nerve::ROOT).count())
}

#[cfg(test)]
pub(crate) fn clear() {
    PENDING.with(|p| *p.borrow_mut() = Keyed::new());
}
