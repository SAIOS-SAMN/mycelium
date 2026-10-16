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
//! [`cech::read_chamber`] then reads what the laws say to read: each circuit's
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
use crate::transducer::cech::{ChamberCircuit, Coherence};
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
pub(super) fn body_coordinates(s: &Shape) -> Vec<u8> {
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
    /// Cells read; of them closed at a joint period, held (some ring
    /// precessing, none escaping), open (some ring escaping), unread.
    pub cells: u64,
    pub closes: u64,
    pub held: u64,
    pub open: u64,
    pub chambers_unread: u64,
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
    /// MU-CLIENT §4.4(1) — returns that came round TWISTED by their pull (a
    /// rotation about the attractor), and of those, the ones whose twist
    /// tilted the orbit's plane (LIFT).
    pub twisted: u64,
    pub lifted: u64,
    /// The strata (78.49(4)): body pairs parting at a Betti axis whose
    /// surgery was read (deformed) or did not factor; and pairs parting at a
    /// finer axis, which no conformal motion carries (apart).
    pub deformed: u64,
    pub unfactored: u64,
    pub apart: u64,
    /// The spectrum (78.50): pairs one motion carries whole (scene motions),
    /// pairs whose several whole motions are one coset and the symmetries
    /// their quotients are, this perception's bodies a scene motion does not
    /// carry (appeared) and the other's it lands nothing on (vanished), and
    /// lines whose ray did not read (unread).
    pub scene_motions: u64,
    pub cosets: u64,
    pub symmetries: u64,
    pub appeared: u64,
    pub vanished: u64,
    pub unread_lines: u64,
    /// A.10.1 — the consolidation gate: relations admitted as thoughts, and
    /// refused by name — monocular, collapsed, unbalanced, destabilizing,
    /// non-rigid (a pair's unit-weight product moved, 78.51(4′)), no succession, an external torque
    /// (A.13); and the reflexive — the identity, one frame seen twice, which
    /// bypasses the gate (78.52 Row 1).
    pub consolidated: u64,
    pub monocular: u64,
    pub collapsed: u64,
    pub unbalanced: u64,
    pub destabilizing: u64,
    /// 78.62 — transitions that raised the torque on some planes and lowered
    /// it on others: admitted, the rising planes filed as demands; and those
    /// whose torque did not split over ℚ, refused unread.
    pub tilted: u64,
    pub stability_unread: u64,
    pub reflexive: u64,
    pub nonrigid: u64,
    pub unsucceeded: u64,
    pub torqued: u64,
    /// 78.57 — transitions whose coherence form did not close, every defect
    /// component filed (each also counted on its own: unbalanced, non-rigid,
    /// external torque).
    pub incoherent: u64,
    /// Incoherences and slipped predictions filed as demands by class; and
    /// those whose class did not read.
    pub demanded: u64,
    pub undemanded: u64,
    /// 78.52 — THE GROUNDS of admitted relations (`note_ground`): filed;
    /// a step that does not return (no finite order of at least two — a
    /// slide, a boost); unplaced (bodies on several charts, or a body with
    /// no nucleus); no succession read at the bodies' barycentre.
    pub grounds: [u64; 4],
}

impl ResonanceTally {
    pub fn line(&self) -> String {
        format!(
            "perception: circuits {} | no circuit: uncarried {} not reciprocal (reverse {} order {} residue {}) unbounded {} | cells {} closes {} held {} open {} unread {} | circuits resonant {} rosette {} parabolic {} boost {} holonomic {} | shear: unperturbed {} flat {} curved {} | thoughts: edges {} several {} kept congruent {} class {} | hours: predicted {} phase slips {} off orbit {} twisted {} lifted {} | strata: deformed {} unfactored {} apart {} | spectrum: scene motions {} cosets {} symmetries {} appeared {} vanished {} unread {} | tensegrity: consolidated {} monocular {} collapsed {} unbalanced {} destabilizing {} tilted {} stability unread {} reflexive {} non-rigid {} no succession {} external torque {} incoherent {} | demanded {} undemanded {} | grounds: filed {} no return {} unplaced {} no succession {}",
            self.circuits,
            self.uncarried,
            self.not_reciprocal[0],
            self.not_reciprocal[1],
            self.not_reciprocal[2],
            self.unbounded,
            self.cells,
            self.closes,
            self.held,
            self.open,
            self.chambers_unread,
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
            self.twisted,
            self.lifted,
            self.deformed,
            self.unfactored,
            self.apart,
            self.scene_motions,
            self.cosets,
            self.symmetries,
            self.appeared,
            self.vanished,
            self.unread_lines,
            self.consolidated,
            self.monocular,
            self.collapsed,
            self.unbalanced,
            self.destabilizing,
            self.tilted,
            self.stability_unread,
            self.reflexive,
            self.nonrigid,
            self.unsucceeded,
            self.torqued,
            self.incoherent,
            self.demanded,
            self.undemanded,
            self.grounds[0],
            self.grounds[1],
            self.grounds[2],
            self.grounds[3],
        )
    }

