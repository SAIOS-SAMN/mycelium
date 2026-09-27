// N4, RX1–RX7 — RESONANCE: the bodies of two held perceptions heard against
// each other, every body with every body on the charts they share; the
// filaments that resonance grows, the tips that found no partner, and the
// rings the filaments close, order by order.
//
//! A perception holds its bodies — its SPORES — by the chart each stands on,
//! at each frame it was perceived at. Two perceptions MEET where their covers
//! share a chart, and on each leaf of that meet EVERY body of the one is
//! heard against EVERY body of the other: where a versor carries the one onto
//! the other and the pair reads the same from both sides (RX1), the two
//! RESONATE through the harmonic order the fit holds through (RX3) and a
//! FILAMENT grows between them, its residue above that order the filament's
//! demand (RX6 at the edge). A body that resonates with several grows several
//! filaments — the hypha branches, nothing is chosen. A body that resonates
//! with none is a FORAGING TIP: what changed between the scenes, its own
//! harmonics filed as a demand. Nothing is related by a key: a key is where a
//! spore or a filament is filed, never what relates it.
//!
//! A RING of three filaments across three perceptions composes their affine
//! actions on each plane's harmonic at each order (RX4). The residues are
//! exact differences of the bodies' own harmonics, `r = h_b − Ad(h_a)`, so
//! around any ring they telescope: the composed map fixes the first body's
//! harmonic, `F(h) = λ(h − h_A) + h_A`, and the ring's residue is `(1 − λ)
//! h_A`. So a ring is coherent at an order exactly where its linear holonomy
//! `λ_k` is the identity (FILLED) or of finite order `ℓ` (CONGRUENT(ℓ)), the
//! `ℓ`-circuit sum then vanishing by itself (RX5); the lowest order where
//! `λ_k` has no finite order is the ring's split, `F(0)` its residue (RX6).

use super::*;
use crate::transducer::address::{put_key_bytes, take_key_bytes};
use crate::transducer::blade::{self, Harmonic, NotReciprocal, PlaneCarriage, Shape, Transition};
use crate::transducer::cga::Mv;
use num_traits::{One, Zero};

/// A perception's spores: its bodies by the chart each stands on, its input
/// frames' (`[0]`) and its output frames' (`[1]`), each chart's bodies in the
/// order the frames hold them.
pub type Spores = [Keyed<Vec<Shape>>; 2];

/// The bodies a family's frames were perceived as, by the chart each stands
/// on (its moduli), input and output apart. The chart files a body; it
/// relates nothing.
pub fn spores_of(frames: &crate::transducer::family::Frames) -> Spores {
    let mut spores: Spores = [Keyed::new(), Keyed::new()];
    for (ins, outs) in frames {
        for (side, bodies) in [(0usize, ins), (1usize, outs)] {
            for b in bodies {
                let key = b.moduli.key();
                match spores[side].get_mut(key.as_bytes()) {
                    Some(v) => v.push(b.shape.clone()),
                    None => {
                        spores[side].insert(key.as_bytes(), vec![b.shape.clone()]);
                    }
                }
            }
        }
    }
    spores
}

/// A BODY'S COORDINATES, written out: its nucleus, its inertia and its
/// quartic, exact.
fn body_coordinates(s: &Shape) -> Vec<u8> {
    let mut body = Vec::new();
    s.blade.spatial.to_bytes(&mut body);
    for x in s.blade.inertia.iter().chain(s.blade.quartic.iter().flat_map(|t| t.iter())) {
        Mv::scalar(x.clone()).to_bytes(&mut body);
    }
    body
}

/// WHERE A PERCEPTION IS FILED BEYOND ITS FORM (A21): its spores written out —
/// each side's charts in key order, and on each chart every body's
/// coordinates. Two scenes of one form whose bodies stand differently are two
/// perceptions. An address, never a relation.
pub fn spores_key(spores: &Spores) -> Vec<u8> {
    let mut k = Vec::new();
    for side in spores {
        for (chart, bodies) in side.scoped() {
            put_key_bytes(&mut k, &chart);
            for s in bodies {
                put_key_bytes(&mut k, &body_coordinates(s));
            }
        }
        // The side's close: an input chart never reads as an output one.
        put_key_bytes(&mut k, &[]);
    }
    k
}

