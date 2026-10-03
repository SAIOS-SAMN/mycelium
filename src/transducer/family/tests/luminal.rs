// 78.77 — LIGHT THROUGH THE SCENE (`family/luminal.rs`): a waveform ray out of
// a body explains the unsaid sites on it and names where it stopped; a ray of
// one lit site is no waveform; an enclosure captures what the output fills
// inside it, read as a standing wave, its rectangle's billiard a translation.

use super::common::grid;
use crate::network::nerve::coord::BivectorKind;
use crate::transducer::family::luminal::{read_pair, Beyond, Stop};

fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// A RAY: a hue-1 body at the left; the output paints hue 1 rightward to a
/// hue-2 body, which absorbs it. The four unsaid sites are explained by one
/// waveform of period 1 in the source's own hue, stopped at the body.
#[test]
fn a_ray_explains_the_light_it_carries_and_names_its_stop() {
    let x = grid(&["100002"]);
    let y = grid(&["111112"]);
    let rem = vec![(0, 1, 0, 1), (0, 2, 0, 1), (0, 3, 0, 1), (0, 4, 0, 1)];
    let p = read_pair(&x, &y, &rem, &mut budget());
    assert_eq!((p.unsaid, p.on_ray, p.neither), (4, 4, 0), "every unsaid site on the ray: {p:?}");
    let ray = p.rays.iter().find(|r| r.dir == (0, 1) && r.source == 1).expect("the rightward ray");
    assert_eq!((ray.lit, ray.period, ray.wave.clone()), (4, 1, vec![1]), "a waveform of period 1 in the source's hue");
    assert_eq!(ray.stop, Stop::Body { hue: 2, beyond: Beyond::Absorbed }, "met and absorbed");
    // One lit site is no waveform: nothing explained.
    let p = read_pair(&grid(&["100"]), &grid(&["110"]), &[(0, 1, 0, 1)], &mut budget());
    assert_eq!((p.on_ray, p.neither), (0, 1), "a single site repeats nothing");
}

/// CAPTURED LIGHT: a hue-3 ring encloses a 2×2 cavity the output fills with
/// hue 4. The four sites are captured; the standing wave has period 1 both
/// ways and fits; the rectangle's billiard — four wall reflections — is a
/// translation, parabolic.
#[test]
fn an_enclosure_captures_a_standing_wave_and_its_billiard_reads() {
    let x = grid(&["3333", "3003", "3003", "3333"]);
    let y = grid(&["3333", "3443", "3443", "3333"]);
    let rem = vec![(1, 1, 0, 4), (1, 2, 0, 4), (2, 1, 0, 4), (2, 2, 0, 4)];
    let p = read_pair(&x, &y, &rem, &mut budget());
    assert_eq!(p.captured + p.both, 4, "the cavity holds every unsaid site: {p:?}");
    let [c] = p.cavities.as_slice() else { panic!("one enclosure: {:?}", p.cavities) };
    assert_eq!((c.wall, c.rect, c.periods, c.fits), (Some(3), Some((2, 2)), Some((1, 1)), Some(true)));
    assert_eq!(c.modes.as_deref(), Some(&[BivectorKind::Parabolic][..]), "two pairs of parallel walls compose a translation");
}
