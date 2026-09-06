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
        for r in (rank + 1)..m.len() {
            let f = m[r][col];
            for c in 0..N {
                let left = m[r][c].checked_mul(p)?;
                let right = m[rank][c].checked_mul(f)?;
                let diff = left.checked_sub(right)?;
                m[r][c] = diff.checked_div(prev)?;
            }
        }
        prev = p;
        rank += 1;
    }
    Some(rank as u32)
}
