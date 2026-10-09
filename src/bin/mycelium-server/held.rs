// THE KERNEL LOCK, COUNTED (2026-10-08): who blocks whom. Every reflex
// worker waited on the kernel while the door deferred nearly every act; no
// reading said which holder kept it. A lock taken through `held!` names its
// site as the holder; an attempt that finds the lock taken charges ONE
// block to the site holding it at that moment. Exact counts, no clock
// (law 9): a site that blocks often is a site that holds long or often.
//
//! `held!(m)` stands for `m.lock()` with the same `Result` shape, so every
//! `if let Ok(..)`, `match`, `.ok()` and `.map(..)` at a site reads as before.

use mycelium::cognition::Kernel;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard, TryLockError};

/// One site's counts: how often it took the lock, how often it had to wait,
/// and how often another site waited on it.
struct Site {
    at: &'static str,
    took: u64,
    waited: u64,
    blocked: u64,
}

struct Book {
    holder: Option<&'static str>,
    /// Which hold this is: every take of the kernel numbered in turn, so two
    /// readings of one hold are told from two holds of one site.
    hold: u64,
    /// 78.152 TU-7 — the final-point advances the door's shelf had counted
    /// when this hold was taken: a hold is measured in TUniversal, never in
    /// seconds.
    advances_at: u64,
    sites: Vec<Site>,
}

static BOOK: Mutex<Book> = Mutex::new(Book { holder: None, hold: 0, advances_at: 0, sites: Vec::new() });

/// 78.152 TU-7 — THE LOCK WATCHDOG: a thread of its own that wakes on each
/// advance of the door's final point and reads the held book and the shelf,
/// never the kernel. A hold standing across an aperture
/// (`physics::aperture_cycle`) of final-point advances is named once, with
/// the step it stands in and how many advances it spans. A court whose
/// final point does not move (two keepers down) wakes it never: time does
/// not advance, and no hold is measured against it.
pub(crate) fn spawn_watchdog() {
    std::thread::spawn(|| {
        let shelf = crate::tshelf::shelf();
        let span = u64::from(mycelium::physics::aperture_cycle());
        let mut named: Option<u64> = None;
        let mut seen = shelf.advances();
        loop {
            seen = shelf.wait_advance(seen);
            let Some((site, hold, at)) = BOOK.lock().ok().and_then(|b| b.holder.map(|h| (h, b.hold, b.advances_at))) else {
                continue;
            };
            let Some(spans) = seen.checked_sub(at) else { continue };
            if spans >= span && named != Some(hold) {
                named = Some(hold);
                eprintln!("  kernel lock held {site} (hold {hold}, step {}) across {spans} final-point advances", stepping());
            }
        }
    });
}

/// THE HOLD STANDING NOW: its site and its number, or none. Read without the
/// kernel (the observer's door), so a court whose kernel one thread holds
/// past every waiter still says WHICH site holds it — hypha c 2026-10-09:
/// one thread spun 21 minutes under the kernel, thirteen waited, the journal
/// fell silent, and no reading named the site.
pub(crate) fn holding() -> Option<(&'static str, u64)> {
    let b = BOOK.lock().ok()?;
    b.holder.map(|h| (h, b.hold))
}

/// Every take of the kernel through `held!` since boot: the book's hold
/// number. A path that takes no kernel leaves it where it stood (78.152b).
#[cfg(test)]
pub(crate) fn holds() -> u64 {
    BOOK.lock().map_or(0, |b| b.hold)
}

/// THE STEP A LONG HOLD STANDS IN: a site that does several things under
/// one hold names each as it enters it, so a hold that stands across the
/// observer's questions says which step it stands in (hypha a, 2026-10-09:
/// the court loop held 20+ minutes, the site alone could not say whether in
/// the stroke or in the publish).
/// One marker for the server and the library (`observe::step`), so the
/// court stroke names its own steps inside the bin's.
pub(crate) fn step(name: &'static str) {
    mycelium::observe::step(name);
}

pub(crate) fn stepping() -> &'static str {
    mycelium::observe::stepping()
}

