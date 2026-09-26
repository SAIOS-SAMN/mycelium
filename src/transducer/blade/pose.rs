// 72.1c, 51.4, A.11 on n axes — THE RELATIVE POSE (FR1): the versor
// carrying one body's tube onto another's, read from the moment tower on the
// body's own `n` axes, over every plane the two tubes occupy.
//
//! A body is its tube, not its slice (72.1c(1)). Where both bodies stand in
//! one section their relative orientation is a turn in the sensor's plane,
//! and [`relative_rotor`] reads it there with the square base's turns, as it
//! did. Where either extends along an axis off that plane — the succession
//! axis `e₃` — the pose is read here, on every axis the base has, and `θ` is
//! whatever versor carries the one tube onto the other: a turn in `e₁₃`, in
//! `e₂₃`, or in several planes at once.
//!
//! THE READING IS LINEAR AND EXACT, WITH NO EIGENVECTOR AND NO SEARCH. A
//! body's covariant vectors are its odd moment `t = ⟨d |d|²⟩` and its
//! images under the second moment `M = ⟨d dᵀ⟩`: `w_k = M^{k−1} t` for
//! `k = 1, 2, …`, a vector of degree `2k + 1` that turns with the body
//! (`t` degree 3, `Mt` 5, `M²t` 7). If `b = ρ R a`, then
//! `w_k(b) = ρ^{2k+1} R w_k(a)`, so with the columns in hand
//!
//! ```text
//!     R = [w_k(b) / ρ^{2k+1}]  ·  [w_k(a)]⁻¹
//! ```
//!
//! is solved over ℚ by one elimination. Diagonalising `M` would put its
//! principal axes in `ℚ(√Δ)` (55.3); the columns never need them.
//!
//! A FLAT BODY — one section, or one plane of the tube — has columns that
//! span one dimension fewer than the base, and the last column is their
//! dual: `c · x = det[w₀, …, w_{n−2}, x]`, which turns as `det R · R c`. So
//! the dual completes the frame of columns with the orientation, and a flat
//! body's mirror across its own plane is one of its turns, as it is in
//! space.
//!
//! THE MOMENTS ARE THE SITES' AS POINTS on every axis alike. The sensor's
//! lamina is a square on the grid plane and a point along `e₃`; a turn out of
//! the grid plane carries the square's floor onto an axis that has none, so
//! the floor would refuse every such turn at the sensor boundary (checklist
//! (q)). The pose reads the sites as the points they stand at.

use super::*;
use crate::transducer::base::is_grid_axis;
use num_traits::One;

type Q = BigRational;

fn q(n: i64) -> Q {
    Q::from_integer(BigInt::from(n))
}

/// DOES THIS BODY EXTEND OFF THE SENSOR'S PLANE — along the succession axis,
/// or any axis the lamina's square does not span. Read from the sites
/// themselves: two neighbours in the support differ on such an axis, and
/// the sites are not all one value there under any order of them. A body at
/// one section does not, and its pose is a turn in the sensor's plane.
pub fn spans_sections(b: &Blade) -> bool {
    (0..b.n()).filter(|&i| !is_grid_axis(i)).any(|i| {
        b.support.windows(2).any(|w| match w {
            [x, y] => x.axis(i) != y.axis(i),
            _ => false,
        })
    })
}

/// WHETHER THE PAIR IS READ ON THE TUBE: either body extends off the
/// sensor's plane. The one producer of the regime, so a caller that counts
/// it counts what [`relative_pose`] did.
pub fn on_tube(a: &Blade, b: &Blade) -> bool {
    spans_sections(a) || spans_sections(b)
}

