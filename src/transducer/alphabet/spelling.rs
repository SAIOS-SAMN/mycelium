// A2, 20 — THE SPELLING: which word, if any, spells a given block.
//
//! `spell` peels a 2x2 rational block into syllables of the alphabet under
//! a shrinking governor, and refuses when no word spells it. Each step reads
//! the block's own unit and the residual it leaves, never a candidate list.

use crate::transducer::alphabet::{rational_sqrt, LetterId, Word, Q};
use crate::transducer::peel::{peel_nest, PeelNest, ShellMotion, ShellStep, ShrinkGovernor};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};

use super::*;

/// A SPELLING: the alphabet word of a similarity. Dilation lives on the
/// carriage, once; it is not a second coordinate here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spelling {
    pub word: Word,
}

impl Spelling {
    /// The 2×2 block the word speaks (rotation and orientation). ρ is not
    /// here.
    pub fn block(&self) -> Option<[[Q; 2]; 2]> {
        self.word.plane_block()
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        self.word.to_bytes(out);
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Spelling> {
        let word = Word::from_bytes(b, pos)?;
        Some(Spelling { word })
    }
}

/// A Gaussian integer `p + q·i`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Gauss {
    p: BigInt,
    q: BigInt,
}

impl Gauss {
    fn new(p: i64, q: i64) -> Gauss {
        Gauss { p: BigInt::from(p), q: BigInt::from(q) }
    }
    fn mul(&self, o: &Gauss) -> Gauss {
        Gauss { p: &self.p * &o.p - &self.q * &o.q, q: &self.p * &o.q + &self.q * &o.p }
    }
    fn conj(&self) -> Gauss {
        Gauss { p: self.p.clone(), q: -&self.q }
    }
    fn norm(&self) -> BigInt {
        &self.p * &self.p + &self.q * &self.q
    }
    /// Exact division, or `None` when `o` does not divide `self`.
    fn div_exact(&self, o: &Gauss) -> Option<Gauss> {
        let n = o.norm();
        let num = self.mul(&o.conj());
        let (pd, pr) = num.p.div_rem(&n);
        let (qd, qr) = num.q.div_rem(&n);
        if !pr.is_zero() || !qr.is_zero() {
            return None;
        }
        Some(Gauss { p: pd, q: qd })
    }
    /// The valuation at `pi`: how many times `pi` divides exactly, and
    /// what remains. A peel — each division strictly lowers the norm.
    fn valuation(&self, pi: &Gauss) -> (i64, Gauss) {
        let mut v = 0i64;
        let mut z = self.clone();
        while let Some(next) = z.div_exact(pi) {
            if next.norm() >= z.norm() {
                break; // a unit divisor would loop; the alphabet's primes never are
            }
            z = next;
            v += 1;
        }
        (v, z)
    }
}

/// The residual of the spelling peel: the rotation as a Gaussian integer
/// over its weight, and the orientation sheet still to be peeled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SpellResidual {
    z: Gauss,
    n: BigInt,
    flipped: bool,
}

/// One syllable of the tape.
pub(crate) type Syllable = Vec<(LetterId, bool)>;

/// The alphabet's own actions on one pair plane, read once from the letters
/// so every orientation convention is settled by the algebra itself. The
/// (5,12,13) letter exists on four planes only (1.1); on the other two the
/// prime 13 is outside the plane's alphabet and stands as a remainder.
pub(crate) struct SpellStep {
    quarter: LetterId,
    l5: LetterId,
    l13: Option<LetterId>,
    flip: LetterId,
    /// The quarter letter's action on `e0`, as a Gaussian unit.
    quarter_unit: Gauss,
    /// Whether the forward (3,4,5) letter realises `((2+i)/(2−i))²` (true)
    /// or its conjugate.
    l5_forward: bool,
    l13_forward: bool,
    pi5: Gauss,
    pi13: Gauss,
}

