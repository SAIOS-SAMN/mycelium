// N4, RX1–RX7 — SIGHT: the transitions between held perceptions, read from
// both sides, and the rings they close, order by order.
//
//! A perception holds its bodies by chart, at each frame it was perceived
//! at. Two perceptions MEET where their covers share a chart, and the THOUGHT
//! between them is the transition carrying the one body each holds on the
//! meet's leaf chart onto the other's — read both ways (RX1), refused by name
//! where the pair is two relations. The transition fits through an order `m`
//! (RX3); above it its residue is the thought's remainder, filed on the
//! docket as a demand with the residue exact (RX6 at the edge).
//!
//! A RING of three perceptions composes its edges' affine actions on each
//! plane's harmonic at each order (RX4). The residues are exact differences
//! of the bodies' own harmonics, `r = h_b − Ad(h_a)`, so around any ring they
//! telescope: the composed map fixes the first body's harmonic, `F(h) =
//! λ(h − h_A) + h_A`, and the ring's residue is `(1 − λ) h_A`. So a ring is
//! coherent at an order exactly where its linear holonomy `λ_k` is the
//! identity (FILLED) or of finite order `ℓ` (CONGRUENT(ℓ)), the `ℓ`-circuit sum
//! then vanishing by itself (RX5); the lowest order where `λ_k` has no finite
//! order is the ring's split, `F(0)` its residue there (RX6).

use super::*;
use crate::transducer::blade::{self, Harmonic, NotReciprocal, PlaneCarriage, Shape, Transition};
use crate::transducer::cga::Mv;
use num_traits::{One, Zero};

/// A perception's bodies by chart: its input frames' (`[0]`) and its output
/// frames' (`[1]`), each chart's bodies in the order the frames hold them.
pub type Sight = [Keyed<Vec<Shape>>; 2];

/// The bodies a family's frames were perceived as, by the chart each stands
/// on (its moduli), input and output apart.
pub fn sight_of(frames: &crate::transducer::family::Frames) -> Sight {
    let mut sight: Sight = [Keyed::new(), Keyed::new()];
    for (ins, outs) in frames {
        for (side, bodies) in [(0usize, ins), (1usize, outs)] {
            for b in bodies {
                let key = b.moduli.key();
                match sight[side].get_mut(key.as_bytes()) {
                    Some(v) => v.push(b.shape.clone()),
                    None => {
                        sight[side].insert(key.as_bytes(), vec![b.shape.clone()]);
                    }
                }
            }
        }
    }
    sight
}

/// A PERCEPTION'S KEY BEYOND ITS FORM (A21): the bodies it was perceived as,
/// written out — each side's charts in key order, and on each chart every
/// body's coordinates: its nucleus, its inertia and its quartic, exact. Two
/// scenes of one form whose bodies stand differently are two perceptions,
/// and the thought between them is the transition of their bodies.
pub fn sight_key(sight: &Sight) -> Vec<u8> {
    use crate::transducer::address::put_key_bytes;
    let mut k = Vec::new();
    for side in sight {
        for (chart, bodies) in side.scoped() {
            put_key_bytes(&mut k, &chart);
            for s in bodies {
                let mut body = Vec::new();
                s.blade.spatial.to_bytes(&mut body);
                for x in s.blade.inertia.iter().chain(s.blade.quartic.iter().flat_map(|t| t.iter())) {
                    Mv::scalar(x.clone()).to_bytes(&mut body);
                }
                put_key_bytes(&mut k, &body);
            }
        }
        // The side's close: an input chart never reads as an output one.
        put_key_bytes(&mut k, &[]);
    }
    k
}

/// WHY TWO PERCEPTIONS HOLD NO ONE THOUGHT, each by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unrelated {
    /// Their covers share no chart.
    Unshared,
    /// On the meet's chart one of them holds no body, or several: no one
    /// transition, and none is chosen.
    Several,
    /// No versor carries the one body onto the other: the pose's refusal.
    Uncarried(blade::NoPose),
    /// RX1 — the pair reads differently from its two sides.
    NotReciprocal(NotReciprocal),
    /// The meet's leaf charts read different transitions.
    Disagree,
    /// The stroke was spent first.
    Unbounded,
}

