// AT1 (i) — the atlas's coordinates decode to themselves, and the frame's
// spelling orders as the frames do.

use crate::network::nerve::coord::{put_frame, take_frame, ChartAt, Overlap, Transport, VertexAt};
use crate::network::nerve::Coordinate;

fn class() -> Transport {
    Transport {
        kind: crate::transducer::peel::ShellKind::Whole,
        class: crate::transducer::alphabet::Word::identity()
            .rotor()
            .holonomy_key()
            .expect("the identity has a class"),
    }
}

fn chart() -> crate::transducer::stalk::Invariant {
    crate::transducer::stalk::invariant(&[(0, 0, 1, 1), (0, 1, 1, 1)])
}

/// EVERY KIND OF VERTEX, every chart and an overlap between two, read
/// back from its own bytes as itself: `x ≡ decode(encode(x))`.
#[test]
fn vertex_at_round_trips_with_chart_and_overlap() {
    let vertices = [
        VertexAt::Frame,
        VertexAt::Word(class()),
        VertexAt::Rebased { base: class(), own: class() },
        VertexAt::Demand {
            kind: crate::transducer::sentence::Demand::Creation,
            chart: chart(),
            measure: [3, 0, 1, 70_000],
            pieces: 2,
        },
    ];
    for v in &vertices {
        assert_eq!(VertexAt::decode(v.encode().as_bytes()).as_ref(), Ok(v), "{v:?}");
    }
    let at = |k: u64| ChartAt::at(VertexAt::Word(class()), chart(), k);
    for c in [ChartAt::Frame, at(0), at(300)] {
        assert_eq!(ChartAt::decode(c.encode().as_bytes()), Ok(c.clone()));
    }
    let step = Overlap { from: at(1), to: at(2) };
    assert_eq!(Overlap::decode(step.encode().as_bytes()), Ok(step.clone()));
    let star = Overlap { from: ChartAt::Frame, to: at(2) };
    assert_eq!(Overlap::decode(star.encode().as_bytes()), Ok(star));
    let mut trailing = step.encode().as_bytes().to_vec();
    trailing.push(0);
    assert!(Overlap::decode(&trailing).is_err(), "bytes past the last coordinate refuse");
}

/// THE FRAME'S SPELLING is one spelling per frame, ordered as the frames
/// are, with no wall below the value's own width.
#[test]
fn put_frame_orders_bytes_as_frames_and_round_trips() {
    let frames = [0u64, 1, 255, 256, 65_536, u64::MAX];
    let spelled: Vec<Vec<u8>> = frames
        .iter()
        .map(|&k| {
            let mut b = Vec::new();
            put_frame(k, &mut b);
            b
        })
        .collect();
    assert!(spelled.windows(2).all(|w| w[0] < w[1]), "byte order is numeric order");
    for (k, b) in frames.iter().zip(&spelled) {
        let mut pos = 0usize;
        assert_eq!(take_frame(b, &mut pos), Some(*k));
        assert_eq!(pos, b.len());
    }
    let mut pos = 0usize;
    assert_eq!(take_frame(&[1, 0], &mut pos), None, "a leading zero is a second spelling of one frame");
}
