// A2, 20 — THE SPELLING: which word, if any, spells a given block.
//
//! `spell` peels a 2x2 rational block into syllables of the alphabet under
//! a shrinking governor, and refuses when no word spells it. Each step reads
//! the block's own unit and the residual it leaves, never a candidate list.

use crate::core::gaussian::Gauss;
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

    /// The word whole (LD0 step 6): a spelling's word is spelled, and the
    /// writing says so rather than assuming it.
    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        self.word.put_whole(out);
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Spelling> {
        let word = Word::take_whole(b, pos)?;
        Some(Spelling { word })
    }
}

// `Gauss` was defined here, privately, with fields `p, q` and its own
// `mul`, `conj`, `norm`, `div_exact` and `valuation` — a second Gaussian
// integer beside `core::gaussian::Gauss`, which had `mul`, `conj`, `norm`
// and the Euclidean division already. One object in two carriers is law
// 12's own shape, and the two could drift on a sign or a rounding without
// anything noticing. The peel uses the core's, which gained `div_exact`
// and `valuation` (the exact division and the shell count) to serve it.

/// THE ADJOINED SHELLS A RESIDUAL CARRIES: which generators of the basis
/// divide it and at what signed exponent, with the arc and the weight
/// after they all leave.
pub(crate) struct AdjoinedShells {
    cited: Vec<(usize, i64)>,
    arc: Gauss,
    weight: BigInt,
}

/// WHAT ONE READING OF A RESIDUAL FOUND: the alphabet's own prime
/// exponents, the adjoined generators that divide it with their signed
/// exponents, and the unit that is left. Named fields rather than a tuple,
/// because the tuple grew a fourth member and `(m5, m13, cited, unit)` at
/// four call sites is a positional read of a carrier (rule 15).
pub(crate) struct Read {
    m5: i64,
    m13: i64,
    cited: Vec<(usize, i64)>,
    unit: Gauss,
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

/// A GENERATOR THE PEEL MAY DIVIDE BY BEYOND THE SEVENTEEN (37.7, L23a):
/// the Gaussian its rotor is, and the citation that names it.
///
/// The ring branch was a second division loop over a second basis — the
/// crystal's adjoined generators — sitting beside this one and reached
/// only where this one had already failed. They are one loop over one
/// basis. A mind that holds an adjoined generator peels with it; a mind
/// that holds none peels with the seventeen alone and reads exactly what
/// it read before.
#[derive(Clone, Debug)]
pub struct AdjoinedPrime {
    pub pi: Gauss,
    pub form: crate::transducer::address::Address,
    pub vertex: crate::transducer::address::Address,
    /// The generator's own rotor, as the presentation answered it. The
    /// composition is replayed from THIS and not from a second lookup by
    /// address: the mind already answered what it holds, and asking again
    /// through another index is a second carrier for one answer.
    pub rotor: super::Rotor,
}

/// ONE SHELL THE PEEL TOOK. A letter of the seventeen, or a citation of a
/// generator the alphabet does not contain.
///
/// The core was `Syllable` alone, so a shell could only ever be letters
/// and the ring had to live elsewhere to say anything else. A word is
/// letters and refuses a citation; a DERIVATION carries both, which is
/// what `Derivation::factored` is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpellCore {
    Letters(Syllable),
    Cited {
        form: crate::transducer::address::Address,
        vertex: crate::transducer::address::Address,
        /// The signed exponent this shell removed: `+1` for the prime,
        /// `−1` for its conjugate.
        exponent: i64,
    },
}

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
    /// THE BASIS BEYOND THE SEVENTEEN (37.7, L23a): the adjoined
    /// generators this mind holds, in key order. Empty is a mind that
    /// holds none, and the peel then reads exactly what it always read.
    adjoined: Vec<AdjoinedPrime>,
}

pub(crate) fn unit_of_block(b: &[[Q; 2]; 2]) -> Option<Gauss> {
    let as_int = |v: &Q| -> Option<BigInt> {
        if v.denom().is_one() {
            Some(v.numer().clone())
        } else {
            None
        }
    };
    Some(Gauss::new(as_int(&b[0][0])?, as_int(&b[1][0])?))
}

