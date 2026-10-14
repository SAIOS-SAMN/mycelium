// 78.160 P1 — THE PRESENTATION BY HOLES: every word vertex a language holds,
// filed under the holes its whole motion winds about, so a motion it is asked
// of is deconstructed over what is held, hole by hole (78.100 D3), at the
// coordinates the motion itself names — no chart, no lane, no search.
//
//! A court holds one over the words it ruled and answers a mind's residue with
//! it: the mind's pressure is the ask, the residue's own holes select what is
//! sent, and the exact composition of what is cited is the certificate. A
//! word's motion is its least member's carriage composed with its rotor's base
//! action (`conformal_of_rotor`), read in Cl(4,1) — never the rotor alone.

use super::coordinates::{Coordinates, HoleIndex, Unread};
use super::{Form, Vertex};
use crate::network::nerve::Keyed;
use crate::transducer::address::Address;
use crate::transducer::cga::Mv;
use num_rational::BigRational;

/// ONE GENERATOR OF THE PRESENTATION: the form and the word vertex inside it,
/// the vertex's whole motion and that motion's coordinates, its hue map, and
/// whether a court ruled it (the genome) or the mind stands it (its dialect).
#[derive(Clone, Debug)]
pub struct Generator {
    pub form: Address,
    pub vertex: Address,
    pub motion: Mv,
    pub coords: Coordinates,
    pub gauge: Vec<(u8, u8)>,
    pub ruled: bool,
}

/// THE HELD GENERATORS BY THE HOLES THEY STAND ON and by the modes of their
/// hue maps — keyed by [`super::generator_key`], the same key the lane covers
/// file a generator under, so one generator is one entry whatever reaches it.
/// A language's words, never what a mind has heard: whether a heard ring
/// SOUNDS like something heard is read against the mind's memory of heard
/// rings (MU-CLIENT §4.9), not here.
#[derive(Default)]
pub struct Presentation {
    holes: HoleIndex,
    gauges: Keyed<Keyed<()>>,
    held: Keyed<Generator>,
    /// 78.166 1c — the generators by the CLASS of their turn (`Rotor::class_key`,
    /// a conjugacy invariant): where a motion's class is a held word's, the
    /// motion may be that word transported, and the conjugator is solved
    /// exactly (`conjugate_of`). A generator that turns nothing has no class
    /// here.
    classes: Keyed<Keyed<()>>,
}

/// The class key of a motion's turn: its rotor's conjugacy class, where it
/// has one and turns something.
fn turn_class(motion: &Mv) -> Option<(crate::transducer::alphabet::Rotor, Address)> {
    let f = crate::transducer::element::conformal_factors(motion).ok()?;
    let r = crate::transducer::element::rotor_of_factors(&f).ok()?;
    if r.projective_eq(&crate::transducer::alphabet::Rotor::identity()) {
        return None;
    }
    let class = r.class_key()?;
    Some((r, class))
}

/// THE LANES A WORD IS READ ON (law 5): `S` the boosts and dilations, `G` the
/// hue map, `T` the null slides, `H` the turns — a mode by its kind, never a
/// sum of lanes.
pub const LANE_S: usize = 0;
pub const LANE_G: usize = 1;
pub const LANE_T: usize = 2;
pub const LANE_H: usize = 3;

/// The lane a mode of a motion stands on, by its kind (78.71 F5).
pub fn lane_of(kind: crate::network::nerve::coord::BivectorKind) -> Option<usize> {
    use crate::network::nerve::coord::BivectorKind;
    match kind {
        BivectorKind::Elliptic => Some(LANE_H),
        BivectorKind::Hyperbolic => Some(LANE_S),
        BivectorKind::Parabolic => Some(LANE_T),
        BivectorKind::Trivial => None,
    }
}

/// ONE MODE OF A HUE MAP: a cycle it closes, its hues in the order the map
/// carries them from the least; or one hue carried along a path that does not
/// close (a merge, a hue sent to the void).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GaugeMode {
    Cycle(Vec<u8>),
    Move(u8, u8),
}

impl GaugeMode {
    /// THE MODE'S KEY: a cycle by the hues it stands on, in hue order (its
    /// support, where any power of it stands too); a move by itself.
    fn key(&self) -> Vec<u8> {
        match self {
            GaugeMode::Cycle(hues) => {
                let mut k = vec![1u8];
                let mut support = hues.clone();
                support.sort_unstable();
                k.extend(support);
                k
            }
            GaugeMode::Move(a, b) => vec![0u8, *a, *b],
        }
    }
}

