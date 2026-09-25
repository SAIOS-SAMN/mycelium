// 78.7 as amended at 0f7fb7c4, VS1 — THE VISCOSITY OF A PAIR OF PLANES: a
// face of a held loop bent by the exact pivot on two planes, the loop of the
// two bends re-read, and the holonomy's class after set against the class
// the same loop gives a flat form — per pair of planes and per lane, never
// summed.
//
//! THE BEND is a pivot move on one face, parameter-free and exact: on an
//! ELLIPTIC plane its quarter turn `1 + e_ij`, on a HYPERBOLIC plane its blade
//! (its half turn) — no boost with a parameter enters (rule 5). On the
//! spatial lane `𝒮` it is a `Placement` on the face's carriage over the
//! algebra's planes; on the holonomic lane `𝒽` a `Word` move, the
//! quarter-turn letter on the six rotor planes. The gauge `𝒢` and the
//! temporal `𝒯` lanes carry no generator on a pair of planes: NO BEND ON
//! THIS LANE, by name, never zero.
//!
//! THE READING. A single bend and its exact inverse restore the class —
//! apply then unapply writes the face back, and the circuit read is a function of
//! the faces — so path dependence lives on the LOOP of two bends, `A, B,
//! A⁻¹, B⁻¹`, which multiplies the face by `K = B⁻¹A⁻¹BA`. The algebra's own
//! part is subtracted by reading `K` alone, the loop on a flat form. Four
//! arms per pair and lane (78.7 as amended):
//!
//! - INVISCID: neither bend moves the circuit's class — both are gauge.
//! - ELASTIC: a bend moves the class, and the loop reads what the flat form
//!   reads (`K`'s own class) or restores the form's own class.
//! - CURVED: the loop reads a class neither — the form's own curvature felt
//!   through the bend: content, and reversible (the unbend restores it).
//! - VISCOUS: the bend or the loop triggers an irreversible event of the
//!   ledger, read by the metabolism's counters before and after, or loses a
//!   reading the circuit had; booked as the lane's entropy. Pure geometry is
//!   never viscous: viscosity is the ledger's.
//!
//! Every re-read is QUIET (`paths_carried_quiet`): the trial's traversals are not
//! the corpus's, and the census's cumulative counters do not count them.

use super::*;
use crate::transducer::alphabet::LetterId;

/// THE LANES A BEND IS DEFINED ON, by the physics module's names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BendLane {
    /// `𝒮` — the carriage, bent on the algebra's planes.
    Spatial,
    /// `𝒽` — the word, bent by the quarter-turn letter on a rotor plane.
    Holonomic,
}

impl BendLane {
    /// The lane's index in the physics module's four (𝒮, 𝒢, 𝒯, 𝒽).
    pub fn index(self) -> usize {
        match self {
            BendLane::Spatial => 0,
            BendLane::Holonomic => 3,
        }
    }
}

/// THE FOUR ARMS OF A BEND'S RESPONSE, and the path whose class did not read
/// before it — classed as nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    Inviscid,
    Elastic,
    Curved,
    Viscous,
    NoClass,
}

/// ONE PAIR OF PLANES ON ONE LANE: the planes (masks of the algebra on `𝒮`,
/// rotor-plane indices on `𝒽`), the arms read over every face and member
/// path of every held loop, and the bends refused for want of an inverse.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PairViscosity {
    pub lane: BendLane,
    pub planes: (usize, usize),
    /// Inviscid, elastic, curved, viscous — in that order.
    pub arms: [u64; 4],
    pub refused: u64,
    /// Trials the stroke's budget did not reach (rule 10): unread, never
    /// read as inviscid.
    pub unread: u64,
    /// Member paths whose class did not read before the bend (OR3's unread
    /// circuits): no arm is classed off a class that is not there (rule 11).
    pub no_class: u64,
}

/// A CIRCUIT'S CLASS on one lane: the conjugacy class of the whole transport
/// on `𝒮`, of the edge word's rotor on `𝒽`; `None` where it does not read.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Class {
    Spatial(Option<crate::transducer::address::Address>),
    Holonomic(Option<crate::network::nerve::coord::Holonomy>),
}

