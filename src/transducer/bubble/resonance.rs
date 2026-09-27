// N4, RX1–RX3, 78.11, 78.26(5) — PERCEPTION ACROSS SCENES IS A CELL'S ORBIT
// ALGEBRA: the bodies held perceptions stand on one chart are one cell, the
// transitions between them its circuits, read by the cell reader the scene's
// own cells are read by — and what closes is a thought, filed where the meet
// graph reads thoughts.
//
//! A perception holds its input bodies by the chart each stands on (their
//! moduli: every axis invariant under every versor, so only bodies on one
//! chart can be carried onto each other). The chart's fibre in `by_chart` is
//! every held perception with a body there. When a perception is held, each
//! of its bodies is read against each body of that fibre: a versor carrying
//! the one onto the other, read the same from both sides (RX1), is a CIRCUIT
//! of the chart's cell, its holonomy the transition's versor. The cell's
//! constructors are all the chart's bodies' nuclei.
//!
//! [`cech::read_cell`] then reads what the laws say to read: each circuit's
//! modes off its own action — HARMONIC (elliptic of finite order: the orbit
//! returns), LUMINAL (a boost: it never returns), a parabolic slide, a
//! rosette; the capacity of the algebra the circuits generate; whether the
//! modes keep the round (HOLONOMIC where one carries it off); and coherence
//! jointly — closed at the joint period, or open with each incoherence's
//! class by content.
//!
//! WHAT IS FILED, by the machinery that already files it:
//! - Two perceptions related by ONE transition on the chart are joined by an
//!   edge of the meet complex carrying its rotor and carriage; the complex's
//!   own fill classifies it (Filled, Congruent at its period, a class), and
//!   `decide_neighbors` acts on the fill. That edge is the THOUGHT.
//! - Two perceptions related by SEVERAL transitions hold no one edge — none
//!   is chosen (rule 15); each transition's classification is filed at its
//!   class coordinate (`index_congruent`), so the congruences stand by class.
//! - Each incoherence the cell reads is a demand on the chart by its class
//!   (the CS2 arm, `docket_demand`), answered where a held word spells it.
//!
//! Nothing between bodies is stored: a transition is re-read when a
//! perception is held, and what stands is the fill, the class, the demand.

use super::*;
use crate::transducer::blade::{self, NotReciprocal, Shape, Transition};
use crate::transducer::cech::{CellCircuit, Coherence};
use crate::transducer::cga::Mv;

/// A perception's bodies by the chart each stands on (their moduli), in the
/// order its input frames hold them.
pub type Spores = Keyed<Vec<Shape>>;

