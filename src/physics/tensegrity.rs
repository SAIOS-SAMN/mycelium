// A.10.1 — ORBITAL TENSEGRITY: coherence is the stationary equilibrium of a
// constellation of at least two co-present congruent frames, circulating
// under one generator, their attitudes balancing it and their mutual
// inversive distances kept (ruled and amended 2026-09-28, the operator's
// word; MU-RECURSION 78.51).
//
//! The physics is composed of what the register already holds, never built
//! beside it:
//!
//! - A FRAME is a body's NUCLEUS `s` (its dual sphere, `Blade::nucleus`) and
//!   its POSE `θ` — the versor carrying the constellation's first frame onto
//!   it, at its canonical ray (`Mv::canonical`), so `θ_j θ_i⁻¹` is the
//!   transition from frame `i` to frame `j`, gauge-free.
//! - THE COUPLING is ATTITUDE, POSE ONLY (78.51(4′), Row 1 of 78.52):
//!   `V_ij = θ_j θ̃_i`, `B_ij = ⟨V_ij⟩₂`. A frame's hour is carried IN its
//!   pose by the hour rotor ([`succession`]), so no height is stored or read
//!   and no axial slide stands between frames. The transverse chord `s_j s_i`
//!   stands in no bracket: a chord across the axis turns with the turn.
//!   Poses compose by the reverse (78.32(1): rays).
//! - THE PAIR, read in the algebra and never as a squared scalar (78.59):
//!   its regime is GR1's (`cga::pair_regime` — the blade's geometric square,
//!   the rotor's double-angle point: a Coxeter well at `m = 2, 3, 4, 6`,
//!   tangency parabolic), and its separation the product at unit weight
//!   ([`unit_product`]). METRIC RIGIDITY: every pair's product kept across
//!   the transition ([`rigid`]); against a step that dilates, kept relative
//!   to the step's own image; the common ratio read as the dilation lane
//!   ([`similarity`]).
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
//! `s_i ∧ s_j = 0` (repair C): a body seen at two hours stands at two places
//! on its succession, two nuclei, never a collapse.
//!
//! THE HOURS WIND ON A CLIFFORD TORUS (78.52 Row 1, ruled 2026-09-28): the
//! hour rotor pairs the step's turn in `e₁e₂` with the same turn in the
//! orthogonal plane `e₃e₊` — an ISOCLINIC rotor, the two planes commuting —
//! centred on the constellation's own centre. Hour `m` is the rotor's `m`-th
//! power, and the hours close after the turn's own period `ℓ`: a Villarceau
//! winding, no ruler, no unit step. A slide, which has no turn, advances
//! along its own parabolic parameter: its hour is the slide's own power.
//!
//! A.13 · DISCRETE ANGULAR MOMENTUM (78.52 Row 1): a body's momentum is its
//! inertia acting on the step's bivector, `Π = M₂(B)` ([`momentum`]), read at
//! the body's own centroid; an unforced orbit carries it, `Π_{m+1} =
//! g Π_m g̃`, and what it does not carry is the external torque
//! ([`external_torque`]) — the demand for the body or field that exerted it.

use crate::transducer::cga::{self, Mv, PairRegime, E1, E2, E3, EP};
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
    /// The pair `(i, j)` did not keep its unit-weight product across the
    /// transition: `delta` is its ratio `after / before` (its product after,
    /// where it was zero) — shear, drift, inversion, or bodies flying apart.
    NonRigid { i: usize, j: usize, delta: Q },
    /// The step reads no succession: it does not factor, turns off the spin
    /// plane, or — with no turn — is no pure slide.
    NoSuccession,
    /// A.13 — the momentum was not carried from one hour to the next: `tau`,
    /// the external torque, is what some other body or field exerted.
    ExternalTorque { tau: Mv },
    /// 78.57 — THE COHERENCE FORM DID NOT CLOSE: every defect the transition
    /// carries, each in its own grade, never summed (`Defect`).
    Incoherent { defect: Box<Defect> },
}

/// 78.57 — THE COHERENCE FORM OF ONE TRANSITION: its defects, each kept in its
/// own grade and NEVER SUMMED into a scalar — no weights, no loss. A sum
/// would destroy the one thing the engine needs to know: which blade is
/// missing. The form closes exactly where every component vanishes over ℚ;
/// a component that does not is the missing blade itself, filed as the next
/// direction of growth.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Defect {
    /// Δω — per constellation, the net torque its couplings leave on the
    /// generator (a bivector, `½Σ[B_ij, G]`): the rotor still missing.
    pub torsion: Vec<Mv>,
    /// Every pair `(i, j)` whose unit-weight product moved (grade 0, its
    /// ratio): shear, drift, separation.
    pub metric: Vec<(usize, usize, Q)>,
    /// τ_ext — every hour step whose momentum the orbit did not carry (A.13):
    /// the external torque some unmodelled body or field exerted.
    pub momentum: Vec<Mv>,
}

