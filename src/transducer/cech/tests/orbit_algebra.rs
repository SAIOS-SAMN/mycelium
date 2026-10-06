// 78.11, CG1 — a cell's orbit algebra: congruence per mode ray by commutator
// with the round, the failing grade, mode and plane on the refusal;
// coherence jointly; nothing read where nothing crossed or nothing read.

use crate::network::nerve::coord::BivectorKind;
use crate::transducer::cech::*;
use crate::transducer::cga::at_three::translator;
use crate::transducer::cga::{self, ConstructorOrbit, Mv, E1, E2, E3};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// The round `e₁ ∧ e₂` and the two constructors whose meet it is.
fn round() -> (Mv, Vec<Mv>) {
    let nuclei = vec![Mv::blade(E1, q(1, 1)), Mv::blade(E2, q(1, 1))];
    (meet_of(&nuclei).expect("two nuclei meet"), nuclei)
}

/// The `n`th overlap a fixture's rings step over: the Frame's chart to the
/// Frame's own member on chart `[n + 1]` at frame 0 — one overlap per ring,
/// each its own coordinates (AT1: a face is an overlap, never a slot).
fn overlap(n: u64) -> crate::network::nerve::coord::Overlap {
    use crate::network::nerve::coord::{ChartAt, Overlap, VertexAt};
    Overlap {
        from: ChartAt::Frame,
        to: ChartAt::at(VertexAt::Frame, crate::transducer::stalk::Invariant::counts(&[n + 1]), 0),
    }
}

/// Circuits crossing a cell, each its own ring — one step over its own face
/// — with the hue its steps compose: none carries a hue map, so the composed
/// map is empty, measured.
fn at(holonomies: Vec<Result<Mv, CircuitUnread>>) -> Vec<CellCircuit> {
    holonomies
        .into_iter()
        .zip(0u64..)
        .map(|(holonomy, i)| CellCircuit { ring: CellCircuit::ring_of(&[(overlap(i), true)]), holonomy, hue: Some(Vec::new()) })
        .collect()
}

/// A CIRCUIT TURNING IN THE ROUND'S OWN PLANE KEEPS IT: every mode ray
/// commutes with the round — congruent — and the 90° turn closes at
/// four; the constructors close on their two rays.
#[test]
fn a_turn_in_the_rounds_plane_is_congruent_and_closes() {
    let (m, nuclei) = round();
    let turn = cga::rotator(&q(1, 1), &q(1, 1));
    let cell = read_cell(1, Some(&m), Some(&nuclei), &at(vec![Ok(turn)]), &[], &mut stroke());
    assert_eq!(cell.congruence, Congruence::Congruent);
    assert_eq!(cell.coherence, Coherence::Closes(4));
    assert_eq!(cell.constructors, ConstructorOrbit::Closed { rays: 2 });
}

/// A CIRCUIT CARRYING THE ROUND IS REFUSED WITH ITS CONTENT: at the cell's
/// grade, the failing circuit and mode, the mode's class and plane, and the
/// bracket — the direction the round is carried.
#[test]
fn a_turn_that_carries_the_round_refuses_with_grade_mode_and_plane() {
    let (m, nuclei) = round();
    let across = cga::rotator_on_blade(&q(1, 1), &q(1, 1), E2 | E3);
    let cell = read_cell(2, Some(&m), Some(&nuclei), &at(vec![Ok(Mv::scalar(q(1, 1))), Ok(across)]), &[], &mut stroke());
    let Congruence::Incongruent { grade, failures } = &cell.congruence else { panic!("refused: {:?}", cell.congruence) };
    assert_eq!(*grade, 2, "the failing grade rides the refusal");
    let [f] = failures.as_slice() else { panic!("one failing mode: {failures:?}") };
    assert_eq!((f.circuit, f.mode, f.kind, f.plane.clone()), (1, 0, BivectorKind::Elliptic, vec![E2 | E3]));
    assert!(
        matches!(&f.round, RoundKept::Carried(bracket) if !bracket.is_zero()),
        "the bracket is the direction the round is carried: {:?}",
        f.round
    );
}

