// AI-GENERATED CONTENT — see DISCLAIMER.md.

//! Form tape: v4 encode and decode. Addresses are len16 + bytes.

use super::chain::Cell;
use super::form::*;
use super::transport::*;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{self, Rotor, Word};
use crate::transducer::element::{self, Element};
use crate::transducer::stalk;

fn word_to_form_bytes(w: &Word, contact: Option<alphabet::Contact>, out: &mut Vec<u8>) {
    w.to_bytes(out);
    match contact {
        None => out.push(0),
        Some(c) => {
            out.push(1);
            out.extend_from_slice(&c.anchor.to_le_bytes());
            out.push(c.ring);
        }
    }
}

fn word_from_form_bytes(b: &[u8], pos: &mut usize) -> Option<(Word, Option<alphabet::Contact>)> {
    let w = Word::from_bytes(b, pos)?;
    let tag = *b.get(*pos)?;
    *pos += 1;
    if tag == 0 {
        return Some((w, None));
    }
    if tag != 1 {
        return None;
    }
    let anchor = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
    *pos += 2;
    let ring = *b.get(*pos)?;
    *pos += 1;
    Some((w, Some(alphabet::Contact { anchor, ring })))
}

fn bytes_left(b: &[u8], pos: usize) -> usize {
    b.len().saturating_sub(pos)
}

fn take_count(b: &[u8], pos: usize, n: usize, item: usize) -> Option<usize> {
    if item == 0 {
        return Some(n);
    }
    if n > bytes_left(b, pos) / item {
        return None;
    }
    Some(n)
}

fn put_inv(inv: &stalk::Invariant, out: &mut Vec<u8>) -> Option<()> {
    let n = u16::try_from(inv.0.len()).ok()?;
    out.extend_from_slice(&n.to_le_bytes());
    for a in &inv.0 {
        out.extend_from_slice(&a.to_le_bytes());
    }
    Some(())
}

fn take_inv(b: &[u8], pos: &mut usize) -> Option<stalk::Invariant> {
    let n = take_u16(b, pos)? as usize;
    let n = take_count(b, *pos, n, 8)?;
    let mut axes = Vec::with_capacity(n);
    for _ in 0..n {
        let v = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
        *pos += 8;
        axes.push(v);
    }
    Some(stalk::Invariant(axes))
}

