// LAW-COUPLING v1 (proposal 78.175 §5): the dipole parcel and its
// fluidity (F1–F2), the dipole coupling on a turning plane (F3) and the
// light ledger (F4), each fixture an exact input and its exact output.

use super::bodies::{q, units};
use crate::coordinate::{Plane, Square};
use crate::coupling::*;
use crate::passage::PassageRefusal;
use crate::Q;
use num_traits::{One, Signed, Zero};

/// τ₁₂, the turning between planes 1 and 2, `e₊₁e₊₂`: the fixtures' plane.
fn tau() -> crate::coordinate::Key {
    units((Square::Positive, 1), (Square::Positive, 2))
}

fn parcel(a: i64, o: i64) -> Parcel {
    Parcel::of(q(a, 1), q(o, 1)).expect("frames")
}

fn stated(pole: Q, open: Q) -> Dipole {
    Dipole::stated(pole, open).expect("a dipole")
}

/// A neighbour on τ₁₂ at the round separation `delta`, in degrees.
fn neighbour(w: Q, d: Dipole, n: Q, delta: Q) -> Neighbour {
    Neighbour::of(w, d, n, Alignment::on(tau(), delta).expect("an alignment")).expect("a neighbour")
}

fn field_of(range: &[Neighbour]) -> Field {
    field(&tau(), range).expect("a field")
}

/// F1–F2, THE PARCEL'S POLE AND OPEN SHARE: `A = 3, Q = 1` gives `σ = ½`,
/// `o = ¼`, `t = ⅓`; hydrogen (`Z = 1, U = 1`) holds `A = 2, Q = 1`,
/// `σ = ⅓`, `t = ½`, `o = ⅓`; the throat (`A = Q`) `σ = 0`, `o = ½`; the
/// open pole `σ = −1`, `o = 1`, no ratio; the closed pole `σ = +1`, `o = 0`.
#[test]
fn parcel_reads_pole_and_open_share() {
    let read = |p: &Parcel| {
        let d = p.dipole().expect("a dipole");
        (d.pole, d.open, p.ratio())
    };
    assert_eq!(read(&parcel(3, 1)), (q(1, 2), q(1, 4), Some(q(1, 3))));
    let hydrogen = Parcel::kind(&q(1, 1), &q(1, 1)).expect("hydrogen");
    assert_eq!((hydrogen.held(), hydrogen.open()), (&q(2, 1), &q(1, 1)));
    assert_eq!(read(&hydrogen), (q(1, 3), q(1, 3), Some(q(1, 2))));
    assert_eq!(read(&parcel(1, 1)), (q(0, 1), q(1, 2), Some(q(1, 1))));
    assert_eq!(read(&parcel(0, 1)), (q(-1, 1), q(1, 1), None));
    assert_eq!(read(&parcel(1, 0)), (q(1, 1), q(0, 1), Some(q(0, 1))));
}

/// F1, AN OPEN-HEAVY KIND NEEDS `U > 3Z/2`: `Z = 2, U = 4` holds `A = 2,
/// Q = 4`, `σ = −⅓`, `o = ⅔`; `Z = 2, U = 3` stands at the throat. Over
/// every kind `Z ∈ 1..=6`, `U ∈ 0..=3Z`, the pole is below zero exactly
/// where `2U > 3Z`, and `σ = (A − Q)/(A + Q)`.
#[test]
fn open_heavy_kind_needs_more_than_half() {
    let d = Parcel::kind(&q(2, 1), &q(4, 1)).and_then(|p| p.dipole()).expect("a kind");
    assert_eq!((d.pole, d.open), (q(-1, 3), q(2, 3)));
    assert_eq!(Parcel::kind(&q(2, 1), &q(3, 1)).and_then(|p| p.dipole()).map(|d| d.pole), Ok(q(0, 1)));
    for z in 1..=6i64 {
        for u in 0..=3 * z {
            let p = Parcel::kind(&q(z, 1), &q(u, 1)).expect("a kind");
            let d = p.dipole().expect("a dipole");
            assert_eq!(d.pole.is_negative(), 2 * u > 3 * z, "Z = {z}, U = {u}");
            assert_eq!(d.pole, (p.held() - p.open()) / (p.held() + p.open()));
        }
    }
}

