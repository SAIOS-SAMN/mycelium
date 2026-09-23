// A6, A7 — the chart: read from the rotor and never from a matrix, a prefix
// of one chart is its coarser stratum, and refinement splits a stratum by
// the coordinate that arrived.

use crate::transducer::element::{from_canonical, from_connection};
use crate::transducer::germinate::germinate;
use crate::transducer::element::{self, identity_versor, Element};
use crate::transducer::stalk::*;


fn chain(sites: &[(i64, i64)]) -> element::CanonicalChain {
    let raw: Vec<Link> = sites.iter().map(|&(r, c)| (r, c, 0u8, 1u8)).collect();
    element::canonical_chain(&raw).expect("one entry per site is A5's chain")
}

#[test]
fn chart_axes_read_from_the_rotor_not_a_matrix() {
    let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let rot = crate::transducer::alphabet::Word::from_letters(&[(q, false)]);
    let w = from_connection(&rot, &identity_versor(), &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0])
        .expect("connection");
    let inv = invariant_of(&w);
    assert!(!inv.0.is_empty(), "a connection has a chart");
    // A SOURCE-TEXT GATE STOOD HERE and it is deleted. It `include_str!`d
    // four production files and asserted their BYTES did not contain
    // "word.fiber(" or "GERM_DIM" — two spellings that exist nowhere in the
    // tree, so the assertion could only ever pass, and what it checked was
    // the text of the engine rather than the behaviour of it. That is §4.6,
    // and it is the gate disease reappearing inside the test suite after
    // eighteen gate files were deleted for being exactly this.
    //
    // The property it claimed is checked below on the object: the chart is
    // read off the rotor, so it is invariant under the motions a rotor
    // spells and it refines by the coordinate that arrived. A test that
    // greps for a forbidden identifier proves nothing about either, and it
    // goes green on a file that has been rewritten to do the wrong thing
    // under a different name.
    let q2 = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let twice = crate::transducer::alphabet::Word::from_letters(&[(q2, false), (q2, false)]);
    let half = from_connection(&twice, &identity_versor(), &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0])
        .expect("connection");
    assert_ne!(
        inv.0,
        invariant_of(&half).0,
        "a quarter turn and a half turn are different charts, read from the rotor itself"
    );
}

#[test]
fn chart_prefix_is_its_coarser_stratum() {
    let coarse = Invariant::counts(&[1]).key();
    let fine = Invariant::counts(&[1, 2]).key();
    let sib = Invariant::counts(&[1, 3]).key();
    assert!(
        fine.as_bytes().starts_with(coarse.as_bytes()),
        "a finer chart's key starts with the coarser chart"
    );
    assert!(sib.as_bytes().starts_with(coarse.as_bytes()));
    assert!(!fine.as_bytes().starts_with(sib.as_bytes()));
    let mut k: crate::network::nerve::Keyed<u8> = crate::network::nerve::Keyed::new();
    k.insert(coarse.as_bytes(), 0);
    k.insert(fine.as_bytes(), 1);
    k.insert(sib.as_bytes(), 2);
    let at = crate::network::nerve::Prefix::new(coarse.as_bytes()).expect("a coarse key is a coordinate");
    let fiber: Vec<_> = k.fiber(at).map(|(key, v)| (key, *v)).collect();
    assert_eq!(fiber.len(), 3, "the coarse key's fiber is the coarser stratum");
    assert!(fiber.iter().all(|(key, _)| key.starts_with(coarse.as_bytes())));
}

/// The chart is invariant under translation, rotation, reflection, and
/// exact dilation, with no group enumeration.
#[test]
fn the_chart_is_invariant_under_the_conformal_group() {
    let l = chain(&[(0, 0), (1, 0), (2, 0), (2, 1)]);
    let base = invariant(&l);
    let moved = chain(&[(7, -3), (7, -2), (7, -1), (6, -1)]); // rotated + translated
    assert_eq!(invariant(&moved), base, "an isometry image shares the chart");
    let mirrored = chain(&[(0, 1), (1, 1), (2, 1), (2, 0)]);
    assert_eq!(invariant(&mirrored), base, "a reflection shares the chart");
    let big: Vec<Link> = l
        .iter()
        .flat_map(|&(r, c, b, a)| {
            [(2 * r, 2 * c), (2 * r, 2 * c + 1), (2 * r + 1, 2 * c), (2 * r + 1, 2 * c + 1)]
                .into_iter()
                .map(move |(rr, cc)| (rr, cc, b, a))
        })
        .collect();
    assert_eq!(invariant(&big), base, "a dilation shares the chart");
}

