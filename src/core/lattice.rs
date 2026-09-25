// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 16.18, 16.12 — THE INTEGER LATTICE a form's free part spans: its rank,
//! and the Hermite form that says what a word was made of.
//!
//! A18 reads the rank (16.18) to audit a form's dimension; A12 reads the
//! Hermite form with its transform `U` to factor a word through the words a
//! mind already holds (16.12), and the residue it leaves is the boundary
//! demand, unique and with no choice in it. Both are exact over ℤ by
//! fraction-free elimination and refuse on overflow rather than round
//! (law 4).

/// THE HERMITE FORM AND ITS TRANSFORM: the reduced rows `H`, and `U` giving
/// each of them as an integer combination of the input rows. `U` is the
/// factorisation — an HNF without it decides membership and cannot say what
/// the word was made of.
pub type Hnf<const N: usize> = (Vec<[i128; N]>, Vec<Vec<i128>>);

/// Rank of the lattice spanned by integer rows: fraction-free Gaussian
/// elimination (Bareiss) over i128, exact. `None` when a multiply, subtract
/// or divide overflows; the caller refuses.
pub fn z_rank<const N: usize>(rows: &[[i64; N]]) -> Option<u32> {
    let mut m: Vec<[i128; N]> = rows
        .iter()
        .map(|r| std::array::from_fn(|i| r[i] as i128))
        .collect();
    let mut rank = 0usize;
    let mut prev = 1i128;
    for col in 0..N {
        let Some(piv) = (rank..m.len()).find(|&r| m[r][col] != 0) else {
            continue;
        };
        m.swap(rank, piv);
        let p = m[rank][col];
        let (head, tail) = m.split_at_mut(rank + 1);
        let pivot = &head[rank];
        for row in tail.iter_mut() {
            let f = row[col];
            for (x, &pv) in row.iter_mut().zip(pivot.iter()) {
                let left = x.checked_mul(p)?;
                let right = pv.checked_mul(f)?;
                let diff = left.checked_sub(right)?;
                *x = diff.checked_div(prev)?;
            }
        }
        prev = p;
        rank += 1;
    }
    Some(rank as u32)
}

/// A SPARSE INTEGER ROW: its non-zero entries as `(column, value)`, columns
/// strictly ascending. The values are exact integers of any size (STD1, the
/// PH1 clause): a bound on the size of an exact number is a free constant,
/// and a dimension is paid for, never declared (A.6).
pub type SparseRow = Vec<(usize, num_bigint::BigInt)>;

/// THE RANK OF A SPARSE INTEGER MATRIX OF ANY WIDTH (BQ4, 71.2(6)): the rank
/// over ℚ of the rows, by exact fraction-free elimination over ℤ.
///
/// Written once beside [`z_rank`], which needs its width at compile time; a
/// boundary map has as many columns as its complex has cells, so this one
/// takes the rows as they come. Each row is reduced against the pivot
/// already standing at its leading column, `r ← p₀·r − r₀·p`, and divided by
/// the gcd of what is left, so the entries stay the integers they are; a
/// row that reduces to nothing is dependent, and one that reaches a column
/// with no pivot becomes its pivot. The rank is the number of pivots.
///
/// EXACT AT ANY WIDTH AND ANY SIZE: the entries are `BigInt`, so nothing
/// overflows and nothing is refused for its size; the pivots stand at the
/// columns the rows reach. The cost is the caller's to pay from its budget,
/// as it always was.
pub fn sparse_rank(rows: &[SparseRow]) -> usize {
    use num_bigint::BigInt;
    use num_integer::Integer;
    use num_traits::{One, Zero};
    let mut pivots: Vec<Option<Vec<(usize, BigInt)>>> = Vec::new();
    let mut rank = 0usize;
    for row in rows {
        let mut r: Vec<(usize, BigInt)> = row.iter().filter(|(_, v)| !v.is_zero()).cloned().collect();
        while let [(lead, a), ..] = r.as_slice() {
            let (lead, a) = (*lead, a.clone());
            if lead >= pivots.len() {
                pivots.resize(lead + 1, None);
            }
            let Some(p) = pivots[lead].as_ref() else {
                pivots[lead] = Some(r);
                rank += 1;
                break;
            };
            let b = match p.as_slice() {
                [(_, b), ..] => b.clone(),
                // A pivot is a non-empty row by construction.
                [] => break,
            };
            // r ← b·r − a·p, merged by column; the lead cancels.
            let mut next: Vec<(usize, BigInt)> = Vec::with_capacity(r.len() + p.len());
            let (mut i, mut j) = (0usize, 0usize);
            while i < r.len() || j < p.len() {
                let (ci, cj) = (r.get(i).map(|e| e.0), p.get(j).map(|e| e.0));
                let (col, v) = match (ci, cj) {
                    (Some(x), Some(y)) if x == y => {
                        let v = &r[i].1 * &b - &p[j].1 * &a;
                        i += 1;
                        j += 1;
                        (x, v)
                    }
                    (Some(x), Some(y)) if x < y => {
                        let v = &r[i].1 * &b;
                        i += 1;
                        (x, v)
                    }
                    (Some(x), None) => {
                        let v = &r[i].1 * &b;
                        i += 1;
                        (x, v)
                    }
                    (_, Some(y)) => {
                        let v = -(&p[j].1 * &a);
                        j += 1;
                        (y, v)
                    }
                    (None, None) => break,
                };
                if !v.is_zero() {
                    next.push((col, v));
                }
            }
            let g = next.iter().fold(BigInt::zero(), |g, (_, v)| g.gcd(v));
            if g > BigInt::one() {
                for e in next.iter_mut() {
                    e.1 = &e.1 / &g;
                }
            }
            r = next;
        }
    }
    rank
}