impl Defect {
    /// Whether the form closes: every component zero, exactly.
    pub fn closes(&self) -> bool {
        self.torsion.is_empty() && self.metric.is_empty() && self.momentum.is_empty()
    }
}

/// THE PAIR'S PRODUCT AT UNIT WEIGHT (78.59, the operator's ruling "fix the
/// normalisation": no flat square). Each nucleus is read at unit weight
/// `ŝ = s / w`, `w = −⟨s e_∞⟩₀`, and the pair's invariant is `ŝ_a · ŝ_b =
/// ⟨s_a s_b⟩₀ / (w_a w_b)` — first degree in each body, exact: for two points
/// it is `−½` their squared distance, for spheres the power of the pair. An
/// isometry keeps it; a dilation scales every pair by one ratio. It replaces
/// `δ² = ⟨s_a s_b⟩₀² / (s_a² s_b²)`, a scalar squared and divided by squares.
/// `None` for a nucleus of no finite weight (a flat through `e_∞`).
pub fn unit_product(a: &Mv, b: &Mv) -> Option<Q> {
    let einf = cga::e_inf();
    let (wa, wb) = (-a.geo(&einf).scalar_part(), -b.geo(&einf).scalar_part());
    if wa.is_zero() || wb.is_zero() {
        return None;
    }
    Some(a.geo(b).scalar_part() / (wa * wb))
}

/// METRIC RIGIDITY, WHOLE (78.51(4′), at unit weight): every pair of
/// `bodies` (each `(before, after)`) is read by [`unit_product`] on both
/// sides, and every pair whose product moved is returned with its ratio
/// `after / before` (its product after, where it was zero) — none chosen
/// over another (rule 4). Strict: a dilation moves every pair and is read
/// against the step that dilates, never excused here (a single pair flying
/// apart is a common ratio of one pair). Pairs of no finite weight are
/// [`unmeasured_pairs`], never kept.
pub fn metric_defect(bodies: &[(Mv, Mv)]) -> Vec<(usize, usize, Q)> {
    let (ratios, mut out) = pair_ratios(bodies);
    out.extend(ratios.into_iter().filter(|(_, _, r)| !num_traits::One::is_one(r)));
    out
}

/// THE ONE RATIO every measured pair's unit-weight product moved by, where
/// there is one: `1` for an isometry, another rational for a dilation — the
/// expansion or collapse lane, a READING, never an exemption from rigidity.
/// `None` where the pairs moved by different ratios, or no pair was measured.
pub fn similarity(bodies: &[(Mv, Mv)]) -> Option<Q> {
    let (ratios, _) = pair_ratios(bodies);
    match ratios.as_slice() {
        [(_, _, r), rest @ ..] if rest.iter().all(|(_, _, x)| x == r) => Some(r.clone()),
        _ => None,
    }
}

/// The pairs whose product cannot be read at unit weight, before or after.
pub fn unmeasured_pairs(bodies: &[(Mv, Mv)]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for (i, (bi, ai)) in bodies.iter().enumerate() {
        for (j, (bj, aj)) in bodies.iter().enumerate().skip(i + 1) {
            if unit_product(bi, bj).is_none() || unit_product(ai, aj).is_none() {
                out.push((i, j));
            }
        }
    }
    out
}

/// One pair `(i, j)` and a rational read of it.
type PairRead = (usize, usize, Q);

/// Each measured pair's ratio `after / before`, and apart the pairs whose
/// product was zero and moved (with the product after).
fn pair_ratios(bodies: &[(Mv, Mv)]) -> (Vec<PairRead>, Vec<PairRead>) {
    let (mut ratios, mut moved_from_zero) = (Vec::new(), Vec::new());
    for (i, (bi, ai)) in bodies.iter().enumerate() {
        for (j, (bj, aj)) in bodies.iter().enumerate().skip(i + 1) {
            let (Some(pb), Some(pa)) = (unit_product(bi, bj), unit_product(ai, aj)) else { continue };
            match (pb.is_zero(), pa.is_zero()) {
                (true, true) => {}
                (true, false) => moved_from_zero.push((i, j, pa)),
                (false, _) => ratios.push((i, j, pa / pb)),
            }
        }
    }
    (ratios, moved_from_zero)
}

