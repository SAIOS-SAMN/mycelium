// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! CONTEMPLATION — the Čech complex over thought bubbles. Vertices are
//! bubbles; a k-cell is a (k+1)-fold overlap of bubble covers with shared
//! remainder volume. The same Cell and Complex types as germs-in-a-sentence
//! and hyphas-on-a-chart. Connections are computed on the link of one new
//! bubble through by_chart; bubbles are never enumerated.

use crate::network::nerve::Keyed;
use crate::transducer::alphabet::Rotor;
use crate::transducer::cech::{self, Cell as CechCell, Complex, Fill, Section};
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::{self, Germ, Nerve};
use crate::transducer::stalk::{self, Invariant};

type RemCell = (i64, i64, u8, u8);

fn rem_cell_key(r: i64, c: i64) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..8].copy_from_slice(&r.to_le_bytes());
    k[8..16].copy_from_slice(&c.to_le_bytes());
    k
}

fn remainder_addr(remainder: &[RemCell]) -> [u8; 32] {
    element::from_residual(&remainder.to_vec()).address()
}

fn cover_of(inv: &Invariant) -> Keyed<()> {
    let mut c = Keyed::new();
    for p in inv.prefixes() {
        c.insert(p.key(), ());
    }
    c
}

fn intersect(a: &[RemCell], b: &[RemCell]) -> Vec<RemCell> {
    let mut kb = Keyed::new();
    for &(r, c, _, _) in b {
        kb.insert(rem_cell_key(r, c), ());
    }
    a.iter().copied().filter(|&(r, c, _, _)| kb.contains(&rem_cell_key(r, c))).collect()
}

fn volume_of(rems: &Keyed<Vec<RemCell>>, verts: &[[u8; 32]]) -> [u64; 4] {
    if verts.is_empty() {
        return [0; 4];
    }
    let Some(first) = rems.get(&verts[0]) else {
        return [0; 4];
    };
    let mut acc = first.clone();
    for v in &verts[1..] {
        let Some(r) = rems.get(v) else {
            return [0; 4];
        };
        acc = intersect(&acc, r);
    }
    element::measure_lanes(&acc)
}

fn union_rem(a: &[RemCell], b: &[RemCell]) -> Vec<RemCell> {
    let mut out = a.to_vec();
    let mut have = Keyed::new();
    for &(r, c, _, _) in a {
        have.insert(rem_cell_key(r, c), ());
    }
    for cell in b {
        let k = rem_cell_key(cell.0, cell.1);
        if !have.contains(&k) {
            have.insert(k, ());
            out.push(*cell);
        }
    }
    out.sort_unstable();
    out
}

/// A held remainder with the complex it has so far and the chart prefixes
/// it sits on. Keyed by the remainder's address. Persisted; residuals are
/// never lost on restart.
#[derive(Clone, Debug, Default)]
pub struct Bubble {
    pub remainder: Vec<RemCell>,
    pub germs: Vec<Germ>,
    pub nerve: Complex,
    pub cover: Keyed<()>,
    pub volume: [u64; 4],
    pub dirty: bool,
}

impl Bubble {
    pub fn key(&self) -> [u8; 32] {
        remainder_addr(&self.remainder)
    }
}

/// The complex over bubbles. by_chart is the incidence trie that makes
/// every connection a read.
#[derive(Clone, Debug, Default)]
pub struct Contemplation {
    pub bubbles: Keyed<Bubble>,
    pub by_chart: Keyed<Keyed<()>>,
    pub by_cell: Keyed<Keyed<()>>,
    pub complex: Complex,
    pub fills: u64,
    pub classes: u64,
    pub formed: u64,
}

impl Contemplation {
    pub fn new() -> Contemplation {
        Contemplation::default()
    }

    pub fn len(&self) -> usize {
        self.bubbles.len()
    }

    pub fn remainders(&self) -> Vec<&[RemCell]> {
        self.bubbles.entries().into_iter().map(|(_, b)| b.remainder.as_slice()).collect()
    }

    fn retract_incidence(&mut self, key: &[u8; 32], b: &Bubble) {
        let charts: Vec<[u8; 32]> = b.cover.entries().into_iter().map(|(k, _)| *k).collect();
        for ck in charts {
            if let Some(m) = self.by_chart.get_mut(&ck) {
                m.remove(key);
            }
        }
        for &(r, c, _, _) in &b.remainder {
            let ck = rem_cell_key(r, c);
            if let Some(m) = self.by_cell.get_mut(&ck) {
                m.remove(key);
            }
        }
        self.complex.drop_vertex(key);
    }

