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
/// - [`NoPose::Unoriented`] the columns span fewer than `n − 1` dimensions
///   on both: the tower at these orders leaves a continuous turn free;
/// - [`NoPose::Irrational`] `ρ²` is not a rational square;
/// - [`NoPose::Mirrored`] an improper `R` carries the tower and no proper
///   one does.
pub fn relative_pose(a: &Blade, b: &Blade) -> Result<Pose, NoPose> {
    if a.support.is_empty() || b.support.is_empty() {
        return Err(NoPose::Bodiless);
    }
    let n = a.n();
    if b.n() != n {
        return Err(NoPose::Incongruent);
    }
    match on_tube(a, b) {
        false => relative_rotor(a, b, crate::transducer::base::GRID_PLANE),
        true => tube_pose(a, b, n),
    }
}

/// A BODY'S POINT TOWER ON `n` AXES: the second moment `M` and the odd
/// moment `t` about the sites' mean, each normalised by the count (A.12), and
/// `tr M`, the scale's reading.
struct Tower {
    m: Vec<Vec<Q>>,
    t: Vec<Q>,
    tr: Q,
}

fn tower(b: &Blade, n: usize) -> Option<Tower> {
    let count = q(i64::try_from(b.support.len()).ok()?);
    let mut mean = vec![Q::zero(); n];
    for s in &b.support {
        for (i, x) in mean.iter_mut().enumerate() {
            *x += q(s.axis(i)?);
        }
    }
    for x in mean.iter_mut() {
        *x = &*x / &count;
    }
    let mut m = vec![vec![Q::zero(); n]; n];
    let mut t = vec![Q::zero(); n];
    for s in &b.support {
        let d: Vec<Q> = mean.iter().enumerate().map(|(i, c)| s.axis(i).map(|v| q(v) - c)).collect::<Option<_>>()?;
        let r2 = d.iter().fold(Q::zero(), |acc, x| acc + x * x);
        for (row, di) in m.iter_mut().zip(&d) {
            for (x, dj) in row.iter_mut().zip(&d) {
                *x += di * dj;
            }
        }
        for (x, di) in t.iter_mut().zip(&d) {
            *x += di * &r2;
        }
    }
    for row in m.iter_mut() {
        for x in row.iter_mut() {
            *x = &*x / &count;
        }
    }
    for x in t.iter_mut() {
        *x = &*x / &count;
    }
    let tr = m.iter().enumerate().fold(Q::zero(), |acc, (i, row)| acc + &row[i]);
    Some(Tower { m, t, tr })
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

/// DOES `R` CARRY THE TOWER: orthogonal, and `M_b = ρ² R M_a Rᵀ`. The odd
/// moment is carried by construction — it is the first column.
fn carries_tower(r: &[Vec<Q>], ta: &Tower, tb: &Tower, rho2: &Q, n: usize) -> bool {
    let col = |j: usize| -> Vec<Q> { r.iter().map(|row| row[j].clone()).collect() };
    let dot = |x: &[Q], y: &[Q]| x.iter().zip(y).fold(Q::zero(), |acc, (a, b)| acc + a * b);
    let orthogonal = (0..n).all(|j| {
        (0..n).all(|k| {
            dot(&col(j), &col(k))
                == match j == k {
                    true => Q::one(),
                    false => Q::zero(),
                }
        })
    });
    if !orthogonal {
        return false;
    }
    // `R M_a Rᵀ`, entry by entry, against `M_b / ρ²`.
    let rm: Vec<Vec<Q>> = r.iter().map(|row| (0..n).map(|j| dot(row, &col_of(&ta.m, j))).collect()).collect();
    (0..n).all(|i| (0..n).all(|j| dot(&rm[i], &r[j]) * rho2 == tb.m[i][j]))
}

fn col_of(m: &[Vec<Q>], j: usize) -> Vec<Q> {
    m.iter().map(|row| row[j].clone()).collect()
}

/// THE VERSOR OF AN ORTHOGONAL `R` over ℚ, as a product of reflections
/// (Cartan–Dieudonné), each vector rational: at column `k`, the current
/// image `x` of `e_k` is carried home by the reflection in `u = x − e_k`,
/// which fixes the columns already home. `R = H₁ ⋯ H_m`, so `V = u₁ ⋯ u_m`
/// and `V x Ṽ ∝ R x` — projective, no weight divided and no root.
fn versor_of(r: &[Vec<Q>], n: usize) -> Mv {
    let mut rest: Vec<Vec<Q>> = r.to_vec();
    let mut v = Mv::scalar(Q::one());
    for k in 0..n {
        let mut u = col_of(&rest, k);
        u[k] -= Q::one();
        if u.iter().all(Zero::is_zero) {
            continue;
        }
        let uu = u.iter().fold(Q::zero(), |acc, x| acc + x * x);
        for j in 0..n {
            let f = u.iter().zip(&rest).fold(Q::zero(), |acc, (ui, row)| acc + ui * &row[j]) * q(2) / &uu;
            for (row, ui) in rest.iter_mut().zip(&u) {
                row[j] = &row[j] - &f * ui;
            }
        }
        let vector = u.iter().enumerate().fold(Mv::zero(), |acc, (i, x)| acc.add(&Mv::blade(cga::base_axis(i + 1), x.clone())));
        v = v.geo(&vector);
    }
    v
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
    if ca.len() + 1 < n {
        return Err(NoPose::Unoriented);
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
    let mut wa = ca.clone();
    // A FLAT BODY'S FRAME OF COLUMNS IS COMPLETED BY THEIR DUAL, the proper
    // completion first; its mirror is read only where the proper one fails.
    let flat = wa.len() + 1 == n;
    if flat {
        let (da, db) = (dual(&ca, n), dual(&wb, n));
        wa.push(da);
        wb.push(db);
    }
    let proper = solve(&wa, &wb, n).filter(|r| carries_tower(r, &ta, &tb, &rho2, n));
    let r = match (proper, flat) {
        (Some(r), _) => r,
        (None, true) => {
            if let Some(last) = wb.last_mut() {
                for x in last.iter_mut() {
                    *x = -x.clone();
                }
            }
            return Err(match solve(&wa, &wb, n).filter(|r| carries_tower(r, &ta, &tb, &rho2, n)) {
                Some(_) => NoPose::Mirrored,
                None => NoPose::Incongruent,
            });
        }
        (None, false) => return Err(NoPose::Incongruent),
    };
    match det(r.clone()) < Q::zero() {
        true => Err(NoPose::Mirrored),
        false => Ok(Pose::Turn(versor_of(&r, n))),
    }
}
