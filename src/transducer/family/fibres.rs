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
//! whether `R` keeps it — a pivot `R` moves, its flow moves too: a lift; only
//! where both pivots are kept are `R`'s modes read (once per pair of
//! transports) and each mode's generator split against the fibre (78.75). A
//! flip is read one dimension up, as the 180° turn it unfolds to. Each cross
//! pair spends the stroke; a stroke that ends leaves the rest unread, by
//! count.
//!
//! A REFUSAL INVERTED (78.75). A lifted fibre in a frame whose pivots fix a
//! geoid is not left as pressure: the part of its lift that keeps the geoid,
//! at each pivot it moves along it, is a white-hole source
//! (`physics::source`) — a null flow fixing the pivot, inside the manifold —
//! and the percept's field reads it as flux.

use crate::physics::capacitor::{shape_of, Carried, Fibre, Shape};
use crate::transducer::base::{is_grid_axis, Site};
use crate::transducer::cga::{self, Mv};
use crate::transducer::element::Versor;
use num_rational::BigRational;
use num_traits::Zero;

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
/// the relative motion `ṽ₁ v₂` flows by torsion alone on the fibre joining
/// their sources — each of its modes' generators split against it (78.75), a
/// flip read one dimension up; else standing. Exact.
pub(crate) fn relate_sections(first: &Piece, second: &Piece, budget: &mut crate::physics::Budget) -> Sections {
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
    let relative = first.carriage.reverse().geo(&second.carriage);
    let n = u8::try_from(first.n.max(second.n)).ok();
    let modes = n.and_then(|n| crate::physics::capacitor::modes_of(&relative, n, budget));
    match (joining, modes) {
        (Some(f), Some(modes)) => match f.reads(&modes) {
            Carried::Rigid | Carried::Torsion => Sections::Coherent,
            Carried::Field | Carried::Lift => Sections::Standing,
        },
        _ => Sections::Standing,
    }
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

/// A class pair, its two keys in order, with what its fibres said.
pub type PairSaid = ((Vec<u8>, Vec<u8>), Said);

/// WHAT THE QUOTIENT READ: per class pair (ordered by key) its fibres'
/// torsion over observed; the classes whose turns are torsion beyond their
/// group's charged member; and the groups themselves.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Quotient {
    pub said: Vec<PairSaid>,
    pub free: Vec<Vec<u8>>,
    pub groups: Vec<Vec<Vec<u8>>>,
    /// 78.75 — the refusals this read inverted: each white-hole source as
    /// the field line it is, `1 + N`, with the mass of its pivots.
    pub sources: Vec<super::pairing::FieldLine>,
}

#[cfg(test)]
thread_local! {
    /// The last quotient this thread read — at most one percept's, replaced
    /// by the next — so a test can ask what one extraction found.
    static LAST: std::cell::RefCell<Option<Quotient>> = const { std::cell::RefCell::new(None) };
}

/// The quotient this thread's last percept read, taken — what one
/// extraction found, for the test that ran it.
#[cfg(test)]
pub fn take_last() -> Option<Quotient> {
    LAST.with(|l| l.borrow_mut().take())
}

fn step(n: &mut u64, by: u64) {
    let _ = crate::physics::metabolism::step_or_refuse(n, by);
}

/// THE SELF-CHECK ON A PIECE'S COORDINATES: does its versor carry its input
/// pivot onto its output pivot? `None` where it landed nowhere readable.
#[cfg(test)]
pub(crate) fn aligned(p: &Piece) -> Option<bool> {
    Some(p.carriage.sandwich(&p.point).projective_eq(p.out.as_ref()?))
}

/// One source held by its null ray's canonical coordinates: the generator
/// first seen, and every pivot mass that emanated along that ray summed.
fn hold_source(held: &mut crate::network::nerve::Keyed<(Mv, u64)>, src: &crate::physics::source::Source, mass: u64) {
    let Some(ray) = src.generator.canonical() else { return };
    let mut key = Vec::new();
    ray.put_canonical(&mut key);
    match held.get_mut(&key) {
        Some((_, m)) => step(m, mass),
        None => {
            held.insert(&key, (src.generator.clone(), mass));
        }
    }
}

