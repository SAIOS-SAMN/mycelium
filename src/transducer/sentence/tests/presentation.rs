// 78.160 P1 — THE PRESENTATION BY HOLES, proved by the algebra on real
// percepts: a held slide cites a slide along its own hole by the integer
// ratio of the two slides' counts, and the held slide's power composes to the motion exactly; a
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

/// A held slide cites a longer slide on its hole and composes to it.
#[test]
fn deconstruct_cites_held_slide_by_count_ratio() {
    let one = (vec![vec![1u8, 0, 0, 0, 0, 0, 0]], vec![vec![0u8, 1, 0, 0, 0, 0, 0]]);
    let three = (vec![vec![1u8, 0, 0, 0, 0, 0, 0]], vec![vec![0u8, 0, 0, 1, 0, 0, 0]]);
    let held = crate::transducer::family::cipher_form(&one.0, &one.1);
    let mut p = Presentation::default();
    let form = Address::from(vec![7u8; 8]);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 12);
    assert!(!p.file_form(&form, &held, &mut budget).filed.is_empty(), "the held slide's vertex is filed under its hole");
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

/// A slide is read on the null lane, and a held slide re-derives its cube.
#[test]
fn read_word_cites_slide_on_null_lane() {
    use crate::transducer::sentence::presentation::LANE_T;
    let one = (vec![vec![1u8, 0, 0, 0, 0, 0, 0]], vec![vec![0u8, 1, 0, 0, 0, 0, 0]]);
    let three = (vec![vec![1u8, 0, 0, 0, 0, 0, 0]], vec![vec![0u8, 0, 0, 1, 0, 0, 0]]);
    let held = crate::transducer::family::cipher_form(&one.0, &one.1);
    let mut p = Presentation::default();
    let form = Address::from(vec![5u8; 8]);
    assert!(!p.learn_form(&form, &held, false).filed.is_empty(), "the dialect's slide is learned");
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

/// A hue cycle and its powers stand on one mode, and its order returns it.
#[test]
fn gauge_modes_reads_cycle_powers_on_one_mode() {
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

/// A motion on a hole nothing holds is residue and composes to nothing.
#[test]
fn deconstruct_leaves_unheld_hole_as_residue() {
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

/// The coordinate and the vertex of a form's one word vertex.
fn the_word(f: &crate::transducer::sentence::Form) -> (Vec<u8>, Vertex) {
    let words: Vec<(Vec<u8>, Vertex)> =
        f.vertices.scoped().filter(|(_, v)| matches!(v, Vertex::Word { .. })).map(|(k, v)| (k, v.clone())).collect();
    let [w] = words.as_slice() else { panic!("one word vertex in the scene, read {}", words.len()) };
    w.clone()
}

/// The one word vertex of a scene, at its own coordinate, as its address.
fn scene_word(a: &[Vec<u8>], b: &[Vec<u8>]) -> (Vec<u8>, Vertex, Address) {
    let (at, w) = the_word(&crate::transducer::family::cipher_form(&a.to_vec(), &b.to_vec()));
    let Vertex::Word { address, .. } = &w else { panic!("a word vertex") };
    let address = address.clone();
    (at, w, address)
}

/// A form whose every word reads is filed whole with no refusal.
#[test]
fn learn_form_files_readable_form() {
    use crate::transducer::sentence::generator_key;
    let one = (vec![vec![1u8, 0, 0, 0, 0, 0, 0]], vec![vec![0u8, 1, 0, 0, 0, 0, 0]]);
    let held = crate::transducer::family::cipher_form(&one.0, &one.1);
    let mut p = Presentation::default();
    let form = Address::from(vec![3u8; 8]);
    let learned = p.learn_form(&form, &held, true);
    assert!(learned.refused.is_empty(), "every word vertex reads, read {:?}", learned.refused);
    assert_eq!(p.refusals_len(), 0);
    // Every vertex stands in exactly one of the three.
    assert_eq!(learned.filed.len() + learned.not_words.len(), held.vertices.len());
    assert_eq!(learned.filed.len(), p.len(), "each filed vertex is one generator");
    let (_, w) = the_word(&held);
    let Vertex::Word { address, .. } = &w else { panic!("a word vertex") };
    assert_eq!(learned.filed, vec![address.clone()]);
    let g = p.generator(&generator_key(&form, address)).expect("held at its generator key");
    assert!(g.ruled && !is_still(&g.motion), "the slide moves, as ruled");
}

/// An unreadable word is refused by name at its coordinate, and the rest of
/// the form is filed.
#[test]
fn learn_form_refuses_unreadable_word() {
    use crate::transducer::sentence::generator_key;
    use crate::transducer::sentence::presentation::{LearnRefusal, Unlearned};
    let one = (vec![vec![1u8, 0, 0, 0, 0, 0, 0]], vec![vec![0u8, 1, 0, 0, 0, 0, 0]]);
    let three = (vec![vec![1u8, 0, 0, 0, 0, 0, 0]], vec![vec![0u8, 0, 0, 1, 0, 0, 0]]);
    let mut f = crate::transducer::family::cipher_form(&one.0, &one.1);
    let (_, _, kept) = scene_word(&one.0, &one.1);
    // A turn by 90°, its least member's hue listing sending hue 1 two ways:
    // no map (rule 11). Its motion reads; its hue map does not.
    let ell = vec![vec![0u8, 0, 0, 0, 0], vec![0, 2, 2, 2, 0], vec![0, 2, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]];
    let quarter = vec![vec![0u8, 0, 0, 0, 0], vec![0, 2, 2, 0, 0], vec![0, 0, 2, 0, 0], vec![0, 0, 2, 0, 0], vec![0, 0, 0, 0, 0]];
    let (no_map_at, mut v, no_map) = scene_word(&ell, &quarter);
    if let Vertex::Word { acts, .. } = &mut v {
        let (k, _) = acts.entries().next().expect("the turn has a member");
        acts.at_mut(&k).expect("its member").gauge = vec![(1, 2), (1, 3)];
    }
    f.vertices.put(&v.at().expect("a word vertex has a coordinate"), v);
    // A turn by 180° stripped of its members: no chart to read a motion off.
    let half = vec![vec![0u8, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 2, 0], vec![0, 2, 2, 2, 0], vec![0, 0, 0, 0, 0]];
    let (bare_at, mut v, bare) = scene_word(&ell, &half);
    let at = v.at().expect("a word vertex has a coordinate");
    if let Vertex::Word { acts, .. } = &mut v {
        let keys: Vec<_> = acts.entries().map(|(k, _)| k).collect();
        for k in keys {
            acts.drop_at(&k);
        }
    }
    f.vertices.put(&at, v);
    assert_eq!(f.vertices.len(), 4, "the frame and three word vertices, each at its own coordinate");
    let mut p = Presentation::default();
    let form = Address::from(vec![4u8; 8]);
    let learned = p.learn_form(&form, &f, false);
    assert_eq!(learned.filed, vec![kept.clone()], "the readable slide is filed");
    // Each unreadable word is refused once, by its reason at its coordinate.
    let expected = [
        LearnRefusal { at: no_map_at, vertex: no_map.clone(), reason: Unlearned::NoMap },
        LearnRefusal { at: bare_at, vertex: bare.clone(), reason: Unlearned::NoMember },
    ];
    assert_eq!(learned.refused.len(), expected.len(), "two refusals, read {:?}", learned.refused);
    assert!(expected.iter().all(|r| learned.refused.contains(r)), "each unreadable word is refused by its reason at its coordinate, read {:?}", learned.refused);
    assert_eq!(learned.filed.len() + learned.refused.len() + learned.not_words.len(), f.vertices.len());
    // Each refusal is held where its generator would have stood, and nothing is held there.
    assert_eq!(p.refusal(&generator_key(&form, &no_map)), Some(Unlearned::NoMap));
    assert_eq!(p.refusal(&generator_key(&form, &bare)), Some(Unlearned::NoMember));
    assert!(p.generator(&generator_key(&form, &no_map)).is_none());
    assert_eq!(p.refusal(&generator_key(&form, &kept)), None);
    // What was filed reads as before: the held slide cubed re-derives the longer one.
    let target = motions(&three.0, &three.1);
    let [m] = target.as_slice() else { panic!("one moving vertex in the longer slide") };
    assert!(p.read_word(m, &[], false).expect("the slide reads").composes);
    // Unfiled, the form leaves no generator and no refusal.
    p.unfile_form(&form, &f);
    assert!(p.is_empty());
    assert_eq!(p.refusals_len(), 0);
}
