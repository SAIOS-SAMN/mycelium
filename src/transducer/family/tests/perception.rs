// 20 — PERCEPTION: what one pair form costs the sensor, and what the
// adjacency of a scene's bodies is.

use super::common::*;
use crate::transducer::base::Site;
use crate::transducer::family::*;

/// A pair form has its frame and a word for the carried body, and nothing is
/// held between readings: a second reading reads the same form.
#[test]
fn pair_form_reads_compartments_once() {
let a = grid(&["1100", "0000"]);
let b = grid(&["0011", "0000"]);
let f1 = cipher_form(&a, &b);
assert!(
    f1.vertices.scoped().any(|(_, v)| matches!(v, Vertex::Frame)),
    "every pair form has a frame"
);
assert!(
    f1.vertices.len() >= 2,
    "frame plus at least one body, got {}",
    f1.vertices.len()
);
assert!(
    f1.vertices.scoped().any(|(_, v)| matches!(v, Vertex::Word { .. })),
    "a carried body is a word vertex"
);
let f2 = cipher_form(&a, &b);
assert_eq!(f1.vertices.len(), f2.vertices.len());
assert_eq!(f1.faces.len(), f2.faces.len());
}

/// 50.3, 78.103 — A RIGID SCENE HAS IDENTITY EDGES, AND A RELATIVE MOTION IS
/// ITS OWN EDGE. Two touching bodies translated together: the pair-body
/// carries rigidly, `V_ij = V_i`, and the edge between them is the identity.
/// One of them moved alone: no versor carries the pair, and the edge is the
/// relative motion `ṽ₁·v₂` — here the first bar's translation undone, a
/// placement with the identity word — never a remainder edge (78.63 governs
/// 50.3: a rigidity is a symptom).
#[test]
fn a_rigid_scene_has_identity_edges_and_a_relative_motion_is_its_own_edge() {
use crate::transducer::family::perceive::Carried;
let body = |rows: &[&str]| perceive(&grid(rows)).into_iter().next().expect("one body");
let (a1, a2) = (body(&["1100", "0000"]), body(&["0000", "2200"]));
let (b1, b2) = (body(&["0110", "0000"]), body(&["0000", "0220"]));
let mut budget = super::common::budget();
let v1 = carriage(&a1, &b1, &mut budget).expect("a translated bar carries").v;
let v2 = carriage(&a2, &b2, &mut budget).expect("the second bar carries").v;
let (edge, carried) = edge_transport((&v1, &v2), &a1, &a2, &b1, &b2, &mut budget).expect("the pair moved rigidly: an edge transport");
assert_eq!(carried, Carried::Rigid, "one versor carries the pair-body");
assert!(edge.word.is_identity(), "a rigid motion is the identity on the edge: {:?}", edge.word);
assert!(
    crate::transducer::element::same_versor(&edge.carriage, &crate::transducer::element::identity_versor()),
    "and carries no relative placement"
);
let b2_still = body(&["0000", "2200"]);
let v2_still = carriage(&a2, &b2_still, &mut budget).expect("a still bar carries by the identity").v;
let (relative, carried) =
    edge_transport((&v1, &v2_still), &a1, &a2, &b1, &b2_still, &mut budget).expect("a differential motion is an edge");
assert_eq!(carried, Carried::Relative, "no versor carries the pair: the edge is their relative motion");
assert!(relative.word.is_identity(), "two translations differ by a translation: no turn");
assert!(
    !crate::transducer::element::same_versor(&relative.carriage, &crate::transducer::element::identity_versor()),
    "and the placement between them is the first bar's motion undone"
);
assert!(
    crate::transducer::element::same_versor(&relative.carriage, &v1.reverse().geo(&v2_still)),
    "it is ṽ₁·v₂ itself"
);
}

/// 78.103 — A TEAR STAYS A BREACH: a body no conformal motion carries (its
/// motion not a versor) relates to nothing, and the edge is refused
/// `NotRigid`, by name.
#[test]
fn a_tear_is_refused_by_name() {
let body = |rows: &[&str]| perceive(&grid(rows)).into_iter().next().expect("one body");
let (a1, a2) = (body(&["1100", "0000"]), body(&["0000", "2200"]));
let (b1, b2) = (body(&["0110", "0000"]), body(&["0000", "2200"]));
let mut budget = super::common::budget();
let v1 = carriage(&a1, &b1, &mut budget).expect("a translated bar carries").v;
let torn = crate::transducer::cga::Mv::blade(crate::transducer::cga::E1, num_rational::BigRational::from_integer(1.into()))
    .add(&crate::transducer::cga::Mv::blade(crate::transducer::cga::E1 | crate::transducer::cga::E2, num_rational::BigRational::from_integer(1.into())));
assert!(!crate::transducer::cga::is_versor(&torn), "a mixed-grade element moves nothing conformally");
let refused = edge_transport((&v1, &torn), &a1, &a2, &b1, &b2, &mut budget);
assert!(
    matches!(refused.as_ref().err().map(|r| r.as_slice()), Some([EdgeRefusal::NotRigid])),
    "a tear is refused by name: {refused:?}"
);
}

/// 78.103 — A PAIR WITH NO BODIES IS RELATED BY ITS TWO MOTIONS: a cover's
/// piece holds the versor that carried it and no body, and its edge to
/// another member is `ṽᵢ·vⱼ` — two translations differ by a translation, a
/// placement with the identity word — and a motion that is no versor is a
/// tear, refused by name.
#[test]
fn a_bodiless_pair_is_related_by_its_two_motions() {
use crate::transducer::cga::{translator_at, Mv, E1, E2};
use crate::transducer::family::perceive::{relative_edge, Carried};
let q = |n: i64| num_rational::BigRational::from_integer(n.into());
let (v1, v2) = (translator_at(&[q(1), q(0), q(0)]), translator_at(&[q(0), q(2), q(0)]));
let (edge, carried) = relative_edge(&v1, &v2).expect("two motions relate");
assert_eq!(carried, Carried::Relative);
assert!(edge.word.is_identity(), "no turn between two translations");
assert!(crate::transducer::element::same_versor(&edge.carriage, &v1.reverse().geo(&v2)), "the edge is ṽ₁·v₂");
let torn = Mv::blade(E1, q(1)).add(&Mv::blade(E1 | E2, q(1)));
assert!(matches!(relative_edge(&v1, &torn).err().as_deref(), Some([EdgeRefusal::NotRigid])), "a tear stays refused");
}