pub(crate) fn unit_of_block(b: &[[Q; 2]; 2]) -> Option<Gauss> {
    let as_int = |v: &Q| -> Option<BigInt> {
        if v.denom().is_one() {
            Some(v.numer().clone())
        } else {
            None
        }
    };
    Some(Gauss { p: as_int(&b[0][0])?, q: as_int(&b[1][0])? })
}

impl SpellStep {
    /// The step on one pair plane (50.4). The orientation sheet is the
    /// half-turn through the least axis outside the plane, realised as
    /// two quarter letters on the plane it shares with `plane.1` (1.2).
    fn on(plane: (u8, u8)) -> Option<SpellStep> {
        let quarter = LetterId::quarter(plane)?;
        let l5 = LetterId::pythagorean_5(plane)?;
        let l13 = LetterId::pythagorean_13(plane);
        let third = (0u8..4).find(|k| *k != plane.0 && *k != plane.1)?;
        let flip = LetterId::quarter((plane.1.min(third), plane.1.max(third)))?;
        let quarter_unit = unit_of_block(&Word::raw(vec![(quarter, false)]).block_on(plane)?)?;
        let pi5 = Gauss::new(2, 1);
        let pi13 = Gauss::new(3, 2);
        // ((2+i)/(2−i))² = (2+i)⁴/25 and ((3+2i)/(3−2i))² = (3+2i)⁴/169:
        // compare with the letters' measured actions.
        let t5 = pi5.mul(&pi5).mul(&pi5).mul(&pi5);
        let t13 = pi13.mul(&pi13).mul(&pi13).mul(&pi13);
        let action = |id: LetterId, weight: i64| -> Option<Gauss> {
            let b = Word::raw(vec![(id, false)]).block_on(plane)?;
            let w = q(weight);
            unit_of_block(&[[&b[0][0] * &w, &b[0][1] * &w], [&b[1][0] * &w, &b[1][1] * &w]])
        };
        let a5 = action(l5, 25)?;
        let l5_forward = if a5 == t5 {
            true
        } else if a5 == t5.conj() {
            false
        } else {
            return None;
        };
        let l13_forward = match l13 {
            None => true,
            Some(id) => {
                let a13 = action(id, 169)?;
                if a13 == t13 {
                    true
                } else if a13 == t13.conj() {
                    false
                } else {
                    return None;
                }
            }
        };
        Some(SpellStep { quarter, l5, l13, flip, quarter_unit, l5_forward, l13_forward, pi5, pi13 })
    }

    /// The signed prime exponents `(m5, m13)` and the unit remainder of a
    /// residual — `None` when a prime outside the alphabet remains, the
    /// prime 13 included on a plane that has no (5,12,13) letter.
    fn read(&self, r: &SpellResidual) -> Option<(i64, i64, Gauss)> {
        let (v5p, z) = r.z.valuation(&self.pi5);
        let (v5m, z) = z.valuation(&self.pi5.conj());
        let (v13p, z) = z.valuation(&self.pi13);
        let (v13m, rest) = z.valuation(&self.pi13.conj());
        if self.l13.is_none() && (v13p != 0 || v13m != 0) {
            return None;
        }
        // The weight after the same primes leave it.
        let mut n = r.n.clone();
        for _ in 0..(v5p + v5m) / 2 {
            let (d, m) = n.div_rem(&BigInt::from(5));
            if !m.is_zero() {
                return None;
            }
            n = d;
        }
        for _ in 0..(v13p + v13m) / 2 {
            let (d, m) = n.div_rem(&BigInt::from(13));
            if !m.is_zero() {
                return None;
            }
            n = d;
        }
        if (v5p + v5m) % 2 != 0 || (v13p + v13m) % 2 != 0 {
            return None;
        }
        let unit = rest.div_exact(&Gauss { p: n, q: BigInt::zero() })?;
        if !unit.norm().is_one() {
            return None;
        }
        // On the circle the prime's exponent is half the valuation
        // difference (the weight carries the other half); one letter is
        // two of it, so an odd exponent is a half letter.
        Some(((v5p - v5m) / 2, (v13p - v13m) / 2, unit))
    }

