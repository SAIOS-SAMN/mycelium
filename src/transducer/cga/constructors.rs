// 78.12(3), CS1 — THE CONSTRUCTORS ARE THE FOUNDATION: a constructor is a
// reflection, a generator is a product of constructors, a generator carries a
// constructor to a constructor, and the orbit of the constructors under the
// generators they build closes or stays open by the mode that carries one off.
//
//! A CONSTRUCTOR is a rational vector `v` with `v² ∈ ℚ*` — a nucleus, a
//! section's sphere, a mirror (R1). A GENERATOR is a product of them
//! (`versor_of_reflections`, Cartan–Dieudonné; R2): the tether `V = s_b s_a`
//! is two, a circuit's holonomy the product of its members' constructors in
//! the circuit's order. Nothing is parameterised.
//!
//! GENERATORS BUILD CONSTRUCTORS: `Ω v Ω⁻¹` is a vector again with the same
//! square — a nucleus carried round a circuit is a nucleus — so the ORBIT of
//! a set of constructors under a set of generators is a set of constructors,
//! read as rays. It CLOSES when that set is closed under its generators — a
//! finite orbit — and stays OPEN BY NAME where a mode of a generator that
//! never closes (parabolic, hyperbolic, or a turn of no admitted order)
//! moves a constructor: that mode carries it off and the orbit never
//! returns. The closure is a keyed worklist under the stroke's budget, never
//! a search; a spent budget refuses by name.

use super::motion::motion_modes;
use super::multivector::Mv;
use crate::network::nerve::coord::BivectorKind;
use crate::network::nerve::Keyed;
use num_traits::Zero;

/// Is `v` a constructor: a pure vector whose square is a non-zero rational.
pub fn is_constructor(v: &Mv) -> bool {
    v.sub(&v.grade(1)).is_zero() && !v.geo(v).scalar_part().is_zero()
}

/// `Ω v Ω⁻¹`: the constructor `v` carried by the generator `Ω` (R2). `None`
/// where `Ω` does not invert or `v` is not a constructor.
pub fn carry(generator: &Mv, v: &Mv) -> Option<Mv> {
    if !is_constructor(v) {
        return None;
    }
    let carried = generator.geo(v).geo(&generator.versor_inverse()?);
    is_constructor(&carried).then_some(carried)
}

/// WHAT THE ORBIT OF A SET OF CONSTRUCTORS UNDER A SET OF GENERATORS READ.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstructorOrbit {
    /// Closed under the generators: the distinct constructor rays it holds.
    Closed { rays: usize },
    /// A mode of generator `generator` that never closes moves constructor
    /// `constructor`: it is carried off, by a mode of this class.
    Open { generator: usize, constructor: usize, kind: BivectorKind },
    /// The stroke's budget was spent before the orbit closed: the rays held
    /// when it was (rule 10).
    Unbounded { rays: usize },
    /// A generator is not a versor or a constructor is not a reflection.
    Unread,
}

fn ray_key(v: &Mv) -> Option<Vec<u8>> {
    let mut key = Vec::new();
    v.canonical()?.put_canonical(&mut key);
    Some(key)
}

/// THE ORBIT OF `constructors` UNDER `generators` (CS1). First the modes: a
/// generator's mode that never closes and moves a constructor opens the
/// orbit by name. Then the closure: every constructor carried by every
/// generator, new rays filed by their own coordinates and carried in turn,
/// one read of the stroke's budget per carrying.
pub fn constructor_orbit(
    constructors: &[Mv],
    generators: &[Mv],
    budget: &mut crate::physics::Budget,
) -> ConstructorOrbit {
    if !constructors.iter().all(is_constructor) || generators.iter().any(|g| super::is_versor(g).is_none()) {
        return ConstructorOrbit::Unread;
    }
    for (gi, g) in generators.iter().enumerate() {
        let Some(Ok(modes)) = motion_modes(g, budget) else { continue };
        for m in modes.iter().filter(|m| m.order.is_none()) {
            for (ci, v) in constructors.iter().enumerate() {
                if carry(&m.ray, v).is_some_and(|w| w != *v) {
                    return ConstructorOrbit::Open { generator: gi, constructor: ci, kind: m.kind };
                }
            }
        }
    }
    let mut held: Keyed<()> = Keyed::new();
    let mut work: Vec<Mv> = Vec::new();
    for v in constructors {
        let Some(k) = ray_key(v) else { return ConstructorOrbit::Unread };
        if !held.contains(&k) {
            held.insert(&k, ());
            work.push(v.clone());
        }
    }
    while let Some(v) = work.pop() {
        for g in generators {
            if !budget.spend() {
                return ConstructorOrbit::Unbounded { rays: held.len() };
            }
            let Some(w) = carry(g, &v) else { return ConstructorOrbit::Unread };
            let Some(k) = ray_key(&w) else { return ConstructorOrbit::Unread };
            if !held.contains(&k) {
                held.insert(&k, ());
                work.push(w);
            }
        }
    }
    ConstructorOrbit::Closed { rays: held.len() }
}
