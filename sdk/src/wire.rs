// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-PRECURSORS §4.2 — THE WIRE COMPATIBILITY STATEMENT: the wire version
// the SDK speaks and every act it may send, by the library's own codes.

//! The wire this SDK speaks.
//!
//! [`WIRE_VERSION`] is the wire's version, `major << 8 | minor`, and
//! [`ACT_TABLE`] names every act a client built on this SDK may send, each
//! by its code on the wire — the library's `ACT_*` constants, never new
//! numbers. An act a court sends a court (an offer, an install, a fold, a
//! keeper's tick) is not in it.

use mycelium::anastome::dialogue::{
    ACT_ADOPT, ACT_COVER, ACT_DIGESTS, ACT_DOMAIN, ACT_FETCH, ACT_LAWS, ACT_OBSERVE, ACT_PEERS, ACT_PROBE, ACT_QUOTE,
    ACT_RECEIPTS, ACT_ROWS, ACT_SEAT, ACT_SOLVE, ACT_SPEECH, ACT_SYLLABUS, ACT_TASK, ACT_WALLET, ACT_WHY,
};

/// The wire's version, `major << 8 | minor`: `0x0104` is v1.4.
pub use mycelium::anastome::dialogue::WIRE_VERSION;

/// EVERY ACT THE SDK MAY SEND, `(code, name)`, in code order.
pub const ACT_TABLE: &[(u8, &str)] = &[
    (ACT_QUOTE, "quote"),
    (ACT_TASK, "task"),
    (ACT_SOLVE, "solve"),
    (ACT_SYLLABUS, "syllabus"),
    (ACT_ADOPT, "adopt"),
    (ACT_DIGESTS, "digests"),
    (ACT_FETCH, "fetch"),
    (ACT_PROBE, "probe"),
    (ACT_PEERS, "peers"),
    (ACT_SPEECH, "speech"),
    (ACT_COVER, "cover"),
    (ACT_RECEIPTS, "receipts"),
    (ACT_ROWS, "rows"),
    (ACT_OBSERVE, "observe"),
    (ACT_LAWS, "laws"),
    (ACT_WHY, "why"),
    (ACT_WALLET, "wallet"),
    (ACT_DOMAIN, "domain"),
    (ACT_SEAT, "seat"),
];