pub(crate) fn encode_form_v2(f: &Form, out: &mut Vec<u8>) -> Option<()> {
    out.extend_from_slice(&(f.vertices.len() as u16).to_le_bytes());
    for v in &f.vertices {
        match v {
            Vertex::Frame => out.push(0),
            Vertex::Word {
                address,
                word,
                carriage,
                gauge,
                class,
                contact,
            } => {
                out.push(1);
                crate::transducer::a21::put_addr(out, address);
                word_to_form_bytes(word, *contact, out);
                carriage.to_bytes(out);
                out.extend_from_slice(&(gauge.len() as u16).to_le_bytes());
                for &(a, b) in gauge {
                    out.push(a);
                    out.push(b);
                }
                put_inv(class, out)?;
            }
            Vertex::Glued {
                address,
                chart,
                magnitude,
                depth,
                placement,
            } => {
                out.push(2);
                crate::transducer::a21::put_addr(out, address);
                put_inv(chart, out)?;
                for m in magnitude {
                    out.extend_from_slice(&m.to_le_bytes());
                }
                out.extend_from_slice(&depth.to_le_bytes());
                word_to_form_bytes(placement, None, out);
            }
            Vertex::Rebased {
                by,
                of,
                carriage,
                gauge,
                class,
            } => {
                out.push(3);
                out.extend_from_slice(&by.to_le_bytes());
                out.extend_from_slice(&of.to_le_bytes());
                carriage.to_bytes(out);
                out.extend_from_slice(&(gauge.len() as u16).to_le_bytes());
                for &(a, b) in gauge {
                    out.push(a);
                    out.push(b);
                }
                put_inv(class, out)?;
            }
            Vertex::Unfolded { measure } => {
                out.push(4);
                match measure {
                    Measure::Occupied => out.push(0),
                    Measure::Remainder => out.push(1),
                    Measure::Ring { anchor } => {
                        out.push(2);
                        out.extend_from_slice(&anchor.to_le_bytes());
                    }
                }
            }
        }
    }
    out.extend_from_slice(&(f.faces.len() as u16).to_le_bytes());
    for face in &f.faces {
        out.extend_from_slice(&face.i.to_le_bytes());
        out.extend_from_slice(&face.j.to_le_bytes());
        word_to_form_bytes(&face.transport.word, None, out);
        face.transport.carriage.to_bytes(out);
        out.extend_from_slice(&(face.transport.gauge.len() as u16).to_le_bytes());
        for &(a, b) in &face.transport.gauge {
            out.push(a);
            out.push(b);
        }
    }
    out.extend_from_slice(&(f.loops.len() as u16).to_le_bytes());
    for lp in &f.loops {
        out.extend_from_slice(&(lp.vertices.len() as u16).to_le_bytes());
        for &i in &lp.vertices {
            out.extend_from_slice(&i.to_le_bytes());
        }
        out.extend_from_slice(&(lp.faces.len() as u16).to_le_bytes());
        for &i in &lp.faces {
            out.extend_from_slice(&i.to_le_bytes());
        }
        out.extend_from_slice(&lp.chord.to_le_bytes());
        word_to_form_bytes(&lp.holonomy, None, out);
        lp.carriage.to_bytes(out);
    }
    out.extend_from_slice(&(f.remainder.len() as u32).to_le_bytes());
    for &(r, c, before, after) in &f.remainder {
        out.extend_from_slice(&r.to_le_bytes());
        out.extend_from_slice(&c.to_le_bytes());
        out.push(before);
        out.push(after);
    }
    out.extend_from_slice(&f.depth.to_le_bytes());
    out.extend_from_slice(&(f.certificates.len() as u16).to_le_bytes());
    for (_, cert) in f.certificates.fiber(crate::network::nerve::ROOT) {
        cert.to_bytes(out);
    }
    let ((hn, hd), (wn, wd)) = f.ratio;
    out.extend_from_slice(&(hn as u16).to_le_bytes());
    out.extend_from_slice(&(hd as u16).to_le_bytes());
    out.extend_from_slice(&(wn as u16).to_le_bytes());
    out.extend_from_slice(&(wd as u16).to_le_bytes());
    let sec: Vec<(Vec<u8>, Vec<u16>)> = f
        .sections
        .fiber(crate::network::nerve::ROOT)
        .map(|(k, v)| (k, v.clone()))
        .collect();
    out.extend_from_slice(&(sec.len() as u16).to_le_bytes());
    for (k, ix) in sec {
        crate::transducer::a21::put_addr(out, &k);
        out.extend_from_slice(&(ix.len() as u16).to_le_bytes());
        for i in ix {
            out.extend_from_slice(&i.to_le_bytes());
        }
    }
    out.extend_from_slice(&(f.torsion.len() as u16).to_le_bytes());
    for (k, FiledLoop { sigma, period: ell, chord }) in f.torsion.fiber(crate::network::nerve::ROOT) {
        crate::transducer::a21::put_addr(out, &k);
        out.extend_from_slice(&(sigma.len() as u16).to_le_bytes());
        for &(a, b) in sigma {
            out.push(a);
            out.push(b);
        }
        out.extend_from_slice(&ell.to_le_bytes());
        out.extend_from_slice(&chord.to_le_bytes());
    }
    Some(())
}

fn take_u16(b: &[u8], pos: &mut usize) -> Option<u16> {
    let v = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
    *pos += 2;
    Some(v)
}

