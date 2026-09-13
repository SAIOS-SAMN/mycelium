// 25 — THE IDEA: one bubble, its remainder, and the volume it holds.
//
//! A `Bubble` is a thought under contemplation: the form it carries, the
//! remainder cells it has not accounted for, the words it glues and is
//! glued by, and the class it was seeded from.

use super::*;

/// THE KEYED READ OF A HELD WORD'S SECTION BY ADDRESS: the rotation the
/// chart names and the carriage it names it at. Both halves, because a
/// transition is `D(ρ)·T(t)·R` and a reader that answers `R` alone makes
/// every chart read as if it sat at the origin — which is not a fact about
/// the charts but about the reader's type.
pub type SectionAt<'a> = &'a dyn Fn(&[u8]) -> Option<(Rotor, crate::transducer::element::Versor)>;

/// One class the mesh or a mind holds on a chart: key, order, representative.
#[derive(Clone, Debug)]
pub struct ClassRecord {
    pub key: crate::transducer::address::Address,
    pub order: Order,
    pub representative: Rotor,
}

pub(crate) type RemCell = (i64, i64, u8, u8);

pub(crate) fn rem_cell_key(r: i64, c: i64) -> crate::transducer::address::Address {
    let mut buf = vec![crate::transducer::address::TAG_CELL];
    buf.extend_from_slice(&r.to_le_bytes());
    buf.extend_from_slice(&c.to_le_bytes());
    crate::transducer::address::Address::new(buf)
}

/// Receipts since a bubble's charts last moved, on the kernel's own K
/// (law 9). The mark is a K this clock has already passed, so the extent
/// is exact.
///
/// `None` where the mark is ABOVE the clock: a K that went backwards is a
/// broken count, and an extent of zero read off one is a measurement this
/// clock cannot make, not a bubble that moved just now (rule 11).
pub(crate) fn held_since(now: u64, moved_k: u64) -> Option<u64> {
    now.checked_sub(moved_k)
}

/// The address of a chain A5 has accepted. Total: the type is the proof,
/// so there is nothing here to refuse.
pub(crate) fn remainder_addr(
    remainder: &element::CanonicalChain,
) -> crate::transducer::address::Address {
    element::from_canonical(remainder).address()
}

/// The idea's key: the form's vertex key. Remainder is cargo, not the name.
pub(crate) fn idea_key(f: &Form) -> crate::transducer::address::Address {
    sentence::form_vertex_key(f)
}

