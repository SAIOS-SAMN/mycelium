// 25 — THE CONTEMPLATION: every bubble a mind is holding at once, and the
// incidence between them.
//
//! `Contemplation` is the complex of held thoughts: the bubbles, their
//! remainders, the horizon of classes seeded from the syllabus, and the
//! cavity. Holding a form admits a vertex and links it to what it meets.

use super::*;

/// The complex over bubbles. by_chart is the incidence trie that makes
/// every connection a read.
///
/// X3: THE BUBBLES ARE THE OBJECT, and every walk of them below reads them
/// with `scoped`. A contemplation IS the set of thoughts one mind holds at
/// once — there is no coarser coordinate above it to descend from, and a
/// bubble left out of a cohesion, a component count or a shed is a thought
/// the mind holds and does not account for. The carriers INSIDE a bubble —
/// its cover, its remainders, its nerve's sections — are scoped by the
/// bubble the same way, and `by_chart` is the one carrier here that is
/// read by a real coordinate: `link(chart)`, the incidence read this type
/// exists for.
/// HOW LONG A SPEAKING FORM DELIBERATED, and which path it came by.
///
/// Lane 𝒯 is the scene-state span the bundle stood over, and a span of zero
/// has two entirely different causes: a form that was HELD and has not yet
/// aged, and a form that was never held at all because it closed in the
/// stroke it was extracted in. Reporting both as `0` is what made 𝒯
/// unreadable on the wire — 46 spoken magnitudes at c62940b, all `T = 0`,
/// with no way to tell which path produced any of them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Deliberation {
    /// Held in contemplation for this many receipts of TUniversal.
    Held(u64),
    /// Never held: minted in the stroke it was extracted in.
    Fresh,
}

impl Deliberation {
    /// The span as lane 𝒯 carries it. A form that was never held stood over
    /// no receipts, which is a reading of zero and not a placeholder.
    pub fn span(self) -> u64 {
        match self {
            Deliberation::Held(t) => t,
            Deliberation::Fresh => 0,
        }
    }

