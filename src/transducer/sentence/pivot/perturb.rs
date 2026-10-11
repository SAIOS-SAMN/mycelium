// 78.46(4′) — THE PERTURBED HOLONOMY: a circuit read against its neighbours
// inside one anastomosis, by the PAIR COMPOSITE of versors the engine
// already holds.
//
//! Two circuits `o` and `n` crossing one certified cell (R3: routes are
//! compared inside one anastomosis, never across the form), each based at
//! its component's basepoint by its lasso (R5, Axiom A: `ρ = θ Ω θ̃`),
//! compose to
//!
//! ```text
//!     H(o, n) = ρ_n⁻¹ · ρ_o          (T1, C1(a))
//! ```
//!
//! exact, a versor product, the identity ray exactly where the two routes
//! agree. Several neighbours are never folded into one product — no order is
//! given. The pairs are read by C1(a) without a pairwise walk: `H(o, n)` is
//! the identity ray exactly when `ρ_o` and `ρ_n` are one ray, so a cell's
//! circuits keyed by their based ray's canonical bytes hold every pair at
//! once — two circuits under one key agree, under two keys they do not.
//!
//! A cell's circuits read FLAT together where they all stand under one key
//! (C1(b)), CURVED otherwise; a cell one circuit crosses is "unperturbed" by
//! name. Nothing is filed per pair: the new content of two curved circuits
//! lies in the orbit algebra's bracket closure (78.11), and a vacuum's pair
//! composite `ρ_n⁻¹` is `n`'s own class, never filed twice (law 12).
//!
//! WHERE THE MIND CONSUMES IT: the pivot's trial (78.7, law 10). A move that
//! splits two circuits that agreed — a pair composite the identity before
//! and non-trivial after — has raised a coordinate the move did not act on,
//! and the trial refuses it as `unclosed`. A pair already non-trivial before
//! the move does not refuse it.

use super::*;

/// What the pair composites read, counted where they are read (§8.1).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PerturbTally {
    /// Cells one circuit crosses: "unperturbed".
    pub unperturbed: u64,
    /// Cells whose circuits all agree (C1(b)).
    pub flat: u64,
    /// Cells two of whose circuits disagree.
    pub curved: u64,
    /// Of the circuits crossing cells, the vacuum ones — trivial alone, kept
    /// beside the orbits (79.3), read here against their neighbours.
    pub vacua: u64,
    /// Trial moves the unperturbed trial admitted — discharged, rank held,
    /// no lane risen — that reached the cells' reading.
    pub admitted: u64,
    /// Of those, the moves refused because they split two circuits that
    /// agreed.
    pub unclosed: u64,
    /// Routes refused by the register (R1): a based route that is no versor
    /// or has no canonical ray, or a circuit whose whole transport is unread.
    pub refused: u64,
}

impl PerturbTally {
    pub fn add(&mut self, o: &PerturbTally) {
        self.unperturbed += o.unperturbed;
        self.flat += o.flat;
        self.curved += o.curved;
        self.vacua += o.vacua;
        self.admitted += o.admitted;
        self.unclosed += o.unclosed;
        self.refused += o.refused;
    }

    pub fn line(&self) -> String {
        format!(
            "perturbed (78.46): cells unperturbed {} flat {} curved {} | vacuum circuits read in cells {} | trial moves admitted unperturbed {} refused unclosed {} | routes refused by the register {}",
            self.unperturbed, self.flat, self.curved, self.vacua, self.admitted, self.unclosed, self.refused
        )
    }
}

/// EVERY CERTIFIED CELL'S CIRCUITS BY THEIR BASED RAY: cell key → (circuit
/// chord → the canonical bytes of each of its based routes). Two circuits of
/// one cell agree exactly where their rays' bytes are equal (C1(a)).
#[derive(Debug, Clone, Default)]
pub(crate) struct Agreements {
    cells: Keyed<Keyed<Vec<Vec<u8>>, coord::Overlap>>,
}

impl Agreements {
    /// THE CELLS' AGREEMENTS on the given faces — the form's, or a trial's
    /// after its move — over every cycle, kept or vacuum. The tally reads
    /// each cell flat, curved or unperturbed, and counts a refused route.
    pub(crate) fn read(form: &Form, faces: &Faces, cycles: &Cycles, t: &mut PerturbTally) -> Agreements {
        let mut cells: Keyed<Keyed<Vec<Vec<u8>>, coord::Overlap>> = Keyed::new();
        let vacua_from = cycles.kept.len();
        for (i, lp) in cycles.kept.iter().chain(cycles.vacua.iter()).enumerate() {
            let crossed = form.cells_crossed(lp);
            if crossed.is_empty() {
                continue;
            }
            if i >= vacua_from {
                t.vacua += 1;
            }
            let Some(rays) = based_rays(faces, &form.vertices, lp) else {
                t.refused += 1;
                continue;
            };
            for ck in crossed {
                match cells.get_mut(&ck) {
                    Some(held) => {
                        held.put(&lp.chord, rays.clone());
                    }
                    None => {
                        let mut held = Keyed::new();
                        held.put(&lp.chord, rays.clone());
                        cells.insert(&ck, held);
                    }
                }
            }
        }
        for (_, circuits) in cells.scoped() {
            match circuits.len() {
                0 | 1 => t.unperturbed += 1,
                _ => match one_ray(circuits) {
                    true => t.flat += 1,
                    false => t.curved += 1,
                },
            }
        }
        Agreements { cells }
    }