/// WHERE A HELD FORM IS FILED: a perception at its form's key and its spores
/// written out; any other idea at its form's key alone. The one derivation —
/// the hold, the drive's rekey and every reader of `Bubble::key` go through
/// it, so a perception is never refiled under its bare form.
pub(crate) fn perceived_key(form_key: crate::transducer::address::Address, spores: &Spores) -> crate::transducer::address::Address {
    if spores.iter().all(|s| s.is_empty()) {
        return form_key;
    }
    let mut k = form_key.as_bytes().to_vec();
    put_key_bytes(&mut k, &spores_key(spores));
    crate::transducer::address::Address::from(k)
}

/// A SPORE'S ADDRESS in the hyphae: its perception's ordinal, the chart it
/// stands on, its coordinates — each delimited, so a spore's address is a
/// prefix of every filament it grows.
fn spore_key(ord: u16, chart: &[u8], coords: &[u8]) -> Vec<u8> {
    let mut k = ord.to_le_bytes().to_vec();
    put_key_bytes(&mut k, chart);
    put_key_bytes(&mut k, coords);
    k
}

/// A spore's address read back: its perception's ordinal and the chart it
/// stands on, and where the address ends.
fn spore_at(key: &[u8]) -> Option<(u16, Vec<u8>, usize)> {
    let [lo, hi, ..] = key else { return None };
    let ord = u16::from_le_bytes([*lo, *hi]);
    let mut p = 2;
    let chart = take_key_bytes(key, &mut p)?;
    take_key_bytes(key, &mut p)?;
    Some((ord, chart, p))
}

/// WHY A PAIR DOES NOT RESONATE, each by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unrelated {
    /// The two perceptions' covers share no chart.
    Unshared,
    /// No versor carries the one body onto the other: dissonant.
    Uncarried(blade::NoPose),
    /// RX1 — the pair reads differently from its two sides.
    NotReciprocal(NotReciprocal),
    /// The stroke was spent first.
    Unbounded,
}

/// ONE FILAMENT between two resonating bodies: the leaf chart they stand on,
/// each body's coordinates, and the transition read from each side.
pub struct Filament {
    pub chart: Vec<u8>,
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub there: Transition,
    pub back: Transition,
}

/// WHAT TWO PERCEPTIONS' BODIES DID WHEN HEARD AGAINST EACH OTHER: the
/// filaments grown, the bodies that grew several, the foraging tips — each
/// with its chart and its own harmonics — and each dissonant pair's name.
pub struct Resonance {
    pub filaments: Vec<Filament>,
    pub branching: u64,
    pub foraging: Vec<(Vec<u8>, Vec<Mv>)>,
    pub dissonant: Vec<Unrelated>,
}

/// WHAT A RING OF THREE FILAMENTS READ AT ONE ORDER (RX5, RX6).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RingOrder {
    /// `λ_k = 1` on every plane: closed as it is.
    Filled,
    /// `λ_k` of order `ℓ` on every plane that did not fill.
    Congruent(u32),
    /// On this plane `λ_k` has no finite order the algebra admits: the ring
    /// splits here, `F(0)` its residue.
    Split { plane: (usize, usize), residue: [num_rational::BigRational; 2] },
    /// A plane the ring carries off itself, or a transport that did not
    /// read: unread by name.
    Unread,
}

/// THE RESONANCE READING: counts cumulative at their producer (§6.5), and
/// the hyphae's shape — spores, filaments, components, cycle rank, 2-core —
/// read off the filaments held when the reading is taken.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ResonanceTally {
    /// Perception pairs that grew at least one filament.
    pub related: u64,
    pub unshared: u64,
    /// Dissonant body pairs, by why: no versor; RX1 by what failed (no
    /// reverse, the order, a residue); the stroke spent.
    pub uncarried: u64,
    pub not_reciprocal: [u64; 3],
    pub unbounded: u64,
    /// Filaments grown, by the order they resonate through: 2, 3, 4.
    pub filaments: [u64; 3],
    /// Bodies that grew several filaments toward one perception.
    pub branching: u64,
    /// Bodies that resonated with nothing on a shared chart.
    pub foraging: u64,
    /// RX6 at the edge: filaments whose fit leaves a residue; the demands
    /// filed for them and for the foraging tips (a class that did not read
    /// files none, counted).
    pub edge_split: u64,
    pub demanded: u64,
    pub undemanded: u64,
    /// Rings read: coherent through order 2, 3, 4; split at 2, 3, 4; unread.
    pub coherent: [u64; 3],
    pub split: [u64; 3],
    pub rings_unread: u64,
    /// The hyphae as held: spores, filaments (each once), components, cycle
    /// rank `E − V + C`, and the spores of its 2-core.
    pub spores: u64,
    pub held: u64,
    pub components: u64,
    pub cycle_rank: u64,
    pub core: u64,
}

