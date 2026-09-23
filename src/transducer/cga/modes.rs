// 75, 78.11, MD1 — THE MODES OF A GENERATOR: a bivector split into its
// commuting 2-blades, `m` of them read off the bivector, each with its plane
// and its class, exact over ℚ or refused by name.
//
//! A generator `B ∈ ⋀²ℚ^{n+1,1}` acts on vectors by `F(x) = ⟨x B⟩₁`, a map
//! skew to the metric, and `F²` is self-adjoint. Its SCALAR INVARIANTS AT
//! EVERY EXPONENT — the power sums `tr((F²)ᵏ)` for `k` up to the dimension,
//! gathered into `F²`'s characteristic polynomial by Faddeev–LeVerrier — are
//! what the two-part split read only at the square (`⟨B²⟩₀`, `⟨B²⟩₄`). Every
//! non-zero root `λ` of that polynomial is one mode's value, its eigenspace
//! the mode's plane (or planes, where modes share a value: an isoclinic
//! pair, held whole), and the generalised kernel carries at most one
//! parabolic mode — `Cl(n+1, 1)` has one timelike direction, so at most one
//! mode is not elliptic.
//!
//! EACH MODE IS `B` PROJECTED ONTO ITS OWN PLANES: the eigenspaces are
//! mutually orthogonal, so the outermorphism of the projector onto one
//! carries `B` to exactly its part there, with no cross term to drop. No
//! angle, no root of a mode's value, no approximation.
//!
//! EXACT OR REFUSED (rule 4, rule 11). A factor of the characteristic
//! polynomial with no rational root leaves the split undone over ℚ, and it is
//! refused by name with the factor's degree — never rounded onto a nearby
//! split. Two roots are read exactly (a rational square discriminant); a
//! factor of degree three or more that has not already split is refused by
//! its degree, a named departure until a ruling reads it.

use super::multivector::{one, Mv};
use crate::network::nerve::coord::BivectorKind;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

type Q = BigRational;

/// ONE MODE OF A GENERATOR: the part of `B` on its own planes, its class by
/// the sign of its square, and how many planes it spans — two or more where
/// modes share a value, an isoclinic set held whole (no split among them is
/// chosen, checklist (r)).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mode {
    pub blade: Mv,
    pub kind: BivectorKind,
    pub planes: usize,
}

/// WHY A GENERATOR DID NOT SPLIT OVER ℚ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Undecomposed {
    /// The characteristic polynomial's non-zero part has a factor of this
    /// degree with no rational root: the modes' values lie in an extension.
    Irrational(usize),
    /// The eigenspaces do not span the base: a non-diagonalisable part other
    /// than the one parabolic mode the signature allows.
    Defective,
}

/// The generators of the algebra `B` lives in, as masks in ascending bit
/// order — the base axes, `e₊` and `e₋`, `n + 2` of them — so a product of
/// two in this order is the blade the carrier writes.
fn generators(n: u8) -> Vec<usize> {
    let mut masks: Vec<usize> = (1..=usize::from(n)).map(super::base_axis).collect();
    masks.push(super::EP);
    masks.push(super::EM);
    masks.sort_unstable();
    masks
}

fn column(v: &Mv, masks: &[usize]) -> Vec<Q> {
    masks.iter().map(|&m| v.coeff(m).clone()).collect()
}

fn mul(a: &[Vec<Q>], b: &[Vec<Q>]) -> Vec<Vec<Q>> {
    let d = a.len();
    (0..d).map(|i| (0..d).map(|j| (0..d).fold(Q::zero(), |acc, k| acc + &a[i][k] * &b[k][j])).collect()).collect()
}

fn identity(d: usize) -> Vec<Vec<Q>> {
    (0..d).map(|i| (0..d).map(|j| if i == j { one() } else { Q::zero() }).collect()).collect()
}

