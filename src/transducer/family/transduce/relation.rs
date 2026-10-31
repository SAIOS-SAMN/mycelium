// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! O10 — HOW A PAIR'S TWO FRAMES RELATE, typed: is the relation a product of
//! a signed swap, the re-period, an offset and a re-enclosure, or a residue
//! no such product carries? A reading of one pair, for the census of the
//! corpus; it decides nothing a mind does.

use super::{Enclosed, SignedSwap, Transduction, TransductionRefusal};
use crate::network::nerve::Keyed;
use crate::transducer::family::perceive::Grid;

/// THE CLASS OF ONE PAIR'S RELATION, the first that holds in this order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairRelation {
    /// Every input body is carried whole onto one output body by this
    /// signed swap and the re-period of each circle (`Transduction::
    /// carried`): positions and shapes both, in turns.
    Carried(SignedSwap),
    /// The output is `rows × cols` copies of the input, at least two. Read
    /// as one map of each circle this is the wrap `t ↦ k·t`, which crosses
    /// the seam and is refused `Seam` (O5); held as `k` sub-enclosures with
    /// `k` seams it is a nested enclosure, not built here.
    Cover { rows: usize, cols: usize },
    /// Every painted output body is an input body moved by this signed swap
    /// and the frame's re-period up to an offset: its bands' boundary turns
    /// equal once each body is centred by its least boundary turn on each
    /// circle (O2 (b)). Placements in a re-periodded frame read here.
    Offset(SignedSwap),
    /// Every painted output body is an input body moved by this signed swap
    /// and re-enclosed: held in its own enclosure (the arcs it spans on each
    /// circle, its seam its least boundary), its turns equal, whatever each
    /// frame's periods. Crops, extractions and bodies re-periodded inside an
    /// unchanged frame read here (D3.2 §D.3).
    Reenclosed(SignedSwap),
    /// None of the above: the residue, whose change stands as its record.
    Unenclosed,
}

/// The periods a body's shape is read in: its frame's, or its own
/// enclosure's.
#[derive(Clone, Copy)]
enum Held {
    InFrame,
    OwnEnclosure,
}

/// The shape of body `body` of `e` in turns, centred by its least boundary
/// turn on each circle (O2 (b)): each band's four boundary turns less the
/// least, as fractions in lowest terms, in band order, of the frame's
/// periods or of the body's own span. Hue is not shape.
fn centred_shape(e: &Enclosed, body: usize, held: Held) -> Vec<u8> {
    use crate::transducer::address::put_key_count;
    // The least and the greatest boundary turn on each circle, as indices
    // of the circle's own period: O2 (b) names the least as the origin, and
    // the two together are the body's enclosure. Neither is a choice.
    let mut span: Option<[usize; 4]> = None;
    for (_, b) in e.body_bands(body) {
        span = Some(match span {
            None => [b.rows.0, b.rows.1, b.cols.0, b.cols.1],
            Some([r0, r1, c0, c1]) => [
                if b.rows.0 < r0 { b.rows.0 } else { r0 },
                if b.rows.1 > r1 { b.rows.1 } else { r1 },
                if b.cols.0 < c0 { b.cols.0 } else { c0 },
                if b.cols.1 > c1 { b.cols.1 } else { c1 },
            ],
        });
    }
    let Some([r_least, r_most, c_least, c_most]) = span else { return Vec::new() };
    let mut out = Vec::new();
    let mut fraction = |n: usize, d: usize| {
        let g = num_integer::gcd(n, d);
        match (n.checked_div(g), d.checked_div(g)) {
            (Some(n), Some(d)) => {
                put_key_count(&mut out, n);
                put_key_count(&mut out, d);
            }
            _ => put_key_count(&mut out, 0),
        }
    };
    let (p, q) = match held {
        Held::InFrame => (e.enclosure.rows, e.enclosure.cols),
        Held::OwnEnclosure => (r_most - r_least, c_most - c_least),
    };
    for (_, b) in e.body_bands(body) {
        fraction(b.rows.0 - r_least, p);
        fraction(b.rows.1 - r_least, p);
        fraction(b.cols.0 - c_least, q);
        fraction(b.cols.1 - c_least, q);
    }
    out
}

/// Is `y` the input `x` (periods `h × w`) repeated across `y`'s enclosure?
/// Read at the sensor, both rasters once.
fn is_cover(x: &Grid, y: &Grid, h: usize, w: usize) -> bool {
    y.iter().enumerate().all(|(r, row)| {
        row.iter().enumerate().all(|(c, &hue)| x.get(r % h).and_then(|xr| xr.get(c % w)) == Some(&hue))
    })
}

/// THE RELATION OF ONE PAIR ([`PairRelation`]): `Empty` or `Ragged` where
/// a frame has no enclosure, `Spent` where a record does not read under the
/// stroke its periods need. Each signed swap is one of the eight, read in
/// [`SignedSwap::ALL`]'s order: the group, never a search tree.
pub fn relation_of(x: &Grid, y: &Grid) -> Result<PairRelation, TransductionRefusal> {
    let from = Enclosed::of(x)?;
    let to = Enclosed::of(y)?;
    for s in SignedSwap::ALL {
        let moved = s.spoken_onto(&from, s.enclosure(from.enclosure))?;
        let need = match super::need_of(&moved, y) {
            Some(n) => n,
            None => 0,
        };
        let t = Transduction::of(&moved, y, &mut crate::physics::Physics::default().stroke_n(need))?;
        if t.relations().iter().all(Result::is_ok) {
            return Ok(PairRelation::Carried(s));
        }
    }
    let (p, q) = (from.enclosure, to.enclosure);
    if q.rows % p.rows == 0 && q.cols % p.cols == 0 && (q.rows, q.cols) != (p.rows, p.cols) {
        let (rows, cols) = (q.rows / p.rows, q.cols / p.cols);
        if is_cover(x, y, p.rows, p.cols) {
            return Ok(PairRelation::Cover { rows, cols });
        }
    }
    let ground = crate::arc::ground();
    let painted: Vec<usize> = (0..to.bodies).filter(|&b| to.body_hue(b) != Some(ground)).collect();
    if painted.is_empty() {
        return Ok(PairRelation::Unenclosed);
    }
    for (held, class) in [
        (Held::InFrame, PairRelation::Offset as fn(SignedSwap) -> PairRelation),
        (Held::OwnEnclosure, PairRelation::Reenclosed),
    ] {
        for s in SignedSwap::ALL {
            let moved = s.moved(&from)?;
            let mut shapes: Keyed<()> = Keyed::new();
            for body in 0..moved.bodies {
                if moved.body_hue(body) != Some(ground) {
                    shapes.insert(centred_shape(&moved, body, held), ());
                }
            }
            if painted.iter().all(|&b| shapes.contains(&centred_shape(&to, b, held))) {
                return Ok(class(s));
            }
        }
    }
    Ok(PairRelation::Unenclosed)
}
