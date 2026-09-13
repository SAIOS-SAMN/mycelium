// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 18.2, 21 step 1 — THE TRANSPORT of a face: the relative word, carriage
//! versor and hue map from one vertex to another, and the gauge algebra
//! those coordinates compose and invert by.

use super::form::{Form, Vertex};
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{Derivation, Word};
use crate::transducer::element::{self, Versor};

/// Face transport: the relative word, carriage versor and hue map from vertex
/// `i` to vertex `j`. The same three coordinates `transport_class` keys on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transport {
    pub word: Word,
    pub carriage: Versor,
    pub gauge: Vec<(u8, u8)>,
}

/// WHAT A FACE CARRIES AT ONE MEMBER PAIR (21 step 2, 18.2): the relative
/// carriage between the two bodies, the relative hue map, and 50.3's edge
/// transport — the stabilizer element the pair's rigid motion picks out, or
/// `None` for a remainder edge, a pair no single versor carries.
///
/// These are MEMBER coordinates. A face's WORD is not: it is `w_j ∘ w_i⁻¹`
/// read from the two derivations, which are vertex coordinates, so one word
/// serves the face however many charts its endpoints act on. That asymmetry is
/// why the face splits the way it does and why the rotor half of the skeleton
/// — holonomy, torsion, class, the pivot's word moves — is untouched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FacePlace {
    pub carriage: Versor,
    pub gauge: Vec<(u8, u8)>,
    pub edge: Option<Transport>,
}

/// The key a member pair is filed under on a face: the two charts' own keys,
/// each length-delimited so they read apart, `i` then `j`. Content (law 2), so
/// the order is the trie's and the same on every node.
pub fn face_pair_key(
    ci: &crate::transducer::stalk::Invariant,
    cj: &crate::transducer::stalk::Invariant,
) -> Vec<u8> {
    let mut buf = Vec::new();
    crate::transducer::address::put_bytes(&mut buf, ci.key().as_bytes());
    crate::transducer::address::put_bytes(&mut buf, cj.key().as_bytes());
    buf
}

/// Read a member-pair key back as its two chart keys.
pub fn read_face_pair_key(k: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    let mut p = 0usize;
    let ci = crate::transducer::address::take_bytes(k, &mut p)?;
    let cj = crate::transducer::address::take_bytes(k, &mut p)?;
    Some((ci, cj))
}

impl Transport {
    pub fn from_word(word: Word) -> Transport {
        Transport {
            word,
            carriage: element::identity_versor(),
            gauge: Vec::new(),
        }
    }

    pub fn inverse(&self) -> Option<Transport> {
        let word = self.word.inverse();
        let carriage = self.carriage.versor_inverse()?;
        Some(Transport {
            word,
            carriage,
            gauge: gauge_invert(&self.gauge),
        })
    }

    /// Move `form` into this transport's frame (3.10): rebase by word (3.2),
    /// regauge (3.6), compose every carriage. `None` off the plane.
    pub fn refract(&self, form: &Form) -> Option<Form> {
        form.refract(self)
    }
}

impl Form {
    /// The one function that moves a form between frames (3.10).
    pub fn refract(&self, gamma: &Transport) -> Option<Form> {
        gamma.word.form()?;
        let mut f = self.clone();
        f.rebase_by_word(&gamma.word);
        f.regauge_listing(&gamma.gauge);
        f.compose_carriages(&gamma.carriage);
        f.loops = Form::loops_of(&f.faces, f.vertices.len());
        f.reindex();
        let mut addr = crate::transducer::address::Address::new(Vec::new());
        for v in &self.vertices {
            if let Vertex::Word { address, .. } = v {
                addr = address.clone();
                break;
            }
        }
        let frame = gamma.word.key();
        let hx = |a: &crate::transducer::address::Address| {
            a.as_bytes().iter().map(|b| format!("{b:02x}")).collect::<String>()
        };
        eprintln!("refracted {} into {} by Γ", hx(&addr), hx(&frame));
        Some(f)
    }

