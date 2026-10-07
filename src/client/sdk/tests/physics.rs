// 78.131 — UNIVERSAL PHYSICS: the laws' forms, proved at a universe's own
// reference (every ratio one) and on the Earth/Sun cases, the Earth's and
// Sun's values passed in by the test — the SDK holds none.

use crate::client::sdk::certified::Encl;
use crate::client::sdk::physics::*;
use num_bigint::BigInt;
use num_rational::BigRational as Q;
use num_traits::{One, Signed, Zero};

const W: u32 = 64;

fn q(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}

fn tight(e: &Encl) -> bool {
    e.width() <= Q::new(BigInt::one(), BigInt::one() << 56u32)
}

/// Water, as our universe's: boiling 373.15 K at 1 atm, L/R ≈ 4890 K,
/// triple 273.16 K at 611.657 Pa, critical 647.096 K at 22.064 MPa.
fn water() -> Substance {
    Substance {
        tb: q(37315, 100),
        p_ref: q(101_325, 1),
        l_over_r: q(4890, 1),
        t_triple: q(27316, 100),
        p_triple: q(611_657, 1000),
        t_crit: q(647_096, 1000),
        p_crit: q(22_064_000, 1),
    }
}

#[test]
fn every_scale_is_one_at_the_universes_own_reference() {
    let laws = Laws::reference();
    for (name, e) in [
        ("chemistry", laws.chemistry(W)),
        ("atom", laws.atom(W)),
        ("density", laws.density(W)),
        ("pressure", laws.pressure(W)),
        ("proton", laws.proton(W)),
        ("star_mass", laws.star_mass(W)),
        ("star_light", laws.star_light(W)),
        ("star_life", laws.star_life(W)),
        ("star_radius", laws.star_radius(W)),
        ("star_min", laws.star_min(W)),
        ("jeans", laws.jeans(W)),
        ("star_teff", laws.star_teff(W)),
        ("coincidence", laws.coincidence(W)),
    ] {
        let e = e.expect(name);
        assert!(e.contains(&Q::one()) && tight(&e), "{name} at the reference: {e:?}");
    }
    assert_eq!(laws.half_life(&q(7, 1), W), Encl::exact(Q::one()), "exp(0) = 1");
    // A universe with twice the fine structure: chemistry scales as α².
    let doubled = Laws { alpha: q(2, 1), ..Laws::reference() };
    assert!(doubled.chemistry(W).expect("").contains(&q(4, 1)));
    assert!(doubled.star_radius(W).expect("").contains(&q(1, 4)), "radius ∝ 1/α²");
}

