// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! LIFT — A FRAME HELD BY ITS BODIES' BOUNDARIES, at any rank `n`. The
//! sensor reads the sample once (law 11); what leaves this organ is the
//! frame's enclosure and its bands, never a site of a body's interior.

use super::{Enclosure, TransductionRefusal, Turn};
use crate::transducer::sample::{Place, Sample, Symbol};

/// A BAND: an `n`-dimensional box of one symbol, a span `[lo, hi)` of site
/// boundaries on each phase. Bands are read in canonical address order:
/// the maximal runs along the last axis, each continued along every earlier
/// axis while the slice before shows the same box of the same symbol. Every
/// boundary of a band is a boundary of its body somewhere along it (a run is
/// maximal, and a band ends where its slice's box does not continue), so a
/// body's bands are its boundary record. The symbol is compared by equality
/// alone (MU-SENSING §3.3).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Band {
    pub spans: Vec<(usize, usize)>,
    symbol: Vec<u8>,
}

impl Band {
    /// A band of `spans`, one per phase, holding `symbol`.
    pub fn new(spans: Vec<(usize, usize)>, symbol: Symbol<'_>) -> Band {
        Band { spans, symbol: symbol.bytes().to_vec() }
    }

    /// The band's symbol.
    pub fn symbol(&self) -> Symbol<'_> {
        Symbol::new(&self.symbol)
    }

    /// The band's two boundary turns on each phase of `enclosure`, in axis
    /// order.
    pub fn turns(&self, enclosure: &Enclosure) -> Vec<(Turn, Turn)> {
        self.spans.iter().zip(enclosure.periods()).map(|(&(a, b), &p)| (Turn::new(a, p), Turn::new(b, p))).collect()
    }
}

/// A FRAME AS ITS ENCLOSURE AND ITS BODIES' BANDS. The bands partition the
/// frame (ground is a symbol and its bodies are bodies), in the order the
/// sensor met each: an order of content. `body_of[i]` is band `i`'s body:
/// bands of one symbol in contact through a face of grade `n − 1` (spans
/// abutting on one phase, overlapping with positive length on every other)
/// are one body, as sites of one symbol differing on one axis by one are
/// (MU-SENSING §3.2; MU-RECURSION 71.2 (2)).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Enclosed {
    pub enclosure: Enclosure,
    pub bands: Vec<Band>,
    pub body_of: Vec<usize>,
    pub bodies: usize,
    width: usize,
    place: Place,
}

/// A band while the lift reads it: its spans over the phases from the
/// slice's first axis on, and the offset of one of its sites, whose symbol
/// it holds.
type Lifted = (Vec<(usize, usize)>, usize);

