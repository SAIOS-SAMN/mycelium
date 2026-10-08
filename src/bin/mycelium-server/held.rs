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
    sites: Vec<Site>,
}

static BOOK: Mutex<Book> = Mutex::new(Book { holder: None, sites: Vec::new() });

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
            (m.lock().map_err(|_| ())?, true)
        }
    };
    if let Ok(mut b) = BOOK.lock() {
        b.holder = Some(at);
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
