// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Verification core for the court binary (#162, three-binary split).
//!
//! The court runs on this module and the shared exact algebra only — no
//! market, no door, no epigenome, no task registry. Replaying a candidate
//! reduction needs the minimal cellular replayer:
//!
//!   replay: (W, ∂₁Q) → (δσ̂ = 0)  ∧  strict ≺_DM descent
//!
//! plus the two-docket ruling over the federated basis and the π*
//! derivation from pooled evidence. Keeping this separate lets the court
//! binary run in constrained validator environments without linking the
//! server's role-specific machinery.

use crate::anastome::dialogue;
use crate::network::nerve::Keyed;
use crate::transducer::anchor::{derive_carrying_all, Sel};
use crate::transducer::cech;
use crate::transducer::element::{self, Element, Versor};
use crate::transducer::stalk::{self, Stalks};
use crate::transducer::toroid::{self, HolonomicLoopWitness};

/// Deterministic re-derivation of a spoken word from its own evidence
/// (the contradiction docket). Returns None when the word replays clean;
/// Some(reason) when the speech was false. A residual word re-derives from
/// its chain (from_residual is deterministic, the chain canonically
/// framed); a CONNECTION re-derives by inversion of its group component,
/// with its remainder chain re-deriving as any germ's; every torsion cycle
/// is walked; and the install must strictly descend (0⃗ ≺_DM μ⃗).
pub fn blind_replay(spoken: &Element) -> Option<&'static str> {
    let chain = spoken.sub();
    // A CONNECTION is a word with two components: the group component (an
    // invertible motion and gauge on the spatial fibre) and the remainder
    // chain. Its chain part re-derives like any germ's; its group part is
    // verified by inversion — a connection the court cannot invert cannot
    // glue and is false speech. Its magnitude carries the volume it speaks
    // across its family on top of the chain's own measure, so it is bounded
    // below by the chain, never equal to it.
    let connection = spoken.is_connection();
    // THE ALPHABET IS THE LAW OF SPEECH: a connection is a word only when
    // its motion is spelled in the seventeen letters and the tape
    // re-derives the block the fibre holds. A tape that disagrees with its
    // fibre is false speech; a connection without a tape was never a word.
    if connection {
        match spoken.letters_rederive() {
            None => return Some("connection has no letters — not a word of the alphabet"),
            Some(false) => return Some("motion does not re-derive from its letters"),
            Some(true) => {}
        }
    }
    match (chain.is_empty(), connection) {
        (true, false) => {
            // An OPENED AXIS with no chain: the germinated fibres must be
            // group elements whose planar block spells, and the word must
            // stand on a spelled base motion.
            let axes = spoken.opened_axes();
            if axes.is_empty() {
                return Some("empty chain — nothing was derived");
            }
            if spoken.solution().is_none() {
                return Some("opened axis is not invertible");
            }
            for axis in axes {
                let Some(m) = spoken.fiber(axis) else { continue };
                if m.len() < 3 {
                    return Some("opened axis has no planar block");
                }
                let block = [[m[0][0].clone(), m[0][1].clone()], [m[1][0].clone(), m[1][1].clone()]];
                if crate::transducer::alphabet::spell(&block).is_none() {
                    return Some("opened axis does not spell in the alphabet");
                }
            }
        }
        (true, true) => {
            if spoken.solution().is_none() {
                return Some("connection is not invertible");
            }
        }
        (false, _) => {
            let min_r = chain.iter().map(|(r, _, _, _)| *r).min().unwrap_or(0);
            let min_c = chain.iter().map(|(_, c, _, _)| *c).min().unwrap_or(0);
            if (min_r, min_c) != (0, 0) {
                return Some("chain not framed to origin");
            }
            if !chain.windows(2).all(|w| w[0] <= w[1]) {
                return Some("chain not canonically ordered");
            }
            let rederived = element::from_residual(&chain.to_vec());
            match connection {
                // The chain lanes (spatial, gauge, shell) re-derive from the
                // chain; the fibre lane is the word's own sections and cycles,
                // which the court counts on the word itself.
                false => {
                    if rederived.address() != spoken.address() {
                        return Some("address does not re-derive from the chain");
                    }
                    if [0, 1, 3].iter().any(|&l| rederived.magnitude()[l] != spoken.magnitude()[l]) {
                        return Some("magnitude does not re-derive from the chain");
                    }
                }
                true => {
                    if rederived.sub() != spoken.sub() {
                        return Some("remainder does not re-derive from the chain");
                    }
                    if [0, 1, 3].iter().any(|&l| spoken.magnitude()[l] < rederived.magnitude()[l]) {
                        return Some("magnitude below the remainder's own measure");
                    }
                    if spoken.solution().is_none() {
                        return Some("connection is not invertible");
                    }
                }
            }
        }
    }
    if !crate::economy::descends(&[0; 4], &spoken.magnitude()) {
        return Some("no strict descent — the germ closes nothing");
    }
    // THE LOOP CHECK on the germinated axis: every torsion cycle the word
    // speaks is walked from its base assignment. A period is believed only
    // as a witnessed closed loop; an open walk names the anholonomity the
    // label was filing over, and a closure at another period is a winding
    // the word misspoke. Replay was Δ = 0 on the chain; this is ∮ dX = 0 on
    // the fibre.
    for (stored, witness) in toroid::witness_stored(spoken) {
        match witness {
            HolonomicLoopWitness::Closed { period, .. } if period == stored => {}
            HolonomicLoopWitness::Closed { .. } => return Some("torsion period does not re-derive — the winding was misspoken"),
            HolonomicLoopWitness::Open { .. } => return Some("torsion cycle does not close — the period was a filing"),
        }
    }
    None
}

