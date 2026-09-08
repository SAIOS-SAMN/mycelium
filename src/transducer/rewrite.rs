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
    Torse { sigma: Vec<(u8, u8)>, period: u32 },
    Refine { chart: Invariant, axis: u64 },
    Reanchor { gamma: Vec<Word> },
    AxisOpen { class: Rotor, degree: u8 },
    Translation { a: crate::transducer::address::Address, b: crate::transducer::address::Address, gauge: Versor },
    /// `cites` are the words that speak the retired one; `freed` is the lane
    /// vector the language no longer carries. The court checks from the tape
    /// alone that a retirement cites something other than itself and frees
    /// something; that the cites actually reproduce the retired substance is
    /// a re-derivation over the language, and it is the kernel's, made
    /// before the mint (`verify_utterance` in `try_refine`).
    Retire { cites: Vec<crate::transducer::address::Address>, freed: [u64; 4] },
}

#[derive(Debug, Clone)]
pub struct Rewrite {
    pub kind: RewriteKind,
    pub before: Vec<crate::transducer::address::Address>,
    pub after: crate::transducer::address::Address,
    pub claimed: [u64; 4],
    pub payload: RewritePayload,
}

impl Rewrite {
    pub fn to_tape(&self) -> Vec<u8> {
        let mut out = vec![REWRITE_TAG, self.kind as u8];
        out.push(self.before.len().min(255) as u8);
        for a in self.before.iter().take(255) {
            crate::transducer::address::put_addr(&mut out, a);
        }
        crate::transducer::address::put_addr(&mut out, &self.after);
        for m in &self.claimed {
            out.extend_from_slice(&m.to_le_bytes());
        }
        match &self.payload {
            RewritePayload::Condense { sections, gauge } => {
                out.extend_from_slice(&(sections.len() as u16).to_le_bytes());
                for s in sections {
                    out.extend_from_slice(&(s.len() as u16).to_le_bytes());
                    for &(f, t) in s {
                        out.push(f);
                        out.push(t);
                    }
                }
                gauge.to_bytes(&mut out);
            }
            RewritePayload::Torse { sigma, period } => {
                out.extend_from_slice(&(sigma.len() as u16).to_le_bytes());
                for &(f, t) in sigma {
                    out.push(f);
                    out.push(t);
                }
                out.extend_from_slice(&period.to_le_bytes());
            }
            RewritePayload::Refine { chart, axis } => {
                out.push(chart.0.len().min(255) as u8);
                for a in chart.0.iter().take(255) {
                    out.extend_from_slice(&a.to_le_bytes());
                }
                out.extend_from_slice(&axis.to_le_bytes());
            }
            RewritePayload::Reanchor { gamma } => {
                out.extend_from_slice(&(gamma.len() as u16).to_le_bytes());
                for w in gamma {
                    w.to_bytes(&mut out);
                }
            }
            RewritePayload::AxisOpen { class, degree } => {
                out.push(*degree);
                class.to_bytes(&mut out);
            }
            RewritePayload::Translation { a, b, gauge } => {
                crate::transducer::address::put_addr(&mut out, a);
                crate::transducer::address::put_addr(&mut out, b);
                gauge.to_bytes(&mut out);
            }
            RewritePayload::Retire { cites, freed } => {
                out.extend_from_slice(&(cites.len() as u16).to_le_bytes());
                for a in cites {
                    crate::transducer::address::put_addr(&mut out, a);
                }
                for m in freed {
                    out.extend_from_slice(&m.to_le_bytes());
                }
            }
        }
        out
    }