/// 78.57 — THE COHERENCE FORM BEGUN: the structure read first — frames that
/// do not stand, or a strain the transition raises, leave no form to read and
/// are refused — then Δω, the torque `after`'s couplings leave on its
/// generator. The caller adds the transition's metric defect and τ_ext (they depend on
/// which bodies it relates) and asks `Defect::closes`.
pub fn coherence(before: &Constellation, after: &Constellation) -> Result<Defect, Refusal> {
    frames_stand(after)?;
    if let (Some(b), Some(a)) = (strain(before), strain(after)) {
        if a > b {
            return Err(Refusal::DestabilizingOperation { delta_strain: a - b });
        }
    }
    let mut defect = Defect::default();
    let tau = net_torque(after);
    if !tau.is_zero() {
        defect.torsion.push(tau);
    }
    Ok(defect)
}

/// THE FRAMES STAND (the structural half of `equilibrium`): at least two, no
/// two collapsed into one. Where they do not, the coherence form cannot be
/// read at all — a refusal, never a defect.
pub fn frames_stand(c: &Constellation) -> Result<(), Refusal> {
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
    Ok(())
}

/// THE ATTITUDE COUPLING `V_ij = θ_j θ̃_i` between two frames (78.51(4′),
/// 78.52 Row 1): the relative pose, the hour carried in it.
pub fn coupling(i: &Frame, j: &Frame) -> Mv {
    j.pose.geo(&i.pose.reverse())
}

/// THE HOUR ROTOR (78.52 Row 1): the succession a step winds its hours on,
/// centred at `centre`.
/// - A TURN `R₁₂ = s + b e₁e₂` (the step's rotation factor, its rational
///   point exact) is paired with the same turn in the orthogonal plane,
///   `R₃₊ = s + b e₃e₊`: `R_hour = T_c (R₁₂ R₃₊) T̃_c`, isoclinic, the two
///   planes commuting; it closes after the turn's own period.
/// - A step with NO TURN — a slide (parabolic), a breathing or boost
///   (hyperbolic), no inversion — winds on its own parameter: its hour rotor
///   is its carriage `T·K·D` itself, never put on the elliptic torus.
///
/// `None` where the step does not factor, turns outside `e₁e₂`, inverts, or
/// is the identity (reflexive, never an orbit, decided before any
/// succession is asked).
///
/// THE TORUS'S UNIT (78.52(1)): the plane `e₃e₊` carries a length — the unit
/// sphere about the centre is its pole, `∝ e₊`, and a body of exactly that
/// extent returns to its own ray at half the period.
pub fn succession(step: &Mv, centre: &[Q]) -> Option<Mv> {
    use num_traits::One;
    let f = crate::transducer::element::conformal_factors(step).ok()?;
    let r = f.rotation.canonical()?;
    let (s, b) = (r.scalar_part(), r.coeff(E1 | E2).clone());
    if !r.sub(&cga::rotator(&s, &b)).is_zero() {
        return None;
    }
    match b.is_zero() {
        false => {
            let isoclinic = cga::rotator(&s, &b).geo(&cga::rotator_on_blade(&s, &b, E3 | EP));
            Some(centred(&isoclinic, centre))
        }
        // No turn: the carriage winds on its own parameter where it is one
        // fascicle's stream, PARABOLIC or HYPERBOLIC by its streamline (78.52
        // Row 2); an elliptic reading here would be a turn the factors did
        // not see, and a composite of two fascicles winds on no one parameter.
        true => {
            use crate::physics::fascicle::{streamline_of, Streamline};
            let moves = !f.rho.is_one() || f.transverses() || !f.translation.iter().all(Zero::is_zero);
            let carriage = f.carriage();
            let flows = matches!(streamline_of(&carriage), Some(Streamline::Parabolic | Streamline::Hyperbolic));
            (moves && !f.inverted && flows).then_some(carriage)
        }
    }
}

/// HOUR `m`'S FRAME (78.52 Row 1): the body's nucleus `s` at its first hour,
/// carried by the hour rotor's `m`-th power — the nucleus to its place on the
/// succession, the pose the power itself. Each power held at its canonical
/// ray.
pub fn hour(s: &Mv, r_hour: &Mv, m: u64) -> Frame {
    let mut power = Mv::scalar(num_traits::One::one());
    for _ in 0..m {
        let next = r_hour.geo(&power);
        power = match next.canonical() {
            Some(ray) => ray,
            None => next,
        };
    }
    carried(&Frame { nucleus: s.clone(), pose: Mv::scalar(num_traits::One::one()) }, &power)
}

