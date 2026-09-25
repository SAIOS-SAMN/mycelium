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
    /// 54.7 — THE CONJUGACY AS THE COORDINATE IT IS, not its bytes. It was
    /// a `Vec<u8>` holding a class key, which made every reader of it
    /// trust a comment; it is the holonomy class itself now, so a
    /// correspondence match reads `(I₁, I₂, kind)` and can say which.
    class: crate::network::nerve::coord::Holonomy,
}

impl TransportClass {
    /// 54.7 R5 — THE TRANSPORT COORDINATE THIS CLASS IS.
    ///
    /// Not bytes: the coordinate value, which carries its own encoding. A
    /// carrier that answers the transport class takes this and nothing
    /// else, and a reader walking one gets the shell kind and the
    /// conjugacy back rather than a slice it has to believe a comment
    /// about (rule 2).
    pub(crate) fn key(&self) -> crate::network::nerve::coord::Transport {
        crate::network::nerve::coord::Transport {
            kind: self.kind,
            class: self.class.clone(),
        }
    }
}

pub(crate) fn transport_class(v: &Vertex) -> Option<TransportClass> {
    match v {
        // The class is the composed product's, never the residue's: two
        // minds reaching one vertex by different derivations correspond
        // (50.2 step 2), and a class read off a residue would split them.
        Vertex::Word { derivation, .. } => Some(TransportClass {
            kind: crate::transducer::peel::ShellKind::Whole,
            class: derivation.rotor().holonomy_key()?,
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

// `member_section_key` AND `class_section_key` STOOD HERE AND RETIRED WITH
// `Form.sections` AS A CARRIER (AT1, ruled (a)): a member holds its frames,
// and the sections at both depths are READ off the members
// ([`Form::sections`]), which builds the same two coordinates itself.

/// THE KEY OF AN ORDERED PAIR OF TRANSPORT CLASSES: `a`'s key then `b`'s,
/// each length-delimited, so the relatives from `a`'s bodies to `b`'s are
/// one stratum and the reverse direction is another.
pub(crate) fn class_pair_key(a: &TransportClass, b: &TransportClass) -> Vec<u8> {
    let mut k = Vec::new();
    use crate::network::nerve::Coordinate;
    crate::transducer::address::put_key_bytes(&mut k, a.key().encode().as_bytes());
    crate::transducer::address::put_key_bytes(&mut k, b.key().encode().as_bytes());
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
/// Every overlap is read once and files its relative under both directions,
/// keyed by the overlap read that way, so a caller holding two classes reads
/// their stratum by key (rule 1). A pair whose words differ contributes
/// nothing: the two ends do not speak one word there and there is no rebase
/// to make. Each member is read AT THE OVERLAP'S FRAME — the section standing
/// there (AT1).
pub(crate) fn relatives_by_class(
    faces: &crate::transducer::sentence::Faces,
    vertices: &crate::transducer::sentence::Vertices,
) -> crate::network::nerve::Keyed<crate::network::nerve::Keyed<(crate::transducer::alphabet::Word, Versor)>> {
    use crate::network::nerve::coord::ChartAt;
    use crate::network::nerve::Coordinate;
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
    for (o, _) in faces.entries() {
        let (ChartAt::At(a), ChartAt::At(b)) = (&o.from, &o.to) else { continue };
        let (Some(vi), Some(vj)) = (vertices.at(&a.vertex), vertices.at(&b.vertex)) else {
            continue;
        };
        let (Some(ci), Some(cj)) = (transport_class(vi), transport_class(vj)) else {
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
        let (Some(mi), Some(mj)) = (vi.acting_at(&a.chart, a.frame), vj.acting_at(&b.chart, b.frame)) else {
            continue;
        };
        // Each carriage's reverse, the incidence read as rays (78.32(1)).
        let is_versor = crate::transducer::cga::is_versor;
        if !is_versor(&mi.carriage) || !is_versor(&mj.carriage) {
            continue;
        }
        let (inv_i, inv_j) = (mi.carriage.reverse(), mj.carriage.reverse());
        // From `i`'s body to `j`'s under `(class_i, class_j)`, and back
        // under `(class_j, class_i)`: the one incidence, both directions.
        file(
            class_pair_key(&ci, &cj),
            o.encode().as_bytes().to_vec(),
            (wj.clone(), mj.carriage.geo(&inv_i)),
        );
        file(
            class_pair_key(&cj, &ci),
            o.reversed().encode().as_bytes().to_vec(),
            (wj.clone(), mi.carriage.geo(&inv_j)),
        );
    }
    out
}

pub(crate) fn hue_maps_of_form(form: &Form) -> crate::network::nerve::Keyed<Vec<(u8, u8)>> {
    let mut m = form.hue_maps.clone();
    for (_, v) in form.vertices.scoped() {
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
