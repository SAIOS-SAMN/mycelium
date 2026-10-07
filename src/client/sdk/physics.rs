// 78.131 — UNIVERSAL PHYSICS: the FORMS of the physical laws, as functions
// of a universe's constants (the operator's program, step 1; the
// coordinator's boundary 2026-10-07).
//
//! UNIVERSAL — here, in every universe: the laws' forms, built on exact
//! rationals and `sdk::certified` enclosures. PER UNIVERSE — never here: the
//! constants' values, the Big Bang, all matter and its distribution. This
//! module holds no table of any universe; it holds laws parameterized by
//! one. Every scale ratio is exactly one where every ratio to the
//! universe's reference is one.

use super::certified::{exp, ln, pi, planck_tail, powq, sqrt, tan, Encl};
use crate::physics::source::{keeping, Source};
use crate::transducer::cga::{base_at, point_at, sphere_at, translator_at, Mv, E1, E2};
use num_bigint::BigInt;
use num_rational::BigRational as Q;
use num_traits::{One, Signed, Zero};

fn q(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}

fn int(n: i64) -> Q {
    Q::from_integer(BigInt::from(n))
}

/// `Π base_i^{e_i}` for positive bases and rational exponents: exact where
/// every exponent is an integer, an enclosure otherwise.
fn product(terms: &[(&Q, Q)], w: u32) -> Option<Encl> {
    let w2 = w + 32;
    let mut out = Encl::exact(Q::one());
    for (base, e) in terms {
        if !base.is_positive() {
            return None;
        }
        let factor = if e.is_integer() {
            let n = i32::try_from(e.to_integer()).ok()?;
            Encl::exact(num_traits::pow::Pow::pow(*base, n))
        } else {
            powq(&Encl::exact((*base).clone()), e, w2)?
        };
        out = out.mul(&factor).round(w2);
    }
    Some(out.round(w))
}

// ———————————————————————————— A. the laws ————————————————————————————

/// A UNIVERSE'S LAWS as ratios to its own reference constants: the fine
/// structure `alpha`, the electron-to-proton mass ratio `beta`, the
/// gravitational coupling `alpha_g`, the nuclear efficiency `nuclear`. The
/// reference values themselves are the universe's.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Laws {
    pub alpha: Q,
    pub beta: Q,
    pub alpha_g: Q,
    pub nuclear: Q,
}

impl Laws {
    /// Every ratio one: the universe's own reference.
    pub fn reference() -> Laws {
        Laws { alpha: Q::one(), beta: Q::one(), alpha_g: Q::one(), nuclear: Q::one() }
    }

    /// Chemistry's energy scale: `a² b g^½`.
    pub fn chemistry(&self, w: u32) -> Option<Encl> {
        product(&[(&self.alpha, int(2)), (&self.beta, int(1)), (&self.alpha_g, q(1, 2))], w)
    }

    /// The atom's size: `1/(a b g^½)`.
    pub fn atom(&self, w: u32) -> Option<Encl> {
        product(&[(&self.alpha, int(-1)), (&self.beta, int(-1)), (&self.alpha_g, q(-1, 2))], w)
    }

    /// Condensed matter's density: `a³ b³ g²`.
    pub fn density(&self, w: u32) -> Option<Encl> {
        product(&[(&self.alpha, int(3)), (&self.beta, int(3)), (&self.alpha_g, int(2))], w)
    }

    /// Condensed matter's pressure: `a⁵ b⁴ g²`.
    pub fn pressure(&self, w: u32) -> Option<Encl> {
        product(&[(&self.alpha, int(5)), (&self.beta, int(4)), (&self.alpha_g, int(2))], w)
    }

    /// The proton's mass scale: `g^½`.
    pub fn proton(&self, w: u32) -> Option<Encl> {
        product(&[(&self.alpha_g, q(1, 2))], w)
    }

    /// A star's mass scale: `1/g`.
    pub fn star_mass(&self, w: u32) -> Option<Encl> {
        product(&[(&self.alpha_g, int(-1))], w)
    }

    /// A star's luminosity scale: `b² g^½ / a²`.
    pub fn star_light(&self, w: u32) -> Option<Encl> {
        product(&[(&self.beta, int(2)), (&self.alpha_g, q(1, 2)), (&self.alpha, int(-2))], w)
    }

    /// A star's lifetime scale: `e a² / (b² g^{3/2})`.
    pub fn star_life(&self, w: u32) -> Option<Encl> {
        product(&[(&self.nuclear, int(1)), (&self.alpha, int(2)), (&self.beta, int(-2)), (&self.alpha_g, q(-3, 2))], w)
    }

    /// A star's radius scale: `1/(g a²)`.
    pub fn star_radius(&self, w: u32) -> Option<Encl> {
        product(&[(&self.alpha_g, int(-1)), (&self.alpha, int(-2))], w)
    }

    /// The least star's mass scale: `a^{3/2} / (g b^{3/4})`.
    pub fn star_min(&self, w: u32) -> Option<Encl> {
        product(&[(&self.alpha, q(3, 2)), (&self.alpha_g, int(-1)), (&self.beta, q(-3, 4))], w)
    }

