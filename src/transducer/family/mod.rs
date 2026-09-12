// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 20, 21 — THE MOVES ON A BUNDLE OF TRAIN PAIRS: scenes perceived as pair
//! forms, the rule those forms already agree on, and that rule spoken back
//! onto a scene. A bundle it does not close is a withheld form.

pub mod correspondence;
pub mod cover;
pub mod cursor;
pub mod derivation;
pub mod descent;
pub mod extract;
pub mod memo;
pub mod orbit;
pub mod pair;
pub mod perceive;
pub mod pairing;
pub mod pieces;
pub mod speak;
pub mod trace;

pub(crate) use correspondence::*;
pub use derivation::*;
pub use descent::*;

/// ONE TRAIN PAIR — the scene a rule is shown and the scene it must speak
/// (38). The grids are the corpus's own carrier; the pair is what a rule is
/// extracted from and replayed against.
pub type TrainPair = (perceive::Grid, perceive::Grid);

/// THE FAMILY a rule is extracted from: every train pair of one task. A
/// rule speaks the family or it is not a rule (21).
pub type Family = [TrainPair];
pub use extract::*;
pub use memo::*;
pub use orbit::*;
pub(crate) use pair::*;
pub use pairing::*;
pub use perceive::*;
pub use speak::*;

use super::blade::{self, Shape};
use super::element::{self, Element, Versor};
use super::sentence::{Form, FormFace, Vertex};
use super::stalk::{Invariant, Standing};
use std::cell::Cell;

/// Source of crystallized forms on a class. Standing is a coordinate on
/// the working atlas, never a gate.
pub trait Crystal {
    /// 24.2: the closed forms held at a CITED FORM's chart (`chart_of_form`),
    /// which is what comprehension of a citation asks for.
    fn on_class(&self, class: &Invariant) -> Vec<(Element, Form, Standing)>;
    /// 21 step 3: the closed forms carrying a body on a BODY's chart (its
    /// moduli), which is what the glue asks for. A form is filed under
    /// `chart_of_form` and a body under its moduli, two coordinate systems
    /// that meet only at the empty root, so one reader cannot answer both.
    fn on_body(&self, chart: &Invariant) -> Vec<(Element, Form, Standing)>;
    fn speak_form(&self, f: &Form, x: &Grid, placement: &super::alphabet::Word) -> Option<Speech>;
    /// Keyed read of a crystallized form by address. No scan.
    fn by_address(&self, addr: &[u8]) -> Option<(Element, Form)>;
    /// Held words on `chart` whose class vector occupies `axis` (12.6): the
    /// vocabulary a pivot may write onto a loop on that chart. One keyed read
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
        for (k, g) in held.fiber(crate::network::nerve::ROOT) {
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
    fn on_class(&self, _class: &Invariant) -> Vec<(Element, Form, Standing)> {
        Vec::new()
    }
    fn on_body(&self, _chart: &Invariant) -> Vec<(Element, Form, Standing)> {
        Vec::new()
    }
    fn speak_form(&self, f: &Form, x: &Grid, placement: &super::alphabet::Word) -> Option<Speech> {
        apply_form(f, x, placement, self)
    }
    fn by_address(&self, _addr: &[u8]) -> Option<(Element, Form)> {
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

/// A pair form derived against NO presentation, under one stroke: the
/// reading a mind that holds nothing gets, which is the cipher's (50.2
/// step 2). Named so a reviewer sees at the call site that no speaker's
/// language is in the form — every test whose claim is about perception
/// rather than about what a mind holds builds its fixture here, and a
/// test about the language passes its own crystal to `pair_form`.
#[cfg(test)]
pub fn pair_form_uncrystallized(
    a: &Grid,
    b: &Grid,
    scale: Option<((usize, usize), (usize, usize))>,
) -> Form {
    let mut budget = crate::physics::Physics::default().stroke();
    // A crystal that holds nothing has nothing to factor through, so the
    // factorisation is measured the moment it starts and cannot end on
    // the budget: this expect is the absence of a presentation, not an
    // unwrap of one that might be there.
    pair_form(&NoCrystal, &mut budget, a, b, scale)
        .expect("a presentation of no generators measures at once")
}

/// The scene frame as a form vertex.
pub fn frame_vertex() -> Vertex {
    Vertex::Frame
}

pub(crate) fn cell_pos_key(r: i64, c: i64) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..8].copy_from_slice(&r.to_le_bytes());
    k[8..16].copy_from_slice(&c.to_le_bytes());
    k
}

pub(crate) fn pair_key(a: u16, b: u16) -> [u8; 32] {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    let mut k = [0u8; 32];
    k[..2].copy_from_slice(&lo.to_le_bytes());
    k[2..4].copy_from_slice(&hi.to_le_bytes());
    k
}

pub(crate) fn decode_pair_key(k: &[u8]) -> Option<(u16, u16)> {
    let lo = u16::from_le_bytes(k.get(..2)?.try_into().ok()?);
    let hi = u16::from_le_bytes(k.get(2..4)?.try_into().ok()?);
    Some((lo, hi))
}

/// The family's frame ratio ((h_num, h_den), (w_num, w_den)), identical
/// across pairs or refused.
pub fn family_ratio(family: &[(Grid, Grid)]) -> Option<((usize, usize), (usize, usize))> {
    let mut ratio = None;
    for (a, b) in family {
        let (fa, fb) = (frame_of(a), frame_of(b));
        if fa.0 == 0 || fa.1 == 0 || fb.0 == 0 || fb.1 == 0 {
            return None;
        }
        let (gh, gw) = (gcd(fb.0, fa.0).max(1), gcd(fb.1, fa.1).max(1));
        let r = ((fb.0 / gh, fa.0 / gh), (fb.1 / gw, fa.1 / gw));
        match &ratio {
            None => ratio = Some(r),
            Some(held) if *held != r => return None,
            _ => {}
        }
    }
    ratio
}

/// One body carried onto another: the transport and the gauge map.
#[derive(Clone, Debug)]
pub struct Carriage {
    pub v: Versor,
    pub sigma: Vec<(u8, u8)>,
}

/// Carry `a` onto `b`: the versor from the moments, and the hue map read
/// cell by cell — functional, or refused.
pub fn carriage(a: &Body, b: &Body) -> Option<Carriage> {
    let v = blade::versor_between(&a.shape, &b.shape)?;
    let mut sigma: crate::network::nerve::Keyed<u8> = crate::network::nerve::Keyed::new();
    for &(r, c, h) in &a.cells {
        let p = element::carry_center(&v, r, c)?;
        let (tr, tc) = element::cell_of(&p)?;
        let h2 = b.cells.binary_search_by(|x| (x.0, x.1).cmp(&(tr, tc))).ok().map(|i| b.cells[i].2)?;
        let k = super::element::hue_pair_key(h, 0);
        match sigma.get(&k) {
            Some(t) if *t != h2 => return None,
            Some(_) => {}
            None => {
                sigma.insert(k, h2);
            }
        }
    }
    let mut out: Vec<(u8, u8)> = sigma
        .fiber(crate::network::nerve::ROOT)
        .filter_map(|(k, t)| super::element::hue_pair_of(&k).map(|(h, _)| (h, *t)))
        .collect();
    out.sort_unstable();
    Some(Carriage { v, sigma: out })
}

#[cfg(test)]
mod tests;