    /// Every count in the reading's order, one place (write and read both).
    fn fields(&mut self) -> [&mut u64; 53] {
        let [nr0, nr1, nr2] = &mut self.not_reciprocal;
        let [g0, g1, g2, g3] = &mut self.grounds;
        [
            &mut self.circuits,
            &mut self.uncarried,
            nr0,
            nr1,
            nr2,
            &mut self.unbounded,
            &mut self.cells,
            &mut self.closes,
            &mut self.held,
            &mut self.open,
            &mut self.chambers_unread,
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
            &mut self.scene_motions,
            &mut self.cosets,
            &mut self.symmetries,
            &mut self.appeared,
            &mut self.vanished,
            &mut self.unread_lines,
            &mut self.consolidated,
            &mut self.monocular,
            &mut self.collapsed,
            &mut self.unbalanced,
            &mut self.destabilizing,
            &mut self.tilted,
            &mut self.stability_unread,
            &mut self.reflexive,
            &mut self.nonrigid,
            &mut self.unsucceeded,
            &mut self.torqued,
            &mut self.incoherent,
            &mut self.demanded,
            &mut self.undemanded,
            g0,
            g1,
            g2,
            g3,
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
    /// What each of the row's transitions carries, per lane, beside it
    /// (`lanes_of`); `None` where no transition carries the pair.
    lanes: Vec<Option<[u64; 4]>>,
    /// A.10.1 — this body's nucleus, and each of the other's bodies' on the
    /// chart, in their order: the frames a constellation is read on.
    nucleus: Option<Mv>,
    theirs: Vec<Option<Mv>>,
    /// Each of the other's bodies' mass: its sites (78.51(3)).
    their_mass: Vec<u64>,
    /// A.13 — this body and each of the other's as momentum reads them:
    /// centroid, mass, second moments (78.52 Row 1).
    body: Option<crate::physics::tensegrity::Body>,
    their_bodies: Vec<Option<crate::physics::tensegrity::Body>>,
}

/// ONE LINE OF A PAIR'S SPECTRUM (78.50): a ray, the bodies of this
/// perception it carries (by row), the bodies of the other it lands on (by
/// chart and place), and what it carries, per lane.
struct Line {
    t: Transition,
    carried: Keyed<()>,
    lands: Keyed<()>,
    lanes: [u64; 4],
}

/// The physics' verdict on a relation: admitted, or refused by name.
type Verdict = Result<(), crate::physics::tensegrity::Refusal>;

/// A GROUND FROM ITS BODIES AND ITS STEP FORWARD (78.52): the step at its
/// canonical ray, its class and its return (a period of at least two), and
/// the coupling — the step's succession at the bodies' barycentre. Refused
/// with what failed: the step does not return, or reads no succession.
fn ground_of(bodies: Vec<crate::physics::tensegrity::GroundBody>, forward: &Mv) -> Result<crate::physics::tensegrity::Ground, Ungrounded> {
    use crate::transducer::alphabet::Order;
    let generator = forward.canonical().ok_or(Ungrounded::NoReturn)?;
    let f = crate::transducer::element::conformal_factors(&generator).map_err(|_| Ungrounded::NoReturn)?;
    let Some(Order::Finite(ell)) = crate::transducer::element::rotor_of_factors(&f).ok().and_then(|r| r.order()) else {
        return Err(Ungrounded::NoReturn);
    };
    let period = u64::from(ell);
    if period < 2 {
        return Err(Ungrounded::NoReturn);
    }
    let class = generator.class_key().ok_or(Ungrounded::NoSuccession)?;
    let draft = crate::physics::tensegrity::Ground { class, period, bodies, generator: generator.clone(), coupling: generator };
    let centre = crate::physics::tensegrity::ground_centre(&draft).ok_or(Ungrounded::NoSuccession)?;
    let coupling = crate::physics::tensegrity::succession(&draft.generator, &centre)
        .and_then(|c| c.canonical())
        .ok_or(Ungrounded::NoSuccession)?;
    Ok(crate::physics::tensegrity::Ground { coupling, ..draft })
}

/// Why an admitted relation filed no ground — its slot in
/// `ResonanceTally::grounds` (slot 0 counts the grounds filed).
#[derive(Clone, Copy)]
enum Ungrounded {
    NoReturn = 1,
    Unplaced = 2,
    NoSuccession = 3,
}

/// A PAIR OF IDEAS' KEY (78.52): the two keys, least first, each delimited —
/// the coordinate a relation's ground is filed under, whichever side reads it.
pub(super) fn pair_key(a: &[u8], b: &[u8]) -> Vec<u8> {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    let mut k = Vec::new();
    crate::transducer::address::put_key_bytes(&mut k, lo);
    crate::transducer::address::put_key_bytes(&mut k, hi);
    // MU-CLIENT §4.13 — COMPACTED (the operator's ruling of 2026-10-16: a
    // hash is valid where nothing is lost): every reader holds the two
    // thoughts' keys and derives the pair, and nothing reads the pair back
    // out of the key, so the two whole keys are its 32-byte hash.
    crate::chronometry::payload_hash(&k).to_vec()
}

/// THE LINE OF THE TETHER between two thoughts (MU-CLIENT §4.13, Reading 2):
/// the key of the edge they span, the one the tether's own flex winds on
/// (`Contemplation::tether`), so the relation's flux and its tether are let
/// go together.
pub(super) fn tether_line(a: &[u8], b: &[u8]) -> Vec<u8> {
    crate::transducer::cech::simplex_key(&crate::transducer::cech::sorted_verts([a, b])).as_bytes().to_vec()
}

/// A row's key among a line's carried bodies: its place, big-endian.
fn row_key(i: usize) -> [u8; 8] {
    (i as u64).to_be_bytes()
}

/// A landing's key: the chart, then the body's place on it, big-endian.
fn land_key(chart: &[u8], j: usize) -> Vec<u8> {
    let mut k = Vec::new();
    crate::transducer::address::put_key_bytes(&mut k, chart);
    k.extend_from_slice(&(j as u64).to_be_bytes());
    k
}

/// THE PAIR'S SPECTRUM (78.50, 78.44): its transitions filed by their
/// canonical rays — the coordinates a ray is — each with the bodies it
/// carries and lands on and what it carries per lane. Transient: read, filed
/// from, dropped. A ray with no canonical form is filed nowhere, counted by
/// the caller as the pair's unread lines.
fn spectrum_of(rows: &[Row]) -> (Keyed<Line>, u64) {
    let mut spectrum: Keyed<Line> = Keyed::new();
    let mut unread = 0u64;
    for (i, row) in rows.iter().enumerate() {
        for (j, (t, lanes)) in row.ts.iter().zip(&row.lanes).enumerate() {
            let (Some(t), Some(lanes)) = (t, lanes) else { continue };
            let Some(ray) = t.versor.canonical() else {
                unread += 1;
                continue;
            };
            let mut k = Vec::new();
            ray.put_canonical(&mut k);
            if spectrum.get(&k).is_none() {
                spectrum.insert(&k, Line { t: t.clone(), carried: Keyed::new(), lands: Keyed::new(), lanes: [0; 4] });
            }
            if let Some(line) = spectrum.get_mut(&k) {
                line.carried.insert(row_key(i), ());
                line.lands.insert(land_key(&row.chart, j), ());
                if !crate::core::lanes::lanes_step(&mut line.lanes, lanes) {
                    unread += 1;
                }
            }
        }
    }
    (spectrum, unread)
}

/// Do two lines land on the same bodies — so their quotient carries the
/// other's configuration onto itself, a symmetry of it?
fn same_landing(a: &Line, b: &Line) -> bool {
    let (x, y): (Vec<Vec<u8>>, Vec<Vec<u8>>) = (a.lands.scoped().map(|(k, _)| k).collect(), b.lands.scoped().map(|(k, _)| k).collect());
    x == y
}

/// Do two lines carry the same bodies of this perception?
fn same_carried(a: &Line, b: &Line) -> bool {
    let (x, y): (Vec<Vec<u8>>, Vec<Vec<u8>>) =
        (a.carried.scoped().map(|(k, _)| k).collect(), b.carried.scoped().map(|(k, _)| k).collect());
    x == y
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
    // THE STEP'S RETURN: an elliptic turn returns at its period; a SLIDE —
    // a pure translation `T(d)` — never returns, and its hours are its
    // integer multiples, read exactly (`slide_of`). Anything else has no
    // hours to read.
    //
    // THE SLIDE IS READ FIRST: a pure translation's rotor is the identity,
    // whose order reads 1, and a slide read as a turn of period 1 is a
    // one-hour orbit it is not.
    let slide = slide_of(&step);
    let period = match (&slide, halves(&step).and_then(|(r, _)| r.order())) {
        (Some(_), _) => None,
        (None, Some(crate::transducer::alphabet::Order::Finite(ell))) => Some(ell),
        (None, _) => return Hour::NoOrbit,
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
    let on_orbit = match (period, slide) {
        // A turn: on the orbit where `g^h t_1` is the identity ray for some
        // `h` below the period — read along the period, which bounds it.
        (Some(ell), _) => {
            let mut at = one;
            let mut on = false;
            for _ in 0..ell {
                if unit_ray(&at.geo(&t1.versor)) {
                    on = true;
                    break;
                }
                at = step.geo(&at);
            }
            on
        }
        // A slide: the residue between what was seen and what was predicted
        // is a translation by a whole number of steps exactly where the body
        // stands at another hour.
        (None, Some(d)) => slide_of(&residue).is_some_and(|e| whole_multiple(&e, &d)),
        (None, None) => false,
    };
    Hour::Slipped { on_orbit, residue }
}

/// A SLIDE'S STEP: `v`'s translation `d` where `v` is a pure translation —
/// no turn (its rotation the identity ray), no dilation, no transversion, no
/// inversion — and `d` not zero. `None` otherwise.
fn slide_of(v: &Mv) -> Option<Vec<num_rational::BigRational>> {
    use num_traits::{One, Zero};
    let f = crate::transducer::element::conformal_factors(v).ok()?;
    let pure = unit_ray(&f.rotation) && f.rho.is_one() && !f.transverses() && !f.inverted;
    (pure && !f.translation.iter().all(Zero::is_zero)).then_some(f.translation)
}

/// Is `e` a whole multiple `k·d` of the step `d`, `k` an integer — read
/// exactly off the first coordinate `d` does not vanish on?
fn whole_multiple(e: &[num_rational::BigRational], d: &[num_rational::BigRational]) -> bool {
    use num_traits::Zero;
    let Some((ei, di)) = e.iter().zip(d).find(|(_, di)| !di.is_zero()) else { return false };
    let k = ei / di;
    k.is_integer() && e.len() == d.len() && e.iter().zip(d).all(|(x, y)| *x == &k * y)
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
        // 78.56 III — ORBITS ARE STRICTLY INTRA-TASK: a perception meets only
        // perceptions of its own family on a chart. Paired across families,
        // an orbit balanced across disjoint tasks that share no Hamiltonian
        // (τ_ext ≠ 0 by construction), and no thought ever grounded on the
        // fleet; what tasks share lives in the genome, as words.
        let family = me.family.clone();
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
                .filter(|(_, a)| self.bubbles.get(a).is_some_and(|b| b.family == family))
                .collect();
            let mut circuits: Vec<ChamberCircuit> = Vec::new();
            // The cell's bodies: this perception's and the fibre's on the chart.
            let mut bodies = mine.len();
            for (o, other) in &fibre {
                let Some(theirs) = self.bubbles.get(other).and_then(|b| b.spores.get(chart)).cloned() else { continue };
                bodies += theirs.len();
                for x in mine {
                    let mut row = Row {
                        chart: chart.clone(),
                        ts: Vec::with_capacity(theirs.len()),
                        lanes: Vec::with_capacity(theirs.len()),
                        nucleus: x.blade.vertex_nucleus(),
                        theirs: theirs.iter().map(|y| y.blade.vertex_nucleus()).collect(),
                        their_mass: theirs.iter().map(|y| y.blade.support.len() as u64).collect(),
                        body: crate::physics::tensegrity::body_of_shape(x),
                        their_bodies: theirs.iter().map(crate::physics::tensegrity::body_of_shape).collect(),
                    };
                    for y in &theirs {
                        match transition(x, y, budget) {
                            Ok(t) => {
                                self.resonance.circuits += 1;
                                circuits.push(ChamberCircuit {
                                    ring: circuit_ring(ok, x, *o, y),
                                    holonomy: Ok(t.versor.clone()),
                                    // A body is read hue-blind; the empty
                                    // listing is what the transition carries.
                                    hue: Some(Vec::new()),
                                });
                                row.lanes.push(lanes_of(x, y));
                                row.ts.push(Some(t));
                            }
                            Err(Unrelated::Unbounded) => {
                                self.resonance.note(Unrelated::Unbounded);
                                return;
                            }
                            Err(why) => {
                                self.resonance.note(why);
                                row.ts.push(None);
                                row.lanes.push(None);
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
            let cell = crate::transducer::cech::read_chamber(grade, None, None, &circuits, &orders, budget);
            self.note_chamber(&cell, chart, key, budget);
            self.note_shear(&circuits);
        }
        self.file_thoughts(key, &to, budget);
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
    fn note_shear(&mut self, circuits: &[ChamberCircuit]) {
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

    /// The cell's reading counted; each SHEAR held in the mind's field, its
    /// holonomy a field line whose turn flows on its plane's orbit, the
    /// precession kept as content (78.73, MU-CLIENT §4.4) — wound on the line
    /// of the thought whose perception read the cell (`key`, MU-CLIENT
    /// §4.13); and each ESCAPE filed as a demand on the chart by its class
    /// (the CS2 arm).
    fn note_chamber(&mut self, cell: &crate::transducer::cech::ChamberAlgebra, chart: &[u8], key: &[u8], budget: &mut crate::physics::Budget) {
        let shears: Vec<crate::transducer::family::pairing::FieldLine> = cell
            .coherence
            .shears()
            .iter()
            .map(|s| crate::transducer::family::pairing::FieldLine {
                carriage: s.residue.clone(),
                gauge: Vec::new(),
                mass: 1,
                free: false,
                line: crate::transducer::cech::Thread::Wound(key.to_vec()),
            })
            .collect();
        if !shears.is_empty() {
            let _ = crate::transducer::family::pairing::mark_flux(&[], &shears, budget);
        }
        let escapes = match &cell.coherence {
            Coherence::Open { escapes, .. } => escapes.len(),
            _ => 0,
        };
        if !shears.is_empty() || escapes > 0 {
            crate::observe::note_mind(|m| {
                m.shears += shears.len() as u64;
                m.escapes += escapes as u64;
            });
        }
        self.resonance.cells += 1;
        let r = &cell.rings;
        self.resonance.resonant += r.resonant as u64;
        self.resonance.rosette += r.rosette as u64;
        self.resonance.parabolic += r.parabolic as u64;
        self.resonance.boost += r.boost as u64;
        self.resonance.holonomic += r.holonomic as u64;
        match &cell.coherence {
            Coherence::Closes(_) => self.resonance.closes += 1,
            Coherence::Held { .. } => self.resonance.held += 1,
            Coherence::Unread => self.resonance.chambers_unread += 1,
            Coherence::Open { escapes, .. } => {
                self.resonance.open += 1;
                let charts: Vec<stalk::Invariant> = stalk::Invariant::from_key(chart).into_iter().collect();
                for inc in escapes {
                    match &inc.class {
                        // 78.68 ROW 8 — ANSWERED BY THE MIND'S OWN LETTER: a
                        // residue it has constructed and holds is said, not
                        // asked for again.
                        Ok(_) if self.holds_own_letter(&inc.residue) => {}
                        Ok(class) => {
                            // 78.114 — with the ring's holonomy as its residue.
                            self.docket_demand(class, &charts, Some(&inc.residue));
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
    fn file_thoughts(&mut self, key: &[u8], to: &Keyed<Vec<Row>>, budget: &mut crate::physics::Budget) {
        let mut rems: Keyed<Vec<Link>> = Keyed::new();
        if let Some(b) = self.bubbles.get(key) {
            rems.insert(key, b.remainder.placed());
        }
        for (other, rows) in to.scoped() {
            if let Some(b) = self.bubbles.get(&other) {
                rems.insert(&other, b.remainder.placed());
            }
            let all: Vec<&Transition> = rows.iter().flat_map(|r| r.ts.iter().flatten()).collect();
            if all.is_empty() {
                continue;
            }
            // 0. THE GROUND (78.52(6) Stage C): a grounded thought is met by
            // its own ground — this perception its predicted next hour, or
            // not; the thought's hours are its ground's, never re-read off
            // the bodies it has gathered.
            if self.join_ground(key, &other, rows, budget).is_some() {
                continue;
            }
            // 1. THE HOURS (78.48(7)): where every body of this perception
            // stands at the hour the other's orbit predicts, by one step, the
            // orbit continues — the edge carries the step, and what the step
            // carried from the latest hour to this one.
            let hours: Vec<Hour> = rows.iter().map(|r| hour_of(&r.ts)).collect();
            let steps: Vec<&Mv> = hours
                .iter()
                .filter_map(|h| match h {
                    Hour::Predicted(g) => Some(g),
                    Hour::NoOrbit | Hour::Slipped { .. } => None,
                })
                .collect();
            self.resonance.predicted += steps.len() as u64;
            let every = hours.iter().all(|h| matches!(h, Hour::Predicted(_)));
            if let (true, [g, more @ ..]) = (every, steps.as_slice()) {
                if let (true, Some((rotor, carriage))) = (more.iter().all(|s| s.projective_eq(g)), halves(g)) {
                    if let Err(why) = self.gate(rows, g, true, &tether_line(key, &other), budget) {
                        self.note_refusal(&why, g, rows);
                        continue;
                    }
                    self.note_ground(key, &other, rows, g, true, budget);
                    let latest = rows.iter().filter_map(|r| match r.lanes.as_slice() {
                        [.., Some(l)] => Some(*l),
                        _ => None,
                    });
                    let lanes = self.lanes_summed(latest);
                    if let [row, ..] = rows.as_slice() {
                        self.carry_vortices_round(&rotor, &row.chart);
                    }
                    let _ = self.tether(key, &other, rotor, carriage, lanes);
                    self.resonance.edges += 1;
                    continue;
                }
            }
            // 2. THE SPECTRUM (78.50): the pair's rays, each with what it
            // carries and lands on. A SCENE MOTION is a ray that carries the
            // whole of one side onto the other — every body of this
            // perception carried, or every body of the other landed on.
            let (spectrum, unread) = spectrum_of(rows);
            self.resonance.unread_lines += unread;
            let theirs = Self::their_bodies(rows);
            let whole: Vec<&Line> = spectrum
                .scoped()
                .map(|(_, l)| l)
                .filter(|l| {
                    (0..rows.len()).all(|i| l.carried.contains(&row_key(i)))
                        || theirs.scoped().all(|(k, _)| l.lands.contains(&k))
                })
                .collect();
            match whole.as_slice() {
                // ONE MOTION CARRIES THE WHOLE CONFIGURATION: the thought.
                [line] => {
                    self.note_unmatched(rows, &theirs, line);
                    match self.gate(rows, &line.t.versor, false, &tether_line(key, &other), budget) {
                        Ok(()) => {
                            self.note_ground(key, &other, rows, &line.t.versor, false, budget);
                            let chart = Self::chart_of_rows(rows);
                            self.scene_edge(key, &other, &line.t, line.lanes, &chart);
                        }
                        Err(why) => self.note_refusal(&why, &line.t.versor, rows),
                    }
                }
                // SEVERAL, CARRYING THE SAME BODIES ONTO THE SAME BODIES: one
                // coset — they differ by symmetries of the configuration. The
                // edge carries the canonical representative; each quotient, a
                // symmetry, is filed by its class at its period.
                [first, rest @ ..] if rest.iter().all(|l| same_landing(l, first) && same_carried(l, first)) => {
                    let members: Vec<Transition> = whole.iter().map(|l| l.t.clone()).collect();
                    if let Some(rep) = crate::transducer::blade::coset_representative(&members) {
                        if let Err(why) = self.gate(rows, &rep.versor, false, &tether_line(key, &other), budget) {
                            self.note_refusal(&why, &rep.versor, rows);
                            continue;
                        }
                        let chart = Self::chart_of_rows(rows);
                        self.scene_edge(key, &other, rep, first.lanes, &chart);
                        self.resonance.cosets += 1;
                        for m in &members {
                            let q = m.versor.geo(&rep.versor.reverse());
                            if unit_ray(&q) {
                                continue;
                            }
                            self.resonance.symmetries += 1;
                            if let Some((r, c)) = halves(&q) {
                                let fill = crate::transducer::cech::classify(&r, &c, sentence::CycleType::Identity);
                                self.complex.index_congruent(&fill);
                            }
                        }
                        self.note_unmatched(rows, &theirs, first);
                    }
                }
                // NO ONE MOTION CARRIES THE SCENE: the spectrum is kept, each
                // ray by its class, and every slipped hour is the demand of the
                // thought it slipped against, worked at that thought's return.
                _ => {
                    self.resonance.several += 1;
                    for (_, line) in spectrum.scoped() {
                        self.keep_orbit(key, &other, &line.t);
                    }
                    for (row, h) in rows.iter().zip(&hours) {
                        if let Hour::Slipped { on_orbit, residue } = h {
                            self.file_slip(&row.chart, residue, *on_orbit, &other);
                        }
                    }
                }
            }
        }
        let reopened = self.complex.coboundary_on_link(key, |verts| volume_of(&rems, verts));
        self.note_decohered(reopened, super::meet::Adjunct::Candidate);
        self.decide_neighbors(key);
    }

    /// A.10.1 — THE CONSOLIDATION GATE, before any thought is formed. The
    /// other's bodies are the constellation BEFORE, and with this perception's
    /// bodies joined, AFTER; the physics decides (`physics::tensegrity`):
    /// admitted where every constellation's attitudes stand in equilibrium,
    /// its strain did not rise, and — in a scene — every pair of bodies kept
    /// its inversive distance.
    ///
    /// THE REFLEXIVE (78.52 Row 1): an identity relation is one frame seen
    /// twice — the diagonal morphism, no constellation — and bypasses the
    /// gate; its fill (Filled) decides.
    ///
    /// THE SUCCESSION (78.52 Row 1): the relation's hour rotor
    /// (`tensegrity::succession`) — its turn paired isoclinically with the
    /// same turn in `e₃e₊`, or a slide's own parameter — centred on the
    /// constellation's own centre (78.51(3), rule 15). Hour `m`'s frame is the
    /// body's nucleus at its first hour carried by the rotor's `m`-th power
    /// (`tensegrity::hour`), the rotor the constellation's generator.
    ///
    /// - On an ORBIT (tier 1) each of this perception's bodies is ONE body
    ///   across its hours: the other's bodies its hours `0 … m−1` in arrival
    ///   order, carried back by `t_j t̃_1`, and this body hour `m`, carried
    ///   back by `t_1`; `c` the body's own centroid at its first hour. A.13:
    ///   the orbit's step must carry each hour's momentum to the next, or the
    ///   external torque is refused.
    /// - In a SCENE (tier 2) the other's bodies stand at hour 0 as they are,
    ///   this perception's at hour 1 carried back by the scene's motion, one
    ///   constellation per chart; `c` the other's barycentre, each body
    ///   weighed by its mass (its sites).
    ///
    /// What the admitted relation holds in the mind's field is wound on its
    /// tether's line, `line` (MU-CLIENT §4.13, Reading 2).
    fn gate(&mut self, rows: &[Row], generator: &Mv, orbit: bool, line: &[u8], budget: &mut crate::physics::Budget) -> Result<(), crate::physics::tensegrity::Refusal> {
        use crate::physics::tensegrity::{
            barycentre, coherence, external_torque, hour, metric_defect, succession, Constellation, Defect, Refusal,
        };
        // 78.57 — THE COHERENCE FORM: every defect of this transition read,
        // each in its own grade; never the first alone, never summed.
        let mut defect = Defect::default();
        use crate::transducer::cga;
        type Q = num_rational::BigRational;
        // MU-CLIENT §4.5 — THE BODIES THAT MET in a scene: each by its slot
        // (fused, precessing, contact) and the sphere pair's versor.
        let mut met: Vec<(usize, Mv)> = Vec::new();
        let mut coupled: Vec<crate::physics::tensegrity::Coupling> = Vec::new();
        if unit_ray(generator) {
            self.resonance.reflexive += 1;
            return Ok(());
        }
        let ray = |m: Mv| match m.canonical() {
            Some(c) => c,
            None => m,
        };
        // The constellations: per row on an orbit, per chart in a scene.
        let mut groups: Vec<Vec<&Row>> = Vec::new();
        match orbit {
            true => groups.extend(rows.iter().map(|r| vec![r])),
            false => {
                let mut charts: Keyed<()> = Keyed::new();
                for r in rows {
                    charts.insert(&r.chart, ());
                }
                groups.extend(charts.scoped().map(|(chart, _)| rows.iter().filter(|r| r.chart == chart).collect::<Vec<_>>()));
            }
        }
        for on in groups {
            let [first, ..] = on.as_slice() else { continue };
            let [Some(t11), ..] = first.ts.as_slice() else { continue };
            // Each of the other's bodies at its first hour, with its mass.
            let mut theirs: Vec<(Mv, Q)> = Vec::new();
            for ((t, s), m) in first.ts.iter().zip(&first.theirs).zip(&first.their_mass) {
                let (Some(t), Some(s)) = (t, s) else { continue };
                let m = Q::from_integer((*m).into());
                theirs.push(match orbit {
                    true => {
                        let back = t.versor.geo(&t11.versor.reverse());
                        (ray(back.reverse().sandwich(s)), m)
                    }
                    false => (s.clone(), m),
                });
            }
            // The centre: on an orbit the body's own at its first hour — the
            // first to arrive, arrival order being the orbit's content — in a
            // scene the barycentre of all. A centre that does not read (a
            // nucleus at infinity) leaves no frame standing.
            let centres: Option<Vec<(Vec<Q>, Q)>> = match (orbit, theirs.as_slice()) {
                (true, [(s, m), ..]) => cga::base_at(s, usize::from(s.n())).map(|c| vec![(c, m.clone())]),
                (true, []) => None,
                (false, _) => theirs.iter().map(|(s, m)| cga::base_at(s, usize::from(s.n())).map(|c| (c, m.clone()))).collect(),
            };
            let Some(star) = centres.as_deref().and_then(barycentre) else {
                return Err(Refusal::MonocularOrbit);
            };
            let Some(r_hour) = succession(generator, &star.centre).and_then(|r| r.canonical()) else {
                return Err(Refusal::NoSuccession);
            };
            let before: Vec<_> = (0u64..)
                .zip(&theirs)
                .map(|(m, (s, _))| hour(s, &r_hour, if orbit { m } else { 0 }))
                .collect();
            let next = if orbit { theirs.len() as u64 } else { 1 };
            let mut after = before.clone();
            for r in &on {
                let (Some(x), [Some(t), ..]) = (&r.nucleus, r.ts.as_slice()) else { continue };
                let carry = if orbit { &t.versor } else { generator };
                after.push(hour(&ray(carry.sandwich(x)), &r_hour, next));
            }
            let (before, after) = (
                Constellation { frames: before, generator: r_hour.clone() },
                Constellation { frames: after, generator: r_hour },
            );
            // THE STRUCTURE FIRST — frames that do not stand, or a strain that
            // rises, leave no form to read, refused — then Δω, the torque the
            // couplings leave on the generator.
            let read = coherence(&before, &after, budget)?;
            defect.torsion.extend(read.torsion);
            defect.tilted.extend(read.tilted);
            // τ_ext — A.13 (78.52 Row 1): on an orbit, the step carries each
            // hour's momentum to the next — the other's bodies in arrival
            // order, then this one — and what it did not carry is an external
            // torque, every step read.
            if orbit {
                for r in &on {
                    let hours: Option<Vec<&crate::physics::tensegrity::Body>> =
                        first.their_bodies.iter().chain(std::iter::once(&r.body)).map(Option::as_ref).collect();
                    let Some(hours) = hours else { continue };
                    for pair in hours.windows(2) {
                        if let [a, b] = pair {
                            match external_torque(a, b, generator) {
                                Some(tau) if !tau.is_zero() => defect.momentum.push(tau),
                                Some(_) => {}
                                None => return Err(Refusal::NoSuccession),
                            }
                        }
                    }
                }
            }
            // METRIC RIGIDITY (78.51(4′), at unit weight by 78.59): in a
            // scene, each of this perception's bodies against the other's body
            // the motion lands it on, every pair whose unit-weight product moved. A body landing on
            // none appeared, and stands in no pair.
            if !orbit {
                let mut bodies: Vec<(Mv, Mv)> = Vec::new();
                for r in &on {
                    let Some(x) = &r.nucleus else { continue };
                    let landed = generator.sandwich(x);
                    if let Some((s, _)) = theirs.iter().find(|(s, _)| s.projective_eq(&landed)) {
                        bodies.push((s.clone(), x.clone()));
                    }
                    // A COLLISION OF BODIES (78.52 Row 2): this body, carried
                    // into the other's configuration, against each of the
                    // other's bodies it does not land on — their spheres cut
                    // (fused at a crystallographic angle, precessing at any
                    // other) or touch (contact). Disjoint spheres are two
                    // bodies of one system, never a collision; one sphere
                    // twice is the landing itself.
                    let landed = ray(landed);
                    for (s, _) in theirs.iter().filter(|(s, _)| !s.projective_eq(&landed)) {
                        let slot = match cga::pair_regime(&[s.clone(), landed.clone()], 0, 1) {
                            cga::PairRegime::Finite(_) => 0,
                            cga::PairRegime::Rosette => 1,
                            cga::PairRegime::Parabolic => 2,
                            cga::PairRegime::Boost | cga::PairRegime::Null => continue,
                        };
                        met.push((slot, s.geo(&landed)));
                    }
                    // SPIN COUPLED TO ORBIT (MU-CLIENT §4.4): the scene's step
                    // carrying this body about the other's barycentre, split
                    // into orbit and spin and read as a lock or a drift.
                    if let (Some(from), Some(to)) = (cga::base_at(x, usize::from(x.n())), cga::base_at(&landed, usize::from(landed.n()))) {
                        if let Some(c) = crate::physics::tensegrity::spin_orbit(generator, &star, &from, &to) {
                            coupled.push(c);
                        }
                    }
                }
                defect.metric.extend(metric_defect(&bodies));
            }
        }
        // THE FORM CLOSES OR IT FILES WHAT IS MISSING: zero on every
        // component, exactly, and the constellation locks; otherwise the
        // whole defect is the answer, blade by blade.
        if !defect.closes() {
            return Err(Refusal::Incoherent { defect: Box::new(defect) });
        }
        // The admitted scene's collisions: a fusion returns and sends
        // nothing; a precession's sphere-pair turn and a contact's null flux
        // are held in the mind's field (𝒽, 𝒯), the torsion and the graze its
        // content.
        use crate::physics::tensegrity::Coupling;
        let drifts = coupled.iter().filter_map(|c| match c {
            Coupling::Drift(d) => Some(d),
            Coupling::Lock(..) => None,
        });
        let held: Vec<crate::transducer::family::pairing::FieldLine> = met
            .iter()
            .filter(|(slot, _)| *slot != 0)
            .map(|(_, v)| v)
            .chain(drifts)
            .map(|v| crate::transducer::family::pairing::FieldLine {
                carriage: v.clone(),
                gauge: Vec::new(),
                mass: 1,
                free: false,
                line: crate::transducer::cech::Thread::Wound(line.to_vec()),
            })
            .collect();
        if !held.is_empty() {
            let _ = crate::transducer::family::pairing::mark_flux(&[], &held, budget);
        }
        if !met.is_empty() || !coupled.is_empty() {
            crate::observe::note_mind(|m| {
                for (slot, _) in &met {
                    m.bodies[*slot] += 1;
                }
                for c in &coupled {
                    match c {
                        Coupling::Lock(p, q) => m.lock(*p, *q),
                        Coupling::Drift(_) => m.drifts += 1,
                    }
                }
            });
        }
        self.resonance.consolidated += 1;
        Ok(())
    }

    /// 78.52 — THE GROUND OF AN ADMITTED RELATION, filed by the pair it
    /// relates for the merge its fill decides, with the hours the merged
    /// thought will have seen.
    /// - On an ORBIT (one body of this perception against one body's hours
    ///   `0 … m−1`): the base is that body's first hour; the thought will have
    ///   seen `m + 1`; the generator the orbit's step `g = t₂ t̃₁`.
    /// - In a SCENE (every body of this perception on one chart against every
    ///   body of the other there, 78.52(6) Stage C): the base is the other's
    ///   bodies in the order they arrived; the thought will have seen 2; the
    ///   generator the scene's motion forward, `t̃` (the transition carries
    ///   this perception's bodies onto the other's).
    ///
    /// The coupling is the generator's succession at the base's barycentre;
    /// the period its return. A grounded other is joined by its own ground
    /// (`join_ground`), never here; a relation that does not return (a slide,
    /// a boost) has no ground and speaks no word.
    ///
    /// MU-CLIENT §4.13 (Q1 as ruled) — a ground filed winds its line: its
    /// generator admitted on the thought's own line once for each hour seen
    /// (`wind_ground`), so its orbit returns when its winding does.
    fn note_ground(&mut self, key: &[u8], other: &[u8], rows: &[Row], generator: &Mv, orbit: bool, budget: &mut crate::physics::Budget) {
        if self.bubbles.get(other).is_some_and(|b| b.ground.is_some()) {
            return;
        }
        let [first, rest @ ..] = rows else { return self.ungrounded(Ungrounded::Unplaced) };
        if rest.iter().any(|r| r.chart != first.chart) {
            return self.ungrounded(Ungrounded::Unplaced);
        }
        let bodies: Option<Vec<crate::physics::tensegrity::GroundBody>> = match orbit {
            true => {
                let ([], [Some(nucleus), ..], [Some(body), ..]) =
                    (rest, first.theirs.as_slice(), first.their_bodies.as_slice())
                else {
                    return self.ungrounded(Ungrounded::Unplaced);
                };
                Some(vec![crate::physics::tensegrity::GroundBody { nucleus: nucleus.clone(), body: body.clone() }])
            }
            false => first
                .theirs
                .iter()
                .zip(&first.their_bodies)
                .map(|(s, b)| match (s, b) {
                    (Some(nucleus), Some(body)) => {
                        Some(crate::physics::tensegrity::GroundBody { nucleus: nucleus.clone(), body: body.clone() })
                    }
                    _ => None,
                })
                .collect(),
        };
        let Some(bodies) = bodies else { return self.ungrounded(Ungrounded::Unplaced) };
        let hours = if orbit { first.theirs.len() as u64 + 1 } else { 2 };
        let forward = if orbit { generator.clone() } else { generator.reverse() };
        match ground_of(bodies, &forward) {
            Ok(ground) => {
                let grounded = super::idea::Grounded { chart: first.chart.clone(), hours, ground, twist: None };
                self.wind_ground(&grounded, hours, budget);
                self.grounds.insert(pair_key(key, other), grounded);
                self.resonance.grounds[0] += 1;
            }
            // MU-CLIENT §4.12 — A RELATION THAT NEVER RETURNS IS AN ESCAPING
            // ORBIT, NEVER NOTHING: no ground (it has no period to wind), and
            // it is DOCKETED as the demand it is — its class, the chart it
            // stands on as its cover, its motion the residue — for the demand
            // taker to bind (`take_demands`, the one taker). Demanding it
            // return would be squaring (78.168 §2); dropping it left every
            // scene of slides with no thought at all (read on the fleet
            // 2026-10-15: grounds 0, no return 17).
            // A motion the cell already docketed (an open cell's escape) is
            // that one demand. Either way it stands in the scene of the
            // perception that filed it (`Contemplation::demands_in_scene`).
            Err(Ungrounded::NoReturn) if self.docketed_residue(&forward) => self.ungrounded(Ungrounded::NoReturn),
            Err(Ungrounded::NoReturn) => {
                self.ungrounded(Ungrounded::NoReturn);
                let class = halves(&forward).and_then(|(r, c)| sentence::ClassVector::of_rotor(&r, c, sentence::CycleType::Identity));
                match (class, stalk::Invariant::from_key(&first.chart)) {
                    (Some(k), Some(chart)) => {
                        self.docket_demand(&k, &[chart], Some(&forward));
                        self.resonance.demanded += 1;
                    }
                    _ => self.resonance.undemanded += 1,
                }
            }
            Err(why) => self.ungrounded(why),
        }
    }

    fn ungrounded(&mut self, why: Ungrounded) {
        self.resonance.grounds[why as usize] += 1;
    }

    /// 78.52(6) Stage C — A NEW PERCEPTION MEETS A GROUNDED THOUGHT BY ITS
    /// GROUND. The thought has seen `h` hours of its return; its ground
    /// predicts hour `h`, `g^h` carrying each of its bodies. Where this
    /// perception's bodies on the ground's chart are exactly that
    /// configuration — as many, each landing on one — the relation is read
    /// by the physics: the constellation of every hour through `h` balanced
    /// (law II), each pair's inversive distance kept, each body's momentum
    /// carried from hour `h − 1` (A.13). Admitted, it is the thought's next
    /// hour: the edge carries the step, and the ground is filed for the merge
    /// with `h + 1` hours seen; refused, the refusal is filed by name as the
    /// gate's are. `Some(true)` joined, `Some(false)` refused; `None` where the
    /// thought holds no ground or the perception is not its predicted hour —
    /// the other readings decide.
    fn join_ground(&mut self, key: &[u8], other: &[u8], rows: &[Row], budget: &mut crate::physics::Budget) -> Option<bool> {
        let (joined, generator) = self.read_ground(key, other, rows, budget)?;
        match joined {
            Ok(()) => Some(true),
            Err(why) => {
                self.note_refusal(&why, &generator, rows);
                Some(false)
            }
        }
    }

    /// MU-CLIENT §4.4(1), §4.12 — THE RETURN TWISTED BY ITS PULL: the
    /// rotation about the orbit's attractor (the centre its generator holds,
    /// `cga::centre_of`) that carries the constellation the orbit predicted at
    /// this hour onto the bodies perceived (`cga::twist_about`), every body by
    /// the one twist. The ground is SHEARED by it — its bodies stand where the
    /// twist puts them at hour 0, the orbit predicted from there — and the
    /// twist ACCRUES on the thought's winding, its content (curvature is
    /// content). Read against the orbit's plane it splits into DRAG (the
    /// rate) and LIFT (the tilt, 78.46), each counted. `None` where no
    /// rotation of the pull carries it: the body was carried off the
    /// attractor (a slide away, a change of size, another body).
    fn twisted_return(&mut self, held: super::idea::Grounded, mine: &[(&Mv, &crate::physics::tensegrity::Body)], power: &Mv) -> Option<super::idea::Grounded> {
        use crate::physics::tensegrity::{carried_body, GroundBody};
        let g = &held.ground;
        let centre = match crate::transducer::cga::centre_of(&g.generator, g.generator.n()) {
            crate::transducer::cga::Centre::Fixed { point, .. } => point,
            _ => return None,
        };
        let n = usize::from(g.generator.n());
        let [first, ..] = g.bodies.as_slice() else { return None };
        let predicted_first = power.sandwich(&first.nucleus);
        let lands = |tau: &Mv| {
            g.bodies.iter().all(|b| {
                let q = tau.sandwich(&power.sandwich(&b.nucleus));
                mine.iter().any(|(s, _)| s.projective_eq(&q))
            })
        };
        let tau = mine.iter().find_map(|(s, _)| crate::transducer::cga::twist_about(&centre, &predicted_first, s, n).filter(|t| lands(t)))?;
        // The twist read at hour 0: the ground sheared where its hours begin.
        let at_zero = power.reverse().geo(&tau).geo(power);
        let bodies: Option<Vec<GroundBody>> = g
            .bodies
            .iter()
            .map(|b| Some(GroundBody { nucleus: at_zero.sandwich(&b.nucleus), body: carried_body(&b.body, &at_zero)? }))
            .collect();
        let sheared = ground_of(bodies?, &g.generator).ok()?;
        self.resonance.twisted += 1;
        if let Some(split) = crate::transducer::cga::lift_drag(&g.generator.grade(2), &tau.grade(2)) {
            if !split.lift.is_zero() {
                self.resonance.lifted += 1;
            }
        }
        let twist = Some(match &held.twist {
            Some(t) => at_zero.geo(t),
            None => at_zero,
        });
        Some(super::idea::Grounded { ground: sheared, twist, ..held })
    }

    /// `join_ground`'s reading: the verdict and the ground's generator.
    fn read_ground(&mut self, key: &[u8], other: &[u8], rows: &[Row], budget: &mut crate::physics::Budget) -> Option<(Verdict, Mv)> {
        use crate::physics::tensegrity::{carried_body, coherence, external_torque, hour, metric_defect, Constellation, Refusal};
        let held = self.bubbles.get(other)?.ground.clone()?;
        let mine: Vec<(&Mv, &crate::physics::tensegrity::Body)> = rows
            .iter()
            .filter(|r| r.chart == held.chart)
            .filter_map(|r| match (&r.nucleus, &r.body) {
                (Some(s), Some(b)) => Some((s, b)),
                _ => None,
            })
            .collect();
        if mine.len() != held.ground.bodies.len() {
            return None;
        }
        let h = held.hours % held.ground.period;
        let mut power = Mv::scalar(num_traits::One::one());
        for _ in 0..h {
            let next = held.ground.generator.geo(&power);
            power = match next.canonical() {
                Some(ray) => ray,
                None => next,
            };
        }
        // MU-CLIENT §4.4(1) — THE ORBIT IS SHEARED BY ITS PULL: where the
        // perception does not stand exactly where the orbit predicted, the
        // twist of the pull is read (`twisted_return`) and the ground stands
        // sheared — read from where the orbit now is, never demanded back
        // onto a perfect circle. No twist of the pull carries it there: it was
        // carried off the attractor, and the hours below read the slip.
        let exact = held.ground.bodies.iter().all(|base| {
            let predicted = power.sandwich(&base.nucleus);
            mine.iter().any(|(s, _)| s.projective_eq(&predicted))
        });
        let grounded = match exact {
            true => held,
            false => self.twisted_return(held, &mine, &power)?,
        };
        let g = &grounded.ground;
        // Each body of the ground, at its predicted place, against the one of
        // this perception's bodies that stands there.
        let mut matched: Vec<(&crate::physics::tensegrity::GroundBody, &Mv, &crate::physics::tensegrity::Body)> =
            Vec::with_capacity(mine.len());
        for base in &g.bodies {
            let predicted = power.sandwich(&base.nucleus);
            let (s, b) = mine.iter().find(|(s, _)| s.projective_eq(&predicted))?;
            matched.push((base, s, b));
        }
        // THE HOURS BEFORE THIS ONE are the orbit's most recent return
        // (MU-CLIENT §4.4(2)): the last `period − 1` hours seen, each at its
        // hour on the orbit — within the first return exactly `0 … h−1`; past
        // it the window winds on, so a thought coming round again is read
        // against the return it completed, never against no hours at all.
        let seen = grounded.hours;
        let start = match seen.checked_sub(g.period - 1) {
            Some(s) => s,
            None => 0,
        };
        let before: Vec<_> = (start..seen).flat_map(|m| g.bodies.iter().map(move |b| hour(&b.nucleus, &g.coupling, m % g.period))).collect();
        let mut after = before.clone();
        after.extend(g.bodies.iter().map(|b| hour(&b.nucleus, &g.coupling, h)));
        // 78.57 — THE COHERENCE FORM of the next hour, whole: Δω, then the metric defect of
        // every matched body, then τ_ext of every body's step; closed only
        // where every component vanishes.
        let verdict = coherence(
            &Constellation { frames: before, generator: g.coupling.clone() },
            &Constellation { frames: after, generator: g.coupling.clone() },
            budget,
        )
        .and_then(|mut defect| {
            defect.metric.extend(metric_defect(
                &matched.iter().map(|(base, s, _)| (base.nucleus.clone(), (*s).clone())).collect::<Vec<_>>(),
            ));
            // A.13: each body carried to hour h − 1, then by one step to this
            // perception's body at hour h. Hour 0 again (a return completed)
            // is carried from the ground's last hour.
            let steps = if h == 0 { g.period - 1 } else { h - 1 };
            for (base, _, b) in &matched {
                let mut carried = base.body.clone();
                for _ in 0..steps {
                    carried = carried_body(&carried, &g.generator).ok_or(Refusal::NoSuccession)?;
                }
                match external_torque(&carried, b, &g.generator) {
                    Some(tau) if !tau.is_zero() => defect.momentum.push(tau),
                    Some(_) => {}
                    None => return Err(Refusal::NoSuccession),
                }
            }
            match defect.closes() {
                true => Ok(()),
                false => Err(Refusal::Incoherent { defect: Box::new(defect) }),
            }
        });
        let generator = g.generator.clone();
        if let Err(why) = verdict {
            return Some((Err(why), generator));
        }
        let Some((rotor, carriage)) = halves(&generator) else {
            return Some((Err(Refusal::NoSuccession), generator));
        };
        self.resonance.consolidated += 1;
        self.resonance.predicted += 1;
        let _ = self.tether(key, other, rotor, carriage, [0; 4]);
        self.resonance.edges += 1;
        // MU-CLIENT §4.4(2) — THE WINDING ADVANCES, never stops at the period:
        // the hour past a return is hour 0 again (`hours % period` above), and
        // the thought keeps every hour it has seen, so each return it completes
        // is read (`closing_ground`). A count past its carrier is refused.
        let Some(seen) = grounded.hours.checked_add(1) else {
            return Some((Err(Refusal::NoSuccession), g.generator.clone()));
        };
        let next = super::idea::Grounded { hours: seen, ..grounded.clone() };
        // MU-CLIENT §4.13 (Q1 as ruled) — the hour joined is one more
        // transition of the thought's orbit in the field, on its own line.
        self.wind_ground(&next, 1, budget);
        self.grounds.insert(pair_key(key, other), next);
        Some((Ok(()), generator))
    }

    /// A REFUSED CONSOLIDATION, filed by name (A.10.1 §4): an unbalanced,
    /// destabilizing, non-rigid or externally torqued relation stands as a
    /// demand on its charts by its generator's class — the torque it left its
    /// residue, the change in the pair's inversive distance, or the external
    /// torque; a monocular orbit, a collapse or a step with no succession is
    /// counted.
    fn note_refusal(&mut self, why: &crate::physics::tensegrity::Refusal, generator: &Mv, rows: &[Row]) {
        use crate::physics::tensegrity::Refusal;
        // The residues this refusal files, each its own demand: one blade
        // per defect component, never summed (78.57).
        let residues: Vec<Option<Mv>> = match why {
            Refusal::MonocularOrbit => {
                self.resonance.monocular += 1;
                return;
            }
            Refusal::SingularCollapse => {
                self.resonance.collapsed += 1;
                return;
            }
            Refusal::UnbalancedTensegrity { net_torque } => {
                self.resonance.unbalanced += 1;
                vec![Some(net_torque.clone())]
            }
            // 78.62 — the planes the operation raised, each its own demand.
            Refusal::DestabilizingOperation { rising } => {
                self.resonance.destabilizing += 1;
                rising.iter().cloned().map(Some).collect()
            }
            Refusal::StabilityUnread { .. } => {
                self.resonance.stability_unread += 1;
                return;
            }
            Refusal::NoSuccession => {
                self.resonance.unsucceeded += 1;
                return;
            }
            // A.13: the external torque — what exerted it is the demand.
            Refusal::ExternalTorque { tau } => {
                self.resonance.torqued += 1;
                vec![Some(tau.clone())]
            }
            // The metric demand: the pair's unit-weight ratio.
            Refusal::NonRigid { delta, .. } => {
                self.resonance.nonrigid += 1;
                vec![Some(Mv::scalar(delta.clone()))]
            }
            // 78.57 — THE COHERENCE FORM, whole: each component counted on its
            // own counter and filed as its own demand — Δω the missing rotor,
            // the metric defect the missing tangency, τ_ext the unmodelled force.
            Refusal::Incoherent { defect } => {
                self.resonance.incoherent += 1;
                if !defect.torsion.is_empty() {
                    self.resonance.unbalanced += 1;
                }
                if !defect.metric.is_empty() {
                    self.resonance.nonrigid += 1;
                }
                if !defect.momentum.is_empty() {
                    self.resonance.torqued += 1;
                }
                if !defect.tilted.is_empty() {
                    self.resonance.tilted += 1;
                }
                defect
                    .torsion
                    .iter()
                    .cloned()
                    .chain(defect.metric.iter().map(|(_, _, d)| Mv::scalar(d.clone())))
                    .chain(defect.momentum.iter().cloned())
                    .chain(defect.tilted.iter().cloned())
                    .map(Some)
                    .collect()
            }
        };
        let class = halves(generator).and_then(|(r, c)| sentence::ClassVector::of_rotor(&r, c, sentence::CycleType::Identity));
        let Some(class) = class else {
            self.resonance.undemanded += 1;
            return;
        };
        let mut charts: Keyed<()> = Keyed::new();
        for r in rows {
            charts.insert(&r.chart, ());
        }
        let charts: Vec<stalk::Invariant> = charts.scoped().filter_map(|(k, _)| stalk::Invariant::from_key(&k)).collect();
        for residue in &residues {
            self.docket_demand(&class, &charts, residue.as_ref());
            self.resonance.demanded += 1;
        }
    }

    /// Lanes summed lane by lane (law 5); a sum past its carrier is counted
    /// and stands where it was (STD1).
    fn lanes_summed(&mut self, each: impl Iterator<Item = [u64; 4]>) -> [u64; 4] {
        let mut lanes = [0u64; 4];
        for l in each {
            if !crate::core::lanes::lanes_step(&mut lanes, &l) {
                self.overflowed += 1;
            }
        }
        lanes
    }

    /// A SCENE MOTION'S EDGE (78.50): the ray that carries the whole
    /// configuration, written as the complex carries an edge — its rotor and
    /// carriage — with what it carried, per lane. The complex's fill decides
    /// what the thought is.
    fn scene_edge(&mut self, key: &[u8], other: &[u8], t: &Transition, lanes: [u64; 4], chart: &[u8]) {
        let Some((rotor, carriage)) = halves(&t.versor) else {
            self.resonance.undemanded += 1;
            return;
        };
        self.carry_vortices_round(&rotor, chart);
        let _ = self.tether(key, other, rotor, carriage, lanes);
        self.resonance.edges += 1;
        self.resonance.scene_motions += 1;
    }

    /// MU-CLIENT §4.8, §7.1 R3 — THE VORTEX CARRIED ROUND ON A NEW
    /// PERCEPTION: a perception joins a thought by a motion, and that motion
    /// is read against every vortex held at its rotor's conjugacy class
    /// (`collapse_on`, the one reader a glue and the syllabus collapse by).
    /// The candidate the perception's own move carries — whose orbit the new
    /// scene brings round — is the fill that collapses the superposition;
    /// the others are a symmetry the mind measured, kept for its letters
    /// (Expansion 4). A perception carrying no candidate collapses nothing:
    /// the vortex is carried on whole.
    fn carry_vortices_round(&mut self, rotor: &crate::transducer::alphabet::Rotor, chart: &[u8]) {
        let Some(chart) = stalk::Invariant::from_key(chart) else { return };
        let kept = self.collapse_on(std::slice::from_ref(rotor), &chart, "perception");
        self.vortex_symmetries.extend(kept);
    }

    /// The chart the pair's first row stands on: where a scene's motion
    /// meets a thought; none for no row.
    fn chart_of_rows(rows: &[Row]) -> Vec<u8> {
        match rows {
            [row, ..] => row.chart.clone(),
            [] => Vec::new(),
        }
    }

    /// The other's bodies on the charts the pair shares, by their landing
    /// keys.
    fn their_bodies(rows: &[Row]) -> Keyed<()> {
        let mut theirs: Keyed<()> = Keyed::new();
        for row in rows {
            for j in 0..row.ts.len() {
                theirs.insert(land_key(&row.chart, j), ());
            }
        }
        theirs
    }

    /// WHAT THE SCENE MOTION LEAVES UNMATCHED: this perception's bodies it
    /// does not carry — this is the newer scene, so they APPEARED — and the
    /// other's bodies it lands nothing on, which VANISHED.
    fn note_unmatched(&mut self, rows: &[Row], theirs: &Keyed<()>, line: &Line) {
        self.resonance.appeared += (0..rows.len()).filter(|&i| !line.carried.contains(&row_key(i))).count() as u64;
        self.resonance.vanished += theirs.scoped().filter(|(k, _)| !line.lands.contains(k)).count() as u64;
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
    /// A SLIP OFF A THOUGHT'S ORBIT, counted and HELD ON THAT THOUGHT
    /// (MU-CLIENT §4.4(3), §7.1): its own demand, worked at its return
    /// (`work_slips`). Where no thought stands to hold it, it is docketed now.
    fn file_slip(&mut self, chart: &[u8], residue: &Mv, on_orbit: bool, thought: &[u8]) {
        match on_orbit {
            true => self.resonance.phase_slips += 1,
            false => self.resonance.off_orbit += 1,
        }
        match self.bubbles.get_mut(thought) {
            Some(b) => b.slips.push((chart.to_vec(), residue.clone())),
            None => self.docket_slip(chart, residue),
        }
    }

    /// AT A THOUGHT'S RETURN, ITS SLIPS WORKED: each docketed as the demand
    /// its residue's class is, on the chart it slipped on, for the demand
    /// taker to construct.
    pub(crate) fn work_slips(&mut self, thought: &[u8]) {
        let slips = match self.bubbles.get_mut(thought) {
            Some(b) => std::mem::take(&mut b.slips),
            None => return,
        };
        for (chart, residue) in slips {
            self.docket_slip(&chart, &residue);
        }
    }

    /// A slip's residue docketed as the demand its class is, on its chart.
    fn docket_slip(&mut self, chart: &[u8], residue: &Mv) {
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
