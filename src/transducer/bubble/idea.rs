// 25 — THE IDEA: one bubble, its remainder, and the volume it holds.
//
//! A `Bubble` is a thought under contemplation: the form it carries, the
//! remainder links it has not accounted for, the words it glues and is
//! glued by, and the class it was seeded from.

use super::*;

/// THE KEYED READ OF A HELD WORD'S SECTION BY ADDRESS: the rotation the
/// chart names and the carriage it names it at. Both halves, because a
/// transition is `D(ρ)·T(t)·R` and a reader that answers `R` alone makes
/// every chart read as if it sat at the origin — which is not a fact about
/// the charts but about the reader's type.
pub type SectionAt<'a> = &'a dyn Fn(&[u8]) -> Option<(Rotor, crate::transducer::element::Versor)>;

/// ONE SECTION THIS IDEA HOLDS: the chart it stands on, the rotation it
/// speaks there, and the carriage it speaks it by. The chart is a
/// coordinate, which is what lets the cover be READ rather than searched
/// for.
pub type Held = (Invariant, Rotor, crate::transducer::element::Versor);

/// One class the mesh or a mind holds on a chart: key, order, representative.
#[derive(Clone, Debug)]
pub struct ClassRecord {
    pub key: crate::transducer::address::Address,
    pub order: Order,
    pub representative: Rotor,
}

