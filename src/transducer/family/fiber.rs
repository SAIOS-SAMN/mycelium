// 52.5 item 2, 50.5 — the fibers of a side, keyed chart → frame → bodies,
// and the demands a frame's quotient leaves where it reaches nothing.

use super::perceive::{frame_of_body, Demanded};
use super::*;
use crate::network::nerve::Keyed;
use crate::transducer::sentence::Demand;

/// THE FIBERS CARRIER'S OWN KEY: a frame as `i64`, little-endian, keying
/// `fibers`'s inner trie. It is NOT [`crate::transducer::sentence::frame_key`],
/// which is the frame COORDINATE a section is read at — big-endian, `u16`,
/// and in succession order. Two keys of one frame for two carriers, named
/// apart so a reader cannot take one for the other.
pub(super) fn fiber_key(k: i64) -> [u8; 8] {
    k.to_le_bytes()
}

pub(super) fn fiber_frame_of_key(k: &[u8]) -> Option<i64> {
    Some(i64::from_le_bytes(k.try_into().ok()?))
}

/// A body's address as content: its compartments written out (A21).
pub(super) fn body_key(b: &Body) -> Vec<u8> {
    let mut k = Vec::with_capacity(b.compartments.len() * 29);
    for l in &b.compartments {
        l.site.put(&mut k);
        k.push(l.hue);
    }
    k
}

pub(super) fn compartments_of(bodies: &[Body]) -> Vec<Compartment> {
    bodies.iter().flat_map(|b| b.compartments.iter().cloned()).collect()
}

/// THE FIBERS OF A SIDE: chart key → the chart, and frame → the sections
/// of that chart standing there (Y4, 24.4, 52.5 item 2). Keyed reads all
/// the way down; nothing is read whole to find a body.
pub(super) fn fibers(bodies: &[Body]) -> Keyed<(Invariant, Keyed<Vec<Body>>)> {
    let mut out: Keyed<(Invariant, Keyed<Vec<Body>>)> = Keyed::new();
    for b in bodies {
        let Some(k) = frame_of_body(b) else { continue };
        let ck = b.moduli.key();
        if !out.contains(&ck) {
            out.insert(&ck, (b.moduli.clone(), Keyed::new()));
        }
        let Some((_, frames)) = out.get_mut(&ck) else { continue };
        match frames.get_mut(&fiber_key(k)) {
            Some(v) => v.push(b.clone()),
            None => {
                frames.insert(fiber_key(k), vec![b.clone()]);
            }
        }
    }
    out
}

/// The demands a cover leaves on the union of a chart's fiber at one frame
/// (50.5, 50.8 case 4). An output compartment no piece reached is what a
/// `Source` over the chart stands for, with the number of input instances
/// standing there, every one of them standing and none first-found: the
/// input compartments no piece reached ride the same demand, since they are the
/// bodies the outputs may be the image of. Where every output compartment was
/// reached, an input instance no piece of which was reached vanished — the
/// empty chart on every axis — and what else was left on the input rides a
/// `Source` of its own. The first demand is the one the cover's standing
/// orbits keep alive.
pub(super) fn unreached_demands(
    instances: &[Body],
    a0: &Body,
    b0: &Body,
    reached: &Keyed<()>,
    chart: &Invariant,
    n_in: usize,
) -> Vec<Demanded> {
    let void = crate::arc::ground();
    let mut out_links: Vec<Link> = Vec::new();
    for l in &b0.compartments {
        if let (false, Some((r, c))) = (reached.contains(&l.site.key()), on_grid(&l.site)) {
            out_links.push((r, c, void, l.hue));
        }
    }
    let mut out = Vec::new();
    let mut gone: Keyed<()> = Keyed::new();
    if out_links.is_empty() {
        for body in instances {
            // The instance read in its frame's own coordinates, where the
            // cover measured (16.5).
            let at_zero = super::perceive::at_frame_zero(&body.compartments);
            let untouched = at_zero.iter().all(|l| !reached.contains(&l.site.key()));
            if untouched {
                out.push(vanished(body));
                for l in &at_zero {
                    gone.insert(l.site.key(), ());
                }
            }
        }
    }
    let mut links: Vec<Link> = out_links;
    for l in &a0.compartments {
        let k = l.site.key();
        if let (false, false, Some((r, c))) = (reached.contains(&k), gone.contains(&k), on_grid(&l.site)) {
            links.push((r, c, l.hue, void));
        }
    }
    links.sort_unstable();
    links.dedup();
    if !links.is_empty() || out.is_empty() {
        out.push((Demand::Source { multiplicity: n_in }, chart.clone(), links));
    }
    out
}

/// A body that vanished: the empty chart on every axis (50.5).
pub(super) fn vanished(body: &Body) -> Demanded {
    #[expect(clippy::cast_possible_truncation, reason = "AT1 debt — a slot number of the chart's Vec (an axis position in the body's moduli Invariant); the atlas row replaces the slot by the chart's coordinates")]
    let axes: Vec<u16> = (0..body.moduli.0.len() as u16).collect();
    let links: Vec<Link> = body
        .compartments
        .iter()
        .filter_map(|l| on_grid(&l.site).map(|(r, c)| (r, c, l.hue, crate::arc::ground())))
        .collect();
    (Demand::TopologicalDeformation { image: Invariant(Vec::new()), axes }, body.moduli.clone(), links)
}
