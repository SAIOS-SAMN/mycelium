// 78.7 as amended, VS1 — a face of a held loop bent on two planes, the loop
// re-read against the flat form's: inviscid, elastic, curved, viscous.

use super::common::*;
use crate::transducer::alphabet::Word;
use crate::transducer::cga::{self, Mv};
use crate::transducer::sentence::*;
use crate::transducer::stalk;

fn q(n: i64) -> num_rational::BigRational {
    num_rational::BigRational::from_integer(n.into())
}

/// Two bodies on the Frame's star and a chord between them carrying `edge`.
fn held(edge: Transport) -> Form {
    let mut f = Form::default();
    for n in 1..=2u8 {
        f.vertices.push(word_vertex_on(n, Word::identity(), stalk::Invariant::counts(&[u64::from(n)])));
    }
    f.faces.push(f.face_of(0, 1).expect("star a"));
    f.faces.push(f.face_of(0, 2).expect("star b"));
    f.faces.push(f.face_carrying_with_edge(1, 2, Word::identity(), Some(edge)).expect("a chord"));
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    assert_eq!(f.orbits.len(), 1, "the fixture must hold one loop, or nothing is tested");
    f
}

fn totals(rs: &[PairViscosity], lane: BendLane) -> [u64; 4] {
    rs.iter().filter(|r| r.lane == lane).fold([0; 4], |mut t, r| {
        for (a, b) in t.iter_mut().zip(r.arms) {
            *a += b;
        }
        t
    })
}

/// A FLAT FORM READS INVISCID OR ELASTIC ON EVERY PAIR OF PLANES, WITH ZERO
/// ENTROPY: its loop carries only a hue, so every bend's loop reads the flat
/// loop's own class, by construction.
#[test]
fn a_flat_form_reads_inviscid_or_elastic_everywhere() {
    let flat = held(Transport { word: Word::identity(), carriage: crate::transducer::element::identity_versor(), gauge: vec![(0, 1), (1, 0)] });
    let before = crate::physics::metabolism::metabolism().viscous;
    let rs = read_viscosity(&flat, &mut crate::physics::Physics::default().stroke_n(1 << 16));
    for lane in [BendLane::Spatial, BendLane::Holonomic] {
        let [_, _, curved, viscous] = totals(&rs, lane);
        assert_eq!((curved, viscous), (0, 0), "{lane:?}: nothing curved, nothing viscous");
    }
    assert!(totals(&rs, BendLane::Spatial)[1] > 0, "the bends move the class and the loop reads the flat loop");
    assert_eq!(crate::physics::metabolism::metabolism().viscous, before, "no entropy booked");
}

/// A SCREW READS CURVED ON THE PLANE ITS LOOP TURNS IN: the screw turns in
/// `e₁₂` and slips along `e₃`; the quarter turns on `e₁₃` and `e₂₃` loop to a
/// `K` turning in `e₁₂`, the screw's own plane, and the loop reads a class
/// the flat loop does not — the form's curvature felt through the bend. A
/// pair of boosts on `e₁₋` and `e₂₋`, planes the screw does not couple to,
/// reads no curvature. Nothing is viscous and no entropy is booked.
#[test]
fn a_screw_reads_curved_on_the_plane_its_loop_turns_in() {
    use cga::{E1, E2, E3, EM};
    let screw = cga::rotator(&q(1), &q(1)).geo(&cga::translator_at(&[q(0), q(0), q(1)]));
    let form = held(Transport { word: Word::identity(), carriage: screw, gauge: Vec::new() });
    let before = crate::physics::metabolism::metabolism().viscous;
    let rs = read_viscosity(&form, &mut crate::physics::Physics::default().stroke_n(1 << 16));
    let arms = |a: usize, b: usize| {
        rs.iter().find(|r| r.lane == BendLane::Spatial && r.planes == (a, b)).map(|r| r.arms).expect("the pair is read")
    };
    assert!(arms(E1 | E3, E2 | E3)[2] > 0, "e₁₃·e₂₃ loops through e₁₂, where the screw turns: curved");
    assert_eq!(arms(E1 | EM, E2 | EM)[2], 0, "two boosts the screw does not couple to: not curved");
    assert_eq!(totals(&rs, BendLane::Spatial)[3], 0, "curved is not viscous");
    assert_eq!(crate::physics::metabolism::metabolism().viscous, before, "no entropy booked");
}

/// ITEM 6 — A BEND WITH NO INVERSE IS REFUSED BY NAME: the un-pivot of a
/// placement whose translator is on the light cone writes nothing and says
/// so, where it used to skip silently.
#[test]
fn a_move_with_no_inverse_is_refused_never_skipped() {
    let mut form = held(Transport { word: Word::identity(), carriage: crate::transducer::element::identity_versor(), gauge: vec![(0, 1), (1, 0)] });
    let null = Mv::scalar(q(1)).add(&Mv::blade(cga::E1 | cga::EM, q(1)));
    let by = PivotBy::Placement { face: 0, translator: null };
    assert!(!form.unapply_move(&by), "no inverse, no un-pivot");
}