impl ResonanceTally {
    pub fn line(&self) -> String {
        format!(
            "resonance: perceptions related {} | dissonant: unshared {} uncarried {} not reciprocal (reverse {} order {} residue {}) unbounded {} | filaments through 2/3/4 {:?} branching {} foraging tips {} | edge splits {} demanded {} undemanded {} | rings coherent through 2/3/4 {:?} split at 2/3/4 {:?} unread {} | hyphae: spores {} filaments {} components {} cycle rank {} 2-core {}",
            self.related,
            self.unshared,
            self.uncarried,
            self.not_reciprocal[0],
            self.not_reciprocal[1],
            self.not_reciprocal[2],
            self.unbounded,
            self.filaments,
            self.branching,
            self.foraging,
            self.edge_split,
            self.demanded,
            self.undemanded,
            self.coherent,
            self.split,
            self.rings_unread,
            self.spores,
            self.held,
            self.components,
            self.cycle_rank,
            self.core,
        )
    }

    /// Every count in the reading's order, one place (write and read both).
    fn fields(&mut self) -> [&mut u64; 27] {
        let [nr0, nr1, nr2] = &mut self.not_reciprocal;
        let [f0, f1, f2] = &mut self.filaments;
        let [c0, c1, c2] = &mut self.coherent;
        let [s0, s1, s2] = &mut self.split;
        [
            &mut self.related,
            &mut self.unshared,
            &mut self.uncarried,
            nr0,
            nr1,
            nr2,
            &mut self.unbounded,
            f0,
            f1,
            f2,
            &mut self.branching,
            &mut self.foraging,
            &mut self.edge_split,
            &mut self.demanded,
            &mut self.undemanded,
            c0,
            c1,
            c2,
            s0,
            s1,
            s2,
            &mut self.rings_unread,
            &mut self.spores,
            &mut self.held,
            &mut self.components,
            &mut self.cycle_rank,
            &mut self.core,
        ]
    }

