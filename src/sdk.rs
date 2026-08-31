// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE KERNEL SDK — how an external kernel joins the mesh and DEVELOPS on it.
//! Sodalitum, netstratum, or any newcomer links this to: take a stable mesh
//! identity, PERCEIVE a scene pair (segment it — perception is the kernel's, never
//! the hypha's), SUBMIT the A→B demand to a hypha, receive the judge's VERDICT, and
//! LEARN the taught ANASTOME element so no dialect drifts from the mesh. No engine
//! internals leak across this seam: cells in, verdict out. Ported from the old
//! mycelium SDK (Session / kernel emblem / observe) onto the v0.1.0 percept wire.

use crate::cognition::{encode_percept, perceive};
use crate::network::wire::{Frame, Stratum};
use std::io::{self, Read, Write};
use std::net::TcpStream;

/// A kernel's stable mesh IDENTITY. The name deterministically fixes a 32-byte
/// address (its "voice"/seat), so a kernel keeps the same identity across
/// reconnects (residency — the seat outlives the socket). Consilience needs two
/// DISTINCT kernels to agree, so distinct names give distinct voices.
#[derive(Clone, Debug)]
pub struct Kernel {
    name: String,
    voice: [u8; 32],
}

impl Kernel {
    /// Seat a kernel by name — the emblem digest is its durable address.
    pub fn named(name: &str) -> Self {
        let voice = *blake3::hash(format!("mycelium-kernel:{name}").as_bytes()).as_bytes();
        Kernel { name: name.to_string(), voice }
    }
    pub fn voice(&self) -> [u8; 32] {
        self.voice
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// The judge's answer to a submitted demand, plus any taught ANASTOME element the
/// kernel learns (the mesh's word for what it just described).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// The demand is an installed primitive — recalled (the mesh already speaks it).
    Recall { taught: Vec<u8> },
    /// A novel germ awaiting a second kernel's voice (consilience pending).
    GermPending,
    /// A novel germ that reached two-voice consilience and installed.
    Installed { taught: Vec<u8> },
    /// The demand is a COMPOSITION of installed generators — the algebra spans it,
    /// nothing minted (a sentence).
    Composed,
    /// The scene fragmented into per-body sub-demands.
    Fragmented,
    /// No motor closes it and no coboundary germinated — an empty/degenerate demand.
    Pivot,
    /// An unrecognized reply code (forward-compat).
    Unknown(u8),
}

impl Verdict {
    pub fn label(&self) -> &'static str {
        match self {
            Verdict::Recall { .. } => "recall",
            Verdict::GermPending => "germ-pending",
            Verdict::Installed { .. } => "installed",
            Verdict::Composed => "composed",
            Verdict::Fragmented => "fragmented",
            Verdict::Pivot => "pivot",
            Verdict::Unknown(_) => "unknown",
        }
    }
    /// The taught ANASTOME element the mesh handed back (empty when none).
    pub fn taught(&self) -> &[u8] {
        match self {
            Verdict::Recall { taught } | Verdict::Installed { taught } => taught,
            _ => &[],
        }
    }
}

/// A kernel's session with a hypha. The v0.1.0 door serves one demand per
/// connection, so [`Session::submit`] dials per demand — reconnection IS the
/// residency model (the kernel's identity is stable, the socket is not).
pub struct Session {
    addr: String,
    kernel: Kernel,
}

impl Session {
    /// Point a kernel at a hypha address (e.g. `"127.0.0.1:7500"`). No socket is
    /// held; each submit dials fresh.
    pub fn open(addr: &str, kernel: Kernel) -> Self {
        Session { addr: addr.to_string(), kernel }
    }

    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    /// PERCEIVE the scene pair (segment A and B into bodies) and SUBMIT it as an
    /// A→B demand under this kernel's voice; return the judge's verdict with any
    /// taught element learned. The hypha only judges — the segmentation is ours.
    pub fn submit(&self, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) -> io::Result<Verdict> {
        let (a_bodies, b_bodies) = perceive(a, b);
        let mut pbody = vec![0x00u8]; // kind 0 = a client-origin percept
        pbody.extend_from_slice(&encode_percept(&a_bodies, &b_bodies, &self.kernel.voice));
        let frame = Frame { lane: Stratum::Spatial, body: pbody };
        let bytes = frame.encode().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "frame encode"))?;

        let mut s = TcpStream::connect(&self.addr)?;
        s.write_all(&bytes)?;
        let mut head = [0u8; 5];
        s.read_exact(&mut head)?;
        let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
        let mut body = vec![0u8; len];
        s.read_exact(&mut body)?;
        Ok(decode_verdict(&body))
    }
}

/// Decode a hypha reply: `[code][metabolism 4][taught_len u16][taught…]`.
fn decode_verdict(body: &[u8]) -> Verdict {
    let taught = {
        let tl = body.get(5..7).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize).unwrap_or(0);
        body.get(7..7 + tl).map(<[u8]>::to_vec).unwrap_or_default()
    };
    match body.first() {
        Some(1) => Verdict::Recall { taught },
        Some(2) => Verdict::GermPending,
        Some(3) => Verdict::Installed { taught },
        Some(4) => Verdict::Pivot,
        Some(5) => Verdict::Fragmented,
        Some(6) => Verdict::Composed,
        Some(&c) => Verdict::Unknown(c),
        None => Verdict::Unknown(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A kernel's identity is stable and name-derived; distinct names → distinct
    /// voices (so two named kernels can reach consilience, one cannot with itself).
    #[test]
    fn kernel_identity_is_stable_and_distinct() {
        assert_eq!(Kernel::named("sod").voice(), Kernel::named("sod").voice(), "stable seat");
        assert_ne!(Kernel::named("sod").voice(), Kernel::named("net").voice(), "distinct kernels");
    }

    #[test]
    fn verdicts_decode_from_the_wire_reply() {
        // code 3 (installed), metabolism 4 bytes, taught len 2, taught [0xAB,0xCD].
        let body = vec![3u8, 0, 0, 0, 0, 2, 0, 0xAB, 0xCD];
        match decode_verdict(&body) {
            Verdict::Installed { taught } => assert_eq!(taught, vec![0xAB, 0xCD]),
            v => panic!("expected Installed, got {v:?}"),
        }
        assert_eq!(decode_verdict(&[6, 0, 0, 0, 0, 0, 0]), Verdict::Composed);
        assert_eq!(decode_verdict(&[2, 0, 0, 0, 0, 0, 0]), Verdict::GermPending);
    }
}