#[test]
fn orbits_are_closed_fibres_with_exact_points() {
    assert_eq!(kepler_period(&Q::one(), &Q::one()), Some(Q::one()), "one year at q = m = 1");
    // a = 2 (q² = 2 has no rational q; q = 2 is a = 4): T = 8 at M = 1.
    assert_eq!(kepler_period(&q(2, 1), &Q::one()), Some(q(8, 1)));
    assert_eq!(received_flux(&Q::one(), &q(2, 1)), Some(q(1, 16)), "L/q⁴");
    assert_eq!(orbit_phase(&q(1, 4), 3, &q(4, 1)), Some(Q::zero()), "a closed fibre returns");
    assert_eq!(orbit_phase(&Q::zero(), 6, &q(4, 1)), Some(q(1, 2)));
    // Every rotor point is exactly on the circle; the orbit closes.
    for t in [q(0, 1), q(1, 3), q(-7, 2), q(5, 1)] {
        let (x, y) = circle_point(&t);
        assert_eq!(&x * &x + &y * &y, Q::one(), "on the circle at t = {t}");
    }
    // Uniform in angle: the quarter points land exactly, the eighth on the
    // diagonal, each within its stated bound of 2π·phase.
    let at = |n, d| {
        let p = orbit_point(&q(n, d), W).expect("");
        assert_eq!(&p.x * &p.x + &p.y * &p.y, Q::one(), "on the circle at {n}/{d}");
        assert!(p.angle_error < q(1, 1 << 60), "the bound is tight: {}", p.angle_error);
        p
    };
    for (n, d, x, y) in [(0, 1, 1, 0), (1, 4, 0, 1), (1, 2, -1, 0), (3, 4, 0, -1)] {
        let p = at(n, d);
        assert_eq!((p.x, p.y), (q(x, 1), q(y, 1)), "phase {n}/{d}");
    }
    for (n, d, sx, sy) in [(1, 8, 1, 1), (3, 8, -1, 1), (5, 8, -1, -1), (7, 8, 1, -1)] {
        let p = at(n, d);
        assert!((&p.x - q(sx, 1) * &p.y * q(sy, 1)).abs() < q(1, 1 << 58), "phase {n}/{d} on its diagonal");
        assert!(p.x * q(sx, 1) > Q::zero() && p.y * q(sy, 1) > Q::zero(), "phase {n}/{d}'s quadrant");
    }
    // 30°: (√3/2, ½).
    let p = at(1, 12);
    assert!((p.y - q(1, 2)).abs() < q(1, 1 << 58), "sin 30° = ½");
    assert_eq!(orbit_point(&Q::one(), W), None, "a phase is in [0, 1)");
    // Nesting: a moon at phase 0 about a planet at phase ½ about a core.
    assert_eq!(nested_position(&[(q(10, 1), q(1, 2)), (Q::one(), Q::zero())], W), Some((q(-9, 1), Q::zero())));
    assert!(spaced(&q(7, 1), &q(8, 1)) && !spaced(&q(7, 1), &q(79, 10)), "8/7 apart");
    assert_eq!(escape_speed_squared(&Q::one(), &q(2, 1)), Some(Q::one()));
    assert!(hill_radius(&Q::one(), &q(3, 1), &q(1, 1), W).expect("").contains(&Q::one()), "(3/3)^(1/3) = 1");
    let body = TidalBody { m: Q::one(), r: Q::one(), c: q(1, 3), quality: q(3, 1), k2: Q::one() };
    assert_eq!(tidal_lock_time(&body, &Q::one(), &Q::one(), &Q::one()), Some(q(1, 3)), "ω₀·c·Q/3");
    assert_eq!(tidal_lock_time(&body, &Q::one(), &q(2, 1), &Q::one()), Some(q(64, 3)), "a⁶");
    let body = TidalBody { quality: q(63, 4), ..body };
    assert_eq!(circularisation_time(&body, &Q::one(), &Q::one(), &Q::one()), Some(Q::one()));
}

/// The Sun's calibration, as our universe's (passed in by the test): the
/// piecewise mass–luminosity and mass–radius laws, 10 Gyr, 5772 K, the
/// hydrogen-burning limit 0.08, the habitable zone's fluxes 1.1 and 0.36.
fn sun() -> StellarCalibration {
    let seg = |upto: (i64, i64), coeff: (i64, i64), exponent: (i64, i64)| PowerSegment {
        upto: q(upto.0, upto.1),
        coeff: q(coeff.0, coeff.1),
        exponent: q(exponent.0, exponent.1),
    };
    StellarCalibration {
        luminosity: vec![
            seg((43, 100), (23, 100), (23, 10)),
            seg((2, 1), (1, 1), (4, 1)),
            seg((55, 1), (14, 10), (7, 2)),
            seg((150, 1), (32_000, 1), (1, 1)),
        ],
        radius: vec![seg((1, 1), (1, 1), (4, 5)), seg((150, 1), (1, 1), (57, 100))],
        life: q(10, 1),
        teff: q(5772, 1),
        min_mass: q(8, 100),
        hz_inner: q(11, 10),
        hz_outer: q(36, 100),
    }
}