    /// The reading on the observer's wire.
    pub fn put(&self, out: &mut Vec<u8>) {
        for v in self.clone().fields() {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }

    /// The reading off the observer's wire.
    pub fn take(b: &[u8], p: &mut usize) -> Option<ResonanceTally> {
        let mut t = ResonanceTally::default();
        for v in t.fields() {
            *v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
        }
        Some(t)
    }

    fn note(&mut self, why: Unrelated) {
        match why {
            Unrelated::Unshared => self.unshared += 1,
            Unrelated::Uncarried(_) => self.uncarried += 1,
            Unrelated::NotReciprocal(NotReciprocal::NoReverse) => self.not_reciprocal[0] += 1,
            Unrelated::NotReciprocal(NotReciprocal::OrderAsymmetric) => self.not_reciprocal[1] += 1,
            Unrelated::NotReciprocal(NotReciprocal::Residue(_)) => self.not_reciprocal[2] += 1,
            Unrelated::Unbounded => self.unbounded += 1,
        }
    }
}

/// An order's place in the tally's rows.
fn order_ix(k: Harmonic) -> usize {
    match k {
        Harmonic::Two => 0,
        Harmonic::Three => 1,
        Harmonic::Four => 2,
    }
}

/// A residue on its plane as the element it is: `a + b·e_ij`, the plane's
/// own even element — exact, never a ray.
fn residue_element(plane: (usize, usize), v: &[num_rational::BigRational; 2]) -> Mv {
    let bivector = crate::transducer::cga::base_axis(plane.0 + 1) | crate::transducer::cga::base_axis(plane.1 + 1);
    Mv::scalar(v[0].clone()).add(&Mv::blade(bivector, v[1].clone()))
}

/// A FORAGING TIP'S HARMONICS: every order's harmonic of the region on every
/// plane of its base, each as its plane's element — what the body is, that
/// nothing on the other side resonated with.
fn moments_of(s: &Shape) -> Vec<Mv> {
    let n = s.blade.n();
    let mut out = Vec::new();
    for p in crate::transducer::base::planes(n) {
        for k in Harmonic::ALL {
            if let Some(h) = blade::harmonic(&s.blade, p, blade::Tower::Region, k) {
                out.push(residue_element(p, &h));
            }
        }
    }
    out
}

/// RX4 — ONE ORDER OF A RING, ON ONE PLANE: the composed affine action of its
/// transitions on the harmonic of order `k` on plane `p`, as the function it
/// is. `None` where a transport does not read or the ring carries `p` onto
/// another plane.
fn ring_map(ring: &[&Transition], n: usize, p: (usize, usize), k: Harmonic) -> Option<impl Fn(&[num_rational::BigRational; 2]) -> [num_rational::BigRational; 2]> {
    let mut plane = p;
    let mut steps: Vec<(PlaneCarriage, [num_rational::BigRational; 2])> = Vec::with_capacity(ring.len());
    for t in ring {
        let c = PlaneCarriage::of(&t.rotation, &t.rho, n, plane)?;
        let zero = [num_rational::BigRational::zero(), num_rational::BigRational::zero()];
        let r = t.residue.iter().find(|r| r.order == k && r.plane == c.image).map_or(zero, |r| r.vector.clone());
        plane = c.image;
        steps.push((c, r));
    }
    (plane == p).then_some(move |h: &[num_rational::BigRational; 2]| {
        steps.iter().fold(h.clone(), |acc, (c, r)| {
            let carried = c.carry(&acc, k);
            [&carried[0] + &r[0], &carried[1] + &r[1]]
        })
    })
}

/// RX5, RX6 — A RING'S READING AT EACH ORDER, lowest first: on every plane
/// of the base, the affine holonomy's linear part tested by its action on
/// the plane's two axes and its fixed point (`F(0)` against `F` of the
/// axes): FILLED where `F` is the identity, CONGRUENT(ℓ) where `F^ℓ` is, the
/// least `ℓ` of the algebra's cyclotomic orders. The first order a plane
/// splits at ends the reading.
pub fn ring_reading(ring: &[&Transition], n: u8) -> Vec<(Harmonic, RingOrder)> {
    use num_rational::BigRational as Q;
    let probes = [[Q::zero(), Q::zero()], [Q::one(), Q::zero()], [Q::zero(), Q::one()]];
    let orders = crate::transducer::cga::cyclotomic_orders(n);
    let n = usize::from(n);
    let mut out = Vec::new();
    for k in Harmonic::ALL {
        let mut ell: Option<u32> = None;
        for p in crate::transducer::base::planes(n) {
            let Some(f) = ring_map(ring, n, p, k) else {
                out.push((k, RingOrder::Unread));
                return out;
            };
            if probes.iter().all(|h| f(h) == *h) {
                continue;
            }
            let power = |l: u32, h: &[Q; 2]| (0..l).fold(h.clone(), |acc, _| f(&acc));
            match orders.iter().copied().filter(|&l| l >= 2).find(|&l| probes.iter().all(|h| power(l, h) == *h)) {
                Some(l) => ell = Some(ell.map_or(l, |e| num_integer::Integer::lcm(&e, &l))),
                None => {
                    out.push((k, RingOrder::Split { plane: p, residue: f(&probes[0]) }));
                    return out;
                }
            }
        }
        // Every plane that did not fill closed at a finite order, whose
        // least common multiple is the ring's; none unfilled is filled.
        out.push((
            k,
            match ell {
                None => RingOrder::Filled,
                Some(l) => RingOrder::Congruent(l),
            },
        ));
    }
    out
}

/// A perception's input bodies on one chart; none where it holds none there.
fn input_bodies_on<'a>(p: &'a Bubble, chart: &[u8]) -> &'a [Shape] {
    match p.spores[0].get(chart) {
        Some(bodies) => bodies,
        None => &[],
    }
}

