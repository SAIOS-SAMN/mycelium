// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 20, 21, 52.5 — THE MOVES ON A FAMILY OF TRAIN PAIRS: the family's frames
//! perceived once, its one form read as a quotient per chart, and that
//! form spoken back onto a scene. A family it does not close is a withheld
//! form.

mod atlas;
pub mod bridge;
mod chord;
mod comprehend;
pub use comprehend::{take_cells, take_progress, take_spend, take_viscosity};
pub(crate) use comprehend::pivot_cells;
pub mod contact;
pub mod correspondence;
pub mod cover;
pub mod covered;
pub mod cursor;
pub mod deflation;
pub mod derivation;
pub mod extract;
mod fiber;
pub mod fibres;
pub mod flux;
pub mod hyperfluid;
pub mod incidence;
pub mod finish;
pub mod confluence;
pub mod incongruence;
pub mod interpret;
pub mod invert;
pub mod luminal;
pub mod memo;
pub mod orbit;
pub mod pair;
mod percept;
pub(crate) mod percept_reading;
pub(crate) use comprehend::take_percepts;
pub mod perceive;
pub mod pairing;
pub mod rebase;
pub mod reconstruct;
pub mod refine;
pub mod scene;
pub mod pieces;
pub mod span;
pub mod speak;
pub mod trace;
mod witness;

pub(crate) use correspondence::*;
pub use deflation::*;
pub use derivation::*;
pub use finish::extraction_spend;

/// ONE TRAIN PAIR — the scene a rule is shown and the scene it must speak
/// (38). The grids are the corpus's own carrier; the pair is what a rule is
/// extracted from and replayed against.
pub type TrainPair = (perceive::Grid, perceive::Grid);

/// THE FAMILY a rule is extracted from: every train pair of one task. A
/// rule speaks the family or it is not a rule (21).
pub type Family = [TrainPair];
pub use extract::*;
pub(crate) use incidence::*;
pub use memo::*;
pub use orbit::*;
pub use pairing::*;
pub(crate) use rebase::*;
pub use perceive::*;
pub use scene::*;
pub use speak::*;

use crate::network::nerve::{coord, Coordinate};

use super::blade::{self, Shape};
use super::element::{self, Element, Versor};
use super::base::Site;
use super::sentence::{Form, Link, Vertex};
use super::stalk::{Invariant, Standing};
use std::cell::Cell;

