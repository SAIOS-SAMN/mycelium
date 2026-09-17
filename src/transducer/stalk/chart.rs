// A6, A7 — THE CHART: the invariant coordinates a word is filed under, read
// off the rotor and the versor and never off a matrix.
//
//! A chart is a vector of exact coordinates, coarsest first, so a prefix of
//! one chart is its coarser stratum. Two words congruent under the conformal
//! group share a chart; two of equal volume and different structure do not.

use super::*;

/// A chain cell: (row, col, hue before, hue after).
pub(crate) type Cell = (i64, i64, u8, u8);

/// Standing of a word on the working atlas: a coordinate, never a gate.
/// Ruled weights above Stood, Stood above Held.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Standing {
    /// Court-ratified, persisted in the genome.
    Ruled = 0,
    /// Spoken and stood by this mind, dialect, memory only.
    Stood = 1,
    /// An idea this mind is holding, not yet a word.
    Held = 2,
}

impl Standing {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Standing::Ruled),
            1 => Some(Standing::Stood),
            2 => Some(Standing::Held),
            _ => None,
        }
    }
}

/// The moduli of a support, in axis order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Invariant(pub Vec<u64>);

impl Invariant {
    /// Drop the deepest axis: the coarser chart this one refines.
    pub fn peel(&self) -> Invariant {
        let mut axes = self.0.clone();
        axes.pop();
        Invariant(axes)
    }

    /// Every prefix of this chart, finest first, including the empty chart.
    /// Chart-space distance is prefix depth, never a norm.
    pub fn prefixes(&self) -> Vec<Invariant> {
        let mut out = Vec::with_capacity(self.0.len() + 1);
        let mut cur = self.clone();
        loop {
            out.push(cur.clone());
            if cur.0.is_empty() {
                break;
            }
            cur = cur.peel();
        }
        out
    }

    /// The axes written out, coarsest first, no count. A prefix of the
    /// key is a coarser stratum of the same chart.
    pub fn key(&self) -> crate::transducer::address::Address {
        let mut buf = vec![crate::transducer::address::TAG_CHART];
        for a in &self.0 {
            buf.extend_from_slice(&a.to_le_bytes());
        }
        crate::transducer::address::Address::new(buf)
    }

    /// Decode canonical axes from a chart key.
    pub fn from_key(k: &[u8]) -> Option<Invariant> {
        if crate::transducer::address::tag_of(k)? != crate::transducer::address::TAG_CHART {
            return None;
        }
        let mut axes = Vec::new();
        let mut p = 1usize;
        while p + 8 <= k.len() {
            let a = u64::from_le_bytes(k.get(p..p + 8)?.try_into().ok()?);
            axes.push(a);
            p += 8;
        }
        Some(Invariant(axes))
    }
}

/// Closure of one chart a thought has been probed through. Never a number:
/// the loop closed (Flat), returned twisted (Twisted), a Čech class on the
/// nerve (Class — the same coefficient the cochains speak), or found no
/// section (Cavity). Magnitudes are read from remainders, never stored here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Closure {
    Flat,
    Twisted(Box<Versor>),
    /// A Čech class at a degree, a Rotor of the alphabet.
    Class(Box<Rotor>, u8),
    Cavity,
}

impl Closure {
    pub fn is_flat(&self) -> bool {
        matches!(self, Closure::Flat)
    }
}

/// Measure the moduli of a chain's support (hues are fibre coordinates over
/// the base point, not part of the geometry class).
pub fn invariant(chain: &[Cell]) -> Invariant {
    let mut cells = element::positions(chain);
    cells.sort_unstable();
    cells.dedup();
    if cells.is_empty() {
        return Invariant(Vec::new());
    }
    let shape = crate::transducer::blade::shape_of_section(&cells);
    Invariant(crate::transducer::blade::moduli(&shape.blade))
}

/// THE INVARIANT OF A WORD, read on its kind. A residual word (a chain) is
/// charted by its support's moduli. A connection is charted on its fibres:
/// the conjugacy class of the affine block's linear part (trace and
/// determinant, exact rationals packed as the moduli are) and the cycle
/// type of the gauge permutation. The translation part is the versor's
/// business on the fibre, exactly as position is quotiented for a residual
/// word: two identical motions at different origins share the chart. A
/// leading axis of 0 keeps connection charts disjoint from residual charts,
/// whose first axis is a component count of at least one. A word that is
/// neither — no support, no group component — has the empty chart:
/// novelty, not a stratum.
pub fn invariant_of(word: &element::Element) -> Invariant {
    if word.is_rewrite() {
        return Invariant(vec![3]);
    }
    if word.is_sentence() {
        // A sentence whose tape names no chart sits at the root stratum,
        // which is what the empty invariant is.
        return match crate::transducer::sentence::chart_of(word) {
            Some(inv) => inv,
            None => Invariant(Vec::new()),
        };
    }
    if word.is_connection() {
        // A connection whose rotor names no chart sits at the root.
        return match connection_chart(word) {
            Some(inv) => inv,
            None => Invariant(Vec::new()),
        };
    }
    if !word.sub().is_empty() {
        return invariant(word.sub());
    }
    Invariant(Vec::new())
}