    /// The word a journal line names the path by.
    pub fn label(self) -> &'static str {
        match self {
            Deliberation::Held(_) => "held",
            Deliberation::Fresh => "fresh",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Contemplation {
    pub bubbles: Keyed<Bubble>,
    pub by_chart: Keyed<Keyed<()>>,
    pub by_cell: Keyed<Keyed<()>>,
    pub complex: Complex,
    /// Bubbles whose cover still has work; written when dirty is set.
    pub(crate) dirty: Keyed<()>,
    /// The ring's resume cursor: `None` is a walk that has not started,
    /// which is an absence and not an empty key (rule 11, X3).
    pub(crate) expand_cursor: Option<Vec<u8>>,
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
    /// EXPANSION 4 — THE VORTICES BY THEIR OWN COORDINATE: the after-class
    /// a vortex is held under → the bubbles holding one there. A collapse
    /// is asked at a class key (`collapse_vortex`), and the bubbles that
    /// can answer are the ones filed under it — a keyed read, never a walk
    /// of every bubble asking each whether it holds that class (rule 1).
    /// Written where a bubble's incidence is admitted and wherever a pivot
    /// rewrote a form (`refile_vortex`); not persisted, rebuilt as the
    /// other indices are.
    pub(crate) by_vortex: Keyed<Keyed<()>>,
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
    /// THE SPAN A FORM'S IDEA HAS STOOD OVER, in receipts — lane 𝒯, the
    /// same `held_since(now, moved_k)` that `expand_with` hands `pivot_loop`
    /// as its `t_extent`. One quantity, one reader (law 12).
    ///
    /// A form this contemplation does not hold stood over NO receipts: it
    /// was minted in the stroke it was extracted in. That is a reading of
    /// zero, not a placeholder for one. `moved_k` is stamped from `now_k`
    /// and this clock only advances, so the subtraction cannot go backwards
    /// while the contemplation is the one holding both.
    /// WHERE A SPEAKING FORM CAME FROM, and how long it deliberated. The two
    /// are one reading: a span of zero from a held bubble and a span of zero
    /// from a form that was never held are different facts, and reporting
    /// both as `0` is what made 𝒯 unreadable on the wire — 46 magnitudes
    /// with `T = 0` and no way to tell which path produced them.
    ///
    /// `None` where the clock went backwards — a K below the mark is a broken
    /// counter, and a span read off one is a measurement this contemplation
    /// cannot make, never a zero (rule 11).
    pub fn span_of(&self, form: &crate::transducer::sentence::Form) -> Option<Deliberation> {
        let key = super::idea::idea_key(form);
        match self.bubbles.get(key.as_bytes()) {
            // Held nowhere: minted in the stroke it was extracted in, so it
            // stood over no receipts. That is a reading of zero.
            None => Some(Deliberation::Fresh),
            Some(b) => super::idea::held_since(self.now_k, b.moved_k).map(Deliberation::Held),
        }
    }

    /// The TUniversal this contemplation was last stamped with. See
    /// [`Deliberation`] for what a span read against it means.
    pub fn now_k(&self) -> u64 {
        self.now_k
    }

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
            .scoped()
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
        for (_, b) in self.bubbles.scoped() {
            for (ck, _) in b.cover.scoped() {
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

    /// The cells every cavity entry holds together. One entry is a whole
    /// remainder, so the entry count says how many retirements happened and
    /// this says what they kept — and a thousand entries of one cell and ten
    /// of a thousand are the same count and not the same carrier.
    pub fn cavity_cells(&self) -> usize {
        self.cavity.scoped().map(|(_, r)| r.len()).sum()
    }

    /// WHAT THE CONTEMPLATION ACTUALLY HOLDS (§8.1, measurement only).
    ///
    /// `ideas` counts BUBBLES, and a bubble is not a unit of memory. Measured
    /// on the fleet: voices at 328–873 MB with `ideas` 66–86, `genome` 67–73,
    /// `dialect` 6–13 and `cavity` 0 — every counted carrier flat and small
    /// while the process grows by roughly a gigabyte an hour. The cavity was
    /// the candidate and this cleared it; nothing else was readable.
    ///
    /// These are the carriers a bubble holds that no line reports, and they
    /// are the ones with weight in them:
    /// - `nerve.sections` — a `Section` carries a γ and a carriage, and the
    ///   tree's own note at `Acting::carriage` is that a versor is 1024
    ///   bytes.
    /// - `nerve` cells across every degree, each carrying a `Rotor`.
    /// - `horizon` — one `(Rotor, Order, Standing)` per class the idea met,
    ///   and `paint_standing` writes one per standing loop of every form
    ///   held. A rotor is sixteen `BigRational`, each of them heap.
    /// - `bundle` — the training pairs, COPIED per bubble.
    /// - `certificates` and `vortex`, which hold certificates carrying class
    ///   keys and moves.
    ///
    /// Counts and never bytes: a size in bytes would be a number this code
    /// cannot derive exactly, and rule 11 refuses an invented one. What the
    /// counts do is say WHICH carrier grows, which is the question.
    pub fn weight(&self) -> [usize; 5] {
        let mut out = [0usize; 5];
        for (_, b) in self.bubbles.scoped() {
            out[0] += b.nerve.sections.len();
            out[1] += b.nerve.degrees.iter().map(|d| d.len()).sum::<usize>();
            out[2] += b.horizon.len();
            out[3] += b.bundle.as_ref().map_or(0, |p| p.len());
            out[4] += b.certificates.len()
                + b.vortex.scoped().map(|(_, v)| v.len()).sum::<usize>();
        }
        out
    }

    /// THE CAVITY ENTRIES A CHART CAN RE-HOLD, read at that chart. The same
    /// descent `pivot_on` makes, exposed so the keying can be asserted
    /// without the drive: a cavity keyed by the bubble alone answers nothing
    /// here, which is why the drain had to copy the whole carrier instead.
    ///
    /// `None` for a chart with no key — an empty prefix names no coordinate,
    /// and a caller whose chart turned out empty must say which reading it
    /// wanted (rule 11, `Prefix::new`).
    pub fn cavity_keys_under(
        &self,
        inv: &crate::transducer::stalk::Invariant,
    ) -> Option<Vec<Vec<u8>>> {
        let k = inv.key();
        Some(self.cavity.fiber_at(k.as_bytes())?.map(|(k, _)| k).collect())
    }

    pub fn remainders(&self) -> Vec<&[RemCell]> {
        self.bubbles.scoped().map(|(_, b)| b.remainder.as_slice()).collect()
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
        let charts: Vec<Vec<u8>> = b.cover.scoped().map(|(k, _)| k).collect();
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
        let classes: Vec<Vec<u8>> = b.vortex.scoped().map(|(k, _)| k).collect();
        for ck in classes {
            if let Some(m) = self.by_vortex.get_mut(&ck) {
                m.remove(key);
            }
        }
        self.complex.drop_vertex(key);
    }

    /// THE VORTICES A BUBBLE HOLDS, FILED AT THEIR CLASSES. A pivot rewrites
    /// the form in place (`take_form`) and may leave a vortex on it that
    /// was not there when the bubble's incidence was admitted; the index
    /// is re-read off the bubble at every such site. An entry a collapse
    /// consumed leaves with it (`collapse_on`); one for a class the bubble
    /// no longer holds answers `None` at the collapse and is dropped there.
    pub(crate) fn refile_vortex(&mut self, key: &[u8]) {
        let classes: Vec<Vec<u8>> = match self.bubbles.get(key) {
            Some(b) => b.vortex.scoped().map(|(k, _)| k).collect(),
            None => return,
        };
        for ck in classes {
            self.by_vortex.nested_insert(&ck, key, ());
        }
    }

    /// EXPANSION 4 — COLLAPSE EVERY VORTEX A ROTOR ANSWERS. A vortex holds
    /// several certified discharges of one loop; a ruled word whose rotor
    /// is one of its candidates decides it, and the differences between
    /// that candidate and the others — rotations the mind measured (37.7)
    /// — ride out to be adjoined, each with the chart the deciding word
    /// was met on.
    ///
    /// Read at the vortex's own coordinate: the rotor's class key names
    /// the bubbles holding a vortex of that class (`by_vortex`), and the
    /// rotor's own key names the candidate. It was read under the arriving
    /// word's CHART, which for a learned form is the form's chart and
    /// shares no prefix with a bubble's body charts, and through
    /// `cech::rotor_of`, which answers `None` for a form: on 48bfdbd 24
    /// vortices formed in ten minutes and none was ever asked.
    pub(crate) fn collapse_on(
        &mut self,
        rotors: &[Rotor],
        chart: &Invariant,
        by: &'static str,
    ) -> Vec<(Rotor, Invariant)> {
        let mut out = Vec::new();
        for r in rotors {
            let Some(ck) = r.class_key() else {
                continue;
            };
            let rk = r.key();
            let holders: Vec<Vec<u8>> = self.by_vortex.link(ck.as_bytes()).map(|(k, _)| k).collect();
            for bk in holders {
                let collapsed = self
                    .bubbles
                    .get_mut(&bk)
                    .and_then(|b| b.collapse_vortex(ck.as_bytes(), rk.as_bytes(), by));
                match collapsed {
                    Some(syms) => {
                        if let Some(m) = self.by_vortex.get_mut(ck.as_bytes()) {
                            m.remove(&bk);
                        }
                        for s in syms {
                            out.push((s, chart.clone()));
                        }
                    }
                    // The candidate was not this rotor, or the class is no
                    // longer held there: nothing collapsed, and an index
                    // entry for a class the bubble no longer holds leaves.
                    None => {
                        let held = self.bubbles.get(&bk).is_some_and(|b| b.vortex.contains(ck.as_bytes()));
                        if !held {
                            if let Some(m) = self.by_vortex.get_mut(ck.as_bytes()) {
                                m.remove(&bk);
                            }
                        }
                    }
                }
            }
        }
        out
    }

    /// LINEAGE THROUGH MUTATION: the entry follows the form it holds.
    ///
    /// A bubble's key is its form's address, and a pivot REWRITES THE FORM IN
    /// PLACE — `expand_all` takes the slot by the key it entered with and
    /// `expand_with` calls `take_form` inside it. So the bubble keeps its
    /// `moved_k` and everything it has built, while its key stops matching
    /// the form it now holds, and every reader that looks it up by the
    /// current form misses it: `span_of` answers "not held" for a bubble that
    /// has been deliberating for thousands of receipts, and `hold_form`
    /// inserts a SECOND bubble for the same lineage while the first is
    /// orphaned under a key nothing computes any more.
    ///
    /// That is why 𝒯 read zero on every spoken word even after the clock
    /// (R5) and the phase-change stamp (R6) were both correct: a form with an
    /// open remainder speaks only once it closes, closing mutates it, and
    /// mutation lost the lookup. Measured at c62940b: 1694 holds against 20
    /// spoken forms, and `span_of` never once finding a bubble.
    ///
    /// The entry moves with its inception stamp and its accumulated state, so
    /// a resolved sentence carries the true extent of its deliberation —
    /// the bubble that deliberated is the bubble that speaks.
    ///
    /// Where the destination is already held, two lineages have converged on
    /// one form. The survivor is the one already under that key, and it takes
    /// the EARLIER inception of the two: the deliberation began when the
    /// first of them began, and a later stamp would shorten a span that was
    /// actually stood.
    pub(crate) fn rekey(&mut self, from: &[u8], to: &[u8]) {
        if from == to {
            return;
        }
        let Some(moving) = self.bubbles.remove(from) else {
            return;
        };
        self.retract_incidence(from, &moving);
        let was_dirty = self.dirty.remove(from).is_some();
        match self.bubbles.get_mut(to) {
            Some(standing) => {
                standing.moved_k = standing.moved_k.min(moving.moved_k);
            }
            None => {
                self.admit_incidence(to, &moving);
                self.bubbles.insert(to, moving);
            }
        }
        if was_dirty {
            self.dirty.insert(to, ());
        }
    }

    pub(crate) fn admit_incidence(&mut self, key: impl AsRef<[u8]>, b: &Bubble) {
        let key = key.as_ref();
        for (ck, _) in b.cover.scoped() {
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
        for (ck, _) in b.vortex.scoped() {
            self.by_vortex.nested_insert(&ck, key, ());
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
        for (bk, _) in ideas.scoped() {
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
                b.cover.scoped().map(|(k, _)| k).collect::<Vec<_>>(),
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
        let nbs: Vec<Vec<u8>> = neighbors.scoped().map(|(k, _)| k).collect();
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
            for (ck, _) in cover_of(&stalk::invariant(r)).scoped() {
                cover.insert(&ck, ());
            }
        }
        if !f.remainder.is_empty() {
            for (ck, _) in cover_of(&stalk::invariant(&f.remainder)).scoped() {
                cover.insert(&ck, ());
            }
        }
        let mut glues = Keyed::new();
        for v in &f.vertices {
            match v {
                // EVERY CHART THE VERTEX ACTS ON is a chart the bubble's
                // cover holds: a vertex acting on several is on several.
                Vertex::Word { acts, .. } => {
                    for (ck, _) in acts.scoped() {
                        cover.insert(&ck, ());
                    }
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
        // RE-HOLDING AN UNMUTATED FORM IS NOT MOVEMENT (ruled 2026-09-14).
        //
        // `moved_k` is the PHASE CHANGE stamp — "TUniversal when this
        // bubble's charts last moved" — and this rebuilt the whole bubble on
        // every hold, stamping `moved_k = now_k` each time. D8 re-holds a
        // withheld form on every stroke, so the birth coordinate was
        // re-stamped every stroke and no bubble could ever age:
        // `held_since(now_k, moved_k)` was zero by construction, `t_extent`
        // perpetually zero, lane 𝒯 unable to accumulate. Measured at
        // 9a08fba: 58 of 58 spoken magnitudes with 𝒯 = 0, and `span_of`
        // never once refusing — it was answering `Some(0)` every time.
        //
        // A form retained across an evaluation stroke has moved ZERO in
        // configuration space. The key IS the topology and the carriages:
        // `form_vertex_key` writes every vertex, every member and every gauge
        // (law 2), so a bubble found under this key is that same form and
        // nothing about it has changed.
        //
        // What it has ACCUMULATED is kept with it — the complex it built, the
        // cover it expanded, the classes its horizon met, who glued it.
        // Rebuilding those was not only a clock fault: it threw the expansion
        // away every stroke, which is what kept re-entering the cover and
        // re-stamping the clock in the first place.
        //
        // `moved_k` still advances where a transformation actually mutates
        // the charts: `expand_with` stamps it when a chart enters the cover,
        // and a pivot rewrites the form, which changes the key and starts a
        // new bubble at its own phase change.
        let mut bubble = bubble;
        if let Some(prior) = self.bubbles.get(key.as_bytes()) {
            bubble.moved_k = prior.moved_k;
            bubble.nerve = prior.nerve.clone();
            bubble.cover = prior.cover.clone();
            bubble.glued_by = prior.glued_by.clone();
            // The horizon is a union: this stroke's painting of the form's
            // standing classes, over the classes expansion has already met.
            for (hk, hv) in prior.horizon.scoped() {
                if !bubble.horizon.contains(&hk) {
                    bubble.horizon.insert(&hk, hv.clone());
                }
            }
        }
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
        seen.scoped().map(|(k, _)| k).collect()
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
        if b.remainders.scoped().any(|(_, r)| !r.is_empty()) {
            return None;
        }
        let mag = match b.bundle.as_deref() {
            Some(bundle) => {
                let (pay, rems) = crate::transducer::family::form_deflation(f, bundle, crystal);
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
        let keys: Vec<Vec<u8>> = self.bubbles.scoped().map(|(k, _)| k).collect();
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
        for (_, b) in self.bubbles.scoped() {
            n += b.horizon.len();
        }
        n
    }

    pub fn potential(&self) -> [u64; 4] {
        let mut p = [0u64; 4];
        for (_, b) in self.bubbles.scoped() {
            for (t, d) in p.iter_mut().zip(&b.potential) {
                *t = t.saturating_add(*d);
            }
        }
        p
    }

}
