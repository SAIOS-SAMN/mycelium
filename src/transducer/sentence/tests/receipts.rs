// 54.5 Road (d) — THE RECEIPTS ENVELOPE: what a word attests, read back.
//
//! Three carriers rode the tape because the key excludes them as EVIDENCE
//! — a vertex's citations, a chart's citations of another chart, and the
//! pivots a crystallization rests on. These are the readings that say the
//! envelope carries them instead: that it survives the record, that a
//! 1-simplex carries the whole affine transition and not a bare rotor, and
//! that a triangle of edges composes to a holonomy the envelope can read
//! without anything else in hand.

use super::common::*;
use crate::transducer::alphabet::Word;
use crate::transducer::element::{NerveEdge, NerveTriangle, ReceiptsEnvelope};
use crate::transducer::sentence::*;

fn leaf(plane: (u8, u8)) -> Form {
    let q = crate::transducer::alphabet::LetterId::quarter(plane).expect("quarter");
    closed_word_form(Word::from_letters(&[(q, false)]), None)
}

fn addr_of(f: &Form) -> crate::transducer::address::Address {
    as_element_form(f, [1, 0, 0, 0]).expect("tape").address()
}

/// A translator by `(dx, dy)` — the `D(ρ)·T(t)` half of a transition.
fn shift(dx: i64, dy: i64) -> crate::transducer::element::Versor {
    use num_rational::BigRational;
    use num_bigint::BigInt;
    let q = |n: i64| BigRational::from(BigInt::from(n));
    crate::transducer::cga::translator(&q(dx), &q(dy), &q(0))
}

/// THE ENVELOPE SURVIVES THE RECORD (B1, B2).
///
/// The three carriers the key excludes go out on the element and come back
/// off it. Before this, they came back off `Element.description` — the tape
/// — and that is the whole reason the tape could not be retired.
#[test]
fn an_envelope_rides_the_record_and_reads_back() {
    let a = leaf((0, 1));
    let a_at = addr_of(&a);
    let mut host = Form::default();
    host.glue_append_under(&a, a_at.clone(), Word::identity(), [0, 0, 0, 1], shift(3, 0))
        .expect("host cites a");
    host.loops = Form::loops_of(&host.faces, &host.vertices);
    host.reindex();

    let e = as_element_form(&host, [1, 0, 0, 0]).expect("mint");
    assert_eq!(e.receipts().nerve.len(), 1, "one citation is one 1-simplex");
    assert_eq!(e.receipts().nerve[0].target, a_at, "the edge names the cited form");

    let bytes = e.to_bytes();
    let mut p = 0usize;
    let back = crate::transducer::element::Element::from_bytes(&bytes, &mut p).expect("record");
    assert_eq!(
        back.receipts(),
        e.receipts(),
        "the envelope is the same object on both sides of the record"
    );
    assert_eq!(back.address(), e.address(), "and the address did not move for it");
}

/// THE ENVELOPE IS NOT IN THE ADDRESS, AND THAT IS THE POINT (§1.5).
///
/// Evidence is not identity. Two words whose geometry is one and whose
/// attestations differ are ONE word at ONE address — otherwise a mind that
/// pivots would rename its own utterance and orphan every citation of it,
/// which is the argument that moved the filing at `df1eeaa`.
#[test]
fn evidence_does_not_rename_the_word() {
    let a = leaf((0, 1));
    let a_at = addr_of(&a);
    let mut host = Form::default();
    host.glue_append_under(&a, a_at, Word::identity(), [0, 0, 0, 1], shift(1, 0))
        .expect("host cites a");
    host.loops = Form::loops_of(&host.faces, &host.vertices);
    host.reindex();

    let plain = as_element_form(&host, [1, 0, 0, 0]).expect("mint");
    let mut loaded = plain.clone();
    let mut env = plain.receipts().clone();
    env.certificates.push(cert(Axis::Placement, [1, 0, 0, 0]));
    loaded.set_receipts(env);

    assert_ne!(
        plain.receipts(),
        loaded.receipts(),
        "the two words attest different things"
    );
    assert_eq!(
        plain.address(),
        loaded.address(),
        "and are one word at one address"
    );
}

