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
    /// Incoherences filed as demands by class; and those whose class did not
    /// read.
    pub demanded: u64,
    pub undemanded: u64,
}

impl ResonanceTally {
    pub fn line(&self) -> String {
        format!(
            "perception: circuits {} | no circuit: uncarried {} not reciprocal (reverse {} order {} residue {}) unbounded {} | cells {} closes {} open {} unread {} | circuits resonant {} rosette {} parabolic {} boost {} holonomic {} | shear: unperturbed {} flat {} curved {} | thoughts: edges {} several {} kept congruent {} class {} | demanded {} undemanded {}",
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
            self.demanded,
            self.undemanded,
        )
    }

    /// Every count in the reading's order, one place (write and read both).
    fn fields(&mut self) -> [&mut u64; 24] {
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

/// A transition's two halves as the complex carries an edge: the rotor on
/// the letters' chart and the carriage `T·D`, split as the fill splits a
/// transport. `None` where the versor does not split.
fn halves(t: &Transition) -> Option<(crate::transducer::alphabet::Rotor, crate::transducer::element::Versor)> {
    let f = crate::transducer::element::conformal_factors(&t.versor).ok()?;
    let rotor = crate::transducer::element::rotor_of_factors(&f).ok()?;
    Some((rotor, f.carriage()))
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
        // Each other perception's transitions from this one, over every chart.
        let mut to: Keyed<Vec<Transition>> = Keyed::new();
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
                                match to.get_mut(other) {
                                    Some(ts) => ts.push(t),
                                    None => {
                                        to.insert(other, vec![t]);
                                    }
                                }
                            }
                            Err(Unrelated::Unbounded) => {
                                self.resonance.note(Unrelated::Unbounded);
                                return;
                            }
                            Err(why) => self.resonance.note(why),
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

    /// THE THOUGHTS: each other perception related by one transition — or by
    /// several that are one ray — is joined to this one by the edge the
    /// complex fills; several distinct ones are each classified and filed by
    /// class, none chosen as the edge. Then the complex climbs and decides,
    /// as for any held idea.
    fn file_thoughts(&mut self, key: &[u8], to: &Keyed<Vec<Transition>>) {
        let mut rems: Keyed<Vec<Link>> = Keyed::new();
        if let Some(b) = self.bubbles.get(key) {
            rems.insert(key, b.remainder.placed());
        }
        for (other, ts) in to.scoped() {
            let [first, rest @ ..] = ts.as_slice() else { continue };
            let one = rest.iter().all(|t| t.versor.projective_eq(&first.versor));
            match (one, halves(first)) {
                (true, Some((rotor, carriage))) => {
                    if let Some(b) = self.bubbles.get(&other) {
                        rems.insert(&other, b.remainder.placed());
                    }
                    let vol = volume_of(&rems, &[key, other.as_slice()]);
                    self.complex.set_edge(key, &other, rotor, carriage, vol);
                    self.resonance.edges += 1;
                }
                _ => {
                    self.resonance.several += 1;
                    for t in ts {
                        self.keep_orbit(key, &other, t);
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
        let Some((rotor, carriage)) = halves(t) else {
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
}