/// F2, THE FLUIDITY AND ITS INVERSION: `ν(⅓) = ¼`, `ν(3) = ¾`, `ν(1) = ½`;
/// `ν(1/t) = 1 − ν(t)` for every `t` tried, so the inversion exchanges the
/// fluidity and the viscosity (the held share); the frames exchanged give
/// `o ↦ 1 − o` and `σ ↦ −σ`, and `o = ν(Q/A)`.
#[test]
fn fluidity_inverts() {
    assert_eq!(open_share(&q(1, 3)), Ok(q(1, 4)));
    assert_eq!(open_share(&q(3, 1)), Ok(q(3, 4)));
    assert_eq!(open_share(&q(1, 1)), Ok(q(1, 2)));
    for t in [q(1, 7), q(2, 5), q(1, 1), q(9, 4), q(1000, 3)] {
        assert_eq!(open_share(&(Q::one() / &t)).expect("a share"), Q::one() - open_share(&t).expect("a share"));
    }
    for (a, o) in [(3, 1), (2, 1), (1, 1), (5, 7), (0, 1)] {
        let (p, d) = (parcel(a, o), parcel(a, o).dipole().expect("a dipole"));
        let inv = p.inverted().dipole().expect("a dipole");
        assert_eq!((inv.open, inv.pole), (Q::one() - &d.open, -d.pole.clone()));
        if let Some(t) = p.ratio() {
            assert_eq!(open_share(&t), Ok(d.open));
        }
    }
}

/// The turnings about axes 1, 2 and 3: `e₊₂e₊₃`, `e₊₃e₊₁`, `e₊₁e₊₂`.
fn three() -> [crate::coordinate::Key; 3] {
    [units((Square::Positive, 2), (Square::Positive, 3)), units((Square::Positive, 3), (Square::Positive, 1)), tau()]
}

/// F1 PER PLANE, THE EVEN SPLIT: hydrogen founded on three turning planes
/// holds `(2/3, 1/3)` on each, every plane reading `σ = ⅓`, `o = ⅓`; the
/// kind `Z = 4/3, U = 1` (frames `(3, 1)`) holds `(1, 1/3)`, each plane
/// `σ = ½`, `o = ¼`. A BOND ON ONE PLANE: hydrogen's open third on `e₊₁e₊₂`
/// closing gives that plane `(1, 0)`, `σ = 1`, `o = 0`, the other two
/// unchanged; the peel of the same third returns it.
#[test]
fn planes_split_evenly_and_bond_one_at_a_time() {
    let read = |b: &Parcels| -> Vec<(Q, Q)> { b.planes().iter().map(|(_, p)| p.dipole().map(|d| (d.pole, d.open)).expect("a dipole")).collect() };
    let hydrogen = Parcels::founded(&q(1, 1), &q(1, 1), &three()).expect("founded");
    assert!(hydrogen.planes().iter().all(|(_, p)| (p.held(), p.open()) == (&q(2, 3), &q(1, 3))));
    assert_eq!(read(&hydrogen), vec![(q(1, 3), q(1, 3)); 3]);
    let kind = Parcels::founded(&q(4, 3), &q(1, 1), &three()).expect("founded");
    assert_eq!(kind.on(&tau()).map(|p| (p.held().clone(), p.open().clone())), Some((q(1, 1), q(1, 3))));
    assert_eq!(read(&kind), vec![(q(1, 2), q(1, 4)); 3]);
    let bonded = hydrogen.bond(&tau(), &q(1, 3)).expect("a bond");
    assert_eq!(read(&bonded), vec![(q(1, 3), q(1, 3)), (q(1, 3), q(1, 3)), (q(1, 1), q(0, 1))]);
    assert_eq!(bonded.bond(&tau(), &q(-1, 3)), Ok(hydrogen.clone()));
    assert_eq!(hydrogen.bond(&tau(), &q(1, 2)), Err(CouplingRefusal::NotFrames { held: Box::new(q(7, 6)), open: Box::new(q(-1, 6)) }));
    let off = units((Square::Positive, 1), (Square::Positive, 4));
    assert_eq!(hydrogen.bond(&off, &q(1, 3)), Err(CouplingRefusal::NoPlane(off)));
    assert_eq!(Parcels::founded(&q(1, 1), &q(1, 1), &[]), Err(CouplingRefusal::NoPlanes));
    assert_eq!(Parcels::founded(&q(1, 1), &q(1, 1), &[tau(), tau()]), Err(CouplingRefusal::PlaneRepeated(tau())));
    let frame = units((Square::Positive, 1), (Square::Negative, 1));
    assert_eq!(Parcels::founded(&q(1, 1), &q(1, 1), std::slice::from_ref(&frame)),Err(CouplingRefusal::NotTurningPlane { key: frame, plane: Some(Plane::Frame) }));
}