    /// Steps of the quarter letter from a unit back to 1.
    fn unit_steps(&self, u: &Gauss) -> Option<u64> {
        let one = Gauss::new(1, 0);
        let mut cur = one.clone();
        for k in 0..4u64 {
            if &cur == u {
                return Some(k.min(4 - k));
            }
            cur = cur.mul(&self.quarter_unit);
        }
        None
    }

    fn distance(&self, r: &SpellResidual) -> Option<u64> {
        let (m5, m13, unit) = self.read(r)?;
        Some(r.flipped as u64 + m5.unsigned_abs() + m13.unsigned_abs() + self.unit_steps(&unit)?)
    }
}

impl ShellStep for SpellStep {
    type Residual = SpellResidual;
    type Core = Syllable;

    /// Peel one syllable, outer shell first: the orientation sheet, then a
    /// prime-5 letter, then a prime-13 letter, then a unit turn. `None`
    /// when what remains is not a whole letter — the arc rides out.
    fn shell(&self, r: &SpellResidual) -> Option<(Syllable, SpellResidual)> {
        if r.flipped {
            return Some((vec![(self.flip, false), (self.flip, false)], SpellResidual { flipped: false, ..r.clone() }));
        }
        let (m5, m13, unit) = self.read(r)?;
        let remove = |z: &Gauss, pi: &Gauss, forward: bool| -> Option<Gauss> {
            let conj = pi.conj();
            let (out, back): (&Gauss, &Gauss) = if forward { (pi, &conj) } else { (&conj, pi) };
            Some(z.div_exact(out)?.div_exact(out)?.mul(back).mul(back))
        };
        if m5.abs() >= 2 {
            let forward = m5 > 0;
            let z = remove(&r.z, &self.pi5, forward)?;
            return Some((vec![(self.l5, forward != self.l5_forward)], SpellResidual { z, ..r.clone() }));
        }
        if m5 != 0 {
            return None; // a half letter: not in the language
        }
        if m13.abs() >= 2 {
            let l13 = self.l13?;
            let forward = m13 > 0;
            let z = remove(&r.z, &self.pi13, forward)?;
            return Some((vec![(l13, forward != self.l13_forward)], SpellResidual { z, ..r.clone() }));
        }
        if m13 != 0 {
            return None;
        }
        // The unit: one quarter letter toward 1, oriented by the shorter arc.
        let one = Gauss::new(1, 0);
        if unit == one {
            return None; // identity: the governor reads Closed before this
        }
        let q_inv = self.quarter_unit.conj();
        let forward = unit == self.quarter_unit || unit == self.quarter_unit.mul(&self.quarter_unit);
        let (id_inv, step) = if forward { (false, q_inv) } else { (true, self.quarter_unit.clone()) };
        let z = r.z.mul(&step);
        Some((vec![(self.quarter, id_inv)], SpellResidual { z, ..r.clone() }))
    }
}

pub(crate) struct SpellGovernor<'a> {
    step: &'a SpellStep,
    last: Option<u64>,
}

impl ShrinkGovernor for SpellGovernor<'_> {
    type Residual = SpellResidual;
    fn read(&mut self, r: &SpellResidual) -> ShellMotion {
        if !r.flipped && r.z.q.is_zero() && r.z.p == r.n {
            return ShellMotion::Closed;
        }
        let d = self.step.distance(r);
        let motion = match (d, self.last) {
            (Some(now), Some(before)) if now < before => ShellMotion::Shrinking,
            (Some(_), None) => ShellMotion::Shrinking,
            _ => ShellMotion::Stalled,
        };
        self.last = d;
        motion
    }
}