#[test]
fn stars_read_their_universes_calibration() {
    let cal = sun();
    let one = Q::one();
    assert_eq!(luminosity(&cal, &one, W), Some(Encl::exact(one.clone())), "the unit star");
    assert!(stellar_radius(&cal, &one, W).expect("").contains(&one));
    assert_eq!(lifetime(&cal, &one, W), Some(Encl::exact(q(10, 1))), "10 Gyr");
    assert!(effective_temperature(&cal, &one, W).expect("").contains(&q(5772, 1)));
    assert_eq!(luminosity(&cal, &q(2, 1), W), Some(Encl::exact(q(16, 1))), "M⁴");
    assert_eq!(lifetime(&cal, &q(2, 1), W), Some(Encl::exact(q(5, 4))), "fuel over burn");
    let red = luminosity(&cal, &q(1, 4), W).expect("");
    assert!(red.hi < q(1, 50), "a red dwarf is faint: {red:?}");
    assert_eq!(luminosity(&cal, &q(5, 100), W), None, "below hydrogen burning");
    assert_eq!(luminosity(&cal, &q(200, 1), W), None, "past the calibration");
    // The Earth at q = 1 lies inside the Sun's habitable zone.
    let (inner, outer) = habitable_zone(&cal, &Encl::exact(one.clone()), W).expect("");
    assert!(inner.hi < one && outer.lo > one, "{inner:?} {outer:?}");
    assert_eq!(fraction_above(&q(5772, 1), &Q::zero(), W), Some(Encl::exact(one.clone())), "all light above zero");
    let hot = fraction_above(&q(10_000, 1), &q(20_000, 1), W).expect("");
    let cool = fraction_above(&q(5772, 1), &q(20_000, 1), W).expect("");
    assert_eq!(cool.lt(&hot), Some(true), "a hotter star sends more light past a threshold");
}

#[test]
fn a_star_composes_its_calibration_with_its_universes_laws() {
    let cal = sun();
    let at_ref = star(&cal, &Laws::reference(), &Q::one(), W).expect("");
    assert!(at_ref.luminosity.contains(&Q::one()) && at_ref.radius.contains(&Q::one()));
    assert!(at_ref.lifetime.contains(&q(10, 1)) && at_ref.teff.contains(&q(5772, 1)));
    // g = 2: star_mass halves, so a unit-mass star reads the calibration at
    // m·g = 2 (L = 16), scaled by star_light √2 and star_radius ½.
    let heavy = Laws { alpha_g: q(2, 1), ..Laws::reference() };
    let s = star(&cal, &heavy, &Q::one(), W).expect("");
    let l2 = s.luminosity.square();
    assert!(l2.contains(&q(512, 1)), "L² = (16√2)² = 512: {l2:?}");
    let r_cal = stellar_radius(&cal, &q(2, 1), W).expect("");
    assert!(s.radius.contains(&(&r_cal.lo / q(2, 1))) || s.radius.contains(&(&r_cal.hi / q(2, 1))), "radius ×½");
    let fates = Fates {
        collapse: q(8, 1),
        black_hole: q(20, 1),
        white_dwarf: q(6, 10),
        neutron_star: q(14, 10),
        hole_fraction: q(1, 2),
    };
    assert_eq!(death(&fates, &Q::one(), false), Death::WhiteDwarf);
    assert_eq!(death(&fates, &Q::one(), true), Death::Ia);
    assert_eq!(death(&fates, &q(8, 1), true), Death::NeutronStar, "collapse from 8, binary or not");
    assert_eq!(death(&fates, &q(30, 1), false), Death::BlackHole);
    assert_eq!(remnant(&fates, Death::WhiteDwarf, &Q::one()), q(6, 10));
    assert_eq!(remnant(&fates, Death::Ia, &Q::one()), Q::zero());
    assert_eq!(remnant(&fates, Death::NeutronStar, &q(10, 1)), q(14, 10));
    assert_eq!(remnant(&fates, Death::BlackHole, &q(30, 1)), q(15, 1));
}