/// THE NAMED L1 READINGS: the hemisphere's reading of a round separation,
/// `hemisphere_l1(0°, 45°, 90°, 135°, 180°) = 1, ½, 0, −½, −1`, refused
/// past `[0°, 180°]`; the pole's L1 arc `θ_L1 = 180°·q/(z + q)`, 0 at the
/// closed pole, 60 for hydrogen, 45 for `(3, 1)`, 90 at the throat, 180 at
/// the open pole.
#[test]
fn l1_readings_are_named() {
    let read = |d: i64| hemisphere_l1(&q(d, 1));
    assert_eq!([0, 45, 90, 135, 180].map(read), [Ok(q(1, 1)), Ok(q(1, 2)), Ok(q(0, 1)), Ok(q(-1, 2)), Ok(q(-1, 1))]);
    assert_eq!(hemisphere_l1(&q(-1, 1)), Err(CouplingRefusal::NotSeparation(Box::new(q(-1, 1)))));
    let arcs: Vec<Q> = [(1, 0), (2, 1), (3, 1), (1, 1), (0, 1)].iter().map(|(a, o)| parcel(*a, *o).arc_l1().expect("an arc")).collect();
    assert_eq!(arcs, [q(0, 1), q(60, 1), q(45, 1), q(90, 1), q(180, 1)]);
}

/// F1–F2 REFUSED BY NAME: frames below zero or none, a kind past `U = 3Z`,
/// a negative ratio, a stated dipole out of its bounds.
#[test]
fn parcel_refusals_are_named() {
    let frames = |a: Q, o: Q| Err(CouplingRefusal::NotFrames { held: Box::new(a), open: Box::new(o) });
    assert_eq!(Parcel::of(q(-1, 1), q(1, 1)), frames(q(-1, 1), q(1, 1)));
    assert_eq!(Parcel::of(q(0, 1), q(0, 1)), frames(q(0, 1), q(0, 1)));
    assert_eq!(Parcel::kind(&q(1, 1), &q(4, 1)), frames(q(-1, 1), q(4, 1)));
    assert_eq!(open_share(&q(-1, 2)), Err(CouplingRefusal::NotRatio(Box::new(q(-1, 2)))));
    assert_eq!(Dipole::stated(q(2, 1), q(1, 2)), Err(CouplingRefusal::NotDipole { pole: Box::new(q(2, 1)), open: Box::new(q(1, 2)) }));
    assert!(matches!(Dipole::stated(q(0, 1), q(3, 2)), Err(CouplingRefusal::NotDipole { .. })));
}

/// Hydrogen's dipole on its frames, `Z = 1, U = 1`: `(2, 1)`, `σ = ⅓`,
/// `o = ⅓`.
fn hydrogen() -> Dipole {
    Parcel::kind(&q(1, 1), &q(1, 1)).and_then(|p| p.dipole()).expect("hydrogen")
}

