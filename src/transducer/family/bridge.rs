// 78.84 — THE BRIDGE: the corpus's own motions put through the universal law
// register (78.62 L0) — does the physics the hyphae will publish admit the
// motion the world shows a mind? Read, never decided: nothing here admits or
// refuses a word.
//
//! ONE PAIR'S TRANSITIONS, as the perception itself carries them. Every input
//! body is read against every output body on its chart (its moduli: only
//! there can a versor carry one onto the other); a CARRIAGE (`carriage`: the
//! versor and the hue map it carries) relates them. Carriages that are one
//! motion — one canonical ray — are ONE TRANSITION holding every body they
//! carry, so rigidity and momentum are read between bodies moved together.
//! Each body's state is its nucleus and its body as A.13 reads it
//! (`tensegrity::body_of_shape`, the one producer). The heights are the
//! pair's own order, input then output.
//!
//! WHAT IS NOT A TRANSITION AT ALL is read too: an input body no output body
//! on its chart is carried onto — its change (growth, carving, a recolour of
//! some of its sites, a reshaping) is no conformal motion, and the register
//! has nothing to say of it. That share is the bridge's first reading: how
//! much of the world's change the laws, as written, can even be asked about.

use super::*;
use crate::physics::laws::{verify, Breach, BodyState, Transition};
use crate::physics::tensegrity::body_of_shape;

/// WHAT THE LAWS SAID OF ONE PAIR (or, summed, of a slice).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Bridge {
    /// Input bodies; those some output body on the input's chart is carried onto;
    /// those none is (no transition to ask the laws of).
    pub bodies: u64,
    pub carried: u64,
    pub uncarried: u64,
    /// Transitions verified, those holding two bodies or more, and admitted.
    pub transitions: u64,
    pub several: u64,
    pub admitted: u64,
    /// Transitions breaching each law, in `LAWS` order — as wide as the
    /// register, so a law added is read here without a second edit.
    pub by_law: [u64; crate::physics::laws::LAWS.len()],
    /// Breaches by kind: not a versor, undecomposed, not carried, momentum
    /// unread, momentum not conserved, non-rigid, rigidity unmeasured, height
    /// not advanced, not unitary, mass not conserved, flux unplaced.
    pub breaches: [u64; BREACH_KINDS],
    /// Transitions whose pairs all moved by one ratio: 1 (isometric) or
    /// another rational (a dilation).
    pub isometric: u64,
    pub dilating: u64,
    /// Transitions with a body whose nucleus or A.13 body did not read.
    pub unbuilt: u64,
    /// Of the transitions momentum refused: those carrying one body, and
    /// those whose motion is odd (a reflection: grade-one or grade-three
    /// parts), where the step's bivector is not the whole motion.
    pub momentum_single: u64,
    pub momentum_odd: u64,
    /// Of them, those whose motion dilates (its conformal factor `ρ ≠ 1`) —
    /// and, of those, where the body's mass (its sites) changed.
    pub momentum_dilating: u64,
    pub momentum_mass_changed: u64,
    /// LV2's evidence: of the momentum refusals, those where every body's
    /// momentum after is a RATIONAL MULTIPLE of the momentum the motion
    /// carried — conserved as a ray, the multiple the expansion lane — and
    /// those where all bodies share one multiple.
    pub momentum_projective: u64,
    pub momentum_one_ratio: u64,
}

impl Bridge {
    pub fn add(&mut self, o: &Bridge) {
        self.bodies += o.bodies;
        self.carried += o.carried;
        self.uncarried += o.uncarried;
        self.transitions += o.transitions;
        self.several += o.several;
        self.admitted += o.admitted;
        self.isometric += o.isometric;
        self.dilating += o.dilating;
        self.unbuilt += o.unbuilt;
        self.momentum_single += o.momentum_single;
        self.momentum_odd += o.momentum_odd;
        self.momentum_dilating += o.momentum_dilating;
        self.momentum_mass_changed += o.momentum_mass_changed;
        self.momentum_projective += o.momentum_projective;
        self.momentum_one_ratio += o.momentum_one_ratio;
        self.by_law.iter_mut().zip(o.by_law).for_each(|(a, b)| *a += b);
        self.breaches.iter_mut().zip(o.breaches).for_each(|(a, b)| *a += b);
    }
}

/// How many kinds of breach the register names: `breach_slot`'s range.
pub const BREACH_KINDS: usize = 11;

fn breach_slot(b: &Breach) -> usize {
    match b {
        Breach::NotAVersor => 0,
        Breach::Undecomposed(_) => 1,
        Breach::NotCarried { .. } => 2,
        Breach::MomentumUnread { .. } => 3,
        Breach::MomentumNotConserved { .. } => 4,
        Breach::NonRigid { .. } => 5,
        Breach::RigidityUnmeasured { .. } => 6,
        Breach::HeightNotAdvanced { .. } => 7,
        Breach::NotUnitary => 8,
        Breach::MassNotConserved { .. } => 9,
        Breach::FluxUnplaced { .. } => 10,
    }
}

