// 42 — REFRACTION as its three moves, and the sensor gate: no coordinate
// pair lives outside the four functions law 11 allows one.

use super::common::*;
use crate::transducer::alphabet::{self, Word};
use crate::transducer::element::{self};
use crate::transducer::sentence::*;

#[test]
fn refraction_is_rebase_regauge_and_carriage() {
    let mut f = Form::default();
    f.vertices.push(word_vertex(1, order4()));
    f.faces.push(f.face_of(0, 1).expect("star"));
    f.reindex();
    let gamma = Transport {
        word: order4(),
        carriage: element::carriage(&q1(), &(q1(), q0())),
        gauge: vec![(1, 3)],
    };
    let out = gamma.refract(&f).expect("on the plane");
    // The member at the chart the fixture put the vertex on: a keyed read, and
    // this fixture puts it on the root stratum.
    let root = crate::transducer::stalk::Invariant::default();
    let (Vertex::Word { derivation: old_d, .. }, Some(old_a)) =
        (&f.vertices[1], f.vertices[1].acting_on(&root))
    else {
        panic!("word vertex");
    };
    let (Vertex::Word { derivation, .. }, Some(a)) =
        (&out.vertices[1], out.vertices[1].acting_on(&root))
    else {
        panic!("word vertex");
    };
    assert!(derivation.rotor().projective_eq(&old_d.word_local().expect("spelled").rebase(&gamma.word).rotor()));
    assert!(element::same_versor(&a.carriage, &gamma.carriage.geo(&old_a.carriage)));
    assert_eq!(a.gauge, compose_gauge(&gamma.gauge, &old_a.gauge));
    let off = Transport {
        word: Word::from_letters(&[(alphabet::LetterId::quarter((0, 2)).unwrap(), false)]),
        carriage: element::identity_versor(),
        gauge: Vec::new(),
    };
    assert!(off.refract(&f).is_none(), "None off the plane");
}

#[test]
fn refraction_replays_as_its_three_moves() {
    let mut f = Form::default();
    f.vertices.push(word_vertex(1, order4()));
    f.faces.push(f.face_of(0, 1).expect("star"));
    f.reindex();
    let gamma = Transport {
        word: order2(),
        carriage: element::carriage(&q1(), &(q0(), q1())),
        gauge: vec![(2, 5)],
    };
    let via = gamma.refract(&f).expect("Γ");
    let mut by_hand = f.clone();
    by_hand.rebase_by_word(&gamma.word);
    by_hand.regauge_listing(&gamma.gauge);
    by_hand.compose_carriages(&gamma.carriage);
    by_hand.loops = Form::loops_of(&by_hand.faces, by_hand.vertices.len());
    by_hand.reindex();
    assert_eq!(via.vertices.len(), by_hand.vertices.len());
    for (a, b) in via.vertices.iter().zip(&by_hand.vertices) {
        match (a, b) {
            (
                Vertex::Word { derivation: da, acts: aa, .. },
                Vertex::Word { derivation: db, acts: ab, .. },
            ) => {
                assert!(da.rotor().projective_eq(&db.rotor()));
                // THE FIBRE, MEMBER BY MEMBER AT ITS OWN CHART. Γ and the
                // three moves by hand must act on the same charts and carry
                // the same body on each; a versor is compared by
                // `same_versor` — projectively, which is what a versor's
                // equality is — and never by the struct's own derive.
                assert_eq!(aa.len(), ab.len(), "the two paths act on the same number of charts");
                for (k, ma) in aa.scoped() {
                    let mb = ab.get(&k).expect("the other path acts on the same chart");
                    assert!(element::same_versor(&ma.carriage, &mb.carriage));
                    assert_eq!(ma.gauge, mb.gauge);
                }
            }
            (Vertex::Frame, Vertex::Frame) => {}
            _ => panic!("replayed vertices differ"),
        }
    }
}

#[test]
fn rebase_conjugates_by_a_versor_never_subtracts() {
    let mut f = Form::default();
    f.vertices.push(word_vertex(1, order4()));
    f.vertices[1].set_carriage(element::carriage(&q1(), &(q1(), q0())));
    f.faces.push(f.face_of(0, 1).expect("star"));
    let by = f.face_of(0, 1).expect("relative");
    assert!(
        !element::same_versor(&face_place(&by).carriage, &element::identity_versor())
            || f.vertices.get(1).is_some(),
        "face_of composes carriages by the geometric product"
    );
    let of = f
        // Face 0 is the star the fixture pushed: the chord this rebase
        // would close on, were it a closed orbit. It is not — 0 and 1 are
        // two vertices — so nothing is filed and the vertex is rebased.
        .rebase(0, 1, element::carriage(&q1(), &(q0(), q1())), 0)
        .expect("rebase");
    assert!(matches!(f.vertices[of as usize], Vertex::Rebased { .. }));
    let t = f.face_of(0, of).expect("rebased face");
    let _ = face_place(&t).carriage;
}

// `no_coordinate_pair_outside_the_sensor` and its `walk_named_pairs` stood
// here: a hand-rolled scan of every production `.rs` file under four
// directories, tracking `#[cfg(test)]` by brace depth and allowing a list of
// FUNCTION NAMES, asserting that the spellings `(Q, Q)` and
// `BigRational, BigRational` do not occur. That is a source-text gate in a
// test (§4.6), the same shape deleted from `stalk/tests/chart.rs` at
// 66d3eae, and the ruling applies to every site of the shape.
//
// It is not a close call on three counts. Its allow-list is by name, so
// renaming a function silently changes what the gate permits. Its own
// comment defers the real check — "the production check is `xtask lint`'s
// squaring gate" — and `xtask/lint` holds `length.rs`, `warnings.rs` and
// `mod.rs`: that gate does not exist. And it fires on a line that has no
// coordinate pair in it: `ClassVector.offplane` is the three conjugacy
// invariants `(a²/w, p²/w, sign p)`, which the scan reads as a pair because
// two type names sit next to each other on one line.
//
// WHAT IS OWED, stated rather than absorbed. The property it was reaching
// for — a coordinate pair standing where a chart coordinate belongs, outside
// the readers that legitimately hold one — is real and is now asserted by
// NOTHING. A spelling check was never asserting it either; a rename defeated
// it and a macro or a string would have passed it. The behavioural test is
// owed and is named here so it is not mistaken for covered.