/// THE RELATIVE POSE OF TWO BODIES (FR1, 72.1c(1), (4)): `θ` carrying `a`'s
/// tube onto `b`'s, over every plane the two occupy.
///
/// Both at one section: [`relative_rotor`] on the sensor's plane, unchanged.
/// On the tube: the linear reading above, refused by name where it does not
/// close —
///
/// - [`NoPose::Bodiless`] a body with no support;
/// - [`NoPose::Incongruent`] bodies on bases of different `n` (strata of
///   different `n` are disjoint, BQ4, and no motion of one base is on the
///   other), or a solved `R` that is not orthogonal or does not carry `M`;
/// - [`NoPose::Isotropy`] the columns span different dimensions — one body
///   orients where the other does not, and no motion changes that;
/// - where the columns span fewer than `n − 1` dimensions on both, the even
///   tower reads the pose ([`even_pose`], FR1b): a symmetric body is
///   [`Pose::HalfOpen`], its `ℤ₂` held as it is at one section;
/// - [`NoPose::Unoriented`] only where no order orients: a stabilizer the
///   even tower leaves beyond `ℤ₂`;
/// - [`NoPose::Irrational`] `ρ²` is not a rational square;
/// - [`NoPose::Mirrored`] an improper `R` carries the tower and no proper
///   one does.
pub fn relative_pose(a: &Blade, b: &Blade) -> Result<Pose, NoPose> {
    if a.support.is_empty() || b.support.is_empty() {
        return Err(NoPose::Bodiless);
    }
    let n = a.n();
    if b.n() != n {
        return Err(NoPose::Incongruent(Incongruence::Base));
    }
    match on_tube(a, b) {
        false => relative_rotor(a, b, crate::transducer::base::GRID_PLANE),
        true => tube_pose(a, b, n),
    }
}

/// A BODY'S POINT TOWER ON `n` AXES: the second moment `M`, the odd moment
/// `t` and the fourth order's symmetric part `N = ⟨d dᵀ |d|²⟩` about the
/// sites' mean, each normalised by the count (A.12), and `tr M`, the scale's
/// reading. `M` and `N` are the EVEN tower: they turn with the body and
/// cannot see a half turn.
struct Tower {
    m: Vec<Vec<Q>>,
    t: Vec<Q>,
    n4: Vec<Vec<Q>>,
    tr: Q,
}

/// THE POINT TOWER'S ONE PRODUCER (78.38): each site as the point it stands
/// at, about the sites' mean, on `n` axes, beside the count every moment is
/// normalised by. No lamina's own square enters, so every moment read off
/// these is floor-free at every order and scales exactly under a similarity
/// — the tower every RELATION between shapes reads. The nucleus and its
/// radius, which carry A.12's floor, are the lamina tower's.
pub(super) fn centred(b: &Blade, n: usize) -> Option<(Q, Vec<Vec<Q>>)> {
    let count = q(i64::try_from(b.support.len()).ok()?);
    if count.is_zero() {
        return None;
    }
    let mut mean = vec![Q::zero(); n];
    for s in &b.support {
        for (i, x) in mean.iter_mut().enumerate() {
            *x += q(s.axis(i)?);
        }
    }
    for x in mean.iter_mut() {
        *x = &*x / &count;
    }
    let points = b
        .support
        .iter()
        .map(|s| mean.iter().enumerate().map(|(i, c)| s.axis(i).map(|v| q(v) - c)).collect::<Option<Vec<Q>>>())
        .collect::<Option<Vec<Vec<Q>>>>()?;
    Some((count, points))
}

fn tower(b: &Blade, n: usize) -> Option<Tower> {
    let (count, points) = centred(b, n)?;
    let mut m = vec![vec![Q::zero(); n]; n];
    let mut t = vec![Q::zero(); n];
    let mut n4 = vec![vec![Q::zero(); n]; n];
    for d in &points {
        let r2 = d.iter().fold(Q::zero(), |acc, x| acc + x * x);
        for ((row, row4), di) in m.iter_mut().zip(n4.iter_mut()).zip(d) {
            for ((x, x4), dj) in row.iter_mut().zip(row4.iter_mut()).zip(d) {
                let p = di * dj;
                *x4 += &p * &r2;
                *x += p;
            }
        }
        for (x, di) in t.iter_mut().zip(d) {
            *x += di * &r2;
        }
    }
    for row in m.iter_mut().chain(n4.iter_mut()) {
        for x in row.iter_mut() {
            *x = &*x / &count;
        }
    }
    for x in t.iter_mut() {
        *x = &*x / &count;
    }
    let tr = m.iter().enumerate().fold(Q::zero(), |acc, (i, row)| acc + &row[i]);
    Some(Tower { m, t, n4, tr })
}

