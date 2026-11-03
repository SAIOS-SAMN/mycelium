// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE SAMPLE'S SEAMS, PEELED (MU-SENSING §3.4, condition 2; the operator's
//! ruling of 2026-11-04): the outer seam of a sample (its sites with some
//! coordinate at `0` or at `extent − 1`), and, where one body covers that
//! whole seam, the body as the sample's MEMBRANE and the inner seam it
//! encloses, level by level. A framed interior is read through its frame;
//! nested frames repeat the reading per level. Read at the sensor, once per
//! sample: sites and their contact of grade `n − 1`, nothing else.

use crate::transducer::sample::{AxisReading, Sample};

/// ONE LEVEL OF A SAMPLE'S SEAMS: the symbols its seam shows, and the
/// symbol of the body covering the whole seam where one body does (the
/// level's membrane, peeled to read the next level inside it).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Level {
    pub seam: Vec<Vec<u8>>,
    pub membrane: Option<Vec<u8>>,
}

/// The neighbours of site `i` in contact of grade `n − 1`: one axis moved
/// by one, inside the extents.
fn neighbours(i: usize, extents: &[usize], strides: &[usize]) -> Vec<usize> {
    let mut out = Vec::new();
    for (&e, &stride) in extents.iter().zip(strides) {
        let c = (i / stride) % e;
        if c > 0 {
            out.push(i - stride);
        }
        if c + 1 < e {
            out.push(i + stride);
        }
    }
    out
}

/// THE LEVELS OF A SAMPLE'S SEAMS, outermost first. The outer seam is
/// level 0. Where every site of a level's seam lies in one body (one
/// symbol, joined by contact of grade `n − 1` among the sites not yet
/// peeled), the body is that level's membrane: it is peeled, and the
/// next level's seam is the unpeeled sites in contact with it. The reading
/// ends at a level no one body covers, or where nothing is left inside.
pub fn levels(x: &Sample) -> Vec<Level> {
    let extents: Vec<usize> = (0..x.rank())
        .filter_map(|axis| match x.extent(axis) {
            AxisReading::Read(e) => Some(e),
            AxisReading::Unread => None,
        })
        .collect();
    let mut strides = vec![1usize; extents.len()];
    for k in (1..extents.len()).rev() {
        if let (Some(&s), Some(&e)) = (strides.get(k), extents.get(k)) {
            if let Some(slot) = strides.get_mut(k - 1) {
                *slot = s * e;
            }
        }
    }
    let symbols: Vec<&[u8]> = x.symbols().map(|s| s.bytes()).collect();
    // Each site's level of peeling: `None` while unpeeled.
    let mut peeled: Vec<Option<usize>> = vec![None; symbols.len()];
    // Each site's membership of the body read at a level: the level.
    let mut member: Vec<Option<usize>> = vec![None; symbols.len()];
    let mut seam: Vec<usize> = (0..symbols.len())
        .filter(|&i| extents.iter().zip(&strides).any(|(&e, &stride)| {
            let c = (i / stride) % e;
            c == 0 || c + 1 == e
        }))
        .collect();
    let mut out = Vec::new();
    let mut level = 0usize;
    loop {
        let mut shown: Vec<Vec<u8>> = Vec::new();
        for &i in &seam {
            if let Some(s) = symbols.get(i) {
                if !shown.iter().any(|h| h.as_slice() == *s) {
                    shown.push(s.to_vec());
                }
            }
        }
        let [start, ..] = seam.as_slice() else { break };
        // The body of the seam's least site: its symbol's sites joined by
        // contact among the unpeeled. Any seam site gives the same body
        // where one body covers the seam, which is all this asks.
        let symbol = symbols.get(*start).copied();
        let mut body = vec![*start];
        if let Some(slot) = member.get_mut(*start) {
            *slot = Some(level);
        }
        let mut at = 0usize;
        while let Some(&i) = body.get(at) {
            at += 1;
            for j in neighbours(i, &extents, &strides) {
                if peeled.get(j) == Some(&None) && symbols.get(j).copied() == symbol && member.get(j) != Some(&Some(level)) {
                    if let Some(slot) = member.get_mut(j) {
                        *slot = Some(level);
                    }
                    body.push(j);
                }
            }
        }
        let covered = seam.iter().all(|&i| member.get(i) == Some(&Some(level)));
        let membrane = match (covered, symbol) {
            (true, Some(s)) => Some(s.to_vec()),
            _ => None,
        };
        let peel = membrane.is_some();
        out.push(Level { seam: shown, membrane });
        if !peel {
            break;
        }
        for &i in &body {
            if let Some(slot) = peeled.get_mut(i) {
                *slot = Some(level);
            }
        }
        let mut next: Vec<usize> = Vec::new();
        for &i in &body {
            for j in neighbours(i, &extents, &strides) {
                if peeled.get(j) == Some(&None) {
                    next.push(j);
                }
            }
        }
        next.sort_unstable();
        next.dedup();
        seam = next;
        level += 1;
    }
    out
}
