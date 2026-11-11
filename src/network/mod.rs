// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 8, 17.7 — THE NETWORK: the wire that delimits a body on one lane, the
//! nerve that homes a word by incidence and finds a node by identity, and
//! the mesh record a node binds itself by, the encrypted channel an envelope
//! may ride between two hosts, and the choice of channel per peer.

pub mod wire;
pub mod noise;
pub mod channel;
pub mod nerve;
pub mod mesh;

#[cfg(test)]
mod tests;
