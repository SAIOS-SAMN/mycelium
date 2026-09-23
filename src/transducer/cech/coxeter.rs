// 78.14, 78.15, GR2, CX1, SY1 — A SET OF CONSTRUCTORS READ AS A REFLECTION
// GROUP: its Coxeter system recognised where it is one, its closure read by
// the theorem or refused with certainty, the worklist kept as the check; and
// the product of the constructors in each circuit's order, read as a motion.
//
//! RECOGNITION (GR2). The constructors fall into components by their
//! non-orthogonal pairs. A component is a COXETER SYSTEM where (i) its
//! Gram form's off-diagonals can all be made `≤ 0` by signs on the
//! constructors — an exact sign balance along the component's pairs,
//! unique up to one global sign, no choice — (ii) its constructors are
//! linearly independent, (iii) every pair is crystallographic or at
//! infinity (`δ² ∈ {0, 1/4, 1/2, 3/4}` or `δ² ≥ 1`), and (iv) every
//! constructor is a real mirror (`s² > 0`). A recognised component is FINITE
//! where its span is definite, AFFINE where it is positive semidefinite and
//! degenerate, HYPERBOLIC otherwise — Coxeter's theorem, no iteration. `A₂`'s
//! three roots are not a system (dependent): the check reads them.
//!
//! CLOSURE. Certain refusals first: a rosette or a boost pair (GR1), an
//! affine or hyperbolic recognised component, and — CX1 — a product of the
//! constructors of infinite order (every element of a finite group has
//! finite order). What neither refuses is FINITE by the theorem where every
//! component is recognised, and otherwise CERTIFIED by the worklist: the
//! group's elements closed on a keyed carrier under the stroke's budget,
//! counted as rays; the worklist is never the reading.
//!
//! THE PRODUCT PER CIRCUIT (CX1). `P = s_π(k) ⋯ s_π(0)` in the circuit's
//! order: its grades named (the top the meet, the lower the Gram entries on
//! constructors), its weight `P P̃ = Π s_a²`, read as a motion by the action
//! reader on `Pin` (MR1b, `det` named). It is the COXETER ELEMENT, with its
//! exponents and Coxeter number, only where the set is a recognised system.
//! SY1: the sealing step splits into what it adds, `s_last ∧ P_{k−1}` —
//! the blade `ascend` deposits — and what it shares, `s_last ⌋ P_{k−1}`.

use crate::network::nerve::coord::BivectorKind;
use crate::network::nerve::Keyed;
use crate::transducer::cga::{self, MotionMode, Mv, PairRegime, Undecomposed};
use num_rational::BigRational;
use num_traits::{Signed, Zero};

type Q = BigRational;

/// ONE COMPONENT'S CLASS.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentClass {
    Finite,
    Affine,
    Hyperbolic,
    /// Not a Coxeter system: signs that do not balance, dependent
    /// constructors, a non-crystallographic pair, or an imaginary mirror.
    Unrecognized,
}

/// WHY A GROUP IS INFINITE, certainly.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// A pair whose product has no finite order (GR1).
    Pair(usize, usize, PairRegime),
    /// A recognised component whose span is degenerate.
    Affine(Vec<usize>),
    /// A recognised component whose span is indefinite.
    Hyperbolic(Vec<usize>),
    /// A circuit's product of infinite order, by the class of the mode that
    /// never returns (CX1).
    Product { circuit: usize, kind: BivectorKind },
}

/// THE GROUP'S CLOSURE.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Closure {
    /// Finite: by the theorem where every component is recognised; the
    /// group's order where the worklist read it within the stroke.
    Finite { by_theorem: bool, order: Option<u64> },
    Infinite(Refusal),
    /// The worklist certifying a non-system ran out of stroke at this many
    /// elements (rule 10).
    Unbounded { elements: u64 },
    /// A null constructor: it reflects nothing.
    Unread,
}

/// A SET OF CONSTRUCTORS READ AS A REFLECTION GROUP (GR2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoxeterReading {
    pub components: Vec<(Vec<usize>, ComponentClass)>,
    pub closure: Closure,
}

/// The components of the constructors under their non-orthogonal pairs, each
/// ascending, in the order of their least member.
fn components(g: &[Vec<Q>]) -> Vec<Vec<usize>> {
    let d = g.len();
    let mut seen = vec![false; d];
    let mut out = Vec::new();
    for start in 0..d {
        if seen[start] {
            continue;
        }
        let mut comp = vec![start];
        seen[start] = true;
        let mut at = 0;
        while at < comp.len() {
            let a = comp[at];
            for b in 0..d {
                if !seen[b] && !g[a][b].is_zero() {
                    seen[b] = true;
                    comp.push(b);
                }
            }
            at += 1;
        }
        comp.sort_unstable();
        out.push(comp);
    }
    out
}

