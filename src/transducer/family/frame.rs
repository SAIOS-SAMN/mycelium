use super::SectionSample;
use crate::transducer::sentence::Vertex;

/// The scene frame as a form vertex.
pub fn frame_vertex() -> Vertex {
    Vertex::Frame
}

/// THE FAMILY'S RE-PERIOD ((h_num, h_den), (w_num, w_den)): each circle's
/// exact ratio `h′/h`, `w′/w` in lowest terms (`transduce::Enclosure::
/// reperiod`), identical across pairs or refused — never rounded, and
/// `None` where a frame has no enclosure (empty or ragged).
pub fn family_ratio(family: &[(SectionSample, SectionSample)]) -> Option<((usize, usize), (usize, usize))> {
    use super::transduce::Enclosure;
    let mut ratio = None;
    for (a, b) in family {
        let r = match Enclosure::of(a.sample()).reperiod(&Enclosure::of(b.sample()))?.as_slice() {
            &[first, second] => (first, second),
            _ => return None,
        };
        match &ratio {
            None => ratio = Some(r),
            Some(held) if *held != r => return None,
            _ => {}
        }
    }
    ratio
}
