// A9 — THE SELECTION TERM: what a footprint selects on a scene, and how it
// reads it.
//
//! `Selection` is a recursive selection over a scene: a hue, a delta, a
//! difference, a ring. It is EVALUATED against a scene and never guessed
//! at; where it selects nothing it says so.

use super::*;
use crate::transducer::alphabet::ALPHABET_SAMPLE_RANK;
use crate::transducer::sentence::form::clause::Order;

/// A term of the selection algebra. Constructors = operators, arguments =
/// measured coordinates (hue byte, substrate axis index, component ordinal).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    /// K_h — the support of hue `h`; 255 is the whole occupied support.
    Hue(u8),
    /// The k-th connected component of the term's support (canonical order:
    /// by least site) — connectivity read along edges.
    Comp(Box<Selection>, u8),
    /// δ — the coboundary: sites of the support with an edge-neighbour
    /// outside it.
    Delta(Box<Selection>),
    /// The star (the edge dilation): every site adjacent to the support,
    /// support included.
    Star(Box<Selection>),
    /// Extremal projection onto an axis of the plane: sites achieving the
    /// extremum. Code `2a` the least along the plane's axis `a`, `2a + 1`
    /// the greatest.
    Ext(Box<Selection>, u8),
    /// ∩ — algebraic intersection of supports.
    Inter(Box<Selection>, Box<Selection>),
    /// ∖ — set difference of supports.
    Minus(Box<Selection>, Box<Selection>),
    /// Connected closure: the sites of the second support edge-reachable
    /// from the first (seed ∩ within, grown through within) — pure Level-0
    /// connectivity, the operator that composes enclosure:
    /// enclosed(K) = K ∖ Flood(K ∩ δ(whole), K).
    Flood(Box<Selection>, Box<Selection>),
    /// ORDER-INVARIANT component selection: of the support's connected
    /// components, the one UNIQUELY extremizing a measured C₀ invariant.
    /// Code 0..4 = (max |K|, min |K|, max L1 extent diameter, min diameter).
    /// A tie selects NOTHING — no unique extremum, no selection (exactness,
    /// never a coin flip).
    ArgExt(Box<Selection>, u8),
    /// SITE TRANSPORT intersection: a ∩ Shift_d(b) where d is the exact
    /// least-corner difference extrema(a) − extrema(b) — the offset is
    /// DERIVED from the two supports at evaluation time, never a parameter.
    Transport(Box<Selection>, Box<Selection>),
}

pub(crate) fn sorted(mut v: Vec<SectionSite>) -> Vec<SectionSite> {
    v.sort_unstable();
    v.dedup();
    v
}

pub(crate) fn has(s: &[SectionSite], p: SectionSite) -> bool {
    s.binary_search(&p).is_ok()
}

/// The connected components of a support of the section, each sorted: the
/// scene is one sample's, so its sites stand in frame 0 and are handed to
/// the base as compartments there.
pub(crate) fn planar_components(sup: &[SectionSite]) -> Vec<Vec<SectionSite>> {
    use crate::transducer::blade::Compartment;
    let compartments: Vec<Compartment> = sup.iter().map(|&s| Compartment::from(PaintedSite::new(s, 0))).collect();
    components(&compartments)
        .into_iter()
        .map(|c| sorted(c.iter().filter_map(|l| SectionSite::of_family(&l.site)).collect()))
        .collect()
}

/// THE FACE NEIGHBOURS OF A SITE OF THE SECTION (MU-SENSING §3.2): one step
/// back and one forward along each axis of the plane, axis by axis, the
/// contact of grade `n − 1`. A step that leaves `i64` names no site.
pub(crate) fn neighbours(p: SectionSite) -> impl Iterator<Item = SectionSite> {
    (0..ALPHABET_SAMPLE_RANK).flat_map(move |axis| {
        [-1i64, 1].into_iter().filter_map(move |step| {
            let mut d = [0i64; ALPHABET_SAMPLE_RANK];
            *d.get_mut(axis)? = step;
            p.moved(d)
        })
    })
}

/// THE AXIS OF THE PLANE AND THE ORDER AN EXTREMAL PROJECTION'S CODE NAMES:
/// code `2a` the least coordinate along the plane's axis `a`, `2a + 1` the
/// greatest; a code past the plane's axes names the greatest along its
/// last axis, as the code's byte has always read.
fn extremal(code: u8) -> (usize, Order) {
    let axis = usize::from(code / 2);
    match (axis < ALPHABET_SAMPLE_RANK, code % 2) {
        (true, 0) => (axis, Order::Least),
        (true, _) => (axis, Order::Greatest),
        (false, _) => (ALPHABET_SAMPLE_RANK - 1, Order::Greatest),
    }
}

