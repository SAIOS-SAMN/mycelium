// 78.74 (as amended) — THE FIBRES OF A FRAME, AND THE QUOTIENT BY TORSION:
// every pair of pieces a frame carries is a fibre between their pivots;
// torsion — the part of a relative motion that keeps both pivots and every
// level — is ELIMINATED, and what survives is the signal, read as graded,
// exact certainty, never as a verdict at a pole.
//
//! A PIECE is a pivot: the centroid of the sites it was carried FROM, in the
//! vertex coordinates its own versor acts in; its mass the count of those
//! sites; its transport its own versor. The pieces are the cover's carried
//! pieces — never a chart's union body, which is a first moment of many and
//! hid every fibre between them (2026-10-03: 154 fibres, all rigid).
//!
//! TWO TRANSPORT CLASSES ARE ONE ALGORITHM UP TO TORSION where, on the
//! fibres joining their pieces, the relative motion `R = ṽₐ v_b` keeps both
//! pivots and every level. The certainty of that is counted, never measured
//! (the metric is indefinite, so no part of a motion has a size): the
//! fibres that said torsion over the fibres observed, `t / n`, exact. Only
//! what is certain is eliminated: classes whose every observed fibre said
//! torsion are merged; a partial certainty is reported as the signal it is.
//!
//! THE DRAW. A merged group is one algorithm, so its turns are charged once:
//! the group's least class key — the canonical order the form already
//! chooses its derivations by — carries the charge, and every other member's
//! turns, torsion beyond it, are carried free. A class alone is never free:
//! its turn IS its algorithm (the free-frame rule of 72e6d621 exempted a
//! whole scene's turn, which is the rule itself; withdrawn).
//!
//! BOUNDED BY THE STROKE AND BY CLASS. Pieces of one (class, carriage) are
//! carried rigidly against each other and are counted, not multiplied out;
//! across two of them `R` is formed once and each pivot is asked once
//! whether `R` keeps it; only where both pivots are kept is the fibre built
//! and its levels asked. Each cross pair spends the stroke; a stroke that
//! ends leaves the rest unread, by count.

use crate::physics::capacitor::{shape_of, Carried, Fibre, Shape};
use crate::transducer::base::{is_grid_axis, Site};
use crate::transducer::cga::{self, Mv};
use crate::transducer::element::Versor;
use num_rational::BigRational;
use num_traits::Zero;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

/// One piece a frame carries.
#[derive(Clone, Debug)]
pub(crate) struct Piece {
    pub frame: u64,
    /// The transport class it belongs to, by its encoded coordinate.
    pub class: Vec<u8>,
    pub point: Mv,
    /// Where it landed: the centroid of the sites it was carried TO.
    pub out: Option<Mv>,
    pub mass: u64,
    /// The compartments it landed as.
    pub out_mass: u64,
    pub carriage: Versor,
    pub n: usize,
}

/// 78.74 — HOW TWO SECTIONS OF ONE CLASS RELATE, where a class carries a
/// second piece on one chart by a second carriage (the multiplicity).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sections {
    /// Their geoids overlay — equal weighed length and masses: a copy.
    Congruent,
    /// Their carriages are one motion up to torsion about the fibre joining
    /// their sources.
    Coherent,
    /// Neither: a genuine second algorithm; its demand stands.
    Standing,
}

/// A section's geoid: the switch of the fibre from where it was to where it
/// landed, weighed by its compartments at each end. `None` for a section
/// carried in place (one pivot, no fibre) or landing nowhere readable.
fn geoid(p: &Piece) -> Option<Mv> {
    let fibre = Fibre::new(
        p.point.clone(),
        p.out.clone()?,
        BigRational::from_integer(p.mass.into()),
        BigRational::from_integer(p.out_mass.into()),
    )?;
    Some(fibre.switch())
}

