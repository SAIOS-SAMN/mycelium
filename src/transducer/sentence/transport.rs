// AI-GENERATED CONTENT — see DISCLAIMER.md.

//! Face transport: word, carriage versor, and hue map, and the gauge algebra.

use super::form::{Form, Vertex};
use crate::network::nerve::{Keyed, ROOT};
use crate::transducer::alphabet::Word;
use crate::transducer::element::{self, Versor};

/// Face transport: the relative word, carriage versor and hue map from vertex
/// `i` to vertex `j`. The same three coordinates `transport_class` keys on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transport {
    pub word: Word,
    pub carriage: Versor,
    pub gauge: Vec<(u8, u8)>,
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
        if gamma.word.form().is_none() {
            return None;
        }
        let mut f = self.clone();
        f.rebase_by_word(&gamma.word);
        f.regauge_listing(&gamma.gauge);
        f.compose_carriages(&gamma.carriage);
        f.loops = Form::loops_of(&f.faces, f.vertices.len());
        f.reindex();
        let mut addr = crate::transducer::a21::Address::new(Vec::new());
        for v in &self.vertices {
            if let Vertex::Word { address, .. } = v {
                addr = address.clone();
                break;
            }
        }
        let frame = gamma.word.key();
        let hx = |a: &crate::transducer::a21::Address| {
            a.as_bytes().iter().map(|b| format!("{b:02x}")).collect::<String>()
        };
        eprintln!("refracted {} into {} by Γ", hx(&addr), hx(&frame));
        Some(f)
    }

    pub(crate) fn rebase_by_word(&mut self, by: &Word) {
        for v in &mut self.vertices {
            match v {
                Vertex::Word { word, .. } => *word = word.rebase(by),
                Vertex::Glued { placement, .. } => *placement = placement.rebase(by),
                _ => {}
            }
        }
        for face in &mut self.faces {
            face.transport.word = face.transport.word.rebase(by);
        }
    }

    pub(crate) fn regauge_listing(&mut self, g: &[(u8, u8)]) {
        for v in &mut self.vertices {
            match v {
                Vertex::Word { gauge, .. } | Vertex::Rebased { gauge, .. } => {
                    *gauge = compose_gauge(g, gauge);
                }
                _ => {}
            }
        }
        for face in &mut self.faces {
            face.transport.gauge = compose_gauge(g, &face.transport.gauge);
        }
    }

    pub(crate) fn compose_carriages(&mut self, c: &Versor) {
        for v in &mut self.vertices {
            match v {
                Vertex::Word { carriage, .. } | Vertex::Rebased { carriage, .. } => {
                    *carriage = c.geo(carriage);
                }
                _ => {}
            }
        }
        for face in &mut self.faces {
            face.transport.carriage = c.geo(&face.transport.carriage);
        }
        for lp in &mut self.loops {
            lp.carriage = c.geo(&lp.carriage);
        }
    }
}

#[cfg(test)]
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
    for (k, _) in hues.fiber(ROOT) {
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
    for (k, _) in hues.fiber(ROOT) {
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

pub(crate) fn vertex_carriage(v: &Vertex, form: &Form) -> Option<Versor> {
    match v {
        Vertex::Word { carriage, .. } => Some(carriage.clone()),
        Vertex::Rebased { by, carriage, .. } => match form.vertices.get(*by as usize) {
            Some(Vertex::Word { carriage: c, .. }) => Some(carriage.geo(c)),
            Some(Vertex::Frame) => Some(carriage.clone()),
            _ => None,
        },
        Vertex::Frame | Vertex::Glued { .. } | Vertex::Unfolded { .. } => {
            Some(element::identity_versor())
        }
    }
}

pub(crate) fn vertex_gauge(v: &Vertex) -> Vec<(u8, u8)> {
    match v {
        Vertex::Word { gauge, .. } | Vertex::Rebased { gauge, .. } => gauge.clone(),
        _ => Vec::new(),
    }
}
