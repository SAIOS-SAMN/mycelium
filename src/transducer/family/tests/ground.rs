// THE AMBIENT GROUND (ruled 2026-10-18), DEDUCED (MU-SENSING §3.4; track
// 26 S2): a family drawn on a canvas of one symbol is read with that symbol
// as the void the eye does not see; the channels are never relabelled
// (78.56 II); no reader stands on a code where no family is open.

use super::fixtures::*;
use crate::transducer::family::transduce::{Ground, Unground};
use crate::transducer::family::*;

/// The family's rows with every `from` written `to`: the same scene on
/// another canvas.
fn on_canvas(family: &[(SectionSample, SectionSample)], to: u8) -> Vec<(SectionSample, SectionSample)> {
    let paint = |g: &SectionSample| -> SectionSample {
        let mut out = g.clone();
        for (site, h) in g.sites() {
            if h == 0 {
                out.paint(site, to).expect("a site of the sample");
            }
        }
        out
    };
    family.iter().map(|(x, y)| (paint(x), paint(y))).collect()
}

/// THE GROUND IS READ OFF THE FAMILY: the canvas the bar moves on, whatever
/// its symbol; two columns that trade places are each carried as a body,
/// and the family reads no ground.
#[test]
fn ground_of_reads_family_canvas() {
    let bar = vec![(grid(&["1100", "0000", "0000"]), grid(&["0011", "0000", "0000"]))];
    assert_eq!(crate::transducer::raster::ground_of(&bar), Ground::One(0), "drawn on 0");
    assert_eq!(crate::transducer::raster::ground_of(&on_canvas(&bar, 7)), Ground::One(7), "drawn on 7");
    let traded = vec![(grid(&["12", "12"]), grid(&["21", "21"]))];
    assert_eq!(crate::transducer::raster::ground_of(&traded), Ground::None, "each column is a body");
}

/// TRACK 26 S2 — A VORTEX IS READ ONCE PER CANDIDATE, each run its own
/// perspective (`Ground::One` of the candidate), every result kept in the
/// vortex's order and none picked; `One` and `None` are read once.
#[test]
fn vortex_ground_read_once_per_candidate() {
    let halves = vec![(grid(&["0055", "0055"]), grid(&["0055", "0055"]))];
    assert_eq!(crate::transducer::raster::ground_of(&halves), Ground::Vortex(vec![0, 5]), "a vortex of 0 and 5");
    let mut runs = 0usize;
    let read = crate::transducer::raster::on_each_perspective_of(&halves, || {
        runs += 1;
        crate::transducer::raster::ground()
    });
    assert_eq!(runs, 2, "one run per candidate");
    assert_eq!(read, vec![(Ground::One(0), Ground::One(0)), (Ground::One(5), Ground::One(5))], "each run on its own candidate, both kept");
    let bar = vec![(grid(&["1100", "0000", "0000"]), grid(&["0011", "0000", "0000"]))];
    let once = crate::transducer::raster::on_each_perspective_of(&bar, crate::transducer::raster::ground);
    assert_eq!(once, vec![(Ground::One(0), Ground::One(0))], "one ground, one run");
}