pub(crate) fn form_mag(f: &Form) -> [u64; 4] {
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

pub(crate) fn rem_cells_of(b: &Bubble) -> Vec<RemCell> {
    let mut out = b.remainder.to_vec();
    for (_, r) in b.remainders.scoped() {
        out.extend_from_slice(r);
    }
    out
}

pub(crate) fn cover_of(inv: &Invariant) -> Keyed<()> {
    let mut c = Keyed::new();
    for p in inv.prefixes() {
        c.insert(p.key(), ());
    }
    c
}

pub(crate) fn volume_of(rems: &Keyed<Vec<RemCell>>, verts: &[impl AsRef<[u8]>]) -> [u64; 4] {
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

pub(crate) fn union_rem(a: &[RemCell], b: &[RemCell]) -> Vec<RemCell> {
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
    /// A5's chain and the translator that places it in the scene (§2.1):
    /// the chain is the identity this bubble is addressed by, the
    /// translator is where it sits. A bubble holding a chain A5 refuses is
    /// never built, so the address never refuses.
    pub remainder: element::PlacedChain,
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
    pub moved_k: u64,
    /// Addresses this idea glued (crystallized forms it rests on).
    pub glues: Keyed<()>,
    /// Ideas that glued this bubble; a non-empty set is not shed.
    pub glued_by: Keyed<()>,
    /// Filed torsion `(σ, ℓ, chord)` keyed by class, written from `pivot_loop`.
    pub torsion: Keyed<crate::transducer::sentence::FiledLoop>,
    /// Certified discharges this idea's crystallization rests on (A20).
    pub certificates: Keyed<PivotCertificate>,
    /// Incomparable discharges held as cells (25.1) until a glue or ruling.
    /// Outer key is the after class; inner is the discharging word's M1 key.
    pub vortex: Keyed<Keyed<PivotCertificate>>,
    /// The form this idea is (chapter 30 step 6).
    pub form: Option<Form>,
    /// Pairs the idea germinated from; trial speaks over them for μ⃗.
    pub bundle: Option<Vec<crate::transducer::family::TrainPair>>,
    /// Per-pair remainder chains, keyed by pair index. `remainder` is their union.
    pub remainders: Keyed<Vec<RemCell>>,
}

impl Bubble {
    /// Read off what the bubble holds on every call, never stored beside
    /// it (rule 6, law 12): the remainder carries A5's proof in its type,
    /// so the address never refuses.
    pub fn key(&self) -> crate::transducer::address::Address {
        match &self.form {
            Some(f) => idea_key(f),
            None => remainder_addr(&self.remainder),
        }
    }

    /// THE ROTOR THIS BUBBLE'S FORM SPEAKS, when it speaks ONE.
    ///
    /// `find_map` stood here and returned the FIRST Word vertex's rotor. A
    /// form with several word vertices has several rotors, and which one
    /// arrives first is an insertion order — so this answered a different
    /// motion on a node that took the same bodies in differently (rule 15,
    /// and the D10 representative-reader fault at a new site).
    ///
    /// It costs more than a wrong answer downstream: `rotor_between`
    /// composes two of these into `rb ∘ ra⁻¹` and calls that the transport
    /// between two bubbles, so two nodes could breed different transports
    /// from identical mathematics.
    ///
    /// A form whose word vertices all speak one rotor up to projective
    /// equality has that rotor. One that speaks several has no single one,
    /// and says so — absence, not a representative. Nothing here is chosen.
    pub(crate) fn rotor(&self) -> Option<Rotor> {
        let f = self.form.as_ref()?;
        let mut held: Option<Rotor> = None;
        for v in &f.vertices {
            let Vertex::Word { derivation, .. } = v else {
                continue;
            };
            let r = derivation.rotor();
            match &held {
                None => held = Some(r),
                Some(h) if h.projective_eq(&r) => {}
                Some(_) => return None,
            }
        }
        held
    }

    /// THE SECTIONS THIS BUBBLE'S FORM CARRIES: one per word vertex per
    /// chart it acts on — the rotation it speaks there and the carriage it
    /// speaks it by. Keyed by both, so two vertices holding one section are
    /// one section and one vertex acting on two charts is two.
    ///
    /// [`Bubble::rotor`] answers ONE rotor or absence, which is right for
    /// `rotor_between` — two bubbles have one transport between them or they
    /// have none. It is wrong for the cover: a form that speaks several
    /// rotors has several sections, and asking it for one and taking the
    /// absence dropped exactly the multi-section forms D7–D10 exist to
    /// carry. A reader whose non-matching arm leaves no reading is the same
    /// fault in a different place.
    pub(crate) fn sections(&self) -> Keyed<(Rotor, crate::transducer::element::Versor)> {
        let mut out = Keyed::new();
        let Some(f) = self.form.as_ref() else {
            return out;
        };
        for v in &f.vertices {
            let Vertex::Word { derivation, acts, .. } = v else {
                continue;
            };
            let r = derivation.rotor();
            // index: keyed fibre of this vertex's members — its own charts,
            // never a walk over the form (rule 1).
            for (_, a) in acts.scoped() {
                let mut k = r.key().into_vec();
                a.carriage.to_bytes(&mut k);
                if !out.contains(&k) {
                    out.insert(k, (r.clone(), (*a.carriage).clone()));
                }
            }
        }
        out
    }

    /// Walk outward on the cover trie from this bubble's cover, one chart
    /// per stroke, only over overlaps with nonzero volume. Extended grows
    /// the cover; Congruent grows it with a torsion axis; Class writes the
    /// horizon and stops that branch.
    pub fn expand(&mut self, atlas: &dyn AtlasRead, crystal: &dyn Crystal, budget: &mut Budget) -> usize {
        self.expand_with(atlas, &|_| None, 0, crystal, budget).0
    }

    /// `atlas` is the language this thought expands over (the working
    /// atlas, 24.1); `section_at` is the keyed read of a held word's
    /// SECTION by address — its rotation and its carriage — so the Čech arm
    /// reads the word the chart names and never a table built for the
    /// stroke.
    ///
    /// It was `Fn(&[u8]) -> Option<Rotor>`, and a carriage could not be
    /// obtained through it even in principle. The one live caller reads
    /// `working_word(a)` and had the whole element in hand; the type threw
    /// the placement away before `extend` could ask about it.
    pub fn expand_with(
        &mut self,
        atlas: &dyn AtlasRead,
        section_at: SectionAt,
        now: u64,
        crystal: &dyn Crystal,
        budget: &mut Budget,
    ) -> (usize, [u64; 4]) {
        // THE POTENTIAL IS A READING, NEVER A GATE (L7b, law 9): the lane
        // measure of everything this idea holds open — its form's remainder
        // and its per-pair remainders, the same cells `admit_incidence`
        // keys — with T the receipts since its charts last moved, zero when
        // zero. It was floored at one on T so that the gate below would
        // pass for a form with no remainder, which paced every such idea by
        // a clock that admitted nothing (Cut B+1 C4). The pivot loop runs
        // on every dirty idea under the budget (25.1); the budget is the
        // only thing that stops it.
        self.potential = element::measure_lanes(&rem_cells_of(self));
        // A clock that went backwards measures no extent, and an idea whose
        // T cannot be read is not expanded against one.
        let Some(t) = held_since(now, self.moved_k) else {
            return (0, [0; 4]);
        };
        self.potential[2] = t;
        if budget.remaining() == 0 {
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
        let last = atlas.children(&peel, after, budget, &mut |child| {
            let ck = child.key();
            if self.cover.contains(&ck) {
                return;
            }
            spent += 1;
            // index: keyed fiber at this sibling chart
            let members = atlas.members(&child);
            let mine = self.sections();
            let vol = self.potential;
            // EVERY MEMBER AGAINST EVERY SECTION. Three readers collapsed
            // here and the classifier downstream could not see past any of
            // them.
            //
            // `members.first()` took the head of the fibre and made it THE
            // chart, under an `#[expect]` arguing the head is the least
            // address so every node reads the same rotor. That is canonical
            // and it is still a member chosen by its position (rule 15): a
            // chart with several members has several sections, and one of
            // them was standing for all of them.
            //
            // `self.rotor()` asked the form for ONE rotor and returned on
            // absence, so every form speaking several was dropped from the
            // cover leaving no reading at all.
            //
            // And both sections were built by `Section::at`, whose carriage
            // is the identity by construction — so `extend`'s `flat` was
            // true on this path whatever the placements were, and a curved
            // holonomy on a non-identity carriage could not be REPRESENTED,
            // let alone filed. Measured across two corpus slices: 2778
            // classified cover loops, flat 2778, filed 0, standing 0. The
            // carriage field's own doc says it "hid in exactly the readings
            // that were already zero".
            //
            // The bound is content on both sides — this bubble's own
            // sections and this chart's own fibre — which is the boundary
            // rule 10 allows, not a search.
            let mut closed = true;
            let mut met = false;
            for head in &members {
                let Some((gamma_b, car_b)) = section_at(head.as_slice()) else {
                    continue;
                };
                for (_, (gamma_a, car_a)) in mine.scoped() {
                    met = true;
                    let cell = CechCell {
                        vertices: {
                            let mut v = vec![self.key().into_vec(), head.clone()];
                            v.sort_unstable();
                            v
                        },
                        coefficient: gamma_b.clone(),
                        // What the cell expects of the far side, both
                        // halves, exactly as the coefficient above is the
                        // far side's rotation.
                        carriage: car_b.clone(),
                        volume: vol,
                        fill: Fill::Open,
                    };
                    let a = Section {
                        addr: self.key(),
                        gamma: gamma_a.clone(),
                        carriage: car_a.clone(),
                    };
                    let b = Section {
                        addr: crate::transducer::address::Address::from(head.clone()),
                        gamma: gamma_b.clone(),
                        carriage: car_b.clone(),
                    };
                    match cech::extend(&a, &b, &cell) {
                        Fill::Filled(..) => {}
                        Fill::Congruent(g, ell) => {
                            self.nerve.file_congruent(self.key(), head, g, ell);
                        }
                        Fill::Class(_, r) => {
                            closed = false;
                            if r.class_key().is_none_or(|k| !self.horizon.contains(k.as_bytes())) {
                                horizon_put(&mut self.horizon, *r, Standing::Held);
                            }
                        }
                        // 37.7 at a cell: a rotation the seventeen do not
                        // spell, which seeds no horizon and closes nothing.
                        Fill::Unspellable(_) => closed = false,
                        Fill::Open | Fill::Already => closed = false,
                    }
                }
            }
            // THE CHART ENTERS THE COVER WHEN EVERY SECTION CLOSES ON IT,
            // not when one of them does. A chart one section fills and
            // another stands a class on is not covered — it is the chart
            // the standing class is about. The old reader could not state
            // this because it had one section and one member.
            if met && closed {
                self.cover.insert(ck, ());
                self.moved_k = now;
            }
        });
        if let Some(k) = last {
            self.cursor = k;
        }
        let mut pivoted = [0u64; 4];
        if budget.remaining() > 0 {
            if let Some(mut form) = self.form.clone() {
                let key = self.key();
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
    pub(crate) fn collapse_vortex(&mut self, class: &[u8], word: &[u8], by: &'static str) -> bool {
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

    /// Total: the form's remainder carries A5's proof in its own type, so
    /// there is nothing to ask here (§4.4).
    pub(crate) fn take_form(&mut self, f: Form) {
        self.remainder = f.remainder.clone();
        self.torsion = f.torsion.clone();
        self.certificates = f.certificates.clone();
        self.vortex = f.vortex.clone();
        self.form = Some(f);
    }
}
