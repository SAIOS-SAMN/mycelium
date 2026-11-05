// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ACT — THE SIGNED PERMUTATIONS OF THE `n` PHASES (D3.2 §D.1, D.2, at rank
//! `n`). Among the maps of `(ℚ/ℤ)ⁿ` that carry arcs to arcs (`GL(n, ℤ)`),
//! those that carry the seam set onto itself are exactly the signed
//! permutations of the phases: the matrix's `k`-th image vector is the image of a
//! coordinate circle, and for that circle to stay on the seams it must be
//! `±eₖ` for one `k`. They form the hyperoctahedral group, `2ⁿ·n!`
//! elements, generated here and never listed. At rank 2 they are the eight
//! signed swaps of two circles. A turn carries no period: a permutation
//! moves the turns of one phase onto another and the periods travel with
//! the enclosure, so a quarter turn followed by any re-period is exact on
//! turns. The only obstruction is at projection, where a moved boundary turn
//! must land on a boundary of the output.

use super::{Band, Enclosed, Enclosure, TransductionRefusal, Turn};
use crate::transducer::sample::Sample;

/// THE SENSE A PHASE IS CARRIED IN: kept, or reversed `t ↦ −t`. A reversal
/// carries the arc `[a, b)` to `[1 − b, 1 − a)`: it fixes the seam and the
/// turn of order two, and never crosses either.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sense {
    Kept,
    Reversed,
}

impl Sense {
    /// The sense of two carried one after the other.
    fn then(self, other: Sense) -> Sense {
        match self == other {
            true => Sense::Kept,
            false => Sense::Reversed,
        }
    }

    /// The sense as the matrix entry `±1`.
    fn sign(self) -> i64 {
        match self {
            Sense::Kept => 1,
            Sense::Reversed => -1,
        }
    }
}

/// ONE SIGNED PERMUTATION of the `n` phases: output phase `j` reads source
/// phase `images[j].0` in sense `images[j].1`, so a site's turns `t` move to
/// `t′ⱼ = ±t_{σ(j)}`. No phase is privileged: the identity is one element
/// among `2ⁿ·n!`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedPermutation {
    images: Vec<(usize, Sense)>,
}

/// THE GROUP OF RANK `n`, GENERATED: the permutations in lexicographic
/// order, and for each the senses counted with the last phase the least
/// significant. The identity comes first. Each element is produced from the
/// one before; no table is written.
pub struct Generated {
    order: Option<Vec<usize>>,
    senses: Vec<Sense>,
}

impl Iterator for Generated {
    type Item = SignedPermutation;

    fn next(&mut self) -> Option<SignedPermutation> {
        let order = self.order.as_mut()?;
        let out = SignedPermutation { images: order.iter().copied().zip(self.senses.iter().copied()).collect() };
        // The senses advance as a count; where the count returns to all
        // kept, the permutation advances to its lexicographic successor.
        let mut carried = true;
        for s in self.senses.iter_mut().rev() {
            match *s {
                Sense::Kept => {
                    *s = Sense::Reversed;
                    carried = false;
                    break;
                }
                Sense::Reversed => *s = Sense::Kept,
            }
        }
        if carried && !next_order(order) {
            self.order = None;
        }
        Some(out)
    }
}

/// The lexicographic successor of `order` in place, or `false` at the last.
fn next_order(order: &mut [usize]) -> bool {
    let Some(i) = (1..order.len()).rev().find(|&i| order.get(i - 1) < order.get(i)) else { return false };
    let pivot = i - 1;
    let Some(j) = (i..order.len()).rev().find(|&j| order.get(j) > order.get(pivot)) else { return false };
    order.swap(pivot, j);
    if let Some(tail) = order.get_mut(i..) {
        tail.reverse();
    }
    true
}

