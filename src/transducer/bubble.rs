// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! CONTEMPLATION — the Čech complex over thought bubbles. Vertices are
//! bubbles; a k-cell is a (k+1)-fold overlap of bubble covers with shared
//! remainder volume. The same Cell and Complex types as germs-in-a-sentence
//! and hyphas-on-a-chart. Connections are computed on the link of one new
//! bubble through by_chart; bubbles are never enumerated.

use crate::network::nerve::{Keyed, ROOT};
use crate::physics::Budget;
use crate::transducer::alphabet::{Order, Rotor};
use crate::transducer::cech::{self, Cell as CechCell, Complex, Fill, Section};
use crate::transducer::element::{self, Element};
use crate::transducer::family::Crystal;
use crate::transducer::sentence::{self, Form, PivotCertificate, Vertex};
use crate::transducer::stalk::{self, Invariant, Standing, Stalks};

/// One class the mesh or a mind holds on a chart: key, order, representative.
#[derive(Clone, Debug)]
pub struct ClassRecord {
    pub key: crate::transducer::a21::Address,
    pub order: Order,
    pub representative: Rotor,
}

fn horizon_put(h: &mut Keyed<(Rotor, Order, Standing)>, r: Rotor, standing: Standing) {
    let Some(order) = r.word_order() else {
        return;
    };
    let Some(key) = r.class_key() else {
        return;
    };
    h.insert(key, (r, order, standing));
}

type RemCell = (i64, i64, u8, u8);

fn rem_cell_key(r: i64, c: i64) -> crate::transducer::a21::Address {
    let mut buf = vec![crate::transducer::a21::TAG_CELL];
    buf.extend_from_slice(&r.to_le_bytes());
    buf.extend_from_slice(&c.to_le_bytes());
    crate::transducer::a21::Address::new(buf)
}

fn remainder_addr(remainder: &[RemCell]) -> crate::transducer::a21::Address {
    element::from_residual(&remainder.to_vec()).address()
}

/// The idea's key: the form's vertex key. Remainder is cargo, not the name.
fn idea_key(f: &Form) -> crate::transducer::a21::Address {
    sentence::form_vertex_key(f)
}

fn form_mag(f: &Form) -> [u64; 4] {
    let mut m = element::measure_lanes(&f.remainder);
    for v in &f.vertices {
        if let Vertex::Glued { magnitude, .. } = v {
            for i in 0..4 {
                m[i] = m[i].saturating_add(magnitude[i]);
            }
        }
    }
    m
}

/// 13.12: two forms are one section when a coboundary on their join coheres.
fn form_one_section(a: &Form, b: &Form) -> bool {
    if idea_key(a) == idea_key(b) {
        return true;
    }
    let mut f = a.clone();
    let off = f.vertices.len() as u16;
    if off == 0 {
        return b.coheres();
    }
    for v in b.vertices.iter().skip(1) {
        f.vertices.push(v.clone());
    }
    for face in &b.faces {
        let i = if face.i == 0 { 0 } else { face.i.saturating_add(off.saturating_sub(1)) };
        let j = if face.j == 0 { 0 } else { face.j.saturating_add(off.saturating_sub(1)) };
        f.faces.push(sentence::FormFace {
            i,
            j,
            transport: face.transport.clone(),
        });
    }
    f.remainder = union_rem(&a.remainder, &b.remainder);
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    match f.solve_coboundary() {
        Some(gamma) => {
            let _ = f.regauge(&gamma);
            f.coheres()
        }
        None => false,
    }
}

fn absorb_keyed<V: Clone>(dst: &mut Keyed<V>, src: &Keyed<V>) {
    for (k, v) in src.fiber(ROOT) {
        dst.insert(&k, v.clone());
    }
}

fn absorb_vortex(
    dst: &mut Keyed<Keyed<sentence::PivotCertificate>>,
    src: &Keyed<Keyed<sentence::PivotCertificate>>,
) {
    for (after, inner) in src.fiber(ROOT) {
        match dst.get_mut(&after) {
            Some(d) => absorb_keyed(d, inner),
            None => {
                dst.insert(after, inner.clone());
            }
        }
    }
}