/// Row-style Hermite normal form over ℤ with the transform that produced
/// it: `U · G = H`, `U` unimodular, `H` in row echelon with positive
/// pivots. Exact over `i128` throughout; `None` on any overflow, so a
/// lattice too large to reduce is refused rather than wrapped (rule 11).
///
/// A12 needs the transform, not only the form: membership of `r` in the
/// lattice is decided by reducing against `H`, and the coefficients of that
/// reduction pulled back through `U` are the exponents over the ORIGINAL
/// generators — which is the factorisation, the thing comprehension is
/// looking for. An HNF without `U` decides membership and cannot say what
/// the word was made of.
pub fn hnf_with_transform<const N: usize>(rows: &[[i64; N]]) -> Option<Hnf<N>> {
    let mut h: Vec<[i128; N]> = rows
        .iter()
        .map(|r| std::array::from_fn(|i| r[i] as i128))
        .collect();
    // u[i] expresses h[i] as an integer combination of the input rows.
    let mut u: Vec<Vec<i128>> = (0..h.len())
        .map(|i| (0..h.len()).map(|j| i128::from(i == j)).collect())
        .collect();
    let mut rank = 0usize;
    for col in 0..N {
        // Euclid on the column: repeatedly reduce the lower rows against
        // the pivot until one entry is left. No division that is not exact,
        // and no choice of pivot by size — the loop terminates because
        // every step strictly reduces a non-negative remainder.
        while let Some(piv) = (rank..h.len()).find(|&r| h[r][col] != 0) {
            h.swap(rank, piv);
            u.swap(rank, piv);
            let mut done = true;
            let pivot = h[rank];
            for r in (rank + 1)..h.len() {
                if h[r][col] == 0 {
                    continue;
                }
                let q = h[r][col].checked_div(h[rank][col])?;
                if q != 0 {
                    for (x, &pv) in h[r].iter_mut().zip(pivot.iter()) {
                        let d = pv.checked_mul(q)?;
                        *x = x.checked_sub(d)?;
                    }
                    for c in 0..u[rank].len() {
                        let d = u[rank][c].checked_mul(q)?;
                        u[r][c] = u[r][c].checked_sub(d)?;
                    }
                }
                if h[r][col] != 0 {
                    done = false;
                }
            }
            if done {
                break;
            }
        }
        if rank < h.len() && h[rank][col] != 0 {
            if h[rank][col] < 0 {
                for x in h[rank].iter_mut() {
                    *x = x.checked_neg()?;
                }
                for c in 0..u[rank].len() {
                    u[rank][c] = u[rank][c].checked_neg()?;
                }
            }
            rank += 1;
        }
    }
    h.truncate(rank);
    u.truncate(rank);
    Some((h, u))
}

/// Reduce `r` against an HNF basis. Returns the integer coefficients over
/// the `H` rows and the canonical residue that is left. `r` is in the
/// lattice iff the residue is the zero vector, and the residue is unique
/// for a given `H` — which is what makes a remainder a coordinate and not a
/// choice (A12 step 5: "the HNF residue is unique … the exact boundary
/// demand, with no choice in it").
pub fn reduce_against<const N: usize>(
    h: &[[i128; N]],
    r: &[i64; N],
) -> Option<(Vec<i128>, [i128; N])> {
    let mut res: [i128; N] = std::array::from_fn(|i| r[i] as i128);
    let mut coeffs = vec![0i128; h.len()];
    for (i, row) in h.iter().enumerate() {
        let Some(p) = (0..N).find(|&c| row[c] != 0) else {
            continue;
        };
        if res[p] == 0 {
            continue;
        }
        // EUCLIDEAN, NOT TRUNCATING. `checked_div` rounds toward zero, so it
        // leaves a residue in `(−|d|, |d|)`: two points of ONE coset reduce
        // to two representatives, and the uniqueness the doc above claims —
        // A12 step 5's "the exact boundary demand, with no choice in it" —
        // is false below the axis. `div_euclid` leaves `rem_euclid`, which
        // is in `[0, |d|)` by construction and is the coset representative.
        let q = res[p].checked_div_euclid(row[p])?;
        if q == 0 {
            continue;
        }
        coeffs[i] = q;
        for c in 0..N {
            let d = row[c].checked_mul(q)?;
            res[c] = res[c].checked_sub(d)?;
        }
    }
    Some((coeffs, res))
}