fn class_of(lane: BendLane, c: &PathCarried) -> Class {
    match lane {
        BendLane::Spatial => Class::Spatial(c.composite().and_then(|m| m.class_key())),
        BendLane::Holonomic => Class::Holonomic(c.edge.as_ref().and_then(|e| e.word.rotor().holonomy_key())),
    }
}

fn read(lane: BendLane, faces: &[FormFace], vertices: &[Vertex], lp: &Orbit) -> Option<Keyed<Class>> {
    let carried = paths_carried_quiet(faces, vertices, lp)?;
    let mut out = Keyed::new();
    for (k, c) in carried.scoped() {
        out.insert(&k, class_of(lane, c));
    }
    Some(out)
}

fn unread(c: &Class) -> bool {
    matches!(c, Class::Spatial(None) | Class::Holonomic(None))
}

/// THE BEND on one plane: the move, and its generator's own class-reading
/// element (the versor on `𝒮`, the word on `𝒽`).
fn bend(lane: BendLane, face: u16, plane: usize) -> Option<PivotBy> {
    match lane {
        BendLane::Spatial => {
            let blade = crate::transducer::cga::Mv::blade(plane, num_rational::BigRational::from_integer(1.into()));
            let square = blade.geo(&blade).scalar_part();
            let one = crate::transducer::cga::Mv::scalar(num_rational::BigRational::from_integer(1.into()));
            let generator = match num_traits::Signed::is_negative(&square) {
                true => one.add(&blade),
                false => blade,
            };
            Some(PivotBy::Placement { face, translator: generator })
        }
        BendLane::Holonomic => {
            let (i, j) = ROTOR_PLANES.get(plane).copied()?;
            let letter = LetterId::quarter((i, j))?;
            Some(PivotBy::Word { face, word: Box::new(Word::from_letters(&[(letter, false)])), side: Side::Right })
        }
    }
}

/// The six rotor planes, in the alphabet's own order.
const ROTOR_PLANES: [(u8, u8); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];

/// `K`'s own class: the loop's effect on a flat face — the identity carriage
/// or word bent `A, B, A⁻¹, B⁻¹` — read as the circuit reads its class.
fn flat_loop_class(lane: BendLane, a: &PivotBy, b: &PivotBy) -> Option<Class> {
    match (lane, a, b) {
        (BendLane::Spatial, PivotBy::Placement { translator: ga, .. }, PivotBy::Placement { translator: gb, .. }) => {
            // `g̃_b g̃_a g_b g_a`, read by its class key — a ray (78.32(1)).
            let is_versor = crate::transducer::cga::is_versor;
            (is_versor(ga) && is_versor(gb)).then_some(())?;
            let k = gb.reverse().geo(&ga.reverse()).geo(gb).geo(ga);
            Some(Class::Spatial(k.class_key()))
        }
        (BendLane::Holonomic, PivotBy::Word { word: wa, .. }, PivotBy::Word { word: wb, .. }) => {
            let w = pivoted_face(&Word::identity(), wa, Side::Right);
            let w = pivoted_face(&w, wb, Side::Right);
            let w = unpivoted_face(&w, wa, Side::Right);
            let w = unpivoted_face(&w, wb, Side::Right);
            Some(Class::Holonomic(w.rotor().holonomy_key()))
        }
        _ => None,
    }
}

/// The ledger's irreversible counters, as one comparable reading.
fn ledger() -> (u64, u64, u64, [u64; 4], [u64; 4]) {
    let m = crate::physics::metabolism::metabolism();
    (m.entropy(), m.sealed, m.depleted, m.drawn, m.restored)
}

