// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 20, 21 — THE MOVES ON A BUNDLE OF TRAIN PAIRS: scenes perceived as pair
//! forms, the rule those forms already agree on, and that rule spoken back
//! onto a scene. A bundle it does not close is a withheld form.

pub mod descent;
pub mod extract;
pub mod orbit;
pub mod pair;
pub mod perceive;
pub mod speak;

pub use descent::*;

/// ONE TRAIN PAIR — the scene a rule is shown and the scene it must speak
/// (38). The grids are the corpus's own carrier; the pair is what a rule is
/// extracted from and replayed against.
pub type TrainPair = (perceive::Grid, perceive::Grid);

/// THE FAMILY a rule is extracted from: every train pair of one task. A
/// rule speaks the family or it is not a rule (21).
pub type Family = [TrainPair];
pub use extract::*;
pub use orbit::*;
pub use pair::*;
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
        let (tr, tc) = element::cell_of(&p);
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