/// EVERY EXTREMAL CODE THE PLANE HOLDS: the least and the greatest along
/// each of its axes ([`extremal`]), generated from the plane's rank.
pub(crate) fn extremal_codes() -> impl Iterator<Item = u8> {
    (0..=u8::MAX).take_while(|&code| usize::from(code / 2) < ALPHABET_SAMPLE_RANK)
}

/// The coordinate of a site along one axis of the plane.
fn along(s: &SectionSite, axis: usize) -> Option<i64> {
    s.coordinates().get(axis).copied()
}

impl Selection {
    /// EVALUATE the term over a scene (the percept's painted sites) to its
    /// support set — exact, deterministic, replayable on any scene.
    ///
    /// `Ext` IS an extremum on a substrate axis and `ArgExt` IS the
    /// component uniquely extremizing a measured invariant: the reading is
    /// the operator's definition, not a candidate picked by a scalar, and
    /// `ArgExt` selects nothing on a tie rather than breaking one (rule 4).
    /// `Transport` derives its offset from the two least corners.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — an extremum on an axis is this algebra's operator, and a tie selects nothing"
    )]
    pub fn eval(&self, scene: &[PaintedSite]) -> Vec<SectionSite> {
        match self {
            Selection::Hue(h) => sorted(scene.iter().filter(|p| *h == 255 || p.symbol == *h).map(|p| p.site).collect()),
            Selection::Comp(e, k) => {
                let sup = e.eval(scene);
                // The scene is one sample's: its sites stand in frame 0.
                let mut comps: Vec<Vec<SectionSite>> = planar_components(&sup);
                // A component IS its least site, in the coordinate order the
                // support is already written in; a component with no site
                // has no least one and sorts after those that do.
                #[expect(
                    clippy::disallowed_methods,
                    reason = "rule 4 — canonical key order: the component's least site, the support's own coordinate"
                )]
                comps.sort_by_key(|c| match c.as_slice() {
                    [least, ..] => Some(*least),
                    [] => None,
                });
                // A term naming a component the support does not have
                // selects nothing (§2.4), which is the empty support and
                // not a default standing in for one (rule 11).
                #[expect(
                    clippy::manual_unwrap_or_default,
                    reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; the empty support is measured here, not defaulted to"
                )]
                match comps.into_iter().nth(*k as usize) {
                    Some(c) => c,
                    None => Vec::new(),
                }
            }
            Selection::Delta(e) => {
                let sup = e.eval(scene);
                sup.iter().copied().filter(|&p| neighbours(p).any(|q| !has(&sup, q))).collect()
            }
            Selection::Star(e) => {
                let sup = e.eval(scene);
                let mut out = sup.clone();
                for &p in &sup {
                    out.extend(neighbours(p));
                }
                sorted(out)
            }
            Selection::Ext(e, code) => {
                let sup = e.eval(scene);
                if sup.is_empty() {
                    return sup;
                }
                // The extremum of a non-empty support exists; an empty one
                // was returned above, and selects nothing here (§2.4).
                let (axis, order) = extremal(*code);
                let values = sup.iter().filter_map(|s| along(s, axis));
                let extremum = match order {
                    Order::Least => values.min(),
                    Order::Greatest => values.max(),
                };
                let Some(m) = extremum else { return Vec::new() };
                sup.into_iter().filter(|s| along(s, axis) == Some(m)).collect()
            }
            Selection::Inter(a, b) => {
                let (sa, sb) = (a.eval(scene), b.eval(scene));
                sa.into_iter().filter(|p| has(&sb, *p)).collect()
            }
            Selection::Minus(a, b) => {
                let (sa, sb) = (a.eval(scene), b.eval(scene));
                sa.into_iter().filter(|p| !has(&sb, *p)).collect()
            }
            Selection::ArgExt(e, code) => {
                let sup = e.eval(scene);
                let comps: Vec<Vec<SectionSite>> = planar_components(&sup);
                if comps.is_empty() {
                    return Vec::new();
                }
                // A component with no sites has no diameter to measure, so
                // the measure refuses rather than reading zero for it
                // (rule 11); `components` returns none such, and a term
                // whose measure refuses selects nothing below. The diameter
                // is the L1 extent: the span along each axis of the plane,
                // added axis by axis.
                let measure = |k: &Vec<SectionSite>| -> Option<i64> {
                    match code {
                        0 | 1 => Some(k.len() as i64),
                        _ => {
                            let mut diameter = 0i64;
                            for axis in 0..ALPHABET_SAMPLE_RANK {
                                let on = || k.iter().filter_map(|s| along(s, axis));
                                diameter += on().max()? - on().min()?;
                            }
                            Some(diameter)
                        }
                    }
                };
                let Some(vals) = comps.iter().map(measure).collect::<Option<Vec<i64>>>() else {
                    return Vec::new();
                };
                let target = match code {
                    0 | 2 => vals.iter().max(),
                    _ => vals.iter().min(),
                };
                let Some(target) = target.copied() else { return Vec::new() };
                let winners: Vec<&Vec<SectionSite>> = comps
                    .iter()
                    .zip(&vals)
                    .filter(|(_, v)| **v == target)
                    .map(|(k, _)| k)
                    .collect();
                match winners.as_slice() {
                    [only] => (*only).clone(),
                    _ => Vec::new(), // tie: no unique extremum, no selection
                }
            }
            Selection::Transport(a, b) => {
                let (sa, sb) = (a.eval(scene), b.eval(scene));
                if sa.is_empty() || sb.is_empty() {
                    return Vec::new();
                }
                // Each support's least corner: the least coordinate along
                // each axis of the plane.
                let corner = |s: &[SectionSite]| SectionSite::span_of(s.iter().copied()).map(|(c, _)| c);
                let (Some(a), Some(b)) = (corner(&sa), corner(&sb)) else {
                    return Vec::new();
                };
                let Some(d) = a.offset_from(b) else { return Vec::new() };
                let shifted: Vec<SectionSite> = sorted(sb.iter().filter_map(|s| s.moved(d.coordinates())).collect());
                sa.into_iter().filter(|p| has(&shifted, *p)).collect()
            }
            Selection::Flood(seed, within) => {
                let (sd, w) = (seed.eval(scene), within.eval(scene));
                let mut reached: Vec<SectionSite> = sd.into_iter().filter(|p| has(&w, *p)).collect();
                reached.sort_unstable();
                let mut frontier = reached.clone();
                while let Some(p) = frontier.pop() {
                    for q in neighbours(p) {
                        if has(&w, q) && !has(&reached, q) {
                            // Either arm is the seat `q` belongs at, so the
                            // search answers a position and never a failure
                            // to be unwrapped (§2.4).
                            let (Ok(i) | Err(i)) = reached.binary_search(&q);
                            reached.insert(i, q);
                            frontier.push(q);
                        }
                    }
                }
                reached
            }
        }
    }

    /// A21 KEY of the term: its tape under the selection tag. The term's
    /// own canonical coordinates, so two nodes order a set of terms alike
    /// without measuring any of them against a scene.
    pub fn key(&self) -> crate::transducer::address::Address {
        let mut buf = vec![crate::transducer::address::TAG_SEL];
        self.to_bytes(&mut buf);
        crate::transducer::address::Address::new(buf)
    }

    /// Node count — the term's cost (minimality prefers shallow derivations).
    pub fn size(&self) -> usize {
        match self {
            Selection::Hue(_) => 1,
            Selection::Comp(e, _) | Selection::Delta(e) | Selection::Star(e) | Selection::Ext(e, _) => 1 + e.size(),
            Selection::Inter(a, b) | Selection::Minus(a, b) | Selection::Flood(a, b) | Selection::Transport(a, b) => {
                1 + a.size() + b.size()
            }
            Selection::ArgExt(e, _) => 1 + e.size(),
        }
    }

    /// Levels from the root to the deepest leaf: a leaf is one.
    pub fn height(&self) -> usize {
        match self {
            Selection::Hue(_) => 1,
            Selection::Comp(e, _) | Selection::Delta(e) | Selection::Star(e) | Selection::Ext(e, _) | Selection::ArgExt(e, _) => 1 + e.height(),
            Selection::Inter(a, b) | Selection::Minus(a, b) | Selection::Flood(a, b) | Selection::Transport(a, b) => {
                1 + a.height().max(b.height())
            }
        }
    }
}

/// THE DEEPEST TERM THE LANGUAGE MINTS, in levels (MU-DERIVATIONS D4.4
/// (iii)): the derive's closure (`anchor::derive::candidates`) is three
/// levels — a base at most `ArgExt(Inter(Hue, Star(Hue)))` (four), a Level-2
/// step at most the enclosure `Minus(t, Flood(Inter(t, δ·), t))` (three
/// more), and a Level-3 step on a component projection, which stays within
/// it. A decoded term deeper than any the producer mints is no word of the
/// language and is refused, which bounds every walker of a decoded tree —
/// `Drop`, `Clone`, the codec, `eval` — at once, with no stack measured.
/// EXACT: derive_closure_height_equals_selection_height
pub const SELECTION_HEIGHT: usize = 4 + 3;
