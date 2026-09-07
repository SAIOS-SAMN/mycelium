// A13, A21 — THE KEYED RELATION: a radix trie over variable-length A21
// keys, and the walks that read it.
//
//! Every index in this tree is one of these. A key is the object's own
//! coordinates written out, so a fiber, a link, a cursor and a shell are
//! reads of structure rather than searches over a list. `meet_nodes` is the
//! intersection of two relations, computed by descending both at once.

//!
//! Four organs (Stage 8b S31): `relation.rs` the keyed relation and its
//! reads; `trie.rs` the radix nodes and the three moves over them;
//! `walks.rs` the fiber, link, cursor and shell; `work.rs` the counters the
//! enumeration gate reads.

pub mod relation;
pub mod trie;
pub mod walks;
#[cfg(test)]
pub mod work;

pub use relation::*;
pub(crate) use trie::*;
pub use walks::*;
