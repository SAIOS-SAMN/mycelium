// AI-GENERATED CONTENT — see DISCLAIMER.md.

//! Face transport: word, shift, and hue map, and the gauge algebra.

use super::form::{Form, Vertex};
use crate::network::nerve::{Keyed, ROOT};
use crate::transducer::alphabet::Word;

/// Face transport: the relative word, translation and hue map from vertex
/// `i` to vertex `j`. The same three coordinates `transport_class` keys on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transport {
    pub word: Word,
    pub shift: (num_rational::BigRational, num_rational::BigRational),
    pub gauge: Vec<(u8, u8)>,
}

impl Transport {
    pub fn from_word(word: Word) -> Transport {
        Transport {
            word,
            shift: q2_zero(),
            gauge: Vec::new(),
        }
    }

    pub fn inverse(&self) -> Option<Transport> {
        let word = self.word.inverse();
        let mapped = apply_linear(&word, &self.shift)?;
        Some(Transport {
            word,
            shift: (-mapped.0, -mapped.1),
            gauge: gauge_invert(&self.gauge),
        })
    }
}

pub(crate) fn q2_zero() -> (num_rational::BigRational, num_rational::BigRational) {
    let z = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    (z.clone(), z)
}

pub(crate) fn apply_linear(
    word: &Word,
    t: &(num_rational::BigRational, num_rational::BigRational),
) -> Option<(num_rational::BigRational, num_rational::BigRational)> {
    let b = word.plane_block()?;
    Some((
        &b[0][0] * &t.0 + &b[0][1] * &t.1,
        &b[1][0] * &t.0 + &b[1][1] * &t.1,
    ))
}

pub(crate) fn hue_key(h: u8) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[0] = h;
    k
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
    for (k, _) in hues.fiber(&ROOT) {
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
    for (k, _) in hues.fiber(&ROOT) {
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

pub(crate) fn vertex_translation(
    v: &Vertex,
    form: &Form,
) -> (num_rational::BigRational, num_rational::BigRational) {
    match v {
        Vertex::Word { placement, .. } => placement.clone(),
        Vertex::Rebased { by, shift, .. } => match form.vertices.get(*by as usize) {
            Some(Vertex::Word { placement, .. }) => {
                (placement.0.clone() + &shift.0, placement.1.clone() + &shift.1)
            }
            _ => shift.clone(),
        },
        _ => q2_zero(),
    }
}

pub(crate) fn vertex_gauge(v: &Vertex) -> Vec<(u8, u8)> {
    match v {
        Vertex::Word { gauge, .. } | Vertex::Rebased { gauge, .. } => gauge.clone(),
        _ => Vec::new(),
    }
}