/// The input bodies a family's frames were perceived as, by chart.
pub fn spores_of(frames: &crate::transducer::family::Frames) -> Spores {
    let mut spores: Spores = Keyed::new();
    for (ins, _) in frames {
        for b in ins {
            let key = b.moduli.key();
            match spores.get_mut(key.as_bytes()) {
                Some(v) => v.push(b.shape.clone()),
                None => {
                    spores.insert(key.as_bytes(), vec![b.shape.clone()]);
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

/// WHERE A PERCEPTION IS FILED BEYOND ITS FORM (A21): its charts in key
/// order, and on each every body's coordinates. An address, never a relation.
pub fn spores_key(spores: &Spores) -> Vec<u8> {
    use crate::transducer::address::put_key_bytes;
    let mut k = Vec::new();
    for (chart, bodies) in spores.scoped() {
        put_key_bytes(&mut k, &chart);
        for s in bodies {
            put_key_bytes(&mut k, &body_coordinates(s));
        }
    }
    k
}

/// WHERE A HELD FORM IS FILED: a perception at its form's key and its bodies
/// written out; any other idea at its form's key alone. The one derivation —
/// the hold, the drive's rekey and every reader of `Bubble::key` go through
/// it, so a perception is never refiled under its bare form.
pub(crate) fn perceived_key(form_key: crate::transducer::address::Address, spores: &Spores) -> crate::transducer::address::Address {
    if spores.is_empty() {
        return form_key;
    }
    let mut k = form_key.as_bytes().to_vec();
    crate::transducer::address::put_key_bytes(&mut k, &spores_key(spores));
    crate::transducer::address::Address::from(k)
}

/// WHY A PAIR OF BODIES IS NO CIRCUIT, each by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unrelated {
    /// No versor carries the one body onto the other.
    Uncarried(blade::NoPose),
    /// RX1 — the pair reads differently from its two sides.
    NotReciprocal(NotReciprocal),
    /// The stroke was spent first.
    Unbounded,
}

/// THE PERCEPTION READING, cumulative at its producer (§6.5): the body pairs
/// read and why some are no circuit; the cells read and how they cohere; the
/// circuits by the regime of their modes (78.26(5)); and what was filed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ResonanceTally {
    /// Circuits: body pairs a reciprocal transition carries.
    pub circuits: u64,
    pub uncarried: u64,
    /// RX1 refusals, by what failed: no reverse, the order, a residue.
    pub not_reciprocal: [u64; 3],
    pub unbounded: u64,
    /// Cells read; of them closed at a joint period, open, unread.
    pub cells: u64,
    pub closes: u64,
    pub open: u64,
    pub cells_unread: u64,
    /// The circuits by their modes: resonant (harmonic, returns), rosette,
    /// parabolic, boost (luminal); and those carrying the round off
    /// (holonomic).
    pub resonant: u64,
    pub rosette: u64,
    pub parabolic: u64,
    pub boost: u64,
    pub holonomic: u64,
    /// The shear (78.46(4′), C1): cells one circuit crosses (unperturbed),
    /// cells whose circuits all agree on one ray (flat), and cells two of
    /// whose circuits disagree (curved).
    pub unperturbed: u64,
    pub flat: u64,
    pub curved: u64,
    /// Thoughts: perception pairs joined by their one transition's edge; and
    /// pairs holding several, none chosen, each orbit kept by its class —
    /// congruent (the congruence index) or a standing class (both horizons).
    pub edges: u64,
    pub several: u64,
    pub kept_congruent: u64,
    pub kept_class: u64,
    /// The hours (78.48(7)): bodies standing at the hour another idea's orbit
    /// predicts; and the slipped predictions — on the orbit at another hour
    /// (phase), or off it.
    pub predicted: u64,
    pub phase_slips: u64,
    pub off_orbit: u64,
    /// The strata (78.49(4)): body pairs parting at a Betti axis whose
    /// surgery was read (deformed) or did not factor; and pairs parting at a
    /// finer axis, which no conformal motion carries (apart).
    pub deformed: u64,
    pub unfactored: u64,
    pub apart: u64,
    /// Incoherences and slipped predictions filed as demands by class; and
    /// those whose class did not read.
    pub demanded: u64,
    pub undemanded: u64,
}

impl ResonanceTally {
    pub fn line(&self) -> String {
        format!(
            "perception: circuits {} | no circuit: uncarried {} not reciprocal (reverse {} order {} residue {}) unbounded {} | cells {} closes {} open {} unread {} | circuits resonant {} rosette {} parabolic {} boost {} holonomic {} | shear: unperturbed {} flat {} curved {} | thoughts: edges {} several {} kept congruent {} class {} | hours: predicted {} phase slips {} off orbit {} | strata: deformed {} unfactored {} apart {} | demanded {} undemanded {}",
            self.circuits,
            self.uncarried,
            self.not_reciprocal[0],
            self.not_reciprocal[1],
            self.not_reciprocal[2],
            self.unbounded,
            self.cells,
            self.closes,
            self.open,
            self.cells_unread,
            self.resonant,
            self.rosette,
            self.parabolic,
            self.boost,
            self.holonomic,
            self.unperturbed,
            self.flat,
            self.curved,
            self.edges,
            self.several,
            self.kept_congruent,
            self.kept_class,
            self.predicted,
            self.phase_slips,
            self.off_orbit,
            self.deformed,
            self.unfactored,
            self.apart,
            self.demanded,
            self.undemanded,
        )
    }

    /// Every count in the reading's order, one place (write and read both).
    fn fields(&mut self) -> [&mut u64; 30] {
        let [nr0, nr1, nr2] = &mut self.not_reciprocal;
        [
            &mut self.circuits,
            &mut self.uncarried,
            nr0,
            nr1,
            nr2,
            &mut self.unbounded,
            &mut self.cells,
            &mut self.closes,
            &mut self.open,
            &mut self.cells_unread,
            &mut self.resonant,
            &mut self.rosette,
            &mut self.parabolic,
            &mut self.boost,
            &mut self.holonomic,
            &mut self.unperturbed,
            &mut self.flat,
            &mut self.curved,
            &mut self.edges,
            &mut self.several,
            &mut self.kept_congruent,
            &mut self.kept_class,
            &mut self.predicted,
            &mut self.phase_slips,
            &mut self.off_orbit,
            &mut self.deformed,
            &mut self.unfactored,
            &mut self.apart,
            &mut self.demanded,
            &mut self.undemanded,
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
            Unrelated::Uncarried(_) => self.uncarried += 1,
            Unrelated::NotReciprocal(NotReciprocal::NoReverse) => self.not_reciprocal[0] += 1,
            Unrelated::NotReciprocal(NotReciprocal::OrderAsymmetric) => self.not_reciprocal[1] += 1,
            Unrelated::NotReciprocal(NotReciprocal::Residue(_)) => self.not_reciprocal[2] += 1,
            Unrelated::Unbounded => self.unbounded += 1,
        }
    }
}

/// ONE BODY READ AGAINST ANOTHER (RX1): the coset of versors carrying `a`
/// onto `b`, its representative, and the reverse read from `b`'s side.
fn transition(a: &Shape, b: &Shape, budget: &mut crate::physics::Budget) -> Result<Transition, Unrelated> {
    let coset = |x: &Shape, y: &Shape, budget: &mut crate::physics::Budget| {
        blade::versor_coset(x, y, budget).map_err(|why| match why {
            blade::NoPose::Unbounded => Unrelated::Unbounded,
            other => Unrelated::Uncarried(other),
        })
    };
    let ab = coset(a, b, budget)?;
    let ba = coset(b, a, budget)?;
    let t = blade::coset_representative(&ab).ok_or(Unrelated::Uncarried(blade::NoPose::Ungauged))?;
    blade::reciprocal(t, &ba, a.blade.n()).map_err(Unrelated::NotReciprocal)?;
    Ok(t.clone())
}

/// A versor's two halves as the complex carries an edge: the rotor on the
/// letters' chart and the carriage `T·D`, split as the fill splits a
/// transport. `None` where the versor does not split.
fn halves(v: &Mv) -> Option<(crate::transducer::alphabet::Rotor, crate::transducer::element::Versor)> {
    let f = crate::transducer::element::conformal_factors(v).ok()?;
    let rotor = crate::transducer::element::rotor_of_factors(&f).ok()?;
    Some((rotor, f.carriage()))
}

/// ONE BODY OF A PERCEPTION AGAINST ANOTHER IDEA'S BODIES ON ONE CHART: the
/// transition to each of them, in the order they arrived — `None` where no
/// reciprocal transition carries the pair.
struct Row {
    chart: Vec<u8>,
    ts: Vec<Option<Transition>>,
    /// What the row's transitions carry, per lane, summed lane by lane
    /// (`lanes_of`, law 5).
    lanes: [u64; 4],
}

/// WHAT A TRANSITION CARRIES, PER LANE (78.49(5), `measure_lanes`): the
/// chain whose before-support is the carried body's sites and whose
/// after-support is its image's — 𝒮 the sites that appear or vanish, 𝒽 the
/// shell's change; 𝒢 reads nothing, for a body is read hue-blind, and 𝒯
/// nothing, for a chain holds no clock. `None` where a site is off the grid's
/// two axes.
fn lanes_of(x: &Shape, y: &Shape) -> Option<[u64; 4]> {
    let mut at: Keyed<(i64, i64, u8, u8)> = Keyed::new();
    for (s, before) in x.blade.support.iter().map(|s| (s, true)).chain(y.blade.support.iter().map(|s| (s, false))) {
        let (r, c) = (s.axis(0)?, s.axis(1)?);
        let k = super::remainder_site_key(r, c);
        let link = match (at.get(&k).copied(), before) {
            (Some((_, _, b, _)), false) => (r, c, b, 1),
            (Some((_, _, _, a)), true) => (r, c, 1, a),
            (None, true) => (r, c, 1, 0),
            (None, false) => (r, c, 0, 1),
        };
        at.insert(&k, link);
    }
    let links: Vec<(i64, i64, u8, u8)> = at.scoped().map(|(_, l)| *l).collect();
    Some(crate::transducer::element::measure_lanes(&links))
}

/// WHERE TWO CHARTS PART: the first axis at which their moduli differ, or
/// where one ends — the deformation's coordinate (78.49(4)). The axes run
/// `n`, then the Betti numbers, then the turns and the ratios.
fn parting(a: &stalk::Invariant, b: &stalk::Invariant) -> usize {
    a.0.iter().zip(&b.0).take_while(|(x, y)| x == y).count()
}

/// A parting at a Betti axis (`β₀ β₁ β₂`, axes 1 to 3) is TOPOLOGICAL: a
/// surgery can carry it. At any finer axis no conformal motion carries one
/// body onto the other, every axis being invariant under every versor.
fn topological(axis: usize) -> bool {
    (1..=3).contains(&axis)
}

/// WHERE A BODY STANDS ON ANOTHER IDEA'S ORBIT (78.48(7)).
enum Hour {
    /// The other's bodies are no uniform orbit: fewer than two, a pair no
    /// transition carries, a first step of no finite order, or an arrival
    /// off its hour.
    NoOrbit,
    /// The body is the orbit's NEXT HOUR, as predicted: it joins by the
    /// orbit's step.
    Predicted(Mv),
    /// The prediction slipped: the body stands on the orbit at another hour
    /// (a phase slip) or off it, and the residue is what was seen against
    /// what the orbit predicted at its next hour.
    Slipped { on_orbit: bool, residue: Mv },
}

/// Is `v` the identity ray — a non-zero scalar and nothing else?
fn unit_ray(v: &Mv) -> bool {
    let s = v.scalar_part();
    !num_traits::Zero::is_zero(&s) && v.sub(&Mv::scalar(s)).is_zero()
}

/// THE HOURS INSIDE A RETURN (78.48(7), 78.43). The other idea's bodies, in
/// the order they arrived, are samples of an orbit where each arrived one
/// application of its STEP after the last: `y_j = g^{j−1} y_1`, read off the
/// transitions from this body as `t_j ∝ g^{j−1} t_1`, `g = t_2 t̃_1` the first
/// observed step. The step's period ℓ is its rotor's order — the lcm of its
/// two planes' periods, orbit and spin, so the hours count both. With `m`
/// hours seen, the orbit predicts the next, `g^m y_1`; this body `x = t̃_1
/// y_1` is predicted exactly where `g^m t_1` is the identity ray. Otherwise
/// the slip is read: on the orbit at another hour (`g^h t_1` the identity for
/// some `h < ℓ`, read along the orbit's own period) or off it, the residue
/// `t̃_1 (g^m)˜` in either case. Exact; bounded by the period.
fn hour_of(row: &[Option<Transition>]) -> Hour {
    let Some(ts) = row.iter().map(Option::as_ref).collect::<Option<Vec<&Transition>>>() else { return Hour::NoOrbit };
    let [t1, t2, ..] = ts.as_slice() else { return Hour::NoOrbit };
    let step = t2.versor.geo(&t1.versor.reverse());
    let Some(crate::transducer::alphabet::Order::Finite(ell)) = halves(&step).and_then(|(r, _)| r.order()) else {
        return Hour::NoOrbit;
    };
    let one = Mv::scalar(num_traits::One::one());
    let mut power = one.clone();
    for t in &ts {
        if !t.versor.projective_eq(&power.geo(&t1.versor)) {
            return Hour::NoOrbit;
        }
        power = step.geo(&power);
    }
    if unit_ray(&power.geo(&t1.versor)) {
        return Hour::Predicted(step);
    }
    let residue = t1.versor.reverse().geo(&power.reverse());
    let mut at = one;
    let mut on_orbit = false;
    for _ in 0..ell {
        if unit_ray(&at.geo(&t1.versor)) {
            on_orbit = true;
            break;
        }
        at = step.geo(&at);
    }
    Hour::Slipped { on_orbit, residue }
}

/// A circuit's coordinates: the two perceptions' ordinals and the two
/// bodies' coordinates, each delimited.
fn circuit_ring(a: u16, x: &Shape, b: u16, y: &Shape) -> Vec<u8> {
    use crate::transducer::address::put_key_bytes;
    let mut k = Vec::new();
    put_key_bytes(&mut k, &a.to_le_bytes());
    put_key_bytes(&mut k, &body_coordinates(x));
    put_key_bytes(&mut k, &b.to_le_bytes());
    put_key_bytes(&mut k, &body_coordinates(y));
    k
}

impl Contemplation {
    /// N4 — A PERCEPTION HELD MEETS THE CELLS ITS BODIES STAND ON. On each
    /// chart its bodies stand on, the chart's fibre is read (`by_chart`), the
    /// transitions from its bodies to the fibre's bodies are the cell's
    /// circuits, and the cell reader reads them. Then what the reading says
    /// is filed: a pair's one transition as the edge the complex fills, a
    /// pair's several by class, each incoherence as a demand by class.
    pub(crate) fn perceive_cells(&mut self, key: &[u8], budget: &mut crate::physics::Budget) {
        let Some(me) = self.bubbles.get(key) else { return };
        let Some(ok) = self.ordinals.get(key).copied() else { return };
        let charts: Vec<(Vec<u8>, Vec<Shape>)> = me.spores.scoped().map(|(c, b)| (c, b.clone())).collect();
        // Each other idea's rows: per body of this one, its transitions to
        // the other's bodies in their arrival order, over every chart.
        let mut to: Keyed<Vec<Row>> = Keyed::new();
        for (chart, mine) in &charts {
            let fibre: Vec<(u16, Vec<u8>)> = self
                .by_chart
                .link(chart)
                .filter_map(|(id, _)| Self::ordinal_of_key(&id))
                .filter_map(|o| self.address_at(o).map(|a| (o, a.to_vec())))
                .filter(|(_, a)| a.as_slice() != key)
                .collect();
            let mut circuits: Vec<CellCircuit> = Vec::new();
            // The cell's bodies: this perception's and the fibre's on the chart.
            let mut bodies = mine.len();
            for (o, other) in &fibre {
                let Some(theirs) = self.bubbles.get(other).and_then(|b| b.spores.get(chart)).cloned() else { continue };
                bodies += theirs.len();
                for x in mine {
                    let mut row = Row { chart: chart.clone(), ts: Vec::with_capacity(theirs.len()), lanes: [0; 4] };
                    for y in &theirs {
                        match transition(x, y, budget) {
                            Ok(t) => {
                                self.resonance.circuits += 1;
                                circuits.push(CellCircuit {
                                    ring: circuit_ring(ok, x, *o, y),
                                    holonomy: Ok(t.versor.clone()),
                                    // A body is read hue-blind; the empty
                                    // listing is what the transition carries.
                                    hue: Some(Vec::new()),
                                });
                                match lanes_of(x, y) {
                                    Some(l) if crate::core::lanes::lanes_step(&mut row.lanes, &l) => {}
                                    _ => self.overflowed += 1,
                                }
                                row.ts.push(Some(t));
                            }
                            Err(Unrelated::Unbounded) => {
                                self.resonance.note(Unrelated::Unbounded);
                                return;
                            }
                            Err(why) => {
                                self.resonance.note(why);
                                row.ts.push(None);
                            }
                        }
                    }
                    match to.get_mut(other) {
                        Some(rows) => rows.push(row),
                        None => {
                            to.insert(other, vec![row]);
                        }
                    }
                }
            }
            if circuits.is_empty() {
                continue;
            }
            // `k + 1` bodies make a `k`-cell; a circuit stands on two at least.
            let grade = bodies - 1;
            let orders = vec![None; circuits.len()];
            // THE BODIES OF DIFFERENT SCENES ARE NO ANASTOMOSIS: one body
            // seen twice is not two bodies fused, so the cell has no
            // constructors and no round — the transitions carry each nucleus
            // into another scene, and an orbit of constructors under them
            // would never close. What the cell reads is its circuits: their
            // modes, their joint coherence, the capacity of their algebra.
            let cell = crate::transducer::cech::read_cell(grade, None, None, &circuits, &orders, budget);
            self.note_cell(&cell, chart);
            self.note_shear(&circuits);
        }
        self.file_thoughts(key, &to);
        for (chart, mine) in &charts {
            if !self.perceive_strata(key, chart, mine, budget) {
                return;
            }
        }
    }

    /// 78.49(4) — THE STRATA ABOVE A CHART: the atlas is read, never a bucket.
    /// From the chart, one axis is peeled at a time — a prefix of the key is a
    /// coarser stratum (A21) — and the first stratum at which another held
    /// perception stands on a different chart is the DEEPEST SHARED ONE: the
    /// chart's nearest neighbourhood, read as a keyed fibre (`fiber_at`), no
    /// index added. The root, every body, is never read (law 3). Each body
    /// there is read against this perception's bodies where their charts
    /// PART: at a Betti axis a surgery may carry one onto the other
    /// (`Surgery::factor`), and the deformation — its core's versor, by its
    /// class, on both charts — is a demand; at a finer axis no conformal
    /// motion carries them, counted apart. `false` where the stroke ran out.
    fn perceive_strata(&mut self, key: &[u8], chart: &[u8], mine: &[Shape], budget: &mut crate::physics::Budget) -> bool {
        let Some(here) = stalk::Invariant::from_key(chart) else { return true };
        let mut at = here.peel();
        while !at.0.is_empty() {
            let near: Vec<(Vec<u8>, Vec<u8>)> = match self.by_chart.fiber_at(at.key().as_bytes()) {
                Some(fiber) => fiber
                    .filter(|(c, _)| c.as_slice() != chart)
                    .flat_map(|(c, members)| {
                        members
                            .scoped()
                            .filter_map(|(id, _)| Self::ordinal_of_key(&id).and_then(|o| self.address_at(o)).map(<[u8]>::to_vec))
                            .filter(|a| a.as_slice() != key)
                            .map(move |a| (c.clone(), a))
                            .collect::<Vec<_>>()
                    })
                    .filter(|(c, a)| self.bubbles.get(a).is_some_and(|b| b.spores.get(c).is_some()))
                    .collect(),
                None => Vec::new(),
            };
            if near.is_empty() {
                at = at.peel();
                continue;
            }
            for (theirs_chart, other) in near {
                let Some(there) = stalk::Invariant::from_key(&theirs_chart) else { continue };
                let axis = parting(&here, &there);
                let Some(theirs) = self.bubbles.get(&other).and_then(|b| b.spores.get(&theirs_chart)).cloned() else { continue };
                for x in mine {
                    for y in &theirs {
                        if !topological(axis) {
                            self.resonance.apart += 1;
                            continue;
                        }
                        if budget.remaining() == 0 {
                            self.resonance.note(Unrelated::Unbounded);
                            return false;
                        }
                        match crate::transducer::blade::surgery::Surgery::factor(x, y, budget) {
                            Some((_, core)) => self.file_deformation(&here, &there, &core),
                            None => self.resonance.unfactored += 1,
                        }
                    }
                }
            }
            return true;
        }
        true
    }

    /// A DEFORMATION READ (78.49(4)): the surgery's core carried by `core`, a
    /// demand on both charts by the core motion's class vector — the zero
    /// class for a change in place — with the core's ray as its residue: the
    /// question of the deformation.
    fn file_deformation(&mut self, here: &stalk::Invariant, there: &stalk::Invariant, core: &Mv) {
        self.resonance.deformed += 1;
        let class = halves(core).and_then(|(r, c)| sentence::ClassVector::of_rotor(&r, c, sentence::CycleType::Identity));
        match class {
            Some(k) => {
                self.docket_demand(&k, &[here.clone(), there.clone()], Some(core));
                self.resonance.demanded += 1;
            }
            None => self.resonance.undemanded += 1,
        }
    }

    /// THE SHEAR ON THE CELL'S ORBITS (78.46(4′), 78.48(4)): the pair
    /// composite `ρ_n⁻¹ ρ_o` is the identity exactly where two orbits are one
    /// ray (C1(a)), so the circuits keyed by their canonical rays hold every
    /// pair at once — under one key they agree (FLAT), under several each is
    /// sheared by the others (CURVED); one circuit alone is UNPERTURBED. A
    /// ray that does not read canonically is keyed by nothing and so agrees
    /// with nothing (rule 11).
    fn note_shear(&mut self, circuits: &[CellCircuit]) {
        let mut rays: Keyed<()> = Keyed::new();
        let mut unkeyed = false;
        for c in circuits {
            match c.holonomy.as_ref().ok().and_then(Mv::canonical) {
                Some(ray) => {
                    let mut k = Vec::new();
                    ray.put_canonical(&mut k);
                    rays.insert(&k, ());
                }
                None => unkeyed = true,
            }
        }
        match (circuits, rays.len(), unkeyed) {
            ([_], _, _) => self.resonance.unperturbed += 1,
            (_, 1, false) => self.resonance.flat += 1,
            _ => self.resonance.curved += 1,
        }
    }

    /// The cell's reading counted, and each incoherence filed as a demand on
    /// the chart by its class (the CS2 arm).
    fn note_cell(&mut self, cell: &crate::transducer::cech::CellAlgebra, chart: &[u8]) {
        self.resonance.cells += 1;
        let r = &cell.rings;
        self.resonance.resonant += r.resonant as u64;
        self.resonance.rosette += r.rosette as u64;
        self.resonance.parabolic += r.parabolic as u64;
        self.resonance.boost += r.boost as u64;
        self.resonance.holonomic += r.holonomic as u64;
        match &cell.coherence {
            Coherence::Closes(_) => self.resonance.closes += 1,
            Coherence::Unread => self.resonance.cells_unread += 1,
            Coherence::Open(incoherent) => {
                self.resonance.open += 1;
                let charts: Vec<stalk::Invariant> = stalk::Invariant::from_key(chart).into_iter().collect();
                for inc in incoherent {
                    match &inc.class {
                        Ok(class) => {
                            self.docket_demand(class, &charts, None);
                            self.resonance.demanded += 1;
                        }
                        Err(_) => self.resonance.undemanded += 1,
                    }
                }
            }
        }
    }

    /// THE THOUGHTS: each other idea related by one transition — or by
    /// several that are one ray — is joined to this one by the edge the
    /// complex fills. Where there are several, each of this perception's
    /// bodies is read against the other's ORBIT (78.48(7)): every body at the
    /// hour the orbit predicts, by one step, joins by that step — the orbit
    /// continued; otherwise each transition is kept by its class, none chosen
    /// as the edge, and every slipped prediction is a demand at its chart.
    /// Then the complex climbs and decides, as for any held idea.
    fn file_thoughts(&mut self, key: &[u8], to: &Keyed<Vec<Row>>) {
        let mut rems: Keyed<Vec<Link>> = Keyed::new();
        if let Some(b) = self.bubbles.get(key) {
            rems.insert(key, b.remainder.placed());
        }
        for (other, rows) in to.scoped() {
            if let Some(b) = self.bubbles.get(&other) {
                rems.insert(&other, b.remainder.placed());
            }
            let all: Vec<&Transition> = rows.iter().flat_map(|r| r.ts.iter().flatten()).collect();
            let [first, rest @ ..] = all.as_slice() else { continue };
            let one = rest.iter().all(|t| t.versor.projective_eq(&first.versor));
            let hours: Vec<Hour> = match one {
                true => Vec::new(),
                false => rows.iter().map(|r| hour_of(&r.ts)).collect(),
            };
            let steps: Vec<&Mv> = hours
                .iter()
                .filter_map(|h| match h {
                    Hour::Predicted(g) => Some(g),
                    Hour::NoOrbit | Hour::Slipped { .. } => None,
                })
                .collect();
            self.resonance.predicted += steps.len() as u64;
            // The edge: the one transition, or the orbit's one step where
            // every body stands at its predicted hour.
            let every = hours.iter().all(|h| matches!(h, Hour::Predicted(_)));
            let edge = match (one, steps.as_slice(), every) {
                (true, _, _) => halves(&first.versor),
                (false, [g, more @ ..], true) if more.iter().all(|s| s.projective_eq(g)) => halves(g),
                _ => None,
            };
            match edge {
                Some((rotor, carriage)) => {
                    // THE EDGE CARRIES WHAT ITS TRANSITIONS CARRIED, lane by
                    // lane (78.49(5)); shared remainder volume stays the
                    // 2-cells' measure, read by the climb below.
                    let mut lanes = [0u64; 4];
                    if !rows.iter().all(|r| crate::core::lanes::lanes_step(&mut lanes, &r.lanes)) {
                        self.overflowed += 1;
                    }
                    self.complex.set_edge(key, &other, rotor, carriage, lanes);
                    self.resonance.edges += 1;
                }
                None => {
                    self.resonance.several += 1;
                    for t in &all {
                        self.keep_orbit(key, &other, t);
                    }
                    for (row, h) in rows.iter().zip(&hours) {
                        if let Hour::Slipped { on_orbit, residue } = h {
                            self.file_slip(&row.chart, residue, *on_orbit);
                        }
                    }
                }
            }
        }
        let reopened = self.complex.coboundary_on_link(key, |verts| volume_of(&rems, verts));
        self.note_decohered(reopened, super::meet::Adjunct::Candidate);
        self.decide_neighbors(key);
    }

    /// ONE ORBIT OF A PAIR THAT HOLDS SEVERAL, KEPT BY ITS OWN COORDINATES
    /// (78.48(3), (6)): classified as the complex classifies an edge; a
    /// congruence filed at its class in the congruence index, and a
    /// congruence or a standing class written into BOTH perceptions'
    /// horizons, which marks them for the pivot. Never keyed by the pair —
    /// an index keyed by two addresses copies both.
    fn keep_orbit(&mut self, a: &[u8], b: &[u8], t: &Transition) {
        let Some((rotor, carriage)) = halves(&t.versor) else {
            self.resonance.undemanded += 1;
            return;
        };
        let fill = crate::transducer::cech::classify(&rotor, &carriage, sentence::CycleType::Identity);
        let held = match &fill {
            Fill::Congruent(..) => {
                self.complex.index_congruent(&fill);
                self.resonance.kept_congruent += 1;
                true
            }
            Fill::Class(..) => {
                self.resonance.kept_class += 1;
                true
            }
            Fill::Filled(..) | Fill::Unspellable(_) | Fill::Unread(_) | Fill::Open => false,
        };
        if held {
            for v in [a, b] {
                if let Some(bubble) = self.bubbles.get_mut(v) {
                    horizon_put(&mut bubble.horizon, rotor.clone(), Standing::Held);
                    bubble.dirty = true;
                    self.dirty.insert(v, ());
                }
            }
        }
    }

    /// A SLIPPED PREDICTION IS A DEMAND AT ITS CHART (78.48(7), (8)): the
    /// error localised to the hour the orbit predicted — its residue, what
    /// was seen against what was predicted, classified as an edge is and
    /// filed by that class with the residue as its ray; a phase slip (on the
    /// orbit at another hour) and a body off the orbit counted apart.
    fn file_slip(&mut self, chart: &[u8], residue: &Mv, on_orbit: bool) {
        match on_orbit {
            true => self.resonance.phase_slips += 1,
            false => self.resonance.off_orbit += 1,
        }
        let class = halves(residue).and_then(|(r, c)| match crate::transducer::cech::classify(&r, &c, sentence::CycleType::Identity) {
            Fill::Congruent(k, ..) | Fill::Class(k, _) => Some(k),
            Fill::Filled(..) | Fill::Unspellable(_) | Fill::Unread(_) | Fill::Open => None,
        });
        match class {
            Some(k) => {
                let charts: Vec<stalk::Invariant> = stalk::Invariant::from_key(chart).into_iter().collect();
                self.docket_demand(&k, &charts, Some(residue));
                self.resonance.demanded += 1;
            }
            None => self.resonance.undemanded += 1,
        }
    }
}