/// CG1b — A TURN INSIDE THE SPAN KEEPS THE ROUND AND MOVES THE CONSTRUCTORS:
/// the 90° turn in `e₁₂` keeps `e₁ ∧ e₂` and carries `e₁` onto `e₂`'s
/// ray — a permutation of the two. A turn by `(3, 4, 5)` in the same plane
/// keeps the round and carries every nucleus off the set.
#[test]
fn a_turn_inside_the_span_keeps_the_round_and_not_the_constructors() {
    let (m, nuclei) = round();
    let turn_90 = read_cell(1, Some(&m), Some(&nuclei), &at(vec![Ok(cga::rotator(&q(1, 1), &q(1, 1)))]), &[], &mut stroke());
    assert_eq!((turn_90.congruence.clone(), turn_90.permutes.clone()), (Congruence::Congruent, Permutes::All));
    assert_eq!(turn_90.modes[0].permutation, Some(vec![1, 0]), "e₁ ↦ e₂, e₂ ↦ −e₁: the swap, as rays");
    let rosette = read_cell(1, Some(&m), Some(&nuclei), &at(vec![Ok(cga::rotator(&q(3, 1), &q(4, 1)))]), &[], &mut stroke());
    assert_eq!(rosette.congruence, Congruence::Congruent, "the round is kept");
    assert_eq!(rosette.permutes, Permutes::Moves(vec![(0, 0)]), "and the constructors are not");
}

/// CG1b — THE ROUND REVERSED IS THE HELD ℤ₂, NEVER INCONGRUENT: a 180° turn
/// in `e₂₃` carries `e₁ ∧ e₂` to `−e₁ ∧ e₂`, and it permutes the
/// constructors (`e₁` fixed, `e₂` onto its own ray).
#[test]
fn a_reversed_round_reads_the_held_z2() {
    let (m, nuclei) = round();
    let turn_180 = Mv::blade(E2 | E3, q(1, 1));
    let cell = read_cell(1, Some(&m), Some(&nuclei), &at(vec![Ok(turn_180)]), &[], &mut stroke());
    assert_eq!(cell.modes[0].round, RoundKept::Reversed);
    assert_eq!(cell.congruence, Congruence::HeldZ2 { reversed: 1 });
    assert_eq!(cell.permutes, Permutes::All);
}

/// A ZERO MEET IS NO ROUND: five nuclei in a four-dimensional span wedge to
/// zero, which commutes with everything — degeneration, not invariance. The
/// Gram form reads the configuration in its place.
#[test]
fn a_zero_meet_is_no_round_and_the_gram_reads_in_its_place() {
    let nuclei: Vec<Mv> = [E1, E2, E3, cga::EP, E1].iter().enumerate().map(|(i, &m)| Mv::blade(m, q(i as i64 + 1, 1))).collect();
    let m = meet_of(&nuclei).expect("the nuclei wedge");
    assert!(m.is_zero(), "dependent nuclei: the meet vanishes");
    let turn = cga::rotator(&q(1, 1), &q(1, 1));
    let cell = read_cell(4, Some(&m), Some(&nuclei), &at(vec![Ok(turn)]), &[], &mut stroke());
    assert_eq!(cell.congruence, Congruence::NoRound(NoRound::ZeroMeet), "never Congruent: ZM1 names the zero meet");
    let g = cell.gram.expect("the Gram form reads");
    assert_eq!((g.rank, g.dependent()), (4, 1), "four directions, one dependence");
}