    /// The Jeans mass scale: `a³ b^{3/2} / g^{1/4}`.
    pub fn jeans(&self, w: u32) -> Option<Encl> {
        product(&[(&self.alpha, int(3)), (&self.beta, q(3, 2)), (&self.alpha_g, q(-1, 4))], w)
    }

    /// A star's effective-temperature scale: `(light / radius²)^{1/4}`.
    pub fn star_teff(&self, w: u32) -> Option<Encl> {
        let w2 = w + 32;
        let light = self.star_light(w2)?;
        let r = self.star_radius(w2)?;
        let ratio = light.div(&r.square())?;
        fourth_root(&ratio, w)
    }

    /// The coincidence of stellar and chemical temperatures: `teff /
    /// chemistry` — the habitability ratio.
    pub fn coincidence(&self, w: u32) -> Option<Encl> {
        let w2 = w + 32;
        Some(self.star_teff(w2)?.div(&self.chemistry(w2)?)?.round(w))
    }

    /// A decay's half-life scale at Gamow factor `gamow`: `exp(gamow·(alpha −
    /// 1))` — one at the reference.
    pub fn half_life(&self, gamow: &Q, w: u32) -> Encl {
        exp(&Encl::exact(gamow * (&self.alpha - Q::one())), w)
    }
}

/// `x^{1/4}` for `x > 0`.
fn fourth_root(x: &Encl, w: u32) -> Option<Encl> {
    if !x.lo.is_positive() {
        return None;
    }
    let w2 = w + 32;
    let lo = powq(&Encl::exact(x.lo.clone()), &q(1, 4), w2)?.lo;
    let hi = powq(&Encl::exact(x.hi.clone()), &q(1, 4), w2)?.hi;
    Some(Encl { lo, hi }.round(w))
}

// ———————————————————————————— B. the bounce ————————————————————————————

/// THE HORIZON of mass `m` about `centre` (G = c = 1): the dual sphere of
/// squared radius `4m²` — `r = 2m` is never taken as a root.
pub fn horizon(centre: &[Q], m: &Q) -> Mv {
    sphere_at(centre, &(int(4) * m * m))
}

/// INVERSION IN A SPHERE: `σXσ⁻¹` — distance `d` from the centre goes to
/// `r²/d` on the same ray, the sphere is fixed pointwise, and the map is its
/// own inverse (up to the homogeneous scale a conformal point carries).
/// `None` where `σ` has no inverse (a null `σ`: a point, not a sphere).
pub fn invert(sigma: &Mv, x: &Mv) -> Option<Mv> {
    let s2 = sigma.geo(sigma).scalar_part();
    if s2.is_zero() {
        return None;
    }
    Some(sigma.geo(x).geo(&sigma.scale(&s2.recip())))
}

/// Where a base point `x` EMERGES through the sphere `σ`: the base of its
/// inversion. `None` at the centre (it goes to infinity).
pub fn emerges(sigma: &Mv, x: &[Q]) -> Option<Vec<Q>> {
    base_at(&invert(sigma, &point_at(x))?, x.len())
}

/// AN EINSTEIN–CARTAN BOUNCE: a horizon, and the white-hole source at its
/// equatorial crossing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bounce {
    pub horizon: Mv,
    pub source: Source,
}

impl Bounce {
    /// The bounce of mass `m` and spin `a ∈ (0, 1)` about `centre` (three base
    /// coordinates): the spin is the turn bivector `a·e₁e₂` carried to the
    /// centre, its part the horizon keeps (`physics::source::keeping`)
    /// pushing the equatorial crossing `P = centre + (2m, 0, 0)` — the source
    /// `N = P ∧ (P·K)`, verified null, fixing `P` and keeping the horizon.
    /// `None` for a spin outside `(0, 1)`, a non-positive mass, or a source
    /// the algebra does not hold.
    pub fn of(centre: &[Q], m: &Q, a: &Q) -> Option<Bounce> {
        if centre.len() != 3 || !m.is_positive() || !a.is_positive() || a >= &Q::one() {
            return None;
        }
        let sigma = horizon(centre, m);
        let t = translator_at(centre);
        let turn = t.geo(&Mv::blade(E1 | E2, a.clone())).geo(&t.reverse());
        let k = keeping(&sigma, &turn)?;
        let crossing: Vec<Q> = centre
            .iter()
            .zip([int(2) * m, Q::zero(), Q::zero()])
            .map(|(c, d)| c + d)
            .collect();
        let source = Source::of(&sigma, &point_at(&crossing), &k)?;
        Some(Bounce { horizon: sigma, source })
    }
}

/// A UNIVERSE'S RECYCLING LAW (its parameters passed in): the fraction of
/// what falls that crosses the horizon (`crossing`), the fraction of what
/// crosses that is radiated (`radiated`), and the jets' timescale `tau`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recycling {
    pub crossing: Q,
    pub radiated: Q,
    pub tau: Q,
}