/// THE BANDS OF ONE SLICE: the sites `offset + Σ cᵢ·strideᵢ` over the axes
/// `axis..n`. The last axis reads its maximal runs; every earlier axis
/// reads the bands of each slice after it and continues a band of the
/// slice before while that slice held the same spans and an equal symbol.
fn bands_of(symbols: &[Symbol<'_>], periods: &[usize], strides: &[usize], axis: usize, offset: usize) -> Vec<Lifted> {
    let (Some(&period), Some(&stride)) = (periods.get(axis), strides.get(axis)) else { return Vec::new() };
    let at = |c: usize| offset + c * stride;
    if axis + 1 == periods.len() {
        let mut out = Vec::new();
        let mut from = 0usize;
        for c in 1..=period {
            if c == period || symbols.get(at(c)) != symbols.get(at(from)) {
                out.push((vec![(from, c)], at(from)));
                from = c;
            }
        }
        return out;
    }
    let mut out: Vec<Lifted> = Vec::new();
    // The bands open at the slice before, `(spans past this axis, band)`,
    // sorted by spans: the bands of one slice partition it, so no two hold
    // the same spans, and the spans locate a band without reading its
    // symbol's order.
    let mut open: Vec<(Vec<(usize, usize)>, usize)> = Vec::new();
    for c in 0..period {
        let mut next = Vec::new();
        for (spans, site) in bands_of(symbols, periods, strides, axis + 1, at(c)) {
            let held = open.binary_search_by(|(s, _)| s.cmp(&spans)).ok().and_then(|k| open.get(k)).map(|&(_, i)| i);
            let continued = held.filter(|&i| out.get(i).is_some_and(|b| symbols.get(b.1) == symbols.get(site)));
            let i = match continued.and_then(|i| out.get_mut(i).map(|b| (i, b))) {
                Some((i, band)) => {
                    if let Some(first) = band.0.first_mut() {
                        first.1 = c + 1;
                    }
                    i
                }
                None => {
                    let mut full = Vec::with_capacity(spans.len() + 1);
                    full.push((c, c + 1));
                    full.extend_from_slice(&spans);
                    out.push((full, site));
                    out.len() - 1
                }
            };
            next.push((spans, i));
        }
        next.sort_unstable();
        open = next;
    }
    out
}

/// The strides of canonical address order: the last axis fastest.
fn strides_of(periods: &[usize]) -> Vec<usize> {
    let mut strides = vec![1usize; periods.len()];
    for k in (1..periods.len()).rev() {
        if let (Some(&s), Some(&p)) = (strides.get(k), periods.get(k)) {
            if let Some(slot) = strides.get_mut(k - 1) {
                *slot = s * p;
            }
        }
    }
    strides
}

/// Every offset of the box `spans` under `strides`, in canonical order.
fn offsets_of(spans: &[(usize, usize)], strides: &[usize]) -> Vec<usize> {
    let mut out = vec![0usize];
    for (&(lo, hi), &stride) in spans.iter().zip(strides) {
        out = out.iter().flat_map(|&o| (lo..hi).map(move |c| o + c * stride)).collect();
    }
    out
}

impl Enclosed {
    /// THE FRAME LIFTED: one read per site at the sensor, then the bands and
    /// bodies on boundaries alone. A sample presents at least one axis and
    /// no empty one (S0), so every sample lifts.
    pub fn of(sample: &Sample) -> Enclosed {
        let enclosure = Enclosure::of(sample);
        let symbols: Vec<Symbol<'_>> = sample.symbols().collect();
        let strides = strides_of(enclosure.periods());
        let bands: Vec<Band> = bands_of(&symbols, enclosure.periods(), &strides, 0, 0)
            .into_iter()
            .filter_map(|(spans, site)| Some(Band::new(spans, *symbols.get(site)?)))
            .collect();
        let (body_of, bodies) = bodies_of(&bands, &enclosure);
        Enclosed { enclosure, bands, body_of, bodies, width: sample.width(), place: sample.place().clone() }
    }

    /// The source's symbol width.
    pub fn width(&self) -> usize {
        self.width
    }

    /// The lifted sample's place in its family.
    pub fn place(&self) -> &Place {
        &self.place
    }

    /// The frame held in `enclosure` with `bands`, the bodies read again:
    /// a moved frame, whose bands are the moved ones.
    pub(super) fn with_bands(&self, enclosure: Enclosure, bands: Vec<Band>) -> Enclosed {
        let (body_of, bodies) = bodies_of(&bands, &enclosure);
        Enclosed { enclosure, bands, body_of, bodies, width: self.width, place: self.place.clone() }
    }

    /// The bands of body `body`, in band order.
    pub fn body_bands(&self, body: usize) -> impl Iterator<Item = (usize, &Band)> {
        self.bands.iter().enumerate().filter(move |(i, _)| self.body_of.get(*i) == Some(&body))
    }

    /// The symbol of body `body`: every band of a body has one.
    pub fn body_symbol(&self, body: usize) -> Option<Symbol<'_>> {
        self.body_bands(body).map(|(_, b)| b.symbol()).next()
    }

    /// PROJECT (D3.2 §E) — THE FRAME SPOKEN ONTO ANOTHER ENCLOSURE by the
    /// re-period alone: each band written over the sites its turns land on.
    /// `Rank` where `to` has another rank; every band boundary turn must
    /// land on a boundary of `to`, or the projection refuses `OffSite`
    /// naming the first that does not (a boundary inside an output
    /// compartment would speak two symbols there, and none is chosen). This
    /// is a speech: a sample is written, and nowhere else does this organ
    /// write one.
    pub fn spoken_onto(&self, to: &Enclosure) -> Result<Sample, TransductionRefusal> {
        if to.rank() != self.enclosure.rank() {
            return Err(TransductionRefusal::Rank { from: self.enclosure.rank(), to: to.rank() });
        }
        use crate::transducer::sample::SampleRefusal;
        let sample_refused = |r| TransductionRefusal::Sample(r);
        let mut sites = 1usize;
        for (axis, &q) in to.periods().iter().enumerate() {
            sites = sites.checked_mul(q).ok_or(sample_refused(SampleRefusal::SitesOverflow { axis }))?;
        }
        let width = self.width;
        let len = sites.checked_mul(width).ok_or(sample_refused(SampleRefusal::BytesOverflow { sites, width }))?;
        // Every byte is written: the landed bands partition `to` as the
        // bands partition the frame, so the buffer's first fill is never
        // read.
        let mut out = vec![0u8; len];
        let strides = strides_of(to.periods());
        for b in &self.bands {
            let mut landed = Vec::with_capacity(b.spans.len());
            for ((t0, t1), &q) in b.turns(&self.enclosure).iter().zip(to.periods()) {
                landed.push((t0.lands(q)?, t1.lands(q)?));
            }
            for o in offsets_of(&landed, &strides) {
                let from = o * self.width;
                if let Some(slot) = out.get_mut(from..from + self.width) {
                    slot.copy_from_slice(b.symbol().bytes());
                }
            }
        }
        Sample::new(to.periods().to_vec(), self.width, out, self.place.clone()).map_err(sample_refused)
    }

    /// THE FRAME'S FULL CONTENT AS ITS BOUNDARY RECORDS (O3): the rank,
    /// the periods, the symbol width, then every band's boundary indices,
    /// phase by phase, and its symbol's bytes, in band order. Lossless
    /// across ranks and widths (track 26 S5): the rank says how many
    /// periods and spans a band holds, the width how many bytes its symbol
    /// is, and the bands partition the frame, so the frame is read back
    /// from these bytes alone.
    pub fn put_content(&self, out: &mut Vec<u8>) {
        use crate::transducer::address::put_key_count;
        put_key_count(out, self.enclosure.periods().len());
        for &p in self.enclosure.periods() {
            put_key_count(out, p);
        }
        put_key_count(out, self.width);
        put_key_count(out, self.bands.len());
        for b in &self.bands {
            for &(lo, hi) in &b.spans {
                put_key_count(out, lo);
                put_key_count(out, hi);
            }
            out.extend_from_slice(&b.symbol);
        }
    }
}