/// THE KEY A REMAINDER LINK IS FILED UNDER: its site, written out. It opens
/// with the simplex tag, the one byte a site key and a simplex key share.
pub(crate) fn remainder_site_key(r: i64, c: i64) -> crate::transducer::address::Address {
    let mut buf = vec![crate::transducer::address::TAG_SIMPLEX];
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

/// Every remainder link this bubble holds: the form's own and each pair's.
pub(crate) fn remainder_links_of(b: &Bubble) -> Vec<Link> {
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

pub(crate) fn volume_of(rems: &Keyed<Vec<Link>>, verts: &[impl AsRef<[u8]>]) -> [u64; 4] {
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

pub(crate) fn union_rem(a: &[Link], b: &[Link]) -> Vec<Link> {
    let mut out = a.to_vec();
    let mut have = Keyed::new();
    for &(r, c, _, _) in a {
        have.insert(remainder_site_key(r, c), ());
    }
    for link in b {
        let k = remainder_site_key(link.0, link.1);
        if !have.contains(&k) {
            have.insert(k, ());
            out.push(*link);
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
    /// Incomparable discharges held as candidates (25.1) until a glue or
    /// ruling. Outer key is the after class; inner is the discharging
    /// word's M1 key.
    pub vortex: Keyed<Keyed<PivotCertificate>>,
    /// The form this idea is (chapter 30 step 6).
    pub form: Option<Form>,
    /// Pairs the idea germinated from; trial speaks over them for μ⃗.
    pub bundle: Option<Vec<crate::transducer::family::TrainPair>>,
    /// Per-pair remainder chains, keyed by pair index. `remainder` is their union.
    pub remainders: Keyed<Vec<Link>>,
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
    /// A form with several word vertices has several rotors, and which one
    /// arrives first is an insertion order, so a representative read would
    /// answer a different motion on a node that took the same bodies in
    /// differently (rule 15). It would cost more than a wrong answer
    /// downstream: `rotor_between` composes two of these into `rb ∘ ra⁻¹`
    /// and calls that the transport between two bubbles, so two nodes
    /// would breed different transports from identical mathematics.
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
    /// rotors has several sections, and a reader that asks for one and
    /// takes the absence drops exactly the multi-section forms.
    pub(crate) fn sections(&self) -> Keyed<Held> {
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
                let mut k = a.chart.key().into_vec();
                r.put_canonical(&mut k);
                // A4 output (17.2): one projective carriage is one section.
                // Raw bytes made a carriage and its negation two.
                a.carriage.put_canonical(&mut k);
                if !out.contains(&k) {
                    out.insert(
                        k,
                        (a.chart.clone(), r.clone(), (*a.carriage).clone()),
                    );
                }
            }
        }
        out
    }

    /// MEET THIS IDEA'S OWN CHARTS. Each of its sections stands on a chart;
    /// the other sections there are that chart's fibre, read at its key. A
    /// meet that fills grows the cover, one that is congruent grows it with
    /// a torsion axis, and one that stands a class writes the horizon and
    /// leaves the chart out of the cover.
    pub fn expand(&mut self, atlas: &dyn AtlasRead, crystal: &dyn Crystal, budget: &mut Budget) -> usize {
        self.expand_with(atlas, &|_| None, 0, crystal, budget).0
    }

    /// `atlas` is the language this thought expands over (the working
    /// atlas, 24.1); `section_at` is the keyed read of a held word's
    /// SECTION by address — its rotation and its carriage — so the Čech arm
    /// reads the word the chart names and never a table built for the
    /// stroke, and the word's placement reaches `extend`.
    pub fn expand_with(
        &mut self,
        atlas: &dyn AtlasRead,
        section_at: SectionAt,
        now: u64,
        crystal: &dyn Crystal,
        budget: &mut Budget,
    ) -> (usize, [u64; 4], Vec<(Rotor, crate::transducer::stalk::Invariant)>) {
        // 37.7's candidates, carried out rather than dropped: the rotors this
        // stroke met that the seventeen cannot spell, each with the chart it
        // was met on. The bubble cannot mint — minting writes a word into the
        // language and the language is the entity's — so it carries them.
        let mut adjoined: Vec<(Rotor, crate::transducer::stalk::Invariant)> = Vec::new();
        // THE POTENTIAL IS A READING, NEVER A GATE (L7b, law 9): the lane
        // measure of everything this idea holds open — its form's remainder
        // and its per-pair remainders, the same links `admit_incidence`
        // keys — with T the receipts since its charts last moved, zero when
        // zero. Nothing is floored so that a gate would pass: the pivot
        // loop runs on every dirty idea under the budget (25.1), and the
        // budget is the only thing that stops it.
        self.potential = element::measure_lanes(&remainder_links_of(self));
        // A clock that went backwards measures no extent, and an idea whose
        // T cannot be read is not expanded against one.
        let Some(t) = held_since(now, self.moved_k) else {
            return (0, [0; 4], Vec::new());
        };
        self.potential[2] = t;
        if budget.remaining() == 0 {
            return (0, [0; 4], Vec::new());
        }
        let mut spent = 0usize;
        // THE CHARTS THIS IDEA MEETS ARE ITS OWN SECTIONS' CHARTS, and their
        // members are a keyed read at those coordinates. Nothing is walked
        // and nothing is discovered.
        //
        // A Čech simplex is the OVERLAP of two sections, and two sections
        // overlap when they are sections over the same chart. A sibling of
        // my chart that none of my vertices acts on is not an overlap; it
        // is a stranger, and `extend` asked about it would classify a
        // transition between bodies that share nothing. The cover is built
        // out of incidence in the sheaf, never out of adjacency in a trie,
        // and needs no cursor to resume: an idea's expansion depends on its
        // coordinates and not on where the last stroke stopped.
        //
        // The charts are already in hand. Every `Vertex::Word` names the
        // charts it acts on and the carriage it acts by at each, so the
        // sections are a reading of the form and the far side of each meet
        // is `members(chart)` — the fibre, keyed, at an address this idea
        // already holds. Both bounds are content: this idea's sections, and
        // one chart's own fibre (rule 10).
        //
        // EVERY MEMBER AGAINST EVERY SECTION, and both sections carry their
        // real carriage — without which `extend`'s `flat` is true by
        // construction and `Fill::Class` is unreachable.
        let mine = self.sections();
        let me = self.key();
        let vol = self.potential;
        for (_, (chart, gamma_a, car_a)) in mine.scoped() {
            let ck = chart.key();
            if self.cover.contains(&ck) {
                continue;
            }
            spent += 1;
            // index: keyed fibre at a chart this idea itself stands on
            let members = atlas.members(chart);
            let mut closed = true;
            let mut met = false;
            for head in &members {
                // A section does not overlap itself.
                if head.as_slice() == me.as_bytes() {
                    continue;
                }
                let Some((gamma_b, car_b)) = section_at(head.as_slice()) else {
                    continue;
                };
                met = true;
                let simplex = Simplex {
                    vertices: {
                        let mut v = vec![me.clone().into_vec(), head.clone()];
                        v.sort_unstable();
                        v
                    },
                    coefficient: gamma_b.clone(),
                    // What the simplex expects of the far side, both halves,
                    // exactly as the coefficient above is its rotation.
                    carriage: car_b.clone(),
                    volume: vol,
                    fill: Fill::Open,
                };
                let a = Section {
                    addr: me.clone(),
                    gamma: gamma_a.clone(),
                    carriage: car_a.clone(),
                };
                let b = Section {
                    addr: crate::transducer::address::Address::from(head.clone()),
                    gamma: gamma_b,
                    carriage: car_b,
                };
                match cech::extend(&a, &b, &simplex) {
                    Fill::Filled(..) => {}
                    Fill::Congruent(g, ell) => {
                        self.nerve.file_congruent(me.clone(), head, g, ell);
                    }
                    Fill::Class(_, r) => {
                        closed = false;
                        if r.class_key().is_none_or(|k| !self.horizon.contains(k.as_bytes())) {
                            horizon_put(&mut self.horizon, *r, Standing::Held);
                        }
                    }
                    // 37.7 at a simplex: a rotation the seventeen do not
                    // spell seeds no horizon, closes nothing, and is an
                    // adjunction candidate — the rotor the mind measured and
                    // has no letter for — carried out to the entity, which
                    // mints and stands it (`Derivation::adjoined`).
                    Fill::Unspellable(r) => {
                        closed = false;
                        adjoined.push((r.as_ref().clone(), chart.clone()));
                    }
                    Fill::Open | Fill::Already => closed = false,
                }
            }
            // THE CHART ENTERS THE COVER WHEN EVERY SECTION CLOSES ON IT,
            // not when one of them does. A chart one section fills and
            // another stands a class on is not covered — it is the chart the
            // standing class is about.
            if met && closed {
                self.cover.insert(ck, ());
                self.moved_k = now;
            }
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
                    match s {
                        // 37.7 from the loop's own residue, joining the
                        // cover's own candidates on the way out; the entity
                        // mints it through the one construction.
                        crate::transducer::sentence::Snap::Adjoin { rotor, chart, .. } => {
                            adjoined.push((rotor.as_ref().clone(), chart.clone()));
                        }
                        // A held loop of infinite order the basis did not
                        // discharge stands on the form as the demand the
                        // peel types its holonomy as — the prime it lacks,
                        // or the rotor it cannot spell, carried as witness
                        // (16.16 step 6, 50.4). A rotor the seventeen spell
                        // is no demand on the language (37.7 adjoins what
                        // the presentation lacks); the form's vertices
                        // carry no links, so the demand declares none.
                        crate::transducer::sentence::Snap::Demand { rotor, chart, .. } => {
                            if let Some(Err(kind)) = crate::transducer::alphabet::factor_rotor(rotor) {
                                form.stand_demand(kind, chart, Vec::new());
                            }
                        }
                        _ => {}
                    }
                }
                self.take_form(form);
            }
        }
        self.dirty = spent > 0 && budget.remaining() == 0;
        (spent, pivoted, adjoined)
    }

    /// THE VORTEX HERE STANDING AT A CONJUGACY STRATUM AND HOLDING `word`.
    ///
    /// A vortex is filed under the loop's own class vector (`vortex_put`),
    /// whose depth-one coordinate is the holonomy rotor's conjugacy class.
    /// A deciding word knows that stratum and nothing finer: not the
    /// carriage the loop stood at, not its hue. So the boundary is parsed
    /// out of each key through the class schema and compared at depth one
    /// — never a byte count, and never the whole key.
    ///
    /// The walk is this bubble's own carrier, which is one idea's boundary
    /// (rule 1). A key that does not parse as a class vector is not a
    /// vortex of this system and is passed over rather than matched.
    fn vortex_at_stratum(&self, stratum: &[u8], word: &[u8]) -> Option<Vec<u8>> {
        let schema = crate::transducer::sentence::ClassVector::class_schema();
        // index: this bubble's own vortices, its boundary
        for (class, candidates) in self.vortex.scoped() {
            if !candidates.contains(word) {
                continue;
            }
            let Ok(end) = schema.boundary(&class, 1) else {
                continue;
            };
            if class.get(..end) == Some(stratum) {
                return Some(class);
            }
        }
        None
    }

    /// A glue or a syllabus class standing at the stratum a held vortex's
    /// loop stands in applies the certificate keyed by the deciding move;
    /// the others of that vortex are not applied (dialect candidates).
    pub(crate) fn collapse_vortex(
        &mut self,
        stratum: &[u8],
        word: &[u8],
        by: &'static str,
    ) -> Option<Vec<crate::transducer::alphabet::Rotor>> {
        let class = self.vortex_at_stratum(stratum, word)?;
        let mut inner = self.vortex.remove(&class)?;
        let Some(win) = inner.remove(word) else {
            if !inner.is_empty() {
                self.vortex.insert(&class, inner);
            }
            return None;
        };
        // EXPANSION 4 — THE LOSERS ARE A SYMMETRY, NOT A TIE BROKEN. Two
        // candidates that both discharge one loop to one class prove the
        // loop admits `w_a⁻¹·w_b`, an element that stabilises that class
        // because each leaves it where the other does. That is a rotation
        // the mind MEASURED, which is 37.7's object exactly, and a collapse
        // that only cleared the vortex would throw it away — the mind
        // resolving its own superposition by forgetting one half of it.
        //
        // The symmetries are carried out, not minted here, for the reason
        // every other adjunction is: minting writes a word into the language
        // and a bubble does not hold the language.
        let losers: Vec<PivotCertificate> = inner.scoped().map(|(_, c)| c.clone()).collect();
        let symmetries = crate::transducer::sentence::symmetries(&win, &losers);
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
            "  vortex {:02x}{:02x}… collapsed by {by} to axis {axis:?}, {} symmetries kept",
            self.key()[0],
            self.key()[1],
            symmetries.len()
        );
        Some(symmetries)
    }

    /// THE CONJUGACY STRATA THIS BUBBLE'S VORTICES STAND AT: the depth-one
    /// coordinate of each outer key of `vortex`. The contemplation files
    /// the bubble under each (`by_vortex`), so a deciding word's own class
    /// key reaches it by a keyed read, and there is one construction of
    /// that coordinate for the writer and the reader (law 12).
    ///
    /// A key that does not parse as a class vector contributes no stratum
    /// rather than contributing its own bytes as one (rule 11).
    pub(crate) fn vortex_strata(&self) -> Vec<Vec<u8>> {
        let schema = crate::transducer::sentence::ClassVector::class_schema();
        let mut out = Vec::new();
        // index: this bubble's own vortices, its boundary
        for (class, _) in self.vortex.scoped() {
            let Ok(end) = schema.boundary(&class, 1) else {
                continue;
            };
            if let Some(s) = class.get(..end) {
                out.push(s.to_vec());
            }
        }
        out
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
