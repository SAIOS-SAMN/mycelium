// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-ECONOMY §1, §2.5, §9 — μ AS A HOLDING OF MODES. Every amount of μ the
//! economy carries — a balance, a bond, an escrow or held value, an event's
//! amount, a wire field — is a [`Mu`], and a `Mu` holds its modes
//! (`economy::modes::Modes`): the exact components of each lane's vector
//! that the work moved, keyed by their coordinates (A21). The lanes S, G, T
//! and H are the coordinate's dimensions, and μ their culmination as
//! coordinates; nothing is held as one value per lane.
//!
//! What no motion of the network made — a genesis allocation, a subsidy, a
//! count read off a measure ([`Mu::counts`], [`Mu::of`]) — is held at each
//! lane's founding mode, the lane's identity motion, never with empty modes
//! (§2.5). A mint whose proof cites a motion carries that motion's modes
//! ([`Mu::of_modes`]).
//!
//! A lane's whole ([`Mu::lane`], [`Mu::lanes`]) is a READING of the
//! holding, as is every comparison against an ask, a room or a cap
//! (`shortfall`, `capped`, `lane_moves`): readings carry no modes of their
//! own and are held at the founding modes. A draw takes each lane's amount
//! from that lane's modes in proportion, exactly in ℚ, never from another
//! lane, and is refused naming the lane short ([`MuRefusal::Short`]); the
//! modes drawn travel with the transfer ([`Mu::draw_out`]). The four lanes
//! are never summed, never converted and never ordered against each other.

use crate::core::q::Q;
use crate::economy::modes::{Mode, Modes};
use crate::economy::LaneMove;
use num_bigint::BigInt;
use num_traits::{Signed, Zero};

/// μ AS A HOLDING OF MODES (MU-ECONOMY §1): each lane's vector by its
/// modes, behind one box so an amount is one pointer wherever it is carried.
/// Equality is mode by mode.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Mu(Box<Modes>);

/// WHY AN AMOUNT OF μ WAS REFUSED, by name: what a draw lacked, or what a
/// carried amount did not read as.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MuRefusal {
    /// A draw asked more than was held on this lane: the first lane short,
    /// in lane order.
    Short { lane: usize },
    /// A lane would hold a negative amount.
    Negative { lane: usize },
    /// A carried amount is not in lowest terms: one value has one writing.
    NotReduced { lane: usize },
    /// The bytes do not read as a holding of modes.
    Malformed,
}

impl From<[u64; 4]> for Mu {
    /// A count per lane, held at each lane's founding mode.
    fn from(n: [u64; 4]) -> Mu {
        Mu::counts(n)
    }
}

impl PartialEq<[u64; 4]> for Mu {
    /// Read lane by lane: each lane's whole is exactly that count.
    fn eq(&self, n: &[u64; 4]) -> bool {
        self.lanes().iter().zip(n).all(|(q, &c)| q.is_integer() && q.numer() == &BigInt::from(c))
    }
}

impl std::fmt::Display for Mu {
    /// The four lanes' wholes as exact rationals, `[S, G, T, H]`: a reading.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [s, g, t, h] = self.lanes();
        write!(f, "[{s}, {g}, {t}, {h}]")
    }
}

/// The founding holding of four per-lane amounts, each known non-negative.
fn founded(lanes: &[Q; 4]) -> Mu {
    let mut held = Modes::new();
    for (lane, a) in (0u8..).zip(lanes.iter()) {
        if let Some(m) = Mode::founding(lane) {
            held.add_mode(m, a.clone());
        }
    }
    Mu(Box::new(held))
}

impl Mu {
    /// FORMAT: the least writing of a holding, its count of modes (u32 LE)
    /// and no mode: the holding of nothing.
    pub const MIN_LEN: usize = 4;

    /// Nothing on every lane.
    pub fn zero() -> Mu {
        Mu::default()
    }

    /// A count per lane, `n/1`, held at each lane's founding mode.
    pub fn counts(n: [u64; 4]) -> Mu {
        founded(&n.map(|c| Q::from_integer(BigInt::from(c))))
    }

    /// Four exact per-lane amounts held at the founding modes, or the first
    /// negative lane refused by name.
    pub fn of(lanes: [Q; 4]) -> Result<Mu, MuRefusal> {
        match lanes.iter().position(Signed::is_negative) {
            Some(lane) => Err(MuRefusal::Negative { lane }),
            None => Ok(founded(&lanes)),
        }
    }