impl SignedPermutation {
    /// A SIGNED PERMUTATION FROM ITS IMAGES, or `Oblique` where the sources
    /// are not each phase exactly once: such a map moves the seam set.
    pub fn new(images: Vec<(usize, Sense)>) -> Result<SignedPermutation, TransductionRefusal> {
        let mut seen = vec![0usize; images.len()];
        for &(k, _) in &images {
            match seen.get_mut(k) {
                Some(n) if *n == 0 => *n = 1,
                _ => return Err(TransductionRefusal::Oblique),
            }
        }
        Ok(SignedPermutation { images })
    }

    /// The identity of rank `n`.
    pub fn identity(rank: usize) -> SignedPermutation {
        SignedPermutation { images: (0..rank).map(|k| (k, Sense::Kept)).collect() }
    }

    /// EVERY SIGNED PERMUTATION OF RANK `n`, generated ([`Generated`]).
    pub fn all(rank: usize) -> Generated {
        Generated { order: Some((0..rank).collect()), senses: vec![Sense::Kept; rank] }
    }

    /// The rank the permutation acts on.
    pub fn rank(&self) -> usize {
        self.images.len()
    }

    /// The images, output phase by output phase.
    pub fn images(&self) -> &[(usize, Sense)] {
        &self.images
    }

    /// `self` and then `other`: output phase `j` of the composite reads
    /// `self`'s image of `other`'s source. `Rank` where the ranks differ.
    pub fn then(&self, other: &SignedPermutation) -> Result<SignedPermutation, TransductionRefusal> {
        if self.rank() != other.rank() {
            return Err(TransductionRefusal::Rank { from: self.rank(), to: other.rank() });
        }
        let mut images = Vec::with_capacity(self.rank());
        for &(k, s) in &other.images {
            let &(source, first) = self.images.get(k).ok_or(TransductionRefusal::Oblique)?;
            images.push((source, first.then(s)));
        }
        Ok(SignedPermutation { images })
    }

    /// The inverse: source phase `k` read back from the output phase that
    /// carried it, in the same sense.
    pub fn inverse(&self) -> SignedPermutation {
        let mut images = vec![(0usize, Sense::Kept); self.rank()];
        for (j, &(k, s)) in self.images.iter().enumerate() {
            if let Some(slot) = images.get_mut(k) {
                *slot = (j, s);
            }
        }
        SignedPermutation { images }
    }

    /// The integer matrix on the turns, `t′ = M·t`: line `j` holds `±1` at
    /// its source phase and 0 elsewhere.
    pub fn matrix(&self) -> Vec<Vec<i64>> {
        self.images
            .iter()
            .map(|&(k, s)| (0..self.rank()).map(|c| if c == k { s.sign() } else { 0 }).collect())
            .collect()
    }

    /// THE SIGNED PERMUTATION A MATRIX IS, or `Oblique`: a square integer
    /// matrix outside the group (a shear, a scale, an entry off `{−1, 0,
    /// 1}`, two entries in a line, two lines on one source) moves the seam
    /// set, and no motion of the worksheet is read from it.
    pub fn of_matrix(m: &[Vec<i64>]) -> Result<SignedPermutation, TransductionRefusal> {
        let mut images = Vec::with_capacity(m.len());
        for line in m {
            if line.len() != m.len() {
                return Err(TransductionRefusal::Oblique);
            }
            let mut held = None;
            for (k, &x) in line.iter().enumerate() {
                match (x, held) {
                    (0, _) => {}
                    (1, None) => held = Some((k, Sense::Kept)),
                    (-1, None) => held = Some((k, Sense::Reversed)),
                    _ => return Err(TransductionRefusal::Oblique),
                }
            }
            images.push(held.ok_or(TransductionRefusal::Oblique)?);
        }
        SignedPermutation::new(images)
    }