/// 78.26(5) — COHERENCE IS JOINT, AND AN INCOHERENCE IS HELD BY CONTENT: a
/// rosette leaves the cell open by its ring — the ring's coordinates, its
/// class, its one elliptic mode of no order — never a bare hole; a screw
/// holds it open by its parabolic mode, its turn beside it; a ring whose hue
/// did not read carries its residue and names why its class did not.
#[test]
fn an_incoherence_is_held_by_its_ring_class_and_modes() {
    let (m, nuclei) = round();
    let rosette = cga::rotator(&q(3, 1), &q(4, 1));
    let held = read_cell(1, Some(&m), Some(&nuclei), &at(vec![Ok(rosette.clone())]), &[], &mut stroke());
    assert_eq!(held.congruence, Congruence::Congruent);
    let Coherence::Open(rings) = &held.coherence else { panic!("a rosette never returns: {:?}", held.coherence) };
    let [ring] = rings.as_slice() else { panic!("one ring, none chosen: {rings:?}") };
    assert_eq!(ring.ring, CellCircuit::ring_of(&[(overlap(0), true)]), "the ring's own coordinates");
    assert!(ring.class.is_ok(), "its class read by the one producer: {:?}", ring.class);
    assert!(
        matches!(ring.modes.as_slice(), [ResidueMode { kind: BivectorKind::Elliptic, order: None, .. }]),
        "one elliptic mode of no order: {:?}",
        ring.modes
    );
    assert_eq!(held.refusals().unordered, 1, "the rosette, counted where it stands");
    let screw = cga::rotator(&q(1, 1), &q(1, 1)).geo(&translator(&q(0, 1), &q(0, 1), &q(1, 1)));
    let open = read_cell(1, Some(&m), Some(&nuclei), &at(vec![Ok(screw)]), &[], &mut stroke());
    let Coherence::Open(rings) = &open.coherence else { panic!("a screw never returns: {:?}", open.coherence) };
    assert!(
        rings[0].modes.iter().any(|r| r.kind == BivectorKind::Parabolic),
        "held open by its parabolic mode: {:?}",
        rings[0].modes
    );
    let unhued = vec![CellCircuit { ring: CellCircuit::ring_of(&[(overlap(0), true)]), holonomy: Ok(rosette), hue: None }];
    let cell = read_cell(1, Some(&m), Some(&nuclei), &unhued, &[], &mut stroke());
    let Coherence::Open(rings) = &cell.coherence else { panic!("still open: {:?}", cell.coherence) };
    assert_eq!(rings[0].class, Err(ClassUnread::HueUnread), "the hue unread, named on the class");
}

/// 78.26(5) — THE RINGS THROUGH A CELL, BY THEIR RESIDUE: the identity is
/// coherent; a 90° turn resonant (no kind: the alphabet spells it); a
/// turn by `(3, 4, 5)` a rosette, harmonic (D); a screw parabolic, harmonic,
/// its turn of finite order (C); a 90° turn across the round resonant and
/// holonomic (no kind); a circuit refused at the cone luminal (C); one unread
/// for another reason is on the refusals and not here.
#[test]
fn the_rings_through_a_cell_read_by_resonance_level_and_kind() {
    let (m, nuclei) = round();
    let identity = Mv::scalar(q(1, 1));
    let turn_90 = cga::rotator(&q(1, 1), &q(1, 1));
    let rosette = cga::rotator(&q(3, 1), &q(4, 1));
    let screw = cga::rotator(&q(1, 1), &q(1, 1)).geo(&translator(&q(0, 1), &q(0, 1), &q(1, 1)));
    let across = cga::rotator_on_blade(&q(1, 1), &q(1, 1), E2 | E3);
    let circuits = at(vec![
        Ok(identity),
        Ok(turn_90),
        Ok(rosette),
        Ok(screw),
        Ok(across),
        Err(CircuitUnread::Cone),
        Err(CircuitUnread::NotRigid),
    ]);
    let cell = read_cell(1, Some(&m), Some(&nuclei), &circuits, &[], &mut stroke());
    assert_eq!(
        cell.rings,
        Rings {
            coherent: 1,
            resonant: 2,
            rosette: 1,
            parabolic: 1,
            boost: 0,
            harmonic: 2,
            luminal: 1,
            holonomic: 1,
            d: 1,
            c: 2,
        },
        "each ring on each axis, none collapsed"
    );
}

/// NOTHING IS CLASSED OFF WHAT DID NOT READ: an unread circuit leaves the
/// cell unread by count; a missing round is named; a cell no circuit
/// crosses was asked nothing. CV1b — AND EACH SILENCE CARRIES ITS CONTENT:
/// the circuit unread by OR3's reason and the capacity refused as no circuit
/// read; the nuclei unread; the capacity of a cell no circuit crosses the
/// same.
#[test]
fn unread_circuits_a_missing_round_and_no_circuit_are_named() {
    let (m, nuclei) = round();
    let unread = read_cell(1, Some(&m), Some(&nuclei), &at(vec![Err(CircuitUnread::NotRigid)]), &[], &mut stroke());
    assert_eq!((unread.congruence, unread.coherence), (Congruence::Unread { unread: 1 }, Coherence::Unread));
    assert_eq!(unread.circuits_unread, vec![CircuitUnread::NotRigid], "the circuit is unread by OR3's reason");
    assert_eq!(unread.capacity, Err(CapacityRefused::NoneRead), "and no circuit read for the capacity");
    let turn = cga::rotator(&q(1, 1), &q(1, 1));
    let bare = read_cell(1, None, None, &at(vec![Ok(turn)]), &[], &mut stroke());
    assert_eq!(
        bare.congruence,
        Congruence::NoRound(NoRound::NucleiUnread),
        "no round because the nuclei did not read, not a zero meet"
    );
    assert_eq!(bare.gram, Err(NucleiUnread), "and no Gram form, by the same content");
    let none = read_cell(1, Some(&m), Some(&nuclei), &[], &[], &mut stroke());
    assert_eq!((none.congruence, none.capacity), (Congruence::NoCircuit, Err(CapacityRefused::NoneRead)));
}