/// HOW TWO SECTIONS RELATE: congruent where their geoids overlay
/// (`S₁² = S₂²`, `S² = m_P m_Q d²`, with equal masses); else coherent where
/// the relative motion `ṽ₁ v₂` keeps both sources and every level of the
/// fibre joining them; else standing. Exact.
pub(crate) fn relate_sections(first: &Piece, second: &Piece) -> Sections {
    if let (Some(g1), Some(g2)) = (geoid(first), geoid(second)) {
        let same_masses = first.mass == second.mass && first.out_mass == second.out_mass;
        if same_masses && g1.geo(&g1).scalar_part() == g2.geo(&g2).scalar_part() {
            return Sections::Congruent;
        }
    }
    let joining = Fibre::new(
        first.point.clone(),
        second.point.clone(),
        BigRational::from_integer(first.mass.into()),
        BigRational::from_integer(second.mass.into()),
    );
    match joining.map(|f| f.carried_by(&first.carriage.reverse().geo(&second.carriage))) {
        Some(Carried::Rigid | Carried::Torsion) => Sections::Coherent,
        _ => Sections::Standing,
    }
}

/// One frame-axis step of a class: the frames, the motor its input body
/// took, and that body's pivot at each end.
pub(crate) struct Step {
    pub from: u64,
    pub to: u64,
    pub motor: Versor,
    pub at_from: Option<Mv>,
    pub at_to: Option<Mv>,
}

/// 78.74 — COHESION, THE GAUGE OF A CLASS'S COHERENCE STRUCTURE: along its
/// frame axis, each step's motor is a coherence vector, and the change
/// between two chained steps, `D = s̃ t`, is smooth where it is rigid or
/// torsion about the fibre the second step spans — the class relates its
/// frames the same way there — and a bend where it is field or lift. The
/// gauge is smooth changes over changes, exact (counted, never a norm).
/// Returns (smooth, changes, unread).
pub(crate) fn cohesion(class: &[u8], steps: &[Step]) -> (u64, u64, u64) {
    let (mut smooth, mut changes, mut unread) = (0u64, 0u64, 0u64);
    for pair in steps.windows(2) {
        let [s, t] = pair else { continue };
        // Only chained steps make a change: a gap is no neighbour.
        if s.to != t.from {
            continue;
        }
        let d = s.motor.reverse().geo(&t.motor);
        let one = BigRational::from_integer(1.into());
        let carried = match (&t.at_from, &t.at_to) {
            _ if d.projective_eq(&Mv::scalar(one.clone())) => Some(Carried::Rigid),
            (Some(p), Some(q)) => Fibre::new(p.clone(), q.clone(), one.clone(), one.clone()).map(|f| f.carried_by(&d)),
            _ => None,
        };
        match carried {
            Some(Carried::Rigid | Carried::Torsion) => {
                step(&mut smooth, 1);
                step(&mut changes, 1);
            }
            Some(Carried::Field | Carried::Lift) => step(&mut changes, 1),
            None => step(&mut unread, 1),
        }
    }
    note(COH_SMOOTH, smooth);
    note(COH_CHANGES, changes);
    note(COH_UNREAD, unread);
    if changes > 0 || unread > 0 {
        PERCEPT.with(|p| p.borrow_mut().cohesion.push((class.to_vec(), smooth, changes)));
    }
    (smooth, changes, unread)
}

/// 78.74 — a multiplicity's two sections, related and counted; the demand
/// stands only where they are `Standing`.
pub(crate) fn note_section(relation: Sections) {
    note(
        match relation {
            Sections::Congruent => SEC_CONGRUENT,
            Sections::Coherent => SEC_COHERENT,
            Sections::Standing => SEC_STANDING,
        },
        1,
    );
    PERCEPT.with(|p| p.borrow_mut().sections.push(relation));
}

/// What one percept's sections and cohesion read, gathered on its thread
/// until its fibres are read (`read_pieces`), then handed to its quotient.
#[derive(Default)]
struct PerceptReading {
    sections: Vec<Sections>,
    cohesion: Vec<(Vec<u8>, u64, u64)>,
}

thread_local! {
    static PERCEPT: std::cell::RefCell<PerceptReading> = std::cell::RefCell::new(PerceptReading::default());
}

/// A percept begins: what an earlier one left unread is let go.
pub(crate) fn begin() {
    PERCEPT.with(|p| *p.borrow_mut() = PerceptReading::default());
}