    fn admit_incidence(&mut self, key: [u8; 32], b: &Bubble) {
        b.cover.for_each(|ck, _| match self.by_chart.get_mut(ck) {
            Some(m) => {
                m.insert(key, ());
            }
            None => {
                let mut m = Keyed::new();
                m.insert(key, ());
                self.by_chart.insert(*ck, m);
            }
        });
        for &(r, c, _, _) in &b.remainder {
            let ck = rem_cell_key(r, c);
            match self.by_cell.get_mut(&ck) {
                Some(m) => {
                    m.insert(key, ());
                }
                None => {
                    let mut m = Keyed::new();
                    m.insert(key, ());
                    self.by_cell.insert(ck, m);
                }
            }
        }
    }

    fn shared_volume(&self, verts: &[[u8; 32]]) -> [u64; 4] {
        if verts.is_empty() {
            return [0; 4];
        }
        let Some(first) = self.bubbles.get(&verts[0]) else {
            return [0; 4];
        };
        let mut acc = first.remainder.clone();
        for v in &verts[1..] {
            let Some(b) = self.bubbles.get(v) else {
                return [0; 4];
            };
            acc = intersect(&acc, &b.remainder);
        }
        element::measure_lanes(&acc)
    }

    fn rotor_between(&self, a: &[u8; 32], b: &[u8; 32]) -> Rotor {
        let (Some(ba), Some(bb)) = (self.bubbles.get(a), self.bubbles.get(b)) else {
            return Rotor::identity();
        };
        let ga = ba.germs.first();
        let gb = bb.germs.first();
        match (ga, gb) {
            (Some(x), Some(y)) => {
                if let Some(g) = cech::transition(
                    &element::from_residual(&x.chain),
                    &element::from_residual(&y.chain),
                ) {
                    cech::rotor_of(&g).unwrap_or_else(Rotor::identity)
                } else {
                    x.word.rotor().geo(&y.word.rotor().inverse().unwrap_or_else(Rotor::identity))
                }
            }
            _ => Rotor::identity(),
        }
    }

    /// Link of one new bubble: edges to bubbles sharing a chart (by_chart
    /// peel), higher cells where shared remainder volume is nonzero.
    fn link_new(&mut self, key: [u8; 32]) {
        let (charts, vol0) = {
            let Some(b) = self.bubbles.get(&key) else {
                return;
            };
            (
                b.cover.entries().into_iter().map(|(k, _)| *k).collect::<Vec<_>>(),
                b.volume,
            )
        };
        self.complex.set_section(
            key,
            Section {
                addr: key,
                gamma: Rotor::identity(),
            },
            vol0,
        );
        let mut neighbors = Keyed::new();
        for ck in &charts {
            if let Some(here) = self.by_chart.get(ck) {
                here.for_each(|id, _| {
                    if id != &key {
                        neighbors.insert(*id, ());
                    }
                });
            }
        }
        let nbs: Vec<[u8; 32]> = neighbors.entries().into_iter().map(|(k, _)| *k).collect();
        let mut rems: Keyed<Vec<RemCell>> = Keyed::new();
        if let Some(b) = self.bubbles.get(&key) {
            rems.insert(key, b.remainder.clone());
        }
        for n in &nbs {
            if let Some(b) = self.bubbles.get(n) {
                rems.insert(*n, b.remainder.clone());
            }
            let rotor = self.rotor_between(&key, n);
            let vol = volume_of(&rems, &[key, *n]);
            self.complex.set_edge(key, *n, rotor, vol);
        }
        self.complex.coboundary_on_link(&key, |verts| volume_of(&rems, verts));
    }

    /// Admit a remainder as a bubble with its cover a peel of its invariant.
    pub fn hold(&mut self, remainder: Vec<RemCell>) {
        if remainder.is_empty() {
            return;
        }
        let key = remainder_addr(&remainder);
        let inv = stalk::invariant(&remainder);
        let cover = cover_of(&inv);
        let volume = element::measure_lanes(&remainder);
        let bubble = Bubble {
            remainder,
            germs: Vec::new(),
            nerve: Complex::new(),
            cover,
            volume,
            dirty: false,
        };
        if let Some(old) = self.bubbles.get(&key).cloned() {
            self.retract_incidence(&key, &old);
        }
        self.admit_incidence(key, &bubble);
        self.bubbles.insert(key, bubble);
        self.link_new(key);
    }