pub(crate) fn decode_form_v2(b: &[u8], pos: &mut usize) -> Option<Form> {
    let nv = take_count(b, *pos, take_u16(b, pos)? as usize, 1)?;
    if nv == 0 {
        return None;
    }
    let mut vertices = Vec::with_capacity(nv);
    for _ in 0..nv {
        let kind = *b.get(*pos)?;
        *pos += 1;
        let v = match kind {
            0 => Vertex::Frame,
            1 => {
                let address = crate::transducer::a21::take_addr(b, pos)?;
                let (word, contact) = word_from_form_bytes(b, pos)?;
                let carriage = element::Versor::from_bytes(b, pos)?;
                let ng = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
                let mut gauge = Vec::with_capacity(ng);
                for _ in 0..ng {
                    gauge.push((*b.get(*pos)?, *b.get(*pos + 1)?));
                    *pos += 2;
                }
                let class = take_inv(b, pos)?;
                Vertex::Word {
                    address,
                    word,
                    carriage: Box::new(carriage),
                    gauge,
                    class,
                    contact,
                }
            }
            2 => {
                let address = crate::transducer::a21::take_addr(b, pos)?;
                let chart = take_inv(b, pos)?;
                let mut magnitude = [0u64; 4];
                for m in &mut magnitude {
                    *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                }
                let depth = take_u16(b, pos)?;
                let (placement, _) = word_from_form_bytes(b, pos)?;
                Vertex::Glued {
                    address,
                    chart,
                    magnitude,
                    depth,
                    placement,
                }
            }
            3 => {
                let by = take_u16(b, pos)?;
                let of = take_u16(b, pos)?;
                let carriage = element::Versor::from_bytes(b, pos)?;
                let ng = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
                let mut gauge = Vec::with_capacity(ng);
                for _ in 0..ng {
                    gauge.push((*b.get(*pos)?, *b.get(*pos + 1)?));
                    *pos += 2;
                }
                let class = take_inv(b, pos)?;
                Vertex::Rebased {
                    by,
                    of,
                    carriage: Box::new(carriage),
                    gauge,
                    class,
                }
            }
            4 => {
                let m = *b.get(*pos)?;
                *pos += 1;
                let measure = match m {
                    0 => Measure::Occupied,
                    1 => Measure::Remainder,
                    2 => {
                        let anchor = take_u16(b, pos)?;
                        Measure::Ring { anchor }
                    }
                    _ => return None,
                };
                Vertex::Unfolded { measure }
            }
            _ => return None,
        };
        vertices.push(v);
    }
    let nf = take_count(b, *pos, take_u16(b, pos)? as usize, 4)?;
    let mut faces = Vec::with_capacity(nf);
    for _ in 0..nf {
        let i = take_u16(b, pos)?;
        let j = take_u16(b, pos)?;
        let (word, _) = word_from_form_bytes(b, pos)?;
        let carriage = element::Versor::from_bytes(b, pos)?;
        let ng = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
        let mut gauge = Vec::with_capacity(ng);
        for _ in 0..ng {
            gauge.push((*b.get(*pos)?, *b.get(*pos + 1)?));
            *pos += 2;
        }
        faces.push(FormFace {
            i,
            j,
            transport: Transport {
                word,
                carriage,
                gauge,
            },
        });
    }
    let nl = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
    let mut loops = Vec::with_capacity(nl);
    for _ in 0..nl {
        let nv = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
        if nv == 0 {
            return None;
        }
        let mut vs = Vec::with_capacity(nv);
        for _ in 0..nv {
            vs.push(take_u16(b, pos)?);
        }
        let n = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
        let mut fs = Vec::with_capacity(n);
        for _ in 0..n {
            fs.push(take_u16(b, pos)?);
        }
        let chord = take_u16(b, pos)?;
        let (holonomy, _) = word_from_form_bytes(b, pos)?;
        let carriage = element::Versor::from_bytes(b, pos)?;
        loops.push(Loop {
            vertices: vs,
            faces: fs,
            chord,
            holonomy,
            carriage,
        });
    }
    let nr = take_count(b, *pos, take_u32(b, pos)? as usize, 18)?;
    let mut remainder = Vec::with_capacity(nr);
    for _ in 0..nr {
        remainder.push(take_cell(b, pos)?);
    }
    let depth = take_u16(b, pos)?;
    let ncert = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
    let mut certificates = Keyed::new();
    for _ in 0..ncert {
        let c = PivotCertificate::from_bytes(b, pos)?;
        certificates.insert(c.key(), c);
    }
    let hn = take_u16(b, pos)?;
    let hd = take_u16(b, pos)?;
    let wn = take_u16(b, pos)?;
    let wd = take_u16(b, pos)?;
    if hd == 0 || wd == 0 {
        return None;
    }
    let ratio = ((hn as usize, hd as usize), (wn as usize, wd as usize));
    let ns = take_u16(b, pos)? as usize;
    let mut sections = Keyed::new();
    for _ in 0..ns {
        let k = crate::transducer::a21::take_addr(b, pos)?;
        let nv = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
        let mut ix = Vec::with_capacity(nv);
        for _ in 0..nv {
            ix.push(take_u16(b, pos)?);
        }
        sections.insert(k, ix);
    }
    let nt = take_u16(b, pos)? as usize;
    let mut torsion = Keyed::new();
    for _ in 0..nt {
        let k = crate::transducer::a21::take_addr(b, pos)?;
        let nsig = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
        let mut sigma = Vec::with_capacity(nsig);
        for _ in 0..nsig {
            sigma.push((*b.get(*pos)?, *b.get(*pos + 1)?));
            *pos += 2;
        }
        let ell = take_u32(b, pos)?;
        let chord = take_u16(b, pos)?;
        torsion.insert(k, FiledLoop { sigma, period: ell, chord });
    }
    let mut glued = Keyed::new();
    for (i, v) in vertices.iter().enumerate() {
        if let Vertex::Glued { address, .. } = v {
            glued.insert(address, i as u16);
        }
    }
    let mut form = Form {
        vertices,
        faces,
        loops,
        remainder,
        depth,
        hue_maps: Keyed::new(),
        ratio,
        sections,
        certificates,
        torsion,
        glued,
        vortex: Keyed::new(),
        by_address: Keyed::new(),
        star: Keyed::new(),
        carriages: Keyed::new(),
        by_rebased: Keyed::new(),
        by_unfolded: Keyed::new(),
    };
    form.reindex();
    Some(form)
}