/// A BODY AS A.13 READS IT: its centroid, its mass (its sites), and its
/// second moments about the centroid on its base axes, `moments[a][b] =
/// ∫ x_a x_b` (`Blade::moment2`), exact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Body {
    pub centre: Vec<Q>,
    pub mass: Q,
    pub moments: Vec<Vec<Q>>,
}

/// `e_a ∧ e_b` for base axes `a`, `b` counted from 0.
fn plane(a: usize, b: usize) -> Mv {
    let e = |i: usize| Mv::blade(cga::base_axis(i + 1), num_traits::One::one());
    e(a).geo(&e(b)).grade(2)
}

/// A.13 · THE DISCRETE ANGULAR MOMENTUM `Π = M₂(B)` of a body under the
/// bivector `B` (78.52 Row 1). `B` is read at the body's centroid (`T̃_c B
/// T_c`); on each spatial plane the inertia acts as a rigid body's does,
/// `M₂(e_a ∧ e_b) = Σ_k (C_ka e_k ∧ e_b − C_kb e_k ∧ e_a)` — `Σ x ∧ (x · B)`
/// over the body — and every other part, which carries the centroid, is
/// weighed by the mass; the result is carried back to the centroid's place.
/// Exact over ℚ; no velocity, no derivative (law 1): `B` is a step's own
/// bivector.
pub fn momentum(body: &Body, b: &Mv) -> Mv {
    let t = cga::translator_at(&body.centre);
    let local = t.reverse().geo(b).geo(&t).grade(2);
    let n = body.moments.len();
    let mut pi = Mv::zero();
    for mask in 0..local.slot_count() {
        let c = local.coeff(mask);
        if mask.count_ones() != 2 || c.is_zero() {
            continue;
        }
        let axes: Vec<usize> = (0..n).filter(|&i| mask & cga::base_axis(i + 1) != 0).collect();
        match axes.as_slice() {
            [a, b] => {
                for (k, row) in body.moments.iter().enumerate() {
                    let (cka, ckb) = (row.get(*a), row.get(*b));
                    if let (Some(cka), Some(ckb)) = (cka, ckb) {
                        pi = pi.add(&plane(k, *b).scale(&(c * cka)).sub(&plane(k, *a).scale(&(c * ckb))));
                    }
                }
            }
            _ => pi = pi.add(&Mv::blade(mask, c * &body.mass)),
        }
    }
    t.geo(&pi).geo(&t.reverse())
}

/// ONE BODY OF A GROUND at hour 0: its nucleus and its body as A.13 reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroundBody {
    pub nucleus: Mv,
    pub body: Body,
}

/// A THOUGHT'S GROUND (78.52): the constellation a thought holds — PRIVATE to
/// the mind, never on the wire (the operator's word, 2026-09-28: a court
/// replays transcribed speech, not a mind's thinking). The CLASS of its
/// generator, the PERIOD of its return, its BODIES at hour 0 in the order
/// they arrived (one body or a scene's several), the generator `g`, and the
/// coupling `V₀₁` (the hour rotor at the bodies' barycentre). Every other
/// hour is re-derived, `R_hourᵐ F₀`; the barycentre, the attitude couplings
/// and the rigidity matrix are derived from the bodies, never stored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ground {
    pub class: crate::transducer::address::Address,
    pub period: u64,
    pub bodies: Vec<GroundBody>,
    pub generator: Mv,
    pub coupling: Mv,
}

impl Ground {
    /// THE GROUND'S COORDINATES as a key (78.52(6) Stage C): the chart its
    /// hours stand on, then the class, the period, and each body's nucleus
    /// at hour 0 — the thought filed where it stands, two orbits of one class
    /// at two places never one key.
    pub fn key_on(&self, chart: &[u8]) -> Option<crate::transducer::address::Address> {
        // Filed under the CLASS stratum: the thought is its class, period
        // and ground on its chart — no form stands for it.
        let mut k = vec![crate::transducer::address::TAG_CLASS];
        crate::transducer::address::put_key_bytes(&mut k, chart);
        crate::transducer::address::put_addr(&mut k, &self.class)?;
        k.extend_from_slice(&self.period.to_le_bytes());
        for b in &self.bodies {
            b.nucleus.put_canonical(&mut k);
        }
        Some(crate::transducer::address::Address::from(k))
    }
}

