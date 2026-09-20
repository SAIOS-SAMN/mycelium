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
use crate::network::nerve::Keyed;
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
            Vertex::Glued {
                address,
                chart,
                magnitude,
                depth,
                placement,
                carriage,
            } => {
                out.push(2);
                crate::transducer::address::put_addr(out, address)?;
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
    }
    // THE LOOPS ARE NOT ON THE TAPE (54.5, as ruled 2026-09-20). §1.5 has
    // said since it was written that "loops and holonomies are derived and
    // never keyed", and the tape carried them anyway — a second statement
    // of the incidence the faces above already are (law 12).
    //
    // Measured, it was the WHOLE of what the tape held beyond the key: 2
    // parts in 10 000, `loops + carriages`, everything else either in the
    // key or empty. So this block was the tape's entire reason to exist,
    // and it was reason for nothing: `loops_of` re-derives it from the
    // faces deterministically.
    //
    // AND IT COST THE GEOMETRY. `Loop.vertices` is a list of VERTEX
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
                // THE FIBRE, one member per chart (D7): a count, then each
                // member's carriage, gauge and chart.
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
                let carriage = Box::new(element::Versor::from_bytes(b, pos)?);
                Vertex::Glued {
                    address,
                    chart,
                    magnitude,
                    depth,
                    placement,
                    carriage,
                }
            }
            3 => {
                let by = take_u16(b, pos)?;
                let of = take_u16(b, pos)?;
                // The fibre, as a Word vertex's (v8, D10).
                let nm = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
                let mut acts: Keyed<crate::transducer::sentence::Acting> = Keyed::new();
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
        faces.push(FormFace { i, j, word, at, pairs });
    }
    // THE LOOPS ARE RE-DERIVED, NOT READ (§1.5, 54.5). Nothing is taken
    // from the tape here because the writer puts nothing there; the cycle
    // basis is a theorem of the faces just parsed, and `loops_of` proves
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
    let mut sections = Keyed::new();
    for _ in 0..ns {
        let k = crate::transducer::address::take_addr(b, pos)?;
        let nv = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
        let mut ix = Vec::with_capacity(nv);
        for _ in 0..nv {
            ix.push(take_u16(b, pos)?);
        }
        sections.insert(k, section_of_frames(ix));
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
        // The period as filed: finite at `ℓ`, `0` for infinite order, A19's
        // spelling of it (52.3).
        let ell = take_u32(b, pos)?;
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
    // `glued` is `reindex`'s to build (law 12), and `reindex` runs below.
    let mut form = Form {
        vertices,
        faces,
        // Re-derived below, from the faces just parsed (§1.5, 54.5).
        loops: Vec::new(),
        remainder,
        depth,
        hue_maps: Keyed::new(),
        ratio,
        sections,
        certificates,
        torsion,
        axes,
        glued: Keyed::new(),
        vortex: Keyed::new(),
        by_address: Keyed::new(),
        by_class: Keyed::new(),
        by_transport: Keyed::new(),
        star: Keyed::new(),
        carriages: Keyed::new(),
        by_rebased: Keyed::new(),
        by_unfolded: Keyed::new(),
        demands: Keyed::new(),
    };
    form.reindex();
    // THE CYCLE BASIS IS A THEOREM OF THE INCIDENCE, proved here the same
    // way the extractor proves it, rather than read back from a carrier
    // that would be a second statement of the faces (law 12, §1.5).
    form.loops = Form::loops_of(&form.faces, &form.vertices);
    Some(form)
}

/// THE TAPE'S LAST VERSION (54.5). 14, and the byte is never bumped for the
/// form again.
///
/// 13 → 14 REMOVES the loop block, and is the only bump this file will take
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
/// placed, and a loop's period filed as `0` for infinite order (52.3).
pub const FORM_TAPE_VERSION: u8 = 14;

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
/// loops — which coheres vacuously and is closed, as a generator should be.
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
    f.loops = Form::loops_of(&f.faces, &f.vertices);
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
    let mut word = vec![element::SENTENCE_TAG, FORM_TAPE_VERSION];
    let Some(()) = encode_form_v2(f, &mut word) else {
        trace::trace("mint/encode", trace::Wrote::Nothing, None);
        return None;
    };
    let n = word.len();
    e.set_description(word);
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
    if e.description().get(1).copied() != Some(FORM_TAPE_VERSION) {
        return None;
    }
    // THE FORM COMES OUT OF THE ADDRESS (54.5, as ruled 2026-09-20), and
    // the geometry is all it comes out with.
    //
    // This parsed the tape, and `Element::address` parsed the tape to get
    // here, so the two were one cycle with a byte run at the centre.
    // Switching it failed 20 tests with a single cause — a form decoded
    // from its address arrived with its FILED CYCLES gone, and
    // `form_loops_replay` refused it.
    //
    // The filing was never the form's to carry. `pivot_loop` takes the mesh
    // horizon, the mind's crystal and a budget and files only in its
    // `NoneDischarged` branch, so the map does not factor through the form:
    // two minds holding one complex at different budgets file differently.
    // Keying it would make a pivot RENAME its own word and orphan every
    // citation of it. It is an attestation, `Address × Signer →
    // FiledLoop`, and it rides `Element.filing` — the word as one mind
    // minted it, which is the smallest carrier that has both indices.
    //
    // So a decoded form holds its member complex, its faces and its derived
    // cycles, and ZERO assertions about what a stroke did to them. The
    // filing moved to `Element.filing` and the court reads it there, which
    // is the half of this that landed.
    //
    // THE READER STAYS ON THE TAPE FOR ONE MORE REASON, and it is not the
    // filing. Switching it leaves 28 tests refusing `REFUSE_FACE`, "face
    // does not re-derive from its vertices" — because `face_rederives`
    // recomputes each member pair's relative carriage from the two
    // vertices, and the key stores every carriage in A4 CANONICAL form. The
    // canonical of a product is not the product of the canonicals, so
    // `canonical(a) · canonical(b)⁻¹` and the stored `canonical(a·b⁻¹)` can
    // differ by a scale. The tape preserved exact bytes and hid it.
    //
    // That is a real question about what A4 canonicalization preserves, not
    // a gap in this reader, and it is the last thing standing between the
    // address and the tape. `from_address_retracts_every_corpus_form` does
    // not catch it: it asserts the KEY round-trips, which it does, and the
    // court's predicates are a stronger claim the guard does not yet make.
    let b = e.description();
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
    f.faces.get(chord as usize).map(|face| face.word.rotor())
}

/// A sentence Element: tagged tape, not an opened-axis connection.
pub fn is_sentence(e: &Element) -> bool {
    e.is_sentence()

}