/// 50.3 — AN EXCHANGE: two congruent bodies that swap places carry as one
/// pair-body by the identity, and the edge transport is the exchange —
/// `V_i⁻¹`, a placement with the identity word, finite on the word.
#[test]
fn edge_transport_lies_in_the_stabilizer_or_the_exchange() {
let body = |rows: &[&str]| perceive(&grid(rows)).into_iter().next().expect("one body");
let (a1, a2) = (body(&["100"]), body(&["002"]));
let (b1, b2) = (body(&["001"]), body(&["200"]));
let mut budget = super::common::budget();
let v1 = carriage(&a1, &b1, &mut budget).expect("a translated compartment carries").v;
let v2 = carriage(&a2, &b2, &mut budget).expect("and the other").v;
let (edge, _) = edge_transport((&v1, &v2), &a1, &a2, &b1, &b2, &mut budget).expect("the pair-body is carried by the identity: an edge");
assert!(edge.word.is_identity(), "the exchange has the identity word");
assert!(
    !crate::transducer::element::same_versor(&edge.carriage, &crate::transducer::element::identity_versor()),
    "and a placement: the bodies exchanged"
);
assert!(matches!(edge.word.order(), crate::transducer::alphabet::Order::Finite(_)));
}

/// 50.5 — A CREATED BODY IS A SOURCE OR A CREATION DEMAND, never a word
/// with the identity: a new bar some input chart carries onto is a
/// `Source` with that multiplicity; a new body no input chart carries
/// onto is a `Creation`. Both ride the pair form as demand vertices with
/// the links they stand for, and the form is not closed.
#[test]
fn a_created_body_is_a_source_or_creation_demand() {
use crate::transducer::sentence::Demand;
let f = cipher_form(&grid(&["1100", "0000", "0000"]), &grid(&["1100", "0000", "0220"]));
let demands: Vec<_> = f.demand_vertices().collect();
assert_eq!(demands.len(), 1, "one demand: {:?}", f.vertices);
let (_, kind, _, links) = demands[0];
assert_eq!(*kind, Demand::Source { multiplicity: 1 }, "the input bar's chart carries onto the new bar");
assert_eq!(links, &[(2, 1, 0, 2), (2, 2, 0, 2)], "the created links, hue 0 before");
assert!(!f.is_closed(), "a sentence with a demand is not closed");
assert_eq!(
    f.vertices.scoped().filter(|(_, v)| matches!(v, Vertex::Word { .. })).count(),
    1,
    "the carried bar is the one word vertex; the created bar is no word: {:?}",
    f.vertices
);

let f = cipher_form(&grid(&["1100", "0000", "0000"]), &grid(&["1100", "0000", "0222"]));
let demands: Vec<_> = f.demand_vertices().collect();
assert_eq!(demands.len(), 1);
assert_eq!(*demands[0].1, Demand::Creation, "no input chart carries onto a bar of three");
}

/// 50.5 — A DEFORMED BODY IS A DEFORMATION DEMAND: an input body no versor
/// carries onto the output body over it names that image's chart and the
/// axes they differ on, and the image is consumed by the deformation — it
/// is not a second demand, a creation; a body that vanished names the
/// empty chart on every axis.
#[test]
fn a_deformed_body_is_a_deformation_demand() {
use crate::transducer::sentence::Demand;
let f = cipher_form(&grid(&["1100"]), &grid(&["1000"]));
let kinds: Vec<&Demand> = f.demand_vertices().map(|(_, k, _, _)| k).collect();
assert_eq!(kinds.len(), 1, "one demand: the deformation, its image consumed: {kinds:?}");
assert!(matches!(kinds[0], Demand::TopologicalDeformation { image, axes } if !image.0.is_empty() && !axes.is_empty()), "{kinds:?}");
let (_, _, _, links) = f.demand_vertices().next().expect("the demand");
assert_eq!(links, &[(0, 0, 1, 1), (0, 1, 1, 0)], "the deformation stands for the union of both bodies, hue before and after");

let f = cipher_form(&grid(&["1100", "0000", "0220"]), &grid(&["1100", "0000", "0000"]));
let kinds: Vec<&Demand> = f.demand_vertices().map(|(_, k, _, _)| k).collect();
assert_eq!(kinds.len(), 1, "{kinds:?}");
assert!(matches!(kinds[0], Demand::TopologicalDeformation { image, axes } if image.0.is_empty() && !axes.is_empty()), "a vanished body deforms to the empty chart on every axis: {kinds:?}");
for (_, kind, _, _) in f.demand_vertices() {
    assert_eq!(kind.typed(), Ok(()), "every demand the eyes emit is typed");
}
}

/// 50.5 — DEMANDS RIDE THE SENTENCE ONE PER (KIND, CHART): two created bars
/// of one chart are one demand vertex carrying both bodies' links.
#[test]
fn demands_ride_the_sentence_one_per_chart() {
use crate::transducer::sentence::Demand;
let f = cipher_form(&grid(&["1100000", "0000000", "0000000"]), &grid(&["1100000", "0000000", "0220022"]));
let demands: Vec<_> = f.demand_vertices().collect();
assert_eq!(demands.len(), 1, "one vertex for one (kind, chart): {:?}", f.vertices);
let (idx, kind, chart, links) = demands[0].clone();
assert_eq!(*kind, Demand::Source { multiplicity: 1 });
assert_eq!(links.len(), 4, "both bars' links ride it");
// AT1 — THE DEMAND IS KEYED BY ITS COORDINATE, which opens with its kind and
// its chart: the `demands` position index retired into the carrier's own
// stratum, so the read by (kind, chart) is the vertex's key.
assert!(
    matches!(&idx, crate::network::nerve::coord::VertexAt::Demand { kind: k, chart: c, .. } if k == kind && c == chart),
    "and it is keyed by (kind, chart): {idx:?}"
);
assert!(
    matches!(f.vertices.at(&idx), Some(Vertex::Demand { .. })),
    "and it is read by that key"
);
}

