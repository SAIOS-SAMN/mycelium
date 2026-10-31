// 12.6 — THE TRAJECTORY and THE PIVOT: the trajectory certifies and never selects, the
// pivot reads only occupied axes, and what cannot be ordered is held.

use super::common::*;
use crate::network::nerve::Keyed;
use super::pivot_orbit::Hold;
use crate::transducer::alphabet::{Order, Word};
use crate::transducer::element;
use crate::transducer::sentence::*;

/// Pivot candidates are read from occupied axes only, and the occupied axes
/// come in declaration order.
#[test]
fn coordinates_lists_occupied_axes_in_declaration_order() {
    let id = ClassVector::of_word(&Word::identity()).expect("identity");
    assert!(id.occupied().is_empty() && id.is_zero());
    let k2 = ClassVector::of_word(&word_turn_180()).expect("180° turn");
    let occ = k2.coordinates();
    assert_eq!(occ, vec![Axis::Orientation(0)], "sheet parity occupies Sheet(0), got {occ:?}");
    let inf = ClassVector::of_word(&infinite()).expect("infinite");
    assert!(
        inf.free_part(),
        "a pythagorean-5 holonomy is free"
    );
    let mut seen = occ.clone();
    seen.extend(inf.coordinates());
    let mut declaration = seen.clone();
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — canonical key order: 12.6's own axis order, which is what a declaration is written in"
    )]
    declaration.sort_by_key(|a| match a {
        Axis::Orientation(i) => (0, *i),
        Axis::Unit(i) => (1, *i),
        Axis::Prime5(i) => (2, *i),
        Axis::Prime13(i) => (3, *i),
        Axis::Hue => (4, 0),
        Axis::Placement => (5, 0),
        Axis::Dilation => (6, 0),
        Axis::OffPlane => (7, 0),
        Axis::Open => (8, 0),
    });
    assert_eq!(seen, declaration, "occupied axes already in declaration order");
}

/// A16 step 5: SEVERAL discharges are a vortex. Not "several that cannot be
/// ordered" — several, whatever each one's μ⃗ says, because nothing orders them.
#[test]
fn select_discharge_holds_several_as_vortex() {
    let a = cert(Axis::Unit(0), [2, 0, 1, 0]);
    let b = cert(Axis::Hue, [0, 3, 0, 1]);
    assert!(
        matches!(select_discharge(&[a, b]), Discharge::Vortex(_)),
        "two certified discharges are a vortex, not a pick"
    );
}