/// The open-heavy kind `Z = 2, U = 4` on its frames: `(2, 4)`, `σ = −⅓`,
/// `o = ⅔`.
fn open_heavy() -> Parcel {
    Parcel::kind(&q(2, 1), &q(4, 1)).expect("an open-heavy kind")
}

/// F3 ON FRAMES, OPPOSED POLES LOCK: `x = ⅕`, `x̄ = ½`, the body hydrogen
/// (`σ = ⅓`, `o = ⅓`); one neighbour the open-heavy kind `(2, 4)` (`σ = −⅓`,
/// `o = ⅔`), `w = 1`, `n = ⅔`, `Δ = 0°` (`h = 1`): `F = −4/27`,
/// `x̄ ⊖ x = ⅓`, `y = 4/729`, `x′ = (⅕ + 4/729)/(1 + 4/3645) = 749/3649`,
/// and `⅕ < 749/3649 < ½`.
#[test]
fn opposed_poles_lock_on_frames() {
    let f = field_of(&[neighbour(q(1, 1), open_heavy().dipole().expect("a dipole"), q(2, 3), q(0, 1))]);
    assert_eq!(f, Field::Held { holding: q(1, 1), field: q(-4, 27) });
    let p = pull(&q(1, 5), &q(1, 2), &hydrogen(), &f).expect("a pull");
    assert_eq!(p, Pull { gap: q(1, 3), push: q(4, 729), rate: q(749, 3649) });
    assert!(q(1, 5) < p.rate && p.rate < q(1, 2));
}

/// F3 ON FRAMES, THE POLES REVERSED SPLIT: the neighbour's pair of frames
/// with its roles swapped, `(2, 4) ↦ (4, 2)` (`σ = ⅓`, `o = ⅓`), against the
/// hydrogen body: `F = 2/27`, `y = −2/729`,
/// `x′ = (⅕ − 2/729)/(1 − 2/3645) = 719/3643`, below ⅕.
#[test]
fn reversed_poles_split_on_frames() {
    let reversed = open_heavy().inverted();
    assert_eq!((reversed.held(), reversed.open()), (&q(4, 1), &q(2, 1)));
    let f = field_of(&[neighbour(q(1, 1), reversed.dipole().expect("a dipole"), q(2, 3), q(0, 1))]);
    assert_eq!(f.value(), q(2, 27));
    let p = pull(&q(1, 5), &q(1, 2), &hydrogen(), &f).expect("a pull");
    assert_eq!(p, Pull { gap: q(1, 3), push: q(-2, 729), rate: q(719, 3643) });
    assert!(p.rate < q(1, 5));
}

/// F3 ON FRAMES, A BODY ALONE IS UNTOUCHED, and a neighbour at the throat
/// (frames `(1, 1)`, `σ = 0`) contributes nothing: the hydrogen body keeps
/// `x′ = x = ⅕` both.
#[test]
fn alone_or_throat_keeps_rate() {
    let alone = field_of(&[]);
    assert_eq!(alone, Field::Alone);
    assert_eq!(pull(&q(1, 5), &q(1, 2), &hydrogen(), &alone).map(|p| (p.push, p.rate)), Ok((q(0, 1), q(1, 5))));
    let throat = parcel(1, 1).dipole().expect("the throat");
    let f = field_of(&[neighbour(q(1, 1), throat, q(2, 3), q(0, 1))]);
    assert_eq!(f, Field::Held { holding: q(1, 1), field: q(0, 1) });
    assert_eq!(pull(&q(1, 5), &q(1, 2), &hydrogen(), &f).map(|p| (p.push, p.rate)), Ok((q(0, 1), q(1, 5))));
}

