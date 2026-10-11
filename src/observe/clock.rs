// 78.21, 78.27(1), EC1 — ACT1's READING: THE CLOCK, per court and per voice.
// Where each node stands on TUniversal, by height with its own `K` beside
// it, and the three counts that say whether its acts stand at their nodes.
//
//! Three clocks, named apart (78.21(1)): TUniversal is the height, the one
//! clock every node shares; a node's `K` is its own stroke count, which is
//! how much it has done and never when it is. Disparity is read in HEIGHT
//! only — per peer the gap from its last admitted height to this node's
//! head, over the fleet the spread of heads — and never against a
//! threshold (rule 5). No wall-clock value is carried (law 9).
//!
//! The three counts (78.27 Q1, EC1) are per node, since boot:
//!
//! - acts sealed WITHOUT A FRAME — own-line acts that named a word no row
//!   of this court measures, so their μ⃗ was read as `[0; 4]`;
//! - words held WITHOUT AN ADMITTING STAMP, by row — basis, provisional,
//!   the listing book, the cavity, the court's federation;
//! - Ruling receipts ADMITTED against rulings whose economy APPLIED here.
//!
//! The genesis is the mesh origin this court adopted, its id and epoch.
//! Its height is not carried: the first receipt of any line stands at
//! height zero by construction (`ReceiptChain::next_stamp`), so a carried
//! genesis height would be a constant standing where a reading goes.

/// ONE COURT ON THE CLOCK.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CourtClock {
    /// Its own `K`: the orbit its next act stands in (78.23 as amended).
    pub k: u64,
    /// THE HEAD'S STAMP, `h:S:G:T:H:K.i` (78.21, 78.24, 78.23 as amended):
    /// the tick, the frame the head act measured, its orbit and its place
    /// in the orbit.
    pub head: [u64; 7],
    /// The mesh origin this court adopted — `H_genesis` and its epoch —
    /// or none where it has adopted no origin.
    pub genesis: Option<([u8; 32], u32)>,
    /// Per peer line this ledger holds: its signer, the last height this
    /// court admitted from it, and the gap from that height to this
    /// court's head (signed: a peer's line may stand above this head).
    pub peers: Vec<([u8; 32], u64, i64)>,
    /// The head's height and `K` when the snapshot was last written whole.
    pub persisted: Option<(u64, u64)>,
    /// The last fold's root: its height and `K`.
    pub folded: Option<(u64, u64)>,
    /// Own-line acts sealed without a frame.
    pub unframed: u64,
    /// Words held without an admitting stamp: basis, provisional, listing,
    /// cavity, federation.
    pub unstamped: [u64; 5],
    /// EC1 — PEERS' RULINGS ON THIS BOOK, per ruling arm (upheld, demoted,
    /// slashed, retired): `[admitted, applied, awaiting, refused]`, where
    /// admitted = applied + awaiting + refused.
    pub rulings: [[u64; 4]; 4],
    /// F5 — the refused rulings per arm by reason (`RulingRefused::ALL`):
    /// no party, a speaker not ruled, does not decompose.
    pub ruling_refusals: [[u64; 3]; 4],
    /// Its orbits against the peers' (OK1).
    pub orbits: Orbits,
}

/// OK1, 78.23 as amended — A NODE'S ORBITS AGAINST THE OTHERS': the closes
/// withheld because the orbit's frame met another's on every lane at one
/// tick, and at the node's last close the DISTANCE to each other orbit
/// closed at that tick — per lane, signed, exact, never a root and never
/// summed (law 5). A collision is prevented at the boundary; what the
/// console names is what the boundary prevented.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Orbits {
    pub withheld: u64,
    pub distance: Vec<([u8; 32], [i128; 4])>,
}

impl Orbits {
    #[must_use]
    fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        out.extend_from_slice(&self.withheld.to_le_bytes());
        crate::transducer::address::put_u32_count(out, self.distance.len())?;
        for (vk, d) in &self.distance {
            out.extend_from_slice(vk);
            for lane in d {
                out.extend_from_slice(&lane.to_le_bytes());
            }
        }
        Some(())
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Orbits> {
        let withheld = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        *p += 8;
        let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
        *p += 4;
        // An entry is its signer and four signed lanes: 96 bytes.
        if b.len().checked_sub(*p)? < n.checked_mul(96)? {
            return None;
        }
        let mut distance = Vec::with_capacity(n);
        for _ in 0..n {
            let vk: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            let mut d = [0i128; 4];
            for lane in d.iter_mut() {
                *lane = i128::from_le_bytes(b.get(*p..*p + 16)?.try_into().ok()?);
                *p += 16;
            }
            distance.push((vk, d));
        }
        Some(Orbits { withheld, distance })
    }
}

/// ONE MIND ON THE CLOCK (78.21(3), 78.49): its own `K`, the court it last
/// heard, its genome's write, its radius — no orbits against other signers,
/// since a voice seals nothing.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VoiceClock {
    /// Its own `K`: the orbits the mind acted in.
    pub k: u64,
    /// Its court's height when it last heard it.
    pub heard: u64,
    /// The court's height it had heard, and its own `K`, when its genome
    /// was last written whole.
    pub persisted: Option<(u64, u64)>,
    /// ITS RADIUS (78.23 as amended): LC1's dialect span read as the
    /// deepest A21 prefix its genome's charts reach — the most axes any
    /// stratum's chart carries. Read with the curve, once.
    pub radius: u64,
    /// 78.152 TU-7 — THE TUNIVERSAL POINT IT KEEPS TIME BY: the last final
    /// point its door replies carried, `origin/h:S:G:T:H:K`, whole; `None`
    /// before the first.
    pub point: Option<crate::chronometry::tuniversal::TPoint>,
}