/// 50.5 (F step 5) — THE DEMANDS RIDE THE RULE: over a bundle of two pairs
/// that each create a bar of one chart, the rule form carries one `Source`
/// demand vertex with both pairs' links, and the sentence mints whole —
/// its measure the deflation its words closed and the links its demand
/// asks for — where a form with a remainder and no demand mints nothing.
#[test]
fn demands_ride_the_rule_form_and_the_sentence_mints_whole() {
use crate::transducer::sentence::Demand;
// The bars land at two different offsets from the 1-bars, so no one echo
// says them (78.167 Phase B, Step 3): they stay the demand's to declare.
let family = vec![
    (grid(&["1100", "0000", "0000", "0000"]), grid(&["1100", "0000", "0220", "0000"])),
    (grid(&["0000", "1100", "0000", "0000"]), grid(&["0000", "1100", "0000", "2200"])),
];
let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the bundle extracts");
let demands: Vec<_> = rule.form.demand_vertices().collect();
assert_eq!(demands.len(), 1, "one (kind, chart) over the bundle: {:?}", rule.form.vertices);
let (_, kind, _, links) = demands[0];
assert_eq!(*kind, Demand::Source { multiplicity: 1 });
assert_eq!(links.len(), 4, "both pairs' created links ride it: {links:?}");
let word = mint_sentence(&rule, &family, &NoCrystal, 0).expect("a sentence with demands mints whole");
assert!(word.is_sentence());
assert!(word.magnitude().iter().any(|&m| m > 0), "it measures what it asks for: {:?}", word.magnitude());
let back = crate::transducer::sentence::from_element_form(&word).expect("its tape decodes");
assert_eq!(back.demand_vertices().count(), 1, "the demand rides the tape");
assert!(!back.is_closed());

// A DEMAND DECLARES LINKS, NOT AN EXEMPTION. The mint above is spoken
// because its demand names every remainder link — the court asks
// `undeclared_remainder`, and on this family there is none. A voice that
// asked only whether ANY demand vertex existed would mint a form declaring
// one link and leaving twenty, and the court would refuse it.
let undeclared = crate::court::verdict::body::undeclared_remainder(
    &form_deflation(&rule.form, &family, &NoCrystal).1,
    &rule.form,
);
assert!(
    undeclared.is_empty(),
    "this form's demand covers what it leaves, which is why it mints; undeclared {undeclared:?} on {:?} with sections {:?}",
    rule.form.vertices.scoped().map(|(_, v)| match v {
        Vertex::Word { address, acts, .. } => format!("word {:02x}{:02x} acts {:?}", address.as_bytes()[0], address.as_bytes()[1], acts.scoped().map(|(_, a)| (a.chart.clone(), a.gauge.clone(), element::same_versor(&a.carriage, &element::identity_versor()))).collect::<Vec<_>>()),
        other => format!("{other:?}"),
    }).collect::<Vec<_>>(),
    rule.form.sections().scoped().map(|(k, v)| (k.len(), v.clone())).collect::<Vec<_>>()
);
// Strip the links the demand declares. The form still HOLDS a demand
// vertex — so the old test, which asked only whether one existed, still
// passes it — and it now declares nothing, so the remainder is undeclared
// and the voice withholds. The bodies ride the record and not the key
// (AT1), so each demand is emptied where it stands.
let mut bare = rule.clone();
let demanded: Vec<crate::network::nerve::coord::VertexAt> = bare.form.demands().map(|(at, _)| at).collect();
for at in &demanded {
    if let Some(Vertex::Demand { bodies, .. }) = bare.form.vertices.at_mut(at) {
        bodies.clear();
    }
}
assert_eq!(
    bare.form.demand_vertices().count(),
    1,
    "it still has a demand vertex, which is all the old clause asked"
);
assert!(
    !crate::court::verdict::body::undeclared_remainder(
        &form_deflation(&bare.form, &family, &NoCrystal).1,
        &bare.form
    )
    .is_empty(),
    "and the court would now find the remainder undeclared"
);
// 50.5 — THE MINT DECLARES WHAT WAS LEFT UNSPOKEN. The empty demand exempts
// nothing, as before; what changed is that the voice no longer withholds
// the form for it — the unspoken links are declared as the demand they
// are, on the chart they stand on, extending the demand already there, and
// the court's own reader then finds nothing undeclared. A demand that
// declares nothing still exempts nothing: it is filled, not honoured.
let spoken = mint_sentence(&bare, &family, &NoCrystal, 0).expect("spoken with the remainder declared");
let back = crate::transducer::sentence::from_element_form(&spoken).expect("its tape decodes");
assert_eq!(back.demand_vertices().count(), 1, "the existing demand is extended, not doubled");
assert!(
    crate::court::verdict::body::undeclared_remainder(
        &form_deflation(&back, &family, &NoCrystal).1,
        &back
    )
    .is_empty(),
    "the court finds nothing undeclared on the spoken form"
);
let declared: Vec<_> = back.demand_vertices().flat_map(|(_, _, _, c)| c.to_vec()).collect();
assert_eq!(declared.len(), 4, "the four created links are the declared remainder: {declared:?}");
}

