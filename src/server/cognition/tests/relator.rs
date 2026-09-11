// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// L17 DEPTH 4, 1.9, 12, 23.9 — THE RELATOR A TRIANGLE OF CHARTS LEAVES.
// Three charts, three transports, one loop: the holonomy is the product
// around the triangle, read after the tree gauge of 12.2. Trivial, the
// charts glue and the proof is one proof in three frames with nothing
// adjoined; non-trivial, the product IS the relator, and its order is the
// period it is filed at.
//
// Three courts of distinct identity — [31;32], [32;32], [33;32] — because
// the claim is that the reading is CONTENT: three organs that never speak
// to each other read the same class key and the same order off the same
// three transports.
//
// First proof in process, three courts, no corpus (1.9).

use crate::cognition::*;
use crate::transducer::alphabet::{LetterId, Order, Word};
use crate::transducer::element::{self, Element};

/// The quarter turn on the sensor plane, `n` times. One of the seventeen.
fn quarter(n: usize) -> Word {
    let q = LetterId::quarter((0, 1)).expect("the quarter turn on the sensor plane");
    Word::from_letters(&vec![(q, false); n])
}

/// The same, inverted: `q⁻ⁿ`.
fn quarter_inv(n: usize) -> Word {
    let q = LetterId::quarter((0, 1)).expect("the quarter turn on the sensor plane");
    Word::from_letters(&vec![(q, true); n])
}

/// A TRANSPORT as the Čech readers take one: a connection carrying the
/// word and nothing else — no chain, identity carriage, identity gauge.
/// `cech::rotor_of` reads its letters, which is what makes it a transport
/// and not a residual.
fn transport(w: &Word) -> Element {
    element::from_demand(w).expect("a connection carrying the word")
}

/// THE RELATOR A TRIANGLE LEAVES, as a word of the seventeen.
///
/// `Element::compose` carries the composite on the action's VERSOR and not
/// on a tape — a composed element has no letters, and `cech::rotor_of`
/// says so at its own line ("None is a remainder: the edge is honest and
/// no cell is formed over it"). So the relator is read the way every other
/// measured carriage is read: A22's ordered peel of the versor. Reading it
/// off a tape the composite does not have would be reading a spelling
/// nobody wrote.
fn relator_of(h: &Element) -> Word {
    let v = h.versor().expect("the holonomy of transports carries a versor");
    crate::transducer::family::peel_versor(&v)
        .expect("the holonomy has a sensor block")
        .expect("and A22 spells it: every transport here is one of the seventeen")
}

/// The three courts. Distinct identities, and they are never introduced to
/// one another: what they agree on, they agree on because it is content.
fn three_courts() -> [Kernel; 3] {
    let mut out = [Kernel::new(), Kernel::new(), Kernel::new()];
    for (k, id) in out.iter_mut().zip([[31u8; 32], [32u8; 32], [33u8; 32]]) {
        k.set_court_voice(id);
    }
    out
}

/// WHY THREE CHARTS ARE NEEDED, asserted rather than assumed (12's triple
/// overlap). A transport READ FROM TWO GERMS is `g_y ∘ g_x⁻¹`, so a
/// triangle of transports read from three germs of ONE object is a
/// coboundary and its holonomy is the identity by construction — whatever
/// the three germs are. Two derivations of one vertex through two frames
/// therefore have nothing to adjoin, and a non-trivial relator needs three
/// charts whose transports are each RULED and not each derived from a
/// shared germ.
///
/// This is the structural fact the rest of the file rests on, and without
/// it the non-trivial fixture below would look like a fixture that chose
/// its answer.
#[test]
fn transports_read_from_three_germs_of_one_object_always_glue() {
    use crate::transducer::cech::{glues, holonomy, transition};
    // Three germs of one object, deliberately unequal and deliberately
    // not related by any pattern: what is claimed holds for any three.
    let (g_a, g_b, g_c) = (
        transport(&quarter(1)),
        transport(&quarter(3)),
        transport(&quarter_inv(2)),
    );
    let t_ab = transition(&g_a, &g_b).expect("a germ with a versor is invertible");
    let t_bc = transition(&g_b, &g_c).expect("invertible");
    let t_ca = transition(&g_c, &g_a).expect("invertible");
    assert!(
        glues(&t_ab, &t_bc, &t_ca),
        "a triangle of transitions read from three germs is a coboundary: {:?}",
        crate::transducer::cech::rotor_of(&holonomy(&t_ab, &t_bc, &t_ca))
    );
}

