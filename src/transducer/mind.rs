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
///
/// MU-CLIENT §4.2, §7.1 R1 — A SCRATCH FIELD is the field as it stood when a
/// carrier took a scene to perceive OFF the mind's field (`scratch`): the
/// long law reads it and moves it as perception always does, and every flux
/// it admitted is RECORDED, so that what the mind's field receives is the
/// law's ADMISSION (`into_admission`, then `admit` under a brief hold) — the
/// flux landing on the field as it stands then, the lanes booked outside the
/// flux, the words taught, the lexicon's new words and the covers parked.
#[derive(Default)]
pub struct MindField {
    field: Field,
    lanes: Lanes,
    lexicon: Lexicon,
    taught: Vec<Taught>,
    pending: Keyed<Pending>,
    recording: Option<Recorded>,
}

/// ONE ADMISSION OF FLUX: the circuits with their masses, and the lines.
pub(crate) type Flux = (Vec<(crate::transducer::element::Versor, u64)>, Vec<crate::transducer::family::pairing::FieldLine>);

/// WHAT A SCRATCH FIELD'S FLUX WAS: each admission's circuits and lines, in
/// the order admitted, and the ledgers those admissions booked.
#[derive(Default)]
pub(crate) struct Recorded {
    pub(crate) flux: Vec<Flux>,
    pub(crate) booked: Lanes,
}

/// A LONG LAW'S ADMISSION into the mind's field (R1): its flux to land, the
/// lanes it booked outside its flux, the words it taught, the lexicon it
/// read and grew, and the covers it parked.
pub struct Admission {
    recorded: Recorded,
    lanes: Lanes,
    taught: Vec<Taught>,
    lexicon: Lexicon,
    pending: Keyed<Pending>,
}

thread_local! {
    static FIELD: RefCell<Option<Field>> = const { RefCell::new(None) };
    static LANES: RefCell<Option<Lanes>> = const { RefCell::new(None) };
    static LEXICON: RefCell<Option<Lexicon>> = const { RefCell::new(None) };
    static TAUGHT: RefCell<Option<Vec<Taught>>> = const { RefCell::new(None) };
    static PENDING: RefCell<Option<Keyed<Pending>>> = const { RefCell::new(None) };
    static RECORDING: RefCell<Option<Recorded>> = const { RefCell::new(None) };
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
        if let Some(x) = RECORDING.with(|c| c.borrow_mut().take()) {
            m.recording = Some(x);
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
        RECORDING.with(|c| *c.borrow_mut() = self.recording.take());
        let _leave = Leave(self);
        f()
    }

    /// WHAT A SCRATCH FIELD'S LAW COMES TO, for the mind's field to admit:
    /// its recorded flux, the lanes it booked beyond `from` (the lanes it
    /// began at) less what its flux booked — the replay books the flux again
    /// where it lands — its taught words, lexicon and parked covers. `None`
    /// for a field that was no scratch.
    pub(crate) fn into_admission(self, from: &Lanes) -> Option<Admission> {
        let recorded = self.recording?;
        let mut lanes: Lanes = [[0; 4]; 4];
        for (l, row) in lanes.iter_mut().enumerate() {
            for (lane, slot) in row.iter_mut().enumerate() {
                let moved = self.lanes[l][lane].checked_sub(from[l][lane])?;
                *slot = moved.checked_sub(recorded.booked[l][lane])?;
            }
        }
        Some(Admission { recorded, lanes, taught: self.taught, lexicon: self.lexicon, pending: self.pending })
    }
}

/// THE ADMISSION OF A LONG LAW, LANDED ON THE THINKING MIND'S FIELD (R1):
/// its flux admitted again on the field as it stands now (`mark_flux`, which
/// books what it moved), its other lanes added, its taught words handed on,
/// the lexicon's words it learned and the covers it parked joined to the
/// mind's — a word or a cover the mind already holds is the mind's. Answers
/// `false` where no mind is thinking on this thread, and nothing landed.
pub(crate) fn admit(a: Admission, budget: &mut crate::physics::Budget) -> bool {
    if with(&FIELD, |_| ()).is_err() {
        return false;
    }
    for (circuits, lines) in &a.recorded.flux {
        let _ = crate::transducer::family::pairing::mark_flux(circuits, lines, budget);
    }
    let _ = with(&LANES, |mine| {
        for (row, add) in mine.iter_mut().zip(&a.lanes) {
            for (slot, x) in row.iter_mut().zip(add) {
                let _ = crate::physics::metabolism::step_or_refuse(slot, *x);
            }
        }
    });
    let _ = with(&TAUGHT, |mine| mine.extend(a.taught));
    let _ = with(&LEXICON, |mine| mine.absorb(a.lexicon));
    let mut parked = a.pending;
    let keys: Vec<Vec<u8>> = parked.scoped().map(|(k, _)| k).collect();
    let _ = with(&PENDING, |mine| {
        for k in keys {
            if let (false, Some(p)) = (mine.contains(&k), parked.remove(&k)) {
                mine.insert(&k, p);
            }
        }
    });
    true
}

/// A SCRATCH OF THE FIELD THINKING ON THIS THREAD (R1): its tubes and
/// orbits, lanes and lexicon as they stand in the thread's slots, with the
/// lanes it begins at; a scratch of no field where none is thinking here.
pub(crate) fn scratch_here() -> (MindField, Lanes) {
    let field = match with(&FIELD, |f| f.clone()) {
        Ok(f) => f,
        Err(_) => Field::default(),
    };
    let lanes = match with(&LANES, |l| *l) {
        Ok(l) => l,
        Err(_) => [[0; 4]; 4],
    };
    let lexicon = match with(&LEXICON, |l| l.clone()) {
        Ok(l) => l,
        Err(_) => Lexicon::default(),
    };
    let scratch = MindField { field, lanes, lexicon, taught: Vec::new(), pending: Keyed::new(), recording: Some(Recorded::default()) };
    (scratch, lanes)
}

/// A SCRATCH FIELD'S FLUX, RECORDED where it is admitted (`mark_flux`): the
/// circuits and lines, and the ledgers they booked. Nothing where no scratch
/// is thinking on this thread.
pub(crate) fn record_flux(
    circuits: &[(crate::transducer::element::Versor, u64)],
    lines: &[crate::transducer::family::pairing::FieldLine],
    booked: &Lanes,
) {
    RECORDING.with(|c| {
        if let Some(r) = c.borrow_mut().as_mut() {
            r.flux.push((circuits.to_vec(), lines.to_vec()));
            for (row, add) in r.booked.iter_mut().zip(booked) {
                for (slot, x) in row.iter_mut().zip(add) {
                    let _ = crate::physics::metabolism::step_or_refuse(slot, *x);
                }
            }
        }
    });
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
