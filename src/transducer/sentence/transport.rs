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
    /// BQ5a — WHAT A CONTACT'S EDGE HALF HOLDS WHERE NO SINGLE TRANSPORT
    /// FORMED (78.6): the held coset, or the tether at one frame. `None` on
    /// every face that is not a contact, and on a contact whose edge formed
    /// or was refused by name. In memory only, like the form's cells; the
    /// tape at the recut (BQ5b).
    pub refraction: Option<Refraction>,
}

/// BQ5a — A CONTACT EDGE THAT IS NOT ONE TRANSPORT, by what it holds (78.6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refraction {
    /// 78.6(1) — A HELD STABILIZER RIDES THE EDGE AS A COSET: one transport
    /// per candidate of the bodies' held poses, `(θ_bθ_a⁻¹)·V` for each,
    /// never a representative (checklist (r)).
    Coset(Vec<Transport>),
    /// 78.6(2) — A PIECE AT ONE FRAME: the tether `V = s_b s_a` between the
    /// face's two BODIES is content there and is carried; the refraction
    /// `θ_bθ_a⁻¹` is content across a transition the piece does not have,
    /// so it is unread BY NAME — the gauge half reads `static`, never the
    /// identity and never the union's rigidity (rule 11, checklist (m)).
    /// 78.18(a) — each end's pose as far as it was read: a member that did
    /// transit keeps its pose on the edge by name, the one at one frame is
    /// named so, and a later fibre can complete the refraction.
    Static { tether: Versor, poses: [HeldPose; 2] },
}

/// 78.18(a) — ONE END'S POSE ON A STATIC EDGE, by what was read of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeldPose {
    /// The member stands at one frame: no transition to read a pose across.
    OneFrame,
    /// The member transited: its pose's candidates (one for a turn, the held
    /// coset's for a held stabilizer).
    Read(Vec<Versor>),
    /// The member transited and its pose refused, by name.
    Refused(crate::transducer::blade::NoPose),
    /// 78.107 — NOT READ AT THIS VANTAGE: the face crossed the wire carrying
    /// its interface, the tether `V`, and not the bodies a pose is read off;
    /// a court reading the contact statically by `V` names its poses so.
    Unread,
}

// `face_pair_key` AND `read_face_pair_key` STOOD HERE (a face's member pair
// as two chart keys) AND ARE DELETED (AT1): a face IS its overlap, keyed by
// its two charts' coordinates, and holds one pair.

impl Transport {
    pub fn from_word(word: Word) -> Transport {
        Transport {
            word,
            carriage: element::identity_versor(),
            gauge: Vec::new(),
        }
    }

    /// THE TRANSPORT READ BACKWARD (78.32(1)): the free involution on each
    /// of its factors — the word's inverse on its letters, the carriage's
    /// reverse (a ray, read by proportion wherever it is read), the gauge's
    /// inverse permutation. Never an inversion: nothing is divided. It was
    /// `inverse`, dividing the carriage by its weight; a transport's
    /// carriage was admitted as a versor when the transport was made.
    pub fn reverse(&self) -> Transport {
        Transport {
            word: self.word.inverse(),
            carriage: self.carriage.reverse(),
            gauge: gauge_invert(&self.gauge),
        }
    }

    /// Move `form` into this transport's frame (3.10): rebase by word (3.2),
    /// regauge (3.6), compose every carriage. `None` off the plane.
    pub fn refract(&self, form: &Form) -> Option<Form> {
        form.refract(self)
    }
}

/// 78.60 — WHAT RELATES A HELD FORM TO ANOTHER, read exactly: whether the
/// other can be RELAYED as (held, Γ) and refracted by its receiver.
#[derive(Clone, Debug)]
pub enum Relation {
    /// `held.refract(Γ)` is the other form, address for address (T1): it
    /// travels as the held word's address and Γ.
    Refracted(Transport),
    /// One word the alphabet spells conjugates every word vertex, and the
    /// refraction by it with the carriage and gauge read off the members
    /// does not land on the other — the placement differs from vertex to
    /// vertex, or a citation does: the word half of a refraction.
    Conjugate(Word),
    /// One rotor conjugates every word vertex and the alphabet cannot
    /// spell it: the measured obstruction a generator is made of (37.7).
    Unspellable(crate::transducer::alphabet::Rotor),
    /// No one rotor conjugates them, or they stand on no common vertex set.
    Unrelated,
}