    /// THE SIGNED PERMUTATION A WORD OF THE ALPHABET IS on the alphabet's
    /// sensor plane, read off its images of the plane's two generators (law
    /// 11: an action held by its images). A word that does not keep the
    /// plane, or whose block is not a signed permutation (the `(3, 4, 5)`
    /// and `(5, 12, 13)` letters), has no action on turns: `Oblique`.
    pub fn of_word(word: &crate::transducer::alphabet::Word) -> Result<SignedPermutation, TransductionRefusal> {
        let block = word.plane_block().ok_or(TransductionRefusal::Oblique)?;
        let entry = |q: &crate::core::Q| -> Result<i64, TransductionRefusal> {
            if !q.is_integer() {
                return Err(TransductionRefusal::Oblique);
            }
            i64::try_from(q.to_integer()).map_err(|_| TransductionRefusal::Oblique)
        };
        let m = block.iter().map(|line| line.iter().map(entry).collect()).collect::<Result<Vec<Vec<i64>>, _>>()?;
        SignedPermutation::of_matrix(&m)
    }

    /// A SITE'S BOUNDARY TURNS MOVED, one per phase of `self`'s rank: each
    /// output phase takes its source's turn, reversed as `p − i` of the same
    /// period `p`. `Rank` where the turns are of another rank.
    pub fn carry(&self, turns: &[Turn]) -> Result<Vec<Turn>, TransductionRefusal> {
        if turns.len() != self.rank() {
            return Err(TransductionRefusal::Rank { from: turns.len(), to: self.rank() });
        }
        let mut out = Vec::with_capacity(turns.len());
        for &(k, s) in &self.images {
            let &t = turns.get(k).ok_or(TransductionRefusal::Oblique)?;
            out.push(match s {
                Sense::Kept => t,
                Sense::Reversed => t.reversed(),
            });
        }
        Ok(out)
    }

    /// The enclosure a frame of `e` is held in once moved: each period
    /// travels with its phase.
    pub fn enclosure(&self, e: &Enclosure) -> Result<Enclosure, TransductionRefusal> {
        if e.rank() != self.rank() {
            return Err(TransductionRefusal::Rank { from: e.rank(), to: self.rank() });
        }
        Enclosure::new(self.images.iter().filter_map(|&(k, _)| e.periods().get(k).copied()).collect())
    }

    /// A band moved, in the moved enclosure's indices: each output phase
    /// takes its source's span, and a reversed span `[a, b)` of period `p`
    /// reads `[p − b, p − a)`.
    pub fn band(&self, b: &Band, e: &Enclosure) -> Band {
        let spans = self
            .images
            .iter()
            .filter_map(|&(k, s)| {
                let (lo, hi) = *b.spans.get(k)?;
                let p = *e.periods().get(k)?;
                Some(match s {
                    Sense::Kept => (lo, hi),
                    Sense::Reversed => (p - hi, p - lo),
                })
            })
            .collect();
        Band::new(spans, b.symbol())
    }

    /// PROJECT A MOVED FRAME (D3.2 §E): the frame `e` moved by this
    /// permutation and re-periodded onto `to`, spoken as a sample. Each
    /// moved band's boundary turns must land on boundaries of `to`, or the
    /// projection refuses `OffSite` naming the first that does not: the
    /// obstruction a quarter turn between frames of different extent meets,
    /// and the only one. On turns the motion is exact.
    pub fn spoken_onto(&self, e: &Enclosed, to: &Enclosure) -> Result<Sample, TransductionRefusal> {
        let enclosure = self.enclosure(&e.enclosure)?;
        let bands = e.bands.iter().map(|b| self.band(b, &e.enclosure)).collect();
        e.with_bands(enclosure, bands).spoken_onto(to)
    }

    /// THE FRAME MOVED, lifted again in its own moved enclosure: the moved
    /// bands spoken onto the moved enclosure's own periods (always landing)
    /// and read back as bands, so the moved frame's bands are maximal again.
    pub fn moved(&self, e: &Enclosed) -> Result<Enclosed, TransductionRefusal> {
        let own = self.enclosure(&e.enclosure)?;
        Ok(Enclosed::of(&self.spoken_onto(e, &own)?))
    }
}
