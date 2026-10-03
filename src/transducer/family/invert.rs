// 78.76 — THE REMAINDER INVERTED: noise is signal missing an inversion (the
// operator, 2026-10-02). What a form fails to speak of its scene is read as a
// scene of its own, perceived and extracted by the same eyes, and the
// generators that produce it join the form — judged only by speech.
//
//! THE LOOP, closed where it was open. A form is signal where speaking it
//! reproduces the plane (`form_deflation`); the remainder is the noise,
//! site by site. The pivot drive wrote its moves onto FACE carriers, and
//! speech reads only VERTICES (`apply_form_on`), so no pivot move could
//! change what was spoken and the remainder could not fall (fleet:
//! `pivoted [0, 0, 0, 0]`). The inversion acts on what speech reads:
//!
//! 1. Speak the form over its family; the remainder per pair.
//! 2. THE REMAINDER AS A SCENE: each pair's input against an output holding
//!    only the remainder's observed hues — what the form left unsaid, painted
//!    alone. Every pair is kept, so frames stay the family's own; a pair with
//!    nothing left asks for nothing.
//! 3. Perceive and extract that scene (`extract_rule_with`): the exact
//!    generators — word vertices, each a class of transport with its members
//!    — that carry the input's bodies onto the remainder.
//! 4. Those vertices join the form; speak again. ACCEPTED only where no
//!    pair's remainder grew and the whole fell: judged by the plane, never by
//!    the crystal's own consistency.
//!
//! WHAT IT CANNOT INVERT, BY NAME: a site the form painted where the scene
//! holds nothing (an erasure — adding a generator cannot unpaint); a
//! generator whose coordinate another vertex of the form already holds (a
//! conflict, never overwritten); a remainder scene the eyes extract nothing
//! from.

use super::*;

/// WHAT ONE INVERSION DID: the remainder before and after (links, summed over
/// the pairs), the generators extracted from it and those that joined, the
/// coordinate conflicts, the erasures it could not express, and whether the
/// speech accepted it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Inversion {
    pub before: u64,
    pub after: u64,
    pub extracted: u64,
    /// The remainder scene's demand vertices: what its own extraction could
    /// not account for as words.
    pub demands: u64,
    /// The demands by kind, in `Demand`'s own order: rotor prime, standing,
    /// source, creation, deformation, dislocation.
    pub kinds: [u64; 6],
    pub joined: u64,
    pub conflicts: u64,
    pub erasures: u64,
    pub accepted: bool,
    /// The remainder scene extracted no rule at all.
    pub unextracted: bool,
    /// Accepted and nothing is left: the form closes its family.
    pub closed: bool,
}

/// HOW THE REMAINDER IS PAINTED AS A SCENE: its unsaid sites alone, or each
/// whole observed body holding an unsaid site. The sites alone cut a body
/// that was half spoken into a fragment, whose shape then differs from every
/// input body by construction — a deformation the scene did not hold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Painted {
    Sites,
    Bodies,
}

/// THE REMAINDER AS A SCENE: per pair, an output of the observed frame's size
/// holding what the form left unsaid — its sites, or the whole observed bodies
/// they stand in; the erasures (painted, observed absent) counted, not
/// expressible as a body.
pub fn remainder_scene(
    family: &[(Grid, Grid)],
    remainders: &[Vec<(i64, i64, u8, u8)>],
    painted: Painted,
) -> (Vec<(Grid, Grid)>, u64) {
    let mut erasures = 0u64;
    let scene = family
        .iter()
        .zip(remainders)
        .map(|((x, y), rem)| {
            let mut m: Grid = y.iter().map(|row| vec![0u8; row.len()]).collect();
            let mut paint = |r: i64, c: i64, h: u8| {
                if let (Ok(ur), Ok(uc)) = (usize::try_from(r), usize::try_from(c)) {
                    if let Some(cell) = m.get_mut(ur).and_then(|row| row.get_mut(uc)) {
                        *cell = h;
                    }
                }
            };
            let unsaid: Vec<(i64, i64)> = rem
                .iter()
                .filter_map(|&(r, c, spoken, observed)| match observed {
                    0 => {
                        if spoken != 0 {
                            erasures += 1;
                        }
                        None
                    }
                    _ => Some((r, c)),
                })
                .collect();
            match painted {
                Painted::Sites => {
                    for &(r, c, _, observed) in rem {
                        if observed != 0 {
                            paint(r, c, observed);
                        }
                    }
                }
                // Each observed body (a hue-blind component of the scene's
                // output) holding an unsaid site, painted whole.
                Painted::Bodies => {
                    for body in perceive(y) {
                        let at = |s: &crate::transducer::base::Site| Some((s.axis(0)?, s.axis(1)?));
                        let holds = body.compartments.iter().any(|l| at(&l.site).is_some_and(|p| unsaid.contains(&p)));
                        if holds {
                            for l in &body.compartments {
                                if let Some((r, c)) = at(&l.site) {
                                    paint(r, c, l.hue);
                                }
                            }
                        }
                    }
                }
            }
            (x.clone(), m)
        })
        .collect();
    (scene, erasures)
}