/// WHAT A RING OF THREE PERCEPTIONS READ AT ONE ORDER (RX5, RX6).
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

/// THE SIGHT READING, cumulative at its producer (§6.5).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SightTally {
    /// Thoughts formed: perceptions related by a reciprocal transition.
    pub related: u64,
    pub unshared: u64,
    pub several: u64,
    pub uncarried: u64,
    /// RX1 refusals, by what failed: no reverse, the order, a residue.
    pub not_reciprocal: [u64; 3],
    pub disagree: u64,
    pub unbounded: u64,
    /// RX6 at the edge: thoughts whose fit leaves a residue, and the demands
    /// filed for them (a class that did not read files none, counted).
    pub edge_split: u64,
    pub demanded: u64,
    pub undemanded: u64,
    /// Rings read: coherent through order 2, 3, 4; split at 2, 3, 4; unread.
    pub coherent: [u64; 3],
    pub split: [u64; 3],
    pub rings_unread: u64,
}

impl SightTally {
    pub fn line(&self) -> String {
        format!(
            "sight: thoughts related {} | unrelated: unshared {} several {} uncarried {} not reciprocal (reverse {} order {} residue {}) disagree {} unbounded {} | edge splits {} demanded {} undemanded {} | rings coherent through 2/3/4 {:?} split at 2/3/4 {:?} unread {}",
            self.related,
            self.unshared,
            self.several,
            self.uncarried,
            self.not_reciprocal[0],
            self.not_reciprocal[1],
            self.not_reciprocal[2],
            self.disagree,
            self.unbounded,
            self.edge_split,
            self.demanded,
            self.undemanded,
            self.coherent,
            self.split,
            self.rings_unread,
        )
    }

    /// Every count in the reading's order, one place (write and read both).
    fn fields(&mut self) -> [&mut u64; 19] {
        let [nr0, nr1, nr2] = &mut self.not_reciprocal;
        let [c0, c1, c2] = &mut self.coherent;
        let [s0, s1, s2] = &mut self.split;
        [
            &mut self.related,
            &mut self.unshared,
            &mut self.several,
            &mut self.uncarried,
            nr0,
            nr1,
            nr2,
            &mut self.disagree,
            &mut self.unbounded,
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
        ]
    }