/// F3 ON FRAMES, A HALF ALIGNMENT: the lock's pair at `Δ = 45°` (`h = ½`):
/// `F = −2/27`, `y = 2/729`, `x′ = (⅕ + 2/729)/(1 + 2/3645) = 739/3647`.
/// Two neighbours weigh by holding: the open-heavy kind at `w = 1` and
/// hydrogen at `w = 3`, `F = (−4/27 + 3·2/27)/4 = 1/54`.
#[test]
fn field_weighs_alignment_and_holding() {
    let half = field_of(&[neighbour(q(1, 1), open_heavy().dipole().expect("a dipole"), q(2, 3), q(45, 1))]);
    assert_eq!(half.value(), q(-2, 27));
    assert_eq!(pull(&q(1, 5), &q(1, 2), &hydrogen(), &half), Ok(Pull { gap: q(1, 3), push: q(2, 729), rate: q(739, 3647) }));
    let two = field_of(&[
        neighbour(q(1, 1), open_heavy().dipole().expect("a dipole"), q(2, 3), q(0, 1)),
        neighbour(q(3, 1), hydrogen(), q(2, 3), q(0, 1)),
    ]);
    assert_eq!(two, Field::Held { holding: q(4, 1), field: q(1, 54) });
}

/// THE PURE Z POLE HAS NO MOMENT: frames `(1, 0)`, `σ = 1`, `o = 0`. As a
/// neighbour its term `w·σ·o·n·h` is 0 (it does not couple): alone in the
/// range `F = 0`, and beside the open-heavy kind it adds its holding and no
/// term, `F = −4/27 / 2 = −2/27`. As the body it is not moved by any field
/// (it is not coupled): against the open-heavy kind (`F = −4/27`) its push
/// `−σ·F·o·(x̄ ⊖ x)` is 0 and `x′ = x = ⅕`; so for every field in `[−1, 1]`.
#[test]
fn pure_z_pole_neither_couples_nor_is_coupled() {
    let pole = parcel(1, 0).dipole().expect("the pure Z pole");
    assert_eq!((pole.pole.clone(), pole.open.clone()), (q(1, 1), q(0, 1)));
    let alone = field_of(&[neighbour(q(1, 1), pole.clone(), q(2, 3), q(0, 1))]);
    assert_eq!(alone, Field::Held { holding: q(1, 1), field: q(0, 1) });
    assert_eq!(pull(&q(1, 5), &q(1, 2), &hydrogen(), &alone).map(|p| (p.push, p.rate)), Ok((q(0, 1), q(1, 5))));
    let beside = field_of(&[neighbour(q(1, 1), pole.clone(), q(2, 3), q(0, 1)), neighbour(q(1, 1), open_heavy().dipole().expect("a dipole"), q(2, 3), q(0, 1))]);
    assert_eq!(beside, Field::Held { holding: q(2, 1), field: q(-2, 27) });
    let f = field_of(&[neighbour(q(1, 1), open_heavy().dipole().expect("a dipole"), q(2, 3), q(0, 1))]);
    assert_eq!(pull(&q(1, 5), &q(1, 2), &pole, &f), Ok(Pull { gap: q(1, 3), push: q(0, 1), rate: q(1, 5) }));
    for value in [q(-1, 1), q(-1, 3), q(1, 2), q(1, 1)] {
        let any = Field::Held { holding: q(1, 1), field: value };
        assert_eq!(pull(&q(-99, 100), &q(99, 100), &pole, &any).map(|p| p.rate), Ok(q(-99, 100)));
    }
}