/// THE SCALAR INVARIANTS OF `A` AT EVERY EXPONENT: its characteristic
/// polynomial `det(x − A)`, coefficients low to high, by Faddeev–LeVerrier —
/// `cₖ` from `tr(A Mₖ)`, exact over ℚ, no root taken.
fn characteristic(a: &[Vec<Q>]) -> Vec<Q> {
    let d = a.len();
    let mut coeff = vec![Q::zero(); d + 1];
    coeff[d] = one();
    let mut m = vec![vec![Q::zero(); d]; d];
    for k in 1..=d {
        let mut next = mul(a, &m);
        for (i, row) in next.iter_mut().enumerate() {
            row[i] += &coeff[d + 1 - k];
        }
        m = next;
        let am = mul(a, &m);
        let tr = (0..d).fold(Q::zero(), |acc, i| acc + &am[i][i]);
        coeff[d - k] = -tr / Q::from_integer(num_bigint::BigInt::from(k));
    }
    coeff
}

/// The polynomial with its vanishing leading coefficients dropped: its
/// degree, read off the coefficients themselves.
fn trim(mut p: Vec<Q>) -> Vec<Q> {
    while let [_, .., lead] = p.as_slice() {
        if !lead.is_zero() {
            break;
        }
        p.pop();
    }
    p
}

/// `p mod q` over ℚ, `q` non-zero.
fn rem(p: &[Q], q: &[Q]) -> Vec<Q> {
    let mut r = trim(p.to_vec());
    let q = trim(q.to_vec());
    let lead = q[q.len() - 1].clone();
    while r.len() >= q.len() && !(r.len() == 1 && r[0].is_zero()) {
        let f = &r[r.len() - 1] / &lead;
        let shift = r.len() - q.len();
        for (i, c) in q.iter().enumerate() {
            r[shift + i] = &r[shift + i] - &f * c;
        }
        r = trim(r);
        if r.len() < q.len() {
            break;
        }
        if r.len() == q.len() && r[r.len() - 1].is_zero() {
            break;
        }
    }
    r
}

fn gcd(a: &[Q], b: &[Q]) -> Vec<Q> {
    let (mut a, mut b) = (trim(a.to_vec()), trim(b.to_vec()));
    while !(b.len() == 1 && b[0].is_zero()) {
        let r = rem(&a, &b);
        a = b;
        b = r;
    }
    a
}

/// `p / q` exactly, `q` dividing `p`.
fn quotient(p: &[Q], q: &[Q]) -> Vec<Q> {
    let mut r = trim(p.to_vec());
    let q = trim(q.to_vec());
    let lead = q[q.len() - 1].clone();
    let Some(span) = r.len().checked_sub(q.len()) else { return vec![Q::zero()] };
    let mut out = vec![Q::zero(); span + 1];
    while r.len() >= q.len() && !(r.len() == 1 && r[0].is_zero()) {
        let f = &r[r.len() - 1] / &lead;
        let shift = r.len() - q.len();
        out[shift] = f.clone();
        for (i, c) in q.iter().enumerate() {
            r[shift + i] = &r[shift + i] - &f * c;
        }
        r.pop();
        r = trim(r);
    }
    trim(out)
}

/// THE DISTINCT NON-ZERO ROOTS of `p`, exact, or the degree of the factor
/// that has none over ℚ. `p` is taken square-free (`p / gcd(p, p′)`) and
/// stripped of its root at zero first, so each value is read once.
fn nonzero_roots(p: &[Q]) -> Result<Vec<Q>, Undecomposed> {
    let p = trim(p.to_vec());
    let derivative: Vec<Q> = p
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, c)| c * Q::from_integer(num_bigint::BigInt::from(i)))
        .collect();
    let free = quotient(&p, &gcd(&p, &derivative));
    // The root at zero divided out: the leading zero coefficients dropped.
    let q: Vec<Q> = free.iter().skip_while(|c| c.is_zero()).cloned().collect();
    match q.as_slice() {
        [_] => Ok(Vec::new()),
        [c0, c1] => Ok(vec![-c0 / c1]),
        [c0, c1, c2] => {
            let disc = c1 * c1 - Q::from_integer(4.into()) * c2 * c0;
            let s = crate::transducer::alphabet::rational_sqrt(&disc).ok_or(Undecomposed::Irrational(2))?;
            let two_a = Q::from_integer(2.into()) * c2;
            let mut roots = vec![(-c1 - &s) / &two_a, (-c1 + &s) / &two_a];
            roots.sort();
            Ok(roots)
        }
        more => Err(Undecomposed::Irrational(more.len() - 1)),
    }
}