/// READ EVERY FRAME'S FIBRES AND TAKE THE QUOTIENT BY TORSION.
pub(crate) fn read_pieces(pieces: &[Piece], budget: &mut crate::physics::Budget) -> Quotient {
    let mut said: Vec<PairSaid> = Vec::new();
    let mut sources: crate::network::nerve::Keyed<(Mv, u64)> = crate::network::nerve::Keyed::new();
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
        let shape = shared.map(|n| shape_of(&points, n));
        // 78.75 — THE FRAME'S GEOID AS ONE VECTOR, where its pivots fix a
        // round: the manifold a refusal's inversion is kept inside.
        let sigma = match &shape {
            Some(Shape::Stable(round) | Shape::Determined(round)) => crate::physics::source::geoid_vector(round),
            _ => None,
        };
        let base = shared.and_then(|n| u8::try_from(n).ok());
        // The frame's transports: each (class, carriage) with its pieces.
        let mut groups: Vec<(&[u8], &Versor, Vec<&Piece>)> = Vec::new();
        for p in &here {
            match groups.iter_mut().find(|(c, v, _)| *c == p.class.as_slice() && v.projective_eq(&p.carriage)) {
                Some((_, _, members)) => members.push(p),
                None => groups.push((&p.class, &p.carriage, vec![p])),
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
                // THE RELATIVE MOTION'S MODES, read once for the pair of
                // transports and only where a fibre asks: a pivot the finite
                // motion moves is moved by its flow too (a flow fixing it
                // would fix it at every point), so that fibre is a lift
                // without reading; the rest are read off the generators.
                let mut modes: Option<Option<Vec<cga::MotionMode>>> = None;
                for (pi, ki) in ma.iter().zip(&kept_a) {
                    for (pj, kj) in mb.iter().zip(&kept_b) {
                        if !budget.spend() {
                            continue;
                        }
                        // The tether between the two pieces, measured: one
                        // pivot (two pieces on one centroid) is no tether.
                        let fibre = Fibre::new(
                            pi.point.clone(),
                            pj.point.clone(),
                            BigRational::from_integer(pi.mass.into()),
                            BigRational::from_integer(pj.mass.into()),
                        );
                        let Some(fibre) = fibre else {
                            continue;
                        };
                        let needs_modes = (*ki && *kj) || sigma.is_some();
                        if needs_modes && modes.is_none() {
                            modes = Some(base.and_then(|n| crate::physics::capacitor::modes_of(&r, n, budget)));
                        }
                        let read = modes.as_ref().and_then(|m| m.as_deref());
                        let carried = match (ki, kj, read) {
                            (true, true, Some(m)) => fibre.reads(m),
                            (true, true, None) => continue,
                            _ => Carried::Lift,
                        };
                        // 78.75 — A REFUSAL INVERTED: a lifted fibre in a
                        // frame whose pivots fix a geoid gives, at each pivot
                        // its lift moves along the geoid, a white-hole source
                        // — the part of the lift that keeps the geoid, made a
                        // null flow fixing the pivot. Signal recovered from
                        // the refusal; a lift with none stays noise.
                        if let (Carried::Lift, Some(sigma), Some(m)) = (carried, &sigma, read) {
                            let lift = m.iter().fold(Mv::zero(), |l, mode| l.add(&fibre.split(&mode.ray.grade(2)).lift));
                            if let Some(k) = crate::physics::source::keeping(sigma, &lift) {
                                for (p, mass) in [(&pi.point, pi.mass), (&pj.point, pj.mass)] {
                                    if let Some(src) = crate::physics::source::Source::of(sigma, p, &k) {
                                        hold_source(&mut sources, &src, mass);
                                    }
                                }
                            }
                        }
                        let binding = fibre.strength();
                        step(&mut s.observed, 1);
                        s.bind_observed += &binding;
                        // Rigid and torsion agree: both keep the fibre's
                        // pivots and levels (the identity is in the torsion
                        // group). Field and lift are work: they disagree.
                        if let Carried::Rigid | Carried::Torsion = carried {
                            step(&mut s.agree, 1);
                            s.bind_agree += &binding;
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
    let mut q = quotient(pieces, said);
    // Each source once, by its null ray, carrying the mass of every pivot it
    // emanates from: a field line the percept's field reads for its flux.
    q.sources = sources
        .scoped()
        .map(|(_, (n, mass))| super::pairing::FieldLine {
            carriage: Mv::scalar(BigRational::from_integer(1.into())).add(n),
            gauge: Vec::new(),
            mass: *mass,
            free: false,
        })
        .collect();
    #[cfg(test)]
    LAST.with(|l| *l.borrow_mut() = Some(q.clone()));
    q
}

/// THE QUOTIENT: classes merged where every fibre between them agreed; each
/// group's least key charged, the rest free.
///
/// NEVER BLINDLY TRANSITIVE. Torsion is relative to each fibre, so `a ≡ b`
/// and `b ≡ d` can hold while the fibres between `a` and `d` disagree. Two
/// groups merge only where every pair across them that was observed is
/// certain; a merge a disagreeing pair would cross is refused.
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
        if !s.certain() {
            continue;
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
            continue;
        }
        let moved = groups.remove(high);
        groups[low].extend(moved);
    }
    for g in &mut groups {
        g.sort_unstable();
    }
    let free = groups.iter().filter(|g| g.len() > 1).flat_map(|g| g.iter().skip(1).cloned()).collect();
    Quotient { said, free, groups, ..Quotient::default() }
}
