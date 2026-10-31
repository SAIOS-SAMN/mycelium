// 10.2, 21 step 1 — THE GAUGE: the hue permutation, its profile, and the
// fiber of assignments one body can be spoken with.
//
//! A gauge is a set of (before, after) hue pairs: its profile and key,
//! permutation composition and inversion, the hue map between two
//! assignments, the transition versor, and the reads that cross a word's fibre.

use super::*;

/// THE GAUGE PROFILE of an assignment: its transition classes with the classes'
/// counts reduced to proportions (the counts divided by the counts' common
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
    // A hue the permutation does not move maps to itself: that is what
    // the permutation says about it (rule 11).
    let apply = |sigma: &[(u8, u8)], h: u8| -> u8 {
        match sigma.iter().find(|(f, _)| *f == h) {
            Some((_, t)) => *t,
            None => h,
        }
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
    #[expect(
        clippy::cast_possible_truncation,
        reason = "STD1 C1 — the sensor's: a hue below PALETTE = 10, the slots of the [u8; PALETTE] it fills"
    )]
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
pub fn transition(
    from: &[(i64, i64, u8, u8)],
    to: &[(i64, i64, u8, u8)],
    budget: &mut crate::physics::Budget,
) -> Option<Versor> {
    let from_shape = shape_of_chain(from)?;
    transition_from_shape(&from_shape, to, budget)
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
    Some(crate::transducer::blade::shape_of_section(&a))
}

/// The versor carrying an already-lifted shape onto `to`'s support.
pub fn transition_from_shape(
    from: &crate::transducer::blade::Shape,
    to: &[(i64, i64, u8, u8)],
    budget: &mut crate::physics::Budget,
) -> Option<Versor> {
    let to_shape = shape_of_chain(to)?;
    crate::transducer::blade::versor_between(from, &to_shape, budget).ok()
}

/// Apply a hue-map σ (functional from↦to relations) to a transition assignment:
/// every hue present must be in σ's domain, else the orbit ends (None).
pub fn apply_hue_map(sigma: &[(u8, u8)], asg: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let map = |h: u8| sigma.iter().find(|(f, _)| *f == h).map(|(_, t)| *t);
    asg.iter().map(|(b, a)| Some((map(*b)?, map(*a)?))).collect()
}

/// One transition assignment: the (before, after) hue of each link.
pub type Assignment = Vec<(u8, u8)>;

/// One row of the profile table: a gauge charge's key and the assignments
/// of that charge — the assignments themselves, never positions (78.43(1)).
pub(crate) type ChargeRow = (Vec<u8>, Vec<Assignment>);

/// WHY A HUE MAP HAS NO PERIOD ON A BASE, by name (78.43(2)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeriodRefused {
    /// A hue the base carries is not in the map's domain: the orbit ends at
    /// the first step (the traversal's `apply_hue_map` answered `None`).
    OffDomain,
    /// The map names a hue past the sensor's PALETTE, or does not permute
    /// its hues: no orbit of the base returns (`gauge_is_perm`).
    NotAPermutation,
    /// The lcm of the cycle lengths passes the period's carrier.
    PastCarrier,
}

/// THE PERIOD OF A BASE UNDER σ, READ BEFORE THE FIRST REVOLUTION (78.43(2)):
/// the lcm of the lengths of σ's cycles through the hues the base carries,
/// off [`hue_cycles`](crate::transducer::sentence::hue_cycles), the one
/// producer. `MAX_ORBIT` walked the orbit and gave up at 32 — a telescope
/// that stops before the planet returns; the period is set by the cycles the
/// way a planet's return is set by its period, and nothing caps it.
///
/// `u64` by the type's domain: a permutation of the u8 hues has order up to
/// Landau's g(256) ≈ 2⁵⁴. A base of no moving hue returns at once: 1.
pub fn orbit_period(sigma: &[(u8, u8)], base: &[(u8, u8)]) -> Result<u64, PeriodRefused> {
    let listed = |h: u8| sigma.iter().any(|(f, _)| *f == h);
    if !base.iter().all(|&(b, a)| listed(b) && listed(a)) {
        return Err(PeriodRefused::OffDomain);
    }
    if !gauge_is_perm(sigma) {
        return Err(PeriodRefused::NotAPermutation);
    }
    let carried = |c: &[u8]| base.iter().any(|&(b, a)| c.contains(&b) || c.contains(&a));
    let mut period = 1u64;
    for c in crate::transducer::sentence::hue_cycles(sigma).iter().filter(|c| carried(c)) {
        let len = u64::try_from(c.len()).map_err(|_| PeriodRefused::PastCarrier)?;
        let g = num_integer::Integer::gcd(&period, &len);
        period = (period / g).checked_mul(len).ok_or(PeriodRefused::PastCarrier)?;
    }
    Ok(period)
}