/// THE BOOKS OF A BOUNCE, exact: `fallen + arrived = growth + radiated +
/// reservoir + returned` after every act, and `growth` is the child's
/// content — what crossed and was not radiated becomes the matter on the
/// other side (a sink here, a source there, on continuity).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Books {
    pub fallen: Q,
    pub arrived: Q,
    pub growth: Q,
    pub radiated: Q,
    pub reservoir: Q,
    pub returned: Q,
}

impl Books {
    /// `m` falls in: `crossed = m·h` (of it `crossed·ε` radiated, the rest
    /// growth), and `m − crossed` held in the reservoir. `None` for a
    /// negative mass or a fraction outside `[0, 1]`.
    pub fn fall(&mut self, law: &Recycling, m: &Q) -> Option<()> {
        let unit = |f: &Q| !f.is_negative() && f <= &Q::one();
        if m.is_negative() || !unit(&law.crossing) || !unit(&law.radiated) {
            return None;
        }
        let crossed = m * &law.crossing;
        let lost = &crossed * &law.radiated;
        self.fallen += m;
        self.growth += &crossed - &lost;
        self.radiated += lost;
        self.reservoir += m - crossed;
        Some(())
    }

    /// `m` arrives from a paired sphere — a declared source — and emerges
    /// into this universe through the white hole: it is returned outward.
    pub fn arrive(&mut self, m: &Q) -> Option<()> {
        if m.is_negative() {
            return None;
        }
        self.arrived += m;
        self.returned += m;
        Some(())
    }

    /// The jets over `dt`: they return `reservoir·(1 − e^{−dt/τ})`, fixed at
    /// its enclosure's LOWER bound — never more than the law allows, so the
    /// books stay exact and no matter is invented. Returns what was returned.
    pub fn jet(&mut self, law: &Recycling, dt: &Q, w: u32) -> Option<Q> {
        if !law.tau.is_positive() || dt.is_negative() {
            return None;
        }
        let kept = exp(&Encl::exact(-(dt / &law.tau)), w);
        let back = &self.reservoir * (Q::one() - &kept.hi);
        if back.is_negative() {
            return None;
        }
        self.reservoir -= &back;
        self.returned += &back;
        Some(back)
    }

    /// The identity, read exactly.
    pub fn balanced(&self) -> bool {
        &self.fallen + &self.arrived == &self.growth + &self.radiated + &self.reservoir + &self.returned
    }
}

// ———————————————————————————— C. gravity and orbits as fibres ————————————————————————————

/// Kepler, in the q-form: with `q² = a` (the semi-major axis in the
/// universe's unit) and `m² = M` (the central mass in its unit), the period
/// is `q³/m` exactly — one unit at `q = m = 1`.
pub fn kepler_period(q_axis: &Q, m: &Q) -> Option<Q> {
    if !m.is_positive() {
        return None;
    }
    Some(q_axis * q_axis * q_axis / m)
}

/// The flux received at `q² = a` from a source of luminosity `l`: `l / q⁴`.
pub fn received_flux(l: &Q, q_axis: &Q) -> Option<Q> {
    if q_axis.is_zero() {
        return None;
    }
    let q2 = q_axis * q_axis;
    Some(l / (&q2 * &q2))
}

/// AN ORBIT'S PHASE after `k` steps of a period `period` (in steps): `frac(phase0
/// + k/period)`, exact — a closed fibre returns to its phase.
pub fn orbit_phase(phase0: &Q, k: u64, period: &Q) -> Option<Q> {
    if !period.is_positive() {
        return None;
    }
    let p = phase0 + Q::from_integer(BigInt::from(k)) / period;
    Some(&p - p.floor())
}

/// THE PYTHAGOREAN ROTOR'S POINT at parameter `t`: `((1−t²)/(1+t²),
/// 2t/(1+t²))`, an exact rational point on the unit circle — no trigonometry.
pub fn circle_point(t: &Q) -> (Q, Q) {
    let t2 = t * t;
    let d = Q::one() + &t2;
    ((Q::one() - &t2) / &d, int(2) * t / d)
}

/// AN ORBIT'S POINT: exactly on the unit circle, its angle within
/// `angle_error` (radians, a proven bound) of `2π·phase`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrbitPoint {
    pub x: Q,
    pub y: Q,
    pub angle_error: Q,
}

/// THE ORBIT'S POINT at a phase `φ ∈ [0, 1)`, UNIFORM IN ANGLE (78.43: time is
/// distance along the orbit): the rotor's point at `t = tan(πφ)`, the
/// half-angle of `2πφ`, by certified `tan`, fixed on the grid `2^-w` — an
/// exact rational point on the circle. The phase is carried to `φ′ ∈
/// [−¼, ¼]` first, the point of `φ ∉ [−¼, ¼]` the antipode of `φ − ½`'s, so
/// `|t| ≤ 1` and `φ = ½` is `(−1, 0)` with no infinite parameter. The angle
/// is `2·atan t̂`, and `atan` is 1-Lipschitz, so the error is at most
/// `2·|t̂ − t|`: the grid's half step plus the enclosure's half width.
pub fn orbit_point(phase: &Q, w: u32) -> Option<OrbitPoint> {
    if phase.is_negative() || phase >= &Q::one() {
        return None;
    }
    let quarter = q(1, 4);
    let (reduced, antipode) = if phase <= &quarter {
        (phase.clone(), false)
    } else if phase < &q(3, 4) {
        (phase - q(1, 2), true)
    } else {
        (phase - Q::one(), false)
    };
    let w2 = w + 16;
    let t = tan(&pi(w2).scale(&reduced).round(w2), w2)?;
    let grid = Q::from_integer(BigInt::one() << w);
    let mid = (&t.lo + &t.hi) / int(2);
    let fixed = (&mid * &grid + q(1, 2)).floor() / &grid;
    let angle_error = grid.recip() + t.width();
    let (x, y) = circle_point(&fixed);
    let (x, y) = if antipode { (-x, -y) } else { (x, y) };
    Some(OrbitPoint { x, y, angle_error })
}