/// The supports a form hands `named_faces`, keyed by each
/// vertex's own coordinate. A test writes them in vertex order because
/// that is what it is asserting about; the reader is keyed, so a vertex
/// with no support simply has no entry.
fn supports_at(rows: &[Vec<(i64, i64)>]) -> crate::network::nerve::Keyed<Vec<Site>> {
    let mut k = crate::network::nerve::Keyed::new();
    // KEYED BY MEMBER — vertex and chart (D10). These fixtures put every body
    // on the root stratum, which is one chart per vertex, so the member key is
    // the vertex's with that chart's; every support stands in frame 0.
    for (i, s) in rows.iter().enumerate() {
        if !s.is_empty() {
            k.insert(root_member(u16::try_from(i).expect("a fixture row, a few vertices")), s.iter().map(|&(r, c)| Site::from((r, c, 0))).collect());
        }
    }
    k
}

/// Sites as the sensor hands them over (71.2(1)).
fn sites<T: Copy + Into<Site>>(v: &[T]) -> Vec<Site> {
    v.iter().map(|&t| t.into()).collect()
}

/// The identity's class, which every fixture body is a member of.
fn identity_at() -> crate::network::nerve::coord::VertexAt {
    Vertex::word_on(
        crate::transducer::alphabet::Derivation::spelled(crate::transducer::alphabet::Word::identity()),
        crate::transducer::stalk::Invariant::default(),
        Box::new(element::identity_versor()),
        Vec::new(),
        [0u64],
        None,
    )
    .at()
    .expect("the identity has a class")
}

/// The member key of fixture member `i`: `0` the Frame's, at the root
/// stratum; every other a member of the identity's class on chart `[i]`
/// (AT1: a vertex is keyed by its coordinate, so members of one class are
/// told apart by their charts, never by a position). The chart's count is
/// written little-endian, so for a fixture of a few members the key order
/// is `i`'s order, as the vertex order was.
fn root_member(i: u16) -> Vec<u8> {
    match i {
        0 => crate::transducer::family::member_key(
            &crate::network::nerve::coord::VertexAt::Frame,
            crate::transducer::stalk::Invariant::default().key().as_bytes(),
        ),
        _ => crate::transducer::family::member_key(
            &identity_at(),
            crate::transducer::stalk::Invariant::counts(&[u64::from(i)]).key().as_bytes(),
        ),
    }
}

/// The fixture member a key names, read back out of it: the Frame is `0`,
/// every other member its chart's count.
fn member_index(k: &[u8]) -> Option<u16> {
    let (v, chart) = crate::transducer::family::read_member_key(k)?;
    match v {
        crate::network::nerve::coord::VertexAt::Frame => Some(0),
        _ => match crate::transducer::stalk::Invariant::from_key(&chart)?.0.as_slice() {
            [crate::transducer::stalk::ChartAxis::Count(n)] => u16::try_from(*n).ok(),
            _ => None,
        },
    }
}

/// The member pairs `named_faces` names, without the incidences on each —
/// the reading the chain and star tests assert on (H1c: the reader lives
/// with the tests that read it).
fn named_pairs(
    frame: Option<&[u8]>,
    supports: &crate::network::nerve::Keyed<Vec<Site>>,
) -> Vec<(Vec<u8>, Vec<u8>)> {
    named_faces(frame, supports).into_iter().map(|nf| (nf.a, nf.b)).collect()
}

/// T1 — THREE TOUCHING BODIES HAVE NO ADJACENCY FACE, AND THE ABSENCE IS
/// THE READING (60.5, ruled 2026-09-22).
///
/// This asserted two adjacency faces beside the Frame star and a
/// 1-skeleton that is not a tree. The three supports are `(0,0)(0,1)`,
/// `(0,2)(0,3)` and `(0,4)(0,5)`: PAIRWISE DISJOINT, sharing no cell of
/// the base. Both faces came from `ring_of_support` dilating each support
/// by one column until the neighbours collided — the fixture's own name
/// says "touching", and touching is exactly what a Čech nerve does not
/// see.
///
/// The relation the test was reaching for is REAL. Three bodies in a row
/// do relate, and the engine should carry that. But contact is not a
/// shared coordinate: it is the conformal strain between their spheres
/// (59.3), condensed where a mind can spell the interface (59.6). Until
/// the tether lands (T2), the incidence is absent, and a test that
/// manufactured it with a stencil was asserting the dilation.
///
/// So what stands here is the star alone — `E − V + 1 = 0`, a tree, `b₁ =
/// 0` — and this test is where the tether's arrival will be visible: when
/// T2 lands, these three bodies gain their faces back from geometry
/// rather than from a raster, and this assertion is the one that changes.
#[test]
fn a_row_of_three_touching_bodies_has_no_adjacency_face_until_the_tether() {
let supports = supports_at(&[
    vec![],
    vec![(0, 0), (0, 1)],
    vec![(0, 2), (0, 3)],
    vec![(0, 4), (0, 5)],
]);
let pairs = named_pairs(Some(&root_member(0)), &supports);
// The pairs are MEMBER pairs, so the vertex each names is read back out of
// its key rather than being the whole of it.
let ends: Vec<(u16, u16)> = pairs
    .iter()
    .filter_map(|(a, b)| Some((member_index(a)?, member_index(b)?)))
    .collect();
let adj: Vec<_> = ends.iter().copied().filter(|&(i, j)| i != 0 && j != 0).collect();
assert_eq!(adj.len(), 0, "no adjacency face without the stencil, got {adj:?} from {ends:?}");
// THE STAR IS STILL THERE, so this is not an empty generator reporting
// nothing: every body is related to the Frame, and it is only the
// body-to-body incidence that the dilation was inventing.
assert_eq!(ends.len(), 3, "the Frame star stands over all three, got {ends:?}");
// AT1 — A FACE IS AN OVERLAP OF TWO PLACED CHARTS and these ends are
// fixture members, not charts a form holds, so the 1-skeleton is read on the
// member pairs themselves: its edges are the generator's output.
let faces: Vec<(u16, u16)> = ends.clone();
// THE 1-SKELETON IS NOT A STAR TREE, and that is arithmetic on the
// generator's own output (54.5, as ruled 2026-09-20). It was read through
// `orbits_of`, which no longer answers it: these faces are `FormFace::new`
// — no member pair, identity word — so every cycle over them encloses
// nothing and is pruned as trivial. It could not be pruned before only
// because `paths_carried` searched a frontier with nothing to step onto
// and answered EMPTY, and an unread orbit is not a flat one. The traversal
// reads the cycle now and says what it is.
//
// AND NOW THE STAR AND SITE FACES DO FORM A TREE. The cycle rank `E − V +
// 1` is read on the generator's own output, and with the dilation gone it
// is zero: three vertices on one Frame, three edges, no independent
// cycle. Every cycle the corpus carried through this builder was a cycle
// the stencil closed.
assert_eq!(faces.len() + 1 - 4, 0, "the 1-skeleton is the star tree: no independent cycle");
}