/// DOES `section` STAND ON THE BASE'S ORBIT UNDER σ, one of `σ¹(base) …
/// σ^(ℓ−1)(base)` — decided ON THE CYCLES, never by generating the orbit
/// (78.43(2)): the hue map carrying the base onto the section must be one
/// power `σᵗ` on every hue the base carries, `t ≢ 0` modulo the period.
/// Each moving hue gives `t` modulo its cycle's length (its offset along the
/// cycle); the congruences must agree (the congruences' common solution, read one
/// cycle at a time over at most that cycle's length); a fixed hue must stay.
/// `false` where σ has no period on the base.
pub fn on_orbit(sigma: &[(u8, u8)], base: &[(u8, u8)], section: &[(u8, u8)]) -> bool {
    let Ok(period) = orbit_period(sigma, base) else { return false };
    let Some(tau) = derive_hue_map(base, section) else { return false };
    let cycles = crate::transducer::sentence::hue_cycles(sigma);
    // Each hue's cycle and its place on it, read by the hue itself: a table
    // over the u8 hue domain, written once off the cycles.
    let mut places: [Option<(usize, usize)>; 256] = [None; 256];
    for (ci, c) in cycles.iter().enumerate() {
        for (p, &h) in c.iter().enumerate() {
            places[usize::from(h)] = Some((ci, p));
        }
    }
    let place = |h: u8| places[usize::from(h)];
    // `t ≡ r (mod m)`, the congruences met so far: m = 1 is no constraint.
    let (mut r, mut m) = (0u64, 1u64);
    for &(h, image) in &tau {
        match (place(h), place(image)) {
            (None, _) => {
                if image != h {
                    return false;
                }
            }
            (Some((ci, p)), Some((cj, q))) if ci == cj => {
                let Some(len) = cycles.get(ci).map(Vec::len) else { return false };
                let (Ok(len), Ok(offset)) = (u64::try_from(len), u64::try_from((q + len - p) % len)) else {
                    return false;
                };
                // The common solution of `t ≡ r (mod m)` and `t ≡ offset
                // (mod len)`: one of `r + k·m` for `k` below `len`, or none.
                let mut met = None;
                for k in 0..len {
                    let Some(x) = k.checked_mul(m).and_then(|km| km.checked_add(r)) else { return false };
                    if x % len == offset {
                        met = Some(x);
                        break;
                    }
                }
                let Some(x) = met else { return false };
                let g = num_integer::Integer::gcd(&m, &len);
                let Some(lcm) = (m / g).checked_mul(len) else { return false };
                (r, m) = (x % lcm, lcm);
            }
            _ => return false,
        }
    }
    // Every hue the base carries moved as one power `t` (mod m, which is the
    // period: every cycle through a base hue was met); `t ≡ 0` is the base
    // itself, which the orbit's members `σ¹ … σ^(ℓ−1)` do not include.
    r % period != 0
}

/// Derive the hue-map carrying assignment A onto assignment B link by link —
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
    crate::transducer::address::put_key_count(&mut buf, asg.len());
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
    for &link in chain {
        let k = hue_pair_key(link.2, link.3);
        match idx.get_mut(&k) {
            Some(fiber) => fiber.push(link),
            None => {
                idx.insert(k, vec![link]);
            }
        }
    }
    let mut first = None;
    let mut many = false;
    // The index this function just built, keyed by hue pair: the question
    // is whether it holds ONE class or more, so it is read as a whole and
    // the read stops at the second.
    for (k, fiber) in idx.scoped() {
        match first {
            None => first = Some((hue_pair_of(&k)?, fiber.clone())),
            Some(_) => {
                many = true;
                break;
            }
        }
    }
    match (many, first) {
        (false, Some((class, links))) => Some(GaugeFibers::One(class, links)),
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
        let base = self.sub.to_vec();
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
                .filter_map(|(r, c, b, a)| {
                    // `base_pairs` is built from these links, so every
                    // pair is in it; a link whose pair is not is no
                    // variant of this base and is not spelled (rule 11).
                    let i = base_pairs.iter().position(|p| *p == (*b, *a))?;
                    let (nb, na) = *asg.get(i)?;
                    Some((*r, *c, nb, na))
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
    /// the charge's key: none is no such gauge, one is the citation, many is
    /// multiplicity on the fibre. One read; the axis is never read whole.
    ///
    /// THE ASSIGNMENTS THEMSELVES, never the assignments' positions (78.43(1)): a
    /// position in `assignments()` is "the third thing seen", and a reader
    /// that held one had to look the assignment up after. The table holds
    /// each assignment under its charge, and no reader holds a position.
    pub fn assignments_by_profile(&self, key: &[u8]) -> Vec<Vec<(u8, u8)>> {
        let table = self.profiles.get_or_init(|| {
            let mut rows: Vec<ChargeRow> = Vec::new();
            let mut keyed: Vec<(Vec<u8>, Assignment)> =
                self.assignments().into_iter().map(|a| (gauge_key(&a), a)).collect();
            keyed.sort_unstable();
            for (k, a) in keyed {
                match rows.last_mut() {
                    Some((last, held)) if *last == k => held.push(a),
                    _ => rows.push((k, vec![a])),
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
        // A HUE cycle's members, to its period — READ and verified at every
        // writer into `torsion` (PD1), so the enumeration is bounded by a
        // reading, never a stored claim. A holonomy moves no hue: no members.
        for TorsionCycle { generator, period: l, .. } in &self.torsion {
            let Some(sigma) = generator.hue() else { continue };
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
    /// cycle's orbit, decided ON THE CYCLES ([`on_orbit`], 78.43(2)): the
    /// orbit is never generated to ask whether one assignment is on it.
    /// (`matching_assignment`, which answered a position in `assignments()`,
    /// retired with its last reader — 78.43(1).)
    pub fn matches_chain(&self, chain: &[(i64, i64, u8, u8)]) -> bool {
        if self.sub.is_empty() || positions(&self.sub) != positions(chain) {
            return false;
        }
        let asg = assignment(chain);
        let base = assignment(&self.sub);
        if asg == base || self.sections.contains(&asg) {
            return true;
        }
        self.torsion.iter().any(|t| t.generator.hue().is_some_and(|sigma| on_orbit(sigma, &base, &asg)))
    }

}
