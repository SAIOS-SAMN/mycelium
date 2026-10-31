// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 24.3 — THE SESSION: one client's contact with the mesh, every exchange
// bounded by the transport aperture.

//! A session with a hypha.
//!
//! [`Session::open`] dials one hypha's address with a [`Voice`]; the session
//! learns the rest of the nerve. [`Session::speak`] sends one act and reads
//! its reply; [`Session::speak_keyed`] routes an act to the hypha its key
//! lives on. Every reply carries the final TUniversal point the session last
//! heard ([`Session::tuniversal`]). A hypha whose worker pool is full
//! defers every act; `speak` answers a deferral as a `WouldBlock` refusal
//! that [`crate::deferral::deferral_of`] reads, and the act is spoken again.

/// A mind's session with a hypha: `open`, `speak`, `speak_keyed`,
/// `tuniversal`; the domain acts `found_domain`, `reserve` and
/// `reserve_reading`; the seat acts (see [`crate::seat`]).
pub use mycelium::sdk::Session;

/// How far one exchange may run, in the mind's own strokes.
pub use mycelium::sdk::TransportAperture;

/// A final point of TUniversal: `origin/h:S:G:T:H:K`.
pub use mycelium::chronometry::tuniversal::TPoint;

/// The voice a session speaks as.
pub use crate::voice::Voice;