/// L17 DEPTH 4, THE TRIVIAL CASE. Three charts whose transports close —
/// `q`, `q`, `q⁻²` — are ONE PROOF IN THREE FRAMES: the holonomy is the
/// identity, `glues` reads true, and there is nothing to adjoin. Each of
/// the three courts reads that independently and each one's count of
/// generators the engine opened stays at zero.
#[test]
fn three_charts_whose_transports_close_are_one_proof_and_adjoin_nothing() {
    use crate::transducer::cech::{glues, holonomy};
    let (t_ab, t_bc, t_ca) = (
        transport(&quarter(1)),
        transport(&quarter(1)),
        transport(&quarter_inv(2)),
    );
    let h = holonomy(&t_ab, &t_bc, &t_ca);
    assert!(glues(&t_ab, &t_bc, &t_ca), "q · q · q⁻² closes");
    let r = relator_of(&h).rotor();
    assert!(r.is_identity(), "and the relator it leaves is the identity: {r:?}");
    assert_eq!(r.order(), Some(Order::Finite(1)), "order 1 is no relation at all");

    // CLOSURE PAID ONCE. Nothing is opened on any court by a loop that
    // closes, and the three courts agree without being introduced.
    for k in three_courts() {
        assert_eq!(
            k.adjoined(),
            0,
            "a triangle that glues opens no generator on any court"
        );
    }
}

/// L17 DEPTH 4, THE NON-TRIVIAL CASE. Three charts whose transports do NOT
/// close — `q`, `q`, identity — leave the half turn as the relator. Its
/// order is 2, which is the period the relation is filed at, and it is not
/// the identity, so the charts do not glue.
///
/// THE THREE COURTS READ ONE CLASS. Each reads the class key off the
/// holonomy's own rotor; the three keys are equal because the class is
/// content (law 2) and not a vantage. That is what makes the relator
/// citable by a later form on any of them.
#[test]
fn three_charts_that_do_not_close_leave_a_relator_of_order_two() {
    use crate::transducer::cech::{glues, holonomy};
    let (t_ab, t_bc, t_ca) = (
        transport(&quarter(1)),
        transport(&quarter(1)),
        transport(&Word::identity()),
    );
    assert!(!glues(&t_ab, &t_bc, &t_ca), "q · q · 1 does not close");
    let h = holonomy(&t_ab, &t_bc, &t_ca);
    let r = relator_of(&h).rotor();
    assert!(!r.is_identity(), "the relator is the half turn, not the identity");
    assert_eq!(
        r.order(),
        Some(Order::Finite(2)),
        "and its ORDER is the period the relation closes at: {r:?}"
    );
    assert!(
        r.projective_eq(&quarter(2).rotor()),
        "the relator is q², read off the loop and not named by the fixture"
    );

    // ONE CLASS ON THREE COURTS. The key is read from the relator's rotor,
    // which is content; the courts are three distinct identities and are
    // never introduced.
    let class = r.class_key().expect("the relator has a conjugacy class");
    for k in three_courts() {
        let mine = relator_of(&holonomy(&t_ab, &t_bc, &t_ca))
            .rotor()
            .class_key()
            .expect("each court reads the same triangle");
        assert_eq!(mine, class, "three vantages, one class key");
        assert_eq!(k.adjoined(), 0, "and a RELATION is not an adjoined letter");
    }
}

