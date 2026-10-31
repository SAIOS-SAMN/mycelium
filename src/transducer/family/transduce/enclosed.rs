// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! LIFT — A FRAME HELD BY ITS BODIES' BOUNDARIES. The sensor reads the
//! raster once (law 11); what leaves this organ is the frame's enclosure and
//! its bands, never a site of a body's interior.

use super::{Enclosure, TransductionRefusal, Turn};
use crate::transducer::family::perceive::Grid;

/// A BAND: a maximal run of one hue whose column boundary turns stand
/// unchanged over consecutive row arcs. Its four indices are boundary turns
/// of its frame — rows `[rows.0/h, rows.1/h)`, columns `[cols.0/w,
/// cols.1/w)` — and every one of them is a boundary of its body somewhere
/// along it (a run is maximal in its row, and a band ends where its row's
/// run does not continue), so a body's bands are its boundary record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Band {
    pub rows: (usize, usize),
    pub cols: (usize, usize),
    pub hue: u8,
}

impl Band {
    /// The band's two row boundary turns and two column boundary turns in
    /// `enclosure`.
    pub fn turns(&self, enclosure: &Enclosure) -> [Turn; 4] {
        [
            Turn::new(self.rows.0, enclosure.rows),
            Turn::new(self.rows.1, enclosure.rows),
            Turn::new(self.cols.0, enclosure.cols),
            Turn::new(self.cols.1, enclosure.cols),
        ]
    }
}

/// A FRAME AS ITS ENCLOSURE AND ITS BODIES' BANDS. The bands partition the
/// frame (ground is a hue and its bodies are bodies), in the order the
/// sensor met each band's first row arc and, within it, its first column
/// boundary: an order of content. `body_of[i]` is band `i`'s body: the
/// bands of one hue that touch along a row boundary of positive length are
/// one body, as 4-adjacent sites of one hue are.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Enclosed {
    pub enclosure: Enclosure,
    pub bands: Vec<Band>,
    pub body_of: Vec<usize>,
    pub bodies: usize,
}

/// The runs of one row: `(from, to, hue)` for each maximal run of one hue.
fn runs_of(row: &[u8]) -> Vec<(usize, usize, u8)> {
    let mut out = Vec::new();
    let mut from = 0usize;
    for (c, pair) in row.windows(2).enumerate() {
        if pair[0] != pair[1] {
            out.push((from, c + 1, pair[0]));
            from = c + 1;
        }
    }
    if let Some(&hue) = row.get(from) {
        out.push((from, row.len(), hue));
    }
    out
}

impl Enclosed {
    /// THE FRAME LIFTED: `Empty` where it has no row or no column, `Ragged`
    /// where its rows disagree. One read per site at the sensor, then the
    /// bands and bodies on boundaries alone.
    pub fn of(g: &Grid) -> Result<Enclosed, TransductionRefusal> {
        let enclosure = Enclosure::of(g)?;
        let mut bands: Vec<Band> = Vec::new();
        // The bands open at the previous row, `(from, to, hue, band)`, in
        // column order.
        let mut open: Vec<(usize, usize, u8, usize)> = Vec::new();
        for (r, row) in g.iter().enumerate() {
            let mut next = Vec::new();
            let mut held = open.iter().peekable();
            for (from, to, hue) in runs_of(row) {
                // Advance past the open bands that start before this run:
                // both lists are in column order, so one pass reads them.
                while held.peek().is_some_and(|&&(f, ..)| f < from) {
                    held.next();
                }
                let continued = match held.peek() {
                    Some(&&(f, t, h, i)) if (f, t, h) == (from, to, hue) => Some(i),
                    _ => None,
                };
                let i = match continued.and_then(|i| bands.get_mut(i).map(|b| (i, b))) {
                    Some((i, band)) => {
                        band.rows.1 = r + 1;
                        i
                    }
                    None => {
                        bands.push(Band { rows: (r, r + 1), cols: (from, to), hue });
                        bands.len() - 1
                    }
                };
                next.push((from, to, hue, i));
            }
            open = next;
        }
        let (body_of, bodies) = bodies_of(&bands, enclosure.rows);
        Ok(Enclosed { enclosure, bands, body_of, bodies })
    }