/// Distinct orbits separate on structure: a square, a bar, and a ring.
#[test]
fn structure_separates_equal_volumes() {
    let square = chain(&[(0, 0), (0, 1), (1, 0), (1, 1)]);
    let bar = chain(&[(0, 0), (0, 1), (0, 2), (0, 3)]);
    let ring = chain(&[(0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2)]);
    assert_ne!(invariant(&square), invariant(&bar));
    assert_ne!(invariant(&square), invariant(&ring));
    // The chart opens with the base's n, then its n Betti numbers, then the
    // turns (BQ4).
    assert_eq!(&invariant(&square).0[..5], &[3u64, 1, 0, 0, 4].map(ChartAxis::Count), "the base of three, one body, no hole, no cavity, four turns");
    assert_eq!(&invariant(&ring).0[..5], &[3u64, 1, 1, 0, 8].map(ChartAxis::Count), "the base of three, one body, one hole, no cavity, eight turns");
}

/// The lookup finds a rotated sibling's base point through its chart and
/// returns the versor; an empty stratum is novelty; the atlas holds
/// addresses, so a removal follows one path and nothing shifts.
#[test]
fn the_chart_finds_the_base_point_and_the_versor() {
    let held: Vec<Element> = vec![
        from_canonical(&chain(&[(0, 0), (0, 1), (0, 2)])),         // a bar
        from_canonical(&chain(&[(0, 0), (0, 1), (1, 0)])),         // an L
        from_canonical(&chain(&[(0, 0), (0, 1), (1, 0), (1, 1)])), // a square
    ];
    let mut stalks = Stalks::over_elements(held.iter());
    assert_eq!(stalks.len(), 3);
    let at = |a: &[u8]| held.iter().find(|w| w.address().as_bytes() == a);
    let bar_rotated = from_canonical(&chain(&[(5, 5), (6, 5), (7, 5)]));
    let (a, v) = congruent(&stalks, at, &bar_rotated).expect("the bar's orbit");
    assert_eq!(a, held[0].address());
    assert!(!element::same_versor(&v, &identity_versor()), "a rotation carries the vertical bar onto the base");
    let l_reflected = from_canonical(&chain(&[(3, 3), (3, 2), (4, 3)]));
    assert_eq!(congruent(&stalks, at, &l_reflected).map(|(a, _)| a), Some(held[1].address()));
    let zigzag = from_canonical(&chain(&[(0, 0), (0, 1), (1, 1), (1, 2)]));
    assert!(stalks.fiber(&invariant_of(&zigzag)).is_empty(), "an empty stratum is novelty");
    assert_eq!(congruent(&stalks, at, &zigzag), None);
    stalks.remove(&held[1].address());
    assert_eq!(stalks.len(), 2);
    let sq = from_canonical(&chain(&[(9, 9), (9, 10), (10, 9), (10, 10)]));
    assert_eq!(congruent(&stalks, at, &sq).map(|(a, _)| a), Some(held[2].address()));
    assert_eq!(congruent(&stalks, at, &l_reflected), None);
}