/// A NESTED POSITION: a body orbits its parent, which may orbit a core —
/// each level `(radius, phase)` from the innermost centre outward, summed.
pub fn nested_position(levels: &[(Q, Q)], w: u32) -> Option<(Q, Q)> {
    let mut at = (Q::zero(), Q::zero());
    for (r, phase) in levels {
        let p = orbit_point(phase, w)?;
        at = (&at.0 + r * p.x, &at.1 + r * p.y);
    }
    Some(at)
}

/// THE SPACING LAW: successive orbits' `q` at least `8/7` apart, so orbits
/// never cross.
pub fn spaced(q_inner: &Q, q_outer: &Q) -> bool {
    q_outer * int(7) >= q_inner * int(8)
}

/// The Hill radius in the q-form: `q²·(m/(3M))^{1/3}`.
pub fn hill_radius(q_axis: &Q, m: &Q, big_m: &Q, w: u32) -> Option<Encl> {
    if !m.is_positive() || !big_m.is_positive() {
        return None;
    }
    let cube = powq(&Encl::exact(m / (int(3) * big_m)), &q(1, 3), w + 16)?;
    Some(cube.scale(&(q_axis * q_axis)).round(w))
}

/// The escape speed's square: `2M/r` (G = 1).
pub fn escape_speed_squared(big_m: &Q, r: &Q) -> Option<Q> {
    r.is_positive().then(|| int(2) * big_m / r)
}

/// A TIDED BODY (the body's own parameters, passed in — R1): mass `m`, radius
/// `r`, moment-of-inertia factor `c` (`I = c·m·R²`), tidal quality `quality`,
/// Love number `k2`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TidalBody {
    pub m: Q,
    pub r: Q,
    pub c: Q,
    pub quality: Q,
    pub k2: Q,
}

/// The tidal-locking timescale: `ω₀ a⁶ I Q / (3 M² k₂ R⁵)` (G = 1).
pub fn tidal_lock_time(body: &TidalBody, spin0: &Q, a: &Q, big_m: &Q) -> Option<Q> {
    let r = &body.r;
    let den = int(3) * big_m * big_m * &body.k2 * r * r * r * r * r;
    if den.is_zero() {
        return None;
    }
    let a6 = a * a * a * a * a * a;
    Some(spin0 * a6 * &body.c * &body.m * r * r * &body.quality / den)
}

/// The circularisation timescale: `(4/63)(Q/k₂)(m/M)(a/R)⁵ / n`.
pub fn circularisation_time(body: &TidalBody, big_m: &Q, a: &Q, n: &Q) -> Option<Q> {
    if body.k2.is_zero() || big_m.is_zero() || body.r.is_zero() || n.is_zero() {
        return None;
    }
    let ratio = a / &body.r;
    let r5 = &ratio * &ratio * &ratio * &ratio * &ratio;
    Some(q(4, 63) * (&body.quality / &body.k2) * (&body.m / big_m) * r5 / n)
}

// ———————————————————————————— D. stars ————————————————————————————

/// ONE SEGMENT of a piecewise power law: `coeff · M^exponent` for masses up
/// to `upto` (inclusive), in the calibration's own units.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PowerSegment {
    pub upto: Q,
    pub coeff: Q,
    pub exponent: Q,
}

/// A UNIVERSE'S STELLAR CALIBRATION (R1 — passed in; the SDK has no
/// default): the mass–luminosity and mass–radius laws as power-law segments
/// in increasing `upto` order (the calibration's order is its content), the
/// unit star's lifetime and effective temperature, the least main-sequence
/// mass, and the fluxes bounding the habitable zone (inner the larger).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StellarCalibration {
    pub luminosity: Vec<PowerSegment>,
    pub radius: Vec<PowerSegment>,
    pub life: Q,
    pub teff: Q,
    pub min_mass: Q,
    pub hz_inner: Q,
    pub hz_outer: Q,
}

/// The segment law at mass `m`: the first segment whose `upto` holds it;
/// `None` below the least mass or past the last segment.
fn segment_law(segments: &[PowerSegment], min_mass: &Q, m: &Q, w: u32) -> Option<Encl> {
    if m < min_mass {
        return None;
    }
    let seg = segments.iter().find(|s| m <= &s.upto)?;
    product(&[(&seg.coeff, Q::one()), (m, seg.exponent.clone())], w)
}

