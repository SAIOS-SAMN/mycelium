// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ECONOMY — THE DIMENSIONAL LEDGER (alpha Step 4). Every balance and every
//! receipt is a polytonic μ⃗ 4-vector (𝒮, 𝒢, 𝒯, 𝒽) — scalar amounts are
//! BANNED (a scalar balance is a flattened ledger). Every entry is ANCHORED to
//! the chronometric chain (K-step + head hash — the unforgeable temporal
//! binding; Step 5's court audits these). The treasury is a QUOTING ENGINE:
//! yield follows the surface (pressure + reserve deficit + entropic excess) and
//! the cohesion pair (C, I) differentiates the market — closure work prices by
//! C, coverage work by I, never summed. PoUW = paid decomposition of real open
//! demand (settle_use); PoWc = the ≺_DM descent gate + consilience receipts
//! sealed on the chain. Torsion extraction (torse) is the strongest coin —
//! structure condensed from disorder. Redundancy is structurally suppressed by
//! puncture/condense (same-geometry siblings cannot coexist as cards), which is
//! the anti-Goodhart inverse-redundancy law realized in the carrier itself.

//!
//! Five organs (Stage 8b S12): `lanes.rs` what a word measures and how two
//! measurements compare (law 5); `treasury.rs` who is bonded and what the
//! audit says (E1); `escrow.rs` value set aside against a listing (E1, E2);
//! `settle.rs` what each closed act pays (E2, 26); `persist.rs` the ledger
//! on disk (28).

pub mod escrow;
pub mod lanes;
pub mod persist;
pub mod settle;
pub mod treasury;

pub use lanes::*;
pub use treasury::*;

use crate::transducer::element::Element;

#[cfg(test)]
mod tests;
