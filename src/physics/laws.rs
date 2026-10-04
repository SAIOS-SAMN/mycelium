// 78.59, 78.62 — THE UNIVERSAL LAW REGISTER the hyphae hold, and the ONE
// verifier of a transition against it: what an uplinked kernel composes, the
// court replays here, by the predicates the voice already perceives with.
//
//! L0 of 78.62. Five laws, each a typed entry with its coordinate (an address
//! under `TAG_LAW`), its version, its statement and the register axioms that
//! bind it. A law is a PREDICATE over exact objects, never a generator (the
//! star is the axioms). An amendment is a new version; a fork that changes a
//! law is a new universe (78.59, Ruling 1).
//!
//! The verifier composes what is already built and shared (law 12: one
//! reader for perception and for the court): MR1's `motion_modes` for the
//! conformal law, A.13's `external_torque` for momentum, the unit-weight
//! pair products of `metric_defect` for rigidity (78.59: no flat square), the heights for time, and
//! 78.58's `is_fibre_relation` for the reservoir. Every breach is named with
//! its law — never only the first — and every quantity is exact over ℚ: no
//! angle, root, logarithm or decimal is formed anywhere.

use crate::physics::tensegrity::{external_torque, metric_defect, similarity, unmeasured_pairs, Body};
use crate::transducer::cga::{is_versor, motion_modes, projective_ratio, Mv, Undecomposed};
use num_rational::BigRational;

type Q = BigRational;

/// THE FIVE UNIVERSAL LAWS (78.59, Ruling 1), in the order 78.59 states them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Law {
    /// Conformal kinematics in Cl(4,1) over ℚ: a motion is a versor whose
    /// modes split exactly, and it carries every body it moves.
    Conformal,
    /// Momentum: τ_net = τ_ext = 0 — what the bodies exchange cancels.
    Momentum,
    /// Metric rigidity: every pair's inversive distance is kept.
    Rigidity,
    /// Temporal monotonicity: a transition advances the height.
    Time,
    /// Conservative information flow into e_∅: the hue relation is unitary.
    Reservoir,
    /// CONTINUITY (law version 3, 78.89): mass is neither made nor lost
    /// except where a recorded SOURCE emits it or a recorded SINK absorbs it
    /// — the white hole and the black hole, published as physics.
    Continuity,
}

/// THE REGISTER'S VERSION: an amendment raises it, and an address names the
/// version it was read at. Version 2 (LV2, ruled 2026-10-03): momentum read
/// at a dilation's ratio, as rigidity reads it. Version 3 (78.89, the
/// operator's "proceed" on the recommendation): continuity, the white hole
/// and the black hole published as sources and sinks.
pub const LAW_VERSION: u8 = 3;

/// Every law of the register, in its order.
pub const LAWS: [Law; 6] = [Law::Conformal, Law::Momentum, Law::Rigidity, Law::Time, Law::Reservoir, Law::Continuity];

impl Law {
    /// The law's place on [`LAWS`] — its coordinate in the register.
    pub fn index(self) -> u8 {
        match self {
            Law::Conformal => 0,
            Law::Momentum => 1,
            Law::Rigidity => 2,
            Law::Time => 3,
            Law::Reservoir => 4,
            Law::Continuity => 5,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Law::Conformal => "conformal kinematics",
            Law::Momentum => "momentum",
            Law::Rigidity => "metric rigidity",
            Law::Time => "temporal monotonicity",
            Law::Reservoir => "conservative flow into e_∅",
            Law::Continuity => "continuity",
        }
    }