/// A main-sequence star's luminosity at mass `m`, in the unit star's.
pub fn luminosity(cal: &StellarCalibration, m: &Q, w: u32) -> Option<Encl> {
    segment_law(&cal.luminosity, &cal.min_mass, m, w)
}

/// A main-sequence star's radius at mass `m`, in the unit star's.
pub fn stellar_radius(cal: &StellarCalibration, m: &Q, w: u32) -> Option<Encl> {
    segment_law(&cal.radius, &cal.min_mass, m, w)
}

/// The main-sequence lifetime: fuel over burn, `life · m / L`.
pub fn lifetime(cal: &StellarCalibration, m: &Q, w: u32) -> Option<Encl> {
    let w2 = w + 32;
    Some(Encl::exact(&cal.life * m).div(&luminosity(cal, m, w2)?)?.round(w))
}

/// The effective temperature, Stefan–Boltzmann on the star's own surface:
/// `teff · (L / R²)^{1/4}`.
pub fn effective_temperature(cal: &StellarCalibration, m: &Q, w: u32) -> Option<Encl> {
    let w2 = w + 32;
    let l = luminosity(cal, m, w2)?;
    let r = stellar_radius(cal, m, w2)?;
    Some(fourth_root(&l.div(&r.square())?, w2)?.scale(&cal.teff).round(w))
}

/// The habitable zone of a star of luminosity `l`, in the q-form (`q² = a`):
/// where the received flux `l/q⁴` equals each bounding flux, `q = (l/S)^{1/4}`
/// — `(inner, outer)`.
pub fn habitable_zone(cal: &StellarCalibration, l: &Encl, w: u32) -> Option<(Encl, Encl)> {
    let at = |s: &Q| -> Option<Encl> {
        if !s.is_positive() {
            return None;
        }
        fourth_root(&l.scale(&s.recip()), w)
    };
    Some((at(&cal.hz_inner)?, at(&cal.hz_outer)?))
}

/// The fraction of a black body's light at `teff` carried by photons above
/// the energy of temperature `threshold`: the Planck tail at `threshold /
/// teff`.
pub fn fraction_above(teff: &Q, threshold: &Q, w: u32) -> Option<Encl> {
    if !teff.is_positive() || threshold.is_negative() {
        return None;
    }
    planck_tail(&Encl::exact(threshold / teff), w)
}

/// A STAR READ IN A UNIVERSE: the calibration composed with the universe's
/// `Laws` — the mass in the calibration's units is `m / star_mass = m·g`
/// (exact: `star_mass` is `1/g`), the luminosity scaled by `star_light`, the
/// radius by `star_radius`, the lifetime by `star_life`, and the effective
/// temperature read off the scaled pair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Star {
    pub luminosity: Encl,
    pub radius: Encl,
    pub lifetime: Encl,
    pub teff: Encl,
}

/// The star of mass `m` (in the universe's unit) under `laws` and `cal`.
pub fn star(cal: &StellarCalibration, laws: &Laws, m: &Q, w: u32) -> Option<Star> {
    let w2 = w + 32;
    let m_cal = m * &laws.alpha_g;
    let l = luminosity(cal, &m_cal, w2)?.mul(&laws.star_light(w2)?).round(w2);
    let r = stellar_radius(cal, &m_cal, w2)?.mul(&laws.star_radius(w2)?).round(w2);
    let life = lifetime(cal, &m_cal, w2)?.mul(&laws.star_life(w2)?).round(w2);
    let teff = fourth_root(&l.div(&r.square())?, w2)?.scale(&cal.teff).round(w);
    Some(Star { luminosity: l.round(w), radius: r.round(w), lifetime: life.round(w), teff })
}

/// A UNIVERSE'S STELLAR FATES (passed in — the thresholds and remnants are
/// its own, in the calibration's mass units): core collapse from
/// `collapse`, a black hole from `black_hole`, a binary's white dwarf below
/// `collapse` detonating as a type Ia; the remnants — a white dwarf's mass,
/// a neutron star's, and the fraction of the star a black hole keeps.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fates {
    pub collapse: Q,
    pub black_hole: Q,
    pub white_dwarf: Q,
    pub neutron_star: Q,
    pub hole_fraction: Q,
}

/// HOW A STAR DIES.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Death {
    WhiteDwarf,
    Ia,
    NeutronStar,
    BlackHole,
}

/// The death of a star of mass `m`, `binary` if it has a companion to accrete.
pub fn death(f: &Fates, m: &Q, binary: bool) -> Death {
    if m >= &f.black_hole {
        Death::BlackHole
    } else if m >= &f.collapse {
        Death::NeutronStar
    } else if binary {
        Death::Ia
    } else {
        Death::WhiteDwarf
    }
}

/// The remnant's mass: the white dwarf's or neutron star's, the black hole's
/// kept fraction of `m`; a type Ia leaves none.
pub fn remnant(f: &Fates, kind: Death, m: &Q) -> Q {
    match kind {
        Death::WhiteDwarf => f.white_dwarf.clone(),
        Death::Ia => Q::zero(),
        Death::NeutronStar => f.neutron_star.clone(),
        Death::BlackHole => &f.hole_fraction * m,
    }
}

