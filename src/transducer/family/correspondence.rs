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

/// THE ONE VERTEX OF A CLASS ON A PAIR FORM. A pair form's vertices each
/// act on one chart, so on a pair form this is exact — the class and the
/// chart are in bijection there and there is nothing for it to pass over.
///
/// `one_vertex_of_class` was the alias extraction used to build the RULE's
/// vertex, and on a rule the bijection is what fails: a class spanning two
/// charts has two vertices per pair form and this answered the first. It is
/// gone; correspondence reads every member (D7b) and nothing asks a form for
/// "the" vertex of a class any more.
pub(crate) fn match_vertex<'a>(f: &'a Form, class: &TransportClass) -> Option<&'a Vertex> {
    f.vertices
        .iter()
        .find(|v| transport_class(v).as_ref() == Some(class))
}

/// The unique relative word between two ends on a pair form, read from
/// that form's named faces. Several instances of a class are a section:
/// every such face must be one M1 vertex.
pub(crate) fn face_word_between(pf: &Form, a: &End, b: &End) -> Option<crate::transducer::alphabet::Word> {
    let mut held: Option<crate::transducer::alphabet::Word> = None;
    for f in &pf.faces {
        let ea = end_of(pf.vertices.get(f.i as usize)?)?;
        let eb = end_of(pf.vertices.get(f.j as usize)?)?;
        let (w, ok) = if &ea == a && &eb == b {
            (f.transport.word.clone(), true)
        } else if &ea == b && &eb == a {
            (f.transport.word.inverse(), true)
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
pub(crate) fn word_place_of_class(
    pf: &Form,
    tc: &TransportClass,
) -> Option<(crate::transducer::alphabet::Word, Versor)> {
    match match_vertex(pf, tc) {
        Some(v @ Vertex::Word { derivation, .. }) => {
            Some((derivation.word_local()?.clone(), v.carriage_of()?.clone()))
        }
        _ => None,
    }
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