/// `A` as the columns of a basis of `ker(A − λ)`.
fn eigenspace(a: &[Vec<Q>], lambda: &Q) -> Vec<Vec<Q>> {
    let d = a.len();
    let shifted: Vec<Vec<Q>> =
        (0..d).map(|i| (0..d).map(|j| if i == j { &a[i][j] - lambda } else { a[i][j].clone() }).collect()).collect();
    super::nullspace(shifted, d)
}

/// The inverse of an invertible matrix, column by column through the one
/// elimination (`nullspace`).
fn invert(t: &[Vec<Q>]) -> Option<Vec<Vec<Q>>> {
    let d = t.len();
    let mut cols: Vec<Vec<Q>> = Vec::with_capacity(d);
    for j in 0..d {
        let rows: Vec<Vec<Q>> = (0..d)
            .map(|i| {
                let mut row = t[i].clone();
                row.push(if i == j { -one() } else { Q::zero() });
                row
            })
            .collect();
        match super::nullspace(rows, d + 1).as_slice() {
            [v] if v[d] == one() => cols.push(v[..d].to_vec()),
            _ => return None,
        }
    }
    Some((0..d).map(|i| (0..d).map(|j| cols[j][i].clone()).collect()).collect())
}

/// `B` carried by the outermorphism of `P`: every `e_k ∧ e_l` to
/// `P e_k ∧ P e_l`.
fn project(b: &Mv, p: &[Vec<Q>], masks: &[usize]) -> Mv {
    let d = masks.len();
    let image = |k: usize| (0..d).fold(Mv::zero(), |acc, i| acc.add(&Mv::blade(masks[i], p[i][k].clone())));
    let mut out = Mv::zero();
    for k in 0..d {
        for l in (k + 1)..d {
            let c = b.coeff(masks[k] | masks[l]);
            if c.is_zero() {
                continue;
            }
            out = out.add(&image(k).geo(&image(l)).grade(2).scale(c));
        }
    }
    out
}

fn kind_of(blade: &Mv) -> BivectorKind {
    let s = blade.geo(blade).scalar_part();
    match (blade.is_zero(), s.is_negative(), s.is_positive()) {
        (true, _, _) => BivectorKind::Trivial,
        (false, true, _) => BivectorKind::Elliptic,
        (false, _, true) => BivectorKind::Hyperbolic,
        _ => BivectorKind::Parabolic,
    }
}

/// A MODE'S OWN CLOSURE UNDER A MOTION (75, 78.11 link 3): the least order
/// `ℓ` of the algebra's cyclotomic set at which `motion^ℓ` fixes every
/// vector of the mode's plane — the plane read as the image of the mode's
/// own action, `⟨e_k B_j⟩₁`. Cayley is not multiplicative over commuting
/// parts, so a mode's closure is read on the motion's own action and never
/// off `cayley(B_j)`. `None` for a parabolic or hyperbolic mode, open by
/// name, and for an elliptic mode that closes at no order the algebra
/// admits: a rosette.
pub fn mode_order(motion: &Mv, mode: &Mode) -> Option<u32> {
    if mode.kind != BivectorKind::Elliptic {
        return None;
    }
    let n = motion.n().max(mode.blade.n());
    let plane: Vec<Mv> = generators(n)
        .into_iter()
        .map(|m| Mv::blade(m, one()).geo(&mode.blade).grade(1))
        .filter(|v| !v.is_zero())
        .collect();
    let inverse = motion.versor_inverse()?;
    let orders = super::cyclotomic_orders(n);
    let bound = orders.iter().copied().fold(1, u32::max);
    let (mut acc, mut acc_inv) = (motion.clone(), inverse.clone());
    for k in 1..=bound {
        if orders.contains(&k) && plane.iter().all(|v| acc.geo(v).geo(&acc_inv).sub(v).is_zero()) {
            return Some(k);
        }
        acc = acc.geo(motion);
        acc_inv = inverse.geo(&acc_inv);
    }
    None
}