/// A 1-SIMPLEX CARRIES THE WHOLE AFFINE TRANSITION, NOT A BARE ROTOR.
///
/// An inter-chart transition is a space-group element `g_ij = T(τ)·R`. An
/// edge that kept only `R` would force every chart overlap to be concentric
/// and throw the translational solder form away — the same collapse that
/// made every intra-form loop read flat.
///
/// The reading is the CARTAN SPLIT: `screw_of` solves `(I − R)p = t` by the
/// normal equations over ℚ, exactly. Here `R` is the identity, so `I − R` is
/// zero, `Im(I − R)` is trivial and the whole translation lands in
/// `ker(I − R)` as the twist — a pure dislocation, which is what a citation
/// glued three units over IS.
#[test]
fn a_citation_carries_its_slip_and_not_only_its_bend() {
    use num_traits::Zero;
    let a = leaf((0, 1));
    let a_at = addr_of(&a);
    let mut host = Form::default();
    host.glue_append_under(&a, a_at, Word::identity(), [0, 0, 0, 1], shift(3, 0))
        .expect("host cites a");
    host.loops = Form::loops_of(&host.faces, &host.vertices);
    host.reindex();

    let e = as_element_form(&host, [1, 0, 0, 0]).expect("mint");
    let [edge] = e.receipts().nerve.as_slice() else {
        panic!("one citation is one edge");
    };
    let screw = edge.screw().expect("a translator splits");
    assert!(
        screw.centre.iter().all(num_traits::Zero::is_zero),
        "the identity rotor bends nowhere, so there is no centre to hold still"
    );
    assert_eq!(
        screw.twist[0],
        num_rational::BigRational::from(num_bigint::BigInt::from(3)),
        "the Burgers vector is exactly the three units the citation was glued over"
    );
    assert!(screw.twist[1].is_zero() && screw.twist[2].is_zero(), "and nothing else moved");
}

/// THE SITE OF AN EDGE IS ITS OWN COORDINATE, NOT ITS POSITION (rule 15).
///
/// `host()` is the key contribution of the `Glued` vertex an edge came off,
/// asked of `put_vertex` — the one writer of that record — and never
/// re-written beside it. It hand-wrote the record beside `put_vertex`'s
/// `Glued` arm until 2026-09-21, and two byte runs that agree is identity
/// by bytes, which law 2 and §1.5 refuse.
///
/// The reading: glue two citations in one order and then the other. The
/// vertices land at different POSITIONS, and the envelope's listing is the
/// same either way, because `of_form` files edges through a `Keyed` on
/// their sites and a PATRICIA trie IS the order of its keys.
///
/// AND TWO CITATIONS OF DIFFERENT FORMS STAND AT ONE SITE — gauge
/// universality, asserted below: the host form has ONE `Glued` vertex and
/// the cover has TWO edges at it, because two isomorphic local boundaries
/// ARE one boundary and what distinguishes them is incidence in `N(𝒰)`.
#[test]
fn an_edge_stands_at_a_site_and_not_at_an_index() {
    let a = leaf((0, 1));
    let b = leaf((0, 2));
    let a_at = addr_of(&a);
    let b_at = addr_of(&b);
    assert_ne!(a_at, b_at, "two leaves at two planes are two forms");

    let sites = |first: (&Form, crate::transducer::address::Address),
                 second: (&Form, crate::transducer::address::Address)| {
        let mut host = Form::default();
        for (f, at) in [first, second] {
            host.glue_append_under(f, at, Word::identity(), [0, 0, 0, 1], shift(2, 0))
                .expect("cite");
        }
        host.loops = Form::loops_of(&host.faces, &host.vertices);
        host.reindex();
        let e = as_element_form(&host, [1, 0, 0, 0]).expect("mint");
        e.receipts().nerve.iter().map(NerveEdge::host).collect::<Vec<_>>()
    };
    let forward = sites((&a, a_at.clone()), (&b, b_at.clone()));
    let backward = sites((&b, b_at), (&a, a_at));
    assert_eq!(forward.len(), 2, "two citations are two 1-simplices");
    assert_eq!(
        forward, backward,
        "the listing is by the edges' own sites, so the order they were glued in is not in it"
    );
    let [x, y] = forward.as_slice() else { panic!("two edges") };
    assert_eq!(x, y, "and both stand at ONE site: the boundary is the same boundary");
}