    /// The bands of body `body`, in band order.
    pub fn body_bands(&self, body: usize) -> impl Iterator<Item = (usize, &Band)> {
        self.bands.iter().enumerate().filter(move |(i, _)| self.body_of.get(*i) == Some(&body))
    }

    /// The hue of body `body`: every band of a body has one.
    pub fn body_hue(&self, body: usize) -> Option<u8> {
        self.body_bands(body).map(|(_, b)| b.hue).next()
    }

    /// PROJECT (D3.2 §E) — THE FRAME SPOKEN ONTO ANOTHER ENCLOSURE by the
    /// re-period alone: each band written over the output sites its turns
    /// land on. Every band boundary turn must land on a boundary of `to`, or
    /// the projection refuses `OffSite` naming the first that does not (a
    /// boundary inside an output compartment would speak two hues there,
    /// and none is chosen). This is a speech: a grid is written, `to.rows ×
    /// to.cols` sites, and nowhere else does this organ write one.
    pub fn spoken_onto(&self, to: Enclosure) -> Result<Grid, TransductionRefusal> {
        let mut out: Grid = vec![vec![0; to.cols]; to.rows];
        for b in &self.bands {
            let [r0, r1, c0, c1] = b.turns(&self.enclosure);
            let (r0, r1, c0, c1) = (r0.lands(to.rows)?, r1.lands(to.rows)?, c0.lands(to.cols)?, c1.lands(to.cols)?);
            for row in out.iter_mut().take(r1).skip(r0) {
                for site in row.iter_mut().take(c1).skip(c0) {
                    *site = b.hue;
                }
            }
        }
        Ok(out)
    }

    /// THE FRAME'S FULL CONTENT AS ITS BOUNDARY RECORDS (O3): the two
    /// periods, then every band's four boundary indices and hue, in band
    /// order. Lossless: the bands partition the frame, so the frame is read
    /// back from these bytes alone.
    pub fn put_content(&self, out: &mut Vec<u8>) {
        use crate::transducer::address::put_key_count;
        put_key_count(out, self.enclosure.rows);
        put_key_count(out, self.enclosure.cols);
        put_key_count(out, self.bands.len());
        for b in &self.bands {
            put_key_count(out, b.rows.0);
            put_key_count(out, b.rows.1);
            put_key_count(out, b.cols.0);
            put_key_count(out, b.cols.1);
            out.push(b.hue);
        }
    }
}

/// THE BODIES OF A FRAME'S BANDS: bands of one hue meeting along a row
/// boundary over a column span of positive length are one body. At each
/// row boundary the bands ending there and those starting there are read in
/// column order, one pass each, so the contacts are read off boundaries.
fn bodies_of(bands: &[Band], rows: usize) -> (Vec<usize>, usize) {
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
    let mut ending: Vec<Vec<(usize, usize, usize)>> = vec![Vec::new(); rows + 1];
    let mut starting: Vec<Vec<(usize, usize, usize)>> = vec![Vec::new(); rows + 1];
    for (i, b) in bands.iter().enumerate() {
        if let Some(e) = ending.get_mut(b.rows.1) {
            e.push((b.cols.0, b.cols.1, i));
        }
        if let Some(s) = starting.get_mut(b.rows.0) {
            s.push((b.cols.0, b.cols.1, i));
        }
    }
    for (above, below) in ending.iter_mut().zip(starting.iter_mut()) {
        above.sort_unstable();
        below.sort_unstable();
        let (mut i, mut j) = (0usize, 0usize);
        while let (Some(&(a0, a1, ai)), Some(&(b0, b1, bi))) = (above.get(i), below.get(j)) {
            if a0 < b1 && b0 < a1 && bands.get(ai).map(|b| b.hue) == bands.get(bi).map(|b| b.hue) {
                let (ra, rb) = (root(&parent, ai), root(&parent, bi));
                if ra != rb {
                    let (lo, hi) = if ra < rb { (ra, rb) } else { (rb, ra) };
                    if let Some(slot) = parent.get_mut(hi) {
                        *slot = lo;
                    }
                }
            }
            if a1 <= b1 {
                i += 1;
            } else {
                j += 1;
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
