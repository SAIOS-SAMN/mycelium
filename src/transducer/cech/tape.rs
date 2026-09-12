// 28 — THE COMPLEX ON DISK: every cell written whole, and a tape that does
// not parse refused rather than partly restored.

use super::*;

impl Complex {
    pub fn byte_len(&self) -> usize {
        let mut n = 0usize;
        // index: keyed cells of this complex (persistence size)
        for (k, d) in self.by_key.fiber(ROOT) {
            if let Some(c) = self.degrees.get(*d as usize).and_then(|l| l.get(&k)) {
                n += 32 * c.vertices.len() + 32 + 32;
            }
        }
        n
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.sections.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.sections, |k, s| {
            crate::transducer::address::put_addr(out, k);
            crate::transducer::address::put_addr(out, &s.addr);
            s.gamma.to_bytes(out);
            s.carriage.to_bytes(out);
        });
        out.extend_from_slice(&(self.degrees.len() as u32).to_le_bytes());
        for layer in &self.degrees {
            out.extend_from_slice(&(layer.len() as u32).to_le_bytes());
            crate::snapshot::persist(layer, |_, c| {
                out.push(c.vertices.len() as u8);
                for v in &c.vertices {
                    crate::transducer::address::put_addr(out, v);
                }
                c.coefficient.to_bytes(out);
                c.carriage.to_bytes(out);
                for x in &c.volume {
                    out.extend_from_slice(&x.to_le_bytes());
                }
                // A CLASS IS WRITTEN AS ITS COORDINATES. `class_key` already
                // writes 12.6's axes out in order — conjugacy invariants,
                // plane slots, hue, carriage — and `from_key` reads them
                // back, so the wire carries the coordinates and no second
                // codec exists to drift from them (law 12).
                match &c.fill {
                    Fill::Open | Fill::Already => out.push(0),
                    Fill::Filled(g, carriage) => {
                        out.push(1);
                        g.to_bytes(out);
                        carriage.to_bytes(out);
                    }
                    Fill::Class(k, r) => {
                        out.push(2);
                        crate::transducer::address::put_addr(out, k.class_key());
                        r.to_bytes(out);
                    }
                    Fill::Congruent(g, ell) => {
                        out.push(3);
                        g.to_bytes(out);
                        out.extend_from_slice(&ell.to_le_bytes());
                    }
                    Fill::Unspellable(r) => {
                        out.push(4);
                        r.to_bytes(out);
                    }
                }
            });
        }
        out.extend_from_slice(&(self.remainder.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.remainder, |k, _| {
            crate::transducer::address::put_addr(out, k);
        });
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Complex> {
        let take_u32 = |b: &[u8], pos: &mut usize| -> Option<u32> {
            let v = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
            *pos += 4;
            Some(v)
        };
        let take_id = |b: &[u8], pos: &mut usize| crate::transducer::address::take_addr(b, pos);
        let mut c = Complex::new();
        let ns = take_u32(b, pos)? as usize;
        for _ in 0..ns.min(1 << 16) {
            let v = take_id(b, pos)?;
            let addr = take_id(b, pos)?;
            let gamma = Rotor::from_bytes(b, pos)?;
            let carriage = Versor::from_bytes(b, pos)?;
            c.sections.insert(v, Section { addr, gamma, carriage });
        }
        let nd = take_u32(b, pos)? as usize;
        for _ in 0..nd.min(64) {
            let n = take_u32(b, pos)? as usize;
            for _ in 0..n.min(1 << 16) {
                let nv = *b.get(*pos)? as usize;
                *pos += 1;
                let mut verts = Vec::with_capacity(nv.min(32));
                for _ in 0..nv.min(32) {
                    verts.push(take_id(b, pos)?);
                }
                let coefficient = Rotor::from_bytes(b, pos)?;
                let carriage = Versor::from_bytes(b, pos)?;
                let mut volume = [0u64; 4];
                for x in &mut volume {
                    *x = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                }
                let tag = *b.get(*pos)?;
                *pos += 1;
                let fill = match tag {
                    1 => Fill::Filled(
                        Rotor::from_bytes(b, pos)?,
                        Box::new(Versor::from_bytes(b, pos)?),
                    ),
                    2 => {
                        let k = take_id(b, pos)?;
                        let kappa =
                            crate::transducer::sentence::ClassVector::from_key(k.as_bytes())?;
                        Fill::Class(Box::new(kappa), Box::new(Rotor::from_bytes(b, pos)?))
                    }
                    3 => {
                        let g = Rotor::from_bytes(b, pos)?;
                        let ell = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
                        // A17, as at every other tape: a congruence is
                        // believed only where the rotation closes at ℓ.
                        if ell == 0 {
                            return None;
                        }
                        *pos += 4;
                        Fill::Congruent(g, ell)
                    }
                    4 => Fill::Unspellable(Box::new(Rotor::from_bytes(b, pos)?)),
                    _ => Fill::Open,
                };
                c.remember(Cell {
                    vertices: sorted_verts(verts),
                    coefficient,
                    carriage,
                    volume,
                    fill,
                });
            }
        }
        // A tape that ends here carries no remainder cells.
        let nr = match take_u32(b, pos) {
            Some(n) => n as usize,
            None => 0,
        };
        for _ in 0..nr.min(1 << 16) {
            let k = take_id(b, pos)?;
            c.remainder.insert(k, ());
        }
        Some(c)
    }

}
