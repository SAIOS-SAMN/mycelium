// 78.160 P1 — THE PRESENTATION BY HOLES, proved by the algebra on real
// percepts: a held slide cites a slide along its own hole by the integer
// ratio of their counts, and their power composes to the motion exactly; a
// motion on a hole nothing holds stands as residue and composes to nothing.

use crate::transducer::address::Address;
use crate::transducer::sentence::presentation::{is_still, motion_power, vertex_motion, Presentation};
use crate::transducer::sentence::Vertex;

/// The whole motions of a scene's word vertices that move something.
fn motions(a: &[Vec<u8>], b: &[Vec<u8>]) -> Vec<crate::transducer::cga::Mv> {
    let f = crate::transducer::family::cipher_form(&a.to_vec(), &b.to_vec());
    f.vertices
        .scoped()
        .filter(|(_, v)| matches!(v, Vertex::Word { .. }))
        .filter_map(|(_, v)| vertex_motion(v))
        .filter(|m| !is_still(m))
        .collect()
}

#[test]
fn a_held_slide_cites_a_longer_slide_on_its_hole_and_composes_to_it() {
    let one = (vec![vec![1u8, 0, 0, 0, 0, 0, 0]], vec![vec![0u8, 1, 0, 0, 0, 0, 0]]);
    let three = (vec![vec![1u8, 0, 0, 0, 0, 0, 0]], vec![vec![0u8, 0, 0, 1, 0, 0, 0]]);
    let held = crate::transducer::family::cipher_form(&one.0, &one.1);
    let mut p = Presentation::default();
    let form = Address::from(vec![7u8; 8]);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 12);
    assert!(p.file_form(&form, &held, &mut budget) > 0, "the held slide's vertex is filed under its hole");
    let target = motions(&three.0, &three.1);
    let [m] = target.as_slice() else { panic!("one moving vertex in the longer slide, read {}", target.len()) };
    let d = p.deconstruct(m, &mut budget).expect("the slide's coordinates read");
    assert_eq!(d.residue, 0, "every hole of the longer slide is cited");
    let [(f, _, e)] = d.cites.as_slice() else { panic!("one citation, read {:?}", d.cites) };
    assert_eq!(f, &form);
    assert_eq!(*e, 3, "the count ratio is the exponent");
    assert!(d.composes, "the held slide cubed is the longer slide, exactly");
    // The reverse slide is the same hole at the negative count.
    let back = motions(&three.1, &three.0);
    let [b] = back.as_slice() else { panic!("one moving vertex back") };
    let d = p.deconstruct(b, &mut budget).expect("the reverse slide's coordinates read");
    assert!(d.composes);
    let [c, ..] = d.cites.as_slice() else { panic!("a citation") };
    assert_eq!(c.2, -3);
    assert!(is_still(&motion_power(m, -1).geo(m)), "a motion's −1 power undoes it");
}

#[test]
fn a_slide_is_read_on_the_null_lane_and_a_held_slide_re_derives_its_cube() {
    use crate::transducer::sentence::presentation::LANE_T;
    let one = (vec![vec![1u8, 0, 0, 0, 0, 0, 0]], vec![vec![0u8, 1, 0, 0, 0, 0, 0]]);
    let three = (vec![vec![1u8, 0, 0, 0, 0, 0, 0]], vec![vec![0u8, 0, 0, 1, 0, 0, 0]]);
    let held = crate::transducer::family::cipher_form(&one.0, &one.1);
    let mut p = Presentation::default();
    let form = Address::from(vec![5u8; 8]);
    assert!(p.learn_form(&form, &held, false) > 0, "the dialect's slide is learned");
    let target = motions(&three.0, &three.1);
    let [m] = target.as_slice() else { panic!("one moving vertex in the longer slide") };
    let r = p.read_word(m, &[], false).expect("the slide reads");
    assert!(r.composes, "the held slide cubed re-derives the longer one: different mathematics, one word");
    assert_eq!(r.residue, [0; 4]);
    assert!(r.cites.iter().all(|c| c.2 == LANE_T && c.3 == 3), "cited on the null lane at the count ratio, read {:?}", r.cites);
    // A court replays only what it ruled: the dialect's slide is not cited
    // where only ruled words may be, and the slide's hole is then residue on T.
    let ruled = p.read_word(m, &[], true).expect("the slide reads");
    assert!(!ruled.composes && ruled.cites.is_empty());
    assert!(ruled.residue[LANE_T] > 0);
}

#[test]
fn a_hue_cycle_and_its_powers_stand_on_one_mode_and_its_order_returns_it() {
    use crate::transducer::sentence::presentation::{gauge_modes, gauge_power, GaugeMode};
    let cycle = vec![(1u8, 2u8), (2, 3), (3, 1)];
    let modes = gauge_modes(&cycle).expect("a cycle is a map");
    assert_eq!(modes, vec![GaugeMode::Cycle(vec![1, 2, 3])]);
    let squared = gauge_power(&cycle, 2);
    assert_eq!(gauge_modes(&squared).expect("its square is a map"), vec![GaugeMode::Cycle(vec![1, 3, 2])], "the square turns the same hues the other way");
    assert!(gauge_power(&cycle, 3).is_empty(), "the cycle's order returns every hue");
    // A merge is a move on a path, not a cycle.
    assert_eq!(gauge_modes(&[(4, 0)]).expect("a merge is a map"), vec![GaugeMode::Move(4, 0)]);
}

#[test]
fn a_motion_on_a_hole_nothing_holds_is_residue_and_composes_to_nothing() {
    let across = (vec![vec![1u8, 0, 0, 0, 0]], vec![vec![0u8, 1, 0, 0, 0]]);
    let down = (vec![vec![1u8], vec![0], vec![0], vec![0]], vec![vec![0u8], vec![0], vec![1], vec![0]]);
    let held = crate::transducer::family::cipher_form(&across.0, &across.1);
    let mut p = Presentation::default();
    let form = Address::from(vec![9u8; 8]);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 12);
    p.file_form(&form, &held, &mut budget);
    let target = motions(&down.0, &down.1);
    let [m] = target.as_slice() else { panic!("one moving vertex in the downward slide") };
    let d = p.deconstruct(m, &mut budget).expect("its coordinates read");
    assert!(d.cites.is_empty(), "no held word stands on the downward hole");
    assert_eq!(d.residue, d.holes);
    assert!(!d.composes);
    // And unfiled, the held slide cites nothing.
    p.unfile_form(&form, &held);
    assert!(p.is_empty());
}
