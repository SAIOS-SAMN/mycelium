// Fixture constructors for the blade's tests (MU-STANDARDS 3.3): a
// rectangle in frame 0, its exact dilation, and a cell list.

pub fn rect(r0: i64, c0: i64, h: i64, w: i64) -> Vec<(i64, i64, i64)> {
    let mut v: Vec<(i64, i64, i64)> =
        (0..h).flat_map(|r| (0..w).map(move |c| (r0 + r, c0 + c, 0))).collect();
    v.sort_unstable();
    v
}

pub fn dilate(cells: &[(i64, i64, i64)], k: i64) -> Vec<(i64, i64, i64)> {
    let mut v: Vec<(i64, i64, i64)> = cells
        .iter()
        .flat_map(|&(r, c, f)| (0..k).flat_map(move |i| (0..k).map(move |j| (k * r + i, k * c + j, f))))
        .collect();
    v.sort_unstable();
    v
}

/// A frame-0 scene: `(row, col, hue)` lifted to the base.
pub fn cells(v: &[(i64, i64, u8)]) -> Vec<(i64, i64, i64, u8)> {
    v.iter().map(|&(r, c, h)| (r, c, 0, h)).collect()
}

/// A support in frame 0.
pub fn at_frame_0(v: &[(i64, i64)]) -> Vec<(i64, i64, i64)> {
    v.iter().map(|&(r, c)| (r, c, 0)).collect()
}