/// THE MODES OF A HUE MAP, each once: every cycle it closes and every move on
/// a path that does not close. `None` where the listing sends a hue two ways —
/// no map (rule 11). Each hue is followed at most around its own cycle, the
/// palette's ten (a boundary, rule 10).
pub fn gauge_modes(g: &[(u8, u8)]) -> Option<Vec<GaugeMode>> {
    let t = super::hue_table(g)?;
    let image = |h: u8| t.get(usize::from(h)).copied().flatten();
    let mut out: Keyed<GaugeMode> = Keyed::new();
    for &(from, to) in g {
        if from == to {
            continue;
        }
        // Around from's orbit under the map, until it returns or leaves.
        let mut cycle = vec![from];
        let mut at = to;
        let mut closes = false;
        while cycle.len() <= t.len() {
            if at == from {
                closes = true;
                break;
            }
            cycle.push(at);
            match image(at) {
                Some(next) if next != at => at = next,
                _ => break,
            }
        }
        let mode = match closes {
            true => {
                let least = cycle.iter().copied().fold(u8::MAX, u8::min);
                let start = cycle.iter().position(|&h| h == least)?;
                cycle.rotate_left(start);
                GaugeMode::Cycle(cycle)
            }
            false => GaugeMode::Move(from, to),
        };
        out.insert(mode.key(), mode);
    }
    Some(out.scoped().map(|(_, m)| m.clone()).collect())
}

/// THE EXPONENT `e` WITH `mode = held^e` ON ONE GAUGE MODE: a cycle on the
/// same hues is a power of another exactly where the target carries the held
/// cycle's least hue `e` steps along it and every hue alike; a move only as
/// itself. Read along the cycle, its own length (a boundary).
fn gauge_exponent(mode: &GaugeMode, held: &GaugeMode) -> Option<i64> {
    match (mode, held) {
        (GaugeMode::Move(a, b), GaugeMode::Move(c, d)) => (a == c && b == d).then_some(1),
        (GaugeMode::Cycle(m), GaugeMode::Cycle(h)) => {
            let n = h.len();
            if m.len() != n || n < 2 {
                return None;
            }
            let pos = |x: u8| h.iter().position(|&y| y == x);
            // Where the target carries h[0], read off its own listing order.
            let i0 = pos(m[0])?;
            let to = m.get(1).copied()?;
            let e = (pos(to)? + n - i0) % n;
            if e == 0 {
                return None;
            }
            let ok = (0..n).all(|i| {
                let from = m[i];
                let next = m[(i + 1) % n];
                pos(from).is_some_and(|p| h[(p + e) % n] == next)
            });
            ok.then(|| i64::try_from(e).ok()).flatten()
        }
        _ => None,
    }
}

/// A HUE MAP RAISED TO `e`: composed with itself `e` times (its inverse for a
/// negative `e`) — at most its own order, the palette's ten.
pub fn gauge_power(g: &[(u8, u8)], e: i64) -> Vec<(u8, u8)> {
    let base = match e < 0 {
        true => super::transport::gauge_invert(g),
        false => g.to_vec(),
    };
    let mut out: Vec<(u8, u8)> = Vec::new();
    for _ in 0..e.unsigned_abs() {
        out = super::transport::compose_gauge(&base, &out);
    }
    out
}

/// 78.161 — A WORD READ OVER WHAT IS HELD, LANE BY LANE: each mode of its
/// whole motion and each mode of its hue map cited to a held generator that
/// stands on it, at an exponent, on the lane the mode belongs to; what no held
/// generator reaches is that lane's residue. It COMPOSES where nothing is left
/// on any lane and the citations' powers multiply back to the word exactly —
/// the motion projectively (1.5), the hue map as a map. Two minds meet where
/// one re-derives the other's word: the same word reached by different
/// mathematics.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Reading {
    /// Each citation: the form, the word vertex in it, the lane, the exponent.
    pub cites: Vec<(Address, Address, usize, i64)>,
    pub residue: [u64; 4],
    pub composes: bool,
    /// 78.163 — WHAT THE READER SAID FOR IT: the product of the generators
    /// it cited, each at its exponent — its own derivation of the motion,
    /// whatever it composes to; the identity where it cited none.
    pub derived: Option<Mv>,
    /// And the hue map its cited generators compose to.
    pub derived_gauge: Vec<(u8, u8)>,
}

