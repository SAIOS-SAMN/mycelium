// 78.152 TU-5 — THE POINT: a final TUniversal coordinate, `h:S:G:T:H:K`
// exact, distinct across universes by its ORIGIN (the genesis hash and the
// epoch), written whole as `origin/h:S:G:T:H:K`. The four lanes are exact
// rationals, μ issued on each lane (ruled 2026-11-01).

use crate::core::Q;
use crate::economy::Mu;

/// A UNIVERSE'S ORIGIN (45.5): the chain hash of its mesh's first receipt and
/// the epoch it was cut at. Two points of two origins are never compared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Origin {
    pub genesis: [u8; 32],
    pub epoch: u32,
}

/// THE POINT (TU-5): a keeper's reading as it stands final — the tick `h`
/// (the keeper court's head height), the court's cumulative issuance per
/// lane `S:G:T:H` (exact non-negative rationals, `economy::Mu`), and `K`, the
/// orbits it closed — at its origin. No rounding, no average (TU-3).
/// Ordered as its coordinates are written — origin, tick, each lane by its
/// exact value in lane order, `K` — a canonical key, never a measure.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TPoint {
    pub origin: Origin,
    pub h: u64,
    pub lanes: Mu,
    pub k: u64,
}

impl Ord for TPoint {
    fn cmp(&self, other: &TPoint) -> std::cmp::Ordering {
        (self.origin, self.h, self.lanes.lanes(), self.k).cmp(&(other.origin, other.h, other.lanes.lanes(), other.k))
    }
}