/// ONE BODY HEARD AGAINST ANOTHER (RX1, RX3): the coset of versors carrying
/// `a` onto `b`, its representative, and the reverse read from `b`'s side.
fn resonate(a: &Shape, b: &Shape, budget: &mut crate::physics::Budget) -> Result<(Transition, Transition), Unrelated> {
    let coset = |x: &Shape, y: &Shape, budget: &mut crate::physics::Budget| {
        blade::versor_coset(x, y, budget).map_err(|why| match why {
            blade::NoPose::Unbounded => Unrelated::Unbounded,
            other => Unrelated::Uncarried(other),
        })
    };
    let ab = coset(a, b, budget)?;
    let ba = coset(b, a, budget)?;
    let t = blade::coset_representative(&ab).ok_or(Unrelated::Uncarried(blade::NoPose::Ungauged))?;
    let back = blade::reciprocal(t, &ba, a.blade.n()).map_err(Unrelated::NotReciprocal)?;
    Ok((t.clone(), back.clone()))
}

impl Contemplation {
    /// THE MEET'S LEAVES between two held ideas: the shared charts no other
    /// shared chart refines — a cover is a chain of prefixes, and its deepest
    /// member is the one nothing in it extends.
    pub(crate) fn meet_leaves(&self, a: &[u8], b: &[u8]) -> Vec<Vec<u8>> {
        let Some(meet) = self.shared_chain(&[a.to_vec(), b.to_vec()]) else { return Vec::new() };
        meet.iter().filter(|k| !meet.iter().any(|o| o.len() > k.len() && o.starts_with(k))).cloned().collect()
    }

    /// THE RESONANCE OF TWO PERCEPTIONS (N4, RX1–RX3): on each leaf of their
    /// meet, every input body of the one heard against every input body of
    /// the other. Each pair that resonates grows a filament; a body that
    /// resonates with several grows them all; a body that resonates with
    /// none, on either side, is a foraging tip. Refused only where the two
    /// share no chart, or the stroke is spent.
    pub(crate) fn resonate_percepts(&self, a: &[u8], b: &[u8], budget: &mut crate::physics::Budget) -> Result<Resonance, Unrelated> {
        let (pa, pb) = (self.bubbles.get(a).ok_or(Unrelated::Unshared)?, self.bubbles.get(b).ok_or(Unrelated::Unshared)?);
        let leaves = self.meet_leaves(a, b);
        if leaves.is_empty() {
            return Err(Unrelated::Unshared);
        }
        let mut out = Resonance { filaments: Vec::new(), branching: 0, foraging: Vec::new(), dissonant: Vec::new() };
        for leaf in &leaves {
            let (xs, ys) = (input_bodies_on(pa, leaf), input_bodies_on(pb, leaf));
            let mut heard_y = vec![0u64; ys.len()];
            for x in xs {
                let mut heard_x = 0u64;
                for (y, hy) in ys.iter().zip(heard_y.iter_mut()) {
                    match resonate(x, y, budget) {
                        Ok((there, back)) => {
                            heard_x += 1;
                            *hy += 1;
                            out.filaments.push(Filament {
                                chart: leaf.clone(),
                                from: body_coordinates(x),
                                to: body_coordinates(y),
                                there,
                                back,
                            });
                        }
                        Err(Unrelated::Unbounded) => return Err(Unrelated::Unbounded),
                        Err(why) => out.dissonant.push(why),
                    }
                }
                match heard_x {
                    0 => out.foraging.push((leaf.clone(), moments_of(x))),
                    1 => {}
                    _ => out.branching += 1,
                }
            }
            for (y, hy) in ys.iter().zip(&heard_y) {
                match hy {
                    0 => out.foraging.push((leaf.clone(), moments_of(y))),
                    1 => {}
                    _ => out.branching += 1,
                }
            }
        }
        // A body on a chart the two do not meet on has nothing on the other
        // side to resonate with: a foraging tip, on either side.
        for p in [pa, pb] {
            for (chart, bodies) in p.spores[0].scoped() {
                if !leaves.contains(&chart) {
                    for s in bodies {
                        out.foraging.push((chart.clone(), moments_of(s)));
                    }
                }
            }
        }
        Ok(out)
    }