/// A MOTION DECONSTRUCTED OVER A PRESENTATION: the generators cited, each
/// `(form, vertex, exponent)`; whether their product IS the motion, exactly
/// (projectively, 1.5); how many of its holes no held generator reached; and
/// how many holes it has.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Deconstructed {
    pub cites: Vec<(Address, Address, i64)>,
    pub composes: bool,
    pub residue: u64,
    pub holes: u64,
}

/// THE WHOLE MOTION OF A WORD VERTEX: its least member's carriage composed
/// with its rotor's base action — the member the vertex's own key order puts
/// first (content order, rule 15). `None` for any other vertex, or one whose
/// rotor has no base action.
pub fn vertex_motion(v: &Vertex) -> Option<Mv> {
    let Vertex::Word { derivation, acts, .. } = v else { return None };
    let (_, a) = acts.scoped().next()?;
    crate::transducer::element::conformal_of_rotor(&derivation.rotor(), &a.carriage)
}

/// THE COORDINATE OF WHAT ONE MEMBER DOES (A21): the chart it acts on, its
/// whole motion's canonical ray (its carriage composed with its rotor's base
/// action) and its hue map, each length-prefixed — the key a held form is
/// filed at and a vertex asks at, so a glue is one keyed read. `None` where
/// the rotor has no base action.
pub fn act_key(
    chart: &crate::transducer::stalk::Invariant,
    rotor: &crate::transducer::alphabet::Rotor,
    carriage: &Mv,
    gauge: &[(u8, u8)],
) -> Option<Vec<u8>> {
    let motion = crate::transducer::element::conformal_of_rotor(rotor, carriage)?;
    let mut k = Vec::new();
    crate::transducer::address::put_key_bytes(&mut k, chart.key().as_bytes());
    crate::transducer::address::put_key_bytes(&mut k, &motion_key(&motion)?);
    let hues: Vec<u8> = gauge.iter().flat_map(|&(a, b)| [a, b]).collect();
    crate::transducer::address::put_key_bytes(&mut k, &hues);
    Some(k)
}

/// Whether a motion is the identity ray.
pub fn is_still(m: &Mv) -> bool {
    m.projective_eq(&Mv::scalar(BigRational::from_integer(1.into())))
}

/// A MOTION'S CANONICAL KEY: its ray written out (A1), the coordinate a
/// mind files what it was answered for it under.
pub fn motion_key(m: &Mv) -> Option<Vec<u8>> {
    let mut k = Vec::new();
    m.canonical()?.put_canonical(&mut k);
    Some(k)
}

/// `mᵉ` FOR AN EXACT INTEGER EXPONENT, as a ray: a negative power is the
/// reverse's (a versor's inverse up to its weight), squared and multiplied
/// over the bits of `|e|` — the carrier's own width, never a cap.
pub fn motion_power(m: &Mv, e: i64) -> Mv {
    let base = match e < 0 {
        true => m.reverse(),
        false => m.clone(),
    };
    let mut r = Mv::scalar(BigRational::from_integer(1.into()));
    let mut square = base;
    let mut rest = e.unsigned_abs();
    while rest > 0 {
        if rest & 1 == 1 {
            r = r.geo(&square);
        }
        rest >>= 1;
        if rest > 0 {
            square = square.geo(&square);
        }
    }
    r
}

impl Presentation {
    /// How many generators it holds.
    pub fn len(&self) -> usize {
        self.held.len()
    }

    pub fn is_empty(&self) -> bool {
        self.held.is_empty()
    }

    /// FILE A FORM'S WORD VERTICES under the holes their motions wind about,
    /// each motion's coordinates read under the stroke. A vertex that moves
    /// nothing has no hole and is not filed; one whose modes do not read on
    /// this stroke is not filed either, and the count says how many were.
    pub fn file_form(&mut self, form: &Address, f: &Form, budget: &mut crate::physics::Budget) -> usize {
        let mut filed = 0usize;
        for (_, v) in f.vertices.scoped() {
            let Some(motion) = vertex_motion(v) else { continue };
            let coords = match is_still(&motion) {
                true => Coordinates { holes: Vec::new() },
                false => match Coordinates::of(&motion, budget) {
                    Ok(c) => c,
                    Err(_) => continue,
                },
            };
            filed += usize::from(self.file_vertex(form, v, motion, coords, true));
        }
        filed
    }

