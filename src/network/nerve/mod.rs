// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE NERVE — keyed relations over 32-byte identities and the Cover trie
//! that homes a word by incidence: longest-common-prefix descent on the
//! chart's lane, never XOR. Join peers seed the first digest; after that
//! the trie is the topology.
//!
//! Three organs (Stage 8b S10): `keyed.rs` the radix trie every index in
//! this tree is built from (A13, A21); `cover.rs` the trie that homes a
//! word by incidence (16.11); `router.rs` the XOR overlay that finds a node
//! by identity (16.10).

pub mod cover;
pub mod keyed;
pub mod router;

pub use cover::*;
pub use keyed::*;
pub use router::*;

#[cfg(test)]
mod tests;