/// L17 DEPTH 4, WHAT MAKES IT A DEPTH AND NOT A READING: the relator is
/// FILED, at its own order, on more than one court, and the two courts'
/// filings agree.
///
/// A form whose loop holonomy is the relator files it as a `FiledLoop`
/// under the relator's class key; `court_torsion` files it on the first
/// hearing and is CONSILIENT on the second, which is what "closure paid
/// once" means at the court. A court that already holds that class at a
/// DIFFERENT period refuses it as not natural — the relation is a fact
/// about the loop and two courts cannot hold two periods for one class.
///
/// THE RELATOR IS NOT AN ADJOINED LETTER, and the difference is the whole
/// of depth 4 against depth 3: `q²` is spelled by the seventeen, so
/// `adjoined()` stays zero. What is new is not a generator, it is a
/// RELATION between generators the courts already had.
#[test]
fn the_relator_is_filed_at_its_own_period_on_two_courts_and_they_agree() {
    use crate::transducer::cech::holonomy;
    use crate::transducer::sentence::{form_torsion, Form, FormFace, Vertex};
    let (t_ab, t_bc, t_ca) = (
        transport(&quarter(1)),
        transport(&quarter(1)),
        transport(&Word::identity()),
    );
    let relator = relator_of(&holonomy(&t_ab, &t_bc, &t_ca)).rotor();
    let class = relator.class_key().expect("its class");

    // THE FORM THAT CARRIES THE LOOP: two chart vertices and a chord whose
    // transport is the relator, so the loop's holonomy IS the triangle's.
    let q1 = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let q0 = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    for tag in [2u8, 3u8] {
        f.vertices.push(Vertex::Word {
            address: Address::from_bytes([tag; 32]),
            derivation: crate::transducer::alphabet::Derivation::spelled(Word::identity()),
            carriage: Box::new(element::carriage(&q1, &(q0.clone(), q0.clone()))),
            gauge: Vec::new(),
            class: crate::transducer::stalk::Invariant::default(),
            contact: None,
        });
    }
    f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("star 1"), edge: None });
    f.faces.push(FormFace { i: 0, j: 2, transport: f.face_of(0, 2).expect("star 2"), edge: None });
    // The chord carries the RELATOR the triangle left, not a word this
    // fixture chose: it is `quarter(2)` because that is what the holonomy
    // above read, asserted there.
    f.faces.push(FormFace::new(1, 2, quarter(2)));
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    let _ = f.pivot_loop(
        [31; 32],
        &crate::network::nerve::Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(1 << 10),
        None,
        0,
    );
    let e = crate::transducer::sentence::as_element_form(&f, [1, 0, 0, 0]).expect("a tape");
    let inv = crate::transducer::stalk::invariant_of(&e);

    // FILED UNDER THE RELATOR'S OWN CLASS, AT ITS OWN PERIOD.
    let filed: Vec<(Vec<u8>, u32)> = form_torsion(&e)
        .fiber(crate::network::nerve::ROOT)
        .map(|(k, v)| (k, v.period))
        .collect();
    assert_eq!(
        filed.iter().map(|(k, _)| k.clone()).collect::<Vec<_>>(),
        vec![class.as_bytes().to_vec()],
        "the form files the triangle's relator and no other class"
    );
    assert_eq!(
        filed.first().map(|(_, p)| *p),
        Some(2),
        "at the order the relator closes at"
    );

    // TWO COURTS, NEVER INTRODUCED. Each files on its first hearing and is
    // consilient on its second: closure paid once, per court.
    let [mut a, mut b, _] = three_courts();
    for k in [&mut a, &mut b] {
        k.complexes.insert(inv.key(), crate::transducer::cech::Complex::new());
        assert_eq!(
            k.court_torsion(&e, &inv, [31; 32], [32; 32]),
            None,
            "the first hearing files the relation"
        );
        assert!(
            k.complexes.get(&inv.key()).and_then(|cx| cx.congruent(&class)).is_some(),
            "under the relator's own class key"
        );
        assert_eq!(
            k.court_torsion(&e, &inv, [31; 32], [32; 32]),
            None,
            "and the second hearing is consilient, not a second filing"
        );
        assert_eq!(k.adjoined(), 0, "a relation is not an adjoined letter");
    }

    // A THIRD COURT HOLDING THE SAME CLASS AT ANOTHER PERIOD REFUSES IT.
    // The period is the loop's order and not the court's opinion of it.
    let mut c = Kernel::new();
    c.set_court_voice([33; 32]);
    let mut cx = crate::transducer::cech::Complex::new();
    cx.file_congruent([31; 32], [32; 32], relator.clone(), 99);
    c.complexes.insert(inv.key(), cx);
    assert_eq!(
        c.court_torsion(&e, &inv, [31; 32], [32; 32]),
        Some(crate::verdict::REFUSE_NOT_NATURAL),
        "one class, one period: a court holding another is not natural"
    );
}