// ———————————————————————————— E. radiation and thermodynamics ————————————————————————————

/// Stefan–Boltzmann: the equilibrium temperature `T = T_unit · flux^{1/4}`.
pub fn stefan_boltzmann(t_unit: &Q, flux: &Q, w: u32) -> Option<Encl> {
    Some(fourth_root(&Encl::exact(flux.clone()), w + 16)?.scale(t_unit).round(w))
}

/// A SUBSTANCE (passed in — it is a universe's): its normal boiling point
/// `tb` at `p_ref`, its latent heat over the gas constant `l_over_r`, its
/// triple point `(t_triple, p_triple)` and critical point `(t_crit, p_crit)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Substance {
    pub tb: Q,
    pub p_ref: Q,
    pub l_over_r: Q,
    pub t_triple: Q,
    pub p_triple: Q,
    pub t_crit: Q,
    pub p_crit: Q,
}

/// Clausius–Clapeyron's boiling point at pressure `p`: `1/T = 1/Tb −
/// (R/L)·ln(p/p_ref)`, capped at the critical temperature.
pub fn boiling_point(s: &Substance, p: &Q, w: u32) -> Option<Encl> {
    let w2 = w + 32;
    let lnp = ln(&Encl::exact(p / &s.p_ref), w2)?;
    let inv = Encl::exact(s.tb.recip()).sub(&lnp.scale(&s.l_over_r.recip()));
    let t = inv.recip()?;
    if t.lo >= s.t_crit {
        return Some(Encl::exact(s.t_crit.clone()));
    }
    let cap = |v: Q| if v > s.t_crit { s.t_crit.clone() } else { v };
    Some(Encl { lo: cap(t.lo), hi: cap(t.hi) }.round(w))
}

/// The saturation pressure at `T`: `p_t · exp((L/R)(1/T_t − 1/T))`.
pub fn saturation_pressure(s: &Substance, t: &Q, w: u32) -> Option<Encl> {
    if !t.is_positive() {
        return None;
    }
    let arg = &s.l_over_r * (s.t_triple.recip() - t.recip());
    Some(exp(&Encl::exact(arg), w + 16).scale(&s.p_triple).round(w))
}

/// A SUBSTANCE'S PHASE.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Solid,
    Liquid,
    Gas,
    Supercritical,
}

/// The phase at `(T, p)` from the triple and critical points: past both
/// critical values supercritical; below the saturation curve gas; else solid
/// below the triple temperature, liquid above. `None` where `p` lies within
/// the saturation curve's enclosure — undecided, never guessed.
pub fn phase(s: &Substance, t: &Q, p: &Q, w: u32) -> Option<Phase> {
    if t > &s.t_crit && p > &s.p_crit {
        return Some(Phase::Supercritical);
    }
    if t > &s.t_crit {
        return Some(Phase::Gas);
    }
    let sat = saturation_pressure(s, t, w)?;
    if p < &sat.lo {
        return Some(Phase::Gas);
    }
    if p <= &sat.hi {
        return None;
    }
    Some(if t < &s.t_triple { Phase::Solid } else { Phase::Liquid })
}

/// Landauer: the bits an energy budget erases scale as `T_ref / T`.
pub fn landauer_bits(t_ref: &Q, t: &Q) -> Option<Q> {
    t.is_positive().then(|| t_ref / t)
}

/// The thermal speed and noise scale `√(T / T_ref)`.
pub fn thermal_speed(t: &Q, t_ref: &Q, w: u32) -> Option<Encl> {
    if !t_ref.is_positive() {
        return None;
    }
    sqrt(&Encl::exact(t / t_ref), w)
}

/// Cognition's depth: `landauer × speed`.
pub fn cognition_depth(t: &Q, t_ref: &Q, w: u32) -> Option<Encl> {
    Some(thermal_speed(t, t_ref, w + 8)?.scale(&landauer_bits(t_ref, t)?).round(w))
}

// ———————————————————————————— F. nuclei and shells ————————————————————————————

/// THE LIQUID DROP'S COEFFICIENTS (passed in — they are a universe's): volume,
/// surface, Coulomb, asymmetry, pairing, in the universe's energy unit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Semf {
    pub volume: Q,
    pub surface: Q,
    pub coulomb: Q,
    pub asymmetry: Q,
    pub pairing: Q,
}

