// CS2, 78.32(c) — the pivot read against the cells its orbits cross: a held
// form's move REACHES where it raises the crossing cell's closed algebra and
// is DERIVABLE where the algebra already holds it. Note only: the pivot
// applies as it does without the reading.

use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::bubble::*;
use crate::transducer::sentence::{Form, FormCell, Vertex};

/// A crystal offering one word on the axes it occupies, as the contemplation's
/// own pivot fixture does.
struct OneWord(Word);

impl crate::transducer::family::Crystal for OneWord {
    fn on_class(&self, _c: &crate::transducer::stalk::Invariant) -> Vec<(&element::Element, &Form, Standing)> {
        Vec::new()
    }
    fn on_body(&self, c: &crate::transducer::stalk::Invariant) -> Vec<(&element::Element, &Form, Standing)> {
        self.on_class(c)
    }
    fn speak_form(
        &self,
        _f: &Form,
        _x: &crate::transducer::family::Grid,
        _p: &Word,
    ) -> Option<crate::transducer::family::Speech> {
        None
    }
    fn by_address(&self, _a: &[u8]) -> Option<(&element::Element, &Form)> {
        None
    }
    fn on_axis(&self, _c: &crate::transducer::stalk::Invariant, axis: crate::transducer::sentence::Axis) -> Vec<Word> {
        match crate::transducer::sentence::ClassVector::of_word(&self.0) {
            Some(v) if v.occupies(axis) => vec![self.0.clone()],
            _ => Vec::new(),
        }
    }
    fn generators_under(
        &self,
        _c: &crate::transducer::stalk::Invariant,
        _budget: &mut crate::physics::Budget,
    ) -> Vec<(crate::transducer::address::Address, crate::transducer::address::Address, Word)> {
        Vec::new()
    }
}

/// Two word vertices on two charts, two star faces and a chord between them
/// carrying `chord`: one open orbit. The chord face joins two bodies, and a
/// certified cell stands over them, so the orbit's circuit crosses it. The
/// second body stands at `placed`, so the circuit's composite is the chord
/// conjugated by it.
fn held_form_over_a_cell(chord: &Word, placed: &Word) -> Form {
    let one = num_rational::BigRational::from_integer(1.into());
    let zero = num_rational::BigRational::from_integer(0.into());
    let mut f = Form::default();
    for tag in [2u8, 3u8] {
        let carriage = match tag {
            2 => element::carriage(&one, &(zero.clone(), zero.clone())),
            _ => element::versor_of_word(placed).expect("a placement is a versor"),
        };
        f.vertices.push(Vertex::word_on(
            crate::transducer::alphabet::Derivation::spelled(Word::identity()),
            crate::transducer::stalk::Invariant::counts(&[u64::from(tag)]),
            Box::new(carriage),
            vec![(1, 2)],
            None,
        ));
    }
    f.faces.push(f.face_of(0, 1).expect("star 1"));
    f.faces.push(f.face_of(0, 2).expect("star 2"));
    f.faces.push(f.face_carrying(1, 2, chord.clone()).expect("a chord between two bodies"));
    // The chord between the two bodies carries its edge (50.3): the chord
    // word's versor, as pairing leaves it on a perceived form.
    let carriage = element::versor_of_word(chord).expect("a quarter turn is a versor");
    if let Some(face) = f.faces.get_mut(2) {
        let at: Vec<Vec<u8>> = face.at.scoped().map(|(k, _)| k).collect();
        for k in at {
            if let Some(place) = face.at.get_mut(&k) {
                place.edge = Some(crate::transducer::sentence::Transport {
                    word: chord.clone(),
                    carriage: carriage.clone(),
                    gauge: place.gauge.clone(),
                });
            }
        }
    }
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    let (a, b) = (vec![0xa], vec![0xb]);
    for face in 0..f.faces.len() as u32 {
        f.face_bodies.insert(face.to_be_bytes(), (a.clone(), b.clone()));
    }
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let nuclei = vec![
        crate::transducer::cga::Mv::blade(crate::transducer::cga::E1, q(1)),
        crate::transducer::cga::Mv::blade(crate::transducer::cga::E2, q(1)),
    ];
    f.cells.insert(vec![0xc], FormCell { bodies: vec![a, b], witnesses: Vec::new(), nuclei: Some(nuclei) });
    f
}

/// Pivot one held form under a crystal offering `offered`, and read the tally.
fn pivot(form: Form, offered: &Word) -> crate::transducer::sentence::PivotTally {
    let mut c = Contemplation::new();
    c.hold_form(form, &[], &[], [0; 4]);
    let k = c.bubbles.scoped().next().map(|(k, _)| k).expect("held");
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 12);
    let mut tally = crate::transducer::sentence::PivotTally::default();
    if let Some(b) = c.bubbles.get_mut(&k) {
        let atlas = crate::transducer::stalk::Stalks::default();
        let _ = b.expand_with(&atlas, &|_| None, 0, &OneWord(offered.clone()), &mut budget, &mut tally);
    }
    tally
}

/// A HELD FORM'S PIVOT READ AGAINST THE CELL ITS ORBIT CROSSES. With both
/// bodies at the origin the circuit's composite is the chord's quarter turn
/// in `e₁₂`, the cell's closed algebra is that turn's (rank 3), and the move
/// that discharges the orbit — the same turn — is already in it: DERIVABLE.
/// With the second body placed by a quarter turn in `e₂₃` the composite is
/// the chord's turn carried to another plane, and the same discharging move
/// raises the cell's algebra: REACHED. Note only: the pivot makes the same
/// move with the reading as without it.
#[test]
fn a_pivot_reads_its_move_against_the_cells_its_orbit_crosses() {
    let w = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("a quarter on e12"), false)]);
    let e23 = Word::from_letters(&[(LetterId::quarter((1, 3)).expect("a quarter on e23"), false)]);

    let at_origin = held_form_over_a_cell(&w, &Word::identity());
    let cells = crate::transducer::family::pivot_cells(&at_origin, &mut crate::physics::Physics::default().stroke_n(1 << 12));
    let [cell] = cells.algebras.as_slice() else { panic!("one certified cell") };
    assert_eq!(cell.capacity, Ok(3), "the chord's quarter turn: rank three");
    let derivable = pivot(at_origin.clone(), &w);
    assert_eq!(
        derivable,
        crate::transducer::sentence::PivotTally { derivable: 1, ..Default::default() },
        "the discharging move is already in the cell's algebra"
    );

    let placed = held_form_over_a_cell(&w, &e23);
    let reached = pivot(placed, &w);
    assert_eq!(
        reached,
        crate::transducer::sentence::PivotTally { reached: 1, ..Default::default() },
        "the same move raises the algebra of a cell whose composite turns in another plane"
    );

    // NOTE ONLY: the snaps are the pivot's with the reading or without it.
    let horizon = crate::network::nerve::Keyed::new();
    let budget = || crate::physics::Physics::default().stroke_n(1 << 12);
    let (mut plain, mut read) = (at_origin.clone(), at_origin);
    let without = plain.pivot_orbit(b"idea", &horizon, &OneWord(w.clone()), &mut budget(), None, 0);
    let mut tally = crate::transducer::sentence::PivotTally::default();
    let with = read.pivot_orbit_read(b"idea", &horizon, &OneWord(w), &mut budget(), None, 0, Some(&cells), &mut tally);
    assert_eq!(with, without, "the reading changes no move");
    assert_eq!(tally.derivable, 1, "and reads the move it did not change");
}
