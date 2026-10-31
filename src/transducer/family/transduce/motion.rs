// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ACT — THE EIGHT SIGNED SWAPS OF THE TWO CIRCLES (D3.2 §D.1, D.2). Among
//! the maps of `(ℚ/ℤ)²` that carry arcs to arcs (`GL(2, ℤ)`), those that
//! carry the seam set onto itself are exactly the signed permutations of
//! the two circles: a column of the matrix is the image of a coordinate
//! circle, and for that circle to stay on the seams it must be `±e₁` or
//! `±e₂`. They are the frame's rotations by 90°, 180° and 270°, its two
//! reflections and its two transposes, and the identity. A turn carries no
//! period: a swap moves the row turns onto the column circle and the
//! periods travel with the enclosure, so a 90° turn followed by any
//! re-period is exact on turns. The only obstruction is at projection,
//! where a moved boundary turn must land on a boundary of the output.

use super::{Band, Enclosed, Enclosure, TransductionRefusal};
use crate::transducer::family::perceive::Grid;

/// ONE SIGNED SWAP, acting on a site's turns `(t_r, t_c)`: the two circles
/// exchanged when `swap`, then the row circle negated when `negate_rows`
/// and the column circle when `negate_cols`. A negation `t ↦ −t` carries
/// the arc `[a, b)` to `[1 − b, 1 − a)`: it fixes the seam and the
/// epicenter, the turn of order two, and never crosses either.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SignedSwap {
    pub swap: bool,
    pub negate_rows: bool,
    pub negate_cols: bool,
}

impl SignedSwap {
    pub const IDENTITY: SignedSwap = SignedSwap { swap: false, negate_rows: false, negate_cols: false };
    /// `(t_r, t_c) ↦ (−t_c, t_r)`: the frame turned by 90°, counterclockwise
    /// as it is drawn (rows down, columns right).
    pub const TURN_90: SignedSwap = SignedSwap { swap: true, negate_rows: true, negate_cols: false };
    /// `(t_r, t_c) ↦ (−t_r, −t_c)`.
    pub const TURN_180: SignedSwap = SignedSwap { swap: false, negate_rows: true, negate_cols: true };
    /// `(t_r, t_c) ↦ (t_c, −t_r)`.
    pub const TURN_270: SignedSwap = SignedSwap { swap: true, negate_rows: false, negate_cols: true };
    /// `(t_r, t_c) ↦ (−t_r, t_c)`: the rows read in the other order.
    pub const FLIP_ROWS: SignedSwap = SignedSwap { swap: false, negate_rows: true, negate_cols: false };
    /// `(t_r, t_c) ↦ (t_r, −t_c)`.
    pub const FLIP_COLS: SignedSwap = SignedSwap { swap: false, negate_rows: false, negate_cols: true };
    /// `(t_r, t_c) ↦ (t_c, t_r)`.
    pub const TRANSPOSE: SignedSwap = SignedSwap { swap: true, negate_rows: false, negate_cols: false };
    /// `(t_r, t_c) ↦ (−t_c, −t_r)`.
    pub const ANTI_TRANSPOSE: SignedSwap = SignedSwap { swap: true, negate_rows: true, negate_cols: true };

    /// The eight, the identity first and then in the order of the three
    /// coordinates: the whole group, never a search over it.
    pub const ALL: [SignedSwap; 8] = [
        SignedSwap::IDENTITY,
        SignedSwap::FLIP_COLS,
        SignedSwap::FLIP_ROWS,
        SignedSwap::TURN_180,
        SignedSwap::TRANSPOSE,
        SignedSwap::TURN_270,
        SignedSwap::TURN_90,
        SignedSwap::ANTI_TRANSPOSE,
    ];

    /// The swap's integer matrix on `(t_r, t_c)`, columns the images of the
    /// row and column circles.
    pub fn matrix(&self) -> [[i64; 2]; 2] {
        let (sr, sc) = (if self.negate_rows { -1 } else { 1 }, if self.negate_cols { -1 } else { 1 });
        if self.swap {
            [[0, sr], [sc, 0]]
        } else {
            [[sr, 0], [0, sc]]
        }
    }

