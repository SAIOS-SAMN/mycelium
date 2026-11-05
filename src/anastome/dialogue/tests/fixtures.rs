// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 47.1, MU-HOSTING §5a.10 — fixtures for the dialogue's claims: the
// readings a receipt section is compared by, since none of its parts has `==`.

use crate::chronometry::{KStepReceipt, Parent};
use crate::court::admission::{Predecessor, SignedPrefix};

/// ONE SIGNED PREFIX, by its coordinates written out: the origin, the
/// predecessor it commits to as its 32 bytes, its place and height, chart
/// and payload.
pub(super) fn one_prefix(a: &SignedPrefix, b: &SignedPrefix) -> bool {
    (a.genesis, a.epoch, a.prev_receipt_hash.bytes(), a.at, a.height) == (b.genesis, b.epoch, b.prev_receipt_hash.bytes(), b.at, b.height)
        && (&a.chart, &a.payload) == (&b.chart, &b.payload)
}

/// ONE RECEIPT, read by the link sense: each re-derives its own hash, and
/// `a` stands at the stamp of `b` with a chain continuing across the two.
pub(super) fn one_receipt(a: &KStepReceipt, b: &KStepReceipt) -> bool {
    a.rederives() && b.rederives() && Predecessor::cites(&Parent::of(a), &Parent::of(b))
}

/// ONE RUN OF RECEIPTS: the same count, each pair `one_receipt`.
pub(super) fn one_run(a: &[KStepReceipt], b: &[KStepReceipt]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| one_receipt(x, y))
}