/// 78.60 — SOLVE Γ WITH `held.refract(Γ) = other`, never by search. A
/// refraction conjugates every word vertex by one rotor and keeps its
/// coordinate (a word vertex is keyed by its class, which conjugation
/// keeps), so the word vertices pair by coordinate and ONE γ is solved for
/// all of them at once (`simultaneous_conjugators`, one elimination over
/// ℚ). A citation moves under the rebase and is not paired; the refraction
/// by each spelled γ is then taken and its address compared with the
/// other's — the proof, whatever was paired.
pub fn relation(held: &Form, other: &Form) -> Relation {
    use crate::transducer::alphabet::bundle::simultaneous_conjugators;
    if held.vertices.len() != other.vertices.len() || held.faces.len() != other.faces.len() {
        return Relation::Unrelated;
    }
    let mut pairs = Vec::new();
    for (at, va) in held.vertices.entries() {
        if !matches!(at, coord::VertexAt::Word(_)) {
            continue;
        }
        let Some(vb) = other.vertices.at(&at) else { return Relation::Unrelated };
        let (Some(wa), Some(wb)) = (va.word_in(&held.vertices), vb.word_in(&other.vertices)) else {
            return Relation::Unrelated;
        };
        pairs.push((wa.rotor(), wb.rotor()));
    }
    // Zero is not a word (`as_element_form`); the address is the form's,
    // whatever unit magnitude it is minted at.
    let address = |f: &Form| super::as_element_form(f, [0, 0, 0, 1]).map(|e| e.address());
    let Some(target) = address(other) else { return Relation::Unrelated };
    // THE PLACEMENT AND THE HUES OF Γ, read off one word vertex acting on
    // one chart in both forms: a refraction composes Γ's carriage and gauge
    // into every member (`compose_carriages`, `regauge_listing`) and the
    // rebase touches neither, so `c_other = Γc·c_held` and
    // `g_other = Γg ∘ g_held` there, exactly. The first such vertex in
    // coordinate order is read; the address comparison below is the proof.
    let placed = held.vertices.entries().find_map(|(at, va)| {
        let (Vertex::Word { acts: aa, .. }, Some(Vertex::Word { acts: ab, .. })) = (va, other.vertices.at(&at)) else {
            return None;
        };
        let (ea, eb): (Vec<_>, Vec<_>) = (aa.entries().collect(), ab.entries().collect());
        match (ea.as_slice(), eb.as_slice()) {
            ([(_, ma)], [(_, mb)]) => Some((mb.carriage.geo(&ma.carriage.reverse()), gauge_after(&ma.gauge, &mb.gauge)?)),
            _ => None,
        }
    });
    let mut read = Relation::Unrelated;
    for g in simultaneous_conjugators(&pairs) {
        match crate::transducer::alphabet::factor_rotor(&g) {
            Some(Ok(w)) => {
                let gamma = match &placed {
                    Some((carriage, gauge)) => Transport { word: w.clone(), carriage: carriage.clone(), gauge: gauge.clone() },
                    None => Transport::from_word(w.clone()),
                };
                if held.refract(&gamma).and_then(|f| address(&f)).is_some_and(|a| a == target) {
                    return Relation::Refracted(gamma);
                }
                read = Relation::Conjugate(w);
            }
            _ => {
                if matches!(read, Relation::Unrelated) {
                    read = Relation::Unspellable(g);
                }
            }
        }
    }
    read
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
        f.orbits = Form::orbits_of(&f.faces, &f.vertices);
        f.reindex();
        Some(f)
    }

    pub(crate) fn rebase_by_word(&mut self, by: &Word) {
        // A CONJUGATION KEEPS EVERY CLASS (15.3), so a Word vertex stays at
        // its coordinate; a Glued vertex's site is its record, which the
        // placement is part of, so a citation MOVES and takes its overlaps
        // with it (AT1).
        let keys: Vec<coord::VertexAt> = self.vertices.entries().map(|(k, _)| k).collect();
        let mut moved: Vec<(coord::VertexAt, coord::VertexAt)> = Vec::new();
        for k in keys {
            let Some(v) = self.vertices.at_mut(&k) else { continue };
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
                Vertex::Glued { placement, .. } => {
                    *placement = placement.rebase(by);
                    if let Some(to) = v.at().filter(|to| *to != k) {
                        moved.push((k, to));
                    }
                }
                _ => {}
            }
        }
        for (from, to) in moved {
            if let Some(held) = self.vertices.drop_at(&from) {
                self.vertices.put(&to, held);
                self.rekey_vertex(&from, &to);
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
        // first five minutes of the deployment on this layout — the defect was
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
        // A FACE'S WORD IS DERIVED from its ends (AT1), which were rebased
        // above; what a face holds of its own — the pivot's factors — is
        // conjugated the same way.
        let overlaps: Vec<coord::Overlap> = self.faces.entries().map(|(k, _)| k).collect();
        for o in overlaps {
            if let Some(f) = self.faces.at_mut(&o) {
                f.left = f.left.rebase(by);
                f.right = f.right.rebase(by);
            }
        }
    }

    pub(crate) fn regauge_listing(&mut self, g: &[(u8, u8)]) {
        let keys: Vec<coord::VertexAt> = self.vertices.entries().map(|(k, _)| k).collect();
        for k in keys {
            let Some(v) = self.vertices.at_mut(&k) else { continue };
            match v {
                // EVERY MEMBER IS REGAUGED. A word vertex acting on several
                // charts composes the map into the gauge at each chart: one
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
        // EVERY OVERLAP'S HUE MAP. The regauge is a change of listing and it
        // moves the map at every pair of bodies, one overlap each.
        let overlaps: Vec<coord::Overlap> = self.faces.entries().map(|(k, _)| k).collect();
        for o in overlaps {
            if let Some(f) = self.faces.at_mut(&o) {
                f.place.gauge = compose_gauge(g, &f.place.gauge);
            }
        }
    }

    pub(crate) fn compose_carriages(&mut self, c: &Versor) {
        let keys: Vec<coord::VertexAt> = self.vertices.entries().map(|(k, _)| k).collect();
        for k in keys {
            let Some(v) = self.vertices.at_mut(&k) else { continue };
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
        // EVERY OVERLAP'S CARRIAGE: a change of frame moves the placement
        // wherever there is one. An orbit's carriage is read off its
        // overlaps (AT1) and follows by construction.
        let overlaps: Vec<coord::Overlap> = self.faces.entries().map(|(k, _)| k).collect();
        for o in overlaps {
            if let Some(f) = self.faces.at_mut(&o) {
                f.place.carriage = c.geo(&f.place.carriage);
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

/// 78.60 — THE HUE MAP Γg WITH `Γg ∘ first = second`, solved on first's
/// image: a member's hue map is a fibre relation that need not be
/// injective (a merge records into e_∅, 78.58), so `second ∘ first⁻¹` is
/// no identity even where Γ moved no hue. Each hue h either map names
/// sets `Γg(first(h)) = second(h)`; two hues first joins that second sends
/// apart refuse (`None`: no one hue map carries first onto second). Fixed
/// points are the identity's and are not written.
pub(crate) fn gauge_after(first: &[(u8, u8)], second: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let (a, b) = (gauge_keyed(first), gauge_keyed(second));
    let mut hues: Keyed<()> = Keyed::new();
    for &(x, _) in first.iter().chain(second.iter()) {
        hues.insert(hue_key(x), ());
    }
    let mut out: Keyed<u8> = Keyed::new();
    for (k, _) in hues.scoped() {
        let h = k[0];
        let (from, to) = (gauge_image(&a, h), gauge_image(&b, h));
        match out.get(&hue_key(from)) {
            Some(&t) if t != to => return None,
            _ => {
                out.insert(hue_key(from), to);
            }
        }
    }
    Some(out.scoped().map(|(k, &t)| (k[0], t)).filter(|&(h, t)| h != t).collect())
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
/// points, and every question about it becomes a scan: `hue_of` reads the whole
/// listing to answer about ONE hue, and two listings are compared by reading
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
/// Two gauges COMPOSE where the two gauges' hues do not meet, or where they meet and
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
    #[expect(
        clippy::cast_possible_truncation,
        reason = "STD1 C1 — the sensor's: a hue below PALETTE = 10, the HueTable's ten slots"
    )]
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
            match form.vertices.at(by) {
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
        // A Rebased vertex keeps `of`'s hue map at each chart, so each kind
        // answers from its own fibre.
        Vertex::Word { .. } | Vertex::Rebased { .. } => match v.acting_on(chart) {
            Some(a) => a.gauge.clone(),
            None => Vec::new(),
        },
        _ => Vec::new(),
    }
}