/// The point tower's second moment `M`, the one reading the transition's
/// covariance takes of it on the sites (law 12: `tower` is its producer).
pub(super) fn point_second_moment(b: &Blade, n: usize) -> Option<Vec<Vec<Q>>> {
    tower(b, n).map(|t| t.m)
}

fn apply(m: &[Vec<Q>], v: &[Q]) -> Vec<Q> {
    m.iter().map(|row| row.iter().zip(v).fold(Q::zero(), |acc, (x, y)| acc + x * y)).collect()
}

/// The columns as the columns of an `n`-row matrix: independent exactly
/// where its kernel is empty.
fn independent(cols: &[Vec<Q>], n: usize) -> bool {
    let rows: Vec<Vec<Q>> = (0..n).map(|i| cols.iter().map(|c| c[i].clone()).collect()).collect();
    cga::nullspace(rows, cols.len()).is_empty()
}

/// THE COVARIANT COLUMNS `t, Mt, M²t, …` while they stay independent. The
/// sequence is a Krylov sequence, so the first column that depends on those
/// before it ends it: every later one depends too.
fn columns(tw: &Tower, n: usize) -> Vec<Vec<Q>> {
    let mut cols: Vec<Vec<Q>> = Vec::new();
    let mut w = tw.t.clone();
    while cols.len() < n {
        let mut trial = cols.clone();
        trial.push(w.clone());
        if !independent(&trial, n) {
            break;
        }
        let next = apply(&tw.m, &w);
        cols.push(w);
        w = next;
    }
    cols
}

/// The determinant by the elimination's own pivots: the first row at or
/// below the diagonal holding the column, which is the algorithm and not a
/// choice.
fn det(mut rows: Vec<Vec<Q>>) -> Q {
    let k = rows.len();
    let mut d = Q::one();
    for c in 0..k {
        let Some(p) = (c..k).find(|&i| !rows[i][c].is_zero()) else {
            return Q::zero();
        };
        if p != c {
            rows.swap(p, c);
            d = -d;
        }
        let lead = rows[c][c].clone();
        d *= &lead;
        let pivot = rows[c].clone();
        for row in rows.iter_mut().skip(c + 1) {
            let f = &row[c] / &lead;
            for (x, y) in row.iter_mut().zip(&pivot) {
                *x = &*x - &f * y;
            }
        }
    }
    d
}

/// THE DUAL OF `n − 1` COLUMNS: `c · x = det[w₀, …, w_{n−2}, x]`, so
/// `c(R w) = det R · R c(w)` for `R` orthogonal. Exact, one determinant per
/// axis.
fn dual(cols: &[Vec<Q>], n: usize) -> Vec<Q> {
    (0..n)
        .map(|i| {
            let rows: Vec<Vec<Q>> = (0..n)
                .map(|r| {
                    let mut row: Vec<Q> = cols.iter().map(|c| c[r].clone()).collect();
                    row.push(match r == i {
                        true => Q::one(),
                        false => Q::zero(),
                    });
                    row
                })
                .collect();
            det(rows)
        })
        .collect()
}

/// `R` WITH `R · W_a = W_b`, row by row: `W_aᵀ rᵢ = (row i of W_b)`, the
/// kernel of `[W_aᵀ | −y]` with its last coordinate one. `None` where `W_a`
/// is singular.
fn solve(wa: &[Vec<Q>], wb: &[Vec<Q>], n: usize) -> Option<Vec<Vec<Q>>> {
    (0..n)
        .map(|i| {
            let rows: Vec<Vec<Q>> = wa
                .iter()
                .zip(wb)
                .map(|(ca, cb)| {
                    let mut row = ca.clone();
                    row.push(-cb[i].clone());
                    row
                })
                .collect();
            match cga::nullspace(rows, n + 1).as_slice() {
                [v] if v[n].is_one() => Some(v[..n].to_vec()),
                _ => None,
            }
        })
        .collect()
}