/// THE CENTROID OF SITES IN VERTEX COORDINATES, as a conformal point: each
/// axis's mean, and one half on each grid axis (a compartment's square spans
/// it; a frame coordinate is the offset alone) — `carry::vertex_centroid`'s
/// reading of a lifted body, read off the sites themselves. `None` for no
/// sites or sites of two bases.
pub(crate) fn vertex_point(sites: &[Site]) -> Option<(Mv, usize)> {
    let (first, rest) = sites.split_first()?;
    let n = first.n();
    if rest.iter().any(|s| s.n() != n) {
        return None;
    }
    let count = BigRational::from_integer(i64::try_from(sites.len()).ok()?.into());
    let half = BigRational::new(1.into(), 2.into());
    let centroid: Vec<BigRational> = (0..n)
        .map(|i| {
            let sum: BigRational = sites.iter().filter_map(|s| s.axis(i)).map(|a| BigRational::from_integer(a.into())).sum();
            let mean = sum / &count;
            match is_grid_axis(i) {
                true => mean + &half,
                false => mean,
            }
        })
        .collect();
    Some((cga::point_at(&centroid), n))
}

/// A piece from what its cover carried. `None` where its input sites give no
/// pivot.
pub(crate) fn piece_of(frame: u64, class: Vec<u8>, carriage: &Versor, domain: &[Site], codomain: &[Site]) -> Option<Piece> {
    let (point, n) = vertex_point(domain)?;
    let out = vertex_point(codomain).map(|(p, _)| p);
    let mass = u64::try_from(domain.len()).ok()?;
    let out_mass = u64::try_from(codomain.len()).ok()?;
    Some(Piece { frame, class, point, out, mass, out_mass, carriage: carriage.clone(), n })
}

/// 78.74 — WHAT THE FRAMES' FIBRES DID, in this process.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FibreTally {
    pub pairs: u64,
    pub rigid: u64,
    pub torsion: u64,
    pub field: u64,
    pub lift: u64,
    /// Two pieces on one centroid: no fibre.
    pub unpivoted: u64,
    /// Cross pairs the stroke did not reach.
    pub unread: u64,
    pub stable: u64,
    /// Exactly enough pivots to fix a round, and none left to test it.
    pub determined: u64,
    pub emergent: u64,
    pub open: u64,
    /// The self-check: a piece's versor carries its input pivot onto its
    /// output pivot, or does not.
    pub aligned: u64,
    pub misaligned: u64,
    /// Class pairs observed; those certain (every fibre torsion); those
    /// partly so.
    pub class_pairs: u64,
    pub certain: u64,
    pub partial: u64,
    /// Classes before the quotient by torsion, and after.
    pub classes: u64,
    pub algorithms: u64,
    /// Merges refused because a pair across the two groups disagreed.
    pub conflicted: u64,
    /// Multiplicities by how their two sections related: eliminated as a
    /// copy, eliminated as one motion up to torsion, or standing.
    pub sections_congruent: u64,
    pub sections_coherent: u64,
    pub sections_standing: u64,
    /// Cohesion: smooth changes along classes' frame axes, all changes, and
    /// changes with no fibre to read them on.
    pub smooth: u64,
    pub changes: u64,
    pub changes_unread: u64,
}

impl FibreTally {
    /// THE COHESION GAUGE: smooth changes over changes, exact; `None` before
    /// any change was read.
    pub fn cohesion(&self) -> Option<BigRational> {
        (self.changes > 0).then(|| BigRational::new(self.smooth.into(), self.changes.into()))
    }
}

impl FibreTally {
    /// THE TORSION ELIMINATED: `1 − algorithms / classes`, exact, or `None`
    /// before any class was read.
    pub fn eliminated(&self) -> Option<BigRational> {
        (self.classes > 0).then(|| {
            BigRational::from_integer(1.into()) - BigRational::new(self.algorithms.into(), self.classes.into())
        })
    }
}