/// 78.26(2) — A CELL'S CLOSED ALGEBRA CONTAINS WHAT ITS CIRCUITS GENERATE AND
/// NOT WHAT THEY DO NOT: a 90° turn's cell holds the square of its own
/// action, and not the action of a turn in another plane; a cell whose
/// capacity did not read contains nothing it can say.
#[test]
fn a_cells_algebra_contains_its_own_products_and_no_other_plane() {
    let (m, nuclei) = round();
    // Both turns in the one algebra the wider of them stands in.
    let across = cga::rotator_on_blade(&q(1, 1), &q(1, 1), E2 | E3);
    let turn = cga::rotator(&q(1, 1), &q(1, 1)).widened(across.n());
    let cell = read_cell(1, Some(&m), Some(&nuclei), &at(vec![Ok(turn.clone())]), &[], &mut stroke());
    let r = cga::action_of(&turn).expect("a versor acts");
    assert_eq!(algebra_contains(&cell, &r.after(&r)), Some(true), "the 180° turn is in the algebra");
    let other = cga::action_of(&across).expect("a versor acts");
    assert_eq!(algebra_contains(&cell, &other), Some(false), "a turn in another plane is not");
    // An action of another algebra's width is not compared: unread, never false.
    let narrow = cga::action_of(&cga::rotator(&q(1, 1), &q(1, 1))).expect("a versor acts");
    assert_eq!(algebra_contains(&cell, &narrow), None, "another width is not read against this algebra");
    let none = read_cell(1, Some(&m), Some(&nuclei), &[], &[], &mut stroke());
    assert_eq!(algebra_contains(&none, &r), None, "no algebra read, nothing contained");
}

/// CV1b — THE BUDGET SPENT IS NAMED AT ITS PRODUCT: a stroke of one read
/// spends it on the first product of the capacity's closure, and the
/// refusal says which product it was.
#[test]
fn a_spent_capacity_names_the_product_it_ran_out_at() {
    let (m, nuclei) = round();
    let turn = cga::rotator(&q(1, 1), &q(1, 1));
    let mut starved = crate::physics::Physics::default().stroke_n(0);
    let cell = read_cell(1, Some(&m), Some(&nuclei), &at(vec![Ok(turn)]), &[], &mut starved);
    assert_eq!(cell.capacity, Err(CapacityRefused::Spent { product: 1 }), "spent at the first product");
}

/// STD1 (PH1) — THE SPAN'S GRADE AT ANY SIZE: entries of 200 bits, far past
/// `i128`, join exactly. A vector and its double are one ray, so the blade
/// they span is grade 1 and the double lies in it (`v ∧ A = 0`); a third
/// vector off it raises the grade to 2, and the grade is the same whatever
/// order the vectors join in. Nothing is refused for its size.
#[test]
fn the_span_reads_its_grade_at_any_size_and_in_any_order() {
    use crate::core::span::{grade_of, Span};
    use num_bigint::BigInt;
    let big = BigInt::from(1) << 200usize;
    let a = vec![(0usize, big.clone()), (3, big.clone() + 1)];
    let twice = vec![(0usize, &big * 2), (3, (&big + 1) * 2)];
    let other = vec![(1usize, big.clone() * 3), (3, BigInt::from(7))];
    assert_eq!(grade_of(&[a.clone(), twice.clone()]), 1, "one ray and its double");
    assert!(Span::of(std::slice::from_ref(&a)).holds(&twice), "the double lies in the span: its outside part is empty");
    assert_eq!(grade_of(&[a.clone(), twice.clone(), other.clone()]), 2, "and a third off it raises the grade");
    assert_eq!(grade_of(&[other, twice, a]), 2, "in any order");
}