    /// The filaments from perception `a` to perception `b`, each once.
    #[cfg(test)]
    pub(crate) fn filaments_between(&self, a: &[u8], b: &[u8]) -> Vec<Transition> {
        let (Some(&oa), Some(&ob)) = (self.ordinals.get(a), self.ordinals.get(b)) else { return Vec::new() };
        let Some(fiber) = self.hyphae.fiber_at(&oa.to_le_bytes()) else { return Vec::new() };
        fiber.filter(|(k, _)| Self::far_end(k).is_some_and(|(o, _)| o == ob)).map(|(_, t)| t.clone()).collect()
    }

    /// A filament's far spore: its perception's ordinal and its address,
    /// read off the filament's key after the near spore's address.
    fn far_end(key: &[u8]) -> Option<(u16, Vec<u8>)> {
        let (_, _, end) = spore_at(key)?;
        let far = key.get(end..)?;
        let (o, _, _) = spore_at(far)?;
        Some((o, far.to_vec()))
    }

    /// N4 — THE FILAMENTS A NEW PERCEPTION GROWS: with every held perception
    /// its cover meets, its bodies heard against theirs. Each filament is
    /// filed both ways at its spores' addresses; a filament whose fit leaves
    /// a residue files it as its demand (RX6), and a foraging tip its own
    /// harmonics. Then the rings through the new perception are read.
    pub(crate) fn link_perceived(&mut self, key: &[u8], budget: &mut crate::physics::Budget) {
        let Some(me) = self.bubbles.get(key) else { return };
        if me.spores[0].is_empty() {
            return;
        }
        let Some(ok) = self.ordinals.get(key).copied() else { return };
        let charts: Vec<Vec<u8>> = me.cover.scoped().map(|(k, _)| k).collect();
        let mut neighbors: Keyed<()> = Keyed::new();
        for ck in &charts {
            for (id, _) in self.by_chart.link(ck) {
                let Some(addr) = Self::ordinal_of_key(&id).and_then(|o| self.address_at(o)) else { continue };
                if addr != key && self.bubbles.get(addr).is_some_and(|b| !b.spores[0].is_empty()) {
                    neighbors.insert(addr, ());
                }
            }
        }
        let nbs: Vec<Vec<u8>> = neighbors.scoped().map(|(k, _)| k).collect();
        for n in &nbs {
            let heard = match self.resonate_percepts(key, n, budget) {
                Ok(r) => r,
                Err(why) => {
                    self.resonance.note(why);
                    continue;
                }
            };
            let Some(on) = self.ordinals.get(n.as_slice()).copied() else { continue };
            for why in heard.dissonant {
                self.resonance.note(why);
            }
            self.resonance.branching += heard.branching;
            if !heard.filaments.is_empty() {
                self.resonance.related += 1;
            }
            for f in heard.filaments {
                let (near, far) = (spore_key(ok, &f.chart, &f.from), spore_key(on, &f.chart, &f.to));
                self.resonance.filaments[order_ix(f.there.through)] += 1;
                self.file_edge_split(&f.there, &f.chart);
                self.hyphae.insert([near.as_slice(), far.as_slice()].concat(), f.there);
                self.hyphae.insert([far.as_slice(), near.as_slice()].concat(), f.back);
            }
            for (chart, moments) in heard.foraging {
                self.resonance.foraging += 1;
                self.file_foraging(&chart, moments);
            }
        }
        self.read_rings(ok);
    }

    /// A retracted perception's filaments leave with it, both ways.
    pub(crate) fn cut_hyphae(&mut self, ord: u16) {
        let out: Vec<Vec<u8>> = match self.hyphae.fiber_at(&ord.to_le_bytes()) {
            Some(fiber) => fiber.map(|(k, _)| k).collect(),
            None => return,
        };
        for k in out {
            if let Some((_, far)) = Self::far_end(&k) {
                let near = &k[..k.len() - far.len()];
                self.hyphae.remove(&[far.as_slice(), near].concat());
            }
            self.hyphae.remove(&k);
        }
    }

