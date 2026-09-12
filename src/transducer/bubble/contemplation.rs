// 25 — THE CONTEMPLATION: every bubble a mind is holding at once, and the
// incidence between them.
//
//! `Contemplation` is the complex of held thoughts: the bubbles, their
//! remainders, the horizon of classes seeded from the syllabus, and the
//! cavity. Holding a form admits a vertex and links it to what it meets.

use super::*;

/// The complex over bubbles. by_chart is the incidence trie that makes
/// every connection a read.
#[derive(Clone, Debug, Default)]
pub struct Contemplation {
    pub bubbles: Keyed<Bubble>,
    pub by_chart: Keyed<Keyed<()>>,
    pub by_cell: Keyed<Keyed<()>>,
    pub complex: Complex,
    /// Bubbles whose cover still has work; written when dirty is set.
    pub(crate) dirty: Keyed<()>,
    pub(crate) expand_cursor: Vec<u8>,
    /// The dirty ring's orbit (45.7, S13.3): the circuits `expand_all`
    /// completed and the strokes the last one took — the mind's local time
    /// beside K, written once per circuit as `[{tag}] orbit ring`.
    pub(crate) ring: crate::physics::Orbit,
    pub fills: u64,
    pub torsion_merges: u64,
    pub classes: u64,
    pub formed: u64,
    /// Bubbles retired to a cavity chart, address kept, remainder in the tape.
    pub(crate) cavity: Keyed<Vec<RemCell>>,
    pub(crate) pending_rewrites: Vec<Element>,
    /// TUniversal the entity last stamped; T-room and potential[2] read it.
    pub(crate) now_k: u64,
    /// Mesh-seeded classes, keyed by class key, Standing::Held. Not persisted.
    pub(crate) seeded: Keyed<(Rotor, Order, Standing)>,
}

impl Contemplation {
    pub fn new() -> Contemplation {
        Contemplation::default()
    }

    /// The mind's OWN K (45.4), never the mesh's height. A height advances
    /// only on admitted receipts (45.2), so a contemplation clocked by one
    /// froze at 0 on a mesh that admitted nothing: every bubble's extent
    /// read as the same value, `retire_on_t` could not tell them apart, and
    /// ideas accumulated for the life of the process.
    pub fn set_now(&mut self, k: u64) {
        self.now_k = k;
    }

    pub fn len(&self) -> usize {
        self.bubbles.len()
    }

    /// No bubble is held: nothing is under contemplation.
    pub fn is_empty(&self) -> bool {
        self.bubbles.is_empty()
    }

    /// True when a live idea glued this address (a dialect entry so marked
    /// is not shed).
    pub fn glues_word(&self, addr: &[u8]) -> bool {
        self.bubbles
            .fiber(ROOT)
            .any(|(_, b)| b.glues.contains(addr))
    }

