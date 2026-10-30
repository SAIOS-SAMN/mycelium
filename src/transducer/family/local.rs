// MU-CLIENT §5.6 (W, C, S) — THE LOCAL DISCHARGE: what a word closes on its
// own section of a family, read by decompressing it onto each pair.
//
//! `measured_replay` decompresses a FORM onto a family and pays only a pair
//! it closes whole. A word rarely closes a whole pair, and a connection was
//! never decompressed at all (its decoder path was missing). This organ reads
//! the SECTION a word is: the sites of each pair its speech changed and said
//! right — its local section over the open set where it agrees with the pair
//! (a sheaf's local section, never a whole-scene claim). Three readers share
//! one measure:
//! - [`word_local`] — a CONNECTION decompressed onto the pair's whole before
//!   side: its versor carries every painted site, its hue map recolours them,
//!   its section is every site its speech changed, and the pair pays only
//!   where nothing it changed is false (W);
//! - [`form_local`] — a FORM spoken onto each pair (`apply_form_on`), its
//!   section the sites it closed where it asserted nothing false (C, S);
//! - [`measure_sites`] — the demand of the pair over a set of sites, priced
//!   by the one deflation reader (`deflation_of`).
//!
//! Sites are sets, read per pair; a premium is a difference of sets, never a
//! difference of sums (S). A site carries no clock: lane 𝒯 — the ground a
//! closure's motions covered — is never read here.

use super::*;
use crate::network::nerve::Keyed;

/// A WORD'S SECTION OVER A FAMILY: per pair, the sites it closed; and what
/// those sites' demand measures, per lane, summed over the pairs (the pairs
/// are disjoint scenes).
#[derive(Clone, Debug, Default)]
pub struct Local {
    pub pay: [u64; 4],
    pub sites: Vec<Keyed<()>>,
}

impl Local {
    /// The section with `other`'s sites removed, pair by pair, and measured
    /// on `family`: what this section closes that `other` does not (C: a
    /// conjugate's transport; S: a sentence's gluing). A pair `other` does
    /// not reach keeps all of this section's sites there.
    pub fn beyond(&self, other: &Local, family: &[(Grid, Grid)]) -> Local {
        let sites: Vec<Keyed<()>> = self
            .sites
            .iter()
            .enumerate()
            .map(|(pi, held)| {
                let mut left = Keyed::new();
                for (k, ()) in held.scoped() {
                    if !other.sites.get(pi).is_some_and(|o| o.contains(&k)) {
                        left.insert(k, ());
                    }
                }
                left
            })
            .collect();
        Local { pay: measure_sites(family, &sites), sites }
    }

    /// The union of several sections, pair by pair: every site any of them
    /// closed (S: what the sentence's words close on their own).
    pub fn union(sections: &[Local], pairs: usize) -> Local {
        let mut sites: Vec<Keyed<()>> = (0..pairs).map(|_| Keyed::new()).collect();
        for s in sections {
            for (pi, section_sites) in s.sites.iter().enumerate() {
                if let Some(united) = sites.get_mut(pi) {
                    for (k, ()) in section_sites.scoped() {
                        united.insert(k, ());
                    }
                }
            }
        }
        Local { pay: [0; 4], sites }
    }
}

/// The hue at `(r, c)` of `g`, or `None` off the grid.
fn hue_at(g: &Grid, r: i64, c: i64) -> Option<u8> {
    let (r, c) = (usize::try_from(r).ok()?, usize::try_from(c).ok()?);
    g.get(r)?.get(c).copied()
}

/// THE DEMAND OF EACH PAIR OVER ITS SITES, priced: the pair's residual links
/// (`residual_of`, the sites where before and after differ) standing at the
/// section's sites, closed to nothing — `deflation_of` of the move from
/// their measure to zero, the one deflation reader. Summed over the pairs,
/// a sum past its carrier refused lane by lane and left where it stood.
pub fn measure_sites(family: &[(Grid, Grid)], sites: &[Keyed<()>]) -> [u64; 4] {
    let mut pay = [0u64; 4];
    for ((x, y), at) in family.iter().zip(sites) {
        if at.is_empty() {
            continue;
        }
        let links: Vec<(i64, i64, u8, u8)> =
            residual_of(Some(x), y).into_iter().filter(|&(r, c, _, _)| at.contains(&site_key(r, c))).collect();
        let closed = crate::economy::deflation_of(&crate::economy::lane_moves(&[0; 4], &element::measure_lanes(&links)));
        if let Some(next) = crate::core::lanes::lanes_add(pay, closed) {
            pay = next;
        }
    }
    pay
}