/// ALGEBRA CHECK OF `pull`, on stated dipoles no frames hold (on frames
/// `σ = 1 − 2o`, so `σ = ±1` with `o = ½` is no body); not served as a
/// fixture. The body `σ = +1, o = ½`; a neighbour `w = 1, σ = −1, o = ½,
/// n = ⅔`, `Δ = 0°`: `F = −⅓`, `y = 1/18`, `x′ = 23/91`; its pole
/// reversed, `F = ⅓`, `y = −1/18`, `x′ = 13/89`; at `Δ = 45°`, `F = −⅙`,
/// `y = 1/36`, `x′ = 41/181`; two of holdings 1 and 3, poles `−1` and `+1`,
/// `F = ⅙`.
#[test]
fn algebra_check_of_pull() {
    let body = stated(q(1, 1), q(1, 2));
    let one = |pole: i64, delta: i64| field_of(&[neighbour(q(1, 1), stated(q(pole, 1), q(1, 2)), q(2, 3), q(delta, 1))]);
    let lock = one(-1, 0);
    assert_eq!(lock, Field::Held { holding: q(1, 1), field: q(-1, 3) });
    assert_eq!(pull(&q(1, 5), &q(1, 2), &body, &lock), Ok(Pull { gap: q(1, 3), push: q(1, 18), rate: q(23, 91) }));
    let split = one(1, 0);
    assert_eq!(split.value(), q(1, 3));
    assert_eq!(pull(&q(1, 5), &q(1, 2), &body, &split), Ok(Pull { gap: q(1, 3), push: q(-1, 18), rate: q(13, 89) }));
    let half = one(-1, 45);
    assert_eq!(half.value(), q(-1, 6));
    assert_eq!(pull(&q(1, 5), &q(1, 2), &body, &half), Ok(Pull { gap: q(1, 3), push: q(1, 36), rate: q(41, 181) }));
    let two = field_of(&[
        neighbour(q(1, 1), stated(q(-1, 1), q(1, 2)), q(2, 3), q(0, 1)),
        neighbour(q(3, 1), stated(q(1, 1), q(1, 2)), q(2, 3), q(0, 1)),
    ]);
    assert_eq!(two, Field::Held { holding: q(4, 1), field: q(1, 6) });
}

/// F3 ON THE FRAMES: the body `A = 3, Q = 1` (`σ = ½`, `o = ¼`) and a
/// hydrogen neighbour (`σ = ⅓`, `o = ⅓`, `w = 1`, `n = ⅔`, `Δ = 0°`):
/// `F = 2/27`, `y = −1/324`, `x′ = (⅕ − 1/324)/(1 − 1/1620) = 319/1619`.
#[test]
fn frames_read_pull() {
    let body = parcel(3, 1).dipole().expect("a dipole");
    let f = field_of(&[neighbour(q(1, 1), hydrogen(), q(2, 3), q(0, 1))]);
    assert_eq!(f.value(), q(2, 27));
    assert_eq!(pull(&q(1, 5), &q(1, 2), &body, &f), Ok(Pull { gap: q(1, 3), push: q(-1, 324), rate: q(319, 1619) }));
}

/// F3 NEAR THE BOUND: the open pole (`σ = −1`, `o = 1`) at `x = 99/100`
/// against `x̄ = −99/100`, one open-pole neighbour at `n = 1`, `Δ = 0°`:
/// `F = −1`, `x̄ ⊖ x = −19800/19801`, `y = 19800/19801`, and
/// `x′ = 3940299/3940300`, inside the bound.
#[test]
fn near_bound_stays_inside() {
    let open = parcel(0, 1).dipole().expect("the open pole");
    let f = field_of(&[neighbour(q(1, 1), open.clone(), q(1, 1), q(0, 1))]);
    assert_eq!(f.value(), q(-1, 1));
    let p = pull(&q(99, 100), &q(-99, 100), &open, &f).expect("a pull");
    assert_eq!(p, Pull { gap: q(-19800, 19801), push: q(19800, 19801), rate: q(3_940_299, 3_940_300) });
    assert!(p.rate < Q::one());
}

