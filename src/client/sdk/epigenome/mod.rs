// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 24.1, 24.2 — THE EPIGENOME: the working atlas, comprehension, the
//! dialect lifecycle, and the Crystal it presents to the extractor.

pub mod atlas;
pub mod comprehend;
pub mod crystal;
pub mod dialect;
mod persist;
pub mod remainder;
pub mod shed;

pub(crate) use persist::genome_refused;
pub(crate) use persist::{is_residual, is_rewrite_act};
pub use persist::GenomeRefused;
#[cfg(test)]
pub use persist::{PRE_ORBIT_GENOME_MAGIC, PRE_TRIE_GENOME_MAGIC};

use crate::transducer::sentence::presentation::Presentation;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// 78.160, MU-CLIENT §4.9 — THE MIND'S VOCABULARY: the words it deduced and
/// reconstructed, by the holes they wind about — never an ear's (an ear
/// transduces and keeps nothing). The field writes it as the mind learns and
/// sheds words, and reads a word over it. A hold is one read or one write,
/// never across a wait on a counterparty. A poisoned lock still holds a whole
/// presentation (every write is one call that completes or panics before it
/// is seen), so it is read on.
#[derive(Clone, Default)]
pub struct Vocabulary(Arc<RwLock<Presentation>>);

impl Vocabulary {
    pub(crate) fn read(&self) -> RwLockReadGuard<'_, Presentation> {
        match self.0.read() {
            Ok(g) => g,
            Err(p) => p.into_inner(),
        }
    }

    pub(crate) fn write(&self) -> RwLockWriteGuard<'_, Presentation> {
        match self.0.write() {
            Ok(g) => g,
            Err(p) => p.into_inner(),
        }
    }
}
