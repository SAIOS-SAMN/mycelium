// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 28 — PERSISTENCE: the MYC sections a mind is written to, section by
//! section, and read back from, with everything derived re-derived rather
//! than restored.

pub mod read;
pub mod write;

/// THE LEDGER SECTION (78.24, 78.23 as amended): the ledger as its trie,
/// every stamp `h‖vk‖K‖i`, a fold's root at an orbit's close.
pub const LEDGER_TAG: &[u8; 4] = b"LDG2";
/// THE SETTLEMENTS SECTION: each settlement at its place `K‖i`.
pub const SETTLEMENTS_TAG: &[u8; 4] = b"SET2";
/// THE TWO SECTIONS BEFORE THE ORBIT COORDINATES (ACT1-orbit), refused BY
/// NAME and never migrated, as [`crate::transducer::element::Element::PRE_WHOLE_MAGIC`]
/// is: their stamps carry no place in the orbit, so under this reader a
/// stamp's height would be read into the next stamp's orbit. One equality
/// refuses each; there is no reader for them (§10: the recut is the road).
pub const PRE_ORBIT_LEDGER_TAG: &[u8; 4] = b"LDG1";
pub const PRE_ORBIT_SETTLEMENTS_TAG: &[u8; 4] = b"SET1";
