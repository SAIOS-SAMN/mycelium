// THE COURT LOCK, COUNTED (2026-10-08) AND METERED (78.153b). The book and
// the guard are the library's (`mycelium::server::held`), so the library's
// own holds — the synthesis thread, the back-fill — are counted with the
// binary's; this file is the binary's face on it: the watchdog on the door's
// shelf and the `held!` the sites write.
//
//! `held!(m)` stands for `m.lock()` with the same `Result` shape, so every
//! `if let Ok(..)`, `match`, `.ok()` and `.map(..)` at a site reads as before
//! — and what it holds is a `Held`, whose only way in is `Held::run(|k,
//! reads| …)`: the court and the hold's one stroke together.

pub(crate) use mycelium::server::held::{holding, stepping, yield_to_waiters};
use mycelium::server::held::Held;
use mycelium::cognition::Court;
use std::sync::Mutex;

/// 78.152 TU-7 — THE LOCK WATCHDOG: a thread of its own that wakes on each
/// advance of the door's final point and reads the held book and the shelf,
/// never the court. A hold standing across an aperture
/// (`physics::aperture_cycle`) of final-point advances is named once, with
/// the step it stands in and how many advances it spans. A court whose
/// final point does not move (two keepers down) wakes it never: time does
/// not advance, and no hold is measured against it.
pub(crate) fn spawn_watchdog() {
    // The book stamps each hold with the shelf's advances at its take.
    mycelium::server::held::read_advances_with(|| crate::tshelf::shelf().advances());
    std::thread::spawn(|| {
        let shelf = crate::tshelf::shelf();
        let span = u64::from(mycelium::physics::aperture_cycle());
        let mut named: Option<u64> = None;
        let mut seen = shelf.advances();
        loop {
            seen = shelf.wait_advance(seen);
            let Some((site, hold, at)) = mycelium::server::held::holding_since() else {
                continue;
            };
            let Some(spans) = seen.checked_sub(at) else { continue };
            if spans >= span && named != Some(hold) {
                named = Some(hold);
                eprintln!("  court lock held {site} (hold {hold}, step {}) across {spans} final-point advances", stepping());
            }
        }
    });
}

/// The takes of the court THIS thread made through `held!` (78.152b): a
/// path that takes no court leaves it where it stood, read apart from the
/// holds other threads of the process make beside it.
#[cfg(test)]
pub(crate) fn holds_here() -> u64 {
    mycelium::server::held::holds_here()
}

/// THE STEP A LONG HOLD STANDS IN (`mycelium::server::held::step`).
pub(crate) fn step(name: &'static str) {
    mycelium::server::held::step(name);
}

/// Take the court at site `at`, with the hold's one stroke; `Err(())` where
/// the lock is poisoned, as `lock()` refuses.
pub(crate) fn lock<'a>(m: &'a Mutex<Court>, at: &'static str) -> Result<Held<'a>, ()> {
    mycelium::server::held::lock(m, at)
}

/// `held!(court)` — `court.lock()` counted at this file and line.
macro_rules! held {
    ($m:expr) => {
        $crate::held::lock(&$m, concat!(file!(), ":", line!()))
    };
}
pub(crate) use held;