    /// FILE ONE WORD VERTEX under the holes its motion winds about and the
    /// modes of its hue map. A vertex that moves nothing and recolours nothing
    /// has nothing to be read by and is not filed.
    fn file_vertex(&mut self, form: &Address, v: &Vertex, motion: Mv, coords: Coordinates, ruled: bool) -> bool {
        let Vertex::Word { address, acts, .. } = v else { return false };
        let gauge = match acts.scoped().next() {
            Some((_, a)) => a.gauge.clone(),
            None => Vec::new(),
        };
        // A listing that is no map has no modes to be read by (rule 11).
        let Some(modes) = gauge_modes(&gauge) else { return false };
        if coords.holes.is_empty() && modes.is_empty() {
            return false;
        }
        let key = super::generator_key(form, address);
        self.admit(&key, &modes, Generator { form: form.clone(), vertex: address.clone(), motion, coords, gauge, ruled });
        true
    }

    /// ONE GENERATOR ADMITTED at `key`: under its holes and its hue map's
    /// `modes`.
    fn admit(&mut self, key: &[u8], modes: &[GaugeMode], g: Generator) {
        self.holes.file(key, &g.coords);
        for m in modes {
            self.gauges.nested_insert(m.key(), key, ());
        }
        if let Some((_, class)) = turn_class(&g.motion) {
            self.classes.nested_insert(class.as_bytes(), key, ());
        }
        self.held.insert(key, g);
    }

    /// 78.166 1c — A TURN READ AS A HELD WORD TRANSPORTED: a generator whose
    /// turn is in `rotor`'s conjugacy class (by its class key, one keyed
    /// read), and the rotor `g` with `g · W · g̃ = rotor`, solved exactly
    /// (`simultaneous_conjugators`, one elimination over ℚ) and checked by
    /// composing it. Only the ruled where `ruled_only` (what a court can
    /// replay). `None` where no held word of the class is a conjugate of it.
    pub fn conjugate_of(
        &self,
        rotor: &crate::transducer::alphabet::Rotor,
        ruled_only: bool,
    ) -> Option<(Address, Address, crate::transducer::alphabet::Rotor)> {
        let class = rotor.class_key()?;
        let fibre = self.classes.get(class.as_bytes())?;
        for (k, ()) in fibre.scoped() {
            let Some(held) = self.held.get(&k).filter(|g| g.ruled || !ruled_only) else { continue };
            let Some((w, _)) = turn_class(&held.motion) else { continue };
            for g in crate::transducer::alphabet::bundle::simultaneous_conjugators(&[(w.clone(), rotor.clone())]) {
                if g.geo(&w).geo(&g.reverse()).projective_eq(rotor) {
                    return Some((held.form.clone(), held.vertex.clone(), g));
                }
            }
        }
        None
    }

    /// The mirror of [`Presentation::admit`]: the generator at `key` leaves
    /// its holes and its hue modes, and a fibre left empty leaves with it.
    fn retire(&mut self, key: &[u8]) {
        let Some(g) = self.held.remove(key) else { return };
        self.holes.unfile(key, &g.coords);
        let leave = |index: &mut Keyed<Keyed<()>>, at: &[u8]| {
            let empty = match index.get_mut(at) {
                Some(f) => {
                    f.remove(key);
                    f.is_empty()
                }
                None => false,
            };
            if empty {
                index.remove(at);
            }
        };
        // A generator was filed only where its listing is a map.
        for m in gauge_modes(&g.gauge).into_iter().flatten() {
            leave(&mut self.gauges, &m.key());
        }
        if let Some((_, class)) = turn_class(&g.motion) {
            leave(&mut self.classes, class.as_bytes());
        }
    }

    /// The generator held at `key`.
    pub fn generator(&self, key: &[u8]) -> Option<&Generator> {
        self.held.get(key)
    }

    /// 78.160 — A FORM LEARNED into the vocabulary, each word vertex's motion
    /// read by its own modes on a stroke sized by what those modes need
    /// (`cga::modes_need`, 78.111 C1b) — never by a stroke sized for
    /// something else — and filed under its holes and its hue map's modes;
    /// `ruled` where a court ruled the form, else the mind's dialect. What
    /// does not read (no base action, a motion past its carrier) is not
    /// filed.
    pub fn learn_form(&mut self, form: &Address, f: &Form, ruled: bool) -> usize {
        let mut heard = 0usize;
        for (_, v) in f.vertices.scoped() {
            let Some(motion) = vertex_motion(v) else { continue };
            let coords = match is_still(&motion) {
                true => Coordinates { holes: Vec::new() },
                false => {
                    let Some(need) = crate::transducer::cga::modes_need(&motion) else { continue };
                    let mut budget = crate::physics::Physics::default().stroke_n(need);
                    match Coordinates::of(&motion, &mut budget) {
                        Ok(c) => c,
                        Err(_) => continue,
                    }
                }
            };
            heard += usize::from(self.file_vertex(form, v, motion, coords, ruled));
        }
        heard
    }

