// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 24.3 — THE SESSION: one client's contact with the mesh, every exchange
// bounded by the transport aperture; the deferral a full hypha answers it
// with (23.8, S13.4, MU-CONFORMANCE); and the reference mind opened on it
// (24).

//! A session with a hypha.
//!
//! [`Session::open`] dials one hypha's address with a
//! [`Voice`](crate::voice::Voice); the session learns the rest of the
//! nerve. [`Session::speak`] sends one act and reads its reply;
//! [`Session::speak_keyed`] routes an act to the hypha its key lives on.
//! Every reply carries the final TUniversal point the session last heard
//! ([`Session::tuniversal`]).
//!
//! ## The deferral
//!
//! A hypha whose worker pool is full answers every act with one 43-byte
//! reply, the DEFERRAL: the pending outcome code
//! ([`PENDING_CODE`](crate::PENDING_CODE)), a height of zero, a reading of
//! the door's reserve deficit on each base lane S, G, T and H (a reading per
//! lane, never a holding), and nothing taught. Nothing was ruled; the act
//! was not read; the client speaks it again later. Every client, of either
//! conformance profile, recognises a deferral before it decodes a reply,
//! and never reads one as a reply that did not decode.
//!
//! Two routes reach it:
//!
//! - [`Session::speak`] recognises the deferral itself and answers it as a
//!   `WouldBlock` refusal carrying the [`Deferral`]; [`deferral_of`] reads
//!   it back off that refusal;
//! - a client on a transport of its own reads the reply bytes with
//!   [`read_deferral`].
//!
//! ## The reference mind
//!
//! A client that runs the library's own mind rather than its own opens one
//! with [`Mind::open`] (a fresh genome) or `Mind::open_persistent` (the
//! genome restored from a file, with what the file came to answered as a
//! [`GenomeReading`]).

/// A mind's session with a hypha: `open`, `speak`, `speak_keyed`,
/// `tuniversal`; the domain acts `found_domain`, `reserve` and
/// `reserve_reading`; the seat acts (see [`crate::seat`]).
pub use mycelium::sdk::Session;

/// How far one exchange may run, in the mind's own strokes.
pub use mycelium::sdk::TransportAperture;

/// A final point of TUniversal, `origin/h:S:G:T:H:K`: LAW-COORDINATE v3's
/// one-component, non-negative case, each base lane one exact non-negative
/// component, and `K` the orbits closed.
pub use mycelium::chronometry::tuniversal::TPoint;

/// The deferral as read: the door's reserve deficit read on each lane S,
/// G, T, H, as the hypha stated it; a reading per lane, never a holding.
pub use mycelium::sdk::Deferral;

/// The deferral read off a reply's bytes, or none where the reply is any
/// other answer: the one reader of the deferral's bytes.
pub use mycelium::sdk::read_deferral;

/// The deferral a session's refusal carries: `Session::speak` answers a
/// deferral as a `WouldBlock` refusal holding it.
pub use mycelium::sdk::deferral_of;

/// The reference mind: a voice with its genome and epigenome.
pub use mycelium::sdk::Mind;

/// What a genome file came to when a mind was opened on it.
pub use mycelium::sdk::GenomeReading;