impl SpellStep {
    /// The step on one pair plane (50.4). The orientation sheet is the
    /// half-turn through the least axis outside the plane, realised as
    /// two quarter letters on the plane it shares with `plane.1` (1.2).
    /// The step on one pair plane over a basis (50.4, L23a): the
    /// alphabet's own primes and whatever the caller adjoined. An empty
    /// basis is the seventeen alone.
    fn on_with(plane: (u8, u8), adjoined: Vec<AdjoinedPrime>) -> Option<SpellStep> {
        let quarter = LetterId::quarter(plane)?;
        let l5 = LetterId::pythagorean_5(plane)?;
        let l13 = LetterId::pythagorean_13(plane);
        let third = (0u8..4).find(|k| *k != plane.0 && *k != plane.1)?;
        let flip = LetterId::quarter((plane.1.min(third), plane.1.max(third)))?;
        let quarter_unit = unit_of_block(&Word::raw(vec![(quarter, false)]).block_on(plane)?)?;
        let pi5 = Gauss::from_i64(2, 1);
        let pi13 = Gauss::from_i64(3, 2);
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
        Some(SpellStep {
            quarter,
            l5,
            l13,
            flip,
            quarter_unit,
            l5_forward,
            l13_forward,
            pi5,
            pi13,
            adjoined,
        })
    }

    /// THE ADJOINED SHELLS a residual carries: the index of each generator
    /// that divides it and the signed exponent, with the arc and the weight
    /// after they all leave.
    ///
    /// The exponent is the VALUATION ITSELF, not half of it. The alphabet's
    /// `pi5` is the PRIME `2 + i` and its letter realises `(π̄/π)²`, so a
    /// letter is two shells of the prime and the exponent halves. An
    /// adjoined generator's `pi` is its ROTOR'S ARC, which is already the
    /// rotor squared, so one exact division is one application of the
    /// generator — which is what `a_mind_taught_an_adjoined_generator`
    /// asserts by taking `π₁₇` itself and reading exponent one.
    ///
    /// The weight loses `√N(pi)` per shell, and that root is a rational
    /// integer exactly because the arc is a square. An adjoined `pi` that
    /// is not one fails the division and the shell is simply not taken —
    /// refused by the arithmetic rather than by a guard ahead of it.
    fn adjoined_shells(&self, z: &Gauss, n: &BigInt) -> Option<AdjoinedShells> {
        let (mut z, mut n) = (z.clone(), n.clone());
        let mut cited: Vec<(usize, i64)> = Vec::new();
        for (i, gen) in self.adjoined.iter().enumerate() {
            if gen.pi.is_unit() || gen.pi.is_zero() {
                continue;
            }
            let root = gen.pi.norm().sqrt();
            if &root * &root != gen.pi.norm() {
                continue;
            }
            let (vp, rest) = z.valuation(&gen.pi);
            let (vm, rest) = rest.valuation(&gen.pi.conj());
            if vp == 0 && vm == 0 {
                continue;
            }
            let mut w = n.clone();
            for _ in 0..(vp + vm) {
                let (d, m) = w.div_rem(&root);
                if !m.is_zero() {
                    return None;
                }
                w = d;
            }
            z = rest;
            n = w;
            cited.push((i, vp - vm));
        }
        Some(AdjoinedShells { cited, arc: z, weight: n })
    }

    /// The signed prime exponents, the adjoined shells, and the unit
    /// remainder of a residual — `None` when a prime outside the BASIS
    /// remains, the prime 13 included on a plane that has no (5,12,13)
    /// letter. The basis is the seventeen plus whatever the mind adjoined.
    fn read(&self, r: &SpellResidual) -> Option<Read> {
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
        // THE BASIS BEYOND THE SEVENTEEN, after the alphabet's own. A mind
        // that holds no adjoined generator reads exactly what it read
        // before this line existed; a mind that holds one divides by it
        // here, where the ring used to have to run afterwards on a target
        // this reader had already refused.
        let AdjoinedShells { cited, arc: rest, weight: n } = self.adjoined_shells(&rest, &n)?;
        let unit = rest.div_exact(&Gauss::new(n, BigInt::zero()))?;
        if !unit.norm().is_one() {
            return None;
        }
        // On the circle the prime's exponent is half the valuation
        // difference (the weight carries the other half); one letter is
        // two of it, so an odd exponent is a half letter.
        Some(Read { m5: (v5p - v5m) / 2, m13: (v13p - v13m) / 2, cited, unit })
    }