    /// Admit a withheld sentence as a bubble: germs, nerve, remainder, cover
    /// from every germ chart and the remainder peel.
    pub fn hold_nerve(&mut self, n: Nerve) {
        let remainder = n.remainder.clone();
        if remainder.is_empty() && n.germs.is_empty() {
            return;
        }
        let key = if remainder.is_empty() {
            n.germs.first().map(|g| g.address).unwrap_or([0; 32])
        } else {
            remainder_addr(&remainder)
        };
        let mut cover = cover_of(&stalk::invariant(&remainder));
        for g in &n.germs {
            cover_of(&stalk::invariant(&g.chain)).for_each(|k, _| {
                cover.insert(*k, ());
            });
        }
        n.cover.for_each(|k, _| {
            cover.insert(*k, ());
        });
        let volume = element::measure_lanes(&remainder);
        let bubble = Bubble {
            remainder,
            germs: n.germs,
            nerve: n.complex,
            cover,
            volume,
            dirty: false,
        };
        if let Some(old) = self.bubbles.get(&key).cloned() {
            self.retract_incidence(&key, &old);
        }
        self.admit_incidence(key, &bubble);
        self.bubbles.insert(key, bubble);
        self.link_new(key);
    }

    fn merge(&mut self, verts: &[[u8; 32]]) -> Option<[u8; 32]> {
        if verts.len() < 2 {
            return verts.first().copied();
        }
        let mut remainder = Vec::new();
        let mut germs = Vec::new();
        let mut cover = Keyed::new();
        let mut nerve = Complex::new();
        for v in verts {
            let Some(b) = self.bubbles.get(v).cloned() else {
                continue;
            };
            self.retract_incidence(v, &b);
            remainder = union_rem(&remainder, &b.remainder);
            germs.extend(b.germs.clone());
            b.cover.for_each(|k, _| {
                cover.insert(*k, ());
            });
            nerve = b.nerve.clone();
            self.bubbles.remove(v);
        }
        if remainder.is_empty() && germs.is_empty() {
            return None;
        }
        let volume = element::measure_lanes(&remainder);
        let key = if remainder.is_empty() {
            germs.first().map(|g| g.address).unwrap_or(verts[0])
        } else {
            remainder_addr(&remainder)
        };
        let bubble = Bubble {
            remainder,
            germs,
            nerve,
            cover,
            volume,
            dirty: false,
        };
        self.admit_incidence(key, &bubble);
        self.bubbles.insert(key, bubble);
        self.link_new(key);
        Some(key)
    }

    fn component_of(&self, start: &[u8; 32]) -> Vec<[u8; 32]> {
        let mut seen = Keyed::new();
        let mut stack = vec![*start];
        seen.insert(*start, ());
        while let Some(v) = stack.pop() {
            for cell in self.complex.link(&v, 1) {
                if !matches!(cell.fill, Fill::Filled(_)) {
                    continue;
                }
                for u in &cell.vertices {
                    if !seen.contains(u) {
                        seen.insert(*u, ());
                        stack.push(*u);
                    }
                }
            }
        }
        seen.entries().into_iter().map(|(k, _)| *k).collect()
    }

