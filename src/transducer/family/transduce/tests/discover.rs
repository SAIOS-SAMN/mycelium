// Dynamic phase-period enclosure discovery (MU-SENSING §0, #32 S4):
// an unranked 1D stream discovers its own dimensionality (1D, 2D, 3D) from
// topological closure (β₀) and exact rational interior coherence, with no
// hardcoded 2D or dataset header.

use crate::physics::Physics;
use crate::transducer::family::transduce::{discover_enclosure, Enclosed};
use crate::transducer::sample::Place;

fn budget() -> crate::physics::Budget {
    Physics::default().stroke_n(1 << 20)
}

#[test]
fn discover_keeps_true_1d_signal_at_rank_one() {
    // Three contiguous 1D bands: folding into 3x4 or 4x3 cannot reduce β₀ = 3
    // and strictly lowers interior coherence.
    let stream = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];
    let d = discover_enclosure(&stream, 3, &mut budget()).expect("discovers");
    assert_eq!(d.enclosure.periods(), &[12]);
    assert_eq!(d.components, 3);
}

#[test]
fn discover_folds_unranked_stream_into_2d_square_enclosure() {
    // A 4x4 enclosure containing a 2x2 solid body, flattened to 16 raw bytes
    // with NO [4, 4] header.
    let stream = vec![
        0, 0, 0, 0,
        0, 1, 1, 0,
        0, 1, 1, 0,
        0, 0, 0, 0,
    ];
    let d = discover_enclosure(&stream, 3, &mut budget()).expect("discovers");
    assert_eq!(d.enclosure.periods(), &[4, 4]);
    assert_eq!(d.components, 2);

    // Lifts directly into a rank-2 Sample and Enclosed body representation.
    let sample = d.into_sample(stream, Place::new(vec![0])).expect("lifts to sample");
    let enc = Enclosed::of(&sample);
    assert_eq!(enc.enclosure.periods(), &[4, 4]);
}

#[test]
fn discover_folds_unranked_stream_into_non_square_2d_rectangle() {
    // A 3x5 rectangle with a vertical stripe at column 1, flattened to 15 raw bytes.
    let stream = vec![
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
    ];
    let d = discover_enclosure(&stream, 3, &mut budget()).expect("discovers");
    assert_eq!(d.enclosure.periods(), &[3, 5]);
    assert_eq!(d.components, 3);
}

#[test]
fn discover_folds_unranked_stream_into_3d_cube_enclosure() {
    // A 4x4x4 cube with a 2x2x2 solid core at [1..3, 1..3, 1..3], flattened
    // to 64 raw bytes with NO [4, 4, 4] header.
    let mut stream = vec![0u8; 64];
    for z in 1..3 {
        for y in 1..3 {
            for x in 1..3 {
                stream[z * 16 + y * 4 + x] = 1;
            }
        }
    }
    let d = discover_enclosure(&stream, 3, &mut budget()).expect("discovers");
    assert_eq!(d.enclosure.periods(), &[4, 4, 4]);
    assert_eq!(d.components, 2);
}