/// The deterministic outcome of ruling one candidate word — every arm
/// names what the mathematics found; callers match these states
/// exhaustively rather than branching on booleans or out-parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
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

/// The two-docket ruling for one candidate word against the federated
/// basis — a pure map over the current state; the caller applies
/// [`Ruling::Admitted`] itself. `stalks` is the federation's stalk space
/// (index-aligned with `federation`): the +axis check selects the fiber
/// over the candidate's invariant and runs the exact orbit test there.
pub fn rule(
    federation: &[([u8; 32], Element)],
    index: &Keyed<usize>,
    stalks: &Stalks,
    addr: &[u8; 32],
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
                let third = stalks
                    .fiber(&stalk::invariant_of(spoken))
                    .iter()
                    .filter_map(|a| index.get(a).copied())
                    .find(|&i| federation[i].0 != *addr && !federation[i].1.axes().is_empty());
                match third {
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
            let mut obstruction = match diff.as_slice() {
                [] => Element::identity(spoken.dim()),
                _ => element::from_residual(&diff),
            };
            if let Some(h) = group_view {
                for a in h.axes() {
                    if let Some(m) = h.fiber(a) {
                        obstruction.set_fiber(a, m.clone());
                    }
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
                let representative = stalks
                    .fiber(&stalk::invariant_of(spoken))
                    .iter()
                    .filter_map(|a| index.get(a).copied())
                    .find(|&i| federation[i].0 != *addr && !federation[i].1.axes().is_empty());
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
            let at = |a: &[u8; 32]| index.get(a).and_then(|&i| federation.get(i)).map(|(_, e)| e);
            let congruent = stalk::congruent(stalks, at, spoken).filter(|(a, _)| a != addr);
            match congruent.and_then(|(a, g)| index.get(&a).map(|&base| (base, g))) {
                Some((base, g)) => Ruling::Attached { base, g },
                None => Ruling::Admitted(spoken.clone()),
            }
        }
    }
}

/// The symmetric difference of two canonically ordered chains — one merge
/// walk, never a lookup: the cells one view speaks and the other does not.
fn symmetric_difference(a: &[(i64, i64, u8, u8)], b: &[(i64, i64, u8, u8)]) -> Vec<(i64, i64, u8, u8)> {
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

/// π* derivation from a pooled evidence family: the first anchor term
/// common to every evidence's carrying set (the intersection move — one
/// evidence never binds; the caller holds families of at least two).
pub fn derive_pi(family: &[(Vec<(i64, i64)>, Vec<(i64, i64, u8)>)]) -> Option<Sel> {
    let sets: Vec<Vec<Sel>> =
        family.iter().map(|(fp, scene)| derive_carrying_all(fp, scene)).collect();
    let (first, rest) = sets.split_first()?;
    first.iter().find(|t| rest.iter().all(|s| s.contains(*t))).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::from_residual;
    use crate::transducer::germinate::germinate;

    fn clean_word() -> Element {
        from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap())
    }

    fn stalks_of(fed: &[([u8; 32], Element)]) -> Stalks {
        Stalks::over_elements(fed.iter().map(|(_, e)| e))
    }

    fn index_of(fed: &[([u8; 32], Element)]) -> Keyed<usize> {
        let mut k = Keyed::new();
        for (i, (a, _)) in fed.iter().enumerate() {
            k.insert(*a, i);
        }
        k
    }

    /// A clean word replays; an unparseable body is a named Slashed state.
    #[test]
    fn replay_accepts_clean_and_names_false_speech() {
        let w = clean_word();
        assert!(blind_replay(&w).is_none(), "a clean derivation replays");
        let ruling = rule(&[], &Keyed::new(), &Stalks::default(),&w.address(), None, None);
        assert!(
            matches!(rule(&[], &Keyed::new(), &Stalks::default(),&w.address(), Some(&w), Some(&[0; 4])), Ruling::Slashed(_)),
            "a word that deflated nothing on the book is slashed"
        );
        assert_eq!(ruling, Ruling::Slashed("unparseable word"));
        assert_eq!(ruling.code(), dialogue::RULING_SLASHED);
    }

    /// The two-docket ruling over a federation is an exhaustive state map:
    /// new geometry admits (the element rides the arm), same substance is
    /// consilient, a same-geometry sibling demotes.
    #[test]
    fn the_ruling_maps_the_federation_correctly() {
        let w = clean_word();
        let addr = w.address();
        // Empty federation: new geometry admits — the sealed element is
        // the arm's payload, not a flag.
        match rule(&[], &Keyed::new(), &Stalks::default(),&addr, Some(&w), None) {
            Ruling::Admitted(sealed) => assert_eq!(sealed.address(), addr),
            other => panic!("new geometry must admit, got {other:?}"),
        }
        // Same address, same substance: consilience (no re-admission).
        let fed = vec![(addr, w.clone())];
        let stalks = stalks_of(&fed);
        assert!(matches!(rule(&fed, &index_of(&fed), &stalks,&addr, Some(&w), None), Ruling::Consilient(_)));
        assert_eq!(rule(&fed, &index_of(&fed), &stalks,&addr, Some(&w), None).code(), dialogue::RULING_UPHELD);
        // Different address, identical base geometry: attached (+axis) by
        // the identity transport.
        let mut other = addr;
        other[0] ^= 0xFF;
        match rule(&fed, &index_of(&fed), &stalks,&other, Some(&w), None) {
            Ruling::Attached { base: 0, g } => {
                assert!(element::same_versor(&g, &element::identity_versor()), "the identity transport")
            }
            other => panic!("identical geometry attaches, got {other:?}"),
        }
        // A rotated sibling (the same geometry as a D4 image, other hues)
        // resolves through the chart to the same base with a rotating
        // transport; a genuinely different geometry finds an empty stratum
        // and admits.
        let rotated = from_residual(
            &germinate(&[(0, 0, 3u8), (1, 0, 3)], &[(0, 0, 4u8), (1, 0, 4)]).unwrap(),
        );
        match rule(&fed, &index_of(&fed), &stalks,&rotated.address(), Some(&rotated), None) {
            Ruling::Attached { base: 0, g } => {
                assert!(!element::same_versor(&g, &element::identity_versor()), "the transport rotates")
            }
            other => panic!("a rotated placement attaches, got {other:?}"),
        }
        let bent = from_residual(
            &germinate(&[(0, 0, 1u8), (0, 1, 1), (1, 0, 1)], &[(0, 0, 2u8), (0, 1, 2), (1, 0, 2)])
                .unwrap(),
        );
        assert!(matches!(rule(&fed, &index_of(&fed), &stalks,&bent.address(), Some(&bent), None), Ruling::Admitted(_)));
    }
}