    /// THE HOLDING OF THESE MODES: a mint carrying the modes of the motion
    /// its proof cites (MU-ECONOMY §2.5).
    pub fn of_modes(modes: Modes) -> Mu {
        Mu(Box::new(modes))
    }

    /// The modes held.
    pub fn modes(&self) -> &Modes {
        &self.0
    }

    /// THE LANE'S WHOLE (`0..4`, S G T H): a reading of its modes.
    pub fn lane(&self, lane: usize) -> Q {
        u8::try_from(lane).map_or_else(|_| Q::zero(), |l| self.0.lane_whole(l))
    }

    /// The four lanes' wholes, in lane order: a reading.
    pub fn lanes(&self) -> [Q; 4] {
        std::array::from_fn(|l| self.lane(l))
    }

    /// Nothing held.
    pub fn is_zero(&self) -> bool {
        self.0.is_empty()
    }

    /// The lanes that hold something, by index.
    pub fn occupied(&self) -> Vec<usize> {
        (0..4).filter(|&l| !self.lane(l).is_zero()).collect()
    }

    /// Whether every lane's whole is a whole amount.
    #[cfg(test)]
    pub fn is_integral(&self) -> bool {
        self.lanes().iter().all(|q| q.is_integer())
    }

    /// `self + other`, mode by mode, exactly.
    pub fn add(&self, other: &Mu) -> Mu {
        let mut out = self.clone();
        out.step(other);
        out
    }

    /// `self += other`, mode by mode, exactly.
    pub fn step(&mut self, other: &Mu) {
        self.0.add(&other.0);
    }