const SLOTS: usize = 25;
static FIBRES: [AtomicU64; SLOTS] = [const { AtomicU64::new(0) }; SLOTS];
const PAIRS: usize = 0;
const RIGID: usize = 1;
const TORSION: usize = 2;
const FIELD: usize = 3;
const LIFT: usize = 4;
const UNPIVOTED: usize = 5;
const UNREAD: usize = 6;
const STABLE: usize = 7;
const DETERMINED: usize = 8;
const EMERGENT: usize = 9;
const OPEN: usize = 10;
const ALIGNED: usize = 11;
const MISALIGNED: usize = 12;
const CLASS_PAIRS: usize = 13;
const CERTAIN: usize = 14;
const PARTIAL: usize = 15;
const CLASSES: usize = 16;
const ALGORITHMS: usize = 17;
const CONFLICTED: usize = 18;
const SEC_CONGRUENT: usize = 19;
const SEC_COHERENT: usize = 20;
const SEC_STANDING: usize = 21;
const COH_SMOOTH: usize = 22;
const COH_CHANGES: usize = 23;
const COH_UNREAD: usize = 24;

fn note(slot: usize, by: u64) {
    if by > 0 && FIBRES[slot].fetch_update(Relaxed, Relaxed, |held| held.checked_add(by)).is_err() {
        let mut past = u64::MAX;
        let _ = crate::physics::metabolism::step_or_refuse(&mut past, by);
    }
}

/// The fibres' tally since the process began.
pub fn fibre_tally() -> FibreTally {
    let r = |i: usize| FIBRES[i].load(Relaxed);
    FibreTally {
        pairs: r(PAIRS),
        rigid: r(RIGID),
        torsion: r(TORSION),
        field: r(FIELD),
        lift: r(LIFT),
        unpivoted: r(UNPIVOTED),
        unread: r(UNREAD),
        stable: r(STABLE),
        determined: r(DETERMINED),
        emergent: r(EMERGENT),
        open: r(OPEN),
        aligned: r(ALIGNED),
        misaligned: r(MISALIGNED),
        class_pairs: r(CLASS_PAIRS),
        certain: r(CERTAIN),
        partial: r(PARTIAL),
        classes: r(CLASSES),
        algorithms: r(ALGORITHMS),
        conflicted: r(CONFLICTED),
        sections_congruent: r(SEC_CONGRUENT),
        sections_coherent: r(SEC_COHERENT),
        sections_standing: r(SEC_STANDING),
        smooth: r(COH_SMOOTH),
        changes: r(COH_CHANGES),
        changes_unread: r(COH_UNREAD),
    }
}

/// k choose 2, or `None` past the carrier.
fn pairs_in(k: u64) -> Option<u64> {
    k.checked_mul(k.checked_sub(1)?)?.checked_div(2)
}

/// One class pair's fibres, by what they said: observed, and how many AGREED
/// — carried the pair one into the other up to torsion. The identity is in
/// the torsion group, so a fibre carried rigidly agrees as a twisted one
/// does: two classes moving alike are one algorithm.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Said {
    pub observed: u64,
    pub agree: u64,
    /// 78.74 — the same fibres WEIGHED BY THEIR BINDING (`m m / d²`, exact):
    /// what agreed and what was observed, so a pair joined by strong tethers
    /// is read apart from one joined by weak ones.
    pub bind_agree: BigRational,
    pub bind_observed: BigRational,
}

impl Said {
    /// THE CERTAINTY that the pair is one algorithm up to torsion: the
    /// fibres that agreed over those observed, exact; `None` before any was
    /// observed.
    pub fn certainty(&self) -> Option<BigRational> {
        (self.observed > 0).then(|| BigRational::new(self.agree.into(), self.observed.into()))
    }

    fn certain(&self) -> bool {
        self.observed > 0 && self.agree == self.observed
    }

    /// 78.74 — THE BINDING CERTAINTY: agreeing binding over observed
    /// binding, exact; `None` before any was observed.
    pub fn binding_certainty(&self) -> Option<BigRational> {
        (!self.bind_observed.is_zero()).then(|| &self.bind_agree / &self.bind_observed)
    }

    fn absorb(&mut self, other: &Said) {
        step(&mut self.observed, other.observed);
        step(&mut self.agree, other.agree);
        self.bind_agree += &other.bind_agree;
        self.bind_observed += &other.bind_observed;
    }
}