    /// THE SIGNED SWAP A MATRIX IS, or `Oblique`: a matrix of `GL(2, ℤ)`
    /// outside the eight — a shear, a scale, anything with an entry off
    /// `{−1, 0, 1}` or two entries in a row — moves the seam set, and no
    /// motion of the worksheet is read from it.
    pub fn of_matrix(m: [[i64; 2]; 2]) -> Result<SignedSwap, TransductionRefusal> {
        let unit = |x: i64| x == 1 || x == -1;
        match m {
            [[a, 0], [0, d]] if unit(a) && unit(d) => Ok(SignedSwap { swap: false, negate_rows: a < 0, negate_cols: d < 0 }),
            [[0, b], [c, 0]] if unit(b) && unit(c) => Ok(SignedSwap { swap: true, negate_rows: b < 0, negate_cols: c < 0 }),
            _ => Err(TransductionRefusal::Oblique),
        }
    }

    /// THE SIGNED SWAP A WORD OF THE ALPHABET IS on the sensor's plane
    /// `(e₀, e₁)`, read off its images of the two generators (law 11: an
    /// action held by its images). A word that does not keep the plane, or
    /// whose block is not a signed permutation (the `(3, 4, 5)` and
    /// `(5, 12, 13)` letters), has no action on turns: `Oblique`.
    pub fn of_word(word: &crate::transducer::alphabet::Word) -> Result<SignedSwap, TransductionRefusal> {
        let block = word.plane_block().ok_or(TransductionRefusal::Oblique)?;
        let entry = |q: &crate::core::Q| -> Result<i64, TransductionRefusal> {
            if !q.is_integer() {
                return Err(TransductionRefusal::Oblique);
            }
            i64::try_from(q.to_integer()).map_err(|_| TransductionRefusal::Oblique)
        };
        let [[a, b], [c, d]] = &block;
        SignedSwap::of_matrix([[entry(a)?, entry(b)?], [entry(c)?, entry(d)?]])
    }

    /// The enclosure a frame of `e` is held in once moved: each period
    /// travels with its circle.
    pub fn enclosure(&self, e: Enclosure) -> Enclosure {
        if self.swap {
            Enclosure { rows: e.cols, cols: e.rows }
        } else {
            e
        }
    }

    /// A band moved, in the moved enclosure's indices: its arcs exchanged
    /// when the circles are, and each negated arc `[a, b)` of period `p`
    /// read as `[p − b, p − a)`.
    pub fn band(&self, b: &Band, e: Enclosure) -> Band {
        let moved = self.enclosure(e);
        let (rows, cols) = if self.swap { (b.cols, b.rows) } else { (b.rows, b.cols) };
        let flip = |(a, z): (usize, usize), p: usize| (p - z, p - a);
        Band {
            rows: if self.negate_rows { flip(rows, moved.rows) } else { rows },
            cols: if self.negate_cols { flip(cols, moved.cols) } else { cols },
            hue: b.hue,
        }
    }

    /// PROJECT A MOVED FRAME (D3.2 §E): the frame `e` moved by this swap
    /// and re-periodded onto `to`, spoken as a grid. Each moved band's
    /// boundary turns must land on boundaries of `to`, or the projection
    /// refuses `OffSite` naming the first that does not — the obstruction
    /// a 90° turn between frames of different extent meets, and the only
    /// one: on turns the motion is exact.
    pub fn spoken_onto(&self, e: &Enclosed, to: Enclosure) -> Result<Grid, TransductionRefusal> {
        let moved = Enclosed {
            enclosure: self.enclosure(e.enclosure),
            bands: e.bands.iter().map(|b| self.band(b, e.enclosure)).collect(),
            body_of: e.body_of.clone(),
            bodies: e.bodies,
        };
        moved.spoken_onto(to)
    }

    /// THE FRAME MOVED, lifted again in its own moved enclosure: the moved
    /// bands spoken onto the moved enclosure's own periods (always landing) and read back
    /// as bands, so the moved frame's bands are maximal runs again.
    pub fn moved(&self, e: &Enclosed) -> Result<Enclosed, TransductionRefusal> {
        Enclosed::of(&self.spoken_onto(e, self.enclosure(e.enclosure))?)
    }
}
