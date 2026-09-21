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
    /// 54.7 R5 — THE ONE WRITER OF A TRANSPORT COORDINATE.
    ///
    /// The return type names the system these bytes belong to, so a carrier
    /// that answers the transport class takes this and nothing else. The
    /// bytes open with the shell kind rather than an A21 tag, so there is
    /// no tag to check and the type is the whole gate — which is the honest
    /// statement of what this key is.
    pub(crate) fn key(&self) -> crate::network::nerve::Key<crate::network::nerve::coord::Transport> {
        let mut buf = vec![self.kind.as_u8()];
        crate::transducer::address::put_bytes(&mut buf, &self.word_class);
        crate::network::nerve::Key::declared(&buf)
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
pub(crate) fn member_section_key(
    v: &Vertex,
    chart: &Invariant,
) -> Option<crate::network::nerve::Key<crate::network::nerve::coord::MemberSection>> {
    let Vertex::Word { address, .. } = v else { return None };
    let member = v.acting_on(chart)?;
    // THE TRANSPORT CLASS OPENS A FINER COORDINATE. What is written here
    // is a member SECTION — class, chart, address, carriage — and it is a
    // key of that system and not of the transport one, so the coordinate
    // is left behind explicitly at the point the extension begins.
    let mut k = transport_class(v)?.key().into_vec();
    crate::transducer::address::put_bytes(&mut k, chart.key().as_bytes());
    crate::transducer::address::put_bytes(&mut k, address.as_bytes());
    let mut carriage = Vec::new();
    member.carriage.put_canonical(&mut carriage);
    crate::transducer::address::put_bytes(&mut k, &carriage);
    Some(crate::network::nerve::Key::declared(&k))
}

/// THE CLASS-LEVEL SECTION: a transport class read as the COARSE STRATUM of
/// [`member_section_key`], which opens with exactly these bytes.
///
/// 54.7 R5 — `Form.sections` is read at both depths and nothing said so.
/// Its doc comment names only the class level; `extract` writes member keys
/// into it as well, and the court reads both. They are one system — a
/// member section is a class key with chart, address and carriage written
/// over it — so a class key IS a prefix of a member key and the two reads
/// are a coarse and a fine stratum of one carrier, not two carriers sharing
/// a name. This is where a class key becomes that coordinate, said once.
pub(crate) fn class_section_key(
    tc: &TransportClass,
) -> crate::network::nerve::Key<crate::network::nerve::coord::MemberSection> {
    crate::network::nerve::Key::declared(tc.key().as_bytes())
}

/// THE KEY OF AN ORDERED PAIR OF TRANSPORT CLASSES: `a`'s key then `b`'s,
/// each length-delimited, so the relatives from `a`'s bodies to `b`'s are
/// one stratum and the reverse direction is another.
pub(crate) fn class_pair_key(a: &TransportClass, b: &TransportClass) -> Vec<u8> {
    let mut k = Vec::new();
    crate::transducer::address::put_bytes(&mut k, &a.key());
    crate::transducer::address::put_bytes(&mut k, &b.key());
    k
}

/// THE RELATIVE PLACEMENTS BETWEEN EVERY TWO CLASSES ON ONE FORM, read off
/// THE FORM'S OWN FACES in one pass: ordered class pair → member pair →
/// `(w_b, C_b · C_a⁻¹)`, the word the far end speaks and the relative
/// carriage `rebase_named_faces` asks about.
///
/// The incidence is the face: a face exists because two bodies' supports meet
/// or sit at ring distance one (21 step 2), so the face says which body of
/// class `a` relates to which body of class `b`. Taking the product of the two
/// classes' placements would relate bodies nothing in the scene relates.
///
/// Every face is walked once and files its relative under both directions,
/// so a caller holding two classes reads their stratum by key (rule 1). A
/// pair whose words differ contributes nothing: the two ends do not speak
/// one word there and there is no rebase to make. Each vertex's class is
/// read once, from the vertices, not once per face end.
pub(crate) fn relatives_by_class(
    faces: &[crate::transducer::sentence::FormFace],
    vertices: &[Vertex],
) -> crate::network::nerve::Keyed<crate::network::nerve::Keyed<(crate::transducer::alphabet::Word, Versor)>> {
    let classes: Vec<Option<TransportClass>> = vertices.iter().map(transport_class).collect();
    let mut out: crate::network::nerve::Keyed<crate::network::nerve::Keyed<_>> =
        crate::network::nerve::Keyed::new();
    let mut file = |pair: Vec<u8>, at: Vec<u8>, value: (crate::transducer::alphabet::Word, Versor)| {
        match out.get_mut(&pair) {
            Some(rels) => {
                rels.insert(&at, value);
            }
            None => {
                let mut rels = crate::network::nerve::Keyed::new();
                rels.insert(&at, value);
                out.insert(&pair, rels);
            }
        }
    };
    for face in faces {
        let (Some(vi), Some(vj)) = (vertices.get(face.i as usize), vertices.get(face.j as usize)) else {
            continue;
        };
        let (Some(Some(ci)), Some(Some(cj))) = (classes.get(face.i as usize), classes.get(face.j as usize))
        else {
            continue;
        };
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
        let (forward, backward) = (class_pair_key(ci, cj), class_pair_key(cj, ci));
        for (pk, _) in face.at.scoped() {
            let Some((ik, jk)) = crate::transducer::sentence::read_face_pair_key(&pk) else {
                continue;
            };
            let (Some(cci), Some(ccj)) = (vi.chart_keyed(&ik), vj.chart_keyed(&jk)) else {
                continue;
            };
            let (Some(mi), Some(mj)) = (vi.acting_on(&cci), vj.acting_on(&ccj)) else {
                continue;
            };
            let (Some(inv_i), Some(inv_j)) = (mi.carriage.versor_inverse(), mj.carriage.versor_inverse())
            else {
                continue;
            };
            // From `i`'s body to `j`'s under `(class_i, class_j)`, and back
            // under `(class_j, class_i)`: the one incidence, both directions.
            file(
                forward.clone(),
                crate::transducer::sentence::face_pair_key(&cci, &ccj),
                (wj.clone(), mj.carriage.geo(&inv_i)),
            );
            file(
                backward.clone(),
                crate::transducer::sentence::face_pair_key(&ccj, &cci),
                (wj.clone(), mi.carriage.geo(&inv_j)),
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