    /// A PERCEPTION'S FILAMENTS LIFTED OFF IT as it is refiled: each as its
    /// near spore past the ordinal, its far spore, and both directions.
    pub(crate) fn lift_hyphae(&mut self, key: &[u8]) -> Vec<(Vec<u8>, Vec<u8>, Transition, Transition)> {
        let Some(ord) = self.ordinals.get(key).copied() else { return Vec::new() };
        let out: Vec<Vec<u8>> = match self.hyphae.fiber_at(&ord.to_le_bytes()) {
            Some(fiber) => fiber.map(|(k, _)| k).collect(),
            None => return Vec::new(),
        };
        let mut lifted = Vec::new();
        for k in out {
            let Some((_, far)) = Self::far_end(&k) else { continue };
            let near = &k[..k.len() - far.len()];
            let back = self.hyphae.remove(&[far.as_slice(), near].concat());
            if let (Some(there), Some(back)) = (self.hyphae.remove(&k), back) {
                lifted.push((near[2..].to_vec(), far, there, back));
            }
        }
        lifted
    }

    /// The lifted filaments refiled at the perception's ordinal where it
    /// now stands, both ways.
    pub(crate) fn refile_hyphae(&mut self, key: &[u8], lifted: Vec<(Vec<u8>, Vec<u8>, Transition, Transition)>) {
        let Some(ord) = self.ordinals.get(key).copied() else { return };
        for (rest, far, there, back) in lifted {
            let near = [&ord.to_le_bytes()[..], &rest].concat();
            self.hyphae.insert([near.as_slice(), far.as_slice()].concat(), there);
            self.hyphae.insert([far.as_slice(), near.as_slice()].concat(), back);
        }
    }

    /// The demand's class where no turn is read: the identity's.
    fn still_class() -> Option<sentence::ClassVector> {
        sentence::ClassVector::of_rotor(
            &crate::transducer::alphabet::Rotor::identity(),
            crate::transducer::element::identity_versor(),
            sentence::CycleType::Identity,
        )
    }

    /// RX6 AT THE EDGE — a filament whose fit holds through `m < 4` leaves its
    /// residue above `m`: the lowest order's residue on each plane is filed
    /// as a demand on the filament's chart, the residue exact, its class the
    /// transition's own turn.
    fn file_edge_split(&mut self, t: &Transition, chart: &[u8]) {
        // The lowest order a residue stands at, read up the filtration.
        let Some(lowest) = Harmonic::ALL.into_iter().find(|k| t.residue.iter().any(|r| r.order == *k)) else { return };
        self.resonance.edge_split += 1;
        let charts: Vec<stalk::Invariant> = stalk::Invariant::from_key(chart).into_iter().collect();
        let class = crate::transducer::element::base_rotor_of(&t.rotation).ok().and_then(|r| {
            sentence::ClassVector::of_rotor(&r, crate::transducer::element::identity_versor(), sentence::CycleType::Identity)
        });
        let Some(class) = class else {
            self.resonance.undemanded += 1;
            return;
        };
        for r in t.residue.iter().filter(|r| r.order == lowest) {
            self.docket_residue(&class, &charts, residue_element(r.plane, &r.vector));
            self.resonance.demanded += 1;
        }
    }

    /// A FORAGING TIP ON THE DOCKET: a body nothing on the other side
    /// resonated with, its harmonics each filed as a demand on its chart —
    /// what changed between the scenes, held as the question it is.
    fn file_foraging(&mut self, chart: &[u8], moments: Vec<Mv>) {
        let Some(class) = Self::still_class() else {
            self.resonance.undemanded += 1;
            return;
        };
        let charts: Vec<stalk::Invariant> = stalk::Invariant::from_key(chart).into_iter().collect();
        for m in moments {
            self.docket_residue(&class, &charts, m);
            self.resonance.demanded += 1;
        }
    }

