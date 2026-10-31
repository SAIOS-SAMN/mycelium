// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 16.12, A12 — A WORD'S FACTORISATION OVER THE WORDS A MIND HOLDS: the
//! integer exponent each held generator is cited to, and the residue left.
//!
//! The rows are exponent vectors — how many times each letter a word is
//! spelled with appears — so the rows' combinations are integer by meaning: a
//! word is not cited half a time. The Hermite form with its transform `U`
//! factors a word through the held words, and the residue it leaves is the
//! boundary demand, unique and with no choice in it. Exact over ℤ at any
//! size, refusing nothing for its size (law 4, STD1). The dimension of a
//! span is not read here: that is the blade's grade (`core::span`).

/// THE HERMITE FORM AND ITS TRANSFORM: the reduced rows `H`, and `U` giving
/// each of them as an integer combination of the input rows. `U` is the
/// factorisation — an HNF without it decides membership and cannot say what
/// the word was made of. Exact integers of any size (STD1).
pub type Hnf<const N: usize> = (Vec<[num_bigint::BigInt; N]>, Vec<Vec<num_bigint::BigInt>>);

/// Row-style Hermite normal form over ℤ with the transform that produced
/// it: `U · G = H`, `U` unimodular, `H` in row echelon with positive
/// pivots. Exact integers of any size throughout (STD1): nothing overflows
/// and nothing is refused for its size.
///
/// A12 needs the transform, not only the form: membership of `r` among the
/// held words' combinations is decided by reducing against `H`, and the
/// coefficients of that reduction pulled back through `U` are the exponents
/// over the ORIGINAL generators — which is the factorisation, the thing
/// comprehension is looking for. An HNF without `U` decides membership and
/// cannot say what the word was made of.
pub fn hnf_with_transform<const N: usize>(rows: &[[i64; N]]) -> Hnf<N> {
    use num_bigint::BigInt;
    use num_traits::{Signed, Zero};
    let mut h: Vec<[BigInt; N]> = rows.iter().map(|r| std::array::from_fn(|i| BigInt::from(r[i]))).collect();
    // u[i] expresses h[i] as an integer combination of the input rows.
    let mut u: Vec<Vec<BigInt>> =
        (0..h.len()).map(|i| (0..h.len()).map(|j| BigInt::from(u8::from(i == j))).collect()).collect();
    let mut rank = 0usize;
    for col in 0..N {
        // Euclid on the column: repeatedly reduce the lower rows against
        // the pivot until one entry is left. No division that is not exact,
        // and no choice of pivot by size — the loop terminates because
        // every step strictly reduces a non-negative remainder.
        while let Some(piv) = (rank..h.len()).find(|&r| !h[r][col].is_zero()) {
            h.swap(rank, piv);
            u.swap(rank, piv);
            let mut done = true;
            let pivot = h[rank].clone();
            let upivot = u[rank].clone();
            for r in (rank + 1)..h.len() {
                if h[r][col].is_zero() {
                    continue;
                }
                // Truncating toward zero: each step strictly shrinks the
                // lower entry's magnitude.
                let q = &h[r][col] / &pivot[col];
                if !q.is_zero() {
                    for (x, pv) in h[r].iter_mut().zip(pivot.iter()) {
                        *x -= pv * &q;
                    }
                    for (x, pv) in u[r].iter_mut().zip(upivot.iter()) {
                        *x -= pv * &q;
                    }
                }
                if !h[r][col].is_zero() {
                    done = false;
                }
            }
            if done {
                break;
            }
            // A remainder stands below: the FIRST such row by position
            // becomes the pivot. Its entry is a remainder of the old pivot,
            // strictly smaller in magnitude, so Euclid terminates — and no
            // pivot is chosen by its size.
            if let Some(next) = ((rank + 1)..h.len()).find(|&r| !h[r][col].is_zero()) {
                h.swap(rank, next);
                u.swap(rank, next);
            }
        }
        if rank < h.len() && !h[rank][col].is_zero() {
            if h[rank][col].is_negative() {
                for x in h[rank].iter_mut() {
                    *x = -&*x;
                }
                for x in u[rank].iter_mut() {
                    *x = -&*x;
                }
            }
            rank += 1;
        }
    }
    h.truncate(rank);
    u.truncate(rank);
    (h, u)
}

/// Reduce `r` against an HNF basis. Returns the integer coefficients over
/// the `H` rows and the canonical residue that is left. `r` is a combination
/// of the held words iff the residue is the zero vector, and the residue is
/// unique for a given `H` — which is what makes a remainder a coordinate and
/// not a choice (A12 step 5: "the HNF residue is unique … the exact boundary
/// demand, with no choice in it"). Exact at any size (STD1).
pub fn reduce_against<const N: usize>(
    h: &[[num_bigint::BigInt; N]],
    r: &[i64; N],
) -> (Vec<num_bigint::BigInt>, [num_bigint::BigInt; N]) {
    use num_bigint::BigInt;
    use num_integer::Integer;
    use num_traits::Zero;
    let mut res: [BigInt; N] = std::array::from_fn(|i| BigInt::from(r[i]));
    let mut coeffs = vec![BigInt::zero(); h.len()];
    for (i, row) in h.iter().enumerate() {
        let Some(p) = (0..N).find(|&c| !row[c].is_zero()) else {
            continue;
        };
        if res[p].is_zero() {
            continue;
        }
        // EUCLIDEAN, NOT TRUNCATING: the HNF's pivots are positive, so the
        // floor quotient leaves a remainder in `[0, d)` — the coset
        // representative, unique below the axis as above it.
        let q = res[p].div_floor(&row[p]);
        if q.is_zero() {
            continue;
        }
        for c in 0..N {
            res[c] -= &row[c] * &q;
        }
        coeffs[i] = q;
    }
    (coeffs, res)
}
