// 10.2, 21 step 1 — THE GAUGE: the hue permutation, its profile, and the
// fiber of assignments one body can be spoken with.
//
//! A gauge is a set of (before, after) hue pairs: its profile and key,
//! permutation composition and inversion, the hue map between two
//! assignments, the transition versor, and the reads that walk a word's fibre.

use super::*;

/// THE GAUGE PROFILE of an assignment: its transition classes with their
/// counts reduced to proportions (the counts divided by their common
/// factor), so a dilation by ρ — which scales every count by ρ² — has the
/// same profile. Sorted; one run-length pass over the sorted assignment.
pub fn gauge_profile(asg: &[(u8, u8)]) -> Vec<((u8, u8), u32)> {
    let mut sorted = asg.to_vec();
    sorted.sort_unstable();
    let mut out: Vec<((u8, u8), u32)> = Vec::new();
    for t in sorted {
        match out.last_mut() {
            Some((k, n)) if *k == t => *n += 1,
            _ => out.push((t, 1)),
        }
    }
    let g = out.iter().fold(0u32, |g, (_, n)| gcd(g, *n));
    if g > 1 {
        for (_, n) in out.iter_mut() {
            *n /= g;
        }
    }
    out
}

pub(crate) fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// The key of a gauge profile: the charge coordinates written out.
pub fn gauge_key(asg: &[(u8, u8)]) -> Vec<u8> {
    let mut buf = vec![crate::transducer::address::TAG_GAUGE];
    for ((b, a), n) in gauge_profile(asg) {
        buf.push(b);
        buf.push(a);
        buf.extend_from_slice(&n.to_le_bytes());
    }
    buf
}

pub(crate) fn compose_perm(a: &[(u8, u8)], b: &[(u8, u8)]) -> Vec<(u8, u8)> {
    if a.is_empty() {
        return b.to_vec();
    }
    if b.is_empty() {
        return a.to_vec();
    }
    let apply = |sigma: &[(u8, u8)], h: u8| -> u8 {
        sigma.iter().find(|(f, _)| *f == h).map(|(_, t)| *t).unwrap_or(h)
    };
    let mut out: Vec<(u8, u8)> = Vec::new();
    let mut seen: Vec<u8> = Vec::new();
    for &(f, _) in a.iter().chain(b.iter()) {
        if seen.contains(&f) {
            continue;
        }
        seen.push(f);
        let t = apply(b, apply(a, f));
        if t != f {
            out.push((f, t));
        }
    }
    out.sort_unstable();
    out
}

pub(crate) fn gauge_is_perm(sigma: &[(u8, u8)]) -> bool {
    let mut image: [u8; PALETTE] = std::array::from_fn(|i| i as u8);
    for &(f, t) in sigma {
        if (f as usize) >= PALETTE || (t as usize) >= PALETTE {
            return false;
        }
        image[f as usize] = t;
    }
    let mut seen = [false; PALETTE];
    image.iter().all(|&t| {
        let i = t as usize;
        if i >= PALETTE || seen[i] {
            false
        } else {
            seen[i] = true;
            true
        }
    })
}

pub(crate) fn invert_perm(a: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let mut out: Vec<(u8, u8)> = Vec::new();
    let mut tos: Vec<u8> = Vec::new();
    for &(f, t) in a {
        if tos.contains(&t) {
            return None;
        }
        tos.push(t);
        if f != t {
            out.push((t, f));
        }
    }
    out.sort_unstable();
    Some(out)
}

/// The versor carrying `from`'s support onto `to`'s, solved in closed form
/// on the two shapes (see `blade::versor_between`) and verified corner by
/// corner. None when no rational versor carries one onto the other — the
/// exact congruence test on the conformal orbit, returning the transport
/// operator rather than a verdict.
pub fn transition(from: &[(i64, i64, u8, u8)], to: &[(i64, i64, u8, u8)]) -> Option<Versor> {
    let from_shape = shape_of_chain(from)?;
    transition_from_shape(&from_shape, to)
}

/// The shape of a chain's support (None for an empty chain) — lifted once
/// and reused across the candidates a lookup compares it with.
pub fn shape_of_chain(chain: &[(i64, i64, u8, u8)]) -> Option<crate::transducer::blade::Shape> {
    let mut a = positions(chain);
    a.sort_unstable();
    a.dedup();
    if a.is_empty() {
        return None;
    }
    Some(crate::transducer::blade::shape_of(&a))
}

