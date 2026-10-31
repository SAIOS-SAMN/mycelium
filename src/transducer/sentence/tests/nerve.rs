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
use crate::transducer::stalk;

/// A closed one-word form at a given 90°-turn plane, so three of them
/// are three distinct addresses. The plane is one of the BASE's own —
/// `(0,1)`, `(0,3)`, `(1,3)` — since AT1 places the star overlap from the
/// word's action on the base, and a turn through `e₂` carries the base off
/// itself and places none.
fn leaf(plane: (u8, u8)) -> Form {
    let q = crate::transducer::alphabet::LetterId::turn_90(plane).expect("90° turn");
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
fn glue_key_independent_of_arrival_order() {
    let a = leaf((0, 1));
    let a_at = addr_of(&a);

    // `b` is a form one level deeper: it cites `a`.
    let mut b = Form::default();
    b.glue(&a, a_at.clone(), Word::identity(), [0, 0, 0, 1]).expect("b cites a");
    let b_at = addr_of(&b);
    assert_eq!(a.depth_of(), Some(0), "a cites nothing");
    assert_eq!(b.depth_of(), Some(1), "b cites a depth-0 form");

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
/// stored `depth` field of each `Glued` vertex and never recurses into the cited form, and
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
fn glue_under_admits_mutual_citation() {
    use crate::transducer::element::{identity_versor, translator_versor};
    let a = leaf((0, 1));
    let b = leaf((0, 3));
    let (a_at, b_at) = (addr_of(&a), addr_of(&b));

    let mut u = a.clone();
    u.glue_under(&b, b_at, Word::identity(), [0, 0, 0, 1], translator_versor(3, 0))
        .expect("U cites V");
    let mut v = b.clone();
    v.glue_under(&a, a_at, Word::identity(), [0, 0, 0, 1], identity_versor())
        .expect("V cites U — the same overlap, written from the other side");

    // Both citations stand, and each form's address carries its own edge.
    assert_ne!(form_vertex_key(&u), form_vertex_key(&v));
    let back = from_address(&form_vertex_key(&u)).expect("U recovers");
    assert_eq!(form_vertex_key(&back), form_vertex_key(&u), "a cycle's edge re-derives");
}

/// THE COCYCLE CONDITION ON A 2-SIMPLEX OF THE CITATION NERVE:
/// `φ_ij · φ_jk · φ_ki = 1`.
///
/// This is what replaces the depth refusal. A cycle is not forbidden; it is
/// MEASURED, and what it measures is the holonomy of the orbit it closes. A
/// triangle whose composite transition is the identity versor GLUES — the
/// three charts agree on the triple overlap and carry one section — and
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
fn glue_under_triangle_closes_or_carries_holonomy() {
    use crate::transducer::element::{identity_versor, same_versor, translator_versor, Versor};

    // Three charts of a cover, and the three transitions around the charts'
    // triple overlap. Composed, the translations cancel exactly over ℚ:
    // `T(3,0) · T(0,5) · T(−3,−5) = 1`. Nothing is rounded and nothing is
    // near: the identity is the identity (rule 11).
    let charts = [leaf((0, 1)), leaf((0, 3)), leaf((1, 3))];
    let phi_ij = translator_versor(3, 0);
    let phi_jk = translator_versor(0, 5);
    let phi_ki = phi_ij.geo(&phi_jk).reverse();

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
        let back = from_address(&key).expect("the edge's address decodes");
        let phi = back
            .glued_vertices()
            .find_map(|(_, v)| match v {
                Vertex::Glued { carriage, .. } => Some((**carriage).clone()),
                _ => None,
            })
            .expect("a nerve edge carries its morphism");
        phi
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

/// 1.7 PART 1 — A CITATION TRUNCATED TO ITS INTRINSIC COORDINATES NAMES
/// TWO FORMS, AND SO IS NOT A NAME.
///
/// A word's own key is 97.94% cited addresses, measured on a live genome at
/// 288 sentences: 18 313 275 bytes of 18 698 090, over 621 citations naming
/// 440 distinct forms. The cause is recursion — `A_G` writes `A_H` whole for
/// every form `G` cites, so a citation inlines its entire transitive subtree
/// and the name compounds with the citation DAG rather than with the form.
/// Each citation averages 29 490 bytes against 1 336 bytes for a form's own
/// cells, which is that subtree, counted.
///
/// THE OBVIOUS CUT DOES NOT HOLD. Writing only the cited form's intrinsic
/// coordinates into the `Glued` vertex — its chart, magnitude, depth,
/// placement and the transition φ, all of them bounded — terminates the
/// recursion and collides. This is the collision: two forms with the same
/// own-cells citing two different sub-complexes agree on every one of those
/// coordinates and are two forms. Under the truncation they would be one,
/// and a court resolving the citation would have a set where it needs an
/// object.
///
/// So the compounding is real and the cut is not free. Bounded, non-
/// positional and fully determining are three properties a name of an
/// arbitrary DAG cannot have at once: the address is recursive (today), or
/// bounded and ambiguous (this test), or a digest — which the
/// Anti-Flattening Law admits as transport and cold-cache index only, never
/// as a coordinate. The fourth road is that a citation stops being part of
/// the name at all and rides the receipt, as a derivation's generators
/// already do (`form_vertex_key_ignores_citations_and_certificates`, `cites` 0 bytes in the key). That
/// is a ruling about what a form IS, and it is not taken by a serializer.
#[test]
fn glue_separates_isomorphic_boundaries_by_cover_only() {
    let (x, y) = (leaf((0, 3)), leaf((1, 3)));
    assert_ne!(form_vertex_key(&x), form_vertex_key(&y), "two distinct sub-complexes");

    // The same own-cells, citing two different forms at the same place.
    let cite = |s: &Form| -> Form {
        let mut g = leaf((0, 1));
        g.glue(s, addr_of(s), Word::identity(), [0, 0, 0, 1]).expect("cites");
        g
    };
    let (g1, g2) = (cite(&x), cite(&y));

    let glued_of = |f: &Form| {
        // 54.7 B5a — THE CITATION IS THE COVER'S EDGE. Its boundary is on
        // the vertex and its incidence is here, and an edge carries both.
        let [e] = f.cover.as_slice() else {
            panic!("one citation");
        };
        (e.target.clone(), e.chart.clone(), e.magnitude, e.depth, e.placement.clone(), (*e.carriage).clone())
    };
    let (a1, c1, m1, d1, p1, phi1) = glued_of(&g1);
    let (a2, c2, m2, d2, p2, phi2) = glued_of(&g2);

    // EVERY BOUNDED COORDINATE OF THE CITATION AGREES.
    assert_eq!(c1.key(), c2.key(), "the cited forms' charts are one");
    assert_eq!(m1, m2, "and the cited forms' magnitudes");
    assert_eq!(d1, d2, "and the cited forms' depths");
    assert_eq!(p1.rotor().class_key(), p2.rotor().class_key(), "and the cited forms' placements");
    assert!(
        crate::transducer::element::same_versor(&phi1, &phi2),
        "and the morphisms they act under"
    );

    // AND THE ONLY THING THAT SEPARATES THEM IS THE CITED ADDRESS — the
    // unbounded, recursive half, which is no longer in either key.
    assert_ne!(a1, a2, "the cited addresses differ, and they are all that does");
    // 54.7 B5a — SO THE TWO FORMS ARE ONE FORM. Every bounded coordinate
    // agrees, and the one that disagreed was the cited address; with it
    // gone from the key, two isomorphic local boundaries reach one
    // coordinate. That is gauge universality and it is the point: what
    // distinguishes these two citations is incidence in `N(𝒰)`, which is
    // not a property of either chart, and it is carried on the cover.
    //
    // This asserted the opposite, and the opposite is what forced the
    // topology: `key(A) ⊃ key(B)` made the address space provably acyclic
    // by well-ordering, so `b₁` of the nerve was zero by construction
    // rather than by measurement.
    assert_eq!(
        form_vertex_key(&g1),
        form_vertex_key(&g2),
        "two isomorphic local boundaries are one boundary, and the key says so"
    );
    assert_ne!(
        g1.cover, g2.cover,
        "and the cover is where they differ, because the incidence is the cover's fact"
    );
}

/// 54.4 — EVERY ORBIT LIFTS, because the cycle basis is built where the
/// transport is.
///
/// This is the property that was false on the whole deployment: 281 real orbits
/// across three voices, every face of every one carrying a member pair — no
/// tears — and ZERO closed member paths. `orbits_of` built its spanning tree
/// and fundamental cycles over the VERTEX graph, `adj` indexed by `f.i`,
/// while `extract.rs` built the faces over the MEMBER graph, whose nodes are
/// `member_key(vertex, chart)`. A vertex acting on several charts is several
/// nodes to the face builder and one node to the cycle basis, so a cycle of
/// the vertex graph need not be a cycle of the member graph — and
/// `paths_carried`, which traverses members, reported "there is no lift" and
/// "there is no holonomy" as the same empty answer.
///
/// Built on the member complex the lift exists BY CONSTRUCTION: the arrival
/// chart at each node is the departure chart, because the node IS the chart.
/// So every orbit has at least one closed member path, and the composite the
/// classifier reads is never empty for want of one.
///
/// `orbit_words` yields one entry per (orbit, member path), so an orbit with no
/// path contributes nothing and the count is the test.
#[test]
fn orbit_words_lift_every_orbit() {
    let cases: Vec<(&str, Form)> = vec![
        ("a chord with a 90° turn", chord_form(&[word_turn_90()])),
        ("two chords of one class", chord_form(&[word_turn_90(), word_turn_90()])),
        ("a chord whose EDGE carries the word", edge_chord_form(&[word_turn_90()])),
        ("a coboundary triangle", coboundary_chord_form()),
        ("an infinite-order chord", chord_form(&[infinite()])),
    ];
    for (what, f) in cases {
        assert!(!f.orbits.is_empty(), "{what}: the fixture must have an orbit");
        assert!(
            f.orbit_words().len() >= f.orbits.len(),
            "{what}: {} orbits carry only {} member paths — an orbit with no lift is an orbit \
             of the wrong complex",
            f.orbits.len(),
            f.orbit_words().len()
        );
    }
}

/// AND THE SPURIOUS CYCLES ARE GONE. `k` charts on one vertex are `k`
/// parallel Frame-edges in the vertex graph and `k − 1` fundamental cycles
/// that enclose nothing — measured on the development deployment as an exact identity, the
/// count of Frame-only orbits equalling `members − vertices` on all three
/// voices (285 = 860 − 575, 266 = 702 − 436, 300 = 769 − 469).
///
/// In the member complex `Frame → (v, c₁)` and `Frame → (v, c₂)` is a PATH
/// of length two between two distinct nodes, not a cycle. This builds that
/// shape directly: one vertex standing on two charts, reached from the Frame
/// at each, and nothing else.
#[test]
fn orbits_of_two_charts_one_vertex_is_empty() {
    let mut f = Form::default();
    // Two members at two charts — the two members the deployment's split
    // vertices present, here as the smallest form that has them. AT1 (Q1):
    // one word class is ONE vertex, so this is literally one vertex on two
    // charts, each overlapped from the Frame's.
    stand_with_star(&mut f, &word_vertex_on(1, Word::identity(), stalk::Invariant::counts(&[1])));
    stand_with_star(&mut f, &word_vertex_on(2, Word::identity(), stalk::Invariant::counts(&[2])));
    assert_eq!(f.vertices.len(), 2, "the Frame and one vertex of two members");
    assert_eq!(f.faces.len(), 2, "two star overlaps");
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    assert!(
        f.orbits.is_empty(),
        "a star of two members encloses nothing; the vertex graph counted {} cycles",
        f.orbits.len()
    );
}

/// 54.4 — A VERTEX ON TWO CHARTS, AND THE CYCLE THROUGH BOTH OF THEM.
///
/// `orbit_words_lift_every_orbit` runs on fixtures whose vertices
/// each act on ONE chart, where the member complex and the vertex graph
/// coincide and the property is true for free. This is the shape the deployment
/// actually has — 140 members on 74 vertices, every real orbit passing
/// through one that splits — and it is the shape that has to lift.
///
/// Two bodies, each standing on two charts, related by faces that cross
/// between the charts: `(1,cA) — (2,cB)` and `(2,cB) — (1,cA)` is not a
/// cycle of the vertex graph in any useful sense, but `(1,cA) → (2,cB) →
/// (1,cA)` through two distinct faces is a genuine element of `Z₁(M)`.
#[test]
fn orbit_words_lift_cycle_through_split_vertex() {
    let a = stalk::Invariant::counts(&[7]);
    let b = stalk::Invariant::counts(&[9]);
    let mut f = Form::default();
    // TWO VERTICES, EACH ACTING ON TWO CHARTS. `Vertex::word_on` takes one
    // chart, so each vertex stands a member per chart and the second JOINS
    // the first — which is what a vertex with a section IS (18.2, 21 step 1).
    // AT1 (Q1): two vertices are two CLASSES, so the second is the 90°
    // turn; two of the identity would be one vertex.
    let mut charts = Vec::new();
    for word in [Word::identity(), word_turn_90()] {
        let at_a = f.stand_member(&word_vertex_on(1, word.clone(), a.clone()), FRAME).expect("member on a");
        let at_b = f.stand_member(&word_vertex_on(1, word, b.clone()), FRAME).expect("member on b");
        assert_eq!(at_a.vertex(), at_b.vertex(), "one class, one vertex");
        charts.push((at_a, at_b));
    }
    let [(v_a, v_b), (w_a, w_b)] = charts.as_slice() else { panic!("two vertices") };
    for v in [v_a, w_a] {
        assert_eq!(
            f.vertices.at(&v.vertex()).map(|x| x.charts().len()),
            Some(2),
            "the fixture must split, or nothing is tested"
        );
    }

    // A CYCLE THROUGH BOTH CHARTS OF EACH SPLIT VERTEX. One overlap relates
    // one member pair at one frame (AT1 Q2), so the parallel faces between
    // one pair are gone with the faces that had several; the cycle is
    // `(1,a) → (2,a) → (1,b) → (2,b) → (1,a)`, crossing between the charts,
    // and it exists only because the basis names the CHART: in the vertex
    // graph it is two parallel edges between two vertices, doubled.
    for (from, to) in [(v_a, w_a), (w_a, v_b), (v_b, w_b), (w_b, v_a)] {
        f.place_overlap(from.clone(), to.clone(), None).expect("an overlap between two members");
    }
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();

    assert!(!f.orbits.is_empty(), "two faces between two members are a cycle of M");
    assert!(
        f.orbit_words().len() >= f.orbits.len(),
        "{} orbits through a split vertex carry only {} member paths",
        f.orbits.len(),
        f.orbit_words().len()
    );
}
