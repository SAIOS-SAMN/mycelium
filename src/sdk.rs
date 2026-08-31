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
    /// The demand DECOMPOSED into a canonical sentence of installed generators (the
    /// confluent normal form) — the mesh already speaks it, nothing minted.
    Decomposed { taught: Vec<u8> },
    /// A novel generator awaiting a second kernel's voice (consilience pending).
    GermPending,
    /// An irreducible residual that reached consilience and adjoined (KB completion).
    Installed { taught: Vec<u8> },
    /// The scene fragmented into per-body sub-demands.
    Fragmented,
    /// A degenerate demand — nothing to reduce.
    Pivot,
    /// An unrecognized reply code (forward-compat).
    Unknown(u8),
}

impl Verdict {
    pub fn label(&self) -> &'static str {
        match self {
            Verdict::Decomposed { .. } => "decomposed",
            Verdict::GermPending => "germ-pending",
            Verdict::Installed { .. } => "installed",
            Verdict::Fragmented => "fragmented",
            Verdict::Pivot => "pivot",
            Verdict::Unknown(_) => "unknown",
        }
    }
    /// The taught ANASTOME element the mesh handed back (empty when none).
    pub fn taught(&self) -> &[u8] {
        match self {
            Verdict::Decomposed { taught } | Verdict::Installed { taught } => taught,
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
    /// A→B demand under this kernel's voice; return the judge's [`Reply`] — the
    /// verdict with any taught element, plus the mesh's per-lane METABOLISM
    /// (pressure) the kernel paces its intake against. The hypha only judges.
    pub fn submit(&self, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) -> io::Result<Reply> {
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
        Ok(Reply { verdict: decode_verdict(&body), metabolism: decode_metabolism(&body) })
    }
}

/// A hypha's answer: the verdict plus the per-lane METABOLISM (pressure) — the
/// backpressure signal a kernel throttles its intake against so the mesh is not
/// brute-forced (the syntropic/entropic metabolism as governor, never a scalar).
#[derive(Clone, Debug)]
pub struct Reply {
    pub verdict: Verdict,
    pub metabolism: [u8; 4],
}

impl Reply {
    /// The most-pressured lane — the polytonic backpressure a kernel paces on
    /// (Liebig: the bottleneck lane governs intake).
    pub fn load(&self) -> u8 {
        self.metabolism.iter().copied().max().unwrap_or(0)
    }
}

/// The per-lane pressure bytes from a reply (`body[1..5]`).
fn decode_metabolism(body: &[u8]) -> [u8; 4] {
    std::array::from_fn(|i| body.get(1 + i).copied().unwrap_or(0))
}

/// Decode a hypha reply: `[code][metabolism 4][taught_len u16][taught…]`.
fn decode_verdict(body: &[u8]) -> Verdict {
    let taught = {
        let tl = body.get(5..7).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize).unwrap_or(0);
        body.get(7..7 + tl).map(<[u8]>::to_vec).unwrap_or_default()
    };
    match body.first() {
        Some(1) => Verdict::Decomposed { taught },
        Some(2) => Verdict::GermPending,
        Some(3) => Verdict::Installed { taught },
        Some(4) => Verdict::Pivot,
        Some(5) => Verdict::Fragmented,
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
        assert_eq!(decode_verdict(&[1, 0, 0, 0, 0, 0, 0]), Verdict::Decomposed { taught: vec![] });
        assert_eq!(decode_verdict(&[2, 0, 0, 0, 0, 0, 0]), Verdict::GermPending);
    }
}
