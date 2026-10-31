// 78.152 TU-3, TU-4, TU-7 — THE COURT'S TUNIVERSAL STANDING ON THE OBSERVER:
// the door's quorum-final point and its advances, each keeper as this court
// holds it, and the clock's residual on the keeper triangle — read off the
// door's shelf and the keepers, which live outside the court.
//
//! A reading on the observer, never folded into the point (TU-4): the final
//! point is the shelf's, each keeper's `final_of` is the keepers' own, the
//! residual's edges and holonomy are carried exact (`i128` per lane), never
//! averaged and never a root.

use crate::chronometry::tuniversal::{Edge, Keepers, Residual, TPoint, Tick};

/// The writing's version, its first byte: a reader of another layout refuses
/// rather than reading one field as another.
pub const TSTANDING_VERSION: u8 = 1;

/// ONE KEEPER AS THIS COURT HOLDS IT: its key, whether it is this court's
/// own, its newest verified reading's tick (`None` where none is held), and
/// its own final point (TU-3, `Keepers::final_of`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct KeeperSeen {
    pub key: [u8; 32],
    pub own: bool,
    pub newest: Option<Tick>,
    pub final_of: Option<TPoint>,
}

/// THE COURT'S TUNIVERSAL STANDING: the door's final point (`None` before
/// the first), how many times it advanced, the keeper set in key order, and
/// the residual (TU-4).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TStanding {
    pub final_point: Option<TPoint>,
    pub advances: u64,
    pub keepers: Vec<KeeperSeen>,
    pub residual: Residual,
}

/// Each keeper `k` holds, in key order, as the observer carries it.
pub fn keepers_seen(k: &Keepers) -> Vec<KeeperSeen> {
    let local = k.own_key();
    k.keepers()
        .into_iter()
        .map(|key| KeeperSeen { key, own: key == local, newest: k.newest(&key).map(|r| r.tick), final_of: k.final_of(&key) })
        .collect()
}

fn put_tick(out: &mut Vec<u8>, t: &Tick) {
    out.extend_from_slice(&t.h.to_le_bytes());
    for l in t.lanes {
        out.extend_from_slice(&l.to_le_bytes());
    }
    out.extend_from_slice(&t.k.to_le_bytes());
}

fn put_lanes(out: &mut Vec<u8>, d: &[i128; 4]) {
    for l in d {
        out.extend_from_slice(&l.to_le_bytes());
    }
}

fn put_point(out: &mut Vec<u8>, p: Option<&TPoint>) {
    match p {
        None => out.push(0),
        Some(p) => {
            out.push(1);
            p.put(out);
        }
    }
}

fn take_u8(b: &[u8], p: &mut usize) -> Option<u8> {
    let v = *b.get(*p)?;
    *p += 1;
    Some(v)
}

fn take_u64(b: &[u8], p: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

fn take_key(b: &[u8], p: &mut usize) -> Option<[u8; 32]> {
    let v: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
    *p += 32;
    Some(v)
}

fn take_lanes(b: &[u8], p: &mut usize) -> Option<[i128; 4]> {
    let mut d = [0i128; 4];
    for l in &mut d {
        *l = i128::from_le_bytes(b.get(*p..*p + 16)?.try_into().ok()?);
        *p += 16;
    }
    Some(d)
}

fn take_point(b: &[u8], p: &mut usize) -> Option<Option<TPoint>> {
    match take_u8(b, p)? {
        0 => Some(None),
        1 => Some(Some(TPoint::take(b, p)?)),
        _ => None,
    }
}

/// Does the body from `at` hold `m` entries of at least `item` bytes each?
fn holds(b: &[u8], at: usize, m: usize, item: usize) -> bool {
    match (b.len().checked_sub(at), m.checked_mul(item)) {
        (Some(rest), Some(need)) => rest >= need,
        _ => false,
    }
}

impl TStanding {
    /// The standing written out, its version first. `None` where a count
    /// passes its four bytes (STD1 3c).
    #[must_use]
    pub(crate) fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        out.push(TSTANDING_VERSION);
        put_point(out, self.final_point.as_ref());
        out.extend_from_slice(&self.advances.to_le_bytes());
        crate::transducer::address::put_u32_count(out, self.keepers.len())?;
        for s in &self.keepers {
            out.extend_from_slice(&s.key);
            out.push(u8::from(s.own));
            match &s.newest {
                None => out.push(0),
                Some(t) => {
                    out.push(1);
                    put_tick(out, t);
                }
            }
            put_point(out, s.final_of.as_ref());
        }
        crate::transducer::address::put_u32_count(out, self.residual.edges.len())?;
        for e in &self.residual.edges {
            out.extend_from_slice(&e.from);
            out.extend_from_slice(&e.to);
            out.extend_from_slice(&e.h.to_le_bytes());
            put_lanes(out, &e.delta);
        }
        match &self.residual.holonomy {
            None => out.push(0),
            Some(d) => {
                out.push(1);
                put_lanes(out, d);
            }
        }
        Some(())
    }

    /// The inverse of [`TStanding::put`]; `None` for another version or a
    /// body that does not hold the standing.
    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<TStanding> {
        if take_u8(b, p)? != TSTANDING_VERSION {
            return None;
        }
        let final_point = take_point(b, p)?;
        let advances = take_u64(b, p)?;
        let m = usize::try_from(u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?)).ok()?;
        *p += 4;
        // A keeper is at least its key and three flag bytes.
        if !holds(b, *p, m, 35) {
            return None;
        }
        let mut keepers = Vec::with_capacity(m);
        for _ in 0..m {
            let key = take_key(b, p)?;
            let own = match take_u8(b, p)? {
                0 => false,
                1 => true,
                _ => return None,
            };
            let newest = match take_u8(b, p)? {
                0 => None,
                1 => {
                    let h = take_u64(b, p)?;
                    let mut lanes = [0u64; 4];
                    for l in &mut lanes {
                        *l = take_u64(b, p)?;
                    }
                    Some(Tick { h, lanes, k: take_u64(b, p)? })
                }
                _ => return None,
            };
            keepers.push(KeeperSeen { key, own, newest, final_of: take_point(b, p)? });
        }
        let m = usize::try_from(u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?)).ok()?;
        *p += 4;
        // An edge is two keys, a tick and four signed lanes: 136 bytes.
        if !holds(b, *p, m, 136) {
            return None;
        }
        let mut edges = Vec::with_capacity(m);
        for _ in 0..m {
            let (from, to, h) = (take_key(b, p)?, take_key(b, p)?, take_u64(b, p)?);
            edges.push(Edge { from, to, h, delta: take_lanes(b, p)? });
        }
        let holonomy = match take_u8(b, p)? {
            0 => None,
            1 => Some(take_lanes(b, p)?),
            _ => return None,
        };
        Some(TStanding { final_point, advances, keepers, residual: Residual { edges, holonomy } })
    }
}
