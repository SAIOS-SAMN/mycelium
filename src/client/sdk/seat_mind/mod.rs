// 78.146 — THE MIND AT ITS SEAT: what a seated mind perceives of its game and
// remembers of it, read off the house's typed exact records, and (in
// `choice`) the call it chooses from what its own calls were seen to do.
//
//! A PERCEPT IS A SERIES, NEVER A PICTURE. Each record of a percept is
//! located by its own coordinates — its kind, the keys it carries (the
//! house's own names for what it is about), its occurrence among the records
//! sharing both, and the argument's position — and every non-key argument is
//! one SERIES over the house's percept sequence (the seat's beats) and its
//! local time. Nothing is painted into sites it is not.
//!
//! A QUANTITY'S CHANGE IS A MOTION OF ITS COORDINATE. A Q, a BigQ or a Count
//! is a point of its own line ℚ¹, and the change between two beats is held as
//! the motions of that line that carry one value to the other, exact:
//! - THE SLIDE by `Δ = to − from`, the translator's point on its null flow
//!   (`translator_at`, read on the plane `translator_at([1])` itself, so
//!   `t = Δ` and two slides compose by addition);
//! - THE SCALE by `ρ = to / from` where both stand on one side of zero, the
//!   dilator's point on its boost flow (`dilator`, read on `dilator(2)`'s
//!   plane, `t = (ρ − 1)/(ρ + 1)`, so two scales compose by product).
//!
//! A multivector argument is held as the geometric object it is — a full
//! Cl(4,1) versor, odd ones (reflections) included; a flag as itself. Two
//! values are one where they write one run of bytes (`Arg ==`), so a
//! multivector is compared coefficient for coefficient and never only
//! projectively.
//!
//! 78.148 — A VERSOR'S CHANGE IS THE VERSOR THAT CARRIES IT (the field
//! law): between consecutive beats the change of a multivector series
//! is `c = v₊ · ṽ / ⟨v ṽ⟩₀`, the exact carrier with `c · v = v₊` — divided
//! by the earlier value's weight (`weight_of`, read), so it is normalised
//! exactly — or REFUSED where either value is no versor or the weight is
//! zero (`carrier`). It is never flattened to numbers.
//!
//! AN ORBIT IS A RETURN, READ. A series returns with period `p` where its
//! last `2p` changes repeat with period `p` and the slides of one period
//! compose to the flow's identity (`FlowPoint::returned`); a multivector's
//! where its last `2p` CARRIERS repeat with period `p` (a steady carrier is
//! the orbit of period one; a carrier that is the scalar one, a value that
//! stands still, is fixed); a flag's — or a multivector series whose
//! carrier was refused — where its last `2p` values repeat; period 1 is a
//! fixed value. A record's period is the lcm of its arguments' (time is distance
//! along an orbit, 78.43), and the period's span in the house's local time is
//! read off the two beats, never assumed.
//!
//! BOUNDED AS THE RELAY IS (a sink keeps its bound): a SEAT holds what one
//! frame could carry of its values as they write (`frame_budget`), shared
//! among its series, each series' oldest dropped and counted past its share;
//! changes and carriers are derived where read, never held (Concordance
//! 1.13). OWED, named: a series whose locator the house stops sending keeps
//! its last share until the seat sheds it, and the hand-over of a recognised
//! orbit to the contemplation as a form.

mod change;
mod choice;
mod mind;
mod own;
mod perceived;
mod series;
pub mod task;

pub use change::*;
pub use choice::*;
pub use mind::*;
pub use own::*;
pub use perceived::*;
pub use series::*;