/// THE GROUND'S BARYCENTRE (78.52(6) Stage C): its bodies' centres weighed by
/// their masses — derived, never carried. `None` for a ground with no body
/// or massless bodies.
pub fn ground_centre(g: &Ground) -> Option<Vec<Q>> {
    let bodies: Vec<(Vec<Q>, Q)> = g.bodies.iter().map(|b| (b.body.centre.clone(), b.body.mass.clone())).collect();
    barycentre(&bodies).map(|s| s.centre)
}

/// A BODY CARRIED BY A RIGID STEP `g` (78.52, the word's replay): its centroid
/// by the sandwich, its second moments turned by the step's rotation factor,
/// `C′ = R C Rᵀ`, `R`'s columns the images of the base axes read at unit
/// weight; the mass unchanged. `None` where the step does not factor, scales
/// or transverses (no rigid carriage), or the centroid goes to infinity.
pub fn carried_body(body: &Body, g: &Mv) -> Option<Body> {
    let f = crate::transducer::element::conformal_factors(g).ok()?;
    if !num_traits::One::is_one(&f.rho) || f.transverses() || f.inverted {
        return None;
    }
    let centre = cga::base_at(&g.sandwich(&cga::point_at(&body.centre)), body.centre.len())?;
    let r = &f.rotation;
    let w = r.geo(&r.reverse()).scalar_part();
    if w.is_zero() {
        return None;
    }
    let n = body.moments.len();
    let e = |i: usize| Mv::blade(cga::base_axis(i + 1), num_traits::One::one());
    // image[a][b]: the b-th coordinate of the image of axis a.
    let image: Vec<Vec<Q>> = (0..n)
        .map(|a| {
            let v = r.sandwich(&e(a));
            (0..n).map(|b| v.coeff(cga::base_axis(b + 1)) / &w).collect()
        })
        .collect();
    let mut moments = vec![vec![Q::zero(); n]; n];
    for (b, row) in moments.iter_mut().enumerate() {
        for (d, slot) in row.iter_mut().enumerate() {
            let mut s = Q::zero();
            for (a, ca) in body.moments.iter().enumerate() {
                for (c, cac) in ca.iter().enumerate() {
                    s += &image[a][b] * cac * &image[c][d];
                }
            }
            *slot = s;
        }
    }
    Some(Body { centre, mass: body.mass.clone(), moments })
}

/// A.13 · THE EXTERNAL TORQUE across one hour (78.52 Row 1): the body
/// `before` carried by `step` to the body `after`, both read under the
/// step's own bivector; `τ_ext = Π_after − g Π_before g̃`, zero exactly where
/// the orbit carried the momentum unforced.
///
/// A versor acts as a ray: its sandwich scales by its weight `g g̃`, so the
/// carried momentum is read at unit weight — divided by it, exactly (the
/// scale is content here: momentum is compared coefficient by coefficient).
/// `None` for a null versor, which has no weight to read.
pub fn external_torque(before: &Body, after: &Body, step: &Mv) -> Option<Mv> {
    let b = step.grade(2);
    let w = step.geo(&step.reverse()).scalar_part();
    if w.is_zero() {
        return None;
    }
    let carried = step.sandwich(&momentum(before, &b)).scale(&(Q::from_integer(1.into()) / w));
    Some(momentum(after, &b).sub(&carried))
}

/// TWO FRAMES' PAIR: their product at unit weight ([`unit_product`]) and
/// their regime, read in the algebra (GR1, 78.59) — never a squared ratio.
pub fn inversive(i: &Frame, j: &Frame) -> (Option<Q>, PairRegime) {
    (unit_product(&i.nucleus, &j.nucleus), cga::pair_regime(&[i.nucleus.clone(), j.nucleus.clone()], 0, 1))
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

/// METRIC RIGIDITY (78.51(4′)): each body `(before, after)` — its nucleus on
/// either side of a transition, in the order the caller holds them — and
/// every pair's unit-weight product kept exactly. Rigid motions and
/// reflections keep it; shear, drift, separation, dilation and inversion do
/// not, and the first pair in that order that did not is refused with its
/// ratio. A dilation is admitted where a STEP dilates: the verifier reads
/// rigidity against the step's own image of each body (`physics::laws`).
pub fn rigid(bodies: &[(Mv, Mv)]) -> Result<(), Refusal> {
    match metric_defect(bodies).into_iter().next() {
        Some((i, j, delta)) => Err(Refusal::NonRigid { i, j, delta }),
        None => Ok(()),
    }
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

/// STATIONARY EQUILIBRIUM (A.10.1 laws I, II): at least two co-present
/// frames, no two collapsed into one, and the net torque zero, exactly.
pub fn equilibrium(c: &Constellation) -> Result<(), Refusal> {
    frames_stand(c)?;
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