/// Threads blocked on the kernel through `held!` right now, and how many
/// such waits have been served since boot.
static WAITING: AtomicU64 = AtomicU64::new(0);
static SERVED: AtomicU64 = AtomicU64::new(0);

/// THE COURT LETS THE QUEUE IN BEFORE IT TAKES THE KERNEL AGAIN. The court
/// loop re-took the kernel the instant it let go — 14 806 times in five
/// minutes on hypha c — and a std mutex is not fair, so a reflex worker
/// woken by the release lost the race every time: the workers starved, the
/// pool filled, and the door deferred nearly every act (2026-10-08). Each
/// wait queued when this is called is served once before it returns; one
/// that queues after waits its turn behind the next stroke. Counts, no
/// clock.
pub(crate) fn yield_to_waiters() {
    let queued = WAITING.load(Ordering::Acquire);
    if queued == 0 {
        return;
    }
    // Past 2⁶⁴ served waits there is no turn to count to, and none is waited.
    let Some(until) = SERVED.load(Ordering::Acquire).checked_add(queued) else { return };
    while SERVED.load(Ordering::Acquire) < until && WAITING.load(Ordering::Acquire) > 0 {
        std::thread::yield_now();
    }
}

fn with_site(book: &mut Book, at: &'static str, f: impl FnOnce(&mut Site)) {
    if let Some(s) = book.sites.iter_mut().find(|s| s.at == at) {
        f(s);
        return;
    }
    let mut s = Site { at, took: 0, waited: 0, blocked: 0 };
    f(&mut s);
    book.sites.push(s);
}

/// The kernel, held by one named site.
pub(crate) struct Held<'a> {
    guard: MutexGuard<'a, Kernel>,
}

impl Deref for Held<'_> {
    type Target = Kernel;
    fn deref(&self) -> &Kernel {
        &self.guard
    }
}

impl DerefMut for Held<'_> {
    fn deref_mut(&mut self) -> &mut Kernel {
        &mut self.guard
    }
}

impl Drop for Held<'_> {
    fn drop(&mut self) {
        if let Ok(mut b) = BOOK.lock() {
            b.holder = None;
        }
    }
}

/// Take the kernel at site `at`; `Err(())` where the lock is poisoned, as
/// `lock()` refuses.
pub(crate) fn lock<'a>(m: &'a Mutex<Kernel>, at: &'static str) -> Result<Held<'a>, ()> {
    let (guard, waited) = match m.try_lock() {
        Ok(g) => (g, false),
        Err(TryLockError::Poisoned(_)) => return Err(()),
        Err(TryLockError::WouldBlock) => {
            if let Ok(mut b) = BOOK.lock() {
                let blocker = b.holder;
                if let Some(h) = blocker {
                    with_site(&mut b, h, |s| {
                        s.blocked += 1;
                        if s.blocked.is_power_of_two() {
                            eprintln!("  kernel lock {} blocked others {} (took {} waited {})", s.at, s.blocked, s.took, s.waited);
                        }
                    });
                }
            }
            WAITING.fetch_add(1, Ordering::AcqRel);
            let g = m.lock();
            WAITING.fetch_sub(1, Ordering::AcqRel);
            SERVED.fetch_add(1, Ordering::AcqRel);
            (g.map_err(|_| ())?, true)
        }
    };
    if let Ok(mut b) = BOOK.lock() {
        b.holder = Some(at);
        b.advances_at = crate::tshelf::shelf().advances();
        // A hold numbered past 2⁶⁴ takes is not numbered again; the site
        // still names it.
        if let Some(n) = b.hold.checked_add(1) {
            b.hold = n;
        }
        with_site(&mut b, at, |s| {
            s.took += 1;
            if waited {
                s.waited += 1;
            }
        });
    }
    Ok(Held { guard })
}

/// `held!(kernel)` — `kernel.lock()` counted at this file and line.
macro_rules! held {
    ($m:expr) => {
        $crate::held::lock(&$m, concat!(file!(), ":", line!()))
    };
}
pub(crate) use held;
