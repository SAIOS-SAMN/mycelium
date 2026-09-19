// 28 — THE TAPE: a contemplation as bytes.
//
//! `THT3` is the tape this codec reads. One that does not parse whole is
//! refused rather than partly restored; a restored contemplation holds the
//! same bubbles, the same incidence and the same horizon as the one written.

use super::*;

impl Contemplation {

    pub const THT3: &'static [u8; 4] = b"THT3";

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(Self::THT3);
        out.extend_from_slice(&(self.bubbles.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.bubbles, |k, b| {
            crate::snapshot::put_key(out, k);
            out.extend_from_slice(&(b.remainder.len() as u32).to_le_bytes());
            for &(r, c, hb, ha) in b.remainder.as_slice() {
                out.extend_from_slice(&r.to_le_bytes());
                out.extend_from_slice(&c.to_le_bytes());
                out.push(hb);
                out.push(ha);
            }
            out.extend_from_slice(&(b.cover.len() as u32).to_le_bytes());
            crate::snapshot::persist(&b.cover, |ck, _| {
                crate::snapshot::put_key(out, ck);
            });
            for x in &b.volume {
                out.extend_from_slice(&x.to_le_bytes());
            }
            out.push(b.dirty as u8);
            out.extend_from_slice(&0u32.to_le_bytes());
            out.extend_from_slice(&(b.horizon.len() as u32).to_le_bytes());
            crate::snapshot::persist(&b.horizon, |hk, (r, order, standing)| {
                crate::snapshot::put_key(out, hk);
                r.to_bytes(out);
                out.extend_from_slice(&order.to_u32().to_le_bytes());
                out.push(standing.as_u8());
            });
            for x in &b.potential {
                out.extend_from_slice(&x.to_le_bytes());
            }
            out.extend_from_slice(b"TOR1");
            out.extend_from_slice(&(b.torsion.len() as u32).to_le_bytes());
            crate::snapshot::persist(&b.torsion, |tk, filed| {
                let (sigma, ell, chord) = (&filed.sigma, &filed.period, &filed.chord);
                crate::snapshot::put_key(out, tk);
                out.extend_from_slice(&(sigma.len() as u16).to_le_bytes());
                for &(a, c) in sigma {
                    out.push(a);
                    out.push(c);
                }
                out.extend_from_slice(&ell.to_le_bytes());
                out.extend_from_slice(&chord.to_le_bytes());
            });
            out.extend_from_slice(&(b.certificates.len() as u32).to_le_bytes());
            crate::snapshot::persist(&b.certificates, |_, c| {
                c.to_bytes(out);
            });
            out.extend_from_slice(&(b.vortex.len() as u32).to_le_bytes());
            crate::snapshot::persist(&b.vortex, |after, inner| {
                crate::snapshot::put_key(out, after);
                out.extend_from_slice(&(inner.len() as u32).to_le_bytes());
                crate::snapshot::persist(inner, |_, c| {
                    c.to_bytes(out);
                });
            });
            if let Some(f) = &b.form {
                let mut fb = Vec::new();
                if sentence::encode_form_v2(f, &mut fb).is_some() {
                    out.extend_from_slice(b"FRM1");
                    out.extend_from_slice(&(fb.len() as u32).to_le_bytes());
                    out.extend_from_slice(&fb);
                }
            }
        });
        self.complex.to_bytes(out);
        out.extend_from_slice(&self.fills.to_le_bytes());
        out.extend_from_slice(&self.torsion_merges.to_le_bytes());
        out.extend_from_slice(&self.classes.to_le_bytes());
        out.extend_from_slice(&self.formed.to_le_bytes());
        out.extend_from_slice(&(self.cavity.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.cavity, |k, rem| {
            crate::snapshot::put_key(out, k);
            out.extend_from_slice(&(rem.len() as u32).to_le_bytes());
            for &(r, c, hb, ha) in rem {
                out.extend_from_slice(&r.to_le_bytes());
                out.extend_from_slice(&c.to_le_bytes());
                out.push(hb);
                out.push(ha);
            }
        });
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Contemplation> {
        if b.get(*pos..*pos + 4) != Some(&Self::THT3[..]) {
            return None;
        }
        *pos += 4;
        let take_u32 = |b: &[u8], pos: &mut usize| -> Option<u32> {
            let v = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
            *pos += 4;
            Some(v)
        };
        let take_count = |b: &[u8], pos: usize, n: usize, item: usize| -> Option<usize> {
            if item == 0 {
                return Some(n);
            }
            if n > b.len().checked_sub(pos)? / item {
                return None;
            }
            Some(n)
        };
        let n = take_count(b, *pos, take_u32(b, pos)? as usize, 1)?;
        let mut c = Contemplation::new();
        for _ in 0..n {
            let key = crate::transducer::address::Address::from(crate::snapshot::take_key(b, pos)?);
            let nr = take_count(b, *pos, take_u32(b, pos)? as usize, 18)?;
            let mut remainder = Vec::with_capacity(nr);
            for _ in 0..nr {
                let r = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
                let col = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
                let hb = *b.get(*pos)?;
                *pos += 1;
                let ha = *b.get(*pos)?;
                *pos += 1;
                remainder.push((r, col, hb, ha));
            }
            let nc = take_count(b, *pos, take_u32(b, pos)? as usize, 1)?;
            let mut cover = Keyed::new();
            for _ in 0..nc {
                let ck = crate::transducer::address::Address::from(crate::snapshot::take_key(b, pos)?);
                cover.insert(ck, ());
            }
            let mut volume = [0u64; 4];
            for x in &mut volume {
                *x = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
            }
            let dirty = *b.get(*pos)? != 0;
            *pos += 1;
            let elen = take_u32(b, pos)? as usize;
            let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
            *pos = end;
            let mut horizon: Keyed<(Rotor, Order, Standing)> = Keyed::new();
            if *pos + 4 <= b.len() {
                let nh = take_count(b, *pos, take_u32(b, pos)? as usize, 1)?;
                for _ in 0..nh {
                    let hk = crate::transducer::address::Address::from(crate::snapshot::take_key(b, pos)?);
                    let r = Rotor::from_bytes(b, pos)?;
                    let ell = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
                    *pos += 4;
                    let st = Standing::from_u8(*b.get(*pos)?)?;
                    *pos += 1;
                    horizon.insert(hk, (r, Order::from_u32(ell), st));
                }
            }
            let mut potential = volume;
            if *pos + 32 <= b.len() {
                for x in &mut potential {
                    *x = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                }
            }
            let mut torsion = Keyed::new();
            let mut certificates = Keyed::new();
            let mut vortex = Keyed::new();
            if b.get(*pos..*pos + 4) == Some(b"TOR1") {
                *pos += 4;
                // A count that does not fit remaining bytes is refused, never
                // truncated: a σ longer than the stream would desync the tape.
                let nt = take_count(b, *pos, take_u32(b, pos)? as usize, 40)?;
                for _ in 0..nt {
                    let tk = crate::transducer::address::Address::from(crate::snapshot::take_key(b, pos)?);
                    let ns = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?) as usize;
                    *pos += 2;
                    let ns = take_count(b, *pos, ns, 2)?;
                    let mut sigma = Vec::with_capacity(ns);
                    for _ in 0..ns {
                        let a = *b.get(*pos)?;
                        *pos += 1;
                        let cpair = *b.get(*pos)?;
                        *pos += 1;
                        sigma.push((a, cpair));
                    }
                    let ell = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
                    // A17: a loop is filed at the period it CLOSES at, so a
                    // filed zero is a malformed tape and not a loop of no
                    // extent.
                    if ell == 0 {
                        return None;
                    }
                    *pos += 4;
                    let chord = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
                    *pos += 2;
                    torsion.insert(
                        tk,
                        crate::transducer::sentence::FiledLoop { sigma, period: ell, chord },
                    );
                }
                let nc = take_count(b, *pos, take_u32(b, pos)? as usize, 1)?;
                for _ in 0..nc {
                    let c = sentence::PivotCertificate::from_bytes(b, pos)?;
                    certificates.insert(c.key(), c);
                }
                let nv = take_count(b, *pos, take_u32(b, pos)? as usize, 1)?;
                for _ in 0..nv {
                    let after = crate::transducer::address::Address::from(crate::snapshot::take_key(b, pos)?);
                    let nc = take_count(b, *pos, take_u32(b, pos)? as usize, 1)?;
                    let mut inner = Keyed::new();
                    for _ in 0..nc {
                        let c = sentence::PivotCertificate::from_bytes(b, pos)?;
                        let wk = c.key();
                        inner.insert(wk, c);
                    }
                    vortex.insert(after, inner);
                }
            }
            let form = if b.get(*pos..*pos + 4) == Some(b"FRM1") {
                *pos += 4;
                let flen = take_u32(b, pos)? as usize;
                let end = pos.checked_add(flen).filter(|e| *e <= b.len())?;
                let mut fp = 0usize;
                let f = sentence::decode_form_v2(&b[*pos..end], &mut fp);
                *pos += flen;
                f
            } else {
                None
            };
            // A tape whose remainder puts two transitions on one site does
            // not parse; it is refused, never restored as a bubble that
            // cannot be named.
            let remainder = crate::transducer::element::place_chain(&remainder)?;
            let bubble = Bubble {
                remainder,
                nerve: Complex::new(),
                cover,
                horizon,
                potential,
                volume,
                dirty,
                moved_k: 0,
                glues: Keyed::new(),
                glued_by: Keyed::new(),
                torsion,
                certificates,
                vortex,
                // Not persisted: re-derived from the demand's own bodies
                // at hold time (law 12).
                candidates: Keyed::new(),
                // Re-derived from the form the tape decoded, never carried
                // on it (law 12).
                // Z4d: the escrow is what THIS voice paid this session, not
                // a fact of the tape; a restored thought is re-paid on the
                // next stroke that holds it (law 12).
                escrow: volume,
                until: None,
                rotor: form.as_ref().and_then(Bubble::rotor_of_form),
                form,
                bundle: None,
                remainders: Keyed::new(),
            };
            c.admit_incidence(&key, &bubble);
            c.bubbles.insert(&key, bubble);
        }
        c.complex = Complex::from_bytes(b, pos)?;
        if *pos + 24 <= b.len() {
            c.fills = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
            if *pos + 24 <= b.len() {
                c.torsion_merges = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
            }
            c.classes = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
            c.formed = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
        if *pos + 4 <= b.len() {
            let nc = take_count(b, *pos, take_u32(b, pos)? as usize, 1)?;
            for _ in 0..nc {
                let k = crate::transducer::address::Address::from(crate::snapshot::take_key(b, pos)?);
                let nr = take_count(b, *pos, take_u32(b, pos)? as usize, 18)?;
                let mut rem = Vec::with_capacity(nr);
                for _ in 0..nr {
                    let r = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let col = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let hb = *b.get(*pos)?;
                    *pos += 1;
                    let ha = *b.get(*pos)?;
                    *pos += 1;
                    rem.push((r, col, hb, ha));
                }
                c.cavity.insert(k, rem);
            }
        }
        Some(c)
    }

}
