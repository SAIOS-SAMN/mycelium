// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-ECONOMY §1 — μ AS EXACT LANE AMOUNTS: one non-negative rational per
//! lane, S, G, T, H, and nothing else.
//!
//! μ is measured as a share of a frame, and a share is a rational: a
//! holding that could carry only whole counts would floor every share it
//! was paid. Every amount of μ — a balance, a bond, an escrow, held value,
//! a ledger row, a wire field — is a [`Mu`]. A count `n` read off a measure
//! is the rational `n/1` ([`Mu::counts`]), so a measurement keeps its value
//! exactly.
//!
//! The four lanes are never summed, never converted and never ordered
//! against each other: equality is lane by lane, and the one comparison is
//! the per-lane reading [`Mu::lane_moves`]. Arithmetic is exact and has no
//! carrier to pass: an addition always lands, and a draw past a holding is
//! refused naming the lane short ([`MuRefusal::Short`]),
//! never saturated and never floored. Every lane of a `Mu` is at least zero;
//! a negative amount is refused where it would be made or read.
//!
//! The held modes of MU-RECURSION 78.162 (`economy::modes`) remain the next
//! step: a holding here is four exact amounts, not the modes that moved.

use crate::core::q::Q;
use crate::economy::LaneMove;
use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};

/// μ ON THE FOUR LANES: an exact non-negative rational per lane, in lane
/// order S, G, T, H. No order is defined between two of them; equality is
/// lane by lane. The lanes are held behind one box, so an amount is one
/// pointer wherever it is carried.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Mu(Box<[Q; 4]>);

/// WHY AN AMOUNT OF μ WAS REFUSED, by name: what a draw lacked, or what a
/// carried amount did not read as.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MuRefusal {
    /// A draw asked more than was held on this lane: the first lane short,
    /// in lane order (the amounts are the caller's two operands).
    Short { lane: usize },
    /// A lane would hold a negative amount.
    Negative { lane: usize },
    /// A carried lane is not in lowest terms (or its denominator is not
    /// positive): one value has one writing.
    NotReduced { lane: usize },
    /// The bytes do not read as four lanes.
    Malformed,
}

impl From<[u64; 4]> for Mu {
    /// A count per lane is the rational `n/1` on that lane: a measurement
    /// keeps its value exactly.
    fn from(n: [u64; 4]) -> Mu {
        Mu::counts(n)
    }
}

impl PartialEq<[u64; 4]> for Mu {
    /// Lane by lane: the amount on each lane is exactly that count.
    fn eq(&self, n: &[u64; 4]) -> bool {
        self.0.iter().zip(n).all(|(q, &c)| q.is_integer() && q.numer() == &BigInt::from(c))
    }
}

impl std::fmt::Display for Mu {
    /// The four lanes as exact rationals, `[S, G, T, H]`: a whole amount as
    /// its integer, a share as `n/d`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [s, g, t, h] = &*self.0;
        write!(f, "[{s}, {g}, {t}, {h}]")
    }
}

impl Mu {
    /// The lanes held, each already known non-negative.
    fn new(lanes: [Q; 4]) -> Mu {
        Mu(Box::new(lanes))
    }

    /// Nothing on every lane.
    pub fn zero() -> Mu {
        Mu::default()
    }

    /// A count per lane, each the rational `n/1`.
    pub fn counts(n: [u64; 4]) -> Mu {
        Mu::new(n.map(|c| Q::from_integer(BigInt::from(c))))
    }

    /// Four exact amounts, or the first negative lane refused by name.
    pub fn of(lanes: [Q; 4]) -> Result<Mu, MuRefusal> {
        match lanes.iter().position(Signed::is_negative) {
            Some(lane) => Err(MuRefusal::Negative { lane }),
            None => Ok(Mu::new(lanes)),
        }
    }

    /// The amount on one lane (`0..4`, S G T H).
    pub fn lane(&self, lane: usize) -> &Q {
        &self.0[lane]
    }

    /// The four lanes, in lane order.
    pub fn lanes(&self) -> &[Q; 4] {
        &self.0
    }