/// TRACK 26 S2 — A MEMBRANE IS HELD, NEVER FLATTENED: a frame of 8 around a
/// canvas of 0 holding a dot, the frame recoloured 7 in the output. Read on
/// the interior's perspective (0), the frame is a body of the percept with
/// its symbol and its sites, and the pair's transduction carries its change
/// 8 → 7 among its incidences, on the gauge lane (one painted symbol to
/// another). "Never the ground" is never "not perceived".
#[test]
fn membrane_is_held_and_membrane_change_is_read() {
    use crate::transducer::family::transduce::Transduction;
    let x = grid(&["888888", "800008", "801008", "800008", "800008", "888888"]);
    let y = grid(&["777777", "700007", "701007", "700007", "700007", "777777"]);
    let family = vec![(x.clone(), y.clone())];
    assert!(crate::transducer::raster::ground_of(&family).holds(&0), "the framed interior is a candidate");
    let bodies = crate::transducer::raster::on_ground(Ground::One(0), || perceive(&x));
    let frame = bodies.iter().find(|b| b.compartments.iter().all(|l| l.hue == 8)).expect("the frame is a body of the percept");
    assert_eq!(frame.compartments.len(), 20, "every site of the frame, edge sites included");
    let (a, b) = (x.sample().clone(), y.sample().clone());
    let need = crate::transducer::family::transduce::need_of(&a, &b).expect("one rank");
    let t = Transduction::of(&a, &b, &mut crate::physics::Physics::default().stroke_n(need)).expect("the need is the stroke");
    assert!(
        t.changed().any(|i| t.from.bands.get(i.band_in).is_some_and(|band| band.symbol().bytes() == [8])
            && t.to.bands.get(i.band_out).is_some_and(|band| band.symbol().bytes() == [7])),
        "the frame's change 8 → 7 is an incidence of the transduction"
    );
    let [_, gauge] = t.lane_shares(&Ground::One(vec![0]));
    assert_eq!(gauge, crate::core::Q::new(20.into(), 36.into()), "the frame's 20 of 36 sites changed, painted to painted");
}

/// OUTSIDE EVERY FAMILY NO GROUND IS READ: the reading is `None`, so every
/// symbol, 0 among them, is a body; a writer that needs a symbol is
/// refused `Unscoped`; and the painted sites are every site.
#[test]
fn unscoped_reader_never_defaults_to_zero() {
    assert_eq!(crate::transducer::raster::ground(), Ground::None);
    assert!(!crate::transducer::raster::is_ground(0), "0 is no ground by default");
    assert_eq!(crate::transducer::raster::void(), Err(Unground::Unscoped));
    let g = grid(&["00", "01"]);
    assert_eq!(crate::transducer::raster::painted_sites(&g).len(), 4, "every site is painted");
    let inside = crate::transducer::raster::on_ground(Ground::One(0), || crate::transducer::raster::painted_sites(&g).len());
    assert_eq!(inside, 1, "on a ground of 0, the one painted site");
}

/// A SCENE ON A CANVAS IS BODIES, NOT ONE BODY. The bar on a canvas of 7:
/// read with no family open, every symbol is a body and the hue-blind
/// scene is one body; read on its family's ground, it is the bar alone.
/// The twin families — one on 0, one on 7 — extract one translation each,
/// and the rule spoken on a fourth frame of the 7 canvas paints the bar
/// onto the 7 canvas: 7 stays 7 on every grid.
#[test]
fn on_ground_of_reads_bodies_and_speaks_on_canvas() {
    let family = vec![
        (grid(&["1100", "0000", "0000"]), grid(&["0011", "0000", "0000"])),
        (grid(&["0000", "1100", "0000"]), grid(&["0000", "0011", "0000"])),
        (grid(&["0000", "0000", "1100"]), grid(&["0000", "0000", "0011"])),
    ];
    let canvas = on_canvas(&family, 7);
    let (x, _) = &canvas[0];
    assert_eq!(perceive(x).len(), 1, "with no family open, the canvas and the bar are one body");
    let seen = crate::transducer::raster::on_ground_of(&canvas, || perceive(x));
    let [bar] = seen.as_slice() else { panic!("on its ground the scene is the bar alone: {} bodies", seen.len()) };
    assert_eq!(bar.compartments.len(), 2, "the bar's two cells");
    let plain = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the plain family extracts");
    let drawn = extract_rule(&canvas, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the family on its canvas extracts");
    assert_eq!(drawn.form.vertices.len(), plain.form.vertices.len(), "one rule, whatever the canvas");
    let fourth = on_canvas(&[(grid(&["0000", "0000", "0000", "1100"]), grid(&["0000", "0000", "0000", "0011"]))], 7);
    let (tx, ty) = &fourth[0];
    let spoken = crate::transducer::raster::on_ground_of(&canvas, || speak_rule(&drawn, tx, &NoCrystal).and_then(|s| s.grid()));
    assert_eq!(spoken.as_ref(), Some(ty), "spoken onto its own canvas");
}
