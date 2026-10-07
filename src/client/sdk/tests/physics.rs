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
    assert_eq!(orbit_point(&Q::zero()), Some((Q::one(), Q::zero())));
    assert_eq!(orbit_point(&q(1, 2)), Some((q(-1, 1), Q::zero())));
    let (_, y_quarter) = orbit_point(&q(1, 4)).expect("");
    let (_, y_three) = orbit_point(&q(3, 4)).expect("");
    assert!(y_quarter > Q::zero() && y_three < Q::zero(), "the upper half, then the lower");
    // Nesting: a moon at phase 0 about a planet at phase ½ about a core.
    assert_eq!(nested_position(&[(q(10, 1), q(1, 2)), (Q::one(), Q::zero())]), Some((q(-9, 1), Q::zero())));
    assert!(spaced(&q(7, 1), &q(8, 1)) && !spaced(&q(7, 1), &q(79, 10)), "8/7 apart");
    assert_eq!(escape_speed_squared(&Q::one(), &q(2, 1)), Some(Q::one()));
    assert!(hill_radius(&Q::one(), &q(3, 1), &q(1, 1), W).expect("").contains(&Q::one()), "(3/3)^(1/3) = 1");
    let body = TidalBody { m: Q::one(), r: Q::one(), c: q(1, 3), quality: q(3, 1), k2: Q::one() };
    assert_eq!(tidal_lock_time(&body, &Q::one(), &Q::one(), &Q::one()), Some(q(1, 3)), "ω₀·c·Q/3");
    assert_eq!(tidal_lock_time(&body, &Q::one(), &q(2, 1), &Q::one()), Some(q(64, 3)), "a⁶");
    let body = TidalBody { quality: q(63, 4), ..body };
    assert_eq!(circularisation_time(&body, &Q::one(), &Q::one(), &Q::one()), Some(Q::one()));
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
    assert_eq!(at(118), (7, Block::P, 6, Some(18)), "the seventh period's close");
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
