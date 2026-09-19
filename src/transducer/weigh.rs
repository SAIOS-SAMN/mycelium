// Z10, §8.1 — WHAT A CARRIER ACTUALLY COSTS: its keys AND its values, in
// bytes, walked.
//
//! THE INSTRUMENT WAS COUNTING HALF THE OBJECT. `Keyed::key_bytes` summed
//! the bytes a carrier's keys occupy and reported the genome at `0 bytes,
//! +65 entries` — sixty-five decoded forms absorbed in six minutes, each
//! the size of a fleet form, weighed as nothing. A table that reads 3 MB a
//! minute against a process growing 157 MB a minute is not reading the
//! process.
//!
//! So a weight here is the whole object: the tape it carries, the form it
//! decoded to, every vertex's address and carriage, every face's transport.
//! A `Held`'s form above all, because that is what the genome holds and
//! the genome is where long-term memory lives by design.
//!
//! WHAT IS EXACT AND WHAT IS NOT, stated rather than blurred. Byte buffers,
//! slices and counts are exact. An exact rational is two arbitrary-precision
//! integers whose heap footprint is their magnitude, and `bits()` is what
//! the library will tell us about it, so a `Q` is weighed as the bytes its
//! numerator and denominator need plus the two headers — close, and never
//! claimed as the allocator's own figure. The reconciliation against
//! jemalloc's `allocated` is what says whether the difference matters.
//!
//! It reads and never decides: nothing in the engine branches on a weight.

use crate::transducer::element::{Element, Held, Versor};
use crate::transducer::sentence::{Form, Vertex};

/// The bytes an object reaches, including everything it owns.
pub trait Weigh {
    fn weigh(&self) -> usize;
}

/// Two arbitrary-precision integers and their headers. `bits()` is the
/// magnitude the library reports; the rest is the struct itself.
fn weigh_q(q: &crate::core::Q) -> usize {
    use num_traits::Signed;
    let n = q.numer().abs().bits().div_ceil(8) as usize;
    let d = q.denom().abs().bits().div_ceil(8) as usize;
    n + d + 2 * std::mem::size_of::<num_bigint::BigInt>()
}

impl Weigh for crate::transducer::alphabet::Rotor {
    fn weigh(&self) -> usize {
        // index: the rotor's sixteen blades, a fixed physical size
        std::mem::size_of::<Self>() + (0..16).map(|m| weigh_q(self.coeff(m))).sum::<usize>()
    }
}

impl Weigh for Versor {
    fn weigh(&self) -> usize {
        // index: the conformal algebra's thirty-two blades, a fixed size
        std::mem::size_of::<Self>() + (0..32).map(|m| weigh_q(self.coeff(m))).sum::<usize>()
    }
}

impl Weigh for Vertex {
    fn weigh(&self) -> usize {
        let mut n = std::mem::size_of::<Self>();
        match self {
            Vertex::Frame => {}
            Vertex::Word { address, derivation, acts, .. } => {
                n += address.as_bytes().len() + derivation.rotor().weigh();
                for (k, a) in acts.scoped() {
                    n += k.len() + a.chart.key().as_bytes().len() + a.carriage.weigh() + a.gauge.len() * 2;
                }
            }
            Vertex::Rebased { acts, .. } => {
                for (k, a) in acts.scoped() {
                    n += k.len() + a.carriage.weigh() + a.gauge.len() * 2;
                }
            }
            Vertex::Glued { address, chart, .. } => {
                n += address.as_bytes().len() + chart.key().as_bytes().len();
            }
            Vertex::Demand { chart, bodies, .. } => {
                n += chart.key().as_bytes().len() + bodies.len() * std::mem::size_of::<crate::transducer::sentence::Link>();
            }
            Vertex::Unfolded { .. } => {}
        }
        n
    }
}

impl Weigh for Form {
    fn weigh(&self) -> usize {
        let mut n = std::mem::size_of::<Self>();
        for v in &self.vertices {
            n += v.weigh();
        }
        for f in &self.faces {
            n += std::mem::size_of_val(f) + f.word.rotor().weigh();
            for (k, p) in f.at.scoped() {
                n += k.len() + p.carriage.weigh();
                if let Some(e) = &p.edge {
                    n += e.word.rotor().weigh() + e.carriage.weigh() + e.gauge.len() * 2;
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
                + lp.holonomy.rotor().weigh();
            for (k, c) in lp.carriages.scoped() {
                n += k.len() + c.weigh();
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
            n += c.weigh();
        }
        n
    }
}

impl Weigh for Element {
    fn weigh(&self) -> usize {
        std::mem::size_of::<Self>()
            + self.description().len()
            + self.sections().iter().map(|s| s.len() * 2).sum::<usize>()
    }
}

impl Weigh for Held {
    fn weigh(&self) -> usize {
        match self {
            Held::Word(e) => e.weigh(),
            Held::Sentence(e, f) => e.weigh() + f.weigh(),
        }
    }
}
