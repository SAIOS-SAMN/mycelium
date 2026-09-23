// Z10, §8.1 — WHAT A CARRIER ACTUALLY COSTS: its keys AND its values, in
// bytes, walked.
//
//! THE INSTRUMENT WAS COUNTING HALF THE OBJECT. `Keyed::key_bytes` summed
//! the bytes a carrier's keys occupy and reported the genome at `0 bytes,
//! +65 entries` — sixty-five decoded forms absorbed in six minutes, each
//! the size of a fleet form, counted as nothing. A table that reads 3 MB a
//! minute against a process growing 157 MB a minute is not reading the
//! process.
//!
//! So a count here is the whole object: the tape it carries, the form it
//! decoded to, every vertex's address and carriage, every face's transport.
//! A `Held`'s form above all, because that is what the genome holds and
//! the genome is where long-term memory lives by design.
//!
//! WHAT IS EXACT AND WHAT IS NOT, stated rather than blurred. Byte buffers,
//! slices and counts are exact. An exact rational is two arbitrary-precision
//! integers whose heap footprint is their magnitude, and `bits()` is what
//! the library will tell us about it, so a `Q` is counted as the bytes its
//! numerator and denominator need plus the two headers — close, and never
//! claimed as the allocator's own figure. The reconciliation against
//! jemalloc's `allocated` is what says whether the difference matters.
//!
//! It reads and never decides: nothing in the engine branches on a size.
//!
//! THE NOUN IS `bytes`, NOT `weight`. `Rotor::weight` is `Φ Φ̃`, Study's
//! condition on a versor and a fact of the algebra (A1), and the peel's
//! Gaussian weight is a third thing again. A byte count is none of those,
//! and three readings of one word is what §2.1 refuses; renamed from
//! `Weigh`/`weigh` at the coordinator's ruling of 2026-09-19.

use crate::transducer::element::{Element, Held, Versor};
use crate::transducer::sentence::{Form, Vertex};

/// The bytes an object reaches, including everything it owns.
pub trait Bytes {
    fn bytes(&self) -> usize;
}

/// THE HEAP BEHIND A RATIONAL, and only that.
///
/// It added `2 * size_of::<BigInt>()` for the two integers' headers, and
/// those headers are INLINE in the struct the caller has already counted
/// with `size_of::<Self>()` — a `Versor` is thirty-two `BigRational`s laid
/// out in an array, so every header was counted twice. Calibrated on the
/// allocator: the walk read 1.76x the allocator's figure on a thousand
/// versors and 1.44x on a 713-vertex form, heavy where I had expected it
/// light.
///
/// What is left is the magnitude: the limbs each integer needs, which is
/// what `bits()` reports and what the allocator actually went and got.
fn bytes_q(q: &crate::core::Q) -> usize {
    use num_traits::Signed;
    let n = q.numer().abs().bits().div_ceil(8) as usize;
    let d = q.denom().abs().bits().div_ceil(8) as usize;
    n + d
}

impl Bytes for crate::transducer::alphabet::Rotor {
    fn bytes(&self) -> usize {
        // index: the rotor's sixteen blades, a fixed physical size
        std::mem::size_of::<Self>() + (0..16).map(|m| bytes_q(self.coeff(m))).sum::<usize>()
    }
}

impl Bytes for Versor {
    fn bytes(&self) -> usize {
        // index: every slot the value's own algebra holds (71.2(3))
        std::mem::size_of::<Self>() + (0..self.slot_count()).map(|m| bytes_q(self.coeff(m))).sum::<usize>()
    }
}

impl Bytes for crate::transducer::blade::Blade {
    fn bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            + self.spatial.bytes()
            + self.inertia.iter().map(bytes_q).sum::<usize>()
            + self.quartic.iter().flat_map(|t| t.iter()).map(bytes_q).sum::<usize>()
            + self.gauge.iter().map(|(_, q)| 1 + bytes_q(q)).sum::<usize>()
            + bytes_q(&self.shell)
            + self
                .support
                .iter()
                .map(|s| std::mem::size_of_val(s) + s.n() * std::mem::size_of::<i64>())
                .sum::<usize>()
    }
}

impl Bytes for Vertex {
    fn bytes(&self) -> usize {
        let mut n = std::mem::size_of::<Self>();
        match self {
            Vertex::Frame => {}
            Vertex::Word { address, derivation, acts, .. } => {
                n += address.as_bytes().len() + derivation.rotor().bytes();
                for (k, a) in acts.scoped() {
                    n += k.len() + a.chart.key().as_bytes().len() + a.carriage.bytes() + a.gauge.len() * 2;
                }
            }
            Vertex::Rebased { acts, .. } => {
                for (k, a) in acts.scoped() {
                    n += k.len() + a.carriage.bytes() + a.gauge.len() * 2;
                }
            }
            Vertex::Glued { chart, .. } => {
                n += chart.key().as_bytes().len();
            }
            Vertex::Demand { chart, bodies, .. } => {
                n += chart.key().as_bytes().len() + bodies.len() * std::mem::size_of::<crate::transducer::sentence::Link>();
            }
            Vertex::Unfolded { .. } => {}
        }
        n
    }
}

impl Bytes for Form {
    fn bytes(&self) -> usize {
        let mut n = std::mem::size_of::<Self>();
        for v in &self.vertices {
            n += v.bytes();
        }
        for f in &self.faces {
            n += std::mem::size_of_val(f) + f.word.rotor().bytes();
            for (k, p) in f.at.scoped() {
                n += k.len() + p.carriage.bytes();
                if let Some(e) = &p.edge {
                    n += e.word.rotor().bytes() + e.carriage.bytes() + e.gauge.len() * 2;
                }
            }
            if let Some(ps) = &f.pairs {
                n += ps.key_bytes();
            }
        }
        for lp in &self.loops {
            n += std::mem::size_of_val(lp)
                + lp.vertices.len() * 2
                + lp.faces.len() * 2
                + lp.holonomy.rotor().bytes();
            for (k, c) in lp.carriages.scoped() {
                n += k.len() + c.bytes();
            }
        }
        // The indexes the form keeps current; keys only, their values are
        // the indices above.
        n += self.by_address.key_bytes()
            + self.by_class.key_bytes()
            + self.by_transport.key_bytes()
            + self.star.key_bytes()
            + self.carriages.key_bytes()
            + self.glued.key_bytes()
            + self.sections.key_bytes()
            + self.hue_maps.key_bytes()
            + self.torsion.key_bytes()
            + self.axes.key_bytes();
        for (_, c) in self.carriages.scoped() {
            n += c.bytes();
        }
        n
    }
}

impl Bytes for Element {
    fn bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            + self.description().len()
            + self.sections().iter().map(|s| s.len() * 2).sum::<usize>()
    }
}

impl Bytes for Held {
    fn bytes(&self) -> usize {
        match self {
            Held::Word(e) => e.bytes(),
            Held::Sentence(e, f) => e.bytes() + f.bytes(),
        }
    }
}
