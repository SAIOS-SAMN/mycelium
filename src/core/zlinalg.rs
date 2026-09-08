// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Exact integer linear algebra: lattice rank by fraction-free elimination.

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
pub fn hnf_with_transform<const N: usize>(
    rows: &[[i64; N]],
) -> Option<(Vec<[i128; N]>, Vec<Vec<i128>>)> {
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
        loop {
            let Some(piv) = (rank..h.len()).find(|&r| h[r][col] != 0) else {
                break;
            };
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
        let q = res[p].checked_div(row[p])?;
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