/// What the fibres between two classes have said in this process, since
/// the pair first agreed; `None` for a pair never tracked.
pub fn moving_said(a: &[u8], b: &[u8]) -> Option<Said> {
    let (a, b) = match a <= b {
        true => (a, b),
        false => (b, a),
    };
    MOVING.lock().ok()?.get(&pair_key(a, b)?).cloned()
}

/// THE MOVING ALGORITHM, ACROSS PERCEPTS: per class pair, its fibres'
/// agreement over observed, held for this process — so a pair's certainty
/// rises as frames agree and falls as the algorithm moves. A pair enters
/// once it has agreed and is counted from then on: the sink holds the pairs
/// that carry the signal, bounded by the agreement seen, never every pair
/// two pieces ever made.
static MOVING: std::sync::LazyLock<std::sync::Mutex<crate::network::nerve::Keyed<Said>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(crate::network::nerve::Keyed::new()));

fn pair_key(a: &[u8], b: &[u8]) -> Option<Vec<u8>> {
    let mut key = Vec::with_capacity(a.len() + b.len() + 2);
    key.extend_from_slice(&u16::try_from(a.len()).ok()?.to_le_bytes());
    key.extend_from_slice(a);
    key.extend_from_slice(b);
    Some(key)
}

/// THE MOVING ALGORITHM'S READING: pairs tracked, how many are certain (every
/// fibre since they first agreed) and how many partial, and their fibres'
/// agreement over observed summed — exact.
pub fn moving_reading() -> Option<(usize, u64, u64, Said)> {
    let held = MOVING.lock().ok()?;
    let mut sum = Said::default();
    let (mut certain, mut partial) = (0u64, 0u64);
    for (_, s) in held.scoped() {
        sum.absorb(s);
        match s.certain() {
            true => step(&mut certain, 1),
            false => step(&mut partial, 1),
        }
    }
    Some((held.len(), certain, partial, sum))
}

/// A class pair, its two keys in order, with what its fibres said.
pub type PairSaid = ((Vec<u8>, Vec<u8>), Said);

fn track(said: &[PairSaid]) {
    let Ok(mut held) = MOVING.lock() else { return };
    for ((a, b), s) in said {
        let Some(key) = pair_key(a, b) else { continue };
        match held.get_mut(&key) {
            Some(h) => h.absorb(s),
            None if s.agree > 0 => {
                held.insert(&key, s.clone());
            }
            None => {}
        }
    }
}

/// WHAT THE QUOTIENT READ: per class pair (ordered by key) its fibres'
/// torsion over observed; the classes whose turns are torsion beyond their
/// group's charged member; and the groups themselves.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Quotient {
    pub said: Vec<PairSaid>,
    pub free: Vec<Vec<u8>>,
    pub groups: Vec<Vec<Vec<u8>>>,
    /// This read's pieces, and the self-check on their coordinates.
    pub pieces: u64,
    pub aligned: u64,
    pub misaligned: u64,
    /// This percept's multiplicities by how their sections related.
    pub sections: Vec<Sections>,
    /// Per class: smooth changes and changes along its frame axis.
    pub cohesion: Vec<(Vec<u8>, u64, u64)>,
}

thread_local! {
    /// The last quotient this thread read — at most one percept's, replaced
    /// by the next — so a reader can ask what one extraction found.
    static LAST: std::cell::RefCell<Option<Quotient>> = const { std::cell::RefCell::new(None) };
}

/// The quotient this thread's last percept read, taken — what one
/// extraction found, for the reader that ran it.
pub fn take_last() -> Option<Quotient> {
    LAST.with(|l| l.borrow_mut().take())
}

fn step(n: &mut u64, by: u64) {
    let _ = crate::physics::metabolism::step_or_refuse(n, by);
}

/// THE SELF-CHECK ON A PIECE'S COORDINATES: does its versor carry its input
/// pivot onto its output pivot? `None` where it landed nowhere readable.
pub(crate) fn aligned(p: &Piece) -> Option<bool> {
    Some(p.carriage.sandwich(&p.point).projective_eq(p.out.as_ref()?))
}