    /// 78.161 — A LETTER LEARNED into the vocabulary: a word spelled by
    /// letters (a connection, an adjoined letter) is one generator, its motion
    /// its spelling at its carriage (`element::conformal_of`) and its hue map
    /// its sigma, filed at its own address as both form and vertex. What does
    /// not read (no letters, no base action) is not filed.
    pub fn learn_letter(&mut self, addr: &Address, e: &crate::transducer::element::Element, ruled: bool) -> bool {
        let Some(l) = e.letters() else { return false };
        let Some(motion) = crate::transducer::element::conformal_of(&l.spelling.word, &l.carriage) else { return false };
        self.file_motion(addr, motion, l.sigma, ruled)
    }

    /// ONE MOTION AND HUE MAP FILED AS A GENERATOR at its own address, as
    /// both form and vertex: under the holes its modes wind about (read on
    /// the stroke they need) and the modes of its hue map. `false` where it
    /// does not read, or moves nothing on either.
    fn file_motion(&mut self, addr: &Address, motion: Mv, gauge: Vec<(u8, u8)>, ruled: bool) -> bool {
        let coords = match is_still(&motion) {
            true => Coordinates { holes: Vec::new() },
            false => {
                let Some(need) = crate::transducer::cga::modes_need(&motion) else { return false };
                let mut budget = crate::physics::Physics::default().stroke_n(need);
                match Coordinates::of(&motion, &mut budget) {
                    Ok(c) => c,
                    Err(_) => return false,
                }
            }
        };
        let Some(modes) = gauge_modes(&gauge) else { return false };
        if coords.holes.is_empty() && modes.is_empty() {
            return false;
        }
        let key = super::generator_key(addr, addr);
        self.admit(&key, &modes, Generator { form: addr.clone(), vertex: addr.clone(), motion, coords, gauge, ruled });
        true
    }

    /// The mirror of [`Presentation::learn_letter`].
    pub fn unfile_letter(&mut self, addr: &Address) {
        self.retire(&super::generator_key(addr, addr));
    }

