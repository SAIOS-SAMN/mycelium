// 78.74 — THE FIBRES OF A FRAME: every pair of bodies a frame carries is a
// fibre between their pivots, and their relative motion is read against it
// as rigid, torsion, field or lift (`physics::capacitor`).
//
//! A body is a pivot — its centroid in the vertex coordinates its transport
//! acts in, its compartments its mass. Two bodies at one frame are a fibre,
//! and `j`'s motion seen from `i`'s frame, `R = ṽᵢ vⱼ`, is what the fibre
//! undergoes. Torsion does no work (78.74): a frame whose fibres are all
//! carried rigidly or twisted, none stretched in its levels and none lifted,
//! moved no charge and reshaped nothing — its turning is free.
//!
//! BOUNDED BY CLASS, NOT BY PAIRS. Bodies a frame carries by one transport
//! are carried rigidly against one another (`R ∝ 1`) and are counted so,
//! never multiplied out; only pairs across two transports are read, and each
//! transport pair's `R` once.

use crate::physics::capacitor::{shape_of, Carried, Fibre, Shape};
use crate::transducer::cga::Mv;
use crate::transducer::element::Versor;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

/// One body a frame carries: its pivot, mass, transport, and base dimension.
pub(crate) struct Pivot {
    pub frame: u64,
    pub point: Mv,
    pub mass: u64,
    pub carriage: Versor,
    pub n: usize,
}

/// 78.74 — WHAT THE FRAMES' FIBRES DID, in this process.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FibreTally {
    pub pairs: u64,
    pub rigid: u64,
    pub torsion: u64,
    pub field: u64,
    pub lift: u64,
    /// Two bodies on one centroid (or a mass that is no mass): no fibre.
    pub unpivoted: u64,
    pub stable: u64,
    pub emergent: u64,
    pub open: u64,
    /// Frames whose turning did no work on any fibre.
    pub free_frames: u64,
}

const SLOTS: usize = 10;
static FIBRES: [AtomicU64; SLOTS] = [const { AtomicU64::new(0) }; SLOTS];

fn note(slot: usize, by: u64) {
    if FIBRES[slot].fetch_update(Relaxed, Relaxed, |held| held.checked_add(by)).is_err() {
        let mut past = u64::MAX;
        let _ = crate::physics::metabolism::step_or_refuse(&mut past, by);
    }
}

/// The fibres' tally since the process began.
pub fn fibre_tally() -> FibreTally {
    let r = |i: usize| FIBRES[i].load(Relaxed);
    FibreTally {
        pairs: r(0),
        rigid: r(1),
        torsion: r(2),
        field: r(3),
        lift: r(4),
        unpivoted: r(5),
        stable: r(6),
        emergent: r(7),
        open: r(8),
        free_frames: r(9),
    }
}

/// k choose 2, or `None` past the carrier.
fn pairs_in(k: u64) -> Option<u64> {
    k.checked_mul(k.checked_sub(1)?)?.checked_div(2)
}

/// READ EVERY FRAME'S FIBRES; the frames whose turning did no work.
pub(crate) fn read_frames(pivots: &[Pivot]) -> Vec<u64> {
    let mut frames: Vec<u64> = pivots.iter().map(|p| p.frame).collect();
    frames.sort_unstable();
    frames.dedup();
    let mut free = Vec::new();
    for k in frames {
        let here: Vec<&Pivot> = pivots.iter().filter(|p| p.frame == k).collect();
        // The frame's bodies stand on one base: its `n` is the one they
        // share, never chosen among them. Bodies that disagree have no one
        // round to stand on, and the shape is open by name (rule 4).
        let shared = match here.split_first() {
            Some((first, rest)) => rest.iter().all(|p| p.n == first.n).then_some(first.n),
            None => continue,
        };
        let points: Vec<Mv> = here.iter().map(|p| p.point.clone()).collect();
        let shape = match shared {
            Some(n) => shape_of(&points, n),
            None => Shape::Open,
        };
        note(
            match shape {
                Shape::Stable(_) => 6,
                Shape::Emergent => 7,
                Shape::Open => 8,
            },
            1,
        );
        // The frame's transports, each with the bodies it carries.
        let mut classes: Vec<(&Versor, Vec<&Pivot>)> = Vec::new();
        for p in &here {
            match classes.iter_mut().find(|(v, _)| v.projective_eq(&p.carriage)) {
                Some((_, members)) => members.push(p),
                None => classes.push((&p.carriage, vec![p])),
            }
        }
        let mut worked = false;
        let mut read = 0u64;
        for (_, members) in &classes {
            // One transport carries its bodies rigidly against each other:
            // counted, never multiplied out.
            if let Some(n) = pairs_in(members.len() as u64) {
                note(0, n);
                note(1, n);
                if let Some(r) = read.checked_add(n) {
                    read = r;
                }
            }
        }
        for (a, (va, ma)) in classes.iter().enumerate() {
            for (vb, mb) in classes.iter().skip(a + 1) {
                let r = va.reverse().geo(vb);
                for pi in ma {
                    for pj in mb {
                        note(0, 1);
                        let fibre = Fibre::new(
                            pi.point.clone(),
                            pj.point.clone(),
                            num_rational::BigRational::from_integer(pi.mass.into()),
                            num_rational::BigRational::from_integer(pj.mass.into()),
                        );
                        let Some(fibre) = fibre else {
                            note(5, 1);
                            continue;
                        };
                        if let Some(r) = read.checked_add(1) {
                            read = r;
                        }
                        match fibre.carried_by(&r) {
                            Carried::Rigid => note(1, 1),
                            Carried::Torsion => note(2, 1),
                            Carried::Field => {
                                note(3, 1);
                                worked = true;
                            }
                            Carried::Lift => {
                                note(4, 1);
                                worked = true;
                            }
                        }
                    }
                }
            }
        }
        if read > 0 && !worked {
            note(9, 1);
            free.push(k);
        }
    }
    free
}
