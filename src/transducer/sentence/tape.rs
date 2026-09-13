// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 19 — THE FORM TAPE: `SENTENCE_TAG` version 7 in both directions, with
//! addresses as len16 + bytes, the torsion a sentence tape files, the
//! demand vertices (50.5) and the edge transport on every face (50.3).
//!
//! v7 (50.2 step 2, ruled 2026-09-10) carries a word vertex's
//! DERIVATION where v6 carried a spelling: the generators it factored
//! through, cited by (form, vertex) with exponents in application order,
//! the residue in letters, and the composed product. v6 is refused on
//! read, as v5 is: the version is one equality and there is no reader
//! for an older tape (MU-STANDARDS §10).

use super::codec::{
    derivation_from_form_bytes, derivation_to_form_bytes, put_inv, take_cell, take_count, take_inv,
    take_u16, take_u32, word_from_form_bytes, word_to_form_bytes,
};
use super::form::*;
use super::transport::*;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::Rotor;
use crate::transducer::element::{self, Element};

pub(crate) fn encode_form_v2(f: &Form, out: &mut Vec<u8>) -> Option<()> {
    out.extend_from_slice(&(f.vertices.len() as u16).to_le_bytes());
    for v in &f.vertices {
        match v {
            Vertex::Frame => out.push(0),
            Vertex::Word { address, derivation, acts, contact } => {
                out.push(1);
                crate::transducer::address::put_addr(out, address);
                derivation_to_form_bytes(derivation, *contact, out)?;
                // THE FIBRE (v8, D7): the member count, then each member's
                // carriage, gauge and chart, in the trie's order — which is
                // the chart keys sorted, and a chart key is content, so two
                // encodings of one fibre are one tape.
                out.extend_from_slice(&(acts.len() as u16).to_le_bytes());
                for (_, a) in acts.scoped() {
                    a.carriage.to_bytes(out);
                    out.extend_from_slice(&(a.gauge.len() as u16).to_le_bytes());
                    for &(x, y) in &a.gauge {
                        out.push(x);
                        out.push(y);
                    }
                    put_inv(&a.chart, out)?;
                }
            }
            Vertex::Glued {
                address,
                chart,
                magnitude,
                depth,
                placement,
            } => {
                out.push(2);
                crate::transducer::address::put_addr(out, address);
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
            Vertex::Demand { kind, chart, bodies } => {
                out.push(5);
                kind.to_bytes(out);
                put_inv(chart, out)?;
                out.extend_from_slice(&(u32::try_from(bodies.len()).ok()?).to_le_bytes());
                for &(r, c, before, after) in bodies {
                    out.extend_from_slice(&r.to_le_bytes());
                    out.extend_from_slice(&c.to_le_bytes());
                    out.push(before);
                    out.push(after);
                }
            }
        }
    }
    out.extend_from_slice(&(f.faces.len() as u16).to_le_bytes());
    for face in &f.faces {
        out.extend_from_slice(&face.i.to_le_bytes());
        out.extend_from_slice(&face.j.to_le_bytes());
        put_transport(&face.transport, out);
        match &face.edge {
            None => out.push(0),
            Some(t) => {
                out.push(1);
                put_transport(t, out);
            }
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
    // The remainder rides the tape AS IT SITS IN THE SCENE, so the decoder
    // re-derives both A5's chain and the translator from one reading.
    let placed = f.remainder.placed();
    out.extend_from_slice(&(placed.len() as u32).to_le_bytes());
    for &(r, c, before, after) in &placed {
        out.extend_from_slice(&r.to_le_bytes());
        out.extend_from_slice(&c.to_le_bytes());
        out.push(before);
        out.push(after);
    }
    out.extend_from_slice(&f.depth.to_le_bytes());
    out.extend_from_slice(&(f.certificates.len() as u16).to_le_bytes());
    // THE FORM'S OWN TAPE, and the tape is the object (law 2): every one of
    // this form's four keyed carriers goes out whole, in key order, behind
    // the count that was just written from the same carrier's length.
    for (_, cert) in f.certificates.scoped() {
        cert.to_bytes(out);
    }
    let ((hn, hd), (wn, wd)) = f.ratio;
    out.extend_from_slice(&(hn as u16).to_le_bytes());
    out.extend_from_slice(&(hd as u16).to_le_bytes());
    out.extend_from_slice(&(wn as u16).to_le_bytes());
    out.extend_from_slice(&(wd as u16).to_le_bytes());
    let sec: Vec<(Vec<u8>, Vec<u16>)> = f
        .sections
        .scoped()
        .map(|(k, v)| (k, v.clone()))
        .collect();
    out.extend_from_slice(&(sec.len() as u16).to_le_bytes());
    for (k, ix) in sec {
        crate::transducer::address::put_addr(out, &k);
        out.extend_from_slice(&(ix.len() as u16).to_le_bytes());
        for i in ix {
            out.extend_from_slice(&i.to_le_bytes());
        }
    }
    out.extend_from_slice(&(f.torsion.len() as u16).to_le_bytes());
    for (k, FiledLoop { sigma, period: ell, chord }) in f.torsion.scoped() {
        crate::transducer::address::put_addr(out, &k);
        out.extend_from_slice(&(sigma.len() as u16).to_le_bytes());
        for &(a, b) in sigma {
            out.push(a);
            out.push(b);
        }
        out.extend_from_slice(&ell.to_le_bytes());
        out.extend_from_slice(&chord.to_le_bytes());
    }
    out.extend_from_slice(&(f.axes.len() as u16).to_le_bytes());
    for (k, chord) in f.axes.scoped() {
        crate::transducer::address::put_addr(out, &k);
        out.extend_from_slice(&chord.to_le_bytes());
    }
    Some(())
}

fn put_transport(t: &Transport, out: &mut Vec<u8>) {
    word_to_form_bytes(&t.word, None, out);
    t.carriage.to_bytes(out);
    out.extend_from_slice(&(t.gauge.len() as u16).to_le_bytes());
    for &(a, b) in &t.gauge {
        out.push(a);
        out.push(b);
    }
}

fn take_transport(b: &[u8], pos: &mut usize) -> Option<Transport> {
    let (word, _) = word_from_form_bytes(b, pos)?;
    let carriage = element::Versor::from_bytes(b, pos)?;
    let ng = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
    let mut gauge = Vec::with_capacity(ng);
    for _ in 0..ng {
        gauge.push((*b.get(*pos)?, *b.get(*pos + 1)?));
        *pos += 2;
    }
    Some(Transport { word, carriage, gauge })
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
                let address = crate::transducer::address::take_addr(b, pos)?;
                let (derivation, contact) = derivation_from_form_bytes(b, pos)?;
                // THE FIBRE, one member per chart (v8, D7): a count, then
                // each member's carriage, gauge and chart. v7 wrote one
                // carriage, one gauge and one class with no count, which is
                // this shape at exactly one member — and §10 deletes the
                // reader for a retired layout rather than keeping both, so
                // there is no v7 arm here.
                let nm = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
                let mut acts: crate::network::nerve::Keyed<crate::transducer::sentence::Acting> =
                    crate::network::nerve::Keyed::new();
                for _ in 0..nm {
                    let carriage = element::Versor::from_bytes(b, pos)?;
                    let ng = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
                    let mut gauge = Vec::with_capacity(ng);
                    for _ in 0..ng {
                        gauge.push((*b.get(*pos)?, *b.get(*pos + 1)?));
                        *pos += 2;
                    }
                    let chart = take_inv(b, pos)?;
                    let member = crate::transducer::sentence::Acting {
                        chart,
                        carriage: Box::new(carriage),
                        gauge,
                    };
                    acts.insert(member.key(), member);
                }
                Vertex::Word { address, derivation, acts, contact }
            }
            2 => {
                let address = crate::transducer::address::take_addr(b, pos)?;
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
            5 => {
                let kind = crate::transducer::sentence::Demand::from_bytes(b, pos)?;
                let chart = take_inv(b, pos)?;
                let nb = take_count(b, *pos, take_u32(b, pos)? as usize, 18)?;
                let mut bodies = Vec::with_capacity(nb);
                for _ in 0..nb {
                    bodies.push(take_cell(b, pos)?);
                }
                Vertex::Demand { kind, chart, bodies }
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
        let transport = take_transport(b, pos)?;
        let edge = match *b.get(*pos)? {
            0 => {
                *pos += 1;
                None
            }
            1 => {
                *pos += 1;
                Some(take_transport(b, pos)?)
            }
            _ => return None,
        };
        faces.push(FormFace { i, j, transport, edge });
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
    let mut cells = Vec::with_capacity(nr);
    for _ in 0..nr {
        cells.push(take_cell(b, pos)?);
    }
    // A tape whose remainder puts two transitions on one site does not
    // parse (A5 step 3). The refusal is the decoder's, and the court reads
    // it as `REFUSE_FORM_TAPE`.
    let remainder = crate::transducer::element::place_chain(&cells)?;
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
        let k = crate::transducer::address::take_addr(b, pos)?;
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
        let k = crate::transducer::address::take_addr(b, pos)?;
        let nsig = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
        let mut sigma = Vec::with_capacity(nsig);
        for _ in 0..nsig {
            sigma.push((*b.get(*pos)?, *b.get(*pos + 1)?));
            *pos += 2;
        }
        let ell = take_u32(b, pos)?;
        // A17: a loop is filed at the period it CLOSES at, so a filed zero
        // is a malformed tape and not a loop of no extent.
        if ell == 0 {
            return None;
        }
        let chord = take_u16(b, pos)?;
        torsion.insert(k, FiledLoop { sigma, period: ell, chord });
    }
    let na = take_count(b, *pos, take_u16(b, pos)? as usize, 4)?;
    let mut axes = Keyed::new();
    for _ in 0..na {
        let k = crate::transducer::address::take_addr(b, pos)?;
        let chord = take_u16(b, pos)?;
        axes.insert(k, chord);
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
        axes,
        glued,
        vortex: Keyed::new(),
        by_address: Keyed::new(),
        by_class: Keyed::new(),
        star: Keyed::new(),
        carriages: Keyed::new(),
        by_rebased: Keyed::new(),
        by_unfolded: Keyed::new(),
        demands: Keyed::new(),
    };
    form.reindex();
    Some(form)
}

/// The tape layout this tree writes and reads (chapter 19). A layout change
/// is a recut: no reader for an older version exists (MU-STANDARDS §10).
/// Version 6 (chapter 50, L1) adds the demand vertex and the edge transport.
///
/// Version 8 (D7) writes a word vertex's FIBRE — a member count, then per
/// member the carriage, the gauge and the chart — where v7 wrote one
/// carriage, one gauge and one class with no count. A vertex acting on one
/// chart is that shape at exactly one member.
///
/// THIS BUMP OWES A RECUT AND DOES NOT SCHEDULE ONE. The fleet is down and
/// Stage 16's instrument runs in process with no ledger, no wire and no
/// home, so nothing here needs the fleet up. The owed list already carries
/// S13.6's `ActKind`, S13.9's fold root, L15's lane vector on the receipt
/// and L11c's cell coordinate; one recut pays all of them together
/// whenever the operator next brings the fleet up.
pub const FORM_TAPE_VERSION: u8 = 8;

/// SENTENCE_TAG v6: the Form tape. Hue maps are derived from vertices.
/// Addresses are len16 + bytes. `mag` is the descent the form measures over
/// its bundle. Zero is not a word.
pub fn as_element_form(f: &Form, mag: [u64; 4]) -> Option<Element> {
    if f.vertices.is_empty() || mag.iter().all(|&x| x == 0) {
        return None;
    }
    let mut e = Element::identity();
    let mut word = vec![element::SENTENCE_TAG, FORM_TAPE_VERSION];
    encode_form_v2(f, &mut word)?;
    e.set_description(word);
    e.set_magnitude(mag);
    Some(e)
}

/// Read a Form from a v6 tape. Every other version is refused.
///
/// THE DECODER, AND IT IS NAMED WHEREVER IT RUNS (L16). It was `pub` and
/// called at 81 sites; then it was `pub(crate)` behind
/// `Element::form`'s `OnceLock`, which parsed the tape once per COPY of
/// the element rather than once per word, and had to be taken by hand at
/// every mutation site.
///
/// Now the ORGANS hold `element::Held`, which decodes here exactly once —
/// where the organ takes the word in — and re-decodes through
/// `Held::update` when the word changes. A caller that has an element and
/// no held word calls this by name, so the parse is visible at its own
/// line and a reader can see that it is one.
pub(crate) fn from_element_form(e: &Element) -> Option<Form> {
    if !e.is_sentence() {
        return None;
    }
    let b = e.description();
    if b.get(1).copied() != Some(FORM_TAPE_VERSION) {
        return None;
    }
    let mut pos = 2usize;
    decode_form_v2(b, &mut pos)
}

/// Filed torsion on a sentence tape: `Form.torsion`, keyed by class.
/// Empty when the word is not a sentence, or when the tape is v1 and the
/// cycles live only on the element's residual section.
pub fn form_torsion(e: &Element) -> Keyed<FiledLoop> {
    match from_element_form(e) {
        Some(f) => f.torsion,
        None => Keyed::new(),
    }
}

/// Whether a spoken word carries a torsion cycle: the form first, then
/// the residual element's section so a v1 tape that adopted torsion still
/// reads as a bundle.
pub fn carries_torsion(e: &Element) -> bool {
    if let Some(f) = from_element_form(e) {
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

/// A sentence Element: tagged tape, not an opened-axis connection.
pub fn is_sentence(e: &Element) -> bool {
    e.is_sentence()

}
