// §8.1, audit of Phase N — THE OBSERVER: what the live fleet reads about
// itself, held as whole typed readings and served to a diagnostic console.
//
//! The fleet printed its instruments as prose and dropped the counters
//! behind them; a question about a running voice was answered by reading
//! its log. This module holds the readings themselves — the plane census in
//! its algebra (which dimensions are used and which are flat), every
//! refusal that passes a reading over (what is lost), the allocator over
//! the source's own clock (what is held), and a court's standing — keyed
//! by source and kind, so a hypha can answer for itself and for its voices
//! and any hypha can run the console (`mycelium-server --console`).

mod carriage;
mod cells;
mod circuit;
mod clock;
mod fleet;
mod learning;
mod reading;
mod store;
#[cfg(test)]
mod tests;

pub use carriage::*;
pub use cells::*;
pub use circuit::*;
pub use clock::*;
pub use fleet::*;
pub use learning::*;
pub use reading::*;
pub use store::*;
