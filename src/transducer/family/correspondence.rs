// 21 step 1 as ruled in 52.5 — THE TRANSPORT CLASS: what names a vertex of
// the family's form, and the relatives two classes carry on its faces.
//
//! A transport class is kind and the conjugacy of the word (15.4), never
//! the hue map and never the body's moduli, which are section coordinates
//! (rule 7). The readers here name; every decision is made by the moves
//! in `extract.rs` that call them.

use super::*;

/// Correspondence class of a vertex: kind and conjugacy of the word.
/// The hue map is a section coordinate on `Form.hue_maps`, never a matching
/// key. The body's moduli stay on the vertex as a section coordinate.
/// Frame matches by kind alone and is not a class.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TransportClass {
    kind: crate::transducer::peel::ShellKind,
    word_class: Vec<u8>,
}

impl TransportClass {
    pub(crate) fn key(&self) -> Vec<u8> {
        let mut buf = vec![self.kind.as_u8()];
        crate::transducer::address::put_bytes(&mut buf, &self.word_class);
        buf
    }
}

pub(crate) fn transport_class(v: &Vertex) -> Option<TransportClass> {
    match v {
        // The class is the composed product's, never the residue's: two
        // minds reaching one vertex by different derivations correspond
        // (50.2 step 2), and a class read off a residue would split them.
        Vertex::Word { derivation, .. } => Some(TransportClass {
            kind: crate::transducer::peel::ShellKind::Whole,
            word_class: derivation.rotor().class_key()?.into_vec(),
        }),
        _ => None,
    }
}

/// The key of one frame of the family by its index, for a carrier keyed
/// per frame: the frame is the corpus's coordinate (51.1).
pub(crate) fn pair_idx_key(i: u16) -> Vec<u8> {
    let mut k = vec![0u8];
    k.extend_from_slice(&i.to_le_bytes());
    k
}

/// THE KEY A MEMBER'S SECTION IS FILED UNDER: the transport class, the
/// chart, the vertex's address and the carriage the member acts by, so
/// `Form.sections` answers which frames of the family that member was
/// measured on (18.2, 21 step 1, 52.5 item 3). The address and the
/// carriage are in the key because two vertices of one class can stand on
/// one chart — a cover's pieces at two frames, a chart whose frames read
/// two versors — and each is claimed on its own frames; two members whose
/// every coordinate agrees act alike and are claimed alike.
pub(crate) fn member_section_key(v: &Vertex, chart: &Invariant) -> Option<Vec<u8>> {
    let Vertex::Word { address, .. } = v else { return None };
    let member = v.acting_on(chart)?;
    let mut k = transport_class(v)?.key();
    crate::transducer::address::put_bytes(&mut k, chart.key().as_bytes());
    crate::transducer::address::put_bytes(&mut k, address.as_bytes());
    let mut carriage = Vec::new();
    member.carriage.put_canonical(&mut carriage);
    crate::transducer::address::put_bytes(&mut k, &carriage);
    Some(k)
}

/// THE RELATIVE PLACEMENTS BETWEEN TWO CLASSES ON ONE FORM, keyed by the
/// member pair, read off THE FORM'S OWN FACES between them.
///
/// The incidence is the face: a face exists because two bodies' supports meet
/// or sit at ring distance one (21 step 2), so the face says which body of
/// class `a` relates to which body of class `b`. Taking the product of the two
/// classes' placements would relate bodies nothing in the scene relates.
///
/// The value is `(w_b, C_b · C_a⁻¹)` — the word the far end speaks and the
/// relative carriage `rebase_named_faces` asks about. A pair whose words differ
/// contributes nothing: the two ends do not speak one word there and there is
/// no rebase to make.
pub(crate) fn relatives_between(
    pf: &Form,
    a: &TransportClass,
    b: &TransportClass,
) -> crate::network::nerve::Keyed<(crate::transducer::alphabet::Word, Versor)> {
    let mut out = crate::network::nerve::Keyed::new();
    let (ka, kb) = (a.key(), b.key());
    for face in &pf.faces {
        let (Some(vi), Some(vj)) = (
            pf.vertices.get(face.i as usize),
            pf.vertices.get(face.j as usize),
        ) else {
            continue;
        };
        let (Some(ci), Some(cj)) = (
            transport_class(vi).map(|c| c.key()),
            transport_class(vj).map(|c| c.key()),
        ) else {
            continue;
        };
        // The face runs a→b or b→a; either way the relative asked for is the
        // one from `a`'s body to `b`'s.
        let forward = ci == ka && cj == kb;
        if !(forward || (ci == kb && cj == ka)) {
            continue;
        }
        let (Vertex::Word { derivation: di, .. }, Vertex::Word { derivation: dj, .. }) = (vi, vj)
        else {
            continue;
        };
        let (Some(wi), Some(wj)) = (di.word_local(), dj.word_local()) else {
            continue;
        };
        if wi != wj {
            continue;
        }
        for (pk, _) in face.at.scoped() {
            let Some((ik, jk)) = crate::transducer::sentence::read_face_pair_key(&pk) else {
                continue;
            };
            let (Some(cca), Some(ccb)) = (
                if forward { vi.chart_keyed(&ik) } else { vj.chart_keyed(&jk) },
                if forward { vj.chart_keyed(&jk) } else { vi.chart_keyed(&ik) },
            ) else {
                continue;
            };
            let (Some(ma), Some(mb)) = (
                if forward { vi.acting_on(&cca) } else { vj.acting_on(&cca) },
                if forward { vj.acting_on(&ccb) } else { vi.acting_on(&ccb) },
            ) else {
                continue;
            };
            let Some(inv) = ma.carriage.versor_inverse() else {
                continue;
            };
            out.insert(
                crate::transducer::sentence::face_pair_key(&cca, &ccb),
                (wj.clone(), mb.carriage.geo(&inv)),
            );
        }
    }
    out
}

pub(crate) fn hue_maps_of_form(form: &Form) -> crate::network::nerve::Keyed<Vec<(u8, u8)>> {
    let mut m = form.hue_maps.clone();
    for v in &form.vertices {
        // ONE HUE MAP PER MEMBER: a vertex acting on several charts carries
        // a gauge at each, and `hue_maps` is keyed by rotor and gauge, so
        // each is its own section (21 step 1).
        if let Vertex::Word { derivation, acts, .. } = v {
            for (_, a) in acts.scoped() {
                if let Some(k) =
                    crate::transducer::sentence::hue_section_key_of(&derivation.rotor(), &a.gauge)
                {
                    if !m.contains(&k) {
                        m.insert(k, a.gauge.clone());
                    }
                }
            }
        }
    }
    m
}