    /// What the law says, as 78.59 rules it.
    pub fn statement(self) -> &'static str {
        match self {
            Law::Conformal => "a motion is a versor of Cl(4,1) over ℚ whose modes split exactly, and it carries every body it moves",
            Law::Momentum => "τ_net = τ_ext = 0: the torques the bodies exchange cancel, and no momentum is made or lost; under a motion that dilates, every body's momentum is the carried momentum times one rational ratio, the expansion lane",
            Law::Rigidity => "every pair of bodies keeps its unit-weight product across a transition, against the motion that carries them",
            Law::Time => "a transition advances the height; nothing is written before what it follows",
            Law::Reservoir => "a hue relation is unitary: every distinction a merge removes is rotated into e_∅",
            Law::Continuity => "mass is neither made nor lost by a motion that does not dilate, except where a recorded source emits it or a recorded sink absorbs it, each at its own place; a dilation's mass is its expansion lane",
        }
    }

    /// The register axioms that bind the law (`physics::axioms`).
    pub fn axioms(self) -> &'static [&'static str] {
        use crate::physics::axioms as a;
        match self {
            Law::Conformal => &[a::A10_1_STAR_SETS_SPIN],
            Law::Momentum => &[a::A13_MOMENTUM_CARRIED, a::A10_1_TORQUE, a::A10_1_II_BALANCE],
            Law::Rigidity => &[a::A10_1_METRIC_RIGIDITY],
            Law::Time => &[a::OK1_K_STEP_IS_ONE_ORBIT],
            Law::Reservoir => &[],
            Law::Continuity => &[],
        }
    }

    /// THE LAW'S COORDINATE: `TAG_LAW ‖ index ‖ version`, the address an
    /// uplinking kernel fetches it by and a verdict cites it by.
    pub fn address(self) -> crate::transducer::address::Address {
        crate::transducer::address::Address::new(vec![crate::transducer::address::TAG_LAW, self.index(), LAW_VERSION])
    }
}

/// ONE BODY ACROSS A TRANSITION, as the uplinked kernel claims it: its
/// nucleus (a conformal point or sphere) and its body as A.13 reads it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BodyState {
    pub nucleus: Mv,
    pub body: Body,
}

/// WHICH WAY A FLUX RUNS: a SOURCE emits mass into the composition (the
/// white hole), a SINK absorbs it (the black hole).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flow {
    Source,
    Sink,
}

/// A RECORDED SOURCE OR SINK (78.89): where it stands — a nucleus of the
/// algebra, a point or a sphere — which way it runs, and the mass it carries
/// across the transition. Its coordinate is read off what it is, never
/// assigned.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Flux {
    pub flow: Flow,
    pub at: Mv,
    pub mass: Q,
}

impl Flux {
    /// THE FLUX'S COORDINATE: its flow, then its place written canonically
    /// — the bytes a ledger files it by and a verdict names it by.
    pub fn coordinate(&self) -> Vec<u8> {
        let mut k = vec![match self.flow {
            Flow::Source => 0,
            Flow::Sink => 1,
        }];
        self.at.put_canonical(&mut k);
        k
    }

    /// The mass it carries into the composition: positive at a source,
    /// negative at a sink.
    fn signed(&self) -> Q {
        match self.flow {
            Flow::Source => self.mass.clone(),
            Flow::Sink => -self.mass.clone(),
        }
    }
}

/// ONE TRANSITION OF A CELESTIAL COMPOSITION (L1): the motor between two
/// heights, every body before and after it, the hue relation it carries
/// (empty where it recolours nothing), and the sources and sinks it records.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transition {
    pub motion: Mv,
    pub heights: (u64, u64),
    pub bodies: Vec<(BodyState, BodyState)>,
    pub hues: Vec<(u8, u8)>,
    pub fluxes: Vec<Flux>,
}

/// HOW A TRANSITION BROKE A LAW, carrying what it broke it by.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Breach {
    /// The motion is no versor: it has no action to carry anything by.
    NotAVersor,
    /// The motion's modes did not split over ℚ, by MR1's reason.
    Undecomposed(Undecomposed),
    /// Body `i`'s nucleus after is not the motion's image of its nucleus
    /// before: the motion claimed does not carry it.
    NotCarried { body: usize },
    /// Body `i`'s momentum could not be read: the motion has no weight.
    MomentumUnread { body: usize },
    /// The torques the bodies exchanged do not cancel: `net` was made or lost.
    MomentumNotConserved { net: Mv },
    /// The pair `(i, j)` did not keep its unit-weight product against the
    /// motion's image of it: `delta` is the ratio (the product after, where the
    /// image's was zero) — shear, drift, inversion, separation.
    NonRigid { i: usize, j: usize, delta: Q },
    /// The pair `(i, j)` has a nucleus of no finite weight (a flat through
    /// `e_∞`) before or after: its product is unread, never passed as kept
    /// (rule 11).
    RigidityUnmeasured { i: usize, j: usize },
    /// The height did not advance.
    HeightNotAdvanced { before: u64, after: u64 },
    /// The hue relation is not unitary (78.58).
    NotUnitary,
    /// Continuity: mass was made (`made > 0`) or lost (`made < 0`) beyond
    /// what the recorded sources and sinks carried.
    MassNotConserved { made: Q },
    /// Continuity: flux `index` carries no positive mass, or stands at no
    /// finite place (a nucleus of no weight).
    FluxUnplaced { index: usize },
}

