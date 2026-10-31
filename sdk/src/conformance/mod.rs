// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-PRECURSORS §4 P5, MU-CONFORMANCE — THE CLIENT CONFORMANCE READ: the
// statement a client pins to, its codec checks in process, and its suite
// against a running network.

//! The client conformance read.
//!
//! [`STATEMENT`] names every act a client speaks to a hypha: its code on the
//! wire (an `ACT_TABLE` code), its sub-mode where it has one, whether a
//! client MUST or MAY speak it, the decoder its reply is read by, and the
//! exact fields that decoder reads back. The MUST acts are the client loop:
//! take a task, solve it and read the verdict, read the law register and its
//! version. A hypha serves the same statement on `ACT_LAWS` mode 6
//! ([`encode_statement_request`], [`decode_statement_reply`]).
//!
//! Two reads check a client against it, both by decoded exact values and
//! never by golden bytes (Concordance 1.4):
//!
//! - [`check_codecs`], in process: a client is the set of acts it speaks;
//!   each act it speaks is encoded and decoded at both ends, and each MUST
//!   act it does not speak fails by name;
//! - [`run`], against a running network: each MUST act spoken to a hypha
//!   and its reply decoded and checked, each failure named.
//!
//! A client is aligned with a release where both pass at that release's tag.

mod codecs;
mod live;
mod report;

pub use codecs::check_codecs;
pub use live::run;
pub use report::{Finding, Report};

/// The statement, its acts, the need of each, and the statement as read off
/// the wire.
pub use mycelium::anastome::dialogue::conformance::{ActRead, ActRequirement, Need, Statement, StatementRead, STATEMENT};

/// The statement's codec: the bytes a hypha serves, and the reader of them.
pub use mycelium::anastome::dialogue::conformance::{decode_statement_reply, encode_statement};

/// The request for the statement, `[ACT_LAWS][6]`.
pub use mycelium::physics::laws::codec::encode_statement_request;
