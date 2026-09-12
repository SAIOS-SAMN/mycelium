// 42 — THE RULING: what a court decides about a word, and the difference
// between two chains that decides it.

use super::*;

/// The deterministic outcome of ruling one candidate word — every arm
/// names what the mathematics found; callers match these states
/// exhaustively rather than branching on booleans or out-parameters.
#[derive(Debug, Clone)]
pub enum Ruling {
    /// The word does not replay from its own evidence (or does not parse):
    /// false speech, the failed axis named.
    Slashed(&'static str),
    /// The federated view and the spoken view are one sheaf: the Čech
    /// obstruction on their overlap has no axis and no chain — the identity
    /// element rides the arm. Cross-instance consilience, the strongest
    /// confluence witness.
    Consilient(Element),
    /// Two views of one object that do not glue — a dialect, recorded and
    /// never punished. The obstruction rides the arm: the holonomy element
    /// (a group view-change, or the triangle's composite) when the views
    /// are group elements; the symmetric difference of the chains, as a
    /// residual word, when the overlap is only the additive coboundary.
    Dialect(Element),
    /// New address, a placement of a federated base geometry — the bundle
    /// connects: `base` indexes the federation entry and `g` is the
    /// transport carrying the word into the base frame. The word attaches as
    /// a section of that base, never a sibling entry.
    Attached { base: usize, g: Versor },
    /// Genuinely new geometry: the word joins the federation. The sealed
    /// element rides the arm — admission IS the match, never a flag.
    Admitted(Element),
}

impl PartialEq for Ruling {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Ruling::Slashed(a), Ruling::Slashed(b)) => a == b,
            (Ruling::Consilient(a), Ruling::Consilient(b))
            | (Ruling::Dialect(a), Ruling::Dialect(b))
            | (Ruling::Admitted(a), Ruling::Admitted(b)) => a.same_vertex(b),
            (
                Ruling::Attached { base: b1, g: g1 },
                Ruling::Attached { base: b2, g: g2 },
            ) => b1 == b2 && element::same_versor(g1, g2),
            _ => false,
        }
    }
}
impl Eq for Ruling {}

impl Ruling {
    /// The wire ruling byte this outcome seals and speaks.
    pub fn code(&self) -> u8 {
        match self {
            Ruling::Slashed(_) => dialogue::RULING_SLASHED,
            Ruling::Attached { .. } => dialogue::RULING_ATTACHED,
            Ruling::Consilient(_) | Ruling::Dialect(_) | Ruling::Admitted(_) => dialogue::RULING_UPHELD,
        }
    }

    /// The named ground of the ruling (receipt/log text).
    pub fn reason(&self) -> &'static str {
        match self {
            Ruling::Slashed(r) => r,
            Ruling::Consilient(_) => "federated consilience: the overlap glues",
            Ruling::Dialect(_) => "dialect: the obstruction on the overlap is recorded",
            Ruling::Attached { .. } => "+axis: attached as a section of the base",
            Ruling::Admitted(_) => "extends the tautology",
        }
    }
}

/// The unique other word on the spoken chart's fiber. Multiplicity keeps
/// every sibling (returns None) rather than picking one.
pub(crate) fn unique_other_on_fiber(
    stalks: &Stalks,
    index: &Keyed<usize>,
    federation: &[(crate::transducer::address::Address, Element)],
    spoken: &Element,
    skip: &[u8],
) -> Option<usize> {
    let mut named = None;
    for a in stalks.fiber(&stalk::invariant_of(spoken)) {
        let Some(&i) = index.get(&a) else {
            continue;
        };
        if federation[i].0.as_bytes() == skip || federation[i].1.axes().is_empty() {
            continue;
        }
        if named.replace(i).is_some() {
            return None;
        }
    }
    named
}

