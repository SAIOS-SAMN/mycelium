// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 3.9, M2 — A REWRITE: an optimisation spoken as a word so the court can
//! replay it both ways. The tape names the move, the input addresses, the
//! output address, and the move's own content.

use super::alphabet::{Rotor, Word};
use super::element::{Element, Versor, REWRITE_TAG};
use super::stalk::Invariant;

/// Spatial, gauge, temporal, holonomic (shell).
pub const LANE_SPATIAL: usize = 0;
pub const LANE_GAUGE: usize = 1;
pub const LANE_TEMPORAL: usize = 2;
pub const LANE_SHELL: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewriteKind {
    Condense = 0,
    Torse = 1,
    Refine = 2,
    Reanchor = 3,
    AxisOpen = 4,
    Translation = 5,
    /// Retirement by citation (3.8, chapter 27, T10): a word whose substance
    /// other words reproduce exactly retires, and the language shrinks toward
    /// the geodesic. The structure changes; the result does not, which is why
    /// this is convergence and not a vote.
    Retire = 6,
}

impl RewriteKind {
    pub fn from_u8(v: u8) -> Option<Self> {
        Some(match v {
            0 => RewriteKind::Condense,
            1 => RewriteKind::Torse,
            2 => RewriteKind::Refine,
            3 => RewriteKind::Reanchor,
            4 => RewriteKind::AxisOpen,
            5 => RewriteKind::Translation,
            6 => RewriteKind::Retire,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone)]
pub enum RewritePayload {
    Condense { sections: Vec<Vec<(u8, u8)>>, gauge: Versor },
    /// The cycle's GENERATOR — a hue σ or a holonomy's ray — and the period
    /// it claims, in its own carrier, `u64`; replay re-reads the period off
    /// the generator (PD1, 78.43(2)).
    Torse { generator: crate::transducer::element::Generator, period: u64 },
    Refine { chart: Invariant, axis: u64 },
    Reanchor { gamma: Vec<Word> },
    /// Boxed: a rotor is half a kilobyte inline, the one large variant here
    /// since the versor carrier holds its coefficients on the heap (BQ3).
    AxisOpen { class: Box<Rotor>, degree: u8 },
    Translation { a: crate::transducer::address::Address, b: crate::transducer::address::Address, gauge: Versor },
    /// `cites` are the words that speak the retired one; `freed` is the lane
    /// vector the language no longer carries. The court checks from the tape
    /// alone that a retirement cites something other than itself and frees
    /// something; that the cites actually reproduce the retired substance is
    /// a re-derivation over the language, and it is the kernel's, made
    /// before the mint (`verify_utterance` in `try_refine`).
    Retire { cites: Vec<crate::transducer::address::Address>, freed: [u64; 4] },
}

/// A rewrite names the words it rewrote and the word it reached. Those
/// names are CITATIONS — addresses proven to fit the prefix every tape
/// cites them through — so writing this tape, and the key an element
/// built from it carries, are both total. An address that does not fit is
/// refused at [`mint`], where the element is built (47.1).
#[derive(Debug, Clone)]
pub struct Rewrite {
    pub kind: RewriteKind,
    pub before: Vec<crate::transducer::address::Cited>,
    pub after: crate::transducer::address::Cited,
    pub claimed: [u64; 4],
    pub payload: RewritePayload,
}

impl Rewrite {
    /// `None` where a section address does not fit its citation prefix.
    /// Every COUNT on the tape is written by its own coordinate (STD1 3c-2,
    /// ruled (d)) — the tape is the word's description, keyed whole into
    /// its address — so no count has a field to overflow. The truncating
    /// form wrote the tape short and `from_tape` then read a different
    /// rewrite out of it.
    pub fn to_tape(&self) -> Option<Vec<u8>> {
        use crate::transducer::address::{put_key_bytes, put_key_count};
        let mut out = vec![REWRITE_TAG, self.kind as u8];
        put_key_count(&mut out, self.before.len());
        for a in &self.before {
            crate::transducer::address::put_cited(&mut out, a);
        }
        crate::transducer::address::put_cited(&mut out, &self.after);
        for m in &self.claimed {
            out.extend_from_slice(&m.to_le_bytes());
        }
        match &self.payload {
            RewritePayload::Condense { sections, gauge } => {
                put_key_count(&mut out, sections.len());
                for s in sections {
                    put_key_count(&mut out, s.len());
                    for &(f, t) in s {
                        out.push(f);
                        out.push(t);
                    }
                }
                // Every versor and rotor on this tape as its ray (A4).
                gauge.put_canonical(&mut out);
            }
            RewritePayload::Torse { generator, period } => {
                generator.put(&mut out);
                out.extend_from_slice(&period.to_le_bytes());
            }
            RewritePayload::Refine { chart, axis } => {
                // The chart's own key (law 2), one encoding with the trie.
                put_key_bytes(&mut out, chart.key().as_bytes());
                out.extend_from_slice(&axis.to_le_bytes());
            }
            RewritePayload::Reanchor { gamma } => {
                put_key_count(&mut out, gamma.len());
                for w in gamma {
                    w.put_whole(&mut out);
                }
            }
            RewritePayload::AxisOpen { class, degree } => {
                out.push(*degree);
                class.put_canonical(&mut out);
            }
            RewritePayload::Translation { a, b, gauge } => {
                crate::transducer::address::put_addr(&mut out, a)?;
                crate::transducer::address::put_addr(&mut out, b)?;
                gauge.put_canonical(&mut out);
            }
            RewritePayload::Retire { cites, freed } => {
                put_key_count(&mut out, cites.len());
                for a in cites {
                    crate::transducer::address::put_addr(&mut out, a)?;
                }
                for m in freed {
                    out.extend_from_slice(&m.to_le_bytes());
                }
            }
        }
        Some(out)
    }

    pub fn from_tape(b: &[u8]) -> Option<Self> {
        use crate::transducer::address::{take_key_bytes, take_key_count};
        if crate::transducer::address::tag_of(b) != Some(REWRITE_TAG) {
            return None;
        }
        let kind = RewriteKind::from_u8(*b.get(1)?)?;
        let mut pos = 2usize;
        // A rewrite names WHAT IT REWROTE. One that names nothing rewrote
        // nothing, which is not a rewrite with an empty before — it is a
        // malformed tape, and every reader may then take `n − 1` whole.
        let n = match take_key_count(b, &mut pos)? {
            0 => return None,
            n => n,
        };
        let mut before = Vec::with_capacity(n.min(b.len()));
        for _ in 0..n {
            before.push(crate::transducer::address::take_cited(b, &mut pos)?);
        }
        let after = crate::transducer::address::take_cited(b, &mut pos)?;
        let mut claimed = [0u64; 4];
        for m in &mut claimed {
            *m = u64::from_le_bytes(b.get(pos..pos + 8)?.try_into().ok()?);
            pos += 8;
        }
        let payload = match kind {
            RewriteKind::Condense => {
                let ns = take_key_count(b, &mut pos)?;
                let mut sections = Vec::with_capacity(ns.min(b.len()));
                for _ in 0..ns {
                    let k = take_key_count(b, &mut pos)?;
                    let mut s = Vec::with_capacity(k.min(b.len()));
                    for _ in 0..k {
                        s.push((*b.get(pos)?, *b.get(pos + 1)?));
                        pos += 2;
                    }
                    sections.push(s);
                }
                let gauge = Versor::from_bytes(b, &mut pos)?;
                RewritePayload::Condense { sections, gauge }
            }
            RewriteKind::Torse => {
                let generator = crate::transducer::element::Generator::take(b, &mut pos)?;
                let period = u64::from_le_bytes(b.get(pos..pos + 8)?.try_into().ok()?);
                // A17, as at every other tape: a torsion is believed only
                // where the orbit closes at ℓ, so a filed zero is malformed.
                if period == 0 {
                    return None;
                }
                RewritePayload::Torse { generator, period }
            }
            RewriteKind::Refine => {
                let chart = Invariant::from_key(&take_key_bytes(b, &mut pos)?)?;
                let axis = u64::from_le_bytes(b.get(pos..pos + 8)?.try_into().ok()?);
                RewritePayload::Refine { chart, axis }
            }
            RewriteKind::Reanchor => {
                let n = take_key_count(b, &mut pos)?;
                let mut gamma = Vec::with_capacity(n.min(b.len()));
                for _ in 0..n {
                    gamma.push(Word::take_whole(b, &mut pos)?);
                }
                RewritePayload::Reanchor { gamma }
            }
            RewriteKind::AxisOpen => {
                let degree = *b.get(pos)?;
                pos += 1;
                let class = Box::new(Rotor::from_bytes(b, &mut pos)?);
                RewritePayload::AxisOpen { class, degree }
            }
            RewriteKind::Translation => {
                let a = crate::transducer::address::take_addr(b, &mut pos)?;
                let baddr = crate::transducer::address::take_addr(b, &mut pos)?;
                let gauge = Versor::from_bytes(b, &mut pos)?;
                RewritePayload::Translation { a, b: baddr, gauge }
            }
            RewriteKind::Retire => {
                let n = take_key_count(b, &mut pos)?;
                let mut cites = Vec::with_capacity(n.min(b.len()));
                for _ in 0..n {
                    cites.push(crate::transducer::address::take_addr(b, &mut pos)?);
                }
                let mut freed = [0u64; 4];
                for m in &mut freed {
                    *m = u64::from_le_bytes(b.get(pos..pos + 8)?.try_into().ok()?);
                    pos += 8;
                }
                RewritePayload::Retire { cites, freed }
            }
        };
        Some(Rewrite { kind, before, after, claimed, payload })
    }

    pub fn from_element(e: &Element) -> Option<Self> {
        Self::from_tape(e.description())
    }

    /// Yield re-read from the tape's own content, never from the live basis.
    pub fn yield_from_payload(&self) -> [u64; 4] {
        let mut mu = [0u64; 4];
        match &self.payload {
            RewritePayload::Condense { sections, .. } => {
                mu[LANE_GAUGE] = sections.len() as u64;
                if mu[LANE_GAUGE] == 0 {
                    // Condensing n inputs closes n − 1 joins. A rewrite
                    // names what it rewrote and the tape refuses one that
                    // names nothing, so n ≥ 1 and the subtraction is total.
                    mu[LANE_GAUGE] = self.before.len() as u64 - 1;
                }
            }
            RewritePayload::Torse { period, .. } => {
                // A torsion of period ℓ closes ℓ − 1. A17 believes a period
                // only where the orbit closes at it and the tape refuses a
                // zero, so ℓ ≥ 1 and the subtraction is total.
                mu[LANE_SHELL] = *period - 1;
            }
            RewritePayload::Refine { .. } => {
                mu[LANE_SHELL] = 1;
            }
            RewritePayload::Reanchor { gamma } => {
                mu[LANE_SPATIAL] = gamma.iter().filter(|w| !w.is_identity()).count() as u64;
            }
            RewritePayload::AxisOpen { .. } => {
                mu[LANE_SHELL] = 1;
            }
            RewritePayload::Translation { .. } => {
                mu[LANE_GAUGE] = 2;
            }
            // The compression IS the yield: the description the language no
            // longer carries, on the lanes it occupied. Not a constant.
            RewritePayload::Retire { freed, .. } => mu = *freed,
        }
        mu
    }
}

/// A TORSE'S YIELD off the cycles it is priced by, `Σ(ℓ − 1)` on the shell
/// lane as MU-ECONOMY has it. ℓ ≥ 1 at every filing and every tape (A17), so
/// `ℓ − 1` is total. RM1: a Torse the court mints at a Congruent fill is
/// priced off its own record's cycle, never off `after`'s element.
///
/// The sum in the period's carrier, exact or refused: past it, the zero
/// vector, which the court refuses as it refuses any yield that is not work
/// (rule 11) — never a sum wrapped into a smaller one.
pub fn torse_yield(cycles: &[crate::transducer::element::TorsionCycle]) -> [u64; 4] {
    let mut mu = [0u64; 4];
    let mut shell = 0u64;
    for c in cycles {
        match c.period.checked_sub(1).and_then(|beyond| shell.checked_add(beyond)) {
            Some(s) => shell = s,
            None => return [0; 4],
        }
    }
    mu[LANE_SHELL] = shell;
    mu
}

/// Lane vector read on the carriers. Zero on every lane is not work.
pub fn yield_vector(kind: RewriteKind, before: &[Element], after: &Element) -> [u64; 4] {
    let mut mu = [0u64; 4];
    match kind {
        RewriteKind::Condense => {
            let before_sec: usize = before.iter().map(|e| e.sections().len()).sum();
            // Two readings of one condense: the sections it GAINED, and the
            // joins it CLOSED — n inputs close the n − 1 seams between
            // them, which `split_first` counts without a subtraction, and
            // no input closes none (the count of an empty set). Where the
            // condense gained no section there is no gain to read, and the
            // joins stand alone rather than a zero standing in for one.
            let joins = match before.split_first() {
                Some((_, rest)) => rest.len(),
                None => 0,
            };
            mu[LANE_GAUGE] = after
                .sections()
                .len()
                .checked_sub(before_sec)
                .map_or(joins, |gained| gained.max(joins)) as u64;
        }
        RewriteKind::Torse => mu = torse_yield(after.torsion()),
        RewriteKind::Refine => {
            // The chart the refine started from, and the one it reached.
            // A refine over no input started from no chart, which is a
            // depth of zero by definition (rule 11).
            let ba = match before.split_first() {
                Some((e, _)) => crate::transducer::stalk::invariant_of(e).0.len(),
                None => 0,
            };
            let aa = crate::transducer::stalk::invariant_of(after).0.len();
            mu[LANE_SHELL] = match aa.checked_sub(ba) {
                // Axes GAINED: a refine that reached deeper is worth the
                // axes it opened.
                Some(deeper @ 1..) => deeper as u64,
                // One that reached no deeper still SPLIT one stratum, and
                // the split is the work this measures — the refinement
                // itself, not a floor put under a measurement.
                _ => 1,
            };
        }
        RewriteKind::Reanchor => {
            mu[LANE_SPATIAL] = before.len().max(1) as u64;
        }
        RewriteKind::AxisOpen => {
            mu[LANE_SHELL] = 1;
            // Past its carrier, the zero vector — refused as Retire's is.
            match crate::core::lanes::lanes_add(mu, after.magnitude()) {
                Some(v) => mu = v,
                None => return [0; 4],
            }
        }
        RewriteKind::Translation => {
            mu[LANE_GAUGE] = 2;
        }
        RewriteKind::Retire => {
            // The language shrank by exactly the descriptions it no longer
            // carries. The retired words are disjoint carriers, so their
            // measures accumulate lane-wise (12.6) and never meet. An
            // overflow yields the zero vector, which `settle_rewrite` and
            // `replay` both refuse -- a compression too large to measure is
            // not paid, it is refused (rule 11).
            for e in before {
                match crate::core::lanes::lanes_add(mu, e.magnitude()) {
                    Some(v) => mu = v,
                    None => return [0; 4],
                }
            }
        }
    }
    mu
}

pub fn mint(
    kind: RewriteKind,
    before: &[crate::transducer::address::Address],
    after: crate::transducer::address::Address,
    payload: RewritePayload,
    claimed: [u64; 4],
) -> Option<Element> {
    // 47.1: AN ADDRESS TOO LONG TO CITE IS REFUSED WHERE THE ELEMENT IS
    // BUILT. The rewrite's tape and the key of the element carrying it
    // both cite through a u16 prefix; one that does not fit was dropped
    // silently by the writer and left a tape that did not decode.
    let before: Vec<crate::transducer::address::Cited> =
        before.iter().map(crate::transducer::address::Address::cited).collect::<Option<_>>()?;
    let mut rw = Rewrite { kind, before, after: after.cited()?, claimed, payload };
    let y = rw.yield_from_payload();
    if y.iter().all(|&x| x == 0) {
        return None;
    }
    rw.claimed = y;
    Some(Element::from_rewrite_tape(rw.to_tape()?, y))
}

/// Blind replay of a Rewrite: both directions, then the yield re-read.
pub fn replay(e: &Element) -> Option<&'static str> {
    // A TAPE THAT DOES NOT DECODE IS A REFUSAL, not an absence of one. This
    // read `Rewrite::from_element(e)?`, and `?` on a function whose `None`
    // MEANS "no refusal" turned an undecodable tape into a clean bill: the
    // court replayed a rewrite it could not read and reported nothing
    // against it. Every other arm below refuses on a property of the decoded
    // rewrite, so the one input none of them could examine was the one input
    // that passed.
    let Some(rw) = Rewrite::from_element(e) else {
        return Some("rewrite tape does not decode");
    };
    let y = rw.yield_from_payload();
    if y.iter().all(|&x| x == 0) {
        return Some("rewrite is not work");
    }
    if y != rw.claimed {
        return Some("rewrite yield does not re-read on the carriers");
    }
    match &rw.payload {
        RewritePayload::Condense { sections, gauge } => {
            if sections.is_empty() && rw.before.len() < 2 {
                return Some("condense has no absorbed sections");
            }
            if !crate::transducer::cga::is_versor(gauge) {
                return Some("condense gauge is not invertible");
            }
        }
        RewritePayload::Torse { generator, period } => {
            if *period == 0 {
                return Some("torse does not close under its period");
            }
            // A Torse closes at ITS period (A17): the generator's order
            // exactly, read off it — never a multiple, which the tape would
            // be paid for (PD1). `closes_at` of either kind.
            if generator.order() != Some(*period) {
                return Some("torse period is not its generator's order");
            }
        }
        RewritePayload::Refine { chart, axis } => {
            let refined = crate::transducer::stalk::Stalks::refine_axis(chart, *axis);
            if crate::transducer::stalk::Stalks::peel(&refined) != *chart {
                return Some("refine does not invert by peel");
            }
        }
        RewritePayload::Reanchor { gamma } => {
            for w in gamma {
                if w.inverse().compose(w).is_identity() || w.compose(&w.inverse()).is_identity() {
                    continue;
                }
                if !w.is_identity() && w.inverse().letters().is_empty() && !w.letters().is_empty() {
                    return Some("re-anchor gauge does not invert");
                }
            }
        }
        RewritePayload::AxisOpen { class, .. } => {
            if class.is_identity() {
                return Some("axis open has no class");
            }
        }
        RewritePayload::Translation { a, b, gauge } => {
            if a == b {
                return Some("translation sections coincide");
            }
            if !crate::transducer::cga::is_versor(gauge) {
                return Some("translation gauge is not invertible");
            }
        }
        RewritePayload::Retire { cites, .. } => {
            // A word retires BECAUSE something else speaks it. No cite is a
            // deletion, and a word cited as its own proof proves nothing.
            if cites.is_empty() {
                return Some("retirement cites nothing that speaks it");
            }
            if cites.iter().any(|c| rw.before.iter().any(|b| b.addr() == c)) {
                return Some("retirement cites the word it retires");
            }
            // The result is unchanged: a retirement moves the structure and
            // not the meaning (3.8, T10), so the word it names as `after` is
            // the word it retires. A retirement naming a different vertex is
            // claiming a change of meaning it did not make.
            if !rw.before.contains(&rw.after) {
                return Some("retirement changes the vertex it names");
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 47.1 AT THE CONSTRUCTOR: an address too long to cite is refused
    /// where the element is built. The writer used to drop the slot and
    /// leave a tape that did not decode, and the key of the element
    /// carrying it fell through to the form tag alone — every such element
    /// at one address, which is law 2 broken at its root.
    #[test]
    fn mint_refuses_address_too_long_to_cite() {
        let long = crate::transducer::address::Address::new(vec![0u8; u16::MAX as usize + 1]);
        let short = crate::transducer::address::Address::from_bytes([7u8]);
        assert!(long.cited().is_none(), "it does not fit the prefix");
        assert!(short.cited().is_some(), "and an address that fits does");
        let payload = || RewritePayload::Retire { cites: vec![short.clone()], freed: [1, 0, 0, 0] };
        assert!(
            mint(
                RewriteKind::Retire,
                std::slice::from_ref(&long),
                short.clone(),
                payload(),
                [1, 0, 0, 0],
            )
            .is_none(),
            "a rewrite that cannot name what it rewrote is not minted"
        );
        assert!(
            mint(RewriteKind::Retire, std::slice::from_ref(&short), long, payload(), [1, 0, 0, 0])
                .is_none(),
            "nor one that cannot name what it reached"
        );
    }

    /// PD1's return — A TORSE CLOSES AT ITS PERIOD, its generator's order
    /// exactly: a Torse at twice the order would be paid twice over, and is
    /// refused on replay by name; at the order, for either kind, it mints and
    /// replays green (the court's Congruent fill mints the holonomy kind).
    #[test]
    fn replay_refuses_torse_at_twice_generator_order() {
        use crate::transducer::address::Address;
        use crate::transducer::element::Generator;
        let q = crate::transducer::alphabet::LetterId::turn_90((0, 1)).expect("the sensor plane");
        let turn_90 = crate::transducer::alphabet::Word::from_letters(&[(q, false)]).rotor();
        let swap = Generator::Hue(vec![(1, 2), (2, 1)]);
        let turn = Generator::Holonomy(Box::new(turn_90));
        for (generator, order) in [(swap, 2u64), (turn, 4)] {
            let torse = |period: u64| {
                let payload = RewritePayload::Torse { generator: generator.clone(), period };
                let mu = [0, 0, 0, period - 1];
                mint(RewriteKind::Torse, &[Address::from_bytes([1u8])], Address::from_bytes([2u8]), payload, mu)
                    .expect("a Torse that yields is minted")
            };
            assert!(replay(&torse(order)).is_none(), "{generator:?} at its order replays green");
            assert_eq!(
                replay(&torse(2 * order)),
                Some("torse period is not its generator's order"),
                "{generator:?} at twice its order is refused by name"
            );
        }
    }

    /// A condense rewrite round-trips through its tape and replays both ways.
    #[test]
    fn mint_condense_round_trips_and_replays() {
        let after = crate::transducer::address::Address::from_bytes([7u8]);
        let payload = RewritePayload::Condense {
            sections: vec![vec![(1, 4)]],
            gauge: crate::transducer::element::identity_versor(),
        };
        let e = mint(
            RewriteKind::Condense,
            &[
                crate::transducer::address::Address::from_bytes([1u8]),
                crate::transducer::address::Address::from_bytes([2u8]),
            ],
            after,
            payload,
            [0, 1, 0, 0],
        )
        .expect("work");
        assert!(e.is_rewrite());
        assert!(replay(&e).is_none(), "both ways re-derive");
        let back = Rewrite::from_element(&e).unwrap();
        assert_eq!(back.claimed[LANE_GAUGE], 1);
        assert_eq!(back.before.len(), 2);
    }

    /// G6. Retirement by citation is the compression axis: the structure
    /// moves toward the geodesic and the result does not move at all (3.8,
    /// T10). The yield is the description the language stopped carrying, on
    /// the lanes it occupied — never a constant, which is what the `Refine`
    /// arm this replaces paid (`yield_from_payload` hardcoded 1 on the shell
    /// lane and `yield_vector` carried `.max(1)` so the two would agree).
    #[test]
    fn mint_retire_pays_freed_lanes() {
        use crate::transducer::address::Address;
        let word = Address::from_bytes([1u8]);
        let speaker = Address::from_bytes([2u8]);
        let freed = [3u64, 0, 0, 5];
        let e = mint(
            RewriteKind::Retire,
            std::slice::from_ref(&word),
            word.clone(),
            RewritePayload::Retire { cites: vec![speaker.clone()], freed },
            freed,
        )
        .expect("a retirement that frees something is work");
        assert!(replay(&e).is_none(), "it re-derives from its own tape");
        let back = Rewrite::from_element(&e).expect("round trip");
        assert_eq!(back.claimed, freed, "the yield is the measure, not a constant");
        assert_eq!(back.after.addr(), &word, "the vertex it names is the one it retires");
        match back.payload {
            RewritePayload::Retire { ref cites, freed: f } => {
                assert_eq!(cites, &vec![speaker], "the cites survive the tape");
                assert_eq!(f, freed);
            }
            _ => panic!("the tape decodes to the move it encoded"),
        }
    }

    /// The three things a retirement cannot be, checkable from the tape
    /// alone. That the cites genuinely reproduce the retired substance is a
    /// re-derivation over the language and is the kernel's, made before the
    /// mint; the court checks here what the tape can prove.
    #[test]
    fn replay_refuses_retirement_proving_nothing() {
        use crate::transducer::address::Address;
        let word = Address::from_bytes([1u8]);
        let other = Address::from_bytes([2u8]);
        let freed = [3u64, 0, 0, 0];
        let mk = |cites: Vec<Address>, after: Address| {
            mint(
                RewriteKind::Retire,
                std::slice::from_ref(&word),
                after,
                RewritePayload::Retire { cites, freed },
                freed,
            )
            .map(|e| replay(&e))
        };
        assert_eq!(
            mk(Vec::new(), word.clone()),
            Some(Some("retirement cites nothing that speaks it")),
            "a deletion is not a retirement"
        );
        assert_eq!(
            mk(vec![word.clone()], word.clone()),
            Some(Some("retirement cites the word it retires")),
            "a word is not its own proof"
        );
        assert_eq!(
            mk(vec![other.clone()], other),
            Some(Some("retirement changes the vertex it names")),
            "the meaning does not move"
        );
        assert!(
            mint(
                RewriteKind::Retire,
                std::slice::from_ref(&word),
                word.clone(),
                RewritePayload::Retire { cites: vec![Address::from_bytes([2u8])], freed: [0; 4] },
                [0; 4],
            )
            .is_none(),
            "a retirement that frees nothing is not work"
        );
    }

    /// A rewrite whose yield is zero on every lane is not work and is not minted.
    #[test]
    fn mint_refuses_zero_yield() {
        assert!(mint(
            RewriteKind::Condense,
            &[crate::transducer::address::Address::from_bytes([1u8])],
            crate::transducer::address::Address::from_bytes([2u8]),
            RewritePayload::Condense { sections: vec![], gauge: crate::transducer::element::identity_versor() },
            [0; 4],
        )
        .is_none());
    }
}