/// A connection is charted on its fibres: the linear class and the
/// gauge's cycle type, never a translation and never the empty chart;
/// residual charts and connection charts are disjoint families.
#[test]
fn a_connection_is_charted_on_its_fibres_not_on_an_empty_chain() {
    let a = from_connection(&crate::transducer::alphabet::Word::identity(), &element::translator_versor(0, 2), &[(1, 2), (2, 1)], &Vec::new(), [4, 2, 0, 0]).expect("spells");
    let b = from_connection(&crate::transducer::alphabet::Word::identity(), &element::translator_versor(5, -3), &[(1, 2), (2, 1)], &Vec::new(), [4, 2, 0, 0]).expect("spells");
    let c = from_connection(&crate::transducer::alphabet::Word::identity(), &element::translator_versor(0, 2), &[(1, 2), (2, 3), (3, 1)], &Vec::new(), [4, 2, 0, 0]).expect("spells");
    assert!(!invariant_of(&a).0.is_empty(), "a connection never lands on the empty chart");
    assert_eq!(invariant_of(&a), invariant_of(&b), "two identical motions at different origins share the chart");
    assert_ne!(invariant_of(&a), invariant_of(&c), "a different cycle type is a different chart");
    assert_eq!(invariant_of(&a).0[0], ChartAxis::Count(0), "the connection family's leading axis");
    let word = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
    assert!(matches!(invariant_of(&word).0[0], ChartAxis::Count(n) if n >= 1), "a residual chart leads with its component count");
    // Congruence on connections: the fibre's representative and the transition.
    let stalks = Stalks::over_elements([&a, &c].into_iter());
    let at = |x: &[u8]| [&a, &c].into_iter().find(|w| w.address().as_bytes() == x);
    assert_eq!(congruent(&stalks, at, &b).map(|(x, _)| x), Some(a.address()), "b is a's word, placed elsewhere");
    assert_eq!(congruent(&stalks, at, &c), None, "c has no other word on its chart");
}

/// Connecting the bundle: a rotated, differently-hued sibling attaches
/// to the base as a section pulled back through V, the connection records
/// (assignment, V), a second observation adds nothing, and the whole
/// bundle round-trips its bytes.
#[test]
fn the_sibling_attaches_as_a_transported_section() {
    let mut base = from_canonical(
        &germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap(),
    );
    let sibling = from_canonical(
        &germinate(&[(0, 0, 3u8), (1, 0, 3)], &[(0, 0, 4u8), (1, 0, 4)]).unwrap(),
    );
    let v = element::transition(sibling.sub(), base.sub()).expect("congruent");
    assert_eq!(base.attach(&sibling, &v), 1);
    assert_eq!(base.attach(&sibling, &v), 0, "a second observation adds nothing");
    let bytes = base.to_bytes();
    let mut pos = 0usize;
    let back = Element::from_bytes(&bytes, &mut pos).expect("round trip");
    assert_eq!(back.address(), base.address(), "the bundle round-trips");
}

/// A class refines one axis; members re-home under the new prefix.
#[test]
fn refine_splits_the_stratum_by_the_new_coordinate() {
    let a = from_canonical(&chain(&[(0, 0), (0, 1), (0, 2)]));
    let b = from_canonical(
        &element::canonical_chain(&[(0, 0, 2, 3), (0, 1, 2, 3), (0, 2, 2, 3)])
            .expect("one entry per site is A5's chain"),
    );
    let mut stalks = Stalks::over_elements([&a, &b].into_iter());
    let inv = invariant_of(&a);
    assert_eq!(stalks.fiber(&inv).len(), 2);
    let class_coord = 7u64;
    stalks.split_stratum(&inv, |addr| {
        if a.address().as_bytes() == addr {
            vec![ChartAxis::Count(0)]
        } else {
            vec![ChartAxis::Count(class_coord)]
        }
    });
    assert!(stalks.fiber(&inv).is_empty(), "the coarse chart is empty after the split");
    assert_eq!(stalks.fiber(&Stalks::refine_axes(&inv, &[ChartAxis::Count(0)])), vec![a.address().into_vec()]);
    assert_eq!(stalks.fiber(&Stalks::refine_axes(&inv, &[ChartAxis::Count(class_coord)])), vec![b.address().into_vec()]);
    let peeled = Stalks::peel(&Stalks::refine_axes(&inv, &[ChartAxis::Count(class_coord)]));
    assert_eq!(peeled, inv, "peel drops the class axis");
    // The prefixes run coarse-last, so the tail is the root stratum.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 15 — canonical key order: `prefixes` writes the strata from finest to coarsest, so its tail is the root"
    )]
    let coarsest = inv.prefixes().last().map(|p| p.0.is_empty());
    assert_eq!(coarsest, Some(true), "the coarsest prefix is the root stratum");
}