    pub(crate) fn rebase_by_word(&mut self, by: &Word) {
        for v in &mut self.vertices {
            match v {
                // A rebase is conjugation (3.2), and a conjugated product
                // is a product of conjugated generators — which these
                // citations, naming held forms by address, are not. The
                // derivation does not survive the change of frame, so it
                // is not claimed in the new one: the rebased vertex is
                // spelled (50.2 step 2's boundary case), exactly as far
                // as the mathematics carries.
                Vertex::Word { derivation, .. } => {
                    let Some(w) = derivation.word_local() else {
                        continue;
                    };
                    *derivation = Derivation::spelled(w.rebase(by));
                }
                Vertex::Glued { placement, .. } => *placement = placement.rebase(by),
                _ => {}
            }
        }
        for face in &mut self.faces {
            face.word = face.word.rebase(by);
        }
    }

    pub(crate) fn regauge_listing(&mut self, g: &[(u8, u8)]) {
        for v in &mut self.vertices {
            match v {
                // EVERY MEMBER IS REGAUGED. A word vertex acting on several
                // charts composes the map into each of their gauges: one
                // member is the single-chart case and reads as before.
                Vertex::Word { acts, .. } => {
                    // The chart keys are unchanged by a regauge, so they are
                    // read once and each member reached by its own key —
                    // `Keyed` has no mutable walk, and this is the pattern
                    // `ReceiptChain::reindex_placed` already uses.
                    let charts: Vec<Vec<u8>> = acts.scoped().map(|(k, _)| k).collect();
                    for k in charts {
                        if let Some(a) = acts.get_mut(&k) {
                            a.gauge = compose_gauge(g, &a.gauge);
                        }
                    }
                }
                // A Rebased vertex's fibre regauges the same way its base's
                // does — every member, at its own chart.
                Vertex::Rebased { acts, .. } => {
                    let charts: Vec<Vec<u8>> = acts.scoped().map(|(k, _)| k).collect();
                    for k in charts {
                        if let Some(a) = acts.get_mut(&k) {
                            a.gauge = compose_gauge(g, &a.gauge);
                        }
                    }
                }
                _ => {}
            }
        }
        // EVERY MEMBER PAIR'S HUE MAP. The regauge is a change of listing and
        // it moves the map at every pair of bodies the face relates, not one.
        for face in &mut self.faces {
            let pairs: Vec<Vec<u8>> = face.at.scoped().map(|(k, _)| k).collect();
            for k in pairs {
                if let Some(p) = face.at.get_mut(&k) {
                    p.gauge = compose_gauge(g, &p.gauge);
                }
            }
        }
    }

    pub(crate) fn compose_carriages(&mut self, c: &Versor) {
        for v in &mut self.vertices {
            match v {
                // EVERY MEMBER'S CARRIAGE composes: the placement moves the
                // vertex wherever it acts, on every chart it acts on.
                Vertex::Word { acts, .. } => {
                    let charts: Vec<Vec<u8>> = acts.scoped().map(|(k, _)| k).collect();
                    for k in charts {
                        if let Some(a) = acts.get_mut(&k) {
                            *a.carriage = c.geo(&a.carriage);
                        }
                    }
                }
                Vertex::Rebased { acts, .. } => {
                    let charts: Vec<Vec<u8>> = acts.scoped().map(|(k, _)| k).collect();
                    for k in charts {
                        if let Some(a) = acts.get_mut(&k) {
                            *a.carriage = c.geo(&a.carriage);
                        }
                    }
                }
                _ => {}
            }
        }
        // EVERY MEMBER PAIR'S CARRIAGE, and every MEMBER PATH's around each
        // loop: a change of frame moves the placement wherever there is one.
        for face in &mut self.faces {
            let pairs: Vec<Vec<u8>> = face.at.scoped().map(|(k, _)| k).collect();
            for k in pairs {
                if let Some(p) = face.at.get_mut(&k) {
                    p.carriage = c.geo(&p.carriage);
                }
            }
        }
        for lp in &mut self.loops {
            let paths: Vec<Vec<u8>> = lp.carriages.scoped().map(|(k, _)| k).collect();
            for k in paths {
                if let Some(v) = lp.carriages.get_mut(&k) {
                    *v = c.geo(v);
                }
            }
        }
    }
}

