// THE AMBIENT GROUND (ruled 2026-10-18), DEDUCED (MU-SENSING §3.4; track
// 26 S2): a family drawn on a canvas of one symbol is read with that symbol
// as the void the eye does not see; the channels are never relabelled
// (78.56 II); no reader stands on a code where no family is open.

use super::common::*;
use crate::transducer::family::transduce::{Ground, Unground};
use crate::transducer::family::*;

/// The family's rows with every `from` written `to`: the same scene on
/// another canvas.
fn on_canvas(family: &[(Grid, Grid)], to: u8) -> Vec<(Grid, Grid)> {
    let paint = |g: &Grid| -> Grid { g.iter().map(|row| row.iter().map(|&h| if h == 0 { to } else { h }).collect()).collect() };
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
