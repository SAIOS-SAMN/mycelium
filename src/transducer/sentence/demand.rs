// 50.5 — THE DEMAND: what the speaker could not spell or the eyes could not
// pair, typed; it rides the sentence as a vertex and reconstructs nothing.
//
//! Five kinds and no other. A demand of no kind is not a demand, and the
//! court refuses a vertex whose kind carries nothing (`REFUSE_DEMAND_UNTYPED`).
//! The key is the kind byte and the coordinates it carries, written out (A21).

use crate::core::Q;
use crate::transducer::address::{Address, TAG_DEMAND};
use crate::transducer::alphabet::{put_q, take_q, Rotor};
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
    /// A rotor block the ordered peel (50.4) does not decompose, and the
    /// rotor it stood on: the canonical holonomy the peel could not spell,
    /// which is the candidate generator of 37.7 and carries itself as its
    /// witness (16.16 step 6). The block is that rotor's reading on the
    /// plane the peel stopped at and cannot witness it alone (checklist
    /// (m)). Written and keyed as the ray's canonical coordinates (A1).
    /// Boxed: four rationals and sixteen beside variants of a few bytes.
    Standing { block: Box<[[Q; 2]; 2]>, rotor: Box<Rotor> },
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
    /// standing witness with no weight or the identity, a source no chart
    /// carries onto (that is a creation), a deformation on no axis.
    pub fn typed(&self) -> Result<(), &'static str> {
        let ok = match self {
            // A witness of all zeros names a prime it does not carry:
            // 37.7 P2's gcd has nothing to read (`REFUSE_LETTER_WITNESS`
            // is the court's own check on the same block). The prime is
            // OUTSIDE THE ALPHABET or the demand is not one — asked of
            // the alphabet itself, never of the literal 13, which is the
            // (5,12,13) letter's norm written out as a constant (rule 6).
            Demand::RotorPrime { plane, prime, witness } => {
                plane.0 != plane.1
                    && crate::transducer::alphabet::prime_outside_the_alphabet(
                        &num_bigint::BigInt::from(*prime),
                        *plane,
                    )
                    .is_some()
                    && witness.iter().flatten().any(|q| !q.is_zero())
            }
            // THE WITNESS IS THE CONTENT (checklist (m)): a versor that is
            // not the identity. The block is that rotor's reading on one
            // plane and may be zero — a rotor carrying the plane wholly off
            // itself has a zero shadow there and is a demand for all of it.
            Demand::Standing { rotor, .. } => rotor.weight().is_some() && !rotor.is_identity(),
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
            Demand::Standing { block, rotor } => {
                for row in block.iter() {
                    for q in row {
                        put_q(out, q);
                    }
                }
                // THE WITNESS IS THE RAY'S CANONICAL COORDINATES (A1, rule
                // 6): `λΦ` and `Φ` are one element (1.5), and this is the
                // key `demand_key` files the vertex under, so one ray is
                // one vertex and never one per scale.
                rotor.put_canonical(out);
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
                let rotor = Rotor::from_bytes(b, pos)?;
                Demand::Standing { block: Box::new([[a, bq], [c, d]]), rotor: Box::new(rotor) }
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
                if n > b.len().checked_sub(*pos)? / 2 {
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

/// THE TARGET A PROBE NAMES WHEN IT ANSWERS ONE DEMAND (L8q): the
/// listing's address and the demand's own key, both under one tag and
/// each length-delimited, so the court reads them back apart without
/// knowing how long either is. A probe at the listing's address alone
/// cannot say which vertex it closes, and a court that cannot say that
/// can only subtract cells.
pub fn demand_at(listing: &[u8], kind: &Demand, chart: &Invariant) -> Vec<u8> {
    let mut buf = vec![crate::transducer::address::TAG_DEMAND_AT];
    crate::transducer::address::put_bytes(&mut buf, listing);
    buf.extend_from_slice(&demand_key(kind, chart));
    buf
}

/// Read such a target back as (listing address, demand key). `None` when
/// the target names a listing and no vertex of it — which is the older
/// probe, and stays a probe against the listing's own chain.
pub fn read_demand_at(target: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    let (&tag, _) = target.split_first()?;
    if tag != crate::transducer::address::TAG_DEMAND_AT {
        return None;
    }
    let mut p = 1usize;
    let listing = crate::transducer::address::take_bytes(target, &mut p)?;
    let demand = target.get(p..)?.to_vec();
    if listing.is_empty() || demand.is_empty() {
        return None;
    }
    Some((listing, demand))
}