/// ACT1'S READING: a node on the clock, a court or a voice. The court's is
/// held behind a box: it carries its peers and its rulings per arm, and a
/// voice's reading does not pay for their room.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Clock {
    Court(Box<CourtClock>),
    Voice(VoiceClock),
}

impl Clock {
    /// The node's height on TUniversal: a court's head, a voice's court as
    /// last heard.
    pub fn height(&self) -> u64 {
        match self {
            Clock::Court(c) => c.head[0],
            Clock::Voice(v) => v.heard,
        }
    }

    /// The node's own `K`.
    pub fn k(&self) -> u64 {
        match self {
            Clock::Court(c) => c.k,
            Clock::Voice(v) => v.k,
        }
    }

    /// `None` where the peers' count passes its four bytes (STD1 3c).
    #[must_use]
    pub(crate) fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        let u64s = |out: &mut Vec<u8>, vs: &[u64]| {
            for v in vs {
                out.extend_from_slice(&v.to_le_bytes());
            }
        };
        let pair = |out: &mut Vec<u8>, p: Option<(u64, u64)>| match p {
            None => out.push(0),
            Some((h, k)) => {
                out.push(1);
                out.extend_from_slice(&h.to_le_bytes());
                out.extend_from_slice(&k.to_le_bytes());
            }
        };
        match self {
            Clock::Court(c) => {
                out.push(0);
                u64s(out, &[c.k]);
                u64s(out, &c.head);
                match c.genesis {
                    None => out.push(0),
                    Some((id, epoch)) => {
                        out.push(1);
                        out.extend_from_slice(&id);
                        out.extend_from_slice(&epoch.to_le_bytes());
                    }
                }
                crate::transducer::address::put_u32_count(out, c.peers.len())?;
                for (vk, h, gap) in &c.peers {
                    out.extend_from_slice(vk);
                    out.extend_from_slice(&h.to_le_bytes());
                    out.extend_from_slice(&gap.to_le_bytes());
                }
                pair(out, c.persisted);
                pair(out, c.folded);
                u64s(out, &[c.unframed]);
                u64s(out, &c.unstamped);
                for arm in &c.rulings {
                    u64s(out, arm);
                }
                for arm in &c.ruling_refusals {
                    u64s(out, arm);
                }
                c.orbits.put(out)
            }
            Clock::Voice(v) => {
                out.push(1);
                u64s(out, &[v.k, v.heard]);
                pair(out, v.persisted);
                u64s(out, &[v.radius]);
                match &v.point {
                    None => out.push(0),
                    Some(t) => {
                        out.push(1);
                        t.put(out);
                    }
                }
                Some(())
            }
        }
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Clock> {
        let u64_at = |b: &[u8], p: &mut usize| -> Option<u64> {
            let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(v)
        };
        let byte = |b: &[u8], p: &mut usize| -> Option<u8> {
            let v = *b.get(*p)?;
            *p += 1;
            Some(v)
        };
        let pair = |b: &[u8], p: &mut usize| -> Option<Option<(u64, u64)>> {
            match byte(b, p)? {
                0 => Some(None),
                1 => Some(Some((u64_at(b, p)?, u64_at(b, p)?))),
                _ => None,
            }
        };
        match byte(b, p)? {
            0 => {
                let k = u64_at(b, p)?;
                let mut head = [0u64; 7];
                for v in head.iter_mut() {
                    *v = u64_at(b, p)?;
                }
                let genesis = match byte(b, p)? {
                    0 => None,
                    1 => {
                        let id: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                        *p += 32;
                        let epoch = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
                        *p += 4;
                        Some((id, epoch))
                    }
                    _ => return None,
                };
                let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
                *p += 4;
                // A peer entry is its signer, a height and a gap: 48 bytes.
                if b.len().checked_sub(*p)? < n.checked_mul(48)? {
                    return None;
                }
                let mut peers = Vec::with_capacity(n);
                for _ in 0..n {
                    let vk: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                    *p += 32;
                    let h = u64_at(b, p)?;
                    let gap = i64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
                    *p += 8;
                    peers.push((vk, h, gap));
                }
                let persisted = pair(b, p)?;
                let folded = pair(b, p)?;
                let unframed = u64_at(b, p)?;
                let mut unstamped = [0u64; 5];
                for v in unstamped.iter_mut() {
                    *v = u64_at(b, p)?;
                }
                let mut rulings = [[0u64; 4]; 4];
                for arm in rulings.iter_mut() {
                    for v in arm.iter_mut() {
                        *v = u64_at(b, p)?;
                    }
                }
                let mut ruling_refusals = [[0u64; 3]; 4];
                for arm in ruling_refusals.iter_mut() {
                    for v in arm.iter_mut() {
                        *v = u64_at(b, p)?;
                    }
                }
                let orbits = Orbits::take(b, p)?;
                Some(Clock::Court(Box::new(CourtClock {
                    k,
                    head,
                    genesis,
                    peers,
                    persisted,
                    folded,
                    unframed,
                    unstamped,
                    rulings,
                    ruling_refusals,
                    orbits,
                })))
            }
            1 => {
                let (k, heard) = (u64_at(b, p)?, u64_at(b, p)?);
                let persisted = pair(b, p)?;
                let radius = u64_at(b, p)?;
                let point = match byte(b, p)? {
                    0 => None,
                    1 => Some(crate::chronometry::tuniversal::TPoint::take(b, p)?),
                    _ => return None,
                };
                Some(Clock::Voice(VoiceClock { k, heard, persisted, radius, point }))
            }
            _ => None,
        }
    }
}
