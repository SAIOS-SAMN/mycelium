// Fixture constructors for the blade's tests (MU-STANDARDS 3.3): a
// rectangle, its exact dilation, and a cell list.

pub fn rect(r0: i64, c0: i64, h: i64, w: i64) -> Vec<(i64, i64)> {
    let mut v: Vec<(i64, i64)> = (0..h).flat_map(|r| (0..w).map(move |c| (r0 + r, c0 + c))).collect();
    v.sort_unstable();
    v
}

pub fn dilate(cells: &[(i64, i64)], k: i64) -> Vec<(i64, i64)> {
    let mut v: Vec<(i64, i64)> = cells
        .iter()
        .flat_map(|&(r, c)| (0..k).flat_map(move |i| (0..k).map(move |j| (k * r + i, k * c + j))))
        .collect();
    v.sort_unstable();
    v
}

pub fn cells(v: &[(i64, i64, u8)]) -> Vec<(i64, i64, u8)> {
    v.to_vec()
}