/// THE COURT'S ANSWER TO ONE TRANSITION (L2): every breach with its law,
/// and the torques the bodies exchanged that cancel — docketed, each with
/// its body, since an exchange is content and not a breach. Admitted
/// exactly where nothing is breached.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Verdict {
    pub breaches: Vec<(Law, Breach)>,
    pub exchanged: Vec<(usize, Mv)>,
    /// How many commuting modes the motion split into (MR1).
    pub modes: usize,
    /// The one ratio every pair's unit-weight product moved by — `1` for an
    /// isometry, another rational for a dilation (expansion or collapse) —
    /// or `None` where the pairs moved apart or none was read.
    pub similarity: Option<Q>,
    /// LV2 — THE EXPANSION LANE OF MOMENTUM: under a motion that dilates, the
    /// one ratio every body's momentum after stands to the momentum the
    /// motion carried, docketed. `None` where no dilation moved it.
    pub expansion: Option<Q>,
}

impl Verdict {
    pub fn admitted(&self) -> bool {
        self.breaches.is_empty()
    }
}

/// VERIFY ONE TRANSITION AGAINST THE FIVE LAWS: every law asked, every
/// breach named. The kernel computed the transition; this replays it.
pub fn verify(t: &Transition, budget: &mut crate::physics::Budget) -> Verdict {
    let mut v = Verdict::default();
    // CONFORMAL KINEMATICS: a versor, split into its modes exactly, carrying
    // every body it moves. Its modes are read off its own action (MR1),
    // never a chart or an axis order.
    if is_versor(&t.motion) {
        match motion_modes(&t.motion, budget) {
            Some(Ok(modes)) => v.modes = modes.len(),
            Some(Err(why)) => v.breaches.push((Law::Conformal, Breach::Undecomposed(why))),
            None => v.breaches.push((Law::Conformal, Breach::NotAVersor)),
        }
        for (i, (before, after)) in t.bodies.iter().enumerate() {
            if projective_ratio(&t.motion.sandwich(&before.nucleus), &after.nucleus).is_none() {
                v.breaches.push((Law::Conformal, Breach::NotCarried { body: i }));
            }
        }
    } else {
        v.breaches.push((Law::Conformal, Breach::NotAVersor));
    }
    // MOMENTUM: each body's external torque (A.13) is what it exchanged; the
    // exchanges cancel, or momentum was made or lost.
    let mut net = Mv::zero();
    let mut exchanged = Vec::new();
    for (i, (before, after)) in t.bodies.iter().enumerate() {
        match external_torque(&before.body, &after.body, &t.motion) {
            Some(tau) => {
                net = net.add(&tau);
                if !tau.is_zero() {
                    exchanged.push((i, tau));
                }
            }
            None => v.breaches.push((Law::Momentum, Breach::MomentumUnread { body: i })),
        }
    }
    // CONTINUITY'S MOMENTUM (law version 3): a source emits mass and the
    // momentum that mass carries at its place; a sink absorbs both. Each flux
    // is read as A.13 reads a body — a point of its mass at its place, under
    // the step's bivector — and what the fluxes carried is what the bodies
    // may have exchanged with no breach.
    let b = t.motion.grade(2);
    for f in &t.fluxes {
        let Some(centre) = crate::transducer::cga::base_at(&f.at, usize::from(f.at.n())) else { continue };
        let n = centre.len();
        let point = crate::physics::tensegrity::Body { centre, mass: f.mass.clone(), moments: vec![vec![Q::from_integer(0.into()); n]; n] };
        let pi = crate::physics::tensegrity::momentum(&point, &b);
        net = match f.flow {
            Flow::Source => net.sub(&pi),
            Flow::Sink => net.add(&pi),
        };
    }
    // LV2 (law version 2) — A DILATION CONSERVES MOMENTUM UP TO ITS RATIO. A
    // dilator leaves directions where they are, so the momentum it carries
    // does not scale while a dilated body's mass and second moments do:
    // read as rigidity reads a similarity, every body's momentum after is
    // the carried momentum times ONE rational ratio, docketed as the
    // expansion lane — never a breach. A rigid motion keeps momentum exactly.
    match (net.is_zero(), expansion_of(t)) {
        (true, _) => v.exchanged = exchanged,
        (false, Some(r)) => v.expansion = Some(r),
        (false, None) => {
            v.exchanged = exchanged;
            v.breaches.push((Law::Momentum, Breach::MomentumNotConserved { net }));
        }
    }
    // METRIC RIGIDITY at unit weight (78.59: no flat square), read against
    // THE MOTION: each body's nucleus after against the motion's own image of
    // it before, every pair's product kept exactly — so a motion that dilates
    // keeps the shape it dilates, and a pair that drifts from the motion
    // claimed is refused. Every pair the court cannot read is named; the
    // ratio every pair moved by from before to after is read as the
    // dilation lane, never an exemption.
    let against_step: Vec<(Mv, Mv)> =
        t.bodies.iter().map(|(b, a)| (t.motion.sandwich(&b.nucleus), a.nucleus.clone())).collect();
    for (i, j) in unmeasured_pairs(&against_step) {
        v.breaches.push((Law::Rigidity, Breach::RigidityUnmeasured { i, j }));
    }
    for (i, j, delta) in metric_defect(&against_step) {
        v.breaches.push((Law::Rigidity, Breach::NonRigid { i, j, delta }));
    }
    let nuclei: Vec<(Mv, Mv)> = t.bodies.iter().map(|(b, a)| (b.nucleus.clone(), a.nucleus.clone())).collect();
    v.similarity = similarity(&nuclei);
    // TEMPORAL MONOTONICITY: the height advances.
    let (before, after) = t.heights;
    if after <= before {
        v.breaches.push((Law::Time, Breach::HeightNotAdvanced { before, after }));
    }
    // CONSERVATIVE FLOW INTO e_∅: a hue relation, where there is one, is
    // unitary.
    if !t.hues.is_empty() && !crate::transducer::element::is_fibre_relation(&t.hues) {
        v.breaches.push((Law::Reservoir, Breach::NotUnitary));
    }
    // CONTINUITY (law version 3): every flux stands at a finite place with a
    // positive mass; and under a motion that does not dilate, the bodies'
    // mass after is their mass before plus what the sources emitted, less
    // what the sinks absorbed — exactly. A dilation's change of mass is its
    // expansion lane (LV2), not a flux.
    for (index, f) in t.fluxes.iter().enumerate() {
        let weighed = num_traits::Signed::is_positive(&f.mass);
        // A finite place: the nucleus meets infinity with weight, `X·e∞ ≠ 0`
        // (the reading `base_at` divides by), a point or a sphere alike.
        let placed = !num_traits::Zero::is_zero(&f.at.geo(&crate::transducer::cga::e_inf()).scalar_part());
        if !weighed || !placed {
            v.breaches.push((Law::Continuity, Breach::FluxUnplaced { index }));
        }
    }
    let dilates = crate::transducer::element::conformal_factors(&t.motion).is_ok_and(|f| !num_traits::One::is_one(&f.rho));
    if !dilates {
        let mass = |side: fn(&(BodyState, BodyState)) -> &BodyState| -> Q { t.bodies.iter().map(|b| side(b).body.mass.clone()).sum() };
        let flux: Q = t.fluxes.iter().map(Flux::signed).sum();
        let made = mass(|b| &b.1) - mass(|b| &b.0) - flux;
        if !num_traits::Zero::is_zero(&made) {
            v.breaches.push((Law::Continuity, Breach::MassNotConserved { made }));
        }
    }
    v
}