/// THE VERSOR OF THE MAP `R` over ℚ, as a product of constructors: a motion
/// fixing `e₊`, `e₋`, spelled by the constructors' one Cartan–Dieudonné
/// factorizer in the algebra's own metric ([`cga::constructors_of_base_turn`])
/// — an even count of reflections for a turn, odd for a mirror, both a pose.
/// `V = u₁ ⋯ u_m`, `V x Ṽ ∝ R x` — projective, no weight divided and no root.
/// `None` where `R` is no isometry of the metric (no reflections compose to
/// it) or at a null step: the refusal is carried, never assumed away.
fn versor_of(r: &[Vec<Q>], n: usize) -> Option<Mv> {
    let us = cga::constructors_of_base_turn(r, u8::try_from(n).ok()?).ok()?;
    Some(us.iter().fold(Mv::scalar(Q::one()), |v, u| v.geo(u)))
}

/// DOES THE MAP CARRY THE ONE TUBE ONTO THE OTHER, read by the algebra and
/// never by a Euclidean oracle or the lattice: `R`'s versor from the
/// Cartan–Dieudonné, even or odd, dilated and placed between the two bodies'
/// centres, and verified by the transition's covariance on the point tower
/// (`carried_on`, 78.38(C)) — the tower the pose reads, whose floor-free
/// moments turn out of the grid plane. The versor where it carries, `None`
/// where it does not.
fn carried(r: &[Vec<Q>], a: &Blade, b: &Blade, rho: &Q, n: usize) -> Option<Mv> {
    let v = versor_of(r, n)?;
    super::carry::carried_on(a, b, &cga::dilator(rho).geo(&v), super::tower::Tower::Sites).map(|_| v)
}

/// THE POSE ON THE TUBE: the columns, the dual where a body is flat, the
/// solve, the tower's check, the versor.
fn tube_pose(a: &Blade, b: &Blade, n: usize) -> Result<Pose, NoPose> {
    let (Some(ta), Some(tb)) = (tower(a, n), tower(b, n)) else {
        return Err(NoPose::Bodiless);
    };
    let (ca, cb) = (columns(&ta, n), columns(&tb, n));
    if ca.len() != cb.len() {
        return Err(NoPose::Isotropy);
    }
    if ta.tr.is_zero() {
        return Err(NoPose::Unoriented);
    }
    let rho2 = &tb.tr / &ta.tr;
    let Some(rho) = rational_sqrt(&rho2) else {
        return Err(NoPose::Irrational);
    };
    // `w_k(b) / ρ^{2k+1}`, `k` from one — `t` has degree 3, `Mt` 5, `M²t`
    // 7: `b`'s columns at `a`'s scale, so `R` is a turn.
    let mut scale = &rho * &rho2;
    let mut wb: Vec<Vec<Q>> = Vec::new();
    for c in &cb {
        wb.push(c.iter().map(|x| x / &scale).collect());
        scale = &scale * &rho2;
    }
    // FR1b — TOO FEW COLUMNS IS A SYMMETRY, NOT A REFUSAL: the even tower
    // reads what the odd one cannot.
    if ca.len() + 1 < n {
        return even_pose(a, b, &ta, &tb, &ca, &wb, &rho, n);
    }
    let mut wa = ca.clone();
    // A FLAT BODY'S FRAME OF COLUMNS IS COMPLETED BY THEIR DUAL, the proper
    // completion first; its mirror — the same frame read orientation-reversed
    // — where the proper one does not carry.
    let flat = wa.len() + 1 == n;
    if flat {
        let (da, db) = (dual(&ca, n), dual(&wb, n));
        wa.push(da);
        wb.push(db);
    }
    if let Some(v) = solve(&wa, &wb, n).and_then(|r| carried(&r, a, b, &rho, n)) {
        return Ok(Pose::Turn(v));
    }
    if !flat {
        return Err(NoPose::Incongruent(Incongruence::Tube));
    }
    if let Some(last) = wb.last_mut() {
        for x in last.iter_mut() {
            *x = -x.clone();
        }
    }
    match solve(&wa, &wb, n).and_then(|r| carried(&r, a, b, &rho, n)) {
        Some(v) => Ok(Pose::Turn(v)),
        None => Err(NoPose::Incongruent(Incongruence::Tube)),
    }
}

