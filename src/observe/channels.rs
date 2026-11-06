// MU-PRECURSORS §3.1 — THE CHANNELS READING: every peer a host met, by name,
// the channel it spoke (Noise or plaintext) and the side it stood on, with how
// many connections, so a plaintext peer is named on the observer and never
// hidden behind the sealed ones.
//
//! Read whole at the question off the channel's own book
//! (`network::channel::book`). A dialled peer is named by the address
//! dialled; an accepted one by its static Noise key, or in plaintext by its
//! address without the port it chose.

use crate::network::channel::{Kind, Met, Side};

/// THE PEERS A HOST MET, in the order first met.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Channels {
    pub met: Vec<Met>,
}

/// FORMAT: channel kind byte of a Noise channel in the channels reading.
const KIND_NOISE: u8 = 0;
/// FORMAT: channel kind byte of a plaintext channel in the channels reading.
const KIND_PLAIN: u8 = 1;
/// FORMAT: side byte of a dialled peer in the channels reading.
const SIDE_DIALLED: u8 = 0;
/// FORMAT: side byte of an accepted peer in the channels reading.
const SIDE_ACCEPTED: u8 = 1;

impl Channels {
    /// The book as it stands now.
    pub fn now() -> Channels {
        Channels { met: crate::network::channel::book() }
    }

    /// `[count u32]` then per peer `[kind u8][side u8][count u64][peer: len
    /// u16, bytes]`; `None` where a count or a name passes its field.
    pub(crate) fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        crate::transducer::address::put_u32_count(out, self.met.len())?;
        for m in &self.met {
            out.push(match m.kind {
                Kind::Noise => KIND_NOISE,
                Kind::Plain => KIND_PLAIN,
            });
            out.push(match m.side {
                Side::Dialled => SIDE_DIALLED,
                Side::Accepted => SIDE_ACCEPTED,
            });
            out.extend_from_slice(&m.count.to_le_bytes());
            let len = u16::try_from(m.peer.len()).ok()?;
            out.extend_from_slice(&len.to_le_bytes());
            out.extend_from_slice(m.peer.as_bytes());
        }
        Some(())
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Channels> {
        let n = usize::try_from(u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?)).ok()?;
        *p += 4;
        // Each peer is at least its two bytes, its count and its name's length.
        if n > b.len().checked_sub(*p)? / (2 + 8 + 2) {
            return None;
        }
        let mut met = Vec::with_capacity(n);
        for _ in 0..n {
            let kind = match *b.get(*p)? {
                KIND_NOISE => Kind::Noise,
                KIND_PLAIN => Kind::Plain,
                _ => return None,
            };
            let side = match *b.get(*p + 1)? {
                SIDE_DIALLED => Side::Dialled,
                SIDE_ACCEPTED => Side::Accepted,
                _ => return None,
            };
            *p += 2;
            let count = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            let len = usize::from(u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?));
            *p += 2;
            let peer = String::from_utf8(b.get(*p..*p + len)?.to_vec()).ok()?;
            *p += len;
            met.push(Met { peer, kind, side, count });
        }
        Some(Channels { met })
    }

    /// The reading as the console prints it: the count of peers per channel,
    /// then each peer by name, a plaintext one said in capitals.
    pub fn lines(&self) -> Vec<String> {
        let noise = self.met.iter().filter(|m| m.kind == Kind::Noise).count();
        let plain = self.met.iter().filter(|m| m.kind == Kind::Plain).count();
        let mut out = vec![format!("channels: {noise} peers inside Noise, {plain} in plaintext")];
        for m in &self.met {
            let kind = match m.kind {
                Kind::Noise => "noise",
                Kind::Plain => "PLAINTEXT",
            };
            let side = match m.side {
                Side::Dialled => "dialled",
                Side::Accepted => "accepted",
            };
            out.push(format!("  {kind} {side} {} · {} connections", m.peer, m.count));
        }
        out
    }
}