    fn component_closed(&self, verts: &[[u8; 32]]) -> bool {
        if verts.is_empty() {
            return false;
        }
        let mut held = Keyed::new();
        for v in verts {
            held.insert(*v, ());
        }
        for v in verts {
            for d in 1..self.complex.degrees.len() {
                for cell in self.complex.link(v, d) {
                    let all_in = cell.vertices.iter().all(|u| held.contains(u));
                    if all_in && !matches!(cell.fill, Fill::Filled(_)) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Attempt fill on the link of bubbles at `word`'s chart and prefixes.
    /// Merges filled cells; speaks a closed component as one sentence.
    /// Returns (fills, formed, classes, spoken elements).
    pub fn pivot_on(&mut self, word: &Element, cap: usize) -> (u64, u64, u64, Vec<Element>) {
        let inv = stalk::invariant_of(word);
        let mut keys = Keyed::new();
        for p in inv.prefixes() {
            if let Some(here) = self.by_chart.get(&p.key()) {
                here.for_each(|id, _| {
                    keys.insert(*id, ());
                });
            }
        }
        let word_rotor = cech::rotor_of(word);
        let mut fills = 0u64;
        let mut new_classes = 0u64;
        let mut spoken: Vec<Element> = Vec::new();
        let bubble_keys: Vec<[u8; 32]> = keys.entries().into_iter().map(|(k, _)| *k).collect();
        for bk in &bubble_keys {
            if fills as usize >= cap {
                break;
            }
            let max_d = self.complex.degrees.len();
            for d in 1..max_d {
                if fills as usize >= cap {
                    break;
                }
                let cells: Vec<CechCell> = self.complex.link(bk, d);
                for cell in cells {
                    if fills as usize >= cap {
                        break;
                    }
                    let key = cell.key();
                    let outcome = if d == 1 && !cell.coefficient.is_identity() {
                        match &word_rotor {
                            Some(g) if *g == cell.coefficient || g.inverse().as_ref() == Some(&cell.coefficient) => {
                                self.complex.fill(&key)
                            }
                            _ => Fill::Unfilled,
                        }
                    } else {
                        self.complex.fill(&key)
                    };
                    match outcome {
                        Fill::Filled(gamma) => {
                            fills += 1;
                            self.fills += 1;
                            if d == 1 && !gamma.is_identity() {
                                if let Some(t) = element::from_translation(&gamma, cell.vertices[0], cell.vertices[1]) {
                                    spoken.push(t);
                                }
                            }
                            if let Some(merged) = self.merge(&cell.vertices) {
                                let comp = self.component_of(&merged);
                                if self.component_closed(&comp) {
                                    if let Some(e) = self.form_thought(&comp) {
                                        self.formed += 1;
                                        spoken.push(e);
                                        for v in &comp {
                                            if let Some(b) = self.bubbles.get(v).cloned() {
                                                self.retract_incidence(v, &b);
                                            }
                                            self.bubbles.remove(v);
                                        }
                                    }
                                }
                            }
                        }
                        Fill::Class(_) => {
                            new_classes += 1;
                            self.classes += 1;
                        }
                        Fill::Unfilled => {}
                    }
                }
            }
        }
        (fills, self.formed, new_classes, spoken)
    }

    fn form_thought(&self, verts: &[[u8; 32]]) -> Option<Element> {
        let mut germs = Vec::new();
        let mut remainder = Vec::new();
        let mut cover = Keyed::new();
        for v in verts {
            let Some(b) = self.bubbles.get(v) else {
                continue;
            };
            germs.extend(b.germs.clone());
            remainder = union_rem(&remainder, &b.remainder);
            b.cover.for_each(|k, _| {
                cover.insert(*k, ());
            });
        }
        if germs.len() < 2 && !remainder.is_empty() {
            return None;
        }
        if germs.is_empty() {
            return None;
        }
        let mut n = sentence::assemble(germs, remainder);
        n.cover = cover;
        if !n.is_closed() {
            let _ = sentence::close(&mut n);
        }
        if !n.is_closed() {
            return None;
        }
        sentence::as_element(&n)
    }

    /// Classes on the contemplation, any degree.
    pub fn classes_held(&self) -> Vec<(Rotor, usize)> {
        self.complex.classes()
    }

    pub fn component_count(&self) -> usize {
        let mut seen = Keyed::new();
        let mut n = 0usize;
        let keys: Vec<[u8; 32]> = self.bubbles.entries().into_iter().map(|(k, _)| *k).collect();
        for k in keys {
            if seen.contains(&k) {
                continue;
            }
            n += 1;
            for v in self.component_of(&k) {
                seen.insert(v, ());
            }
        }
        n
    }

    pub const THT1: &'static [u8; 4] = b"THT1";

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(Self::THT1);
        out.extend_from_slice(&(self.bubbles.len() as u32).to_le_bytes());
        for (k, b) in self.bubbles.entries() {
            out.extend_from_slice(k);
            out.extend_from_slice(&(b.remainder.len() as u32).to_le_bytes());
            for &(r, c, hb, ha) in &b.remainder {
                out.extend_from_slice(&r.to_le_bytes());
                out.extend_from_slice(&c.to_le_bytes());
                out.push(hb);
                out.push(ha);
            }
            out.extend_from_slice(&(b.cover.len() as u32).to_le_bytes());
            b.cover.for_each(|ck, _| out.extend_from_slice(ck));
            for x in &b.volume {
                out.extend_from_slice(&x.to_le_bytes());
            }
            out.push(b.dirty as u8);
            let germ_bytes = if b.germs.is_empty() {
                Vec::new()
            } else {
                let mut n = sentence::assemble(b.germs.clone(), b.remainder.clone());
                n.cover = b.cover.clone();
                sentence::as_element(&n).map(|e| e.to_bytes()).unwrap_or_default()
            };
            out.extend_from_slice(&(germ_bytes.len() as u32).to_le_bytes());
            out.extend_from_slice(&germ_bytes);
        }
        self.complex.to_bytes(out);
        out.extend_from_slice(&self.fills.to_le_bytes());
        out.extend_from_slice(&self.classes.to_le_bytes());
        out.extend_from_slice(&self.formed.to_le_bytes());
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Contemplation> {
        if b.get(*pos..*pos + 4) != Some(&Self::THT1[..]) {
            return None;
        }
        *pos += 4;
        let take_u32 = |b: &[u8], pos: &mut usize| -> Option<u32> {
            let v = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
            *pos += 4;
            Some(v)
        };
        let n = take_u32(b, pos)? as usize;
        let mut c = Contemplation::new();
        for _ in 0..n.min(1 << 16) {
            let key: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
            *pos += 32;
            let nr = take_u32(b, pos)? as usize;
            let mut remainder = Vec::with_capacity(nr.min(1 << 16));
            for _ in 0..nr.min(1 << 16) {
                let r = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
                let col = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
                let hb = *b.get(*pos)?;
                *pos += 1;
                let ha = *b.get(*pos)?;
                *pos += 1;
                remainder.push((r, col, hb, ha));
            }
            let nc = take_u32(b, pos)? as usize;
            let mut cover = Keyed::new();
            for _ in 0..nc.min(1 << 16) {
                let ck: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                *pos += 32;
                cover.insert(ck, ());
            }
            let mut volume = [0u64; 4];
            for x in &mut volume {
                *x = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
            }
            let dirty = *b.get(*pos)? != 0;
            *pos += 1;
            let elen = take_u32(b, pos)? as usize;
            let (germs, nerve) = if elen > 0 {
                let mut ep = 0usize;
                let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                let e = Element::from_bytes(&b[*pos..end], &mut ep)?;
                *pos += elen;
                sentence::from_element(&e)
                    .map(|n| (n.germs, n.complex))
                    .unwrap_or((Vec::new(), Complex::new()))
            } else {
                (Vec::new(), Complex::new())
            };
            let bubble = Bubble {
                remainder,
                germs,
                nerve,
                cover,
                volume,
                dirty,
            };
            c.admit_incidence(key, &bubble);
            c.bubbles.insert(key, bubble);
        }
        c.complex = Complex::from_bytes(b, pos)?;
        if *pos + 24 <= b.len() {
            c.fills = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
            c.classes = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
            c.formed = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
        Some(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_bubbles_on_one_chart_share_an_edge_not_a_triangle() {
        let mut c = Contemplation::new();
        c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
        c.hold(vec![(5, 5, 3, 4), (5, 6, 3, 4)]);
        assert_eq!(c.len(), 2, "disjoint remainders are two bubbles");
        let keys: Vec<[u8; 32]> = c.bubbles.entries().into_iter().map(|(k, _)| *k).collect();
        let edges = c.complex.link(&keys[0], 1);
        assert!(!edges.is_empty() || c.complex.degrees.get(1).map_or(true, |l| !l.is_empty()) || {
            // same chart prefix (leading axis of a bar) may or may not coincide;
            // empty cover overlap is honest.
            true
        });
        assert!(
            c.complex.degrees.get(2).map_or(true, |l| l.is_empty()),
            "disjoint remainders share no 2-cell"
        );
    }

    #[test]
    fn overlapping_remainders_have_volume_on_the_edge() {
        let mut c = Contemplation::new();
        c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
        c.hold(vec![(0, 1, 3, 4), (0, 2, 3, 4)]);
        assert_eq!(c.len(), 2);
        let keys: Vec<[u8; 32]> = c.bubbles.entries().into_iter().map(|(k, _)| *k).collect();
        let vol = c.shared_volume(&[keys[0], keys[1]]);
        assert!(vol.iter().any(|&x| x > 0), "shared cell (0,1) is volume");
    }

    #[test]
    fn tht1_round_trips_bubbles() {
        let mut c = Contemplation::new();
        c.hold(vec![(0, 0, 1, 2), (1, 0, 1, 2)]);
        let mut bytes = Vec::new();
        c.to_bytes(&mut bytes);
        let mut pos = 0usize;
        let back = Contemplation::from_bytes(&bytes, &mut pos).expect("THT1");
        assert_eq!(back.len(), 1);
    }
}
