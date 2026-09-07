// The work counters the enumeration gate reads: what one fetched vertex, one
// held thought and one meet actually cost, so a test can assert the same
// number at N and at 4N.
//
//! Incremented in test builds on Keyed get/insert, Rotor::geo, and
//! cech::extend.

use std::cell::Cell;
thread_local! {
    static GETS: Cell<u64> = const { Cell::new(0) };
    static INSERTS: Cell<u64> = const { Cell::new(0) };
    static GEOS: Cell<u64> = const { Cell::new(0) };
    static EXTENDS: Cell<u64> = const { Cell::new(0) };
}
pub fn reset() {
    GETS.with(|c| c.set(0));
    INSERTS.with(|c| c.set(0));
    GEOS.with(|c| c.set(0));
    EXTENDS.with(|c| c.set(0));
}
pub fn snapshot() -> (u64, u64, u64, u64) {
    (
        GETS.with(|c| c.get()),
        INSERTS.with(|c| c.get()),
        GEOS.with(|c| c.get()),
        EXTENDS.with(|c| c.get()),
    )
}
pub fn total() -> u64 {
    let (a, b, c, d) = snapshot();
    a.saturating_add(b).saturating_add(c).saturating_add(d)
}
pub fn count_get() {
    GETS.with(|c| c.set(c.get().saturating_add(1)));
}
pub fn count_insert() {
    INSERTS.with(|c| c.set(c.get().saturating_add(1)));
}
pub fn count_geo() {
    GEOS.with(|c| c.set(c.get().saturating_add(1)));
}
pub fn count_extend() {
    EXTENDS.with(|c| c.set(c.get().saturating_add(1)));
}
