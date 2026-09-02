// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! CONGRUENCE — the senses' agreement, the first level of coherence.
//!
//! A reading is what perception hands to thought: the bodies the eye
//! lifted, what the ear heard of each, and what touch found between
//! them. The senses compound; where they contradict, the contradiction is
//! the signal (the convergence engine's transmutation signal), never a
//! vote. Coherence within one kernel and contradiction between kernels
//! are the next levels, and belong to thought and the court.

use super::ear::Harmonics;
use crate::transducer::element::shape_of_chain;

/// One perceived body with what the ear heard of it.
#[derive(Debug, Clone)]
pub struct Percept {
    pub cells: Vec<(i64, i64, u8, u8)>,
    pub harmonics: Harmonics,
}

/// The reading of a chain: its bodies, each heard.
#[derive(Debug, Clone, Default)]
pub struct Reading {
    pub bodies: Vec<Percept>,
}

/// READ a chain through eye, ear, and touch.
pub fn read(chain: &[(i64, i64, u8, u8)]) -> Reading {
    let bodies = super::touch::pieces(chain)
        .into_iter()
        .filter_map(|cells| {
            let shape = shape_of_chain(&cells)?;
            Some(Percept { harmonics: Harmonics::of(&shape.blade), cells })
        })
        .collect();
    Reading { bodies }
}

impl Reading {
    /// Two readings are congruent when their bodies sound alike, body for
    /// body, in order: the same strata, whatever the placement.
    pub fn congruent(&self, other: &Reading) -> bool {
        self.bodies.len() == other.bodies.len()
            && self.bodies.iter().zip(&other.bodies).all(|(a, b)| a.harmonics == b.harmonics)
    }
}
