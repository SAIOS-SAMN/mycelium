// 28 — THE COMPLEX ON DISK: every cell written whole, and a tape that does
// not parse refused rather than partly restored.

use super::*;
use crate::snapshot::counted;

impl Complex {
    pub fn byte_len(&self) -> usize {
        let mut n = 0usize;
        // The cells of THIS complex — one chart's nerve — and the size on
        // the tape is the size of all of them.
        for (k, d) in self.by_key.scoped() {
            if let Some(c) = self.degrees.get(*d as usize).and_then(|l| l.get(&k)) {
                n += 32 * c.vertices.len() + 32 + 32;
            }
        }
        n
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        // EVERY COUNT HERE IS A READING OF WHAT WAS WRITTEN. Each of these
        // wrote `carrier.len()` — a stored counter — and then wrote the
        // content by walking the carrier, so one number had two independent
        // sources and nothing held them together (law 12). In the field they
        // parted: a court's COC1 section announced five complexes and held
        // four, and its own reader believed the announcement and read the
        // next section as content. `counted` writes the body first and
        // measures it as it goes, so the number is that act's own measure.
        counted(out, &self.sections, |body, k, s: &Section| {
            crate::transducer::address::put_addr(body, k)?;
            crate::transducer::address::put_addr(body, &s.addr)?;
            s.gamma.to_bytes(body);
            s.carriage.to_bytes(body);
            Some(())
        });
        out.extend_from_slice(&(self.degrees.len() as u64).to_le_bytes());
        for layer in &self.degrees {
            counted(out, layer, |body, _, c: &Cell| {
                // The vertex count rides in ONE BYTE, so a cell on more than
                // 255 vertices cannot be framed here and is refused rather
                // than written with a count that wrapped.
                crate::transducer::address::put_u8_count(body, c.vertices.len())?;
                for v in &c.vertices {
                    crate::transducer::address::put_addr(body, v)?;
                }
                c.coefficient.to_bytes(body);
                c.carriage.to_bytes(body);
                for x in &c.volume {
                    body.extend_from_slice(&x.to_le_bytes());
                }
                // A CLASS IS WRITTEN AS ITS COORDINATES. `class_key` already
                // writes 12.6's axes out in order — conjugacy invariants,
                // plane slots, hue, carriage — and `from_key` reads them
                // back, so the wire carries the coordinates and no second
                // codec exists to drift from them (law 12).
                match &c.fill {
                    Fill::Open | Fill::Already => body.push(0),
                    Fill::Filled(g, carriage) => {
                        body.push(1);
                        g.to_bytes(body);
                        carriage.to_bytes(body);
                    }
                    Fill::Class(k, r) => {
                        body.push(2);
                        crate::transducer::address::put_addr(body, k.class_key())?;
                        r.to_bytes(body);
                    }
                    Fill::Congruent(g, ell) => {
                        body.push(3);
                        g.to_bytes(body);
                        body.extend_from_slice(&ell.to_le_bytes());
                    }
                    Fill::Unspellable(r) => {
                        body.push(4);
                        r.to_bytes(body);
                    }
                }
                Some(())
            });
        }
        counted(out, &self.remainder, |body, k, _: &()| {
            crate::transducer::address::put_addr(body, k)
        });
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Complex> {
        let take_id = |b: &[u8], pos: &mut usize| crate::transducer::address::take_addr(b, pos);
        // EVERY SECTION HERE IS FRAMED, and the frame is the one in
        // `snapshot::framed` because `to_bytes` writes through `counted`.
        // A count alone was bounded by the bytes remaining, which refuses a
        // count that cannot possibly fit and says nothing about the frame
        // being lost mid-section — the failure that cost two courts their
        // ledgers in DEF1. Each loop below arrives at the byte its section
        // declared or the complex is refused, naming the section.
        let mut c = Complex::new();
        let (ns, end) = crate::snapshot::take_frame(b, pos)?;
        for _ in 0..ns {
            let v = take_id(b, pos)?;
            let addr = take_id(b, pos)?;
            let gamma = Rotor::from_bytes(b, pos)?;
            let carriage = Versor::from_bytes(b, pos)?;
            c.sections.insert(v, Section { addr, gamma, carriage });
        }
        if *pos != end {
            return None;
        }
        let nd = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
        *pos += 8;
        let nd = usize::try_from(nd).ok()?;
        if nd > b.len().checked_sub(*pos)? {
            return None;
        }
        for _ in 0..nd {
            let (n, end) = crate::snapshot::take_frame(b, pos)?;
            for _ in 0..n {
                let nv = *b.get(*pos)? as usize;
                *pos += 1;
                let mut verts = Vec::new();
                for _ in 0..nv {
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
            if *pos != end {
                return None;
            }
        }
        // A tape that ends here carries no remainder cells.
        let (nr, end) = match crate::snapshot::take_frame(b, pos) {
            Some(f) => f,
            None => return Some(c),
        };
        for _ in 0..nr {
            let k = take_id(b, pos)?;
            c.remainder.insert(k, ());
        }
        if *pos != end {
            return None;
        }
        Some(c)
    }

}
