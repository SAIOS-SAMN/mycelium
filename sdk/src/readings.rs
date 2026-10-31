// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// §8.1, 78.62 — THE READINGS A CLIENT ASKS A HYPHA FOR: observations, each
// typed, none moving the book.

//! The typed readings a client may ask a hypha for.
//!
//! Each is an observation: it measures and moves nothing. The request is
//! encoded here and spoken with [`Session::speak`](crate::session::Session::speak);
//! the reply is read by its decoder.
//!
//! - the observer's readings, by prefix ([`encode_query`], [`decode_reply`]);
//! - why a word or act was ruled as it was ([`encode_why`], [`decode_why_reply`]);
//! - the law register's parameters, core clock and lexicon;
//! - a court's receipts and the economy's rows over a span;
//! - a domain's reserve ([`ReserveReading`], also `Session::reserve_reading`);
//! - the probe's reading of a target.

/// The observer's readings: the query by prefix, the reply, and the typed
/// reading each entry holds.
pub use mycelium::observe::{decode_reply, encode_query, Held, Reading};

/// Why: the ask, the answer, and the codec of each.
pub use mycelium::anastome::dialogue::why::{decode_why_reply, encode_why, Why, WhyAsk};

/// The law register: parameters, core clock and lexicon.
pub use mycelium::physics::laws::codec::{
    decode_clock_reply, decode_lexicon_reply, decode_parameters_reply, encode_clock_request, encode_lexicon_request,
    encode_parameters_request, ServedTerm,
};
/// The vocabulary a lexicon term belongs to.
pub use mycelium::physics::laws::lexicon::Vocabulary;

/// A court's receipts over a span, and the place a span is named by.
pub use mycelium::anastome::dialogue::{decode_receipts_reply, encode_receipts_request};
/// A place on a court's chain.
pub use mycelium::chronometry::Place;

/// The economy's rows over a span.
pub use mycelium::anastome::dialogue::{decode_rows_reply, encode_rows_request};

/// A domain's reserve, as the hypha reads it.
pub use mycelium::anastome::dialogue::domain::ReserveReading;

/// The probe's reading of a target.
pub use mycelium::anastome::dialogue::ProbeReading;