/// A 2-SIMPLEX READS ITS HOLONOMY OFF ITS THREE EDGES (law 12).
///
/// The Čech differential `(δφ)_ijk = φ_jk · φ_ik⁻¹ · φ_ij` evaluates on a
/// TRIPLE overlap, and with a carrier only for 1-simplices that composite
/// has nowhere to be registered — the nerve would have generators and no
/// relations. `NerveTriangle` is the carrier; the holonomy is a READING of
/// the three edges and never a field beside them.
///
/// Three translations that sum to zero close. The same three with one
/// altered do not, and what they carry is the composite — content, not an
/// error.
#[test]
fn a_triangle_reads_the_cocycle_off_its_edges() {
    use num_traits::Zero;
    let a = leaf((0, 1));
    let a_at = addr_of(&a);
    let mk = |dx: i64, dy: i64| NerveEdge {
        target: a_at.clone(),
        chart: crate::transducer::sentence::chart_of_form(&a).expect("chart"),
        magnitude: [0, 0, 0, 1],
        depth: 0,
        placement: Word::identity(),
        carriage: Box::new(shift(dx, dy)),
    };
    let closes = [mk(1, 0), mk(0, 1), mk(-1, -1)];
    let tri = NerveTriangle { edges: [0, 1, 2], attached: None };
    let screw = tri.screw(&closes).expect("the composite splits");
    assert!(
        screw.twist.iter().all(num_traits::Zero::is_zero),
        "φ_ij φ_jk φ_ki = 1: three displacements summing to zero close the triangle"
    );

    let bent = [mk(1, 0), mk(0, 1), mk(-1, 0)];
    let screw = NerveTriangle { edges: [0, 1, 2], attached: None }
        .screw(&bent)
        .expect("the composite splits");
    assert!(
        !screw.twist[1].is_zero(),
        "a triangle that does not close CARRIES its holonomy — one unit of slip, \
         which is an H² obstruction and not a refusal"
    );
    assert_eq!(
        screw.twist[1],
        num_rational::BigRational::from(num_bigint::BigInt::from(1)),
        "and the obstruction is exact"
    );
}

/// AN ENVELOPE WITH NOTHING TO SAY IS EMPTY, NOT ABSENT (rule 11).
///
/// A form with no citation, no factored vertex and no certificate attests
/// nothing, and the record says so in four zero counts rather than omitting
/// the block — so a reader can tell "this word attests nothing" from "this
/// record was written before the envelope rode it".
#[test]
fn a_word_that_attests_nothing_says_so() {
    let f = leaf((0, 1));
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("mint");
    assert!(e.receipts().is_empty(), "a leaf cites nothing and pivots nothing");

    let mut block = Vec::new();
    ReceiptsEnvelope::default().to_block(&mut block);
    assert_eq!(block.len(), 4 + 32, "len prefix and four zero u64 counts");
    let mut p = 0usize;
    assert_eq!(
        ReceiptsEnvelope::from_block(&block, &mut p),
        Some(ReceiptsEnvelope::default()),
        "and it reads back as the empty envelope"
    );
    assert_eq!(p, block.len(), "the block's own length bounds the parse");

    // AND A ZERO-LENGTH BLOCK IS NOT THE EMPTY ENVELOPE. The writer never
    // makes one, so its presence is a record this build cannot read — and
    // admitting it would let a word that failed to serialize its evidence
    // read back as a word with no evidence to give. That is the
    // empty-carrier collapse, and it is refused rather than caught later.
    let mut p = 0usize;
    assert_eq!(
        ReceiptsEnvelope::from_block(&0u32.to_le_bytes(), &mut p),
        None,
        "an envelope that did not serialize is not an envelope with nothing in it"
    );
}

