// 12.6, A20 — THE PIVOT LOOP: what a pivot discharges, what it holds as
// a vortex, and what it writes. Split from `pivot.rs` at the §1.1
// 600-line reading bound when the `Crystal` fixture grew the two reads
// of 50.2 step 2; the items are moved, not rewritten (§1.6).

use super::common::*;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::Word;
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::*;
use crate::transducer::stalk::{self, Standing};

/// The crystal both pivot files hand the loop: the words a mind holds on
/// an axis. It lives here with the tests that exercise the loop; `pivot.rs`
/// reads it across the split.
pub(super) struct Hold(pub Vec<Word>);
impl crate::transducer::family::Crystal for Hold {
    fn on_class(
        &self,
        _class: &stalk::Invariant,
    ) -> Vec<(
        Element,
        Form,
        Standing,
    )> {
        Vec::new()
    }
    fn on_body(&self, c: &stalk::Invariant) -> Vec<(Element, Form, Standing)> {
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
    fn by_address(&self, _a: &[u8]) -> Option<(Element, Form)> {
        None
    }
    fn on_axis(&self, _chart: &stalk::Invariant, axis: Axis) -> Vec<Word> {
        self.0
            .iter()
            .filter(|w| ClassVector::of_word(w).is_some_and(|cv| cv.occupies(axis)))
            .cloned()
            .collect()
    }
    /// Words on axes, which is the pivot's vocabulary and not a
    /// presentation: these words live in no form, so nothing here can be
    /// cited by (form, vertex).
    fn generators_under(
        &self,
        _chart: &stalk::Invariant,
        _budget: &mut crate::physics::Budget,
    ) -> Vec<(
        crate::transducer::address::Address,
        crate::transducer::address::Address,
        Word,
    )> {
        Vec::new()
    }
}

#[test]
fn pivot_loop_discharges_held_word_and_vortices_a_pair() {
    let q = order4();
    let qinv = q.inverse();
    let mut f = chord_form(std::slice::from_ref(&q));
    let snaps = f.pivot_loop(
        [0x11; 32],
        &Keyed::new(),
        &Hold(vec![q.clone()]),
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    let Some(Snap::Discharged { certificate, .. }) = snaps.iter().find(|s| matches!(s, Snap::Discharged { .. })) else {
        panic!("a crystal holding q discharges: {snaps:?}");
    };
    assert_eq!(
        certificate.before,
        ClassVector::of_word(&q).expect("q").class_key()
    );
    assert_eq!(
        certificate.after,
        ClassVector::of_word(&Word::identity()).expect("identity").class_key()
    );
    // S9.1: the certificate states the MOVE — one unit out of torsion —
    // rather than two audits for a reader to subtract for itself.
    assert_eq!(
        certificate.rank_move,
        RankMove::Discharged { from: RankAxis::Torsion },
        "the loop's finite order left torsion and an axis opened"
    );
    assert!(
        f.certificates.contains(&certificate.key()),
        "the form carries the discharge by key"
    );
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("the discharged form crystallizes");
    assert!(
        matches!(f.vertices.first(), Some(Vertex::Frame)),
        "Frame is vertex 0 of the crystallized form"
    );
    assert_eq!(
        crate::court::verdict::blind_replay(&e),
        None,
        "a certified discharge replays: every after-face is face_of, then the certificate"
    );
    let mut tampered = e.form().cloned().expect("v2 tape");
    let fi = tampered
        .certificates
        .fiber(crate::network::nerve::ROOT)
        .next()
        .expect("one certificate")
        .1
        .by
        .face() as usize;
    tampered.faces[fi].transport.word = q.clone();
    let bad = as_element_form(&tampered, e.magnitude()).expect("tampered tape");
    assert_eq!(
        crate::court::verdict::blind_replay(&bad),
        Some(crate::court::verdict::REFUSE_LOOP),
        "tampering a certified face re-opens an unfiled loop"
    );

    let mut f = chord_form(std::slice::from_ref(&q));
    let snaps = f.pivot_loop(
        [0x22; 32],
        &Keyed::new(),
        &Hold(vec![q.clone(), qinv]),
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    let Some(Snap::Unmeasured { held, .. }) = snaps.iter().find(|s| matches!(s, Snap::Unmeasured { .. })) else {
        panic!("q and q⁻¹ unmeasured are a vortex: {snaps:?}");
    };
    assert_eq!(held.len(), 2, "two certificates, got {held:?}");
    assert!(!f.vortex.is_empty(), "the form holds the vortex cells");

    let mut f = chord_form(std::slice::from_ref(&q));
    let mut horizon = Keyed::new();
    let r = q.rotor();
    horizon.insert(r.class_key().expect("q class"), (r, q.order(), Standing::Held));
    let snaps = f.pivot_loop(
        [0x33; 32],
        &horizon,
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    assert!(snaps.is_empty(), "a horizon holding q files silently: {snaps:?}");
    assert!(
        q.rotor().class_key().is_some_and(|k| f.torsion.contains(k.as_bytes())),
        "filed torsion is keyed by the class"
    );
}

#[test]
fn pivot_writes_the_chord_transport_only() {
    let q = order4();
    let mut f = chord_form(std::slice::from_ref(&q));
    let verts_before = f.vertices.clone();
    let chord = f.loops[0].chord;
    let snaps = f.pivot_loop(
        [0xc0; 32],
        &Keyed::new(),
        &Hold(vec![q.clone()]),
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    assert!(
        snaps.iter().any(|s| matches!(s, Snap::Discharged { .. })),
        "a held quarter discharges: {snaps:?}"
    );
    assert_eq!(
        f.vertices, verts_before,
        "a pivot writes no vertex placement or gauge"
    );
    for (fi, face) in f.faces.iter().enumerate() {
        if fi as u16 == chord {
            continue;
        }
        assert_eq!(
            f.face_of(face.i, face.j).as_ref(),
            Some(&face.transport),
            "tree face {fi} re-derives"
        );
    }
}

#[test]
fn pivot_discharges_only_its_axis_and_raises_none() {
    // T14: Placement.
    let mut f = chord_form(&[Word::identity()]);
    for fa in &mut f.faces {
        fa.transport.gauge.clear();
    }
    let chord = f.loops[0].chord as usize;
    f.faces[chord].transport.carriage = element::carriage(&q1(), &(q1(), q0()));
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    let before = f.class_vector(0).expect("loop");
    assert!(before.occupies(Axis::Placement));
    let _ = f.pivot_loop(
        [0xb4; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    let after = f.class_vector(0).expect("loop remains");
    assert!(!after.occupies(Axis::Placement), "Placement zeroes");
    assert!(
        before.agrees_except(&after, Axis::Placement),
        "no other coordinate moved"
    );

    // T14: Orientation.
    let mut f = chord_form(&[order2()]);
    for fa in &mut f.faces {
        fa.transport.gauge.clear();
    }
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    let before = f.class_vector(0).expect("ori");
    assert!(before.occupies(Axis::Orientation(0)), "got {:?}", before.coordinates());
    let _ = f.pivot_loop(
        [0xb5; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    let after = f.class_vector(0).expect("ori after");
    assert!(
        !after.occupies(Axis::Orientation(0)),
        "Orientation zeroes, got {:?}",
        after.coordinates()
    );
    assert!(before.agrees_except(&after, Axis::Orientation(0)));

    // T14: Dilation.
    let mut f = chord_form(&[Word::identity()]);
    for fa in &mut f.faces {
        fa.transport.gauge.clear();
    }
    let chord = f.loops[0].chord as usize;
    let two = num_rational::BigRational::from_integer(num_bigint::BigInt::from(2));
    f.faces[chord].transport.carriage = element::carriage(&two, &(q0(), q0()));
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    let before = f.class_vector(0).expect("dil");
    assert!(
        before.occupies(Axis::Dilation),
        "a dilator residue occupies Dilation, got {:?}",
        before.coordinates()
    );
    let _ = f.pivot_loop(
        [0xb6; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    let after = f.class_vector(0).expect("dil after");
    assert!(!after.occupies(Axis::Dilation), "Dilation zeroes, got {:?}", after.coordinates());
    assert!(before.agrees_except(&after, Axis::Dilation));
}

/// 12.6, T21 (iii), S9.1 — A PIVOT MOVES ONE UNIT OR NONE. `Rank::moved`
/// names the move: the rank held, or exactly one unit left torsion, free or
/// off-plane for `axes`. Everything else is `Inadmissible` and carries its
/// four deltas so the refusal can say what happened.
///
/// The predicate this replaces was a four-coordinate monotone, and it
/// admitted `(−3, −2, −1, +7)` — three coordinates emptied and seven axes
/// opened — as readily as A18's one step. That is not a pivot that went
/// well; it is a different thing the audit could not name.
#[test]
fn a_rank_move_is_one_unit_or_none() {
    let before = Rank { torsion: 3, free: 2, offplane: 1, axes: 0 };

    // A18's step, from each coordinate it can leave: one unit, whether or
    // not the discharge opened a chart axis for it to land in. Today none
    // does — `rank_of_rows` writes `axes: 0` unconditionally — so both
    // shapes are asserted and the second is the one every live pivot makes.
    for (after, from) in [
        (Rank { torsion: 2, free: 2, offplane: 1, axes: 1 }, RankAxis::Torsion),
        (Rank { torsion: 2, free: 2, offplane: 1, axes: 0 }, RankAxis::Torsion),
        (Rank { torsion: 3, free: 1, offplane: 1, axes: 1 }, RankAxis::Free),
        (Rank { torsion: 3, free: 2, offplane: 0, axes: 0 }, RankAxis::OffPlane),
    ] {
        assert_eq!(Rank::moved(before, after), RankMove::Discharged { from });
    }
    // A pivot that leaves the audit alone is certified by its closure.
    assert_eq!(Rank::moved(before, before), RankMove::Equal);
    // Everything else is named, with its deltas: two units out of torsion,
    // however welcome the fall looks; the sprawl the old monotone admitted;
    // an axis closing, which is a coordinate lost and not a saving.
    for (a, b, delta) in [
        (before, Rank { torsion: 1, free: 2, offplane: 1, axes: 1 }, [-2, 0, 0, 1]),
        (before, Rank { torsion: 0, free: 0, offplane: 0, axes: 7 }, [-3, -2, -1, 7]),
        (Rank { axes: 2, ..before }, Rank { axes: 0, ..before }, [0, 0, 0, -2]),
    ] {
        assert_eq!(Rank::moved(a, b), RankMove::Inadmissible { delta });
    }
}