#[test]
fn the_horizon_inverts_and_the_bounce_recycles_exactly() {
    let o = [Q::zero(), Q::zero(), Q::zero()];
    let sigma = horizon(&o, &Q::one());
    // r² = 4: distance 4 emerges at 1, the horizon (2) is fixed, an involution.
    assert_eq!(emerges(&sigma, &[q(4, 1), Q::zero(), Q::zero()]), Some(vec![Q::one(), Q::zero(), Q::zero()]));
    assert_eq!(emerges(&sigma, &[Q::zero(), q(2, 1), Q::zero()]), Some(vec![Q::zero(), q(2, 1), Q::zero()]));
    let x = [q(3, 1), q(-5, 2), q(7, 3)];
    let once = emerges(&sigma, &x).expect("");
    assert_eq!(emerges(&sigma, &once), Some(x.to_vec()), "inversion is its own inverse");
    let c = [q(10, 1), q(-3, 1), q(1, 2)];
    let off = horizon(&c, &q(3, 1));
    let y = [q(1, 1), q(2, 1), q(5, 1)];
    assert_eq!(emerges(&off, &emerges(&off, &y).expect("")), Some(y.to_vec()), "about any centre");
    assert_eq!(emerges(&off, &[q(10, 1), q(-3, 1), q(13, 2)]), Some(vec![q(10, 1), q(-3, 1), q(13, 2)]), "on the horizon");

    let bounce = Bounce::of(&o, &Q::one(), &q(1, 2)).expect("the source holds at the crossing");
    assert_eq!(bounce.horizon, sigma);
    assert!(Bounce::of(&c, &q(3, 1), &q(9, 10)).is_some(), "about any centre");
    assert_eq!(Bounce::of(&o, &Q::one(), &Q::one()), None, "spin below one");
    assert_eq!(Bounce::of(&o, &Q::one(), &Q::zero()), None, "spin above zero");

    let law = Recycling { crossing: q(3, 10), radiated: q(1, 10), tau: q(5, 1) };
    let mut books = Books::default();
    books.fall(&law, &q(100, 1)).expect("");
    assert_eq!((books.growth.clone(), books.radiated.clone(), books.reservoir.clone()), (q(27, 1), q(3, 1), q(70, 1)));
    books.arrive(&q(4, 1)).expect("");
    assert_eq!(books.jet(&law, &Q::zero(), W), Some(Q::zero()), "no time, no jets");
    let back = books.jet(&law, &q(5, 1), W).expect("");
    assert!(back > q(4424, 100) && back < q(4425, 100), "70·(1 − 1/e): {back}");
    books.fall(&law, &q(7, 3)).expect("");
    assert!(books.balanced(), "fallen + arrived = growth + radiated + reservoir + returned");
    assert_eq!(books.fall(&Recycling { crossing: q(3, 2), ..law }, &Q::one()), None, "a fraction is in [0, 1]");
}

#[test]
fn water_boils_and_phases_read_off_its_triple_and_critical_points() {
    let w = water();
    let at_ref = boiling_point(&w, &w.p_ref, W).expect("");
    assert!(at_ref.contains(&q(37315, 100)), "373.15 K at its reference pressure: {at_ref:?}");
    let higher = boiling_point(&w, &q(202_650, 1), W).expect("");
    assert_eq!(at_ref.lt(&higher), Some(true), "boils hotter at higher pressure");
    let crushed = boiling_point(&w, &q(10_000_000_000, 1), W).expect("");
    assert_eq!(crushed, Encl::exact(q(647_096, 1000)), "capped at the critical temperature");
    assert_eq!(phase(&w, &q(29815, 100), &q(101_325, 1), W), Some(Phase::Liquid), "room temperature");
    assert_eq!(phase(&w, &q(26315, 100), &q(101_325, 1), W), Some(Phase::Solid), "below the triple point");
    assert_eq!(phase(&w, &q(40000, 100), &q(101_325, 1), W), Some(Phase::Gas), "above boiling at 1 atm");
    assert_eq!(phase(&w, &q(700, 1), &q(30_000_000, 1), W), Some(Phase::Supercritical));
    assert!(stefan_boltzmann(&Q::one(), &q(16, 1), W).expect("").contains(&q(2, 1)), "T ∝ flux^¼");
    assert_eq!(landauer_bits(&q(300, 1), &q(150, 1)), Some(q(2, 1)));
    assert!(cognition_depth(&q(4, 1), &Q::one(), W).expect("").contains(&q(1, 2)), "bits ¼ × speed 2");
}