/// F3, `|F| ≤ 1` AND THE BOUND HOLDS BY CONSTRUCTION: over ranges of the
/// extreme and middle dipoles, nearnesses, alignments and holdings, the
/// field stands in `[−1, 1]`, and every pull from rates near either bound
/// toward rates near either bound stays inside `(−1, 1)`.
#[test]
fn field_and_pull_stay_bounded() {
    let poles = [q(-1, 1), q(-1, 2), q(0, 1), q(1, 3), q(1, 1)];
    let separations = [q(180, 1), q(135, 1), q(90, 1), q(60, 1), q(0, 1)];
    let shares = [q(0, 1), q(1, 2), q(1, 1)];
    let rates = [q(-999, 1000), q(-1, 2), q(0, 1), q(99, 100), q(999, 1000)];
    for (i, sigma) in poles.iter().enumerate() {
        for o in &shares {
            for n in &shares {
                let range: Vec<Neighbour> = separations
                    .iter()
                    .enumerate()
                    .map(|(j, d)| neighbour(Q::from_integer((1 + j).into()), stated(poles[(i + j) % poles.len()].clone(), o.clone()), n.clone(), d.clone()))
                    .collect();
                let f = field_of(&range);
                assert!(f.value().abs() <= Q::one());
                for x in &rates {
                    for x_bar in &rates {
                        let p = pull(x, x_bar, &stated(sigma.clone(), o.clone()), &f).expect("a pull");
                        assert!(p.push.abs() < Q::one() && p.rate.abs() < Q::one(), "{x} toward {x_bar}");
                    }
                }
            }
        }
    }
}

/// F3 REFUSED BY NAME: a rate at the bound, a holding of zero or below,
/// an alignment off a turning plane or at a separation past `[0°, 180°]`,
/// a neighbour on
/// another plane, a nearness past `[0, 1]`.
#[test]
fn coupling_refusals_are_named() {
    let body = stated(q(1, 1), q(1, 2));
    assert_eq!(pull(&q(1, 1), &q(1, 2), &body, &Field::Alone), Err(CouplingRefusal::NotShare(Box::new(q(1, 1)))));
    assert_eq!(pull(&q(1, 5), &q(-3, 2), &body, &Field::Alone), Err(CouplingRefusal::NotShare(Box::new(q(-3, 2)))));
    let along = Alignment::on(tau(), q(1, 1)).expect("an alignment");
    for w in [q(0, 1), q(-1, 1)] {
        assert_eq!(Neighbour::of(w.clone(), body.clone(), q(1, 2), along.clone()), Err(CouplingRefusal::NoHolding(Box::new(w))));
    }
    assert_eq!(Neighbour::of(q(1, 1), body.clone(), q(3, 2), along), Err(CouplingRefusal::NotNearness(Box::new(q(3, 2)))));
    let frame = units((Square::Positive, 1), (Square::Negative, 1));
    assert_eq!(Alignment::on(frame.clone(), q(1, 1)), Err(CouplingRefusal::NotTurningPlane { key: frame.clone(), plane: Some(Plane::Frame) }));
    assert_eq!(field(&frame, &[]), Err(CouplingRefusal::NotTurningPlane { key: frame, plane: Some(Plane::Frame) }));
    for delta in [q(-1, 2), q(181, 1)] {
        assert_eq!(Alignment::on(tau(), delta.clone()), Err(CouplingRefusal::NotSeparation(Box::new(delta))));
    }
    let other = units((Square::Positive, 2), (Square::Positive, 3));
    let off = Neighbour::of(q(1, 1), body, q(1, 2), Alignment::on(other.clone(), q(0, 1)).expect("an alignment")).expect("a neighbour");
    assert_eq!(field(&tau(), &[off]), Err(CouplingRefusal::PlanesDiffer { field: tau(), neighbour: other }));
}