/// TWO FORMS CITING TWO DIFFERENT FORMS ARE ONE FORM (B5, 54.7).
///
/// This is the coordinate reading of the eviction, one level up from
/// `an_edge_stands_at_a_site_and_not_at_an_index`: not just that the two
/// EDGES stand at one site, but that the two CITING FORMS reach one
/// address. A host says "I have an open boundary that mates under φ at a
/// chart of this rank and this depth". Two hosts saying that say the same
/// thing, and WHICH form each boundary was instantiated against is the
/// cover's fact — incidence in `N(𝒰)` — and not a property of either
/// chart.
///
/// Before the eviction the cited address was inline, so these were two
/// addresses and the difference between them was 29 490 bytes of another
/// form's coordinates that neither host had any claim to.
#[test]
fn two_hosts_citing_two_forms_reach_one_coordinate() {
    let a = leaf((0, 1));
    let b = leaf((0, 2));
    let a_at = addr_of(&a);
    let b_at = addr_of(&b);
    assert_ne!(a_at, b_at, "two leaves at two planes are two forms");

    let cite = |f: &Form, at: crate::transducer::address::Address| {
        let mut host = Form::default();
        host.glue_append_under(f, at, Word::identity(), [0, 0, 0, 1], shift(2, 0))
            .expect("cite");
        host.loops = Form::loops_of(&host.faces, &host.vertices);
        host.reindex();
        host
    };
    let ha = cite(&a, a_at.clone());
    let hb = cite(&b, b_at);
    assert_eq!(
        form_vertex_key(&ha),
        form_vertex_key(&hb),
        "two isomorphic local boundaries ARE one boundary"
    );

    // AND THE ENVELOPES SEPARATE THEM. Gauge universality is not a loss of
    // information — it is the information moving to the carrier that owns
    // it. The two words are one word; the two ATTESTATIONS are two.
    let ea = as_element_form(&ha, [1, 0, 0, 0]).expect("mint");
    let eb = as_element_form(&hb, [1, 0, 0, 0]).expect("mint");
    assert_eq!(ea.address(), eb.address(), "one address");
    assert_ne!(ea.receipts(), eb.receipts(), "two covers");
}

/// A CITATION COSTS THE SAME WHATEVER IT CITES, AND THAT IS WHAT SEVERS
/// THE ACYCLICITY (B5, 54.7).
///
/// Inlining `A_c` meant `key(A) ⊃ key(B)`, so `|A| > |B|` for every
/// citation, and by well-ordering of the naturals no cycle `A → … → A`
/// could exist — the cognitive manifold was a directed acyclic forest
/// because of how it was NAMED, and `b₁` of the nerve was zero by
/// construction rather than by measurement.
///
/// The reading that the constraint is gone is not a byte run's absence; it
/// is that the host's key has the same DIMENSION whichever form it cites.
/// Here one host cites a leaf and the other cites a form that already
/// carries a citation of its own — strictly larger, by a margin that grows
/// with depth — and the two hosts stand at keys of one size. With no strict
/// order on lengths there is no well-ordering argument, and a cycle is
/// expressible.
#[test]
fn a_citation_does_not_grow_with_what_it_cites() {
    let a = leaf((0, 1));
    let a_at = addr_of(&a);

    let cite = |f: &Form, at: crate::transducer::address::Address| {
        let mut host = Form::default();
        host.glue_append_under(f, at, Word::identity(), [0, 0, 0, 1], shift(2, 0))
            .expect("cite");
        host.loops = Form::loops_of(&host.faces, &host.vertices);
        host.reindex();
        host
    };
    let one = cite(&a, a_at.clone());
    let one_at = addr_of(&one);
    assert!(
        one_at.as_bytes().len() > a_at.as_bytes().len(),
        "a form that cites is larger than the leaf it cites — it has more cells"
    );

    let two = cite(&one, one_at);
    assert_eq!(
        form_vertex_key(&one).as_bytes().len(),
        form_vertex_key(&two).as_bytes().len(),
        "the citation is O(1) in what it cites: placement, depth, carriage, rank"
    );
}

/// A CITED ADDRESS IS NOT BOUNDED BY 64 KB, and the codec must not be
/// either.
///
/// `put_addr` is the wire embedding — `len16 + bytes` — and refuses an
/// address it cannot fit. A form's address is its coordinates written out
/// (law 2) and a 1 655-vertex form's runs to 10 MB, so an envelope written
/// with `put_addr` would have refused every citation of a real form and
/// come back attesting nothing. The envelope uses the key-field embedding
/// (u32) throughout.
#[test]
fn an_envelope_carries_an_address_wider_than_the_wire_prefix() {
    let wide = crate::transducer::address::Address::new(vec![7u8; 70_000]);
    let env = ReceiptsEnvelope {
        derivations: vec![crate::transducer::element::DerivationCite {
            vertex: wide.clone(),
            generators: vec![(wide.clone(), wide.clone(), 1)],
            residue: Word::identity(),
        }],
        ..ReceiptsEnvelope::default()
    };
    let mut block = Vec::new();
    env.to_block(&mut block);
    let mut p = 0usize;
    let back = ReceiptsEnvelope::from_block(&block, &mut p).expect("a wide address round-trips");
    assert_eq!(back, env, "the 70 000-byte address survives, whole");
    assert_eq!(p, block.len(), "and the block is consumed exactly");
}