    /// Nothing on every lane.
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(Zero::is_zero)
    }

    /// The lanes that hold something, by index.
    pub fn occupied(&self) -> Vec<usize> {
        (0..4).filter(|&l| !self.0[l].is_zero()).collect()
    }

    /// Whether every lane is a whole amount.
    pub fn is_integral(&self) -> bool {
        self.0.iter().all(|q| q.is_integer())
    }

    /// `self + other`, lane by lane, exactly.
    pub fn add(&self, other: &Mu) -> Mu {
        Mu::new(std::array::from_fn(|l| &self.0[l] + &other.0[l]))
    }

    /// `self += other`, lane by lane, exactly.
    pub fn step(&mut self, other: &Mu) {
        for (a, b) in self.0.iter_mut().zip(other.0.iter()) {
            *a += b;
        }
    }

    /// The lane-wise sum of amounts.
    pub fn sum<'a>(it: impl IntoIterator<Item = &'a Mu>) -> Mu {
        let mut total = Mu::zero();
        for m in it {
            total.step(m);
        }
        total
    }

    /// `self − asked`, WHOLE OR REFUSED: the holding less what is asked on
    /// every lane, or the first lane where the holding is short, named.
    /// Nothing is saturated and nothing is floored.
    pub fn draw(&self, asked: &Mu) -> Result<Mu, MuRefusal> {
        if let Some(lane) = (0..4).find(|&l| self.0[l] < asked.0[l]) {
            return Err(MuRefusal::Short { lane });
        }
        Ok(Mu::new(std::array::from_fn(|l| &self.0[l] - &asked.0[l])))
    }

    /// THE SHORTFALL OF `self` AGAINST `room`, per lane: what `self` needs
    /// beyond the room, and zero where the lane fits. A reading, never a
    /// verdict; the floor at zero is the accounting floor of a deficit.
    pub fn shortfall(&self, room: &Mu) -> Mu {
        Mu::new(std::array::from_fn(|l| match self.0[l] > room.0[l] {
            true => &self.0[l] - &room.0[l],
            false => Q::zero(),
        }))
    }

    /// `self` CAPPED BY `cap`, lane by lane: `self` less its shortfall
    /// against the cap — on each lane the lesser of the two.
    pub fn capped(&self, cap: &Mu) -> Mu {
        Mu::new(std::array::from_fn(|l| match self.0[l] > cap.0[l] {
            true => cap.0[l].clone(),
            false => self.0[l].clone(),
        }))
    }

    /// THE HOLDING DIVIDED AT AN ASK, lane by lane: the part taken — on
    /// each lane the lesser of the holding and the ask — and what the
    /// holding keeps. The two sum to `self` exactly, and neither is ever
    /// negative.
    pub fn split_at(&self, asked: &Mu) -> (Mu, Mu) {
        let taken = asked.capped(self);
        let kept = Mu::new(std::array::from_fn(|l| &self.0[l] - &taken.0[l]));
        (taken, kept)
    }

    /// ONE OF `n` EQUAL PARTS, exactly: `self / n` on every lane, so `n`
    /// parts are the whole. `None` for no parts.
    pub fn part(&self, n: usize) -> Option<Mu> {
        if n == 0 {
            return None;
        }
        let n = Q::from_integer(BigInt::from(n));
        Some(Mu::new(std::array::from_fn(|l| &self.0[l] / &n)))
    }

    /// THE AMOUNT AS FOUR COUNTS, where every lane is a whole number that
    /// fits a `u64`; `None` otherwise — never rounded. The inverse of
    /// [`Mu::counts`].
    pub fn to_counts(&self) -> Option<[u64; 4]> {
        let mut out = [0u64; 4];
        for (o, q) in out.iter_mut().zip(self.0.iter()) {
            if !q.is_integer() {
                return None;
            }
            *o = u64::try_from(q.to_integer()).ok()?;
        }
        Some(out)
    }

    /// The amount on the lanes `keep` names, nothing on the others.
    pub fn on_lanes(&self, keep: impl Fn(usize) -> bool) -> Mu {
        Mu::new(std::array::from_fn(|l| match keep(l) {
            true => self.0[l].clone(),
            false => Q::zero(),
        }))
    }

    /// EACH LANE'S WHOLE PART AS A COUNT, ⌊q⌋, for a comparison against
    /// counts: a count `n` fits under the amount `q` exactly where it fits
    /// under `⌊q⌋`, so the reading loses nothing a count can ask. `None`
    /// where a whole part passes a `u64`.
    pub fn whole_counts(&self) -> Option<[u64; 4]> {
        let mut out = [0u64; 4];
        for (o, q) in out.iter_mut().zip(self.0.iter()) {
            *o = u64::try_from(q.floor().to_integer()).ok()?;
        }
        Some(out)
    }

    /// EACH LANE OVER ITS OWN WHOLE, exactly: lane `l` divided by `by[l]`,
    /// the share of that whole the lane holds. `None` where a whole is zero.
    pub fn over(&self, by: [u64; 4]) -> Option<Mu> {
        if by.contains(&0) {
            return None;
        }
        Some(Mu::new(std::array::from_fn(|l| &self.0[l] / Q::from_integer(BigInt::from(by[l])))))
    }

    /// `self · r` on every lane, exactly; `None` for a negative factor.
    pub fn scaled(&self, r: &Q) -> Option<Mu> {
        if r.is_negative() {
            return None;
        }
        Some(Mu::new(std::array::from_fn(|l| &self.0[l] * r)))
    }

    /// WHAT EACH LANE DID from `before` to `self`, per lane, with the exact
    /// amount it fell or rose by. Never a verdict and never a total order.
    pub fn lane_moves(&self, before: &Mu) -> [LaneMove<Q>; 4] {
        std::array::from_fn(|l| match self.0[l].cmp(&before.0[l]) {
            std::cmp::Ordering::Less => LaneMove::Fell(&before.0[l] - &self.0[l]),
            std::cmp::Ordering::Equal => LaneMove::Held,
            std::cmp::Ordering::Greater => LaneMove::Rose(&self.0[l] - &before.0[l]),
        })
    }

    /// THE AMOUNT WRITTEN OUT: per lane, in lane order, a sign byte (always
    /// `0`: a lane is never negative), then the numerator's 64-bit limbs by
    /// count and the denominator's limbs by count, least significant limb
    /// first — the writing `ChartAxis::Ratio` gives a rational. The value is
    /// in lowest terms, so one amount has one writing.
    pub fn put(&self, out: &mut Vec<u8>) {
        for q in self.0.iter() {
            out.push(u8::from(q.is_negative()));
            put_limbs(out, q.numer().magnitude());
            put_limbs(out, q.denom().magnitude());
        }
    }

    /// The amount's writing as bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.put(&mut out);
        out
    }

    /// The inverse of [`Mu::put`], WHOLE OR REFUSED BY NAME: a negative
    /// lane, a lane not in lowest terms (a zero is `0/1`), or a writing that
    /// is not the one writing of its value.
    pub fn take(b: &[u8], p: &mut usize) -> Result<Mu, MuRefusal> {
        let mut lanes: [Q; 4] = Default::default();
        for (lane, q) in lanes.iter_mut().enumerate() {
            let sign = *b.get(*p).ok_or(MuRefusal::Malformed)?;
            *p += 1;
            let numer = take_limbs(b, p).ok_or(MuRefusal::Malformed)?;
            let denom = take_limbs(b, p).ok_or(MuRefusal::Malformed)?;
            match sign {
                0 => {}
                1 => return Err(MuRefusal::Negative { lane }),
                _ => return Err(MuRefusal::Malformed),
            }
            if denom.is_zero() || !numer.gcd(&denom).is_one() {
                return Err(MuRefusal::NotReduced { lane });
            }
            *q = Q::new_raw(BigInt::from_biguint(Sign::Plus, numer), BigInt::from_biguint(Sign::Plus, denom));
        }
        Ok(Mu::new(lanes))
    }

    /// [`Mu::take`], reading a refusal as nothing.
    pub fn take_from(b: &[u8], p: &mut usize) -> Option<Mu> {
        Mu::take(b, p).ok()
    }

    /// One amount read from the whole of `b`, or refused.
    pub fn from_bytes(b: &[u8]) -> Result<Mu, MuRefusal> {
        let mut p = 0usize;
        let mu = Mu::take(b, &mut p)?;
        match p == b.len() {
            true => Ok(mu),
            false => Err(MuRefusal::Malformed),
        }
    }
}

/// A magnitude's 64-bit limbs by count, least significant first: zero is
/// the empty run.
fn put_limbs(out: &mut Vec<u8>, m: &BigUint) {
    let digits = m.to_u64_digits();
    crate::transducer::address::put_key_count(out, digits.len());
    for d in &digits {
        out.extend_from_slice(&d.to_le_bytes());
    }
}

/// The inverse of [`put_limbs`]; `None` where the count passes the bytes
/// or the most significant limb is zero (a second writing of a value).
#[expect(clippy::cast_possible_truncation, reason = "a u64 limb split into its low and high u32 halves on purpose; the high half is `>> 32`")]
fn take_limbs(b: &[u8], p: &mut usize) -> Option<BigUint> {
    let n = crate::transducer::address::take_key_count(b, p)?;
    if n > b.len().checked_sub(*p)? / 8 {
        return None;
    }
    let mut digits = Vec::with_capacity(n);
    for _ in 0..n {
        digits.push(u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?));
        *p += 8;
    }
    if let [.., 0] = digits.as_slice() {
        return None;
    }
    Some(BigUint::new(digits.iter().flat_map(|d| [*d as u32, (*d >> 32) as u32]).collect()))
}