/// The versor carrying an already-lifted shape onto `to`'s support.
pub fn transition_from_shape(
    from: &crate::transducer::blade::Shape,
    to: &[(i64, i64, u8, u8)],
) -> Option<Versor> {
    let to_shape = shape_of_chain(to)?;
    crate::transducer::blade::versor_between(from, &to_shape)
}

/// Apply a hue-map σ (functional from↦to relations) to a transition assignment:
/// every hue present must be in σ's domain, else the orbit ends (None).
pub fn apply_hue_map(sigma: &[(u8, u8)], asg: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let map = |h: u8| sigma.iter().find(|(f, _)| *f == h).map(|(_, t)| *t);
    asg.iter().map(|(b, a)| Some((map(*b)?, map(*a)?))).collect()
}

/// Derive the hue-map carrying assignment A onto assignment B cell-wise —
/// `None` if the relation is not functional (one hue demanding two images).
/// This is DERIVATION from the data, never a hypothesized plane.
pub fn derive_hue_map(a: &[(u8, u8)], b: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let mut sigma: Vec<(u8, u8)> = Vec::new();
    let mut bind = |from: u8, to: u8| -> bool {
        match sigma.iter().find(|(f, _)| *f == from) {
            Some((_, t)) => *t == to,
            None => {
                sigma.push((from, to));
                true
            }
        }
    };
    if a.len() != b.len() {
        return None;
    }
    for ((ab, aa), (bb, ba)) in a.iter().zip(b) {
        if !bind(*ab, *bb) || !bind(*aa, *ba) {
            return None;
        }
    }
    sigma.sort_unstable();
    Some(sigma)
}

/// The transition assignment of a centered chain (its fiber coordinate),
/// index-aligned with [`positions`] (chains sort by position first).
pub fn assignment(chain: &[(i64, i64, u8, u8)]) -> Vec<(u8, u8)> {
    chain.iter().map(|(_, _, b, a)| (*b, *a)).collect()
}

/// Canonical key of one hue pair.
pub fn hue_pair_key(before: u8, after: u8) -> Vec<u8> {
    vec![crate::transducer::address::TAG_HUE, before, after]
}

/// The hues a pair key names. The tag is not a colour.
pub fn hue_pair_of(key: &[u8]) -> Option<(u8, u8)> {
    match key {
        [crate::transducer::address::TAG_HUE, b, a] => Some((*b, *a)),
        _ => None,
    }
}

/// Canonical key of a whole assignment (the exact list of pairs).
pub fn assignment_key(asg: &[(u8, u8)]) -> Vec<u8> {
    let mut buf = vec![crate::transducer::address::TAG_ASSIGN];
    buf.extend_from_slice(&(asg.len() as u32).to_le_bytes());
    for &(b, a) in asg {
        buf.push(b);
        buf.push(a);
    }
    buf
}

/// One transition class, or several keyed by the pair.
pub enum GaugeFibers {
    One((u8, u8), Vec<(i64, i64, u8, u8)>),
    Many(crate::network::nerve::Keyed<Vec<(i64, i64, u8, u8)>>),
}

/// THE GAUGE FIBERS of a chain: the chain opened along its fiber
/// coordinate, one sub-chain per transition class. Indexed by the pair
/// key. One class is `One`; two or more is `Many`. Empty is `None`.
pub fn gauge_fibers(chain: &[(i64, i64, u8, u8)]) -> Option<GaugeFibers> {
    if chain.is_empty() {
        return None;
    }
    let mut idx: crate::network::nerve::Keyed<Vec<(i64, i64, u8, u8)>> =
        crate::network::nerve::Keyed::new();
    for &cell in chain {
        let k = hue_pair_key(cell.2, cell.3);
        match idx.get_mut(&k) {
            Some(fiber) => fiber.push(cell),
            None => {
                idx.insert(k, vec![cell]);
            }
        }
    }
    let mut first = None;
    let mut many = false;
    for (k, fiber) in idx.fiber(crate::network::nerve::ROOT) {
        match first {
            None => first = Some((hue_pair_of(&k)?, fiber.clone())),
            Some(_) => {
                many = true;
                break;
            }
        }
    }
    match (many, first) {
        (false, Some((class, cells))) => Some(GaugeFibers::One(class, cells)),
        _ => Some(GaugeFibers::Many(idx)),
    }
}