fn rem_cells_of(b: &Bubble) -> Vec<RemCell> {
    let mut out = b.remainder.clone();
    for (_, r) in b.remainders.fiber(ROOT) {
        out.extend_from_slice(r);
    }
    out
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

fn volume_of(rems: &Keyed<Vec<RemCell>>, verts: &[impl AsRef<[u8]>]) -> [u64; 4] {
    if verts.is_empty() {
        return [0; 4];
    }
    let Some(first) = rems.get(verts[0].as_ref()) else {
        return [0; 4];
    };
    let mut acc = first.clone();
    for v in &verts[1..] {
        let Some(r) = rems.get(v.as_ref()) else {
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
    pub nerve: Complex,
    pub cover: Keyed<()>,
    /// Classes this idea has met, keyed by [`Rotor::class_key`]. Standing is
    /// the coordinate, never a boolean.
    pub horizon: Keyed<(Rotor, Order, Standing)>,
    pub potential: [u64; 4],
    pub volume: [u64; 4],
    pub dirty: bool,
    /// Last chart key visited by expand; the next stroke resumes here.
    pub cursor: Vec<u8>,
    /// TUniversal when this bubble's charts last moved.
    pub moved_tu: u64,
    /// Addresses this idea glued (crystallized forms it rests on).
    pub glues: Keyed<()>,
    /// Ideas that glued this bubble; a non-empty set is not shed.
    pub glued_by: Keyed<()>,
    /// Filed torsion `(σ, ℓ, chord)` keyed by class, written from `pivot_loop`.
    pub torsion: Keyed<(Vec<(u8, u8)>, u32, u16)>,
    /// Certified discharges this idea's crystallization rests on (A20).
    pub certificates: Keyed<PivotCertificate>,
    /// Incomparable discharges held as cells (25.1) until a glue or ruling.
    /// Outer key is the after class; inner is the discharging word's M1 key.
    pub vortex: Keyed<Keyed<PivotCertificate>>,
    /// The form this idea is (chapter 30 step 6).
    pub form: Option<Form>,
    /// Pairs the idea germinated from; trial speaks over them for μ⃗.
    pub bundle: Option<Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)>>,
    /// Per-pair remainder chains, keyed by pair index. `remainder` is their union.
    pub remainders: Keyed<Vec<RemCell>>,
}

impl Bubble {
    pub fn key(&self) -> crate::transducer::a21::Address {
        match &self.form {
            Some(f) => idea_key(f),
            None => remainder_addr(&self.remainder),
        }
    }

    fn rotor(&self) -> Rotor {
        self.form
            .as_ref()
            .and_then(|f| {
                f.vertices.iter().find_map(|v| match v {
                    Vertex::Word { word, .. } => Some(word.rotor()),
                    _ => None,
                })
            })
            .unwrap_or_else(Rotor::identity)
    }

    /// Walk outward on the cover trie from this bubble's cover, one chart
    /// per stroke, only over overlaps with nonzero volume. Extended grows
    /// the cover; Congruent grows it with a torsion axis; Class writes the
    /// horizon and stops that branch.
    pub fn expand(&mut self, atlas: &Stalks, crystal: &dyn Crystal, budget: &mut Budget) -> usize {
        self.expand_with(atlas, &Keyed::new(), 0, crystal, budget).0
    }

    pub fn expand_with(
        &mut self,
        atlas: &Stalks,
        rotors: &Keyed<Rotor>,
        now: u64,
        crystal: &dyn Crystal,
        budget: &mut Budget,
    ) -> (usize, [u64; 4]) {
        self.potential = element::measure_lanes(&self.remainder);
        self.potential[2] = now.saturating_sub(self.moved_tu).max(1);
        if budget.remaining() == 0 || !self.potential.iter().any(|&x| x > 0) {
            return (0, [0; 4]);
        }
        let inv = stalk::invariant(&self.remainder);
        let peel = inv.peel();
        let cursor = self.cursor.clone();
        let after = if cursor.is_empty() {
            None
        } else {
            Some(cursor.as_slice())
        };
        let mut spent = 0usize;
        // index: budgeted sibling keys of one trie node (peel)
        let last = atlas.each_child(&peel, after, budget, |child| {
            let ck = child.key();
            if self.cover.contains(&ck) {
                return;
            }
            spent += 1;
            // index: keyed fiber at this sibling chart
            let members = atlas.fiber(&child);
            let Some(head) = members.first() else {
                return;
            };
            let vol = self.potential;
            let gamma_b = rotors.get(head.as_slice()).cloned().unwrap_or_else(Rotor::identity);
            let cell = CechCell {
                vertices: {
                    let mut v = vec![self.key().into_vec(), head.clone()];
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
                addr: crate::transducer::a21::Address::from(head.clone()),
                gamma: gamma_b,
            };
            match cech::extend(&a, &b, &cell) {
                Fill::Filled(_) => {
                    self.cover.insert(ck, ());
                    self.moved_tu = now;
                }
                Fill::Congruent(g, ell) => {
                    self.cover.insert(ck, ());
                    self.moved_tu = now;
                    self.nerve.file_congruent(self.key(), head, g, ell);
                }
                Fill::Class(r) => {
                    if r.class_key().is_none_or(|ck| !self.horizon.contains(ck.as_bytes())) {
                        horizon_put(&mut self.horizon, r, Standing::Held);
                    }
                }
                Fill::Open | Fill::Already => {}
            }
        });
        if let Some(k) = last {
            self.cursor = k;
        }
        let mut pivoted = [0u64; 4];
        if budget.remaining() > 0 {
            if let Some(mut form) = self.form.clone() {
                let key = self.key();
                let t = now.saturating_sub(self.moved_tu);
                let snaps = form.pivot_loop(
                    key,
                    &self.horizon,
                    crystal,
                    budget,
                    self.bundle.as_deref(),
                    t,
                );
                for s in &snaps {
                    let m = s.lanes();
                    for i in 0..4 {
                        pivoted[i] = pivoted[i].saturating_add(m[i]);
                    }
                }
                self.take_form(form);
            }
        }
        self.dirty = spent > 0 && budget.remaining() == 0;
        (spent, pivoted)
    }

    /// A glue or a syllabus class whose key equals one held certificate's
    /// `after` applies the certificate keyed by that rotor's A1 key; the
    /// others of the class are not applied (dialect cells).
    fn collapse_vortex(&mut self, class: &[u8], word: &[u8], by: &'static str) -> bool {
        let Some(mut inner) = self.vortex.remove(class) else {
            return false;
        };
        let Some(win) = inner.remove(word) else {
            if !inner.is_empty() {
                self.vortex.insert(class, inner);
            }
            return false;
        };
        if let Some(mut f) = self.form.clone() {
            f.apply_pivot(&win);
            f.certificates.insert(win.key(), win.clone());
            f.vortex = Keyed::new();
            self.take_form(f);
        } else {
            self.certificates.insert(win.key(), win.clone());
            self.vortex = Keyed::new();
        }
        let axis = win.axis;
        eprintln!(
            "  vortex {:02x}{:02x}… collapsed by {by} to axis {axis:?}",
            self.key()[0],
            self.key()[1]
        );
        true
    }

    fn take_form(&mut self, f: Form) {
        self.remainder = f.remainder.clone();
        self.torsion = f.torsion.clone();
        self.certificates = f.certificates.clone();
        self.vortex = f.vortex.clone();
        self.form = Some(f);
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
    expand_cursor: Vec<u8>,
    pub fills: u64,
    pub torsion_merges: u64,
    pub classes: u64,
    pub formed: u64,
    /// Bubbles retired to a cavity chart, address kept, remainder in THT1.
    cavity: Keyed<Vec<RemCell>>,
    pending_rewrites: Vec<Element>,
    /// TUniversal the entity last stamped; T-room and potential[2] read it.
    now_tu: u64,
    /// Mesh-seeded classes, keyed by class key, Standing::Held. Not persisted.
    seeded: Keyed<(Rotor, Order, Standing)>,
}

impl Contemplation {
    pub fn new() -> Contemplation {
        Contemplation::default()
    }

    pub fn set_now(&mut self, tu: u64) {
        self.now_tu = tu;
    }

    /// When T-room on the entity's reserve is exhausted, the bubble with
    /// the largest T magnitude (ticks since its charts last moved) goes to
    /// a cavity chart; pivot_on reads it back when its chart moves.
    pub fn retire_on_t(&mut self, room: usize) {
        if self.bubbles.len() <= room.max(1) {
            return;
        }
        let now = self.now_tu;
        let mut best: Option<(Vec<u8>, u64)> = None;
        for (k, b) in self.bubbles.fiber(ROOT) {
            if b.glued_by.len() > 0 {
                continue;
            }
            let t = now.saturating_sub(b.moved_tu).max(1);
            match best {
                Some((_, bt)) if t <= bt => {}
                _ => best = Some((k.clone(), t)),
            }
        }
        let Some((k, _)) = best else {
            return;
        };
        if let Some(b) = self.bubbles.remove(&k) {
            self.retract_incidence(&k, &b);
            self.dirty.remove(&k);
            self.cavity.insert(k, b.remainder);
        }
    }

    pub fn len(&self) -> usize {
        self.bubbles.len()
    }

    /// True when a live idea glued this address (a dialect entry so marked
    /// is not shed).
    pub fn glues_word(&self, addr: &[u8]) -> bool {
        self.bubbles
            .fiber(ROOT)
            .any(|(_, b)| b.glues.contains(addr))
    }

    /// Admit syllabus class records as Held. Copies into every live bubble.
    pub fn seed_horizon(&mut self, records: &[ClassRecord]) -> usize {
        let mut n = 0usize;
        for rec in records {
            if self.seeded.contains(&rec.key) {
                continue;
            }
            self.seeded.insert(
                rec.key.clone(),
                (rec.representative.clone(), rec.order, Standing::Held),
            );
            n += 1;
        }
        let keys: Vec<Vec<u8>> = self.bubbles.fiber(ROOT).map(|(k, _)| k).collect();
        for k in keys {
            if let Some(b) = self.bubbles.get_mut(&k) {
                for rec in records {
                    if !b.horizon.contains(&rec.key) {
                        b.horizon.insert(
                            rec.key.clone(),
                            (rec.representative.clone(), rec.order, Standing::Held),
                        );
                    }
                    b.collapse_vortex(
                        &rec.key,
                        &rec.representative.key(),
                        "ruling",
                    );
                }
            }
        }
        n
    }

    fn paint_seeded(&self, horizon: &mut Keyed<(Rotor, Order, Standing)>) {
        for (k, v) in self.seeded.fiber(ROOT) {
            if !horizon.contains(&k) {
                horizon.insert(&k, v.clone());
            }
        }
    }

    /// Record that idea `bubble` glued crystallized form `word`.
    pub fn mark_glue(&mut self, bubble: &[u8], word: impl AsRef<[u8]>) {
        if let Some(b) = self.bubbles.get_mut(bubble) {
            b.glues.insert(word, ());
        }
    }

    /// Shed ideas no live glue holds. Returns how many bubbles left.
    pub fn shed_unglued(&mut self) -> usize {
        let drop: Vec<Vec<u8>> = self
            .bubbles
            .fiber(ROOT)
            .filter(|(_, b)| b.glued_by.len() == 0 && b.glues.len() == 0 && !b.dirty)
            .map(|(k, _)| k)
            .collect();
        let n = drop.len();
        for k in &drop {
            if let Some(b) = self.bubbles.remove(k) {
                self.retract_incidence(k, &b);
                self.dirty.remove(k);
            }
        }
        n
    }

    /// Per-lane congruent mass on each chart: ideas that share a class.
    /// Compared per lane, never an argmax across lanes.
    pub fn cohesion(&self) -> crate::network::nerve::Keyed<[u64; 4]> {
        let mut out: crate::network::nerve::Keyed<[u64; 4]> = crate::network::nerve::Keyed::new();
        // index: ideas keyed by remainder, then by chart cover
        for (_, b) in self.bubbles.fiber(ROOT) {
            for (ck, _) in b.cover.fiber(ROOT) {
                let add = b.volume;
                match out.get_mut(&ck) {
                    Some(v) => {
                        for l in 0..4 {
                            v[l] = v[l].saturating_add(add[l]);
                        }
                    }
                    None => {
                        out.insert(&ck, add);
                    }
                }
            }
        }
        out
    }

    pub fn cavity_len(&self) -> usize {
        self.cavity.len()
    }

    pub fn remainders(&self) -> Vec<&[RemCell]> {
        self.bubbles.fiber(ROOT).map(|(_, b)| b.remainder.as_slice()).collect()
    }

    /// Unreduced boundary cochain norm of open thoughts: L1 of the
    /// four-lane measure of the held remainders. An unmeasured space is
    /// an open thought (∂g ≠ 0), not infinite discordance.
    pub fn boundary_norm(&self) -> usize {
        let mut acc: Vec<RemCell> = Vec::new();
        for r in self.remainders() {
            acc.extend(r.iter().copied());
        }
        acc.sort_unstable();
        acc.dedup();
        element::measure_lanes(&acc)
            .iter()
            .fold(0usize, |s, &x| s.saturating_add(x as usize))
    }

    fn retract_incidence(&mut self, key: &[u8], b: &Bubble) {
        let charts: Vec<Vec<u8>> = b.cover.fiber(ROOT).map(|(k, _)| k).collect();
        for ck in charts {
            if let Some(m) = self.by_chart.get_mut(&ck) {
                m.remove(key);
            }
        }
        for (r, c, _, _) in rem_cells_of(b) {
            let ck = rem_cell_key(r, c);
            if let Some(m) = self.by_cell.get_mut(&ck) {
                m.remove(key);
            }
        }
        self.complex.drop_vertex(key);
    }

    fn admit_incidence(&mut self, key: impl AsRef<[u8]>, b: &Bubble) {
        let key = key.as_ref();
        for (ck, _) in b.cover.fiber(ROOT) {
            match self.by_chart.get_mut(&ck) {
                Some(m) => {
                    m.insert(key, ());
                }
                None => {
                    let mut m = Keyed::new();
                    m.insert(key, ());
                    self.by_chart.insert(&ck, m);
                }
            }
        }
        for (r, c, _, _) in rem_cells_of(b) {
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

    fn rotor_between(&self, a: &[u8], b: &[u8]) -> Rotor {
        let (Some(ba), Some(bb)) = (self.bubbles.get(a), self.bubbles.get(b)) else {
            return Rotor::identity();
        };
        let ra = ba.rotor();
        let rb = bb.rotor();
        rb.geo(&ra.inverse().unwrap_or_else(Rotor::identity))
    }

    /// Link of one new bubble: edges to bubbles sharing a chart (by_chart
    /// peel), higher cells where shared remainder volume is nonzero.
    fn link_new(&mut self, key: impl AsRef<[u8]>) {
        let key = key.as_ref().to_vec();
        let (charts, vol0) = {
            let Some(b) = self.bubbles.get(&key) else {
                return;
            };
            (
                b.cover.fiber(ROOT).map(|(k, _)| k).collect::<Vec<_>>(),
                b.volume,
            )
        };
        self.complex.set_section(
            &key,
            Section {
                addr: crate::transducer::a21::Address::from(key.clone()),
                gamma: Rotor::identity(),
            },
            vol0,
        );
        let mut neighbors = Keyed::new();
        for ck in &charts {
            for (id, _) in self.by_chart.link(ck) {
                if id.as_slice() != key.as_slice() {
                    neighbors.insert(&id, ());
                }
            }
        }
        let nbs: Vec<Vec<u8>> = neighbors.fiber(ROOT).map(|(k, _)| k).collect();
        let mut rems: Keyed<Vec<RemCell>> = Keyed::new();
        if let Some(b) = self.bubbles.get(&key) {
            rems.insert(&key, b.remainder.clone());
        }
        for n in &nbs {
            if let Some(b) = self.bubbles.get(n) {
                rems.insert(n, b.remainder.clone());
            }
            let rotor = self.rotor_between(&key, n);
            let vol = volume_of(&rems, &[key.as_slice(), n.as_slice()]);
            self.complex.set_edge(&key, n, rotor, vol);
        }
        self.complex.coboundary_on_link(&key, |verts| volume_of(&rems, verts));
        self.decide_neighbors(&key);
    }

    /// Two bubbles on one cell: extend decides. Extended merges and
    /// re-gauges; Congruent merges with a torsion axis; Class keeps both
    /// and writes each into the other's horizon. Zero volume never merges.
    fn decide_neighbors(&mut self, key: &[u8]) {
        let keys = self.complex.link(&key, 1);
        for ck in keys {
            let Some((fill, volume, vertices)) = self.complex.cell(&ck).map(|c| (c.fill.clone(), c.volume, c.vertices.clone())) else {
                continue;
            };
            if !matches!(fill, Fill::Open) {
                continue;
            }
            let vol_nz = volume.iter().any(|&x| x > 0);
            let outcome = self.complex.fill(&ck);
            match outcome {
                Fill::Filled(gamma) if vol_nz => {
                    self.fills += 1;
                    for v in &vertices {
                        if let Some(b) = self.bubbles.get_mut(v) {
                            let keys: Vec<Vec<u8>> =
                                b.nerve.sections.fiber(ROOT).map(|(k, _)| k).collect();
                            for sk in keys {
                                if let Some(sec) = b.nerve.sections.get_mut(&sk) {
                                    sec.gamma = gamma.geo(&sec.gamma);
                                }
                            }
                        }
                    }
                    let _ = self.merge(&vertices);
                    return;
                }
                Fill::Congruent(g, ell) if vol_nz => {
                    self.torsion_merges += 1;
                    let ends = [
                        vertices[0].clone(),
                        vertices.get(1).cloned().unwrap_or_else(|| vertices[0].clone()),
                    ];
                    if let Some(nk) = self.merge(&vertices) {
                        if let Some(b) = self.bubbles.get_mut(nk.as_bytes()) {
                            b.nerve.file_congruent(&ends[0], &ends[1], g, ell);
                        }
                        let before: Vec<crate::transducer::a21::Address> = vertices
                            .iter()
                            .map(|v| crate::transducer::a21::Address::from(v.clone()))
                            .collect();
                        let payload = crate::transducer::rewrite::RewritePayload::Torse {
                            sigma: Vec::new(),
                            period: ell,
                        };
                        if let Some(rw) = crate::transducer::rewrite::mint(
                            crate::transducer::rewrite::RewriteKind::Torse,
                            &before,
                            nk.clone(),
                            payload,
                            [0, 0, (ell as u64).saturating_sub(1).max(1), 0],
                        ) {
                            self.pending_rewrites.push(rw);
                        }
                    }
                    return;
                }
                Fill::Already | Fill::Open => {}
                Fill::Filled(_) | Fill::Congruent(_, _) => {}
                Fill::Class(r) => {
                    self.classes += 1;
                    for v in &vertices {
                        if let Some(b) = self.bubbles.get_mut(v) {
                            if r.class_key().is_none_or(|ck| !b.horizon.contains(ck.as_bytes())) {
                                horizon_put(&mut b.horizon, r.clone(), Standing::Held);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Hold a form as an idea (chapter 30 step 5): the form is the carrier.
    /// The train is the germinating bundle; remainders are per-pair residual
    /// chains. idea_key is the form's vertex key.
    pub fn hold_form(
        &mut self,
        f: &Form,
        train: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)],
        remainders: &[Vec<RemCell>],
    ) {
        if f.vertices.len() <= 1
            && f.faces.is_empty()
            && f.remainder.is_empty()
            && remainders.iter().all(|r| r.is_empty())
        {
            return;
        }
        let key = idea_key(f);
        let mut rem_keyed: Keyed<Vec<RemCell>> = Keyed::new();
        let mut volume = [0u64; 4];
        let mut cover = Keyed::new();
        for (i, r) in remainders.iter().enumerate() {
            rem_keyed.insert(crate::transducer::family::pair_idx_key(i as u16), r.clone());
            if r.is_empty() {
                continue;
            }
            let m = element::measure_lanes(r);
            for l in 0..4 {
                volume[l] = volume[l].saturating_add(m[l]);
            }
            for (ck, _) in cover_of(&stalk::invariant(r)).fiber(ROOT) {
                cover.insert(&ck, ());
            }
        }
        if !f.remainder.is_empty() {
            for (ck, _) in cover_of(&stalk::invariant(&f.remainder)).fiber(ROOT) {
                cover.insert(&ck, ());
            }
        }
        let mut glues = Keyed::new();
        for v in &f.vertices {
            match v {
                Vertex::Word { class, .. } => {
                    cover.insert(class.key(), ());
                }
                Vertex::Glued { address, chart, .. } => {
                    cover.insert(chart.key(), ());
                    glues.insert(address, ());
                }
                _ => {}
            }
        }
        let mut horizon = Keyed::new();
        self.paint_seeded(&mut horizon);
        let certs = f.certificates.clone();
        let bundle = if train.is_empty() {
            None
        } else {
            Some(train.to_vec())
        };
        let bubble = Bubble {
            remainder: f.remainder.clone(),
            nerve: Complex::new(),
            cover,
            horizon,
            potential: volume,
            volume,
            dirty: true,
            cursor: Vec::new(),
            moved_tu: self.now_tu,
            glues,
            glued_by: Keyed::new(),
            torsion: f.torsion.clone(),
            certificates: certs,
            vortex: f.vortex.clone(),
            form: Some(f.clone()),
            bundle,
            remainders: rem_keyed,
        };
        if let Some(old) = self.bubbles.get(key.as_bytes()).cloned() {
            self.retract_incidence(key.as_bytes(), &old);
        }
        self.admit_incidence(&key, &bubble);
        self.bubbles.insert(&key, bubble);
        self.dirty.insert(&key, ());
        self.link_new(&key);
        eprintln!(
            "  form held {:02x}{:02x}…",
            key[0], key[1]
        );
    }

    /// Admit a remainder as a bubble with its cover a peel of its invariant.
    pub fn hold(&mut self, remainder: Vec<RemCell>) {
        if remainder.is_empty() {
            return;
        }
        let key = remainder_addr(&remainder);
        let inv = stalk::invariant(&remainder);
        let cover = cover_of(&inv);
        let mut volume = element::measure_lanes(&remainder);
        volume[2] = volume[2].max(1);
        let mut horizon = Keyed::new();
        self.paint_seeded(&mut horizon);
        let bubble = Bubble {
            remainder,
            nerve: Complex::new(),
            cover,
            horizon,
            potential: volume,
            volume,
            dirty: true,
            cursor: Vec::new(),
            moved_tu: self.now_tu,
            glues: Keyed::new(),
            glued_by: Keyed::new(),
            torsion: Keyed::new(),
            certificates: Keyed::new(),
            vortex: Keyed::new(),
            form: None,
            bundle: None,
            remainders: Keyed::new(),
        };
        if let Some(old) = self.bubbles.get(key.as_bytes()).cloned() {
            self.retract_incidence(key.as_bytes(), &old);
        }
        self.admit_incidence(&key, &bubble);
        self.bubbles.insert(&key, bubble);
        self.dirty.insert(&key, ());
        self.link_new(&key);
    }

    fn merge(&mut self, verts: &[Vec<u8>]) -> Option<crate::transducer::a21::Address> {
        if verts.len() < 2 {
            return verts.first().cloned().map(crate::transducer::a21::Address::from);
        }
        let forms: Vec<Form> = verts
            .iter()
            .filter_map(|v| self.bubbles.get(v.as_slice()).and_then(|b| b.form.clone()))
            .collect();
        if forms.len() >= 2 {
            for w in forms.windows(2) {
                if !form_one_section(&w[0], &w[1]) {
                    eprintln!(
                        "  thought merge refused {:02x}{:02x}… not one section",
                        verts[0][0], verts[0][1]
                    );
                    return verts.first().cloned().map(crate::transducer::a21::Address::from);
                }
            }
        }
        let mut remainder = Vec::new();
        let mut cover = Keyed::new();
        let mut nerve = Complex::new();
        let mut horizon: Keyed<(Rotor, Order, Standing)> = Keyed::new();
        let mut torsion = Keyed::new();
        let mut certificates = Keyed::new();
        let mut vortex = Keyed::new();
        let mut form = None;
        let mut bundle = None;
        let mut remainders: Keyed<Vec<RemCell>> = Keyed::new();
        let mut old_rems: Keyed<Vec<RemCell>> = Keyed::new();
        for v in verts {
            let Some(b) = self.bubbles.get(v.as_slice()).cloned() else {
                continue;
            };
            old_rems.insert(v, b.remainder.clone());
            self.retract_incidence(v, &b);
            remainder = union_rem(&remainder, &b.remainder);
            absorb_keyed(&mut cover, &b.cover);
            absorb_keyed(&mut horizon, &b.horizon);
            absorb_keyed(&mut torsion, &b.torsion);
            absorb_keyed(&mut certificates, &b.certificates);
            absorb_vortex(&mut vortex, &b.vortex);
            absorb_keyed(&mut remainders, &b.remainders);
            if form.is_none() {
                form = b.form.clone();
            }
            if bundle.is_none() {
                bundle = b.bundle.clone();
            }
            nerve = b.nerve.clone();
            self.bubbles.remove(v);
        }
        self.paint_seeded(&mut horizon);
        if remainder.is_empty() && form.is_none() {
            return None;
        }
        let volume = match &form {
            Some(f) => form_mag(f),
            None => element::measure_lanes(&remainder),
        };
        let key = match &form {
            Some(f) => idea_key(f),
            None => remainder_addr(&remainder),
        };
        let bubble = Bubble {
            remainder,
            nerve,
            cover,
            horizon,
            potential: volume,
            volume,
            dirty: false,
            cursor: Vec::new(),
            moved_tu: 0,
            glues: Keyed::new(),
            glued_by: Keyed::new(),
            torsion,
            certificates,
            vortex,
            form,
            bundle,
            remainders,
        };
        self.admit_incidence(&key, &bubble);
        self.bubbles.insert(&key, bubble);
        self.dirty.insert(&key, ());
        self.link_new(&key);
        if verts.len() >= 2 {
            for v in verts {
                if v.as_slice() != key.as_bytes() {
                    if let Some(rem) = old_rems.get(v) {
                        self.cavity.insert(v, rem.clone());
                    }
                }
            }
            let before: Vec<crate::transducer::a21::Address> = verts
                .iter()
                .map(|v| crate::transducer::a21::Address::from(v.clone()))
                .collect();
            let payload = crate::transducer::rewrite::RewritePayload::Condense {
                sections: vec![vec![]],
                gauge: element::identity_versor(),
            };
            if let Some(rw) = crate::transducer::rewrite::mint(
                crate::transducer::rewrite::RewriteKind::Condense,
                &before,
                key.clone(),
                payload,
                [0, verts.len().saturating_sub(1) as u64, 0, 0],
            ) {
                self.pending_rewrites.push(rw);
            }
        }
        Some(key)
    }

    fn component_of(&self, start: &[u8]) -> Vec<Vec<u8>> {
        let mut seen = Keyed::new();
        let mut stack = vec![start.to_vec()];
        seen.insert(start, ());
        while let Some(v) = stack.pop() {
            for ck in self.complex.link(&v, 1) {
                let Some((fill, vertices)) = self.complex.cell(&ck).map(|c| {
                    (matches!(c.fill, Fill::Filled(_)), c.vertices.clone())
                }) else {
                    continue;
                };
                if !fill {
                    continue;
                }
                for u in &vertices {
                    if !seen.contains(u) {
                        seen.insert(u, ());
                        stack.push(u.clone());
                    }
                }
            }
        }
        seen.fiber(ROOT).map(|(k, _)| k).collect()
    }

    fn component_closed(&self, verts: &[Vec<u8>]) -> bool {
        if verts.is_empty() {
            return false;
        }
        let mut held = Keyed::new();
        for v in verts {
            held.insert(v, ());
        }
        // index: vertices of this component; each cell from that vertex's link
        for v in verts {
            for ck in self.complex.link_of(v) {
                let Some(cell) = self.complex.cell(&ck) else {
                    continue;
                };
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

    /// Expand dirty bubbles from the atlas, one Budget of charts per call.
    pub fn expand_all(
        &mut self,
        atlas: &Stalks,
        rotors: &Keyed<Rotor>,
        crystal: &dyn Crystal,
        budget: &mut Budget,
    ) -> (usize, [u64; 4]) {
        let mut spent = 0usize;
        let mut pivoted = [0u64; 4];
        let mut from = self.expand_cursor.clone();
        // index: budgeted keyed cursor over dirty bubbles; each expand spends the rest
        while budget.remaining() > 0 {
            let mut nxt = None;
            {
                let mut cur = self.dirty.cursor(&from, budget);
                if let Some((k, _)) = Iterator::next(&mut cur) {
                    nxt = Some(k.clone());
                }
            }
            let Some(k) = nxt else {
                break;
            };
            from = k.clone();
            self.expand_cursor = k.clone();
            if let Some(b) = self.bubbles.get_mut(&k) {
                let (s, p) = b.expand_with(atlas, rotors, self.now_tu, crystal, budget);
                spent += s;
                for i in 0..4 {
                    pivoted[i] = pivoted[i].saturating_add(p[i]);
                }
                if !b.dirty {
                    self.dirty.remove(&k);
                }
            } else {
                self.dirty.remove(&k);
            }
        }
        (spent, pivoted)
    }

    /// Attempt fill on the link of bubbles at `word`'s chart and prefixes.
    /// Merges filled cells; speaks a closed component as one sentence.
    /// Returns (fills, formed, classes, spoken elements).
    pub fn pivot_on(
        &mut self,
        word: &Element,
        crystal: &dyn Crystal,
        budget: &mut Budget,
    ) -> (u64, u64, u64, Vec<Element>) {
        let inv = stalk::invariant_of(word);
        let prefixes: Vec<Vec<u8>> = inv.prefixes().iter().map(|p| p.key().into_vec()).collect();
        let cav: Vec<(Vec<u8>, Vec<RemCell>)> = self.cavity.fiber(ROOT).map(|(k, r)| (k, r.clone())).collect();
        for (k, rem) in cav {
            let cinv = stalk::invariant(&rem);
            if cinv.prefixes().iter().any(|p| prefixes.iter().any(|q| p.key().as_bytes() == q.as_slice())) {
                self.cavity.remove(&k);
                self.hold(rem);
            }
        }
        let mut keys = Keyed::new();
        for p in inv.prefixes() {
            for (id, _) in self.by_chart.link(&p.key()) {
                keys.insert(&id, ());
            }
        }
        let word_rotor = cech::rotor_of(word);
        let mut fills = 0u64;
        let mut new_classes = 0u64;
        let mut spoken: Vec<Element> = Vec::new();
        spoken.extend(std::mem::take(&mut self.pending_rewrites));
        let bubble_keys: Vec<Vec<u8>> = keys.fiber(ROOT).map(|(k, _)| k).collect();
        if word.sub().is_empty() && crate::transducer::sentence::carries_torsion(word) {
            let addr = word.address();
            for bk in &bubble_keys {
                self.mark_glue(bk, &addr);
                if let Some(b) = self.bubbles.get_mut(addr.as_bytes()) {
                    b.glued_by.insert(bk, ());
                }
            }
        }
        for bk in &bubble_keys {
            if budget.remaining() == 0 {
                break;
            }
            let Some(b) = self.bubbles.get(bk) else {
                continue;
            };
            let horizon = b.horizon.clone();
            let bundle = b.bundle.clone();
            let t = self.now_tu.saturating_sub(b.moved_tu);
            if let Some(mut form) = b.form.clone() {
                let _ = form.pivot_loop(bk, &horizon, crystal, budget, bundle.as_deref(), t);
                if let Some(b) = self.bubbles.get_mut(bk) {
                    b.take_form(form);
                    if let Some(r) = &word_rotor {
                        if let Some(ck) = r.class_key() {
                            b.collapse_vortex(ck.as_bytes(), r.key().as_bytes(), "glue");
                        }
                    }
                }
            }
        }
        // index: by_chart fiber at the word's chart and peels
        for bk in &bubble_keys {
            if budget.remaining() == 0 {
                break;
            }
            // index: link of this bubble
            let cells = self.complex.link_of(bk);
            for key in cells {
                if !budget.spend() {
                    break;
                }
                let Some((d, fill_open, coeff_id, v0, v1, vertices)) = self.complex.cell(&key).map(|c| {
                    (
                        c.degree(),
                        matches!(c.fill, Fill::Open),
                        c.coefficient.is_identity(),
                        c.vertices.first().cloned(),
                        c.vertices.get(1).cloned(),
                        c.vertices.clone(),
                    )
                }) else {
                    continue;
                };
                if d == 0 || !fill_open {
                    continue;
                }
                let outcome = if d == 1 && !coeff_id {
                    match &word_rotor {
                        Some(g) => {
                            let coeff_hit = self.complex.cell(&key).is_some_and(|c| {
                                *g == c.coefficient || g.inverse().as_ref() == Some(&c.coefficient)
                            });
                            if coeff_hit {
                                self.complex.fill_by_gauge(&key, g)
                            } else {
                                self.complex.fill(&key)
                            }
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
                                if let (Some(a), Some(b)) = (v0, v1) {
                                    if let Some(t) = element::from_translation(&gamma, a, b) {
                                        spoken.push(t);
                                    }
                                }
                            }
                            if let Some(merged) = self.merge(&vertices) {
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
                            if let Some(merged) = self.merge(&vertices) {
                                if let Some(b) = self.bubbles.get_mut(merged.as_bytes()) {
                                    let a = vertices[0].clone();
                                    let c = vertices.get(1).cloned().unwrap_or_else(|| a.clone());
                                    b.nerve.file_congruent(&a, &c, g, ell);
                                }
                            }
                        }
                        Fill::Class(r) => {
                            new_classes += 1;
                            self.classes += 1;
                            for v in &vertices {
                                if let Some(b) = self.bubbles.get_mut(v) {
                                    if r.class_key().is_none_or(|ck| !b.horizon.contains(ck.as_bytes())) {
                                        horizon_put(&mut b.horizon, r.clone(), Standing::Held);
                                    }
                                }
                            }
                        }
                        Fill::Open | Fill::Already => {}
                    }
            }
        }
        (fills, self.formed, new_classes, spoken)
    }

    fn form_thought(&self, verts: &[Vec<u8>]) -> Option<Element> {
        let mut forms = Vec::new();
        for v in verts {
            let b = self.bubbles.get(v)?;
            if !b.remainder.is_empty() {
                return None;
            }
            forms.push(b.form.as_ref()?);
        }
        let Some(&f) = forms.first() else {
            return None;
        };
        if !f.is_cauterized() {
            return None;
        }
        let mag = form_mag(f);
        if mag.iter().all(|&x| x == 0) {
            return None;
        }
        sentence::as_element_form(f, mag)
    }

    /// Classes on the contemplation, any degree.
    pub fn classes_held(&self) -> Vec<(Rotor, usize)> {
        self.complex.classes()
    }

    pub fn component_count(&self) -> usize {
        let mut seen = Keyed::new();
        let mut n = 0usize;
        let keys: Vec<Vec<u8>> = self.bubbles.fiber(ROOT).map(|(k, _)| k).collect();
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
    pub const THT2: &'static [u8; 4] = b"THT2";

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(Self::THT2);
        out.extend_from_slice(&(self.bubbles.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.bubbles, |k, b| {
            crate::transducer::a21::put_addr(out, k);
            out.extend_from_slice(&(b.remainder.len() as u32).to_le_bytes());
            for &(r, c, hb, ha) in &b.remainder {
                out.extend_from_slice(&r.to_le_bytes());
                out.extend_from_slice(&c.to_le_bytes());
                out.push(hb);
                out.push(ha);
            }
            out.extend_from_slice(&(b.cover.len() as u32).to_le_bytes());
            crate::snapshot::persist(&b.cover, |ck, _| {
                let _ = crate::transducer::a21::put_addr(out, ck);
            });
            for x in &b.volume {
                out.extend_from_slice(&x.to_le_bytes());
            }
            out.push(b.dirty as u8);
            out.extend_from_slice(&0u32.to_le_bytes());
            out.extend_from_slice(&(b.horizon.len() as u32).to_le_bytes());
            crate::snapshot::persist(&b.horizon, |hk, (r, order, standing)| {
                crate::transducer::a21::put_addr(out, hk);
                r.to_bytes(out);
                out.extend_from_slice(&order.to_u32().to_le_bytes());
                out.push(standing.as_u8());
            });
            for x in &b.potential {
                out.extend_from_slice(&x.to_le_bytes());
            }
            out.extend_from_slice(b"TOR1");
            out.extend_from_slice(&(b.torsion.len() as u32).to_le_bytes());
            crate::snapshot::persist(&b.torsion, |tk, (sigma, ell, chord)| {
                crate::transducer::a21::put_addr(out, tk);
                out.extend_from_slice(&(sigma.len() as u16).to_le_bytes());
                for &(a, c) in sigma {
                    out.push(a);
                    out.push(c);
                }
                out.extend_from_slice(&ell.to_le_bytes());
                out.extend_from_slice(&chord.to_le_bytes());
            });
            out.extend_from_slice(&(b.certificates.len() as u32).to_le_bytes());
            crate::snapshot::persist(&b.certificates, |_, c| {
                c.to_bytes(out);
            });
            out.extend_from_slice(&(b.vortex.len() as u32).to_le_bytes());
            crate::snapshot::persist(&b.vortex, |after, inner| {
                crate::transducer::a21::put_addr(out, after);
                out.extend_from_slice(&(inner.len() as u32).to_le_bytes());
                crate::snapshot::persist(inner, |_, c| {
                    c.to_bytes(out);
                });
            });
            match &b.form {
                Some(f) => {
                    let mut fb = Vec::new();
                    if sentence::encode_form_v2(f, &mut fb).is_some() {
                        out.extend_from_slice(b"FRM1");
                        out.extend_from_slice(&(fb.len() as u32).to_le_bytes());
                        out.extend_from_slice(&fb);
                    }
                }
                None => {}
            }
        });
        self.complex.to_bytes(out);
        out.extend_from_slice(&self.fills.to_le_bytes());
        out.extend_from_slice(&self.torsion_merges.to_le_bytes());
        out.extend_from_slice(&self.classes.to_le_bytes());
        out.extend_from_slice(&self.formed.to_le_bytes());
        out.extend_from_slice(&(self.cavity.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.cavity, |k, rem| {
            crate::transducer::a21::put_addr(out, k);
            out.extend_from_slice(&(rem.len() as u32).to_le_bytes());
            for &(r, c, hb, ha) in rem {
                out.extend_from_slice(&r.to_le_bytes());
                out.extend_from_slice(&c.to_le_bytes());
                out.push(hb);
                out.push(ha);
            }
        });
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Contemplation> {
        if b.get(*pos..*pos + 4) != Some(&Self::THT2[..]) {
            return None;
        }
        *pos += 4;
        let take_u32 = |b: &[u8], pos: &mut usize| -> Option<u32> {
            let v = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
            *pos += 4;
            Some(v)
        };
        let take_count = |b: &[u8], pos: usize, n: usize, item: usize| -> Option<usize> {
            if item == 0 {
                return Some(n);
            }
            if n > b.len().saturating_sub(pos) / item {
                return None;
            }
            Some(n)
        };
        let n = take_count(b, *pos, take_u32(b, pos)? as usize, 1)?;
        let mut c = Contemplation::new();
        for _ in 0..n {
            let key = crate::transducer::a21::take_addr(b, pos)?;
            let nr = take_count(b, *pos, take_u32(b, pos)? as usize, 18)?;
            let mut remainder = Vec::with_capacity(nr);
            for _ in 0..nr {
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
            let nc = take_count(b, *pos, take_u32(b, pos)? as usize, 1)?;
            let mut cover = Keyed::new();
            for _ in 0..nc {
                let ck = crate::transducer::a21::take_addr(b, pos)?;
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
            let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
            *pos = end;
            let mut horizon: Keyed<(Rotor, Order, Standing)> = Keyed::new();
            if *pos + 4 <= b.len() {
                let nh = take_count(b, *pos, take_u32(b, pos)? as usize, 1)?;
                for _ in 0..nh {
                    let hk = crate::transducer::a21::take_addr(b, pos)?;
                    let r = Rotor::from_bytes(b, pos)?;
                    let ell = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
                    *pos += 4;
                    let st = Standing::from_u8(*b.get(*pos)?).unwrap_or(Standing::Held);
                    *pos += 1;
                    horizon.insert(hk, (r, Order::from_u32(ell), st));
                }
            }
            let mut potential = volume;
            if *pos + 32 <= b.len() {
                for x in &mut potential {
                    *x = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                }
            }
            let mut torsion = Keyed::new();
            let mut certificates = Keyed::new();
            let mut vortex = Keyed::new();
            if b.get(*pos..*pos + 4) == Some(b"TOR1") {
                *pos += 4;
                // A count that does not fit remaining bytes is refused, never
                // truncated: a σ longer than the stream would desync the tape.
                let nt = take_count(b, *pos, take_u32(b, pos)? as usize, 40)?;
                for _ in 0..nt {
                    let tk = crate::transducer::a21::take_addr(b, pos)?;
                    let ns = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?) as usize;
                    *pos += 2;
                    let ns = take_count(b, *pos, ns, 2)?;
                    let mut sigma = Vec::with_capacity(ns);
                    for _ in 0..ns {
                        let a = *b.get(*pos)?;
                        *pos += 1;
                        let cpair = *b.get(*pos)?;
                        *pos += 1;
                        sigma.push((a, cpair));
                    }
                    let ell = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
                    *pos += 4;
                    let chord = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
                    *pos += 2;
                    torsion.insert(tk, (sigma, ell, chord));
                }
                let nc = take_count(b, *pos, take_u32(b, pos)? as usize, 1)?;
                for _ in 0..nc {
                    let c = sentence::PivotCertificate::from_bytes(b, pos)?;
                    certificates.insert(c.key(), c);
                }
                let nv = take_count(b, *pos, take_u32(b, pos)? as usize, 1)?;
                for _ in 0..nv {
                    let after = crate::transducer::a21::take_addr(b, pos)?;
                    let nc = take_count(b, *pos, take_u32(b, pos)? as usize, 1)?;
                    let mut inner = Keyed::new();
                    for _ in 0..nc {
                        let c = sentence::PivotCertificate::from_bytes(b, pos)?;
                        let wk = c.key();
                        inner.insert(wk, c);
                    }
                    vortex.insert(after, inner);
                }
            }
            let form = if b.get(*pos..*pos + 4) == Some(b"FRM1") {
                *pos += 4;
                let flen = take_u32(b, pos)? as usize;
                let end = pos.checked_add(flen).filter(|e| *e <= b.len())?;
                let mut fp = 0usize;
                let f = sentence::decode_form_v2(&b[*pos..end], &mut fp);
                *pos += flen;
                f
            } else {
                None
            };
            let bubble = Bubble {
                remainder,
                nerve: Complex::new(),
                cover,
                horizon,
                potential,
                volume,
                dirty,
                cursor: Vec::new(),
                moved_tu: 0,
                glues: Keyed::new(),
                glued_by: Keyed::new(),
                torsion,
                certificates,
                vortex,
                form,
                bundle: None,
                remainders: Keyed::new(),
            };
            c.admit_incidence(&key, &bubble);
            c.bubbles.insert(&key, bubble);
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
        if *pos + 4 <= b.len() {
            let nc = take_count(b, *pos, take_u32(b, pos)? as usize, 1)?;
            for _ in 0..nc {
                let k = crate::transducer::a21::take_addr(b, pos)?;
                let nr = take_count(b, *pos, take_u32(b, pos)? as usize, 18)?;
                let mut rem = Vec::with_capacity(nr);
                for _ in 0..nr {
                    let r = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let col = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let hb = *b.get(*pos)?;
                    *pos += 1;
                    let ha = *b.get(*pos)?;
                    *pos += 1;
                    rem.push((r, col, hb, ha));
                }
                c.cavity.insert(k, rem);
            }
        }
        Some(c)
    }

    pub fn horizon_cells(&self) -> usize {
        let mut n = 0usize;
        for (_, b) in self.bubbles.fiber(ROOT) {
            n += b.horizon.len();
        }
        n
    }

    pub fn potential(&self) -> [u64; 4] {
        let mut p = [0u64; 4];
        for (_, b) in self.bubbles.fiber(ROOT) {
            for l in 0..4 {
                p[l] = p[l].saturating_add(b.potential[l]);
            }
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unmeasured_space_is_a_finite_boundary_norm() {
        let mut c = Contemplation::new();
        assert_eq!(c.boundary_norm(), 0, "no thought is zero, not MAX");
        c.hold(vec![(0, 0, 1, 2)]);
        let n = c.boundary_norm();
        assert!(n > 0 && n < usize::MAX, "∂g ≠ 0 is a finite cochain, got {n}");
    }

    #[test]
    fn two_bubbles_on_one_chart_share_an_edge_not_a_triangle() {
        let mut c = Contemplation::new();
        c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
        c.hold(vec![(5, 5, 3, 4), (5, 6, 3, 4)]);
        assert_eq!(c.len(), 2, "disjoint remainders are two bubbles");
        let keys: Vec<Vec<u8>> = c.bubbles.fiber(ROOT).map(|(k, _)| k).collect();
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
        let keys: Vec<Vec<u8>> = c.bubbles.fiber(ROOT).map(|(k, _)| k).collect();
        assert_eq!(keys.len(), 1);
        assert!(c.bubbles.get(&keys[0]).unwrap().potential.iter().any(|&x| x > 0));
    }

    #[test]
    fn t_room_moves_the_oldest_bubble_to_cavity() {
        let mut c = Contemplation::new();
        c.set_now(10);
        c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
        c.set_now(50);
        c.hold(vec![(5, 5, 3, 4), (5, 6, 3, 4)]);
        assert_eq!(c.len(), 2);
        c.retire_on_t(1);
        assert_eq!(c.len(), 1, "T-room of one keeps the younger bubble");
        assert_eq!(c.cavity_len(), 1, "the older remainder is a cavity chart");
    }

    /// Two pivots on the same bubble with no new word leave fills unchanged.
    #[test]
    fn two_pivots_without_a_new_word_leave_fills() {
        use crate::transducer::element::from_residual;
        let mut c = Contemplation::new();
        let rem = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
        c.hold(rem.clone());
        c.hold(vec![(0, 1, 3, 4), (0, 2, 3, 4)]);
        let word = from_residual(&rem);
        let snap = |c: &Contemplation| {
            let mut out = Vec::new();
            for layer in &c.complex.degrees {
                for (k, cell) in layer.fiber(ROOT) {
                    out.push((k.clone(), cell.fill.clone()));
                }
            }
            out.sort_by(|a, b| a.0.cmp(&b.0));
            out
        };
        let mut budget = crate::physics::Physics::default().stroke_n(64);
        let _ = c.pivot_on(&word, &crate::transducer::family::NoCrystal, &mut budget);
        let before = snap(&c);
        let mut budget = crate::physics::Physics::default().stroke_n(64);
        let _ = c.pivot_on(&word, &crate::transducer::family::NoCrystal, &mut budget);
        let fills = |s: &[(Vec<u8>, Fill)]| s.iter().map(|(_, f)| f.clone()).collect::<Vec<_>>();
        assert_eq!(fills(&snap(&c)), fills(&before), "a second pivot with no new word leaves fills");
    }

    #[test]
    fn tht1_round_trips_bubbles() {
        let mut c = Contemplation::new();
        c.hold(vec![(0, 0, 1, 2), (1, 0, 1, 2)]);
        let mut bytes = Vec::new();
        c.to_bytes(&mut bytes);
        let mut pos = 0usize;
        let back = Contemplation::from_bytes(&bytes, &mut pos).expect("THT2");
        assert_eq!(back.len(), 1);
        assert_eq!(back.potential(), c.potential());
    }

    #[test]
    fn horizon_seeded_class_is_not_rederived() {
        use crate::transducer::alphabet::{LetterId, Word};
        let mut c = Contemplation::new();
        let q = LetterId::quarter((0, 1)).unwrap();
        let r = Word::from_letters(&[(q, false)]).rotor();
        let rec = ClassRecord {
            key: r.class_key().expect("quarter class"),
            order: r.word_order().expect("quarter spells"),
            representative: r.clone(),
        };
        assert_eq!(c.seed_horizon(&[rec.clone()]), 1);
        c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
        let b = c.bubbles.fiber(ROOT).next().map(|(_, b)| b).expect("bubble");
        let (_, _, st) = b.horizon.get(&rec.key).expect("seeded");
        assert_eq!(*st, Standing::Held);
    }

    #[test]
    fn restart_refills_horizon_from_syllabus() {
        use crate::transducer::alphabet::{LetterId, Word};
        let mut c = Contemplation::new();
        assert_eq!(c.horizon_cells(), 0);
        let q = LetterId::quarter((0, 1)).unwrap();
        let r = Word::from_letters(&[(q, false)]).rotor();
        let rec = ClassRecord {
            key: r.class_key().expect("quarter class"),
            order: r.word_order().expect("quarter spells"),
            representative: r,
        };
        let n = c.seed_horizon(std::slice::from_ref(&rec));
        assert_eq!(n, 1, "empty mind refills from the page");
        c.hold(vec![(5, 5, 3, 4)]);
        assert!(
            c.bubbles
                .fiber(ROOT)
                .any(|(_, b)| b.horizon.contains(&rec.key)),
            "a later idea inherits the seeded class"
        );
    }

    #[test]
    fn wrap_glues_crystallized_form() {
        use crate::transducer::element::{Element, FIBER_SPATIAL};
        let mut c = Contemplation::new();
        let rem = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
        c.hold(rem);
        let mut word = Element::identity(3);
        word.record_torsion(vec![(1, 2)], 2, FIBER_SPATIAL);
        let mut budget = crate::physics::Physics::default().stroke_n(32);
        let _ = c.pivot_on(&word, &crate::transducer::family::NoCrystal, &mut budget);
        assert!(
            c.glues_word(&word.address()),
            "a crystallized form is glued into the meeting idea"
        );
    }

    /// A form with Glued vertices and empty remainder is held as the form.
    #[test]
    fn hold_form_keeps_a_glued_open_loop() {
        use crate::transducer::alphabet::Word;
        use crate::transducer::sentence::{Form, FormFace, Vertex};
        let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).unwrap();
        let w = Word::from_letters(&[(q, false)]);
        let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
        let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
        let inner = {
            let mut f = Form::default();
            f.vertices.push(Vertex::Word {
                address: crate::transducer::a21::Address::from(vec![1]),
                word: w.clone(),
                rho: one.clone(),
                placement: (zero.clone(), zero.clone()),
                gauge: vec![(1, 2)],
                class: crate::transducer::stalk::Invariant::default(),
                contact: None,
            });
            f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("vertices"), });
            f.loops = Form::loops_of(&f.faces, f.vertices.len());
            f
        };
        let mut outer = Form::default();
        let _gi = outer
            .glue(&inner, crate::transducer::a21::Address::from(vec![0xab]), Word::identity(), [1, 0, 0, 0])
            .expect("glue");
        assert!(outer.remainder.is_empty(), "no remainder to hold()");
        let mut c = Contemplation::new();
        c.hold_form(&outer, &[], &[]);
        assert_eq!(c.bubbles.len(), 1, "the glued form is an idea");
        let b = c.bubbles.fiber(ROOT).next().map(|(_, b)| b).expect("held");
        assert!(
            b.form.as_ref().is_some_and(|f| {
                f.vertices
                    .iter()
                    .any(|v| matches!(v, Vertex::Glued { .. }))
            }),
            "the bubble carries the Form"
        );
        let mut bytes = Vec::new();
        c.to_bytes(&mut bytes);
        let mut pos = 0usize;
        let back = Contemplation::from_bytes(&bytes, &mut pos).expect("FRM1");
        assert_eq!(back.bubbles.len(), 1, "the glued form survives the tape");

        let mut open = Form::default();
        open.vertices.push(Vertex::Word {
            address: crate::transducer::a21::Address::from(vec![2]),
            word: Word::identity(),
            rho: one.clone(),
            placement: (zero.clone(), zero.clone()),
            gauge: vec![(1, 2)],
            class: crate::transducer::stalk::Invariant::default(),
            contact: None,
        });
        open.vertices.push(Vertex::Word {
            address: crate::transducer::a21::Address::from(vec![3]),
            word: Word::identity(),
            rho: one,
            placement: (zero.clone(), zero),
            gauge: vec![(1, 2)],
            class: crate::transducer::stalk::Invariant::default(),
            contact: None,
        });
        open.faces.push(FormFace {
            i: 0,
            j: 1,
            transport: open.face_of(0, 1).expect("star 1"),
        });
        open.faces.push(FormFace {
            i: 0,
            j: 2,
            transport: open.face_of(0, 2).expect("star 2"),
        });
        open.faces.push(FormFace::new(1, 2, w.clone()));
        open.loops = Form::loops_of(&open.faces, open.vertices.len());
        assert!(
            open.loops.iter().any(|lp| !lp.holonomy.is_identity()),
            "the held form has an open loop"
        );
        let mut c2 = Contemplation::new();
        c2.hold_form(&open, &[], &[]);
        let k = c2.bubbles.fiber(ROOT).next().map(|(k, _)| k).expect("held");
        assert_eq!(k.as_slice(), super::idea_key(&open).as_bytes(), "hold, merge and Bubble::key share idea_key");
        struct HoldQ(Word);
        impl crate::transducer::family::Crystal for HoldQ {
            fn on_class(
                &self,
                _c: &crate::transducer::stalk::Invariant,
            ) -> Vec<(
                crate::transducer::element::Element,
                Form,
                crate::transducer::stalk::Standing,
            )> {
                Vec::new()
            }
            fn speak_form(
                &self,
                _f: &Form,
                _x: &crate::transducer::family::Grid,
                _p: &Word,
            ) -> Option<crate::transducer::family::Speech> {
                None
            }
            fn by_address(&self, _a: &[u8]) -> Option<(crate::transducer::element::Element, Form)> {
                None
            }
            fn on_axis(
                &self,
                _c: &crate::transducer::stalk::Invariant,
                axis: crate::transducer::sentence::Axis,
            ) -> Vec<Word> {
                let v = crate::transducer::sentence::ClassVector::of_word(&self.0);
                if v.occupies(axis) {
                    vec![self.0.clone()]
                } else {
                    Vec::new()
                }
            }
        }
        let mut budget = crate::physics::Physics::default().stroke_n(16);
        if let Some(b) = c2.bubbles.get_mut(&k) {
            let atlas = Stalks::default();
            let _ = b.expand_with(&atlas, &Keyed::new(), 0, &HoldQ(w.clone()), &mut budget);
        }
        let b = c2.bubbles.get(&k).expect("still held");
        let form = b.form.as_ref().expect("form carrier");
        let discharged = !form.certificates.is_empty();
        let filed = !form.torsion.is_empty();
        assert!(
            discharged || filed,
            "the held form discharges or files, certificates {} torsion {}",
            form.certificates.len(),
            form.torsion.len()
        );
        let mag = super::form_mag(form);
        if mag.iter().any(|&x| x > 0) && form.is_cauterized() {
            let e = sentence::as_element_form(form, mag).expect("spoken tape");
            assert!(
                crate::transducer::sentence::carries_torsion(&e) || !form.certificates.is_empty(),
                "the spoken tape carries the certificate or the filed cycle"
            );
        }
    }

    #[test]
    fn hold_form_carries_bundle_and_remainder() {
        use crate::transducer::alphabet::Word;
        use crate::transducer::family::pair_idx_key;
        use crate::transducer::sentence::{Form, FormFace, Vertex};
        let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
        let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
        let mut f = Form::default();
        f.vertices.push(Vertex::Word {
            address: crate::transducer::a21::Address::from(vec![9]),
            word: Word::identity(),
            rho: one,
            placement: (zero.clone(), zero),
            gauge: vec![(1, 2)],
            class: crate::transducer::stalk::Invariant::default(),
            contact: None,
        });
        f.faces.push(FormFace {
            i: 0,
            j: 1,
            transport: f.face_of(0, 1).expect("star"),
        });
        let train = vec![(vec![vec![1u8]], vec![vec![2u8]])];
        let rem = vec![vec![(0i64, 0i64, 1u8, 2u8)]];
        let mut c = Contemplation::new();
        c.hold_form(&f, &train, &rem);
        assert_eq!(c.bubbles.len(), 1);
        let (k, b) = c.bubbles.fiber(ROOT).next().expect("held");
        assert_eq!(k.as_slice(), super::idea_key(&f).as_bytes(), "idea_key stays the form's vertex key");
        assert!(
            b.bundle.as_ref().is_some_and(|t| t.len() == 1),
            "the idea carries the train"
        );
        let cells = b.remainders.get(&pair_idx_key(0)).expect("pair remainder");
        assert_eq!(cells.len(), 1, "remainder is keyed by pair");
        assert!(
            b.remainder.is_empty(),
            "Bubble.remainder is the form's own remainder, not concatenated pairs"
        );
        assert_eq!(
            b.volume,
            crate::transducer::element::measure_lanes(cells),
            "volume is the lane-wise sum of per-pair remainder measures"
        );
    }

    #[test]
    fn horizon_is_keyed_by_class_key_and_round_trips() {
        use crate::transducer::alphabet::{LetterId, Word};
        let mut c = Contemplation::new();
        c.hold(vec![(0, 0, 1, 2), (1, 0, 1, 2)]);
        let q = LetterId::quarter((0, 1)).unwrap();
        let r = Word::from_letters(&[(q, false)]).rotor();
        let ck = r.class_key().expect("quarter class");
        let key = {
            let (k, _) = c.bubbles.fiber(ROOT).next().expect("bubble");
            k
        };
        if let Some(b) = c.bubbles.get_mut(&key) {
            horizon_put(&mut b.horizon, r.clone(), Standing::Held);
        }
        assert!(c.bubbles.get(&key).unwrap().horizon.contains(ck.as_bytes()));
        let mut bytes = Vec::new();
        c.to_bytes(&mut bytes);
        assert_eq!(&bytes[..4], Contemplation::THT2);
        let mut pos = 0usize;
        let back = Contemplation::from_bytes(&bytes, &mut pos).expect("THT2");
        let hb = back.bubbles.get(&key).expect("restored");
        let (got, order, st) = hb.horizon.get(ck.as_bytes()).expect("class key");
        assert_eq!(got.class_key(), Some(ck.clone()));
        assert_eq!(*order, r.word_order().expect("quarter spells"));
        assert_eq!(*st, Standing::Held);
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
            let mut budget = crate::physics::Physics::default().stroke_n(4);
            let _ = c.expand_all(&atlas, &Keyed::new(), &crate::transducer::family::NoCrystal, &mut budget);
            let expand = work::total();
            work::reset();
            let mut charts = 0usize;
            for (_, b) in c.bubbles.fiber(ROOT) {
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
            }
            let quote = work::total();
            assert!(charts > 0);
            (hold, expand, quote)
        };
        let a = run(32);
        let b = run(128);
        eprintln!("work thought N=32 {a:?} N=128 {b:?}");
        assert_eq!(a, b, "4N differs by zero: {a:?} vs {b:?}");
    }

    /// 25.1: a syllabus class whose key is one held certificate's `after`
    /// applies that one; the other vortex cell is not applied.
    #[test]
    fn vortex_collapses_by_glue_or_ruling() {
        use crate::transducer::alphabet::{LetterId, Word};
        use crate::transducer::sentence::{Axis, PivotBy, PivotCertificate, Rank, Side};
        let mut c = Contemplation::new();
        c.hold(vec![(0, 0, 1, 2)]);
        let q = LetterId::quarter((0, 1)).unwrap();
        let w = Word::from_letters(&[(q, false)]);
        let r = w.rotor();
        let after = r.class_key().expect("quarter class");
        let hit = PivotCertificate {
            chord: 0,
            axis: Axis::Unit(0),
            before: crate::transducer::a21::Address::from(vec![1]),
            after: after.clone(),
            rank_before: Rank {
                torsion: 1,
                free: 0,
                offplane: 0,
                axes: 0,
            },
            rank_after: Rank::default(),
            mu: crate::transducer::sentence::Mu::Unmeasured,
            by: PivotBy::Word {
                face: 0,
                word: w.clone(),
                side: Side::Right,
            },
        };
        let sheet = crate::transducer::alphabet::PlaneForm {
            parity: true,
            rot: [0; crate::transducer::alphabet::CAYLEY_ALPHABET_LEN],
        }
        .to_word();
        assert_ne!(w.key(), sheet.key(), "two words, one after class");
        let other = PivotCertificate {
            chord: 0,
            axis: Axis::Hue,
            before: crate::transducer::a21::Address::from(vec![2]),
            after: after.clone(),
            rank_before: Rank {
                torsion: 1,
                free: 0,
                offplane: 0,
                axes: 0,
            },
            rank_after: Rank::default(),
            mu: crate::transducer::sentence::Mu::Unmeasured,
            by: PivotBy::Word {
                face: 0,
                word: sheet,
                side: Side::Left,
            },
        };
        let k = c.bubbles.fiber(ROOT).next().map(|(k, _)| k).expect("bubble");
        let hit_key = hit.key();
        if let Some(b) = c.bubbles.get_mut(&k) {
            sentence::vortex_put(&mut b.vortex, hit);
            sentence::vortex_put(&mut b.vortex, other);
        }
        let rec = ClassRecord {
            key: after,
            order: r.word_order().expect("quarter spells"),
            representative: r,
        };
        assert_eq!(c.seed_horizon(&[rec]), 1);
        let b = c.bubbles.get(&k).expect("bubble");
        assert_eq!(b.vortex.len(), 0, "the vortex collapsed");
        assert!(
            b.certificates.get(&hit_key).is_some(),
            "the matching certificate was applied"
        );
    }
}