/// Source of crystallized forms on a class. Standing is a coordinate on
/// the working atlas, never a gate.
pub trait Crystal {
    /// 24.2: the closed forms held at a CITED FORM's chart (`chart_of_form`),
    /// which is what comprehension of a citation asks for.
    fn on_class(&self, class: &Invariant) -> Vec<(&Element, &Form, Standing)>;
    /// 21 step 3: the closed forms carrying a body on a BODY's chart (its
    /// moduli), which is what the glue asks for. A form is filed under
    /// `chart_of_form` and a body under its moduli, two coordinate systems
    /// that meet only at the empty root, so one reader cannot answer both.
    fn on_body(&self, chart: &Invariant) -> Vec<(&Element, &Form, Standing)>;
    fn speak_form(&self, f: &Form, x: &Grid, placement: &super::alphabet::Word) -> Option<Speech>;
    /// Keyed read of a crystallized form by address. No scan, AND NO CLONE
    /// (Z2 item 2, rule 17): a reader hands back the stratum it holds, and
    /// a caller that resolves one citation 578 times must not pay a form
    /// copy for each. The borrow is the crystal's own and it outlives no
    /// call that does not hold the crystal. A word the crystal holds without
    /// its form — a court's cavity — has its form DERIVED from its address
    /// where it is read, never stored beside it: that arm is owned.
    fn by_address(&self, addr: &[u8]) -> Option<(&Element, std::borrow::Cow<'_, Form>)>;
    /// Held words on `chart` whose class vector occupies `axis` (12.6): the
    /// vocabulary a pivot may write onto an orbit on that chart. One keyed read
    /// of the axis index the crystal filled when the word was admitted.
    fn on_axis(&self, chart: &Invariant, axis: super::sentence::Axis) -> Vec<super::alphabet::Word>;
    /// 1.8, 50.2 step 2 — THE PRESENTATION ON A CHART: the generators
    /// this crystal holds on `chart`, each as the cited form's address,
    /// the address of the `Word` vertex inside it, and that vertex's own
    /// word. A keyed subtree read of the generator index, spending the
    /// budget it is given (rule 10); a crystal that holds nothing on the
    /// chart answers empty, which is a presentation of no generators and
    /// not a refusal.
    fn generators_under(
        &self,
        chart: &Invariant,
        budget: &mut crate::physics::Budget,
    ) -> Vec<(
        crate::transducer::address::Address,
        crate::transducer::address::Address,
        super::alphabet::Word,
    )>;

    /// 42.6 AS AMENDED, L12 — THE PRESENTATION ON ONE LANE. The cover of
    /// the language is the product of four, and a reduction is bounded by
    /// a presentation only when it runs against the lane's own basis:
    /// reducing a target that occupies `𝒢` against a generator occupying
    /// only `𝒽` is a basis no lane has.
    ///
    /// The default answers the `𝒽` cover for lane 3 and nothing for the
    /// others, which is what a crystal that has not split its index holds:
    /// `versor_chart` IS the holonomic lane's chart, so lane 3 was never
    /// missing — the other three were, and answering them from it would
    /// be the union this row exists to refuse.
    fn generators_on_lane(
        &self,
        lane: usize,
        chart: &Invariant,
        budget: &mut crate::physics::Budget,
    ) -> Vec<(
        crate::transducer::address::Address,
        crate::transducer::address::Address,
        super::alphabet::Word,
    )> {
        match lane {
            3 => self.generators_under(chart, budget),
            _ => Vec::new(),
        }
    }

    /// 42.6 AS AMENDED, L12 — THE PRESENTATION AT THE MEET OF THE LANES A
    /// TARGET OCCUPIES. `occupied` is one entry per lane the target carries
    /// content on, with that lane's own chart (`lane_charts_of`).
    ///
    /// THE MEET, NEVER THE UNION. A target occupying `𝒮` and `𝒽` is not
    /// reducible against a generator that lives only on `𝒽`: dividing by it
    /// leaves the `𝒮` content untouched and asserts a bound the presentation
    /// does not give. It is reducible against a generator that lies in BOTH
    /// covers, because that generator moves both coordinates at once, which
    /// is what a single group element does. So a generator enters the basis
    /// when it is held on EVERY occupied lane, and is dropped when any lane
    /// does not hold it.
    ///
    /// FOUR FLAGS, NOT A COUNT. Membership is read per lane and the four
    /// readings are never summed (law 5). A count of lanes hit compared
    /// against a length would be a number deciding a branch (rule 2), and
    /// it cannot tell a generator held twice on one lane from one held on
    /// two — which is the exact confusion the four covers exist to prevent.
    ///
    /// A target occupying NO lane gets no basis. The meet over an empty
    /// family is formally everything, and answering everything here is the
    /// union this row exists to refuse; a target on no lane is the identity
    /// and has nothing to reduce.
    fn generators_at_meet(
        &self,
        occupied: &[(usize, Invariant)],
        budget: &mut crate::physics::Budget,
    ) -> Vec<(
        crate::transducer::address::Address,
        crate::transducer::address::Address,
        super::alphabet::Word,
    )> {
        if occupied.is_empty() {
            return Vec::new();
        }
        let mut lanes_hit: crate::network::nerve::Keyed<[bool; 4]> =
            crate::network::nerve::Keyed::new();
        let mut held: crate::network::nerve::Keyed<(
            crate::transducer::address::Address,
            crate::transducer::address::Address,
            super::alphabet::Word,
        )> = crate::network::nerve::Keyed::new();
        for (lane, chart) in occupied {
            for (form, vertex, w) in self.generators_on_lane(*lane, chart, budget) {
                let k = crate::transducer::sentence::generator_key(&form, &vertex);
                match lanes_hit.get_mut(&k) {
                    Some(flags) => {
                        if let Some(slot) = flags.get_mut(*lane) {
                            *slot = true;
                        }
                    }
                    None => {
                        let mut flags = [false; 4];
                        if let Some(slot) = flags.get_mut(*lane) {
                            *slot = true;
                        }
                        lanes_hit.insert(&k, flags);
                    }
                }
                held.insert(k, (form, vertex, w));
            }
        }
        let mut out = Vec::new();
        for (k, g) in held.scoped() {
            let Some(flags) = lanes_hit.get(&k) else { continue };
            let on_every = occupied
                .iter()
                .all(|(lane, _)| matches!(flags.get(*lane), Some(true)));
            if on_every {
                out.push(g.clone());
            }
        }
        out
    }
}

/// A crystal that holds nothing. Glue is a no-op.
pub struct NoCrystal;

impl Crystal for NoCrystal {
    fn on_class(&self, _class: &Invariant) -> Vec<(&Element, &Form, Standing)> {
        Vec::new()
    }
    fn on_body(&self, _chart: &Invariant) -> Vec<(&Element, &Form, Standing)> {
        Vec::new()
    }
    fn speak_form(&self, f: &Form, x: &Grid, placement: &super::alphabet::Word) -> Option<Speech> {
        apply_form(f, x, placement, self)
    }
    fn by_address(&self, _addr: &[u8]) -> Option<(&Element, std::borrow::Cow<'_, Form>)> {
        None
    }
    fn on_axis(&self, _chart: &Invariant, _axis: super::sentence::Axis) -> Vec<super::alphabet::Word> {
        Vec::new()
    }
    fn generators_under(
        &self,
        _chart: &Invariant,
        _budget: &mut crate::physics::Budget,
    ) -> Vec<(
        crate::transducer::address::Address,
        crate::transducer::address::Address,
        super::alphabet::Word,
    )> {
        Vec::new()
    }
}

/// THE CIPHER'S FORM OF ONE SCENE: derived against NO presentation, under
/// a stroke that reads the whole scene — the reading a mind that holds
/// nothing gets (50.2 step 2). Named so a reviewer sees at the call site
/// that no speaker's language is in the form — every test whose claim is
/// about perception rather than about what a mind holds builds its fixture
/// here, and a test about the language passes its own crystal to
/// `scene_form`.
///
/// IT IS A TEST FIXTURE AND NOT MOVE 1. It was `one_pair_form`, and 52.5
/// deletes `pair_form` per pair — the per-pair extraction arm, which is
/// gone: this calls `scene_form`, which calls `extract_rule_with` on a
/// one-pair family, the same production path every other caller takes. The
/// old name read as the deleted arm and was what P5's own DoD grep
/// (`pair_form(`) was guarding against finding. What it supplies over
/// `scene_form` is the crystal that holds nothing and the stroke, which is
/// a fixture's job.
#[cfg(test)]
pub fn cipher_form(a: &Grid, b: &Grid) -> Form {
    let pair = [(a.clone(), b.clone())];
    let frames = perceive_family(&pair);
    let mut budget = crate::physics::Physics::default().stroke_n(extraction_spend(&pair, &frames) << 4);
    // A crystal that holds nothing has nothing to factor through, so the
    // factorisation is measured the moment it starts and cannot end on
    // the budget: this expect is the absence of a presentation, not an
    // unwrap of one that might be there.
    scene_form_on(&NoCrystal, &mut budget, a, b, &frames).expect("a presentation of no generators measures at once")
}

/// The scene frame as a form vertex.
pub fn frame_vertex() -> Vertex {
    Vertex::Frame
}

/// The key of a site of one section — a grid's or a chain's, the sensor's
/// planar token (16.5). One producer with `sentence::section_site_key` (law
/// 12): the section's site at frame `0`, keyed by `Site::key`; a site of
/// the base is keyed by `Site::key` directly.
pub(crate) fn site_key(r: i64, c: i64) -> Vec<u8> {
    crate::transducer::sentence::section_site_key(r, c)
}

/// A MEMBER OF THE SCENE'S COMPLEX: a vertex and the chart it stands at
/// there (D10). The supports a pairing reads are keyed by this, not by the
/// vertex alone — a face exists because two BODIES' supports meet (21 step 2),
/// and a vertex covering several bodies on several charts is several occupants
/// of the sites they sit on.
///
/// Both halves are length-delimited so they read apart, and the vertex leads so
/// the key order is the vertex order, which is content. AT1 — THE VERTEX BY
/// ITS COORDINATE, never by its place in a list.
pub(crate) fn member_key(vertex: &crate::network::nerve::coord::VertexAt, chart: &[u8]) -> Vec<u8> {
    use crate::network::nerve::Coordinate;
    let mut k = Vec::new();
    crate::transducer::address::put_key_bytes(&mut k, vertex.encode().as_bytes());
    crate::transducer::address::put_key_bytes(&mut k, chart);
    k
}

// `section_key` (member ‖ carriage, 54.7) STOOD HERE AND RETIRED WITH THE
// SLOTS (AT1, ruled (a) on the chords): two pieces of one member are told
// apart by their PIECE-CHARTS — nucleus and shape at the member's frame —
// and two sections of one chart by the frames each holds, so no key widened
// by the carriage names either.

pub(crate) fn read_member_key(k: &[u8]) -> Option<(crate::network::nerve::coord::VertexAt, Vec<u8>)> {
    use crate::network::nerve::Coordinate;
    let mut p = 0usize;
    let v = crate::transducer::address::take_key_bytes(k, &mut p)?;
    let chart = crate::transducer::address::take_key_bytes(k, &mut p)?;
    Some((crate::network::nerve::coord::VertexAt::decode(&v).ok()?, chart))
}

/// The unordered pair of two members, and the two read back out.
pub(crate) fn member_pair_key(a: &[u8], b: &[u8]) -> Vec<u8> {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    let mut k = Vec::new();
    crate::transducer::address::put_key_bytes(&mut k, lo);
    crate::transducer::address::put_key_bytes(&mut k, hi);
    k
}

pub(crate) fn read_member_pair_key(k: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    let mut p = 0usize;
    let a = crate::transducer::address::take_key_bytes(k, &mut p)?;
    let b = crate::transducer::address::take_key_bytes(k, &mut p)?;
    Some((a, b))
}

/// The family's frame ratio ((h_num, h_den), (w_num, w_den)), identical
/// across pairs or refused, measured on the common refinement.
pub fn family_ratio(family: &[(Grid, Grid)]) -> Option<((usize, usize), (usize, usize))> {
    refine::family_ratio(family)
}

/// 78.53 — THE FAMILY'S FIBRE RELATION: the section transition the colour
/// fibre undergoes where the base stands still. At every site that stands
/// in both the input and the output of a train pair (neither the void),
/// the hue there goes to the hue there; over every pair, that is one map or
/// none, lifted unitarily (78.58, `element::lift_fibre`): a hue standing
/// still is no move and is not written, but it keeps its target, and every
/// other hue merged into a target rotates its distinction into the
/// reservoir. `None` where the map is not a function (a hue sent two ways),
/// where it moves no hue, or where no site stands in both scenes. Read in
/// the detector's own channels
/// — an absolute boundary (78.56 II) — so the colour words minds share name
/// the same colours for every mind.
///
/// The ONE reader of it: the mind mints its colour word from it and the
/// court re-derives it from the grids it holds (`element::colour_word`),
/// so the word stands by evidence and never by a count of who said it.
pub fn fibre_relation(family: &[TrainPair]) -> Option<Vec<(u8, u8)>> {
    let mut map: Vec<(u8, u8)> = Vec::new();
    for (x, y) in family {
        // 78.56 — WHERE THE BASE STANDS STILL across a change of extent:
        // each input site at the output site the envelopes' dilation
        // carries it to, never aligned at a corner. A pair whose dilation
        // is irrational has no site that corresponds, and witnesses nothing.
        let Some(s) = crate::arc::envelope_dilation(x, y) else { continue };
        for (site, h) in crate::arc::painted_sites(x) {
            let Some(there) = crate::arc::carried_site(x, y, s, site) else { continue };
            let t = crate::arc::hue_at(y, there);
            if t == 0 {
                continue;
            }
            match map.iter().find(|(f, _)| *f == h) {
                Some(&(_, seen)) if seen != t => return None,
                Some(_) => {}
                None => map.push((h, t)),
            }
        }
    }
    // 78.58 — the function the scenes showed, a hue standing still
    // included, lifted unitarily: a merge rotates its distinction into
    // the reservoir. Dropping the still hues first read a hue recoloured
    // into one that stayed as an injection, and minted a merge as if
    // nothing were lost.
    crate::transducer::element::lift_fibre(&map)
}

/// One body carried onto another: the transport and the gauge map.
#[derive(Clone, Debug)]
pub struct Carriage {
    pub v: Versor,
    pub sigma: Vec<(u8, u8)>,
}

/// Carry `a` onto `b`: the versor from the moments, and the hue map read
/// compartment by compartment on the base `(r, c, k)` — functional, or refused. One
/// reading for a body in one frame and a body across frames: a frame-0
/// body's compartments stand at `k = 0` (51.3), which is their coordinate and
/// not a second path.
pub fn carriage(a: &Body, b: &Body, budget: &mut crate::physics::Budget) -> Option<Carriage> {
    let v = blade::versor_between(&a.shape, &b.shape, budget).ok()?;
    let mut sigma: crate::network::nerve::Keyed<u8> = crate::network::nerve::Keyed::new();
    for l in &a.compartments {
        let h2 = landed_hue(&v, &l.site, b)?;
        let key = super::element::hue_pair_key(l.hue, 0);
        match sigma.get(&key) {
            Some(t) if *t != h2 => return None,
            Some(_) => {}
            None => {
                sigma.insert(key, h2);
            }
        }
    }
    let mut out: Vec<(u8, u8)> = sigma
        .scoped()
        .filter_map(|(k, t)| super::element::hue_pair_of(&k).map(|(h, _)| (h, *t)))
        .collect();
    out.sort_unstable();
    Some(Carriage { v, sigma: out })
}

/// Carry `a` onto `b`: directly via `carriage`, or by factoring a
/// planar projection artifact with `Surgery::factor` to align the homeomorphic core.
/// The continuous versor is preserved; the tape remains an invertible group action.
pub fn carriage_with_core(a: &Body, b: &Body, budget: &mut crate::physics::Budget) -> Option<Carriage> {
    if let Some(car) = carriage(a, b, budget) {
        return Some(car);
    }
    let (surg, v) = crate::transducer::blade::surgery::Surgery::factor(&a.shape, &b.shape, budget)?;
    match surg.kind {
        crate::transducer::blade::surgery::SurgeryKind::FillHole
        | crate::transducer::blade::surgery::SurgeryKind::Puncture
        | crate::transducer::blade::surgery::SurgeryKind::BridgeMerge
        | crate::transducer::blade::surgery::SurgeryKind::SplitCut => {}
    }
    let mut sigma: crate::network::nerve::Keyed<u8> = crate::network::nerve::Keyed::new();
    for l in &a.compartments {
        let Some(h2) = landed_hue(&v, &l.site, b) else { continue };
        let k = super::element::hue_pair_key(l.hue, 0);
        match sigma.get(&k) {
            Some(t) if *t != h2 => return None,
            Some(_) => {}
            None => {
                sigma.insert(k, h2);
            }
        }
    }
    let mut out: Vec<(u8, u8)> = sigma
        .scoped()
        .filter_map(|(k, t)| super::element::hue_pair_of(&k).map(|(h, _)| (h, *t)))
        .collect();
    out.sort_unstable();
    Some(Carriage { v, sigma: out })
}

#[cfg(test)]
mod tests;
