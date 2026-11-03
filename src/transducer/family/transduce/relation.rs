// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! O10 — HOW A PAIR'S TWO FRAMES RELATE, typed: is the relation a product of
//! a signed permutation of the phases, the re-period, an offset and a
//! re-enclosure, or a residue no such product carries? A reading of one
//! pair, at any rank, for the census of the corpus; it decides nothing a
//! mind does.

use super::{Enclosed, SignedPermutation, Transduction, TransductionRefusal};
use crate::network::nerve::Keyed;
use crate::transducer::sample::{Sample, Symbol};

/// THE CLASS OF ONE PAIR'S RELATION, the first that holds in this order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PairRelation {
    /// Every input body is carried whole onto one output body by this
    /// signed permutation and the re-period of each phase
    /// (`Transduction::carried`): positions and shapes both, in turns.
    Carried(SignedPermutation),
    /// The output is copies of the input, `copies[k]` along phase `k`, at
    /// least two in all. Read as one map of each phase this is the wrap
    /// `t ↦ k·t`, which crosses the seam and is refused `Seam` (O5); held
    /// as sub-enclosures, each with its own seam, it is a nested enclosure
    /// (MU-SENSING §3.6), not read here.
    Cover { copies: Vec<usize> },
    /// Every painted output body is an input body moved by this signed
    /// permutation and the frame's re-period up to an offset: its bands'
    /// boundary turns equal once each body is centred by its least boundary
    /// turn on each phase (O2 (b)). Placements in a re-periodded frame read
    /// here.
    Offset(SignedPermutation),
    /// Every painted output body is an input body moved by this signed
    /// permutation and re-enclosed: held in its own enclosure (the arcs it
    /// spans on each phase, its seam its least boundary), its turns equal,
    /// whatever each frame's periods. Crops, extractions and bodies
    /// re-periodded inside an unchanged frame read here (D3.2 §D.3).
    Reenclosed(SignedPermutation),
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
/// turn on each phase (O2 (b)): each band's boundary turns less the least,
/// as fractions in lowest terms, in band order, of the frame's periods or
/// of the body's own span. The symbol is not shape.
fn centred_shape(e: &Enclosed, body: usize, held: Held) -> Vec<u8> {
    use crate::transducer::address::put_key_count;
    // The least and the greatest boundary turn on each phase, as indices of
    // the phase's own period: O2 (b) names the least as the origin, and the
    // two together are the body's enclosure. Neither is a choice.
    let mut span: Option<Vec<(usize, usize)>> = None;
    for (_, b) in e.body_bands(body) {
        span = Some(match span {
            None => b.spans.clone(),
            Some(held) => held.iter().zip(&b.spans).map(|(&(l, m), &(lo, hi))| (l.min(lo), m.max(hi))).collect(),
        });
    }
    let Some(span) = span else { return Vec::new() };
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
    let periods: Vec<usize> = match held {
        Held::InFrame => e.enclosure.periods().to_vec(),
        Held::OwnEnclosure => span.iter().map(|&(l, m)| m - l).collect(),
    };
    for (_, b) in e.body_bands(body) {
        for ((&(lo, hi), &(least, _)), &p) in b.spans.iter().zip(&span).zip(&periods) {
            fraction(lo - least, p);
            fraction(hi - least, p);
        }
    }
    out
}

/// Is `y` the input `x` repeated across `y`'s enclosure: the symbol at each
/// site of `y` the symbol of `x` at the site's coordinates modulo `x`'s
/// extents? Read at the sensor, both samples once.
fn is_cover(x: &Sample, y: &Sample, periods: &[usize]) -> bool {
    y.sites().zip(y.symbols()).all(|(site, symbol)| {
        let axes: Vec<i64> = site
            .axes()
            .iter()
            .zip(periods)
            .filter_map(|(&c, &p)| Some(c.rem_euclid(i64::try_from(p).ok()?)))
            .collect();
        crate::transducer::base::Site::new(axes).is_some_and(|s| x.symbol_at(&s) == Ok(symbol))
    })
}

/// THE RELATION OF ONE PAIR ([`PairRelation`]) on the family's `ground`:
/// `Rank` where the frames differ in rank, `Spent` where a record does not
/// read under the stroke its periods need. The signed permutations are read
/// in [`SignedPermutation::all`]'s order: the group generated, never a
/// search tree.
pub fn relation_of(x: &Sample, y: &Sample, ground: Symbol<'_>) -> Result<PairRelation, TransductionRefusal> {
    let from = Enclosed::of(x);
    let to = Enclosed::of(y);
    let (p, q) = (from.enclosure.periods().to_vec(), to.enclosure.periods().to_vec());
    if p.len() != q.len() {
        return Err(TransductionRefusal::Rank { from: p.len(), to: q.len() });
    }
    for s in SignedPermutation::all(p.len()) {
        let moved = s.spoken_onto(&from, &s.enclosure(&from.enclosure)?)?;
        let need = match super::need_of(&moved, y) {
            Some(n) => n,
            None => 0,
        };
        let t = Transduction::of(&moved, y, &mut crate::physics::Physics::default().stroke_n(need))?;
        if t.relations().iter().all(Result::is_ok) {
            return Ok(PairRelation::Carried(s));
        }
    }
    let divides = p.iter().zip(&q).all(|(&a, &b)| b % a == 0);
    if divides && p != q {
        let copies: Vec<usize> = p.iter().zip(&q).map(|(&a, &b)| b / a).collect();
        if is_cover(x, y, &p) {
            return Ok(PairRelation::Cover { copies });
        }
    }
    let painted: Vec<usize> = (0..to.bodies).filter(|&b| to.body_symbol(b) != Some(ground)).collect();
    if painted.is_empty() {
        return Ok(PairRelation::Unenclosed);
    }
    for (held, class) in [
        (Held::InFrame, PairRelation::Offset as fn(SignedPermutation) -> PairRelation),
        (Held::OwnEnclosure, PairRelation::Reenclosed),
    ] {
        for s in SignedPermutation::all(p.len()) {
            let moved = s.moved(&from)?;
            let mut shapes: Keyed<()> = Keyed::new();
            for body in 0..moved.bodies {
                if moved.body_symbol(body) != Some(ground) {
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