#[cfg(test)]
/// The word's sensor-plane block applied to a point. Named for the block
/// it uses, not for a linearity the engine does not have.
pub(crate) fn apply_plane_block(
    word: &Word,
    t: &(num_rational::BigRational, num_rational::BigRational),
) -> Option<(num_rational::BigRational, num_rational::BigRational)> {
    let b = word.plane_block()?;
    Some((
        &b[0][0] * &t.0 + &b[0][1] * &t.1,
        &b[1][0] * &t.0 + &b[1][1] * &t.1,
    ))
}

pub(crate) fn hue_key(h: u8) -> Vec<u8> {
    vec![h]
}

pub(crate) fn gauge_invert(g: &[(u8, u8)]) -> Vec<(u8, u8)> {
    let mut inv: Vec<(u8, u8)> = g.iter().map(|&(a, b)| (b, a)).collect();
    inv.sort_unstable();
    inv.dedup();
    inv
}

pub(crate) fn gauge_keyed(g: &[(u8, u8)]) -> Keyed<u8> {
    let mut m = Keyed::new();
    for &(a, b) in g {
        m.insert(hue_key(a), b);
    }
    m
}

pub(crate) fn gauge_image(g: &Keyed<u8>, h: u8) -> u8 {
    match g.get(&hue_key(h)) {
        Some(&x) => x,
        None => h,
    }
}

/// THE HUE A GAUGE CARRIES `h` TO, read off the LISTING, or a refusal where
/// the listing names two.
///
/// A gauge is a map. Its listing is a `Vec<(from, to)>` gathered from a
/// chain's cells and deduplicated BY THE PAIR, so a chain in which one input
/// hue lands on two different output hues yields both entries. Two readers
/// scanned that list and returned the first match — `body.rs` on the court
/// and `reading.rs` on the speaker, the same eight lines written twice (law
/// 12) — which is the smaller image after the sort. A hue decided by a Vec's
/// order is a member by position (rule 15), and the decision was invisible:
/// the court read one image, the cell carried the other, and the refusal
/// said only "a cell lands on the wrong hue". That fault is 255 of the 488
/// body refusals on the fleet at e98e8a2, against 0 for `FAIL_NO_VERSOR` and
/// 0 for `FAIL_NO_CARRY` — the algebra was never the problem.
///
/// A listing that names two images for one hue is not a hue map and no
/// vertex can claim it: exact or refused (rule 11). A hue the listing does
/// not name is fixed by it (18.2), which is a fact about the gauge and not a
/// default written here.
pub fn hue_of(gauge: &[(u8, u8)], h: u8) -> Option<u8> {
    // index: the gauge's own listing, its boundary (rule 1)
    let mut held: Option<u8> = None;
    for &(from, to) in gauge {
        if from != h {
            continue;
        }
        match held {
            None => held = Some(to),
            Some(t) if t == to => {}
            Some(_) => return None,
        }
    }
    Some(match held {
        Some(t) => t,
        None => h,
    })
}