/// W — A CONNECTION'S LOCAL DISCHARGE (MU-CLIENT §5.6 W, as the spec reads).
/// The word decompressed by its own letters onto each pair's WHOLE before
/// side: its versor (`conformal_of`, the spelling at its carriage) carries
/// every painted site at once (`carry_painted`, exact) and its hue map
/// (`sigma`) recolours what it carries — the speech is the before side moved
/// whole. Its section on the pair is EVERY SITE ITS SPEECH CHANGED (painted
/// where the before side held another hue, vacated where it held one). It
/// pays on a pair only where it says nothing false there: every changed site
/// holds, in the pair's answer, what the speech says — a site carried off
/// the grid, or two sites carried onto one with two hues, is false, and the
/// pair pays nothing. Never a choice of the bodies it happens to fit. Empty
/// for anything that is not a connection: a letter standing at no frame names
/// no body (78.149(c)).
pub fn word_local(word: &Element, family: &[(Grid, Grid)]) -> Local {
    let mut out = Local { pay: [0; 4], sites: Vec::with_capacity(family.len()) };
    let versor = match (word.is_connection(), word.letters()) {
        (true, Some(l)) => element::conformal_of(&l.spelling.word, &l.carriage).map(|v| (v, l.sigma.clone())),
        _ => None,
    };
    let Some((v, sigma)) = versor else {
        out.sites = family.iter().map(|_| Keyed::new()).collect();
        return out;
    };
    let recolour = |h: u8| match sigma.iter().find(|&&(f, _)| f == h) {
        Some(&(_, t)) => t,
        None => h,
    };
    for (x, y) in family {
        // A pair the word said something false on is no part of its section.
        out.sites.push(match spoken_section(x, y, &v, &recolour) {
            Some(closed) => closed,
            None => Keyed::new(),
        });
    }
    out.pay = measure_sites(family, &out.sites);
    out
}

/// ONE PAIR OF A WORD'S SPEECH: the before side's painted sites carried whole
/// by `v` and recoloured, laid as the after side the word says; its changed
/// sites, each required to hold in the answer what the speech says there.
/// `Some(changed)` where nothing it says is false; `None` where a site is
/// carried off the frame along `e₃`, two sites land on one with two hues, or
/// a changed site's answer differs.
fn spoken_section(x: &Grid, y: &Grid, v: &element::Versor, recolour: &dyn Fn(u8) -> u8) -> Option<Keyed<()>> {
    let void = crate::arc::ground();
    let mut painted: Vec<Painted> = Vec::new();
    for (r, row) in x.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            if h != void {
                painted.push((r as i64, c as i64, h));
            }
        }
    }
    let image = carry_painted(v, &painted)?;
    // THE SPEECH: every painted site vacated, every image site painted.
    let mut said: Keyed<u8> = Keyed::new();
    for &(r, c, _) in &painted {
        said.insert(site_key(r, c), 0);
    }
    let mut landed: Keyed<u8> = Keyed::new();
    for &(r, c, h) in &image {
        let hue = recolour(h);
        match landed.get(&site_key(r, c)) {
            Some(&held) if held != hue => return None,
            _ => {
                landed.insert(site_key(r, c), hue);
            }
        }
    }
    let mut sites: Vec<(i64, i64)> = painted.iter().map(|&(r, c, _)| (r, c)).chain(image.iter().map(|&(r, c, _)| (r, c))).collect();
    sites.sort_unstable();
    sites.dedup();
    let mut changed = Keyed::new();
    for (r, c) in sites {
        let key = site_key(r, c);
        let spoken = match landed.get(&key) {
            Some(&h) => h,
            None => match said.get(&key) {
                Some(&h) => h,
                None => continue,
            },
        };
        // A site the before side does not reach holds absence there.
        let before = match hue_at(x, r, c) {
            Some(h) => h,
            None => 0,
        };
        if spoken == before {
            continue;
        }
        if hue_at(y, r, c) != Some(spoken) {
            return None;
        }
        changed.insert(key, ());
    }
    Some(changed)
}

/// C, S — A FORM'S LOCAL SECTION: the form spoken onto each pair at
/// `placement` (`apply_form_on`, its citations resolved through `crystal`),
/// and on each pair the sites it closed — where it said the answer and the
/// before side did not hold it, a hue or an erasure — provided it asserted
/// nothing false there: a site it spoke a hue the answer does not hold makes
/// the pair no part of its section. A site it left silent (hue 0, absence)
/// where the answer holds a hue asserts nothing.
pub fn form_local(
    form: &crate::transducer::sentence::Form,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    placement: &crate::transducer::alphabet::Word,
) -> Local {
    let mut out = Local { pay: [0; 4], sites: Vec::with_capacity(family.len()) };
    for (pi, (x, y)) in family.iter().enumerate() {
        let mut closed = Keyed::new();
        let spoken = apply_form_on(form, x, placement, crystal, Some(pi)).and_then(|s| s.grid());
        if let Some(g) = spoken {
            let mut false_site = false;
            for (r, row) in g.iter().enumerate() {
                for (c, &s) in row.iter().enumerate() {
                    let (r, c) = (r as i64, c as i64);
                    let answer = hue_at(y, r, c);
                    if s != 0 && answer != Some(s) {
                        false_site = true;
                    }
                    // Said right where the before side did not hold it: a
                    // hue painted, or an erasure (absence said where the
                    // answer is absence and the before side was not).
                    if answer == Some(s) && hue_at(x, r, c) != Some(s) {
                        closed.insert(site_key(r, c), ());
                    }
                }
            }
            if false_site {
                closed = Keyed::new();
            }
        }
        out.sites.push(closed);
    }
    out.pay = measure_sites(family, &out.sites);
    out
}