fn links(remainders: &[Vec<(i64, i64, u8, u8)>]) -> u64 {
    remainders.iter().map(|r| r.len() as u64).sum()
}

/// INVERT A FORM'S REMAINDER (78.76): the form with the generators its
/// remainder's scene extracts, where speaking it leaves less unsaid on no pair
/// more — or the form unchanged, with what was read. `None` where the form
/// leaves nothing unsaid: there is nothing to invert.
pub fn invert_remainder(
    form: &Form,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
) -> Option<(Form, Inversion)> {
    invert_remainder_as(form, family, crystal, budget, Painted::Bodies)
}

/// [`invert_remainder`] with the remainder painted as its sites or its
/// bodies — the instrument reads both.
pub fn invert_remainder_as(
    form: &Form,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
    painted: Painted,
) -> Option<(Form, Inversion)> {
    let (_, remainders) = form_deflation(form, family, crystal);
    let before = links(&remainders);
    if before == 0 {
        return None;
    }
    let (scene, erasures) = remainder_scene(family, &remainders, painted);
    let mut read = Inversion { before, after: before, erasures, ..Inversion::default() };
    let Some(rule) = extract_rule_with(&scene, crystal, budget) else {
        read.unextracted = true;
        return Some((form.clone(), read));
    };
    // The remainder's generators, each joining at its own coordinate. A class
    // the form already holds is ONE VERTEX PER CLASS (Q1): the remainder's
    // members join it on the charts it did not yet act on — a member on a
    // chart it already acts on is a conflict, never overwritten.
    let mut joined = form.clone();
    for (at, v) in rule.form.vertices.entries() {
        match v {
            Vertex::Word { .. } => read.extracted += 1,
            Vertex::Demand { kind, .. } => {
                read.demands += 1;
                let at = match kind {
                    crate::transducer::sentence::Demand::RotorPrime { .. } => 0,
                    crate::transducer::sentence::Demand::Standing { .. } => 1,
                    crate::transducer::sentence::Demand::Source { .. } => 2,
                    crate::transducer::sentence::Demand::Creation => 3,
                    crate::transducer::sentence::Demand::TopologicalDeformation { .. } => 4,
                    crate::transducer::sentence::Demand::Dislocation { .. } => 5,
                };
                read.kinds[at] += 1;
                continue;
            }
            _ => continue,
        }
        let (Some(new_acts), Some(Vertex::Word { acts, .. })) = (v.acts(), joined.vertices.at_mut(&at)) else {
            if joined.put_vertex(v.clone()).is_some() {
                read.joined += 1;
            }
            continue;
        };
        let mut added = false;
        for (sk, a) in new_acts.entries() {
            match acts.at(&sk) {
                Some(_) => read.conflicts += 1,
                None => {
                    acts.put(&sk, a.clone());
                    added = true;
                }
            }
        }
        if added {
            read.joined += 1;
        }
    }
    if read.joined == 0 {
        return Some((form.clone(), read));
    }
    joined.reindex();
    let (_, after) = form_deflation(&joined, family, crystal);
    read.after = links(&after);
    // JUDGED BY THE PLANE: no pair left with more unsaid, and less unsaid in
    // all (12.6: no pair pays for another).
    let no_pair_rose = remainders.iter().zip(&after).all(|(b, a)| a.len() <= b.len());
    read.accepted = no_pair_rose && read.after < before;
    read.closed = read.accepted && read.after == 0;
    match read.accepted {
        true => Some((joined, read)),
        false => Some((form.clone(), read)),
    }
}