/// `second ∘ first` on the union of listed hues; a hue absent from a map
/// is fixed by that map. The listing holds non-fixed points only.
pub(crate) fn compose_gauge(second: &[(u8, u8)], first: &[(u8, u8)]) -> Vec<(u8, u8)> {
    let a = gauge_keyed(first);
    let b = gauge_keyed(second);
    let mut hues: Keyed<()> = Keyed::new();
    for &(x, y) in first.iter().chain(second.iter()) {
        hues.insert(hue_key(x), ());
        hues.insert(hue_key(y), ());
    }
    let mut out = Vec::new();
    // The hues the two gauges name between them, gathered here: the
    // composite is read at every one of them and nowhere else.
    for (k, _) in hues.scoped() {
        let h = k[0];
        let image = gauge_image(&b, gauge_image(&a, h));
        if image != h {
            out.push((h, image));
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

pub(crate) fn relative_gauge(from: &[(u8, u8)], to: &[(u8, u8)]) -> Vec<(u8, u8)> {
    let fm = gauge_keyed(from);
    let tm = gauge_keyed(to);
    let mut hues: Keyed<()> = Keyed::new();
    for &(x, y) in from.iter().chain(to.iter()) {
        hues.insert(hue_key(x), ());
        hues.insert(hue_key(y), ());
    }
    let mut out = Vec::new();
    // The hues the two gauges name between them, gathered here: the
    // composite is read at every one of them and nowhere else.
    for (k, _) in hues.scoped() {
        let h = k[0];
        let b = gauge_image(&fm, h);
        let c = gauge_image(&tm, h);
        if b != c {
            out.push((b, c));
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// THE PLACEMENT A VERTEX HAS AT ONE NAMED CHART. A keyed read: `None` is
/// "this vertex does not act there", and a Rebased vertex's placement is
/// `t · C_by` — the stored relative composed with its base's, which is where
/// `C_of` comes back from (the rebase overwrote the slot it was measured in,
/// so `t` is the only surviving record and law 12 cannot ask it to
/// re-derive).
pub(crate) fn vertex_carriage_at(
    v: &Vertex,
    form: &Form,
    chart: &crate::transducer::stalk::Invariant,
) -> Option<Versor> {
    match v {
        Vertex::Word { .. } => v.acting_on(chart).map(|a| (*a.carriage).clone()),
        // `t · C_by`: the relative this vertex holds AT THIS CHART, composed
        // with the base's placement at the base's own chart. `of`'s slot was
        // overwritten by the rebase, so this is where `C_of` comes back.
        Vertex::Rebased { by, .. } => {
            let relative = &v.acting_on(chart)?.carriage;
            match form.vertices.get(*by as usize) {
                Some(w @ Vertex::Word { .. }) => {
                    let charts = w.charts();
                    // The base stands at ONE chart — `Form::rebase` refuses
                    // otherwise — so the slice pattern reads it and a tape
                    // carrying a multi-member base has no placement here.
                    let [base] = charts.as_slice() else {
                        return None;
                    };
                    w.acting_on(base).map(|a| relative.geo(&a.carriage))
                }
                Some(Vertex::Frame) => Some((**relative).clone()),
                _ => None,
            }
        }
        Vertex::Frame | Vertex::Glued { .. } | Vertex::Unfolded { .. } => {
            Some(element::identity_versor())
        }
        Vertex::Demand { .. } => None,
    }
}

/// THE HUE MAP A VERTEX HAS AT ONE NAMED CHART, empty where it acts on none:
/// a map that moves no hue, not an absence standing in for one.
pub(crate) fn vertex_gauge_at(v: &Vertex, chart: &crate::transducer::stalk::Invariant) -> Vec<(u8, u8)> {
    match v {
        // A Rebased vertex keeps `of`'s hue map at each chart, so both kinds
        // answer from their own fibre.
        Vertex::Word { .. } | Vertex::Rebased { .. } => match v.acting_on(chart) {
            Some(a) => a.gauge.clone(),
            None => Vec::new(),
        },
        _ => Vec::new(),
    }
}

// `vertex_carriage(v, form)` and `vertex_gauge(v)` stood here and read the
// vertex's representative member. They are gone with the last caller: the face
// that needed them now carries a placement PER MEMBER PAIR, so every reader in
// this tree takes the member at the coordinate it is asking about, and there is
// no accessor left through which the collapse can arrive.
