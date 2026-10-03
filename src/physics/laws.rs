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
}

/// THE REGISTER'S VERSION: an amendment raises it, and an address names the
/// version it was read at.
pub const LAW_VERSION: u8 = 1;

/// Every law of the register, in its order.
pub const LAWS: [Law; 5] = [Law::Conformal, Law::Momentum, Law::Rigidity, Law::Time, Law::Reservoir];

impl Law {
    /// The law's place on [`LAWS`] — its coordinate in the register.
    pub fn index(self) -> u8 {
        match self {
            Law::Conformal => 0,
            Law::Momentum => 1,
            Law::Rigidity => 2,
            Law::Time => 3,
            Law::Reservoir => 4,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Law::Conformal => "conformal kinematics",
            Law::Momentum => "momentum",
            Law::Rigidity => "metric rigidity",
            Law::Time => "temporal monotonicity",
            Law::Reservoir => "conservative flow into e_∅",
        }
    }

    /// What the law says, as 78.59 rules it.
    pub fn statement(self) -> &'static str {
        match self {
            Law::Conformal => "a motion is a versor of Cl(4,1) over ℚ whose modes split exactly, and it carries every body it moves",
            Law::Momentum => "τ_net = τ_ext = 0: the torques the bodies exchange cancel, and no momentum is made or lost",
            Law::Rigidity => "every pair of bodies keeps its unit-weight product across a transition, against the motion that carries them",
            Law::Time => "a transition advances the height; nothing is written before what it follows",
            Law::Reservoir => "a hue relation is unitary: every distinction a merge removes is rotated into e_∅",
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

/// ONE TRANSITION OF A CELESTIAL COMPOSITION (L1): the motor between two
/// heights, every body before and after it, and the hue relation it carries
/// (empty where it recolours nothing).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transition {
    pub motion: Mv,
    pub heights: (u64, u64),
    pub bodies: Vec<(BodyState, BodyState)>,
    pub hues: Vec<(u8, u8)>,
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
    for (i, (before, after)) in t.bodies.iter().enumerate() {
        match external_torque(&before.body, &after.body, &t.motion) {
            Some(tau) => {
                net = net.add(&tau);
                if !tau.is_zero() {
                    v.exchanged.push((i, tau));
                }
            }
            None => v.breaches.push((Law::Momentum, Breach::MomentumUnread { body: i })),
        }
    }
    if !net.is_zero() {
        v.breaches.push((Law::Momentum, Breach::MomentumNotConserved { net }));
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
    v
}

pub mod codec;

#[cfg(test)]
mod tests;