/// SPELL a rational similarity of the plane (rotation, reflection and
/// dilation; columns the images of the row and column steps) into the
/// alphabet. `None` when the block is not a similarity, its dilation is
/// not rational, or its rotation has a Gaussian factor outside the
/// alphabet's primes — a remainder, never a rounding.
pub fn spell(block: &[[Q; 2]; 2]) -> Option<Spelling> {
    match spell_peel(block)? {
        (PeelNest::Closed { cores }, rho) => {
            // Cores come out outer to inner (the sheet first); the tape's
            // last letter acts first, so the tape is the cores reversed.
            let mut word = Word::identity();
            for syllable in cores.iter().rev() {
                for &(id, inv) in syllable {
                    word.push(id, inv);
                }
            }
            let sp = Spelling { word };
            let b = sp.block()?;
            // The word is the rotation; ρ is peeled for the caller to put
            // on the carriage. The spelled linear block times ρ is the
            // sensor block.
            for r in 0..2 {
                for c in 0..2 {
                    if &b[r][c] * &rho != block[r][c] {
                        return None;
                    }
                }
            }
            Some(sp)
        }
        (PeelNest::Standing { .. }, _) => None,
    }
}

/// The spelling peel itself: the nest of syllables and, when the peel
/// stalls, the standing arc (the Gaussian remainder the alphabet cannot
/// speak) — carried out whole for the caller to hold as thought.
pub fn spell_peel(block: &[[Q; 2]; 2]) -> Option<(PeelNest<SpellResidualView, Syllable>, Q)> {
    spell_peel_on((PLANE_ROW, PLANE_COL), block)
}

/// The same peel on any pair plane (50.4, A22): the block's rows and
/// columns are the plane's own two axes, and the letters are that plane's.
pub fn spell_peel_on(plane: (u8, u8), block: &[[Q; 2]; 2]) -> Option<(PeelNest<SpellResidualView, Syllable>, Q)> {
    let (a, b, c, d) = (&block[0][0], &block[0][1], &block[1][0], &block[1][1]);
    let rho2 = a * a + c * c;
    if rho2.is_zero() || (b * b + d * d) != rho2 || !(a * b + c * d).is_zero() {
        return None;
    }
    let rho = rational_sqrt(&rho2)?;
    let (r00, r10) = (a / &rho, c / &rho);
    let (r01, r11) = (b / &rho, d / &rho);
    let det = &r00 * &r11 - &r01 * &r10;
    let flipped = if det.is_one() {
        false
    } else if (-&det).is_one() {
        true
    } else {
        return None;
    };
    // Rotation form after the sheet: r11 = r00 and r01 = −r10.
    let (r01, r11) = if flipped { (-r01, -r11) } else { (r01, r11) };
    if r11 != r00 || r01 != -&r10 {
        return None;
    }
    let n = r00.denom().lcm(r10.denom());
    let z = Gauss { p: (&r00 * &n).to_integer(), q: (&r10 * &n).to_integer() };
    if z.norm() != &n * &n {
        return None;
    }
    let step = SpellStep::on(plane)?;
    let mut governor = SpellGovernor { step: &step, last: None };
    let nest = peel_nest(SpellResidual { z, n, flipped }, &step, &mut governor);
    let view = match nest {
        PeelNest::Closed { cores } => PeelNest::Closed { cores },
        PeelNest::Standing { cores, arc } => PeelNest::Standing {
            cores,
            arc: SpellResidualView { p: arc.z.p, q: arc.z.q, weight: arc.n, flipped: arc.flipped },
        },
    };
    Some((view, rho))
}

/// The standing arc of a spelling that did not close: the rotation the
/// alphabet cannot speak, as `(p + q·i)/weight` and the unpeeled sheet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellResidualView {
    pub p: BigInt,
    pub q: BigInt,
    pub weight: BigInt,
    pub flipped: bool,
}

/// The block of a lattice isometry given by the images of the row step
/// and the column step — the sensor's way of reading a motion, spelled.
pub fn spell_images(row_image: (i64, i64), col_image: (i64, i64)) -> Option<Spelling> {
    spell(&[[q(row_image.0), q(col_image.0)], [q(row_image.1), q(col_image.1)]])
}
