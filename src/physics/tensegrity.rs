// A.10.1 — ORBITAL TENSEGRITY: coherence is the stationary equilibrium of a
// constellation of at least two co-present congruent frames, circulating
// under one generator, their mutual tethers balancing its strain (ruled
// 2026-09-28, the operator's word; MU-RECURSION 78.51).
//
//! The physics is composed of what the register already holds, never built
//! beside it:
//!
//! - A FRAME is a body's NUCLEUS `s` (its dual sphere, `Blade::nucleus`) and
//!   its POSE `θ` — the versor carrying the constellation's first frame onto
//!   it, at its canonical ray (`Mv::canonical`), so `θ_j θ_i⁻¹` is the
//!   transition from frame `i` to frame `j`, gauge-free.
//! - THE COUPLING `V_ij = (θ_j θ̃_i) · (s_j s_i)` is A.10's tether `s_j s_i`
//!   (`blade::tether_of`) under A.16's refraction; `B_ij = ⟨V_ij⟩₂`. Poses
//!   compose by the reverse (78.32(1): rays), so every coupling is an exact
//!   ray of the canonical frames.
//! - THE INVERSIVE DISTANCE `δ²_ij = ⟨s_i s_j⟩₀² / (s_i² s_j²)` and its well
//!   are GR1's (`cga::pair_regime`): a Coxeter well `δ² ∈ {0, ¼, ½, ¾}`
//!   (`m = 2, 3, 4, 6`), tangency at 1.
//! - THE NET TORQUE `τ = Σ_{i<j} ½ [B_ij, B_orbit]` — the lift terms of 78.46,
//!   summed in the orbit algebra, order-free — and the STRAIN
//!   `ℰ = ‖τ‖² / ‖B_orbit‖²`, `‖·‖²` the sum of the squared coefficients on
//!   the orthonormal blades (78.51 repair B): positive-definite, so a null
//!   torque (`⟨τ²⟩₀ = 0`, `τ ≠ 0`) reads, and the ratio is free of the
//!   generator's scale.
//!
//! Stationary equilibrium is `τ = 0`, coefficient by coefficient over ℚ (law
//! II, repair A) — never a scalar trace; `ℰ` is its reading, and law IV
//! orders by it. A SINGULAR COLLAPSE is two nuclei at one spacetime locus,
//! `s_i ∧ s_j = 0` (repair C): a body seen at two hours stands at two heights
//! on the spin axis, two nuclei, never a collapse.
//!
//! THE HOURS ARE HEIGHTS (78.51, the ruling of 2026-09-28): hour `m` of a
//! return stands at height `m` on the spin axis `e₃` ([`lifted`]), so a day's
//! frames stand along the axis the spin turns about.

use crate::transducer::cga::{self, Mv, PairRegime};
use num_rational::BigRational;
use num_traits::Zero;

type Q = BigRational;

/// ONE FRAME OF A CONSTELLATION: a body's nucleus and its pose.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Frame {
    pub nucleus: Mv,
    pub pose: Mv,
}

/// AN ORBITAL CONSTELLATION: co-present congruent frames circulating under
/// one generator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Constellation {
    pub frames: Vec<Frame>,
    pub generator: Mv,
}

/// WHY A CONSTELLATION IS NOT COHERENT, OR AN OPERATION IS NOT ADMITTED —
/// each by name, carrying its content (A.10.1 §4).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// Fewer than two co-present frames: no coupling, nothing to balance.
    MonocularOrbit,
    /// The net torque does not vanish: the couplings do not balance the
    /// generator, and the constellation precesses.
    UnbalancedTensegrity { net_torque: Mv },
    /// The operation raises the strain by this much.
    DestabilizingOperation { delta_strain: Q },
    /// Two frames' nuclei coincide as a ray, `s_i ∧ s_j = 0`: two frames
    /// crushed into one.
    SingularCollapse,
}

/// THE COUPLING `V_ij = (θ_j θ̃_i)(s_j s_i)` between two frames.
pub fn coupling(i: &Frame, j: &Frame) -> Mv {
    j.pose.geo(&i.pose.reverse()).geo(&crate::transducer::blade::tether_of(&i.nucleus, &j.nucleus))
}

/// THE INVERSIVE DISTANCE of two frames and its well (GR1).
pub fn inversive(i: &Frame, j: &Frame) -> (Q, PairRegime) {
    cga::pair_regime(&cga::gram_form(&[i.nucleus.clone(), j.nucleus.clone()]), 0, 1)
}

