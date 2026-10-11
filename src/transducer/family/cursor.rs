// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 50.8 — WHAT A COVER HAS SETTLED. A cover that a stroke did not finish
//! keeps what it MEASURED — the pieces it placed, the sites it reached,
//! the input pieces it consumed — and the next stroke goes on from that
//! state. Nothing it measured is discarded (rule 7).
//!
//! THERE IS NO CURSOR HERE. A `Stage` naming which pass is owed and the
//! last key that pass finished would be an ITINERARY, an extrinsic index
//! into a traversal, which is the shape of a tape machine and not of a complex;
//! every write in a cover is keyed, so redoing it writes the same entries
//! at the same addresses, and a carrier that can be recomputed exactly
//! carries no information — only the order a previous stroke happened to
//! read in.
//!
//! WHAT IS OWED IS READ FROM THE STATE, not from a finger pointing at an
//! index. An image piece whose sites are all in `reached` is carried; an
//! input piece in `consumed` is spoken; a meet already in `cover.meets` is
//! recorded. Each of the three passes asks its own carrier and skips what
//! that carrier already answers.
//!
//! WHAT REMAINS UNCARRIED IS NOT A QUEUE POSITION. A piece the cover
//! cannot carry is read again on the next stroke and fails again; it is
//! not "waiting its turn", it is the uncarried support — a standing defect
//! the flow has not relaxed — and `Cover::uncarried` is where it already
//! reports. The cover is incomplete because of it, and saying so is the
//! reading.
//!
//! IT IS NOT A SHARE AND NOT A RATIO: a share hides starvation behind a
//! constant and a halved budget puts a chosen proportion in the same
//! place; `physics/budget` records both.
//!
//! The caller still reads `Unmeasured` and the demand still stands exactly
//! as F typed it — nothing a half-run cover placed reaches a form (rule
//! 11) — but the placements are KEPT here, off the form, to be finished.

use super::cover::{Cover, Supports};
use crate::network::nerve::{coord, Keyed};
use std::cell::RefCell;

/// A COVER PART-WAY THROUGH: what it has placed, what it has consumed,
/// what it has reached. That is the whole of its state and it is
/// coordinate-free — no index, no stage, no position in a traversal. `occ` is
/// not kept either: it is a reading of `placed` and is re-derived, because
/// law 12 keeps one fact on one carrier.
pub(crate) struct Pending {
    pub(crate) cover: Cover,
    /// The two supports of each placed carriage, by the [`coord::Carriage`]
    /// it is; the input pieces spoken, by the [`coord::Piece`] each is; and
    /// the sites of the base either body reached.
    pub(crate) placed: Keyed<Supports, coord::Carriage>,
    pub(crate) consumed: Keyed<(), coord::Piece>,
    pub(crate) reached: Keyed<()>,
}

impl Pending {
    pub(crate) fn new() -> Pending {
        Pending {
            cover: Cover::new(),
            placed: Keyed::new(),
            consumed: Keyed::new(),
            reached: Keyed::new(),
        }
    }
}

thread_local! {
    static PENDING: RefCell<Keyed<Pending>> = RefCell::new(Keyed::new());
}

/// THE ADDRESS OF A DEFORMATION, as content: the two bodies' compartments
/// written out and the frame ratio that was asked for. Sensor bytes at the
/// sensor boundary, which is the one place bytes may be a key (rule 6) and
/// is where the percept memo beside this one is already keyed.
pub(crate) fn deformation_key(
    input: &[super::Compartment],
    image: &[super::Compartment],
    scale: Option<((usize, usize), (usize, usize))>,
) -> Vec<u8> {
    let mut k = Vec::with_capacity((input.len() + image.len()) * 29 + 40);
    for (tag, compartments) in [(0u8, input), (1u8, image)] {
        k.push(tag);
        crate::transducer::address::put_key_count(&mut k, compartments.len());
        for l in compartments {
            l.site.put(&mut k);
            k.push(l.hue);
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

/// Park a cover that ran out of budget; the count parked is read below.
pub(crate) fn park(key: &[u8], pending: Pending) {
    PENDING.with(|p| p.borrow_mut().insert(key, pending));
}

/// How many covers are parked; nothing branches on it.
#[cfg(test)]
pub(crate) fn parked() -> usize {
    PENDING.with(|p| p.borrow().scoped().count())
}

#[cfg(test)]
pub(crate) fn clear() {
    PENDING.with(|p| *p.borrow_mut() = Keyed::new());
}
