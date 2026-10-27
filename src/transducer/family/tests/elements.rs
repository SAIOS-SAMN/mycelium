// THE ACTS AS GEOMETRIC ELEMENTS (ruled 2026-10-26, the English DSL trap):
// a clause is lowered to the element it names and spoken as that element;
// the clause is the element's gloss. The claims: the gloss inverts the
// lowering on every act, the identity carriage IS the hue map, and an
// element no act names is said all the same.

use crate::arc::Grid;
use crate::core::Q;
use crate::transducer::cga::{rotator, translator_at, Mv};
use crate::transducer::family::statement::{speak_clauses, speak_elements, Unstated};
use crate::transducer::sentence::form::clause::{put_clauses, About, Act, Clause, Extent, Paint, Pred, Region, Stop};
use crate::transducer::sentence::form::rule_element::{gloss, lattice_step, lower, Carriage, Lowered, Reach, RuleElement, Trace};

fn grid(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

fn q(n: i64) -> Q {
    Q::from_integer(n.into())
}

fn shift(dr: i64, dc: i64) -> Mv {
    translator_at(&[q(dr), q(dc)])
}

fn drawn(spoken: Result<crate::transducer::family::statement::Spoken, Unstated>) -> Result<Grid, Unstated> {
    let spoken = spoken?;
    let mut g: Grid = vec![vec![0; spoken.frame.1]; spoken.frame.0];
    for (r, c, h) in spoken.painted {
        g[usize::try_from(r).expect("on the grid")][usize::try_from(c).expect("on the grid")] = h;
    }
    Ok(g)
}

fn tape(clauses: &[Clause]) -> Vec<u8> {
    let mut b = Vec::new();
    put_clauses(&mut b, clauses);
    b
}

/// THE GLOSS INVERTS THE LOWERING: every act, lowered to its element and
/// glossed back, is the clause it was — the same tape, byte for byte.
#[test]
fn every_act_lowered_and_glossed_is_the_clause_it_was() {
    let acts = vec![
        Act::Recolour(vec![(1, 2), (3, 0)]),
        Act::Repaint { paint: Paint::Of(Pred::Hue(3)) },
        Act::Repaint { paint: Paint::Hue(4) },
        Act::Carry { motion: rotator(&q(0), &q(1)), about: About::Body, gauge: vec![(1, 5)] },
        Act::Echo { motion: shift(0, 2), about: About::Grid, gauge: Vec::new() },
        Act::Slide { ray: (1, -1), stop: Stop::Contact },
        Act::Extend { ray: (0, 2), paint: Paint::Hue(7), stop: Stop::Border },
        Act::Fill { region: Region::Interior, hue: 4 },
        Act::Connect { paint: Paint::Own },
        Act::Bend { to: Pred::Hue(2), paint: Paint::Hue(3) },
        Act::Copy { template: Pred::Hue(8) },
        Act::Crop(Extent::Interior),
        Act::Scale { up: 1, down: 3 },
        Act::Panes { along_rows: true, gap: true, table: 0b0110, paint: 2 },
        Act::Tile { rows: 1, cols: 2, turns: vec![0, 1] },
        Act::Parts(Box::new(Act::Recolour(vec![(2, 6)]))),
    ];
    for act in acts {
        let clause = Clause { select: Pred::Hue(1), act };
        let lowered = lower(&clause).expect("every act lowers");
        let back = gloss(&lowered).expect("every act is the gloss of its element");
        assert_eq!(tape(&[back]), tape(std::slice::from_ref(&clause)), "glossed back as written: {clause:?}");
    }
    let nested = Clause { select: Pred::All, act: Act::Parts(Box::new(Act::Parts(Box::new(Act::Recolour(Vec::new()))))) };
    assert!(lower(&nested).is_none(), "parts are read one level");
}

/// THE IDENTITY CARRIAGE IS THE HUE MAP: a carry by the identity versor —
/// about the grid or about each body — speaks what the recolour speaks, and
/// its element glosses as the recolour.
#[test]
fn the_identity_carriage_is_the_hue_map() {
    let x = grid(&["110", "000", "033"]);
    let bodies = crate::transducer::family::perceive(&x);
    let map = vec![(1, 2), (3, 0)];
    let recolour = Clause { select: Pred::All, act: Act::Recolour(map.clone()) };
    let want = drawn(speak_clauses(std::slice::from_ref(&recolour), &x, &bodies));
    assert_eq!(want, Ok(grid(&["220", "000", "000"])));
    for about in [About::Grid, About::Body] {
        let carry = Clause { select: Pred::All, act: Act::Carry { motion: Mv::scalar(q(1)), about, gauge: map.clone() } };
        assert_eq!(drawn(speak_clauses(std::slice::from_ref(&carry), &x, &bodies)), want, "the identity carried {about:?}");
        let glossed = gloss(&lower(&carry).expect("lowers")).expect("named");
        assert_eq!(tape(&[glossed]), tape(&[recolour.clone()]), "its gloss is the recolour");
    }
}

/// AN ELEMENT NO ACT NAMES IS SAID: a body carried to its meet keeping its
/// source — a slide that leaves itself behind — lands at the floor and
/// stays where it stood; it has no gloss.
#[test]
fn a_carriage_no_act_names_is_spoken_and_has_no_gloss() {
    let x = grid(&["0100", "0000", "0000", "2222"]);
    let bodies = crate::transducer::family::perceive(&x);
    let echo_to_meet = Lowered {
        select: Pred::Hue(1),
        parts: false,
        element: RuleElement::Carriage(Carriage { motion: shift(1, 0), about: About::Grid, reach: Reach::Meet(Stop::Contact), trace: Trace::Keeps, paint: Paint::Own, gauge: Vec::new() }),
    };
    assert_eq!(drawn(speak_elements(std::slice::from_ref(&echo_to_meet), &x, &bodies)), Ok(grid(&["0100", "0000", "0100", "2222"])));
    assert!(gloss(&echo_to_meet).is_none(), "no act names it");
}

/// ITERATION READS A LATTICE TRANSLATION: a translator by `(2, −1)` is that
/// step; a turn is no step, and a carriage iterating one is refused by name.
#[test]
fn a_carriage_iterates_a_translation_and_refuses_a_turn() {
    assert_eq!(lattice_step(&shift(2, -1)), Some((2, -1)));
    let turn = rotator(&q(0), &q(1));
    assert_eq!(lattice_step(&turn), None, "a turn names no ray");
    let x = grid(&["010", "000"]);
    let bodies = crate::transducer::family::perceive(&x);
    let spun = Lowered {
        select: Pred::All,
        parts: false,
        element: RuleElement::Carriage(Carriage { motion: turn, about: About::Grid, reach: Reach::Meet(Stop::Border), trace: Trace::Clears, paint: Paint::Own, gauge: Vec::new() }),
    };
    assert_eq!(drawn(speak_elements(&[spun], &x, &bodies)), Err(Unstated::Still));
}
