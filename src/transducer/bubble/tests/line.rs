// MU-CLIENT §4.13, §7.1 Phase 3 — A HELD THOUGHT IS ITS ORBIT: memory released
// by the orbit's physics, every claim run through the mind's own field — a
// perception released where its motion is let go, a grounded thought winding
// on past its return, and escrow conserved through a merge.

use crate::chronometry::Cite;
use crate::transducer::bubble::Contemplation;
use crate::transducer::family::{perceive_family, scene_form_on, Grid, NoCrystal};

fn grid(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// Perceive `input → output` in `family`, held on `escrow`, as a voice holds
/// it, in the field thinking on this thread.
fn perceive(c: &mut Contemplation, input: &[&str], output: &[&str], family: &[u8], escrow: [u64; 4]) -> Vec<u8> {
    let (a, b) = (grid(input), grid(output));
    let frames = perceive_family(&[(a.clone(), b.clone())]);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let f = scene_form_on(&NoCrystal, &mut budget, &a, &b, &frames).expect("the scene reads");
    c.hold_perceived(f, &frames, &Vec::new(), &[], escrow, family, &mut budget).expect("held").as_bytes().to_vec()
}

/// THE VENT, as a winding takes it: the field beats, what it let go is let
/// go, and the thoughts newly standing thread their lines.
fn vent(c: &mut Contemplation) {
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let Ok((beat, released)) = crate::transducer::mind::with_field(|f| (f.beat(), f.take_released())) else {
        panic!("a mind is thinking")
    };
    c.release(&released);
    c.thread_new(&mut budget, beat);
}

/// THE FIELD MOVED by a motion of no thought: a 90° turn on `e₁e₃`, free.
fn move_field() {
    use crate::transducer::cga::{Mv, E1, E3};
    let one = num_rational::BigRational::from_integer(1.into());
    let turn = Mv::scalar(one.clone()).add(&Mv::blade(E1 | E3, one));
    let line = crate::transducer::family::pairing::FieldLine { carriage: turn, gauge: Vec::new(), mass: 1, free: false, line: crate::transducer::cech::Thread::Free };
    let _ = crate::transducer::family::pairing::mark_flux(&[], std::slice::from_ref(&line), &mut crate::physics::Physics::default().stroke_n(1 << 16));
}

/// Two Ls three columns apart, each carried one column along: one scene of a
/// slide.
const SLIDE_IN: [&str; 3] = ["10001000", "10001000", "11001100"];
const SLIDE_OUT: [&str; 3] = ["01000100", "01000100", "01100110"];
const SLIDE_ON: [&str; 3] = ["00100010", "00100010", "00110011"];

/// A PERCEPTION OF A SLIDE IS RELEASED WHERE ITS MOTION IS LET GO, and its
/// scene's demand stands. The slide is null flux: its tube stands through the
/// beat of the vent it was perceived at, the perception threaded on it. The
/// second hour, perceived while the first stands, meets it; its slide lands
/// on the same ray and closes all that stood open there, and the first is
/// let go at the vent as that closure. The second finds nothing open to
/// thread: it stands until the field next moves, to be met — a vent with
/// nothing moved lets it go nowhere — and leaves at the beat of that motion.
/// Both gone, their remainders are at the cavity, their escrow returned exactly,
/// their scene counted for the stroke's returns, while the escaping motion
/// between the hours stands on the docket under that scene.
#[test]
fn a_slide_perception_is_released_where_its_motion_is_let_go() {
    let mut field = crate::transducer::mind::MindField::default();
    field.think(|| {
        let mut c = Contemplation::new();
        let first = perceive(&mut c, &SLIDE_IN, &SLIDE_OUT, b"the scene", [1, 0, 0, 0]);
        vent(&mut c);
        assert!(c.bubbles.get(&first).is_some(), "the first stands through the beat it was perceived at");
        let second = perceive(&mut c, &SLIDE_OUT, &SLIDE_ON, b"the scene", [1, 0, 0, 0]);
        let cavity = c.cavity_len();
        vent(&mut c);
        assert!(c.bubbles.get(&first).is_none(), "the first let go as its ray closed");
        vent(&mut c);
        assert!(c.bubbles.get(&second).is_some(), "nothing moved: no time passed, and it stands");
        move_field();
        vent(&mut c);
        assert!(c.bubbles.get(&second).is_none(), "the second, held by nothing, left at the field's next beat");
        assert!(c.cavity_len() > cavity, "their remainders at the cavity");
        assert_eq!(c.take_returned_escrow(), [2, 0, 0, 0], "their escrow returned, exactly");
        let scenes = c.take_released_scenes();
        assert!(!scenes.is_empty() && scenes.iter().all(|s| s == b"the scene"), "their scene counted: {scenes:?}");
        let demanded = c.docket_cites().into_iter().any(|cite| matches!(cite, Cite::Demand { residue: Some(_), .. }));
        assert!(demanded, "the escaping motion stands as a demand");
    });
}

/// ONE ORBIT: the L turned 90° at a time about the grid's point (2, 2), each
/// carried one column in its output.
const HOURS: [([&str; 5], [&str; 5]); 4] = [
    (["010000", "010000", "011000", "000000", "000000"], ["001000", "001000", "001100", "000000", "000000"]),
    (["000000", "001110", "001000", "000000", "000000"], ["000000", "000111", "000100", "000000", "000000"]),
    (["000000", "000000", "001100", "000100", "000100"], ["000000", "000000", "000110", "000010", "000010"]),
    (["000000", "000000", "001000", "111000", "000000"], ["000000", "000000", "000100", "011100", "000000"]),
];

/// A GROUNDED THOUGHT WINDS ON PAST ITS RETURN (Q1, Q2 as ruled). The L's
/// four hours form one thought grounded on the 90° turn; its generator is
/// admitted on its own line once per hour, so at the fourth hour the field's
/// orbit returns with its winding. The thought is not let go there: it
/// stands, its first close made, and the line's return is read as a return.
#[test]
fn a_grounded_thought_winds_on_past_its_return() {
    let mut field = crate::transducer::mind::MindField::default();
    field.think(|| {
        let mut c = Contemplation::new();
        for (input, output) in &HOURS {
            perceive(&mut c, input, output, b"the orbit", [0; 4]);
            vent(&mut c);
        }
        assert_eq!(c.closed_thoughts, 1, "the fourth hour closed its return: {}", c.resonance.line());
        let grounded: Vec<Vec<u8>> = c.bubbles.scoped().filter(|(_, b)| b.ground.is_some()).map(|(k, _)| k).collect();
        assert_eq!(grounded.len(), 1, "the grounded thought stands past its return");
    });
}

/// ESCROW IS CONSERVED THROUGH A MERGE. The L and the L turned 90°, each held
/// on its own escrow, merge into one thought, which holds exactly what the two
/// held — nothing re-drawn, nothing lost, nothing returned undrawn.
#[test]
fn escrow_is_conserved_through_a_merge() {
    let mut field = crate::transducer::mind::MindField::default();
    field.think(|| {
        let mut c = Contemplation::new();
        perceive(&mut c, &["10000", "10000", "11000", "00000"], &["01000", "01000", "01100", "00000"], b"the shape", [1, 0, 0, 2]);
        perceive(&mut c, &["11100", "10000", "00000", "00000"], &["01110", "01000", "00000", "00000"], b"the shape", [0, 3, 0, 1]);
        let held: [u64; 4] = c.bubbles.scoped().fold([0; 4], |mut sum, (_, b)| {
            for (s, e) in sum.iter_mut().zip(b.escrow) {
                *s += e;
            }
            sum
        });
        let returned = c.take_returned_escrow();
        let total: [u64; 4] = std::array::from_fn(|l| held[l] + returned[l]);
        assert_eq!(total, [1, 3, 0, 3], "every unit held or returned, exactly once: held {held:?}, returned {returned:?}");
    });
}
