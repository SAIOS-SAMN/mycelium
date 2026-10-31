// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-PRECURSORS §4 P5, MU-CONFORMANCE — THE CLIENT CONFORMANCE READ: the
// statement a client pins to, by profile, its codec checks in process, and
// its suite against a running network.

//! The client conformance read.
//!
//! [`STATEMENT`] names every act a client speaks to a hypha, in two
//! profiles: [`Profile::Mind`] (a mind's loop) and [`Profile::Kernel`] (a
//! kernel's uplink, its house). For each act: its code on the wire (an
//! `ACT_TABLE` code), its sub-mode where it has one, whether a client of that
//! profile MUST or MAY speak it, the decoder its reply is read by, and the
//! exact fields that decoder reads back. A mind's MUST acts are its loop:
//! take a task, solve it and read the verdict, read the law register. A
//! kernel's are its uplink: the register, the statement, the Concordance's
//! hash and a signature over it, the clock, a transition verified, a domain
//! founded, its manifest granted or refused by name, its universe read; task
//! and solve are MAY for a kernel, spoken for the minds it seats. A hypha
//! serves the same statement, both profiles, on `ACT_LAWS` mode 6
//! ([`encode_statement_request`], [`decode_statement_reply`]).
//!
//! A client declares its profiles, and is held to the MUST acts of every
//! profile it declares ([`Statement::union`]). Two reads check it, both by
//! decoded exact values and never by golden bytes (Concordance 1.4):
//!
//! - [`check_codecs`], in process: a client is the set of acts it speaks;
//!   each act it speaks is encoded and decoded at both ends, and each MUST
//!   act it does not speak fails by name;
//! - [`run`], against a running network: each MUST act spoken to a hypha
//!   and its reply decoded and checked, each failure named.
//!
//! A client is aligned with a release where both pass at that release's tag.
//!
//! ## The deferral
//!
//! Every profile MUST recognise the [deferral](crate::deferral): the one
//! reply a hypha whose worker pool is full gives every act. [`check_codecs`]
//! checks it as the finding [`DEFERRAL`]: a deferral reads back with its
//! exact deficit, and no MUST act's reply decoder of a declared profile
//! accepts its bytes. In [`run`], an act the hypha defers is reported as
//! [`Finding::Deferred`] with the deficit the door stated, never as a reply
//! that did not decode. The suite does not retry it: the SDK's session
//! exposes no measured round trip to pace a retry by, so the deferral is
//! reported and the suite is run again once the door has room.

mod codecs;
mod live;
mod probes;
mod report;
mod speaker;
mod uplink;

pub use codecs::check_codecs;
pub use live::run;
pub use report::{Finding, Report};

/// The name of the deferral's finding: MUST for every profile, checked in
/// process by [`check_codecs`].
pub const DEFERRAL: &str = "deferral";

/// The statement, its profiles and acts, the need of each, and the
/// statement as read off the wire.
pub use mycelium::anastome::dialogue::conformance::{
    ActRead, ActRequirement, Need, Profile, ProfileRead, ProfileStatement, Statement, StatementRead, STATEMENT,
};

/// The statement's codec: the bytes a hypha serves, the reader of them, and
/// the mark of the profiled form.
pub use mycelium::anastome::dialogue::conformance::{decode_statement_reply, encode_statement, FORM_PROFILES};

/// The request for the statement, `[ACT_LAWS][6]`.
pub use mycelium::physics::laws::codec::encode_statement_request;