    /// Steps of the quarter letter from a unit back to 1.
    fn unit_steps(&self, u: &Gauss) -> Option<u64> {
        let one = Gauss::from_i64(1, 0);
        let mut cur = one.clone();
        for k in 0..4u64 {
            if &cur == u {
                return Some(k.min(4 - k));
            }
            cur = cur.mul(&self.quarter_unit);
        }
        None
    }

    /// How far this residual is from closing, in shells. The adjoined
    /// shells count too: a residual a generator divides is one shell
    /// nearer than one it does not, and leaving them out would let the
    /// governor read a peel as stalled while the basis was still removing
    /// content.
    fn distance(&self, r: &SpellResidual) -> Option<u64> {
        let read = self.read(r)?;
        let cited: u64 = read.cited.iter().map(|(_, e)| e.unsigned_abs()).sum();
        Some(
            r.flipped as u64
                + read.m5.unsigned_abs()
                + read.m13.unsigned_abs()
                + cited
                + self.unit_steps(&read.unit)?,
        )
    }
}

impl ShellStep for SpellStep {
    type Residual = SpellResidual;
    type Core = SpellCore;

    /// Peel one shell, outer first: the orientation sheet, then a prime-5
    /// letter, then a prime-13 letter, then an ADJOINED generator, then a
    /// unit turn. `None` when what remains is not a whole shell of the
    /// basis — the arc rides out.
    ///
    /// The adjoined shells come after the alphabet's own, so the seventeen
    /// are always preferred and a mind that holds a generator still spells
    /// with letters whatever it can.
    fn shell(&self, r: &SpellResidual) -> Option<(SpellCore, SpellResidual)> {
        if r.flipped {
            return Some((
                SpellCore::Letters(vec![(self.flip, false), (self.flip, false)]),
                SpellResidual { flipped: false, ..r.clone() },
            ));
        }
        let Read { m5, m13, cited, unit } = self.read(r)?;
        let remove = |z: &Gauss, pi: &Gauss, forward: bool| -> Option<Gauss> {
            let conj = pi.conj();
            let (out, back): (&Gauss, &Gauss) = if forward { (pi, &conj) } else { (&conj, pi) };
            Some(z.div_exact(out)?.div_exact(out)?.mul(back).mul(back))
        };
        if m5.abs() >= 2 {
            let forward = m5 > 0;
            let z = remove(&r.z, &self.pi5, forward)?;
            return Some((
                SpellCore::Letters(vec![(self.l5, forward != self.l5_forward)]),
                SpellResidual { z, ..r.clone() },
            ));
        }
        if m5 != 0 {
            return None; // a half letter: not in the language
        }
        if m13.abs() >= 2 {
            let l13 = self.l13?;
            let forward = m13 > 0;
            let z = remove(&r.z, &self.pi13, forward)?;
            return Some((
                SpellCore::Letters(vec![(l13, forward != self.l13_forward)]),
                SpellResidual { z, ..r.clone() },
            ));
        }
        if m13 != 0 {
            return None;
        }
        // AN ADJOINED SHELL (37.7). One application of the generator per
        // shell, forward or by its conjugate, in the basis's key order.
        if let Some((i, e)) = cited.iter().find(|(_, e)| *e != 0) {
            let gen = self.adjoined.get(*i)?;
            let forward = *e > 0;
            let out = if forward { gen.pi.clone() } else { gen.pi.conj() };
            let z = r.z.div_exact(&out)?;
            let root = gen.pi.norm().sqrt();
            let (n, m) = r.n.div_rem(&root);
            if !m.is_zero() {
                return None;
            }
            return Some((
                SpellCore::Cited {
                    form: gen.form.clone(),
                    vertex: gen.vertex.clone(),
                    exponent: if forward { 1 } else { -1 },
                },
                SpellResidual { z, n, ..r.clone() },
            ));
        }
        // The unit: one quarter letter toward 1, oriented by the shorter arc.
        let one = Gauss::from_i64(1, 0);
        if unit == one {
            return None; // identity: the governor reads Closed before this
        }
        let q_inv = self.quarter_unit.conj();
        let forward = unit == self.quarter_unit || unit == self.quarter_unit.mul(&self.quarter_unit);
        let (id_inv, step) = if forward { (false, q_inv) } else { (true, self.quarter_unit.clone()) };
        let z = r.z.mul(&step);
        Some((SpellCore::Letters(vec![(self.quarter, id_inv)]), SpellResidual { z, ..r.clone() }))
    }
}

