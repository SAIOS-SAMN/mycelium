// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE EYE — light into bodies (the luminal layer).
//!
//! Every body in the system is born here: the sensor's sample lifted into
//! connected components, each a volumetric blade with its moments on the
//! hypersphere; a voice's carried bodies lifted the same way. The eye is
//! the only entry for anything perceived — a kernel never receives a
//! description of another's state, only bodies it lifts itself.

pub use crate::transducer::blade::{components, lift, shape_of, transduce, Blade, Shape};
pub use crate::transducer::family::{perceive as perceive_scene, Body};

/// See a voice: lift each carried body an utterance's citations hold into
/// a shape in the speaker's frame, with the transport that carries it
/// into the listener's. Nothing of the speaker's state is read.
pub fn perceive_voice(
    cites: &[crate::anastome::dialogue::CiteWire],
) -> Vec<(Shape, crate::transducer::element::Versor)> {
    cites
        .iter()
        .filter_map(|(_, _, v, _, body)| Some((crate::transducer::element::shape_of_chain(body)?, v.clone())))
        .collect()
}