/// F3, THE RING LAW'S NEARNESS AND THE NEIGHBOURS: `n(M = 2, g = −1) = ⅔`.
/// Enclosures meet where `|R_i − R_j| ≤ R_i·e_i + R_j·e_j`,
/// `e = m/(m + H(<R))`: `(R, m, H) = (1, 1, 1)` encloses `½`; against
/// `(2, 1, 3)` (`¼`, enclosing `½`) the two touch at the bound (separation
/// 1, reach 1); against `(2, 1, 4)` (enclosing `2/5`) they stand apart
/// (reach `9/10`); against `(2, 1, 1)` (enclosing 1) they overlap (reach
/// `3/2`).
#[test]
fn neighbours_meet_by_enclosure() {
    assert_eq!(nearness(&q(2, 1), &q(-1, 1)), Ok(q(2, 3)));
    assert_eq!(nearness(&q(0, 1), &q(0, 1)), Err(CouplingRefusal::NoWhole { part: Box::new(q(0, 1)), rest: Box::new(q(0, 1)) }));
    assert_eq!(nearness(&q(-1, 1), &q(1, 1)), Err(CouplingRefusal::Negative(Box::new(q(-1, 1)))));
    let at = |r: i64, m: i64, h: i64| Enclosure { reach: q(r, 1), moment: q(m, 1), hold: q(h, 1) };
    let a = at(1, 1, 1);
    assert_eq!(a.enclosed(), Ok(q(1, 2)));
    assert_eq!(meets(&a, &at(2, 1, 3)), Ok(Meeting { separation: q(1, 1), reach: q(1, 1), meets: Meets::Bound }));
    assert_eq!(meets(&a, &at(2, 1, 4)), Ok(Meeting { separation: q(1, 1), reach: q(9, 10), meets: Meets::Apart }));
    assert_eq!(meets(&a, &at(2, 1, 1)), Ok(Meeting { separation: q(1, 1), reach: q(3, 2), meets: Meets::Within }));
    assert_eq!(meets(&a, &at(2, 0, 0)), Err(CouplingRefusal::NoWhole { part: Box::new(q(0, 1)), rest: Box::new(q(0, 1)) }));
    assert_eq!(meets(&at(-1, 1, 1), &a), Err(CouplingRefusal::Negative(Box::new(q(-1, 1)))));
}

/// F4, ONE BELL ON EACH FACE: a body of hold 1 at reach 3 (`δ = ¼`) takes
/// light 8. On the continuum (unlocked `¾`) it receives 6 and 2 drain; on
/// the vacuum (unlocked `¼`) it receives 2 and 6 drain; each ledger closes,
/// 8 = 6 + 2 and 8 = 2 + 6. At the throat (`δ = ½`) both faces take half.
#[test]
fn light_ledger_closes_each_face() {
    assert_eq!(unlocked(Face::Continuum, &q(1, 1), &q(3, 1)), Ok(q(3, 4)));
    assert_eq!(unlocked(Face::Vacuum, &q(1, 1), &q(3, 1)), Ok(q(1, 4)));
    let on = |face: Face| take(&q(8, 1), face, &q(1, 1), &q(3, 1)).expect("light taken");
    let (continuum, vacuum) = (on(Face::Continuum), on(Face::Vacuum));
    assert_eq!(continuum, Taken { received: q(6, 1), drained: q(2, 1) });
    assert_eq!(vacuum, Taken { received: q(2, 1), drained: q(6, 1) });
    for t in [continuum, vacuum] {
        assert_eq!(ledger(&q(8, 1), &t.received, &t.drained), Ok(Ledger { made: q(8, 1), received: t.received.clone(), drained: t.drained.clone() }));
    }
    for face in [Face::Continuum, Face::Vacuum] {
        assert_eq!(unlocked(face, &q(5, 2), &q(5, 2)), Ok(q(1, 2)));
    }
}

/// F4 REFUSED BY NAME: a ledger of made 8, received 6, drained 1 does not
/// balance, residue 1; light below zero; a face of no whole.
#[test]
fn light_ledger_refusals_are_named() {
    assert_eq!(
        ledger(&q(8, 1), &q(6, 1), &q(1, 1)),
        Err(CouplingRefusal::Unbalanced { made: Box::new(q(8, 1)), received: Box::new(q(6, 1)), drained: Box::new(q(1, 1)), residue: Box::new(q(1, 1)) })
    );
    assert_eq!(take(&q(-1, 1), Face::Vacuum, &q(1, 1), &q(3, 1)), Err(CouplingRefusal::Negative(Box::new(q(-1, 1)))));
    assert_eq!(unlocked(Face::Continuum, &q(0, 1), &q(0, 1)), Err(CouplingRefusal::Passage(Box::new(PassageRefusal::NoWhole { span: Q::zero() }))));
}