/// H2b, 70.4 — TWO BODIES MEETING AT A CORNER ARE NAMED A CONTACT FACE.
///
/// Two hue-blind bodies cannot share an edge of the base without being one
/// body, so where two distinct bodies touch they touch at a corner. That
/// corner is named as a contact between the two members, once, and a body
/// whose own support turns a corner is not in contact with itself. A body
/// a compartment away from the others touches nothing and is named by the Frame
/// alone.
#[test]
fn two_bodies_meeting_at_a_corner_are_named_a_contact_face() {
let supports = supports_at(&[
    vec![],
    // An L whose own support turns a corner at (0,1)–(1,0)…
    vec![(0, 0), (0, 1), (1, 0)],
    // …and a block meeting it diagonally at (1,0)–(2,1).
    vec![(2, 1), (2, 2)],
    // A compartment standing clear of both.
    vec![(5, 5)],
]);
let named = named_faces(Some(&root_member(0)), &supports);
let ends = |how: Incidence| -> Vec<(u16, u16)> {
    named
        .iter()
        .filter(|nf| nf.named.hows.contains(&how))
        .filter_map(|nf| {
            let (i, j) = (member_index(&nf.a)?, member_index(&nf.b)?);
            Some((i.min(j), i.max(j)))
        })
        .collect()
};
assert_eq!(ends(Incidence::Contact), vec![(1, 2)], "the corner relates the L and the block, once");
assert_eq!(ends(Incidence::Star).len(), 3, "and the Frame names all three bodies");
assert!(
    ends(Incidence::Site).is_empty(),
    "no two bodies share a site, so no site face is named"
);
}

/// N1, BQ5a — A PAIR THE TOPOLOGY NAMES IS ONE FACE, CARRYING ITS
/// REFRACTION.
///
/// Two L pentominoes, the one the other turned a 180° turn, hooked so they
/// meet at two corners and share no edge. The diagonal names them: they
/// touch at `(1,0)–(0,1)` and `(3,2)–(2,3)`. The condensation field named
/// them a second time, through a band on `δ`; the band is gone and whether
/// is the topology's alone, so one incidence names the pair. What the
/// contact carries is read where the face is built, each member against its
/// own transition (`pairing::contact_edge`).
#[test]
fn a_pair_the_topology_names_is_one_face_carrying_its_refraction() {
let supports = supports_at(&[
    vec![],
    vec![(1, 0), (2, 0), (3, 0), (3, 1), (3, 2)],
    vec![(2, 3), (1, 3), (0, 3), (0, 2), (0, 1)],
]);
let named = named_faces(None, &supports);
let [nf] = named.as_slice() else { panic!("one pair, one face: {named:?}") };
let ends = (member_index(&nf.a), member_index(&nf.b));
assert_eq!(ends, (Some(1), Some(2)), "between the two L's");
assert_eq!(nf.named.hows, vec![Incidence::Contact], "the corner alone names the pair");
// AND ITS WITNESS IS THE TWO CORNERS, as a set (C2): `(1,0)+(0,1)` and
// `(3,2)+(2,3)`, each element at its doubled centre — WHETHER, read by the
// topology. A corner of the plane is an EDGE of the cubes of ℤ³, running
// along the frame axis, so the contact's largest shared cell has grade 1.
let [c] = nf.named.interfaces.as_slice() else { panic!("one contact: {nf:?}") };
assert_eq!(c.witness, sites(&[(1i64, 1i64, 0i64), (5, 5, 0)]), "the two corners the L's share");
assert_eq!(c.grade, 1, "in-plane corners, edges along the frame axis");
}

/// C1, 70.6 — A CORNER BETWEEN TWO BODIES OF SEVERAL MEMBERS OFFERS ONE
/// FACE. Two members stand on one compartment and two on the compartment diagonal to it.
/// H2b related every occupant of one site to every occupant of the other,
/// four faces for one contact: the join of two occupancy simplices written
/// as a clique. The contact is between the two bodies, so it is one face,
/// between the least member at each side. The site chain still joins each
/// side's own occupants.
#[test]
fn a_corner_between_two_bodies_of_several_members_offers_one_face() {
let supports = supports_at(&[
    vec![],
    vec![(0, 0)],
    vec![(0, 0)],
    vec![(1, 1)],
    vec![(1, 1)],
]);
let named = named_faces(None, &supports);
let ends = |how: Incidence| -> Vec<(u16, u16)> {
    named
        .iter()
        .filter(|nf| nf.named.hows.contains(&how))
        .filter_map(|nf| {
            let (i, j) = (member_index(&nf.a)?, member_index(&nf.b)?);
            Some((i.min(j), i.max(j)))
        })
        .collect()
};
assert_eq!(ends(Incidence::Contact), vec![(1, 3)], "one contact face for one body pair, not four");
assert_eq!(ends(Incidence::Site), vec![(1, 2), (3, 4)], "each side's occupants chained on their site");
}