/// A21 as ruled 2026-09-18 — A PREFIX OF A CHART'S KEY IS A COARSER
/// STRATUM'S KEY, in bytes. `Keyed` is a PATRICIA trie over these bytes and
/// `fiber_under`, `link` and `peel` are byte-prefix reads, so this is the
/// one property the encoding has to hold. A count in the key's header would
/// break it — a chart and its prefix would differ in their first bytes —
/// which is why the key carries none and each coordinate carries its own
/// length instead.
#[test]
fn a_prefix_of_a_chart_is_a_byte_prefix_of_its_key() {
    use crate::transducer::stalk::ChartAxis;
    use num_bigint::BigInt;
    use num_rational::BigRational;
    let wide = BigRational::new(BigInt::from(1) << 80, BigInt::from(7));
    let inv = Invariant(vec![
        ChartAxis::Count(1),
        ChartAxis::Ratio(wide),
        ChartAxis::Count(4),
    ]);
    let key = inv.key();
    for coarser in inv.prefixes() {
        assert!(
            key.as_bytes().starts_with(coarser.key().as_bytes()),
            "a coarser stratum's key is a byte prefix: {coarser:?}"
        );
    }
    // AND IT ROUND-TRIPS WHOLE. A ratio wider than one limb is one
    // coordinate, so `peel` drops it entire and never between its numerator
    // and its denominator.
    assert_eq!(Invariant::from_key(key.as_bytes()), Some(inv.clone()), "recovered from its key");
    assert_eq!(inv.peel().0.len(), 2, "one whole coordinate leaves");
    assert_eq!(inv.peel().0[1], inv.0[1], "and the wide ratio it keeps is still itself, whole");
}

/// A7, A21 §2 — A ROTOR'S CONJUGACY STRATUM IS COORDINATES, NOT LIMBS.
///
/// `Rotor::class_axes` packed each invariant into two `u64` limbs through
/// `pack_axis`, and the chart wrote each limb as its own axis. `peel` pops
/// one axis, so it could stop between a numerator and its denominator and
/// leave a chart naming half a rational as a "coarser stratum" — the N7
/// defect `ChartAxis::Ratio` exists to answer. `pack_axis` also refused any
/// invariant that did not fit `i64`/`u64`, so a rotor with large exact
/// invariants had no chart at all (rule 11 answered with a refusal where
/// the value was perfectly exact).
#[test]
fn a_rotors_conjugacy_stratum_is_whole_coordinates() {
    use crate::transducer::alphabet::{LetterId, Rotor, Word, PLANE_COL, PLANE_ROW};
    let quarter = {
        let id = LetterId::quarter((PLANE_ROW, PLANE_COL)).expect("the sensor plane");
        Word::from_letters(&[(id, false)]).rotor()
    };
    let axes = rotor_axes(&quarter).expect("a quarter turn carries a class");
    let [tag, sign_a, sign_p, a2, p2] = axes.as_slice() else {
        panic!("the stratum is the tag, two signs and two invariants: {axes:?}")
    };
    assert_eq!(*tag, ChartAxis::Count(2), "the leading tag");
    assert!(matches!(sign_a, ChartAxis::Count(_)) && matches!(sign_p, ChartAxis::Count(_)));
    assert!(
        matches!(a2, ChartAxis::Ratio(_)) && matches!(p2, ChartAxis::Ratio(_)),
        "each invariant is ONE coordinate carrying its own exact rational"
    );

    // PEEL DROPS A WHOLE COORDINATE. Every prefix of this chart round-trips
    // through the key, which a chart ending on half a rational could not.
    let inv = Invariant(axes.clone());
    let mut at = inv.clone();
    for expected in (0..=axes.len()).rev() {
        assert_eq!(at.0.len(), expected);
        let key = at.key();
        assert_eq!(Invariant::from_key(key.as_bytes()), Some(at.clone()), "{at:?} round-trips");
        if expected == 0 {
            break;
        }
        at = at.peel();
    }

    // An invariant too large for `i64` is charted, not refused: `1/3` alone
    // already has no two's-complement limb pair `pack_axis` would accept
    // without loss, and the rational is written whole here.
    let big = Rotor::scalar(crate::core::Q::new(
        num_bigint::BigInt::parse_bytes(b"123456789012345678901234567890", 10).expect("a big numerator"),
        num_bigint::BigInt::from(7),
    ));
    assert!(rotor_axes(&big).is_some(), "an exact invariant of any size charts");
}