/// STD1 — THE GRADE AND THE FACTORISATION AT ANY SIZE: four vectors of
/// `i64::MAX` on the diagonal span grade 4 exactly, where an `i128`
/// elimination refused; a repeated vector adds nothing. A word's exponents
/// reduce through the Hermite form to a zero residue: a generator is a
/// combination of the generators.
#[test]
fn the_grade_and_the_factorisation_read_any_size() {
    use crate::core::span::{grade_of, vector_of};
    use num_traits::Zero;
    let m = i64::MAX;
    let rows = [[m, 1, 0, 0], [1, m, 1, 0], [0, 1, m, 1], [0, 0, 1, m]];
    assert_eq!(grade_of(&rows.iter().map(vector_of).collect::<Vec<_>>()), 4, "grade four, read exactly");
    let dependent = [rows[0], rows[1], rows[0], rows[3]];
    assert_eq!(grade_of(&dependent.iter().map(vector_of).collect::<Vec<_>>()), 3, "a repeated vector adds nothing");
    let (h, _) = crate::core::exponents::hnf_with_transform(&rows);
    let (_, residue) = crate::core::exponents::reduce_against(&h, &rows[2]);
    assert!(residue.iter().all(|x| x.is_zero()), "a generator factors through the generators");
}

/// 78.68 — THE LIE CLOSURE, EXACT: two turns on planes that share an axis,
/// `e₁₂` and `e₂₃`, do not commute, and their bracket is `e₁₃` up to scale,
/// so they close to so(3), rank 3. Two turns on disjoint planes, `e₁₂` and
/// `e₃₊`, commute and close to rank 2. A held algebra re-closed with a
/// generator it already spans gains nothing.
#[test]
fn the_holonomy_algebra_closes_under_the_commutator() {
    let one = BigRational::from_integer(1.into());
    let e12 = Mv::blade(E1 | E2, one.clone());
    let e23 = Mv::blade(E2 | E3, one.clone());
    let e3p = Mv::blade(E3 | cga::EP, one.clone());
    let so3 = lie_close(&[], &[e12.clone(), e23.clone()], &mut stroke()).expect("closes");
    assert_eq!(so3.len(), 3, "[e₁₂, e₂₃] ∝ e₁₃: so(3)");
    let torus = lie_close(&[], &[e12.clone(), e3p], &mut stroke()).expect("closes");
    assert_eq!(torus.len(), 2, "commuting planes add no bracket");
    let again = lie_close(&so3, &[e12.scale(&q(3, 1))], &mut stroke()).expect("closes");
    assert_eq!(again.len(), 3, "a generator already spanned adds nothing");
    let mut starved = crate::physics::Physics::default().stroke_n(0);
    assert_eq!(lie_close(&[], &[e12, e23], &mut starved), Err(LieRankRefused::Spent { bracket: 1 }), "the stroke is read per bracket");
}

/// 78.110 E2 — THE CIRCUITS CROSSING ONE CELL MEET, BOTH WAYS. Two read
/// circuits — a turn on `e₁e₂`, and one turning on `e₁e₂` and `e₂e₃` at
/// once — are each the orbit against the other as neighbour: two
/// encounters, each read, and the one met by the anticommuting part tilts.
/// Asked of the cell's circuits directly — never on its reading path.
#[test]
fn the_circuits_crossing_a_cell_meet_both_ways() {
    let one = Mv::scalar(q(1, 1));
    let turn = one.add(&Mv::blade(E1 | E2, q(1, 1)));
    let screwed = one.add(&Mv::blade(E1 | E2, q(1, 1))).add(&Mv::blade(E2 | E3, q(1, 1)));
    let before = encounter_tally();
    read_encounters(&[&turn, &screwed], &mut stroke());
    let after = encounter_tally();
    let at_name = |n: &str| ENCOUNTER.iter().position(|x| *x == n).expect("a named slot");
    let grew = |n: &str| after[at_name(n)] - before[at_name(n)];
    assert!(grew("encounters read") + grew("encounters unread") >= 2, "each against the other: {before:?} → {after:?}");
    assert!(grew("lift tilts") >= 1, "the anticommuting part tilts the orbit it meets");
}