/// THE SIGN BALANCE on one component: `ε_a ε_b = −sign(G_ab)` along every
/// non-zero pair, propagated from the least member; `None` where a cycle
/// disagrees. Unique up to the one global sign.
fn sign_balance(g: &[Vec<Q>], comp: &[usize]) -> Option<Vec<i8>> {
    let mut sign: Keyed<i8> = Keyed::new();
    let key = |a: usize| (a as u32).to_be_bytes();
    let [first, ..] = comp else { return Some(Vec::new()) };
    sign.insert(key(*first), 1);
    let mut work = vec![*first];
    while let Some(a) = work.pop() {
        let Some(&sa) = sign.get(&key(a)) else { continue };
        for &b in comp {
            if b == a || g[a][b].is_zero() {
                continue;
            }
            let want = match g[a][b].is_positive() {
                true => -sa,
                false => sa,
            };
            match sign.get(&key(b)) {
                Some(&sb) if sb != want => return None,
                Some(_) => {}
                None => {
                    sign.insert(key(b), want);
                    work.push(b);
                }
            }
        }
    }
    comp.iter().map(|&a| sign.get(&key(a)).copied()).collect()
}

fn class_of(constructors: &[Mv], g: &[Vec<Q>], comp: &[usize]) -> ComponentClass {
    let sub: Vec<Vec<Q>> = comp.iter().map(|&a| comp.iter().map(|&b| g[a][b].clone()).collect()).collect();
    let vs: Vec<Mv> = comp.iter().map(|&a| constructors[a].clone()).collect();
    let real = comp.iter().all(|&a| g[a][a].is_positive());
    let pairs_ok = comp.iter().enumerate().all(|(i, &a)| {
        comp.iter().skip(i + 1).all(|&b| {
            !matches!(cga::pair_regime(g, a, b).1, PairRegime::Rosette | PairRegime::Null)
                && !cga::pair_regime(g, a, b).0.is_negative()
        })
    });
    let independent = cga::vector_rank(&vs) == comp.len();
    if !(real && pairs_ok && independent) || sign_balance(g, comp).is_none() {
        return ComponentClass::Unrecognized;
    }
    // Signs are a congruence by `diag(±1)`: the signature is the Gram form's.
    let (p, q, z) = cga::signature(&sub);
    match (q, z) {
        (0, 0) if p == comp.len() => ComponentClass::Finite,
        (0, _) => ComponentClass::Affine,
        _ => ComponentClass::Hyperbolic,
    }
}

/// THE GROUP THE CONSTRUCTORS' REFLECTIONS GENERATE, its elements closed on a
/// keyed carrier and counted as rays, one read of the stroke per product:
/// the CHECK, never the reading. `Err` with the count where the stroke is
/// spent.
pub fn group_order(constructors: &[Mv], budget: &mut crate::physics::Budget) -> Result<u64, u64> {
    let key = |w: &Mv| -> Vec<u8> {
        let mut k = Vec::new();
        w.put_canonical(&mut k);
        k
    };
    let mut held: Keyed<()> = Keyed::new();
    let one = Mv::scalar(Q::from_integer(1.into()));
    held.insert(key(&one), ());
    let mut work = vec![one];
    while let Some(w) = work.pop() {
        for s in constructors {
            if !budget.spend() {
                return Err(held.len() as u64);
            }
            let next = w.geo(s);
            let k = key(&next);
            if !held.contains(&k) {
                held.insert(&k, ());
                work.push(next);
            }
        }
    }
    Ok(held.len() as u64)
}

/// READ A SET OF CONSTRUCTORS AS A REFLECTION GROUP (GR2), with the
/// circuits' products' certain refusals (CX1) in `products`.
pub fn read_coxeter(
    constructors: &[Mv],
    products: &[CoxeterProduct],
    budget: &mut crate::physics::Budget,
) -> CoxeterReading {
    let g = cga::gram_form(constructors);
    if (0..g.len()).any(|a| g[a][a].is_zero()) {
        return CoxeterReading { components: Vec::new(), closure: Closure::Unread };
    }
    let comps: Vec<(Vec<usize>, ComponentClass)> =
        components(&g).into_iter().map(|c| { let k = class_of(constructors, &g, &c); (c, k) }).collect();
    let pair_refusal = (0..g.len())
        .flat_map(|a| ((a + 1)..g.len()).map(move |b| (a, b)))
        .map(|(a, b)| (a, b, cga::pair_regime(&g, a, b).1))
        .find(|(_, _, r)| matches!(r, PairRegime::Rosette | PairRegime::Boost | PairRegime::Parabolic));
    let refusal = comps
        .iter()
        .find_map(|(c, k)| match k {
            ComponentClass::Affine => Some(Refusal::Affine(c.clone())),
            ComponentClass::Hyperbolic => Some(Refusal::Hyperbolic(c.clone())),
            _ => None,
        })
        .or_else(|| pair_refusal.map(|(a, b, r)| Refusal::Pair(a, b, r)))
        .or_else(|| {
            products.iter().enumerate().find_map(|(i, p)| p.infinite.map(|kind| Refusal::Product { circuit: i, kind }))
        });
    let closure = match refusal {
        Some(r) => Closure::Infinite(r),
        None => {
            let by_theorem = comps.iter().all(|(_, k)| *k == ComponentClass::Finite);
            match (group_order(constructors, budget), by_theorem) {
                (Ok(order), _) => Closure::Finite { by_theorem, order: Some(order) },
                (Err(_), true) => Closure::Finite { by_theorem, order: None },
                (Err(elements), false) => Closure::Unbounded { elements },
            }
        }
    };
    CoxeterReading { components: comps, closure }
}

