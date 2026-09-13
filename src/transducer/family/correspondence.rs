// 21 step 1 — CORRESPONDENCE: what names a vertex across the pairs of a
// bundle, and the relative word two ends agree on.
//
//! A transport class is kind and the conjugacy of the word (15.4), never
//! the hue map and never the body's moduli, which are section coordinates
//! (rule 7). The readers here name candidates; every decision is made by
//! the moves in `extract.rs` that call them.

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

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum End {
    Frame,
    Class(TransportClass),
}

pub(crate) fn end_of(v: &Vertex) -> Option<End> {
    match v {
        Vertex::Frame => Some(End::Frame),
        Vertex::Word { .. } => Some(End::Class(transport_class(v)?)),
        _ => None,
    }
}

// `match_vertex(f, class)` stood here — `vertices.iter().find(...)`, the last
// finder of "the" vertex of a class. It is gone with its last caller:
// `rebase_named_faces` asked it for a class's placement and now reads the pair
// form's own face between the two classes, which is the incidence that says
// which bodies are related. Nothing asks a form for one vertex of a class.

/// The unique relative word between two ends on a pair form, read from
/// that form's named faces. Several instances of a class are a section:
/// every such face must be one M1 vertex.
pub(crate) fn face_word_between(pf: &Form, a: &End, b: &End) -> Option<crate::transducer::alphabet::Word> {
    let mut held: Option<crate::transducer::alphabet::Word> = None;
    for f in &pf.faces {
        let ea = end_of(pf.vertices.get(f.i as usize)?)?;
        let eb = end_of(pf.vertices.get(f.j as usize)?)?;
        let (w, ok) = if &ea == a && &eb == b {
            (f.word.clone(), true)
        } else if &ea == b && &eb == a {
            (f.word.inverse(), true)
        } else {
            (crate::transducer::alphabet::Word::identity(), false)
        };
        if !ok {
            continue;
        }
        match &held {
            None => held = Some(w),
            Some(h) if h != &w => return None,
            _ => {}
        }
    }
    held
}
// `word_place_of_class(pf, tc)` stood here and answered the representative
// member's placement to a caller asking about a CLASS. Its one caller —
// `rebase_named_faces`, comparing two classes across pair forms — reads
// `relatives_between` instead, which takes the relative off the pair form's own
// face and so names which body is compared to which. A "placement of a class"
// is not a thing a class has once it spans two charts, so the reader is gone
// rather than given a chart parameter nobody passes (D10 item 11).

/// THE RELATIVE PLACEMENTS BETWEEN TWO CLASSES ON ONE PAIR FORM, keyed by the
/// member pair, read off THE PAIR FORM'S OWN FACES between them.
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

pub(crate) fn placement_of_class(pair_forms: &[Form], class: &TransportClass) -> Option<crate::transducer::alphabet::Word> {
    let mut held: Option<crate::transducer::alphabet::Word> = None;
    for pf in pair_forms {
        let w = face_word_between(pf, &End::Frame, &End::Class(class.clone()))?;
        match &held {
            None => held = Some(w),
            Some(h) if h != &w => return None,
            _ => {}
        }
    }
    held
}
pub(crate) fn pair_idx_key(i: u16) -> Vec<u8> {
    let mut k = vec![0u8];
    k.extend_from_slice(&i.to_le_bytes());
    k
}

pub(crate) fn end_key(e: &End) -> Vec<u8> {
    match e {
        End::Frame => vec![0xff],
        End::Class(c) => c.key(),
    }
}

pub(crate) fn ends_pair_key(a: &End, b: &End) -> Vec<u8> {
    let (ka, kb) = (end_key(a), end_key(b));
    let (lo, hi) = if ka <= kb { (ka, kb) } else { (kb, ka) };
    let mut buf = Vec::new();
    crate::transducer::address::put_bytes(&mut buf, &lo);
    crate::transducer::address::put_bytes(&mut buf, &hi);
    buf
}

pub(crate) fn index_ends(form: &Form) -> crate::network::nerve::Keyed<u16> {
    let mut idx = crate::network::nerve::Keyed::new();
    for (i, v) in form.vertices.iter().enumerate() {
        if let Some(e) = end_of(v) {
            let k = end_key(&e);
            if !idx.contains(&k) {
                idx.insert(k, i as u16);
            }
        }
    }
    idx
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