/// ONE PAIR OF BENDS ON ONE FACE OF ONE HELD LOOP: an arm per member path,
/// or `None` where a bend has no inverse (refused by name).
fn respond(
    lane: BendLane,
    faces: &[FormFace],
    vertices: &[Vertex],
    lp: &Orbit,
    before: &Keyed<Class>,
    (a, b): (&PivotBy, &PivotBy),
) -> Option<Vec<Arm>> {
    let ledger_before = ledger();
    let mut work = faces.to_vec();
    apply_by(&mut work, a);
    let after_a = read(lane, &work, vertices, lp);
    if !unapply_by(&mut work, a) {
        return None;
    }
    apply_by(&mut work, b);
    let after_b = read(lane, &work, vertices, lp);
    if !unapply_by(&mut work, b) {
        return None;
    }
    // THE LOOP: A, B, A⁻¹, B⁻¹ — the face multiplied by `K`.
    apply_by(&mut work, a);
    apply_by(&mut work, b);
    if !unapply_by(&mut work, a) || !unapply_by(&mut work, b) {
        return None;
    }
    let after_loop = read(lane, &work, vertices, lp);
    let ledger_moved = ledger() != ledger_before;
    let flat = flat_loop_class(lane, a, b);
    Some(
        before
            .scoped()
            .map(|(k, c0)| {
                let at = |m: &Option<Keyed<Class>>| m.as_ref().and_then(|m| m.get(&k).cloned());
                let (ca, cb, cl) = (at(&after_a), at(&after_b), at(&after_loop));
                let lost = !unread(c0) && [&ca, &cb, &cl].iter().any(|c| c.as_ref().is_none_or(unread));
                if unread(c0) {
                    Arm::NoClass
                } else if ledger_moved || lost {
                    Arm::Viscous
                } else if ca.as_ref() == Some(c0) && cb.as_ref() == Some(c0) {
                    Arm::Inviscid
                } else if cl.as_ref() == Some(c0) || cl == flat {
                    Arm::Elastic
                } else {
                    Arm::Curved
                }
            })
            .collect(),
    )
}

/// THE VISCOSITY OF A FORM: every pair of planes on every lane a bend is
/// defined on, read over every face of every held loop, and the viscous
/// arms booked as their lane's entropy. Each trial spends one read of the
/// stroke; a trial the stroke does not reach is counted unread.
///
/// VS1b (78.17) — EACH HELD LOOP'S CLASSES ARE READ ONCE PER LANE, BEFORE
/// ANY TRIAL. A loop none of whose member paths reads a class (OR3's unread
/// circuits) has nothing a bend could move: each of its trials is counted
/// no-class, path by path as a trial would count it, and spends no read. No
/// share of the stroke is set; the percept runs these trials last, so a
/// spent stroke leaves this instrument unread and never perception.
pub fn read_viscosity(form: &Form, budget: &mut crate::physics::Budget) -> Vec<PairViscosity> {
    let spatial: Vec<usize> = crate::transducer::cga::bivector_planes(form.n);
    let lanes: [(BendLane, Vec<usize>); 2] =
        [(BendLane::Spatial, spatial), (BendLane::Holonomic, (0..ROTOR_PLANES.len()).collect())];
    let mut out: Vec<PairViscosity> = Vec::new();
    for (lane, planes) in &lanes {
        // The loops' classes on this lane, read once: `None` where the circuit read
        // itself refuses, and with the count of paths no class reads on.
        let before: Vec<Option<(Keyed<Class>, u64, bool)>> = form
            .orbits
            .iter()
            .map(|lp| {
                let classes = read(*lane, &form.faces, &form.vertices, lp)?;
                let paths = classes.scoped().count() as u64;
                let any = classes.scoped().any(|(_, c)| !unread(c));
                Some((classes, paths, any))
            })
            .collect();
        for (i, &pa) in planes.iter().enumerate() {
            for &pb in planes.iter().skip(i + 1) {
                let mut reading =
                    PairViscosity { lane: *lane, planes: (pa, pb), arms: [0; 4], refused: 0, unread: 0, no_class: 0 };
                for (lp, read_before) in form.orbits.iter().zip(&before) {
                    let mut seen: Vec<u16> = Vec::new();
                    for &f in &lp.faces {
                        if seen.contains(&f) {
                            continue;
                        }
                        seen.push(f);
                        let (Some(a), Some(b)) = (bend(*lane, f, pa), bend(*lane, f, pb)) else { continue };
                        let Some((classes, paths, any)) = read_before else {
                            // The loop does not traverse: the trial would be
                            // refused before it bent anything.
                            reading.refused += 1;
                            continue;
                        };
                        if !any {
                            reading.no_class += paths;
                            continue;
                        }
                        if !budget.spend() {
                            reading.unread += 1;
                            continue;
                        }
                        match respond(*lane, &form.faces, &form.vertices, lp, classes, (&a, &b)) {
                            None => reading.refused += 1,
                            Some(arms) => {
                                for arm in arms {
                                    let slot = match arm {
                                        Arm::Inviscid => 0,
                                        Arm::Elastic => 1,
                                        Arm::Curved => 2,
                                        Arm::Viscous => {
                                            crate::physics::metabolism::note_viscous(lane.index());
                                            3
                                        }
                                        Arm::NoClass => {
                                            reading.no_class += 1;
                                            continue;
                                        }
                                    };
                                    reading.arms[slot] += 1;
                                }
                            }
                        }
                    }
                }
                out.push(reading);
            }
        }
    }
    out
}

