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
///
/// AN EDGE IS ITS OWN OBJECT, and content addressing is what says so. A
/// `Glued` vertex writes the cited form's address INTO the citing form's
/// key, so `key(U) ⊃ key(V)` strictly: two GIVEN forms can never cite each
/// other, whatever any guard says, because each key would have to contain
/// the other. The overlap of `a` and `b` is therefore recorded in two
/// further objects — `a` with the edge to `b`, and `b` with the edge to `a`
/// — which is exactly a nerve's own shape, where a 1-simplex is an object
/// and not a pointer stored inside a 0-simplex.
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

/// THE COCYCLE CONDITION ON A 2-SIMPLEX OF THE CITATION NERVE:
/// `φ_ij · φ_jk · φ_ki = 1`.
///
/// This is what replaces the depth refusal. A cycle is not forbidden; it is
/// MEASURED, and what it measures is the holonomy of the loop it closes. A
/// triangle whose composite transition is the identity versor GLUES — the
/// three charts agree on their triple overlap and carry one section — and
/// one whose composite is not the identity carries that composite as its
/// obstruction. That composite is the content of `H²`, and it is the thing
/// a posetal height can never express: a height orders charts, and an
/// ordering has no triangles to be obstructed on.
///
/// THE TRANSITIONS ARE READ BACK OUT OF THE ADDRESSES, not held in local
/// variables and multiplied. Each edge of the triangle is a form citing the
/// next chart under its φ; each is addressed; each address is decoded by
/// `from_address`; and the three φ the DECODER returns are what compose
/// here. A test that multiplied the three versors it had just written would
/// prove a fact about `Versor::geo` and nothing about this engine — the
/// claim is that the morphism survives the coordinate, which is only
/// testable through it.
#[test]
fn a_triangle_of_citations_closes_or_carries_its_holonomy() {
    use crate::transducer::element::{identity_versor, same_versor, translator_versor, Versor};

    // Three charts of a cover, and the three transitions around their
    // triple overlap. Composed, the translations cancel exactly over ℚ:
    // `T(3,0) · T(0,5) · T(−3,−5) = 1`. Nothing is rounded and nothing is
    // near: the identity is the identity (rule 11).
    let charts = [leaf((0, 1)), leaf((2, 3)), leaf((1, 2))];
    let phi_ij = translator_versor(3, 0);
    let phi_jk = translator_versor(0, 5);
    let phi_ki = phi_ij.geo(&phi_jk).versor_inverse().expect("a translator inverts");

    // ONE EDGE OF THE NERVE: chart `i` with its citation of chart `j` under
    // φ. The edge is its own object, because a content address cannot
    // contain itself.
    let edge = |i: usize, j: usize, phi: &Versor| -> Form {
        let mut e = charts[i].clone();
        e.glue_under(
            &charts[j],
            addr_of(&charts[j]),
            Word::identity(),
            [0, 0, 0, 1],
            phi.clone(),
        )
        .expect("an edge of the nerve");
        e
    };

    // AND THE TRANSITION THE EDGE'S ADDRESS CARRIES, recovered by the only
    // decoder there is.
    let transition_of = |e: &Form| -> Versor {
        let key = form_vertex_key(e);
        let back = from_address(key.as_bytes()).expect("the edge's address decodes");
        back.vertices
            .iter()
            .find_map(|v| match v {
                Vertex::Glued { carriage, .. } => Some((**carriage).clone()),
                _ => None,
            })
            .expect("a nerve edge carries its morphism")
    };

    let round = [
        transition_of(&edge(0, 1, &phi_ij)),
        transition_of(&edge(1, 2, &phi_jk)),
        transition_of(&edge(2, 0, &phi_ki)),
    ];
    let closes = round[0].geo(&round[1]).geo(&round[2]);
    assert!(
        same_versor(&closes, &identity_versor()),
        "the triple overlap glues: δφ = 1 around the 2-simplex, read off the three addresses"
    );

    // AND A TRIANGLE THAT DOES NOT CLOSE carries its composite, exactly.
    // The same three charts with one edge displaced by `T(1,0)` compose to
    // that displacement and to nothing else: the obstruction IS the
    // coordinate, never a flag saying one was found (`curvature is
    // content`).
    let bent = transition_of(&edge(0, 1, &phi_ij.geo(&translator_versor(1, 0))));
    let holonomy = bent.geo(&round[1]).geo(&round[2]);
    assert!(!same_versor(&holonomy, &identity_versor()), "a bent triangle does not glue");
    assert!(
        same_versor(&holonomy, &translator_versor(1, 0)),
        "and what it fails to glue BY is the holonomy it carries"
    );
}