/// READ EVERY FRAME'S FIBRES AND TAKE THE QUOTIENT BY TORSION.
pub(crate) fn read_pieces(pieces: &[Piece], budget: &mut crate::physics::Budget) -> Quotient {
    // The self-check on the coordinates: each piece's versor carries its
    // input pivot onto its output pivot.
    let (mut on, mut off) = (0u64, 0u64);
    for p in pieces {
        match aligned(p) {
            Some(true) => step(&mut on, 1),
            Some(false) => step(&mut off, 1),
            None => {}
        }
    }
    note(ALIGNED, on);
    note(MISALIGNED, off);
    let mut said: Vec<PairSaid> = Vec::new();
    let mut frames: Vec<u64> = pieces.iter().map(|p| p.frame).collect();
    frames.sort_unstable();
    frames.dedup();
    for k in frames {
        let here: Vec<&Piece> = pieces.iter().filter(|p| p.frame == k).collect();
        // The frame's pieces stand on one base: its `n` is the one they
        // share, never chosen among them (rule 4).
        let shared = match here.split_first() {
            Some((first, rest)) => rest.iter().all(|p| p.n == first.n).then_some(first.n),
            None => continue,
        };
        let points: Vec<Mv> = here.iter().map(|p| p.point.clone()).collect();
        note(
            match shared.map(|n| shape_of(&points, n)) {
                Some(Shape::Stable(_)) => STABLE,
                Some(Shape::Determined(_)) => DETERMINED,
                Some(Shape::Emergent) => EMERGENT,
                Some(Shape::Open) | None => OPEN,
            },
            1,
        );
        // The frame's transports: each (class, carriage) with its pieces.
        let mut groups: Vec<(&[u8], &Versor, Vec<&Piece>)> = Vec::new();
        for p in &here {
            match groups.iter_mut().find(|(c, v, _)| *c == p.class.as_slice() && v.projective_eq(&p.carriage)) {
                Some((_, _, members)) => members.push(p),
                None => groups.push((&p.class, &p.carriage, vec![p])),
            }
        }
        for (_, _, members) in &groups {
            if let Some(n) = pairs_in(members.len() as u64) {
                note(PAIRS, n);
                note(RIGID, n);
            }
        }
        for (a, (ca, va, ma)) in groups.iter().enumerate() {
            for (cb, vb, mb) in groups.iter().skip(a + 1) {
                let r = va.reverse().geo(vb);
                // Each pivot asked once whether `R` keeps it.
                let kept_a: Vec<bool> = ma.iter().map(|p| r.sandwich(&p.point).projective_eq(&p.point)).collect();
                let kept_b: Vec<bool> = mb.iter().map(|p| r.sandwich(&p.point).projective_eq(&p.point)).collect();
                let key = match ca <= cb {
                    true => (ca.to_vec(), cb.to_vec()),
                    false => (cb.to_vec(), ca.to_vec()),
                };
                let mut s = Said::default();
                for (pi, ki) in ma.iter().zip(&kept_a) {
                    for (pj, kj) in mb.iter().zip(&kept_b) {
                        if !budget.spend() {
                            note(UNREAD, 1);
                            continue;
                        }
                        note(PAIRS, 1);
                        // The tether between the two pieces, measured: one
                        // pivot (two pieces on one centroid) is no tether.
                        let fibre = Fibre::new(
                            pi.point.clone(),
                            pj.point.clone(),
                            BigRational::from_integer(pi.mass.into()),
                            BigRational::from_integer(pj.mass.into()),
                        );
                        let Some(fibre) = fibre else {
                            note(UNPIVOTED, 1);
                            continue;
                        };
                        let carried = match (ki, kj) {
                            (true, true) => fibre.carried_by(&r),
                            _ => Carried::Lift,
                        };
                        let binding = fibre.strength();
                        step(&mut s.observed, 1);
                        s.bind_observed += &binding;
                        // Rigid and torsion agree: both keep the fibre's
                        // pivots and levels (the identity is in the torsion
                        // group). Field and lift are work: they disagree.
                        match carried {
                            Carried::Rigid => {
                                note(RIGID, 1);
                                step(&mut s.agree, 1);
                                s.bind_agree += &binding;
                            }
                            Carried::Torsion => {
                                note(TORSION, 1);
                                step(&mut s.agree, 1);
                                s.bind_agree += &binding;
                            }
                            Carried::Field => note(FIELD, 1),
                            Carried::Lift => note(LIFT, 1),
                        }
                    }
                }
                // Two members of one class with two carriages are the
                // class's own sections, not two algorithms: they are not a
                // class pair.
                if ca == cb || s.observed == 0 {
                    continue;
                }
                match said.iter_mut().find(|(k, _)| *k == key) {
                    Some((_, held)) => held.absorb(&s),
                    None => said.push((key, s)),
                }
            }
        }
    }
    track(&said);
    let mut q = quotient(pieces, said);
    q.pieces = pieces.len() as u64;
    q.aligned = on;
    q.misaligned = off;
    let gathered = PERCEPT.with(|p| std::mem::take(&mut *p.borrow_mut()));
    q.sections = gathered.sections;
    q.cohesion = gathered.cohesion;
    let kept = q.clone();
    LAST.with(|l| *l.borrow_mut() = Some(kept));
    q
}

