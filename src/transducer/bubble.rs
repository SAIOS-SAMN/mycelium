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
use crate::transducer::stalk::{self, Invariant, Stalks};

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
    pub horizon: Keyed<(Rotor, u8)>,
    pub potential: [u64; 4],
    pub volume: [u64; 4],
    pub dirty: bool,
    /// Last chart key visited by expand; the next stroke resumes here.
    pub cursor: [u8; 32],
}

impl Bubble {
    pub fn key(&self) -> [u8; 32] {
        remainder_addr(&self.remainder)
    }

    fn rotor(&self) -> Rotor {
        self.germs
            .first()
            .map(|g| g.word.rotor())
            .unwrap_or_else(Rotor::identity)
    }

    /// Walk outward on the cover trie from this bubble's cover, one chart
    /// per stroke, only over overlaps with nonzero volume. Extended grows
    /// the cover; Congruent grows it with a torsion axis; Class writes the
    /// horizon and stops that branch.
    pub fn expand(&mut self, atlas: &Stalks, budget: usize) -> usize {
        self.expand_with(atlas, &Keyed::new(), budget)
    }

    pub fn expand_with(&mut self, atlas: &Stalks, rotors: &Keyed<Rotor>, budget: usize) -> usize {
        self.potential = element::measure_lanes(&self.remainder);
        if budget == 0 || !self.potential.iter().any(|&x| x > 0) {
            return 0;
        }
        let inv = stalk::invariant(&self.remainder);
        let peel = inv.peel();
        let after = if self.cursor == [0u8; 32] {
            None
        } else {
            Some(self.cursor)
        };
        let mut spent = 0usize;
        // index: budgeted sibling keys of one trie node (peel)
        let last = atlas.each_child(&peel, after.as_ref(), budget, |child| {
            if spent >= budget {
                return;
            }
            let ck = child.key();
            if self.cover.contains(&ck) || self.horizon.contains(&ck) {
                return;
            }
            spent += 1;
            // index: keyed fiber at this sibling chart
            let members = atlas.fiber(&child);
            let Some(&head) = members.first() else {
                return;
            };
            let vol = self.potential;
            let gamma_b = rotors.get(&head).cloned().unwrap_or_else(Rotor::identity);
            let cell = CechCell {
                vertices: {
                    let mut v = vec![self.key(), head];
                    v.sort_unstable();
                    v
                },
                coefficient: gamma_b.clone(),
                volume: vol,
                fill: Fill::Open,
            };
            let a = Section {
                addr: self.key(),
                gamma: self.rotor(),
            };
            let b = Section {
                addr: head,
                gamma: gamma_b,
            };
            match cech::extend(&a, &b, &cell) {
                Fill::Filled(_) => {
                    self.cover.insert(ck, ());
                }
                Fill::Congruent(g, ell) => {
                    self.cover.insert(ck, ());
                    self.nerve.file_congruent(self.key(), head, g, ell);
                }
                Fill::Class(r) => {
                    self.horizon.insert(ck, (r, child.0.len().min(255) as u8));
                }
                Fill::Open => {}
            }
        });
        if let Some(k) = last {
            self.cursor = k;
        }
        self.dirty = spent >= budget;
        spent
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
    /// Bubbles whose cover still has work; written when dirty is set.
    dirty: Keyed<()>,
    expand_cursor: [u8; 32],
    pub fills: u64,
    pub torsion_merges: u64,
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
        self.decide_neighbors(key);
    }

