// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 23.8, S13.4, MU-CONFORMANCE — THE DEFERRAL: the one reply a hypha whose
// worker pool is full gives every act, which every client recognises and
// retries.

//! The deferral.
//!
//! A hypha whose worker pool is full answers every act with one 43-byte
//! reply, the DEFERRAL: the pending outcome code ([`PENDING_CODE`]), a
//! height of zero, the door's per-lane reserve deficit on S, G, T and H,
//! and nothing taught. Nothing was ruled; the act was not read; the client
//! speaks it again later. Every client, of either conformance profile,
//! recognises a deferral before it decodes a reply, and never reads one as
//! a reply that did not decode.
//!
//! Two routes reach it:
//!
//! - [`crate::session::Session::speak`] recognises the deferral itself and
//!   answers it as a `WouldBlock` refusal carrying the [`Deferral`];
//!   [`deferral_of`] reads it back off that refusal;
//! - a client on a transport of its own reads the reply bytes with
//!   [`read_deferral`] (or [`is_deferral`] and [`deferral_lanes`] with
//!   [`PENDING_CODE`]).
//!
//! ```
//! use mycelium_sdk::deferral::{encode_deferral, read_deferral, Deferral, PENDING_CODE};
//!
//! let reply = encode_deferral(PENDING_CODE, &[1, 0, 2, 0]);
//! assert_eq!(reply.len(), 43);
//! assert_eq!(read_deferral(&reply), Some(Deferral { deficit: [1, 0, 2, 0] }));
//! ```

/// The deferral as read: the door's reserve deficit on the lanes S, G, T,
/// H, as the hypha stated it.
pub use mycelium::sdk::Deferral;

/// The deferral read off a reply's bytes, or none where the reply is any
/// other answer.
pub use mycelium::sdk::read_deferral;

/// The deferral a session's refusal carries: `Session::speak` answers a
/// deferral as a `WouldBlock` refusal holding it.
pub use mycelium::sdk::deferral_of;

/// The pending outcome code a deferral leads with.
pub use mycelium::anastome::dialogue::PENDING_CODE;

/// The deferral's codec: its writer (a hypha's), the reader of its four
/// lanes and the predicate, each given the pending code.
pub use mycelium::anastome::dialogue::{deferral_lanes, encode_deferral, is_deferral};
