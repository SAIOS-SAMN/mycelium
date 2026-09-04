// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THOUGHT — the holonomic layer: a kernel's mathematics over its dialect.
//!
//! Perception yields bodies heard on their harmonics; thought is what a
//! kernel does with them in the bundle over its own language: it
//! deconstructs a body into the words that compose it (comprehension, the
//! adjoint of speech), composes words into sentences with their
//! connection, replays and verifies what other voices said, comprehends a
//! family as a transport, and germinates what no word carries. The five
//! motions of thought are named in [`strokes`]: ascend, descend, expand,
//! contract, puncture.
//!
//! Thought never touches the wire. What it concludes leaves through the
//! kernel's voice (`crate::voice`).

pub use crate::transducer::element::{self, Element, Versor};
pub use crate::transducer::family::{self, comprehend as comprehend_family, speak as speak_family, Transport};
pub use crate::transducer::germinate;
pub use crate::transducer::sentence::{
    self, deconstruct, is_flat, holonomy, replay, replay_operators, verify, verify_utterance, Cite, Sentence,
};
pub use crate::transducer::stalk::{self, Invariant, Stalks};

/// THE STROKES — the five motions a thought makes, each a closed form.
pub mod strokes {
    use crate::perception::ear::Harmonics;
    use crate::transducer::blade::Blade;

    /// ASCEND: a body rises to its harmonics — the invariant it is heard
    /// on, free of size, place, and orientation.
    pub fn ascend(body: &Blade) -> Harmonics {
        Harmonics::of(body)
    }

    /// DESCEND: a comprehended transport is realized on a new scene — the
    /// bodies carried and rendered onto the sensor's frame, or refused.
    pub fn descend(transport: &crate::transducer::family::Transport, scene: &crate::transducer::family::Grid) -> Option<crate::transducer::family::Speech> {
        crate::transducer::family::speak(transport, scene)
    }

    /// EXPAND: a new axis opens — what no word carries germinates as the
    /// residual between two scenes.
    pub fn expand(a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) -> Option<crate::transducer::germinate::ResidualGerm> {
        crate::transducer::germinate::germinate(a, b)
    }

    /// CONTRACT: a word's torsion cycle is extracted, packing its sections
    /// into one orbit (σ^ℓ = 1). Returns the sections absorbed.
    pub fn contract(word: &mut crate::transducer::element::Element) -> usize {
        word.torse()
    }

    /// PUNCTURE: a held word's fiber deepens by a section an utterance
    /// carried and the listener verified. Returns the fibers deepened.
    pub fn puncture(
        words: &mut [std::sync::Arc<crate::transducer::element::Element>],
        index_of: impl Fn(&[u8; 32]) -> Option<usize>,
        cites: &[crate::anastome::dialogue::CiteWire],
    ) -> usize {
        crate::transducer::sentence::puncture_from(words, index_of, cites)
    }
}
