// 54.2 — THE CITATION NERVE: charts do not own each other.
//
//! A form's citations were stratified by DEPTH — `glue_append_under`
//! refused a citation whose depth was not strictly below the host's — and
//! that refusal was read as an acyclicity invariant. It is not one. This
//! file holds the readings that say what it actually was and what stands
//! in its place.

use super::common::*;
use crate::transducer::alphabet::Word;
use crate::transducer::sentence::*;

/// A closed one-word form at a given quarter-turn plane, so three of them
/// are three distinct addresses.
fn leaf(plane: (u8, u8)) -> Form {
    let q = crate::transducer::alphabet::LetterId::quarter(plane).expect("quarter");
    closed_word_form(Word::from_letters(&[(q, false)]), None)
}

fn addr_of(f: &Form) -> crate::transducer::address::Address {
    as_element_form(f, [1, 0, 0, 0]).expect("tape").address()
}

/// THE DEPTH REFUSAL WAS ARRIVAL ORDER, NOT ACYCLICITY (the 63d6e40 defect,
/// in its second home).
///
/// `glue_append_under` read `host = self.depth_of()` — the host's depth AS
/// IT STANDS, which is one more than the deepest citation ALREADY appended
/// — and refused any citation not strictly below it. So the same set of
/// citations reached two outcomes depending on which was appended first:
/// deep-then-shallow admitted both, shallow-then-deep refused the second.
///
/// A DAG invariant is a fact about the citation relation and cannot depend
/// on the order the edges are written in. This one did, which is what makes
/// it a serialisation artifact rather than a topological condition.
#[test]
fn a_citation_set_is_not_an_arrival_order() {
    let a = leaf((0, 1));
    let a_at = addr_of(&a);

    // `b` is a form one level deeper: it cites `a`.
    let mut b = Form::default();
    b.glue(&a, a_at.clone(), Word::identity(), [0, 0, 0, 1]).expect("b cites a");
    let b_at = addr_of(&b);
    assert_eq!(a.depth_of(), 0, "a cites nothing");
    assert_eq!(b.depth_of(), 1, "b cites a depth-0 form");

    // ONE SET OF CITATIONS, TWO ARRIVAL ORDERS.
    let mut deep_first = Form::default();
    let ok_ba = deep_first
        .glue(&b, b_at.clone(), Word::identity(), [0, 0, 0, 1])
        .and_then(|_| deep_first.glue(&a, a_at.clone(), Word::identity(), [0, 0, 0, 1]))
        .is_some();

    let mut shallow_first = Form::default();
    let ok_ab = shallow_first
        .glue(&a, a_at, Word::identity(), [0, 0, 0, 1])
        .and_then(|_| shallow_first.glue(&b, b_at, Word::identity(), [0, 0, 0, 1]))
        .is_some();

    assert!(ok_ba, "deep-then-shallow admits both citations");
    assert!(ok_ab, "shallow-then-deep admits the same two citations");
    assert_eq!(
        form_vertex_key(&deep_first),
        form_vertex_key(&shallow_first),
        "one citation set is one form, whatever order the edges were written in"
    );
}

/// MUTUAL INCIDENCE IS THE 1-SKELETON OF A NERVE, NOT AN INVALID RECURSION.
///
/// Two charts of an open cover overlap each other; neither contains the
/// other. `U_i` citing `U_j` under φ and `U_j` citing `U_i` under φ̃ is one
/// overlap written from both sides, and the depth refusal forbade it —
/// which is why the citation nerve had no 2-simplices and `H²` could not
/// exist over it.
///
/// A citation stores its cited form's ADDRESS and not the form, so a cycle
/// is finite data and every reader of it terminates: `depth_of` reads the
/// stored `depth` field of each `Glued` vertex and never descends, and
/// `from_address` rebuilds a `Glued` vertex from the address bytes without
/// resolving them.
#[test]
fn two_charts_may_cite_each_other() {
    use crate::transducer::element::{identity_versor, translator_versor};
    let a = leaf((0, 1));
    let b = leaf((2, 3));
    let (a_at, b_at) = (addr_of(&a), addr_of(&b));

    let mut u = a.clone();
    u.glue_under(&b, b_at, Word::identity(), [0, 0, 0, 1], translator_versor(3, 0))
        .expect("U cites V");
    let mut v = b.clone();
    v.glue_under(&a, a_at, Word::identity(), [0, 0, 0, 1], identity_versor())
        .expect("V cites U — the same overlap, written from the other side");

    // Both citations stand, and each form's address carries its own edge.
    assert_ne!(form_vertex_key(&u), form_vertex_key(&v));
    let back = from_address(form_vertex_key(&u).as_bytes()).expect("U recovers");
    assert_eq!(form_vertex_key(&back), form_vertex_key(&u), "a cycle's edge re-derives");
}

/// THE COCYCLE CONDITION ON A 2-SIMPLEX: `φ_ij · φ_jk · φ_ki = 1`.
///
/// This is what replaces the depth refusal. A cycle of citations is not
/// forbidden; it is MEASURED, and what it measures is the holonomy of the
/// loop it closes. A cycle whose composite transition is the identity
/// versor glues — the three charts agree on their triple overlap and carry
/// one section — and a cycle whose composite is not the identity carries
/// that composite as its obstruction, which is the content of `H²` and is
/// the thing a posetal height can never express.
///
/// Both readings are taken here, on one triangle, so a pass cannot come
/// from a cocycle that is trivially satisfied by construction.
#[test]
fn a_triangle_of_citations_closes_or_carries_its_holonomy() {
    use crate::transducer::element::{identity_versor, same_versor, translator_versor};
    // Three transitions around a triangle. Composed, the translations
    // cancel exactly over ℚ: `T(3,0) · T(0,5) · T(−3,−5) = 1`.
    let phi_ij = translator_versor(3, 0);
    let phi_jk = translator_versor(0, 5);
    let phi_ki = phi_ij.geo(&phi_jk).versor_inverse().expect("a translator inverts");

    let closes = phi_ij.geo(&phi_jk).geo(&phi_ki);
    assert!(
        same_versor(&closes, &identity_versor()),
        "the triple overlap glues: δφ = 1 around the 2-simplex"
    );

    // AND A TRIANGLE THAT DOES NOT CLOSE carries its composite, exactly.
    // The same three charts with one edge displaced compose to that
    // displacement and to nothing else — the obstruction is the coordinate,
    // never a flag saying one was found.
    let bent = phi_ij.geo(&phi_jk).geo(&translator_versor(1, 0)).geo(&phi_ki);
    assert!(
        !same_versor(&bent, &identity_versor()),
        "a bent triangle does not glue"
    );
    assert!(
        same_versor(&bent, &translator_versor(1, 0)),
        "and what it fails to glue BY is the holonomy it carries"
    );
}
