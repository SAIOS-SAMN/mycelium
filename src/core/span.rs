// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 16.18, 71.2(6), 78.11 — THE SPAN OF A SET OF VECTORS AS THE BLADE IT IS:
//! `A = a₁ ∧ ⋯ ∧ a_k`, held by its factors, its grade the dimension read.
//!
//! No matrix and no rank of one: a vector JOINS the span exactly where its
//! wedge with the blade is non-zero, `v ∧ A ≠ 0`, and then the blade's grade
//! rises by one; where `v ∧ A = 0` the vector lies in the span and the blade
//! is unchanged. What decides it is the part of `v` outside the span — `v`
//! with every factor's component taken out — which is zero exactly when the
//! wedge is. That part is also what the vector adds: the new factor. So the
//! blade is carried as the factors it was built from, one standing at each
//! coordinate a factor leads on, which is the only form in which a wedge of
//! vectors in a space of any dimension stays exact and costs no more than
//! the vectors themselves (the wedge's `C(n, k)` components are never
//! written out).
//!
//! Every factor is a RAY: an exact rational vector read up to scale, carried
//! as the primitive integer vector on it (its entries' gcd divided out), so a
//! vector and its double are one direction and nothing is rounded (law 4).
//! The span turns with what it spans: no basis of the space is consulted, no
//! coordinate grid is laid over it, and the grade is the same whatever order
//! the vectors join in.

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};

/// A VECTOR OF THE SPACE, SPARSE: its non-zero components as `(axis,
/// value)`, axes strictly ascending, exact integers of any size — a ray's
/// primitive representative, or any integer multiple of it (STD1, A.6: a
/// dimension is paid for, never declared).
pub type Vector = Vec<(usize, BigInt)>;

/// THE SPAN OF THE VECTORS JOINED SO FAR, AS A BLADE BY ITS FACTORS: at each
/// axis a factor leads on, that factor (its leading component beside it).
/// Empty is the scalar blade, grade zero.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Span {
    factors: Vec<Option<(BigInt, Vector)>>,
    grade: usize,
}

impl Span {
    /// The scalar blade: nothing spanned.
    pub fn new() -> Span {
        Span::default()
    }

    /// The span of `vectors`: each joined in turn.
    pub fn of(vectors: &[Vector]) -> Span {
        let mut span = Span::new();
        for v in vectors {
            span.join(v);
        }
        span
    }

    /// The blade's grade: the dimension of what it spans.
    pub fn grade(&self) -> usize {
        self.grade
    }

    /// THE PART OF `v` OUTSIDE THE SPAN, as a ray: `v` with each factor's
    /// component on its leading axis taken out, in the factors' own order.
    /// Empty exactly where `v ∧ A = 0` — `v` lies in the span.
    pub fn outside(&self, v: &Vector) -> Vector {
        let mut r: Vector = v.iter().filter(|(_, x)| !x.is_zero()).cloned().collect();
        while let [(lead, a), ..] = r.as_slice() {
            let Some(Some((b, f))) = self.factors.get(*lead) else {
                break;
            };
            r = cancel(&r, a.clone(), b, f);
        }
        r
    }

    /// Whether `v` lies in the span: `v ∧ A = 0`.
    pub fn holds(&self, v: &Vector) -> bool {
        self.outside(v).is_empty()
    }

    /// JOIN `v` TO THE SPAN: `A ← A ∧ v`. Where `v ∧ A ≠ 0` the part of `v`
    /// outside the span becomes a factor and the grade rises by one — and
    /// that is returned `true`; where `v` lies in the span nothing changes,
    /// `false`.
    pub fn join(&mut self, v: &Vector) -> bool {
        let mut r: Vector = v.iter().filter(|(_, x)| !x.is_zero()).cloned().collect();
        while let [(lead, a), ..] = r.as_slice() {
            let (lead, a) = (*lead, a.clone());
            if lead >= self.factors.len() {
                self.factors.resize(lead + 1, None);
            }
            let Some((b, f)) = self.factors[lead].as_ref() else {
                self.factors[lead] = Some((a, r));
                self.grade += 1;
                return true;
            };
            r = cancel(&r, a, b, f);
        }
        false
    }
}

/// The grade of the span of `vectors`: its dimension.
pub fn grade_of(vectors: &[Vector]) -> usize {
    Span::of(vectors).grade()
}

/// `b·r − a·f`, merged by axis: `r`'s component along the factor `f` on
/// their common leading axis cancels (`a` and `b` their leading values), and
/// the ray's content is divided out, so the entries stay the integers they
/// are.
fn cancel(r: &Vector, a: BigInt, b: &BigInt, f: &Vector) -> Vector {
    let mut next: Vector = Vec::with_capacity(r.len() + f.len());
    let (mut i, mut j) = (0usize, 0usize);
    while i < r.len() || j < f.len() {
        let (ci, cj) = (r.get(i).map(|e| e.0), f.get(j).map(|e| e.0));
        let (axis, v) = match (ci, cj) {
            (Some(x), Some(y)) if x == y => {
                let v = &r[i].1 * b - &f[j].1 * &a;
                i += 1;
                j += 1;
                (x, v)
            }
            (Some(x), Some(y)) if x < y => {
                let v = &r[i].1 * b;
                i += 1;
                (x, v)
            }
            (Some(x), None) => {
                let v = &r[i].1 * b;
                i += 1;
                (x, v)
            }
            (_, Some(y)) => {
                let v = -(&f[j].1 * &a);
                j += 1;
                (y, v)
            }
            (None, None) => break,
        };
        if !v.is_zero() {
            next.push((axis, v));
        }
    }
    let g = next.iter().fold(BigInt::zero(), |g, (_, v)| g.gcd(v));
    if g > BigInt::one() {
        for e in next.iter_mut() {
            e.1 = &e.1 / &g;
        }
    }
    next
}

/// A dense integer vector as the sparse one it is.
pub fn vector_of<const N: usize>(v: &[i64; N]) -> Vector {
    v.iter().enumerate().filter(|(_, x)| **x != 0).map(|(i, x)| (i, BigInt::from(*x))).collect()
}
