// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! COGNITION — the kernel loop. A demand (scene A → scene B) enters as raw
//! cells; the transducer lifts blades, the unified solve extracts one conformal
//! motor, conception classifies it against the installed basis, and a novel
//! germ installs ONLY on two-voice consilience (`consensus`). State is
//! Vec-of-relations throughout — the basis is a Vec of motors, admission and the
//! nerve are Vec-backed. No digest-keyed cognition (Anti-Flattening Law); the
//! motor digest below is a WIRE identity for matching receipts, never reasoned on.

use crate::network::consensus::{Admission, Receipt};
use crate::network::nerve::Nerve;
use crate::transducer::blade::transduce;
use crate::transducer::conception::recognize_scene;
use crate::transducer::motor::{ConformalMotor, D4};

/// The verdict of ingesting one demand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The motor is already an installed primitive — recall.
    Recognised(usize),
    /// A novel germ awaiting a second voice (this voice recorded).
    GermPending,
    /// A novel germ that just reached quorum and installed to the basis.
    Installed(usize),
    /// No motor closes the demand — an irreducible residual (a pivot).
    Pivot,
}

/// The kernel's evolving state: the installed generator basis + the germ
/// admission ledger + the Čech peer table.
#[derive(Default)]
pub struct Kernel {
    basis: Vec<ConformalMotor>,
    admission: Admission,
    pub nerve: Nerve,
}

impl Kernel {
    pub fn new() -> Self {
        Kernel { basis: Vec::new(), admission: Admission::new(), nerve: Nerve::new() }
    }

    pub fn basis_len(&self) -> usize {
        self.basis.len()
    }

    /// Ingest a demand from `voice`: transduce → solve → conceive → admit.
    pub fn ingest(&mut self, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)], voice: [u8; 32]) -> Outcome {
        let (ba, bb) = (transduce(a, 0), transduce(b, 1));
        let Some(m) = recognize_scene(&ba, &bb) else {
            return Outcome::Pivot;
        };
        if let Some(i) = self.basis.iter().position(|p| *p == m) {
            return Outcome::Recognised(i);
        }
        let germ = motor_digest(&m);
        if self.admission.record(Receipt { voice, germ }) {
            self.basis.push(m);
            Outcome::Installed(self.basis.len() - 1)
        } else {
            Outcome::GermPending
        }
    }
}

// ── wire identity of a motor (transport only, not cognition) ──────────────────

fn push_ratio(out: &mut Vec<u8>, r: &num_rational::BigRational) {
    for part in [r.numer(), r.denom()] {
        let b = part.to_signed_bytes_le();
        out.extend_from_slice(&(b.len() as u32).to_le_bytes());
        out.extend_from_slice(&b);
    }
}

fn d4_byte(d: D4) -> u8 {
    D4::ALL.iter().position(|x| *x == d).unwrap_or(0) as u8
}

/// Canonical bytes of a motor — for the wire and for the receipt digest.
pub fn motor_bytes(m: &ConformalMotor) -> Vec<u8> {
    let mut out = Vec::new();
    push_ratio(&mut out, &m.dilation);
    out.push(d4_byte(m.rotation));
    push_ratio(&mut out, &m.translation.0);
    push_ratio(&mut out, &m.translation.1);
    out
}

/// The germ's transport identity: a digest of the motor bytes. Used to MATCH
/// consilience receipts across voices — never as a reasoning operand.
pub fn motor_digest(m: &ConformalMotor) -> [u8; 32] {
    *blake3::hash(&motor_bytes(m)).as_bytes()
}

// ── demand wire codec (server ⇄ client) ──────────────────────────────────────

fn push_cells(out: &mut Vec<u8>, cells: &[(i64, i64, u8)]) {
    out.extend_from_slice(&(cells.len() as u32).to_le_bytes());
    for (r, c, h) in cells {
        out.extend_from_slice(&r.to_le_bytes());
        out.extend_from_slice(&c.to_le_bytes());
        out.push(*h);
    }
}

fn read_cells(b: &[u8], p: &mut usize) -> Option<Vec<(i64, i64, u8)>> {
    let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let r = i64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        let c = i64::from_le_bytes(b.get(*p + 8..*p + 16)?.try_into().ok()?);
        let h = *b.get(*p + 16)?;
        *p += 17;
        out.push((r, c, h));
    }
    Some(out)
}

/// Encode a demand body: `voice(32) | scene A | scene B`.
pub fn encode_demand(a: &[(i64, i64, u8)], b: &[(i64, i64, u8)], voice: &[u8; 32]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(voice);
    push_cells(&mut out, a);
    push_cells(&mut out, b);
    out
}

#[allow(clippy::type_complexity)]
pub fn decode_demand(body: &[u8]) -> Option<([u8; 32], Vec<(i64, i64, u8)>, Vec<(i64, i64, u8)>)> {
    let voice: [u8; 32] = body.get(0..32)?.try_into().ok()?;
    let mut p = 32;
    let a = read_cells(body, &mut p)?;
    let b = read_cells(body, &mut p)?;
    Some((voice, a, b))
}

/// One byte on the wire summarising an outcome (server → client reply).
pub fn outcome_code(o: &Outcome) -> u8 {
    match o {
        Outcome::Recognised(_) => 1,
        Outcome::GermPending => 2,
        Outcome::Installed(_) => 3,
        Outcome::Pivot => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scene(sr: i64, sc: i64) -> Vec<(i64, i64, u8)> {
        vec![(sr, sc, 1), (sr, sc + 1, 1), (sr + 1, sc, 1)]
    }

    /// A novel translation germinates: one voice pends, a second installs it;
    /// afterward the same demand recalls.
    #[test]
    fn a_germ_installs_on_two_voices_then_recalls() {
        let mut k = Kernel::new();
        let (a, b) = (scene(0, 0), scene(0, 3)); // +0,+3 translate
        assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
        assert_eq!(k.basis_len(), 0, "one voice does not install");
        assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::Installed(0), "two voices install");
        assert_eq!(k.basis_len(), 1);
        assert_eq!(k.ingest(&a, &b, [3; 32]), Outcome::Recognised(0), "now recalled");
    }

    /// An irreducible residual is a pivot, never installed.
    #[test]
    fn a_pivot_never_installs() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let b = vec![(0, 0, 1), (0, 1, 1), (0, 2, 1)]; // L → bar
        assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::Pivot);
        assert_eq!(k.basis_len(), 0);
    }

    /// The demand codec roundtrips.
    #[test]
    fn the_demand_codec_roundtrips() {
        let (a, b) = (scene(2, 2), scene(2, 5));
        let (v, ra, rb) = decode_demand(&encode_demand(&a, &b, &[9; 32])).unwrap();
        assert_eq!((v, ra, rb), ([9; 32], a, b));
    }
}