pub(crate) struct SpellGovernor<'a> {
    step: &'a SpellStep,
    last: Option<u64>,
}

impl ShrinkGovernor for SpellGovernor<'_> {
    type Residual = SpellResidual;
    fn read(&mut self, r: &SpellResidual) -> ShellMotion {
        if !r.flipped && r.z.b.is_zero() && r.z.a == r.n {
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
    spell_on((PLANE_ROW, PLANE_COL), block)
}

/// THE SAME SPELLING ON ANY PAIR PLANE (50.4, A22). `spell_peel_on`
/// already took a plane and `SpellStep::on_with` already derives that
/// plane's own quarter, primes and flip; only this closing step was
/// pinned to the sensor, and with it every reader that spells a block.
///
/// The verification reads the spelled word's block ON THE SAME PLANE.
/// `Spelling::block` reads the sensor's, so a word spelled on `(2, 3)`
/// would have been checked against its sensor block — the identity — and
/// refused for not matching a rotation it never claimed.
pub fn spell_on(plane: (u8, u8), block: &[[Q; 2]; 2]) -> Option<Spelling> {
    match spell_peel_on(plane, block)? {
        (PeelNest::Closed { cores }, rho) => {
            // Cores come out outer to inner (the sheet first); the tape's
            // last letter acts first, so the tape is the cores reversed.
            let sp = Spelling { word: word_of_cores(&cores)? };
            let b = sp.word.block_on(plane)?;
            // The word is the rotation; ρ is peeled for the caller to put
            // on the carriage. The spelled linear block times ρ is the
            // block on this plane.
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
pub fn spell_peel(block: &[[Q; 2]; 2]) -> Option<(PeelNest<SpellResidualView, SpellCore>, Q)> {
    spell_peel_on((PLANE_ROW, PLANE_COL), block)
}

/// The same peel on any pair plane (50.4, A22): the block's rows and
/// columns are the plane's own two axes, and the letters are that plane's.
pub fn spell_peel_on(
    plane: (u8, u8),
    block: &[[Q; 2]; 2],
) -> Option<(PeelNest<SpellResidualView, SpellCore>, Q)> {
    spell_peel_with(plane, block, Vec::new())
}

/// THE PEEL OVER A BASIS (37.7, L23a): the alphabet's own primes and the
/// adjoined generators the caller holds, in key order.
///
/// This is the ring branch. It was a second division loop over a second
/// basis, living in `family/ring.rs`, reached only on the arm where this
/// peel had already stood — so a mind holding a generator had its target
/// peeled once with the seventeen, refused, and then divided again by the
/// generator from the top. One loop, one basis, one pass.
pub fn spell_peel_with(
    plane: (u8, u8),
    block: &[[Q; 2]; 2],
    adjoined: Vec<AdjoinedPrime>,
) -> Option<(PeelNest<SpellResidualView, SpellCore>, Q)> {
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
    let z = Gauss::new((&r00 * &n).to_integer(), (&r10 * &n).to_integer());
    if z.norm() != &n * &n {
        return None;
    }
    let step = SpellStep::on_with(plane, adjoined)?;
    let mut governor = SpellGovernor { step: &step, last: None };
    let nest = peel_nest(SpellResidual { z, n, flipped }, &step, &mut governor);
    let view = match nest {
        PeelNest::Closed { cores } => PeelNest::Closed { cores },
        PeelNest::Standing { cores, arc } => PeelNest::Standing {
            cores,
            arc: SpellResidualView { p: arc.z.a, q: arc.z.b, weight: arc.n, flipped: arc.flipped },
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