/// The semi-empirical binding of `(Z, A)`: `a_V A − a_S A^{2/3} − a_C Z(Z−1)/A^{1/3}
/// − a_A (A−2Z)²/A + δ`, with `δ = ±a_P/√A` for even–even / odd–odd, zero for
/// odd `A`. `None` for `A = 0` or `Z > A`.
pub fn binding(c: &Semf, z: u32, a: u32, w: u32) -> Option<Encl> {
    if a == 0 || z > a {
        return None;
    }
    let w2 = w + 32;
    let aq = Q::from_integer(BigInt::from(a));
    let zq = Q::from_integer(BigInt::from(z));
    let cube = powq(&Encl::exact(aq.clone()), &q(1, 3), w2)?;
    let surface = cube.square().scale(&c.surface);
    let coulomb = Encl::exact(&c.coulomb * &zq * (&zq - Q::one())).div(&cube)?;
    let excess = &aq - int(2) * &zq;
    let asymmetry = &c.asymmetry * &excess * &excess / &aq;
    let exact = &c.volume * &aq - asymmetry;
    let mut total = Encl::exact(exact).sub(&surface).sub(&coulomb).round(w2);
    let neutrons = a - z;
    let sign = match (z % 2, neutrons % 2) {
        (0, 0) => Some(Q::one()),
        (1, 1) => Some(-Q::one()),
        _ => None,
    };
    if let Some(sign) = sign {
        let root = sqrt(&Encl::exact(aq), w2)?;
        total = total.add(&Encl::exact(&c.pairing * sign).div(&root)?).round(w2);
    }
    Some(total.round(w))
}

/// THE VALLEY OF STABILITY at mass number `A`: the charge where the binding's
/// derivative in `Z` vanishes (pairing aside), `Z₀ = (4a_A + a_C A^{-1/3}) /
/// (8a_A/A + 2a_C A^{-1/3})` — a real coordinate, an enclosure.
pub fn valley(c: &Semf, a: u32, w: u32) -> Option<Encl> {
    if a == 0 {
        return None;
    }
    let w2 = w + 32;
    let aq = Q::from_integer(BigInt::from(a));
    let inv_cube = powq(&Encl::exact(aq.clone()), &q(1, 3), w2)?.recip()?;
    let num = inv_cube.scale(&c.coulomb).add(&Encl::exact(int(4) * &c.asymmetry));
    let den = inv_cube.scale(&(int(2) * &c.coulomb)).add(&Encl::exact(int(8) * &c.asymmetry / aq));
    Some(num.div(&den)?.round(w))
}

/// The binding per nucleon, `B/A`.
pub fn binding_per_nucleon(c: &Semf, z: u32, a: u32, w: u32) -> Option<Encl> {
    let w2 = w + 32;
    let b = binding(c, z, a, w2)?;
    Some(b.scale(&Q::new(BigInt::one(), BigInt::from(a))).round(w))
}

/// THE MOST STABLE MASS NUMBER for charge `Z`: where `B/A` peaks. Pairing
/// splits `A` into its two parities, each a smooth climb and fall, so each
/// is walked up in steps of two while `B/A` rises DECIDEDLY, and the two
/// peaks compared — every comparison on separated enclosures, `None` where
/// one is undecided or no peak falls before `A = 4Z + 8`.
pub fn most_stable_a(c: &Semf, z: u32, w: u32) -> Option<u32> {
    if z == 0 {
        return None;
    }
    let bound = 4 * z + 8;
    let peak = |start: u32| -> Option<(u32, Encl)> {
        let mut a = start;
        let mut here = binding_per_nucleon(c, z, a, w)?;
        loop {
            let next_a = a + 2;
            if next_a > bound {
                return None;
            }
            let next = binding_per_nucleon(c, z, next_a, w)?;
            match here.lt(&next) {
                Some(true) => {
                    a = next_a;
                    here = next;
                }
                Some(false) if next.lt(&here) == Some(true) => return Some((a, here)),
                _ => return None,
            }
        }
    };
    let (a0, b0) = peak(z)?;
    let (a1, b1) = peak(z + 1)?;
    match b0.lt(&b1) {
        Some(true) => Some(a1),
        Some(false) if b1.lt(&b0) == Some(true) => Some(a0),
        _ => None,
    }
}

/// The charge the Madelung filling is served through (the coordinator's R2):
/// past the eighth period's close at 168 into the ninth's `9s` and `6g`, never
/// to an `h` subshell.
pub const AUFBAU_CAP: u32 = 172;

/// A SUBSHELL'S BLOCK by its angular momentum `l`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Block {
    S,
    P,
    D,
    F,
    G,
}

impl Block {
    fn of(l: u32) -> Option<Block> {
        match l {
            0 => Some(Block::S),
            1 => Some(Block::P),
            2 => Some(Block::D),
            3 => Some(Block::F),
            4 => Some(Block::G),
            _ => None,
        }
    }

    /// The block's width: a subshell of angular momentum `l` holds `2(2l+1)`.
    pub fn width(self) -> u32 {
        match self {
            Block::S => 2,
            Block::P => 6,
            Block::D => 10,
            Block::F => 14,
            Block::G => 18,
        }
    }
}

/// One filled or filling subshell: `(n, block, electrons)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Subshell {
    pub n: u32,
    pub block: Block,
    pub electrons: u32,
}