pub(crate) fn connection_chart(word: &element::Element) -> Option<Invariant> {
    let letters = word.letters()?;
    let mut axes = vec![0u64];
    axes.extend(letters.spelling.word.rotor().class_axes()?);
    let mut image: [u8; 10] = std::array::from_fn(|i| i as u8);
    for &(f, t) in &letters.sigma {
        image[f as usize] = t;
    }
    let mut seen = [false; 10];
    let mut cycles: Vec<u64> = Vec::new();
    for start in 0..10 {
        if seen[start] {
            continue;
        }
        let (mut i, mut len) = (start, 0u64);
        while !seen[i] {
            seen[i] = true;
            len += 1;
            i = image[i] as usize;
            if i >= 10 {
                return None;
            }
        }
        if len > 1 {
            cycles.push(len);
        }
    }
    cycles.sort_unstable();
    axes.push(cycles.len() as u64);
    axes.extend(cycles);
    Some(Invariant(axes))
}

/// A PLACEMENT: a transition between two connections that is a pure
/// translation — identity linear block, identity gauge, no chain — is the
/// same word placed elsewhere, and the translation is the versor carrying
/// one onto the other (a section of the base, never a new base). None when
/// the transition moves anything but the origin.
pub fn placement_of(g: &element::Element) -> Option<Versor> {
    if !g.sub().is_empty() {
        return None;
    }
    if let Some(letters) = g.letters() {
        if !letters.spelling.word.is_identity() || !letters.sigma.is_empty() {
            return None;
        }
    } else if !g.gauge().is_empty() || !g.germinated().is_empty() {
        return None;
    }
    let v = g.versor()?;
    let t = element::translator_of(&v)?;
    let d = element::dilator_of(&v)?;
    if !element::same_versor(&d, &element::identity_versor()) {
        return None;
    }
    Some(t)
}

/// CONGRUENCE on elements: the held word that is `spoken`'s base point,
/// with the versor carrying `spoken` onto it. For a residual word the chart
/// narrows the candidates and the versor between shapes decides; exactly
/// one relating word is a base point, several is the measurement that the
/// chart is too coarse (multiplicity — an axis to extend, returned as no
/// base, never a first-wins pick). For a connection the fibre's
/// representative (the first held connection there) is the chart, and the
/// Čech transition decides: the identity is the same word; anything else
/// is a dialect or a section, never a new base.
pub fn congruent<'a>(
    stalks: &Stalks,
    at: impl Fn(&[u8]) -> Option<&'a element::Element>,
    spoken: &element::Element,
) -> Option<(crate::transducer::address::Address, Versor)> {
    let fibre = stalks.fiber(&invariant_of(spoken));
    if spoken.is_connection() {
        let held = fibre.iter().find_map(|a| at(a).filter(|w| w.address() != spoken.address() && !w.axes().is_empty()).map(|w| (a.clone(), w)))?;
        let g = crate::transducer::cech::transition(held.1, spoken)?;
        return match (g.axes().as_slice(), g.sub()) {
            ([], []) => Some((crate::transducer::address::Address::from(held.0), element::identity_versor())),
            _ => placement_of(&g).map(|v| (crate::transducer::address::Address::from(held.0), v)),
        };
    }
    let shape = element::shape_of_chain(spoken.sub())?;
    let mut related = fibre
        .iter()
        .filter_map(|a| at(a).filter(|w| !w.sub().is_empty()).map(|w| (a.clone(), w)))
        .filter_map(|(a, w)| element::transition_from_shape(&shape, w.sub()).map(|v| (crate::transducer::address::Address::from(a), v)));
    let first = related.next()?;
    match related.next() {
        None => Some(first),
        Some(_) => None, // multiplicity: the chart wants another axis
    }
}

/// Congruence over pre-lifted shapes for a chain: `shape_at` yields a held
/// word's shape (lifted once per snapshot), so a lookup lifts only the
/// chain it asks about. Exactly one relating word is the base point.
pub fn congruent_shaped<'a>(
    stalks: &Stalks,
    shape_at: impl Fn(&[u8]) -> Option<&'a crate::transducer::blade::Shape>,
    chain: &[Cell],
) -> Option<(crate::transducer::address::Address, Versor)> {
    let shape = element::shape_of_chain(chain)?;
    let fibre = stalks.fiber(&invariant(chain));
    let mut related = fibre
        .iter()
        .filter_map(|a| shape_at(a).and_then(|held| crate::transducer::blade::versor_between(&shape, held)).map(|v| (crate::transducer::address::Address::from(a.clone()), v)));
    let first = related.next()?;
    match related.next() {
        None => Some(first),
        Some(_) => None,
    }
}
