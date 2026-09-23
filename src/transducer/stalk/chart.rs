// A6, A7 — THE CHART: the invariant coordinates a word is filed under, read
// off the rotor and the versor and never off a matrix.
//
//! A chart is a vector of exact coordinates, coarsest first, so a prefix of
//! one chart is its coarser stratum. Two words congruent under the conformal
//! group share a chart; two of equal volume and different structure do not.

use super::*;

/// A chain link: (row, col, hue before, hue after).
pub(crate) type Link = (i64, i64, u8, u8);

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

/// ONE COORDINATE OF A CHART (A21, ruled 2026-09-18). A chart's axes are
/// whole coordinates, each carrying its own exact value, and never limbs of
/// one packed into the next.
///
/// The axes were `u64` and a rational was TWO of them — a numerator limb
/// and a denominator limb — so an axis was a position in a flat vector and
/// a coordinate was however many positions its value happened to need.
/// `peel` popped one `u64`, which on a two-limb axis left half a rational
/// standing as a "coarser stratum", and index `n` of two charts named the
/// same quantity only while every axis before it was the same width. A
/// rational carried as a pair of integers is law 11's reading at the chart:
/// the coordinate re-derived through a representation that cannot hold it.
///
/// THE PIVOT'S `Axis` IS NOT THIS. `sentence::Axis` names a motion's
/// coordinates — Orientation, Unit, Prime5, Prime13, Hue, Placement,
/// Dilation, OffPlane, Open — and is the slot key of the axis index. These
/// are a body's: what its shape measures. Two vocabularies, two enums; one
/// enum carrying both would put a shape's count where a class vector reads
/// a plane (rule 15).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChartAxis {
    /// A topological count: components, tunnels, cavities, turns.
    Count(u64),
    /// A conformal ratio, exact (law 4): the isoperimetric ratio, the frame
    /// boundary, the anisotropy, the spread, the quartic, a hue fraction,
    /// and a rotor's conjugacy invariants (15.3).
    Ratio(num_rational::BigRational),
}

/// A ROTOR'S CONJUGACY STRATUM AS COORDINATES (15.3, A7): the leading tag,
/// the two signs of the ray, then `a²/w` and `p²/w` — each its own
/// coordinate carrying its own exact value.
///
/// `Rotor::class_axes` stood here and returned `Vec<u64>`, packing each
/// rational into two `u64` limbs through `pack_axis`. A limb is not a
/// coordinate: `peel` could cut between the two halves of one invariant
/// and leave a chart naming half a rational, which is the N7 defect
/// `ChartAxis::Ratio` exists to answer (A21 §2 as ruled 2026-09-18). The
/// same bytes also went out as `ChartAxis::ConnectionClass(u64)`, one
/// opaque integer per limb, so a reader could not tell a sign from a
/// magnitude from a tag.
///
/// These are recorded, never compared. Law 11 forbids forming an angle —
/// a square root, an `arccos` — and `a²/w` is 15.3's exact rational class
/// identity, which `Rotor::class_key` already writes. Writing it as a
/// coordinate is not the scalar selection rule 4 refuses; that was
/// `coset_representative` choosing by it, and it no longer does.
///
/// `None` when the rotor carries no class (15.3: not a versor).
pub fn rotor_axes(r: &crate::transducer::alphabet::Rotor) -> Option<Vec<ChartAxis>> {
    let n = r.normalised();
    // A rotor with no weight of its own is at unit weight, which is what a
    // normalised rotor without one carries.
    let w = match n.weight() {
        Some(w) => w,
        None => num_traits::One::one(),
    };
    let (a, p) = (n.coeff(0).clone(), n.pseudoscalar().clone());
    let neg = |q: &num_rational::BigRational| -> u64 {
        u64::from(num_traits::Signed::is_negative(q))
    };
    let mut axes = vec![ChartAxis::Count(2), ChartAxis::Count(neg(&a)), ChartAxis::Count(neg(&p))];
    if num_traits::Zero::is_zero(&w) {
        return Some(axes);
    }
    axes.push(ChartAxis::Ratio(&a * &a / &w));
    axes.push(ChartAxis::Ratio(&p * &p / &w));
    Some(axes)
}