/// ONE BODY'S STATE as the register reads it.
fn state(b: &Body) -> Option<BodyState> {
    Some(BodyState { nucleus: b.shape.blade.vertex_nucleus()?, body: body_of_shape(&b.shape)? })
}

/// THE LAWS ON ONE PAIR: its transitions, built from the perception's own
/// carriages, each verified.
pub fn bridge_pair(x: &Grid, y: &Grid, budget: &mut crate::physics::Budget) -> Bridge {
    let mut read = Bridge::default();
    let (ins, outs) = (perceive(x), perceive(y));
    // Every carriage, grouped by its canonical ray: one motion, one transition.
    let mut motions: crate::network::nerve::Keyed<Vec<(usize, usize, Carriage)>> = crate::network::nerve::Keyed::new();
    for (i, a) in ins.iter().enumerate() {
        read.bodies += 1;
        let mut any = false;
        for (j, b) in outs.iter().enumerate() {
            if a.moduli.key() != b.moduli.key() {
                continue;
            }
            let Some(c) = carriage(a, b, budget) else { continue };
            any = true;
            let Some(ray) = c.v.canonical() else { continue };
            let mut k = Vec::new();
            ray.put_canonical(&mut k);
            match motions.get_mut(&k) {
                Some(v) => v.push((i, j, c)),
                None => {
                    motions.insert(&k, vec![(i, j, c)]);
                }
            }
        }
        match any {
            true => read.carried += 1,
            false => read.uncarried += 1,
        }
    }
    for (_, members) in motions.scoped() {
        let [(_, _, first), ..] = members.as_slice() else { continue };
        let states: Option<Vec<(BodyState, BodyState)>> =
            members.iter().map(|(i, j, _)| Some((state(ins.get(*i)?)?, state(outs.get(*j)?)?))).collect();
        let Some(bodies) = states else {
            read.unbuilt += 1;
            continue;
        };
        // THE HUE RELATION AS THE REGISTER TAKES IT (78.58): the carriages'
        // function on the hues, lifted unitarily — empty where it moves no
        // hue, a merge's distinction rotated into e_∅. A function that sends
        // one hue two ways has no lift and is passed as it is, for the law to
        // refuse by name.
        let mut function: Vec<(u8, u8)> = members.iter().flat_map(|(_, _, c)| c.sigma.iter().copied()).collect();
        function.sort_unstable();
        function.dedup();
        let moves = function.iter().any(|(f, t)| f != t);
        let hues = match (moves, crate::transducer::element::lift_fibre(&function)) {
            (false, _) => Vec::new(),
            (true, Some(lifted)) => lifted,
            (true, None) => function,
        };
        read.transitions += 1;
        let single = bodies.len() == 1;
        read.several += u64::from(!single);
        // The perception's carriages record no source or sink: a carriage
        // moves a body, it neither emits nor absorbs one.
        let t = Transition { motion: first.v.clone(), heights: (0, 1), bodies, hues, fluxes: Vec::new() };
        let v = verify(&t, budget);
        if v.breaches.iter().any(|(law, _)| *law == crate::physics::laws::Law::Momentum) {
            read.momentum_single += u64::from(single);
            let odd = !t.motion.grade(1).is_zero() || !t.motion.grade(3).is_zero();
            read.momentum_odd += u64::from(odd);
            let dilates = crate::transducer::element::conformal_factors(&t.motion).is_ok_and(|f| !num_traits::One::is_one(&f.rho));
            read.momentum_dilating += u64::from(dilates);
            read.momentum_mass_changed += u64::from(t.bodies.iter().any(|(b, a)| b.body.mass != a.body.mass));
            // The momentum each body carried and the momentum it holds after,
            // read as rays: `verify`'s own comparison, at unit weight.
            let b = t.motion.grade(2);
            let w = t.motion.geo(&t.motion.reverse()).scalar_part();
            let ratios: Option<Vec<num_rational::BigRational>> = (!num_traits::Zero::is_zero(&w))
                .then(|| {
                    t.bodies
                        .iter()
                        .map(|(before, after)| {
                            use crate::physics::tensegrity::momentum;
                            let carried = t.motion.sandwich(&momentum(&before.body, &b)?).scale(&(num_rational::BigRational::from_integer(1.into()) / &w));
                            crate::transducer::cga::projective_ratio(&carried, &momentum(&after.body, &b)?)
                        })
                        .collect()
                })
                .flatten();
            if let Some(rs) = ratios {
                read.momentum_projective += 1;
                read.momentum_one_ratio += u64::from(rs.windows(2).all(|p| p[0] == p[1]));
            }
        }
        read.admitted += u64::from(v.admitted());
        let mut laws = [false; crate::physics::laws::LAWS.len()];
        for (law, b) in &v.breaches {
            read.breaches[breach_slot(b)] += 1;
            if let Some(slot) = laws.get_mut(usize::from(law.index())) {
                *slot = true;
            }
        }
        read.by_law.iter_mut().zip(laws).for_each(|(a, b)| *a += u64::from(b));
        match &v.similarity {
            Some(r) if num_traits::One::is_one(r) => read.isometric += 1,
            Some(_) => read.dilating += 1,
            None => {}
        }
    }
    read
}