/// T13 — μ⃗ NEITHER CERTIFIES NOR SELECTS. Two candidates discharge and one
/// measures lower on every lane: it is still a vortex. Dominance used to
/// apply that one, which made the lower measurement a reason, which is a
/// score (law 1). μ⃗ enters a trial once, to refuse a candidate that RAISES
/// a lane, and never enters the comparison between certified discharges.
#[test]
fn select_discharge_never_selects_by_mu() {
    let loose = cert(Axis::Unit(0), [4, 4, 4, 4]);
    let tight = cert(Axis::Unit(0), [1, 1, 1, 1]);
    let Discharge::Vortex(candidates) = select_discharge(&[loose.clone(), tight.clone()]) else {
        panic!("a lower μ⃗ on every lane is not a reason to apply it");
    };
    assert_eq!(candidates.len(), 2, "both are held: {candidates:?}");
    fn perm(m: [u64; 4]) -> [u64; 4] {
        [m[2], m[1], m[0], m[3]]
    }
    fn perm_mu(mu: Mu) -> Mu {
        match mu {
            Mu::Measured { before, after } => Mu::Measured {
                before,
                after: perm(after),
            },
            Mu::Unmeasured => Mu::Unmeasured,
        }
    }
    let mut lp = loose.clone();
    let mut tp = tight.clone();
    lp.mu = perm_mu(lp.mu);
    tp.mu = perm_mu(tp.mu);
    let permuted = select_discharge(&[lp, tp]);
    assert_eq!(
        permuted,
        select_discharge(&[loose, tight]),
        "permuting the lanes changes nothing, because nothing read them"
    );
    let a = cert(Axis::Unit(0), [2, 0, 0, 0]);
    let b = cert(Axis::Hue, [0, 2, 0, 0]);
    assert!(
        matches!(select_discharge(&[a, b]), Discharge::Vortex(_)),
        "and two on different axes are a vortex too"
    );

    let q = word_turn_90();
    let qinv = q.inverse();
    let x: Vec<Vec<u8>> = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
    ];
    let bodies = crate::transducer::family::perceive(&x);
    assert!(!bodies.is_empty(), "the bar is a body");
    let class = bodies[0].moduli.clone();
    assert!(!class.0.is_empty(), "the body has a chart");
    let mut spoken = Form::default();
    spoken
        .put_vertex(Vertex::word_on(
            crate::transducer::alphabet::Derivation::spelled(q.clone()),
            class.clone(),
            Box::new(element::carriage(&q1(), &(q1() + q1() + q1(), q1() + q1() + q1()))),
            vec![(1, 1)],
            [FRAME],
            None,
        ))
        .expect("q's vertex has a coordinate");
    let y = crate::transducer::family::apply_form(
        &spoken,
        &x,
        &Word::identity(),
        &crate::transducer::family::NoCrystal,
    )
    .and_then(|s| s.grid())
    .expect("q speaks the bar and it fits the frame");
    assert!(
        y.iter().flatten().any(|&h| h != 0),
        "the spoken image is not the empty frame"
    );
    // THE CHORD'S FIRST END STANDS ON THE BODY'S CHART. The chart is the
    // key the member is filed under, so putting the end on `class` is
    // building the member on that chart. AT1: the chord form is built here
    // as `chord_form` builds it — two members of the identity's class, each
    // starred from the Frame, the chord carrying `q` — with the first
    // member on `class` rather than rewritten into place afterwards, since a
    // vertex is keyed by its coordinate and its overlaps by its charts.
    let mut f = Form::default();
    let a = stand_with_star(
        &mut f,
        &Vertex::word_on(
            crate::transducer::alphabet::Derivation::spelled(Word::identity()),
            class,
            Box::new(element::carriage(&q1(), &(q1() + q1() + q1(), q1() + q1() + q1()))),
            vec![(1, 1)],
            [FRAME],
            None,
        ),
    );
    let b = stand_with_star(
        &mut f,
        &word_vertex_on(2, Word::identity(), crate::transducer::stalk::Invariant::counts(&[2])),
    );
    chord(&mut f, &a, &b, q.clone());
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    let bundle = vec![(x, y)];
    let snaps = f.pivot_orbit(
        [0xde; 32],
        &Keyed::new(),
        &Hold(vec![q.clone(), qinv]),
        &mut crate::physics::Physics::default().stroke_n(32),
        Some(&bundle),
        3,
    );
    let Some(Snap::Vortex { held, between, .. }) =
        snaps.iter().find(|s| matches!(s, Snap::Vortex { .. }))
    else {
        panic!("two discharges on one orbit are a vortex: {snaps:?}");
    };
    assert!(held.len() >= 2, "q and q⁻¹ both discharge: {held:?}");
    let (mu0, mu1) = match (&held[0].mu, &held[1].mu) {
        (
            Mu::Measured { before: b0, after: a0 },
            Mu::Measured { after: a1, .. },
        ) => {
            assert_eq!(b0[2], 3, "T is the extent passed in");
            assert_eq!(a0[2], 3);
            (*a0, *a1)
        }
        _ => panic!("a measured vortex carries μ⃗, got {:?}", held[0].mu),
    };
    assert_eq!(mu0, mu1, "a chord write moves no vertex, so μ⃗ is unmoved");
    // S9.16 — THE VORTEX RECORDS THE OBSTRUCTION, and spends it on nothing.
    // Here the two candidates measure the same μ⃗, so every lane HELD; the
    // axes they acted on are what separates them, and that is what a later
    // collapse resolves.
    assert_eq!(between.len(), held.len() - 1, "one reading per adjacency");
    let o = &between[0];
    assert_eq!(
        o.lanes,
        Some([crate::economy::LaneMove::Held; 4]),
        "equal μ⃗ reads as four held lanes, not as a tie score"
    );
    assert_eq!(o.axes, (held[0].axis, held[1].axis), "the axes are the obstruction");
}