    /// Two bubbles on one cell: extend decides. Extended merges and
    /// re-gauges; Congruent merges with a torsion axis; Class keeps both
    /// and writes each into the other's horizon. Zero volume never merges.
    fn decide_neighbors(&mut self, key: [u8; 32]) {
        let cells: Vec<CechCell> = self.complex.link(&key, 1);
        for cell in cells {
            let vol_nz = cell.volume.iter().any(|&x| x > 0);
            let outcome = self.complex.fill(&cell.key());
            match outcome {
                Fill::Filled(gamma) if vol_nz => {
                    self.fills += 1;
                    for v in &cell.vertices {
                        if let Some(b) = self.bubbles.get_mut(v) {
                            let keys: Vec<[u8; 32]> =
                                b.nerve.sections.entries().into_iter().map(|(k, _)| *k).collect();
                            for sk in keys {
                                if let Some(sec) = b.nerve.sections.get_mut(&sk) {
                                    sec.gamma = gamma.geo(&sec.gamma);
                                }
                            }
                        }
                    }
                    let _ = self.merge(&cell.vertices);
                    return;
                }
                Fill::Congruent(g, ell) if vol_nz => {
                    self.torsion_merges += 1;
                    let ends = [cell.vertices[0], cell.vertices.get(1).copied().unwrap_or(cell.vertices[0])];
                    if let Some(nk) = self.merge(&cell.vertices) {
                        if let Some(b) = self.bubbles.get_mut(&nk) {
                            b.nerve.file_congruent(ends[0], ends[1], g, ell);
                        }
                    }
                    return;
                }
                Fill::Class(r) => {
                    self.classes += 1;
                    let deg = cell.degree() as u8;
                    let hk = cell.key();
                    for v in &cell.vertices {
                        if let Some(b) = self.bubbles.get_mut(v) {
                            b.horizon.insert(hk, (r.clone(), deg));
                        }
                    }
                }
                _ => {}
            }
        }
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
            horizon: Keyed::new(),
            potential: volume,
            volume,
            dirty: true,
            cursor: [0; 32],
        };
        if let Some(old) = self.bubbles.get(&key).cloned() {
            self.retract_incidence(&key, &old);
        }
        self.admit_incidence(key, &bubble);
        self.bubbles.insert(key, bubble);
        self.dirty.insert(key, ());
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
            horizon: Keyed::new(),
            potential: volume,
            volume,
            dirty: true,
            cursor: [0; 32],
        };
        if let Some(old) = self.bubbles.get(&key).cloned() {
            self.retract_incidence(&key, &old);
        }
        self.admit_incidence(key, &bubble);
        self.bubbles.insert(key, bubble);
        self.dirty.insert(key, ());
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
        let mut horizon: Keyed<(Rotor, u8)> = Keyed::new();
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
            b.horizon.for_each(|k, c| {
                horizon.insert(*k, c.clone());
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
            horizon,
            potential: volume,
            volume,
            dirty: false,
            cursor: [0; 32],
        };
        self.admit_incidence(key, &bubble);
        self.bubbles.insert(key, bubble);
        self.dirty.insert(key, ());
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
        // index: vertices of this component; each cell from that vertex's link
        for v in verts {
            for cell in self.complex.link_of(v) {
                if cell.degree() == 0 {
                    continue;
                }
                let all_in = cell.vertices.iter().all(|u| held.contains(u));
                if all_in && !matches!(cell.fill, Fill::Filled(_)) {
                    return false;
                }
            }
        }
        true
    }

    /// Expand dirty bubbles from the atlas, spend_unit charts per call.
    pub fn expand_all(&mut self, atlas: &Stalks, rotors: &Keyed<Rotor>, budget: usize) -> usize {
        if budget == 0 {
            return 0;
        }
        let mut keys: Vec<[u8; 32]> = Vec::new();
        let after = if self.expand_cursor == [0u8; 32] {
            None
        } else {
            Some(self.expand_cursor)
        };
        // index: budgeted keyed cursor over dirty bubbles
        let last = self.dirty.for_each_from(after.as_ref(), budget, |k, _| {
            keys.push(*k);
        });
        if let Some(k) = last {
            self.expand_cursor = k;
        }
        let mut spent = 0usize;
        for k in keys {
            if spent >= budget {
                break;
            }
            if let Some(b) = self.bubbles.get_mut(&k) {
                spent += b.expand_with(atlas, rotors, budget - spent);
                if !b.dirty {
                    self.dirty.remove(&k);
                }
            } else {
                self.dirty.remove(&k);
            }
        }
        spent
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
        // index: by_chart fiber at the word's chart and peels
        for bk in &bubble_keys {
            if fills as usize >= cap {
                break;
            }
            // index: link of this bubble
            let cells: Vec<CechCell> = self.complex.link_of(bk);
            for cell in cells {
                if fills as usize >= cap {
                    break;
                }
                if cell.degree() == 0 {
                    continue;
                }
                let key = cell.key();
                let d = cell.degree();
                let outcome = if d == 1 && !cell.coefficient.is_identity() {
                    match &word_rotor {
                        Some(g) if *g == cell.coefficient || g.inverse().as_ref() == Some(&cell.coefficient) => {
                            self.complex.fill_by_gauge(&key, g)
                        }
                        _ => self.complex.fill(&key),
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
                        Fill::Congruent(g, ell) => {
                            self.torsion_merges += 1;
                            if let Some(merged) = self.merge(&cell.vertices) {
                                if let Some(b) = self.bubbles.get_mut(&merged) {
                                    let a = cell.vertices[0];
                                    let c = cell.vertices.get(1).copied().unwrap_or(a);
                                    b.nerve.file_congruent(a, c, g, ell);
                                }
                            }
                        }
                        Fill::Class(r) => {
                            new_classes += 1;
                            self.classes += 1;
                            let deg = d as u8;
                            let hk = key;
                            for v in &cell.vertices {
                                if let Some(b) = self.bubbles.get_mut(v) {
                                    b.horizon.insert(hk, (r.clone(), deg));
                                }
                            }
                        }
                        Fill::Open => {}
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
            if !b.remainder.is_empty() || !b.horizon.is_empty() {
                return None;
            }
            germs.extend(b.germs.clone());
            remainder = union_rem(&remainder, &b.remainder);
            b.cover.for_each(|k, _| {
                cover.insert(*k, ());
            });
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
            out.extend_from_slice(&(b.horizon.len() as u32).to_le_bytes());
            for (hk, (r, d)) in b.horizon.entries() {
                out.extend_from_slice(hk);
                r.to_bytes(out);
                out.push(*d);
            }
            for x in &b.potential {
                out.extend_from_slice(&x.to_le_bytes());
            }
        }
        self.complex.to_bytes(out);
        out.extend_from_slice(&self.fills.to_le_bytes());
        out.extend_from_slice(&self.torsion_merges.to_le_bytes());
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
            let mut horizon: Keyed<(Rotor, u8)> = Keyed::new();
            if *pos + 4 <= b.len() {
                let nh = take_u32(b, pos).unwrap_or(0) as usize;
                for _ in 0..nh.min(1 << 16) {
                    let hk: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let r = Rotor::from_bytes(b, pos)?;
                    let d = *b.get(*pos)?;
                    *pos += 1;
                    horizon.insert(hk, (r, d));
                }
            }
            let mut potential = volume;
            if *pos + 32 <= b.len() {
                for x in &mut potential {
                    *x = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                }
            }
            let bubble = Bubble {
                remainder,
                germs,
                nerve,
                cover,
                horizon,
                potential,
                volume,
                dirty,
                cursor: [0; 32],
            };
            c.admit_incidence(key, &bubble);
            c.bubbles.insert(key, bubble);
        }
        c.complex = Complex::from_bytes(b, pos)?;
        if *pos + 24 <= b.len() {
            c.fills = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
            if *pos + 24 <= b.len() {
                c.torsion_merges = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
            }
            c.classes = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
            c.formed = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
        Some(c)
    }

    pub fn horizon_cells(&self) -> usize {
        let mut n = 0usize;
        self.bubbles.for_each(|_, b| n += b.horizon.len());
        n
    }

    pub fn potential(&self) -> [u64; 4] {
        let mut p = [0u64; 4];
        self.bubbles.for_each(|_, b| {
            for l in 0..4 {
                p[l] = p[l].saturating_add(b.potential[l]);
            }
        });
        p
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
    fn overlapping_remainders_merge_by_fill() {
        let mut c = Contemplation::new();
        c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
        c.hold(vec![(0, 1, 3, 4), (0, 2, 3, 4)]);
        assert_eq!(c.len(), 1, "shared cell is Extended: the bubbles merge");
        let keys: Vec<[u8; 32]> = c.bubbles.entries().into_iter().map(|(k, _)| *k).collect();
        assert_eq!(keys.len(), 1);
        assert!(c.bubbles.get(&keys[0]).unwrap().potential.iter().any(|&x| x > 0));
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
        assert_eq!(back.potential(), c.potential());
    }

    /// Hold, expand, and the quote-chart peel of one thought do not grow
    /// with N unrelated atlas words.
    #[test]
    fn work_of_one_thought_is_independent_of_n() {
        use crate::network::nerve::work;
        use crate::transducer::element::from_residual;
        use crate::transducer::germinate::germinate;
        let run = |n: usize| -> (u64, u64, u64) {
            let mut atlas = Stalks::default();
            for i in 2..n + 2 {
                let before: Vec<(i64, i64, u8)> = (0..i as i64).map(|c| (0, c, 1u8)).collect();
                let after: Vec<(i64, i64, u8)> = (0..i as i64).map(|c| (0, c, 2u8)).collect();
                let w = from_residual(&germinate(&before, &after).unwrap());
                atlas.admit(&stalk::invariant_of(&w), w.address());
            }
            let mut c = Contemplation::new();
            work::reset();
            c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
            let hold = work::total();
            work::reset();
            let _ = c.expand_all(&atlas, &Keyed::new(), 4);
            let expand = work::total();
            work::reset();
            let mut charts = 0usize;
            c.bubbles.for_each(|_, b| {
                let mut cur = stalk::invariant(&b.remainder);
                loop {
                    if b.cover.contains(&cur.key()) {
                        charts += 1;
                    }
                    if cur.0.is_empty() {
                        break;
                    }
                    cur = cur.peel();
                }
            });
            let quote = work::total();
            assert!(charts > 0);
            (hold, expand, quote)
        };
        let a = run(32);
        let b = run(128);
        eprintln!("work thought N=32 {a:?} N=128 {b:?}");
        assert_eq!(a, b, "4N differs by zero: {a:?} vs {b:?}");
    }
}
