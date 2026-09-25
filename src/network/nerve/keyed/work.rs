// Rule 13 — THE WORK COUNTERS: what one fetched vertex, one held thought
// and one meet actually cost.
//
//! Incremented in test builds on Keyed get/insert, Rotor::geo, and
//! cech::extend.

use std::cell::Cell;
thread_local! {
    static GETS: Cell<u64> = const { Cell::new(0) };
    static INSERTS: Cell<u64> = const { Cell::new(0) };
    static GEOS: Cell<u64> = const { Cell::new(0) };
    static EXTENDS: Cell<u64> = const { Cell::new(0) };
    static VISITS: Cell<u64> = const { Cell::new(0) };
}
pub fn reset() {
    GETS.with(|c| c.set(0));
    INSERTS.with(|c| c.set(0));
    GEOS.with(|c| c.set(0));
    EXTENDS.with(|c| c.set(0));
    VISITS.with(|c| c.set(0));
}
pub fn snapshot() -> (u64, u64, u64, u64) {
    (
        GETS.with(|c| c.get()),
        INSERTS.with(|c| c.get()),
        GEOS.with(|c| c.get()),
        EXTENDS.with(|c| c.get()),
    )
}
#[expect(
    clippy::disallowed_methods,
    reason = "rule 11 — an instrument's total (§8.1), read by no cognition and no price: four counters of one thread, each one per event, cannot sum past u64 before 2⁶² events each"
)]
pub fn total() -> u64 {
    let (a, b, c, d) = snapshot();
    a.saturating_add(b).saturating_add(c).saturating_add(d)
}
#[expect(clippy::disallowed_methods, reason = "rule 11 — one per event in one process: a u64 cannot pass 2⁶⁴ events before the process ends")]
pub fn count_get() {
    GETS.with(|c| c.set(c.get().saturating_add(1)));
}
#[expect(clippy::disallowed_methods, reason = "rule 11 — one per event in one process: a u64 cannot pass 2⁶⁴ events before the process ends")]
pub fn count_insert() {
    INSERTS.with(|c| c.set(c.get().saturating_add(1)));
}
#[expect(clippy::disallowed_methods, reason = "rule 11 — one per event in one process: a u64 cannot pass 2⁶⁴ events before the process ends")]
pub fn count_geo() {
    GEOS.with(|c| c.set(c.get().saturating_add(1)));
}
#[expect(clippy::disallowed_methods, reason = "rule 11 — one per event in one process: a u64 cannot pass 2⁶⁴ events before the process ends")]
pub fn count_extend() {
    EXTENDS.with(|c| c.set(c.get().saturating_add(1)));
}

/// TRIE NODES A WALK HAS VISITED. Its own reading and not part of
/// [`total`]: that is the count of keyed READS AND WRITES, one per `get`
/// or `insert`, and a walk that steps through a node takes neither. What
/// this measures is the EXTENT of a fiber — how much of the carrier the
/// walk touched to yield what it yielded — which is the quantity rule 1
/// is about and the one nothing counted.
pub fn visits() -> u64 {
    VISITS.with(|c| c.get())
}
#[expect(clippy::disallowed_methods, reason = "rule 11 — one per event in one process: a u64 cannot pass 2⁶⁴ events before the process ends")]
pub fn count_visit() {
    VISITS.with(|c| c.set(c.get().saturating_add(1)));
}