    /// 78.161 — READ A WORD OVER WHAT IS HELD, LANE BY LANE ([`Reading`]): its
    /// whole motion hole by hole (each hole on the lane its kind names), its
    /// hue map mode by mode (on `G`), over every generator held, or only the
    /// ruled ones where `ruled_only` (what a court can replay). The motion's
    /// modes are read on the stroke they need. `None` where the motion's modes
    /// do not read, or its hue listing is no map (rule 11).
    pub fn read_word(&self, motion: &Mv, gauge: &[(u8, u8)], ruled_only: bool) -> Option<Reading> {
        let eligible = |g: &Generator| g.ruled || !ruled_only;
        let mut out = Reading::default();
        let one = || Mv::scalar(BigRational::from_integer(1.into()));
        // THE MOTION, hole by hole.
        let mut motion_ok = true;
        if !is_still(motion) {
            let need = crate::transducer::cga::modes_need(motion)?;
            let mut budget = crate::physics::Physics::default().stroke_n(need);
            let coords = Coordinates::of(motion, &mut budget).ok()?;
            let d = self.holes.deconstruct(&coords, |k| self.held.get(k).filter(|g| eligible(g)).map(|g| &g.coords));
            let lane_at = |hole: &[u8]| coords.holes.iter().find(|h| h.key == hole).and_then(|h| lane_of(h.kind));
            for h in &d.residue {
                if let Some(l) = lane_of(h.kind) {
                    out.residue[l] += 1;
                }
            }
            // One citation per held generator at one exponent; the same
            // generator at two exponents on two holes is no one power of it.
            let mut cited: Vec<(Vec<u8>, i64)> = Vec::new();
            for c in &d.cites {
                match cited.iter().find(|(w, _)| *w == c.word) {
                    Some((_, e)) if *e == c.exponent => continue,
                    Some(_) => motion_ok = false,
                    None => cited.push((c.word.clone(), c.exponent)),
                }
                let (Some(g), Some(l)) = (self.held.get(&c.word), lane_at(&c.hole)) else { continue };
                out.cites.push((g.form.clone(), g.vertex.clone(), l, c.exponent));
            }
            let composed = cited
                .iter()
                .filter_map(|(k, e)| self.held.get(k).map(|g| motion_power(&g.motion, *e)))
                .fold(one(), |acc, p| acc.geo(&p));
            motion_ok = motion_ok && composed.projective_eq(motion);
            out.derived = Some(composed);
        } else {
            out.derived = Some(one());
        }
        // THE HUE MAP, mode by mode.
        let modes = gauge_modes(gauge)?;
        let mut composed_gauge: Vec<(u8, u8)> = Vec::new();
        let mut gauge_cited: Vec<Vec<u8>> = Vec::new();
        for m in &modes {
            let hit = self.gauges.get(&m.key()).and_then(|fibre| {
                fibre.scoped().find_map(|(k, ())| {
                    let g = self.held.get(&k).filter(|g| eligible(g))?;
                    let held_modes = gauge_modes(&g.gauge)?;
                    let e = held_modes.iter().find(|h| h.key() == m.key()).and_then(|h| gauge_exponent(m, h))?;
                    Some((k, g, e))
                })
            });
            match hit {
                Some((k, g, e)) => {
                    if !gauge_cited.contains(&k) {
                        composed_gauge = super::transport::compose_gauge(&gauge_power(&g.gauge, e), &composed_gauge);
                        gauge_cited.push(k);
                    }
                    out.cites.push((g.form.clone(), g.vertex.clone(), LANE_G, e));
                }
                None => out.residue[LANE_G] += 1,
            }
        }
        let gauge_ok = super::transport::gauge_eq(&composed_gauge, gauge);
        out.derived_gauge = composed_gauge;
        out.composes = out.residue.iter().all(|&r| r == 0) && !out.cites.is_empty() && motion_ok && gauge_ok;
        Some(out)
    }

    /// [`Presentation::deconstruct`] on the stroke the motion's own modes
    /// need — what the mind reads a heard motion against.
    pub fn deconstruct_sized(&self, motion: &Mv) -> Option<Deconstructed> {
        let need = crate::transducer::cga::modes_need(motion)?;
        let mut budget = crate::physics::Physics::default().stroke_n(need);
        self.deconstruct(motion, &mut budget).ok()
    }

    /// The mirror of [`Presentation::file_form`], vertex for vertex.
    pub fn unfile_form(&mut self, form: &Address, f: &Form) {
        for (_, v) in f.vertices.scoped() {
            let Vertex::Word { address, .. } = v else { continue };
            self.retire(&super::generator_key(form, address));
        }
    }

    /// 78.100 D3 — A MOTION DECONSTRUCTED OVER WHAT IS HELD, hole by hole:
    /// each hole cited to a held generator standing on it whose winding
    /// reaches the motion's by an exponent; one generator covering several
    /// holes at one exponent is one citation. It COMPOSES where every hole is
    /// cited and the citations' powers multiply, in the order of the motion's
    /// holes, to the motion itself — exact, or it does not. `Err` by name
    /// where the motion's own coordinates do not read.
    pub fn deconstruct(&self, motion: &Mv, budget: &mut crate::physics::Budget) -> Result<Deconstructed, Unread> {
        let coords = Coordinates::of(motion, budget)?;
        let d = self.holes.deconstruct(&coords, |k| self.held.get(k).map(|g| &g.coords));
        let mut cites: Vec<(Vec<u8>, i64)> = Vec::new();
        for c in &d.cites {
            match cites.iter().find(|(w, _)| *w == c.word) {
                Some((_, e)) if *e == c.exponent => {}
                _ => cites.push((c.word.clone(), c.exponent)),
            }
        }
        let mut composed = Mv::scalar(BigRational::from_integer(1.into()));
        let mut out = Vec::with_capacity(cites.len());
        for (k, e) in &cites {
            let Some(g) = self.held.get(k) else { continue };
            composed = composed.geo(&motion_power(&g.motion, *e));
            out.push((g.form.clone(), g.vertex.clone(), *e));
        }
        let composes = d.residue.is_empty() && !out.is_empty() && composed.projective_eq(motion);
        Ok(Deconstructed {
            cites: out,
            composes,
            residue: d.residue.len() as u64,
            holes: coords.holes.len() as u64,
        })
    }
}