/// The two-docket ruling for one candidate word against the federated
/// basis — a pure map over the current state; the caller applies
/// [`Ruling::Admitted`] itself. `stalks` is the federation's stalk space
/// (index-aligned with `federation`): the +axis check selects the fiber
/// over the candidate's invariant and runs the exact orbit test there.
pub fn rule(
    federation: &[(crate::transducer::address::Address, Element)],
    index: &Keyed<usize>,
    stalks: &Stalks,
    addr: &[u8],
    spoken: Option<&Element>,
    deflation: Option<&[u64; 4]>,
) -> Ruling {
    let Some(spoken) = spoken else {
        return Ruling::Slashed("unparseable word");
    };
    // PROOF OF CONVERGENCE: a word is ratified for what it settled. Two
    // voices installed it, and the install measured the deflation of a
    // listed demand; a word that deflated nothing on the book is speech
    // about nothing the mesh asked for, and is slashed. A word audited
    // from the genesis chain carries no settlement measurement (None) and
    // is judged on its form and congruence alone.
    if deflation.is_some_and(|d| d.iter().all(|&x| x == 0)) {
        return Ruling::Slashed("no deflation on a listed demand");
    }
    if let Some(reason) = blind_replay(spoken) {
        return Ruling::Slashed(reason);
    }
    match index.get(addr).and_then(|&i| federation.get(i)) {
        Some((_, held)) => {
            // ČECH ON THE OVERLAP: the federated view and the spoken view
            // of one object are two charts. When the held view is a group
            // element, the change of view g = spoken ∘ held⁻¹ is the
            // transition on their overlap; a third germ on the same stalk
            // fibre closes the triangle and the composite holonomy H is the
            // obstruction. Identity glues (one sheaf: the views are the
            // coboundary of one assignment); a non-identity element is the
            // dialect, carried on the arm and never punished. A held view
            // with no group axis has only the additive coboundary to
            // compare: the chains themselves.
            // ONE OBSTRUCTION WITH TWO COMPONENTS, on one fibre: the additive
            // coboundary (the symmetric difference of the two chains) and
            // the group transition (spoken ∘ held⁻¹, closed through a third
            // germ on the fibre when one stands there) are measured together
            // for the same pair and composed into one element — the chain
            // is its base, the transition its axes.
            let diff = symmetric_difference(held.sub(), spoken.sub());
            let group_view = match held.axes().as_slice() {
                [] => None,
                _ => cech::transition(held, spoken),
            };
            let group_view = group_view.map(|g| {
                match unique_other_on_fiber(stalks, index, federation, spoken, addr) {
                    Some(i) => {
                        let z = &federation[i].1;
                        match (cech::transition(spoken, z), cech::transition(z, held)) {
                            (Some(g_sz), Some(g_zh)) => cech::holonomy(&g, &g_sz, &g_zh),
                            _ => g,
                        }
                    }
                    None => g,
                }
            });
            // A difference with no A5 form is no obstruction to name: the
            // identity stands where there is nothing to spell, and a
            // conflicting difference is not a chain this can speak.
            let mut obstruction = match diff.as_slice() {
                [] => Element::identity(),
                _ => match element::from_residual(&diff) {
                    Some(e) => e,
                    None => return Ruling::Slashed("obstruction chain has no canonical form"),
                },
            };
            if let Some(h) = group_view {
                if let Some(v) = h.versor() {
                    obstruction.open_axis(v);
                }
                for (_, v) in h.germinated() {
                    obstruction.open_axis(v.clone());
                }
            }
            // δg = 0 read on the element itself: no axis and no chain is
            // the identity, one sheaf; anything else is the dialect.
            match (obstruction.axes().as_slice(), obstruction.sub()) {
                ([], []) => Ruling::Consilient(obstruction),
                _ => Ruling::Dialect(obstruction),
            }
        }
        None => {
            // The +axis constraint evaluates on the ORBIT [Q]/(ℤ² ⋊ D4):
            // a support that is a translation, rotation, or reflection of a
            // federated word's is the same base geometry — a section of the
            // existing thought, never a sibling entry. Coordinate-set
            // equality was the lens that let a handful of morphisms
            // federate as a thousand words. The chart over the candidate's
            // invariant holds every base point it could sit on; the group
            // element decides inside that stratum and rides the ruling as
            // the transport — no federation scan, no discarded connection.
            // A CONNECTION at a new address is heard against the fibre's
            // representative: the transition vanishing is the same word
            // (consilient), a non-identity transition is a dialect carried
            // on the arm, and an empty fibre admits new geometry. Never an
            // empty-support Attached.
            if spoken.is_connection() {
                let representative = unique_other_on_fiber(stalks, index, federation, spoken, addr);
                return match representative.map(|i| (i, cech::transition(&federation[i].1, spoken))) {
                    None | Some((_, None)) => Ruling::Admitted(spoken.clone()),
                    Some((base, Some(g))) => match (g.axes().as_slice(), g.sub()) {
                        ([], []) => Ruling::Consilient(g),
                        // The same motion placed elsewhere attaches as a
                        // section of the base, the translation its versor.
                        _ => match stalk::placement_of(&g) {
                            Some(v) => Ruling::Attached { base, g: v },
                            None => Ruling::Dialect(g),
                        },
                    },
                };
            }
            let at = |a: &[u8]| index.get(a).and_then(|&i| federation.get(i)).map(|(_, e)| e);
            let congruent = stalk::congruent(stalks, at, spoken).filter(|(a, _)| a.as_bytes() != addr);
            match congruent.and_then(|(a, g)| index.get(&a).map(|&base| (base, g))) {
                Some((base, g)) => Ruling::Attached { base, g },
                None => Ruling::Admitted(spoken.clone()),
            }
        }
    }
}

/// The symmetric difference of two canonically ordered chains — one merge
/// walk, never a lookup: the cells one view speaks and the other does not.
pub(crate) fn symmetric_difference(a: &[(i64, i64, u8, u8)], b: &[(i64, i64, u8, u8)]) -> Vec<(i64, i64, u8, u8)> {
    let (mut i, mut j) = (0usize, 0usize);
    let mut out = Vec::new();
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => {
                out.push(a[i]);
                i += 1;
            }
            std::cmp::Ordering::Greater => {
                out.push(b[j]);
                j += 1;
            }
            std::cmp::Ordering::Equal => {
                i += 1;
                j += 1;
            }
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}
