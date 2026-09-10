// 50.5 — THE DEMAND: what the speaker could not spell or the eyes could not
// pair, typed; it rides the sentence as a vertex and reconstructs nothing.
//
//! Five kinds and no other. A demand of no kind is not a demand, and the
//! court refuses a vertex whose kind carries nothing (`REFUSE_DEMAND_UNTYPED`).
//! The key is the kind byte and the coordinates it carries, written out (A21).

use crate::core::Q;
use crate::transducer::address::{Address, TAG_DEMAND};
use crate::transducer::alphabet::{put_q, take_q};
use crate::transducer::stalk::Invariant;
use num_traits::Zero;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Demand {
    /// A plane rotation whose Gaussian factor is a prime outside the
    /// alphabet. `witness` is the plane block the prime was read off —
    /// the RESIDUE of 50.2 step 2, not the measured versor — in A1
    /// canonical form: 37.7 P2 reads `π_p = gcd_{ℤ[i]}(s + t·i, p)` off
    /// it, and two courts agreeing on that factor is what opens the
    /// letter (P3). Boxed for the same reason `Standing`'s block is.
    RotorPrime { plane: (u8, u8), prime: u64, witness: Box<[[Q; 2]; 2]> },
    /// A rotor block the ordered peel (50.4) does not decompose. Boxed:
    /// four rationals beside variants of a few bytes.
    Standing { block: Box<[[Q; 2]; 2]> },
    /// An output body some input charts carry onto: every one of them stands.
    Source { multiplicity: u32 },
    /// An output body no input body carries onto.
    Creation,
    /// A body whose image has different harmonics: no versor exists.
    TopologicalDeformation { image: Invariant, axes: Vec<u16> },
}

impl Demand {
    pub fn kind_byte(&self) -> u8 {
        match self {
            Demand::RotorPrime { .. } => 0,
            Demand::Standing { .. } => 1,
            Demand::Source { .. } => 2,
            Demand::Creation => 3,
            Demand::TopologicalDeformation { .. } => 4,
        }
    }

    /// The word of 34.2 and 34.3's `demand {kind}` lines.
    pub fn label(&self) -> &'static str {
        match self {
            Demand::RotorPrime { .. } => "rotor prime",
            Demand::Standing { .. } => "standing",
            Demand::Source { .. } => "source",
            Demand::Creation => "creation",
            Demand::TopologicalDeformation { .. } => "deformation",
        }
    }

    /// A demand carries the coordinates its kind names, or it is not a
    /// demand: a prime below the alphabet's own or a degenerate plane, a
    /// zero block, a source no chart carries onto (that is a creation), a
    /// deformation on no axis.
    pub fn typed(&self) -> Result<(), &'static str> {
        let ok = match self {
            // A witness of all zeros names a prime it does not carry:
            // 37.7 P2's gcd has nothing to read (`REFUSE_LETTER_WITNESS`
            // is the court's own check on the same block).
            Demand::RotorPrime { plane, prime, witness } => {
                plane.0 != plane.1 && *prime > 13 && witness.iter().flatten().any(|q| !q.is_zero())
            }
            Demand::Standing { block } => block.iter().flatten().any(|q| !q.is_zero()),
            Demand::Source { multiplicity } => *multiplicity > 0,
            Demand::Creation => true,
            Demand::TopologicalDeformation { axes, .. } => !axes.is_empty(),
        };
        if ok {
            Ok(())
        } else {
            Err(crate::court::verdict::REFUSE_DEMAND_UNTYPED)
        }
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.push(self.kind_byte());
        match self {
            Demand::RotorPrime { plane, prime, witness } => {
                out.push(plane.0);
                out.push(plane.1);
                out.extend_from_slice(&prime.to_le_bytes());
                for row in witness.iter() {
                    for q in row {
                        put_q(out, q);
                    }
                }
            }
            Demand::Standing { block } => {
                for row in block.iter() {
                    for q in row {
                        put_q(out, q);
                    }
                }
            }
            Demand::Source { multiplicity } => out.extend_from_slice(&multiplicity.to_le_bytes()),
            Demand::Creation => {}
            Demand::TopologicalDeformation { image, axes } => {
                let _ = crate::transducer::sentence::codec::put_inv(image, out);
                out.extend_from_slice(&(axes.len() as u16).to_le_bytes());
                for a in axes {
                    out.extend_from_slice(&a.to_le_bytes());
                }
            }
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Demand> {
        let kind = *b.get(*pos)?;
        *pos += 1;
        let d = match kind {
            0 => {
                let p0 = *b.get(*pos)?;
                let p1 = *b.get(*pos + 1)?;
                *pos += 2;
                let prime = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
                let w00 = take_q(b, pos)?;
                let w01 = take_q(b, pos)?;
                let w10 = take_q(b, pos)?;
                let w11 = take_q(b, pos)?;
                Demand::RotorPrime {
                    plane: (p0, p1),
                    prime,
                    witness: Box::new([[w00, w01], [w10, w11]]),
                }
            }
            1 => {
                let a = take_q(b, pos)?;
                let bq = take_q(b, pos)?;
                let c = take_q(b, pos)?;
                let d = take_q(b, pos)?;
                Demand::Standing { block: Box::new([[a, bq], [c, d]]) }
            }
            2 => {
                let m = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
                *pos += 4;
                Demand::Source { multiplicity: m }
            }
            3 => Demand::Creation,
            4 => {
                let image = crate::transducer::sentence::codec::take_inv(b, pos)?;
                let n = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?) as usize;
                *pos += 2;
                if n > b.len().saturating_sub(*pos) / 2 {
                    return None;
                }
                let mut axes = Vec::with_capacity(n);
                for _ in 0..n {
                    axes.push(u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?));
                    *pos += 2;
                }
                Demand::TopologicalDeformation { image, axes }
            }
            _ => return None,
        };
        Some(d)
    }

    /// A21: the kind byte and its coordinates, under the demand tag.
    pub fn key(&self) -> Address {
        let mut buf = vec![TAG_DEMAND];
        self.to_bytes(&mut buf);
        Address::new(buf)
    }
}

/// The key a sentence aggregates its demands under: one vertex per
/// (kind, chart), 50.5.
pub fn demand_key(kind: &Demand, chart: &Invariant) -> Vec<u8> {
    let mut buf = Vec::new();
    crate::transducer::address::put_bytes(&mut buf, kind.key().as_bytes());
    crate::transducer::address::put_bytes(&mut buf, chart.key().as_bytes());
    buf
}