/// Whether two frames' nuclei coincide as a ray: `s_i ∧ s_j = 0`.
fn collapsed(i: &Frame, j: &Frame) -> bool {
    i.nucleus.geo(&j.nucleus).grade(2).is_zero()
}

/// THE TORQUE A COUPLING EXERTS ON A GENERATOR (78.51(1)): `τ = ½ [B, G]`,
/// the commutator of the two bivectors — 78.46's LIFT, the part of `B` that
/// anticommutes with `G`, while the part that commutes is its DRAG. Zero
/// exactly where `B` turns about `G`'s own axis; its coefficients are the
/// lever arm by which `B` stands off that axis. Exact, an element of the
/// algebra, never a scalar.
pub fn torque(coupling: &Mv, generator: &Mv) -> Mv {
    let (b, g) = (coupling.grade(2), generator.grade(2));
    b.geo(&g).sub(&g.geo(&b)).scale(&Q::new(1.into(), 2.into()))
}

/// THE NET TORQUE `τ = Σ_{i<j} ½ [B_ij, B_orbit]`: each coupling's
/// [`torque`] over the pairs of one constellation's own frames (rule 1),
/// summed in the algebra.
pub fn net_torque(c: &Constellation) -> Mv {
    let mut tau = Mv::zero();
    for (a, i) in c.frames.iter().enumerate() {
        for j in c.frames.iter().skip(a + 1) {
            tau = tau.add(&torque(&coupling(i, j), &c.generator));
        }
    }
    tau
}

/// A STAR (78.51(2)): the centre a constellation turns about and the mass
/// standing there. Its place is a coordinate, never a constant: a star is
/// ported by giving another.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Star {
    pub centre: Vec<Q>,
    pub mass: Q,
}

/// THE BARYCENTRE of bodies `(centre, mass)`: `C = Σ mᵢcᵢ / Σ mᵢ`, carrying
/// the total mass (78.51(3)) — the star a constellation of several bodies
/// turns about, a single body's its own centroid. `None` where the masses sum
/// to zero or the bodies stand in different dimensions.
pub fn barycentre(bodies: &[(Vec<Q>, Q)]) -> Option<Star> {
    let [(first, _), ..] = bodies else { return None };
    let dim = first.len();
    let mut mass = Q::zero();
    let mut moment = vec![Q::zero(); dim];
    for (c, m) in bodies {
        if c.len() != dim {
            return None;
        }
        mass += m;
        for (s, x) in moment.iter_mut().zip(c) {
            *s += m * x;
        }
    }
    if mass.is_zero() {
        return None;
    }
    Some(Star { centre: moment.into_iter().map(|s| s / &mass).collect(), mass })
}

/// A MOTION CENTRED AT `c` (78.51(3), the intrinsic axis): `T_c V T̃_c`, the
/// motion `V` makes about the origin made about `c` — never about an
/// arbitrary origin (rule 15).
pub fn centred(v: &Mv, c: &[Q]) -> Mv {
    let t = cga::translator_at(c);
    t.geo(v).geo(&t.reverse())
}

/// THE ORBIT ABOUT A STAR (78.51(2)): the turn about the star carrying a body's
/// centre from `before` to `after`. With `u = before − S`, `v = after − S`,
/// the rotor is `(u + v) u = |u|² + v u` — two reflections, exact over ℚ —
/// centred on the star; at the opposite point (`u + v = 0`) it is the spin
/// plane's own bivector `e₁e₂`. The identity where the body stands still at
/// the star. `None` where the body did not keep its distance (`|u|² ≠ |v|²`:
/// no circular orbit of this star carries it) or left the star's own place.
pub fn orbit_about(star: &Star, before: &[Q], after: &[Q]) -> Option<Mv> {
    let rel = |x: &[Q]| -> Option<Vec<Q>> {
        (x.len() == star.centre.len()).then(|| x.iter().zip(&star.centre).map(|(a, s)| a - s).collect())
    };
    let (u, v) = (rel(before)?, rel(after)?);
    let (uu, vv) = (u.iter().map(|x| x * x).fold(Q::zero(), |s, x| s + x), v.iter().map(|x| x * x).fold(Q::zero(), |s, x| s + x));
    if uu != vv {
        return None;
    }
    if uu.is_zero() {
        return Some(Mv::scalar(num_traits::One::one()));
    }
    let (bu, bv) = (cga::base_vector(&u), cga::base_vector(&v));
    let r = bu.add(&bv).geo(&bu);
    let r = match r.is_zero() {
        true => cga::rotator(&Q::zero(), &num_traits::One::one()),
        false => r,
    };
    Some(centred(&r, &star.centre))
}