    /// The reading on the observer's wire.
    pub fn put(&self, out: &mut Vec<u8>) {
        for v in self.clone().fields() {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }

    /// The reading off the observer's wire.
    pub fn take(b: &[u8], p: &mut usize) -> Option<SightTally> {
        let mut t = SightTally::default();
        for v in t.fields() {
            *v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
        }
        Some(t)
    }

    fn note(&mut self, why: Unrelated) {
        match why {
            Unrelated::Unshared => self.unshared += 1,
            Unrelated::Several => self.several += 1,
            Unrelated::Uncarried(_) => self.uncarried += 1,
            Unrelated::NotReciprocal(NotReciprocal::NoReverse) => self.not_reciprocal[0] += 1,
            Unrelated::NotReciprocal(NotReciprocal::OrderAsymmetric) => self.not_reciprocal[1] += 1,
            Unrelated::NotReciprocal(NotReciprocal::Residue(_)) => self.not_reciprocal[2] += 1,
            Unrelated::Disagree => self.disagree += 1,
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

impl Contemplation {
    /// THE MEET'S LEAVES between two held ideas: the shared charts no other
    /// shared chart refines — a cover is a chain of prefixes, and its deepest
    /// member is the one nothing in it extends.
    pub(crate) fn meet_leaves(&self, a: &[u8], b: &[u8]) -> Vec<Vec<u8>> {
        let Some(meet) = self.shared_chain(&[a.to_vec(), b.to_vec()]) else { return Vec::new() };
        meet.iter().filter(|k| !meet.iter().any(|o| o.len() > k.len() && o.starts_with(k))).cloned().collect()
    }

    /// THE THOUGHT BETWEEN TWO PERCEPTIONS (N4, RX1): on each leaf of their
    /// meet, the one input body each holds there, the transition carrying
    /// the one onto the other and its reverse read from the other side. Every
    /// leaf must read one transition.
    pub(crate) fn sight_transition(
        &self,
        a: &[u8],
        b: &[u8],
        budget: &mut crate::physics::Budget,
    ) -> Result<(Transition, Transition), Unrelated> {
        let (ba_, bb_) = (self.bubbles.get(a).ok_or(Unrelated::Unshared)?, self.bubbles.get(b).ok_or(Unrelated::Unshared)?);
        let leaves = self.meet_leaves(a, b);
        let mut read: Option<(Transition, Transition)> = None;
        for leaf in &leaves {
            let (Some([sa]), Some([sb])) = (ba_.sight[0].get(leaf).map(Vec::as_slice), bb_.sight[0].get(leaf).map(Vec::as_slice))
            else {
                return Err(Unrelated::Several);
            };
            let coset = |x: &Shape, y: &Shape, budget: &mut crate::physics::Budget| {
                blade::versor_coset(x, y, budget).map_err(|why| match why {
                    blade::NoPose::Unbounded => Unrelated::Unbounded,
                    other => Unrelated::Uncarried(other),
                })
            };
            let ab = coset(sa, sb, budget)?;
            let ba = coset(sb, sa, budget)?;
            let t = blade::coset_representative(&ab).ok_or(Unrelated::Uncarried(blade::NoPose::Ungauged))?;
            let back = blade::reciprocal(t, &ba, sa.blade.n()).map_err(Unrelated::NotReciprocal)?;
            match &read {
                None => read = Some((t.clone(), back.clone())),
                Some((held, _)) if held.versor.projective_eq(&t.versor) => {}
                Some(_) => return Err(Unrelated::Disagree),
            }
        }
        read.ok_or(Unrelated::Unshared)
    }

    /// Both directions of a thought, filed by the two ideas' ordinals.
    fn transition_key(a: u16, b: u16) -> [u8; 4] {
        let (x, y) = (a.to_le_bytes(), b.to_le_bytes());
        [x[0], x[1], y[0], y[1]]
    }

    /// The thought from `a` to `b`, where one was formed.
    pub(crate) fn transition_of(&mut self, a: &[u8], b: &[u8]) -> Option<Transition> {
        let (oa, ob) = (self.ordinal_for(&std::sync::Arc::from(a))?, self.ordinal_for(&std::sync::Arc::from(b))?);
        self.transitions.get(&Self::transition_key(oa, ob)).cloned()
    }

    /// N4 — THE THOUGHTS A NEW PERCEPTION FORMS: with every held perception
    /// its cover meets, the reciprocal transition between them — an edge of
    /// the meet graph whose rotor is the transition's own turn, filed both
    /// ways. A thought whose fit leaves a residue files it as its demand
    /// (RX6). Then the rings through the new perception are read.
    pub(crate) fn link_perceived(&mut self, key: &[u8], budget: &mut crate::physics::Budget) {
        let Some(me) = self.bubbles.get(key) else { return };
        if me.sight[0].is_empty() {
            return;
        }
        let charts: Vec<Vec<u8>> = me.cover.scoped().map(|(k, _)| k).collect();
        let mut neighbors: Keyed<()> = Keyed::new();
        for ck in &charts {
            for (id, _) in self.by_chart.link(ck) {
                let Some(addr) = Self::ordinal_of_key(&id).and_then(|o| self.address_at(o)) else { continue };
                if addr != key && self.bubbles.get(addr).is_some_and(|b| !b.sight[0].is_empty()) {
                    neighbors.insert(addr, ());
                }
            }
        }
        let nbs: Vec<Vec<u8>> = neighbors.scoped().map(|(k, _)| k).collect();
        for n in &nbs {
            let (ab, ba) = match self.sight_transition(key, n, budget) {
                Ok(pair) => pair,
                Err(why) => {
                    self.sight.note(why);
                    continue;
                }
            };
            let Ok(rotor) = crate::transducer::element::base_rotor_of(&ab.rotation) else {
                self.sight.uncarried += 1;
                continue;
            };
            self.complex.set_edge(key, n, rotor, crate::transducer::element::identity_versor(), [0; 4]);
            let (Some(ok), Some(on)) = (self.ordinal_for(&std::sync::Arc::from(key)), self.ordinal_for(&std::sync::Arc::from(n.as_slice())))
            else {
                continue;
            };
            self.sight.related += 1;
            let leaves = self.meet_leaves(key, n);
            self.file_edge_split(&ab, &leaves);
            self.transitions.insert(Self::transition_key(ok, on), ab);
            self.transitions.insert(Self::transition_key(on, ok), ba);
        }
        let reopened = self.complex.coboundary_on_link(key, |_| [0; 4]);
        self.note_decohered(reopened, super::meet::Adjunct::Candidate);
        self.decide_neighbors(key);
        self.read_rings(key, &nbs);
    }

    /// RX6 AT THE EDGE — a thought whose fit holds through `m < 4` leaves its
    /// residue above `m`: the lowest order's residue on each plane is filed
    /// as a demand on the meet's charts, the residue exact, its class the
    /// transition's own turn.
    fn file_edge_split(&mut self, t: &Transition, leaves: &[Vec<u8>]) {
        // The lowest order a residue stands at, read up the filtration.
        let Some(lowest) = Harmonic::ALL.into_iter().find(|k| t.residue.iter().any(|r| r.order == *k)) else { return };
        self.sight.edge_split += 1;
        let charts: Vec<stalk::Invariant> = leaves.iter().filter_map(|k| stalk::Invariant::from_key(k)).collect();
        let class = crate::transducer::element::base_rotor_of(&t.rotation).ok().and_then(|r| {
            sentence::ClassVector::of_rotor(&r, crate::transducer::element::identity_versor(), sentence::CycleType::Identity)
        });
        let Some(class) = class else {
            self.sight.undemanded += 1;
            return;
        };
        for r in t.residue.iter().filter(|r| r.order == lowest) {
            self.docket_residue(&class, &charts, residue_element(r.plane, &r.vector));
            self.sight.demanded += 1;
        }
    }

    /// RX4–RX6 — THE RINGS THROUGH A NEW PERCEPTION: each pair of its
    /// neighbours that are themselves related closes a ring of three
    /// thoughts, read order by order from the new perception.
    fn read_rings(&mut self, key: &[u8], nbs: &[Vec<u8>]) {
        for (i, b) in nbs.iter().enumerate() {
            for c in nbs.iter().skip(i + 1) {
                let (Some(t1), Some(t2), Some(t3)) = (self.transition_of(key, b), self.transition_of(b, c), self.transition_of(c, key))
                else {
                    continue;
                };
                // The base the ring's transitions act on.
                let n = t1.versor.n();
                let reading = ring_reading(&[&t1, &t2, &t3], n);
                // The filtration read up to where it stopped: a split at the
                // top, coherent through the order beneath it.
                match reading.as_slice() {
                    [.., (m, _), (k, RingOrder::Split { .. })] => {
                        self.sight.split[order_ix(*k)] += 1;
                        self.sight.coherent[order_ix(*m)] += 1;
                    }
                    [(k, RingOrder::Split { .. })] => self.sight.split[order_ix(*k)] += 1,
                    [.., (_, RingOrder::Unread)] | [] => self.sight.rings_unread += 1,
                    [.., (k, _)] => self.sight.coherent[order_ix(*k)] += 1,
                }
            }
        }
    }
}