    pub fn from_tape(b: &[u8]) -> Option<Self> {
        if b.first() != Some(&REWRITE_TAG) {
            return None;
        }
        let kind = RewriteKind::from_u8(*b.get(1)?)?;
        let n = *b.get(2)? as usize;
        let mut pos = 3usize;
        let mut before = Vec::with_capacity(n);
        for _ in 0..n {
            before.push(crate::transducer::address::take_addr(b, &mut pos)?);
        }
        let after = crate::transducer::address::take_addr(b, &mut pos)?;
        let mut claimed = [0u64; 4];
        for m in &mut claimed {
            *m = u64::from_le_bytes(b.get(pos..pos + 8)?.try_into().ok()?);
            pos += 8;
        }
        let payload = match kind {
            RewriteKind::Condense => {
                let ns = u16::from_le_bytes(b.get(pos..pos + 2)?.try_into().ok()?) as usize;
                pos += 2;
                let mut sections = Vec::with_capacity(ns);
                for _ in 0..ns {
                    let k = u16::from_le_bytes(b.get(pos..pos + 2)?.try_into().ok()?) as usize;
                    pos += 2;
                    let mut s = Vec::with_capacity(k);
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
                let ns = u16::from_le_bytes(b.get(pos..pos + 2)?.try_into().ok()?) as usize;
                pos += 2;
                let mut sigma = Vec::with_capacity(ns);
                for _ in 0..ns {
                    sigma.push((*b.get(pos)?, *b.get(pos + 1)?));
                    pos += 2;
                }
                let period = u32::from_le_bytes(b.get(pos..pos + 4)?.try_into().ok()?);
                RewritePayload::Torse { sigma, period }
            }
            RewriteKind::Refine => {
                let nax = *b.get(pos)? as usize;
                pos += 1;
                let mut axes = Vec::with_capacity(nax);
                for _ in 0..nax {
                    axes.push(u64::from_le_bytes(b.get(pos..pos + 8)?.try_into().ok()?));
                    pos += 8;
                }
                let axis = u64::from_le_bytes(b.get(pos..pos + 8)?.try_into().ok()?);
                RewritePayload::Refine { chart: Invariant(axes), axis }
            }
            RewriteKind::Reanchor => {
                let n = u16::from_le_bytes(b.get(pos..pos + 2)?.try_into().ok()?) as usize;
                pos += 2;
                let mut gamma = Vec::with_capacity(n);
                for _ in 0..n {
                    gamma.push(Word::from_bytes(b, &mut pos)?);
                }
                RewritePayload::Reanchor { gamma }
            }
            RewriteKind::AxisOpen => {
                let degree = *b.get(pos)?;
                pos += 1;
                let class = Rotor::from_bytes(b, &mut pos)?;
                RewritePayload::AxisOpen { class, degree }
            }
            RewriteKind::Translation => {
                let a = crate::transducer::address::take_addr(b, &mut pos)?;
                let baddr = crate::transducer::address::take_addr(b, &mut pos)?;
                let gauge = Versor::from_bytes(b, &mut pos)?;
                RewritePayload::Translation { a, b: baddr, gauge }
            }
            RewriteKind::Retire => {
                let n = u16::from_le_bytes(b.get(pos..pos + 2)?.try_into().ok()?) as usize;
                pos += 2;
                let mut cites = Vec::with_capacity(n);
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
                    mu[LANE_GAUGE] = self.before.len().saturating_sub(1) as u64;
                }
            }
            RewritePayload::Torse { period, .. } => {
                mu[LANE_SHELL] = (*period as u64).saturating_sub(1);
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

/// Lane vector read on the carriers. Zero on every lane is not work.
pub fn yield_vector(kind: RewriteKind, before: &[Element], after: &Element) -> [u64; 4] {
    let mut mu = [0u64; 4];
    match kind {
        RewriteKind::Condense => {
            let before_sec: usize = before.iter().map(|e| e.sections().len()).sum();
            let gained = after.sections().len().saturating_sub(before_sec);
            mu[LANE_GAUGE] = gained.max(before.len().saturating_sub(1)) as u64;
        }
        RewriteKind::Torse => {
            mu[LANE_SHELL] = after
                .torsion()
                .iter()
                .map(|c| (c.period as u64).saturating_sub(1))
                .sum();
        }
        RewriteKind::Refine => {
            let ba = before.first().map(|e| crate::transducer::stalk::invariant_of(e).0.len()).unwrap_or(0);
            let aa = crate::transducer::stalk::invariant_of(after).0.len();
            mu[LANE_SHELL] = aa.saturating_sub(ba).max(1) as u64;
        }
        RewriteKind::Reanchor => {
            mu[LANE_SPATIAL] = before.len().max(1) as u64;
        }
        RewriteKind::AxisOpen => {
            mu[LANE_SHELL] = 1;
            let cv = after.magnitude();
            for l in 0..4 {
                mu[l] = mu[l].saturating_add(cv[l]);
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
                let m = e.magnitude();
                for l in 0..4 {
                    match mu[l].checked_add(m[l]) {
                        Some(v) => mu[l] = v,
                        None => return [0; 4],
                    }
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
    let mut rw = Rewrite { kind, before: before.to_vec(), after, claimed, payload };
    let y = rw.yield_from_payload();
    if y.iter().all(|&x| x == 0) {
        return None;
    }
    rw.claimed = y;
    Some(Element::from_rewrite_tape(rw.to_tape(), y))
}

/// Blind replay of a Rewrite: both directions, then the yield re-read.
pub fn replay(e: &Element) -> Option<&'static str> {
    let rw = Rewrite::from_element(e)?;
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
            if gauge.versor_inverse().is_none() {
                return Some("condense gauge is not invertible");
            }
        }
        RewritePayload::Torse { sigma, period } => {
            if *period == 0 || !hue_closed(sigma, *period) {
                return Some("torse does not close under its period");
            }
        }
        RewritePayload::Refine { chart, axis } => {
            let mut key = [0u8; 32];
            key[..8].copy_from_slice(&axis.to_le_bytes());
            let refined = crate::transducer::stalk::Stalks::refine(chart, key);
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
            if gauge.versor_inverse().is_none() {
                return Some("translation gauge is not invertible");
            }
        }
        RewritePayload::Retire { cites, .. } => {
            // A word retires BECAUSE something else speaks it. No cite is a
            // deletion, and a word cited as its own proof proves nothing.
            if cites.is_empty() {
                return Some("retirement cites nothing that speaks it");
            }
            if cites.iter().any(|c| rw.before.iter().any(|b| b == c)) {
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

fn hue_closed(sigma: &[(u8, u8)], period: u32) -> bool {
    if period == 0 {
        return false;
    }
    let apply = |h: u8| -> u8 { sigma.iter().find(|(f, _)| *f == h).map(|(_, t)| *t).unwrap_or(h) };
    for &(f, _) in sigma {
        let mut h = f;
        for _ in 0..period {
            h = apply(h);
        }
        if h != f {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_condense_rewrite_round_trips_and_replays() {
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
    fn a_retirement_pays_what_the_language_stopped_carrying() {
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
        assert_eq!(back.after, word, "the vertex it names is the one it retires");
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
    fn a_retirement_that_proves_nothing_is_refused() {
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

    #[test]
    fn a_zero_vector_is_not_work() {
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