#[test]
fn the_liquid_drop_binds_and_its_valley_reads_the_stable_charge() {
    let only = |volume: i64, pairing: i64| Semf {
        volume: q(volume, 1),
        surface: Q::zero(),
        coulomb: Q::zero(),
        asymmetry: Q::zero(),
        pairing: q(pairing, 1),
    };
    assert!(binding(&only(1, 0), 26, 56, W).expect("").contains(&q(56, 1)), "volume alone is A");
    assert!(binding(&only(0, 1), 2, 4, W).expect("").contains(&q(1, 2)), "even–even: +a_P/√A");
    let odd_odd = binding(&only(0, 1), 1, 2, W).expect("");
    assert!(odd_odd.hi.is_negative(), "odd–odd binds less: {odd_odd:?}");
    assert_eq!(binding(&only(0, 1), 1, 3, W), Some(Encl::exact(Q::zero())), "odd A: no pairing");
    assert_eq!(binding(&only(1, 0), 3, 2, W), None, "Z > A refused");
    // Our universe's coefficients, passed in (MeV): iron-56 binds near 492.
    let ours = Semf {
        volume: q(158, 10),
        surface: q(183, 10),
        coulomb: q(714, 1000),
        asymmetry: q(232, 10),
        pairing: q(12, 1),
    };
    let iron = binding(&ours, 26, 56, W).expect("");
    assert!(iron.lo > q(485, 1) && iron.hi < q(495, 1), "{iron:?}");
    let z0 = valley(&ours, 56, W).expect("");
    assert!(z0.lo > q(25, 1) && z0.hi < q(26, 1), "the valley at A = 56: {z0:?}");
    let per = binding_per_nucleon(&ours, 26, 56, W).expect("");
    assert!(per.lo > q(86, 10) && per.hi < q(89, 10), "about 8.8 MeV per nucleon: {per:?}");
    let a = most_stable_a(&ours, 26, W).expect("");
    assert!((54..=58).contains(&a) && a % 2 == 0, "iron's most stable A: {a}");
}

#[test]
fn madelung_places_every_charge_through_the_g_block() {
    for z in 1..=AUFBAU_CAP {
        let total: u32 = configuration(z).expect("").iter().map(|s| s.electrons).sum();
        assert_eq!(total, z, "the configuration holds Z electrons at {z}");
    }
    assert_eq!(configuration(0), None);
    assert_eq!(configuration(AUFBAU_CAP + 1), None, "no h subshell");
    let at = |z| {
        let p = place(z).expect("");
        (p.period, p.block, p.column, iupac_group(&p))
    };
    assert_eq!(at(1), (1, Block::S, 1, Some(1)), "hydrogen");
    assert_eq!(at(2), (1, Block::S, 2, Some(18)), "helium closes the first period");
    assert_eq!(at(10), (2, Block::P, 6, Some(18)), "neon");
    assert_eq!(at(26), (4, Block::D, 6, Some(8)), "iron");
    assert_eq!(at(79), (6, Block::D, 9, Some(11)), "gold, Madelung's idealised 5d⁹");
    assert_eq!(at(57), (6, Block::F, 1, None), "the f block has no group");
    assert_eq!(at(92), (7, Block::F, 4, None), "uranium, 5f");
    assert_eq!(at(114), (7, Block::P, 2, Some(14)), "flerovium");
    assert_eq!(at(118), (7, Block::P, 6, Some(18)), "the seventh period's close");
    assert_eq!(at(120), (8, Block::S, 2, Some(2)), "the eighth period's 8s²");
    assert_eq!(at(126), (8, Block::G, 6, None), "5g⁶");
    assert_eq!(at(121), (8, Block::G, 1, None), "the g block opens");
    assert_eq!(at(168), (8, Block::P, 6, Some(18)), "the eighth period's close");
    assert_eq!(at(172), (9, Block::G, 2, None), "6g at the cap");
}

#[test]
fn system_laws_are_pure_mathematics() {
    assert_eq!(epidemic_final_size(&Q::one(), W), Some(Encl::exact(Q::zero())), "no epidemic at R₀ ≤ 1");
    // R₀ = 2: z ≈ 0.796812.
    let z = epidemic_final_size(&q(2, 1), W).expect("");
    assert!(z.lo > q(7968, 10000) && z.hi < q(7969, 10000), "{z:?}");
    assert_eq!(lanchester_odds(&Q::one(), &q(2, 1), &Q::one(), &q(1, 1)), Some(q(4, 1)), "the square law");
    assert_eq!(present_value(&q(121, 1), &q(1, 10), 2), Some(q(100, 1)));
    assert!(wright_cost(&q(100, 1), &q(2, 1), &q(4, 5), W).expect("").contains(&q(80, 1)), "one doubling at 0.8");
    assert_eq!(depreciated(&q(100, 1), &q(1, 10), 2), q(81, 1));
}
