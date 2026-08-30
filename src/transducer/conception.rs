// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! NOVEL CONCEPTION — the germination pipeline. A solved conformal motor is NOT
//! dispatched down a fixed branch; it is a PROSPECTIVE GERM. Given a demand
//! `A → B`, the unified `solve_conformal_motor` returns the single motor from the
//! geometry (or None — an irreducible residual, a PIVOT that demands a new
//! fiber). If the motor is already an installed primitive it is recognised
//! (recall). Otherwise it emits as an unadmitted germ carrying its exact residual
//! reduction: because the solve verified topological closure, the demand is
//! annihilated (`Δ_new = 0 ≺_DM Q`) — a strict descent. A germ installs to the
//! basis and broadcasts for peer consilience ONLY when the two-voice replay gate
//! confirms it (Law: two independent mouths, never a single tip).

use crate::transducer::blade::Blade;
use crate::transducer::motor::{solve_conformal_motor, ConformalMotor};

/// The unified recogniser: the single geometric solve, no per-operation branch.
pub fn recognize(a: &Blade, b: &Blade) -> Option<ConformalMotor> {
    solve_conformal_motor(a, b)
}

/// Recognise a whole scene `a → b` as ONE coherent motor. Bodies correspond by
/// their rigid-motion invariant (zero search); the motor from the first pair
/// must reproduce every corresponded pair. `None` ⇒ no single motor closes it —
/// the residual is real (a pivot / germ candidate at the ensemble level).
pub fn recognize_scene(a: &[Blade], b: &[Blade]) -> Option<ConformalMotor> {
    if a.is_empty() || a.len() != b.len() {
        return None;
    }
    let mut ai: Vec<usize> = (0..a.len()).collect();
    let mut bi: Vec<usize> = (0..b.len()).collect();
    ai.sort_by(|&x, &y| a[x].invariant().cmp(&a[y].invariant()));
    bi.sort_by(|&x, &y| b[x].invariant().cmp(&b[y].invariant()));
    for k in 0..a.len() {
        if a[ai[k]].invariant() != b[bi[k]].invariant() {
            return None;
        }
    }
    let m = recognize(&a[ai[0]], &b[bi[0]])?;
    for k in 1..a.len() {
        match recognize(&a[ai[k]], &b[bi[k]]) {
            Some(mk) if mk == m => {}
            _ => return None, // not one coherent motor
        }
    }
    Some(m)
}

/// A prospective germ: a novel motor with its descent proof, awaiting admission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GermCandidate {
    pub motor: ConformalMotor,
    /// The solve verified closure, so the demand reduces to zero: Δ_new ≺_DM Q.
    pub descends: bool,
}

/// The verdict of conceiving a demand against the installed basis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Conception {
    /// The motor is already an installed primitive (index) — recall, no germ.
    Recognised(usize),
    /// A novel motor — a germ candidate, unadmitted until two-voice consilience.
    Germ(GermCandidate),
    /// No motor closes the demand — an irreducible residual: a PIVOT that demands
    /// a new fiber/axis (germination of a new dimension, not a wrong answer).
    Pivot,
}

/// CONCEIVE — route a demand through the germination mechanism against the basis
/// of installed primitive motors. No hardcoded dispatch: the solve's output is
/// classified as recall / germ / pivot.
pub fn conceive(a: &Blade, b: &Blade, basis: &[ConformalMotor]) -> Conception {
    match solve_conformal_motor(a, b) {
        None => Conception::Pivot,
        Some(m) => match basis.iter().position(|p| *p == m) {
            Some(i) => Conception::Recognised(i),
            None => Conception::Germ(GermCandidate { motor: m, descends: true }),
        },
    }
}

/// The TWO-VOICE ADMISSION gate. A germ installs to the basis and broadcasts
/// only when a SECOND independent tip has derived the same germ (consilience) and
/// the residual descends. `voices` counts distinct independent derivations of
/// this germ. (The wire broadcast lands with the network layer; this is the
/// local admission law.)
pub fn admit(candidate: &GermCandidate, voices: usize) -> bool {
    candidate.descends && voices >= 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::blade::{lift, transduce};

    fn a() -> Blade {
        lift(&[(0, 0, 1), (0, 1, 1), (1, 0, 1)], 0)
    }

    /// A novel motor conceives as a germ (unadmitted); one voice cannot admit it,
    /// two independent voices can.
    #[test]
    fn a_novel_motor_conceives_as_a_germ_admitted_by_two_voices() {
        let b = lift(&[(0, 3, 1), (0, 4, 1), (1, 3, 1)], 1); // +0,+3 translate
        match conceive(&a(), &b, &[]) {
            Conception::Germ(g) => {
                assert!(g.descends);
                assert!(!admit(&g, 1), "one tip cannot install a germ");
                assert!(admit(&g, 2), "two independent tips consilience it");
            }
            other => panic!("expected a germ, got {other:?}"),
        }
    }

    /// Once the motor is in the basis, the same demand is recall, not a new germ.
    #[test]
    fn an_installed_motor_is_recalled() {
        let b = lift(&[(0, 3, 1), (0, 4, 1), (1, 3, 1)], 1);
        let m = recognize(&a(), &b).expect("solves");
        assert_eq!(conceive(&a(), &b, &[m]), Conception::Recognised(0));
    }

    /// An irreducible residual conceives as a PIVOT — the demand for a new fiber.
    #[test]
    fn an_irreducible_residual_is_a_pivot() {
        let b = lift(&[(0, 0, 1), (0, 1, 1), (0, 2, 1)], 1); // L → bar: not a motor
        assert_eq!(conceive(&a(), &b, &[]), Conception::Pivot);
    }

    /// A coherent scene motion is recognised as one motor.
    #[test]
    fn a_coherent_scene_is_one_motor() {
        let sa = transduce(&[(0, 0, 1), (5, 5, 2)], 0);
        let sb = transduce(&[(2, 3, 1), (7, 8, 2)], 1);
        assert!(recognize_scene(&sa, &sb).is_some());
    }
}