/// FR1b — THE POSE FROM THE EVEN TOWER, where the odd columns are too few to
/// fix the turn (57.3, 59.5, 67.1).
///
/// A body whose odd moment vanishes — a centrosymmetric body, or a flat one
/// symmetric by a half turn in its own plane — has no column, and at one
/// section the planar reader holds it `HalfOpen`: a `ℤ₂` freedom its own
/// symmetry makes real. The tube holds it the same way. The turn is read
/// from the matrices that INTERTWINE the even tower,
///
/// ```text
///     X M_a = (M_b / ρ²) X,     X N_a = (N_b / ρ⁴) X,
/// ```
///
/// linear in `X` and solved by one elimination: its kernel is `R · C` for
/// `C` in the tower's commutant, so its dimension says how much the even
/// tower leaves free. The orthogonal elements are then found exactly —
///
/// - one line: the turn up to sign, and the sign is the orientation;
/// - two lines: the elements `a X₁ + b X₂` with `XᵀX ∝ 1`, the roots of one
///   binary quadratic, rational or refused as [`NoPose::Irrational`];
/// - three or more, or a pencil of solutions: a stabilizer beyond `ℤ₂` —
///   a line's continuous turn about itself, or a rectangle's three half
///   turns — which this reader does not name a representative for:
///   [`NoPose::Unoriented`].
///
/// Each candidate, and its negative, is kept where its versor — even or odd,
/// a turn or a mirror — carries the one tube onto the other and every odd
/// column there is lands. One carried is a [`Pose::Turn`]; several are the
/// held coset, [`Pose::HalfOpen`], in the quadratic's own root order.
#[expect(clippy::too_many_arguments, reason = "the two bodies, their towers, the odd columns at one scale, the scale and the base's n: the pose's inputs, each read once")]
fn even_pose(a: &Blade, b: &Blade, ta: &Tower, tb: &Tower, ca: &[Vec<Q>], wb: &[Vec<Q>], rho: &Q, n: usize) -> Result<Pose, NoPose> {
    let rho2 = rho * rho;
    let rho4 = &rho2 * &rho2;
    let scaled = |m: &[Vec<Q>], s: &Q| -> Vec<Vec<Q>> { m.iter().map(|row| row.iter().map(|x| x / s).collect()).collect() };
    let (mb, nb) = (scaled(&tb.m, &rho2), scaled(&tb.n4, &rho4));
    // `X A_a − A_b X = 0`, entry `(p, q)`, over the unknowns `X[p][r]` at
    // `p·n + r`.
    let mut rows: Vec<Vec<Q>> = Vec::new();
    for (aa, ab) in [(&ta.m, &mb), (&ta.n4, &nb)] {
        for p in 0..n {
            for qq in 0..n {
                let mut row = vec![Q::zero(); n * n];
                for r in 0..n {
                    row[p * n + r] += &aa[r][qq];
                    row[r * n + qq] -= &ab[p][r];
                }
                rows.push(row);
            }
        }
    }
    let kernel: Vec<Vec<Vec<Q>>> =
        cga::nullspace(rows, n * n).into_iter().map(|v| v.chunks(n).map(<[Q]>::to_vec).collect()).collect();
    let lines: Vec<Vec<Vec<Q>>> = match kernel.as_slice() {
        [] => return Err(NoPose::Incongruent(Incongruence::Tube)),
        [x] => vec![x.clone()],
        [x1, x2] => match orthogonal_lines(x1, x2, n) {
            Lines::Found(l) => l,
            Lines::Irrational => return Err(NoPose::Irrational),
            Lines::Pencil => return Err(NoPose::Unoriented),
        },
        _ => return Err(NoPose::Unoriented),
    };
    // Each candidate's versor carries the tubes (`carried`, the algebra's
    // own carriage), and every odd column there is must land.
    let mut poses: Vec<Mv> = Vec::new();
    for x in &lines {
        // The first column's squared length, READ FROM THE ALGEBRA (law 12):
        // the column as a base vector, squared by the product's own metric.
        let first: Vec<Q> = (0..n).map(|k| x[k][0].clone()).collect();
        let g = cga::base_vector(&first).norm2();
        let Some(lambda) = rational_sqrt(&g) else { continue };
        if lambda.is_zero() {
            continue;
        }
        for sign in [Q::one(), -Q::one()] {
            let r: Vec<Vec<Q>> = x.iter().map(|row| row.iter().map(|v| v * &sign / &lambda).collect()).collect();
            if !ca.iter().zip(wb).all(|(wa, wb)| apply(&r, wa) == *wb) {
                continue;
            }
            let Some(v) = carried(&r, a, b, rho, n) else { continue };
            if !poses.iter().any(|p| p.projective_eq(&v)) {
                poses.push(v);
            }
        }
    }
    // A PAIR IS READ ORIENTATION-REVERSED ONLY WHERE NO TURN CARRIES IT: a
    // body symmetric under a mirror is carried by its reflections as well as
    // by its turns, and its pose is the turns; the mirror is the reading of a
    // pair no turn relates.
    let (turns, mirrors): (Vec<Mv>, Vec<Mv>) = poses.into_iter().partition(|v| v.parity() == Some(false));
    let read = match turns.is_empty() {
        true => mirrors,
        false => turns,
    };
    match read.as_slice() {
        [] => Err(NoPose::Incongruent(Incongruence::Tube)),
        [one] => Ok(Pose::Turn(one.clone())),
        _ => Ok(Pose::HalfOpen(read)),
    }
}