/// THE SPIN (78.51(2)): what of a rigid step is left once its orbit about the
/// star is taken out — `S = Õ · step`, a turn about the body's own centre
/// `before`. The same step read against a star at another place splits into
/// another orbit and another spin: the star's distance sets the spin, read,
/// never assumed. `None` where no orbit of the star carries the body.
pub fn spin_of(step: &Mv, star: &Star, before: &[Q], after: &[Q]) -> Option<Mv> {
    Some(orbit_about(star, before, after)?.reverse().geo(step))
}

/// THE COEFFICIENT NORM `‖A‖² = Σ c²` over the orthonormal blades (78.51
/// repair B): zero exactly where `A` is.
pub fn coefficient_norm(a: &Mv) -> Q {
    (0..a.slot_count()).map(|m| a.coeff(m) * a.coeff(m)).fold(Q::zero(), |s, x| s + x)
}

/// THE STRAIN `ℰ = ‖τ‖² / ‖B_orbit‖²` (A.10.1 law III, repair B). `None`
/// where the generator has no bivector — no orbit to strain.
pub fn strain(c: &Constellation) -> Option<Q> {
    let b = coefficient_norm(&c.generator.grade(2));
    if b.is_zero() {
        return None;
    }
    Some(coefficient_norm(&net_torque(c)) / b)
}

/// HOUR `m`'S FRAME ON THE SPIN AXIS (78.51): the nucleus carried to height
/// `m` along `e₃`, the axis a turn in `e₁e₂` spins about. The pose stands.
pub fn lifted(f: &Frame, m: u64) -> Frame {
    let n = f.nucleus.n().max(3);
    let lift = cga::translator_at(&[Q::zero(), Q::zero(), Q::from_integer(m.into())]).widened(n);
    Frame { nucleus: lift.sandwich(&f.nucleus.widened(n)), pose: f.pose.widened(n) }
}

/// STATIONARY EQUILIBRIUM (A.10.1 laws I, II): at least two co-present
/// frames, no two collapsed into one, and the net torque zero, exactly.
pub fn equilibrium(c: &Constellation) -> Result<(), Refusal> {
    let [first, rest @ ..] = c.frames.as_slice() else { return Err(Refusal::MonocularOrbit) };
    if rest.is_empty() {
        return Err(Refusal::MonocularOrbit);
    }
    let mut seen: Vec<&Frame> = vec![first];
    for f in rest {
        if seen.iter().any(|g| collapsed(g, f)) {
            return Err(Refusal::SingularCollapse);
        }
        seen.push(f);
    }
    let tau = net_torque(c);
    match tau.is_zero() {
        true => Ok(()),
        false => Err(Refusal::UnbalancedTensegrity { net_torque: tau }),
    }
}

/// A FRAME CARRIED BY AN OPERATION `T` (A.10.1 law IV): its nucleus by the
/// sandwich, its pose composed.
pub fn carried(f: &Frame, t: &Mv) -> Frame {
    Frame { nucleus: t.sandwich(&f.nucleus), pose: t.geo(&f.pose) }
}

/// THE FORCE AN OPERATION EXERTS (A.10.1 law IV): `ΔE = ℰ(𝒞') − ℰ(𝒞)`, `𝒞'`
/// the constellation with frame `k` carried by `t`. `None` where either
/// strain has no reading (a parabolic generator) or `k` names no frame.
pub fn force(c: &Constellation, k: usize, t: &Mv) -> Option<Q> {
    let mut after = c.clone();
    let f = after.frames.get_mut(k)?;
    *f = carried(f, t);
    Some(strain(&after)? - strain(c)?)
}

/// THE CONSOLIDATION GATE (A.10.1 §5): a constellation `after` is admitted in
/// place of `before` exactly where it stands in equilibrium (laws I, II) and,
/// where both strains read, the strain did not rise and stands at zero.
/// Where a strain has no reading, law II has decided.
///
/// Repair B makes the strain zero exactly where the torque is, so an `after`
/// in equilibrium reads `ℰ = 0` and never rises: law II decides, and the
/// strain's rise is refused where it is read on its own ([`force`]).
pub fn consolidate(before: &Constellation, after: &Constellation) -> Result<(), Refusal> {
    equilibrium(after)?;
    match (strain(before), strain(after)) {
        (Some(b), Some(a)) if a > b => Err(Refusal::DestabilizingOperation { delta_strain: a - b }),
        _ => Ok(()),
    }
}