/// C2, 70.6 — A BODY MOVING BETWEEN FRAMES TOUCHES THE BODY THAT STOOD.
/// One body stood at `(0, 0)` in frame 0. The other stands at `(0, 2)` in
/// frame 0 and reaches `(0, 1)` in frame 1, beside where the first stood.
/// In no single frame do the two share an edge or a corner. Across frames,
/// `(0, 1, 1)` and `(0, 0, 0)` share an edge of their cubes, so the frame
/// axis relates them. The corner of one frame could not see it, and the
/// twenty adjacencies do.
#[test]
fn a_body_moving_between_frames_touches_the_body_that_stood() {
let mut supports = crate::network::nerve::Keyed::new();
supports.insert(root_member(1), sites(&[(0i64, 0i64, 0i64)]));
supports.insert(root_member(2), sites(&[(0i64, 2i64, 0i64), (0, 2, 1), (0, 1, 1)]));
let named = named_faces(None, &supports);
let [nf] = named.as_slice() else { panic!("one pair, the contact across frames: {named:?}") };
assert_eq!(nf.named.hows, vec![Incidence::Contact], "the frame axis enters contact");
let [contact] = nf.named.interfaces.as_slice() else { panic!("one interface: {nf:?}") };
let interface = &contact.witness;
assert_eq!(interface, &sites(&[(0i64, 1i64, 1i64)]), "one shared edge, between frames 0 and 1");
assert_eq!(contact.grade, 1, "an edge");
assert!(
    interface.iter().all(|s| s.axis(2).is_some_and(|k| k % 2 != 0)),
    "an odd doubled frame coordinate is an element across two frames"
);
// And the corner in one frame alone names nothing here.
let in_frame: Vec<_> = crate::transducer::sentence::contact_cells(&sites(&[(0i64, 0i64, 0i64), (0, 1, 1)]))
    .into_iter()
    .filter(|c| !c.crosses(crate::transducer::base::FRAME_AXIS))
    .collect();
assert!(in_frame.is_empty(), "no in-frame adjacency relates them: {in_frame:?}");
}

/// C2, 70.6 — THE INTERFACE IS A SET WITH A SHAPE, AND TWO SHAPES ARE TWO
/// FACES. One pair of members covers two pairs of bodies. Each pair stands
/// in two consecutive frames, the second body one row down and so never
/// sharing a face with the first. A bar over a bar meets along a straight
/// seam, and an L over an L along a seam that turns. Both are named on the
/// one member pair as two interfaces, never merged into one set. Lifted,
/// their towers differ.
#[test]
fn a_straight_seam_and_a_bent_one_are_two_interfaces() {
let mut supports = crate::network::nerve::Keyed::new();
supports.insert(
    root_member(1),
    sites(&[
        // A bar in frame 0.
        (0i64, 0i64, 0i64), (0, 1, 0), (0, 2, 0), (0, 3, 0),
        // An L in frame 0, far from it.
        (10, 0, 0), (10, 1, 0), (10, 2, 0), (11, 0, 0), (12, 0, 0),
    ]),
);
supports.insert(
    root_member(2),
    sites(&[
        // A bar in frame 1, one row down: every site meets the frame-0 bar
        // at an edge or a corner across the frames.
        (1i64, 0i64, 1i64), (1, 1, 1), (1, 2, 1), (1, 3, 1),
        // The L in frame 1, one row and one column in: the seam turns.
        (11, 1, 1), (11, 2, 1), (11, 3, 1), (12, 1, 1), (13, 1, 1),
    ]),
);
let named = named_faces(None, &supports);
let contacts: Vec<_> = named.iter().filter(|nf| nf.named.hows.contains(&Incidence::Contact)).collect();
let [nf] = contacts.as_slice() else { panic!("one member pair: {named:?}") };
assert_eq!(nf.named.interfaces.len(), 2, "two interfaces between the one pair: {nf:?}");
let tower = |i: &Vec<Site>| {
    let compartments: Vec<crate::transducer::blade::Compartment> =
        i.iter().map(|s| crate::transducer::blade::Compartment { site: s.clone(), hue: 0 }).collect();
    crate::transducer::blade::lift(&compartments)
};
let (a, b) = (tower(&nf.named.interfaces[0].witness), tower(&nf.named.interfaces[1].witness));
assert_ne!(a.inertia, b.inertia, "a straight seam and a bent one lift to two towers");
}

/// BQ5b, A.14 — A CONTACT FAMILY IS CONSTANT ON WHETHER AND CARRIES THE
/// INTERFACE AS WHAT. One contact — two single cubes on the diagonal —
/// placed at four offsets of the plane. WHETHER is the topology's and does
/// not move: one contact on the pair, its largest shared cell of one grade,
/// its witness of one size. WHAT is the interface `V = s_b s_a` between the
/// two bodies' nuclei, carried by every member of the family and moving with
/// the placement as a versor does: `V_k = T_k V₀ T̃_k`, the translator of the
/// offset applied, exact (or its reverse, where the bodies' key order turns
/// with the placement).
#[test]
fn a_contact_family_is_constant_on_whether_and_carries_the_interface_as_what() {
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let read = |dx: i64, dy: i64| {
        let mut supports = crate::network::nerve::Keyed::new();
        supports.insert(root_member(1), sites(&[(dx, dy, 0i64)]));
        supports.insert(root_member(2), sites(&[(dx + 1, dy + 1, 0i64)]));
        let named = named_faces(None, &supports);
        let [nf] = named.as_slice() else { panic!("one pair: {named:?}") };
        assert_eq!(nf.named.hows, vec![Incidence::Contact]);
        let [c] = nf.named.interfaces.as_slice() else { panic!("one contact: {nf:?}") };
        let (Some(sa), Some(sb)) = (&c.nuclei.0, &c.nuclei.1) else { panic!("both nuclei read: {c:?}") };
        let i = crate::transducer::sentence::Interface::between(sa, sb, c.grade).expect("the interface reads");
        (c.grade, c.witness.len(), i)
    };
    let (g0, w0, i0) = read(0, 0);
    for (dx, dy) in [(5, 0), (0, 7), (3, 4)] {
        let (g, w, i) = read(dx, dy);
        assert_eq!((g, w), (g0, w0), "WHETHER is constant over the family at ({dx}, {dy})");
        let t = crate::transducer::cga::at_three::translator(&q(dx), &q(dy), &q(0));
        let moved = t.geo(&i0.v).geo(&t.reverse());
        assert!(
            i.v.projective_eq(&moved) || i.v.projective_eq(&moved.reverse()),
            "WHAT moves with the placement as a versor at ({dx}, {dy})"
        );
        assert_eq!(i.grade, i0.grade);
    }
}