impl Element {
    /// Every concrete rewrite chain this word can speak — the base chain plus
    /// each section/orbit assignment materialized over the base geometry
    /// (positions held, hue-transition classes substituted class-for-class).
    /// The single source for application/projection (the gate and the SDK's
    /// `Entity::project` both speak through this).
    pub fn variant_chains(&self) -> Vec<Vec<(i64, i64, u8, u8)>> {
        let base = self.sub.clone();
        if base.is_empty() {
            return Vec::new();
        }
        let base_pairs = assignment(&base);
        let mut out = vec![base.clone()];
        for asg in self.assignments() {
            if asg.len() != base_pairs.len() || asg == base_pairs {
                continue;
            }
            let variant: Vec<_> = base
                .iter()
                .map(|(r, c, b, a)| {
                    let i = base_pairs.iter().position(|p| *p == (*b, *a)).unwrap_or(0);
                    (*r, *c, asg[i].0, asg[i].1)
                })
                .collect();
            out.push(variant);
        }
        out
    }

    /// Every transition assignment this thought speaks: base, free sections, and
    /// each torsion cycle's ORBIT expanded exactly to its period (bounded by ℓ —
    /// a closed cycle speaks members never individually witnessed).
    /// The assignments this word speaks with a given gauge charge, read by
    /// the charge's key: 0 is no such gauge, 1 is the citation, many is
    /// multiplicity on the fibre. One read; the axis is never walked.
    pub fn assignments_by_profile(&self, key: &[u8]) -> Vec<usize> {
        let table = self.profiles.get_or_init(|| {
            let mut rows: Vec<(Vec<u8>, Vec<usize>)> = Vec::new();
            let mut keyed: Vec<(Vec<u8>, usize)> =
                self.assignments().iter().enumerate().map(|(i, a)| (gauge_key(a), i)).collect();
            keyed.sort_unstable();
            for (k, i) in keyed {
                match rows.last_mut() {
                    Some((last, is)) if *last == k => is.push(i),
                    _ => rows.push((k, vec![i])),
                }
            }
            rows
        });
        match table.binary_search_by(|(k, _)| k.as_slice().cmp(key)) {
            Ok(i) => table[i].1.clone(),
            Err(_) => Vec::new(),
        }
    }

    pub fn assignments(&self) -> Vec<Vec<(u8, u8)>> {
        let base = assignment(&self.sub);
        let mut out = vec![base.clone()];
        out.extend(self.sections.iter().cloned());
        for TorsionCycle { sigma, period: l, .. } in &self.torsion {
            let mut cur = base.clone();
            for _ in 1..*l {
                match apply_hue_map(sigma, &cur) {
                    Some(next) => {
                        if !out.contains(&next) {
                            out.push(next.clone());
                        }
                        cur = next;
                    }
                    None => break,
                }
            }
        }
        out
    }

    /// Does a centered chain SIT on this thought — same base positions, and its
    /// assignment is the base's, a free section's, or a member of a torsion
    /// cycle's orbit (walked to the period, O(ℓ·cells) — the closed-form
    /// membership that makes the compressed fiber CHEAPER than the list it
    /// replaced)?
    /// Which assignment (in [`assignments`] order: base, sections, torsion
    /// orbit) the centered chain sits on, if any — the gauge coordinate a
    /// citation names.
    pub fn matching_assignment(&self, chain: &[(i64, i64, u8, u8)]) -> Option<usize> {
        if self.sub.is_empty() || positions(&self.sub) != positions(chain) {
            return None;
        }
        let asg = assignment(chain);
        self.assignments().iter().position(|a| *a == asg)
    }

    pub fn matches_chain(&self, chain: &[(i64, i64, u8, u8)]) -> bool {
        if self.sub.is_empty() || positions(&self.sub) != positions(chain) {
            return false;
        }
        let asg = assignment(chain);
        let base = assignment(&self.sub);
        if asg == base || self.sections.contains(&asg) {
            return true;
        }
        for TorsionCycle { sigma, period: l, .. } in &self.torsion {
            let mut cur = base.clone();
            for _ in 1..*l {
                match apply_hue_map(sigma, &cur) {
                    Some(next) => {
                        if next == asg {
                            return true;
                        }
                        cur = next;
                    }
                    None => break,
                }
            }
        }
        false
    }

}
