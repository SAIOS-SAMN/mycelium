// A13, A21 — THE KEYED RELATION: a radix trie over variable-length A21
// keys, and the walks that read it.
//
//! A key is the object's own coordinates written out, so a fiber, a link,
//! a cursor and a shell are reads of structure rather than searches over a
//! list. `meet_nodes` intersects two relations by descending both at once.

pub mod relation;
pub mod schema;
pub mod ondisk;
pub mod trie;
pub mod walks;
#[cfg(test)]
pub mod work;

pub use relation::*;
pub use schema::*;
pub(crate) use trie::*;
pub use walks::*;

#[cfg(test)]
mod tests {
    mod ondisk;
}
