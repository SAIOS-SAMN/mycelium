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
pub use persist::GenomeRefused;
#[cfg(test)]
pub use persist::{PRE_ORBIT_GENOME_MAGIC, PRE_TRIE_GENOME_MAGIC};

use crate::transducer::sentence::presentation::Presentation;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// 78.160, MU-CLIENT §5.1 — THE EAR, SHARED BETWEEN TWO CARRIERS: the field
/// writes it as the mind learns and sheds words; the conversation reads it
/// to hear what other minds say, and files the transports a shared thought
/// leaves. A hold is one read or one write, never across a wait on a
/// counterparty. A poisoned lock still holds a whole presentation (every
/// write is one call that completes or panics before it is seen), so it is
/// read on.
#[derive(Clone, Default)]
pub struct Ear(Arc<RwLock<Presentation>>);

impl Ear {
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