/// THE BODIES OF A FRAME'S BANDS: bands of one symbol in contact through a
/// face of grade `n − 1` are one body. On each phase, at each boundary, the
/// bands ending there are read against those starting there, so the
/// contacts are read off boundaries; no phase is read apart from the others.
fn bodies_of(bands: &[Band], enclosure: &Enclosure) -> (Vec<usize>, usize) {
    let mut parent: Vec<usize> = (0..bands.len()).collect();
    // A band's root: parents only ever point to a lesser band, so the walk
    // ends.
    fn root(parent: &[usize], mut i: usize) -> usize {
        while let Some(&p) = parent.get(i) {
            if p == i {
                break;
            }
            i = p;
        }
        i
    }
    for (axis, &period) in enclosure.periods().iter().enumerate() {
        let mut ending: Vec<Vec<usize>> = vec![Vec::new(); period + 1];
        let mut starting: Vec<Vec<usize>> = vec![Vec::new(); period + 1];
        for (i, b) in bands.iter().enumerate() {
            let Some(&(lo, hi)) = b.spans.get(axis) else { continue };
            if let Some(e) = ending.get_mut(hi) {
                e.push(i);
            }
            if let Some(s) = starting.get_mut(lo) {
                s.push(i);
            }
        }
        for (before, after) in ending.iter().zip(&starting) {
            for &ai in before {
                for &bi in after {
                    let (Some(a), Some(b)) = (bands.get(ai), bands.get(bi)) else { continue };
                    let face = a
                        .spans
                        .iter()
                        .zip(&b.spans)
                        .enumerate()
                        .all(|(k, (&(a0, a1), &(b0, b1)))| k == axis || (a0 < b1 && b0 < a1));
                    if !face || a.symbol() != b.symbol() {
                        continue;
                    }
                    let (ra, rb) = (root(&parent, ai), root(&parent, bi));
                    if ra != rb {
                        let (lo, hi) = if ra < rb { (ra, rb) } else { (rb, ra) };
                        if let Some(slot) = parent.get_mut(hi) {
                            *slot = lo;
                        }
                    }
                }
            }
        }
    }
    // Each body numbered by its least band: an order of content.
    let mut number: Vec<Option<usize>> = vec![None; bands.len()];
    let mut body_of = Vec::with_capacity(bands.len());
    let mut bodies = 0usize;
    for i in 0..bands.len() {
        let r = root(&parent, i);
        let n = match number.get(r).copied().flatten() {
            Some(n) => n,
            None => {
                if let Some(slot) = number.get_mut(r) {
                    *slot = Some(bodies);
                }
                bodies += 1;
                bodies - 1
            }
        };
        body_of.push(n);
    }
    (body_of, bodies)
}