/// VO1 — THE TYPED READING THE OBSERVER SERVES: every pair on which a trial
/// ran, by its lane's index, and the ledger's entropy by lane as it stands.
#[expect(
    clippy::cast_possible_truncation,
    reason = "STD1 C1 — the algebra's range: a lane, one of four; a bivector plane mask below the 2^(n+2) slots of Cl(n+1,1)"
)]
pub fn viscosity_reading(readings: &[PairViscosity]) -> crate::observe::Viscosity {
    crate::observe::Viscosity {
        pairs: readings
            .iter()
            .filter(|r| r.arms.iter().any(|&a| a > 0) || r.refused > 0 || r.unread > 0 || r.no_class > 0)
            .map(|r| crate::observe::ViscosityPair {
                lane: r.lane.index() as u8,
                planes: (r.planes.0 as u16, r.planes.1 as u16),
                arms: r.arms,
                refused: r.refused,
                unread: r.unread,
                no_class: r.no_class,
            })
            .collect(),
        entropy: crate::physics::metabolism::metabolism().viscous,
    }
}

/// A bent plane's name on its lane (by the lane's index): the algebra's
/// plane on `𝒮`, the rotor plane on `𝒽`.
pub fn bend_plane_name(lane: u8, plane: u16) -> String {
    let p = usize::from(plane);
    match (usize::from(lane) == BendLane::Holonomic.index(), ROTOR_PLANES.get(p)) {
        (false, _) => crate::transducer::cga::plane_name(p),
        (true, Some(r)) => format!("r{r:?}"),
        (true, None) => format!("r#{p}"),
    }
}

/// THE VISCOSITY LINE, off the typed reading: per lane, the pairs read with
/// their arms, and the lanes with no bend named.
pub fn viscosity_line(v: &crate::observe::Viscosity) -> String {
    let lane = |l: BendLane, name: &str| -> String {
        let pairs: Vec<String> = v
            .pairs
            .iter()
            .filter(|r| usize::from(r.lane) == l.index())
            .map(|r| {
                let (a, b) = (bend_plane_name(r.lane, r.planes.0), bend_plane_name(r.lane, r.planes.1));
                format!(
                    "{a}·{b} inviscid {} elastic {} curved {} viscous {} refused {} unread {} no-class {}",
                    r.arms[0], r.arms[1], r.arms[2], r.arms[3], r.refused, r.unread, r.no_class
                )
            })
            .collect();
        match pairs.is_empty() {
            true => format!("{name}: no held loop"),
            false => format!("{name}: {}", pairs.join(", ")),
        }
    };
    format!(
        "viscosity: {} | {} | 𝒢: no bend on this lane | 𝒯: no bend on this lane | entropy by lane {:?}",
        lane(BendLane::Spatial, "𝒮"),
        lane(BendLane::Holonomic, "𝒽"),
        v.entropy
    )
}