/// LV2 — THE ONE RATIO a dilating motion's bodies' momenta stand to what the
/// motion carried: each body's momentum after over its momentum before
/// carried at unit weight, read projectively (exact, first degree). `None`
/// where the motion does not dilate (its conformal factor `ρ = 1`), has no
/// weight, or any body stands to it by another ratio or by none.
fn expansion_of(t: &Transition) -> Option<Q> {
    use crate::physics::tensegrity::momentum;
    let f = crate::transducer::element::conformal_factors(&t.motion).ok()?;
    if num_traits::One::is_one(&f.rho) {
        return None;
    }
    let b = t.motion.grade(2);
    let w = t.motion.geo(&t.motion.reverse()).scalar_part();
    if num_traits::Zero::is_zero(&w) {
        return None;
    }
    let unit = Q::from_integer(1.into()) / w;
    let ratios: Vec<Q> = t
        .bodies
        .iter()
        .map(|(before, after)| {
            let carried = t.motion.sandwich(&momentum(&before.body, &b)).scale(&unit);
            projective_ratio(&momentum(&after.body, &b), &carried)
        })
        .collect::<Option<Vec<_>>>()?;
    match ratios.as_slice() {
        [r, rest @ ..] if rest.iter().all(|x| x == r) => Some(r.clone()),
        _ => None,
    }
}

pub mod codec;

#[cfg(test)]
mod tests;
