// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 18.2, 21 step 1 — THE TRANSPORT of a face: the relative word, carriage
//! versor and hue map from one vertex to another, and the gauge algebra
//! those coordinates compose and invert by.

use super::form::{Form, Vertex};
use crate::network::nerve::{coord, Keyed};
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
        // OFF THE SENSOR PLANE, WHICH IS WHAT THIS ASKS. It read
        // `gamma.word.form()`, and while a form existed only for words
        // keeping the sensor plane the two questions had one answer. A form
        // is now every plane the word turns, so `Some` no longer implies
        // the sensor is kept, and the precondition this refraction actually
        // has — it rebases and regauges ON the sensor — is the sensor block
        // itself.
        gamma.word.plane_block()?;
        let mut f = self.clone();
        f.rebase_by_word(&gamma.word);
        f.regauge_listing(&gamma.gauge);
        f.compose_carriages(&gamma.carriage);
        f.loops = Form::loops_of(&f.faces, &f.vertices);
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
        // AND THE COVER REBASES WITH THE PORTS IT BONDS TO (54.7 B5a, as
        // measured on the recut at epoch 93).
        //
        // `placement` is part of a citation's SITE — the boundary record
        // `[2][placement][depth][carriage][chart]` that `put_vertex` writes
        // and `NerveEdge::host` asks for. Rebasing the vertex and not the
        // edge moves every port to a new site and leaves every bond naming
        // the old one, which is an orphan port and a phantom edge in one
        // step, for every citation the form holds.
        //
        // It read 10 `REFUSE_ORPHAN_PORT` across the three courts in the
        // first five minutes of the fleet on this layout — the defect was
        // invisible before B5 because the target rode the vertex, so a
        // rebase moved the whole citation at once and there was no second
        // carrier to fall out of step.
        //
        // The order is re-taken because a rebase can reorder the sites.
        for e in &mut self.cover {
            e.placement = e.placement.rebase(by);
        }
        self.cover =
            crate::transducer::element::canonical_nerve(std::mem::take(&mut self.cover));
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
                    // The section keys are unchanged by a regauge, so they
                    // are read once and each member reached by its own key —
                    // `Keyed` has no mutable walk, and this is the pattern
                    // `ReceiptChain::reindex_placed` already uses.
                    //
                    // 54.7 R5 — THEY ARE SECTIONS, NOT CHARTS. This local was
                    // called `charts` and held `chart ‖ carriage`, which is
                    // the same confusion that sent a member key into a chart
                    // index at eight sites. The type now says which it is.
                    let sections: Vec<coord::Section> =
                        acts.entries().map(|(k, _)| k).collect();
                    for k in sections {
                        if let Some(a) = acts.at_mut(&k) {
                            a.gauge = compose_gauge(g, &a.gauge);
                        }
                    }
                }
                // A Rebased vertex's fibre regauges the same way its base's
                // does — every member, at its own section.
                Vertex::Rebased { acts, .. } => {
                    let sections: Vec<coord::Section> =
                        acts.entries().map(|(k, _)| k).collect();
                    for k in sections {
                        if let Some(a) = acts.at_mut(&k) {
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
                    let sections: Vec<coord::Section> =
                        acts.entries().map(|(k, _)| k).collect();
                    for k in sections {
                        if let Some(a) = acts.at_mut(&k) {
                            *a.carriage = c.geo(&a.carriage);
                        }
                    }
                }
                Vertex::Rebased { acts, .. } => {
                    let sections: Vec<coord::Section> =
                        acts.entries().map(|(k, _)| k).collect();
                    for k in sections {
                        if let Some(a) = acts.at_mut(&k) {
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
/// chain's links and deduplicated BY THE PAIR, so a chain in which one
/// input hue lands on two different output hues yields both entries. A hue
/// decided by a Vec's order would be a member by position (rule 15).
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

/// 18.2 — THE GAUGE AS THE MAP IT IS: the ten hues, each carried somewhere
/// or left where it stands.
///
/// A gauge is a partial map on the hues and absence is the identity. Held as
/// a LISTING of `(from, to)` it is the graph of that map written out as
/// points, and every question about it becomes a scan: `hue_of` walks the
/// listing to answer about ONE hue, and two listings are compared by walking
/// one against the other. The hue is the coordinate and the listing has no
/// index at it.
///
/// This is the carrier `connection_chart` and `element::gauge` already use —
/// `let mut image: [u8; 10]` indexed by the hue — so the tree holds one fact
/// two ways (law 12). `None` is the hue nothing moves, kept apart from a hue
/// carried to itself so a map that SAYS `3 → 3` and one that is silent about
/// 3 read alike where it matters and differently where it does not.
pub type HueTable = [Option<u8>; 10];

/// The listing read as the map. `None` where it sends one hue two ways —
/// that is not a hue map and no vertex can claim it (rule 11) — or where a
/// hue stands outside the ten the sensor has.
pub fn hue_table(gauge: &[(u8, u8)]) -> Option<HueTable> {
    let mut t: HueTable = [None; 10];
    for &(from, to) in gauge {
        let slot = t.get_mut(from as usize)?;
        match slot {
            None => *slot = Some(to),
            Some(held) if *held == to => {}
            Some(_) => return None,
        }
    }
    Some(t)
}

/// The map written back as a listing, in hue order. Only the hues it MOVES:
/// a gauge lists its non-fixed points and absence is the identity, so a hue
/// carried to itself is written and one nothing says of is not.
pub fn hue_listing(t: &HueTable) -> Vec<(u8, u8)> {
    (0..10u8)
        .filter_map(|h| t.get(h as usize).and_then(|s| s.map(|to| (h, to))))
        .collect()
}

/// WHAT TWO MEMBERS' GAUGES AGREE ON, or the hue they send two ways.
///
/// Two gauges COMPOSE where their hues do not meet, or where they meet and
/// agree: one member carrying both listings speaks for both, which is what
/// 18.2's partial map means and what the union was written for. Where they
/// meet and DISAGREE they are not one assignment at all — they are two, and
/// the class varies across the bundle exactly as it does when the carriage
/// differs. That is the boundary the extractor already states for the
/// placement: variation across pairs is a SECTION.
#[derive(Clone, Debug)]
pub enum HueAgreement {
    /// One map speaks for both.
    Agreed(Box<HueTable>),
    /// One hue, carried two ways. The coordinate and both answers, so the
    /// caller states what it found rather than that something went wrong.
    Apart { hue: u8, here: u8, there: u8 },
}

/// Read at each hue in turn — the map's own coordinate — so a disagreement
/// is found where it is rather than by comparing two listings against each
/// other.
pub fn hue_agreement(a: &HueTable, b: &HueTable) -> HueAgreement {
    let mut out: HueTable = [None; 10];
    for h in 0..10usize {
        out[h] = match (a[h], b[h]) {
            (None, x) | (x, None) => x,
            (Some(x), Some(y)) if x == y => Some(x),
            (Some(x), Some(y)) => {
                return HueAgreement::Apart { hue: h as u8, here: x, there: y }
            }
        };
    }
    HueAgreement::Agreed(Box::new(out))
}

/// Do two listings name one map? Read through the tables, so two orderings
/// of one map are one answer and a listing that sends a hue two ways is no
/// map on either side (rule 11).
pub fn gauge_eq(a: &[(u8, u8)], b: &[(u8, u8)]) -> bool {
    match (hue_table(a), hue_table(b)) {
        (Some(x), Some(y)) => x == y,
        _ => false,
    }
}

/// Do two listings agree where they meet? True if neither sends any hue to
/// two different images.
pub fn gauge_agrees(a: &[(u8, u8)], b: &[(u8, u8)]) -> bool {
    match (hue_table(a), hue_table(b)) {
        (Some(x), Some(y)) => matches!(hue_agreement(&x, &y), HueAgreement::Agreed(_)),
        _ => false,
    }
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
        // φ — THE MORPHISM THE CITED FORM ACTS UNDER HERE (54.2). It
        // answered `identity_versor()` for every Glued vertex, so the
        // placement face carried the identity and the embedding was not in
        // the key at all.
        Vertex::Glued { carriage, .. } => Some((**carriage).clone()),
        Vertex::Frame | Vertex::Unfolded { .. } => Some(element::identity_versor()),
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
