// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 19 — THE FORM TAPE: `SENTENCE_TAG` at `FORM_TAPE_VERSION` in both
//! directions, with addresses as len16 + bytes, the torsion a sentence
//! tape files, the demand vertices (50.5) and the edge transport on every
//! face (50.3). A word vertex carries its DERIVATION (50.2 step 2): the
//! generators it factored through, cited by (form, vertex) with exponents
//! in application order, the residue in letters, and the composed product.
//! The version is one equality and there is no reader for an older tape
//! (MU-STANDARDS §10).

use super::codec::{
    derivation_from_form_bytes, derivation_to_form_bytes, put_inv, take_count, take_inv, take_link,
    take_u16, take_u32, word_from_form_bytes, word_to_form_bytes,
};
use super::form::*;
use super::transport::*;
use crate::network::nerve::{Coordinate, Keyed};
use crate::transducer::alphabet::Rotor;
use crate::transducer::element::{self, Element};

pub(crate) fn encode_form_v2(f: &Form, out: &mut Vec<u8>) -> Option<()> {
    // 1.5 AND RULE 15 — EVERY ADDRESS IS WRITTEN WHOLE, AND NOTHING NAMES
    // A PLACE.
    //
    // Part 2 put a table of the distinct addresses at the head of the tape
    // and had each vertex name its address by a TAPE-LOCAL ORDINAL. It was
    // argued as transport, and for a message in flight that argument holds
    // — 1.5 permits transport identifiers. It does not survive being
    // carried onto the genome file, where the same shape becomes a number
    // naming a place in a carrier on the cognition path, which is rule 15's
    // defect in a file's costume; and a table is a spreadsheet's noun for a
    // trie's fiber. The directive that asked for it was written in those
    // nouns, and the shape followed the words.
    //
    // Returned here in full, version spent. The within-tape repetition it
    // collapsed — one word gluing one form at twelve vertices, 313 224
    // bytes to 26 104 — is real and is not taken this way: what removes a
    // repeated citation IN KIND is 53.4's quantified citation, one citation
    // per fiber, which is K2's. Bytes repeated across tapes are storage and
    // are not chased; identity is not shortened.
    //
    // 71.2(1), BQ1 — the base's `n` first, as the address writes it.
    out.push(f.n);
    out.extend_from_slice(&(f.vertices.len() as u16).to_le_bytes());
    for v in &f.vertices {
        match v {
            Vertex::Frame => out.push(0),
            Vertex::Word { address, derivation, acts, contact } => {
                out.push(1);
                crate::transducer::address::put_addr(out, address)?;
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
            // 54.7 B5a — THE TAPE STILL NAMES THE TARGET, and it is the
            // FORM's cover that supplies it. The vertex holds the boundary
            // alone, so the writer is handed the edge standing at this
            // vertex's site; a citation whose cover the form does not carry
            // cannot be written, and refusing is the honest answer (rule
            // 11) rather than a zero address that reads back as a citation
            // of nothing.
            v @ Vertex::Glued {
                chart,
                magnitude,
                depth,
                placement,
                carriage,
            } => {
                out.push(2);
                let site = crate::transducer::sentence::vertex_bytes(v);
                let edge = f.cover.iter().find(|e| e.host().as_bytes() == site)?;
                crate::transducer::address::put_addr(out, &edge.target)?;
                put_inv(chart, out)?;
                for m in magnitude {
                    out.extend_from_slice(&m.to_le_bytes());
                }
                out.extend_from_slice(&depth.to_le_bytes());
                word_to_form_bytes(placement, None, out);
                // φ — the transition versor the cited form acts under here.
                carriage.to_bytes(out);
            }
            Vertex::Rebased { by, of, acts } => {
                out.push(3);
                out.extend_from_slice(&by.to_le_bytes());
                out.extend_from_slice(&of.to_le_bytes());
                // THE FIBRE, one member per chart `of` acted on, in trie order
                // — the same shape a Word vertex writes, with the relative in
                // place of the placement (v8, D10).
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
        // THE FACE'S WORD ONCE (v8, D10): it is `w_j ∘ w_i⁻¹` off the two
        // derivations, a vertex coordinate, so one word serves the face.
        word_to_form_bytes(&face.word, None, out);
        // THEN THE MEMBERS, IN CHART-PAIR KEY ORDER — the trie's, which is
        // content — each with the carriage and hue map it carries there and
        // 50.3's edge where the pair has one.
        out.extend_from_slice(&(face.at.len() as u16).to_le_bytes());
        for (pk, place) in face.at.scoped() {
            crate::transducer::address::put_addr(out, &pk);
            place.carriage.to_bytes(out);
            out.extend_from_slice(&(place.gauge.len() as u16).to_le_bytes());
            for &(x, y) in &place.gauge {
                out.push(x);
                out.push(y);
            }
            match &place.edge {
                None => out.push(0),
                Some(t) => {
                    out.push(1);
                    put_transport(t, out);
                }
            }
        }
        // THE SECTION THIS FACE CARRIES (v9): the pairs of the bundle its
        // word was measured on, present or absent by its own tag. A face
        // that crossed the wire without it would arrive claiming every pair
        // of the family, and the court re-derives an edge against every
        // pair it claims.
        match &face.pairs {
            None => out.push(0),
            Some(ps) => {
                out.push(1);
                out.extend_from_slice(&(ps.len() as u16).to_le_bytes());
                // The inner key IS the frame, and its order IS the
                // succession, so the section is written in coordinate order.
                for p in frames_of_section(ps) {
                    out.extend_from_slice(&p.to_le_bytes());
                }
            }
        }
        // THE INTERFACE A CONTACT FACE RUNS ALONG (C2, 70.6), as the address
        // writes it: a count, then each element's doubled centre, a site
        // with its `n`.
        out.extend_from_slice(&(u32::try_from(face.interface.len()).ok()?).to_le_bytes());
        for s in &face.interface {
            s.put(out);
        }
    }
    // THE ORBITS ARE NOT ON THE TAPE (54.5, as ruled 2026-09-20). §1.5 has
    // said since it was written that "loops and holonomies are derived and
    // never keyed", and the tape carried them anyway — a second statement
    // of the incidence the faces above already are (law 12).
    //
    // Measured, it was the WHOLE of what the tape held beyond the key: 2
    // parts in 10 000, `orbits + carriages`, everything else either in the
    // key or empty. So this block was the tape's entire reason to exist,
    // and it was reason for nothing: `orbits_of` re-derives it from the
    // faces deterministically.
    //
    // AND IT COST THE GEOMETRY. `Orbit.vertices` is a list of VERTEX
    // INDICES because that is what the tape could carry cheaply, so a
    // cycle of the member complex had to be projected down to the base to
    // be written — and under that projection a 1-cell between two charts
    // at one vertex, `((v,c_a),(v,c_b))`, becomes the degenerate self-loop
    // `(v,v)` with no orientation. That is the defect `437d253` patched by
    // trying both directions. The frozen layout deformed the mathematics;
    // removing it is what lets the cycle carry its own coordinates.
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
        .map(|(k, v)| (k, frames_of_section(v)))
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
    for (k, FiledOrbit { sigma, periods, chord }) in f.torsion.scoped() {
        crate::transducer::address::put_addr(out, &k);
        out.extend_from_slice(&(sigma.len() as u16).to_le_bytes());
        for &(a, b) in sigma {
            out.push(a);
            out.push(b);
        }
        periods.to_bytes(out);
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
    let base_n = *b.get(*pos)?;
    *pos += 1;
    let nv = take_count(b, *pos, take_u16(b, pos)? as usize, 1)?;
    if nv == 0 {
        return None;
    }
    let mut vertices = Vec::with_capacity(nv);
    // THE COVER, COLLECTED AS THE CITATIONS ARE PARSED (54.7 B5a).
    let mut cover: Vec<crate::transducer::element::NerveEdge> = Vec::new();
    for _ in 0..nv {
        let kind = *b.get(*pos)?;
        *pos += 1;
        let v = match kind {
            0 => Vertex::Frame,
            1 => {
                let address = crate::transducer::address::take_addr(b, pos)?;
                let (derivation, contact) = derivation_from_form_bytes(b, pos)?;
                // THE FIBRE, one member per chart (D7): a count, then each
                // member's carriage, gauge and chart.
                let nm = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
                let mut acts: crate::network::nerve::Keyed<
                    crate::transducer::sentence::Acting,
                    crate::network::nerve::coord::Section,
                > =
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
                    acts.put(&member.key(), member);
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
                let carriage = Box::new(element::Versor::from_bytes(b, pos)?);
                {
                    // 54.7 B5a — THE TARGET GOES TO THE COVER, not onto the
                    // vertex. The tape is the pre-eviction carrier and
                    // names it; where it belongs is the form's 1-skeleton.
                    cover.push(crate::transducer::element::NerveEdge {
                        target: address,
                        chart: chart.clone(),
                        magnitude,
                        depth,
                        placement: placement.clone(),
                        carriage: carriage.clone(),
                    });
                    Vertex::Glued { chart, magnitude, depth, placement, carriage }
                }
            }
            3 => {
                let by = take_u16(b, pos)?;
                let of = take_u16(b, pos)?;
                // The fibre, as a Word vertex's (v8, D10).
                let nm = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
                let mut acts: Keyed<
                    crate::transducer::sentence::Acting,
                    crate::network::nerve::coord::Section,
                > = Keyed::new();
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
                    acts.put(&member.key(), member);
                }
                Vertex::Rebased { by, of, acts }
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
                    bodies.push(take_link(b, pos)?);
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
        // The face's word, then its members in chart-pair key order (v8, D10).
        let (word, _) = word_from_form_bytes(b, pos)?;
        let np = take_count(b, *pos, take_u16(b, pos)? as usize, 3)?;
        let mut at = Keyed::new();
        for _ in 0..np {
            let pk = crate::transducer::address::take_addr(b, pos)?;
            let carriage = element::Versor::from_bytes(b, pos)?;
            let ng = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
            let mut gauge = Vec::with_capacity(ng);
            for _ in 0..ng {
                gauge.push((*b.get(*pos)?, *b.get(*pos + 1)?));
                *pos += 2;
            }
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
            at.insert(pk.as_bytes(), FacePlace { carriage, gauge, edge });
        }
        let pairs = match *b.get(*pos)? {
            0 => {
                *pos += 1;
                None
            }
            1 => {
                *pos += 1;
                let np = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
                let mut ps = Vec::with_capacity(np);
                for _ in 0..np {
                    ps.push(take_u16(b, pos)?);
                }
                Some(section_of_frames(ps))
            }
            _ => return None,
        };
        let ni = take_count(b, *pos, take_u32(b, pos)? as usize, 4)?;
        let mut interface = Vec::with_capacity(ni);
        for _ in 0..ni {
            interface.push(crate::transducer::base::Site::take(b, pos)?);
        }
        faces.push(FormFace { i, j, word, at, pairs, interface });
    }
    // THE ORBITS ARE RE-DERIVED, NOT READ (§1.5, 54.5). Nothing is taken
    // from the tape here because the writer puts nothing there; the cycle
    // basis is a theorem of the faces just parsed, and `orbits_of` proves
    // it the same way for a decoded form as for an extracted one. It runs
    // below, once the vertices are in hand.
    let nr = take_count(b, *pos, take_u32(b, pos)? as usize, 18)?;
    let mut links = Vec::with_capacity(nr);
    for _ in 0..nr {
        links.push(take_link(b, pos)?);
    }
    // A tape whose remainder puts two transitions on one site does not
    // parse (A5 step 3). The refusal is the decoder's, and the court reads
    // it as `REFUSE_FORM_TAPE`.
    let remainder = crate::transducer::element::place_chain(&links)?;
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
    let mut sections: Keyed<Keyed<()>, crate::network::nerve::coord::SectionAt> =
        Keyed::new();
    for _ in 0..ns {
        let k = crate::transducer::address::take_addr(b, pos)?;
        let nv = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
        let mut ix = Vec::with_capacity(nv);
        for _ in 0..nv {
            ix.push(take_u16(b, pos)?);
        }
        // THE TAPE IS WHERE WIRE BYTES BECOME A COORDINATE, and a tape that
        // does not spell one is refused rather than filed under bytes
        // nothing can read back (rule 11).
        let at = crate::network::nerve::coord::SectionAt::decode(&k).ok()?;
        sections.put(&at, section_of_frames(ix));
    }
    let nt = take_u16(b, pos)? as usize;
    let mut torsion: Keyed<FiledOrbit, crate::network::nerve::coord::Holonomy> = Keyed::new();
    for _ in 0..nt {
        let k = crate::transducer::address::take_addr(b, pos)?;
        let nsig = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
        let mut sigma = Vec::with_capacity(nsig);
        for _ in 0..nsig {
            sigma.push((*b.get(*pos)?, *b.get(*pos + 1)?));
            *pos += 2;
        }
        // BOTH PLANE PERIODS as filed: finite at `ℓ`, `0` for the factor
        // that turns forever, A19's spelling of it (52.3). It was ONE `u32`
        // — the `lcm` — until 54.7.
        let periods = crate::transducer::sentence::ToralPeriod::from_pair(take_u32(b, pos)?, take_u32(b, pos)?);
        let chord = take_u16(b, pos)?;
        // THE TAPE IS WHERE WIRE BYTES BECOME A COORDINATE (54.7 R5). The
        // decoder declares the system it read them under; a reader further
        // in gets the coordinate and never the bytes.
        torsion.put(
            &crate::network::nerve::coord::Holonomy::decode(&k).ok()?,
            FiledOrbit { sigma, periods, chord },
        );
    }
    let na = take_count(b, *pos, take_u16(b, pos)? as usize, 4)?;
    let mut axes: Keyed<u16, crate::network::nerve::coord::Holonomy> = Keyed::new();
    for _ in 0..na {
        let k = crate::transducer::address::take_addr(b, pos)?;
        let chord = take_u16(b, pos)?;
        axes.put(&crate::network::nerve::coord::Holonomy::decode(&k).ok()?, chord);
    }
    // `glued` is `reindex`'s to build (law 12), and `reindex` runs below.
    let mut form = Form {
        n: base_n,
        vertices,
        faces,
        // Re-derived below, from the faces just parsed (§1.5, 54.5).
        orbits: Vec::new(),
        remainder,
        depth,
        hue_maps: Keyed::new(),
        ratio,
        sections,
        certificates,
        torsion,
        axes,
        glued: Keyed::new(),
        // THE COVER, OFF THE TAPE'S OWN CITATIONS (54.7, B5a). The tape is
        // the pre-eviction carrier and writes each citation's target, so
        // the edges were collected at the parse site above and nothing is
        // invented for them here — put through the ONE writer of the
        // cover's order, which is the envelope's, so a form decoded from
        // the tape and the same form decoded from its address hold their
        // edges in one order.
        cover: crate::transducer::element::canonical_nerve(cover),
        vortex: Keyed::new(),
        by_address: Keyed::new(),
        by_class: Keyed::new(),
        by_transport: Keyed::new(),
        star: Keyed::new(),
        carriages: Keyed::new(),
        by_rebased: Keyed::new(),
        by_unfolded: Keyed::new(),
        demands: Keyed::new(),
        // A certified cell is perception's reading, never on the tape
        // before the recut (NB1, 78.6 choice (c)): a form read back from its
        // tape holds none.
        cells: Keyed::new(),
    };
    form.reindex();
    // THE CYCLE BASIS IS A THEOREM OF THE INCIDENCE, proved here the same
    // way the extractor proves it, rather than read back from a carrier
    // that would be a second statement of the faces (law 12, §1.5).
    form.orbits = Form::orbits_of(&form.faces, &form.vertices);
    Some(form)
}

/// THE TAPE'S LAST VERSION (54.5). 14, and the byte is never bumped for the
/// form again.
///
/// 13 → 14 REMOVES the orbit block, and is the only bump this file will take
/// that makes the tape SMALLER. §1.5 has always said "loops and holonomies
/// are derived and never keyed"; the tape carried them anyway, and measured
/// they were the whole of `Tape ∖ Key` — 2 parts in 10 000, with every
/// other component either in the key or empty. A v13 tape read by this
/// build would take those bytes for the remainder that follows them, so the
/// byte refuses it, which is the byte's entire job.
///
/// The tape is no longer a living schema. Under 54.5 the genome on disk is
/// the trie of addresses and `from_address` is the only decoder, so the
/// tape survives solely as the wire's frame for an address and its
/// receipts. A format that defines nothing has no versions to take.
///
/// THE BUMPS THAT GOT HERE WERE A MISTAKE AND ARE NAMED. 10 → 11 added a
/// per-tape address table (Z12b part 2); 11 → 12 returned it when the same
/// shape was about to be carried onto the genome file, where a number
/// naming a place in a carrier is rule 15's defect. Both treated the tape
/// as the carrier of cognition, which 54.5 says it is not — and the second
/// bump is what made the previous soak's genomes unreadable to this build,
/// so `--genome` reported an `Element::address` FALLBACK as if it were the
/// form's key.
///
/// 12 → 13 is the last, and it is a correction rather than a change of
/// intent: a citation gained its morphism φ at `01760af` and the tape
/// began carrying it WITHOUT a bump, so the constant named a layout the
/// encoder had already left. A version that does not match its bytes
/// refuses nothing.
///
/// THE MIGRATION BASELINE IS CUT HERE: nothing written before 13 is read
/// by this tree, and §10's rule holds — a layout change is a recut, and
/// this is the last one the form's tape takes.
/// Version 10 is the three-coordinate base (51.7): supports as sites of
/// three coordinates, versors of thirty-two components, the sections
/// placed, and an orbit's period filed as `0` for infinite order (52.3).
///
/// 14 → 15 IS BQ1'S, AND IT BREAKS "NEVER AGAIN" BY NAME. 71.2(1) puts `n`
/// on every site and on the address: the form's `n` opens the tape, every
/// versor carries its own `n` ahead of its coefficients, and an interface
/// element is a site with its count. The bytes changed, and a byte that did
/// not would refuse nothing — which is the defect 12 → 13 names. It lands
/// with the address's own layout change, at the one recut Phase N rides.
pub const FORM_TAPE_VERSION: u8 = 15;

/// 37.7 — MINT A GENERATOR FROM A ROTOR, the one construction that turns a
/// measured action the alphabet cannot spell into a word the language holds.
///
/// `Derivation::adjoined` is the constructor — it cites nothing, spells
/// nothing, and IS the rotor it carries — and the rotor must go in the rotor
/// field rather than the residue, because a `Word` travels as its LETTER
/// TAPE and a letterless word carrying only a cached rotor reaches the wire
/// as the identity.
///
/// NO STAR FACE, and the reason is structural rather than an omission. A
/// face carries a `Transport` whose word is a `Word` — letters — and it is
/// the coboundary of two placements (3.1). The face from the Frame to THIS
/// vertex would be the letter's own action, which is exactly the action the
/// seventeen cannot spell; `face_of` refuses it and is right to. A letter is
/// not a placed body: it is the carrier of an action, with no placement to
/// take a coboundary of. Its form is a Frame and the vertex, no faces, no
/// orbits — which coheres vacuously and is closed, as a generator should be.
///
/// `None` where the rotor has no weight: a rotor that is not a versor is not
/// a generator (15.3), and that is a refusal rather than a generator of
/// zeros (rule 11).
///
/// It lives here beside `as_element_form` rather than at the court's listing
/// because both organs mint: the court from a Gaussian prime the seventeen
/// do not carry, and a voice from a `Fill::Unspellable` its own cover meet
/// produced. One construction, or the two mint different objects.
pub fn generator_form(
    rotor: &crate::transducer::alphabet::Rotor,
    chart: &crate::transducer::stalk::Invariant,
    bodies: &[crate::transducer::sentence::Link],
) -> Option<Element> {
    use crate::transducer::alphabet::Derivation;
    rotor.weight()?;
    let mut f = Form::default();
    f.vertices.push(Vertex::word_on(
        Derivation::adjoined(rotor.clone()),
        chart.clone(),
        crate::transducer::element::identity_versor().into(),
        Vec::new(),
        None,
    ));
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    // What the language could not say without it: the demand's own bodies,
    // measured lane-wise, the same measure a remainder takes. A mint with no
    // bodies measures the lanes of nothing, which `as_element_form` refuses
    // as the zero magnitude — so a caller with no links to name gives the
    // letter the one lane it does occupy, the holonomic.
    let mu = match crate::transducer::element::measure_lanes(bodies) {
        m if m.iter().all(|&x| x == 0) => [0, 0, 0, 1],
        m => m,
    };
    as_element_form(&f, mu)
}

/// SENTENCE_TAG v6: the Form tape. Hue maps are derived from vertices.
/// Addresses are len16 + bytes. `mag` is the descent the form measures over
/// its bundle. Zero is not a word.
pub fn as_element_form(f: &Form, mag: [u64; 4]) -> Option<Element> {
    use crate::transducer::family::trace;
    if f.vertices.is_empty() || mag.iter().all(|&x| x == 0) {
        // THE MINT'S REFUSAL IS A LINE. An empty form or a zero magnitude
        // mints nothing, and under rule 17 a reader that writes no
        // coordinate and returns no demand is a site the timeline must be
        // able to name — this one silently ended `speak_form`'s fast arm.
        trace::trace("mint/form", trace::Wrote::Nothing, None);
        return None;
    }
    let mut e = Element::identity();
    // 54.7 B6 — A SENTENCE MINTS NO TAPE.
    //
    // This wrote `[SENTENCE_TAG][FORM_TAPE_VERSION]` and then the whole
    // form as bytes, and that buffer was the word's identity, its kind and
    // its decoder all at once. All three moved: the address IS the form
    // (law 2) and is set below, the kind is the address's own `TAG_FORM`,
    // and the evidence the key excludes rides `ReceiptsEnvelope`.
    //
    // WORDS, CONNECTIONS AND RESIDUAL CHAINS KEEP THEIRS. For those the
    // description is the content carrier and not a second copy of a
    // coordinate — `letters`, `rewrite` and `residual` read it as the
    // object itself. It is the SENTENCE tape that was redundant, because a
    // sentence's content is a cell complex its key writes out in full.
    let n = 0usize;
    e.set_magnitude(mag);
    // THE ADDRESS IS SET, NOT DERIVED (54.5, as ruled 2026-09-20).
    // `Element::address` derived it by parsing the tape — so a word could
    // not know its own name without one, which is the dependency that kept
    // the tape alive. The mint has the form in hand and the key IS the
    // form (law 2), so it writes the coordinate directly and
    // `from_element_form` reads the form back out of it.
    let _ = e.address.set(crate::transducer::sentence::form_vertex_key(f));
    // THE FILING TRAVELS WITH THE WORD THAT MAKES IT (54.5). It is the
    // mind's attestation about this form's cycles — not a coordinate of the
    // geometry — so it rides the element the mind mints and not the form
    // the address names.
    e.filing = f.torsion.clone();
    // THE RECEIPTS ENVELOPE (54.5 Road (d), B2). The three carriers the key
    // deliberately excludes as EVIDENCE — the citations a vertex factored
    // through, the citations a chart makes of another chart with the affine
    // transition they meet under, and the pivots the crystallization rests
    // on — come off the form here, at the one place a word is made.
    //
    // They are taken from the form and not from its tape, so this is the
    // reading that does not need the tape to exist.
    e.set_receipts(crate::transducer::element::ReceiptsEnvelope::of_form(f));
    // THE DESYNC IS NAMED AT THE MINT, NOT ONLY AT THE COURT (54.7 B5a,
    // §8.1 — instruments before fixes).
    //
    // A `Vertex::Glued` is a port and a `NerveEdge` is a bond, and the two
    // are in bijection by site. The court refuses a form where they are
    // not — `REFUSE_ORPHAN_PORT`, `REFUSE_PHANTOM_EDGE` — and on the recut
    // at epoch 93 it refused 10 speeches for it, at 94 fifteen. That names
    // the READER; it does not name the producer, and two rounds of reading
    // the mutators found one of them (`rebase_by_word`) and missed the
    // rest.
    //
    // So the mint says it. Every form that leaves here as a word is
    // checked against its own cover, and a mismatch prints the counts at
    // the one place that has the form in hand and knows which pass built
    // it. A reading, never a refusal: the mint does not decide what the
    // court decides (law 8).
    {
        let env = e.receipts();
        // AGAINST THE FORM THE COURT WILL HOLD, not the one in hand.
        //
        // The mint has `f`; the court has `from_address(address)`. The
        // bonds are the envelope's and are never round-tripped, while the
        // ports the court sees come back through `put_vertex` and
        // `from_address`. If a placement or a carriage is not a fixed point
        // of that round trip, the two sites differ and every citation is
        // both an orphan port and a phantom edge — at the COURT, while the
        // mint's own form is perfectly in bijection.
        //
        // Checked here on the decoded form, so the reading names which of
        // the two it is: a desync the mint's form already has, or one the
        // decode introduces.
        //
        // A minted form's own address always decodes — that is the retract
        // `from_address_retracts_every_corpus_form` asserts — so an address
        // that does not is its own reading and is said as one, never
        // replaced by the form in hand.
        match crate::transducer::sentence::from_address(&e.address()) {
            None => eprintln!(
                "  mint cover desync: the address does not decode, so the court holds no form"
            ),
            Some(f) => {
                let mut orphan = 0usize;
                for v in &f.vertices {
                    if !matches!(v, crate::transducer::sentence::Vertex::Glued { .. }) {
                        continue;
                    }
                    let site = crate::transducer::sentence::vertex_bytes(v);
                    if !env.nerve.iter().any(|x| x.host().as_bytes() == site) {
                        orphan += 1;
                    }
                }
                let mut phantom = 0usize;
                for x in &env.nerve {
                    let host = x.host();
                    if !f.vertices.iter().any(|v| {
                        matches!(v, crate::transducer::sentence::Vertex::Glued { .. })
                            && crate::transducer::sentence::vertex_bytes(v) == host.as_bytes()
                    }) {
                        phantom += 1;
                    }
                }
                if orphan > 0 || phantom > 0 {
                    let ports = f
                        .vertices
                        .iter()
                        .filter(|v| matches!(v, crate::transducer::sentence::Vertex::Glued { .. }))
                        .count();
                    eprintln!(
                        "  mint cover desync: {ports} ports, {} bonds, {orphan} orphan, {phantom} phantom",
                        env.nerve.len()
                    );
                }
            }
        }
    }
    // The coordinate the mint wrote is the word's ADDRESS (law 2), and the
    // bytes it moved are the tape it laid down — which is also what the
    // wire will carry. Two columns, never one (law 5).
    trace::trace_bytes("mint/form", trace::Wrote::Key(e.address().as_bytes()), None, n as i128);
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
    // 54.7 B6 — THE VERSION GATE IS GONE WITH THE TAPE IT GATED.
    //
    // It read `description()[1] == FORM_TAPE_VERSION` and refused a tape
    // this build could not parse. There is no tape: the form comes out of
    // the ADDRESS, whose layout is versioned by §10 — a layout change is a
    // recut — and whose kind is its own leading `TAG_FORM`, which
    // `is_sentence` above has already read. A gate on a byte buffer that
    // is no longer written would have refused every sentence.
    // THE FORM COMES OUT OF THE ADDRESS (54.5, as ruled 2026-09-20), and
    // the geometry is all it comes out with.
    //
    // This parsed the tape, and `Element::address` parsed the tape to get
    // here, so the two were one cycle with a byte run at the centre.
    // Switching it failed 20 tests with a single cause — a form decoded
    // from its address arrived with its FILED CYCLES gone, and
    // `form_orbits_replay` refused it.
    //
    // The filing was never the form's to carry. `pivot_orbit` takes the mesh
    // horizon, the mind's crystal and a budget and files only in its
    // `NoneDischarged` branch, so the map does not factor through the form:
    // two minds holding one complex at different budgets file differently.
    // Keying it would make a pivot RENAME its own word and orphan every
    // citation of it. It is an attestation, `Address × Signer →
    // FiledOrbit`, and it rides `Element.filing` — the word as one mind
    // minted it, which is the smallest carrier that has both indices.
    //
    // So a decoded form holds its member complex, its faces and its derived
    // cycles, and ZERO assertions about what a stroke did to them. The
    // filing moved to `Element.filing` and the court reads it there, which
    // is the half of this that landed.
    //
    // THE READER NO LONGER WAITS (54.5 Road (d), B4 — attempted twice and
    // landed on the third, 2026-09-20).
    //
    // `from_address` is a faithful retract of the GEOMETRY:
    // `the_address_and_the_tape_agree_on_every_reading_the_court_makes`
    // runs both decoders over 36 corpus forms and finds no divergence on
    // face words, the rank audit, closure, citation depth or the orbits'
    // member paths. Four real defects were fixed getting there — a
    // defaulted citation chart, a byte-exact carriage comparison, a face
    // word read off a presentation rather than an action, and an underived
    // depth.
    //
    // WHAT WAS LEFT WAS NOT GEOMETRY, and the two attempts that failed both
    // failed on that. Three carriers rode the tape that the key deliberately
    // excludes, because each is EVIDENCE and not a coordinate (§1.5,
    // `evidence_is_not_identity`):
    //
    //   `Derivation.generators` — the citations a mind factored through.
    //   A decoded vertex is `spelled`, so the court could not verify a
    //   factored derivation and the economy could not pay a cited
    //   generator's speaker.
    //
    //   `Form.certificates` — the pivots a crystallization rests on (A20).
    //
    //   The filing, which moved to `Element.filing` at df1eeaa.
    //
    // `Tape ∖ Key` measured 2 parts in 10 000 IN BYTES, and that reading
    // missed all of it: certificates were 0 bytes and `cites` was 0 bytes on
    // the fleet, so a carrier empty for an unrelated reason read as a
    // carrier not needed. The fixtures, not the fleet, are where adjudication
    // executes today, and they are what refused.
    //
    // All three now ride `Element.receipts` — the graded nerve incidence and
    // the evidence a court replays with — and the six readers that wanted
    // them take them from there. So the decoder asks the ADDRESS, which is
    // what the form IS (law 2), and the tape it used to parse is now written
    // and read by nothing but itself. B5 has evicted the citation targets
    // from the key; B6 deletes the tape.
    let mut f = crate::transducer::sentence::from_address(&e.address())?;
    // THE COVER, FROM THE WORD'S OWN ENVELOPE (54.7 B5a).
    //
    // `from_address` is a faithful retract of the GEOMETRY and the geometry
    // is all it retracts: the key states each citation's BOUNDARY — a
    // rank, a depth, a placement, a φ — and not which form that boundary
    // mates with, because a chart does not know what it overlaps. So the
    // form comes back with no cover, which is true of a form reconstituted
    // from a key alone.
    //
    // Here there is a WORD in hand, and a word carries what its mind
    // attests: `ReceiptsEnvelope.nerve` is `N(𝒰)`'s 1-skeleton, one edge
    // per incidence, each keyed by the site of the boundary it leaves from.
    // Without this line every reader of the cover on a received sentence —
    // the court speaking a submitted rule, the mind comprehending an
    // utterance, the retirement scope, the use credit — reads an empty
    // cover and resolves nothing, silently.
    f.cover = e.receipts().nerve.clone();
    Some(f)
}

/// RE-MINT A FORM A MIND ALREADY HOLDS, carrying that mind's attestation
/// forward (54.5 Road (d), B4).
///
/// [`as_element_form`] takes the attestation off the FORM, which is right
/// for a fresh mint — the extractor's form carries the filing its own stroke
/// made, the derivations its own peel found and the certificates its own
/// pivot wrote. A form that came back out of an ADDRESS carries none of
/// them, because none of them is geometry: a decoded vertex is `spelled`, a
/// decoded form files nothing and certifies nothing. So a caller that
/// decodes a held word, moves it, and mints the result got an element
/// attesting NOTHING, and its next replay refused.
///
/// The tape hid this completely. A taped form carried its own filing, so
/// `decode → regauge → as_element_form` round-tripped the attestation by
/// accident; the moment the decoder read the address the accident stopped.
/// `germinate::stroke`'s re-anchor is the live site.
///
/// WHY THE FILING SURVIVES THE MOVE, which is a theorem and not a
/// convenience. A re-gauge conjugates: §3.6 rewrites every face word as
/// `γ_j · w · γ_i⁻¹`, so an orbit's holonomy becomes `Ω' = γ_0 Ω γ_0⁻¹`. A
/// `FiledOrbit` is keyed by the CLASS key, which is a conjugacy invariant,
/// so the classes are the same classes and the periods are the same
/// periods. Carrying the filing across is carrying it to the same keys.
///
/// THE NERVE IS NOT CARRIED, IT IS RE-DERIVED. A citation's coordinates —
/// target, chart, depth, placement, transition — are all in the key, so the
/// decoded form still has its `Glued` vertices whole and `of_form` rebuilds
/// the 1-simplices from the NEW geometry, which is where they should come
/// from after a move. Only the derivations and the certificates, which the
/// key excludes as evidence, come from the mind.
pub fn remint_form(f: &Form, mag: [u64; 4], from: &Element) -> Option<Element> {
    let mut e = as_element_form(f, mag)?;
    e.filing = from.filing().clone();
    let mut env = e.receipts().clone();
    env.derivations = from.receipts().derivations.clone();
    env.certificates = from.receipts().certificates.clone();
    e.set_receipts(env);
    Some(e)
}

/// THE SPEAKER'S FILING, keyed by class: the cycles this mind's own pivot
/// stroke filed before it spoke.
///
/// IT READS `Element.filing` AND NOT THE FORM (54.5, the other half of
/// `df1eeaa`). The filing moved off `Form.torsion` because it does not
/// factor through the geometry — `pivot_orbit` takes the mesh horizon, the
/// mind's crystal and a budget, so two minds holding one complex at
/// different budgets file differently — and the court was re-pointed to
/// `Element::filing` then. These two public readers were not, and went on
/// asking the form.
///
/// While the decoder still parsed the tape they kept answering, because a
/// taped form carried its own filing. The moment `from_element_form` read
/// the ADDRESS instead, they answered EMPTY for every word in the tree:
/// seven claims failed at once with one cause, which is the shape of a move
/// that was made at the court and not at its readers.
pub fn form_torsion(
    e: &Element,
) -> Keyed<FiledOrbit, crate::network::nerve::coord::Holonomy> {
    e.filing().clone()
}

/// Whether a spoken word carries a torsion cycle: the speaker's filing
/// first, then the residual element's own section, so a word whose cycles
/// live on the axis rather than on a form's orbits still reads as a bundle.
pub fn carries_torsion(e: &Element) -> bool {
    !e.filing().is_empty() || !e.torsion().is_empty()
}

/// The rotor a filed torsion cycle names: the chord face stored on the
/// class key. None when that face is missing — never an identity rotor.
pub fn torsion_orbit_rotor(
    f: &Form,
    class: &crate::network::nerve::coord::Holonomy,
) -> Option<Rotor> {
    // THE CHORD IS FOUND BY THE CLASS THE COMPLEX DERIVES (54.5, as ruled
    // 2026-09-20). This read `f.torsion.get(class)?.chord` — the FILING's
    // chord — and a form recovered from its address files nothing, so it
    // answered `None` for every word in the tree after B4.
    //
    // It never needed the filing. An orbit's class is a theorem of its faces
    // (`orbit_classes`, through `cech::classify`), so the orbit carrying this
    // class is found by asking the geometry which one does. The filing only
    // ever memoized that answer, and law 12 keeps a theorem out of storage.
    for lp in &f.orbits {
        if f.orbit_classes(lp).iter().any(|row| &row.class == class) {
            return f.faces.get(lp.chord as usize).map(|face| face.word.rotor());
        }
    }
    None
}

/// A sentence Element: tagged tape, not an opened-axis connection.
pub fn is_sentence(e: &Element) -> bool {
    e.is_sentence()

}
