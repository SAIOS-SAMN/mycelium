// MU-CLIENT §4.2 — THE MIND'S OWN FIELD: what a mind's perception holds from
// one tick to the next, owned by the mind and by nothing else.
//
//! A MIND IS NOT A PROCESS. Its tubes and orbits (78.71 (1)), the lanes its
//! contacts and flux moved, the shortcuts it holds and the words newly taught
//! to it, and the covers it parked part-way are its own; two minds in one
//! process keep two fields, and neither perceives through the other's.
//!
//! THE FIELD IS SCOPED, NEVER SHARED. The mind owns a [`MindField`]; for the
//! length of its tick ([`MindField::think`]) the field stands in this
//! thread's slots, so the perception deep in the transducer writes into the
//! mind that is thinking without a field threaded through every reader, and
//! leaves it when the tick ends. Outside a mind's tick — a court perceiving
//! on its worker threads — the readers fall through to the state that court
//! shares across its threads (`pairing`, `rung`, `cursor`).

use crate::network::nerve::Keyed;
use crate::transducer::cech::orbit::Field;
use crate::transducer::family::cursor::Pending;
use crate::transducer::sentence::rung::{Lexicon, Taught};
use std::cell::RefCell;

/// THE LANES A MIND'S CONTACTS AND FLUX MOVED, per ledger (`pairing::Ledger`:
/// condensed, refused, relieved, transported) and per lane, since the mind
/// began.
pub(crate) type Lanes = [[u64; 4]; 4];

/// ONE MIND'S FIELD: its tubes and orbits, its lanes, its lexicon, the words
/// taught it and not yet transcribed, and the covers it parked.
#[derive(Default)]
pub struct MindField {
    field: Field,
    lanes: Lanes,
    lexicon: Lexicon,
    taught: Vec<Taught>,
    pending: Keyed<Pending>,
}

thread_local! {
    static FIELD: RefCell<Option<Field>> = const { RefCell::new(None) };
    static LANES: RefCell<Option<Lanes>> = const { RefCell::new(None) };
    static LEXICON: RefCell<Option<Lexicon>> = const { RefCell::new(None) };
    static TAUGHT: RefCell<Option<Vec<Taught>>> = const { RefCell::new(None) };
    static PENDING: RefCell<Option<Keyed<Pending>>> = const { RefCell::new(None) };
}

/// The field taken back out of this thread's slots when the tick ends —
/// on a panic too, so a mind's field never outlives its tick on the thread.
struct Leave<'a>(&'a mut MindField);

impl Drop for Leave<'_> {
    fn drop(&mut self) {
        // Every slot was filled at `think`, and only the leave empties it.
        let m = &mut *self.0;
        if let Some(x) = FIELD.with(|c| c.borrow_mut().take()) {
            m.field = x;
        }
        if let Some(x) = LANES.with(|c| c.borrow_mut().take()) {
            m.lanes = x;
        }
        if let Some(x) = LEXICON.with(|c| c.borrow_mut().take()) {
            m.lexicon = x;
        }
        if let Some(x) = TAUGHT.with(|c| c.borrow_mut().take()) {
            m.taught = x;
        }
        if let Some(x) = PENDING.with(|c| c.borrow_mut().take()) {
            m.pending = x;
        }
    }
}

impl MindField {
    /// THE MIND THINKS IN ITS OWN FIELD: `f` runs with this field standing
    /// in the thread's slots, and the field is the mind's again after.
    pub fn think<R>(&mut self, f: impl FnOnce() -> R) -> R {
        FIELD.with(|c| *c.borrow_mut() = Some(std::mem::take(&mut self.field)));
        LANES.with(|c| *c.borrow_mut() = Some(self.lanes));
        LEXICON.with(|c| *c.borrow_mut() = Some(std::mem::take(&mut self.lexicon)));
        TAUGHT.with(|c| *c.borrow_mut() = Some(std::mem::take(&mut self.taught)));
        PENDING.with(|c| *c.borrow_mut() = Some(std::mem::take(&mut self.pending)));
        let _leave = Leave(self);
        f()
    }
}

/// `f` on the thinking mind's part in `slot`; outside a mind's tick `f` is
/// handed back, for the caller's process state.
fn with<T: 'static, R, F: FnOnce(&mut T) -> R>(
    slot: &'static std::thread::LocalKey<RefCell<Option<T>>>,
    f: F,
) -> Result<R, F> {
    slot.with(|c| match c.borrow_mut().as_mut() {
        Some(part) => Ok(f(part)),
        None => Err(f),
    })
}

/// The thinking mind's tubes and orbits.
pub(crate) fn with_field<R, F: FnOnce(&mut Field) -> R>(f: F) -> Result<R, F> {
    with(&FIELD, f)
}

/// The thinking mind's lanes.
pub(crate) fn with_lanes<R, F: FnOnce(&mut Lanes) -> R>(f: F) -> Result<R, F> {
    with(&LANES, f)
}

/// The thinking mind's lexicon.
pub(crate) fn with_lexicon<R, F: FnOnce(&mut Lexicon) -> R>(f: F) -> Result<R, F> {
    with(&LEXICON, f)
}

/// The words taught the thinking mind.
pub(crate) fn with_taught<R, F: FnOnce(&mut Vec<Taught>) -> R>(f: F) -> Result<R, F> {
    with(&TAUGHT, f)
}

/// The covers the thinking mind parked.
pub(crate) fn with_pending<R, F: FnOnce(&mut Keyed<Pending>) -> R>(f: F) -> Result<R, F> {
    with(&PENDING, f)
}