/// THE QUOTIENT: classes merged where every fibre between them agreed; each
/// group's least key charged, the rest free.
///
/// NEVER BLINDLY TRANSITIVE. Torsion is relative to each fibre, so `a ≡ b`
/// and `b ≡ d` can hold while the fibres between `a` and `d` disagree. Two
/// groups merge only where every pair across them that was observed is
/// certain; a merge a disagreeing pair would cross is refused and counted
/// (`conflicted`).
///
/// THE STRONGER HOLDS FIRST (78.74). The certain pairs merge in the order
/// of their binding — the summed strength `m m / d²` of the fibres that
/// joined them — so where two merges conflict, the pair bound harder wins.
/// They were taken in the order the frames happened to give them, which
/// decided a conflict by an artifact of reading order.
fn quotient(pieces: &[Piece], mut said: Vec<PairSaid>) -> Quotient {
    #[expect(clippy::disallowed_methods, reason = "rule 4 — an order by the pairs' own binding, exact: the stronger tether holds first")]
    said.sort_by(|(_, x), (_, y)| y.bind_agree.cmp(&x.bind_agree));
    let mut classes: Vec<Vec<u8>> = pieces.iter().map(|p| p.class.clone()).collect();
    classes.sort_unstable();
    classes.dedup();
    let observed = |x: &[u8], y: &[u8]| -> Option<&Said> {
        let key = match x <= y {
            true => (x.to_vec(), y.to_vec()),
            false => (y.to_vec(), x.to_vec()),
        };
        said.iter().find(|(k, _)| *k == key).map(|(_, s)| s)
    };
    let mut groups: Vec<Vec<Vec<u8>>> = classes.iter().map(|c| vec![c.clone()]).collect();
    for ((a, b), s) in &said {
        note(CLASS_PAIRS, 1);
        match (s.certain(), s.agree > 0) {
            (true, _) => note(CERTAIN, 1),
            (false, true) => {
                note(PARTIAL, 1);
                continue;
            }
            (false, false) => continue,
        }
        let ga = groups.iter().position(|g| g.contains(a));
        let gb = groups.iter().position(|g| g.contains(b));
        let (Some(ga), Some(gb)) = (ga, gb) else { continue };
        let (low, high) = match ga < gb {
            true => (ga, gb),
            false => (gb, ga),
        };
        if low == high {
            continue;
        }
        let consistent = groups[low]
            .iter()
            .all(|x| groups[high].iter().all(|y| observed(x, y).is_none_or(Said::certain)));
        if !consistent {
            note(CONFLICTED, 1);
            continue;
        }
        let moved = groups.remove(high);
        groups[low].extend(moved);
    }
    for g in &mut groups {
        g.sort_unstable();
    }
    note(CLASSES, classes.len() as u64);
    note(ALGORITHMS, groups.len() as u64);
    let free = groups.iter().filter(|g| g.len() > 1).flat_map(|g| g.iter().skip(1).cloned()).collect();
    Quotient { said, free, groups, ..Quotient::default() }
}