/// C3, 70.7 — A BODY PAIR IS KEYED BY TWO ADDRESSES, NEVER TWO SITES. The
/// key is the two bodies' `coord::Piece`s written out, the lesser first, and
/// it reads back into exactly those two (A21): no position of the base
/// stands for a body.
#[test]
fn a_body_pair_is_keyed_by_two_addresses_and_reads_back_as_them() {
    use crate::transducer::family::pairing::{body_pair_key, read_body_pair_key};
    use crate::transducer::peel::ShellKind;
    let piece = |s: &[(i64, i64, i64)]| {
        let mut sup = sites(s);
        sup.sort_unstable();
        crate::network::nerve::coord::Piece { kind: ShellKind::Whole, support: sup }
    };
    let (a, b) = (piece(&[(0, 0, 0), (0, 1, 0)]), piece(&[(1, 2, 0)]));
    let enc = |p: &crate::network::nerve::coord::Piece| p.encode().as_bytes().to_vec();
    let (lo, hi) = match enc(&a) <= enc(&b) {
        true => (a.clone(), b.clone()),
        false => (b.clone(), a.clone()),
    };
    let k = body_pair_key(&enc(&lo), &enc(&hi));
    assert_eq!(read_body_pair_key(&k), Some((lo, hi)), "the key is the two addresses, read back");
    let mut long = k.clone();
    long.push(0);
    assert_eq!(read_body_pair_key(&long), None, "and nothing past them");
}

/// A SITE'S OCCUPANTS ARE CHAINED, NOT CLIQUED. k bodies on one site are a
/// simplex. The complex needs k−1 faces to connect them; the k(k−1)/2 −
/// (k−1) chords are cycles that bound by construction. The face count is
/// asserted here, so it cannot go quadratic without a red test.
#[test]
fn one_site_chains_its_occupants_and_does_not_clique_them() {
let run = |k: usize| -> (usize, usize) {
    let mut supports = vec![Vec::new()];
    for _ in 0..k {
        supports.push(vec![(0i64, 0i64)]);
    }
    let pairs = named_pairs(Some(&root_member(0)), &supports_at(&supports));
    // The vertex each member key names, read back out of it.
    let adj = pairs
        .iter()
        .filter(|(a, b)| matches!((member_index(a), member_index(b)), (Some(i), Some(j)) if i != 0 && j != 0))
        .count();
    (pairs.len(), adj)
};
let (all, adj) = run(8);
assert_eq!(adj, 7, "eight bodies on one site chain in seven faces, not 28: {adj}");
assert_eq!(all, 15, "plus the eight-body Frame star: {all}");
let (_, adj4) = run(32);
assert_eq!(adj4, 31, "and thirty-two chain in thirty-one, not 496: {adj4}");
// THE ORBITS ARE LINEAR IN THE OCCUPANTS, WHICH IS THE WHOLE POINT. The
// Frame star already spans every vertex, so each intra-site face is a
// chord and carries one orbit: Frame → a → b → Frame, whose reading is the
// cocycle condition between two placements — `edge_holonomy` skips the
// star faces, so what it measures is the meet, and that is a real
// obstruction the pivot exists to discharge.
//
// So the chain does not abolish this site's orbits; it abolishes the
// DEPENDENT ones. k(k−1)/2 chords carry k(k−1)/2 orbits of which only k−1
// are independent, and the rest are sums of those — fake homology inside
// one contractible site. k−1 in, k−1 out.
// THE COUNT IS THE FACE GENERATOR'S, AND IT IS READ THERE (54.5, as ruled
// 2026-09-20). It was read through `orbits_of`, which no longer answers it:
// these faces are `FormFace::new`, carrying no member pair and an identity
// word, so every cycle over them encloses nothing and `orbit_is_trivial`
// prunes it. It could not prune them before only because `paths_carried`
// searched a frontier that had nothing to step onto and answered EMPTY —
// and an unread orbit is not a flat one, so the orbit was kept. The traversal
// reads the cycle now and says what it is: trivial.
//
// The claim this test carries is about the CHAIN, not about homology —
// k−1 faces and not k(k−1)/2 — and the cycle rank of the 1-skeleton is
// `E − V + 1`, arithmetic over the generator's own output.
let rank = |faces: &[(u16, u16)], n: usize| faces.len() + 1 - n;
assert_eq!(rank(&run_faces(8), 9), 7, "eight occupants carry seven independent cycles, not 28");
assert_eq!(rank(&run_faces(32), 33), 31, "and thirty-two carry thirty-one, not 496");
}

/// The faces eight bodies on one site name, by the member pairs they join
/// (AT1: a face is an overlap of two placed charts, and these are fixture
/// members, so the 1-skeleton is read on the pairs).
fn run_faces(k: usize) -> Vec<(u16, u16)> {
let mut supports = vec![Vec::new()];
for _ in 0..k {
    supports.push(vec![(0i64, 0i64)]);
}
named_pairs(Some(&root_member(0)), &supports_at(&supports))
    .iter()
    .filter_map(|(a, b)| Some((member_index(a)?, member_index(b)?)))
    .collect()
}

