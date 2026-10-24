// §8.1, audit of Phase N — THE OBSERVER: what the live fleet reads about
// itself, held as whole typed readings and served to a diagnostic console.
//
//! The fleet printed its instruments as prose and dropped the counters
//! behind them; a question about a running voice was answered by reading
//! its log. This module holds the readings themselves — the allocator over
//! the source's own clock (what is held), and a court's standing — keyed
//! by source and kind, so a hypha can answer for itself and any hypha can
//! run the console (`mycelium-server --console`). A mind answers for itself
//! on its own post (`Mind`), read straight from the running binary.

mod acts;
mod carriage;
mod carriers;
mod clock;
mod dispatch;
mod judged;
mod economy;
mod fleet;
pub mod live;
mod mind;
mod reading;
mod relay;
mod store;
#[cfg(test)]
mod tests;
mod tuniversal;

pub use acts::*;
pub use carriage::*;
pub use carriers::*;
pub use clock::*;
pub use dispatch::*;
pub use judged::*;
pub use economy::*;
pub use fleet::*;
pub use mind::*;
pub use reading::*;
pub use relay::*;
pub use store::*;
pub use tuniversal::*;

/// THE STEP A LONG HOLD STANDS IN, named by the code that takes it (the
/// court stroke's own steps): read by the server's observer door when one
/// hold of the kernel stands across two of its questions (2026-10-09:
/// hypha a's court stroke held the kernel 20+ minutes in one pass, and the
/// site alone could not say where).
static STEP: std::sync::Mutex<&'static str> = std::sync::Mutex::new("");

/// Name the step now entered.
pub fn step(name: &'static str) {
    if let Ok(mut s) = STEP.lock() {
        *s = name;
    }
}

/// The step last entered.
pub fn stepping() -> &'static str {
    match STEP.lock() {
        Ok(s) => *s,
        Err(_) => "",
    }
}