impl ChartAxis {
    /// The kind byte a coordinate is written under.
    pub fn kind(&self) -> u8 {
        match self {
            ChartAxis::Count(_) => 0,
            ChartAxis::Ratio(_) => 1,
        }
    }

    /// The coordinate's own canonical bytes, without its kind or length.
    pub fn payload(&self) -> Vec<u8> {
        match self {
            ChartAxis::Count(n) => n.to_le_bytes().to_vec(),
            // Sign, then each magnitude's limbs by count, for the numerator
            // and then the denominator: the rational's exact coordinates,
            // written out (law 2). Inside ONE axis, so `peel` cannot cut
            // between them.
            ChartAxis::Ratio(q) => {
                let mut out = Vec::new();
                let (sign, digits) = q.numer().to_u64_digits();
                out.push(u8::from(matches!(sign, num_bigint::Sign::Minus)));
                out.extend_from_slice(&(digits.len() as u32).to_le_bytes());
                for d in &digits {
                    out.extend_from_slice(&d.to_le_bytes());
                }
                let (_, digits) = q.denom().to_u64_digits();
                out.extend_from_slice(&(digits.len() as u32).to_le_bytes());
                for d in &digits {
                    out.extend_from_slice(&d.to_le_bytes());
                }
                out
            }
        }
    }

    /// ONE COORDINATE'S OWN BYTES: its kind, its length, its payload. This
    /// is the unit the chart key is built from AND the unit the stratum
    /// trie branches on, so the coordinate a chart writes and the
    /// coordinate a fiber reads are one construction (law 12).
    pub fn key_bytes(&self) -> Vec<u8> {
        let payload = self.payload();
        let mut out = Vec::with_capacity(payload.len() + 3);
        out.push(self.kind());
        out.extend_from_slice(&(payload.len() as u16).to_le_bytes());
        out.extend_from_slice(&payload);
        out
    }

    /// Read one coordinate back from those bytes, exactly or not at all.
    pub fn from_key_bytes(b: &[u8]) -> Option<ChartAxis> {
        // The layout destructured, not indexed: here the ORDER IS CONTENT —
        // a kind byte, a length, then that many bytes — so the pattern says
        // what the bytes are rather than choosing a member by position
        // (rule 15).
        let [kind, lo, hi, payload @ ..] = b else {
            return None;
        };
        let len = u16::from_le_bytes([*lo, *hi]) as usize;
        if len != payload.len() {
            return None;
        }
        let kind = *kind;
        ChartAxis::from_parts(kind, payload)
    }

    /// Read a coordinate back from its kind and payload. `None` for a kind
    /// this reader does not know or a payload that does not decode: a chart
    /// is recovered from its key exactly or not at all (law 2, rule 11).
    pub fn from_parts(kind: u8, payload: &[u8]) -> Option<ChartAxis> {
        let u64_at = |b: &[u8]| -> Option<u64> { Some(u64::from_le_bytes(b.try_into().ok()?)) };
        match kind {
            0 => Some(ChartAxis::Count(u64_at(payload)?)),
            1 => {
                let mut p = 0usize;
                let take_mag = |p: &mut usize| -> Option<num_bigint::BigUint> {
                    let n = u32::from_le_bytes(payload.get(*p..*p + 4)?.try_into().ok()?) as usize;
                    *p += 4;
                    let mut digits = Vec::with_capacity(n);
                    for _ in 0..n {
                        digits.push(u64_at(payload.get(*p..*p + 8)?)?);
                        *p += 8;
                    }
                    Some(num_bigint::BigUint::new(
                        digits.iter().flat_map(|d| [*d as u32, (*d >> 32) as u32]).collect(),
                    ))
                };
                let [sign_byte, ..] = payload else {
                    return None;
                };
                let negative = *sign_byte != 0;
                p += 1;
                let n = take_mag(&mut p)?;
                let d = take_mag(&mut p)?;
                if p != payload.len() {
                    return None;
                }
                let sign = match negative {
                    true => num_bigint::Sign::Minus,
                    false => num_bigint::Sign::Plus,
                };
                Some(ChartAxis::Ratio(num_rational::BigRational::new(
                    num_bigint::BigInt::from_biguint(sign, n),
                    num_bigint::BigInt::from_biguint(num_bigint::Sign::Plus, d),
                )))
            }
            _ => None,
        }
    }
}