impl PartialOrd for TPoint {
    fn partial_cmp(&self, other: &TPoint) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TPoint {
    /// The least bytes a point takes on the wire: the genesis, the epoch,
    /// `h`, four lanes at zero (`0/1` each, eleven bytes) and `K`.
    /// FORMAT: least TPoint wire length.
    pub const MIN_LEN: usize = 32 + 4 + 8 + 4 * 11 + 8;

    /// The most bytes a pointed frame adds behind its body (TU-7): a point
    /// of at most `u16::MAX` bytes and its two-byte length.
    /// FORMAT: TU-7 trailer bound, the u16 count's wall.
    pub const TRAILER_MAX: usize = u16::MAX as usize + 2;

    /// THE FIXED WRITING (TU-5), one way only: the origin — the genesis in
    /// 64 lowercase hex digits, `.`, the epoch in 8 — then `/`, then the six
    /// fields `h:S:G:T:H:K`. `h` and `K` are 16 lowercase hex digits each; a
    /// lane is its exact rational in lowest terms, `n/d`, numerator and
    /// denominator in lowercase hex with no leading zero (`0/1` for none).
    /// Within one origin the text orders by `h` as the number does; past `h`
    /// a lane's writing has no fixed width, so text order is NOT numeric
    /// order there: two writings are compared by [`TPoint::compare_writings`].
    pub fn print(&self) -> String {
        let mut out = String::with_capacity(64 + 1 + 8 + 1 + 6 * 17);
        for b in self.origin.genesis {
            out.push_str(&format!("{b:02x}"));
        }
        out.push_str(&format!(".{:08x}/", self.origin.epoch));
        out.push_str(&self.local());
        out
    }

    /// The local part alone, `h:S:G:T:H:K`, as `print` writes it.
    pub fn local(&self) -> String {
        let mut fields = vec![format!("{:016x}", self.h)];
        fields.extend(self.lanes.lanes().iter().map(lane_text));
        fields.push(format!("{:016x}", self.k));
        fields.join(":")
    }

    /// THE WRITING READ BACK, exactly: `h` and `K` at their fixed width in
    /// lowercase hex, each lane its one writing (lowest terms, no leading
    /// zero, a denominator at least one), nothing before, between or after.
    /// `None` for any other text — a short field, an uppercase digit, a sign,
    /// a lane not reduced, a seventh field.
    pub fn parse(text: &str) -> Option<TPoint> {
        let (origin, local) = text.split_once('/')?;
        let (genesis_hex, epoch_hex) = origin.split_once('.')?;
        if genesis_hex.len() != 64 {
            return None;
        }
        let mut genesis = [0u8; 32];
        for (i, g) in genesis.iter_mut().enumerate() {
            *g = u8::try_from(hex_exact(genesis_hex.get(2 * i..2 * i + 2)?, 2)?).ok()?;
        }
        let epoch = u32::try_from(hex_exact(epoch_hex, 8)?).ok()?;
        let mut parts = local.split(':');
        let h = hex_exact(parts.next()?, 16)?;
        let mut lanes: [Q; 4] = Default::default();
        for l in &mut lanes {
            *l = lane_of_text(parts.next()?)?;
        }
        let k = hex_exact(parts.next()?, 16)?;
        if parts.next().is_some() {
            return None;
        }
        Some(TPoint { origin: Origin { genesis, epoch }, h, lanes: Mu::of(lanes).ok()?, k })
    }

    /// TWO WRITINGS COMPARED AS THE POINTS THEY WRITE: each read back
    /// ([`TPoint::parse`]) and ordered by the point's own order (origin,
    /// `h`, each lane's exact value, `K`). `None` where either is no point.
    pub fn compare_writings(a: &str, b: &str) -> Option<std::cmp::Ordering> {
        Some(TPoint::parse(a)?.cmp(&TPoint::parse(b)?))
    }

    /// The point written on the wire: the genesis, the epoch (u32 LE), `h`
    /// (u64 LE), the four lanes in the rational lane codec (`Mu::put`), `K`
    /// (u64 LE).
    pub fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.origin.genesis);
        out.extend_from_slice(&self.origin.epoch.to_le_bytes());
        out.extend_from_slice(&self.h.to_le_bytes());
        self.lanes.put(out);
        out.extend_from_slice(&self.k.to_le_bytes());
    }

    /// The point's wire bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(TPoint::MIN_LEN);
        self.put(&mut out);
        out
    }

    /// The point read at `p`; `None` where a field is short or a lane is
    /// refused (negative, not reduced, not its one writing).
    pub fn take(b: &[u8], p: &mut usize) -> Option<TPoint> {
        let genesis: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
        let epoch = u32::from_le_bytes(b.get(*p + 32..*p + 36)?.try_into().ok()?);
        let mut q = *p + 36;
        let h = u64::from_le_bytes(b.get(q..q + 8)?.try_into().ok()?);
        q += 8;
        let lanes = Mu::take(b, &mut q).ok()?;
        let k = u64::from_le_bytes(b.get(q..q + 8)?.try_into().ok()?);
        *p = q + 8;
        Some(TPoint { origin: Origin { genesis, epoch }, h, lanes, k })
    }

    /// THE TU-7 TRAILER: the point's wire bytes, then their count as a u16
    /// LE, so a reader finds the point from the frame's end. `None` where the
    /// point passes `u16::MAX` bytes.
    pub fn put_trailer(&self, out: &mut Vec<u8>) -> Option<()> {
        let bytes = self.to_bytes();
        let n = u16::try_from(bytes.len()).ok()?;
        out.extend_from_slice(&bytes);
        out.extend_from_slice(&n.to_le_bytes());
        Some(())
    }

    /// The trailer read off the end of `body`: the point and where it began.
    /// `None` where the count passes the body or the point does not read
    /// exactly the bytes it counts.
    pub fn take_trailer(body: &[u8]) -> Option<(TPoint, usize)> {
        let end = body.len().checked_sub(2)?;
        let n = usize::from(u16::from_le_bytes(body.get(end..)?.try_into().ok()?));
        let at = end.checked_sub(n)?;
        let mut p = at;
        let point = TPoint::take(body, &mut p)?;
        (p == end).then_some((point, at))
    }

    /// DOES THIS POINT ADVANCE ON `held`: nothing held; or the same origin
    /// at a greater tick; or a later epoch — a universe re-originated
    /// (TU-6 RESET), whose ticks start again. Points of one origin at one
    /// tick never replace one another here: which of two is final is the
    /// keepers' quorum's to say (TU-3), never a reader's tie-break.
    pub fn advances(&self, held: Option<&TPoint>) -> bool {
        match held {
            None => true,
            Some(h) if h.origin == self.origin => self.h > h.h,
            Some(h) => self.origin.epoch > h.origin.epoch,
        }
    }
}

/// A lane's one writing: `n/d` in lowercase hex, lowest terms.
fn lane_text(q: &Q) -> String {
    format!("{}/{}", q.numer().to_str_radix(16), q.denom().to_str_radix(16))
}

/// A lane read back from its one writing, or `None`: each part lowercase hex
/// with no leading zero (zero is `0`), a denominator at least one, and the
/// two in lowest terms.
fn lane_of_text(s: &str) -> Option<Q> {
    use num_integer::Integer as _;
    use num_traits::{One as _, Zero as _};
    let (n, d) = s.split_once('/')?;
    let part = |t: &str| -> Option<num_bigint::BigInt> {
        let digits = !t.is_empty() && t.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c));
        let minimal = t == "0" || !t.starts_with('0');
        if !(digits && minimal) {
            return None;
        }
        num_bigint::BigInt::parse_bytes(t.as_bytes(), 16)
    };
    let (n, d) = (part(n)?, part(d)?);
    if d.is_zero() || !n.gcd(&d).is_one() {
        return None;
    }
    Some(Q::new_raw(n, d))
}

/// `width` lowercase hex digits exactly, as one integer.
fn hex_exact(s: &str, width: usize) -> Option<u64> {
    if s.len() != width || !s.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)) {
        return None;
    }
    u64::from_str_radix(s, 16).ok()
}