    /// The sum of holdings, mode by mode.
    pub fn sum<'a>(it: impl IntoIterator<Item = &'a Mu>) -> Mu {
        let mut total = Mu::zero();
        for m in it {
            total.step(m);
        }
        total
    }

    /// THE MODES `asked` DRAWS, taken out of `self` (MU-ECONOMY §2.5): each
    /// lane's whole of the ask drawn from that lane's modes in proportion,
    /// every lane or none, the first lane short named. The modes drawn are
    /// returned, to travel with the transfer.
    pub fn draw_out(&mut self, asked: &Mu) -> Result<Mu, MuRefusal> {
        let ask = asked.lanes();
        if let Some(lane) = (0..4).find(|&l| self.lane(l) < ask[l]) {
            return Err(MuRefusal::Short { lane });
        }
        let mut drawn = Modes::new();
        for (lane, a) in (0u8..).zip(ask.iter()) {
            drawn.add(&self.0.draw_lane(lane, a).map_err(|_| MuRefusal::Short { lane: usize::from(lane) })?);
        }
        Ok(Mu(Box::new(drawn)))
    }

    /// `self − asked`, WHOLE OR REFUSED: the holding less what the ask draws
    /// from it, or the first lane where the holding is short, named.
    pub fn draw(&self, asked: &Mu) -> Result<Mu, MuRefusal> {
        let mut rest = self.clone();
        rest.draw_out(asked)?;
        Ok(rest)
    }

    /// THE SHORTFALL OF `self` AGAINST `room`, per lane: what `self` needs
    /// beyond the room, and zero where the lane fits. A reading (§2.5).
    pub fn shortfall(&self, room: &Mu) -> Mu {
        let (a, b) = (self.lanes(), room.lanes());
        founded(&std::array::from_fn(|l| match a[l] > b[l] {
            true => &a[l] - &b[l],
            false => Q::zero(),
        }))
    }

    /// `self` CAPPED BY `cap`, read lane by lane: on each lane the lesser of
    /// the two wholes. A reading (§2.5).
    pub fn capped(&self, cap: &Mu) -> Mu {
        let (a, b) = (self.lanes(), cap.lanes());
        founded(&std::array::from_fn(|l| match a[l] > b[l] {
            true => b[l].clone(),
            false => a[l].clone(),
        }))
    }

    /// THE HOLDING DIVIDED AT AN ASK: the part taken — on each lane the
    /// lesser of the holding and the ask, drawn from the lane's modes in
    /// proportion — and what the holding keeps. The two make `self` exactly.
    pub fn split_at(&self, asked: &Mu) -> (Mu, Mu) {
        // The ask capped by the holding draws within it on every lane, so the
        // draw lands; were it refused, nothing is taken.
        let mut kept = self.clone();
        match kept.draw_out(&asked.capped(self)) {
            Ok(taken) => (taken, kept),
            Err(_) => (Mu::zero(), self.clone()),
        }
    }

    /// ONE OF `n` EQUAL PARTS, mode by mode, so `n` parts are the whole.
    /// `None` for no parts.
    pub fn part(&self, n: usize) -> Option<Mu> {
        if n == 0 {
            return None;
        }
        self.scaled(&Q::new(BigInt::from(1), BigInt::from(n)))
    }

    /// THE LANES' WHOLES AS FOUR COUNTS, where each is a whole number that
    /// fits a `u64`; `None` otherwise — never rounded. A reading.
    #[cfg(test)]
    pub fn to_counts(&self) -> Option<[u64; 4]> {
        let mut out = [0u64; 4];
        for (o, q) in out.iter_mut().zip(self.lanes().iter()) {
            if !q.is_integer() {
                return None;
            }
            *o = u64::try_from(q.to_integer()).ok()?;
        }
        Some(out)
    }

    /// The modes on the lanes `keep` names, nothing on the others.
    pub fn on_lanes(&self, keep: impl Fn(usize) -> bool) -> Mu {
        Mu(Box::new(self.0.on_lanes(|l| keep(usize::from(l)))))
    }

    /// EACH LANE'S WHOLE PART AS A COUNT, ⌊q⌋, for a comparison against
    /// counts. A reading; `None` where a whole part passes a `u64`.
    pub fn whole_counts(&self) -> Option<[u64; 4]> {
        let mut out = [0u64; 4];
        for (o, q) in out.iter_mut().zip(self.lanes().iter()) {
            *o = u64::try_from(q.floor().to_integer()).ok()?;
        }
        Some(out)
    }

    /// EACH LANE OVER ITS OWN WHOLE, exactly: lane `l`'s modes divided by
    /// `by[l]`. `None` where a whole is zero.
    pub fn over(&self, by: [u64; 4]) -> Option<Mu> {
        if by.contains(&0) {
            return None;
        }
        let mut out = Modes::new();
        for (lane, d) in (0u8..).zip(by) {
            out.add(&self.0.on_lanes(|l| l == lane).scaled(&Q::new(BigInt::from(1), BigInt::from(d)))?);
        }
        Some(Mu(Box::new(out)))
    }

    /// `self · r`, mode by mode, exactly; `None` for a negative factor.
    pub fn scaled(&self, r: &Q) -> Option<Mu> {
        Some(Mu(Box::new(self.0.scaled(r)?)))
    }

    /// WHAT EACH LANE DID from `before` to `self`, read per lane, with the
    /// exact amount it fell or rose by. Never a verdict and never a total
    /// order.
    pub fn lane_moves(&self, before: &Mu) -> [LaneMove<Q>; 4] {
        let (a, b) = (self.lanes(), before.lanes());
        std::array::from_fn(|l| match a[l].cmp(&b[l]) {
            std::cmp::Ordering::Less => LaneMove::Fell(&b[l] - &a[l]),
            std::cmp::Ordering::Equal => LaneMove::Held,
            std::cmp::Ordering::Greater => LaneMove::Rose(&a[l] - &b[l]),
        })
    }

    /// THE HOLDING WRITTEN OUT: its modes by the `Modes` codec, each mode
    /// and its exact amount in coordinate order (MU-ECONOMY §9). A mode with
    /// no canonical writing is never held, so the writing always lands.
    pub fn put(&self, out: &mut Vec<u8>) {
        let _ = self.0.put(out);
    }

    /// The holding's writing as bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.put(&mut out);
        out
    }

    /// The inverse of [`Mu::put`], WHOLE OR REFUSED BY NAME: bytes that do
    /// not read as a holding of modes, every amount positive and every mode
    /// once, are `Malformed`.
    pub fn take(b: &[u8], p: &mut usize) -> Result<Mu, MuRefusal> {
        Modes::take_from(b, p).map(|m| Mu(Box::new(m))).ok_or(MuRefusal::Malformed)
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