/// The orthogonal lines of a two-dimensional kernel, or why there are none.
enum Lines {
    Found(Vec<Vec<Vec<Q>>>),
    Irrational,
    Pencil,
}

/// THE ELEMENTS `a X₁ + b X₂` WITH `XᵀX ∝ 1`. `G(a, b) = a² G₁₁ + ab (G₁₂ +
/// G₂₁) + b² G₂₂`; each off-diagonal entry, and each diagonal entry against
/// the first, is a binary quadratic in `(a, b)` that must vanish. The first
/// that is not identically zero — in entry order, the matrix's own — gives
/// the candidate ratios, exact where its discriminant is a rational square;
/// each candidate is kept only where every quadratic vanishes on it. All of
/// them identically zero is a pencil: every element is orthogonal, a
/// continuous freedom.
fn orthogonal_lines(x1: &[Vec<Q>], x2: &[Vec<Q>], n: usize) -> Lines {
    let gram = |x: &[Vec<Q>], y: &[Vec<Q>], i: usize, j: usize| (0..n).fold(Q::zero(), |acc, k| acc + &x[k][i] * &y[k][j]);
    let form = |i: usize, j: usize| -> [Q; 3] {
        [gram(x1, x1, i, j), gram(x1, x2, i, j) + gram(x2, x1, i, j), gram(x2, x2, i, j)]
    };
    let mut forms: Vec<[Q; 3]> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            forms.push(form(i, j));
        }
        if i > 0 {
            let (d, d0) = (form(i, i), form(0, 0));
            forms.push([&d[0] - &d0[0], &d[1] - &d0[1], &d[2] - &d0[2]]);
        }
    }
    let Some([a2, ab, b2]) = forms.iter().find(|f| f.iter().any(|c| !c.is_zero())).cloned() else {
        return Lines::Pencil;
    };
    let roots: Vec<(Q, Q)> = match a2.is_zero() {
        // `b (ab·a + b2·b) = 0`.
        true => match ab.is_zero() {
            true => vec![(Q::one(), Q::zero())],
            false => vec![(Q::one(), Q::zero()), (-b2.clone(), ab.clone())],
        },
        false => {
            let disc = &ab * &ab - q(4) * &a2 * &b2;
            let Some(s) = rational_sqrt(&disc) else {
                return Lines::Irrational;
            };
            let two_a = q(2) * &a2;
            match s.is_zero() {
                true => vec![(-ab.clone(), two_a)],
                false => vec![(-&ab + &s, two_a.clone()), (-&ab - &s, two_a)],
            }
        }
    };
    let vanishes = |(a, b): &(Q, Q)| forms.iter().all(|[f0, f1, f2]| (a * a * f0 + a * b * f1 + b * b * f2).is_zero());
    Lines::Found(
        roots
            .iter()
            .filter(|r| vanishes(r))
            .map(|(a, b)| {
                x1.iter()
                    .zip(x2)
                    .map(|(r1, r2)| r1.iter().zip(r2).map(|(u, v)| a * u + b * v).collect())
                    .collect()
            })
            .collect(),
    )
}
