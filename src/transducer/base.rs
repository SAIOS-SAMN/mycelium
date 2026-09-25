// 71 — THE BASE IS ℚⁿ AND n IS DATA: a site of the base, with its
// coordinates and their count carried together.
//
//! A site is a point of ℤⁿ. Its length is the corpus's `n`, read at intake
//! (71.2(1)), and nothing downstream names how many coordinates a site has:
//! a reader asks the site. Two sites of different `n` are different kinds
//! of thing, so every operation between two sites refuses where their
//! lengths differ, rather than reading one as the other.
//!
//! The order is the coordinates' own, lexicographic. It is the order the
//! tuple `(r, c, k)` had at `n = 3`, so a support sorts exactly as before.

/// THE AXES A LAMINA'S UNIT SQUARE SPANS (16.4): the grid's rows and
/// columns, the first two axes of the base. A lamina has extent along these
/// and none along the frame axis or any further base axis; that is what a
/// lamina IS at the sensor, and it is read here once rather than at every
/// site that needs the square.
pub const GRID_AXES: [usize; 2] = [0, 1];

/// THE SENSOR'S OWN PLANE: the two grid axes. A reader that reads on this
/// plane names it at its call (BQ4); every other plane is generated.
pub const GRID_PLANE: (usize, usize) = (GRID_AXES[0], GRID_AXES[1]);

/// THE PLANES OF THE BASE AT `n` (71.2(3)): every pair of base axes `i < j`,
/// `C(n, 2)` of them, generated and never listed. Ordered by the larger axis
/// and then the smaller, so the order is PREFIX-STABLE in `n`: the planes of
/// `n` are the first planes of `n + 1`, as the inertia's pairs are, and the
/// grid plane is always first.
pub fn planes(n: usize) -> Vec<(usize, usize)> {
    (0..n).flat_map(|j| (0..j).map(move |i| (i, j))).collect()
}

/// A PLANE'S PLACE IN [`planes`], read off its two axes: the planes of the
/// larger axis `j` start after the `C(j, 2)` planes below it, and `i` is the
/// place among them. A coordinate, never a search of the list (rule 3).
/// `None` for a pair that is no plane (`i ≥ j`). The place is the same at
/// every `n` that has the plane, since the order is prefix-stable.
pub fn plane_index((i, j): (usize, usize)) -> Option<usize> {
    match i < j {
        true => Some(j * (j - 1) / 2 + i),
        false => None,
    }
}

/// THE AXIS A FAMILY'S FRAMES STAND ALONG (51.3): the third coordinate the
/// grid presents, a frame's place in its family. A section is the laminae
/// of one value of it, and the frame rides on the pose (16.5).
pub const FRAME_AXIS: usize = 2;

/// Is axis `i` one the lamina's square spans?
pub fn is_grid_axis(i: usize) -> bool {
    GRID_AXES.contains(&i)
}

/// A SITE OF THE BASE: its coordinates, `n` of them (71.2(1)).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Site {
    axes: Vec<i64>,
}

impl Site {
    /// A site from its coordinates. Their count is its `n`.
    pub fn new(axes: Vec<i64>) -> Site {
        Site { axes }
    }

    /// How many coordinates the site has: the base's `n`.
    pub fn n(&self) -> usize {
        self.axes.len()
    }

    /// The coordinates, in the base's own axis order.
    pub fn axes(&self) -> &[i64] {
        &self.axes
    }

    /// One coordinate, or `None` for an axis the base does not have.
    pub fn axis(&self, i: usize) -> Option<i64> {
        self.axes.get(i).copied()
    }

    /// The site with axis `i` set to `v`. `None` for an axis the base does
    /// not have.
    pub fn with_axis(&self, i: usize, v: i64) -> Option<Site> {
        let mut axes = self.axes.clone();
        *axes.get_mut(i)? = v;
        Some(Site { axes })
    }

    /// The site moved by `d`, coordinate by coordinate. `None` where `d` is
    /// not a displacement of this base: a different count of coordinates,
    /// or a coordinate that would leave `i64`.
    pub fn offset(&self, d: &[i64]) -> Option<Site> {
        if d.len() != self.axes.len() {
            return None;
        }
        let axes = self
            .axes
            .iter()
            .zip(d)
            .map(|(a, b)| a.checked_add(*b))
            .collect::<Option<Vec<i64>>>()?;
        Some(Site { axes })
    }

    /// THE SITE WRITTEN OUT, `n` FIRST (71.2(1), A21). The count is the
    /// coarsest stratum, so every site of one base shares a prefix and two
    /// bases never interleave. Then each coordinate, little-endian, as the
    /// fixed-width site key was written before `n` became data: within one
    /// `n`, sites keep the order they had.
    pub fn put(&self, out: &mut Vec<u8>) {
        // `n` by its own coordinate (STD1 3c-2, ruled (d)): a prefix still,
        // and total at every `n`.
        crate::transducer::address::put_key_count(out, self.axes.len());
        for a in &self.axes {
            out.extend_from_slice(&a.to_le_bytes());
        }
    }

    /// The site as a key of its own: [`Site::put`] into a fresh buffer.
    pub fn key(&self) -> Vec<u8> {
        let mut k = Vec::with_capacity(4 + 8 * self.axes.len());
        self.put(&mut k);
        k
    }

    /// The inverse of [`Site::put`]. `None` where the bytes do not hold a
    /// count and that many coordinates.
    pub fn take(b: &[u8], p: &mut usize) -> Option<Site> {
        let n = crate::transducer::address::take_key_count(b, p)?;
        let end = p.checked_add(n.checked_mul(8)?)?;
        let body = b.get(*p..end)?;
        let axes = body
            .chunks_exact(8)
            .map(|c| c.try_into().ok().map(i64::from_le_bytes))
            .collect::<Option<Vec<i64>>>()?;
        *p = end;
        Some(Site { axes })
    }
}