/// Two pairs whose bodies differ in shape but share the carriage are
/// one rule vertex. Matching by moduli would split them and leave a
/// frame-only form.
#[test]
fn different_shapes_with_one_carriage_are_one_vertex() {
let family = vec![
    (grid(&["1100"]), grid(&["0011"])),
    (grid(&["111000"]), grid(&["000111"])),
];
let rule = extract_rule_with(&family, &NoCrystal, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("extracts");
let words = rule
    .form
    .vertices
    .scoped()
    .filter(|(_, v)| matches!(v, Vertex::Word { .. }))
    .count();
assert_eq!(
    words, 1,
    "one carriage is one vertex; moduli are a section coordinate, got {words}"
);
assert!(
    rule.form.vertices.scoped().any(|(_, v)| matches!(v, Vertex::Frame)),
    "the frame stays; the form is not frame-only"
);
}

/// Pairing is ∃V on the shapes, not a digest of the moduli axes.
#[test]
fn pairing_is_carriage_not_a_moduli_digest() {
let family = vec![(grid(&["1100", "0000"]), grid(&["0011", "0000"]))];
let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("a translated bar extracts");
let bodies = rule
    .form
    .vertices
    .scoped()
    .filter(|(_, v)| matches!(v, Vertex::Word { .. }))
    .count();
assert_eq!(bodies, 1, "one body is one vertex under ∃V, got {}", rule.form.vertices.len());
let a = perceive(&family[0].0);
let b = perceive(&family[0].1);
assert_eq!(a[0].moduli, b[0].moduli, "the chart is exact axes, not a hash");
assert!(carriage(&a[0], &b[0], &mut super::common::budget()).is_some(), "a versor carries the bar");
}

/// Y4 (24.4, 20, 21 step 1, W10, Rule 2):
/// A scene of five bodies on three charts answers "the chart whose fiber holds one"
/// without a count compared to a literal.
#[test]
fn a_scene_of_five_bodies_on_three_charts_answers_the_chart_whose_fiber_holds_one() {
    let g = grid(&[
        "110333",
        "000000",
        "220444",
        "000000",
        "005000",
    ]);
    let bodies = perceive(&g);
    assert_eq!(bodies.len(), 5, "five bodies perceived: {bodies:?}");
    let fibers = crate::transducer::family::perceive::bodies_by_chart(&bodies);
    assert_eq!(fibers.len(), 3, "three charts represented: {fibers:?}");

    let dot = bodies.iter().find(|b| b.compartments == vec![Compartment::from((4i64, 2i64, 0i64, 5u8))]).expect("the dot");
    let chart = crate::transducer::family::perceive::chart_holding_one(&bodies)
        .expect("unique chart holding one body");
    assert_eq!(chart, dot.moduli, "answers the chart whose fiber holds one");
}

/// Y4: A family whose rule selects that body extracts a rule where it
/// extracted a demand before. 78.167 Phase B: the act reader states it —
/// the bodies not selected erased — and the rule is its clauses alone.
#[test]
fn a_family_whose_rule_selects_that_body_extracts_a_rule_vertex_where_it_extracted_a_demand_before() {
    let family = vec![
        (
            grid(&[
                "110304",
                "000304",
                "220000",
                "000005",
            ]),
            grid(&[
                "000000",
                "000000",
                "000000",
                "000005",
            ]),
        ),
        (
            grid(&[
                "110304",
                "000304",
                "220000",
                "000006",
            ]),
            grid(&[
                "000000",
                "000000",
                "000000",
                "000006",
            ]),
        ),
    ];
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let rule = extract_rule(&family, &mut budget).expect("the family extracts");
    assert!(!rule.form.elements.is_empty(), "the rule states the selection as clauses: {:?}", rule.form.vertices);
    assert!(
        rule.form.demand_vertices().next().is_none(),
        "zero demands where it extracted demands before: {:?}",
        rule.form.vertices
    );
    assert!(rule.form.is_closed(), "the extracted rule form is closed");
}

/// 51.3 — A BODY CARRIES ITS FRAME ON ITS SUPPORT, one carrier: a square
/// perceived in frame 1 has its compartments at `k = 1`, its centroid there,
/// and the same chart as the square in frame 0.
#[test]
fn perception_3d_coordinates_preserve_frame_and_moments() {
    let compartments: Vec<Compartment> =
        [(0i64, 0i64, 1i64, 1u8), (0, 1, 1, 1), (1, 0, 1, 1), (1, 1, 1, 1)].map(Compartment::from).to_vec();
    let b3d = body_of(compartments).expect("a square is heard at n = 3");
    assert_eq!(b3d.compartments.len(), 4);
    assert!(b3d.compartments.iter().all(|c| c.site.axis(2) == Some(1)), "the support stands in its frame");
    assert_eq!(b3d.shape.blade.inertia.len(), 6);
    let c3 = b3d.shape.blade.centroid().expect("3d centroid");
    assert_eq!(c3[2], num_rational::BigRational::from_integer(num_bigint::BigInt::from(1)));
    let at_zero = body_of(
        [(0i64, 0i64, 0i64, 1u8), (0, 1, 0, 1), (1, 0, 0, 1), (1, 1, 0, 1)].map(Compartment::from).to_vec(),
    )
    .expect("a square is heard at n = 3");
    assert_eq!(at_zero.moduli, b3d.moduli, "a frame carries no meaning on the chart (51.1)");
    assert!(carriage(&at_zero, &b3d, &mut super::common::budget()).is_some(), "and the translation along e₃ carries one onto the other");
}

#[test]
fn six_adjacency_connects_bodies_across_frames_into_one_streamline() {
    let frame0 = grid(&["10", "00"]);
    let frame1 = grid(&["10", "00"]);
    let bodies = perceive_frames(&[frame0, frame1]);
    assert_eq!(bodies.len(), 1, "two adjacent compartments across k form one 3D body streamline");
    assert_eq!(
        bodies[0].shape.blade.shell,
        num_rational::BigRational::from_integer(num_bigint::BigInt::from(2))
    );
}

/// Audit N-3 — A SITE OF A BASE WITH ONE AXIS HAS NO GRID PLANE, and its
/// section is refused by name.
#[test]
fn a_sectionless_site_is_counted_at_the_one_producer() {
    use crate::transducer::family::perceive::on_grid;
    assert_eq!(on_grid(&Site::of([4, 5, 0])), Some((4, 5)), "a site of n = 3 has its section");
    assert_eq!(on_grid(&Site::of([4])), None, "a site of n = 1 has no grid plane");
}