/// SENTENCE_TAG v4: the Form tape. Hue maps are derived from vertices.
/// Addresses are len16 + bytes. `mag` is the descent the form measures over
/// its bundle. Zero is not a word.
pub fn as_element_form(f: &Form, mag: [u64; 4]) -> Option<Element> {
    if f.vertices.is_empty() || mag.iter().all(|&x| x == 0) {
        return None;
    }
    let mut e = Element::identity();
    let mut word = vec![element::SENTENCE_TAG, 4];
    encode_form_v2(f, &mut word)?;
    e.set_description(word);
    e.set_magnitude(mag);
    Some(e)
}

/// Read a Form from a v4 tape. An older tape is refused.
pub fn from_element_form(e: &Element) -> Option<Form> {
    if !e.is_sentence() {
        return None;
    }
    let b = e.description();
    if b.get(1).copied() != Some(4) {
        return None;
    }
    let mut pos = 2usize;
    decode_form_v2(b, &mut pos)
}

/// Filed torsion on a sentence tape: `Form.torsion`, keyed by class.
/// Empty when the word is not a sentence, or when the tape is v1 and the
/// cycles live only on the element's residual section.
pub fn form_torsion(e: &Element) -> Keyed<FiledLoop> {
    match e.form() {
        Some(f) => f.torsion.clone(),
        None => Keyed::new(),
    }
}

/// Whether a spoken word carries a torsion cycle: the form first, then
/// the residual element's section so a v1 tape that adopted torsion still
/// reads as a bundle.
pub fn carries_torsion(e: &Element) -> bool {
    if let Some(f) = e.form() {
        if !f.torsion.is_empty() {
            return true;
        }
    }
    !e.torsion().is_empty()
}

/// The rotor a filed torsion cycle names: the chord face stored on the
/// class key. None when that face is missing — never an identity rotor.
pub fn torsion_loop_rotor(f: &Form, class: &[u8]) -> Option<Rotor> {
    let chord = f.torsion.get(class)?.chord;
    f.faces.get(chord as usize).map(|face| face.transport.word.rotor())
}

fn take_u32(b: &[u8], pos: &mut usize) -> Option<u32> {
    let v = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
    *pos += 4;
    Some(v)
}

fn take_cell(b: &[u8], pos: &mut usize) -> Option<Cell> {
    let r = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
    *pos += 8;
    let c = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
    *pos += 8;
    let before = *b.get(*pos)?;
    *pos += 1;
    let after = *b.get(*pos)?;
    *pos += 1;
    Some((r, c, before, after))
}

/// A sentence Element: tagged tape, not an opened-axis connection.
pub fn is_sentence(e: &Element) -> bool {
    e.is_sentence()

}