    /// Record that idea `bubble` glued crystallized form `word`.
    pub fn mark_glue(&mut self, bubble: &[u8], word: impl AsRef<[u8]>) {
        if let Some(b) = self.bubbles.get_mut(bubble) {
            b.glues.insert(word, ());
        }
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

    /// THE BOUNDARY MEASURE of the open thoughts (law 5, L7a): the
    /// four-lane measure of the held remainders, read per lane and never
    /// summed — a sum over lanes was the L1 norm of the convergence
    /// engine's coherence functional, the scalar the doctrine refuses. An
    /// unmeasured space is an open thought (∂g ≠ 0), not infinite
    /// discordance; no thought is `[0; 4]`.
    pub fn boundary_measure(&self) -> [u64; 4] {
        let mut acc: Vec<RemCell> = Vec::new();
        for r in self.remainders() {
            acc.extend(r.iter().copied());
        }
        acc.sort_unstable();
        acc.dedup();
        element::measure_lanes(&acc)
    }

    pub(crate) fn retract_incidence(&mut self, key: &[u8], b: &Bubble) {
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

    pub(crate) fn admit_incidence(&mut self, key: impl AsRef<[u8]>, b: &Bubble) {
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

    /// THE HELD STRATUM AT A CHART (24.1 as ruled 2026-09-09): the Word
    /// vertices of the held forms on `inv`, read at the coordinate the
    /// caller supplies — the chart names its bubbles (`by_chart`), and each
    /// bubble's form names the vertices it holds on that chart
    /// (`Form.by_class`, 18.2). Nothing is materialised ahead of the
    /// question, no vertex list is walked, and no second carrier of held
    /// words exists beside this contemplation.
    pub fn held_at(&self, inv: &Invariant) -> Vec<&Vertex> {
        let chart = inv.key();
        let Some(ideas) = self.by_chart.get(chart.as_bytes()) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        // index: the bubbles this chart names; then the vertices each
        // form's by_class names on it
        for (bk, _) in ideas.fiber(ROOT) {
            let Some(form) = self.bubbles.get(&bk).and_then(|b| b.form.as_ref()) else {
                continue;
            };
            let Some(on) = form.by_class.get(chart.as_bytes()) else {
                continue;
            };
            for &i in on {
                if let Some(v @ Vertex::Word { .. }) = form.vertices.get(i as usize) {
                    out.push(v);
                }
            }
        }
        out
    }

    pub(crate) fn rotor_between(&self, a: &[u8], b: &[u8]) -> Option<Rotor> {
        let ba = self.bubbles.get(a)?;
        let bb = self.bubbles.get(b)?;
        let ra = ba.rotor()?;
        let rb = bb.rotor()?;
        Some(rb.geo(&ra.inverse()?))
    }

    /// Link of one new bubble: edges to bubbles sharing a chart (by_chart
    /// peel), higher cells where shared remainder volume is nonzero.
    pub(crate) fn link_new(&mut self, key: impl AsRef<[u8]>) {
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
            // A bubble's section carries a rotation and no placement, so
            // the carriage is the identity — which is TRUE of it, not a
            // value stood in for one it has (L11e).
            Section::at(crate::transducer::address::Address::from(key.clone()), Rotor::identity()),
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
        // Overlap is read in the SCENE: the chain carried through its own
        // translator (§2.1). Two bubbles holding one chain at two
        // translators are one substance said at two places, and their
        // canonical cells coincide at the origin while their placed cells
        // do not.
        if let Some(b) = self.bubbles.get(&key) {
            rems.insert(&key, b.remainder.placed());
        }
        for n in &nbs {
            if let Some(b) = self.bubbles.get(n) {
                rems.insert(n, b.remainder.placed());
            }
            let vol = volume_of(&rems, &[key.as_slice(), n.as_slice()]);
            match self.rotor_between(&key, n) {
                Some(rotor) => self.complex.set_edge(
                    &key,
                    n,
                    rotor,
                    crate::transducer::element::identity_versor(),
                    vol,
                ),
                None => {
                    let both_remainder = self.bubbles.get(&key).is_some_and(|b| b.form.is_none())
                        && self.bubbles.get(n).is_some_and(|b| b.form.is_none());
                    if both_remainder && vol.iter().any(|&x| x > 0) {
                        let _ = self.merge(&[key.clone(), n.clone()]);
                        break;
                    }
                }
            }
        }
        self.complex.coboundary_on_link(&key, |verts| volume_of(&rems, verts));
        self.decide_neighbors(&key);
    }

    /// Hold a form as an idea (chapter 30 step 5): the form is the carrier,
    /// and this contemplation is the one carrier of held forms (24.1 as
    /// ruled 2026-09-09): its words are read at a chart when a cite asks,
    /// through the form's own `by_class`, which is derived here as the
    /// form is taken into custody. The train is the germinating bundle;
    /// remainders are per-pair residual chains. idea_key is the form's
    /// vertex key.
    pub fn hold_form(
        &mut self,
        f: &Form,
        train: &crate::transducer::family::Family,
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
        // THE FORM'S OWN STANDING LOOPS ENTER THE HORIZON (L24). A bubble
        // was seeded from the mesh's classes alone, so a loop this form
        // carries — a cover's meet cycle whose edge holonomy stands —
        // reached `pivot_loop` as nothing at all: the pivot read the
        // horizon it was given, and the class the perception had just
        // measured was not in it.
        //
        // `Bubble::expand_with` already does exactly this for a cell it
        // meets on the atlas (`Fill::Class` → `horizon_put`). This is the
        // same admission for the classes the form ARRIVED with, so a
        // perceived obstruction and a contemplated one are one kind of
        // object.
        //
        // `Standing::Held`, as the atlas arm writes: the class is met and
        // not ruled. A loop whose class vector occupies nothing is not a
        // horizon member — there is nothing to discharge — and a rotor
        // with no order or no class key is refused by `horizon_put`.
        self.paint_standing(f, &mut horizon);
        let certs = f.certificates.clone();
        let bundle = if train.is_empty() {
            None
        } else {
            Some(train.to_vec())
        };
        let mut held = f.clone();
        held.reindex();
        let bubble = Bubble {
            remainder: f.remainder.clone(),
            nerve: Complex::new(),
            cover,
            horizon,
            potential: volume,
            volume,
            dirty: true,
            cursor: Vec::new(),
            moved_k: self.now_k,
            glues,
            glued_by: Keyed::new(),
            torsion: f.torsion.clone(),
            certificates: certs,
            vortex: f.vortex.clone(),
            form: Some(held),
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
        // A remainder saying two things at one cell has no A5 chain and no
        // address; it is not held (U14).
        let Some(remainder) = element::place_chain(&remainder) else {
            return;
        };
        let key = remainder_addr(&remainder);
        let inv = stalk::invariant(&remainder);
        let cover = cover_of(&inv);
        // The volume is the remainder's own measure (L7b): a T extent of
        // zero is zero, and no lane is floored so that a gate downstream
        // would pass — the pivot loop runs on every dirty idea (25.1), T
        // paces nothing (law 9).
        let volume = element::measure_lanes(&remainder);
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
            moved_k: self.now_k,
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

    pub(crate) fn component_of(&self, start: &[u8]) -> Vec<Vec<u8>> {
        let mut seen = Keyed::new();
        let mut stack = vec![start.to_vec()];
        seen.insert(start, ());
        while let Some(v) = stack.pop() {
            for ck in self.complex.link(&v, 1) {
                let Some((fill, vertices)) = self.complex.cell(&ck).map(|c| {
                    (matches!(c.fill, Fill::Filled(..)), c.vertices.clone())
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

    pub(crate) fn component_closed(&self, verts: &[Vec<u8>]) -> bool {
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
                if all_in && !matches!(cell.fill, Fill::Filled(..)) {
                    return false;
                }
            }
        }
        true
    }

    pub(crate) fn form_thought(&self, verts: &[Vec<u8>]) -> Option<Element> {
        let mut forms = Vec::new();
        for v in verts {
            let b = self.bubbles.get(v)?;
            if !b.remainder.is_empty() {
                return None;
            }
            forms.push(b.form.as_ref()?);
        }
        // The component arrives in KEY ORDER (`component_of` reads it off a
        // keyed fiber), so the head is its least vertex and every node
        // forms the thought on the same one.
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — canonical key order: the component is a keyed fiber, so its head is the least address in it"
        )]
        let &f = forms.first()?;
        if !f.is_cauterized() {
            return None;
        }
        let mag = form_mag(f);
        if mag.iter().all(|&x| x == 0) {
            return None;
        }
        sentence::as_element_form(f, mag)
    }

    /// CAUTERIZE (25.2): an idea whose form is closed — every loop the
    /// identity, filed torsion or an opened axis, every remainder empty —
    /// crystallizes. The tape carries its torsion, certificates and axes,
    /// M2 fixes the address, and its magnitude is the descent it measures
    /// over the bundle it germinated from (26), through the crystal that
    /// resolves what it cites; a form with no bundle carries what it glued.
    /// The bubble leaves the contemplation unless a live idea glued it
    /// (`retire_on_t` skips those). `None` when the form is not closed, a
    /// pair of the bundle is still open, or nothing is measured: a form that
    /// closes nothing is the citation, not a form (4), and stays a thought.
    pub fn crystallize(&mut self, key: &[u8], crystal: &dyn Crystal) -> Option<Element> {
        let b = self.bubbles.get(key)?;
        let f = b.form.as_ref()?;
        if !f.is_cauterized() || !f.remainder.is_empty() {
            return None;
        }
        if b.remainders.fiber(ROOT).any(|(_, r)| !r.is_empty()) {
            return None;
        }
        let mag = match b.bundle.as_deref() {
            Some(bundle) => {
                let (pay, rems) = crate::transducer::family::form_descent(f, bundle, crystal);
                if rems.iter().any(|r| !r.is_empty()) {
                    return None;
                }
                pay
            }
            None => form_mag(f),
        };
        let e = sentence::as_element_form(f, mag)?;
        let (loops, torsion, certificates, axes) =
            (f.loops.len(), f.torsion.len(), f.certificates.len(), f.axes.len());
        let retire = b.glued_by.is_empty();
        if retire {
            let old = b.clone();
            self.retract_incidence(key, &old);
            self.bubbles.remove(key);
            self.dirty.remove(key);
        }
        self.formed += 1;
        eprintln!(
            "  form crystallized {:02x}{:02x}… loops {loops} torsion {torsion} certificates {certificates} axes {axes} carried {mag:?}",
            e.address()[0],
            e.address()[1]
        );
        Some(e)
    }

    /// Classes on the contemplation, any degree.
    pub fn classes_held(
        &self,
    ) -> Vec<(crate::transducer::sentence::ClassVector, Rotor, usize)> {
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
            for (t, d) in p.iter_mut().zip(&b.potential) {
                *t = t.saturating_add(*d);
            }
        }
        p
    }

}