/// THE PRODUCT OF A CELL'S CONSTRUCTORS IN ONE CIRCUIT'S ORDER (CX1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoxeterProduct {
    /// The circuit's order of the cell's constructors.
    pub order: Vec<usize>,
    pub product: Mv,
    /// The grades the product holds: the top is the meet, the lower the Gram
    /// entries placed on constructors.
    pub grades: Vec<u32>,
    /// `P P̃`, and whether it is `Π s_a²` — each sphere's own scalar.
    pub weight: Q,
    pub weight_is_the_spheres: bool,
    /// The action's determinant (MR1b).
    pub det: Option<i8>,
    /// Its modes by the action reader; `None` where not a versor.
    pub modes: Option<Result<Vec<MotionMode>, Undecomposed>>,
    /// The class of a mode that never returns: a certain refusal of
    /// finiteness (CX1). `None` where every mode closes or none read.
    pub infinite: Option<BivectorKind>,
    /// SY1 — the sealing step: what it adds (`s_last ∧ P_{k−1}`, the blade
    /// `ascend` deposits) and what it shares (`s_last ⌋ P_{k−1}`).
    pub seal: Mv,
    pub shared: Mv,
}

/// `s ∧ P` and `s ⌋ P` for a vector `s` and any `P`, grade by grade.
fn outer_inner(s: &Mv, p: &Mv) -> (Mv, Mv) {
    let top = u32::from(p.n().max(s.n())) + 2;
    (0..=top).fold((Mv::zero(), Mv::zero()), |(up, down), g| {
        let part = s.geo(&p.grade(g));
        let down = match g {
            0 => down,
            _ => down.add(&part.grade(g - 1)),
        };
        (up.add(&part.grade(g + 1)), down)
    })
}

/// THE PRODUCT OF `constructors` IN `order` (CX1), built step by step. Its
/// modes are read only where `read_modes`: where the group is already
/// refused with certainty (GR1's pair regime names a tether's motion), the
/// reading would spend the stroke on what is known, and `modes` stays
/// `None` — not read, by name.
pub fn coxeter_product(
    constructors: &[Mv],
    order: &[usize],
    read_modes: bool,
    budget: &mut crate::physics::Budget,
) -> Option<CoxeterProduct> {
    let [first, rest @ ..] = order else { return None };
    let mut p = constructors.get(*first)?.clone();
    let (mut seal, mut shared) = (p.clone(), Mv::zero());
    for &a in rest {
        let s = constructors.get(a)?;
        let (up, down) = outer_inner(s, &p);
        seal = up;
        shared = down;
        p = s.geo(&p);
    }
    let top = u32::from(p.n()) + 2;
    let grades: Vec<u32> = (0..=top).filter(|&g| !p.grade(g).is_zero()).collect();
    let weight = p.geo(&p.reverse()).scalar_part();
    let spheres = order.iter().fold(Q::from_integer(1.into()), |acc, &a| acc * constructors[a].geo(&constructors[a]).scalar_part());
    let modes = match read_modes {
        true => cga::motion_modes(&p, budget),
        false => None,
    };
    let infinite = match &modes {
        Some(Ok(ms)) => ms.iter().find(|m| m.kind != BivectorKind::Elliptic || m.order.is_none()).map(|m| m.kind),
        _ => None,
    };
    Some(CoxeterProduct {
        order: order.to_vec(),
        det: cga::action_det(&p),
        weight_is_the_spheres: weight == spheres,
        product: p,
        grades,
        weight,
        modes,
        infinite,
        seal,
        shared,
    })
}

/// THE CLASS OF A PRODUCT, as a key: its determinant and its modes' `(class,
/// planes, c, reflects)` in the reader's order — equal keys, conjugate
/// actions (C2a). `None` where the modes did not read.
pub fn product_class(p: &CoxeterProduct) -> Option<Vec<u8>> {
    let Some(Ok(ms)) = &p.modes else { return None };
    let mut k = vec![p.det? as u8];
    for m in ms {
        k.push(m.kind.as_u8());
        k.extend_from_slice(&(m.planes as u32).to_le_bytes());
        k.push(u8::from(m.reflects));
        crate::transducer::address::put_bytes(&mut k, &m.c.numer().to_signed_bytes_le());
        crate::transducer::address::put_bytes(&mut k, &m.c.denom().to_signed_bytes_le());
    }
    Some(k)
}