    /// DOES THE MOVE SPLIT AN AGREEMENT (law 10 under 78.46(4′)): two
    /// circuits of one cell under one ray before, and under two after. A
    /// circuit the move left unread is split from whatever it agreed with —
    /// unread is not agreeing (rule 11). Keyed reads only: each circuit's
    /// before-ray is looked up in a key of its cell, and its after-ray is
    /// compared with the first after-ray filed under that key.
    pub(crate) fn split_by(&self, after: &Agreements) -> bool {
        for (ck, before) in self.cells.scoped() {
            let later = after.cells.get(&ck);
            let mut first_after: Keyed<Option<Vec<Vec<u8>>>> = Keyed::new();
            for (chord, rays) in before.entries() {
                let now = later.and_then(|c| c.at(&chord)).cloned();
                let key = rays_key(rays);
                match first_after.get(&key) {
                    None => {
                        first_after.insert(&key, now);
                    }
                    Some(held) => {
                        if held.is_none() || *held != now {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

/// Every circuit of the cell under one ray.
fn one_ray(circuits: &Keyed<Vec<Vec<u8>>, coord::Overlap>) -> bool {
    let mut keys: Keyed<()> = Keyed::new();
    for (_, rays) in circuits.scoped() {
        keys.insert(rays_key(rays), ());
    }
    keys.len() == 1
}

/// A circuit's routes written as one key, each length-delimited.
fn rays_key(rays: &[Vec<u8>]) -> Vec<u8> {
    let mut buf = Vec::new();
    for r in rays {
        crate::transducer::address::put_key_bytes(&mut buf, r);
    }
    buf
}

/// EACH MEMBER PATH'S HOLONOMY AT THE BASEPOINT, as the coordinates two
/// routes agree on: the three carriers the pivot's class vector reads
/// (`class_vectors_on`), each by its canonical bytes —
///
/// - the carriage based by Axiom A (R5), `θ T θ̃`, `T` the product of the
///   overlaps' carriages the route crosses (R1), the operator
///   `form_operators` bases;
/// - the holonomy word the path carries (`path_word`: its edge composite's
///   where it has one), a ray (EQ1) — words are vertex coordinates and need
///   no basing;
/// - the gauge, the fibre's holonomy.
///
/// `H(o, n)` is the identity in every carrier exactly where these agree. Read
/// quietly — `cycles_of` already counted this circuit's steps. `None` where a
/// route does not base as a versor or has no canonical ray (R1).
fn based_rays(faces: &Faces, vertices: &Vertices, lp: &Orbit) -> Option<Vec<Vec<u8>>> {
    let theta = super::orbits::orbit_based_on(faces, lp)?;
    if !crate::transducer::cga::is_versor(&theta) {
        return None;
    }
    let inv = theta.reverse();
    let carried = paths_carried(faces, vertices, lp)?;
    let mut out = Vec::new();
    for (_, c) in carried.scoped() {
        let rho = theta.geo(&c.face.carriage).geo(&inv);
        if !crate::transducer::cga::is_versor(&rho) {
            return None;
        }
        let mut carriage = Vec::new();
        rho.canonical()?.put_canonical(&mut carriage);
        let mut word = Vec::new();
        super::drive::path_word(c).rotor().put_canonical(&mut word);
        let gauge: Vec<u8> = c.face.gauge.iter().flat_map(|&(a, b)| [a, b]).collect();
        let mut bytes = Vec::new();
        for part in [&carriage, &word, &gauge] {
            crate::transducer::address::put_key_bytes(&mut bytes, part);
        }
        out.push(bytes);
    }
    Some(out)
}

impl Form {
    /// THE CELLS A CIRCUIT CROSSES (R3), by their keys: a step crosses a cell
    /// where the overlap it crosses joins two DIFFERENT bodies of the cell,
    /// read off the overlap's body pair (BD2) — the predicate
    /// `comprehend::crossings` reads a cell's circuits by.
    fn cells_crossed(&self, lp: &Orbit) -> Vec<Vec<u8>> {
        let mut out: Keyed<()> = Keyed::new();
        for s in &lp.transitions {
            let Some((x, y)) = self.face_bodies.at(&s.overlap) else { continue };
            if x == y {
                continue;
            }
            for (ck, cell) in self.cells.scoped() {
                if cell.bodies.contains(x) && cell.bodies.contains(y) {
                    out.insert(&ck, ());
                }
            }
        }
        out.scoped().map(|(k, _)| k.to_vec()).collect()
    }

    /// The form's cells read by their circuits' pair composites, on its own
    /// faces and the cycles its reader already traversed (78.46(4′)).
    pub(crate) fn read_agreements(&self, cycles: &Cycles) -> Agreements {
        Agreements::read(self, &self.faces, cycles, &mut PerturbTally::default())
    }
}