    /// RX4–RX6 — THE RINGS THROUGH A NEW PERCEPTION: each filament from one of
    /// its spores to a spore of another perception, on to a spore of a third,
    /// and back — a ring of three filaments, read order by order from the new
    /// perception, each ring once (its two far spores in key order).
    fn read_rings(&mut self, ord: u16) {
        let mut rings: Vec<[Transition; 3]> = Vec::new();
        let Some(fiber) = self.hyphae.fiber_at(&ord.to_le_bytes()) else { return };
        for (k1, t1) in fiber {
            let Some((o1, s1)) = Self::far_end(&k1) else { continue };
            let s0 = &k1[..k1.len() - s1.len()];
            let Some(onward) = self.hyphae.fiber_at(&s1) else { continue };
            for (k2, t2) in onward {
                let Some((o2, s2)) = Self::far_end(&k2) else { continue };
                if o2 == ord || o2 == o1 || s2 <= s1 {
                    continue;
                }
                if let Some(t3) = self.hyphae.get(&[s2.as_slice(), s0].concat()) {
                    rings.push([t1.clone(), t2.clone(), t3.clone()]);
                }
            }
        }
        for [t1, t2, t3] in rings {
            // The base the ring's transitions act on.
            let n = t1.versor.n();
            let reading = ring_reading(&[&t1, &t2, &t3], n);
            // The filtration read up to where it stopped: a split at the
            // top, coherent through the order beneath it.
            match reading.as_slice() {
                [.., (m, _), (k, RingOrder::Split { .. })] => {
                    self.resonance.split[order_ix(*k)] += 1;
                    self.resonance.coherent[order_ix(*m)] += 1;
                }
                [(k, RingOrder::Split { .. })] => self.resonance.split[order_ix(*k)] += 1,
                [.., (_, RingOrder::Unread)] | [] => self.resonance.rings_unread += 1,
                [.., (k, _)] => self.resonance.coherent[order_ix(*k)] += 1,
            }
        }
    }

    /// THE RESONANCE READING with the hyphae's shape read now: every spore a
    /// filament holds, the filaments each once, the components they join,
    /// the cycle rank `E − V + C`, and the 2-core — the spores left once
    /// every spore on fewer than two filaments is peeled, repeatedly.
    pub(crate) fn resonance_reading(&self) -> ResonanceTally {
        let mut degree: Keyed<u64> = Keyed::new();
        let mut ends: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
        for (k, _) in self.hyphae.scoped() {
            let Some((_, far)) = Self::far_end(&k) else { continue };
            let near = k[..k.len() - far.len()].to_vec();
            match degree.get_mut(&near) {
                Some(d) => *d += 1,
                None => {
                    degree.insert(&near, 1);
                }
            }
            ends.push((near, far));
        }
        let mut r = self.resonance;
        r.spores = degree.len() as u64;
        r.held = (ends.len() / 2) as u64;
        // Components: each unseen spore starts one, walked by its filaments.
        let mut seen: Keyed<()> = Keyed::new();
        for (s, _) in degree.scoped() {
            if seen.contains(&s) {
                continue;
            }
            r.components += 1;
            seen.insert(&s, ());
            let mut stack = vec![s];
            while let Some(v) = stack.pop() {
                let Some(fiber) = self.hyphae.fiber_at(&v) else { continue };
                for (k, _) in fiber {
                    if let Some((_, far)) = Self::far_end(&k) {
                        if !seen.contains(&far) {
                            seen.insert(&far, ());
                            stack.push(far);
                        }
                    }
                }
            }
        }
        // Every filament is filed both ways (`link_perceived`, `cut_hyphae`),
        // so each component of `v` spores holds at least `v − 1` of them.
        r.cycle_rank = r.held + r.components - r.spores;
        // The 2-core: peel every spore on fewer than two filaments, and the
        // filaments it held, until none is left to peel.
        let mut peeled: Keyed<()> = Keyed::new();
        loop {
            let leaves: Vec<Vec<u8>> = degree.scoped().filter(|(s, d)| **d < 2 &&!peeled.contains(s)).map(|(s, _)| s).collect();
            if leaves.is_empty() {
                break;
            }
            for s in leaves {
                peeled.insert(&s, ());
                for (near, far) in &ends {
                    if *near == s && !peeled.contains(far) {
                        // The far spore holds this filament's reverse, once.
                        if let Some(d) = degree.get_mut(far) {
                            *d -= 1;
                        }
                    }
                }
            }
        }
        r.core = (degree.len() - peeled.len()) as u64;
        r
    }
}