/// THE MADELUNG CONFIGURATION of charge `Z ≤ 172`: subshells in `n + l`
/// order, within it `n` ascending — constructed in that order, never sorted.
/// The rule's idealised filling: the measured exceptions are a universe's
/// chemistry, not the law's form.
pub fn configuration(z: u32) -> Option<Vec<Subshell>> {
    if z == 0 || z > AUFBAU_CAP {
        return None;
    }
    let mut out = Vec::new();
    let mut left = z;
    let mut sum = 1u32;
    while left > 0 {
        // n + l = sum, l from its largest (n = l + 1) down to zero.
        let mut l = (sum - 1) / 2;
        loop {
            let block = Block::of(l)?;
            let take = left.min(block.width());
            out.push(Subshell { n: sum - l, block, electrons: take });
            left -= take;
            if left == 0 || l == 0 {
                break;
            }
            l -= 1;
        }
        sum += 1;
    }
    Some(out)
}

/// AN ELEMENT'S PLACE (R2): its period, its block, and its column within the
/// block (1 … the block's width).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Place {
    pub period: u32,
    pub block: Block,
    pub column: u32,
}

/// The place of charge `Z`: the period is the count of `s` subshells opened
/// (each opens one), the block and column the last subshell's.
pub fn place(z: u32) -> Option<Place> {
    let config = configuration(z)?;
    let mut period = 0u32;
    let mut last = None;
    for sub in config {
        if sub.block == Block::S {
            period += 1;
        }
        last = Some(sub);
    }
    let last = last?;
    Some(Place { period, block: last.block, column: last.electrons })
}

/// The IUPAC group, for the `s`, `p` and `d` blocks only (R2): `s` columns 1–2
/// (the first period's second, which closes it, 18), `p` 13–18, `d` 3–12.
/// `None` for `f` and `g` — they have no group.
pub fn iupac_group(p: &Place) -> Option<u32> {
    match p.block {
        Block::S if p.period == 1 && p.column == 2 => Some(18),
        Block::S => Some(p.column),
        Block::P => Some(12 + p.column),
        Block::D => Some(2 + p.column),
        Block::F | Block::G => None,
    }
}

// ———————————————————————————— G. system laws ————————————————————————————

/// The epidemic's final size: the root `z ∈ (0, 1)` of `z = 1 − e^{−R₀z}` for
/// `R₀ > 1` (zero at `R₀ ≤ 1`), by bisection on certified signs to width
/// `2^-w`. `f(z) = z − 1 + e^{−R₀z}` is negative just above zero and positive
/// at one; a midpoint whose sign the enclosure cannot decide is refused.
pub fn epidemic_final_size(r0: &Q, w: u32) -> Option<Encl> {
    if r0 <= &Q::one() {
        return Some(Encl::exact(Q::zero()));
    }
    let w2 = w + 16;
    let f = |z: &Q| exp(&Encl::exact(-(r0 * z)), w2).add(&Encl::exact(z - Q::one()));
    // A lower end with f < 0: halve from ½ until the sign is decided negative.
    let mut lo = q(1, 2);
    loop {
        let v = f(&lo);
        if v.hi.is_negative() {
            break;
        }
        if v.lo.is_positive() {
            // f(½) > 0: the root is below; halve down.
            lo /= int(2);
            if lo < Q::new(BigInt::one(), BigInt::one() << w) {
                return None;
            }
            continue;
        }
        return None;
    }
    let mut hi = Q::one();
    let width = Q::new(BigInt::one(), BigInt::one() << w);
    while &hi - &lo > width {
        let mid = (&lo + &hi) / int(2);
        let v = f(&mid);
        if v.hi.is_negative() {
            lo = mid;
        } else if v.lo.is_positive() {
            hi = mid;
        } else {
            return Encl::new(lo, hi);
        }
    }
    Encl::new(lo, hi)
}

/// Lanchester's square law: the odds `(α A²) / (β B²)` — side A prevails
/// above one.
pub fn lanchester_odds(alpha: &Q, a: &Q, beta: &Q, b: &Q) -> Option<Q> {
    let den = beta * b * b;
    (!den.is_zero()).then(|| alpha * a * a / den)
}

/// Present value of `c` received after `n` periods at rate `r`: `c/(1+r)^n`.
pub fn present_value(c: &Q, r: &Q, n: u32) -> Option<Q> {
    let base = Q::one() + r;
    if base.is_zero() {
        return None;
    }
    let mut d = Q::one();
    for _ in 0..n {
        d *= &base;
    }
    Some(c / d)
}

/// Wright's curve: the `n`-th unit's cost `c₁ · n^{log₂ p}`, the progress
/// ratio `p` per people (passed in — never Earth's 0.8).
pub fn wright_cost(c1: &Q, n: &Q, progress: &Q, w: u32) -> Option<Encl> {
    if !n.is_positive() || !progress.is_positive() {
        return None;
    }
    let w2 = w + 32;
    let b = ln(&Encl::exact(progress.clone()), w2)?.div(&ln(&Encl::exact(int(2)), w2)?)?;
    let lnn = ln(&Encl::exact(n.clone()), w2)?;
    Some(exp(&b.mul(&lnn).round(w2), w2).scale(c1).round(w))
}

/// Declining-balance depreciation: `v · (1 − d)^t`.
pub fn depreciated(v: &Q, d: &Q, t: u32) -> Q {
    let keep = Q::one() - d;
    let mut out = v.clone();
    for _ in 0..t {
        out *= &keep;
    }
    out
}