/// 54.2 — AN ORBIT THAT CLOSES WITH MONODROMY IS CLOSED, and one that never
/// returns is not.
///
/// This opened `assert!(!f.is_closed())` on a form whose single orbit is a
/// 90° turn about a fixed point — a `ℤ/4ℤ` class, the most ordinary
/// torsion there is — and that assertion was the whole defect in one line.
/// `coheres` demanded holonomy ≡ 1 on every orbit, which is `H¹ = 0`, and
/// since only closed forms are stood and only closed forms survive the
/// court, curvature could not enter the genome. Measured on the development deployment:
/// 1 870 extracted forms carried orbits, filed torsion was 0 bytes on every
/// voice, `REFUSE_NOT_CLOSED` was 36 of 39 wire refusals.
///
/// Both arms are asserted now, so admitting curvature cannot be mistaken
/// for admitting everything: the compact class closes, and an
/// infinite-order holonomy — a free generator, which never returns — does
/// not. The free part is what a demand is for (52.3).
#[test]
fn pivot_orbit_cauterizes_filed_finite_torsion() {
    let mut f = chord_form(&[word_turn_90()]);
    assert!(
        f.is_closed(),
        "a 90° turn about a fixed point closes at four: the form carries a ℤ/4ℤ \
         class and is closed WITH it, not opened BY it"
    );
    assert!(
        !chord_form(&[infinite()]).is_closed(),
        "and an infinite-order holonomy is not compact: it never returns, and the form stands open"
    );
    let _ = f.pivot_orbit(
        [0xca; 32],
        &crate::network::nerve::Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    assert!(f.is_cauterized(), "filed finite torsion with empty remainder cauterizes");
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("crystallized form");
    assert!(
        carries_torsion(&e),
        "the tape carries the filed cycle"
    );
}

/// Two orbits of one class are both filed, or both discharged.
#[test]
fn pivot_orbit_treats_both_orbits_of_class_alike() {
    let w = word_turn_90();
    fn two_chords(w: &Word) -> Form {
        chord_form(&[w.clone(), w.clone()])
    }
    let mut f = two_chords(&w);
    assert_eq!(f.orbits.len(), 2, "two independent chords");
    let snaps = f.pivot_orbit(
        [0xa3; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    let filed = snaps
        .iter()
        .filter(|s| matches!(s, Snap::TorsionFiled { .. }))
        .count();
    assert_eq!(filed, 2, "both orbits of one class file: {snaps:?}");

    let q = word_turn_90();
    let mut f = two_chords(&q);
    let snaps = f.pivot_orbit(
        [0xa3; 32],
        &Keyed::new(),
        &Hold(vec![q.clone()]),
        &mut crate::physics::Physics::default().stroke_n(32),
        None,
        0,
    );
    let disc = snaps
        .iter()
        .filter(|s| matches!(s, Snap::Discharged { .. }))
        .count();
    assert_eq!(disc, 2, "both orbits discharge: {snaps:?}");
}

/// The pivot never reopens a closed chord.
#[test]
fn pivot_orbit_never_reopens_closed_chord() {
    let mut cob = coboundary_chord_form();
    // AT1 — THE TREE GAUGE RETIRED (ruled), and with it the `Closed` snap
    // this asserted ("a coboundary chord closes by the tree gauge"). A cycle
    // of derived face words telescopes to the identity, so the chord is
    // closed before the pivot reads it; what stands is T2 below — it stays
    // closed and is never demanded.
    let cob_snaps = cob.pivot_orbit(
        [0xcd; 32],
        &crate::network::nerve::Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    assert!(
        !cob_snaps.iter().any(|s| matches!(s, Snap::Demand { .. })),
        "a coboundary chord is not demanded: {cob_snaps:?}"
    );
    assert!(
        cob.orbits
            .iter()
            .all(|lp| orbit_word_on(&cob.faces, &cob.vertices, lp).is_some_and(|w| w.is_identity())),
        "T2: the closed chord stays closed"
    );
    let again = cob.pivot_orbit(
        [0xcd; 32],
        &crate::network::nerve::Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    assert!(
        !again.iter().any(|s| matches!(s, Snap::Demand { .. })),
        "T2: a closed chord is not demanded: {again:?}"
    );

    let mut f = chord_form(&[word_turn_180(), word_turn_90()]);
    let mut budget = crate::physics::Physics::default().stroke_n(16);
    let first = f.pivot_orbit(
        [0xcd; 32],
        &crate::network::nerve::Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut budget,
        None,
        0,
    );
    let closed: Vec<u64> = first
        .iter()
        .filter_map(|s| match s {
            Snap::Closed {
                order: Order::Finite(e),
                ..
            } => Some(u64::from(*e)),
            Snap::TorsionFiled { ell, .. } => Some(*ell),
            _ => None,
        })
        .collect();
    let mut budget = crate::physics::Physics::default().stroke_n(16);
    let second = f.pivot_orbit(
        [0xcd; 32],
        &crate::network::nerve::Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut budget,
        None,
        0,
    );
    for o in closed {
        assert!(
            !second.iter().any(|s| matches!(s, Snap::Demand { .. })) || o > 0,
            "a closed finite chord is not demanded"
        );
        let still = f.orbits.iter().any(|lp| {
            orbit_word_on(&f.faces, &f.vertices, lp).is_some_and(|h| {
                !h.is_identity() && matches!(h.order(), Order::Finite(ell) if u64::from(ell) == o)
            })
        });
        // THE SPECTRUM, not the envelope: a filing whose `lcm` matches and
        // whose planes do not is a different orbit (54.7).
        let filed = f
            .torsion
            .scoped()
            .any(|(_, filed)| filed.periods.yield_units().is_some_and(|u| u + 1 >= o));
        assert!(
            !still || filed,
            "order {o} stayed closed or filed; second {second:?}"
        );
    }
}

/// 12.6, T21 (iii), S9.1 — A PIVOT MOVES ONE UNIT OR NONE. `Rank::moved`
/// names the move: the rank held, or exactly one unit left torsion, free or
/// off-plane for `axes`. Everything else is `Inadmissible` and carries its
/// four deltas so the refusal can say what happened.
///
/// A four-coordinate monotone would admit `(−3, −2, −1, +7)` — three
/// coordinates emptied and seven axes opened — as readily as A18's one
/// step. That is not a pivot that went well; it is a different thing the
/// audit could not name. This exercises the AUDIT and never runs a loop.
#[test]
fn rank_moved_admits_one_unit_or_none() {
    let before = Rank { torsion: 3, free: 2, offplane: 1, axes: 0 };

    // A18's step, from each coordinate it can leave: one unit, whether or
    // not the discharge opened a chart axis for it to land in. Both shapes
    // are asserted; since R14 counted `axes` by class, the first is what an
    // opening over a class two orbits share now makes.
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
