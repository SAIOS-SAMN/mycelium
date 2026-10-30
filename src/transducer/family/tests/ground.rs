// THE AMBIENT GROUND (ruled 2026-10-18): a family drawn on a
// canvas of one hue is read with that hue as the void the eye does not see;
// the channels are never relabelled (78.56 II).

use super::common::*;
use crate::transducer::family::*;

/// The family's rows with every `from` written `to`: the same scene on
/// another canvas.
fn on_canvas(family: &[(Grid, Grid)], to: u8) -> Vec<(Grid, Grid)> {
    let paint = |g: &Grid| -> Grid { g.iter().map(|row| row.iter().map(|&h| if h == 0 { to } else { h }).collect()).collect() };
    family.iter().map(|(x, y)| (paint(x), paint(y))).collect()
}

/// THE GROUND IS READ OFF THE FAMILY: the hue of the greatest count over
/// every grid of its pairs; two hues tied for it name no ground of their
/// own, and the void is hue 0.
#[test]
fn the_ground_is_the_hue_the_family_is_drawn_on() {
    let bar = vec![(grid(&["1100", "0000", "0000"]), grid(&["0011", "0000", "0000"]))];
    assert_eq!(crate::arc::ground_of(&bar), 0, "drawn on hue 0");
    assert_eq!(crate::arc::ground_of(&on_canvas(&bar, 7)), 7, "drawn on hue 7");
    let tied = vec![(grid(&["12", "12"]), grid(&["21", "21"]))];
    assert_eq!(crate::arc::ground_of(&tied), 0, "a tie names no ground: the void is hue 0");
    assert_eq!(crate::arc::ground(), 0, "and no family's reading is open outside one");
}

/// A SCENE ON A CANVAS IS BODIES, NOT ONE BODY. The bar on a canvas of 7:
/// read with hue 0 as the only void, the whole grid is one hue-blind body;
/// read on its family's ground, it is the bar alone. The twin families —
/// one on 0, one on 7 — extract one translation each, and the rule spoken
/// on a fourth frame of the 7 canvas paints the bar onto the 7 canvas:
/// hue 7 stays hue 7 on every grid.
#[test]
fn a_family_on_a_canvas_reads_its_bodies_and_speaks_on_its_canvas() {
    let family = vec![
        (grid(&["1100", "0000", "0000"]), grid(&["0011", "0000", "0000"])),
        (grid(&["0000", "1100", "0000"]), grid(&["0000", "0011", "0000"])),
        (grid(&["0000", "0000", "1100"]), grid(&["0000", "0000", "0011"])),
    ];
    let canvas = on_canvas(&family, 7);
    let (x, _) = &canvas[0];
    assert_eq!(perceive(x).len(), 1, "with hue 0 the only void, the canvas is one body");
    let seen = crate::arc::on_ground_of(&canvas, || perceive(x));
    let [bar] = seen.as_slice() else { panic!("on its ground the scene is the bar alone: {} bodies", seen.len()) };
    assert_eq!(bar.compartments.len(), 2, "the bar's two cells");
    let plain = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the plain family extracts");
    let drawn = extract_rule(&canvas, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the family on its canvas extracts");
    assert_eq!(drawn.form.vertices.len(), plain.form.vertices.len(), "one rule, whatever the canvas");
    let fourth = on_canvas(&[(grid(&["0000", "0000", "0000", "1100"]), grid(&["0000", "0000", "0000", "0011"]))], 7);
    let (tx, ty) = &fourth[0];
    let spoken = crate::arc::on_ground_of(&canvas, || speak_rule(&drawn, tx, &NoCrystal).and_then(|s| s.grid()));
    assert_eq!(spoken.as_ref(), Some(ty), "spoken onto its own canvas");
}