/// THE MODES OF `b`'s bivector part (MD1). Each mode's blade, class and
/// planes; the modes commute and re-sum to `B`, verified before they are
/// returned. The generalised kernel's part is listed first where it is not
/// zero (at most one parabolic mode), then the elliptic and hyperbolic modes
/// by their value, ascending. An empty list is the zero generator.
pub fn modes(b: &Mv) -> Result<Vec<Mode>, Undecomposed> {
    let bivector = b.grade(2);
    if bivector.is_zero() {
        return Ok(Vec::new());
    }
    let masks = generators(bivector.n());
    let d = masks.len();
    let f: Vec<Vec<Q>> = {
        let cols: Vec<Vec<Q>> =
            masks.iter().map(|&m| column(&Mv::blade(m, one()).geo(&bivector).grade(1), &masks)).collect();
        (0..d).map(|i| (0..d).map(|j| cols[j][i].clone()).collect()).collect()
    };
    let a = mul(&f, &f);
    let values = nonzero_roots(&characteristic(&a))?;
    // The generalised kernel: `ker(Aᵈ)`.
    let mut power = identity(d);
    for _ in 0..d {
        power = mul(&power, &a);
    }
    let kernel = super::nullspace(power, d);
    let spaces: Vec<(Option<Q>, Vec<Vec<Q>>)> = std::iter::once((None, kernel))
        .chain(values.iter().map(|l| (Some(l.clone()), eigenspace(&a, l))))
        .collect();
    let columns: Vec<Vec<Q>> = spaces.iter().flat_map(|(_, s)| s.iter().cloned()).collect();
    if columns.len() != d {
        return Err(Undecomposed::Defective);
    }
    let t: Vec<Vec<Q>> = (0..d).map(|i| columns.iter().map(|c| c[i].clone()).collect()).collect();
    let t_inv = invert(&t).ok_or(Undecomposed::Defective)?;
    let mut out: Vec<Mode> = Vec::new();
    let mut at = 0usize;
    for (value, space) in &spaces {
        let width = space.len();
        let indicator: Vec<Vec<Q>> = (0..d)
            .map(|i| (0..d).map(|j| if i == j && i >= at && i < at + width { one() } else { Q::zero() }).collect())
            .collect();
        at += width;
        let p = mul(&mul(&t, &indicator), &t_inv);
        let blade = project(&bivector, &p, &masks);
        if blade.is_zero() {
            continue;
        }
        // Planes the mode spans: its eigenspace's dimension halved, or one
        // for the parabolic part of the kernel.
        let planes = match value {
            Some(_) => width / 2,
            None => 1,
        };
        out.push(Mode { kind: kind_of(&blade), blade, planes });
    }
    // VERIFIED AGAINST THE ALGEBRA, never assumed: a one-plane mode is
    // simple, the modes re-sum to `B`, and they commute pairwise.
    if out.iter().any(|m| m.planes == 1 && !m.blade.geo(&m.blade).grade(4).is_zero()) {
        return Err(Undecomposed::Defective);
    }
    let sum = out.iter().fold(Mv::zero(), |acc, m| acc.add(&m.blade));
    if !sum.sub(&bivector).is_zero() {
        return Err(Undecomposed::Defective);
    }
    for (i, x) in out.iter().enumerate() {
        for y in out.iter().skip(i + 1) {
            if !x.blade.geo(&y.blade).sub(&y.blade.geo(&x.blade)).is_zero() {
                return Err(Undecomposed::Defective);
            }
        }
    }
    Ok(out)
}