/// The moduli of a support, in axis order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Invariant(pub Vec<ChartAxis>);

impl Invariant {
    /// A chart of topological counts alone — the four a body leads with,
    /// and what a test names a chart by.
    pub fn counts(ns: &[u64]) -> Invariant {
        Invariant(ns.iter().map(|&n| ChartAxis::Count(n)).collect())
    }

    /// Drop the deepest axis: the coarser chart this one refines. One whole
    /// coordinate leaves, which is what makes the result a stratum (A21).
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

    /// THE AXES WRITTEN OUT, COARSEST FIRST, EACH KIND-TAGGED AND
    /// LENGTH-PREFIXED (law 2, A21). The object is recovered from its key,
    /// and a prefix of the key is a coarser stratum.
    ///
    /// NO COUNT IN THE HEADER. A count would name how many coordinates the
    /// whole chart has, and a chart's prefix has fewer — so the prefix's
    /// key would differ from the full key in its first bytes and would stop
    /// being a BYTE prefix of it. `Keyed` is a PATRICIA trie over these
    /// bytes and `fiber_under`, `link` and `peel` are byte-prefix reads, so
    /// a header count would break the one property A21 asserts. Each
    /// coordinate carrying its own length is enough to parse: the decode is
    /// self-delimiting without anything knowing the total ahead of it.
    pub fn key(&self) -> crate::transducer::address::Address {
        let mut buf = vec![crate::transducer::address::TAG_CHART];
        for a in &self.0 {
            buf.extend_from_slice(&a.key_bytes());
        }
        crate::transducer::address::Address::new(buf)
    }

    /// Decode canonical axes from a chart key. Total or refused: a key that
    /// ends mid-coordinate decodes to nothing rather than to the
    /// coordinates before it, which would be a different chart read as this
    /// one (rule 11).
    pub fn from_key(k: &[u8]) -> Option<Invariant> {
        if crate::transducer::address::tag_of(k)? != crate::transducer::address::TAG_CHART {
            return None;
        }
        let mut axes = Vec::new();
        let mut p = 1usize;
        while p < k.len() {
            let kind = *k.get(p)?;
            let len = u16::from_le_bytes(k.get(p + 1..p + 3)?.try_into().ok()?) as usize;
            let payload = k.get(p + 3..p + 3 + len)?;
            axes.push(ChartAxis::from_parts(kind, payload)?);
            p += 3 + len;
        }
        Some(Invariant(axes))
    }
}

/// Closure of one chart a thought has been probed through. Never a number:
/// the orbit closed (Flat), returned twisted (Twisted), a Čech class on the
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
pub fn invariant(chain: &[Link]) -> Invariant {
    let mut sites = element::positions(chain);
    sites.sort_unstable();
    sites.dedup();
    if sites.is_empty() {
        return Invariant(Vec::new());
    }
    Invariant(crate::transducer::blade::section_moduli(&sites))
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
        return Invariant::counts(&[3]);
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
    // The lane tag, then the rotor's conjugacy stratum as coordinates —
    // each invariant whole, never a pair of `u64` limbs `peel` could cut
    // between (A21 §2).
    let mut axes = vec![ChartAxis::Count(0)];
    axes.extend(rotor_axes(&letters.spelling.word.rotor())?);
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
    axes.push(ChartAxis::Count(cycles.len() as u64));
    axes.extend(cycles.into_iter().map(ChartAxis::Count));
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
    chain: &[Link],
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
