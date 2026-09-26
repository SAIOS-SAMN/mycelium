// Fixture constructors for the blade's tests (MU-STANDARDS 3.3): a
// rectangle in frame 0, its exact dilation, and a list of compartments. Every
// site is handed to the base through the sensor's own reading (71.2(1)).

use crate::transducer::base::{Site, GRID_AXES};
use crate::transducer::blade::Compartment;

pub fn rect(r0: i64, c0: i64, h: i64, w: i64) -> Vec<Site> {
    let mut v: Vec<Site> =
        (0..h).flat_map(|r| (0..w).map(move |c| Site::from((r0 + r, c0 + c, 0)))).collect();
    v.sort_unstable();
    v
}

/// Each site replaced by the `k × k` block on the grid axes it dilates to;
/// every other coordinate is kept.
pub fn dilate(support: &[Site], k: i64) -> Vec<Site> {
    let [ga, gb] = GRID_AXES;
    let mut v: Vec<Site> = support
        .iter()
        .flat_map(|s| {
            (0..k).flat_map(move |i| {
                (0..k).map(move |j| {
                    let mut axes = s.axes().to_vec();
                    axes[ga] = k * axes[ga] + i;
                    axes[gb] = k * axes[gb] + j;
                    Site::new(axes)
                })
            })
        })
        .collect();
    v.sort_unstable();
    v
}

/// A frame-0 scene: `(row, col, hue)` lifted to compartments on the base.
pub fn compartments(v: &[(i64, i64, u8)]) -> Vec<Compartment> {
    v.iter().map(|&(r, c, h)| Compartment::from((r, c, 0, h))).collect()
}

/// A support in frame 0.
pub fn at_frame_0(v: &[(i64, i64)]) -> Vec<Site> {
    v.iter().map(|&(r, c)| Site::from((r, c, 0))).collect()
}

/// Compartments from anything the sensor reads as one.
pub fn sites_with_hue<T: Copy + Into<Compartment>>(v: &[T]) -> Vec<Compartment> {
    v.iter().map(|&t| t.into()).collect()
}

/// Sites from anything the sensor reads as one.
pub fn sites<T: Copy + Into<Site>>(v: &[T]) -> Vec<Site> {
    v.iter().map(|&t| t.into()).collect()
}
